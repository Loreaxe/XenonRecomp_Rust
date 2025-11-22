pub fn sub_823D9028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9028 size=200
    let mut pc: u32 = 0x823D9028;
    'dispatch: loop {
        match pc {
            0x823D9028 => {
    //   block [0x823D9028..0x823D90F0)
	// 823D9028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D902C: 4815C085  bl 0x825350b0
	ctx.lr = 0x823D9030;
	sub_82535080(ctx, base);
	// 823D9030: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D9034: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 823D9038: 38A10078  addi r5, r1, 0x78
	ctx.r[5].s64 = ctx.r[1].s64 + 120;
	// 823D903C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 823D9040: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 823D9044: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 823D9048: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 823D904C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 823D9050: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 823D9054: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 823D9058: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 823D905C: 3941007C  addi r10, r1, 0x7c
	ctx.r[10].s64 = ctx.r[1].s64 + 124;
	// 823D9060: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 823D9064: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 823D9068: 39010074  addi r8, r1, 0x74
	ctx.r[8].s64 = ctx.r[1].s64 + 116;
	// 823D906C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 823D9070: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823D9074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823D9078: 4BFFFAF1  bl 0x823d8b68
	ctx.lr = 0x823D907C;
	sub_823D8B68(ctx, base);
	// 823D907C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9080: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 823D9084: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 823D9088: 556306BE  clrlwi r3, r11, 0x1a
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 823D908C: 4BFFEE0D  bl 0x823d7e98
	ctx.lr = 0x823D9090;
	sub_823D7E98(ctx, base);
	// 823D9090: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9094: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 823D9098: 3D400200  lis r10, 0x200
	ctx.r[10].s64 = 33554432;
	// 823D909C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823D90A0: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 823D90A4: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 823D90A8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823D90AC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823D90B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D90B4: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D90B8: 80FF0030  lwz r7, 0x30(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D90BC: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D90C0: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 823D90C4: 7D085850  subf r8, r8, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 823D90C8: 54E70026  rlwinm r7, r7, 0, 0, 0x13
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 823D90CC: 54C60026  rlwinm r6, r6, 0, 0, 0x13
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 823D90D0: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 823D90D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D90D8: 48000D69  bl 0x823d9e40
	ctx.lr = 0x823D90DC;
	sub_823D9E40(ctx, base);
	// 823D90DC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D90E0: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 823D90E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823D90E8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823D90EC: 4815C014  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D90F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D90F0 size=140
    let mut pc: u32 = 0x823D90F0;
    'dispatch: loop {
        match pc {
            0x823D90F0 => {
    //   block [0x823D90F0..0x823D917C)
	// 823D90F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D90F4: 4815BFC9  bl 0x825350bc
	ctx.lr = 0x823D90F8;
	sub_82535080(ctx, base);
	// 823D90F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D90FC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 823D9100: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 823D9104: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 823D9108: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 823D910C: 3921005C  addi r9, r1, 0x5c
	ctx.r[9].s64 = ctx.r[1].s64 + 92;
	// 823D9110: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 823D9114: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 823D9118: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823D911C: 4BFFFF0D  bl 0x823d9028
	ctx.lr = 0x823D9120;
	sub_823D9028(ctx, base);
	// 823D9120: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823D9124: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823D9128: 419A0040  beq cr6, 0x823d9168
	if ctx.cr[6].eq {
	pc = 0x823D9168; continue 'dispatch;
	}
	// 823D912C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823D9130: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9134: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9138: 396B15F8  addi r11, r11, 0x15f8
	ctx.r[11].s64 = ctx.r[11].s64 + 5624;
	// 823D913C: 55470E7C  rlwinm r7, r10, 1, 0x19, 0x1e
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 823D9140: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823D9144: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823D9148: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 823D914C: 7D6758AE  lbzx r11, r7, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 823D9150: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 823D9154: 556BE8FE  srwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823D9158: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823D915C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823D9160: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823D9164: 48000008  b 0x823d916c
	pc = 0x823D916C; continue 'dispatch;
	// 823D9168: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823D916C: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823D9170: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823D9174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823D9178: 4815BF94  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823D9180 size=780
    let mut pc: u32 = 0x823D9180;
    'dispatch: loop {
        match pc {
            0x823D9180 => {
    //   block [0x823D9180..0x823D948C)
	// 823D9180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9184: 4815BF11  bl 0x82535094
	ctx.lr = 0x823D9188;
	sub_82535080(ctx, base);
	// 823D9188: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D918C: 82810154  lwz r20, 0x154(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 823D9190: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 823D9194: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823D9198: 568B083C  slwi r11, r20, 1
	ctx.r[11].u32 = ctx.r[20].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823D919C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 823D91A0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 823D91A4: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 823D91A8: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 823D91AC: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 823D91B0: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 823D91B4: 2F130003  cmpwi cr6, r19, 3
	ctx.cr[6].compare_i32(ctx.r[19].s32, 3, &mut ctx.xer);
	// 823D91B8: 3B2B0001  addi r25, r11, 1
	ctx.r[25].s64 = ctx.r[11].s64 + 1;
	// 823D91BC: 419A0074  beq cr6, 0x823d9230
	if ctx.cr[6].eq {
	pc = 0x823D9230; continue 'dispatch;
	}
	// 823D91C0: 2F130011  cmpwi cr6, r19, 0x11
	ctx.cr[6].compare_i32(ctx.r[19].s32, 17, &mut ctx.xer);
	// 823D91C4: 419A0030  beq cr6, 0x823d91f4
	if ctx.cr[6].eq {
	pc = 0x823D91F4; continue 'dispatch;
	}
	// 823D91C8: 2F130012  cmpwi cr6, r19, 0x12
	ctx.cr[6].compare_i32(ctx.r[19].s32, 18, &mut ctx.xer);
	// 823D91CC: 419A0020  beq cr6, 0x823d91ec
	if ctx.cr[6].eq {
	pc = 0x823D91EC; continue 'dispatch;
	}
	// 823D91D0: 2F130013  cmpwi cr6, r19, 0x13
	ctx.cr[6].compare_i32(ctx.r[19].s32, 19, &mut ctx.xer);
	// 823D91D4: 419A005C  beq cr6, 0x823d9230
	if ctx.cr[6].eq {
	pc = 0x823D9230; continue 'dispatch;
	}
	// 823D91D8: 2F130014  cmpwi cr6, r19, 0x14
	ctx.cr[6].compare_i32(ctx.r[19].s32, 20, &mut ctx.xer);
	// 823D91DC: 409A005C  bne cr6, 0x823d9238
	if !ctx.cr[6].eq {
	pc = 0x823D9238; continue 'dispatch;
	}
	// 823D91E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823D91E4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 823D91E8: 4800005C  b 0x823d9244
	pc = 0x823D9244; continue 'dispatch;
	// 823D91EC: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 823D91F0: 48000054  b 0x823d9244
	pc = 0x823D9244; continue 'dispatch;
	// 823D91F4: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 823D91F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823D91FC: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 823D9200: 409A0018  bne cr6, 0x823d9218
	if !ctx.cr[6].eq {
	pc = 0x823D9218; continue 'dispatch;
	}
	// 823D9204: 7E86A378  mr r6, r20
	ctx.r[6].u64 = ctx.r[20].u64;
	// 823D9208: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823D920C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823D9210: 4BFFED21  bl 0x823d7f30
	ctx.lr = 0x823D9214;
	sub_823D7F30(ctx, base);
	// 823D9214: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 823D9218: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 823D921C: 409A0030  bne cr6, 0x823d924c
	if !ctx.cr[6].eq {
	pc = 0x823D924C; continue 'dispatch;
	}
	// 823D9220: 21760001  subfic r11, r22, 1
	ctx.xer.ca = ctx.r[22].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[22].s64;
	// 823D9224: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 823D9228: 557707FE  clrlwi r23, r11, 0x1f
	ctx.r[23].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 823D922C: 48000024  b 0x823d9250
	pc = 0x823D9250; continue 'dispatch;
	// 823D9230: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 823D9234: 48000008  b 0x823d923c
	pc = 0x823D923C; continue 'dispatch;
	// 823D9238: 83C10088  lwz r30, 0x88(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 823D923C: 2F130011  cmpwi cr6, r19, 0x11
	ctx.cr[6].compare_i32(ctx.r[19].s32, 17, &mut ctx.xer);
	// 823D9240: 419AFFB8  beq cr6, 0x823d91f8
	if ctx.cr[6].eq {
	pc = 0x823D91F8; continue 'dispatch;
	}
	// 823D9244: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823D9248: 4BFFFFB4  b 0x823d91fc
	pc = 0x823D91FC; continue 'dispatch;
	// 823D924C: 7F57D378  mr r23, r26
	ctx.r[23].u64 = ctx.r[26].u64;
	// 823D9250: 81610164  lwz r11, 0x164(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(356 as u32) ) } as u64;
	// 823D9254: 38E10084  addi r7, r1, 0x84
	ctx.r[7].s64 = ctx.r[1].s64 + 132;
	// 823D9258: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 823D925C: 82A1015C  lwz r21, 0x15c(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(348 as u32) ) } as u64;
	// 823D9260: 57FA06BE  clrlwi r26, r31, 0x1a
	ctx.r[26].u64 = ctx.r[31].u32 as u64 & 0x0000003Fu64;
	// 823D9264: 92810054  stw r20, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[20].u32 ) };
	// 823D9268: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 823D926C: 57E9C7FE  rlwinm r9, r31, 0x18, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 823D9270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823D9274: 39610088  addi r11, r1, 0x88
	ctx.r[11].s64 = ctx.r[1].s64 + 136;
	// 823D9278: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 823D927C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 823D9280: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 823D9284: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 823D9288: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823D928C: 92A1005C  stw r21, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[21].u32 ) };
	// 823D9290: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823D9294: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823D9298: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 823D929C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 823D92A0: 4BFFEE39  bl 0x823d80d8
	ctx.lr = 0x823D92A4;
	sub_823D80D8(ctx, base);
	// 823D92A4: 3D000010  lis r8, 0x10
	ctx.r[8].s64 = 1048576;
	// 823D92A8: 61080003  ori r8, r8, 3
	ctx.r[8].u64 = ctx.r[8].u64 | 3;
	// 823D92AC: 570B077B  rlwinm. r11, r24, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D92B0: 4182000C  beq 0x823d92bc
	if ctx.cr[0].eq {
	pc = 0x823D92BC; continue 'dispatch;
	}
	// 823D92B4: 3D000030  lis r8, 0x30
	ctx.r[8].s64 = 3145728;
	// 823D92B8: 61080003  ori r8, r8, 3
	ctx.r[8].u64 = ctx.r[8].u64 | 3;
	// 823D92BC: 570B05AD  rlwinm. r11, r24, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D92C0: 41820008  beq 0x823d92c8
	if ctx.cr[0].eq {
	pc = 0x823D92C8; continue 'dispatch;
	}
	// 823D92C4: 65080040  oris r8, r8, 0x40
	ctx.r[8].u64 = ctx.r[8].u64 | 4194304;
	// 823D92C8: 8161016C  lwz r11, 0x16c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(364 as u32) ) } as u64;
	// 823D92CC: 3D20FFFF  lis r9, -1
	ctx.r[9].s64 = -65536;
	// 823D92D0: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 823D92D4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D92D8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 823D92DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 823D92E0: 912B0014  stw r9, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 823D92E4: 53CA4D6C  rlwimi r10, r30, 9, 0x15, 0x16
	ctx.r[10].u64 = (((ctx.r[30].u32).rotate_left(9) as u64) & 0x0000000000000600) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFF9FF);
	// 823D92E8: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 823D92EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823D92F0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 823D92F4: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 823D92F8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 823D92FC: 41980090  blt cr6, 0x823d938c
	if ctx.cr[6].lt {
	pc = 0x823D938C; continue 'dispatch;
	}
	// 823D9300: 419A0054  beq cr6, 0x823d9354
	if ctx.cr[6].eq {
	pc = 0x823D9354; continue 'dispatch;
	}
	// 823D9304: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 823D9308: 41980020  blt cr6, 0x823d9328
	if ctx.cr[6].lt {
	pc = 0x823D9328; continue 'dispatch;
	}
	// 823D930C: 409A0090  bne cr6, 0x823d939c
	if !ctx.cr[6].eq {
	pc = 0x823D939C; continue 'dispatch;
	}
	// 823D9310: 7D39E050  subf r9, r25, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[25].s64;
	// 823D9314: 7D19E850  subf r8, r25, r29
	ctx.r[8].s64 = ctx.r[29].s64 - ctx.r[25].s64;
	// 823D9318: 38FBFFFF  addi r7, r27, -1
	ctx.r[7].s64 = ctx.r[27].s64 + -1;
	// 823D931C: 512869A4  rlwimi r8, r9, 0xd, 6, 0x12
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(13) as u64) & 0x0000000003FFE000) | (ctx.r[8].u64 & 0xFFFFFFFFFC001FFF);
	// 823D9320: 50E8D00A  rlwimi r8, r7, 0x1a, 0, 5
	ctx.r[8].u64 = (((ctx.r[7].u32).rotate_left(26) as u64) & 0x00000000FC000000) | (ctx.r[8].u64 & 0xFFFFFFFF03FFFFFF);
	// 823D9324: 48000074  b 0x823d9398
	pc = 0x823D9398; continue 'dispatch;
	// 823D9328: 812B0024  lwz r9, 0x24(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D932C: 7D19E850  subf r8, r25, r29
	ctx.r[8].s64 = ctx.r[29].s64 - ctx.r[25].s64;
	// 823D9330: 7CF9E050  subf r7, r25, r28
	ctx.r[7].s64 = ctx.r[28].s64 - ctx.r[25].s64;
	// 823D9334: 51280028  rlwimi r8, r9, 0, 0, 0x14
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FFFFF800) | (ctx.r[8].u64 & 0xFFFFFFFF000007FF);
	// 823D9338: 7D39D850  subf r9, r25, r27
	ctx.r[9].s64 = ctx.r[27].s64 - ctx.r[25].s64;
	// 823D933C: 54E75AA8  rlwinm r7, r7, 0xb, 0xa, 0x14
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x001FFFFFu64;
	// 823D9340: 5529B012  slwi r9, r9, 0x16
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(22);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823D9344: 910B0024  stw r8, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 823D9348: 5508003E  slwi r8, r8, 0
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823D934C: 5508057E  clrlwi r8, r8, 0x15
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000007FFu64;
	// 823D9350: 4800002C  b 0x823d937c
	pc = 0x823D937C; continue 'dispatch;
	// 823D9354: 812B0024  lwz r9, 0x24(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D9358: 7D19E850  subf r8, r25, r29
	ctx.r[8].s64 = ctx.r[29].s64 - ctx.r[25].s64;
	// 823D935C: 7CF9E050  subf r7, r25, r28
	ctx.r[7].s64 = ctx.r[28].s64 - ctx.r[25].s64;
	// 823D9360: 51280024  rlwimi r8, r9, 0, 0, 0x12
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FFFFE000) | (ctx.r[8].u64 & 0xFFFFFFFF00001FFF);
	// 823D9364: 393BFFFF  addi r9, r27, -1
	ctx.r[9].s64 = ctx.r[27].s64 + -1;
	// 823D9368: 54E769A4  rlwinm r7, r7, 0xd, 6, 0x12
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0007FFFFu64;
	// 823D936C: 5529D00A  slwi r9, r9, 0x1a
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(26);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823D9370: 910B0024  stw r8, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 823D9374: 5508003E  slwi r8, r8, 0
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823D9378: 550804FE  clrlwi r8, r8, 0x13
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00001FFFu64;
	// 823D937C: 7CE94B78  or r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 823D9380: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 823D9384: 912B0024  stw r9, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 823D9388: 48000014  b 0x823d939c
	pc = 0x823D939C; continue 'dispatch;
	// 823D938C: 812B0024  lwz r9, 0x24(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D9390: 7D19E850  subf r8, r25, r29
	ctx.r[8].s64 = ctx.r[29].s64 - ctx.r[25].s64;
	// 823D9394: 5128000E  rlwimi r8, r9, 0, 0, 7
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000FF000000) | (ctx.r[8].u64 & 0xFFFFFFFF00FFFFFF);
	// 823D9398: 910B0024  stw r8, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 823D939C: 7FE67E70  srawi r6, r31, 0xf
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 15) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[31].s32 >> 15) as i64;
	// 823D93A0: 812B001C  lwz r9, 0x1c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 823D93A4: 7FE46E70  srawi r4, r31, 0xd
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 13) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[31].s32 >> 13) as i64;
	// 823D93A8: 810B0020  lwz r8, 0x20(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D93AC: 7FE35E70  srawi r3, r31, 0xb
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[31].s32 >> 11) as i64;
	// 823D93B0: 80AB0028  lwz r5, 0x28(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823D93B4: 7FFE4E70  srawi r30, r31, 9
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[31].s32 >> 9) as i64;
	// 823D93B8: 80EB002C  lwz r7, 0x2c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 823D93BC: 7FFD4670  srawi r29, r31, 8
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[31].s32 >> 8) as i64;
	// 823D93C0: 7FFC3670  srawi r28, r31, 6
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[31].s32 >> 6) as i64;
	// 823D93C4: 7FFBDE70  srawi r27, r31, 0x1b
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 27) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[31].s32 >> 27) as i64;
	// 823D93C8: 5699A814  slwi r25, r20, 0x15
	ctx.r[25].u32 = ctx.r[20].u32.wrapping_shl(21);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 823D93CC: 577B077E  clrlwi r27, r27, 0x1d
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0x00000007u64;
	// 823D93D0: 50C4173A  rlwimi r4, r6, 2, 0x1c, 0x1d
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(2) as u64) & 0x000000000000000C) | (ctx.r[4].u64 & 0xFFFFFFFFFFFFFFF3);
	// 823D93D4: 80C10080  lwz r6, 0x80(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 823D93D8: 7F7BCB78  or r27, r27, r25
	ctx.r[27].u64 = ctx.r[27].u64 | ctx.r[25].u64;
	// 823D93DC: 5484073E  clrlwi r4, r4, 0x1c
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	// 823D93E0: 7FF9C670  srawi r25, r31, 0x18
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[31].s32 >> 24) as i64;
	// 823D93E4: 52BB1DF8  rlwimi r27, r21, 3, 0x17, 0x1c
	ctx.r[27].u64 = (((ctx.r[21].u32).rotate_left(3) as u64) & 0x00000000000001F8) | (ctx.r[27].u64 & 0xFFFFFFFFFFFFFE07);
	// 823D93E8: 53A67022  rlwimi r6, r29, 0xe, 0, 0x11
	ctx.r[6].u64 = (((ctx.r[29].u32).rotate_left(14) as u64) & 0x00000000FFFFC000) | (ctx.r[6].u64 & 0xFFFFFFFF00003FFF);
	// 823D93EC: 5083103A  rlwimi r3, r4, 2, 0, 0x1d
	ctx.r[3].u64 = (((ctx.r[4].u32).rotate_left(2) as u64) & 0x00000000FFFFFFFC) | (ctx.r[3].u64 & 0xFFFFFFFF00000003);
	// 823D93F0: 7FE4AE70  srawi r4, r31, 0x15
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 21) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[31].s32 >> 21) as i64;
	// 823D93F4: 53791838  rlwimi r25, r27, 3, 0, 0x1c
	ctx.r[25].u64 = (((ctx.r[27].u32).rotate_left(3) as u64) & 0x00000000FFFFFFF8) | (ctx.r[25].u64 & 0xFFFFFFFF00000007);
	// 823D93F8: 50C98812  rlwimi r9, r6, 0x11, 0, 9
	ctx.r[9].u64 = (((ctx.r[6].u32).rotate_left(17) as u64) & 0x00000000FFC00000) | (ctx.r[9].u64 & 0xFFFFFFFF003FFFFF);
	// 823D93FC: 53241838  rlwimi r4, r25, 3, 0, 0x1c
	ctx.r[4].u64 = (((ctx.r[25].u32).rotate_left(3) as u64) & 0x00000000FFFFFFF8) | (ctx.r[4].u64 & 0xFFFFFFFF00000007);
	// 823D9400: 7FE69670  srawi r6, r31, 0x12
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 18) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[31].s32 >> 18) as i64;
	// 823D9404: 507E103A  rlwimi r30, r3, 2, 0, 0x1d
	ctx.r[30].u64 = (((ctx.r[3].u32).rotate_left(2) as u64) & 0x00000000FFFFFFFC) | (ctx.r[30].u64 & 0xFFFFFFFF00000003);
	// 823D9408: 3B13FFED  addi r24, r19, -0x13
	ctx.r[24].s64 = ctx.r[19].s64 + -19;
	// 823D940C: 50861838  rlwimi r6, r4, 3, 0, 0x1c
	ctx.r[6].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x00000000FFFFFFF8) | (ctx.r[6].u64 & 0xFFFFFFFF00000007);
	// 823D9410: 52EA5D28  rlwimi r10, r23, 0xb, 0x14, 0x14
	ctx.r[10].u64 = (((ctx.r[23].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFF7FF);
	// 823D9414: 7FE48E70  srawi r4, r31, 0x11
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 17) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[31].s32 >> 17) as i64;
	// 823D9418: 5529002A  rlwinm r9, r9, 0, 0, 0x15
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823D941C: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823D9420: 7F1D0034  cntlzw r29, r24
	ctx.r[29].u64 = if ctx.r[24].u32 == 0 { 32 } else { ctx.r[24].u32.leading_zeros() as u64 };
	// 823D9424: 50C4083C  rlwimi r4, r6, 1, 0, 0x1e
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(1) as u64) & 0x00000000FFFFFFFE) | (ctx.r[4].u64 & 0xFFFFFFFF00000001);
	// 823D9428: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 823D942C: 55080032  rlwinm r8, r8, 0, 0, 0x19
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9430: 7C694B78  or r9, r3, r9
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[9].u64;
	// 823D9434: 57BDDFFE  rlwinm r29, r29, 0x1b, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0x0000001Fu64;
	// 823D9438: 54A60058  rlwinm r6, r5, 0, 1, 0xc
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 823D943C: 7D08D378  or r8, r8, r26
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[26].u64;
	// 823D9440: 61290002  ori r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u64 | 2;
	// 823D9444: 7C8A3378  or r10, r4, r6
	ctx.r[10].u64 = ctx.r[4].u64 | ctx.r[6].u64;
	// 823D9448: 53BC26F6  rlwimi r28, r29, 4, 0x1b, 0x1b
	ctx.r[28].u64 = (((ctx.r[29].u32).rotate_left(4) as u64) & 0x0000000000000010) | (ctx.r[28].u64 & 0xFFFFFFFFFFFFFFEF);
	// 823D944C: 3BD6FFFF  addi r30, r22, -1
	ctx.r[30].s64 = ctx.r[22].s64 + -1;
	// 823D9450: 53883632  rlwimi r8, r28, 6, 0x18, 0x19
	ctx.r[8].u64 = (((ctx.r[28].u32).rotate_left(6) as u64) & 0x00000000000000C0) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFF3F);
	// 823D9454: 912B001C  stw r9, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 823D9458: 53C735B2  rlwimi r7, r30, 6, 0x16, 0x19
	ctx.r[7].u64 = (((ctx.r[30].u32).rotate_left(6) as u64) & 0x00000000000003C0) | (ctx.r[7].u64 & 0xFFFFFFFFFFFFFC3F);
	// 823D945C: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 823D9460: 5388356A  rlwimi r8, r28, 6, 0x15, 0x15
	ctx.r[8].u64 = (((ctx.r[28].u32).rotate_left(6) as u64) & 0x0000000000000400) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFBFF);
	// 823D9464: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 823D9468: 81210084  lwz r9, 0x84(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 823D946C: 90EB002C  stw r7, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 823D9470: 910B0020  stw r8, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 823D9474: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 823D9478: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823D947C: 8141017C  lwz r10, 0x17c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 823D9480: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823D9484: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 823D9488: 4815BC5C  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9490 size=596
    let mut pc: u32 = 0x823D9490;
    'dispatch: loop {
        match pc {
            0x823D9490 => {
    //   block [0x823D9490..0x823D96E4)
	// 823D9490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9494: 4815BC05  bl 0x82535098
	ctx.lr = 0x823D9498;
	sub_82535080(ctx, base);
	// 823D9498: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D949C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 823D94A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823D94A4: 579806BE  clrlwi r24, r28, 0x1a
	ctx.r[24].u64 = ctx.r[28].u32 as u64 & 0x0000003Fu64;
	// 823D94A8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 823D94AC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 823D94B0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 823D94B4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 823D94B8: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 823D94BC: 7D545378  mr r20, r10
	ctx.r[20].u64 = ctx.r[10].u64;
	// 823D94C0: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 823D94C4: 2B180016  cmplwi cr6, r24, 0x16
	ctx.cr[6].compare_u32(ctx.r[24].u32, 22 as u32, &mut ctx.xer);
	// 823D94C8: 419A0010  beq cr6, 0x823d94d8
	if ctx.cr[6].eq {
	pc = 0x823D94D8; continue 'dispatch;
	}
	// 823D94CC: 2B180017  cmplwi cr6, r24, 0x17
	ctx.cr[6].compare_u32(ctx.r[24].u32, 23 as u32, &mut ctx.xer);
	// 823D94D0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 823D94D4: 409A0008  bne cr6, 0x823d94dc
	if !ctx.cr[6].eq {
	pc = 0x823D94DC; continue 'dispatch;
	}
	// 823D94D8: 7ED9B378  mr r25, r22
	ctx.r[25].u64 = ctx.r[22].u64;
	// 823D94DC: 2B180036  cmplwi cr6, r24, 0x36
	ctx.cr[6].compare_u32(ctx.r[24].u32, 54 as u32, &mut ctx.xer);
	// 823D94E0: 409A0008  bne cr6, 0x823d94e8
	if !ctx.cr[6].eq {
	pc = 0x823D94E8; continue 'dispatch;
	}
	// 823D94E4: 3B000007  li r24, 7
	ctx.r[24].s64 = 7;
	// 823D94E8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 823D94EC: 92DF0004  stw r22, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[22].u32 ) };
	// 823D94F0: 3D40FFFF  lis r10, -1
	ctx.r[10].s64 = -65536;
	// 823D94F4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 823D94F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823D94FC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823D9500: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823D9504: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823D9508: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 823D950C: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 823D9510: 4BFDBFD9  bl 0x823b54e8
	ctx.lr = 0x823D9514;
	sub_823B54E8(ctx, base);
	// 823D9514: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 823D9518: 419A0040  beq cr6, 0x823d9558
	if ctx.cr[6].eq {
	pc = 0x823D9558; continue 'dispatch;
	}
	// 823D951C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823D9520: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 823D9524: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 823D9528: 41980008  blt cr6, 0x823d9530
	if ctx.cr[6].lt {
	pc = 0x823D9530; continue 'dispatch;
	}
	// 823D952C: 576A083C  slwi r10, r27, 1
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823D9530: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 823D9534: 409A0008  bne cr6, 0x823d953c
	if !ctx.cr[6].eq {
	pc = 0x823D953C; continue 'dispatch;
	}
	// 823D9538: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823D953C: 396B001F  addi r11, r11, 0x1f
	ctx.r[11].s64 = ctx.r[11].s64 + 31;
	// 823D9540: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 823D9544: 556BD97E  srwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823D9548: 554AE13E  srwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823D954C: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 823D9550: 5567027E  clrlwi r7, r11, 9
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 823D9554: 48000008  b 0x823d955c
	pc = 0x823D955C; continue 'dispatch;
	// 823D9558: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823D955C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 823D9560: 419A0014  beq cr6, 0x823d9574
	if ctx.cr[6].eq {
	pc = 0x823D9574; continue 'dispatch;
	}
	// 823D9564: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9568: 82FA0004  lwz r23, 4(r26)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 823D956C: 811A0008  lwz r8, 8(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9570: 48000028  b 0x823d9598
	pc = 0x823D9598; continue 'dispatch;
	// 823D9574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823D9578: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823D957C: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 823D9580: 419A0018  beq cr6, 0x823d9598
	if ctx.cr[6].eq {
	pc = 0x823D9598; continue 'dispatch;
	}
	// 823D9584: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 823D9588: 816BE16C  lwz r11, -0x1e94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7828 as u32) ) } as u64;
	// 823D958C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9590: 409A0008  bne cr6, 0x823d9598
	if !ctx.cr[6].eq {
	pc = 0x823D9598; continue 'dispatch;
	}
	// 823D9594: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 823D9598: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 823D959C: 419A000C  beq cr6, 0x823d95a8
	if ctx.cr[6].eq {
	pc = 0x823D95A8; continue 'dispatch;
	}
	// 823D95A0: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 823D95A4: 409A0020  bne cr6, 0x823d95c4
	if !ctx.cr[6].eq {
	pc = 0x823D95C4; continue 'dispatch;
	}
	// 823D95A8: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 823D95AC: 397D004F  addi r11, r29, 0x4f
	ctx.r[11].s64 = ctx.r[29].s64 + 79;
	// 823D95B0: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 823D95B4: 1D4B0050  mulli r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 * 80;
	// 823D95B8: 397D001F  addi r11, r29, 0x1f
	ctx.r[11].s64 = ctx.r[29].s64 + 31;
	// 823D95BC: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823D95C0: 4800001C  b 0x823d95dc
	pc = 0x823D95DC; continue 'dispatch;
	// 823D95C4: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 823D95C8: 397D0027  addi r11, r29, 0x27
	ctx.r[11].s64 = ctx.r[29].s64 + 39;
	// 823D95CC: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 823D95D0: 1D4B0028  mulli r10, r11, 0x28
	ctx.r[10].s64 = ctx.r[11].s64 * 40;
	// 823D95D4: 397D000F  addi r11, r29, 0xf
	ctx.r[11].s64 = ctx.r[29].s64 + 15;
	// 823D95D8: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823D95DC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D95E0: 38BDFFFF  addi r5, r29, -1
	ctx.r[5].s64 = ctx.r[29].s64 + -1;
	// 823D95E4: 517E103A  rlwimi r30, r11, 2, 0, 0x1d
	ctx.r[30].u64 = (((ctx.r[11].u32).rotate_left(2) as u64) & 0x00000000FFFFFFFC) | (ctx.r[30].u64 & 0xFFFFFFFF00000003);
	// 823D95E8: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 823D95EC: 54A5901A  slwi r5, r5, 0x12
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(18);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 823D95F0: 54C6077E  clrlwi r6, r6, 0x1d
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x00000007u64;
	// 823D95F4: 554A04BE  clrlwi r10, r10, 0x12
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 823D95F8: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 823D95FC: 38BBFFFF  addi r5, r27, -1
	ctx.r[5].s64 = ctx.r[27].s64 + -1;
	// 823D9600: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823D9604: 50A61BB8  rlwimi r6, r5, 3, 0xe, 0x1c
	ctx.r[6].u64 = (((ctx.r[5].u32).rotate_left(3) as u64) & 0x000000000003FFF8) | (ctx.r[6].u64 & 0xFFFFFFFFFFFC0007);
	// 823D9608: 57C5801E  slwi r5, r30, 0x10
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(16);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 823D960C: 396B15F8  addi r11, r11, 0x15f8
	ctx.r[11].s64 = ctx.r[11].s64 + 5624;
	// 823D9610: 7CAA5378  or r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[10].u64;
	// 823D9614: 5705083C  slwi r5, r24, 1
	ctx.r[5].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 823D9618: 90DF0024  stw r6, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 823D961C: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 823D9620: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 823D9624: 7D655A2E  lhzx r11, r5, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 823D9628: 556BC73E  rlwinm r11, r11, 0x18, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823D962C: 419A004C  beq cr6, 0x823d9678
	if ctx.cr[6].eq {
	pc = 0x823D9678; continue 'dispatch;
	}
	// 823D9630: 556B83DE  rlwinm r11, r11, 0x10, 0xf, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823D9634: 552A053E  clrlwi r10, r9, 0x14
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 823D9638: 2F17FFFF  cmpwi cr6, r23, -1
	ctx.cr[6].compare_i32(ctx.r[23].s32, -1, &mut ctx.xer);
	// 823D963C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823D9640: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 823D9644: 419A0028  beq cr6, 0x823d966c
	if ctx.cr[6].eq {
	pc = 0x823D966C; continue 'dispatch;
	}
	// 823D9648: 3978FFEA  addi r11, r24, -0x16
	ctx.r[11].s64 = ctx.r[24].s64 + -22;
	// 823D964C: 56EA6824  slwi r10, r23, 0xd
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shl(13);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823D9650: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 823D9654: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823D9658: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 823D965C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823D9660: 51762036  rlwimi r22, r11, 4, 0, 0x1b
	ctx.r[22].u64 = (((ctx.r[11].u32).rotate_left(4) as u64) & 0x00000000FFFFFFF0) | (ctx.r[22].u64 & 0xFFFFFFFF0000000F);
	// 823D9664: 92DF0020  stw r22, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[22].u32 ) };
	// 823D9668: 48000064  b 0x823d96cc
	pc = 0x823D96CC; continue 'dispatch;
	// 823D966C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823D9670: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 823D9674: 48000058  b 0x823d96cc
	pc = 0x823D96CC; continue 'dispatch;
	// 823D9678: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823D967C: 409A003C  bne cr6, 0x823d96b8
	if !ctx.cr[6].eq {
	pc = 0x823D96B8; continue 'dispatch;
	}
	// 823D9680: 578A039D  rlwinm. r10, r28, 0, 0xe, 0xe
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823D9684: 40820034  bne 0x823d96b8
	if !ctx.cr[0].eq {
	pc = 0x823D96B8; continue 'dispatch;
	}
	// 823D9688: 578A056C  rlwinm r10, r28, 0, 0x15, 0x16
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 823D968C: 2F0A0600  cmpwi cr6, r10, 0x600
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1536, &mut ctx.xer);
	// 823D9690: 409A0028  bne cr6, 0x823d96b8
	if !ctx.cr[6].eq {
	pc = 0x823D96B8; continue 'dispatch;
	}
	// 823D9694: 578A04E8  rlwinm r10, r28, 0, 0x13, 0x14
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9698: 2F0A1800  cmpwi cr6, r10, 0x1800
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6144, &mut ctx.xer);
	// 823D969C: 409A001C  bne cr6, 0x823d96b8
	if !ctx.cr[6].eq {
	pc = 0x823D96B8; continue 'dispatch;
	}
	// 823D96A0: 578A0464  rlwinm r10, r28, 0, 0x11, 0x12
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 823D96A4: 2F0A6000  cmpwi cr6, r10, 0x6000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24576, &mut ctx.xer);
	// 823D96A8: 409A0010  bne cr6, 0x823d96b8
	if !ctx.cr[6].eq {
	pc = 0x823D96B8; continue 'dispatch;
	}
	// 823D96AC: 578A03E1  rlwinm. r10, r28, 0, 0xf, 0x10
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823D96B0: 40820008  bne 0x823d96b8
	if !ctx.cr[0].eq {
	pc = 0x823D96B8; continue 'dispatch;
	}
	// 823D96B4: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 823D96B8: 510B25B6  rlwimi r11, r8, 4, 0x16, 0x1b
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(4) as u64) & 0x00000000000003F0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFC0F);
	// 823D96BC: 552A053E  clrlwi r10, r9, 0x14
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000FFFu64;
	// 823D96C0: 556B819E  rlwinm r11, r11, 0x10, 6, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823D96C4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823D96C8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 823D96CC: 1D631400  mulli r11, r3, 0x1400
	ctx.r[11].s64 = ctx.r[3].s64 * 5120;
	// 823D96D0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 823D96D4: 90750000  stw r3, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 823D96D8: 90F40000  stw r7, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 823D96DC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823D96E0: 4815BA08  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D96E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D96E8 size=16
    let mut pc: u32 = 0x823D96E8;
    'dispatch: loop {
        match pc {
            0x823D96E8 => {
    //   block [0x823D96E8..0x823D96F8)
	// 823D96E8: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 823D96EC: 556BD73E  rlwinm r11, r11, 0x1a, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 823D96F0: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 823D96F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D96F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D96F8 size=24
    let mut pc: u32 = 0x823D96F8;
    'dispatch: loop {
        match pc {
            0x823D96F8 => {
    //   block [0x823D96F8..0x823D9710)
	// 823D96F8: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 823D96FC: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 823D9700: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 823D9704: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 823D9708: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823D970C: 4BFFF9E4  b 0x823d90f0
	sub_823D90F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9710 size=4
    let mut pc: u32 = 0x823D9710;
    'dispatch: loop {
        match pc {
            0x823D9710 => {
    //   block [0x823D9710..0x823D9714)
	// 823D9710: 4BFFF258  b 0x823d8968
	sub_823D8968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9718 size=4
    let mut pc: u32 = 0x823D9718;
    'dispatch: loop {
        match pc {
            0x823D9718 => {
    //   block [0x823D9718..0x823D971C)
	// 823D9718: 4BFFF350  b 0x823d8a68
	sub_823D8A68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9720 size=288
    let mut pc: u32 = 0x823D9720;
    'dispatch: loop {
        match pc {
            0x823D9720 => {
    //   block [0x823D9720..0x823D9840)
	// 823D9720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9724: 4815B97D  bl 0x825350a0
	ctx.lr = 0x823D9728;
	sub_82535080(ctx, base);
	// 823D9728: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D972C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823D9730: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 823D9734: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823D9738: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 823D973C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 823D9740: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 823D9744: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 823D9748: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 823D974C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 823D9750: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 823D9754: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
	// 823D9758: 4BFF0A69  bl 0x823ca1c0
	ctx.lr = 0x823D975C;
	sub_823CA1C0(ctx, base);
	// 823D975C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823D9760: 4082000C  bne 0x823d976c
	if !ctx.cr[0].eq {
	pc = 0x823D976C; continue 'dispatch;
	}
	// 823D9764: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823D9768: 480000D0  b 0x823d9838
	pc = 0x823D9838; continue 'dispatch;
	// 823D976C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 823D9770: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 823D9774: 39610084  addi r11, r1, 0x84
	ctx.r[11].s64 = ctx.r[1].s64 + 132;
	// 823D9778: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 823D977C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 823D9780: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 823D9784: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 823D9788: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 823D978C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 823D9790: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 823D9794: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 823D9798: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 823D979C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 823D97A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823D97A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823D97A8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 823D97AC: 4BFFF9D5  bl 0x823d9180
	ctx.lr = 0x823D97B0;
	sub_823D9180(ctx, base);
	// 823D97B0: 7FABE8F8  nor r11, r29, r29
	ctx.r[11].u64 = !(ctx.r[29].u64 | ctx.r[29].u64);
	// 823D97B4: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 823D97B8: 556BF7FE  rlwinm r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823D97BC: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823D97C0: 556BE006  slwi r11, r11, 0x1c
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(28);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823D97C4: 657E8C80  oris r30, r11, 0x8c80
	ctx.r[30].u64 = ctx.r[11].u64 | 2357198848;
	// 823D97C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823D97CC: 4BFF09F5  bl 0x823ca1c0
	ctx.lr = 0x823D97D0;
	sub_823CA1C0(ctx, base);
	// 823D97D0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823D97D4: 40820014  bne 0x823d97e8
	if !ctx.cr[0].eq {
	pc = 0x823D97E8; continue 'dispatch;
	}
	// 823D97D8: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 823D97DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D97E0: 4BFF0A79  bl 0x823ca258
	ctx.lr = 0x823D97E4;
	sub_823CA258(ctx, base);
	// 823D97E4: 4BFFFF80  b 0x823d9764
	pc = 0x823D9764; continue 'dispatch;
	// 823D97E8: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 823D97EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D97F0: 419A002C  beq cr6, 0x823d981c
	if ctx.cr[6].eq {
	pc = 0x823D981C; continue 'dispatch;
	}
	// 823D97F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823D97F8: 4BFF09C9  bl 0x823ca1c0
	ctx.lr = 0x823D97FC;
	sub_823CA1C0(ctx, base);
	// 823D97FC: 7C761B79  or. r22, r3, r3
	ctx.r[22].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 823D9800: 4082001C  bne 0x823d981c
	if !ctx.cr[0].eq {
	pc = 0x823D981C; continue 'dispatch;
	}
	// 823D9804: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 823D9808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D980C: 4BFF0A4D  bl 0x823ca258
	ctx.lr = 0x823D9810;
	sub_823CA258(ctx, base);
	// 823D9810: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 823D9814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823D9818: 4BFFFFC8  b 0x823d97e0
	pc = 0x823D97E0; continue 'dispatch;
	// 823D981C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D9824: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D9828: 53AB0026  rlwimi r11, r29, 0, 0, 0x13
	ctx.r[11].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[11].u64 & 0xFFFFFFFF00000FFF);
	// 823D982C: 5156053E  rlwimi r22, r10, 0, 0x14, 0x1f
	ctx.r[22].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x0000000000000FFF) | (ctx.r[22].u64 & 0xFFFFFFFFFFFFF000);
	// 823D9830: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 823D9834: 92DF0030  stw r22, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[22].u32 ) };
	// 823D9838: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 823D983C: 4815B8B4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9840 size=296
    let mut pc: u32 = 0x823D9840;
    'dispatch: loop {
        match pc {
            0x823D9840 => {
    //   block [0x823D9840..0x823D9968)
	// 823D9840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9844: 4815B86D  bl 0x825350b0
	ctx.lr = 0x823D9848;
	sub_82535080(ctx, base);
	// 823D9848: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D984C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823D9850: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823D9854: 3C806480  lis r4, 0x6480
	ctx.r[4].s64 = 1686110208;
	// 823D9858: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 823D985C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 823D9860: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 823D9864: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 823D9868: 4BFF0959  bl 0x823ca1c0
	ctx.lr = 0x823D986C;
	sub_823CA1C0(ctx, base);
	// 823D986C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823D9870: 4082000C  bne 0x823d987c
	if !ctx.cr[0].eq {
	pc = 0x823D987C; continue 'dispatch;
	}
	// 823D9874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823D9878: 480000E8  b 0x823d9960
	pc = 0x823D9960; continue 'dispatch;
	// 823D987C: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 823D9880: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 823D9884: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 823D9888: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823D988C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 823D9890: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 823D9894: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823D9898: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823D989C: 4BFFFBF5  bl 0x823d9490
	ctx.lr = 0x823D98A0;
	sub_823D9490(ctx, base);
	// 823D98A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D98A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823D98A8: 656B0010  oris r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 1048576;
	// 823D98AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823D98B0: 409A00AC  bne cr6, 0x823d995c
	if !ctx.cr[6].eq {
	pc = 0x823D995C; continue 'dispatch;
	}
	// 823D98B4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 823D98B8: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 823D98BC: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 823D98C0: 419A0010  beq cr6, 0x823d98d0
	if ctx.cr[6].eq {
	pc = 0x823D98D0; continue 'dispatch;
	}
	// 823D98C4: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 823D98C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823D98CC: 409A0008  bne cr6, 0x823d98d4
	if !ctx.cr[6].eq {
	pc = 0x823D98D4; continue 'dispatch;
	}
	// 823D98D0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823D98D4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823D98D8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823D98DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823D98E0: 48006201  bl 0x823dfae0
	ctx.lr = 0x823D98E4;
	sub_823DFAE0(ctx, base);
	// 823D98E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 823D98E8: 40820014  bne 0x823d98fc
	if !ctx.cr[0].eq {
	pc = 0x823D98FC; continue 'dispatch;
	}
	// 823D98EC: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 823D98F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D98F4: 4BFF0965  bl 0x823ca258
	ctx.lr = 0x823D98F8;
	sub_823CA258(ctx, base);
	// 823D98F8: 4BFFFF7C  b 0x823d9874
	pc = 0x823D9874; continue 'dispatch;
	// 823D98FC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823D9900: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 823D9904: 2B0B0800  cmplwi cr6, r11, 0x800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2048 as u32, &mut ctx.xer);
	// 823D9908: 40990010  ble cr6, 0x823d9918
	if !ctx.cr[6].gt {
	pc = 0x823D9918; continue 'dispatch;
	}
	// 823D990C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823D9910: 48006181  bl 0x823dfa90
	ctx.lr = 0x823D9914;
	sub_823DFA90(ctx, base);
	// 823D9914: 4BFFFFD8  b 0x823d98ec
	pc = 0x823D98EC; continue 'dispatch;
	// 823D9918: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D991C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823D9920: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 823D9924: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823D9928: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 823D992C: 51630026  rlwimi r3, r11, 0, 0, 0x13
	ctx.r[3].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFF000) | (ctx.r[3].u64 & 0xFFFFFFFF00000FFF);
	// 823D9930: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 823D9934: 419A0028  beq cr6, 0x823d995c
	if ctx.cr[6].eq {
	pc = 0x823D995C; continue 'dispatch;
	}
	// 823D9938: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 823D993C: 814BE16C  lwz r10, -0x1e94(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7828 as u32) ) } as u64;
	// 823D9940: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823D9944: 409A0018  bne cr6, 0x823d995c
	if !ctx.cr[6].eq {
	pc = 0x823D995C; continue 'dispatch;
	}
	// 823D9948: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823D994C: 914BE16C  stw r10, -0x1e94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-7828 as u32), ctx.r[10].u32 ) };
	// 823D9950: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9954: 556B03FE  clrlwi r11, r11, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 823D9958: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 823D995C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D9960: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823D9964: 4815B79C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9968 size=152
    let mut pc: u32 = 0x823D9968;
    'dispatch: loop {
        match pc {
            0x823D9968 => {
    //   block [0x823D9968..0x823D9A00)
	// 823D9968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D996C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823D9970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823D9974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D9978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D997C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823D9980: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9984: 4182001C  beq 0x823d99a0
	if ctx.cr[0].eq {
	pc = 0x823D99A0; continue 'dispatch;
	}
	// 823D9988: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 823D998C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823D9990: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D9994: 5564273E  srwi r4, r11, 0x1c
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 823D9998: 4BFFEFD1  bl 0x823d8968
	ctx.lr = 0x823D999C;
	sub_823D8968(ctx, base);
	// 823D999C: 48000048  b 0x823d99e4
	pc = 0x823D99E4; continue 'dispatch;
	// 823D99A0: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 823D99A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823D99A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 823D99AC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 823D99B0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 823D99B4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823D99B8: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D99BC: 556B74BE  srwi r11, r11, 0x12
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(18);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823D99C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823D99C4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 823D99C8: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D99CC: 556BEC7E  rlwinm r11, r11, 0x1d, 0x11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 823D99D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823D99D4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 823D99D8: A1630018  lhz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D99DC: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823D99E0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 823D99E4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 823D99E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823D99EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823D99F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823D99F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823D99F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823D99FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9A00 size=68
    let mut pc: u32 = 0x823D9A00;
    'dispatch: loop {
        match pc {
            0x823D9A00 => {
    //   block [0x823D9A00..0x823D9A44)
	// 823D9A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823D9A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823D9A0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D9A10: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 823D9A14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823D9A18: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D9A1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823D9A20: 5564273E  srwi r4, r11, 0x1c
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 823D9A24: 4BFFF045  bl 0x823d8a68
	ctx.lr = 0x823D9A28;
	sub_823D8A68(ctx, base);
	// 823D9A28: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 823D9A2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823D9A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823D9A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823D9A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823D9A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823D9A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9A48 size=380
    let mut pc: u32 = 0x823D9A48;
    'dispatch: loop {
        match pc {
            0x823D9A48 => {
    //   block [0x823D9A48..0x823D9BC4)
	// 823D9A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9A4C: 4815B659  bl 0x825350a4
	ctx.lr = 0x823D9A50;
	sub_82535080(ctx, base);
	// 823D9A50: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D9A54: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 823D9A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823D9A5C: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 823D9A60: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823D9A64: 7F7CF82E  lwzx r27, r28, r31
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 823D9A68: 419A00E4  beq cr6, 0x823d9b4c
	if ctx.cr[6].eq {
	pc = 0x823D9B4C; continue 'dispatch;
	}
	// 823D9A6C: 81050020  lwz r8, 0x20(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9A70: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 823D9A74: 80E50030  lwz r7, 0x30(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D9A78: 7C9F2214  add r4, r31, r4
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 823D9A7C: 551D653E  srwi r29, r8, 0x14
	ctx.r[29].u32 = ctx.r[8].u32.wrapping_shr(20);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 823D9A80: 8125002C  lwz r9, 0x2c(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 823D9A84: 54FE653E  srwi r30, r7, 0x14
	ctx.r[30].u32 = ctx.r[7].u32.wrapping_shr(20);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 823D9A88: 8345001C  lwz r26, 0x1c(r5)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 823D9A8C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 823D9A90: 83250028  lwz r25, 0x28(r5)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 823D9A94: 83050024  lwz r24, 0x24(r5)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 823D9A98: 88642EAE  lbz r3, 0x2eae(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(11950 as u32) ) } as u64;
	// 823D9A9C: 3BBD0200  addi r29, r29, 0x200
	ctx.r[29].s64 = ctx.r[29].s64 + 512;
	// 823D9AA0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 823D9AA4: 3AFE0200  addi r23, r30, 0x200
	ctx.r[23].s64 = ctx.r[30].s64 + 512;
	// 823D9AA8: 57BE04E6  rlwinm r30, r29, 0, 0x13, 0x13
	ctx.r[30].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9AAC: 551D00FE  clrlwi r29, r8, 3
	ctx.r[29].u64 = ctx.r[8].u32 as u64 & 0x1FFFFFFFu64;
	// 823D9AB0: 56E804E6  rlwinm r8, r23, 0, 0x13, 0x13
	ctx.r[8].u64 = ctx.r[23].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9AB4: 7FDEEA14  add r30, r30, r29
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 823D9AB8: 82EB0010  lwz r23, 0x10(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 823D9ABC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 823D9AC0: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9AC4: 54E700EC  rlwinm r7, r7, 0, 3, 0x16
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9AC8: 5529F73E  rlwinm r9, r9, 0x1e, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 823D9ACC: 930B0008  stw r24, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 823D9AD0: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 823D9AD4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823D9AD8: 52EA07AA  rlwimi r10, r23, 0, 0x1e, 0x15
	ctx.r[10].u64 = (((ctx.r[23].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFFFC03) | (ctx.r[10].u64 & 0x00000000000003FC);
	// 823D9ADC: 82EB000C  lwz r23, 0xc(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823D9AE0: 53BA02AA  rlwimi r26, r29, 0, 0xa, 0x15
	ctx.r[26].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000003FFC00) | (ctx.r[26].u64 & 0xFFFFFFFFFFC003FF);
	// 823D9AE4: 83AB0014  lwz r29, 0x14(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 823D9AE8: 50FE0528  rlwimi r30, r7, 0, 0x14, 0x14
	ctx.r[30].u64 = (((ctx.r[7].u32).rotate_left(0) as u64) & 0x0000000000000800) | (ctx.r[30].u64 & 0xFFFFFFFFFFFFF7FF);
	// 823D9AEC: 52F90058  rlwimi r25, r23, 0, 1, 0xc
	ctx.r[25].u64 = (((ctx.r[23].u32).rotate_left(0) as u64) & 0x000000007FF80000) | (ctx.r[25].u64 & 0xFFFFFFFF8007FFFF);
	// 823D9AF0: 53A805FE  rlwimi r8, r29, 0, 0x17, 0x1f
	ctx.r[8].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000000001FF) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFE00);
	// 823D9AF4: 7F091840  cmplw cr6, r9, r3
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[3].u32, &mut ctx.xer);
	// 823D9AF8: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 823D9AFC: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 823D9B00: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 823D9B04: 932B000C  stw r25, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 823D9B08: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 823D9B0C: 4099000C  ble cr6, 0x823d9b18
	if !ctx.cr[6].gt {
	pc = 0x823D9B18; continue 'dispatch;
	}
	// 823D9B10: 8125002C  lwz r9, 0x2c(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 823D9B14: 5523F73E  rlwinm r3, r9, 0x1e, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 823D9B18: 8105002C  lwz r8, 0x2c(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 823D9B1C: 506A16BA  rlwimi r10, r3, 2, 0x1a, 0x1d
	ctx.r[10].u64 = (((ctx.r[3].u32).rotate_left(2) as u64) & 0x000000000000003C) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFC3);
	// 823D9B20: 89242EC8  lbz r9, 0x2ec8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(11976 as u32) ) } as u64;
	// 823D9B24: 5508D73E  rlwinm r8, r8, 0x1a, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000003Fu64;
	// 823D9B28: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823D9B2C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 823D9B30: 40980008  bge cr6, 0x823d9b38
	if !ctx.cr[6].lt {
	pc = 0x823D9B38; continue 'dispatch;
	}
	// 823D9B34: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 823D9B38: E91F0018  ld r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 823D9B3C: 512A35B2  rlwimi r10, r9, 6, 0x16, 0x19
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(6) as u64) & 0x00000000000003C0) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFC3F);
	// 823D9B40: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 823D9B44: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 823D9B48: F91F0018  std r8, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u64 ) };
	// 823D9B4C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 823D9B50: 7CBCF92E  stwx r5, r28, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 823D9B54: 419A0068  beq cr6, 0x823d9bbc
	if ctx.cr[6].eq {
	pc = 0x823D9BBC; continue 'dispatch;
	}
	// 823D9B58: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823D9B5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9B60: 4182000C  beq 0x823d9b6c
	if ctx.cr[0].eq {
	pc = 0x823D9B6C; continue 'dispatch;
	}
	// 823D9B64: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 823D9B68: 48000054  b 0x823d9bbc
	pc = 0x823D9BBC; continue 'dispatch;
	// 823D9B6C: 817F2AA0  lwz r11, 0x2aa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10912 as u32) ) } as u64;
	// 823D9B70: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9B74: 7D6B5039  and. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9B78: 41820044  beq 0x823d9bbc
	if ctx.cr[0].eq {
	pc = 0x823D9BBC; continue 'dispatch;
	}
	// 823D9B7C: 807F34D4  lwz r3, 0x34d4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13524 as u32) ) } as u64;
	// 823D9B80: 817F34D8  lwz r11, 0x34d8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13528 as u32) ) } as u64;
	// 823D9B84: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823D9B88: 4198000C  blt cr6, 0x823d9b94
	if ctx.cr[6].lt {
	pc = 0x823D9B94; continue 'dispatch;
	}
	// 823D9B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D9B90: 480023E9  bl 0x823dbf78
	ctx.lr = 0x823D9B94;
	sub_823DBF78(ctx, base);
	// 823D9B94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823D9B98: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 823D9B9C: 536BF0BE  rlwimi r11, r27, 0x1e, 2, 0x1f
	ctx.r[11].u64 = (((ctx.r[27].u32).rotate_left(30) as u64) & 0x000000003FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFFC0000000);
	// 823D9BA0: 556B0080  rlwinm r11, r11, 0, 2, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9BA4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823D9BA8: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 823D9BAC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823D9BB0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823D9BB4: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 823D9BB8: 915F34D4  stw r10, 0x34d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13524 as u32), ctx.r[10].u32 ) };
	// 823D9BBC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 823D9BC0: 4815B534  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9BC8 size=40
    let mut pc: u32 = 0x823D9BC8;
    'dispatch: loop {
        match pc {
            0x823D9BC8 => {
    //   block [0x823D9BC8..0x823D9BF0)
	// 823D9BC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9BCC: 554B073E  clrlwi r11, r10, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 823D9BD0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823D9BD4: 409A0050  bne cr6, 0x823d9c24
	if !ctx.cr[6].eq {
		sub_823D9C24(ctx, base);
		return;
	}
	// 823D9BD8: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D9BDC: 554ABFBE  rlwinm r10, r10, 0x17, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000001FFu64;
	// 823D9BE0: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 823D9BE4: 409A000C  bne cr6, 0x823d9bf0
	if !ctx.cr[6].eq {
		sub_823D9BF0(ctx, base);
		return;
	}
	// 823D9BE8: 39600012  li r11, 0x12
	ctx.r[11].s64 = 18;
	// 823D9BEC: 48000060  b 0x823d9c4c
	sub_823D9C24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9BF0 size=16
    let mut pc: u32 = 0x823D9BF0;
    'dispatch: loop {
        match pc {
            0x823D9BF0 => {
    //   block [0x823D9BF0..0x823D9C00)
	// 823D9BF0: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 823D9BF4: 409A000C  bne cr6, 0x823d9c00
	if !ctx.cr[6].eq {
		sub_823D9C00(ctx, base);
		return;
	}
	// 823D9BF8: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 823D9BFC: 48000050  b 0x823d9c4c
	sub_823D9C24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9C00 size=16
    let mut pc: u32 = 0x823D9C00;
    'dispatch: loop {
        match pc {
            0x823D9C00 => {
    //   block [0x823D9C00..0x823D9C10)
	// 823D9C00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823D9C04: 409A000C  bne cr6, 0x823d9c10
	if !ctx.cr[6].eq {
		sub_823D9C10(ctx, base);
		return;
	}
	// 823D9C08: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 823D9C0C: 48000040  b 0x823d9c4c
	sub_823D9C24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9C10 size=20
    let mut pc: u32 = 0x823D9C10;
    'dispatch: loop {
        match pc {
            0x823D9C10 => {
    //   block [0x823D9C10..0x823D9C24)
	// 823D9C10: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9C14: 554A056B  rlwinm. r10, r10, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823D9C18: 41820034  beq 0x823d9c4c
	if ctx.cr[0].eq {
		sub_823D9C24(ctx, base);
		return;
	}
	// 823D9C1C: 39600013  li r11, 0x13
	ctx.r[11].s64 = 19;
	// 823D9C20: 4800002C  b 0x823d9c4c
	sub_823D9C24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823D9C24 size=48
    let mut pc: u32 = 0x823D9C24;
    'dispatch: loop {
        match pc {
            0x823D9C24 => {
    //   block [0x823D9C24..0x823D9C54)
	// 823D9C24: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 823D9C28: 409A0024  bne cr6, 0x823d9c4c
	if !ctx.cr[6].eq {
	pc = 0x823D9C4C; continue 'dispatch;
	}
	// 823D9C2C: 554A0043  rlwinm. r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823D9C30: 4182001C  beq 0x823d9c4c
	if ctx.cr[0].eq {
	pc = 0x823D9C4C; continue 'dispatch;
	}
	// 823D9C34: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D9C38: 814A0030  lwz r10, 0x30(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D9C3C: 554A056C  rlwinm r10, r10, 0, 0x15, 0x16
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9C40: 2B0A0400  cmplwi cr6, r10, 0x400
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1024 as u32, &mut ctx.xer);
	// 823D9C44: 409A0008  bne cr6, 0x823d9c4c
	if !ctx.cr[6].eq {
	pc = 0x823D9C4C; continue 'dispatch;
	}
	// 823D9C48: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 823D9C4C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823D9C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9C58 size=484
    let mut pc: u32 = 0x823D9C58;
    'dispatch: loop {
        match pc {
            0x823D9C58 => {
    //   block [0x823D9C58..0x823D9E3C)
	// 823D9C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9C5C: 4815B461  bl 0x825350bc
	ctx.lr = 0x823D9C60;
	sub_82535080(ctx, base);
	// 823D9C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D9C64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823D9C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823D9C6C: 814A05B0  lwz r10, 0x5b0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823D9C70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9C74: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9C78: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823D9C7C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 823D9C80: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 823D9C84: 419901A4  bgt cr6, 0x823d9e28
	if ctx.cr[6].gt {
	pc = 0x823D9E28; continue 'dispatch;
	}
	// 823D9C88: 3D808200  lis r12, -0x7e00
	ctx.r[12].s64 = -2113929216;
	// 823D9C8C: 398C1760  addi r12, r12, 0x1760
	ctx.r[12].s64 = ctx.r[12].s64 + 5984;
	// 823D9C90: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823D9C94: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 823D9C98: 3D80823E  lis r12, -0x7dc2
	ctx.r[12].s64 = -2109865984;
	// 823D9C9C: 398C9CB0  addi r12, r12, -0x6350
	ctx.r[12].s64 = ctx.r[12].s64 + -25424;
	// 823D9CA0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 823D9CA4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 823D9CA8: 60000000  nop
	// 823D9CAC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 823D9CB0: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823D9CB4: 40820174  bne 0x823d9e28
	if !ctx.cr[0].eq {
	pc = 0x823D9E28; continue 'dispatch;
	}
	// 823D9CB8: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9CBC: 4182001C  beq 0x823d9cd8
	if ctx.cr[0].eq {
	pc = 0x823D9CD8; continue 'dispatch;
	}
	// 823D9CC0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 823D9CC4: 39201400  li r9, 0x1400
	ctx.r[9].s64 = 5120;
	// 823D9CC8: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 823D9CCC: 7C8B4B96  divwu r4, r11, r9
	ctx.r[4].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 823D9CD0: 5543053E  clrlwi r3, r10, 0x14
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000FFFu64;
	// 823D9CD4: 48005DBD  bl 0x823dfa90
	ctx.lr = 0x823D9CD8;
	sub_823DFA90(ctx, base);
	// 823D9CD8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9CDC: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9CE0: 41820148  beq 0x823d9e28
	if ctx.cr[0].eq {
	pc = 0x823D9E28; continue 'dispatch;
	}
	// 823D9CE4: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 823D9CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823D9CEC: 916AE16C  stw r11, -0x1e94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7828 as u32), ctx.r[11].u32 ) };
	// 823D9CF0: 48000138  b 0x823d9e28
	pc = 0x823D9E28; continue 'dispatch;
	// 823D9CF4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9CF8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9CFC: 41820018  beq 0x823d9d14
	if ctx.cr[0].eq {
	pc = 0x823D9D14; continue 'dispatch;
	}
	// 823D9D00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9D04: 419A0010  beq cr6, 0x823d9d14
	if ctx.cr[6].eq {
	pc = 0x823D9D14; continue 'dispatch;
	}
	// 823D9D08: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823D9D0C: 38A0000D  li r5, 0xd
	ctx.r[5].s64 = 13;
	// 823D9D10: 48001381  bl 0x823db090
	ctx.lr = 0x823D9D14;
	sub_823DB090(ctx, base);
	// 823D9D14: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9D18: 3FA0B180  lis r29, -0x4e80
	ctx.r[29].s64 = -1317011456;
	// 823D9D1C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D9D20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823D9D24: 55630026  rlwinm r3, r11, 0, 0, 0x13
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9D28: 555E0026  rlwinm r30, r10, 0, 0, 0x13
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9D2C: 4BFF052D  bl 0x823ca258
	ctx.lr = 0x823D9D30;
	sub_823CA258(ctx, base);
	// 823D9D30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823D9D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823D9D38: 4BFF0521  bl 0x823ca258
	ctx.lr = 0x823D9D3C;
	sub_823CA258(ctx, base);
	// 823D9D3C: 480000EC  b 0x823d9e28
	pc = 0x823D9E28; continue 'dispatch;
	// 823D9D40: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9D44: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9D48: 41820018  beq 0x823d9d60
	if ctx.cr[0].eq {
	pc = 0x823D9D60; continue 'dispatch;
	}
	// 823D9D4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9D50: 419A0010  beq cr6, 0x823d9d60
	if ctx.cr[6].eq {
	pc = 0x823D9D60; continue 'dispatch;
	}
	// 823D9D54: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823D9D58: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 823D9D5C: 48001335  bl 0x823db090
	ctx.lr = 0x823D9D60;
	sub_823DB090(ctx, base);
	// 823D9D60: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D9D64: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 823D9D68: 5563003A  rlwinm r3, r11, 0, 0, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9D6C: 4BFFFFCC  b 0x823d9d38
	pc = 0x823D9D38; continue 'dispatch;
	// 823D9D70: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9D74: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9D78: 41820018  beq 0x823d9d90
	if ctx.cr[0].eq {
	pc = 0x823D9D90; continue 'dispatch;
	}
	// 823D9D7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9D80: 419A0010  beq cr6, 0x823d9d90
	if ctx.cr[6].eq {
	pc = 0x823D9D90; continue 'dispatch;
	}
	// 823D9D84: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 823D9D88: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823D9D8C: 48001305  bl 0x823db090
	ctx.lr = 0x823D9D90;
	sub_823DB090(ctx, base);
	// 823D9D90: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D9D94: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 823D9D98: 4BFFFFA0  b 0x823d9d38
	pc = 0x823D9D38; continue 'dispatch;
	// 823D9D9C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9DA0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9DA4: 41820018  beq 0x823d9dbc
	if ctx.cr[0].eq {
	pc = 0x823D9DBC; continue 'dispatch;
	}
	// 823D9DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9DAC: 419A0010  beq cr6, 0x823d9dbc
	if ctx.cr[6].eq {
	pc = 0x823D9DBC; continue 'dispatch;
	}
	// 823D9DB0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823D9DB4: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 823D9DB8: 480012D9  bl 0x823db090
	ctx.lr = 0x823D9DBC;
	sub_823DB090(ctx, base);
	// 823D9DBC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 823D9DC0: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 823D9DC4: 4BFFFF74  b 0x823d9d38
	pc = 0x823D9D38; continue 'dispatch;
	// 823D9DC8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9DCC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9DD0: 4182FFC0  beq 0x823d9d90
	if ctx.cr[0].eq {
	pc = 0x823D9D90; continue 'dispatch;
	}
	// 823D9DD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9DD8: 419AFFB8  beq cr6, 0x823d9d90
	if ctx.cr[6].eq {
	pc = 0x823D9D90; continue 'dispatch;
	}
	// 823D9DDC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 823D9DE0: 4BFFFFA8  b 0x823d9d88
	pc = 0x823D9D88; continue 'dispatch;
	// 823D9DE4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9DE8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9DEC: 4182FFA4  beq 0x823d9d90
	if ctx.cr[0].eq {
	pc = 0x823D9D90; continue 'dispatch;
	}
	// 823D9DF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9DF4: 419AFF9C  beq cr6, 0x823d9d90
	if ctx.cr[6].eq {
	pc = 0x823D9D90; continue 'dispatch;
	}
	// 823D9DF8: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 823D9DFC: 4BFFFF8C  b 0x823d9d88
	pc = 0x823D9D88; continue 'dispatch;
	// 823D9E00: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9E04: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9E08: 41820018  beq 0x823d9e20
	if ctx.cr[0].eq {
	pc = 0x823D9E20; continue 'dispatch;
	}
	// 823D9E0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823D9E10: 419A0010  beq cr6, 0x823d9e20
	if ctx.cr[6].eq {
	pc = 0x823D9E20; continue 'dispatch;
	}
	// 823D9E14: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823D9E18: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 823D9E1C: 48001275  bl 0x823db090
	ctx.lr = 0x823D9E20;
	sub_823DB090(ctx, base);
	// 823D9E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D9E24: 48002395  bl 0x823dc1b8
	ctx.lr = 0x823D9E28;
	sub_823DC1B8(ctx, base);
	// 823D9E28: 3C802480  lis r4, 0x2480
	ctx.r[4].s64 = 612368384;
	// 823D9E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D9E30: 4BFF0429  bl 0x823ca258
	ctx.lr = 0x823D9E34;
	sub_823CA258(ctx, base);
	// 823D9E34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823D9E38: 4815B2D4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823D9E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823D9E40 size=564
    let mut pc: u32 = 0x823D9E40;
    'dispatch: loop {
        match pc {
            0x823D9E40 => {
    //   block [0x823D9E40..0x823DA074)
	// 823D9E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823D9E44: 4815B25D  bl 0x825350a0
	ctx.lr = 0x823D9E48;
	sub_82535080(ctx, base);
	// 823D9E48: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823D9E4C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 823D9E50: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 823D9E54: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 823D9E58: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 823D9E5C: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 823D9E60: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9E64: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 823D9E68: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 823D9E6C: 5569073E  clrlwi r9, r11, 0x1c
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823D9E70: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 823D9E74: 2B090004  cmplwi cr6, r9, 4
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4 as u32, &mut ctx.xer);
	// 823D9E78: 409A001C  bne cr6, 0x823d9e94
	if !ctx.cr[6].eq {
	pc = 0x823D9E94; continue 'dispatch;
	}
	// 823D9E7C: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9E80: 41820014  beq 0x823d9e94
	if ctx.cr[0].eq {
	pc = 0x823D9E94; continue 'dispatch;
	}
	// 823D9E84: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 823D9E88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9E8C: 41820008  beq 0x823d9e94
	if ctx.cr[0].eq {
	pc = 0x823D9E94; continue 'dispatch;
	}
	// 823D9E90: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 823D9E94: 732B1010  andi. r11, r25, 0x1010
	ctx.r[11].u64 = ctx.r[25].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9E98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9E9C: 4182000C  beq 0x823d9ea8
	if ctx.cr[0].eq {
	pc = 0x823D9EA8; continue 'dispatch;
	}
	// 823D9EA0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 823D9EA4: 48000008  b 0x823d9eac
	pc = 0x823D9EAC; continue 'dispatch;
	// 823D9EA8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 823D9EAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823D9EB0: 3FE08200  lis r31, -0x7e00
	ctx.r[31].s64 = -2113929216;
	// 823D9EB4: 419A001C  beq cr6, 0x823d9ed0
	if ctx.cr[6].eq {
	pc = 0x823D9ED0; continue 'dispatch;
	}
	// 823D9EB8: 815F05B0  lwz r10, 0x5b0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823D9EBC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 823D9EC0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 823D9EC4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 823D9EC8: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9ECC: 480011C5  bl 0x823db090
	ctx.lr = 0x823D9ED0;
	sub_823DB090(ctx, base);
	// 823D9ED0: 732B0012  andi. r11, r25, 0x12
	ctx.r[11].u64 = ctx.r[25].u64 & 18;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9ED4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823D9ED8: 408200D0  bne 0x823d9fa8
	if !ctx.cr[0].eq {
	pc = 0x823D9FA8; continue 'dispatch;
	}
	// 823D9EDC: 817F05B0  lwz r11, 0x5b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823D9EE0: 5789653E  srwi r9, r28, 0x14
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823D9EE4: 578A00FE  clrlwi r10, r28, 3
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x1FFFFFFFu64;
	// 823D9EE8: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9EEC: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823D9EF0: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9EF4: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823D9EF8: 7FBED214  add r29, r30, r26
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[26].u64;
	// 823D9EFC: 4816C145  bl 0x82546040
	ctx.lr = 0x823D9F00;
	sub_82546040(ctx, base);
	// 823D9F00: 817F2A88  lwz r11, 0x2a88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10888 as u32) ) } as u64;
	// 823D9F04: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 823D9F08: 419A0018  beq cr6, 0x823d9f20
	if ctx.cr[6].eq {
	pc = 0x823D9F20; continue 'dispatch;
	}
	// 823D9F0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823D9F10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823D9F14: 387F2E30  addi r3, r31, 0x2e30
	ctx.r[3].s64 = ctx.r[31].s64 + 11824;
	// 823D9F18: 48000689  bl 0x823da5a0
	ctx.lr = 0x823D9F1C;
	sub_823DA5A0(ctx, base);
	// 823D9F1C: 4800008C  b 0x823d9fa8
	pc = 0x823D9FA8; continue 'dispatch;
	// 823D9F20: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823D9F24: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823D9F28: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823D9F2C: 40990010  ble cr6, 0x823d9f3c
	if !ctx.cr[6].gt {
	pc = 0x823D9F3C; continue 'dispatch;
	}
	// 823D9F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823D9F34: 4800179D  bl 0x823db6d0
	ctx.lr = 0x823D9F38;
	sub_823DB6D0(ctx, base);
	// 823D9F38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823D9F3C: 39200A31  li r9, 0xa31
	ctx.r[9].s64 = 2609;
	// 823D9F40: 81010104  lwz r8, 0x104(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 823D9F44: 3CE00001  lis r7, 1
	ctx.r[7].s64 = 65536;
	// 823D9F48: 38DD0FFF  addi r6, r29, 0xfff
	ctx.r[6].s64 = ctx.r[29].s64 + 4095;
	// 823D9F4C: 60E70A2F  ori r7, r7, 0xa2f
	ctx.r[7].u64 = ctx.r[7].u64 | 2607;
	// 823D9F50: 57CA0026  rlwinm r10, r30, 0, 0, 0x13
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9F54: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F58: 54C90026  rlwinm r9, r6, 0, 0, 0x13
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 823D9F5C: 3CC0C004  lis r6, -0x3ffc
	ctx.r[6].s64 = -1073479680;
	// 823D9F60: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 823D9F64: 60C63C00  ori r6, r6, 0x3c00
	ctx.r[6].u64 = ctx.r[6].u64 | 15360;
	// 823D9F68: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 823D9F6C: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F70: 38800A31  li r4, 0xa31
	ctx.r[4].s64 = 2609;
	// 823D9F74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823D9F78: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 823D9F7C: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 823D9F80: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F84: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F88: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F8C: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F90: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F94: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F98: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 823D9F9C: 97CB0004  stwu r30, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[11].u32 = ea;
	// 823D9FA0: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 823D9FA4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823D9FA8: 572606F7  rlwinm. r6, r25, 0, 0x1b, 0x1b
	ctx.r[6].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 823D9FAC: 40820010  bne 0x823d9fbc
	if !ctx.cr[0].eq {
	pc = 0x823D9FBC; continue 'dispatch;
	}
	// 823D9FB0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9FB4: 556B0295  rlwinm. r11, r11, 0, 0xa, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9FB8: 41820090  beq 0x823da048
	if ctx.cr[0].eq {
	pc = 0x823DA048; continue 'dispatch;
	}
	// 823D9FBC: 572B07FF  clrlwi. r11, r25, 0x1f
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823D9FC0: 40820068  bne 0x823da028
	if !ctx.cr[0].eq {
	pc = 0x823DA028; continue 'dispatch;
	}
	// 823D9FC4: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 823D9FC8: 419A0018  beq cr6, 0x823d9fe0
	if ctx.cr[6].eq {
	pc = 0x823D9FE0; continue 'dispatch;
	}
	// 823D9FCC: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 823D9FD0: 419A0010  beq cr6, 0x823d9fe0
	if ctx.cr[6].eq {
	pc = 0x823D9FE0; continue 'dispatch;
	}
	// 823D9FD4: 7D78E050  subf r11, r24, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[24].s64;
	// 823D9FD8: 391B0018  addi r8, r27, 0x18
	ctx.r[8].s64 = ctx.r[27].s64 + 24;
	// 823D9FDC: 4800000C  b 0x823d9fe8
	pc = 0x823D9FE8; continue 'dispatch;
	// 823D9FE0: 7D77E050  subf r11, r23, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[23].s64;
	// 823D9FE4: 391B0014  addi r8, r27, 0x14
	ctx.r[8].s64 = ctx.r[27].s64 + 20;
	// 823D9FE8: 7D2BD214  add r9, r11, r26
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 823D9FEC: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823D9FF0: 5567C9FE  srwi r7, r11, 7
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 823D9FF4: 3969007F  addi r11, r9, 0x7f
	ctx.r[11].s64 = ctx.r[9].s64 + 127;
	// 823D9FF8: 5549043E  clrlwi r9, r10, 0x10
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823D9FFC: 556BC9FE  srwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DA000: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA004: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823DA008: 41990008  bgt cr6, 0x823da010
	if ctx.cr[6].gt {
	pc = 0x823DA010; continue 'dispatch;
	}
	// 823DA00C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823DA010: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DA014: 40980008  bge cr6, 0x823da01c
	if !ctx.cr[6].lt {
	pc = 0x823DA01C; continue 'dispatch;
	}
	// 823DA018: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 823DA01C: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA020: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DA024: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DA028: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 823DA02C: 419A001C  beq cr6, 0x823da048
	if ctx.cr[6].eq {
	pc = 0x823DA048; continue 'dispatch;
	}
	// 823DA030: 578B653E  srwi r11, r28, 0x14
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DA034: 578A00FE  clrlwi r10, r28, 3
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA038: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DA03C: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA040: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DA044: 3F8BC000  addis r28, r11, -0x4000
	ctx.r[28].s64 = ctx.r[11].s64 + -1073741824;
	// 823DA048: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 823DA04C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 823DA050: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA054: 7D40D828  lwarx r10, 0, r27
	// lwarx
	let ea = ctx.r[27].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823DA058: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DA05C: 7D20D92D  stwcx. r9, 0, r27
	// stwcx.
	let addr = ctx.r[27].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA060: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA064: 4082FFE8  bne 0x823da04c
	if !ctx.cr[0].eq {
	pc = 0x823DA04C; continue 'dispatch;
	}
	// 823DA068: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823DA06C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 823DA070: 4815B080  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA078 size=268
    let mut pc: u32 = 0x823DA078;
    'dispatch: loop {
        match pc {
            0x823DA078 => {
    //   block [0x823DA078..0x823DA184)
	// 823DA078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA07C: 4815B03D  bl 0x825350b8
	ctx.lr = 0x823DA080;
	sub_82535080(ctx, base);
	// 823DA080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DA088: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 823DA08C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823DA090: 3940FF00  li r10, -0x100
	ctx.r[10].s64 = -256;
	// 823DA094: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 823DA098: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA09C: 7D20F828  lwarx r9, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 823DA0A0: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 823DA0A4: 7D00F92D  stwcx. r8, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA0A8: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA0AC: 4082FFE8  bne 0x823da094
	if !ctx.cr[0].eq {
	pc = 0x823DA094; continue 'dispatch;
	}
	// 823DA0B0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 823DA0B4: 554A052E  rlwinm r10, r10, 0, 0x14, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA0B8: 2B0A0100  cmplwi cr6, r10, 0x100
	ctx.cr[6].compare_u32(ctx.r[10].u32, 256 as u32, &mut ctx.xer);
	// 823DA0BC: 409A00BC  bne cr6, 0x823da178
	if !ctx.cr[6].eq {
	pc = 0x823DA178; continue 'dispatch;
	}
	// 823DA0C0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 823DA0C4: 3FC0FFFF  lis r30, -1
	ctx.r[30].s64 = -65536;
	// 823DA0C8: 3F804000  lis r28, 0x4000
	ctx.r[28].s64 = 1073741824;
	// 823DA0CC: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823DA0D0: 419A004C  beq cr6, 0x823da11c
	if ctx.cr[6].eq {
	pc = 0x823DA11C; continue 'dispatch;
	}
	// 823DA0D4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA0D8: 5548843E  srwi r8, r10, 0x10
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DA0DC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 823DA0E0: 5547043E  clrlwi r7, r10, 0x10
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823DA0E4: 55290295  rlwinm. r9, r9, 0, 0xa, 0xa
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823DA0E8: 4082001C  bne 0x823da104
	if !ctx.cr[0].eq {
	pc = 0x823DA104; continue 'dispatch;
	}
	// 823DA0EC: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA0F0: 556A00FE  clrlwi r10, r11, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA0F4: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823DA0F8: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA0FC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DA100: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 823DA104: 54E93830  slwi r9, r7, 7
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(7);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA108: 550A3830  slwi r10, r8, 7
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DA110: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823DA114: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823DA118: 48005B41  bl 0x823dfc58
	ctx.lr = 0x823DA11C;
	sub_823DFC58(ctx, base);
	// 823DA11C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823DA120: 419A0058  beq cr6, 0x823da178
	if ctx.cr[6].eq {
	pc = 0x823DA178; continue 'dispatch;
	}
	// 823DA124: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823DA128: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823DA12C: 419A004C  beq cr6, 0x823da178
	if ctx.cr[6].eq {
	pc = 0x823DA178; continue 'dispatch;
	}
	// 823DA130: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA134: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA138: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 823DA13C: 5568043E  clrlwi r8, r11, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DA140: 554A0295  rlwinm. r10, r10, 0, 0xa, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DA144: 4082001C  bne 0x823da160
	if !ctx.cr[0].eq {
	pc = 0x823DA160; continue 'dispatch;
	}
	// 823DA148: 57AB653E  srwi r11, r29, 0x14
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DA14C: 57AA00FE  clrlwi r10, r29, 3
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA150: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DA154: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA158: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DA15C: 7FBC5850  subf r29, r28, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 823DA160: 550A3830  slwi r10, r8, 7
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA164: 552B3830  slwi r11, r9, 7
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DA168: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DA16C: 7C8AEA14  add r4, r10, r29
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 823DA170: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 823DA174: 48005AE5  bl 0x823dfc58
	ctx.lr = 0x823DA178;
	sub_823DFC58(ctx, base);
	// 823DA178: 7C0004AC  sync
	// 823DA17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823DA180: 4815AF88  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA188 size=128
    let mut pc: u32 = 0x823DA188;
    'dispatch: loop {
        match pc {
            0x823DA188 => {
    //   block [0x823DA188..0x823DA208)
	// 823DA188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DA190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DA194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DA19C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 823DA1A0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823DA1A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA1A8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823DA1AC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 823DA1B0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA1B4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA1B8: 4082FFE8  bne 0x823da1a0
	if !ctx.cr[0].eq {
	pc = 0x823DA1A0; continue 'dispatch;
	}
	// 823DA1BC: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 823DA1C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823DA1C4: 409A0030  bne cr6, 0x823da1f4
	if !ctx.cr[6].eq {
	pc = 0x823DA1F4; continue 'dispatch;
	}
	// 823DA1C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA1CC: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DA1D0: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 823DA1D4: 409A0014  bne cr6, 0x823da1e8
	if !ctx.cr[6].eq {
	pc = 0x823DA1E8; continue 'dispatch;
	}
	// 823DA1D8: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DA1DC: 4182000C  beq 0x823da1e8
	if ctx.cr[0].eq {
	pc = 0x823DA1E8; continue 'dispatch;
	}
	// 823DA1E0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823DA1E4: 4BFFFFA5  bl 0x823da188
	ctx.lr = 0x823DA1E8;
	sub_823DA188(ctx, base);
	// 823DA1E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DA1EC: 4BFFFA6D  bl 0x823d9c58
	ctx.lr = 0x823DA1F0;
	sub_823D9C58(ctx, base);
	// 823DA1F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DA1F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823DA1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DA1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DA200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DA204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA208 size=228
    let mut pc: u32 = 0x823DA208;
    'dispatch: loop {
        match pc {
            0x823DA208 => {
    //   block [0x823DA208..0x823DA2EC)
	// 823DA208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA20C: 4815AEB1  bl 0x825350bc
	ctx.lr = 0x823DA210;
	sub_82535080(ctx, base);
	// 823DA210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA214: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DA218: 5489653E  srwi r9, r4, 0x14
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA21C: 548A00FE  clrlwi r10, r4, 3
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA220: 816B05B0  lwz r11, 0x5b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823DA224: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA228: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823DA22C: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA230: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DA234: 7FBE2A14  add r29, r30, r5
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[5].u64;
	// 823DA238: 4816BE09  bl 0x82546040
	ctx.lr = 0x823DA23C;
	sub_82546040(ctx, base);
	// 823DA23C: 817F2A88  lwz r11, 0x2a88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10888 as u32) ) } as u64;
	// 823DA240: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 823DA244: 419A0018  beq cr6, 0x823da25c
	if ctx.cr[6].eq {
	pc = 0x823DA25C; continue 'dispatch;
	}
	// 823DA248: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823DA24C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823DA250: 387F2E30  addi r3, r31, 0x2e30
	ctx.r[3].s64 = ctx.r[31].s64 + 11824;
	// 823DA254: 4800034D  bl 0x823da5a0
	ctx.lr = 0x823DA258;
	sub_823DA5A0(ctx, base);
	// 823DA258: 4800008C  b 0x823da2e4
	pc = 0x823DA2E4; continue 'dispatch;
	// 823DA25C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DA260: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DA264: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DA268: 40990010  ble cr6, 0x823da278
	if !ctx.cr[6].gt {
	pc = 0x823DA278; continue 'dispatch;
	}
	// 823DA26C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DA270: 48001461  bl 0x823db6d0
	ctx.lr = 0x823DA274;
	sub_823DB6D0(ctx, base);
	// 823DA274: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DA278: 39200A31  li r9, 0xa31
	ctx.r[9].s64 = 2609;
	// 823DA27C: 3D000300  lis r8, 0x300
	ctx.r[8].s64 = 50331648;
	// 823DA280: 3CE00001  lis r7, 1
	ctx.r[7].s64 = 65536;
	// 823DA284: 57CA0026  rlwinm r10, r30, 0, 0, 0x13
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA288: 60E70A2F  ori r7, r7, 0xa2f
	ctx.r[7].u64 = ctx.r[7].u64 | 2607;
	// 823DA28C: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DA290: 393D0FFF  addi r9, r29, 0xfff
	ctx.r[9].s64 = ctx.r[29].s64 + 4095;
	// 823DA294: 3CC0C004  lis r6, -0x3ffc
	ctx.r[6].s64 = -1073479680;
	// 823DA298: 55290026  rlwinm r9, r9, 0, 0, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA29C: 60C63C00  ori r6, r6, 0x3c00
	ctx.r[6].u64 = ctx.r[6].u64 | 15360;
	// 823DA2A0: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 823DA2A4: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2A8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 823DA2AC: 38800A31  li r4, 0xa31
	ctx.r[4].s64 = 2609;
	// 823DA2B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DA2B4: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 823DA2B8: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 823DA2BC: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2C0: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2C4: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2C8: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2CC: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2D0: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2D4: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2D8: 97CB0004  stwu r30, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2DC: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 823DA2E0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DA2E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DA2E8: 4815AE24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA2F0 size=52
    let mut pc: u32 = 0x823DA2F0;
    'dispatch: loop {
        match pc {
            0x823DA2F0 => {
    //   block [0x823DA2F0..0x823DA324)
	// 823DA2F0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DA2F4: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 823DA2F8: 81233A50  lwz r9, 0x3a50(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14928 as u32) ) } as u64;
	// 823DA2FC: 81033A4C  lwz r8, 0x3a4c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14924 as u32) ) } as u64;
	// 823DA300: 80E33A44  lwz r7, 0x3a44(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14916 as u32) ) } as u64;
	// 823DA304: 80C33A48  lwz r6, 0x3a48(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DA308: 91633460  stw r11, 0x3460(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13408 as u32), ctx.r[11].u32 ) };
	// 823DA30C: 91433464  stw r10, 0x3464(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13412 as u32), ctx.r[10].u32 ) };
	// 823DA310: 91233468  stw r9, 0x3468(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13416 as u32), ctx.r[9].u32 ) };
	// 823DA314: 9103346C  stw r8, 0x346c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13420 as u32), ctx.r[8].u32 ) };
	// 823DA318: 90E33470  stw r7, 0x3470(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13424 as u32), ctx.r[7].u32 ) };
	// 823DA31C: 90C33474  stw r6, 0x3474(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13428 as u32), ctx.r[6].u32 ) };
	// 823DA320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA328 size=60
    let mut pc: u32 = 0x823DA328;
    'dispatch: loop {
        match pc {
            0x823DA328 => {
    //   block [0x823DA328..0x823DA364)
	// 823DA328: 81433460  lwz r10, 0x3460(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13408 as u32) ) } as u64;
	// 823DA32C: 81633464  lwz r11, 0x3464(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13412 as u32) ) } as u64;
	// 823DA330: 81233468  lwz r9, 0x3468(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13416 as u32) ) } as u64;
	// 823DA334: 8103346C  lwz r8, 0x346c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13420 as u32) ) } as u64;
	// 823DA338: 80E33470  lwz r7, 0x3470(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13424 as u32) ) } as u64;
	// 823DA33C: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 823DA340: 394BFF60  addi r10, r11, -0xa0
	ctx.r[10].s64 = ctx.r[11].s64 + -160;
	// 823DA344: 80C33474  lwz r6, 0x3474(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13428 as u32) ) } as u64;
	// 823DA348: 91233A50  stw r9, 0x3a50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14928 as u32), ctx.r[9].u32 ) };
	// 823DA34C: 91033A4C  stw r8, 0x3a4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14924 as u32), ctx.r[8].u32 ) };
	// 823DA350: 90E33A44  stw r7, 0x3a44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14916 as u32), ctx.r[7].u32 ) };
	// 823DA354: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 823DA358: 90C33A48  stw r6, 0x3a48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14920 as u32), ctx.r[6].u32 ) };
	// 823DA35C: 91430038  stw r10, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 823DA360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA368 size=152
    let mut pc: u32 = 0x823DA368;
    'dispatch: loop {
        match pc {
            0x823DA368 => {
    //   block [0x823DA368..0x823DA400)
	// 823DA368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA36C: 4815AD51  bl 0x825350bc
	ctx.lr = 0x823DA370;
	sub_82535080(ctx, base);
	// 823DA370: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DA378: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823DA37C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823DA380: 817F2A90  lwz r11, 0x2a90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DA384: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DA388: 7D4BF050  subf r10, r11, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 823DA38C: 554A07BF  clrlwi. r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DA390: 41820068  beq 0x823da3f8
	if ctx.cr[0].eq {
	pc = 0x823DA3F8; continue 'dispatch;
	}
	// 823DA394: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 823DA398: 409A0010  bne cr6, 0x823da3a8
	if !ctx.cr[6].eq {
	pc = 0x823DA3A8; continue 'dispatch;
	}
	// 823DA39C: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA3A0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA3A4: 40990054  ble cr6, 0x823da3f8
	if !ctx.cr[6].gt {
	pc = 0x823DA3F8; continue 'dispatch;
	}
	// 823DA3A8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 823DA3AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DA3B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DA3B4: 48005945  bl 0x823dfcf8
	ctx.lr = 0x823DA3B8;
	sub_823DFCF8(ctx, base);
	// 823DA3B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DA3BC: 48005AD5  bl 0x823dfe90
	ctx.lr = 0x823DA3C0;
	sub_823DFE90(ctx, base);
	// 823DA3C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 823DA3C4: 4182002C  beq 0x823da3f0
	if ctx.cr[0].eq {
	pc = 0x823DA3F0; continue 'dispatch;
	}
	// 823DA3C8: 817F2A90  lwz r11, 0x2a90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DA3CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DA3D0: 7D4BF050  subf r10, r11, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 823DA3D4: 554A07BF  clrlwi. r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DA3D8: 41820018  beq 0x823da3f0
	if ctx.cr[0].eq {
	pc = 0x823DA3F0; continue 'dispatch;
	}
	// 823DA3DC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 823DA3E0: 409AFFD8  bne cr6, 0x823da3b8
	if !ctx.cr[6].eq {
	pc = 0x823DA3B8; continue 'dispatch;
	}
	// 823DA3E4: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA3E8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA3EC: 4199FFCC  bgt cr6, 0x823da3b8
	if ctx.cr[6].gt {
	pc = 0x823DA3B8; continue 'dispatch;
	}
	// 823DA3F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DA3F4: 48005935  bl 0x823dfd28
	ctx.lr = 0x823DA3F8;
	sub_823DFD28(ctx, base);
	// 823DA3F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823DA3FC: 4815AD10  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA400 size=172
    let mut pc: u32 = 0x823DA400;
    'dispatch: loop {
        match pc {
            0x823DA400 => {
    //   block [0x823DA400..0x823DA4AC)
	// 823DA400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA404: 4815ACB9  bl 0x825350bc
	ctx.lr = 0x823DA408;
	sub_82535080(ctx, base);
	// 823DA408: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA40C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823DA410: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823DA414: 7D5F2A14  add r10, r31, r5
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 823DA418: 817D3A34  lwz r11, 0x3a34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14900 as u32) ) } as u64;
	// 823DA41C: 813D2A90  lwz r9, 0x2a90(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DA420: 7D5E5838  and r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 823DA424: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823DA428: 8169003C  lwz r11, 0x3c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(60 as u32) ) } as u64;
	// 823DA42C: 40980018  bge cr6, 0x823da444
	if !ctx.cr[6].lt {
	pc = 0x823DA444; continue 'dispatch;
	}
	// 823DA430: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA434: 4098006C  bge cr6, 0x823da4a0
	if !ctx.cr[6].lt {
	pc = 0x823DA4A0; continue 'dispatch;
	}
	// 823DA438: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823DA43C: 40990010  ble cr6, 0x823da44c
	if !ctx.cr[6].gt {
	pc = 0x823DA44C; continue 'dispatch;
	}
	// 823DA440: 48000060  b 0x823da4a0
	pc = 0x823DA4A0; continue 'dispatch;
	// 823DA444: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA448: 4098FFF0  bge cr6, 0x823da438
	if !ctx.cr[6].lt {
	pc = 0x823DA438; continue 'dispatch;
	}
	// 823DA44C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823DA450: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823DA454: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DA458: 480058A1  bl 0x823dfcf8
	ctx.lr = 0x823DA45C;
	sub_823DFCF8(ctx, base);
	// 823DA45C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DA460: 48005A31  bl 0x823dfe90
	ctx.lr = 0x823DA464;
	sub_823DFE90(ctx, base);
	// 823DA464: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 823DA468: 41820030  beq 0x823da498
	if ctx.cr[0].eq {
	pc = 0x823DA498; continue 'dispatch;
	}
	// 823DA46C: 817D2A90  lwz r11, 0x2a90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DA470: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823DA474: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 823DA478: 40980010  bge cr6, 0x823da488
	if !ctx.cr[6].lt {
	pc = 0x823DA488; continue 'dispatch;
	}
	// 823DA47C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA480: 40980018  bge cr6, 0x823da498
	if !ctx.cr[6].lt {
	pc = 0x823DA498; continue 'dispatch;
	}
	// 823DA484: 4800000C  b 0x823da490
	pc = 0x823DA490; continue 'dispatch;
	// 823DA488: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA48C: 4198FFD0  blt cr6, 0x823da45c
	if ctx.cr[6].lt {
	pc = 0x823DA45C; continue 'dispatch;
	}
	// 823DA490: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823DA494: 4099FFC8  ble cr6, 0x823da45c
	if !ctx.cr[6].gt {
	pc = 0x823DA45C; continue 'dispatch;
	}
	// 823DA498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DA49C: 4800588D  bl 0x823dfd28
	ctx.lr = 0x823DA4A0;
	sub_823DFD28(ctx, base);
	// 823DA4A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DA4A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823DA4A8: 4815AC64  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA4B0 size=240
    let mut pc: u32 = 0x823DA4B0;
    'dispatch: loop {
        match pc {
            0x823DA4B0 => {
    //   block [0x823DA4B0..0x823DA5A0)
	// 823DA4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA4B4: 4815ABFD  bl 0x825350b0
	ctx.lr = 0x823DA4B8;
	sub_82535080(ctx, base);
	// 823DA4B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA4BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DA4C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823DA4C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823DA4C8: 807E541C  lwz r3, 0x541c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DA4CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA4D0: 41820018  beq 0x823da4e8
	if ctx.cr[0].eq {
	pc = 0x823DA4E8; continue 'dispatch;
	}
	// 823DA4D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA4D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DA4DC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 823DA4E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DA4E4: 4E800421  bctrl
	ctx.lr = 0x823DA4E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DA4E8: 817E54AC  lwz r11, 0x54ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21676 as u32) ) } as u64;
	// 823DA4EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA4F0: 4182001C  beq 0x823da50c
	if ctx.cr[0].eq {
	pc = 0x823DA50C; continue 'dispatch;
	}
	// 823DA4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DA4F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823DA4FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823DA500: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823DA504: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DA508: 4E800421  bctrl
	ctx.lr = 0x823DA50C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DA50C: 83FE2AC8  lwz r31, 0x2ac8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10952 as u32) ) } as u64;
	// 823DA510: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823DA514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DA518: 837E3A34  lwz r27, 0x3a34(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14900 as u32) ) } as u64;
	// 823DA51C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DA520: 835E3A30  lwz r26, 0x3a30(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14896 as u32) ) } as u64;
	// 823DA524: 4BFFFEDD  bl 0x823da400
	ctx.lr = 0x823DA528;
	sub_823DA400(ctx, base);
	// 823DA528: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823DA52C: 419A0028  beq cr6, 0x823da554
	if ctx.cr[6].eq {
	pc = 0x823DA554; continue 'dispatch;
	}
	// 823DA530: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823DA534: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA538: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA53C: 391F0001  addi r8, r31, 1
	ctx.r[8].s64 = ctx.r[31].s64 + 1;
	// 823DA540: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823DA544: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823DA548: 7D1FD838  and r31, r8, r27
	ctx.r[31].u64 = ctx.r[8].u64 & ctx.r[27].u64;
	// 823DA54C: 7D49D12E  stwx r10, r9, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[26].u32), ctx.r[10].u32) };
	// 823DA550: 4082FFE4  bne 0x823da534
	if !ctx.cr[0].eq {
	pc = 0x823DA534; continue 'dispatch;
	}
	// 823DA554: 807E541C  lwz r3, 0x541c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DA558: 93FE2AC8  stw r31, 0x2ac8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(10952 as u32), ctx.r[31].u32 ) };
	// 823DA55C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA560: 41820014  beq 0x823da574
	if ctx.cr[0].eq {
	pc = 0x823DA574; continue 'dispatch;
	}
	// 823DA564: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA568: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DA56C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DA570: 4E800421  bctrl
	ctx.lr = 0x823DA574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DA574: 817E54AC  lwz r11, 0x54ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(21676 as u32) ) } as u64;
	// 823DA578: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA57C: 4182001C  beq 0x823da598
	if ctx.cr[0].eq {
	pc = 0x823DA598; continue 'dispatch;
	}
	// 823DA580: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DA584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DA588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DA58C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 823DA590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DA594: 4E800421  bctrl
	ctx.lr = 0x823DA598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DA598: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823DA59C: 4815AB64  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA5A0 size=88
    let mut pc: u32 = 0x823DA5A0;
    'dispatch: loop {
        match pc {
            0x823DA5A0 => {
    //   block [0x823DA5A0..0x823DA5F8)
	// 823DA5A0: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 823DA5A4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 823DA5A8: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DA5AC: 40980008  bge cr6, 0x823da5b4
	if !ctx.cr[6].lt {
	pc = 0x823DA5B4; continue 'dispatch;
	}
	// 823DA5B0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 823DA5B4: 79690022  rldicl r9, r11, 0x20, 0x20
	ctx.r[9].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 823DA5B8: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA5BC: 7F054840  cmplw cr6, r5, r9
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823DA5C0: 40990008  ble cr6, 0x823da5c8
	if !ctx.cr[6].gt {
	pc = 0x823DA5C8; continue 'dispatch;
	}
	// 823DA5C4: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 823DA5C8: 792907C6  sldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64.wrapping_shl(32);
	ctx.r[9].u32 = ctx.r[9].u64 as u32;
	// 823DA5CC: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 823DA5D0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 823DA5D4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 823DA5D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA5DC: 7D0018A8  ldarx r8, 0, r3
	ctx.reserved.u64 = unsafe { *(base.add(ctx.r[3].u32) as usize) as *const u64 } as u64;
	ctx.r[8].u64 = ctx.reserved.u64.swap_bytes();
	// 823DA5E0: 7F285800  cmpd cr6, r8, r11
	ctx.cr[6].compare_i64(ctx.r[8].s64, ctx.r[11].s64, &mut ctx.xer);
	// 823DA5E4: 409A0014  bne cr6, 0x823da5f8
	if !ctx.cr[6].eq {
		sub_823DA5F8(ctx, base);
		return;
	}
	// 823DA5E8: 7D4019AD  stdcx. r10, 0, r3
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stdcx64(base as *mut u8, addr, ctx.reserved.u64 as u64, ctx.r[10].u64) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA5EC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA5F0: 4082FFE4  bne 0x823da5d4
	if !ctx.cr[0].eq {
	pc = 0x823DA5D4; continue 'dispatch;
	}
	// 823DA5F4: 4800000C  b 0x823da600
	sub_823DA5F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA5F8 size=24
    let mut pc: u32 = 0x823DA5F8;
    'dispatch: loop {
        match pc {
            0x823DA5F8 => {
    //   block [0x823DA5F8..0x823DA610)
	// 823DA5F8: 7D0019AD  stdcx. r8, 0, r3
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stdcx64(base as *mut u8, addr, ctx.reserved.u64 as u64, ctx.r[8].u64) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA5FC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA600: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 823DA604: 7F2A5840  cmpld cr6, r10, r11
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[11].u64, &mut ctx.xer);
	// 823DA608: 409AFF98  bne cr6, 0x823da5a0
	if !ctx.cr[6].eq {
		sub_823DA5A0(ctx, base);
		return;
	}
	// 823DA60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA610 size=48
    let mut pc: u32 = 0x823DA610;
    'dispatch: loop {
        match pc {
            0x823DA610 => {
    //   block [0x823DA610..0x823DA640)
	// 823DA610: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 823DA614: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 823DA618: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 823DA61C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 823DA620: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA624: 7D2018A8  ldarx r9, 0, r3
	ctx.reserved.u64 = unsafe { *(base.add(ctx.r[3].u32) as usize) as *const u64 } as u64;
	ctx.r[9].u64 = ctx.reserved.u64.swap_bytes();
	// 823DA628: 7F295800  cmpd cr6, r9, r11
	ctx.cr[6].compare_i64(ctx.r[9].s64, ctx.r[11].s64, &mut ctx.xer);
	// 823DA62C: 409A0014  bne cr6, 0x823da640
	if !ctx.cr[6].eq {
		sub_823DA640(ctx, base);
		return;
	}
	// 823DA630: 7D4019AD  stdcx. r10, 0, r3
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stdcx64(base as *mut u8, addr, ctx.reserved.u64 as u64, ctx.r[10].u64) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA634: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA638: 4082FFE4  bne 0x823da61c
	if !ctx.cr[0].eq {
	pc = 0x823DA61C; continue 'dispatch;
	}
	// 823DA63C: 4800000C  b 0x823da648
	sub_823DA640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA640 size=36
    let mut pc: u32 = 0x823DA640;
    'dispatch: loop {
        match pc {
            0x823DA640 => {
    //   block [0x823DA640..0x823DA664)
	// 823DA640: 7D2019AD  stdcx. r9, 0, r3
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stdcx64(base as *mut u8, addr, ctx.reserved.u64 as u64, ctx.r[9].u64) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823DA644: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823DA648: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 823DA64C: 7F2A5840  cmpld cr6, r10, r11
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[11].u64, &mut ctx.xer);
	// 823DA650: 409AFFC0  bne cr6, 0x823da610
	if !ctx.cr[6].eq {
		sub_823DA610(ctx, base);
		return;
	}
	// 823DA654: 796A0022  rldicl r10, r11, 0x20, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 823DA658: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DA65C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823DA660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA668 size=184
    let mut pc: u32 = 0x823DA668;
    'dispatch: loop {
        match pc {
            0x823DA668 => {
    //   block [0x823DA668..0x823DA720)
	// 823DA668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA66C: 4815AA51  bl 0x825350bc
	ctx.lr = 0x823DA670;
	sub_82535080(ctx, base);
	// 823DA670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA674: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823DA678: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 823DA67C: 409A007C  bne cr6, 0x823da6f8
	if !ctx.cr[6].eq {
	pc = 0x823DA6F8; continue 'dispatch;
	}
	// 823DA680: 817E2A94  lwz r11, 0x2a94(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DA684: 3D400BAD  lis r10, 0xbad
	ctx.r[10].s64 = 195887104;
	// 823DA688: 614AF00D  ori r10, r10, 0xf00d
	ctx.r[10].u64 = ctx.r[10].u64 | 61453;
	// 823DA68C: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 823DA690: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DA694: 409A0014  bne cr6, 0x823da6a8
	if !ctx.cr[6].eq {
	pc = 0x823DA6A8; continue 'dispatch;
	}
	// 823DA698: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DA69C: 386B1780  addi r3, r11, 0x1780
	ctx.r[3].s64 = ctx.r[11].s64 + 6016;
	// 823DA6A0: 480068A9  bl 0x823e0f48
	ctx.lr = 0x823DA6A4;
	sub_823E0F48(ctx, base);
	// 823DA6A4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823DA6A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823DA6AC: 419A0014  beq cr6, 0x823da6c0
	if ctx.cr[6].eq {
	pc = 0x823DA6C0; continue 'dispatch;
	}
	// 823DA6B0: 817E2A94  lwz r11, 0x2a94(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DA6B4: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 823DA6B8: 7FE903A6  mtctr r31
	ctx.ctr.u64 = ctx.r[31].u64;
	// 823DA6BC: 4E800421  bctrl
	ctx.lr = 0x823DA6C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DA6C0: 894D010C  lbz r10, 0x10c(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[13].u32.wrapping_add(268 as u32) ) } as u64;
	// 823DA6C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823DA6C8: 3BFE2A98  addi r31, r30, 0x2a98
	ctx.r[31].s64 = ctx.r[30].s64 + 10904;
	// 823DA6CC: 83DE2A94  lwz r30, 0x2a94(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DA6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DA6D4: 7D7D5030  slw r29, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 823DA6D8: 483331B5  bl 0x8270d88c
	ctx.lr = 0x823DA6DC;
	// extern call 0x8270D88C → crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql
	crate::xboxkrnl::KeAcquireSpinLockAtRaisedIrql(ctx, base);
	// 823DA6DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DA6E4: 7D6BE878  andc r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[29].u64;
	// 823DA6E8: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 823DA6EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DA6F0: 4833318D  bl 0x8270d87c
	ctx.lr = 0x823DA6F4;
	// extern call 0x8270D87C → crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql
	crate::xboxkrnl::KeReleaseSpinLockFromRaisedIrql(ctx, base);
	// 823DA6F4: 48000024  b 0x823da718
	pc = 0x823DA718; continue 'dispatch;
	// 823DA6F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823DA6FC: 409A001C  bne cr6, 0x823da718
	if !ctx.cr[6].eq {
	pc = 0x823DA718; continue 'dispatch;
	}
	// 823DA700: 3D607FC8  lis r11, 0x7fc8
	ctx.r[11].s64 = 2143813632;
	// 823DA704: 816B6544  lwz r11, 0x6544(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25924 as u32) ) } as u64;
	// 823DA708: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DA70C: 4182000C  beq 0x823da718
	if ctx.cr[0].eq {
	pc = 0x823DA718; continue 'dispatch;
	}
	// 823DA710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DA714: 48005B55  bl 0x823e0268
	ctx.lr = 0x823DA718;
	sub_823E0268(ctx, base);
	// 823DA718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DA71C: 4815A9F0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA720 size=68
    let mut pc: u32 = 0x823DA720;
    'dispatch: loop {
        match pc {
            0x823DA720 => {
    //   block [0x823DA720..0x823DA764)
	// 823DA720: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DA724: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA728: 41820030  beq 0x823da758
	if ctx.cr[0].eq {
	pc = 0x823DA758; continue 'dispatch;
	}
	// 823DA72C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DA730: 3CE0C000  lis r7, -0x4000
	ctx.r[7].s64 = -1073741824;
	// 823DA734: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA738: 556A00FE  clrlwi r10, r11, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA73C: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823DA740: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA744: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DA748: 3D6BC000  addis r11, r11, -0x4000
	ctx.r[11].s64 = ctx.r[11].s64 + -1073741824;
	// 823DA74C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DA750: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DA754: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 823DA758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823DA75C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 823DA760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA768 size=164
    let mut pc: u32 = 0x823DA768;
    'dispatch: loop {
        match pc {
            0x823DA768 => {
    //   block [0x823DA768..0x823DA80C)
	// 823DA768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA76C: 4815A951  bl 0x825350bc
	ctx.lr = 0x823DA770;
	sub_82535080(ctx, base);
	// 823DA770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA774: 54AA653E  srwi r10, r5, 0x14
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shr(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA778: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DA77C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 823DA780: 392A0200  addi r9, r10, 0x200
	ctx.r[9].s64 = ctx.r[10].s64 + 512;
	// 823DA784: 54A700FE  clrlwi r7, r5, 3
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA788: 552804E6  rlwinm r8, r9, 0, 0x13, 0x13
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA78C: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DA790: 3D404000  lis r10, 0x4000
	ctx.r[10].s64 = 1073741824;
	// 823DA794: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 823DA798: 39290200  addi r9, r9, 0x200
	ctx.r[9].s64 = ctx.r[9].s64 + 512;
	// 823DA79C: 7C8A4050  subf r4, r10, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 823DA7A0: 552904E6  rlwinm r9, r9, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA7A4: 556800FE  clrlwi r8, r11, 3
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA7A8: 38E5FFFC  addi r7, r5, -4
	ctx.r[7].s64 = ctx.r[5].s64 + -4;
	// 823DA7AC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 823DA7B0: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 823DA7B4: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 823DA7B8: 7FCA4850  subf r30, r10, r9
	ctx.r[30].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 823DA7BC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DA7C0: 57EA653E  srwi r10, r31, 0x14
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shr(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA7C4: 57E900FE  clrlwi r9, r31, 3
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x1FFFFFFFu64;
	// 823DA7C8: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 823DA7CC: 394A0200  addi r10, r10, 0x200
	ctx.r[10].s64 = ctx.r[10].s64 + 512;
	// 823DA7D0: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA7D4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 823DA7D8: 7FA85050  subf r29, r8, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823DA7DC: 7C0006AC  eieio
	// 823DA7E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DA7E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DA7E8: 48005471  bl 0x823dfc58
	ctx.lr = 0x823DA7EC;
	sub_823DFC58(ctx, base);
	// 823DA7EC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 823DA7F0: 7C0006AC  eieio
	// 823DA7F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DA7F8: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 823DA7FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823DA800: 48005459  bl 0x823dfc58
	ctx.lr = 0x823DA804;
	sub_823DFC58(ctx, base);
	// 823DA804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DA808: 4815A904  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DA810 size=40
    let mut pc: u32 = 0x823DA810;
    'dispatch: loop {
        match pc {
            0x823DA810 => {
    //   block [0x823DA810..0x823DA838)
	// 823DA810: 89432ABD  lbz r10, 0x2abd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DA814: 81634158  lwz r11, 0x4158(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16728 as u32) ) } as u64;
	// 823DA818: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 823DA81C: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DA820: 99432ABD  stb r10, 0x2abd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10941 as u32), ctx.r[10].u8 ) };
	// 823DA824: 394B12C0  addi r10, r11, 0x12c0
	ctx.r[10].s64 = ctx.r[11].s64 + 4800;
	// 823DA828: 396AFF60  addi r11, r10, -0xa0
	ctx.r[11].s64 = ctx.r[10].s64 + -160;
	// 823DA82C: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 823DA830: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 823DA834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DA838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DA838 size=516
    let mut pc: u32 = 0x823DA838;
    'dispatch: loop {
        match pc {
            0x823DA838 => {
    //   block [0x823DA838..0x823DAA3C)
	// 823DA838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DA83C: 4815A881  bl 0x825350bc
	ctx.lr = 0x823DA840;
	sub_82535080(ctx, base);
	// 823DA840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DA844: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823DA848: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DA84C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DA850: 5567F0BE  srwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 823DA854: 81633A44  lwz r11, 0x3a44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14916 as u32) ) } as u64;
	// 823DA858: 418200E0  beq 0x823da938
	if ctx.cr[0].eq {
	pc = 0x823DA938; continue 'dispatch;
	}
	// 823DA85C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA860: 4082000C  bne 0x823da86c
	if !ctx.cr[0].eq {
	pc = 0x823DA86C; continue 'dispatch;
	}
	// 823DA864: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 823DA868: 396B003C  addi r11, r11, 0x3c
	ctx.r[11].s64 = ctx.r[11].s64 + 60;
	// 823DA86C: 3926FFFF  addi r9, r6, -1
	ctx.r[9].s64 = ctx.r[6].s64 + -1;
	// 823DA870: 81033A3C  lwz r8, 0x3a3c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14908 as u32) ) } as u64;
	// 823DA874: 7D4B3214  add r10, r11, r6
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 823DA878: 80A33A48  lwz r5, 0x3a48(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DA87C: 7D2B48F8  nor r11, r9, r9
	ctx.r[11].u64 = !(ctx.r[9].u64 | ctx.r[9].u64);
	// 823DA880: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 823DA884: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA888: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 823DA88C: 7C8AFA14  add r4, r10, r31
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 823DA890: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 823DA894: 41990040  bgt cr6, 0x823da8d4
	if ctx.cr[6].gt {
	pc = 0x823DA8D4; continue 'dispatch;
	}
	// 823DA898: 814333B0  lwz r10, 0x33b0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DA89C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA8A0: 4182002C  beq 0x823da8cc
	if ctx.cr[0].eq {
	pc = 0x823DA8CC; continue 'dispatch;
	}
	// 823DA8A4: 816333B4  lwz r11, 0x33b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13236 as u32) ) } as u64;
	// 823DA8A8: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 823DA8AC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 823DA8B0: 419A0014  beq cr6, 0x823da8c4
	if ctx.cr[6].eq {
	pc = 0x823DA8C4; continue 'dispatch;
	}
	// 823DA8B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DA8B8: 409A0014  bne cr6, 0x823da8cc
	if !ctx.cr[6].eq {
	pc = 0x823DA8CC; continue 'dispatch;
	}
	// 823DA8BC: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 823DA8C0: 4098000C  bge cr6, 0x823da8cc
	if !ctx.cr[6].lt {
	pc = 0x823DA8CC; continue 'dispatch;
	}
	// 823DA8C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DA8C8: 4800016C  b 0x823daa34
	pc = 0x823DAA34; continue 'dispatch;
	// 823DA8CC: 90833A44  stw r4, 0x3a44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14916 as u32), ctx.r[4].u32 ) };
	// 823DA8D0: 4800005C  b 0x823da92c
	pc = 0x823DA92C; continue 'dispatch;
	// 823DA8D4: 81233A4C  lwz r9, 0x3a4c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14924 as u32) ) } as u64;
	// 823DA8D8: 810333B0  lwz r8, 0x33b0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DA8DC: 7D264A14  add r9, r6, r9
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 823DA8E0: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA8E4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 823DA8E8: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 823DA8EC: 7C8AFA14  add r4, r10, r31
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 823DA8F0: 41820028  beq 0x823da918
	if ctx.cr[0].eq {
	pc = 0x823DA918; continue 'dispatch;
	}
	// 823DA8F4: 816333B4  lwz r11, 0x33b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13236 as u32) ) } as u64;
	// 823DA8F8: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 823DA8FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823DA900: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 823DA904: 419AFFC0  beq cr6, 0x823da8c4
	if ctx.cr[6].eq {
	pc = 0x823DA8C4; continue 'dispatch;
	}
	// 823DA908: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DA90C: 409A000C  bne cr6, 0x823da918
	if !ctx.cr[6].eq {
	pc = 0x823DA918; continue 'dispatch;
	}
	// 823DA910: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 823DA914: 4198FFB0  blt cr6, 0x823da8c4
	if ctx.cr[6].lt {
	pc = 0x823DA8C4; continue 'dispatch;
	}
	// 823DA918: 81633A50  lwz r11, 0x3a50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14928 as u32) ) } as u64;
	// 823DA91C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA920: 4199FFA4  bgt cr6, 0x823da8c4
	if ctx.cr[6].gt {
	pc = 0x823DA8C4; continue 'dispatch;
	}
	// 823DA924: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 823DA928: 90833A4C  stw r4, 0x3a4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14924 as u32), ctx.r[4].u32 ) };
	// 823DA92C: 4BFFFA3D  bl 0x823da368
	ctx.lr = 0x823DA930;
	sub_823DA368(ctx, base);
	// 823DA930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DA934: 48000100  b 0x823daa34
	pc = 0x823DAA34; continue 'dispatch;
	// 823DA938: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA93C: 4082000C  bne 0x823da948
	if !ctx.cr[0].eq {
	pc = 0x823DA948; continue 'dispatch;
	}
	// 823DA940: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DA944: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823DA948: 396B001F  addi r11, r11, 0x1f
	ctx.r[11].s64 = ctx.r[11].s64 + 31;
	// 823DA94C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823DA950: 557E0034  rlwinm r30, r11, 0, 0, 0x1a
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA954: 409A0008  bne cr6, 0x823da95c
	if !ctx.cr[6].eq {
	pc = 0x823DA95C; continue 'dispatch;
	}
	// 823DA958: 38E00036  li r7, 0x36
	ctx.r[7].s64 = 54;
	// 823DA95C: 81633A40  lwz r11, 0x3a40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14912 as u32) ) } as u64;
	// 823DA960: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DA964: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA968: 40990008  ble cr6, 0x823da970
	if !ctx.cr[6].gt {
	pc = 0x823DA970; continue 'dispatch;
	}
	// 823DA96C: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 823DA970: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DA974: 81633A3C  lwz r11, 0x3a3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14908 as u32) ) } as u64;
	// 823DA978: 7FEAF214  add r31, r10, r30
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 823DA97C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DA980: 40990008  ble cr6, 0x823da988
	if !ctx.cr[6].gt {
	pc = 0x823DA988; continue 'dispatch;
	}
	// 823DA984: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823DA988: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 823DA98C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 823DA990: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 823DA994: 40980034  bge cr6, 0x823da9c8
	if !ctx.cr[6].lt {
	pc = 0x823DA9C8; continue 'dispatch;
	}
	// 823DA998: 81233A4C  lwz r9, 0x3a4c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14924 as u32) ) } as u64;
	// 823DA99C: 81633A48  lwz r11, 0x3a48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DA9A0: 3929001F  addi r9, r9, 0x1f
	ctx.r[9].s64 = ctx.r[9].s64 + 31;
	// 823DA9A4: 89032ABE  lbz r8, 0x2abe(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10942 as u32) ) } as u64;
	// 823DA9A8: 80C33A38  lwz r6, 0x3a38(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14904 as u32) ) } as u64;
	// 823DA9AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823DA9B0: 553E0034  rlwinm r30, r9, 0, 0, 0x1a
	ctx.r[30].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DA9B4: 61090004  ori r9, r8, 4
	ctx.r[9].u64 = ctx.r[8].u64 | 4;
	// 823DA9B8: 7FEAF214  add r31, r10, r30
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 823DA9BC: 91633A48  stw r11, 0x3a48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14920 as u32), ctx.r[11].u32 ) };
	// 823DA9C0: 90C33A4C  stw r6, 0x3a4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(14924 as u32), ctx.r[6].u32 ) };
	// 823DA9C4: 99232ABE  stb r9, 0x2abe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10942 as u32), ctx.r[9].u8 ) };
	// 823DA9C8: 810333B0  lwz r8, 0x33b0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DA9CC: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DA9D0: 41820044  beq 0x823daa14
	if ctx.cr[0].eq {
	pc = 0x823DAA14; continue 'dispatch;
	}
	// 823DA9D4: 816333B4  lwz r11, 0x33b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13236 as u32) ) } as u64;
	// 823DA9D8: 81433A48  lwz r10, 0x3a48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DA9DC: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823DA9E0: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 823DA9E4: 419A0014  beq cr6, 0x823da9f8
	if ctx.cr[6].eq {
	pc = 0x823DA9F8; continue 'dispatch;
	}
	// 823DA9E8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823DA9EC: 409A0028  bne cr6, 0x823daa14
	if !ctx.cr[6].eq {
	pc = 0x823DAA14; continue 'dispatch;
	}
	// 823DA9F0: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 823DA9F4: 40980020  bge cr6, 0x823daa14
	if !ctx.cr[6].lt {
	pc = 0x823DAA14; continue 'dispatch;
	}
	// 823DA9F8: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 823DA9FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DAA00: 409AFEC4  bne cr6, 0x823da8c4
	if !ctx.cr[6].eq {
	pc = 0x823DA8C4; continue 'dispatch;
	}
	// 823DAA04: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 823DAA08: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 823DAA0C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 823DAA10: 4198FEB4  blt cr6, 0x823da8c4
	if ctx.cr[6].lt {
	pc = 0x823DA8C4; continue 'dispatch;
	}
	// 823DAA14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DAA18: 80A33A48  lwz r5, 0x3a48(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DAA1C: 4BFFF94D  bl 0x823da368
	ctx.lr = 0x823DAA20;
	sub_823DA368(ctx, base);
	// 823DAA20: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 823DAA24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DAA28: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 823DAA2C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DAA30: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DAA34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DAA38: 4815A6D4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DAA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DAA40 size=232
    let mut pc: u32 = 0x823DAA40;
    'dispatch: loop {
        match pc {
            0x823DAA40 => {
    //   block [0x823DAA40..0x823DAB28)
	// 823DAA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DAA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DAA48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DAA4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DAA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DAA54: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 823DAA58: 897F2ABD  lbz r11, 0x2abd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DAA5C: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DAA60: 4182000C  beq 0x823daa6c
	if ctx.cr[0].eq {
	pc = 0x823DAA6C; continue 'dispatch;
	}
	// 823DAA64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DAA68: 480000AC  b 0x823dab14
	pc = 0x823DAB14; continue 'dispatch;
	// 823DAA6C: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DAA70: 807F34CC  lwz r3, 0x34cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13516 as u32) ) } as u64;
	// 823DAA74: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823DAA78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DAA7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823DAA80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823DAA84: 40820010  bne 0x823daa94
	if !ctx.cr[0].eq {
	pc = 0x823DAA94; continue 'dispatch;
	}
	// 823DAA88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DAA8C: 4BFFFDAD  bl 0x823da838
	ctx.lr = 0x823DAA90;
	sub_823DA838(ctx, base);
	// 823DAA90: 48000020  b 0x823daab0
	pc = 0x823DAAB0; continue 'dispatch;
	// 823DAA94: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DAA98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DAA9C: 419A0010  beq cr6, 0x823daaac
	if ctx.cr[6].eq {
	pc = 0x823DAAAC; continue 'dispatch;
	}
	// 823DAAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DAAA4: 480013C5  bl 0x823dbe68
	ctx.lr = 0x823DAAA8;
	sub_823DBE68(ctx, base);
	// 823DAAA8: 48000008  b 0x823daab0
	pc = 0x823DAAB0; continue 'dispatch;
	// 823DAAAC: 4800162D  bl 0x823dc0d8
	ctx.lr = 0x823DAAB0;
	sub_823DC0D8(ctx, base);
	// 823DAAB0: 817F3A54  lwz r11, 0x3a54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14932 as u32) ) } as u64;
	// 823DAAB4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823DAAB8: 813F3A58  lwz r9, 0x3a58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14936 as u32) ) } as u64;
	// 823DAABC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DAAC0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823DAAC4: 917F3A54  stw r11, 0x3a54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14932 as u32), ctx.r[11].u32 ) };
	// 823DAAC8: 40990038  ble cr6, 0x823dab00
	if !ctx.cr[6].gt {
	pc = 0x823DAB00; continue 'dispatch;
	}
	// 823DAACC: 817F33B0  lwz r11, 0x33b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DAAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DAAD4: 409A002C  bne cr6, 0x823dab00
	if !ctx.cr[6].eq {
	pc = 0x823DAB00; continue 'dispatch;
	}
	// 823DAAD8: 815F3A4C  lwz r10, 0x3a4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14924 as u32) ) } as u64;
	// 823DAADC: 813F3A38  lwz r9, 0x3a38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14904 as u32) ) } as u64;
	// 823DAAE0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DAAE4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823DAAE8: 394B00A0  addi r10, r11, 0xa0
	ctx.r[10].s64 = ctx.r[11].s64 + 160;
	// 823DAAEC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 823DAAF0: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 823DAAF4: 419A000C  beq cr6, 0x823dab00
	if ctx.cr[6].eq {
	pc = 0x823DAB00; continue 'dispatch;
	}
	// 823DAAF8: 817F3A3C  lwz r11, 0x3a3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14908 as u32) ) } as u64;
	// 823DAAFC: 917F3A44  stw r11, 0x3a44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14916 as u32), ctx.r[11].u32 ) };
	// 823DAB00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823DAB04: 409A0010  bne cr6, 0x823dab14
	if !ctx.cr[6].eq {
	pc = 0x823DAB14; continue 'dispatch;
	}
	// 823DAB08: 897F2ABD  lbz r11, 0x2abd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DAB0C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 823DAB10: 997F2ABD  stb r11, 0x2abd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10941 as u32), ctx.r[11].u8 ) };
	// 823DAB14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DAB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DAB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DAB20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DAB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DAB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DAB28 size=484
    let mut pc: u32 = 0x823DAB28;
    'dispatch: loop {
        match pc {
            0x823DAB28 => {
    //   block [0x823DAB28..0x823DAD0C)
	// 823DAB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DAB2C: 4815A571  bl 0x8253509c
	ctx.lr = 0x823DAB30;
	sub_82535080(ctx, base);
	// 823DAB30: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DAB34: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 823DAB38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823DAB3C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 823DAB40: 897A2ABD  lbz r11, 0x2abd(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DAB44: 833A3A34  lwz r25, 0x3a34(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(14900 as u32) ) } as u64;
	// 823DAB48: 831A3A30  lwz r24, 0x3a30(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(14896 as u32) ) } as u64;
	// 823DAB4C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DAB50: 4182007C  beq 0x823dabcc
	if ctx.cr[0].eq {
	pc = 0x823DABCC; continue 'dispatch;
	}
	// 823DAB54: 817A541C  lwz r11, 0x541c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DAB58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DAB5C: 419A01A8  beq cr6, 0x823dad04
	if ctx.cr[6].eq {
	pc = 0x823DAD04; continue 'dispatch;
	}
	// 823DAB60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823DAB64: 419A01A0  beq cr6, 0x823dad04
	if ctx.cr[6].eq {
	pc = 0x823DAD04; continue 'dispatch;
	}
	// 823DAB68: 3EA02000  lis r21, 0x2000
	ctx.r[21].s64 = 536870912;
	// 823DAB6C: 3EC04000  lis r22, 0x4000
	ctx.r[22].s64 = 1073741824;
	// 823DAB70: 3EE04100  lis r23, 0x4100
	ctx.r[23].s64 = 1090519040;
	// 823DAB74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DAB78: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DAB7C: 7F0BA840  cmplw cr6, r11, r21
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[21].u32, &mut ctx.xer);
	// 823DAB80: 5545023E  clrlwi r5, r10, 8
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 823DAB84: 7C965850  subf r4, r22, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[22].s64;
	// 823DAB88: 41980008  blt cr6, 0x823dab90
	if ctx.cr[6].lt {
	pc = 0x823DAB90; continue 'dispatch;
	}
	// 823DAB8C: 7C975850  subf r4, r23, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[23].s64;
	// 823DAB90: 807A541C  lwz r3, 0x541c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DAB94: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823DAB98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DAB9C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 823DABA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DABA4: 4E800421  bctrl
	ctx.lr = 0x823DABA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DABA8: 807A541C  lwz r3, 0x541c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DABAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DABB0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DABB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DABB8: 4E800421  bctrl
	ctx.lr = 0x823DABBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DABBC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823DABC0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 823DABC4: 4082FFB0  bne 0x823dab74
	if !ctx.cr[0].eq {
	pc = 0x823DAB74; continue 'dispatch;
	}
	// 823DABC8: 4800013C  b 0x823dad04
	pc = 0x823DAD04; continue 'dispatch;
	// 823DABCC: 83BA2AC8  lwz r29, 0x2ac8(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(10952 as u32) ) } as u64;
	// 823DABD0: 1CBF0003  mulli r5, r31, 3
	ctx.r[5].s64 = ctx.r[31].s64 * 3;
	// 823DABD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823DABD8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 823DABDC: 4BFFF825  bl 0x823da400
	ctx.lr = 0x823DABE0;
	sub_823DA400(ctx, base);
	// 823DABE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823DABE4: 419A00C8  beq cr6, 0x823dacac
	if ctx.cr[6].eq {
	pc = 0x823DACAC; continue 'dispatch;
	}
	// 823DABE8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 823DABEC: 3EA02000  lis r21, 0x2000
	ctx.r[21].s64 = 536870912;
	// 823DABF0: 3EC04000  lis r22, 0x4000
	ctx.r[22].s64 = 1073741824;
	// 823DABF4: 3EE04100  lis r23, 0x4100
	ctx.r[23].s64 = 1090519040;
	// 823DABF8: 807A541C  lwz r3, 0x541c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DABFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DAC00: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DAC04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DAC08: 557C023E  clrlwi r28, r11, 8
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 823DAC0C: 4182002C  beq 0x823dac38
	if ctx.cr[0].eq {
	pc = 0x823DAC38; continue 'dispatch;
	}
	// 823DAC10: 7F1FA840  cmplw cr6, r31, r21
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[21].u32, &mut ctx.xer);
	// 823DAC14: 7C96F850  subf r4, r22, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[22].s64;
	// 823DAC18: 41980008  blt cr6, 0x823dac20
	if ctx.cr[6].lt {
	pc = 0x823DAC20; continue 'dispatch;
	}
	// 823DAC1C: 7C97F850  subf r4, r23, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[23].s64;
	// 823DAC20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DAC24: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823DAC28: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823DAC2C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 823DAC30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DAC34: 4E800421  bctrl
	ctx.lr = 0x823DAC38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DAC38: 817A54AC  lwz r11, 0x54ac(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21676 as u32) ) } as u64;
	// 823DAC3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DAC40: 41820028  beq 0x823dac68
	if ctx.cr[0].eq {
	pc = 0x823DAC68; continue 'dispatch;
	}
	// 823DAC44: 7F1FA840  cmplw cr6, r31, r21
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[21].u32, &mut ctx.xer);
	// 823DAC48: 7C96F850  subf r4, r22, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[22].s64;
	// 823DAC4C: 41980008  blt cr6, 0x823dac54
	if ctx.cr[6].lt {
	pc = 0x823DAC54; continue 'dispatch;
	}
	// 823DAC50: 7C97F850  subf r4, r23, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[23].s64;
	// 823DAC54: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823DAC58: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823DAC5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823DAC60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DAC64: 4E800421  bctrl
	ctx.lr = 0x823DAC68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DAC68: 3D20C001  lis r9, -0x3fff
	ctx.r[9].s64 = -1073676288;
	// 823DAC6C: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DAC70: 61293F00  ori r9, r9, 0x3f00
	ctx.r[9].u64 = ctx.r[9].u64 | 16128;
	// 823DAC74: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 823DAC78: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 823DAC7C: 7D6BC838  and r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[25].u64;
	// 823DAC80: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 823DAC84: 7D2AC12E  stwx r9, r10, r24
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32), ctx.r[9].u32) };
	// 823DAC88: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 823DAC8C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DAC90: 7D4BC838  and r11, r10, r25
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[25].u64;
	// 823DAC94: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DAC98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823DAC9C: 7FE9C12E  stwx r31, r9, r24
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[24].u32), ctx.r[31].u32) };
	// 823DACA0: 7D7DC838  and r29, r11, r25
	ctx.r[29].u64 = ctx.r[11].u64 & ctx.r[25].u64;
	// 823DACA4: 7F8AC12E  stwx r28, r10, r24
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32), ctx.r[28].u32) };
	// 823DACA8: 4082FF50  bne 0x823dabf8
	if !ctx.cr[0].eq {
	pc = 0x823DABF8; continue 'dispatch;
	}
	// 823DACAC: 93BA2AC8  stw r29, 0x2ac8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(10952 as u32), ctx.r[29].u32 ) };
	// 823DACB0: 7C0004AC  sync
	// 823DACB4: 3D607FC8  lis r11, 0x7fc8
	ctx.r[11].s64 = 2143813632;
	// 823DACB8: 93AB0714  stw r29, 0x714(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1812 as u32), ctx.r[29].u32 ) };
	// 823DACBC: 7C0006AC  eieio
	// 823DACC0: 7C0004AC  sync
	// 823DACC4: 807A541C  lwz r3, 0x541c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DACC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DACCC: 41820014  beq 0x823dace0
	if ctx.cr[0].eq {
	pc = 0x823DACE0; continue 'dispatch;
	}
	// 823DACD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DACD4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DACD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DACDC: 4E800421  bctrl
	ctx.lr = 0x823DACE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DACE0: 817A54AC  lwz r11, 0x54ac(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(21676 as u32) ) } as u64;
	// 823DACE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DACE8: 4182001C  beq 0x823dad04
	if ctx.cr[0].eq {
	pc = 0x823DAD04; continue 'dispatch;
	}
	// 823DACEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DACF0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823DACF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DACF8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 823DACFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DAD00: 4E800421  bctrl
	ctx.lr = 0x823DAD04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DAD04: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 823DAD08: 4815A3E4  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DAD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DAD10 size=448
    let mut pc: u32 = 0x823DAD10;
    'dispatch: loop {
        match pc {
            0x823DAD10 => {
    //   block [0x823DAD10..0x823DAED0)
	// 823DAD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DAD14: 4815A385  bl 0x82535098
	ctx.lr = 0x823DAD18;
	sub_82535080(ctx, base);
	// 823DAD18: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DAD1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823DAD20: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 823DAD24: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 823DAD28: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 823DAD2C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 823DAD30: E97D2E30  ld r11, 0x2e30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(11824 as u32) ) };
	// 823DAD34: 3B7D2E30  addi r27, r29, 0x2e30
	ctx.r[27].s64 = ctx.r[29].s64 + 11824;
	// 823DAD38: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 823DAD3C: 7F2B5040  cmpld cr6, r11, r10
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[10].u64, &mut ctx.xer);
	// 823DAD40: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 823DAD44: 409A0008  bne cr6, 0x823dad4c
	if !ctx.cr[6].eq {
	pc = 0x823DAD4C; continue 'dispatch;
	}
	// 823DAD48: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 823DAD4C: 897D2ABE  lbz r11, 0x2abe(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(10942 as u32) ) } as u64;
	// 823DAD50: 5578F7FE  rlwinm r24, r11, 0x1e, 0x1f, 0x1f
	ctx.r[24].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823DAD54: 7D78E214  add r11, r24, r28
	ctx.r[11].u64 = ctx.r[24].u64 + ctx.r[28].u64;
	// 823DAD58: 1EEB000B  mulli r23, r11, 0xb
	ctx.r[23].s64 = ctx.r[11].s64 * 11;
	// 823DAD5C: 28170000  cmplwi r23, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DAD60: 4082000C  bne 0x823dad6c
	if !ctx.cr[0].eq {
	pc = 0x823DAD6C; continue 'dispatch;
	}
	// 823DAD64: 93560000  stw r26, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 823DAD68: 48000160  b 0x823daec8
	pc = 0x823DAEC8; continue 'dispatch;
	// 823DAD6C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 823DAD70: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 823DAD74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823DAD78: 4BFFFCC9  bl 0x823daa40
	ctx.lr = 0x823DAD7C;
	sub_823DAA40(ctx, base);
	// 823DAD7C: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 823DAD80: 4182FFE4  beq 0x823dad64
	if ctx.cr[0].eq {
	pc = 0x823DAD64; continue 'dispatch;
	}
	// 823DAD84: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 823DAD88: 38F9FFFC  addi r7, r25, -4
	ctx.r[7].s64 = ctx.r[25].s64 + -4;
	// 823DAD8C: 617E0A2F  ori r30, r11, 0xa2f
	ctx.r[30].u64 = ctx.r[11].u64 | 2607;
	// 823DAD90: 3D60C004  lis r11, -0x3ffc
	ctx.r[11].s64 = -1073479680;
	// 823DAD94: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 823DAD98: 617F3C00  ori r31, r11, 0x3c00
	ctx.r[31].u64 = ctx.r[11].u64 | 15360;
	// 823DAD9C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 823DADA0: 419A0084  beq cr6, 0x823dae24
	if ctx.cr[6].eq {
	pc = 0x823DAE24; continue 'dispatch;
	}
	// 823DADA4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 823DADA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823DADAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 823DADB0: 4BFFF861  bl 0x823da610
	ctx.lr = 0x823DADB4;
	sub_823DA610(ctx, base);
	// 823DADB4: 39600A31  li r11, 0xa31
	ctx.r[11].s64 = 2609;
	// 823DADB8: 3D000300  lis r8, 0x300
	ctx.r[8].s64 = 50331648;
	// 823DADBC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823DADC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823DADC4: 554A0026  rlwinm r10, r10, 0, 0, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DADC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DADCC: 95670004  stwu r11, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[7].u32 = ea;
	// 823DADD0: 38600A31  li r3, 0xa31
	ctx.r[3].s64 = 2609;
	// 823DADD4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823DADD8: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 823DADDC: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 823DADE0: 392B0FFF  addi r9, r11, 0xfff
	ctx.r[9].s64 = ctx.r[11].s64 + 4095;
	// 823DADE4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 823DADE8: 55290026  rlwinm r9, r9, 0, 0, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DADEC: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 823DADF0: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 823DADF4: 3B600008  li r27, 8
	ctx.r[27].s64 = 8;
	// 823DADF8: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823DADFC: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE00: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE04: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE08: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE0C: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE10: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE14: 978B0004  stwu r28, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[28].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE18: 968B0004  stwu r20, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[20].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE1C: 976B0004  stwu r27, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[27].u32) };
	ctx.r[11].u32 = ea;
	// 823DAE20: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 823DAE24: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 823DAE28: 419A0084  beq cr6, 0x823daeac
	if ctx.cr[6].eq {
	pc = 0x823DAEAC; continue 'dispatch;
	}
	// 823DAE2C: 39200A31  li r9, 0xa31
	ctx.r[9].s64 = 2609;
	// 823DAE30: 817D3A38  lwz r11, 0x3a38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14904 as u32) ) } as u64;
	// 823DAE34: 815D3A3C  lwz r10, 0x3a3c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14908 as u32) ) } as u64;
	// 823DAE38: 3CA00100  lis r5, 0x100
	ctx.r[5].s64 = 16777216;
	// 823DAE3C: 5568653E  srwi r8, r11, 0x14
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DAE40: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 823DAE44: 38600A31  li r3, 0xa31
	ctx.r[3].s64 = 2609;
	// 823DAE48: 95270004  stwu r9, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE4C: 556900FE  clrlwi r9, r11, 3
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DAE50: 39680200  addi r11, r8, 0x200
	ctx.r[11].s64 = ctx.r[8].s64 + 512;
	// 823DAE54: 5548653E  srwi r8, r10, 0x14
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(20);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DAE58: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAE5C: 39080200  addi r8, r8, 0x200
	ctx.r[8].s64 = ctx.r[8].s64 + 512;
	// 823DAE60: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 823DAE64: 94A70004  stwu r5, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE68: 554A00FE  clrlwi r10, r10, 3
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 823DAE6C: 550904E6  rlwinm r9, r8, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAE70: 556B0026  rlwinm r11, r11, 0, 0, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAE74: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 823DAE78: 97C70004  stwu r30, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE7C: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 823DAE80: 394A0FFF  addi r10, r10, 0xfff
	ctx.r[10].s64 = ctx.r[10].s64 + 4095;
	// 823DAE84: 554A0026  rlwinm r10, r10, 0, 0, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAE88: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823DAE8C: 95470004  stwu r10, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE90: 95670004  stwu r11, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE94: 97E70004  stwu r31, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[31].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE98: 94870004  stwu r4, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[7].u32 = ea;
	// 823DAE9C: 94670004  stwu r3, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[7].u32 = ea;
	// 823DAEA0: 97470004  stwu r26, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[26].u32) };
	ctx.r[7].u32 = ea;
	// 823DAEA4: 94C70004  stwu r6, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[7].u32 = ea;
	// 823DAEA8: 97A70004  stwu r29, 4(r7)
	ea = ctx.r[7].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[7].u32 = ea;
	// 823DAEAC: 572B653E  srwi r11, r25, 0x14
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DAEB0: 572A00FE  clrlwi r10, r25, 3
	ctx.r[10].u64 = ctx.r[25].u32 as u64 & 0x1FFFFFFFu64;
	// 823DAEB4: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DAEB8: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAEBC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DAEC0: 91750000  stw r11, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DAEC4: 92F60000  stw r23, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 823DAEC8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823DAECC: 4815A21C  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DAED0 size=232
    let mut pc: u32 = 0x823DAED0;
    'dispatch: loop {
        match pc {
            0x823DAED0 => {
    //   block [0x823DAED0..0x823DAFB8)
	// 823DAED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DAED4: 4815A1DD  bl 0x825350b0
	ctx.lr = 0x823DAED8;
	sub_82535080(ctx, base);
	// 823DAED8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DAEDC: 3D20C000  lis r9, -0x4000
	ctx.r[9].s64 = -1073741824;
	// 823DAEE0: 3D00C002  lis r8, -0x3ffe
	ctx.r[8].s64 = -1073610752;
	// 823DAEE4: 613F3B00  ori r31, r9, 0x3b00
	ctx.r[31].u64 = ctx.r[9].u64 | 15104;
	// 823DAEE8: 3944FFFC  addi r10, r4, -4
	ctx.r[10].s64 = ctx.r[4].s64 + -4;
	// 823DAEEC: 812B2A90  lwz r9, 0x2a90(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DAEF0: 61085800  ori r8, r8, 0x5800
	ctx.r[8].u64 = ctx.r[8].u64 | 22528;
	// 823DAEF4: 80EB3A48  lwz r7, 0x3a48(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DAEF8: 38607FFF  li r3, 0x7fff
	ctx.r[3].s64 = 32767;
	// 823DAEFC: 80CB0030  lwz r6, 0x30(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DAF00: 5524653E  srwi r4, r9, 0x14
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shr(20);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 823DAF04: 54FA07BE  clrlwi r26, r7, 0x1e
	ctx.r[26].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823DAF08: 80AB2A9C  lwz r5, 0x2a9c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DAF0C: 38840200  addi r4, r4, 0x200
	ctx.r[4].s64 = ctx.r[4].s64 + 512;
	// 823DAF10: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 823DAF14: 90EB33BC  stw r7, 0x33bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(13244 as u32), ctx.r[7].u32 ) };
	// 823DAF18: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 823DAF1C: 90CB33B8  stw r6, 0x33b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(13240 as u32), ctx.r[6].u32 ) };
	// 823DAF20: 7F473378  or r7, r26, r6
	ctx.r[7].u64 = ctx.r[26].u64 | ctx.r[6].u64;
	// 823DAF24: 39090004  addi r8, r9, 4
	ctx.r[8].s64 = ctx.r[9].s64 + 4;
	// 823DAF28: 97EA0004  stwu r31, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[31].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF2C: 552600FE  clrlwi r6, r9, 3
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x1FFFFFFFu64;
	// 823DAF30: 548904E6  rlwinm r9, r4, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAF34: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 823DAF38: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 823DAF3C: 946A0004  stwu r3, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF40: 3B800003  li r28, 3
	ctx.r[28].s64 = 3;
	// 823DAF44: 61260002  ori r6, r9, 2
	ctx.r[6].u64 = ctx.r[9].u64 | 2;
	// 823DAF48: 5509653E  srwi r9, r8, 0x14
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DAF4C: 550800FE  clrlwi r8, r8, 3
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x1FFFFFFFu64;
	// 823DAF50: 39290200  addi r9, r9, 0x200
	ctx.r[9].s64 = ctx.r[9].s64 + 512;
	// 823DAF54: 97AA0004  stwu r29, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF58: 552904E6  rlwinm r9, r9, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DAF5C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 823DAF60: 97CA0004  stwu r30, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF64: 61290002  ori r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u64 | 2;
	// 823DAF68: 952A0004  stwu r9, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF6C: 94EA0004  stwu r7, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF70: 976A0004  stwu r27, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[27].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF74: 978A0004  stwu r28, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[28].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF78: 94CA0004  stwu r6, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF7C: 94AA0004  stwu r5, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[10].u32 = ea;
	// 823DAF80: 812B541C  lwz r9, 0x541c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21532 as u32) ) } as u64;
	// 823DAF84: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 823DAF88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823DAF8C: 409A0020  bne cr6, 0x823dafac
	if !ctx.cr[6].eq {
	pc = 0x823DAFAC; continue 'dispatch;
	}
	// 823DAF90: 894B2ABD  lbz r10, 0x2abd(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DAF94: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DAF98: 41820014  beq 0x823dafac
	if ctx.cr[0].eq {
	pc = 0x823DAFAC; continue 'dispatch;
	}
	// 823DAF9C: 814B2A90  lwz r10, 0x2a90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DAFA0: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 823DAFA4: 814B2A90  lwz r10, 0x2a90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DAFA8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 823DAFAC: 39450002  addi r10, r5, 2
	ctx.r[10].s64 = ctx.r[5].s64 + 2;
	// 823DAFB0: 914B2A9C  stw r10, 0x2a9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(10908 as u32), ctx.r[10].u32 ) };
	// 823DAFB4: 4815A14C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DAFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DAFB8 size=212
    let mut pc: u32 = 0x823DAFB8;
    'dispatch: loop {
        match pc {
            0x823DAFB8 => {
    //   block [0x823DAFB8..0x823DB08C)
	// 823DAFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DAFBC: 4815A0E9  bl 0x825350a4
	ctx.lr = 0x823DAFC0;
	sub_82535080(ctx, base);
	// 823DAFC0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DAFC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DAFC8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823DAFCC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 823DAFD0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 823DAFD4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 823DAFD8: 817E2B00  lwz r11, 0x2b00(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(11008 as u32) ) } as u64;
	// 823DAFDC: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 823DAFE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823DAFE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DAFE8: 419A006C  beq cr6, 0x823db054
	if ctx.cr[6].eq {
	pc = 0x823DB054; continue 'dispatch;
	}
	// 823DAFEC: 3B1E2B04  addi r24, r30, 0x2b04
	ctx.r[24].s64 = ctx.r[30].s64 + 11012;
	// 823DAFF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 823DAFF4: 48332689  bl 0x8270d67c
	ctx.lr = 0x823DAFF8;
	// extern call 0x8270D67C → crate::xboxkrnl::KfAcquireSpinLock
	crate::xboxkrnl::KfAcquireSpinLock(ctx, base);
	// 823DAFF8: 817E2B00  lwz r11, 0x2b00(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(11008 as u32) ) } as u64;
	// 823DAFFC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 823DB000: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB004: 419A003C  beq cr6, 0x823db040
	if ctx.cr[6].eq {
	pc = 0x823DB040; continue 'dispatch;
	}
	// 823DB008: 676B8100  oris r11, r27, 0x8100
	ctx.r[11].u64 = ctx.r[27].u64 | 2164260864;
	// 823DB00C: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 823DB010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DB014: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 823DB018: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DB01C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 823DB020: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 823DB024: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 823DB028: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823DB02C: 4BFFF73D  bl 0x823da768
	ctx.lr = 0x823DB030;
	sub_823DA768(ctx, base);
	// 823DB030: 817E2B00  lwz r11, 0x2b00(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(11008 as u32) ) } as u64;
	// 823DB034: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823DB038: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 823DB03C: 917E2B00  stw r11, 0x2b00(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(11008 as u32), ctx.r[11].u32 ) };
	// 823DB040: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 823DB044: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 823DB048: 48332625  bl 0x8270d66c
	ctx.lr = 0x823DB04C;
	// extern call 0x8270D66C → crate::xboxkrnl::KfReleaseSpinLock
	crate::xboxkrnl::KfReleaseSpinLock(ctx, base);
	// 823DB04C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823DB050: 409A0030  bne cr6, 0x823db080
	if !ctx.cr[6].eq {
	pc = 0x823DB080; continue 'dispatch;
	}
	// 823DB054: 817E2B00  lwz r11, 0x2b00(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(11008 as u32) ) } as u64;
	// 823DB058: 576A023E  clrlwi r10, r27, 8
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x00FFFFFFu64;
	// 823DB05C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823DB060: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 823DB064: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 823DB068: 654A8100  oris r10, r10, 0x8100
	ctx.r[10].u64 = ctx.r[10].u64 | 2164260864;
	// 823DB06C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823DB070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DB074: 917E2B00  stw r11, 0x2b00(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(11008 as u32), ctx.r[11].u32 ) };
	// 823DB078: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823DB07C: 4BFFFAAD  bl 0x823dab28
	ctx.lr = 0x823DB080;
	sub_823DAB28(ctx, base);
	// 823DB080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB084: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 823DB088: 4815A06C  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB090 size=196
    let mut pc: u32 = 0x823DB090;
    'dispatch: loop {
        match pc {
            0x823DB090 => {
    //   block [0x823DB090..0x823DB154)
	// 823DB090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB094: 4815A029  bl 0x825350bc
	ctx.lr = 0x823DB098;
	sub_82535080(ctx, base);
	// 823DB098: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB09C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823DB0A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB0A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823DB0A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823DB0AC: 419A00A0  beq cr6, 0x823db14c
	if ctx.cr[6].eq {
	pc = 0x823DB14C; continue 'dispatch;
	}
	// 823DB0B0: 815F2A90  lwz r10, 0x2a90(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DB0B4: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB0B8: 7D3E5850  subf r9, r30, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 823DB0BC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB0C0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823DB0C4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB0C8: 40980084  bge cr6, 0x823db14c
	if !ctx.cr[6].lt {
	pc = 0x823DB14C; continue 'dispatch;
	}
	// 823DB0CC: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB0D0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB0D4: 409A0014  bne cr6, 0x823db0e8
	if !ctx.cr[6].eq {
	pc = 0x823DB0E8; continue 'dispatch;
	}
	// 823DB0D8: 817F33B0  lwz r11, 0x33b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DB0DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DB0E0: 409A006C  bne cr6, 0x823db14c
	if !ctx.cr[6].eq {
	pc = 0x823DB14C; continue 'dispatch;
	}
	// 823DB0E4: 480005ED  bl 0x823db6d0
	ctx.lr = 0x823DB0E8;
	sub_823DB6D0(ctx, base);
	// 823DB0E8: 815F2A90  lwz r10, 0x2a90(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DB0EC: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB0F0: 7D3E5850  subf r9, r30, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 823DB0F4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB0F8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823DB0FC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB100: 4098004C  bge cr6, 0x823db14c
	if !ctx.cr[6].lt {
	pc = 0x823DB14C; continue 'dispatch;
	}
	// 823DB104: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823DB108: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DB10C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DB110: 48004BE9  bl 0x823dfcf8
	ctx.lr = 0x823DB114;
	sub_823DFCF8(ctx, base);
	// 823DB114: 48000014  b 0x823db128
	pc = 0x823DB128; continue 'dispatch;
	// 823DB118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DB11C: 48004D75  bl 0x823dfe90
	ctx.lr = 0x823DB120;
	sub_823DFE90(ctx, base);
	// 823DB120: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 823DB124: 41820020  beq 0x823db144
	if ctx.cr[0].eq {
	pc = 0x823DB144; continue 'dispatch;
	}
	// 823DB128: 815F2A90  lwz r10, 0x2a90(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DB12C: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB130: 7D3E5850  subf r9, r30, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 823DB134: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB138: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823DB13C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB140: 4198FFD8  blt cr6, 0x823db118
	if ctx.cr[6].lt {
	pc = 0x823DB118; continue 'dispatch;
	}
	// 823DB144: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DB148: 48004BE1  bl 0x823dfd28
	ctx.lr = 0x823DB14C;
	sub_823DFD28(ctx, base);
	// 823DB14C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823DB150: 48159FBC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB158 size=340
    let mut pc: u32 = 0x823DB158;
    'dispatch: loop {
        match pc {
            0x823DB158 => {
    //   block [0x823DB158..0x823DB2AC)
	// 823DB158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB15C: 48159F61  bl 0x825350bc
	ctx.lr = 0x823DB160;
	sub_82535080(ctx, base);
	// 823DB160: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DB164: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 823DB168: 54A8077D  rlwinm. r8, r5, 0, 0x1d, 0x1e
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 823DB16C: 54AA46BE  rlwinm r10, r5, 8, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 823DB170: 40820008  bne 0x823db178
	if !ctx.cr[0].eq {
	pc = 0x823DB178; continue 'dispatch;
	}
	// 823DB174: 60A50006  ori r5, r5, 6
	ctx.r[5].u64 = ctx.r[5].u64 | 6;
	// 823DB178: 3C80C004  lis r4, -0x3ffc
	ctx.r[4].s64 = -1073479680;
	// 823DB17C: 54A807BD  rlwinm. r8, r5, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 823DB180: 60833C00  ori r3, r4, 0x3c00
	ctx.r[3].u64 = ctx.r[4].u64 | 15360;
	// 823DB184: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 823DB188: 418200B0  beq 0x823db238
	if ctx.cr[0].eq {
	pc = 0x823DB238; continue 'dispatch;
	}
	// 823DB18C: 54A807FF  clrlwi. r8, r5, 0x1f
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 823DB190: 40820014  bne 0x823db1a4
	if !ctx.cr[0].eq {
	pc = 0x823DB1A4; continue 'dispatch;
	}
	// 823DB194: 390005C8  li r8, 0x5c8
	ctx.r[8].s64 = 1480;
	// 823DB198: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 823DB19C: 95090004  stwu r8, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1A0: 94890004  stwu r4, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1A4: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 823DB1A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DB1AC: 6108057C  ori r8, r8, 0x57c
	ctx.r[8].u64 = ctx.r[8].u64 | 1404;
	// 823DB1B0: 95090004  stwu r8, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1B4: 94C90004  stwu r6, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1B8: 94E90004  stwu r7, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1BC: 409A0008  bne cr6, 0x823db1c4
	if !ctx.cr[6].eq {
	pc = 0x823DB1C4; continue 'dispatch;
	}
	// 823DB1C0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 823DB1C4: 39000578  li r8, 0x578
	ctx.r[8].s64 = 1400;
	// 823DB1C8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 823DB1CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823DB1D0: 3BC00100  li r30, 0x100
	ctx.r[30].s64 = 256;
	// 823DB1D4: 3BA0045E  li r29, 0x45e
	ctx.r[29].s64 = 1118;
	// 823DB1D8: 95090004  stwu r8, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1DC: 95490004  stwu r10, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1E0: 810B31AC  lwz r8, 0x31ac(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12716 as u32) ) } as u64;
	// 823DB1E4: 7D081B78  or r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[3].u64;
	// 823DB1E8: 95090004  stwu r8, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1EC: 94E90004  stwu r7, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[9].u32 = ea;
	// 823DB1F0: 810B2A94  lwz r8, 0x2a94(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DB1F4: 5506653E  srwi r6, r8, 0x14
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(20);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 823DB1F8: 550700FE  clrlwi r7, r8, 3
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB1FC: 39060200  addi r8, r6, 0x200
	ctx.r[8].s64 = ctx.r[6].s64 + 512;
	// 823DB200: 550804E6  rlwinm r8, r8, 0, 0x13, 0x13
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB204: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 823DB208: 61080002  ori r8, r8, 2
	ctx.r[8].u64 = ctx.r[8].u64 | 2;
	// 823DB20C: 95090004  stwu r8, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[9].u32 = ea;
	// 823DB210: 95490004  stwu r10, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[9].u32 = ea;
	// 823DB214: 94890004  stwu r4, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[9].u32 = ea;
	// 823DB218: 97C90004  stwu r30, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[9].u32 = ea;
	// 823DB21C: 810B31AC  lwz r8, 0x31ac(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12716 as u32) ) } as u64;
	// 823DB220: 6508C000  oris r8, r8, 0xc000
	ctx.r[8].u64 = ctx.r[8].u64 | 3221225472;
	// 823DB224: 61085400  ori r8, r8, 0x5400
	ctx.r[8].u64 = ctx.r[8].u64 | 21504;
	// 823DB228: 95090004  stwu r8, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[9].u32 = ea;
	// 823DB22C: 95490004  stwu r10, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[9].u32 = ea;
	// 823DB230: 97A90004  stwu r29, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[9].u32 = ea;
	// 823DB234: 95490004  stwu r10, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[9].u32 = ea;
	// 823DB238: 54AA077B  rlwinm. r10, r5, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DB23C: 41820068  beq 0x823db2a4
	if ctx.cr[0].eq {
	pc = 0x823DB2A4; continue 'dispatch;
	}
	// 823DB240: 814B31AC  lwz r10, 0x31ac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12716 as u32) ) } as u64;
	// 823DB244: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 823DB248: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823DB24C: 7D4A1B78  or r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 823DB250: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 823DB254: 38A0057C  li r5, 0x57c
	ctx.r[5].s64 = 1404;
	// 823DB258: 3C800BAD  lis r4, 0xbad
	ctx.r[4].s64 = 195887104;
	// 823DB25C: 95490004  stwu r10, 4(r9)
	ea = ctx.r[9].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[9].u32 = ea;
	// 823DB260: 6084F00D  ori r4, r4, 0xf00d
	ctx.r[4].u64 = ctx.r[4].u64 | 61453;
	// 823DB264: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 823DB268: 950A0004  stwu r8, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[10].u32 = ea;
	// 823DB26C: 816B2A94  lwz r11, 0x2a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DB270: 5568653E  srwi r8, r11, 0x14
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DB274: 556900FE  clrlwi r9, r11, 3
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB278: 39680200  addi r11, r8, 0x200
	ctx.r[11].s64 = ctx.r[8].s64 + 512;
	// 823DB27C: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB280: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 823DB284: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DB288: 956A0004  stwu r11, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[10].u32 = ea;
	// 823DB28C: 94EA0004  stwu r7, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[10].u32 = ea;
	// 823DB290: 97EA0004  stwu r31, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[31].u32) };
	ctx.r[10].u32 = ea;
	// 823DB294: 94CA0004  stwu r6, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[10].u32 = ea;
	// 823DB298: 94AA0004  stwu r5, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[10].u32 = ea;
	// 823DB29C: 948A0004  stwu r4, 4(r10)
	ea = ctx.r[10].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[10].u32 = ea;
	// 823DB2A0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 823DB2A4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 823DB2A8: 48159E64  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DB2B0 size=28
    let mut pc: u32 = 0x823DB2B0;
    'dispatch: loop {
        match pc {
            0x823DB2B0 => {
    //   block [0x823DB2B0..0x823DB2CC)
	// 823DB2B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DB2B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823DB2B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DB2BC: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 823DB2C0: 816B05B0  lwz r11, 0x5b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823DB2C4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB2C8: 4BFFFDC8  b 0x823db090
	sub_823DB090(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB2D0 size=252
    let mut pc: u32 = 0x823DB2D0;
    'dispatch: loop {
        match pc {
            0x823DB2D0 => {
    //   block [0x823DB2D0..0x823DB3CC)
	// 823DB2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB2D4: 48159DE9  bl 0x825350bc
	ctx.lr = 0x823DB2D8;
	sub_82535080(ctx, base);
	// 823DB2D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB2DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DB2E0: 3FA04000  lis r29, 0x4000
	ctx.r[29].s64 = 1073741824;
	// 823DB2E4: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DB2E8: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB2EC: 41820024  beq 0x823db310
	if ctx.cr[0].eq {
	pc = 0x823DB310; continue 'dispatch;
	}
	// 823DB2F0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DB2F4: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DB2F8: 556A00FE  clrlwi r10, r11, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB2FC: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823DB300: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB304: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DB308: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 823DB30C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DB310: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 823DB314: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 823DB318: 38801080  li r4, 0x1080
	ctx.r[4].s64 = 4224;
	// 823DB31C: 4BFFF725  bl 0x823daa40
	ctx.lr = 0x823DB320;
	sub_823DAA40(ctx, base);
	// 823DB320: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823DB324: 40820018  bne 0x823db33c
	if !ctx.cr[0].eq {
	pc = 0x823DB33C; continue 'dispatch;
	}
	// 823DB328: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 823DB32C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DB330: 83EB4158  lwz r31, 0x4158(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16728 as u32) ) } as u64;
	// 823DB334: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823DB338: 48000078  b 0x823db3b0
	pc = 0x823DB3B0; continue 'dispatch;
	// 823DB33C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 823DB340: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DB344: 556A00FE  clrlwi r10, r11, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB348: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823DB34C: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB350: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DB354: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 823DB358: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DB35C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DB360: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB364: 41820024  beq 0x823db388
	if ctx.cr[0].eq {
	pc = 0x823DB388; continue 'dispatch;
	}
	// 823DB368: 57EA653E  srwi r10, r31, 0x14
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shr(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DB36C: 57E900FE  clrlwi r9, r31, 3
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB370: 394A0200  addi r10, r10, 0x200
	ctx.r[10].s64 = ctx.r[10].s64 + 512;
	// 823DB374: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB378: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 823DB37C: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 823DB380: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823DB384: 48000008  b 0x823db38c
	pc = 0x823DB38C; continue 'dispatch;
	// 823DB388: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823DB38C: 57EB653E  srwi r11, r31, 0x14
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DB390: 57EA00FE  clrlwi r10, r31, 3
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB394: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DB398: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DB39C: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB3A0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DB3A4: 7C7D5850  subf r3, r29, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 823DB3A8: 38831080  addi r4, r3, 0x1080
	ctx.r[4].s64 = ctx.r[3].s64 + 4224;
	// 823DB3AC: 480048AD  bl 0x823dfc58
	ctx.lr = 0x823DB3B0;
	sub_823DFC58(ctx, base);
	// 823DB3B0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 823DB3B4: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 823DB3B8: 397F107C  addi r11, r31, 0x107c
	ctx.r[11].s64 = ctx.r[31].s64 + 4220;
	// 823DB3BC: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 823DB3C0: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 823DB3C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DB3C8: 48159D44  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB3D0 size=344
    let mut pc: u32 = 0x823DB3D0;
    'dispatch: loop {
        match pc {
            0x823DB3D0 => {
    //   block [0x823DB3D0..0x823DB528)
	// 823DB3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB3D4: 48159CE9  bl 0x825350bc
	ctx.lr = 0x823DB3D8;
	sub_82535080(ctx, base);
	// 823DB3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB3E0: 895F2ABD  lbz r10, 0x2abd(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DB3E4: 554B06B5  rlwinm. r11, r10, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB3E8: 4182002C  beq 0x823db414
	if ctx.cr[0].eq {
	pc = 0x823DB414; continue 'dispatch;
	}
	// 823DB3EC: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 823DB3F0: 817F4158  lwz r11, 0x4158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16728 as u32) ) } as u64;
	// 823DB3F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DB3F8: 995F2ABD  stb r10, 0x2abd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10941 as u32), ctx.r[10].u8 ) };
	// 823DB3FC: 394B12C0  addi r10, r11, 0x12c0
	ctx.r[10].s64 = ctx.r[11].s64 + 4800;
	// 823DB400: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DB404: 396AFF60  addi r11, r10, -0xa0
	ctx.r[11].s64 = ctx.r[10].s64 + -160;
	// 823DB408: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 823DB40C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 823DB410: 48000110  b 0x823db520
	pc = 0x823DB520; continue 'dispatch;
	// 823DB414: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823DB418: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 823DB41C: 409A000C  bne cr6, 0x823db428
	if !ctx.cr[6].eq {
	pc = 0x823DB428; continue 'dispatch;
	}
	// 823DB420: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823DB424: 4800000C  b 0x823db430
	pc = 0x823DB430; continue 'dispatch;
	// 823DB428: 3964000E  addi r11, r4, 0xe
	ctx.r[11].s64 = ctx.r[4].s64 + 14;
	// 823DB42C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DB430: 807F34CC  lwz r3, 0x34cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13516 as u32) ) } as u64;
	// 823DB434: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB438: 41820020  beq 0x823db458
	if ctx.cr[0].eq {
	pc = 0x823DB458; continue 'dispatch;
	}
	// 823DB43C: 814300AC  lwz r10, 0xac(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 823DB440: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DB444: 419A0014  beq cr6, 0x823db458
	if ctx.cr[6].eq {
	pc = 0x823DB458; continue 'dispatch;
	}
	// 823DB448: 815F3A40  lwz r10, 0x3a40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14912 as u32) ) } as u64;
	// 823DB44C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DB450: 41990008  bgt cr6, 0x823db458
	if ctx.cr[6].gt {
	pc = 0x823DB458; continue 'dispatch;
	}
	// 823DB454: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 823DB458: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823DB45C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB460: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 823DB464: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823DB468: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 823DB46C: 40820010  bne 0x823db47c
	if !ctx.cr[0].eq {
	pc = 0x823DB47C; continue 'dispatch;
	}
	// 823DB470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB474: 4BFFF3C5  bl 0x823da838
	ctx.lr = 0x823DB478;
	sub_823DA838(ctx, base);
	// 823DB478: 48000020  b 0x823db498
	pc = 0x823DB498; continue 'dispatch;
	// 823DB47C: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DB480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DB484: 419A0010  beq cr6, 0x823db494
	if ctx.cr[6].eq {
	pc = 0x823DB494; continue 'dispatch;
	}
	// 823DB488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB48C: 480009DD  bl 0x823dbe68
	ctx.lr = 0x823DB490;
	sub_823DBE68(ctx, base);
	// 823DB490: 48000008  b 0x823db498
	pc = 0x823DB498; continue 'dispatch;
	// 823DB494: 48000C45  bl 0x823dc0d8
	ctx.lr = 0x823DB498;
	sub_823DC0D8(ctx, base);
	// 823DB498: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DB49C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823DB4A0: 409A001C  bne cr6, 0x823db4bc
	if !ctx.cr[6].eq {
	pc = 0x823DB4BC; continue 'dispatch;
	}
	// 823DB4A4: 895F2ABD  lbz r10, 0x2abd(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DB4A8: 817F4158  lwz r11, 0x4158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16728 as u32) ) } as u64;
	// 823DB4AC: 614A0020  ori r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 | 32;
	// 823DB4B0: 995F2ABD  stb r10, 0x2abd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10941 as u32), ctx.r[10].u8 ) };
	// 823DB4B4: 394B12C0  addi r10, r11, 0x12c0
	ctx.r[10].s64 = ctx.r[11].s64 + 4800;
	// 823DB4B8: 48000028  b 0x823db4e0
	pc = 0x823DB4E0; continue 'dispatch;
	// 823DB4BC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823DB4C0: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 823DB4C4: 93DF3A50  stw r30, 0x3a50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14928 as u32), ctx.r[30].u32 ) };
	// 823DB4C8: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DB4CC: 93BF3A44  stw r29, 0x3a44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14916 as u32), ctx.r[29].u32 ) };
	// 823DB4D0: 93BF3A54  stw r29, 0x3a54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14932 as u32), ctx.r[29].u32 ) };
	// 823DB4D4: 394AFFF2  addi r10, r10, -0xe
	ctx.r[10].s64 = ctx.r[10].s64 + -14;
	// 823DB4D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DB4DC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823DB4E0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DB4E4: 396AFF60  addi r11, r10, -0xa0
	ctx.r[11].s64 = ctx.r[10].s64 + -160;
	// 823DB4E8: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 823DB4EC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 823DB4F0: 817F571C  lwz r11, 0x571c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22300 as u32) ) } as u64;
	// 823DB4F4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB4F8: 40820024  bne 0x823db51c
	if !ctx.cr[0].eq {
	pc = 0x823DB51C; continue 'dispatch;
	}
	// 823DB4FC: 809F3A5C  lwz r4, 0x3a5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14940 as u32) ) } as u64;
	// 823DB500: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB504: 41820018  beq 0x823db51c
	if ctx.cr[0].eq {
	pc = 0x823DB51C; continue 'dispatch;
	}
	// 823DB508: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DB50C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 823DB510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB514: 4BFFFB7D  bl 0x823db090
	ctx.lr = 0x823DB518;
	sub_823DB090(ctx, base);
	// 823DB518: 93BF3A5C  stw r29, 0x3a5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14940 as u32), ctx.r[29].u32 ) };
	// 823DB51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DB520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823DB524: 48159BE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DB528 size=28
    let mut pc: u32 = 0x823DB528;
    'dispatch: loop {
        match pc {
            0x823DB528 => {
    //   block [0x823DB528..0x823DB544)
	// 823DB528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823DB52C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 823DB530: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DB534: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 823DB538: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 823DB53C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DB540: 4BFFFD90  b 0x823db2d0
	sub_823DB2D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB548 size=388
    let mut pc: u32 = 0x823DB548;
    'dispatch: loop {
        match pc {
            0x823DB548 => {
    //   block [0x823DB548..0x823DB6CC)
	// 823DB548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB54C: 48159B6D  bl 0x825350b8
	ctx.lr = 0x823DB550;
	sub_82535080(ctx, base);
	// 823DB550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB558: 895F2ABD  lbz r10, 0x2abd(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DB55C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB560: 83DF3A50  lwz r30, 0x3a50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14928 as u32) ) } as u64;
	// 823DB564: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 823DB568: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DB56C: 4082011C  bne 0x823db688
	if !ctx.cr[0].eq {
	pc = 0x823DB688; continue 'dispatch;
	}
	// 823DB570: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 823DB574: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB578: 4182006C  beq 0x823db5e4
	if ctx.cr[0].eq {
	pc = 0x823DB5E4; continue 'dispatch;
	}
	// 823DB57C: 817F34CC  lwz r11, 0x34cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13516 as u32) ) } as u64;
	// 823DB580: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB584: 41820104  beq 0x823db688
	if ctx.cr[0].eq {
	pc = 0x823DB688; continue 'dispatch;
	}
	// 823DB588: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DB58C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DB590: 409A00F8  bne cr6, 0x823db688
	if !ctx.cr[6].eq {
	pc = 0x823DB688; continue 'dispatch;
	}
	// 823DB594: 7D7EE050  subf r11, r30, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 823DB598: 7D7D1671  srawi. r29, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823DB59C: 418200EC  beq 0x823db688
	if ctx.cr[0].eq {
	pc = 0x823DB688; continue 'dispatch;
	}
	// 823DB5A0: 807F34E0  lwz r3, 0x34e0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13536 as u32) ) } as u64;
	// 823DB5A4: 817F34E4  lwz r11, 0x34e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13540 as u32) ) } as u64;
	// 823DB5A8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB5AC: 4198000C  blt cr6, 0x823db5b8
	if ctx.cr[6].lt {
	pc = 0x823DB5B8; continue 'dispatch;
	}
	// 823DB5B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB5B4: 48000A75  bl 0x823dc028
	ctx.lr = 0x823DB5B8;
	sub_823DC028(ctx, base);
	// 823DB5B8: 57CB653E  srwi r11, r30, 0x14
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DB5BC: 57CA00FE  clrlwi r10, r30, 3
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB5C0: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DB5C4: 67A98100  oris r9, r29, 0x8100
	ctx.r[9].u64 = ctx.r[29].u64 | 2164260864;
	// 823DB5C8: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB5CC: 39030008  addi r8, r3, 8
	ctx.r[8].s64 = ctx.r[3].s64 + 8;
	// 823DB5D0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DB5D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823DB5D8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DB5DC: 911F34E0  stw r8, 0x34e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13536 as u32), ctx.r[8].u32 ) };
	// 823DB5E0: 480000A8  b 0x823db688
	pc = 0x823DB688; continue 'dispatch;
	// 823DB5E4: 817F33B0  lwz r11, 0x33b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DB5E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DB5EC: 419A005C  beq cr6, 0x823db648
	if ctx.cr[6].eq {
	pc = 0x823DB648; continue 'dispatch;
	}
	// 823DB5F0: 57CB653E  srwi r11, r30, 0x14
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DB5F4: 7D5EE050  subf r10, r30, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 823DB5F8: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DB5FC: 7D5D1671  srawi. r29, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[10].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823DB600: 57CA00FE  clrlwi r10, r30, 3
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB604: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB608: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DB60C: 4182007C  beq 0x823db688
	if ctx.cr[0].eq {
	pc = 0x823DB688; continue 'dispatch;
	}
	// 823DB610: 387F3438  addi r3, r31, 0x3438
	ctx.r[3].s64 = ctx.r[31].s64 + 13368;
	// 823DB614: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DB618: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 823DB61C: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 823DB620: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DB624: 4099000C  ble cr6, 0x823db630
	if !ctx.cr[6].gt {
	pc = 0x823DB630; continue 'dispatch;
	}
	// 823DB628: 4BFFFCA9  bl 0x823db2d0
	ctx.lr = 0x823DB62C;
	sub_823DB2D0(ctx, base);
	// 823DB62C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DB630: 67AA8100  oris r10, r29, 0x8100
	ctx.r[10].u64 = ctx.r[29].u64 | 2164260864;
	// 823DB634: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 823DB638: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 823DB63C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823DB640: 913F3440  stw r9, 0x3440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(13376 as u32), ctx.r[9].u32 ) };
	// 823DB644: 48000044  b 0x823db688
	pc = 0x823DB688; continue 'dispatch;
	// 823DB648: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823DB64C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB650: 4BFFF881  bl 0x823daed0
	ctx.lr = 0x823DB654;
	sub_823DAED0(ctx, base);
	// 823DB654: 57CB653E  srwi r11, r30, 0x14
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DB658: 57CA00FE  clrlwi r10, r30, 3
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x1FFFFFFFu64;
	// 823DB65C: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DB660: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823DB664: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB668: 391F3438  addi r8, r31, 0x3438
	ctx.r[8].s64 = ctx.r[31].s64 + 13368;
	// 823DB66C: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DB670: 7D7E2050  subf r11, r30, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 823DB674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823DB678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB67C: 7D661670  srawi r6, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 823DB680: 4BFFF939  bl 0x823dafb8
	ctx.lr = 0x823DB684;
	sub_823DAFB8(ctx, base);
	// 823DB684: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823DB688: 397C001F  addi r11, r28, 0x1f
	ctx.r[11].s64 = ctx.r[28].s64 + 31;
	// 823DB68C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DB690: 55690034  rlwinm r9, r11, 0, 0, 0x1a
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB694: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DB698: 4099001C  ble cr6, 0x823db6b4
	if !ctx.cr[6].gt {
	pc = 0x823DB6B4; continue 'dispatch;
	}
	// 823DB69C: 397CFFFC  addi r11, r28, -4
	ctx.r[11].s64 = ctx.r[28].s64 + -4;
	// 823DB6A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DB6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB6A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DB6AC: 4BFFFD25  bl 0x823db3d0
	ctx.lr = 0x823DB6B0;
	sub_823DB3D0(ctx, base);
	// 823DB6B0: 48000014  b 0x823db6c4
	pc = 0x823DB6C4; continue 'dispatch;
	// 823DB6B4: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DB6B8: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 823DB6BC: 917F3A50  stw r11, 0x3a50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14928 as u32), ctx.r[11].u32 ) };
	// 823DB6C0: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 823DB6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823DB6C8: 48159A40  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB6D0 size=252
    let mut pc: u32 = 0x823DB6D0;
    'dispatch: loop {
        match pc {
            0x823DB6D0 => {
    //   block [0x823DB6D0..0x823DB7CC)
	// 823DB6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DB6D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823DB6DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DB6E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB6E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB6E8: 817F2A90  lwz r11, 0x2a90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DB6EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB6F0: 41820008  beq 0x823db6f8
	if ctx.cr[0].eq {
	pc = 0x823DB6F8; continue 'dispatch;
	}
	// 823DB6F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB6F8: 817F33B0  lwz r11, 0x33b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13232 as u32) ) } as u64;
	// 823DB6FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DB700: 409A005C  bne cr6, 0x823db75c
	if !ctx.cr[6].eq {
	pc = 0x823DB75C; continue 'dispatch;
	}
	// 823DB704: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 823DB708: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB70C: 40820050  bne 0x823db75c
	if !ctx.cr[0].eq {
	pc = 0x823DB75C; continue 'dispatch;
	}
	// 823DB710: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823DB714: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823DB718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB71C: 4BFFF5F5  bl 0x823dad10
	ctx.lr = 0x823DB720;
	sub_823DAD10(ctx, base);
	// 823DB720: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823DB724: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823DB728: 419A0034  beq cr6, 0x823db75c
	if ctx.cr[6].eq {
	pc = 0x823DB75C; continue 'dispatch;
	}
	// 823DB72C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 823DB730: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 823DB734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB738: 4BFFF309  bl 0x823daa40
	ctx.lr = 0x823DB73C;
	sub_823DAA40(ctx, base);
	// 823DB73C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 823DB740: 4182001C  beq 0x823db75c
	if ctx.cr[0].eq {
	pc = 0x823DB75C; continue 'dispatch;
	}
	// 823DB744: 391F3438  addi r8, r31, 0x3438
	ctx.r[8].s64 = ctx.r[31].s64 + 13368;
	// 823DB748: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823DB74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823DB750: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 823DB754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB758: 4BFFF861  bl 0x823dafb8
	ctx.lr = 0x823DB75C;
	sub_823DAFB8(ctx, base);
	// 823DB75C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB760: 4BFFFDE9  bl 0x823db548
	ctx.lr = 0x823DB764;
	sub_823DB548(ctx, base);
	// 823DB764: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 823DB768: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB76C: 40820044  bne 0x823db7b0
	if !ctx.cr[0].eq {
	pc = 0x823DB7B0; continue 'dispatch;
	}
	// 823DB770: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 823DB774: 816BE188  lwz r11, -0x1e78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7800 as u32) ) } as u64;
	// 823DB778: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB77C: 419A0034  beq cr6, 0x823db7b0
	if ctx.cr[6].eq {
	pc = 0x823DB7B0; continue 'dispatch;
	}
	// 823DB780: 897F2ABD  lbz r11, 0x2abd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DB784: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB788: 40820028  bne 0x823db7b0
	if !ctx.cr[0].eq {
	pc = 0x823DB7B0; continue 'dispatch;
	}
	// 823DB78C: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB790: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DB794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DB798: 388BFFFE  addi r4, r11, -2
	ctx.r[4].s64 = ctx.r[11].s64 + -2;
	// 823DB79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB7A0: 4BFFF8F1  bl 0x823db090
	ctx.lr = 0x823DB7A4;
	sub_823DB090(ctx, base);
	// 823DB7A4: 897F2ABD  lbz r11, 0x2abd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DB7A8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DB7AC: 997F2ABD  stb r11, 0x2abd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10941 as u32), ctx.r[11].u8 ) };
	// 823DB7B0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB7B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DB7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DB7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DB7C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823DB7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DB7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB7D0 size=52
    let mut pc: u32 = 0x823DB7D0;
    'dispatch: loop {
        match pc {
            0x823DB7D0 => {
    //   block [0x823DB7D0..0x823DB804)
	// 823DB7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DB7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DB7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB7E0: 83E32A9C  lwz r31, 0x2a9c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB7E4: 93E32AB0  stw r31, 0x2ab0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10928 as u32), ctx.r[31].u32 ) };
	// 823DB7E8: 4BFFFEE9  bl 0x823db6d0
	ctx.lr = 0x823DB7EC;
	sub_823DB6D0(ctx, base);
	// 823DB7EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823DB7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DB7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DB7FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DB800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823DB808 size=120
    let mut pc: u32 = 0x823DB808;
    'dispatch: loop {
        match pc {
            0x823DB808 => {
    //   block [0x823DB808..0x823DB880)
	// 823DB808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DB810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823DB814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DB818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB81C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DB820: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823DB824: 419A0044  beq cr6, 0x823db868
	if ctx.cr[6].eq {
	pc = 0x823DB868; continue 'dispatch;
	}
	// 823DB828: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DB82C: 816B05B0  lwz r11, 0x5b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823DB830: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB834: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB838: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB83C: 409A000C  bne cr6, 0x823db848
	if !ctx.cr[6].eq {
	pc = 0x823DB848; continue 'dispatch;
	}
	// 823DB840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB844: 4BFFFE8D  bl 0x823db6d0
	ctx.lr = 0x823DB848;
	sub_823DB6D0(ctx, base);
	// 823DB848: 815F2A90  lwz r10, 0x2a90(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DB84C: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB850: 7D3E5850  subf r9, r30, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 823DB854: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DB858: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823DB85C: 7D6B4810  subfc r11, r11, r9
	ctx.xer.ca = ctx.r[9].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 823DB860: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 823DB864: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 823DB868: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DB86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DB870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DB874: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823DB878: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DB87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB880 size=100
    let mut pc: u32 = 0x823DB880;
    'dispatch: loop {
        match pc {
            0x823DB880 => {
    //   block [0x823DB880..0x823DB8E4)
	// 823DB880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DB888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DB88C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB894: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB898: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DB89C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB8A0: 4099000C  ble cr6, 0x823db8ac
	if !ctx.cr[6].gt {
	pc = 0x823DB8AC; continue 'dispatch;
	}
	// 823DB8A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB8A8: 4BFFFE29  bl 0x823db6d0
	ctx.lr = 0x823DB8AC;
	sub_823DB6D0(ctx, base);
	// 823DB8AC: 396005C8  li r11, 0x5c8
	ctx.r[11].s64 = 1480;
	// 823DB8B0: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 823DB8B4: 39200D04  li r9, 0xd04
	ctx.r[9].s64 = 3332;
	// 823DB8B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823DB8BC: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 823DB8C0: 95430004  stwu r10, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[3].u32 = ea;
	// 823DB8C4: 95230004  stwu r9, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[3].u32 = ea;
	// 823DB8C8: 95030004  stwu r8, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[3].u32 = ea;
	// 823DB8CC: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 823DB8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823DB8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DB8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DB8DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DB8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB8E8 size=104
    let mut pc: u32 = 0x823DB8E8;
    'dispatch: loop {
        match pc {
            0x823DB8E8 => {
    //   block [0x823DB8E8..0x823DB950)
	// 823DB8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB8EC: 481597D1  bl 0x825350bc
	ctx.lr = 0x823DB8F0;
	sub_82535080(ctx, base);
	// 823DB8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB8F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB8F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823DB8FC: 57BE103A  slwi r30, r29, 2
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 823DB900: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB904: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 823DB908: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 823DB90C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DB910: 40990034  ble cr6, 0x823db944
	if !ctx.cr[6].gt {
	pc = 0x823DB944; continue 'dispatch;
	}
	// 823DB914: 4BFFFDBD  bl 0x823db6d0
	ctx.lr = 0x823DB918;
	sub_823DB6D0(ctx, base);
	// 823DB918: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB91C: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 823DB920: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 823DB924: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DB928: 4099001C  ble cr6, 0x823db944
	if !ctx.cr[6].gt {
	pc = 0x823DB944; continue 'dispatch;
	}
	// 823DB92C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823DB930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB934: 4BFFFA9D  bl 0x823db3d0
	ctx.lr = 0x823DB938;
	sub_823DB3D0(ctx, base);
	// 823DB938: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DB93C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DB940: 41820008  beq 0x823db948
	if ctx.cr[0].eq {
	pc = 0x823DB948; continue 'dispatch;
	}
	// 823DB944: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB948: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DB94C: 481597C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB950 size=116
    let mut pc: u32 = 0x823DB950;
    'dispatch: loop {
        match pc {
            0x823DB950 => {
    //   block [0x823DB950..0x823DB9C4)
	// 823DB950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DB958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DB95C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB964: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB968: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DB96C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DB970: 4099000C  ble cr6, 0x823db97c
	if !ctx.cr[6].gt {
	pc = 0x823DB97C; continue 'dispatch;
	}
	// 823DB974: 4BFFFD5D  bl 0x823db6d0
	ctx.lr = 0x823DB978;
	sub_823DB6D0(ctx, base);
	// 823DB978: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DB97C: 394005C8  li r10, 0x5c8
	ctx.r[10].s64 = 1480;
	// 823DB980: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 823DB984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823DB988: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 823DB98C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DB990: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DB994: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DB998: 809F2A9C  lwz r4, 0x2a9c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DB99C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DB9A0: 4BFFF6F1  bl 0x823db090
	ctx.lr = 0x823DB9A4;
	sub_823DB090(ctx, base);
	// 823DB9A4: 817F2B00  lwz r11, 0x2b00(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11008 as u32) ) } as u64;
	// 823DB9A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DB9AC: 409AFFF8  bne cr6, 0x823db9a4
	if !ctx.cr[6].eq {
	pc = 0x823DB9A4; continue 'dispatch;
	}
	// 823DB9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823DB9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DB9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DB9BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DB9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DB9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DB9C8 size=84
    let mut pc: u32 = 0x823DB9C8;
    'dispatch: loop {
        match pc {
            0x823DB9C8 => {
    //   block [0x823DB9C8..0x823DBA1C)
	// 823DB9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DB9CC: 481596ED  bl 0x825350b8
	ctx.lr = 0x823DB9D0;
	sub_82535080(ctx, base);
	// 823DB9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DB9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DB9D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823DB9DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823DB9E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 823DB9E4: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DB9E8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DB9EC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DB9F0: 4099000C  ble cr6, 0x823db9fc
	if !ctx.cr[6].gt {
	pc = 0x823DB9FC; continue 'dispatch;
	}
	// 823DB9F4: 4BFFFCDD  bl 0x823db6d0
	ctx.lr = 0x823DB9F8;
	sub_823DB6D0(ctx, base);
	// 823DB9F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823DB9FC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 823DBA00: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 823DBA04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823DBA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DBA0C: 4BFFF74D  bl 0x823db158
	ctx.lr = 0x823DBA10;
	sub_823DB158(ctx, base);
	// 823DBA10: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 823DBA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823DBA18: 481596F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823DBA20 size=1092
    let mut pc: u32 = 0x823DBA20;
    'dispatch: loop {
        match pc {
            0x823DBA20 => {
    //   block [0x823DBA20..0x823DBE64)
	// 823DBA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DBA24: 48159685  bl 0x825350a8
	ctx.lr = 0x823DBA28;
	sub_82535080(ctx, base);
	// 823DBA28: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DBA2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DBA30: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 823DBA34: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DBA38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DBA3C: 419A0030  beq cr6, 0x823dba6c
	if ctx.cr[6].eq {
	pc = 0x823DBA6C; continue 'dispatch;
	}
	// 823DBA40: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DBA44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DBA48: 419A0024  beq cr6, 0x823dba6c
	if ctx.cr[6].eq {
	pc = 0x823DBA6C; continue 'dispatch;
	}
	// 823DBA4C: 4BFFFF05  bl 0x823db950
	ctx.lr = 0x823DBA50;
	sub_823DB950(ctx, base);
	// 823DBA50: 4800000C  b 0x823dba5c
	pc = 0x823DBA5C; continue 'dispatch;
	// 823DBA54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823DBA58: 4BFE4971  bl 0x823c03c8
	ctx.lr = 0x823DBA5C;
	sub_823C03C8(ctx, base);
	// 823DBA5C: 815F2AE8  lwz r10, 0x2ae8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10984 as u32) ) } as u64;
	// 823DBA60: 817F2AEC  lwz r11, 0x2aec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10988 as u32) ) } as u64;
	// 823DBA64: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 823DBA68: 409AFFEC  bne cr6, 0x823dba54
	if !ctx.cr[6].eq {
	pc = 0x823DBA54; continue 'dispatch;
	}
	// 823DBA6C: 815F3A38  lwz r10, 0x3a38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14904 as u32) ) } as u64;
	// 823DBA70: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DBA74: 4182004C  beq 0x823dbac0
	if ctx.cr[0].eq {
	pc = 0x823DBAC0; continue 'dispatch;
	}
	// 823DBA78: 817F3A3C  lwz r11, 0x3a3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14908 as u32) ) } as u64;
	// 823DBA7C: 3D204000  lis r9, 0x4000
	ctx.r[9].s64 = 1073741824;
	// 823DBA80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DBA84: 5568653E  srwi r8, r11, 0x14
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DBA88: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 823DBA8C: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 823DBA90: 5549653E  srwi r9, r10, 0x14
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DBA94: 39080200  addi r8, r8, 0x200
	ctx.r[8].s64 = ctx.r[8].s64 + 512;
	// 823DBA98: 38890200  addi r4, r9, 0x200
	ctx.r[4].s64 = ctx.r[9].s64 + 512;
	// 823DBA9C: 550904E6  rlwinm r9, r8, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBAA0: 556800FE  clrlwi r8, r11, 3
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBAA4: 548B04E6  rlwinm r11, r4, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBAA8: 554A00FE  clrlwi r10, r10, 3
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBAAC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 823DBAB0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DBAB4: 7C874850  subf r4, r7, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 823DBAB8: 7C665850  subf r3, r6, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 823DBABC: 4800419D  bl 0x823dfc58
	ctx.lr = 0x823DBAC0;
	sub_823DFC58(ctx, base);
	// 823DBAC0: 3FC0B180  lis r30, -0x4e80
	ctx.r[30].s64 = -1317011456;
	// 823DBAC4: 807F39F4  lwz r3, 0x39f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14836 as u32) ) } as u64;
	// 823DBAC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823DBACC: 4BFEE78D  bl 0x823ca258
	ctx.lr = 0x823DBAD0;
	sub_823CA258(ctx, base);
	// 823DBAD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823DBAD4: 807F39F8  lwz r3, 0x39f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14840 as u32) ) } as u64;
	// 823DBAD8: 4BFEE781  bl 0x823ca258
	ctx.lr = 0x823DBADC;
	sub_823CA258(ctx, base);
	// 823DBADC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823DBAE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DBAE4: 93BF39F4  stw r29, 0x39f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14836 as u32), ctx.r[29].u32 ) };
	// 823DBAE8: 93BF39F8  stw r29, 0x39f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14840 as u32), ctx.r[29].u32 ) };
	// 823DBAEC: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 823DBAF0: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 823DBAF4: 48331DE9  bl 0x8270d8dc
	ctx.lr = 0x823DBAF8;
	// extern call 0x8270D8DC → crate::xboxkrnl::VdSetSystemCommandBufferGpuIdentifierAddress
	crate::xboxkrnl::VdSetSystemCommandBufferGpuIdentifierAddress(ctx, base);
	// 823DBAF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823DBAFC: 807F2A94  lwz r3, 0x2a94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DBB00: 4BFEE759  bl 0x823ca258
	ctx.lr = 0x823DBB04;
	sub_823CA258(ctx, base);
	// 823DBB04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823DBB08: 807F2A90  lwz r3, 0x2a90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBB0C: 4BFEE74D  bl 0x823ca258
	ctx.lr = 0x823DBB10;
	sub_823CA258(ctx, base);
	// 823DBB10: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 823DBB14: 93BF2A94  stw r29, 0x2a94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10900 as u32), ctx.r[29].u32 ) };
	// 823DBB18: 93BF2A90  stw r29, 0x2a90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10896 as u32), ctx.r[29].u32 ) };
	// 823DBB1C: 409A000C  bne cr6, 0x823dbb28
	if !ctx.cr[6].eq {
	pc = 0x823DBB28; continue 'dispatch;
	}
	// 823DBB20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DBB24: 48000338  b 0x823dbe5c
	pc = 0x823DBE5C; continue 'dispatch;
	// 823DBB28: 835B0004  lwz r26, 4(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DBB2C: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DBB30: 833B000C  lwz r25, 0xc(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 823DBB34: 281A0000  cmplwi r26, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DBB38: 83DB0010  lwz r30, 0x10(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 823DBB3C: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 823DBB40: 4082000C  bne 0x823dbb4c
	if !ctx.cr[0].eq {
	pc = 0x823DBB4C; continue 'dispatch;
	}
	// 823DBB44: 3F400000  lis r26, 0
	ctx.r[26].s64 = 0;
	// 823DBB48: 635A8000  ori r26, r26, 0x8000
	ctx.r[26].u64 = ctx.r[26].u64 | 32768;
	// 823DBB4C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 823DBB50: 409A0008  bne cr6, 0x823dbb58
	if !ctx.cr[6].eq {
	pc = 0x823DBB58; continue 'dispatch;
	}
	// 823DBB54: 3F200020  lis r25, 0x20
	ctx.r[25].s64 = 2097152;
	// 823DBB58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DBB5C: 409A0008  bne cr6, 0x823dbb64
	if !ctx.cr[6].eq {
	pc = 0x823DBB64; continue 'dispatch;
	}
	// 823DBB60: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 823DBB64: 7F795B96  divwu r27, r25, r11
	ctx.r[27].u32 = ctx.r[25].u32 / ctx.r[11].u32;
	// 823DBB68: 0CCB0000  twi 6, r11, 0
	// 823DBB6C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 823DBB70: 409A0018  bne cr6, 0x823dbb88
	if !ctx.cr[6].eq {
	pc = 0x823DBB88; continue 'dispatch;
	}
	// 823DBB74: 3C80B580  lis r4, -0x4a80
	ctx.r[4].s64 = -1249902592;
	// 823DBB78: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 823DBB7C: 4BFEE645  bl 0x823ca1c0
	ctx.lr = 0x823DBB80;
	sub_823CA1C0(ctx, base);
	// 823DBB80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823DBB84: 939F39F4  stw r28, 0x39f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14836 as u32), ctx.r[28].u32 ) };
	// 823DBB88: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823DBB8C: 409A0028  bne cr6, 0x823dbbb4
	if !ctx.cr[6].eq {
	pc = 0x823DBBB4; continue 'dispatch;
	}
	// 823DBB90: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 823DBB94: 3C808780  lis r4, -0x7880
	ctx.r[4].s64 = -2021654528;
	// 823DBB98: 7D6BC810  subfc r11, r11, r25
	ctx.xer.ca = ctx.r[25].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	// 823DBB9C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 823DBBA0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 823DBBA4: 5164E086  rlwimi r4, r11, 0x1c, 2, 3
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(28) as u64) & 0x0000000030000000) | (ctx.r[4].u64 & 0xFFFFFFFFCFFFFFFF);
	// 823DBBA8: 4BFEE619  bl 0x823ca1c0
	ctx.lr = 0x823DBBAC;
	sub_823CA1C0(ctx, base);
	// 823DBBAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DBBB0: 93DF39F8  stw r30, 0x39f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14840 as u32), ctx.r[30].u32 ) };
	// 823DBBB4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 823DBBB8: 419A029C  beq cr6, 0x823dbe54
	if ctx.cr[6].eq {
	pc = 0x823DBE54; continue 'dispatch;
	}
	// 823DBBBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823DBBC0: 419A0294  beq cr6, 0x823dbe54
	if ctx.cr[6].eq {
	pc = 0x823DBE54; continue 'dispatch;
	}
	// 823DBBC4: 3C80A580  lis r4, -0x5a80
	ctx.r[4].s64 = -1518338048;
	// 823DBBC8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 823DBBCC: 4BFEE5F5  bl 0x823ca1c0
	ctx.lr = 0x823DBBD0;
	sub_823CA1C0(ctx, base);
	// 823DBBD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DBBD4: 907F2A90  stw r3, 0x2a90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10896 as u32), ctx.r[3].u32 ) };
	// 823DBBD8: 4182027C  beq 0x823dbe54
	if ctx.cr[0].eq {
	pc = 0x823DBE54; continue 'dispatch;
	}
	// 823DBBDC: 3C809580  lis r4, -0x6a80
	ctx.r[4].s64 = -1786773504;
	// 823DBBE0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 823DBBE4: 4BFEE5DD  bl 0x823ca1c0
	ctx.lr = 0x823DBBE8;
	sub_823CA1C0(ctx, base);
	// 823DBBE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DBBEC: 907F2A94  stw r3, 0x2a94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10900 as u32), ctx.r[3].u32 ) };
	// 823DBBF0: 41820264  beq 0x823dbe54
	if ctx.cr[0].eq {
	pc = 0x823DBE54; continue 'dispatch;
	}
	// 823DBBF4: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 823DBBF8: 807F2A90  lwz r3, 0x2a90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBBFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DBC00: 481595D1  bl 0x825351d0
	ctx.lr = 0x823DBC04;
	sub_825351D0(ctx, base);
	// 823DBC04: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 823DBC08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DBC0C: 807F2A94  lwz r3, 0x2a94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DBC10: 481595C1  bl 0x825351d0
	ctx.lr = 0x823DBC14;
	sub_825351D0(ctx, base);
	// 823DBC14: 7F4B0034  cntlzw r11, r26
	ctx.r[11].u64 = if ctx.r[26].u32 == 0 { 32 } else { ctx.r[26].u32.leading_zeros() as u64 };
	// 823DBC18: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823DBC1C: 230B001C  subfic r24, r11, 0x1c
	ctx.xer.ca = ctx.r[11].u32 <= 28 as u32;
	ctx.r[24].s64 = (28 as i64) - ctx.r[11].s64;
	// 823DBC20: 48331CAD  bl 0x8270d8cc
	ctx.lr = 0x823DBC24;
	// extern call 0x8270D8CC → crate::xboxkrnl::MmGetPhysicalAddress
	crate::xboxkrnl::MmGetPhysicalAddress(ctx, base);
	// 823DBC24: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 823DBC28: 48331C95  bl 0x8270d8bc
	ctx.lr = 0x823DBC2C;
	// extern call 0x8270D8BC → crate::xboxkrnl::VdInitializeRingBuffer
	crate::xboxkrnl::VdInitializeRingBuffer(ctx, base);
	// 823DBC2C: 574BBA7E  srwi r11, r26, 9
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DBC30: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 823DBC34: 208B001F  subfic r4, r11, 0x1f
	ctx.xer.ca = ctx.r[11].u32 <= 31 as u32;
	ctx.r[4].s64 = (31 as i64) - ctx.r[11].s64;
	// 823DBC38: 2B040013  cmplwi cr6, r4, 0x13
	ctx.cr[6].compare_u32(ctx.r[4].u32, 19 as u32, &mut ctx.xer);
	// 823DBC3C: 40990008  ble cr6, 0x823dbc44
	if !ctx.cr[6].gt {
	pc = 0x823DBC44; continue 'dispatch;
	}
	// 823DBC40: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 823DBC44: 817F2A90  lwz r11, 0x2a90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBC48: 396B003C  addi r11, r11, 0x3c
	ctx.r[11].s64 = ctx.r[11].s64 + 60;
	// 823DBC4C: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DBC50: 556A00FE  clrlwi r10, r11, 3
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBC54: 39690200  addi r11, r9, 0x200
	ctx.r[11].s64 = ctx.r[9].s64 + 512;
	// 823DBC58: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBC5C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DBC60: 48331C4D  bl 0x8270d8ac
	ctx.lr = 0x823DBC64;
	// extern call 0x8270D8AC → crate::xboxkrnl::VdEnableRingBufferRPtrWriteBack
	crate::xboxkrnl::VdEnableRingBufferRPtrWriteBack(ctx, base);
	// 823DBC64: 3D200BAD  lis r9, 0xbad
	ctx.r[9].s64 = 195887104;
	// 823DBC68: 572BF0BE  srwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DBC6C: 6128F00D  ori r8, r9, 0xf00d
	ctx.r[8].u64 = ctx.r[9].u64 | 61453;
	// 823DBC70: 5749F0BE  srwi r9, r26, 2
	ctx.r[9].u32 = ctx.r[26].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DBC74: 576A003A  rlwinm r10, r27, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBC78: 38E9FFFF  addi r7, r9, -1
	ctx.r[7].s64 = ctx.r[9].s64 + -1;
	// 823DBC7C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DBC80: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 823DBC84: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 823DBC88: 38DEFFFC  addi r6, r30, -4
	ctx.r[6].s64 = ctx.r[30].s64 + -4;
	// 823DBC8C: 3929FFFC  addi r9, r9, -4
	ctx.r[9].s64 = ctx.r[9].s64 + -4;
	// 823DBC90: 38AAFF60  addi r5, r10, -0xa0
	ctx.r[5].s64 = ctx.r[10].s64 + -160;
	// 823DBC94: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 823DBC98: 811F2A90  lwz r8, 0x2a90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBC9C: 917F3A58  stw r11, 0x3a58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14936 as u32), ctx.r[11].u32 ) };
	// 823DBCA0: 939F3A30  stw r28, 0x3a30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14896 as u32), ctx.r[28].u32 ) };
	// 823DBCA4: 93DF3A38  stw r30, 0x3a38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14904 as u32), ctx.r[30].u32 ) };
	// 823DBCA8: 93DF3A4C  stw r30, 0x3a4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14924 as u32), ctx.r[30].u32 ) };
	// 823DBCAC: 913F3A3C  stw r9, 0x3a3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14908 as u32), ctx.r[9].u32 ) };
	// 823DBCB0: 93DF3A50  stw r30, 0x3a50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14928 as u32), ctx.r[30].u32 ) };
	// 823DBCB4: 937F3A40  stw r27, 0x3a40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14912 as u32), ctx.r[27].u32 ) };
	// 823DBCB8: 90FF3A34  stw r7, 0x3a34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14900 as u32), ctx.r[7].u32 ) };
	// 823DBCBC: 93A8003C  stw r29, 0x3c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 823DBCC0: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DBCC4: 93BF2AC8  stw r29, 0x2ac8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10952 as u32), ctx.r[29].u32 ) };
	// 823DBCC8: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 823DBCCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DBCD0: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 823DBCD4: 90BF0038  stw r5, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[5].u32 ) };
	// 823DBCD8: 93BF3A44  stw r29, 0x3a44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14916 as u32), ctx.r[29].u32 ) };
	// 823DBCDC: 409A000C  bne cr6, 0x823dbce8
	if !ctx.cr[6].eq {
	pc = 0x823DBCE8; continue 'dispatch;
	}
	// 823DBCE0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 823DBCE4: 917F2A9C  stw r11, 0x2a9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10908 as u32), ctx.r[11].u32 ) };
	// 823DBCE8: 817F2A9C  lwz r11, 0x2a9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10908 as u32) ) } as u64;
	// 823DBCEC: 815F2A90  lwz r10, 0x2a90(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBCF0: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 823DBCF4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DBCF8: 817F3A48  lwz r11, 0x3a48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14920 as u32) ) } as u64;
	// 823DBCFC: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DBD00: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823DBD04: 813F2A90  lwz r9, 0x2a90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBD08: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DBD0C: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DBD10: 817F2A90  lwz r11, 0x2a90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 823DBD14: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 823DBD18: 48331BC5  bl 0x8270d8dc
	ctx.lr = 0x823DBD1C;
	// extern call 0x8270D8DC → crate::xboxkrnl::VdSetSystemCommandBufferGpuIdentifierAddress
	crate::xboxkrnl::VdSetSystemCommandBufferGpuIdentifierAddress(ctx, base);
	// 823DBD1C: 3D40C011  lis r10, -0x3fef
	ctx.r[10].s64 = -1072627712;
	// 823DBD20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DBD24: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 823DBD28: 614A4800  ori r10, r10, 0x4800
	ctx.r[10].u64 = ctx.r[10].u64 | 18432;
	// 823DBD2C: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 823DBD30: 396B17CC  addi r11, r11, 0x17cc
	ctx.r[11].s64 = ctx.r[11].s64 + 6092;
	// 823DBD34: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 823DBD38: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823DBD3C: 394003FF  li r10, 0x3ff
	ctx.r[10].s64 = 1023;
	// 823DBD40: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823DBD44: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 823DBD48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 823DBD4C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DBD50: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823DBD54: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823DBD58: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 823DBD5C: 4200FFF0  bdnz 0x823dbd4c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x823DBD4C; continue 'dispatch;
	}
	// 823DBD60: 39600800  li r11, 0x800
	ctx.r[11].s64 = 2048;
	// 823DBD64: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 823DBD68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DBD6C: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 823DBD70: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 823DBD74: 93A10090  stw r29, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[29].u32 ) };
	// 823DBD78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823DBD7C: 93A10094  stw r29, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 823DBD80: 93A10098  stw r29, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[29].u32 ) };
	// 823DBD84: B1610082  sth r11, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[11].u16 ) };
	// 823DBD88: B17F2AAE  sth r11, 0x2aae(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10926 as u32), ctx.r[11].u16 ) };
	// 823DBD8C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 823DBD90: B1410080  sth r10, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u16 ) };
	// 823DBD94: B15F2AAC  sth r10, 0x2aac(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10924 as u32), ctx.r[10].u16 ) };
	// 823DBD98: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 823DBD9C: 48331B01  bl 0x8270d89c
	ctx.lr = 0x823DBDA0;
	// extern call 0x8270D89C → crate::xboxkrnl::KiApcNormalRoutineNop
	crate::xboxkrnl::KiApcNormalRoutineNop(ctx, base);
	// 823DBDA0: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 823DBDA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823DBDA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DBDAC: 4BFFE705  bl 0x823da4b0
	ctx.lr = 0x823DBDB0;
	sub_823DA4B0(ctx, base);
	// 823DBDB0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DBDB4: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DBDB8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DBDBC: 40990010  ble cr6, 0x823dbdcc
	if !ctx.cr[6].gt {
	pc = 0x823DBDCC; continue 'dispatch;
	}
	// 823DBDC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DBDC4: 4BFFF90D  bl 0x823db6d0
	ctx.lr = 0x823DBDC8;
	sub_823DB6D0(ctx, base);
	// 823DBDC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DBDCC: 39400D02  li r10, 0xd02
	ctx.r[10].s64 = 3330;
	// 823DBDD0: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 823DBDD4: 3D000003  lis r8, 3
	ctx.r[8].s64 = 196608;
	// 823DBDD8: 61290800  ori r9, r9, 0x800
	ctx.r[9].u64 = ctx.r[9].u64 | 2048;
	// 823DBDDC: 61080A02  ori r8, r8, 0xa02
	ctx.r[8].u64 = ctx.r[8].u64 | 2562;
	// 823DBDE0: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DBDE4: 3D40C010  lis r10, -0x3ff0
	ctx.r[10].s64 = -1072693248;
	// 823DBDE8: 3CE007F0  lis r7, 0x7f0
	ctx.r[7].s64 = 133169152;
	// 823DBDEC: 3CC0C000  lis r6, -0x4000
	ctx.r[6].s64 = -1073741824;
	// 823DBDF0: 3CA00010  lis r5, 0x10
	ctx.r[5].s64 = 1048576;
	// 823DBDF4: 388001DD  li r4, 0x1dd
	ctx.r[4].s64 = 477;
	// 823DBDF8: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DBDFC: 3BC001DC  li r30, 0x1dc
	ctx.r[30].s64 = 476;
	// 823DBE00: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 823DBE04: 607D0033  ori r29, r3, 0x33
	ctx.r[29].u64 = ctx.r[3].u64 | 51;
	// 823DBE08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DBE0C: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE10: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE14: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE18: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE1C: 94AB0004  stwu r5, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[5].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE20: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE24: 815F2A94  lwz r10, 0x2a94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10900 as u32) ) } as u64;
	// 823DBE28: 5548653E  srwi r8, r10, 0x14
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(20);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DBE2C: 554900FE  clrlwi r9, r10, 3
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBE30: 39480200  addi r10, r8, 0x200
	ctx.r[10].s64 = ctx.r[8].s64 + 512;
	// 823DBE34: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBE38: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 823DBE3C: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE40: 97CB0004  stwu r30, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE44: 97AB0004  stwu r29, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[29].u32) };
	ctx.r[11].u32 = ea;
	// 823DBE48: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DBE4C: 4BFFFA35  bl 0x823db880
	ctx.lr = 0x823DBE50;
	sub_823DB880(ctx, base);
	// 823DBE50: 4BFFFCD0  b 0x823dbb20
	pc = 0x823DBB20; continue 'dispatch;
	// 823DBE54: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 823DBE58: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 823DBE5C: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 823DBE60: 48159298  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DBE68 size=92
    let mut pc: u32 = 0x823DBE68;
    'dispatch: loop {
        match pc {
            0x823DBE68 => {
    //   block [0x823DBE68..0x823DBEC4)
	// 823DBE68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DBE6C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DBE70: 812B34CC  lwz r9, 0x34cc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13516 as u32) ) } as u64;
	// 823DBE74: 4182007C  beq 0x823dbef0
	if ctx.cr[0].eq {
		sub_823DBEF0(ctx, base);
		return;
	}
	// 823DBE78: 814B3A44  lwz r10, 0x3a44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14916 as u32) ) } as u64;
	// 823DBE7C: 81050000  lwz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DBE80: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DBE84: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DBE88: 40820014  bne 0x823dbe9c
	if !ctx.cr[0].eq {
	pc = 0x823DBE9C; continue 'dispatch;
	}
	// 823DBE8C: 8149009C  lwz r10, 0x9c(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(156 as u32) ) } as u64;
	// 823DBE90: 81290098  lwz r9, 0x98(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DBE94: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBE98: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 823DBE9C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DBEA0: 812B0030  lwz r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DBEA4: 38E6FFFF  addi r7, r6, -1
	ctx.r[7].s64 = ctx.r[6].s64 + -1;
	// 823DBEA8: 7D085050  subf r8, r8, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823DBEAC: 392900A4  addi r9, r9, 0xa4
	ctx.r[9].s64 = ctx.r[9].s64 + 164;
	// 823DBEB0: 7D033878  andc r3, r8, r7
	ctx.r[3].u64 = ctx.r[8].u64 & !ctx.r[7].u64;
	// 823DBEB4: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823DBEB8: 4098000C  bge cr6, 0x823dbec4
	if !ctx.cr[6].lt {
		sub_823DBEC4(ctx, base);
		return;
	}
	// 823DBEBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823DBEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBEC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DBEC4 size=44
    let mut pc: u32 = 0x823DBEC4;
    'dispatch: loop {
        match pc {
            0x823DBEC4 => {
    //   block [0x823DBEC4..0x823DBEF0)
	// 823DBEC4: 7D435050  subf r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 823DBEC8: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 823DBECC: 810B0038  lwz r8, 0x38(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DBED0: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 823DBED4: 906B3A44  stw r3, 0x3a44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(14916 as u32), ctx.r[3].u32 ) };
	// 823DBED8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DBEDC: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 823DBEE0: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 823DBEE4: 912B0034  stw r9, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 823DBEE8: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 823DBEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DBEF0 size=36
    let mut pc: u32 = 0x823DBEF0;
    'dispatch: loop {
        match pc {
            0x823DBEF0 => {
    //   block [0x823DBEF0..0x823DBF14)
	// 823DBEF0: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DBEF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DBEF8: 409AFFC4  bne cr6, 0x823dbebc
	if !ctx.cr[6].eq {
		sub_823DBE68(ctx, base);
		return;
	}
	// 823DBEFC: 81490098  lwz r10, 0x98(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DBF00: 914B3A50  stw r10, 0x3a50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(14928 as u32), ctx.r[10].u32 ) };
	// 823DBF04: 8169009C  lwz r11, 0x9c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(156 as u32) ) } as u64;
	// 823DBF08: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DBF0C: 80690098  lwz r3, 0x98(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DBF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DBF18 size=12
    let mut pc: u32 = 0x823DBF18;
    'dispatch: loop {
        match pc {
            0x823DBF18 => {
    //   block [0x823DBF18..0x823DBF24)
	// 823DBF18: 8123009C  lwz r9, 0x9c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 823DBF1C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DBF20: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBF24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DBF24 size=80
    let mut pc: u32 = 0x823DBF24;
    'dispatch: loop {
        match pc {
            0x823DBF24 => {
    //   block [0x823DBF24..0x823DBF74)
	// 823DBF24: 3D404000  lis r10, 0x4000
	ctx.r[10].s64 = 1073741824;
	// 823DBF28: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DBF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DBF30: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 823DBF34: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 823DBF38: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 823DBF3C: 5568653E  srwi r8, r11, 0x14
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DBF40: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 823DBF44: 38880200  addi r4, r8, 0x200
	ctx.r[4].s64 = ctx.r[8].s64 + 512;
	// 823DBF48: 5549653E  srwi r9, r10, 0x14
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DBF4C: 554800FE  clrlwi r8, r10, 3
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBF50: 39290200  addi r9, r9, 0x200
	ctx.r[9].s64 = ctx.r[9].s64 + 512;
	// 823DBF54: 548A04E6  rlwinm r10, r4, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBF58: 552904E6  rlwinm r9, r9, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBF5C: 556B00FE  clrlwi r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBF60: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 823DBF64: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823DBF68: 7C874850  subf r4, r7, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 823DBF6C: 7C665850  subf r3, r6, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 823DBF70: 48003CE8  b 0x823dfc58
	sub_823DFC58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBF74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DBF74 size=4
    let mut pc: u32 = 0x823DBF74;
    'dispatch: loop {
        match pc {
            0x823DBF74 => {
    //   block [0x823DBF74..0x823DBF78)
	// 823DBF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DBF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DBF78 size=172
    let mut pc: u32 = 0x823DBF78;
    'dispatch: loop {
        match pc {
            0x823DBF78 => {
    //   block [0x823DBF78..0x823DC024)
	// 823DBF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DBF7C: 48159141  bl 0x825350bc
	ctx.lr = 0x823DBF80;
	sub_82535080(ctx, base);
	// 823DBF80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DBF84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DBF88: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 823DBF8C: 388001F6  li r4, 0x1f6
	ctx.r[4].s64 = 502;
	// 823DBF90: 83BE34D0  lwz r29, 0x34d0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13520 as u32) ) } as u64;
	// 823DBF94: 4BFFEAAD  bl 0x823daa40
	ctx.lr = 0x823DBF98;
	sub_823DAA40(ctx, base);
	// 823DBF98: 897E2ABD  lbz r11, 0x2abd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DBF9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DBFA0: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DBFA4: 4182000C  beq 0x823dbfb0
	if ctx.cr[0].eq {
	pc = 0x823DBFB0; continue 'dispatch;
	}
	// 823DBFA8: 83FE4158  lwz r31, 0x4158(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16728 as u32) ) } as u64;
	// 823DBFAC: 48000060  b 0x823dc00c
	pc = 0x823DC00C; continue 'dispatch;
	// 823DBFB0: 57EB653E  srwi r11, r31, 0x14
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DBFB4: 93FE34D0  stw r31, 0x34d0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13520 as u32), ctx.r[31].u32 ) };
	// 823DBFB8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823DBFBC: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DBFC0: 57EA00FE  clrlwi r10, r31, 3
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x1FFFFFFFu64;
	// 823DBFC4: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DBFC8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823DBFCC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DBFD0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823DBFD4: 3C6BC000  addis r3, r11, -0x4000
	ctx.r[3].s64 = ctx.r[11].s64 + -1073741824;
	// 823DBFD8: 409A0010  bne cr6, 0x823dbfe8
	if !ctx.cr[6].eq {
	pc = 0x823DBFE8; continue 'dispatch;
	}
	// 823DBFDC: 817E34CC  lwz r11, 0x34cc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13516 as u32) ) } as u64;
	// 823DBFE0: 906B0070  stw r3, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 823DBFE4: 4800001C  b 0x823dc000
	pc = 0x823DC000; continue 'dispatch;
	// 823DBFE8: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 823DBFEC: 817E34D4  lwz r11, 0x34d4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13524 as u32) ) } as u64;
	// 823DBFF0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 823DBFF4: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 823DBFF8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 823DBFFC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DC000: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DC004: 388307D8  addi r4, r3, 0x7d8
	ctx.r[4].s64 = ctx.r[3].s64 + 2008;
	// 823DC008: 48003C51  bl 0x823dfc58
	ctx.lr = 0x823DC00C;
	sub_823DFC58(ctx, base);
	// 823DC00C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 823DC010: 397F07D8  addi r11, r31, 0x7d8
	ctx.r[11].s64 = ctx.r[31].s64 + 2008;
	// 823DC014: 907E34D4  stw r3, 0x34d4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13524 as u32), ctx.r[3].u32 ) };
	// 823DC018: 917E34D8  stw r11, 0x34d8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13528 as u32), ctx.r[11].u32 ) };
	// 823DC01C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DC020: 481590EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DC028 size=172
    let mut pc: u32 = 0x823DC028;
    'dispatch: loop {
        match pc {
            0x823DC028 => {
    //   block [0x823DC028..0x823DC0D4)
	// 823DC028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DC02C: 48159091  bl 0x825350bc
	ctx.lr = 0x823DC030;
	sub_82535080(ctx, base);
	// 823DC030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DC034: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823DC038: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 823DC03C: 38800088  li r4, 0x88
	ctx.r[4].s64 = 136;
	// 823DC040: 83BE34DC  lwz r29, 0x34dc(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13532 as u32) ) } as u64;
	// 823DC044: 4BFFE9FD  bl 0x823daa40
	ctx.lr = 0x823DC048;
	sub_823DAA40(ctx, base);
	// 823DC048: 897E2ABD  lbz r11, 0x2abd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10941 as u32) ) } as u64;
	// 823DC04C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DC050: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DC054: 4182000C  beq 0x823dc060
	if ctx.cr[0].eq {
	pc = 0x823DC060; continue 'dispatch;
	}
	// 823DC058: 83FE4158  lwz r31, 0x4158(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16728 as u32) ) } as u64;
	// 823DC05C: 48000060  b 0x823dc0bc
	pc = 0x823DC0BC; continue 'dispatch;
	// 823DC060: 57EB653E  srwi r11, r31, 0x14
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shr(20);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DC064: 93FE34DC  stw r31, 0x34dc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13532 as u32), ctx.r[31].u32 ) };
	// 823DC068: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823DC06C: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 823DC070: 57EA00FE  clrlwi r10, r31, 3
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x1FFFFFFFu64;
	// 823DC074: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC078: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823DC07C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823DC080: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823DC084: 3C6BC000  addis r3, r11, -0x4000
	ctx.r[3].s64 = ctx.r[11].s64 + -1073741824;
	// 823DC088: 409A0010  bne cr6, 0x823dc098
	if !ctx.cr[6].eq {
	pc = 0x823DC098; continue 'dispatch;
	}
	// 823DC08C: 817E34CC  lwz r11, 0x34cc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13516 as u32) ) } as u64;
	// 823DC090: 906B0074  stw r3, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 823DC094: 4800001C  b 0x823dc0b0
	pc = 0x823DC0B0; continue 'dispatch;
	// 823DC098: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 823DC09C: 817E34E0  lwz r11, 0x34e0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13536 as u32) ) } as u64;
	// 823DC0A0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 823DC0A4: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 823DC0A8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 823DC0AC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823DC0B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823DC0B4: 38830088  addi r4, r3, 0x88
	ctx.r[4].s64 = ctx.r[3].s64 + 136;
	// 823DC0B8: 48003BA1  bl 0x823dfc58
	ctx.lr = 0x823DC0BC;
	sub_823DFC58(ctx, base);
	// 823DC0BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 823DC0C0: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 823DC0C4: 907E34E0  stw r3, 0x34e0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13536 as u32), ctx.r[3].u32 ) };
	// 823DC0C8: 917E34E4  stw r11, 0x34e4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(13540 as u32), ctx.r[11].u32 ) };
	// 823DC0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DC0D0: 4815903C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DC0D8 size=124
    let mut pc: u32 = 0x823DC0D8;
    'dispatch: loop {
        match pc {
            0x823DC0D8 => {
    //   block [0x823DC0D8..0x823DC154)
	// 823DC0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DC0DC: 48158FD9  bl 0x825350b4
	ctx.lr = 0x823DC0E0;
	sub_82535080(ctx, base);
	// 823DC0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DC0E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823DC0E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DC0EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823DC0F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823DC0F4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 823DC0F8: 814A05B0  lwz r10, 0x5b0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823DC0FC: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 823DC100: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DC104: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823DC108: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DC10C: 40980008  bge cr6, 0x823dc114
	if !ctx.cr[6].lt {
	pc = 0x823DC114; continue 'dispatch;
	}
	// 823DC110: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823DC114: 3BAA3A7C  addi r29, r10, 0x3a7c
	ctx.r[29].s64 = ctx.r[10].s64 + 14972;
	// 823DC118: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823DC11C: 48331141  bl 0x8270d25c
	ctx.lr = 0x823DC120;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 823DC120: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 823DC124: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 823DC128: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 823DC12C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823DC130: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823DC134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DC138: 4E800421  bctrl
	ctx.lr = 0x823DC13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DC13C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DC140: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823DC144: 48331129  bl 0x8270d26c
	ctx.lr = 0x823DC148;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 823DC148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DC14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823DC150: 48158FB4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DC158 size=96
    let mut pc: u32 = 0x823DC158;
    'dispatch: loop {
        match pc {
            0x823DC158 => {
    //   block [0x823DC158..0x823DC1B8)
	// 823DC158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DC15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DC160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823DC164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DC168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DC16C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DC170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DC174: 816B05B0  lwz r11, 0x5b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 823DC178: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DC17C: 3BCB3A7C  addi r30, r11, 0x3a7c
	ctx.r[30].s64 = ctx.r[11].s64 + 14972;
	// 823DC180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DC184: 483310D9  bl 0x8270d25c
	ctx.lr = 0x823DC188;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 823DC188: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 823DC18C: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 823DC190: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823DC194: 4E800421  bctrl
	ctx.lr = 0x823DC198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823DC198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823DC19C: 483310D1  bl 0x8270d26c
	ctx.lr = 0x823DC1A0;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 823DC1A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823DC1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DC1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DC1AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823DC1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DC1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DC1B8 size=76
    let mut pc: u32 = 0x823DC1B8;
    'dispatch: loop {
        match pc {
            0x823DC1B8 => {
    //   block [0x823DC1B8..0x823DC204)
	// 823DC1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DC1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823DC1C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823DC1C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DC1C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DC1CC: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DC1D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DC1D4: 409A000C  bne cr6, 0x823dc1e0
	if !ctx.cr[6].eq {
	pc = 0x823DC1E0; continue 'dispatch;
	}
	// 823DC1D8: 4BFFFF81  bl 0x823dc158
	ctx.lr = 0x823DC1DC;
	sub_823DC158(ctx, base);
	// 823DC1DC: 48000014  b 0x823dc1f0
	pc = 0x823DC1F0; continue 'dispatch;
	// 823DC1E0: 4BFFFD39  bl 0x823dbf18
	ctx.lr = 0x823DC1E4;
	sub_823DBF18(ctx, base);
	// 823DC1E4: 3C80B180  lis r4, -0x4e80
	ctx.r[4].s64 = -1317011456;
	// 823DC1E8: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 823DC1EC: 4BFEE06D  bl 0x823ca258
	ctx.lr = 0x823DC1F0;
	sub_823CA258(ctx, base);
	// 823DC1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823DC1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823DC1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823DC1FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DC200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823DC208 size=736
    let mut pc: u32 = 0x823DC208;
    'dispatch: loop {
        match pc {
            0x823DC208 => {
    //   block [0x823DC208..0x823DC4E8)
	// 823DC208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823DC20C: 48158E89  bl 0x82535094
	ctx.lr = 0x823DC210;
	sub_82535080(ctx, base);
	// 823DC210: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823DC214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823DC218: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 823DC21C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 823DC220: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 823DC224: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 823DC228: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823DC22C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DC230: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC234: 4099000C  ble cr6, 0x823dc240
	if !ctx.cr[6].gt {
	pc = 0x823DC240; continue 'dispatch;
	}
	// 823DC238: 4BFFF499  bl 0x823db6d0
	ctx.lr = 0x823DC23C;
	sub_823DB6D0(ctx, base);
	// 823DC23C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DC240: 895F2ABC  lbz r10, 0x2abc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 823DC244: 554906F7  rlwinm. r9, r10, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823DC248: 4182000C  beq 0x823dc254
	if ctx.cr[0].eq {
	pc = 0x823DC254; continue 'dispatch;
	}
	// 823DC24C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DC250: 48000094  b 0x823dc2e4
	pc = 0x823DC2E4; continue 'dispatch;
	// 823DC254: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC258: 41820084  beq 0x823dc2dc
	if ctx.cr[0].eq {
	pc = 0x823DC2DC; continue 'dispatch;
	}
	// 823DC25C: 815F3098  lwz r10, 0x3098(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 823DC260: 813F31B8  lwz r9, 0x31b8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12728 as u32) ) } as u64;
	// 823DC264: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC268: 419A000C  beq cr6, 0x823dc274
	if ctx.cr[6].eq {
	pc = 0x823DC274; continue 'dispatch;
	}
	// 823DC26C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DC270: 409A006C  bne cr6, 0x823dc2dc
	if !ctx.cr[6].eq {
	pc = 0x823DC2DC; continue 'dispatch;
	}
	// 823DC274: 815F309C  lwz r10, 0x309c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 823DC278: 813F31BC  lwz r9, 0x31bc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12732 as u32) ) } as u64;
	// 823DC27C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC280: 419A000C  beq cr6, 0x823dc28c
	if ctx.cr[6].eq {
	pc = 0x823DC28C; continue 'dispatch;
	}
	// 823DC284: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DC288: 409A0054  bne cr6, 0x823dc2dc
	if !ctx.cr[6].eq {
	pc = 0x823DC2DC; continue 'dispatch;
	}
	// 823DC28C: 815F30A0  lwz r10, 0x30a0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 823DC290: 813F31C0  lwz r9, 0x31c0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12736 as u32) ) } as u64;
	// 823DC294: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC298: 419A000C  beq cr6, 0x823dc2a4
	if ctx.cr[6].eq {
	pc = 0x823DC2A4; continue 'dispatch;
	}
	// 823DC29C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DC2A0: 409A003C  bne cr6, 0x823dc2dc
	if !ctx.cr[6].eq {
	pc = 0x823DC2DC; continue 'dispatch;
	}
	// 823DC2A4: 815F30A4  lwz r10, 0x30a4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 823DC2A8: 813F31C4  lwz r9, 0x31c4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12740 as u32) ) } as u64;
	// 823DC2AC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC2B0: 419A000C  beq cr6, 0x823dc2bc
	if ctx.cr[6].eq {
	pc = 0x823DC2BC; continue 'dispatch;
	}
	// 823DC2B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DC2B8: 409A0024  bne cr6, 0x823dc2dc
	if !ctx.cr[6].eq {
	pc = 0x823DC2DC; continue 'dispatch;
	}
	// 823DC2BC: 815F30A8  lwz r10, 0x30a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 823DC2C0: 813F31C8  lwz r9, 0x31c8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12744 as u32) ) } as u64;
	// 823DC2C4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC2C8: 419A000C  beq cr6, 0x823dc2d4
	if ctx.cr[6].eq {
	pc = 0x823DC2D4; continue 'dispatch;
	}
	// 823DC2CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DC2D0: 409A000C  bne cr6, 0x823dc2dc
	if !ctx.cr[6].eq {
	pc = 0x823DC2DC; continue 'dispatch;
	}
	// 823DC2D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DC2D8: 48000008  b 0x823dc2e0
	pc = 0x823DC2E0; continue 'dispatch;
	// 823DC2DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DC2E0: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823DC2E4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC2E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823DC2EC: 40820038  bne 0x823dc324
	if !ctx.cr[0].eq {
	pc = 0x823DC324; continue 'dispatch;
	}
	// 823DC2F0: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 823DC2F4: 56E8809E  rlwinm r8, r23, 0x10, 2, 0xf
	ctx.r[8].u64 = ctx.r[23].u32 as u64 & 0x0000FFFFu64;
	// 823DC2F8: 614A2080  ori r10, r10, 0x2080
	ctx.r[10].u64 = ctx.r[10].u64 | 8320;
	// 823DC2FC: 570704BE  clrlwi r7, r24, 0x12
	ctx.r[7].u64 = ctx.r[24].u32 as u64 & 0x00003FFFu64;
	// 823DC300: 56A6809E  rlwinm r6, r21, 0x10, 2, 0xf
	ctx.r[6].u64 = ctx.r[21].u32 as u64 & 0x0000FFFFu64;
	// 823DC304: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC308: 7D0A3B78  or r10, r8, r7
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 823DC30C: 56C804BE  clrlwi r8, r22, 0x12
	ctx.r[8].u64 = ctx.r[22].u32 as u64 & 0x00003FFFu64;
	// 823DC310: 7CC84378  or r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 823DC314: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DC318: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC31C: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823DC320: 480001BC  b 0x823dc4dc
	pc = 0x823DC4DC; continue 'dispatch;
	// 823DC324: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 823DC328: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 823DC32C: 61596100  ori r25, r10, 0x6100
	ctx.r[25].u64 = ctx.r[10].u64 | 24832;
	// 823DC330: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 823DC334: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC338: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DC33C: 3D20C000  lis r9, -0x4000
	ctx.r[9].s64 = -1073741824;
	// 823DC340: 815F31CC  lwz r10, 0x31cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12748 as u32) ) } as u64;
	// 823DC344: 613E6000  ori r30, r9, 0x6000
	ctx.r[30].u64 = ctx.r[9].u64 | 24576;
	// 823DC348: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823DC34C: 40990118  ble cr6, 0x823dc464
	if !ctx.cr[6].gt {
	pc = 0x823DC464; continue 'dispatch;
	}
	// 823DC350: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823DC354: 3B9F31D4  addi r28, r31, 0x31d4
	ctx.r[28].s64 = ctx.r[31].s64 + 12756;
	// 823DC358: 3B7F32C4  addi r27, r31, 0x32c4
	ctx.r[27].s64 = ctx.r[31].s64 + 12996;
	// 823DC35C: 815CFFFC  lwz r10, -4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4 as u32) ) } as u64;
	// 823DC360: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 823DC364: 80DBFFFC  lwz r6, -4(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 823DC368: 80BB0000  lwz r5, 0(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DC36C: 7F185000  cmpw cr6, r24, r10
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[10].s32, &mut ctx.xer);
	// 823DC370: 41990008  bgt cr6, 0x823dc378
	if ctx.cr[6].gt {
	pc = 0x823DC378; continue 'dispatch;
	}
	// 823DC374: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 823DC378: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 823DC37C: 7F174000  cmpw cr6, r23, r8
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[8].s32, &mut ctx.xer);
	// 823DC380: 40990008  ble cr6, 0x823dc388
	if !ctx.cr[6].gt {
	pc = 0x823DC388; continue 'dispatch;
	}
	// 823DC384: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 823DC388: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 823DC38C: 7F164800  cmpw cr6, r22, r9
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[9].s32, &mut ctx.xer);
	// 823DC390: 40980008  bge cr6, 0x823dc398
	if !ctx.cr[6].lt {
	pc = 0x823DC398; continue 'dispatch;
	}
	// 823DC394: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 823DC398: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 823DC39C: 7F155000  cmpw cr6, r21, r10
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[10].s32, &mut ctx.xer);
	// 823DC3A0: 40980008  bge cr6, 0x823dc3a8
	if !ctx.cr[6].lt {
	pc = 0x823DC3A8; continue 'dispatch;
	}
	// 823DC3A4: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 823DC3A8: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 823DC3AC: 4098000C  bge cr6, 0x823dc3b8
	if !ctx.cr[6].lt {
	pc = 0x823DC3B8; continue 'dispatch;
	}
	// 823DC3B0: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 823DC3B4: 41980014  blt cr6, 0x823dc3c8
	if ctx.cr[6].lt {
	pc = 0x823DC3C8; continue 'dispatch;
	}
	// 823DC3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DC3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823DC3C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823DC3C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823DC3C8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 823DC3CC: 54E704BE  clrlwi r7, r7, 0x12
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00003FFFu64;
	// 823DC3D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823DC3D4: 5508809E  rlwinm r8, r8, 0x10, 2, 0xf
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 823DC3D8: 3E80C003  lis r20, -0x3ffd
	ctx.r[20].s64 = -1073545216;
	// 823DC3DC: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 823DC3E0: 62942D01  ori r20, r20, 0x2d01
	ctx.r[20].u64 = ctx.r[20].u64 | 11521;
	// 823DC3E4: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 823DC3E8: 3E600004  lis r19, 4
	ctx.r[19].s64 = 262144;
	// 823DC3EC: 7CC600D0  neg r6, r6
	ctx.r[6].s64 = -ctx.r[6].s64;
	// 823DC3F0: 62730080  ori r19, r19, 0x80
	ctx.r[19].u64 = ctx.r[19].u64 | 128;
	// 823DC3F4: 7CA500D0  neg r5, r5
	ctx.r[5].s64 = -ctx.r[5].s64;
	// 823DC3F8: 54C6047E  clrlwi r6, r6, 0x11
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x00007FFFu64;
	// 823DC3FC: 54A5805E  rlwinm r5, r5, 0x10, 1, 0xf
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 823DC400: 554A809E  rlwinm r10, r10, 0x10, 2, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823DC404: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 823DC408: 552904BE  clrlwi r9, r9, 0x12
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 823DC40C: 7C67E830  slw r7, r3, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[3].u32) << ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 823DC410: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823DC414: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 823DC418: 968B0004  stwu r20, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[20].u32) };
	ctx.r[11].u32 = ea;
	// 823DC41C: 966B0004  stwu r19, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[19].u32) };
	ctx.r[11].u32 = ea;
	// 823DC420: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 823DC424: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823DC428: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC42C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823DC430: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC434: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DC438: 40990010  ble cr6, 0x823dc448
	if !ctx.cr[6].gt {
	pc = 0x823DC448; continue 'dispatch;
	}
	// 823DC43C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823DC440: 4BFFF291  bl 0x823db6d0
	ctx.lr = 0x823DC444;
	sub_823DB6D0(ctx, base);
	// 823DC444: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823DC448: 815F31CC  lwz r10, 0x31cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12748 as u32) ) } as u64;
	// 823DC44C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 823DC450: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 823DC454: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 823DC458: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 823DC45C: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823DC460: 4198FEFC  blt cr6, 0x823dc35c
	if ctx.cr[6].lt {
	pc = 0x823DC35C; continue 'dispatch;
	}
	// 823DC464: 895F2ABF  lbz r10, 0x2abf(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10943 as u32) ) } as u64;
	// 823DC468: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC46C: 41820058  beq 0x823dc4c4
	if ctx.cr[0].eq {
	pc = 0x823DC4C4; continue 'dispatch;
	}
	// 823DC470: 895F2ABC  lbz r10, 0x2abc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 823DC474: 554A0673  rlwinm. r10, r10, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC478: 4182004C  beq 0x823dc4c4
	if ctx.cr[0].eq {
	pc = 0x823DC4C4; continue 'dispatch;
	}
	// 823DC47C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 823DC480: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 823DC484: 3D00C002  lis r8, -0x3ffe
	ctx.r[8].s64 = -1073610752;
	// 823DC488: 3CE00004  lis r7, 4
	ctx.r[7].s64 = 262144;
	// 823DC48C: 61082D01  ori r8, r8, 0x2d01
	ctx.r[8].u64 = ctx.r[8].u64 | 11521;
	// 823DC490: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC494: 60E70081  ori r7, r7, 0x81
	ctx.r[7].u64 = ctx.r[7].u64 | 129;
	// 823DC498: 570604BE  clrlwi r6, r24, 0x12
	ctx.r[6].u64 = ctx.r[24].u32 as u64 & 0x00003FFFu64;
	// 823DC49C: 56EA809E  rlwinm r10, r23, 0x10, 2, 0xf
	ctx.r[10].u64 = ctx.r[23].u32 as u64 & 0x0000FFFFu64;
	// 823DC4A0: 56A5809E  rlwinm r5, r21, 0x10, 2, 0xf
	ctx.r[5].u64 = ctx.r[21].u32 as u64 & 0x0000FFFFu64;
	// 823DC4A4: 7D4A3378  or r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 823DC4A8: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4AC: 56C604BE  clrlwi r6, r22, 0x12
	ctx.r[6].u64 = ctx.r[22].u32 as u64 & 0x00003FFFu64;
	// 823DC4B0: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 823DC4B4: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4B8: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4BC: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4C0: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4C4: 97CB0004  stwu r30, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[30].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4C8: 815F31A4  lwz r10, 0x31a4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12708 as u32) ) } as u64;
	// 823DC4CC: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4D0: 972B0004  stwu r25, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[25].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4D4: 815F31A8  lwz r10, 0x31a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12712 as u32) ) } as u64;
	// 823DC4D8: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 823DC4DC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823DC4E0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823DC4E4: 48158C00  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC4E8 size=28
    let mut pc: u32 = 0x823DC4E8;
    'dispatch: loop {
        match pc {
            0x823DC4E8 => {
    //   block [0x823DC4E8..0x823DC504)
	// 823DC4E8: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DC4EC: 51640038  rlwimi r4, r11, 0, 0, 0x1c
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF8) | (ctx.r[4].u64 & 0xFFFFFFFF00000007);
	// 823DC4F0: 90832948  stw r4, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[4].u32 ) };
	// 823DC4F4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC4F8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 823DC4FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC508 size=12
    let mut pc: u32 = 0x823DC508;
    'dispatch: loop {
        match pc {
            0x823DC508 => {
    //   block [0x823DC508..0x823DC514)
	// 823DC508: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DC50C: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 823DC510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC518 size=28
    let mut pc: u32 = 0x823DC518;
    'dispatch: loop {
        match pc {
            0x823DC518 => {
    //   block [0x823DC518..0x823DC534)
	// 823DC518: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DC51C: 508B1D78  rlwimi r11, r4, 3, 0x15, 0x1c
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x00000000000007F8) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF807);
	// 823DC520: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DC524: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC528: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 823DC52C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC538 size=12
    let mut pc: u32 = 0x823DC538;
    'dispatch: loop {
        match pc {
            0x823DC538 => {
    //   block [0x823DC538..0x823DC544)
	// 823DC538: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DC53C: 5563EE3E  rlwinm r3, r11, 0x1d, 0x18, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 823DC540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC548 size=36
    let mut pc: u32 = 0x823DC548;
    'dispatch: loop {
        match pc {
            0x823DC548 => {
    //   block [0x823DC548..0x823DC56C)
	// 823DC548: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DC54C: 508B1F38  rlwimi r11, r4, 3, 0x1c, 0x1c
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x0000000000000008) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFF7);
	// 823DC550: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 823DC554: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC558: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 823DC55C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC560: 656B0004  oris r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 262144;
	// 823DC564: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC570 size=12
    let mut pc: u32 = 0x823DC570;
    'dispatch: loop {
        match pc {
            0x823DC570 => {
    //   block [0x823DC570..0x823DC57C)
	// 823DC570: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DC574: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 823DC578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC580 size=128
    let mut pc: u32 = 0x823DC580;
    'dispatch: loop {
        match pc {
            0x823DC580 => {
    //   block [0x823DC580..0x823DC600)
	// 823DC580: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC584: 508BF800  rlwimi r11, r4, 0x1f, 0, 0
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(31) as u64) & 0x0000000080000000) | (ctx.r[11].u64 & 0xFFFFFFFF7FFFFFFF);
	// 823DC588: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 823DC58C: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC590: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC594: 40820024  bne 0x823dc5b8
	if !ctx.cr[0].eq {
	pc = 0x823DC5B8; continue 'dispatch;
	}
	// 823DC598: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DC59C: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DC5A0: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC5A4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823DC5A8: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 823DC5AC: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC5B0: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC5B4: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DC5B8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 823DC5BC: 409A000C  bne cr6, 0x823dc5c8
	if !ctx.cr[6].eq {
	pc = 0x823DC5C8; continue 'dispatch;
	}
	// 823DC5C0: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 823DC5C4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC5C8: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC5CC: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC5D0: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC5D4: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC5D8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC5DC: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC5E0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC5E4: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC5E8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC5EC: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC5F0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC5F4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC5F8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC600 size=12
    let mut pc: u32 = 0x823DC600;
    'dispatch: loop {
        match pc {
            0x823DC600 => {
    //   block [0x823DC600..0x823DC60C)
	// 823DC600: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC604: 55630FFE  srwi r3, r11, 0x1f
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823DC608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC610 size=24
    let mut pc: u32 = 0x823DC610;
    'dispatch: loop {
        match pc {
            0x823DC610 => {
    //   block [0x823DC610..0x823DC628)
	// 823DC610: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC614: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC618: 508A2E34  rlwimi r10, r4, 5, 0x18, 0x1a
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(5) as u64) & 0x00000000000000E0) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF1F);
	// 823DC61C: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 823DC620: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC624: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC628 size=100
    let mut pc: u32 = 0x823DC628;
    'dispatch: loop {
        match pc {
            0x823DC628 => {
    //   block [0x823DC628..0x823DC68C)
	// 823DC628: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC62C: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC630: 40820024  bne 0x823dc654
	if !ctx.cr[0].eq {
	pc = 0x823DC654; continue 'dispatch;
	}
	// 823DC634: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DC638: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DC63C: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC640: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823DC644: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 823DC648: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC64C: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC650: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DC654: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC658: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC65C: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC660: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC664: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC668: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC66C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC670: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC674: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC678: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC67C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC680: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC684: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC690 size=12
    let mut pc: u32 = 0x823DC690;
    'dispatch: loop {
        match pc {
            0x823DC690 => {
    //   block [0x823DC690..0x823DC69C)
	// 823DC690: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC694: 5563DF7E  rlwinm r3, r11, 0x1b, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823DC698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC6A0 size=24
    let mut pc: u32 = 0x823DC6A0;
    'dispatch: loop {
        match pc {
            0x823DC6A0 => {
    //   block [0x823DC6A0..0x823DC6B8)
	// 823DC6A0: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC6A4: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC6A8: 508A06FE  rlwimi r10, r4, 0, 0x1b, 0x1f
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x000000000000001F) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFE0);
	// 823DC6AC: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 823DC6B0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC6B4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC6B8 size=100
    let mut pc: u32 = 0x823DC6B8;
    'dispatch: loop {
        match pc {
            0x823DC6B8 => {
    //   block [0x823DC6B8..0x823DC71C)
	// 823DC6B8: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC6BC: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC6C0: 40820024  bne 0x823dc6e4
	if !ctx.cr[0].eq {
	pc = 0x823DC6E4; continue 'dispatch;
	}
	// 823DC6C4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DC6C8: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DC6CC: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC6D0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823DC6D4: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 823DC6D8: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC6DC: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC6E0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DC6E4: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC6E8: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC6EC: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC6F0: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC6F4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC6F8: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC6FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC700: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC704: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC708: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC70C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC710: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC714: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC720 size=12
    let mut pc: u32 = 0x823DC720;
    'dispatch: loop {
        match pc {
            0x823DC720 => {
    //   block [0x823DC720..0x823DC72C)
	// 823DC720: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC724: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823DC728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC730 size=24
    let mut pc: u32 = 0x823DC730;
    'dispatch: loop {
        match pc {
            0x823DC730 => {
    //   block [0x823DC730..0x823DC748)
	// 823DC730: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC734: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC738: 508A44EE  rlwimi r10, r4, 8, 0x13, 0x17
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000001F00) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFE0FF);
	// 823DC73C: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 823DC740: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC744: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC748 size=100
    let mut pc: u32 = 0x823DC748;
    'dispatch: loop {
        match pc {
            0x823DC748 => {
    //   block [0x823DC748..0x823DC7AC)
	// 823DC748: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC74C: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC750: 40820024  bne 0x823dc774
	if !ctx.cr[0].eq {
	pc = 0x823DC774; continue 'dispatch;
	}
	// 823DC754: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DC758: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DC75C: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC760: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823DC764: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 823DC768: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC76C: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC770: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DC774: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC778: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC77C: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC780: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC784: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC788: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC78C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC790: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC794: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC798: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC79C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC7A0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC7A4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC7B0 size=12
    let mut pc: u32 = 0x823DC7B0;
    'dispatch: loop {
        match pc {
            0x823DC7B0 => {
    //   block [0x823DC7B0..0x823DC7BC)
	// 823DC7B0: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC7B4: 5563C6FE  rlwinm r3, r11, 0x18, 0x1b, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823DC7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC7C0 size=24
    let mut pc: u32 = 0x823DC7C0;
    'dispatch: loop {
        match pc {
            0x823DC7C0 => {
    //   block [0x823DC7C0..0x823DC7D8)
	// 823DC7C0: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC7C4: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC7C8: 508AAA14  rlwimi r10, r4, 0x15, 8, 0xa
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(21) as u64) & 0x0000000000E00000) | (ctx.r[10].u64 & 0xFFFFFFFFFF1FFFFF);
	// 823DC7CC: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 823DC7D0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC7D4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC7D8 size=8
    let mut pc: u32 = 0x823DC7D8;
    'dispatch: loop {
        match pc {
            0x823DC7D8 => {
    //   block [0x823DC7D8..0x823DC7E0)
	// 823DC7D8: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DC7DC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC7E0 size=60
    let mut pc: u32 = 0x823DC7E0;
    'dispatch: loop {
        match pc {
            0x823DC7E0 => {
    //   block [0x823DC7E0..0x823DC81C)
	// 823DC7E0: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC7E4: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC7E8: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC7EC: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC7F0: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC7F4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC7F8: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC7FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC800: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC804: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC808: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC80C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC810: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC814: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC820 size=12
    let mut pc: u32 = 0x823DC820;
    'dispatch: loop {
        match pc {
            0x823DC820 => {
    //   block [0x823DC820..0x823DC82C)
	// 823DC820: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC824: 55635F7E  rlwinm r3, r11, 0xb, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 823DC828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC830 size=24
    let mut pc: u32 = 0x823DC830;
    'dispatch: loop {
        match pc {
            0x823DC830 => {
    //   block [0x823DC830..0x823DC848)
	// 823DC830: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC834: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC838: 508A82DE  rlwimi r10, r4, 0x10, 0xb, 0xf
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000001F0000) | (ctx.r[10].u64 & 0xFFFFFFFFFFE0FFFF);
	// 823DC83C: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 823DC840: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC844: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC848 size=8
    let mut pc: u32 = 0x823DC848;
    'dispatch: loop {
        match pc {
            0x823DC848 => {
    //   block [0x823DC848..0x823DC850)
	// 823DC848: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DC84C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC850 size=60
    let mut pc: u32 = 0x823DC850;
    'dispatch: loop {
        match pc {
            0x823DC850 => {
    //   block [0x823DC850..0x823DC88C)
	// 823DC850: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC854: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC858: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC85C: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC860: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC864: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC868: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC86C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC870: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC874: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC878: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC87C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC880: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC884: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC890 size=12
    let mut pc: u32 = 0x823DC890;
    'dispatch: loop {
        match pc {
            0x823DC890 => {
    //   block [0x823DC890..0x823DC89C)
	// 823DC890: A1632E48  lhz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC894: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823DC898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC8A0 size=24
    let mut pc: u32 = 0x823DC8A0;
    'dispatch: loop {
        match pc {
            0x823DC8A0 => {
    //   block [0x823DC8A0..0x823DC8B8)
	// 823DC8A0: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC8A4: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC8A8: 508AC0CE  rlwimi r10, r4, 0x18, 3, 7
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(24) as u64) & 0x000000001F000000) | (ctx.r[10].u64 & 0xFFFFFFFFE0FFFFFF);
	// 823DC8AC: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 823DC8B0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC8B4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC8B8 size=8
    let mut pc: u32 = 0x823DC8B8;
    'dispatch: loop {
        match pc {
            0x823DC8B8 => {
    //   block [0x823DC8B8..0x823DC8C0)
	// 823DC8B8: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DC8BC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC8C0 size=60
    let mut pc: u32 = 0x823DC8C0;
    'dispatch: loop {
        match pc {
            0x823DC8C0 => {
    //   block [0x823DC8C0..0x823DC8FC)
	// 823DC8C0: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC8C4: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC8C8: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC8CC: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC8D0: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC8D4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC8D8: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC8DC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC8E0: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC8E4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC8E8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC8EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC8F0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC8F4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC900 size=12
    let mut pc: u32 = 0x823DC900;
    'dispatch: loop {
        match pc {
            0x823DC900 => {
    //   block [0x823DC900..0x823DC90C)
	// 823DC900: 89632E48  lbz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC904: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823DC908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC910 size=132
    let mut pc: u32 = 0x823DC910;
    'dispatch: loop {
        match pc {
            0x823DC910 => {
    //   block [0x823DC910..0x823DC994)
	// 823DC910: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC914: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 823DC918: 508BF042  rlwimi r11, r4, 0x1e, 1, 1
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(30) as u64) & 0x0000000040000000) | (ctx.r[11].u64 & 0xFFFFFFFFBFFFFFFF);
	// 823DC91C: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DC920: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 823DC924: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 823DC928: 409A0024  bne cr6, 0x823dc94c
	if !ctx.cr[6].eq {
	pc = 0x823DC94C; continue 'dispatch;
	}
	// 823DC92C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DC930: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DC934: 71691010  andi. r9, r11, 0x1010
	ctx.r[9].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823DC938: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 823DC93C: 5529601E  rlwinm r9, r9, 0xc, 0, 0xf
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000FFFFFu64;
	// 823DC940: 55290314  rlwinm r9, r9, 0, 0xc, 0xa
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC944: 55290104  rlwinm r9, r9, 0, 4, 2
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 823DC948: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 823DC94C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823DC950: 409A000C  bne cr6, 0x823dc95c
	if !ctx.cr[6].eq {
	pc = 0x823DC95C; continue 'dispatch;
	}
	// 823DC954: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 823DC958: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC95C: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 823DC960: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 823DC964: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 823DC968: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 823DC96C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC970: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 823DC974: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC978: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 823DC97C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC980: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 823DC984: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC988: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DC98C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DC998 size=12
    let mut pc: u32 = 0x823DC998;
    'dispatch: loop {
        match pc {
            0x823DC998 => {
    //   block [0x823DC998..0x823DC9A4)
	// 823DC998: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 823DC99C: 556317FE  rlwinm r3, r11, 2, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 823DC9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DC9A8 size=52
    let mut pc: u32 = 0x823DC9A8;
    'dispatch: loop {
        match pc {
            0x823DC9A8 => {
    //   block [0x823DC9A8..0x823DC9DC)
	// 823DC9A8: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 823DC9AC: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 823DC9B0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DC9B4: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DC9B8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 823DC9BC: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 823DC9C0: C00B2778  lfs f0, 0x2778(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DC9C4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DC9C8: D0032904  stfs f0, 0x2904(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10500 as u32), tmp.u32 ) };
	// 823DC9CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DC9D0: 656B0800  oris r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 134217728;
	// 823DC9D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DC9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DC9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DC9E0 size=44
    let mut pc: u32 = 0x823DC9E0;
    'dispatch: loop {
        match pc {
            0x823DC9E0 => {
    //   block [0x823DC9E0..0x823DCA0C)
	// 823DC9E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DC9E4: C1832904  lfs f12, 0x2904(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10500 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823DC9E8: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 823DC9EC: C00B20A4  lfs f0, 0x20a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DC9F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DC9F4: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DC9F8: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 823DC9FC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 823DCA00: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 823DCA04: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCA10 size=28
    let mut pc: u32 = 0x823DCA10;
    'dispatch: loop {
        match pc {
            0x823DCA10 => {
    //   block [0x823DCA10..0x823DCA2C)
	// 823DCA10: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DCA14: 51640038  rlwimi r4, r11, 0, 0, 0x1c
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF8) | (ctx.r[4].u64 & 0xFFFFFFFF00000007);
	// 823DCA18: 9083293C  stw r4, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[4].u32 ) };
	// 823DCA1C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCA20: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 823DCA24: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCA30 size=12
    let mut pc: u32 = 0x823DCA30;
    'dispatch: loop {
        match pc {
            0x823DCA30 => {
    //   block [0x823DCA30..0x823DCA3C)
	// 823DCA30: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DCA34: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 823DCA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DCA40 size=156
    let mut pc: u32 = 0x823DCA40;
    'dispatch: loop {
        match pc {
            0x823DCA40 => {
    //   block [0x823DCA40..0x823DCADC)
	// 823DCA40: 548AC63E  rlwinm r10, r4, 0x18, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 823DCA44: 548B863E  rlwinm r11, r4, 0x10, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 823DCA48: 5488463E  srwi r8, r4, 0x18
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823DCA4C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 823DCA50: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 823DCA54: 78890620  clrldi r9, r4, 0x38
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000000000FFu64;
	// 823DCA58: 79080020  clrldi r8, r8, 0x20
	ctx.r[8].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 823DCA5C: 3980000F  li r12, 0xf
	ctx.r[12].s64 = 15;
	// 823DCA60: F941FFE8  std r10, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[10].u64 ) };
	// 823DCA64: F961FFE0  std r11, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u64 ) };
	// 823DCA68: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DCA6C: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 823DCA70: 798C0F86  sldi r12, r12, 0x21
	ctx.r[12].u64 = ctx.r[12].u64.wrapping_shl(33);
	ctx.r[12].u32 = ctx.r[12].u64 as u32;
	// 823DCA74: F901FFF8  std r8, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[8].u64 ) };
	// 823DCA78: C9A1FFE8  lfd f13, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823DCA7C: C801FFE0  lfd f0, -0x20(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823DCA80: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 823DCA84: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DCA88: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 823DCA8C: C961FFF8  lfd f11, -8(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 823DCA90: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 823DCA94: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 823DCA98: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 823DCA9C: FD400018  frsp f10, f0
	ctx.f[10].f64 = (ctx.f[0].f64 as f32) as f64;
	// 823DCAA0: C00B2778  lfs f0, 0x2778(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DCAA4: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 823DCAA8: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 823DCAAC: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DCAB0: D1A328E4  stfs f13, 0x28e4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10468 as u32), tmp.u32 ) };
	// 823DCAB4: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DCAB8: D14328E0  stfs f10, 0x28e0(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10464 as u32), tmp.u32 ) };
	// 823DCABC: EDAC0032  fmuls f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DCAC0: D1A328E8  stfs f13, 0x28e8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10472 as u32), tmp.u32 ) };
	// 823DCAC4: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DCAC8: D00328EC  stfs f0, 0x28ec(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10476 as u32), tmp.u32 ) };
	// 823DCACC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCAD0: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DCAD4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DCAE0 size=128
    let mut pc: u32 = 0x823DCAE0;
    'dispatch: loop {
        match pc {
            0x823DCAE0 => {
    //   block [0x823DCAE0..0x823DCB60)
	// 823DCAE0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DCAE4: C18328E0  lfs f12, 0x28e0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10464 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823DCAE8: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 823DCAEC: C16328EC  lfs f11, 0x28ec(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10476 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823DCAF0: C14328E4  lfs f10, 0x28e4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10468 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 823DCAF4: C12328E8  lfs f9, 0x28e8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10472 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 823DCAF8: C00B20A4  lfs f0, 0x20a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DCAFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DCB00: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DCB04: ED8C683A  fmadds f12, f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 823DCB08: ED4A683A  fmadds f10, f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 823DCB0C: ED6B683A  fmadds f11, f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 823DCB10: EC09683A  fmadds f0, f9, f0, f13
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 823DCB14: FDA0665E  fctidz f13, f12
	ctx.f[13].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 823DCB18: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 823DCB1C: FD80565E  fctidz f12, f10
	ctx.f[12].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 823DCB20: FDA05E5E  fctidz f13, f11
	ctx.f[13].s64 = if ctx.f[11].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[11].f64.trunc() as i64 };
	// 823DCB24: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 823DCB28: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCB2C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 823DCB30: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 823DCB34: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCB38: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 823DCB3C: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 823DCB40: 7D804FAE  stfiwx f12, 0, r9
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 823DCB44: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCB48: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 823DCB4C: 516A402E  rlwimi r10, r11, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 823DCB50: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 823DCB54: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCB58: 5143402E  rlwimi r3, r10, 8, 0, 0x17
	ctx.r[3].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[3].u64 & 0xFFFFFFFF000000FF);
	// 823DCB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCB60 size=36
    let mut pc: u32 = 0x823DCB60;
    'dispatch: loop {
        match pc {
            0x823DCB60 => {
    //   block [0x823DCB60..0x823DCB84)
	// 823DCB60: 816329B8  lwz r11, 0x29b8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10680 as u32) ) } as u64;
	// 823DCB64: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DCB68: 508B556A  rlwimi r11, r4, 0xa, 0x15, 0x15
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(10) as u64) & 0x0000000000000400) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFBFF);
	// 823DCB6C: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 823DCB70: 916329B8  stw r11, 0x29b8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10680 as u32), ctx.r[11].u32 ) };
	// 823DCB74: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 823DCB78: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DCB7C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DCB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCB88 size=12
    let mut pc: u32 = 0x823DCB88;
    'dispatch: loop {
        match pc {
            0x823DCB88 => {
    //   block [0x823DCB88..0x823DCB94)
	// 823DCB88: 816329B8  lwz r11, 0x29b8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10680 as u32) ) } as u64;
	// 823DCB8C: 5563B7FE  rlwinm r3, r11, 0x16, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 823DCB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DCB98 size=64
    let mut pc: u32 = 0x823DCB98;
    'dispatch: loop {
        match pc {
            0x823DCB98 => {
    //   block [0x823DCB98..0x823DCBD8)
	// 823DCB98: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DCB9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DCBA0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DCBA4: 798CA7E6  rldicr r12, r12, 0x34, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(52) & 0xFFFFFFFFFFFFFFFF;
	// 823DCBA8: C00B1848  lfs f0, 0x1848(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DCBAC: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 823DCBB0: C1A1001C  lfs f13, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DCBB4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DCBB8: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 823DCBBC: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 823DCBC0: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCBC4: B163296E  sth r11, 0x296e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10606 as u32), ctx.r[11].u16 ) };
	// 823DCBC8: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DCBCC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DCBD0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DCBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DCBD8 size=48
    let mut pc: u32 = 0x823DCBD8;
    'dispatch: loop {
        match pc {
            0x823DCBD8 => {
    //   block [0x823DCBD8..0x823DCC08)
	// 823DCBD8: A163296E  lhz r11, 0x296e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10606 as u32) ) } as u64;
	// 823DCBDC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 823DCBE0: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 823DCBE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DCBE8: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823DCBEC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 823DCBF0: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 823DCBF4: C00B2144  lfs f0, 0x2144(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DCBF8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DCBFC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DCC00: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DCC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC08 size=20
    let mut pc: u32 = 0x823DCC08;
    'dispatch: loop {
        match pc {
            0x823DCC08 => {
    //   block [0x823DCC08..0x823DCC1C)
	// 823DCC08: 8163571C  lwz r11, 0x571c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(22300 as u32) ) } as u64;
	// 823DCC0C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 823DCC10: 419A000C  beq cr6, 0x823dcc1c
	if ctx.cr[6].eq {
		sub_823DCC1C(ctx, base);
		return;
	}
	// 823DCC14: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823DCC18: 48000008  b 0x823dcc20
	sub_823DCC1C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC1C size=12
    let mut pc: u32 = 0x823DCC1C;
    'dispatch: loop {
        match pc {
            0x823DCC1C => {
    //   block [0x823DCC1C..0x823DCC28)
	// 823DCC1C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DCC20: 9163571C  stw r11, 0x571c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(22300 as u32), ctx.r[11].u32 ) };
	// 823DCC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC28 size=12
    let mut pc: u32 = 0x823DCC28;
    'dispatch: loop {
        match pc {
            0x823DCC28 => {
    //   block [0x823DCC28..0x823DCC34)
	// 823DCC28: 8163571C  lwz r11, 0x571c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(22300 as u32) ) } as u64;
	// 823DCC2C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 823DCC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC38 size=56
    let mut pc: u32 = 0x823DCC38;
    'dispatch: loop {
        match pc {
            0x823DCC38 => {
    //   block [0x823DCC38..0x823DCC70)
	// 823DCC38: 816330A8  lwz r11, 0x30a8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12456 as u32) ) } as u64;
	// 823DCC3C: 90832E64  stw r4, 0x2e64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11876 as u32), ctx.r[4].u32 ) };
	// 823DCC40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DCC44: 409A0008  bne cr6, 0x823dcc4c
	if !ctx.cr[6].eq {
	pc = 0x823DCC4C; continue 'dispatch;
	}
	// 823DCC48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DCC4C: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCC50: 508B0FBC  rlwimi r11, r4, 1, 0x1e, 0x1e
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(1) as u64) & 0x0000000000000002) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFD);
	// 823DCC54: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCC58: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCC5C: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCC60: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCC64: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCC68: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC70 size=8
    let mut pc: u32 = 0x823DCC70;
    'dispatch: loop {
        match pc {
            0x823DCC70 => {
    //   block [0x823DCC70..0x823DCC78)
	// 823DCC70: 80632E64  lwz r3, 0x2e64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11876 as u32) ) } as u64;
	// 823DCC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC78 size=28
    let mut pc: u32 = 0x823DCC78;
    'dispatch: loop {
        match pc {
            0x823DCC78 => {
    //   block [0x823DCC78..0x823DCC94)
	// 823DCC78: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCC7C: 508B177A  rlwimi r11, r4, 2, 0x1d, 0x1d
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(2) as u64) & 0x0000000000000004) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFB);
	// 823DCC80: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCC84: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCC88: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCC8C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCC98 size=12
    let mut pc: u32 = 0x823DCC98;
    'dispatch: loop {
        match pc {
            0x823DCC98 => {
    //   block [0x823DCC98..0x823DCCA4)
	// 823DCC98: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCC9C: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823DCCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCCA8 size=36
    let mut pc: u32 = 0x823DCCA8;
    'dispatch: loop {
        match pc {
            0x823DCCA8 => {
    //   block [0x823DCCA8..0x823DCCCC)
	// 823DCCA8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCCAC: 508B2676  rlwimi r11, r4, 4, 0x19, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x0000000000000070) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF8F);
	// 823DCCB0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCCB4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCCB8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCCBC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCCC0: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCCC4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCCD0 size=12
    let mut pc: u32 = 0x823DCCD0;
    'dispatch: loop {
        match pc {
            0x823DCCD0 => {
    //   block [0x823DCCD0..0x823DCCDC)
	// 823DCCD0: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCCD4: 5563E77E  rlwinm r3, r11, 0x1c, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DCCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCCE0 size=56
    let mut pc: u32 = 0x823DCCE0;
    'dispatch: loop {
        match pc {
            0x823DCCE0 => {
    //   block [0x823DCCE0..0x823DCD18)
	// 823DCCE0: 816330A8  lwz r11, 0x30a8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12456 as u32) ) } as u64;
	// 823DCCE4: 90832E68  stw r4, 0x2e68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11880 as u32), ctx.r[4].u32 ) };
	// 823DCCE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DCCEC: 409A0008  bne cr6, 0x823dccf4
	if !ctx.cr[6].eq {
	pc = 0x823DCCF4; continue 'dispatch;
	}
	// 823DCCF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DCCF4: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCCF8: 508B07FE  rlwimi r11, r4, 0, 0x1f, 0x1f
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x0000000000000001) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFE);
	// 823DCCFC: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCD00: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCD04: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCD08: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCD0C: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCD10: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCD18 size=8
    let mut pc: u32 = 0x823DCD18;
    'dispatch: loop {
        match pc {
            0x823DCD18 => {
    //   block [0x823DCD18..0x823DCD20)
	// 823DCD18: 80632E68  lwz r3, 0x2e68(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11880 as u32) ) } as u64;
	// 823DCD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCD20 size=36
    let mut pc: u32 = 0x823DCD20;
    'dispatch: loop {
        match pc {
            0x823DCD20 => {
    //   block [0x823DCD20..0x823DCD44)
	// 823DCD20: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCD24: 508B3E30  rlwimi r11, r4, 7, 0x18, 0x18
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(7) as u64) & 0x0000000000000080) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF7F);
	// 823DCD28: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCD2C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCD30: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCD34: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCD38: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCD3C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCD48 size=12
    let mut pc: u32 = 0x823DCD48;
    'dispatch: loop {
        match pc {
            0x823DCD48 => {
    //   block [0x823DCD48..0x823DCD54)
	// 823DCD48: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCD4C: 5563CFFE  rlwinm r3, r11, 0x19, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 823DCD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCD58 size=28
    let mut pc: u32 = 0x823DCD58;
    'dispatch: loop {
        match pc {
            0x823DCD58 => {
    //   block [0x823DCD58..0x823DCD74)
	// 823DCD58: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCD5C: 508B456E  rlwimi r11, r4, 8, 0x15, 0x17
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000000700) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF8FF);
	// 823DCD60: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCD64: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCD68: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCD6C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCD78 size=12
    let mut pc: u32 = 0x823DCD78;
    'dispatch: loop {
        match pc {
            0x823DCD78 => {
    //   block [0x823DCD78..0x823DCD84)
	// 823DCD78: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCD7C: 5563C77E  rlwinm r3, r11, 0x18, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823DCD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCD88 size=36
    let mut pc: u32 = 0x823DCD88;
    'dispatch: loop {
        match pc {
            0x823DCD88 => {
    //   block [0x823DCD88..0x823DCDAC)
	// 823DCD88: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCD8C: 508B5CA8  rlwimi r11, r4, 0xb, 0x12, 0x14
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(11) as u64) & 0x0000000000003800) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFC7FF);
	// 823DCD90: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCD94: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCD98: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCD9C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCDA0: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCDA4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCDB0 size=12
    let mut pc: u32 = 0x823DCDB0;
    'dispatch: loop {
        match pc {
            0x823DCDB0 => {
    //   block [0x823DCDB0..0x823DCDBC)
	// 823DCDB0: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCDB4: 5563AF7E  rlwinm r3, r11, 0x15, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 823DCDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCDC0 size=36
    let mut pc: u32 = 0x823DCDC0;
    'dispatch: loop {
        match pc {
            0x823DCDC0 => {
    //   block [0x823DCDC0..0x823DCDE4)
	// 823DCDC0: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCDC4: 508B8B1C  rlwimi r11, r4, 0x11, 0xc, 0xe
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(17) as u64) & 0x00000000000E0000) | (ctx.r[11].u64 & 0xFFFFFFFFFFF1FFFF);
	// 823DCDC8: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCDCC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCDD0: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCDD4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCDD8: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCDDC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCDE8 size=12
    let mut pc: u32 = 0x823DCDE8;
    'dispatch: loop {
        match pc {
            0x823DCDE8 => {
    //   block [0x823DCDE8..0x823DCDF4)
	// 823DCDE8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCDEC: 55637F7E  rlwinm r3, r11, 0xf, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 823DCDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCDF8 size=28
    let mut pc: u32 = 0x823DCDF8;
    'dispatch: loop {
        match pc {
            0x823DCDF8 => {
    //   block [0x823DCDF8..0x823DCE14)
	// 823DCDF8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCDFC: 508B73E2  rlwimi r11, r4, 0xe, 0xf, 0x11
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(14) as u64) & 0x000000000001C000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFE3FFF);
	// 823DCE00: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCE04: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCE08: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCE0C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCE18 size=12
    let mut pc: u32 = 0x823DCE18;
    'dispatch: loop {
        match pc {
            0x823DCE18 => {
    //   block [0x823DCE18..0x823DCE24)
	// 823DCE18: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCE1C: 5563977E  rlwinm r3, r11, 0x12, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 823DCE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCE28 size=28
    let mut pc: u32 = 0x823DCE28;
    'dispatch: loop {
        match pc {
            0x823DCE28 => {
    //   block [0x823DCE28..0x823DCE44)
	// 823DCE28: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCE2C: 508BA256  rlwimi r11, r4, 0x14, 9, 0xb
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(20) as u64) & 0x0000000000700000) | (ctx.r[11].u64 & 0xFFFFFFFFFF8FFFFF);
	// 823DCE30: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCE34: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCE38: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCE3C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCE48 size=12
    let mut pc: u32 = 0x823DCE48;
    'dispatch: loop {
        match pc {
            0x823DCE48 => {
    //   block [0x823DCE48..0x823DCE54)
	// 823DCE48: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCE4C: 5563677E  rlwinm r3, r11, 0xc, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 823DCE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCE58 size=36
    let mut pc: u32 = 0x823DCE58;
    'dispatch: loop {
        match pc {
            0x823DCE58 => {
    //   block [0x823DCE58..0x823DCE7C)
	// 823DCE58: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCE5C: 508BB990  rlwimi r11, r4, 0x17, 6, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x0000000003800000) | (ctx.r[11].u64 & 0xFFFFFFFFFC7FFFFF);
	// 823DCE60: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCE64: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCE68: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCE6C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCE70: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCE74: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCE80 size=12
    let mut pc: u32 = 0x823DCE80;
    'dispatch: loop {
        match pc {
            0x823DCE80 => {
    //   block [0x823DCE80..0x823DCE8C)
	// 823DCE80: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCE84: 55634F7E  rlwinm r3, r11, 9, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 823DCE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCE90 size=36
    let mut pc: u32 = 0x823DCE90;
    'dispatch: loop {
        match pc {
            0x823DCE90 => {
    //   block [0x823DCE90..0x823DCEB4)
	// 823DCE90: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCE94: 508BE804  rlwimi r11, r4, 0x1d, 0, 2
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(29) as u64) & 0x00000000E0000000) | (ctx.r[11].u64 & 0xFFFFFFFF1FFFFFFF);
	// 823DCE98: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCE9C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCEA0: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCEA4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCEA8: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 823DCEAC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCEB8 size=12
    let mut pc: u32 = 0x823DCEB8;
    'dispatch: loop {
        match pc {
            0x823DCEB8 => {
    //   block [0x823DCEB8..0x823DCEC4)
	// 823DCEB8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCEBC: 55631F7E  srwi r3, r11, 0x1d
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823DCEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCEC8 size=28
    let mut pc: u32 = 0x823DCEC8;
    'dispatch: loop {
        match pc {
            0x823DCEC8 => {
    //   block [0x823DCEC8..0x823DCEE4)
	// 823DCEC8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCECC: 508BD0CA  rlwimi r11, r4, 0x1a, 3, 5
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(26) as u64) & 0x000000001C000000) | (ctx.r[11].u64 & 0xFFFFFFFFE3FFFFFF);
	// 823DCED0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 823DCED4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCED8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 823DCEDC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCEE8 size=12
    let mut pc: u32 = 0x823DCEE8;
    'dispatch: loop {
        match pc {
            0x823DCEE8 => {
    //   block [0x823DCEE8..0x823DCEF4)
	// 823DCEE8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 823DCEEC: 5563377E  rlwinm r3, r11, 6, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 823DCEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCEF8 size=20
    let mut pc: u32 = 0x823DCEF8;
    'dispatch: loop {
        match pc {
            0x823DCEF8 => {
    //   block [0x823DCEF8..0x823DCF0C)
	// 823DCEF8: 98832903  stb r4, 0x2903(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10499 as u32), ctx.r[4].u8 ) };
	// 823DCEFC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCF00: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 823DCF04: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF10 size=8
    let mut pc: u32 = 0x823DCF10;
    'dispatch: loop {
        match pc {
            0x823DCF10 => {
    //   block [0x823DCF10..0x823DCF18)
	// 823DCF10: 88632903  lbz r3, 0x2903(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10499 as u32) ) } as u64;
	// 823DCF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF18 size=20
    let mut pc: u32 = 0x823DCF18;
    'dispatch: loop {
        match pc {
            0x823DCF18 => {
    //   block [0x823DCF18..0x823DCF2C)
	// 823DCF18: 98832902  stb r4, 0x2902(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10498 as u32), ctx.r[4].u8 ) };
	// 823DCF1C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCF20: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 823DCF24: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF30 size=8
    let mut pc: u32 = 0x823DCF30;
    'dispatch: loop {
        match pc {
            0x823DCF30 => {
    //   block [0x823DCF30..0x823DCF38)
	// 823DCF30: 88632902  lbz r3, 0x2902(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10498 as u32) ) } as u64;
	// 823DCF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF38 size=20
    let mut pc: u32 = 0x823DCF38;
    'dispatch: loop {
        match pc {
            0x823DCF38 => {
    //   block [0x823DCF38..0x823DCF4C)
	// 823DCF38: 98832901  stb r4, 0x2901(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10497 as u32), ctx.r[4].u8 ) };
	// 823DCF3C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCF40: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 823DCF44: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF50 size=8
    let mut pc: u32 = 0x823DCF50;
    'dispatch: loop {
        match pc {
            0x823DCF50 => {
    //   block [0x823DCF50..0x823DCF58)
	// 823DCF50: 88632901  lbz r3, 0x2901(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10497 as u32) ) } as u64;
	// 823DCF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF58 size=20
    let mut pc: u32 = 0x823DCF58;
    'dispatch: loop {
        match pc {
            0x823DCF58 => {
    //   block [0x823DCF58..0x823DCF6C)
	// 823DCF58: 988328FF  stb r4, 0x28ff(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10495 as u32), ctx.r[4].u8 ) };
	// 823DCF5C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCF60: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 823DCF64: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF70 size=8
    let mut pc: u32 = 0x823DCF70;
    'dispatch: loop {
        match pc {
            0x823DCF70 => {
    //   block [0x823DCF70..0x823DCF78)
	// 823DCF70: 886328FF  lbz r3, 0x28ff(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10495 as u32) ) } as u64;
	// 823DCF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF78 size=20
    let mut pc: u32 = 0x823DCF78;
    'dispatch: loop {
        match pc {
            0x823DCF78 => {
    //   block [0x823DCF78..0x823DCF8C)
	// 823DCF78: 988328FE  stb r4, 0x28fe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10494 as u32), ctx.r[4].u8 ) };
	// 823DCF7C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCF80: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 823DCF84: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF90 size=8
    let mut pc: u32 = 0x823DCF90;
    'dispatch: loop {
        match pc {
            0x823DCF90 => {
    //   block [0x823DCF90..0x823DCF98)
	// 823DCF90: 886328FE  lbz r3, 0x28fe(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10494 as u32) ) } as u64;
	// 823DCF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCF98 size=20
    let mut pc: u32 = 0x823DCF98;
    'dispatch: loop {
        match pc {
            0x823DCF98 => {
    //   block [0x823DCF98..0x823DCFAC)
	// 823DCF98: 988328FD  stb r4, 0x28fd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10493 as u32), ctx.r[4].u8 ) };
	// 823DCF9C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCFA0: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 823DCFA4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCFB0 size=8
    let mut pc: u32 = 0x823DCFB0;
    'dispatch: loop {
        match pc {
            0x823DCFB0 => {
    //   block [0x823DCFB0..0x823DCFB8)
	// 823DCFB0: 886328FD  lbz r3, 0x28fd(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10493 as u32) ) } as u64;
	// 823DCFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DCFB8 size=64
    let mut pc: u32 = 0x823DCFB8;
    'dispatch: loop {
        match pc {
            0x823DCFB8 => {
    //   block [0x823DCFB8..0x823DCFF8)
	// 823DCFB8: 81632944  lwz r11, 0x2944(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 823DCFBC: 21440000  subfic r10, r4, 0
	ctx.xer.ca = ctx.r[4].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[4].s64;
	// 823DCFC0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DCFC4: 556B0032  rlwinm r11, r11, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DCFC8: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 823DCFCC: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 823DCFD0: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DCFD4: 798C67E6  rldicr r12, r12, 0x2c, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(44) & 0xFFFFFFFFFFFFFFFF;
	// 823DCFD8: 91632944  stw r11, 0x2944(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10564 as u32), ctx.r[11].u32 ) };
	// 823DCFDC: 914328B4  stw r10, 0x28b4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10420 as u32), ctx.r[10].u32 ) };
	// 823DCFE0: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DCFE4: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 823DCFE8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCFEC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DCFF0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DCFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DCFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DCFF8 size=12
    let mut pc: u32 = 0x823DCFF8;
    'dispatch: loop {
        match pc {
            0x823DCFF8 => {
    //   block [0x823DCFF8..0x823DD004)
	// 823DCFF8: 81632944  lwz r11, 0x2944(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 823DCFFC: 556306BE  clrlwi r3, r11, 0x1a
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 823DD000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD008 size=8
    let mut pc: u32 = 0x823DD008;
    'dispatch: loop {
        match pc {
            0x823DD008 => {
    //   block [0x823DD008..0x823DD010)
	// 823DD008: 80632E50  lwz r3, 0x2e50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11856 as u32) ) } as u64;
	// 823DD00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD010 size=164
    let mut pc: u32 = 0x823DD010;
    'dispatch: loop {
        match pc {
            0x823DD010 => {
    //   block [0x823DD010..0x823DD0B4)
	// 823DD010: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DD014: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DD018: C00B184C  lfs f0, 0x184c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD01C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DD020: C1A1001C  lfs f13, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD024: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DD028: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD02C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823DD030: 409A0014  bne cr6, 0x823dd044
	if !ctx.cr[6].eq {
	pc = 0x823DD044; continue 'dispatch;
	}
	// 823DD034: C1832A54  lfs f12, 0x2a54(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10836 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823DD038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DD03C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 823DD040: 419A0008  beq cr6, 0x823dd048
	if ctx.cr[6].eq {
	pc = 0x823DD048; continue 'dispatch;
	}
	// 823DD044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DD048: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DD04C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823DD050: 514B5D28  rlwimi r11, r10, 0xb, 0x14, 0x14
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF7FF);
	// 823DD054: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DD058: 409A0014  bne cr6, 0x823dd06c
	if !ctx.cr[6].eq {
	pc = 0x823DD06C; continue 'dispatch;
	}
	// 823DD05C: C1832A5C  lfs f12, 0x2a5c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10844 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823DD060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DD064: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 823DD068: 419A0008  beq cr6, 0x823dd070
	if ctx.cr[6].eq {
	pc = 0x823DD070; continue 'dispatch;
	}
	// 823DD06C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DD070: 514B64E6  rlwimi r11, r10, 0xc, 0x13, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x0000000000001000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFEFFF);
	// 823DD074: D1A32A50  stfs f13, 0x2a50(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10832 as u32), tmp.u32 ) };
	// 823DD078: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD07C: D1A32A58  stfs f13, 0x2a58(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10840 as u32), tmp.u32 ) };
	// 823DD080: 798C6FE6  rldicr r12, r12, 0x2d, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(45) & 0xFFFFFFFFFFFFFFFF;
	// 823DD084: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DD088: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 823DD08C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD090: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD094: 798C5FE6  rldicr r12, r12, 0x2b, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(43) & 0xFFFFFFFFFFFFFFFF;
	// 823DD098: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DD09C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD0A0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DD0A4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD0A8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 823DD0AC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD0B8 size=28
    let mut pc: u32 = 0x823DD0B8;
    'dispatch: loop {
        match pc {
            0x823DD0B8 => {
    //   block [0x823DD0B8..0x823DD0D4)
	// 823DD0B8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DD0BC: C1A32A50  lfs f13, 0x2a50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10832 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD0C0: C00B20A0  lfs f0, 0x20a0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD0C4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823DD0C8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DD0CC: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD0D8 size=152
    let mut pc: u32 = 0x823DD0D8;
    'dispatch: loop {
        match pc {
            0x823DD0D8 => {
    //   block [0x823DD0D8..0x823DD170)
	// 823DD0D8: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DD0DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 823DD0E0: C1A32A50  lfs f13, 0x2a50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10832 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD0E4: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD0E8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823DD0EC: C1A1001C  lfs f13, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD0F0: 409A0010  bne cr6, 0x823dd100
	if !ctx.cr[6].eq {
	pc = 0x823DD100; continue 'dispatch;
	}
	// 823DD0F4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823DD0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DD0FC: 419A0008  beq cr6, 0x823dd104
	if ctx.cr[6].eq {
	pc = 0x823DD104; continue 'dispatch;
	}
	// 823DD100: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DD104: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DD108: C1832A58  lfs f12, 0x2a58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10840 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823DD10C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 823DD110: 514B5D28  rlwimi r11, r10, 0xb, 0x14, 0x14
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF7FF);
	// 823DD114: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DD118: 409A0010  bne cr6, 0x823dd128
	if !ctx.cr[6].eq {
	pc = 0x823DD128; continue 'dispatch;
	}
	// 823DD11C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823DD120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823DD124: 419A0008  beq cr6, 0x823dd12c
	if ctx.cr[6].eq {
	pc = 0x823DD12C; continue 'dispatch;
	}
	// 823DD128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823DD12C: 514B64E6  rlwimi r11, r10, 0xc, 0x13, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x0000000000001000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFEFFF);
	// 823DD130: D1A32A54  stfs f13, 0x2a54(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10836 as u32), tmp.u32 ) };
	// 823DD134: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD138: D1A32A5C  stfs f13, 0x2a5c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10844 as u32), tmp.u32 ) };
	// 823DD13C: 798C67E6  rldicr r12, r12, 0x2c, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(44) & 0xFFFFFFFFFFFFFFFF;
	// 823DD140: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DD144: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 823DD148: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD14C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD150: 798C57E6  rldicr r12, r12, 0x2a, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(42) & 0xFFFFFFFFFFFFFFFF;
	// 823DD154: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DD158: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD15C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DD160: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD164: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 823DD168: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD170 size=16
    let mut pc: u32 = 0x823DD170;
    'dispatch: loop {
        match pc {
            0x823DD170 => {
    //   block [0x823DD170..0x823DD180)
	// 823DD170: C0032A54  lfs f0, 0x2a54(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10836 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD174: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DD178: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD180 size=28
    let mut pc: u32 = 0x823DD180;
    'dispatch: loop {
        match pc {
            0x823DD180 => {
    //   block [0x823DD180..0x823DD19C)
	// 823DD180: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DD184: 508B7C20  rlwimi r11, r4, 0xf, 0x10, 0x10
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(15) as u64) & 0x0000000000008000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF7FFF);
	// 823DD188: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DD18C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD190: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 823DD194: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD1A0 size=12
    let mut pc: u32 = 0x823DD1A0;
    'dispatch: loop {
        match pc {
            0x823DD1A0 => {
    //   block [0x823DD1A0..0x823DD1AC)
	// 823DD1A0: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DD1A4: 55638FFE  rlwinm r3, r11, 0x11, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 823DD1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD1B0 size=24
    let mut pc: u32 = 0x823DD1B0;
    'dispatch: loop {
        match pc {
            0x823DD1B0 => {
    //   block [0x823DD1B0..0x823DD1C8)
	// 823DD1B0: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 823DD1B4: 91632A00  stw r11, 0x2a00(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10752 as u32), ctx.r[11].u32 ) };
	// 823DD1B8: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 823DD1BC: 656B0008  oris r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 524288;
	// 823DD1C0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DD1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD1C8 size=8
    let mut pc: u32 = 0x823DD1C8;
    'dispatch: loop {
        match pc {
            0x823DD1C8 => {
    //   block [0x823DD1C8..0x823DD1D0)
	// 823DD1C8: 80632A00  lwz r3, 0x2a00(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10752 as u32) ) } as u64;
	// 823DD1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD1D0 size=56
    let mut pc: u32 = 0x823DD1D0;
    'dispatch: loop {
        match pc {
            0x823DD1D0 => {
    //   block [0x823DD1D0..0x823DD208)
	// 823DD1D0: 81633098  lwz r11, 0x3098(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12440 as u32) ) } as u64;
	// 823DD1D4: 90832E54  stw r4, 0x2e54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11860 as u32), ctx.r[4].u32 ) };
	// 823DD1D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DD1DC: 409A0008  bne cr6, 0x823dd1e4
	if !ctx.cr[6].eq {
	pc = 0x823DD1E4; continue 'dispatch;
	}
	// 823DD1E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DD1E4: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 823DD1E8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD1EC: 51640036  rlwimi r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF0) | (ctx.r[4].u64 & 0xFFFFFFFF0000000F);
	// 823DD1F0: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 823DD1F4: 908328DC  stw r4, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[4].u32 ) };
	// 823DD1F8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD1FC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD200: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD208 size=8
    let mut pc: u32 = 0x823DD208;
    'dispatch: loop {
        match pc {
            0x823DD208 => {
    //   block [0x823DD208..0x823DD210)
	// 823DD208: 80632E54  lwz r3, 0x2e54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11860 as u32) ) } as u64;
	// 823DD20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD210 size=56
    let mut pc: u32 = 0x823DD210;
    'dispatch: loop {
        match pc {
            0x823DD210 => {
    //   block [0x823DD210..0x823DD248)
	// 823DD210: 8163309C  lwz r11, 0x309c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12444 as u32) ) } as u64;
	// 823DD214: 90832E58  stw r4, 0x2e58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11864 as u32), ctx.r[4].u32 ) };
	// 823DD218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DD21C: 409A0008  bne cr6, 0x823dd224
	if !ctx.cr[6].eq {
	pc = 0x823DD224; continue 'dispatch;
	}
	// 823DD220: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DD224: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 823DD228: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD22C: 508B2636  rlwimi r11, r4, 4, 0x18, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x00000000000000F0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF0F);
	// 823DD230: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 823DD234: 916328DC  stw r11, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[11].u32 ) };
	// 823DD238: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD23C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD240: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD248 size=8
    let mut pc: u32 = 0x823DD248;
    'dispatch: loop {
        match pc {
            0x823DD248 => {
    //   block [0x823DD248..0x823DD250)
	// 823DD248: 80632E58  lwz r3, 0x2e58(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11864 as u32) ) } as u64;
	// 823DD24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD250 size=56
    let mut pc: u32 = 0x823DD250;
    'dispatch: loop {
        match pc {
            0x823DD250 => {
    //   block [0x823DD250..0x823DD288)
	// 823DD250: 816330A0  lwz r11, 0x30a0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12448 as u32) ) } as u64;
	// 823DD254: 90832E5C  stw r4, 0x2e5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11868 as u32), ctx.r[4].u32 ) };
	// 823DD258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DD25C: 409A0008  bne cr6, 0x823dd264
	if !ctx.cr[6].eq {
	pc = 0x823DD264; continue 'dispatch;
	}
	// 823DD260: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DD264: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 823DD268: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD26C: 508B452E  rlwimi r11, r4, 8, 0x14, 0x17
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000000F00) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF0FF);
	// 823DD270: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 823DD274: 916328DC  stw r11, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[11].u32 ) };
	// 823DD278: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD27C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD280: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD288 size=8
    let mut pc: u32 = 0x823DD288;
    'dispatch: loop {
        match pc {
            0x823DD288 => {
    //   block [0x823DD288..0x823DD290)
	// 823DD288: 80632E5C  lwz r3, 0x2e5c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11868 as u32) ) } as u64;
	// 823DD28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD290 size=56
    let mut pc: u32 = 0x823DD290;
    'dispatch: loop {
        match pc {
            0x823DD290 => {
    //   block [0x823DD290..0x823DD2C8)
	// 823DD290: 816330A4  lwz r11, 0x30a4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12452 as u32) ) } as u64;
	// 823DD294: 90832E60  stw r4, 0x2e60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11872 as u32), ctx.r[4].u32 ) };
	// 823DD298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823DD29C: 409A0008  bne cr6, 0x823dd2a4
	if !ctx.cr[6].eq {
	pc = 0x823DD2A4; continue 'dispatch;
	}
	// 823DD2A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823DD2A4: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 823DD2A8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD2AC: 508B6426  rlwimi r11, r4, 0xc, 0x10, 0x13
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0FFF);
	// 823DD2B0: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 823DD2B4: 916328DC  stw r11, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[11].u32 ) };
	// 823DD2B8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD2BC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD2C0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD2C8 size=8
    let mut pc: u32 = 0x823DD2C8;
    'dispatch: loop {
        match pc {
            0x823DD2C8 => {
    //   block [0x823DD2C8..0x823DD2D0)
	// 823DD2C8: 80632E60  lwz r3, 0x2e60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11872 as u32) ) } as u64;
	// 823DD2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD2D0 size=8
    let mut pc: u32 = 0x823DD2D0;
    'dispatch: loop {
        match pc {
            0x823DD2D0 => {
    //   block [0x823DD2D0..0x823DD2D8)
	// 823DD2D0: 90832E6C  stw r4, 0x2e6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11884 as u32), ctx.r[4].u32 ) };
	// 823DD2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD2D8 size=8
    let mut pc: u32 = 0x823DD2D8;
    'dispatch: loop {
        match pc {
            0x823DD2D8 => {
    //   block [0x823DD2D8..0x823DD2E0)
	// 823DD2D8: 80632E6C  lwz r3, 0x2e6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11884 as u32) ) } as u64;
	// 823DD2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD2E0 size=76
    let mut pc: u32 = 0x823DD2E0;
    'dispatch: loop {
        match pc {
            0x823DD2E0 => {
    //   block [0x823DD2E0..0x823DD32C)
	// 823DD2E0: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DD2E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DD2E8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD2EC: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 823DD2F0: C1AB1848  lfs f13, 0x1848(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD2F4: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 823DD2F8: C001001C  lfs f0, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD2FC: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 823DD300: D0032E74  stfs f0, 0x2e74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11892 as u32), tmp.u32 ) };
	// 823DD304: FC00681E  fctiwz f0, f13
	ctx.f[0].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 823DD308: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 823DD30C: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD310: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD314: B1632966  sth r11, 0x2966(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10598 as u32), ctx.r[11].u16 ) };
	// 823DD318: B1632964  sth r11, 0x2964(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10596 as u32), ctx.r[11].u16 ) };
	// 823DD31C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DD320: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD324: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DD328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD330 size=16
    let mut pc: u32 = 0x823DD330;
    'dispatch: loop {
        match pc {
            0x823DD330 => {
    //   block [0x823DD330..0x823DD340)
	// 823DD330: C0032E74  lfs f0, 0x2e74(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11892 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD334: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DD338: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD340 size=68
    let mut pc: u32 = 0x823DD340;
    'dispatch: loop {
        match pc {
            0x823DD340 => {
    //   block [0x823DD340..0x823DD384)
	// 823DD340: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DD344: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DD348: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD34C: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 823DD350: C1AB184C  lfs f13, 0x184c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD354: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 823DD358: C001001C  lfs f0, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD35C: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 823DD360: D0032E78  stfs f0, 0x2e78(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11896 as u32), tmp.u32 ) };
	// 823DD364: FC00681E  fctiwz f0, f13
	ctx.f[0].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 823DD368: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 823DD36C: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD370: B163296A  sth r11, 0x296a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10602 as u32), ctx.r[11].u16 ) };
	// 823DD374: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DD378: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD37C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DD380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD388 size=16
    let mut pc: u32 = 0x823DD388;
    'dispatch: loop {
        match pc {
            0x823DD388 => {
    //   block [0x823DD388..0x823DD398)
	// 823DD388: C0032E78  lfs f0, 0x2e78(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11896 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD38C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DD390: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD398 size=68
    let mut pc: u32 = 0x823DD398;
    'dispatch: loop {
        match pc {
            0x823DD398 => {
    //   block [0x823DD398..0x823DD3DC)
	// 823DD398: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DD39C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823DD3A0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD3A4: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 823DD3A8: C1AB184C  lfs f13, 0x184c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823DD3AC: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 823DD3B0: C001001C  lfs f0, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD3B4: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 823DD3B8: D0032E7C  stfs f0, 0x2e7c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11900 as u32), tmp.u32 ) };
	// 823DD3BC: FC00681E  fctiwz f0, f13
	ctx.f[0].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 823DD3C0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 823DD3C4: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD3C8: B1632968  sth r11, 0x2968(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10600 as u32), ctx.r[11].u16 ) };
	// 823DD3CC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DD3D0: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD3D4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DD3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DD3E0 size=16
    let mut pc: u32 = 0x823DD3E0;
    'dispatch: loop {
        match pc {
            0x823DD3E0 => {
    //   block [0x823DD3E0..0x823DD3F0)
	// 823DD3E0: C0032E7C  lfs f0, 0x2e7c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11900 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DD3E4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DD3E8: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DD3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD3F0 size=32
    let mut pc: u32 = 0x823DD3F0;
    'dispatch: loop {
        match pc {
            0x823DD3F0 => {
    //   block [0x823DD3F0..0x823DD410)
	// 823DD3F0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD3F4: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD3F8: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 823DD3FC: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD400: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD404: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD408: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD410 size=36
    let mut pc: u32 = 0x823DD410;
    'dispatch: loop {
        match pc {
            0x823DD410 => {
    //   block [0x823DD410..0x823DD434)
	// 823DD410: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD414: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD418: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD41C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD420: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD424: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD428: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD42C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD438 size=36
    let mut pc: u32 = 0x823DD438;
    'dispatch: loop {
        match pc {
            0x823DD438 => {
    //   block [0x823DD438..0x823DD45C)
	// 823DD438: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD43C: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD440: 556B0626  rlwinm r11, r11, 0, 0x18, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD444: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD448: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD44C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD450: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD454: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD460 size=36
    let mut pc: u32 = 0x823DD460;
    'dispatch: loop {
        match pc {
            0x823DD460 => {
    //   block [0x823DD460..0x823DD484)
	// 823DD460: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD464: 548A6026  slwi r10, r4, 0xc
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(12);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD468: 556B051E  rlwinm r11, r11, 0, 0x14, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD46C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD470: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD474: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD478: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD47C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD488 size=36
    let mut pc: u32 = 0x823DD488;
    'dispatch: loop {
        match pc {
            0x823DD488 => {
    //   block [0x823DD488..0x823DD4AC)
	// 823DD488: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD48C: 548A801E  slwi r10, r4, 0x10
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD490: 556B0416  rlwinm r11, r11, 0, 0x10, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD494: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD498: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD49C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD4A0: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD4A4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD4B0 size=36
    let mut pc: u32 = 0x823DD4B0;
    'dispatch: loop {
        match pc {
            0x823DD4B0 => {
    //   block [0x823DD4B0..0x823DD4D4)
	// 823DD4B0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD4B4: 548AA016  slwi r10, r4, 0x14
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD4B8: 556B030E  rlwinm r11, r11, 0, 0xc, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD4BC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD4C0: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD4C4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD4C8: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD4CC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD4D8 size=36
    let mut pc: u32 = 0x823DD4D8;
    'dispatch: loop {
        match pc {
            0x823DD4D8 => {
    //   block [0x823DD4D8..0x823DD4FC)
	// 823DD4D8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD4DC: 548AC00E  slwi r10, r4, 0x18
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD4E0: 556B0206  rlwinm r11, r11, 0, 8, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD4E4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD4E8: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD4EC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD4F0: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD4F4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD500 size=28
    let mut pc: u32 = 0x823DD500;
    'dispatch: loop {
        match pc {
            0x823DD500 => {
    //   block [0x823DD500..0x823DD51C)
	// 823DD500: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD504: 508BE006  rlwimi r11, r4, 0x1c, 0, 3
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[11].u64 & 0xFFFFFFFF0FFFFFFF);
	// 823DD508: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 823DD50C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD510: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 823DD514: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD520 size=32
    let mut pc: u32 = 0x823DD520;
    'dispatch: loop {
        match pc {
            0x823DD520 => {
    //   block [0x823DD520..0x823DD540)
	// 823DD520: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD524: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD528: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 823DD52C: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD530: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD534: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD538: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD540 size=36
    let mut pc: u32 = 0x823DD540;
    'dispatch: loop {
        match pc {
            0x823DD540 => {
    //   block [0x823DD540..0x823DD564)
	// 823DD540: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD544: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD548: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD54C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD550: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD554: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD558: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD55C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD568 size=36
    let mut pc: u32 = 0x823DD568;
    'dispatch: loop {
        match pc {
            0x823DD568 => {
    //   block [0x823DD568..0x823DD58C)
	// 823DD568: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD56C: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD570: 556B0626  rlwinm r11, r11, 0, 0x18, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD574: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD578: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD57C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD580: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD584: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD590 size=36
    let mut pc: u32 = 0x823DD590;
    'dispatch: loop {
        match pc {
            0x823DD590 => {
    //   block [0x823DD590..0x823DD5B4)
	// 823DD590: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD594: 548A6026  slwi r10, r4, 0xc
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(12);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD598: 556B051E  rlwinm r11, r11, 0, 0x14, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD59C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD5A0: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD5A4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD5A8: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD5AC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD5B8 size=36
    let mut pc: u32 = 0x823DD5B8;
    'dispatch: loop {
        match pc {
            0x823DD5B8 => {
    //   block [0x823DD5B8..0x823DD5DC)
	// 823DD5B8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD5BC: 548A801E  slwi r10, r4, 0x10
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD5C0: 556B0416  rlwinm r11, r11, 0, 0x10, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD5C4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD5C8: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD5CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD5D0: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD5D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD5E0 size=36
    let mut pc: u32 = 0x823DD5E0;
    'dispatch: loop {
        match pc {
            0x823DD5E0 => {
    //   block [0x823DD5E0..0x823DD604)
	// 823DD5E0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD5E4: 548AA016  slwi r10, r4, 0x14
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD5E8: 556B030E  rlwinm r11, r11, 0, 0xc, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD5EC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD5F0: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD5F4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD5F8: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD5FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD608 size=36
    let mut pc: u32 = 0x823DD608;
    'dispatch: loop {
        match pc {
            0x823DD608 => {
    //   block [0x823DD608..0x823DD62C)
	// 823DD608: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD60C: 548AC00E  slwi r10, r4, 0x18
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823DD610: 556B0206  rlwinm r11, r11, 0, 8, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD614: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823DD618: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD61C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD620: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD624: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD630 size=28
    let mut pc: u32 = 0x823DD630;
    'dispatch: loop {
        match pc {
            0x823DD630 => {
    //   block [0x823DD630..0x823DD64C)
	// 823DD630: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD634: 508BE006  rlwimi r11, r4, 0x1c, 0, 3
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[11].u64 & 0xFFFFFFFF0FFFFFFF);
	// 823DD638: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 823DD63C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD640: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 823DD644: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD650 size=12
    let mut pc: u32 = 0x823DD650;
    'dispatch: loop {
        match pc {
            0x823DD650 => {
    //   block [0x823DD650..0x823DD65C)
	// 823DD650: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD654: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD660 size=12
    let mut pc: u32 = 0x823DD660;
    'dispatch: loop {
        match pc {
            0x823DD660 => {
    //   block [0x823DD660..0x823DD66C)
	// 823DD660: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD664: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD670 size=12
    let mut pc: u32 = 0x823DD670;
    'dispatch: loop {
        match pc {
            0x823DD670 => {
    //   block [0x823DD670..0x823DD67C)
	// 823DD670: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD674: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823DD678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD680 size=12
    let mut pc: u32 = 0x823DD680;
    'dispatch: loop {
        match pc {
            0x823DD680 => {
    //   block [0x823DD680..0x823DD68C)
	// 823DD680: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD684: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 823DD688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD690 size=12
    let mut pc: u32 = 0x823DD690;
    'dispatch: loop {
        match pc {
            0x823DD690 => {
    //   block [0x823DD690..0x823DD69C)
	// 823DD690: A163292C  lhz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD694: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD6A0 size=12
    let mut pc: u32 = 0x823DD6A0;
    'dispatch: loop {
        match pc {
            0x823DD6A0 => {
    //   block [0x823DD6A0..0x823DD6AC)
	// 823DD6A0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD6A4: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 823DD6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD6B0 size=12
    let mut pc: u32 = 0x823DD6B0;
    'dispatch: loop {
        match pc {
            0x823DD6B0 => {
    //   block [0x823DD6B0..0x823DD6BC)
	// 823DD6B0: 8963292C  lbz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD6B4: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD6C0 size=12
    let mut pc: u32 = 0x823DD6C0;
    'dispatch: loop {
        match pc {
            0x823DD6C0 => {
    //   block [0x823DD6C0..0x823DD6CC)
	// 823DD6C0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 823DD6C4: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823DD6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD6D0 size=12
    let mut pc: u32 = 0x823DD6D0;
    'dispatch: loop {
        match pc {
            0x823DD6D0 => {
    //   block [0x823DD6D0..0x823DD6DC)
	// 823DD6D0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD6D4: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD6E0 size=12
    let mut pc: u32 = 0x823DD6E0;
    'dispatch: loop {
        match pc {
            0x823DD6E0 => {
    //   block [0x823DD6E0..0x823DD6EC)
	// 823DD6E0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD6E4: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD6F0 size=12
    let mut pc: u32 = 0x823DD6F0;
    'dispatch: loop {
        match pc {
            0x823DD6F0 => {
    //   block [0x823DD6F0..0x823DD6FC)
	// 823DD6F0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD6F4: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823DD6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD700 size=12
    let mut pc: u32 = 0x823DD700;
    'dispatch: loop {
        match pc {
            0x823DD700 => {
    //   block [0x823DD700..0x823DD70C)
	// 823DD700: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD704: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 823DD708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD710 size=12
    let mut pc: u32 = 0x823DD710;
    'dispatch: loop {
        match pc {
            0x823DD710 => {
    //   block [0x823DD710..0x823DD71C)
	// 823DD710: A1632930  lhz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD714: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD720 size=12
    let mut pc: u32 = 0x823DD720;
    'dispatch: loop {
        match pc {
            0x823DD720 => {
    //   block [0x823DD720..0x823DD72C)
	// 823DD720: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD724: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 823DD728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD730 size=12
    let mut pc: u32 = 0x823DD730;
    'dispatch: loop {
        match pc {
            0x823DD730 => {
    //   block [0x823DD730..0x823DD73C)
	// 823DD730: 89632930  lbz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD734: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DD738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD740 size=12
    let mut pc: u32 = 0x823DD740;
    'dispatch: loop {
        match pc {
            0x823DD740 => {
    //   block [0x823DD740..0x823DD74C)
	// 823DD740: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 823DD744: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823DD748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD750 size=64
    let mut pc: u32 = 0x823DD750;
    'dispatch: loop {
        match pc {
            0x823DD750 => {
    //   block [0x823DD750..0x823DD790)
	// 823DD750: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 823DD754: 3960043F  li r11, 0x43f
	ctx.r[11].s64 = 1087;
	// 823DD758: 409A0008  bne cr6, 0x823dd760
	if !ctx.cr[6].eq {
	pc = 0x823DD760; continue 'dispatch;
	}
	// 823DD75C: 39600400  li r11, 0x400
	ctx.r[11].s64 = 1024;
	// 823DD760: 9163294C  stw r11, 0x294c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10572 as u32), ctx.r[11].u32 ) };
	// 823DD764: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 823DD768: 81432944  lwz r10, 0x2944(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 823DD76C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823DD770: 516A83DE  rlwimi r10, r11, 0x10, 0xf, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x0000000000010000) | (ctx.r[10].u64 & 0xFFFFFFFFFFFEFFFF);
	// 823DD774: 91432944  stw r10, 0x2944(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10564 as u32), ctx.r[10].u32 ) };
	// 823DD778: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD77C: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 823DD780: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD784: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 823DD788: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD790 size=12
    let mut pc: u32 = 0x823DD790;
    'dispatch: loop {
        match pc {
            0x823DD790 => {
    //   block [0x823DD790..0x823DD79C)
	// 823DD790: 8163294C  lwz r11, 0x294c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10572 as u32) ) } as u64;
	// 823DD794: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 823DD798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD7A0 size=16
    let mut pc: u32 = 0x823DD7A0;
    'dispatch: loop {
        match pc {
            0x823DD7A0 => {
    //   block [0x823DD7A0..0x823DD7B0)
	// 823DD7A0: 81033098  lwz r8, 0x3098(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12440 as u32) ) } as u64;
	// 823DD7A4: 90832EFC  stw r4, 0x2efc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12028 as u32), ctx.r[4].u32 ) };
	// 823DD7A8: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DD7AC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD7B0 size=40
    let mut pc: u32 = 0x823DD7B0;
    'dispatch: loop {
        match pc {
            0x823DD7B0 => {
    //   block [0x823DD7B0..0x823DD7D8)
	// 823DD7B0: 8148001C  lwz r10, 0x1c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD7B4: 554B873E  rlwinm r11, r10, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823DD7B8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 823DD7BC: 419A001C  beq cr6, 0x823dd7d8
	if ctx.cr[6].eq {
		sub_823DD7D8(ctx, base);
		return;
	}
	// 823DD7C0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823DD7C4: 419A0014  beq cr6, 0x823dd7d8
	if ctx.cr[6].eq {
		sub_823DD7D8(ctx, base);
		return;
	}
	// 823DD7C8: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 823DD7CC: 419A000C  beq cr6, 0x823dd7d8
	if ctx.cr[6].eq {
		sub_823DD7D8(ctx, base);
		return;
	}
	// 823DD7D0: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 823DD7D4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD7D8 size=12
    let mut pc: u32 = 0x823DD7D8;
    'dispatch: loop {
        match pc {
            0x823DD7D8 => {
    //   block [0x823DD7D8..0x823DD7E4)
	// 823DD7D8: 554B6FFE  rlwinm r11, r10, 0xd, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0007FFFFu64;
	// 823DD7DC: 7D6B2279  xor. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DD7E0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD7E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD7E4 size=96
    let mut pc: u32 = 0x823DD7E4;
    'dispatch: loop {
        match pc {
            0x823DD7E4 => {
    //   block [0x823DD7E4..0x823DD844)
	// 823DD7E4: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD7E8: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 823DD7EC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD7F0: 5569873E  rlwinm r9, r11, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD7F4: 55670416  rlwinm r7, r11, 0, 0x10, 0xb
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD7F8: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DD7FC: 38C90003  addi r6, r9, 3
	ctx.r[6].s64 = ctx.r[9].s64 + 3;
	// 823DD800: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 823DD804: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DD808: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 823DD80C: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 823DD810: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 823DD814: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 823DD818: 556B831E  rlwinm r11, r11, 0x10, 0xc, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD81C: 7CEA5B78  or r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 823DD820: 9148001C  stw r10, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 823DD824: 81432884  lwz r10, 0x2884(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10372 as u32) ) } as u64;
	// 823DD828: 554A0416  rlwinm r10, r10, 0, 0x10, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD82C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DD830: 91632884  stw r11, 0x2884(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10372 as u32), ctx.r[11].u32 ) };
	// 823DD834: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD838: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD83C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD848 size=8
    let mut pc: u32 = 0x823DD848;
    'dispatch: loop {
        match pc {
            0x823DD848 => {
    //   block [0x823DD848..0x823DD850)
	// 823DD848: 80632EFC  lwz r3, 0x2efc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12028 as u32) ) } as u64;
	// 823DD84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD850 size=16
    let mut pc: u32 = 0x823DD850;
    'dispatch: loop {
        match pc {
            0x823DD850 => {
    //   block [0x823DD850..0x823DD860)
	// 823DD850: 8103309C  lwz r8, 0x309c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12444 as u32) ) } as u64;
	// 823DD854: 90832F00  stw r4, 0x2f00(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12032 as u32), ctx.r[4].u32 ) };
	// 823DD858: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DD85C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD860 size=40
    let mut pc: u32 = 0x823DD860;
    'dispatch: loop {
        match pc {
            0x823DD860 => {
    //   block [0x823DD860..0x823DD888)
	// 823DD860: 8148001C  lwz r10, 0x1c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD864: 554B873E  rlwinm r11, r10, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823DD868: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 823DD86C: 419A001C  beq cr6, 0x823dd888
	if ctx.cr[6].eq {
		sub_823DD888(ctx, base);
		return;
	}
	// 823DD870: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823DD874: 419A0014  beq cr6, 0x823dd888
	if ctx.cr[6].eq {
		sub_823DD888(ctx, base);
		return;
	}
	// 823DD878: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 823DD87C: 419A000C  beq cr6, 0x823dd888
	if ctx.cr[6].eq {
		sub_823DD888(ctx, base);
		return;
	}
	// 823DD880: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 823DD884: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD888 size=12
    let mut pc: u32 = 0x823DD888;
    'dispatch: loop {
        match pc {
            0x823DD888 => {
    //   block [0x823DD888..0x823DD894)
	// 823DD888: 554B6FFE  rlwinm r11, r10, 0xd, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0007FFFFu64;
	// 823DD88C: 7D6B2279  xor. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DD890: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD894 size=96
    let mut pc: u32 = 0x823DD894;
    'dispatch: loop {
        match pc {
            0x823DD894 => {
    //   block [0x823DD894..0x823DD8F4)
	// 823DD894: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD898: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 823DD89C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD8A0: 5569873E  rlwinm r9, r11, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD8A4: 55670416  rlwinm r7, r11, 0, 0x10, 0xb
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD8A8: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DD8AC: 38C90003  addi r6, r9, 3
	ctx.r[6].s64 = ctx.r[9].s64 + 3;
	// 823DD8B0: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 823DD8B4: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DD8B8: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 823DD8BC: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 823DD8C0: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 823DD8C4: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 823DD8C8: 556B831E  rlwinm r11, r11, 0x10, 0xc, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD8CC: 7CEA5B78  or r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 823DD8D0: 9148001C  stw r10, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 823DD8D4: 8143288C  lwz r10, 0x288c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10380 as u32) ) } as u64;
	// 823DD8D8: 554A0416  rlwinm r10, r10, 0, 0x10, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD8DC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DD8E0: 9163288C  stw r11, 0x288c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10380 as u32), ctx.r[11].u32 ) };
	// 823DD8E4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD8E8: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD8EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD8F8 size=8
    let mut pc: u32 = 0x823DD8F8;
    'dispatch: loop {
        match pc {
            0x823DD8F8 => {
    //   block [0x823DD8F8..0x823DD900)
	// 823DD8F8: 80632F00  lwz r3, 0x2f00(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12032 as u32) ) } as u64;
	// 823DD8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD900 size=16
    let mut pc: u32 = 0x823DD900;
    'dispatch: loop {
        match pc {
            0x823DD900 => {
    //   block [0x823DD900..0x823DD910)
	// 823DD900: 810330A0  lwz r8, 0x30a0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12448 as u32) ) } as u64;
	// 823DD904: 90832F04  stw r4, 0x2f04(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12036 as u32), ctx.r[4].u32 ) };
	// 823DD908: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DD90C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD910 size=40
    let mut pc: u32 = 0x823DD910;
    'dispatch: loop {
        match pc {
            0x823DD910 => {
    //   block [0x823DD910..0x823DD938)
	// 823DD910: 8148001C  lwz r10, 0x1c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD914: 554B873E  rlwinm r11, r10, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823DD918: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 823DD91C: 419A001C  beq cr6, 0x823dd938
	if ctx.cr[6].eq {
		sub_823DD938(ctx, base);
		return;
	}
	// 823DD920: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823DD924: 419A0014  beq cr6, 0x823dd938
	if ctx.cr[6].eq {
		sub_823DD938(ctx, base);
		return;
	}
	// 823DD928: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 823DD92C: 419A000C  beq cr6, 0x823dd938
	if ctx.cr[6].eq {
		sub_823DD938(ctx, base);
		return;
	}
	// 823DD930: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 823DD934: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD938 size=12
    let mut pc: u32 = 0x823DD938;
    'dispatch: loop {
        match pc {
            0x823DD938 => {
    //   block [0x823DD938..0x823DD944)
	// 823DD938: 554B6FFE  rlwinm r11, r10, 0xd, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0007FFFFu64;
	// 823DD93C: 7D6B2279  xor. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DD940: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD944 size=96
    let mut pc: u32 = 0x823DD944;
    'dispatch: loop {
        match pc {
            0x823DD944 => {
    //   block [0x823DD944..0x823DD9A4)
	// 823DD944: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD948: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 823DD94C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DD950: 5569873E  rlwinm r9, r11, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD954: 55670416  rlwinm r7, r11, 0, 0x10, 0xb
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD958: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DD95C: 38C90003  addi r6, r9, 3
	ctx.r[6].s64 = ctx.r[9].s64 + 3;
	// 823DD960: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 823DD964: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DD968: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 823DD96C: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 823DD970: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 823DD974: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 823DD978: 556B831E  rlwinm r11, r11, 0x10, 0xc, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DD97C: 7CEA5B78  or r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 823DD980: 9148001C  stw r10, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 823DD984: 81432890  lwz r10, 0x2890(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10384 as u32) ) } as u64;
	// 823DD988: 554A0416  rlwinm r10, r10, 0, 0x10, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DD98C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DD990: 91632890  stw r11, 0x2890(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10384 as u32), ctx.r[11].u32 ) };
	// 823DD994: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DD998: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DD99C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DD9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD9A8 size=8
    let mut pc: u32 = 0x823DD9A8;
    'dispatch: loop {
        match pc {
            0x823DD9A8 => {
    //   block [0x823DD9A8..0x823DD9B0)
	// 823DD9A8: 80632F04  lwz r3, 0x2f04(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12036 as u32) ) } as u64;
	// 823DD9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD9B0 size=16
    let mut pc: u32 = 0x823DD9B0;
    'dispatch: loop {
        match pc {
            0x823DD9B0 => {
    //   block [0x823DD9B0..0x823DD9C0)
	// 823DD9B0: 810330A4  lwz r8, 0x30a4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12452 as u32) ) } as u64;
	// 823DD9B4: 90832F08  stw r4, 0x2f08(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12040 as u32), ctx.r[4].u32 ) };
	// 823DD9B8: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823DD9BC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD9C0 size=40
    let mut pc: u32 = 0x823DD9C0;
    'dispatch: loop {
        match pc {
            0x823DD9C0 => {
    //   block [0x823DD9C0..0x823DD9E8)
	// 823DD9C0: 8148001C  lwz r10, 0x1c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD9C4: 554B873E  rlwinm r11, r10, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 823DD9C8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 823DD9CC: 419A001C  beq cr6, 0x823dd9e8
	if ctx.cr[6].eq {
		sub_823DD9E8(ctx, base);
		return;
	}
	// 823DD9D0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823DD9D4: 419A0014  beq cr6, 0x823dd9e8
	if ctx.cr[6].eq {
		sub_823DD9E8(ctx, base);
		return;
	}
	// 823DD9D8: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 823DD9DC: 419A000C  beq cr6, 0x823dd9e8
	if ctx.cr[6].eq {
		sub_823DD9E8(ctx, base);
		return;
	}
	// 823DD9E0: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 823DD9E4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD9E8 size=12
    let mut pc: u32 = 0x823DD9E8;
    'dispatch: loop {
        match pc {
            0x823DD9E8 => {
    //   block [0x823DD9E8..0x823DD9F4)
	// 823DD9E8: 554B6FFE  rlwinm r11, r10, 0xd, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0007FFFFu64;
	// 823DD9EC: 7D6B2279  xor. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823DD9F0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DD9F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DD9F4 size=96
    let mut pc: u32 = 0x823DD9F4;
    'dispatch: loop {
        match pc {
            0x823DD9F4 => {
    //   block [0x823DD9F4..0x823DDA54)
	// 823DD9F4: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 823DD9F8: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 823DD9FC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDA00: 5569873E  rlwinm r9, r11, 0x10, 0x1c, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DDA04: 55670416  rlwinm r7, r11, 0, 0x10, 0xb
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823DDA08: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823DDA0C: 38C90003  addi r6, r9, 3
	ctx.r[6].s64 = ctx.r[9].s64 + 3;
	// 823DDA10: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 823DDA14: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 823DDA18: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 823DDA1C: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 823DDA20: 798CA7E6  rldicr r12, r12, 0x34, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(52) & 0xFFFFFFFFFFFFFFFF;
	// 823DDA24: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 823DDA28: 556B831E  rlwinm r11, r11, 0x10, 0xc, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 823DDA2C: 7CEA5B78  or r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 823DDA30: 9148001C  stw r10, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 823DDA34: 81432894  lwz r10, 0x2894(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10388 as u32) ) } as u64;
	// 823DDA38: 554A0416  rlwinm r10, r10, 0, 0x10, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823DDA3C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 823DDA40: 91632894  stw r11, 0x2894(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10388 as u32), ctx.r[11].u32 ) };
	// 823DDA44: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DDA48: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDA4C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DDA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDA58 size=8
    let mut pc: u32 = 0x823DDA58;
    'dispatch: loop {
        match pc {
            0x823DDA58 => {
    //   block [0x823DDA58..0x823DDA60)
	// 823DDA58: 80632F08  lwz r3, 0x2f08(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12040 as u32) ) } as u64;
	// 823DDA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DDA60 size=36
    let mut pc: u32 = 0x823DDA60;
    'dispatch: loop {
        match pc {
            0x823DDA60 => {
    //   block [0x823DDA60..0x823DDA84)
	// 823DDA60: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DDA64: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDA68: 798C7FE6  rldicr r12, r12, 0x2f, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(47) & 0xFFFFFFFFFFFFFFFF;
	// 823DDA6C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DDA70: D0032980  stfs f0, 0x2980(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10624 as u32), tmp.u32 ) };
	// 823DDA74: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DDA78: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDA7C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DDA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DDA88 size=16
    let mut pc: u32 = 0x823DDA88;
    'dispatch: loop {
        match pc {
            0x823DDA88 => {
    //   block [0x823DDA88..0x823DDA98)
	// 823DDA88: C0032980  lfs f0, 0x2980(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DDA8C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DDA90: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DDA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DDA98 size=36
    let mut pc: u32 = 0x823DDA98;
    'dispatch: loop {
        match pc {
            0x823DDA98 => {
    //   block [0x823DDA98..0x823DDABC)
	// 823DDA98: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DDA9C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDAA0: 798C87E6  rldicr r12, r12, 0x30, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(48) & 0xFFFFFFFFFFFFFFFF;
	// 823DDAA4: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DDAA8: D003297C  stfs f0, 0x297c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10620 as u32), tmp.u32 ) };
	// 823DDAAC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DDAB0: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDAB4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DDAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DDAC0 size=16
    let mut pc: u32 = 0x823DDAC0;
    'dispatch: loop {
        match pc {
            0x823DDAC0 => {
    //   block [0x823DDAC0..0x823DDAD0)
	// 823DDAC0: C003297C  lfs f0, 0x297c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10620 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DDAC4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 823DDAC8: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823DDACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDAD0 size=36
    let mut pc: u32 = 0x823DDAD0;
    'dispatch: loop {
        match pc {
            0x823DDAD0 => {
    //   block [0x823DDAD0..0x823DDAF4)
	// 823DDAD0: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 823DDAD4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDAD8: 5164003A  rlwimi r4, r11, 0, 0, 0x1d
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[4].u64 & 0xFFFFFFFF00000003);
	// 823DDADC: 798C8FE6  rldicr r12, r12, 0x31, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(49) & 0xFFFFFFFFFFFFFFFF;
	// 823DDAE0: 90832978  stw r4, 0x2978(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10616 as u32), ctx.r[4].u32 ) };
	// 823DDAE4: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 823DDAE8: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDAEC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 823DDAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDAF8 size=12
    let mut pc: u32 = 0x823DDAF8;
    'dispatch: loop {
        match pc {
            0x823DDAF8 => {
    //   block [0x823DDAF8..0x823DDB04)
	// 823DDAF8: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 823DDAFC: 556307BE  clrlwi r3, r11, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823DDB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB08 size=36
    let mut pc: u32 = 0x823DDB08;
    'dispatch: loop {
        match pc {
            0x823DDB08 => {
    //   block [0x823DDB08..0x823DDB2C)
	// 823DDB08: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 823DDB0C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDB10: 5164003C  rlwimi r4, r11, 0, 0, 0x1e
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFE) | (ctx.r[4].u64 & 0xFFFFFFFF00000001);
	// 823DDB14: 798C1FE6  rldicr r12, r12, 0x23, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(35) & 0xFFFFFFFFFFFFFFFF;
	// 823DDB18: 908329C0  stw r4, 0x29c0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10688 as u32), ctx.r[4].u32 ) };
	// 823DDB1C: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 823DDB20: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDB24: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DDB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB30 size=12
    let mut pc: u32 = 0x823DDB30;
    'dispatch: loop {
        match pc {
            0x823DDB30 => {
    //   block [0x823DDB30..0x823DDB3C)
	// 823DDB30: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 823DDB34: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 823DDB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB40 size=28
    let mut pc: u32 = 0x823DDB40;
    'dispatch: loop {
        match pc {
            0x823DDB40 => {
    //   block [0x823DDB40..0x823DDB5C)
	// 823DDB40: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DDB44: 508BAA94  rlwimi r11, r4, 0x15, 0xa, 0xa
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(21) as u64) & 0x0000000000200000) | (ctx.r[11].u64 & 0xFFFFFFFFFFDFFFFF);
	// 823DDB48: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 823DDB4C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DDB50: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 823DDB54: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DDB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB60 size=12
    let mut pc: u32 = 0x823DDB60;
    'dispatch: loop {
        match pc {
            0x823DDB60 => {
    //   block [0x823DDB60..0x823DDB6C)
	// 823DDB60: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 823DDB64: 55635FFE  rlwinm r3, r11, 0xb, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 823DDB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB70 size=28
    let mut pc: u32 = 0x823DDB70;
    'dispatch: loop {
        match pc {
            0x823DDB70 => {
    //   block [0x823DDB70..0x823DDB8C)
	// 823DDB70: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDB74: 908328D8  stw r4, 0x28d8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10456 as u32), ctx.r[4].u32 ) };
	// 823DDB78: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DDB7C: 798C37E6  rldicr r12, r12, 0x26, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(38) & 0xFFFFFFFFFFFFFFFF;
	// 823DDB80: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDB84: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DDB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB90 size=8
    let mut pc: u32 = 0x823DDB90;
    'dispatch: loop {
        match pc {
            0x823DDB90 => {
    //   block [0x823DDB90..0x823DDB98)
	// 823DDB90: 806328D8  lwz r3, 0x28d8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10456 as u32) ) } as u64;
	// 823DDB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDB98 size=28
    let mut pc: u32 = 0x823DDB98;
    'dispatch: loop {
        match pc {
            0x823DDB98 => {
    //   block [0x823DDB98..0x823DDBB4)
	// 823DDB98: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DDB9C: 508B26F6  rlwimi r11, r4, 4, 0x1b, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x0000000000000010) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFEF);
	// 823DDBA0: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 823DDBA4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DDBA8: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 823DDBAC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DDBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDBB8 size=12
    let mut pc: u32 = 0x823DDBB8;
    'dispatch: loop {
        match pc {
            0x823DDBB8 => {
    //   block [0x823DDBB8..0x823DDBC4)
	// 823DDBB8: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DDBBC: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 823DDBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDBC8 size=28
    let mut pc: u32 = 0x823DDBC8;
    'dispatch: loop {
        match pc {
            0x823DDBC8 => {
    //   block [0x823DDBC8..0x823DDBE4)
	// 823DDBC8: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DDBCC: 508BC00E  rlwimi r11, r4, 0x18, 0, 7
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[11].u64 & 0xFFFFFFFF00FFFFFF);
	// 823DDBD0: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 823DDBD4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 823DDBD8: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 823DDBDC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 823DDBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDBE8 size=20
    let mut pc: u32 = 0x823DDBE8;
    'dispatch: loop {
        match pc {
            0x823DDBE8 => {
    //   block [0x823DDBE8..0x823DDBFC)
	// 823DDBE8: 8963293C  lbz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 823DDBEC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 823DDBF0: 516A063A  rlwimi r10, r11, 0, 0x18, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000000000FC) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF03);
	// 823DDBF4: 5543063E  clrlwi r3, r10, 0x18
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823DDBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823DDC00 size=36
    let mut pc: u32 = 0x823DDC00;
    'dispatch: loop {
        match pc {
            0x823DDC00 => {
    //   block [0x823DDC00..0x823DDC24)
	// 823DDC00: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 823DDC04: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 823DDC08: 798C07E6  rldicr r12, r12, 0x20, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 823DDC0C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823DDC10: D00329CC  stfs f0, 0x29cc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10700 as u32), tmp.u32 ) };
	// 823DDC14: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 823DDC18: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 823DDC1C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 823DDC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823DDC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823DDC28 size=8
    let mut pc: u32 = 0x823DDC28;
    'dispatch: loop {
        match pc {
            0x823DDC28 => {
    //   block [0x823DDC28..0x823DDC30)
	// 823DDC28: 806329CC  lwz r3, 0x29cc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10700 as u32) ) } as u64;
	// 823DDC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


