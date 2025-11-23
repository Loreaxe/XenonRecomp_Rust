pub fn sub_8246AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246AF38 size=240
    let mut pc: u32 = 0x8246AF38;
    'dispatch: loop {
        match pc {
            0x8246AF38 => {
    //   block [0x8246AF38..0x8246AFE0)
	// 8246AF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246AF3C: 480CA169  bl 0x825350a4
	ctx.lr = 0x8246AF40;
	sub_82535080(ctx, base);
	// 8246AF40: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246AF44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246AF48: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246AF4C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8246AF50: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8246AF54: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246AF58: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8246AF5C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AF60: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8246AF64: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AF68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246AF6C: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 8246AF70: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 8246AF74: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8246AF78: 4BFFF511  bl 0x8246a488
	ctx.lr = 0x8246AF7C;
	sub_8246A488(ctx, base);
	// 8246AF7C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AF80: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 8246AF84: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AF88: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246AF8C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AF90: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8246AF94: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8246AF98: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AF9C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246AFA0: 7C78B82E  lwzx r3, r24, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8246AFA4: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246AFA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246AFAC: 4BFF908D  bl 0x82464038
	ctx.lr = 0x8246AFB0;
	sub_82464038(ctx, base);
	// 8246AFB0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8246AFB4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AFB8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8246AFBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246AFC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AFC4: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8246AFC8: 419A0018  beq cr6, 0x8246afe0
	if ctx.cr[6].eq {
	pc = 0x8246AFE0; continue 'dispatch;
	}
	// 8246AFCC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 8246AFD0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8246AFD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8246AFD8: 4BFFFB11  bl 0x8246aae8
	ctx.lr = 0x8246AFDC;
	sub_8246AAE8(ctx, base);
	// 8246AFDC: 48000008  b 0x8246afe4
	pc = 0x8246AFE4; continue 'dispatch;
            }
            0x8246AFE0 => {
    //   block [0x8246AFE0..0x8246AFE4)
	// 8246AFE0: 480C9B71  bl 0x82534b50
	ctx.lr = 0x8246AFE4;
	sub_82534B50(ctx, base);
	pc = 0x8246AFE4; continue 'dispatch;
            }
            0x8246AFE4 => {
    //   block [0x8246AFE4..0x8246B01C)
	// 8246AFE4: 7D7BEA14  add r11, r27, r29
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[29].u64;
	// 8246AFE8: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8246AFEC: 93BA0004  stw r29, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8246AFF0: 93BA0008  stw r29, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8246AFF4: 9B2BFFFF  stb r25, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[25].u8 ) };
	// 8246AFF8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246AFFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B000: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246B004: 409A0018  bne cr6, 0x8246b01c
	if !ctx.cr[6].eq {
	pc = 0x8246B01C; continue 'dispatch;
	}
	// 8246B008: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246B00C: 7C78B82E  lwzx r3, r24, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8246B010: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246B014: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246B018: 4BFF90A1  bl 0x824640b8
	ctx.lr = 0x8246B01C;
	sub_824640B8(ctx, base);
	pc = 0x8246B01C; continue 'dispatch;
            }
            0x8246B01C => {
    //   block [0x8246B01C..0x8246B028)
	// 8246B01C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246B020: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8246B024: 480CA0D0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246B028 size=396
    let mut pc: u32 = 0x8246B028;
    'dispatch: loop {
        match pc {
            0x8246B028 => {
    //   block [0x8246B028..0x8246B0C4)
	// 8246B028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B02C: 480CA081  bl 0x825350ac
	ctx.lr = 0x8246B030;
	sub_82535080(ctx, base);
	// 8246B030: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B034: 3961006C  addi r11, r1, 0x6c
	ctx.r[11].s64 = ctx.r[1].s64 + 108;
	// 8246B038: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246B03C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8246B040: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8246B044: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8246B048: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8246B04C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8246B050: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8246B054: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B058: 616B000C  ori r11, r11, 0xc
	ctx.r[11].u64 = ctx.r[11].u64 | 12;
	// 8246B05C: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B060: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8246B064: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 8246B068: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B06C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8246B070: 4BFFF419  bl 0x8246a488
	ctx.lr = 0x8246B074;
	sub_8246A488(ctx, base);
	// 8246B074: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246B07C: 419A00FC  beq cr6, 0x8246b178
	if ctx.cr[6].eq {
	pc = 0x8246B178; continue 'dispatch;
	}
	// 8246B080: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B084: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B088: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B08C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8246B090: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8246B094: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8246B098: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246B09C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246B0A0: 40990024  ble cr6, 0x8246b0c4
	if !ctx.cr[6].gt {
	pc = 0x8246B0C4; continue 'dispatch;
	}
	// 8246B0A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B0A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246B0AC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246B0B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246B0B4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246B0B8: 4BFF8F81  bl 0x82464038
	ctx.lr = 0x8246B0BC;
	sub_82464038(ctx, base);
	// 8246B0BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246B0C0: 48000008  b 0x8246b0c8
	pc = 0x8246B0C8; continue 'dispatch;
            }
            0x8246B0C4 => {
    //   block [0x8246B0C4..0x8246B0C8)
	// 8246B0C4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x8246B0C8; continue 'dispatch;
            }
            0x8246B0C8 => {
    //   block [0x8246B0C8..0x8246B124)
	// 8246B0C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B0CC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8246B0D0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8246B0D4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B0D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8246B0DC: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8246B0E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246B0E4: 4BFFFA05  bl 0x8246aae8
	ctx.lr = 0x8246B0E8;
	sub_8246AAE8(ctx, base);
	// 8246B0E8: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 8246B0EC: 9B4BFFFF  stb r26, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[26].u8 ) };
	// 8246B0F0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B0F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246B0F8: 7F1D2040  cmplw cr6, r29, r4
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8246B0FC: 419A0034  beq cr6, 0x8246b130
	if ctx.cr[6].eq {
	pc = 0x8246B130; continue 'dispatch;
	}
	// 8246B100: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B104: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246B108: 409A001C  bne cr6, 0x8246b124
	if !ctx.cr[6].eq {
	pc = 0x8246B124; continue 'dispatch;
	}
	// 8246B10C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B110: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246B114: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246B118: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B11C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246B120: 4BFF8F99  bl 0x824640b8
	ctx.lr = 0x8246B124;
	sub_824640B8(ctx, base);
	pc = 0x8246B124; continue 'dispatch;
            }
            0x8246B124 => {
    //   block [0x8246B124..0x8246B130)
	// 8246B124: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8246B128: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8246B12C: 48000030  b 0x8246b15c
	pc = 0x8246B15C; continue 'dispatch;
            }
            0x8246B130 => {
    //   block [0x8246B130..0x8246B14C)
	// 8246B130: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B134: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246B138: 40980024  bge cr6, 0x8246b15c
	if !ctx.cr[6].lt {
	pc = 0x8246B15C; continue 'dispatch;
	}
	// 8246B13C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B140: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246B144: 41980008  blt cr6, 0x8246b14c
	if ctx.cr[6].lt {
	pc = 0x8246B14C; continue 'dispatch;
	}
	// 8246B148: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246B14C; continue 'dispatch;
            }
            0x8246B14C => {
    //   block [0x8246B14C..0x8246B15C)
	// 8246B14C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B150: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246B158: 48003171  bl 0x8246e2c8
	ctx.lr = 0x8246B15C;
	sub_8246E2C8(ctx, base);
	pc = 0x8246B15C; continue 'dispatch;
            }
            0x8246B15C => {
    //   block [0x8246B15C..0x8246B178)
	// 8246B15C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246B160: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246B164: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246B168: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B16C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246B170: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246B174: 48000014  b 0x8246b188
	pc = 0x8246B188; continue 'dispatch;
            }
            0x8246B178 => {
    //   block [0x8246B178..0x8246B188)
	// 8246B178: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246B17C: 9B590000  stb r26, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8246B180: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B184: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	pc = 0x8246B188; continue 'dispatch;
            }
            0x8246B188 => {
    //   block [0x8246B188..0x8246B1A8)
	// 8246B188: 409A0020  bne cr6, 0x8246b1a8
	if !ctx.cr[6].eq {
	pc = 0x8246B1A8; continue 'dispatch;
	}
	// 8246B18C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B190: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246B194: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246B198: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246B19C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246B1A0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246B1A4: 4BFF8F15  bl 0x824640b8
	ctx.lr = 0x8246B1A8;
	sub_824640B8(ctx, base);
	pc = 0x8246B1A8; continue 'dispatch;
            }
            0x8246B1A8 => {
    //   block [0x8246B1A8..0x8246B1B4)
	// 8246B1A8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8246B1AC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8246B1B0: 480C9F4C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B1B8 size=28
    let mut pc: u32 = 0x8246B1B8;
    'dispatch: loop {
        match pc {
            0x8246B1B8 => {
    //   block [0x8246B1B8..0x8246B1D4)
	// 8246B1B8: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246B1BC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246B1C0: 7D435214  add r10, r3, r10
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 8246B1C4: 396B4BD0  addi r11, r11, 0x4bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 19408;
	// 8246B1C8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246B1CC: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246B1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B1D8 size=8
    let mut pc: u32 = 0x8246B1D8;
    'dispatch: loop {
        match pc {
            0x8246B1D8 => {
    //   block [0x8246B1D8..0x8246B1E0)
	// 8246B1D8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B1E0 size=8
    let mut pc: u32 = 0x8246B1E0;
    'dispatch: loop {
        match pc {
            0x8246B1E0 => {
    //   block [0x8246B1E0..0x8246B1E8)
	// 8246B1E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B1E8 size=8
    let mut pc: u32 = 0x8246B1E8;
    'dispatch: loop {
        match pc {
            0x8246B1E8 => {
    //   block [0x8246B1E8..0x8246B1F0)
	// 8246B1E8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246B1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B1F0 size=12
    let mut pc: u32 = 0x8246B1F0;
    'dispatch: loop {
        match pc {
            0x8246B1F0 => {
    //   block [0x8246B1F0..0x8246B1FC)
	// 8246B1F0: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 8246B1F4: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 8246B1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246B220 size=364
    let mut pc: u32 = 0x8246B220;
    'dispatch: loop {
        match pc {
            0x8246B220 => {
    //   block [0x8246B220..0x8246B2D8)
	// 8246B220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246B228: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246B22C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B230: 8943000C  lbz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246B234: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8246B238: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 8246B23C: 2B09001E  cmplwi cr6, r9, 0x1e
	ctx.cr[6].compare_u32(ctx.r[9].u32, 30 as u32, &mut ctx.xer);
	// 8246B240: 41990134  bgt cr6, 0x8246b374
	if ctx.cr[6].gt {
	pc = 0x8246B374; continue 'dispatch;
	}
	// 8246B244: 3D808247  lis r12, -0x7db9
	ctx.r[12].s64 = -2109276160;
	// 8246B248: 398CB25C  addi r12, r12, -0x4da4
	ctx.r[12].s64 = ctx.r[12].s64 + -19876;
	// 8246B24C: 5520103A  slwi r0, r9, 2
	ctx.r[0].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8246B250: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8246B254: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8246B258: 4E800420  bctr
	match ctx.r[9].u64 {
		0 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		1 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		2 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		3 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		4 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		5 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		6 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		7 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		8 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		9 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		10 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		11 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		12 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		13 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		14 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		15 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		16 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		17 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		18 => {
	pc = 0x8246B374; continue 'dispatch;
		},
		19 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		20 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		21 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		22 => {
	pc = 0x8246B374; continue 'dispatch;
		},
		23 => {
	pc = 0x8246B314; continue 'dispatch;
		},
		24 => {
	pc = 0x8246B354; continue 'dispatch;
		},
		25 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		26 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		27 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		28 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		29 => {
	pc = 0x8246B2D8; continue 'dispatch;
		},
		30 => {
	pc = 0x8246B314; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8246B25C: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B260: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B264: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B268: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B26C: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B270: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B274: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B278: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B27C: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B280: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B284: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B288: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B28C: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B290: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B294: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B298: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B29C: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2A0: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2A4: 8246B374  lwz r18, -0x4c8c(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19596 as u32) ) } as u64;
	// 8246B2A8: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2AC: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2B0: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2B4: 8246B374  lwz r18, -0x4c8c(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19596 as u32) ) } as u64;
	// 8246B2B8: 8246B314  lwz r18, -0x4cec(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19692 as u32) ) } as u64;
	// 8246B2BC: 8246B354  lwz r18, -0x4cac(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19628 as u32) ) } as u64;
	// 8246B2C0: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2C4: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2C8: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2CC: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2D0: 8246B2D8  lwz r18, -0x4d28(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19752 as u32) ) } as u64;
	// 8246B2D4: 8246B314  lwz r18, -0x4cec(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-19692 as u32) ) } as u64;
            }
            0x8246B2D8 => {
    //   block [0x8246B2D8..0x8246B2EC)
	// 8246B2D8: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 8246B2DC: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 8246B2E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246B2E4: 409A0008  bne cr6, 0x8246b2ec
	if !ctx.cr[6].eq {
	pc = 0x8246B2EC; continue 'dispatch;
	}
	// 8246B2E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	pc = 0x8246B2EC; continue 'dispatch;
            }
            0x8246B2EC => {
    //   block [0x8246B2EC..0x8246B314)
	// 8246B2EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246B2F0: 396B4BD0  addi r11, r11, 0x4bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 19408;
	// 8246B2F4: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 8246B2F8: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B2FC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246B300: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B304: 7D6B422E  lhzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8246B308: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246B30C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246B310: 48000064  b 0x8246b374
	pc = 0x8246B374; continue 'dispatch;
            }
            0x8246B314 => {
    //   block [0x8246B314..0x8246B328)
	// 8246B314: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 8246B318: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 8246B31C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246B320: 409A0008  bne cr6, 0x8246b328
	if !ctx.cr[6].eq {
	pc = 0x8246B328; continue 'dispatch;
	}
	// 8246B324: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	pc = 0x8246B328; continue 'dispatch;
            }
            0x8246B328 => {
    //   block [0x8246B328..0x8246B354)
	// 8246B328: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8246B32C: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 8246B330: 394A4BD0  addi r10, r10, 0x4bd0
	ctx.r[10].s64 = ctx.r[10].s64 + 19408;
	// 8246B334: 390A0008  addi r8, r10, 8
	ctx.r[8].s64 = ctx.r[10].s64 + 8;
	// 8246B338: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 8246B33C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246B340: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B344: 7D6B422E  lhzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8246B348: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246B34C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246B350: 48000024  b 0x8246b374
	pc = 0x8246B374; continue 'dispatch;
            }
            0x8246B354 => {
    //   block [0x8246B354..0x8246B368)
	// 8246B354: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 8246B358: 7D7F0734  extsh r31, r11
	ctx.r[31].s64 = ctx.r[11].s16 as i64;
	// 8246B35C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8246B360: 409A0008  bne cr6, 0x8246b368
	if !ctx.cr[6].eq {
	pc = 0x8246B368; continue 'dispatch;
	}
	// 8246B364: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	pc = 0x8246B368; continue 'dispatch;
            }
            0x8246B368 => {
    //   block [0x8246B368..0x8246B374)
	// 8246B368: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B36C: 4BFFB94D  bl 0x82466cb8
	ctx.lr = 0x8246B370;
	sub_82466CB8(ctx, base);
	// 8246B370: 7D63F9D6  mullw r11, r3, r31
	ctx.r[11].s32 = ((ctx.r[3].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	pc = 0x8246B374; continue 'dispatch;
            }
            0x8246B374 => {
    //   block [0x8246B374..0x8246B38C)
	// 8246B374: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8246B378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246B384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246B388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246B3A8 size=236
    let mut pc: u32 = 0x8246B3A8;
    'dispatch: loop {
        match pc {
            0x8246B3A8 => {
    //   block [0x8246B3A8..0x8246B3CC)
	// 8246B3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B3AC: 480C9D11  bl 0x825350bc
	ctx.lr = 0x8246B3B0;
	sub_82535080(ctx, base);
	// 8246B3B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B3B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246B3B8: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246B3BC: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 8246B3C0: 419A000C  beq cr6, 0x8246b3cc
	if ctx.cr[6].eq {
	pc = 0x8246B3CC; continue 'dispatch;
	}
	// 8246B3C4: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 8246B3C8: 409A0008  bne cr6, 0x8246b3d0
	if !ctx.cr[6].eq {
	pc = 0x8246B3D0; continue 'dispatch;
	}
	pc = 0x8246B3CC; continue 'dispatch;
            }
            0x8246B3CC => {
    //   block [0x8246B3CC..0x8246B3D0)
	// 8246B3CC: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	pc = 0x8246B3D0; continue 'dispatch;
            }
            0x8246B3D0 => {
    //   block [0x8246B3D0..0x8246B3F0)
	// 8246B3D0: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 8246B3D4: 409A0064  bne cr6, 0x8246b438
	if !ctx.cr[6].eq {
	pc = 0x8246B438; continue 'dispatch;
	}
	// 8246B3D8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B3DC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8246B3E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246B3E4: 4BFFB68D  bl 0x82466a70
	ctx.lr = 0x8246B3E8;
	sub_82466A70(ctx, base);
	// 8246B3E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246B3EC: 40990044  ble cr6, 0x8246b430
	if !ctx.cr[6].gt {
	pc = 0x8246B430; continue 'dispatch;
	}
	pc = 0x8246B3F0; continue 'dispatch;
            }
            0x8246B3F0 => {
    //   block [0x8246B3F0..0x8246B41C)
	// 8246B3F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246B3F4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B3F8: 4BFFB6A1  bl 0x82466a98
	ctx.lr = 0x8246B3FC;
	sub_82466A98(ctx, base);
	// 8246B3FC: 4BFFFFAD  bl 0x8246b3a8
	ctx.lr = 0x8246B400;
	sub_8246B3A8(ctx, base);
	// 8246B400: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8246B404: 40990018  ble cr6, 0x8246b41c
	if !ctx.cr[6].gt {
	pc = 0x8246B41C; continue 'dispatch;
	}
	// 8246B408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246B40C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B410: 4BFFB689  bl 0x82466a98
	ctx.lr = 0x8246B414;
	sub_82466A98(ctx, base);
	// 8246B414: 4BFFFF95  bl 0x8246b3a8
	ctx.lr = 0x8246B418;
	sub_8246B3A8(ctx, base);
	// 8246B418: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	pc = 0x8246B41C; continue 'dispatch;
            }
            0x8246B41C => {
    //   block [0x8246B41C..0x8246B430)
	// 8246B41C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B420: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8246B424: 4BFFB64D  bl 0x82466a70
	ctx.lr = 0x8246B428;
	sub_82466A70(ctx, base);
	// 8246B428: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8246B42C: 4198FFC4  blt cr6, 0x8246b3f0
	if ctx.cr[6].lt {
	pc = 0x8246B3F0; continue 'dispatch;
	}
	pc = 0x8246B430; continue 'dispatch;
            }
            0x8246B430 => {
    //   block [0x8246B430..0x8246B438)
	// 8246B430: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246B434: 48000024  b 0x8246b458
	pc = 0x8246B458; continue 'dispatch;
            }
            0x8246B438 => {
    //   block [0x8246B438..0x8246B458)
	// 8246B438: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8246B43C: 394A4BD0  addi r10, r10, 0x4bd0
	ctx.r[10].s64 = ctx.r[10].s64 + 19408;
	// 8246B440: 392A000A  addi r9, r10, 0xa
	ctx.r[9].s64 = ctx.r[10].s64 + 10;
	// 8246B444: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246B448: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246B44C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B450: 7D6B4A2E  lhzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246B454: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	pc = 0x8246B458; continue 'dispatch;
            }
            0x8246B458 => {
    //   block [0x8246B458..0x8246B48C)
	// 8246B458: A17E0010  lhz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246B45C: 556A05F0  rlwinm r10, r11, 0, 0x17, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B460: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246B464: 419A0028  beq cr6, 0x8246b48c
	if ctx.cr[6].eq {
	pc = 0x8246B48C; continue 'dispatch;
	}
	// 8246B468: 556BC63E  rlwinm r11, r11, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8246B46C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8246B470: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 8246B474: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8246B478: 556B0738  rlwinm r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B47C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8246B480: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8246B484: 40990008  ble cr6, 0x8246b48c
	if !ctx.cr[6].gt {
	pc = 0x8246B48C; continue 'dispatch;
	}
	// 8246B488: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	pc = 0x8246B48C; continue 'dispatch;
            }
            0x8246B48C => {
    //   block [0x8246B48C..0x8246B494)
	// 8246B48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246B490: 480C9C7C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B498 size=8
    let mut pc: u32 = 0x8246B498;
    'dispatch: loop {
        match pc {
            0x8246B498 => {
    //   block [0x8246B498..0x8246B4A0)
	// 8246B498: 8863000D  lbz r3, 0xd(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 8246B49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B4A0 size=64
    let mut pc: u32 = 0x8246B4A0;
    'dispatch: loop {
        match pc {
            0x8246B4A0 => {
    //   block [0x8246B4A0..0x8246B4E0)
	// 8246B4A0: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 8246B4A4: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 8246B4A8: 419A0040  beq cr6, 0x8246b4e8
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246B4E8);
		return;
	}
	// 8246B4AC: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 8246B4B0: 419A0038  beq cr6, 0x8246b4e8
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246B4E8);
		return;
	}
	// 8246B4B4: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 8246B4B8: 419A0028  beq cr6, 0x8246b4e0
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246B4E0);
		return;
	}
	// 8246B4BC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8246B4C0: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8246B4C4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8246B4C8: 394A4BD0  addi r10, r10, 0x4bd0
	ctx.r[10].s64 = ctx.r[10].s64 + 19408;
	// 8246B4CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B4D0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8246B4D4: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246B4D8: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 8246B4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246B4F0 size=116
    let mut pc: u32 = 0x8246B4F0;
    'dispatch: loop {
        match pc {
            0x8246B4F0 => {
    //   block [0x8246B4F0..0x8246B530)
	// 8246B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246B4F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246B4FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246B500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B504: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246B508: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8246B50C: 4BFFFD15  bl 0x8246b220
	ctx.lr = 0x8246B510;
	sub_8246B220(ctx, base);
	// 8246B510: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8246B514: 419A0028  beq cr6, 0x8246b53c
	if ctx.cr[6].eq {
	pc = 0x8246B53C; continue 'dispatch;
	}
	// 8246B518: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8246B51C: 419A0014  beq cr6, 0x8246b530
	if ctx.cr[6].eq {
	pc = 0x8246B530; continue 'dispatch;
	}
	// 8246B520: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8246B524: 409A0024  bne cr6, 0x8246b548
	if !ctx.cr[6].eq {
	pc = 0x8246B548; continue 'dispatch;
	}
	// 8246B528: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B52C: 48000020  b 0x8246b54c
	pc = 0x8246B54C; continue 'dispatch;
            }
            0x8246B530 => {
    //   block [0x8246B530..0x8246B53C)
	// 8246B530: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B534: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 8246B538: 48000014  b 0x8246b54c
	pc = 0x8246B54C; continue 'dispatch;
            }
            0x8246B53C => {
    //   block [0x8246B53C..0x8246B548)
	// 8246B53C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B540: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8246B544: 48000008  b 0x8246b54c
	pc = 0x8246B54C; continue 'dispatch;
            }
            0x8246B548 => {
    //   block [0x8246B548..0x8246B54C)
	// 8246B548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x8246B54C; continue 'dispatch;
            }
            0x8246B54C => {
    //   block [0x8246B54C..0x8246B564)
	// 8246B54C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246B550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246B554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246B558: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246B55C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246B560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246B568 size=100
    let mut pc: u32 = 0x8246B568;
    'dispatch: loop {
        match pc {
            0x8246B568 => {
    //   block [0x8246B568..0x8246B5A8)
	// 8246B568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246B570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246B574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246B578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B57C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246B580: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246B584: 4BFFFC9D  bl 0x8246b220
	ctx.lr = 0x8246B588;
	sub_8246B220(ctx, base);
	// 8246B588: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8246B58C: 419A0024  beq cr6, 0x8246b5b0
	if ctx.cr[6].eq {
	pc = 0x8246B5B0; continue 'dispatch;
	}
	// 8246B590: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8246B594: 419A0014  beq cr6, 0x8246b5a8
	if ctx.cr[6].eq {
	pc = 0x8246B5A8; continue 'dispatch;
	}
	// 8246B598: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8246B59C: 409A0018  bne cr6, 0x8246b5b4
	if !ctx.cr[6].eq {
	pc = 0x8246B5B4; continue 'dispatch;
	}
	// 8246B5A0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8246B5A4: 48000010  b 0x8246b5b4
	pc = 0x8246B5B4; continue 'dispatch;
            }
            0x8246B5A8 => {
    //   block [0x8246B5A8..0x8246B5B0)
	// 8246B5A8: B3DF0000  sth r30, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8246B5AC: 48000008  b 0x8246b5b4
	pc = 0x8246B5B4; continue 'dispatch;
            }
            0x8246B5B0 => {
    //   block [0x8246B5B0..0x8246B5B4)
	// 8246B5B0: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	pc = 0x8246B5B4; continue 'dispatch;
            }
            0x8246B5B4 => {
    //   block [0x8246B5B4..0x8246B5CC)
	// 8246B5B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246B5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246B5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246B5C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246B5C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246B5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246B5D0 size=284
    let mut pc: u32 = 0x8246B5D0;
    'dispatch: loop {
        match pc {
            0x8246B5D0 => {
    //   block [0x8246B5D0..0x8246B64C)
	// 8246B5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B5D4: 480C9AE9  bl 0x825350bc
	ctx.lr = 0x8246B5D8;
	sub_82535080(ctx, base);
	// 8246B5D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B5DC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8246B5E0: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8246B5E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246B5E8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246B5EC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8246B5F0: 394A4BD0  addi r10, r10, 0x4bd0
	ctx.r[10].s64 = ctx.r[10].s64 + 19408;
	// 8246B5F4: 409A00CC  bne cr6, 0x8246b6c0
	if !ctx.cr[6].eq {
	pc = 0x8246B6C0; continue 'dispatch;
	}
	// 8246B5F8: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 8246B5FC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246B600: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246B604: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B608: 7FAB482E  lwzx r29, r11, r9
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246B60C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8246B610: 419A0068  beq cr6, 0x8246b678
	if ctx.cr[6].eq {
	pc = 0x8246B678; continue 'dispatch;
	}
	// 8246B614: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246B61C: 419A005C  beq cr6, 0x8246b678
	if ctx.cr[6].eq {
	pc = 0x8246B678; continue 'dispatch;
	}
	// 8246B620: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246B624: 4BFFEBFD  bl 0x8246a220
	ctx.lr = 0x8246B628;
	sub_8246A220(ctx, base);
	// 8246B628: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246B62C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246B630: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B634: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246B638: 40980024  bge cr6, 0x8246b65c
	if !ctx.cr[6].lt {
	pc = 0x8246B65C; continue 'dispatch;
	}
	// 8246B63C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B640: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246B644: 41980008  blt cr6, 0x8246b64c
	if ctx.cr[6].lt {
	pc = 0x8246B64C; continue 'dispatch;
	}
	// 8246B648: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246B64C; continue 'dispatch;
            }
            0x8246B64C => {
    //   block [0x8246B64C..0x8246B65C)
	// 8246B64C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B650: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B654: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B658: 48002C71  bl 0x8246e2c8
	ctx.lr = 0x8246B65C;
	sub_8246E2C8(ctx, base);
	pc = 0x8246B65C; continue 'dispatch;
            }
            0x8246B65C => {
    //   block [0x8246B65C..0x8246B678)
	// 8246B65C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246B660: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B664: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246B668: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8246B66C: 4BFFECBD  bl 0x8246a328
	ctx.lr = 0x8246B670;
	sub_8246A328(ctx, base);
	// 8246B670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246B674: 480C9A98  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246B678 => {
    //   block [0x8246B678..0x8246B698)
	// 8246B678: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246B67C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B680: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8246B684: 40980020  bge cr6, 0x8246b6a4
	if !ctx.cr[6].lt {
	pc = 0x8246B6A4; continue 'dispatch;
	}
	// 8246B688: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246B68C: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8246B690: 41990008  bgt cr6, 0x8246b698
	if ctx.cr[6].gt {
	pc = 0x8246B698; continue 'dispatch;
	}
	// 8246B694: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	pc = 0x8246B698; continue 'dispatch;
            }
            0x8246B698 => {
    //   block [0x8246B698..0x8246B6A4)
	// 8246B698: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B69C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B6A0: 48002C29  bl 0x8246e2c8
	ctx.lr = 0x8246B6A4;
	sub_8246E2C8(ctx, base);
	pc = 0x8246B6A4; continue 'dispatch;
            }
            0x8246B6A4 => {
    //   block [0x8246B6A4..0x8246B6C0)
	// 8246B6A4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246B6A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B6AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246B6B0: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8246B6B4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246B6B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246B6BC: 480C9A50  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246B6C0 => {
    //   block [0x8246B6C0..0x8246B6EC)
	// 8246B6C0: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 8246B6C4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246B6C8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 8246B6CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246B6D0: 388984A4  addi r4, r9, -0x7b5c
	ctx.r[4].s64 = ctx.r[9].s64 + -31580;
	// 8246B6D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B6D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B6DC: 7CAB402E  lwzx r5, r11, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8246B6E0: 4BFFEE81  bl 0x8246a560
	ctx.lr = 0x8246B6E4;
	sub_8246A560(ctx, base);
	// 8246B6E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246B6E8: 480C9A24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246B6F0 size=924
    let mut pc: u32 = 0x8246B6F0;
    'dispatch: loop {
        match pc {
            0x8246B6F0 => {
    //   block [0x8246B6F0..0x8246B72C)
	// 8246B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246B6F4: 480C99AD  bl 0x825350a0
	ctx.lr = 0x8246B6F8;
	sub_82535080(ctx, base);
	// 8246B6F8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246B6FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246B700: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8246B704: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8246B708: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 8246B70C: 3BCBCD5C  addi r30, r11, -0x32a4
	ctx.r[30].s64 = ctx.r[11].s64 + -12964;
	// 8246B710: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B714: 8BFD000C  lbz r31, 0xc(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246B718: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246B71C: 419A0010  beq cr6, 0x8246b72c
	if ctx.cr[6].eq {
	pc = 0x8246B72C; continue 'dispatch;
	}
	// 8246B720: 4BFFB139  bl 0x82466858
	ctx.lr = 0x8246B724;
	sub_82466858(ctx, base);
	// 8246B724: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246B728: 48000008  b 0x8246b730
	pc = 0x8246B730; continue 'dispatch;
            }
            0x8246B72C => {
    //   block [0x8246B72C..0x8246B730)
	// 8246B72C: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	pc = 0x8246B730; continue 'dispatch;
            }
            0x8246B730 => {
    //   block [0x8246B730..0x8246B740)
	// 8246B730: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246B734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246B738: 419A0008  beq cr6, 0x8246b740
	if ctx.cr[6].eq {
	pc = 0x8246B740; continue 'dispatch;
	}
	// 8246B73C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x8246B740; continue 'dispatch;
            }
            0x8246B740 => {
    //   block [0x8246B740..0x8246B7B8)
	// 8246B740: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B744: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 8246B748: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246B74C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246B750: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246B754: 4BFF88E5  bl 0x82464038
	ctx.lr = 0x8246B758;
	sub_82464038(ctx, base);
	// 8246B758: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8246B75C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8246B760: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8246B764: 2F1F0014  cmpwi cr6, r31, 0x14
	ctx.cr[6].compare_i32(ctx.r[31].s32, 20, &mut ctx.xer);
	// 8246B768: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8246B76C: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 8246B770: 9B630000  stb r27, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8246B774: 419802BC  blt cr6, 0x8246ba30
	if ctx.cr[6].lt {
	pc = 0x8246BA30; continue 'dispatch;
	}
	// 8246B778: 2F1F001B  cmpwi cr6, r31, 0x1b
	ctx.cr[6].compare_i32(ctx.r[31].s32, 27, &mut ctx.xer);
	// 8246B77C: 419A02B4  beq cr6, 0x8246ba30
	if ctx.cr[6].eq {
	pc = 0x8246BA30; continue 'dispatch;
	}
	// 8246B780: 2F1F001D  cmpwi cr6, r31, 0x1d
	ctx.cr[6].compare_i32(ctx.r[31].s32, 29, &mut ctx.xer);
	// 8246B784: 419A02AC  beq cr6, 0x8246ba30
	if ctx.cr[6].eq {
	pc = 0x8246BA30; continue 'dispatch;
	}
	// 8246B788: 2F1F0014  cmpwi cr6, r31, 0x14
	ctx.cr[6].compare_i32(ctx.r[31].s32, 20, &mut ctx.xer);
	// 8246B78C: 409A013C  bne cr6, 0x8246b8c8
	if !ctx.cr[6].eq {
	pc = 0x8246B8C8; continue 'dispatch;
	}
	// 8246B790: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B794: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246B798: 419A0020  beq cr6, 0x8246b7b8
	if ctx.cr[6].eq {
	pc = 0x8246B7B8; continue 'dispatch;
	}
	// 8246B79C: 4BFFB0BD  bl 0x82466858
	ctx.lr = 0x8246B7A0;
	sub_82466858(ctx, base);
	// 8246B7A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B7A4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246B7A8: 388B8564  addi r4, r11, -0x7a9c
	ctx.r[4].s64 = ctx.r[11].s64 + -31388;
	// 8246B7AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B7B0: 4BFFEDB1  bl 0x8246a560
	ctx.lr = 0x8246B7B4;
	sub_8246A560(ctx, base);
	// 8246B7B4: 48000290  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B7B8 => {
    //   block [0x8246B7B8..0x8246B804)
	// 8246B7B8: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 8246B7BC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8246B7C0: 409A00AC  bne cr6, 0x8246b86c
	if !ctx.cr[6].eq {
	pc = 0x8246B86C; continue 'dispatch;
	}
	// 8246B7C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B7C8: 3BCB855C  addi r30, r11, -0x7aa4
	ctx.r[30].s64 = ctx.r[11].s64 + -31396;
	// 8246B7CC: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B7D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246B7D4: 419A0058  beq cr6, 0x8246b82c
	if ctx.cr[6].eq {
	pc = 0x8246B82C; continue 'dispatch;
	}
	// 8246B7D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B7DC: 4BFFEA45  bl 0x8246a220
	ctx.lr = 0x8246B7E0;
	sub_8246A220(ctx, base);
	// 8246B7E0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246B7E4: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246B7E8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B7EC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246B7F0: 40980024  bge cr6, 0x8246b814
	if !ctx.cr[6].lt {
	pc = 0x8246B814; continue 'dispatch;
	}
	// 8246B7F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B7F8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246B7FC: 41980008  blt cr6, 0x8246b804
	if ctx.cr[6].lt {
	pc = 0x8246B804; continue 'dispatch;
	}
	// 8246B800: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246B804; continue 'dispatch;
            }
            0x8246B804 => {
    //   block [0x8246B804..0x8246B814)
	// 8246B804: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B808: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B80C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B810: 48002AB9  bl 0x8246e2c8
	ctx.lr = 0x8246B814;
	sub_8246E2C8(ctx, base);
	pc = 0x8246B814; continue 'dispatch;
            }
            0x8246B814 => {
    //   block [0x8246B814..0x8246B82C)
	// 8246B814: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246B818: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246B81C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246B820: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8246B824: 4BFFEB05  bl 0x8246a328
	ctx.lr = 0x8246B828;
	sub_8246A328(ctx, base);
	// 8246B828: 4800021C  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B82C => {
    //   block [0x8246B82C..0x8246B84C)
	// 8246B82C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246B830: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B834: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8246B838: 40980024  bge cr6, 0x8246b85c
	if !ctx.cr[6].lt {
	pc = 0x8246B85C; continue 'dispatch;
	}
	// 8246B83C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B840: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8246B844: 41990008  bgt cr6, 0x8246b84c
	if ctx.cr[6].gt {
	pc = 0x8246B84C; continue 'dispatch;
	}
	// 8246B848: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x8246B84C; continue 'dispatch;
            }
            0x8246B84C => {
    //   block [0x8246B84C..0x8246B85C)
	// 8246B84C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B850: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B854: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B858: 48002A71  bl 0x8246e2c8
	ctx.lr = 0x8246B85C;
	sub_8246E2C8(ctx, base);
	pc = 0x8246B85C; continue 'dispatch;
            }
            0x8246B85C => {
    //   block [0x8246B85C..0x8246B86C)
	// 8246B85C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246B860: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8246B864: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8246B868: 480001DC  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B86C => {
    //   block [0x8246B86C..0x8246B8A0)
	// 8246B86C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B870: 3BCB8554  addi r30, r11, -0x7aac
	ctx.r[30].s64 = ctx.r[11].s64 + -31404;
	// 8246B874: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B878: 4BFFE9A9  bl 0x8246a220
	ctx.lr = 0x8246B87C;
	sub_8246A220(ctx, base);
	// 8246B87C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246B880: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246B884: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246B888: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246B88C: 40980024  bge cr6, 0x8246b8b0
	if !ctx.cr[6].lt {
	pc = 0x8246B8B0; continue 'dispatch;
	}
	// 8246B890: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B894: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246B898: 41980008  blt cr6, 0x8246b8a0
	if ctx.cr[6].lt {
	pc = 0x8246B8A0; continue 'dispatch;
	}
	// 8246B89C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246B8A0; continue 'dispatch;
            }
            0x8246B8A0 => {
    //   block [0x8246B8A0..0x8246B8B0)
	// 8246B8A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B8A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B8A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B8AC: 48002A1D  bl 0x8246e2c8
	ctx.lr = 0x8246B8B0;
	sub_8246E2C8(ctx, base);
	pc = 0x8246B8B0; continue 'dispatch;
            }
            0x8246B8B0 => {
    //   block [0x8246B8B0..0x8246B8C8)
	// 8246B8B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246B8B4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246B8B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246B8BC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8246B8C0: 4BFFEA69  bl 0x8246a328
	ctx.lr = 0x8246B8C4;
	sub_8246A328(ctx, base);
	// 8246B8C4: 48000180  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B8C8 => {
    //   block [0x8246B8C8..0x8246B8F8)
	// 8246B8C8: 2F1F0016  cmpwi cr6, r31, 0x16
	ctx.cr[6].compare_i32(ctx.r[31].s32, 22, &mut ctx.xer);
	// 8246B8CC: 419A008C  beq cr6, 0x8246b958
	if ctx.cr[6].eq {
	pc = 0x8246B958; continue 'dispatch;
	}
	// 8246B8D0: 2F1F001A  cmpwi cr6, r31, 0x1a
	ctx.cr[6].compare_i32(ctx.r[31].s32, 26, &mut ctx.xer);
	// 8246B8D4: 419A0084  beq cr6, 0x8246b958
	if ctx.cr[6].eq {
	pc = 0x8246B958; continue 'dispatch;
	}
	// 8246B8D8: 2F1F0018  cmpwi cr6, r31, 0x18
	ctx.cr[6].compare_i32(ctx.r[31].s32, 24, &mut ctx.xer);
	// 8246B8DC: 409A001C  bne cr6, 0x8246b8f8
	if !ctx.cr[6].eq {
	pc = 0x8246B8F8; continue 'dispatch;
	}
	// 8246B8E0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B8E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246B8E8: 388B854C  addi r4, r11, -0x7ab4
	ctx.r[4].s64 = ctx.r[11].s64 + -31412;
	// 8246B8EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B8F0: 4BFFEC71  bl 0x8246a560
	ctx.lr = 0x8246B8F4;
	sub_8246A560(ctx, base);
	// 8246B8F4: 48000150  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B8F8 => {
    //   block [0x8246B8F8..0x8246B918)
	// 8246B8F8: 2F1F001F  cmpwi cr6, r31, 0x1f
	ctx.cr[6].compare_i32(ctx.r[31].s32, 31, &mut ctx.xer);
	// 8246B8FC: 409A001C  bne cr6, 0x8246b918
	if !ctx.cr[6].eq {
	pc = 0x8246B918; continue 'dispatch;
	}
	// 8246B900: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B904: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246B908: 388B8540  addi r4, r11, -0x7ac0
	ctx.r[4].s64 = ctx.r[11].s64 + -31424;
	// 8246B90C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B910: 4BFFEC51  bl 0x8246a560
	ctx.lr = 0x8246B914;
	sub_8246A560(ctx, base);
	// 8246B914: 48000130  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B918 => {
    //   block [0x8246B918..0x8246B948)
	// 8246B918: 2F1F0019  cmpwi cr6, r31, 0x19
	ctx.cr[6].compare_i32(ctx.r[31].s32, 25, &mut ctx.xer);
	// 8246B91C: 409A0128  bne cr6, 0x8246ba44
	if !ctx.cr[6].eq {
	pc = 0x8246BA44; continue 'dispatch;
	}
	// 8246B920: A17D000E  lhz r11, 0xe(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(14 as u32) ) } as u64;
	// 8246B924: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8246B928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B92C: 7D660734  extsh r6, r11
	ctx.r[6].s64 = ctx.r[11].s16 as i64;
	// 8246B930: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8246B934: 409A0014  bne cr6, 0x8246b948
	if !ctx.cr[6].eq {
	pc = 0x8246B948; continue 'dispatch;
	}
	// 8246B938: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B93C: 388B8534  addi r4, r11, -0x7acc
	ctx.r[4].s64 = ctx.r[11].s64 + -31436;
	// 8246B940: 4BFFEC21  bl 0x8246a560
	ctx.lr = 0x8246B944;
	sub_8246A560(ctx, base);
	// 8246B944: 48000100  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B948 => {
    //   block [0x8246B948..0x8246B958)
	// 8246B948: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B94C: 388B8524  addi r4, r11, -0x7adc
	ctx.r[4].s64 = ctx.r[11].s64 + -31452;
	// 8246B950: 4BFFEC11  bl 0x8246a560
	ctx.lr = 0x8246B954;
	sub_8246A560(ctx, base);
	// 8246B954: 480000F0  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B958 => {
    //   block [0x8246B958..0x8246B970)
	// 8246B958: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 8246B95C: 2F1F0016  cmpwi cr6, r31, 0x16
	ctx.cr[6].compare_i32(ctx.r[31].s32, 22, &mut ctx.xer);
	// 8246B960: 409A0010  bne cr6, 0x8246b970
	if !ctx.cr[6].eq {
	pc = 0x8246B970; continue 'dispatch;
	}
	// 8246B964: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246B968: 38AA851C  addi r5, r10, -0x7ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -31460;
	// 8246B96C: 4800000C  b 0x8246b978
	pc = 0x8246B978; continue 'dispatch;
            }
            0x8246B970 => {
    //   block [0x8246B970..0x8246B978)
	// 8246B970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246B974: 38AA850C  addi r5, r10, -0x7af4
	ctx.r[5].s64 = ctx.r[10].s64 + -31476;
	pc = 0x8246B978; continue 'dispatch;
            }
            0x8246B978 => {
    //   block [0x8246B978..0x8246B9B4)
	// 8246B978: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 8246B97C: 41980084  blt cr6, 0x8246ba00
	if ctx.cr[6].lt {
	pc = 0x8246BA00; continue 'dispatch;
	}
	// 8246B980: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 8246B984: 419A007C  beq cr6, 0x8246ba00
	if ctx.cr[6].eq {
	pc = 0x8246BA00; continue 'dispatch;
	}
	// 8246B988: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 8246B98C: 409A0038  bne cr6, 0x8246b9c4
	if !ctx.cr[6].eq {
	pc = 0x8246B9C4; continue 'dispatch;
	}
	// 8246B990: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246B99C: 419A0018  beq cr6, 0x8246b9b4
	if ctx.cr[6].eq {
	pc = 0x8246B9B4; continue 'dispatch;
	}
	// 8246B9A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B9A4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8246B9A8: 388B84FC  addi r4, r11, -0x7b04
	ctx.r[4].s64 = ctx.r[11].s64 + -31492;
	// 8246B9AC: 4BFFEBB5  bl 0x8246a560
	ctx.lr = 0x8246B9B0;
	sub_8246A560(ctx, base);
	// 8246B9B0: 48000094  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B9B4 => {
    //   block [0x8246B9B4..0x8246B9C4)
	// 8246B9B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B9B8: 388B84EC  addi r4, r11, -0x7b14
	ctx.r[4].s64 = ctx.r[11].s64 + -31508;
	// 8246B9BC: 4BFFEBA5  bl 0x8246a560
	ctx.lr = 0x8246B9C0;
	sub_8246A560(ctx, base);
	// 8246B9C0: 48000084  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B9C4 => {
    //   block [0x8246B9C4..0x8246B9E4)
	// 8246B9C4: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 8246B9C8: 409A001C  bne cr6, 0x8246b9e4
	if !ctx.cr[6].eq {
	pc = 0x8246B9E4; continue 'dispatch;
	}
	// 8246B9CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B9D0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8246B9D4: 388B84D8  addi r4, r11, -0x7b28
	ctx.r[4].s64 = ctx.r[11].s64 + -31528;
	// 8246B9D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B9DC: 4BFFEB85  bl 0x8246a560
	ctx.lr = 0x8246B9E0;
	sub_8246A560(ctx, base);
	// 8246B9E0: 48000064  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246B9E4 => {
    //   block [0x8246B9E4..0x8246BA00)
	// 8246B9E4: 2F0B001C  cmpwi cr6, r11, 0x1c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 28, &mut ctx.xer);
	// 8246B9E8: 409A005C  bne cr6, 0x8246ba44
	if !ctx.cr[6].eq {
	pc = 0x8246BA44; continue 'dispatch;
	}
	// 8246B9EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B9F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B9F4: 388B84BC  addi r4, r11, -0x7b44
	ctx.r[4].s64 = ctx.r[11].s64 + -31556;
	// 8246B9F8: 4BFFEB69  bl 0x8246a560
	ctx.lr = 0x8246B9FC;
	sub_8246A560(ctx, base);
	// 8246B9FC: 48000048  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246BA00 => {
    //   block [0x8246BA00..0x8246BA30)
	// 8246BA00: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 8246BA04: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 8246BA08: 394A4BD0  addi r10, r10, 0x4bd0
	ctx.r[10].s64 = ctx.r[10].s64 + 19408;
	// 8246BA0C: 388984AC  addi r4, r9, -0x7b54
	ctx.r[4].s64 = ctx.r[9].s64 + -31572;
	// 8246BA10: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 8246BA14: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246BA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BA1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246BA20: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246BA24: 7CCB482E  lwzx r6, r11, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246BA28: 4BFFEB39  bl 0x8246a560
	ctx.lr = 0x8246BA2C;
	sub_8246A560(ctx, base);
	// 8246BA2C: 48000018  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246BA30 => {
    //   block [0x8246BA30..0x8246BA44)
	// 8246BA30: A17D000E  lhz r11, 0xe(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(14 as u32) ) } as u64;
	// 8246BA34: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8246BA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BA3C: 7D640734  extsh r4, r11
	ctx.r[4].s64 = ctx.r[11].s16 as i64;
	// 8246BA40: 4BFFFB91  bl 0x8246b5d0
	ctx.lr = 0x8246BA44;
	sub_8246B5D0(ctx, base);
	pc = 0x8246BA44; continue 'dispatch;
            }
            0x8246BA44 => {
    //   block [0x8246BA44..0x8246BA80)
	// 8246BA44: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8246BA48: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BA4C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8246BA50: 4BFFE7C1  bl 0x8246a210
	ctx.lr = 0x8246BA54;
	sub_8246A210(ctx, base);
	// 8246BA54: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246BA58: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8246BA5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BA60: 3BE9FFFF  addi r31, r9, -1
	ctx.r[31].s64 = ctx.r[9].s64 + -1;
	// 8246BA64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BA68: 409A0018  bne cr6, 0x8246ba80
	if !ctx.cr[6].eq {
	pc = 0x8246BA80; continue 'dispatch;
	}
	// 8246BA6C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BA70: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246BA74: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BA78: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BA7C: 4BFF863D  bl 0x824640b8
	ctx.lr = 0x8246BA80;
	sub_824640B8(ctx, base);
	pc = 0x8246BA80; continue 'dispatch;
            }
            0x8246BA80 => {
    //   block [0x8246BA80..0x8246BA8C)
	// 8246BA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BA84: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8246BA88: 480C9668  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246BA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246BA90 size=812
    let mut pc: u32 = 0x8246BA90;
    'dispatch: loop {
        match pc {
            0x8246BA90 => {
    //   block [0x8246BA90..0x8246BAC4)
	// 8246BA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246BA94: 480C9621  bl 0x825350b4
	ctx.lr = 0x8246BA98;
	sub_82535080(ctx, base);
	// 8246BA98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246BA9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BAA0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8246BAA4: 388B8594  addi r4, r11, -0x7a6c
	ctx.r[4].s64 = ctx.r[11].s64 + -31340;
	// 8246BAA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246BAAC: 4BFFE555  bl 0x8246a000
	ctx.lr = 0x8246BAB0;
	sub_8246A000(ctx, base);
	// 8246BAB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BAB4: 409A0010  bne cr6, 0x8246bac4
	if !ctx.cr[6].eq {
	pc = 0x8246BAC4; continue 'dispatch;
	}
	// 8246BAB8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8246BABC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BAC0: 480C9644  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BAC4 => {
    //   block [0x8246BAC4..0x8246BAEC)
	// 8246BAC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BAC8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8246BACC: 388B858C  addi r4, r11, -0x7a74
	ctx.r[4].s64 = ctx.r[11].s64 + -31348;
	// 8246BAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BAD4: 4BFFE52D  bl 0x8246a000
	ctx.lr = 0x8246BAD8;
	sub_8246A000(ctx, base);
	// 8246BAD8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BADC: 409A0010  bne cr6, 0x8246baec
	if !ctx.cr[6].eq {
	pc = 0x8246BAEC; continue 'dispatch;
	}
	// 8246BAE0: 3860001F  li r3, 0x1f
	ctx.r[3].s64 = 31;
	// 8246BAE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BAE8: 480C961C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BAEC => {
    //   block [0x8246BAEC..0x8246BB14)
	// 8246BAEC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BAF0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246BAF4: 388B8580  addi r4, r11, -0x7a80
	ctx.r[4].s64 = ctx.r[11].s64 + -31360;
	// 8246BAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BAFC: 4BFFE505  bl 0x8246a000
	ctx.lr = 0x8246BB00;
	sub_8246A000(ctx, base);
	// 8246BB00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BB04: 409A0010  bne cr6, 0x8246bb14
	if !ctx.cr[6].eq {
	pc = 0x8246BB14; continue 'dispatch;
	}
	// 8246BB08: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8246BB0C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BB10: 480C95F4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BB14 => {
    //   block [0x8246BB14..0x8246BB3C)
	// 8246BB14: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BB18: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 8246BB1C: 388B8570  addi r4, r11, -0x7a90
	ctx.r[4].s64 = ctx.r[11].s64 + -31376;
	// 8246BB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BB24: 4BFFE4DD  bl 0x8246a000
	ctx.lr = 0x8246BB28;
	sub_8246A000(ctx, base);
	// 8246BB28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BB2C: 409A0010  bne cr6, 0x8246bb3c
	if !ctx.cr[6].eq {
	pc = 0x8246BB3C; continue 'dispatch;
	}
	// 8246BB30: 3860001A  li r3, 0x1a
	ctx.r[3].s64 = 26;
	// 8246BB34: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BB38: 480C95CC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BB3C => {
    //   block [0x8246BB3C..0x8246BB64)
	// 8246BB3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BB40: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8246BB44: 388B855C  addi r4, r11, -0x7aa4
	ctx.r[4].s64 = ctx.r[11].s64 + -31396;
	// 8246BB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BB4C: 4BFFE4B5  bl 0x8246a000
	ctx.lr = 0x8246BB50;
	sub_8246A000(ctx, base);
	// 8246BB50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BB54: 409A0010  bne cr6, 0x8246bb64
	if !ctx.cr[6].eq {
	pc = 0x8246BB64; continue 'dispatch;
	}
	// 8246BB58: 3860001D  li r3, 0x1d
	ctx.r[3].s64 = 29;
	// 8246BB5C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BB60: 480C95A4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BB64 => {
    //   block [0x8246BB64..0x8246BB90)
	// 8246BB64: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 8246BB68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BB6C: 4BFFE725  bl 0x8246a290
	ctx.lr = 0x8246BB70;
	sub_8246A290(ctx, base);
	// 8246BB70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246BB74: 419A001C  beq cr6, 0x8246bb90
	if ctx.cr[6].eq {
	pc = 0x8246BB90; continue 'dispatch;
	}
	// 8246BB78: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8246BB7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246BB80: 409A0010  bne cr6, 0x8246bb90
	if !ctx.cr[6].eq {
	pc = 0x8246BB90; continue 'dispatch;
	}
	// 8246BB84: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8246BB88: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BB8C: 480C9578  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BB90 => {
    //   block [0x8246BB90..0x8246BBD8)
	// 8246BB90: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8246BB94: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 8246BB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8246BB9C: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 8246BBA0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8246BBA4: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 8246BBA8: 419A0058  beq cr6, 0x8246bc00
	if ctx.cr[6].eq {
	pc = 0x8246BC00; continue 'dispatch;
	}
	// 8246BBAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BBB0: 4BFFE671  bl 0x8246a220
	ctx.lr = 0x8246BBB4;
	sub_8246A220(ctx, base);
	// 8246BBB4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246BBB8: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 8246BBBC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BBC0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246BBC4: 40980024  bge cr6, 0x8246bbe8
	if !ctx.cr[6].lt {
	pc = 0x8246BBE8; continue 'dispatch;
	}
	// 8246BBC8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246BBCC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246BBD0: 41980008  blt cr6, 0x8246bbd8
	if ctx.cr[6].lt {
	pc = 0x8246BBD8; continue 'dispatch;
	}
	// 8246BBD4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246BBD8; continue 'dispatch;
            }
            0x8246BBD8 => {
    //   block [0x8246BBD8..0x8246BBE8)
	// 8246BBD8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BBDC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246BBE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BBE4: 480026E5  bl 0x8246e2c8
	ctx.lr = 0x8246BBE8;
	sub_8246E2C8(ctx, base);
	pc = 0x8246BBE8; continue 'dispatch;
            }
            0x8246BBE8 => {
    //   block [0x8246BBE8..0x8246BC00)
	// 8246BBE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246BBEC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BBF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246BBF4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8246BBF8: 4BFFE731  bl 0x8246a328
	ctx.lr = 0x8246BBFC;
	sub_8246A328(ctx, base);
	// 8246BBFC: 48000024  b 0x8246bc20
	pc = 0x8246BC20; continue 'dispatch;
            }
            0x8246BC00 => {
    //   block [0x8246BC00..0x8246BC20)
	// 8246BC00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BC04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246BC08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BC0C: 480026BD  bl 0x8246e2c8
	ctx.lr = 0x8246BC10;
	sub_8246E2C8(ctx, base);
	// 8246BC10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246BC14: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8246BC18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BC1C: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	pc = 0x8246BC20; continue 'dispatch;
            }
            0x8246BC20 => {
    //   block [0x8246BC20..0x8246BC4C)
	// 8246BC20: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 8246BC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BC28: 4BFFE661  bl 0x8246a288
	ctx.lr = 0x8246BC2C;
	sub_8246A288(ctx, base);
	// 8246BC2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246BC30: 419A00E4  beq cr6, 0x8246bd14
	if ctx.cr[6].eq {
	pc = 0x8246BD14; continue 'dispatch;
	}
	// 8246BC34: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8246BC38: 7FFF1850  subf r31, r31, r3
	ctx.r[31].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8246BC3C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246BC40: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246BC44: 40990008  ble cr6, 0x8246bc4c
	if !ctx.cr[6].gt {
	pc = 0x8246BC4C; continue 'dispatch;
	}
	// 8246BC48: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	pc = 0x8246BC4C; continue 'dispatch;
            }
            0x8246BC4C => {
    //   block [0x8246BC4C..0x8246BC74)
	// 8246BC4C: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 8246BC50: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BC54: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 8246BC58: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 8246BC5C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246BC60: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 8246BC64: 4099001C  ble cr6, 0x8246bc80
	if !ctx.cr[6].gt {
	pc = 0x8246BC80; continue 'dispatch;
	}
	// 8246BC68: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246BC6C: 41980008  blt cr6, 0x8246bc74
	if ctx.cr[6].lt {
	pc = 0x8246BC74; continue 'dispatch;
	}
	// 8246BC70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x8246BC74; continue 'dispatch;
            }
            0x8246BC74 => {
    //   block [0x8246BC74..0x8246BC80)
	// 8246BC74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BC78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246BC7C: 4800264D  bl 0x8246e2c8
	ctx.lr = 0x8246BC80;
	sub_8246E2C8(ctx, base);
	pc = 0x8246BC80; continue 'dispatch;
            }
            0x8246BC80 => {
    //   block [0x8246BC80..0x8246BCC4)
	// 8246BC80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246BC84: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BC88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246BC8C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8246BC90: 4BFFE699  bl 0x8246a328
	ctx.lr = 0x8246BC94;
	sub_8246A328(ctx, base);
	// 8246BC94: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BC98: 7F6BF9AE  stbx r27, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[27].u8) };
	// 8246BC9C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246BCA0: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BCA4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8246BCA8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8246BCAC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246BCB0: 40980024  bge cr6, 0x8246bcd4
	if !ctx.cr[6].lt {
	pc = 0x8246BCD4; continue 'dispatch;
	}
	// 8246BCB4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246BCB8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246BCBC: 40980008  bge cr6, 0x8246bcc4
	if !ctx.cr[6].lt {
	pc = 0x8246BCC4; continue 'dispatch;
	}
	// 8246BCC0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x8246BCC4; continue 'dispatch;
            }
            0x8246BCC4 => {
    //   block [0x8246BCC4..0x8246BCD4)
	// 8246BCC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BCC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246BCCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BCD0: 480025F9  bl 0x8246e2c8
	ctx.lr = 0x8246BCD4;
	sub_8246E2C8(ctx, base);
	pc = 0x8246BCD4; continue 'dispatch;
            }
            0x8246BCD4 => {
    //   block [0x8246BCD4..0x8246BD14)
	// 8246BCD4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246BCD8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BCDC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BCE0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8246BCE4: 4BFFE645  bl 0x8246a328
	ctx.lr = 0x8246BCE8;
	sub_8246A328(ctx, base);
	// 8246BCE8: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246BCEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BCF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BCF4: 409A0020  bne cr6, 0x8246bd14
	if !ctx.cr[6].eq {
	pc = 0x8246BD14; continue 'dispatch;
	}
	// 8246BCF8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BCFC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246BD00: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BD04: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BD08: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BD0C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246BD10: 4BFF83A9  bl 0x824640b8
	ctx.lr = 0x8246BD14;
	sub_824640B8(ctx, base);
	pc = 0x8246BD14; continue 'dispatch;
            }
            0x8246BD14 => {
    //   block [0x8246BD14..0x8246BD24)
	// 8246BD14: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246BD18: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8246BD1C: 3BAB4BD0  addi r29, r11, 0x4bd0
	ctx.r[29].s64 = ctx.r[11].s64 + 19408;
	// 8246BD20: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	pc = 0x8246BD24; continue 'dispatch;
            }
            0x8246BD24 => {
    //   block [0x8246BD24..0x8246BD78)
	// 8246BD24: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BD28: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BD2C: 4BFFE2A5  bl 0x82469fd0
	ctx.lr = 0x8246BD30;
	sub_82469FD0(ctx, base);
	// 8246BD30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BD34: 419A0050  beq cr6, 0x8246bd84
	if ctx.cr[6].eq {
	pc = 0x8246BD84; continue 'dispatch;
	}
	// 8246BD38: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8246BD3C: 397D0184  addi r11, r29, 0x184
	ctx.r[11].s64 = ctx.r[29].s64 + 388;
	// 8246BD40: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8246BD44: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246BD48: 4198FFDC  blt cr6, 0x8246bd24
	if ctx.cr[6].lt {
	pc = 0x8246BD24; continue 'dispatch;
	}
	// 8246BD4C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246BD50: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BD54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BD58: 409A0020  bne cr6, 0x8246bd78
	if !ctx.cr[6].eq {
	pc = 0x8246BD78; continue 'dispatch;
	}
	// 8246BD5C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BD60: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246BD64: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BD68: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BD6C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BD70: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246BD74: 4BFF8345  bl 0x824640b8
	ctx.lr = 0x8246BD78;
	sub_824640B8(ctx, base);
	pc = 0x8246BD78; continue 'dispatch;
            }
            0x8246BD78 => {
    //   block [0x8246BD78..0x8246BD84)
	// 8246BD78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246BD7C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BD80: 480C9384  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BD84 => {
    //   block [0x8246BD84..0x8246BDB0)
	// 8246BD84: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246BD88: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BD8C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BD90: 409A0020  bne cr6, 0x8246bdb0
	if !ctx.cr[6].eq {
	pc = 0x8246BDB0; continue 'dispatch;
	}
	// 8246BD94: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BD98: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246BD9C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BDA0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BDA4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BDA8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246BDAC: 4BFF830D  bl 0x824640b8
	ctx.lr = 0x8246BDB0;
	sub_824640B8(ctx, base);
	pc = 0x8246BDB0; continue 'dispatch;
            }
            0x8246BDB0 => {
    //   block [0x8246BDB0..0x8246BDBC)
	// 8246BDB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246BDB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BDB8: 480C934C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246BDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246BDC0 size=612
    let mut pc: u32 = 0x8246BDC0;
    'dispatch: loop {
        match pc {
            0x8246BDC0 => {
    //   block [0x8246BDC0..0x8246BE24)
	// 8246BDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246BDC4: 480C92F5  bl 0x825350b8
	ctx.lr = 0x8246BDC8;
	sub_82535080(ctx, base);
	// 8246BDC8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246BDCC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BDD0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246BDD4: 388B8580  addi r4, r11, -0x7a80
	ctx.r[4].s64 = ctx.r[11].s64 + -31360;
	// 8246BDD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246BDDC: 4BFFE225  bl 0x8246a000
	ctx.lr = 0x8246BDE0;
	sub_8246A000(ctx, base);
	// 8246BDE0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BDE4: 409A00AC  bne cr6, 0x8246be90
	if !ctx.cr[6].eq {
	pc = 0x8246BE90; continue 'dispatch;
	}
	// 8246BDE8: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 8246BDEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246BDF0: 4BFFE431  bl 0x8246a220
	ctx.lr = 0x8246BDF4;
	sub_8246A220(ctx, base);
	// 8246BDF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246BDF8: 3BC3FFFF  addi r30, r3, -1
	ctx.r[30].s64 = ctx.r[3].s64 + -1;
	// 8246BDFC: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8246BE00: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 8246BE04: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8246BE08: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246BE0C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8246BE10: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8246BE14: 4099001C  ble cr6, 0x8246be30
	if !ctx.cr[6].gt {
	pc = 0x8246BE30; continue 'dispatch;
	}
	// 8246BE18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246BE1C: 41980008  blt cr6, 0x8246be24
	if ctx.cr[6].lt {
	pc = 0x8246BE24; continue 'dispatch;
	}
	// 8246BE20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8246BE24; continue 'dispatch;
            }
            0x8246BE24 => {
    //   block [0x8246BE24..0x8246BE30)
	// 8246BE24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BE28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BE2C: 4800249D  bl 0x8246e2c8
	ctx.lr = 0x8246BE30;
	sub_8246E2C8(ctx, base);
	pc = 0x8246BE30; continue 'dispatch;
            }
            0x8246BE30 => {
    //   block [0x8246BE30..0x8246BE84)
	// 8246BE30: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246BE34: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BE38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246BE3C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8246BE40: 4BFFE4E9  bl 0x8246a328
	ctx.lr = 0x8246BE44;
	sub_8246A328(ctx, base);
	// 8246BE44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BE48: 7FEBF1AE  stbx r31, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u8) };
	// 8246BE4C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BE50: 4BFFFC41  bl 0x8246ba90
	ctx.lr = 0x8246BE54;
	sub_8246BA90(ctx, base);
	// 8246BE54: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246BE58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246BE5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BE60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BE64: 409A0020  bne cr6, 0x8246be84
	if !ctx.cr[6].eq {
	pc = 0x8246BE84; continue 'dispatch;
	}
	// 8246BE68: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BE6C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246BE70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BE74: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246BE78: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BE7C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246BE80: 4BFF8239  bl 0x824640b8
	ctx.lr = 0x8246BE84;
	sub_824640B8(ctx, base);
	pc = 0x8246BE84; continue 'dispatch;
            }
            0x8246BE84 => {
    //   block [0x8246BE84..0x8246BE90)
	// 8246BE84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BE88: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246BE8C: 480C927C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BE90 => {
    //   block [0x8246BE90..0x8246BEE8)
	// 8246BE90: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246BE94: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 8246BE98: 388B8570  addi r4, r11, -0x7a90
	ctx.r[4].s64 = ctx.r[11].s64 + -31376;
	// 8246BE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BEA0: 4BFFE161  bl 0x8246a000
	ctx.lr = 0x8246BEA4;
	sub_8246A000(ctx, base);
	// 8246BEA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246BEA8: 409A00AC  bne cr6, 0x8246bf54
	if !ctx.cr[6].eq {
	pc = 0x8246BF54; continue 'dispatch;
	}
	// 8246BEAC: 3B9F000E  addi r28, r31, 0xe
	ctx.r[28].s64 = ctx.r[31].s64 + 14;
	// 8246BEB0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246BEB4: 4BFFE36D  bl 0x8246a220
	ctx.lr = 0x8246BEB8;
	sub_8246A220(ctx, base);
	// 8246BEB8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246BEBC: 3BC3FFFF  addi r30, r3, -1
	ctx.r[30].s64 = ctx.r[3].s64 + -1;
	// 8246BEC0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8246BEC4: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 8246BEC8: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8246BECC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246BED0: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8246BED4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8246BED8: 4099001C  ble cr6, 0x8246bef4
	if !ctx.cr[6].gt {
	pc = 0x8246BEF4; continue 'dispatch;
	}
	// 8246BEDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246BEE0: 41980008  blt cr6, 0x8246bee8
	if ctx.cr[6].lt {
	pc = 0x8246BEE8; continue 'dispatch;
	}
	// 8246BEE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8246BEE8; continue 'dispatch;
            }
            0x8246BEE8 => {
    //   block [0x8246BEE8..0x8246BEF4)
	// 8246BEE8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BEEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246BEF0: 480023D9  bl 0x8246e2c8
	ctx.lr = 0x8246BEF4;
	sub_8246E2C8(ctx, base);
	pc = 0x8246BEF4; continue 'dispatch;
            }
            0x8246BEF4 => {
    //   block [0x8246BEF4..0x8246BF48)
	// 8246BEF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246BEF8: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BEFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246BF00: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 8246BF04: 4BFFE425  bl 0x8246a328
	ctx.lr = 0x8246BF08;
	sub_8246A328(ctx, base);
	// 8246BF08: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BF0C: 7FEBF1AE  stbx r31, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u8) };
	// 8246BF10: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BF14: 4BFFFB7D  bl 0x8246ba90
	ctx.lr = 0x8246BF18;
	sub_8246BA90(ctx, base);
	// 8246BF18: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246BF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246BF20: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BF24: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BF28: 409A0020  bne cr6, 0x8246bf48
	if !ctx.cr[6].eq {
	pc = 0x8246BF48; continue 'dispatch;
	}
	// 8246BF2C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BF30: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246BF34: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BF38: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246BF3C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246BF40: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246BF44: 4BFF8175  bl 0x824640b8
	ctx.lr = 0x8246BF48;
	sub_824640B8(ctx, base);
	pc = 0x8246BF48; continue 'dispatch;
            }
            0x8246BF48 => {
    //   block [0x8246BF48..0x8246BF54)
	// 8246BF48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BF4C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246BF50: 480C91B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246BF54 => {
    //   block [0x8246BF54..0x8246BFAC)
	// 8246BF54: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 8246BF58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BF5C: 4BFFE32D  bl 0x8246a288
	ctx.lr = 0x8246BF60;
	sub_8246A288(ctx, base);
	// 8246BF60: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246BF64: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8246BF68: 419A00B0  beq cr6, 0x8246c018
	if ctx.cr[6].eq {
	pc = 0x8246C018; continue 'dispatch;
	}
	// 8246BF6C: 3880005D  li r4, 0x5d
	ctx.r[4].s64 = 93;
	// 8246BF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BF74: 4BFFE315  bl 0x8246a288
	ctx.lr = 0x8246BF78;
	sub_8246A288(ctx, base);
	// 8246BF78: 7D7C1850  subf r11, r28, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[28].s64;
	// 8246BF7C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246BF80: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 8246BF84: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8246BF88: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 8246BF8C: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 8246BF90: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246BF94: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 8246BF98: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8246BF9C: 4099001C  ble cr6, 0x8246bfb8
	if !ctx.cr[6].gt {
	pc = 0x8246BFB8; continue 'dispatch;
	}
	// 8246BFA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246BFA4: 41980008  blt cr6, 0x8246bfac
	if ctx.cr[6].lt {
	pc = 0x8246BFAC; continue 'dispatch;
	}
	// 8246BFA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8246BFAC; continue 'dispatch;
            }
            0x8246BFAC => {
    //   block [0x8246BFAC..0x8246BFB8)
	// 8246BFAC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BFB0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8246BFB4: 48002315  bl 0x8246e2c8
	ctx.lr = 0x8246BFB8;
	sub_8246E2C8(ctx, base);
	pc = 0x8246BFB8; continue 'dispatch;
            }
            0x8246BFB8 => {
    //   block [0x8246BFB8..0x8246C00C)
	// 8246BFB8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246BFBC: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246BFC0: 389C0001  addi r4, r28, 1
	ctx.r[4].s64 = ctx.r[28].s64 + 1;
	// 8246BFC4: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 8246BFC8: 4BFFE361  bl 0x8246a328
	ctx.lr = 0x8246BFCC;
	sub_8246A328(ctx, base);
	// 8246BFCC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246BFD0: 7FEBF1AE  stbx r31, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u8) };
	// 8246BFD4: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246BFD8: 4BFFE271  bl 0x8246a248
	ctx.lr = 0x8246BFDC;
	sub_8246A248(ctx, base);
	// 8246BFDC: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8246BFE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246BFE4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246BFE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246BFEC: 409A0020  bne cr6, 0x8246c00c
	if !ctx.cr[6].eq {
	pc = 0x8246C00C; continue 'dispatch;
	}
	// 8246BFF0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246BFF4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246BFF8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246BFFC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246C000: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246C004: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246C008: 4BFF80B1  bl 0x824640b8
	ctx.lr = 0x8246C00C;
	sub_824640B8(ctx, base);
	pc = 0x8246C00C; continue 'dispatch;
            }
            0x8246C00C => {
    //   block [0x8246C00C..0x8246C018)
	// 8246C00C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246C010: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246C014: 480C90F4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246C018 => {
    //   block [0x8246C018..0x8246C024)
	// 8246C018: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246C01C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246C020: 480C90E8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C028 size=60
    let mut pc: u32 = 0x8246C028;
    'dispatch: loop {
        match pc {
            0x8246C028 => {
    //   block [0x8246C028..0x8246C064)
	// 8246C028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C034: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C038: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246C03C: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 8246C040: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 8246C044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C048: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C04C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C050: 4E800421  bctrl
	ctx.lr = 0x8246C054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C054: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C068 size=60
    let mut pc: u32 = 0x8246C068;
    'dispatch: loop {
        match pc {
            0x8246C068 => {
    //   block [0x8246C068..0x8246C0A4)
	// 8246C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C074: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C078: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246C07C: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 8246C080: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 8246C084: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C088: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C090: 4E800421  bctrl
	ctx.lr = 0x8246C094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246C0C0 size=456
    let mut pc: u32 = 0x8246C0C0;
    'dispatch: loop {
        match pc {
            0x8246C0C0 => {
    //   block [0x8246C0C0..0x8246C100)
	// 8246C0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C0C4: 480C8FE5  bl 0x825350a8
	ctx.lr = 0x8246C0C8;
	sub_82535080(ctx, base);
	// 8246C0C8: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C0CC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8246C0D0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8246C0D4: 8978000C  lbz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246C0D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C0DC: 409A0024  bne cr6, 0x8246c100
	if !ctx.cr[6].eq {
	pc = 0x8246C100; continue 'dispatch;
	}
	// 8246C0E0: 80780008  lwz r3, 8(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C0E4: 7CBC31D6  mullw r5, r28, r6
	ctx.r[5].s32 = ((ctx.r[28].s32 as i64 * ctx.r[6].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246C0E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C0EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C0F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C0F4: 4E800421  bctrl
	ctx.lr = 0x8246C0F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C0F8: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 8246C0FC: 480C8FFC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x8246C100 => {
    //   block [0x8246C100..0x8246C144)
	// 8246C100: 7FFC31D6  mullw r31, r28, r6
	ctx.r[31].s32 = ((ctx.r[28].s32 as i64 * ctx.r[6].s32 as i64) as i32);
	ctx.r[31].s64 = ctx.r[31].s32 as i64;
	// 8246C104: 7FEB4E70  srawi r11, r31, 9
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 9) as i64;
	// 8246C108: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 8246C10C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8246C110: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8246C114: 7FBEE3D6  divw r29, r30, r28
	ctx.r[29].s32 = ctx.r[30].s32 / ctx.r[28].s32;
	// 8246C118: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246C11C: 0CDC0000  twi 6, r28, 0
	// 8246C120: 7F4BF850  subf r26, r11, r31
	ctx.r[26].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8246C124: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8246C128: 574B083E  rotlwi r11, r26, 1
	ctx.r[11].u64 = ((ctx.r[26].u32).rotate_left(1)) as u64;
	// 8246C12C: 7F3AE3D6  divw r25, r26, r28
	ctx.r[25].s32 = ctx.r[26].s32 / ctx.r[28].s32;
	// 8246C130: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246C134: 0CDC0000  twi 6, r28, 0
	// 8246C138: 7F8B5878  andc r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 & !ctx.r[11].u64;
	// 8246C13C: 0CABFFFF  twi 5, r11, -1
	// 8246C140: 40990140  ble cr6, 0x8246c280
	if !ctx.cr[6].gt {
	pc = 0x8246C280; continue 'dispatch;
	}
	pc = 0x8246C144; continue 'dispatch;
            }
            0x8246C144 => {
    //   block [0x8246C144..0x8246C154)
	// 8246C144: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 8246C148: 4098000C  bge cr6, 0x8246c154
	if !ctx.cr[6].lt {
	pc = 0x8246C154; continue 'dispatch;
	}
	// 8246C14C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8246C150: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	pc = 0x8246C154; continue 'dispatch;
            }
            0x8246C154 => {
    //   block [0x8246C154..0x8246C18C)
	// 8246C154: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246C158: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246C15C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246C160: 4BFFE1C9  bl 0x8246a328
	ctx.lr = 0x8246C164;
	sub_8246A328(ctx, base);
	// 8246C164: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 8246C168: 419A00BC  beq cr6, 0x8246c224
	if ctx.cr[6].eq {
	pc = 0x8246C224; continue 'dispatch;
	}
	// 8246C16C: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 8246C170: 419A0070  beq cr6, 0x8246c1e0
	if ctx.cr[6].eq {
	pc = 0x8246C1E0; continue 'dispatch;
	}
	// 8246C174: 2F1C0008  cmpwi cr6, r28, 8
	ctx.cr[6].compare_i32(ctx.r[28].s32, 8, &mut ctx.xer);
	// 8246C178: 409A00DC  bne cr6, 0x8246c254
	if !ctx.cr[6].eq {
	pc = 0x8246C254; continue 'dispatch;
	}
	// 8246C17C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246C180: 409900D4  ble cr6, 0x8246c254
	if !ctx.cr[6].gt {
	pc = 0x8246C254; continue 'dispatch;
	}
	// 8246C184: 39610056  addi r11, r1, 0x56
	ctx.r[11].s64 = ctx.r[1].s64 + 86;
	// 8246C188: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8246C18C; continue 'dispatch;
            }
            0x8246C18C => {
    //   block [0x8246C18C..0x8246C1E0)
	// 8246C18C: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8246C190: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246C194: 892BFFFA  lbz r9, -6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-6 as u32) ) } as u64;
	// 8246C198: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246C19C: 990BFFFA  stb r8, -6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-6 as u32), ctx.r[8].u8 ) };
	// 8246C1A0: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 8246C1A4: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C1A8: 892BFFFB  lbz r9, -5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-5 as u32) ) } as u64;
	// 8246C1AC: 990BFFFB  stb r8, -5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-5 as u32), ctx.r[8].u8 ) };
	// 8246C1B0: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246C1B4: 890BFFFF  lbz r8, -1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8246C1B8: 892BFFFC  lbz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8246C1BC: 990BFFFC  stb r8, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u8 ) };
	// 8246C1C0: 992BFFFF  stb r9, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 8246C1C4: 890BFFFE  lbz r8, -2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 8246C1C8: 892BFFFD  lbz r9, -3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-3 as u32) ) } as u64;
	// 8246C1CC: 990BFFFD  stb r8, -3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-3 as u32), ctx.r[8].u8 ) };
	// 8246C1D0: 992BFFFE  stb r9, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[9].u8 ) };
	// 8246C1D4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8246C1D8: 409AFFB4  bne cr6, 0x8246c18c
	if !ctx.cr[6].eq {
	pc = 0x8246C18C; continue 'dispatch;
	}
	// 8246C1DC: 48000078  b 0x8246c254
	pc = 0x8246C254; continue 'dispatch;
            }
            0x8246C1E0 => {
    //   block [0x8246C1E0..0x8246C1F0)
	// 8246C1E0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246C1E4: 40990070  ble cr6, 0x8246c254
	if !ctx.cr[6].gt {
	pc = 0x8246C254; continue 'dispatch;
	}
	// 8246C1E8: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 8246C1EC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8246C1F0; continue 'dispatch;
            }
            0x8246C1F0 => {
    //   block [0x8246C1F0..0x8246C224)
	// 8246C1F0: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8246C1F4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246C1F8: 892BFFFE  lbz r9, -2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 8246C1FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246C200: 990BFFFE  stb r8, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[8].u8 ) };
	// 8246C204: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 8246C208: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C20C: 892BFFFF  lbz r9, -1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8246C210: 990BFFFF  stb r8, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[8].u8 ) };
	// 8246C214: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246C218: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8246C21C: 409AFFD4  bne cr6, 0x8246c1f0
	if !ctx.cr[6].eq {
	pc = 0x8246C1F0; continue 'dispatch;
	}
	// 8246C220: 48000034  b 0x8246c254
	pc = 0x8246C254; continue 'dispatch;
            }
            0x8246C224 => {
    //   block [0x8246C224..0x8246C234)
	// 8246C224: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8246C228: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246C22C: 40990028  ble cr6, 0x8246c254
	if !ctx.cr[6].gt {
	pc = 0x8246C254; continue 'dispatch;
	}
	// 8246C230: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8246C234; continue 'dispatch;
            }
            0x8246C234 => {
    //   block [0x8246C234..0x8246C254)
	// 8246C234: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8246C238: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246C23C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C240: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246C244: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8246C248: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 8246C24C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8246C250: 409AFFE4  bne cr6, 0x8246c234
	if !ctx.cr[6].eq {
	pc = 0x8246C234; continue 'dispatch;
	}
	pc = 0x8246C254; continue 'dispatch;
            }
            0x8246C254 => {
    //   block [0x8246C254..0x8246C280)
	// 8246C254: 80780008  lwz r3, 8(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C258: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246C25C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8246C260: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C264: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C26C: 4E800421  bctrl
	ctx.lr = 0x8246C270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C270: 7FFEF850  subf r31, r30, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 8246C274: 7F7EDA14  add r27, r30, r27
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 8246C278: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8246C27C: 4199FEC8  bgt cr6, 0x8246c144
	if ctx.cr[6].gt {
	pc = 0x8246C144; continue 'dispatch;
	}
            }
            0x8246C280 => {
    //   block [0x8246C280..0x8246C288)
	// 8246C280: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 8246C284: 480C8E74  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C288 size=20
    let mut pc: u32 = 0x8246C288;
    'dispatch: loop {
        match pc {
            0x8246C288 => {
    //   block [0x8246C288..0x8246C29C)
	// 8246C288: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C28C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C290: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C298: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C2B8 size=64
    let mut pc: u32 = 0x8246C2B8;
    'dispatch: loop {
        match pc {
            0x8246C2B8 => {
    //   block [0x8246C2B8..0x8246C2F8)
	// 8246C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C2C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246C2C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C2C8: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C2D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C2D4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246C2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C2DC: 4E800421  bctrl
	ctx.lr = 0x8246C2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246C2E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C2F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246C2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C2F8 size=8
    let mut pc: u32 = 0x8246C2F8;
    'dispatch: loop {
        match pc {
            0x8246C2F8 => {
    //   block [0x8246C2F8..0x8246C300)
	// 8246C2F8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C300 size=140
    let mut pc: u32 = 0x8246C300;
    'dispatch: loop {
        match pc {
            0x8246C300 => {
    //   block [0x8246C300..0x8246C334)
	// 8246C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246C30C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246C310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C314: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246C318: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246C31C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C324: 419A0010  beq cr6, 0x8246c334
	if ctx.cr[6].eq {
	pc = 0x8246C334; continue 'dispatch;
	}
	// 8246C328: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246C32C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246C330: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8246C334; continue 'dispatch;
            }
            0x8246C334 => {
    //   block [0x8246C334..0x8246C370)
	// 8246C334: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C338: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C33C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C340: 419A0030  beq cr6, 0x8246c370
	if ctx.cr[6].eq {
	pc = 0x8246C370; continue 'dispatch;
	}
	// 8246C344: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246C348: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246C34C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246C350: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246C354: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246C358: 409A0018  bne cr6, 0x8246c370
	if !ctx.cr[6].eq {
	pc = 0x8246C370; continue 'dispatch;
	}
	// 8246C35C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C360: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246C364: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C36C: 4E800421  bctrl
	ctx.lr = 0x8246C370;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246C370 => {
    //   block [0x8246C370..0x8246C38C)
	// 8246C370: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8246C374: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246C378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C380: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246C384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246C388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C390 size=56
    let mut pc: u32 = 0x8246C390;
    'dispatch: loop {
        match pc {
            0x8246C390 => {
    //   block [0x8246C390..0x8246C3C8)
	// 8246C390: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246C394: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8246C398: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246C39C: 98A3000C  stb r5, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u8 ) };
	// 8246C3A0: 396B86F0  addi r11, r11, -0x7910
	ctx.r[11].s64 = ctx.r[11].s64 + -30992;
	// 8246C3A4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246C3A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C3AC: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C3B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C3B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246C3B8: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246C3BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246C3C0: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246C3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C3C8 size=100
    let mut pc: u32 = 0x8246C3C8;
    'dispatch: loop {
        match pc {
            0x8246C3C8 => {
    //   block [0x8246C3C8..0x8246C42C)
	// 8246C3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C3D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246C3D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C3D8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246C3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C3E0: 396B86F0  addi r11, r11, -0x7910
	ctx.r[11].s64 = ctx.r[11].s64 + -30992;
	// 8246C3E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246C3E8: 98BF000C  stb r5, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u8 ) };
	// 8246C3EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C3F0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8246C3F4: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246C3F8: 806B91D4  lwz r3, -0x6e2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28204 as u32) ) } as u64;
	// 8246C3FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C400: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C408: 4E800421  bctrl
	ctx.lr = 0x8246C40C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C40C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246C410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246C414: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246C418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246C428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C430 size=128
    let mut pc: u32 = 0x8246C430;
    'dispatch: loop {
        match pc {
            0x8246C430 => {
    //   block [0x8246C430..0x8246C4B0)
	// 8246C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C434: 480C8C89  bl 0x825350bc
	ctx.lr = 0x8246C438;
	sub_82535080(ctx, base);
	// 8246C438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C43C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246C440: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C448: 396B86F0  addi r11, r11, -0x7910
	ctx.r[11].s64 = ctx.r[11].s64 + -30992;
	// 8246C44C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246C450: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8246C454: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246C458: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8246C45C: 98DF000C  stb r6, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u8 ) };
	// 8246C460: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 8246C464: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C468: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8246C46C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246C470: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246C474: 4BFF7BC5  bl 0x82464038
	ctx.lr = 0x8246C478;
	sub_82464038(ctx, base);
	// 8246C478: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246C47C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8246C480: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 8246C484: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246C488: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246C48C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8246C490: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C494: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8246C498: 48002BE1  bl 0x8246f078
	ctx.lr = 0x8246C49C;
	sub_8246F078(ctx, base);
	// 8246C49C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246C4A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246C4A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246C4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246C4AC: 480C8C60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C4B0 size=136
    let mut pc: u32 = 0x8246C4B0;
    'dispatch: loop {
        match pc {
            0x8246C4B0 => {
    //   block [0x8246C4B0..0x8246C538)
	// 8246C4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C4B4: 480C8C05  bl 0x825350b8
	ctx.lr = 0x8246C4B8;
	sub_82535080(ctx, base);
	// 8246C4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C4BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246C4C0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C4C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246C4C8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8246C4CC: 394A86F0  addi r10, r10, -0x7910
	ctx.r[10].s64 = ctx.r[10].s64 + -30992;
	// 8246C4D0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8246C4D4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8246C4D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246C4DC: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 8246C4E0: 997E000C  stb r11, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 8246C4E4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8246C4E8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246C4EC: B39E0006  sth r28, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 8246C4F0: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246C4F4: 4BFF7B45  bl 0x82464038
	ctx.lr = 0x8246C4F8;
	sub_82464038(ctx, base);
	// 8246C4F8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246C4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C500: 396B833C  addi r11, r11, -0x7cc4
	ctx.r[11].s64 = ctx.r[11].s64 + -31940;
	// 8246C504: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8246C508: B39F0006  sth r28, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 8246C50C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C510: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8246C514: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8246C518: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C51C: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8246C520: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8246C524: 4BFFC525  bl 0x82468a48
	ctx.lr = 0x8246C528;
	sub_82468A48(ctx, base);
	// 8246C528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246C52C: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8246C530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246C534: 480C8BD4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C538 size=124
    let mut pc: u32 = 0x8246C538;
    'dispatch: loop {
        match pc {
            0x8246C538 => {
    //   block [0x8246C538..0x8246C594)
	// 8246C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246C544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C548: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C54C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246C550: 396B86F0  addi r11, r11, -0x7910
	ctx.r[11].s64 = ctx.r[11].s64 + -30992;
	// 8246C554: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C558: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C55C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C560: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C564: 419A0030  beq cr6, 0x8246c594
	if ctx.cr[6].eq {
	pc = 0x8246C594; continue 'dispatch;
	}
	// 8246C568: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246C56C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246C570: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246C574: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246C578: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246C57C: 409A0018  bne cr6, 0x8246c594
	if !ctx.cr[6].eq {
	pc = 0x8246C594; continue 'dispatch;
	}
	// 8246C580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C584: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246C588: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C58C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C590: 4E800421  bctrl
	ctx.lr = 0x8246C594;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246C594 => {
    //   block [0x8246C594..0x8246C5B4)
	// 8246C594: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246C598: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246C59C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C5AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246C5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C5B8 size=48
    let mut pc: u32 = 0x8246C5B8;
    'dispatch: loop {
        match pc {
            0x8246C5B8 => {
    //   block [0x8246C5B8..0x8246C5E8)
	// 8246C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C5C4: B081007E  sth r4, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[4].u16 ) };
	// 8246C5C8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C5CC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8246C5D0: 3881007E  addi r4, r1, 0x7e
	ctx.r[4].s64 = ctx.r[1].s64 + 126;
	// 8246C5D4: 4BFFFAED  bl 0x8246c0c0
	ctx.lr = 0x8246C5D8;
	sub_8246C0C0(ctx, base);
	// 8246C5D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C5E8 size=48
    let mut pc: u32 = 0x8246C5E8;
    'dispatch: loop {
        match pc {
            0x8246C5E8 => {
    //   block [0x8246C5E8..0x8246C618)
	// 8246C5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C5F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C5F4: B081007E  sth r4, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[4].u16 ) };
	// 8246C5F8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C5FC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8246C600: 3881007E  addi r4, r1, 0x7e
	ctx.r[4].s64 = ctx.r[1].s64 + 126;
	// 8246C604: 4BFFFABD  bl 0x8246c0c0
	ctx.lr = 0x8246C608;
	sub_8246C0C0(ctx, base);
	// 8246C608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C618 size=48
    let mut pc: u32 = 0x8246C618;
    'dispatch: loop {
        match pc {
            0x8246C618 => {
    //   block [0x8246C618..0x8246C648)
	// 8246C618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C624: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8246C628: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C62C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246C630: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8246C634: 4BFFFA8D  bl 0x8246c0c0
	ctx.lr = 0x8246C638;
	sub_8246C0C0(ctx, base);
	// 8246C638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C648 size=48
    let mut pc: u32 = 0x8246C648;
    'dispatch: loop {
        match pc {
            0x8246C648 => {
    //   block [0x8246C648..0x8246C678)
	// 8246C648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C654: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8246C658: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C65C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246C660: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8246C664: 4BFFFA5D  bl 0x8246c0c0
	ctx.lr = 0x8246C668;
	sub_8246C0C0(ctx, base);
	// 8246C668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C678 size=48
    let mut pc: u32 = 0x8246C678;
    'dispatch: loop {
        match pc {
            0x8246C678 => {
    //   block [0x8246C678..0x8246C6A8)
	// 8246C678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C684: F8810078  std r4, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u64 ) };
	// 8246C688: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C68C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246C690: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8246C694: 4BFFFA2D  bl 0x8246c0c0
	ctx.lr = 0x8246C698;
	sub_8246C0C0(ctx, base);
	// 8246C698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C6A8 size=48
    let mut pc: u32 = 0x8246C6A8;
    'dispatch: loop {
        match pc {
            0x8246C6A8 => {
    //   block [0x8246C6A8..0x8246C6D8)
	// 8246C6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C6B4: F8810078  std r4, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u64 ) };
	// 8246C6B8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C6BC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246C6C0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8246C6C4: 4BFFF9FD  bl 0x8246c0c0
	ctx.lr = 0x8246C6C8;
	sub_8246C0C0(ctx, base);
	// 8246C6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246C6D8 size=48
    let mut pc: u32 = 0x8246C6D8;
    'dispatch: loop {
        match pc {
            0x8246C6D8 => {
    //   block [0x8246C6D8..0x8246C708)
	// 8246C6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C6E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C6E8: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8246C6EC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246C6F0: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8246C6F4: 4BFFF9CD  bl 0x8246c0c0
	ctx.lr = 0x8246C6F8;
	sub_8246C0C0(ctx, base);
	// 8246C6F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C708 size=48
    let mut pc: u32 = 0x8246C708;
    'dispatch: loop {
        match pc {
            0x8246C708 => {
    //   block [0x8246C708..0x8246C738)
	// 8246C708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C714: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8246C718: D8210078  stfd f1, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.f[1].u64 ) };
	// 8246C71C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246C720: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8246C724: 4BFFF99D  bl 0x8246c0c0
	ctx.lr = 0x8246C728;
	sub_8246C0C0(ctx, base);
	// 8246C728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246C72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246C738 size=172
    let mut pc: u32 = 0x8246C738;
    'dispatch: loop {
        match pc {
            0x8246C738 => {
    //   block [0x8246C738..0x8246C770)
	// 8246C738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C73C: 480C8979  bl 0x825350b4
	ctx.lr = 0x8246C740;
	sub_82535080(ctx, base);
	// 8246C740: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C744: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246C748: 897C000C  lbz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246C74C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C750: 409A0020  bne cr6, 0x8246c770
	if !ctx.cr[6].eq {
	pc = 0x8246C770; continue 'dispatch;
	}
	// 8246C754: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C758: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C75C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C764: 4E800421  bctrl
	ctx.lr = 0x8246C768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C768: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 8246C76C: 480C8998  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246C770 => {
    //   block [0x8246C770..0x8246C794)
	// 8246C770: 7CAB4E70  srawi r11, r5, 9
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 9) as i64;
	// 8246C774: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246C778: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8246C77C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246C780: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246C784: 3BE00200  li r31, 0x200
	ctx.r[31].s64 = 512;
	// 8246C788: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8246C78C: 7F6B2850  subf r27, r11, r5
	ctx.r[27].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 8246C790: 4099004C  ble cr6, 0x8246c7dc
	if !ctx.cr[6].gt {
	pc = 0x8246C7DC; continue 'dispatch;
	}
	pc = 0x8246C794; continue 'dispatch;
            }
            0x8246C794 => {
    //   block [0x8246C794..0x8246C7A0)
	// 8246C794: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 8246C798: 40980008  bge cr6, 0x8246c7a0
	if !ctx.cr[6].lt {
	pc = 0x8246C7A0; continue 'dispatch;
	}
	// 8246C79C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	pc = 0x8246C7A0; continue 'dispatch;
            }
            0x8246C7A0 => {
    //   block [0x8246C7A0..0x8246C7DC)
	// 8246C7A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246C7A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246C7A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246C7AC: 4BFFDB7D  bl 0x8246a328
	ctx.lr = 0x8246C7B0;
	sub_8246A328(ctx, base);
	// 8246C7B0: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C7B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246C7B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8246C7BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C7C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C7C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C7C8: 4E800421  bctrl
	ctx.lr = 0x8246C7CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C7CC: 7FDFF050  subf r30, r31, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8246C7D0: 7FBFEA14  add r29, r31, r29
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 8246C7D4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246C7D8: 4199FFBC  bgt cr6, 0x8246c794
	if ctx.cr[6].gt {
	pc = 0x8246C794; continue 'dispatch;
	}
            }
            0x8246C7DC => {
    //   block [0x8246C7DC..0x8246C7E4)
	// 8246C7DC: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 8246C7E0: 480C8924  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246C7E8 size=172
    let mut pc: u32 = 0x8246C7E8;
    'dispatch: loop {
        match pc {
            0x8246C7E8 => {
    //   block [0x8246C7E8..0x8246C820)
	// 8246C7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C7EC: 480C88C9  bl 0x825350b4
	ctx.lr = 0x8246C7F0;
	sub_82535080(ctx, base);
	// 8246C7F0: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C7F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246C7F8: 897C000C  lbz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246C7FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C800: 409A0020  bne cr6, 0x8246c820
	if !ctx.cr[6].eq {
	pc = 0x8246C820; continue 'dispatch;
	}
	// 8246C804: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C808: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C80C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C814: 4E800421  bctrl
	ctx.lr = 0x8246C818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C818: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 8246C81C: 480C88E8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246C820 => {
    //   block [0x8246C820..0x8246C844)
	// 8246C820: 7CAB4E70  srawi r11, r5, 9
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 9) as i64;
	// 8246C824: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246C828: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8246C82C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246C830: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246C834: 3BE00200  li r31, 0x200
	ctx.r[31].s64 = 512;
	// 8246C838: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8246C83C: 7F6B2850  subf r27, r11, r5
	ctx.r[27].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 8246C840: 4099004C  ble cr6, 0x8246c88c
	if !ctx.cr[6].gt {
	pc = 0x8246C88C; continue 'dispatch;
	}
	pc = 0x8246C844; continue 'dispatch;
            }
            0x8246C844 => {
    //   block [0x8246C844..0x8246C850)
	// 8246C844: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 8246C848: 40980008  bge cr6, 0x8246c850
	if !ctx.cr[6].lt {
	pc = 0x8246C850; continue 'dispatch;
	}
	// 8246C84C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	pc = 0x8246C850; continue 'dispatch;
            }
            0x8246C850 => {
    //   block [0x8246C850..0x8246C88C)
	// 8246C850: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246C854: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246C858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246C85C: 4BFFDACD  bl 0x8246a328
	ctx.lr = 0x8246C860;
	sub_8246A328(ctx, base);
	// 8246C860: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C864: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246C868: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8246C86C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C870: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246C874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246C878: 4E800421  bctrl
	ctx.lr = 0x8246C87C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246C87C: 7FDFF050  subf r30, r31, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8246C880: 7FBFEA14  add r29, r31, r29
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 8246C884: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246C888: 4199FFBC  bgt cr6, 0x8246c844
	if ctx.cr[6].gt {
	pc = 0x8246C844; continue 'dispatch;
	}
            }
            0x8246C88C => {
    //   block [0x8246C88C..0x8246C894)
	// 8246C88C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 8246C890: 480C8874  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C898 size=12
    let mut pc: u32 = 0x8246C898;
    'dispatch: loop {
        match pc {
            0x8246C898 => {
    //   block [0x8246C898..0x8246C8A4)
	// 8246C898: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C89C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8246C8A0: 4BFFF820  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C8A8 size=12
    let mut pc: u32 = 0x8246C8A8;
    'dispatch: loop {
        match pc {
            0x8246C8A8 => {
    //   block [0x8246C8A8..0x8246C8B4)
	// 8246C8A8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C8AC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8246C8B0: 4BFFF810  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C8B8 size=12
    let mut pc: u32 = 0x8246C8B8;
    'dispatch: loop {
        match pc {
            0x8246C8B8 => {
    //   block [0x8246C8B8..0x8246C8C4)
	// 8246C8B8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C8BC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246C8C0: 4BFFF800  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C8C8 size=12
    let mut pc: u32 = 0x8246C8C8;
    'dispatch: loop {
        match pc {
            0x8246C8C8 => {
    //   block [0x8246C8C8..0x8246C8D4)
	// 8246C8C8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C8CC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246C8D0: 4BFFF7F0  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C8D8 size=12
    let mut pc: u32 = 0x8246C8D8;
    'dispatch: loop {
        match pc {
            0x8246C8D8 => {
    //   block [0x8246C8D8..0x8246C8E4)
	// 8246C8D8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C8DC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246C8E0: 4BFFF7E0  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C8E8 size=12
    let mut pc: u32 = 0x8246C8E8;
    'dispatch: loop {
        match pc {
            0x8246C8E8 => {
    //   block [0x8246C8E8..0x8246C8F4)
	// 8246C8E8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C8EC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246C8F0: 4BFFF7D0  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C8F8 size=12
    let mut pc: u32 = 0x8246C8F8;
    'dispatch: loop {
        match pc {
            0x8246C8F8 => {
    //   block [0x8246C8F8..0x8246C904)
	// 8246C8F8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C8FC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246C900: 4BFFF7C0  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C908 size=12
    let mut pc: u32 = 0x8246C908;
    'dispatch: loop {
        match pc {
            0x8246C908 => {
    //   block [0x8246C908..0x8246C914)
	// 8246C908: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8246C90C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8246C910: 4BFFF7B0  b 0x8246c0c0
	sub_8246C0C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C918 size=100
    let mut pc: u32 = 0x8246C918;
    'dispatch: loop {
        match pc {
            0x8246C918 => {
    //   block [0x8246C918..0x8246C960)
	// 8246C918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246C920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246C924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246C928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C92C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C930: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246C934: 4BFFFC05  bl 0x8246c538
	ctx.lr = 0x8246C938;
	sub_8246C538(ctx, base);
	// 8246C938: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246C93C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246C940: 419A0020  beq cr6, 0x8246c960
	if ctx.cr[6].eq {
	pc = 0x8246C960; continue 'dispatch;
	}
	// 8246C944: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C948: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246C94C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246C950: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C954: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246C958: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246C95C: 4BFF775D  bl 0x824640b8
	ctx.lr = 0x8246C960;
	sub_824640B8(ctx, base);
	pc = 0x8246C960; continue 'dispatch;
            }
            0x8246C960 => {
    //   block [0x8246C960..0x8246C97C)
	// 8246C960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246C964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246C968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246C96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246C970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246C974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246C978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C980 size=60
    let mut pc: u32 = 0x8246C980;
    'dispatch: loop {
        match pc {
            0x8246C980 => {
    //   block [0x8246C980..0x8246C998)
	// 8246C980: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C984: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246C988: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246C98C: 40990028  ble cr6, 0x8246c9b4
	if !ctx.cr[6].gt {
	pc = 0x8246C9B4; continue 'dispatch;
	}
	// 8246C990: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C994: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	pc = 0x8246C998; continue 'dispatch;
            }
            0x8246C998 => {
    //   block [0x8246C998..0x8246C9B4)
	// 8246C998: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C99C: 7F072000  cmpw cr6, r7, r4
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8246C9A0: 419A001C  beq cr6, 0x8246c9bc
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246C9BC);
		return;
	}
	// 8246C9A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246C9A8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8246C9AC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246C9B0: 4198FFE8  blt cr6, 0x8246c998
	if ctx.cr[6].lt {
	pc = 0x8246C998; continue 'dispatch;
	}
	pc = 0x8246C9B4; continue 'dispatch;
            }
            0x8246C9B4 => {
    //   block [0x8246C9B4..0x8246C9BC)
	// 8246C9B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246C9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246C9D8 size=132
    let mut pc: u32 = 0x8246C9D8;
    'dispatch: loop {
        match pc {
            0x8246C9D8 => {
    //   block [0x8246C9D8..0x8246CA04)
	// 8246C9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246C9DC: 480C86D9  bl 0x825350b4
	ctx.lr = 0x8246C9E0;
	sub_82535080(ctx, base);
	// 8246C9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246C9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246C9E8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246C9EC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8246C9F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246C9F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246C9F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246C9FC: 40990038  ble cr6, 0x8246ca34
	if !ctx.cr[6].gt {
	pc = 0x8246CA34; continue 'dispatch;
	}
	// 8246CA00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x8246CA04; continue 'dispatch;
            }
            0x8246CA04 => {
    //   block [0x8246CA04..0x8246CA34)
	// 8246CA04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CA08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246CA0C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246CA10: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CA14: 4BFFD5F5  bl 0x8246a008
	ctx.lr = 0x8246CA18;
	sub_8246A008(ctx, base);
	// 8246CA18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246CA1C: 419A0024  beq cr6, 0x8246ca40
	if ctx.cr[6].eq {
	pc = 0x8246CA40; continue 'dispatch;
	}
	// 8246CA20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CA24: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8246CA28: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8246CA2C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246CA30: 4198FFD4  blt cr6, 0x8246ca04
	if ctx.cr[6].lt {
	pc = 0x8246CA04; continue 'dispatch;
	}
	pc = 0x8246CA34; continue 'dispatch;
            }
            0x8246CA34 => {
    //   block [0x8246CA34..0x8246CA40)
	// 8246CA34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246CA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246CA3C: 480C86C8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246CA40 => {
    //   block [0x8246CA40..0x8246CA5C)
	// 8246CA40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CA44: 57AA1838  slwi r10, r29, 3
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CA48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246CA4C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246CA50: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246CA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246CA58: 480C86AC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CA60 size=180
    let mut pc: u32 = 0x8246CA60;
    'dispatch: loop {
        match pc {
            0x8246CA60 => {
    //   block [0x8246CA60..0x8246CAB8)
	// 8246CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CA64: 480C864D  bl 0x825350b0
	ctx.lr = 0x8246CA68;
	sub_82535080(ctx, base);
	// 8246CA68: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CA6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246CA70: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8246CA74: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8246CA78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246CA7C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8246CA80: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CA84: 4BFFF90D  bl 0x8246c390
	ctx.lr = 0x8246CA88;
	sub_8246C390(ctx, base);
	// 8246CA88: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CA8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246CA90: 4BFFD791  bl 0x8246a220
	ctx.lr = 0x8246CA94;
	sub_8246A220(ctx, base);
	// 8246CA94: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246CA98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246CA9C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246CAA0: 4BFFF7E9  bl 0x8246c288
	ctx.lr = 0x8246CAA4;
	sub_8246C288(ctx, base);
	// 8246CAA4: 835B0008  lwz r26, 8(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CAA8: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8246CAAC: 4099004C  ble cr6, 0x8246caf8
	if !ctx.cr[6].gt {
	pc = 0x8246CAF8; continue 'dispatch;
	}
	// 8246CAB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8246CAB4: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	pc = 0x8246CAB8; continue 'dispatch;
            }
            0x8246CAB8 => {
    //   block [0x8246CAB8..0x8246CAF8)
	// 8246CAB8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CABC: 7FBE5A14  add r29, r30, r11
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8246CAC0: 839D0004  lwz r28, 4(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CAC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246CAC8: 4BFFD759  bl 0x8246a220
	ctx.lr = 0x8246CACC;
	sub_8246A220(ctx, base);
	// 8246CACC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246CAD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246CAD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246CAD8: 4BFFF7B1  bl 0x8246c288
	ctx.lr = 0x8246CADC;
	sub_8246C288(ctx, base);
	// 8246CADC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246CAE0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CAE4: 4BFFFB35  bl 0x8246c618
	ctx.lr = 0x8246CAE8;
	sub_8246C618(ctx, base);
	// 8246CAE8: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8246CAEC: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8246CAF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8246CAF4: 409AFFC4  bne cr6, 0x8246cab8
	if !ctx.cr[6].eq {
	pc = 0x8246CAB8; continue 'dispatch;
	}
	pc = 0x8246CAF8; continue 'dispatch;
            }
            0x8246CAF8 => {
    //   block [0x8246CAF8..0x8246CB14)
	// 8246CAF8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246CAFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246CB00: 4BFFFB19  bl 0x8246c618
	ctx.lr = 0x8246CB04;
	sub_8246C618(ctx, base);
	// 8246CB04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246CB08: 4BFFFA31  bl 0x8246c538
	ctx.lr = 0x8246CB0C;
	sub_8246C538(ctx, base);
	// 8246CB0C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246CB10: 480C85F0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CB30 size=196
    let mut pc: u32 = 0x8246CB30;
    'dispatch: loop {
        match pc {
            0x8246CB30 => {
    //   block [0x8246CB30..0x8246CB68)
	// 8246CB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CB34: 480C8575  bl 0x825350a8
	ctx.lr = 0x8246CB38;
	sub_82535080(ctx, base);
	// 8246CB38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CB3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246CB40: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8246CB44: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246CB48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246CB4C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8246CB50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246CB54: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CB58: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 8246CB5C: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8246CB60: 4198007C  blt cr6, 0x8246cbdc
	if ctx.cr[6].lt {
	pc = 0x8246CBDC; continue 'dispatch;
	}
	// 8246CB64: 573B1838  slwi r27, r25, 3
	ctx.r[27].u32 = ctx.r[25].u32.wrapping_shl(3);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	pc = 0x8246CB68; continue 'dispatch;
            }
            0x8246CB68 => {
    //   block [0x8246CB68..0x8246CBAC)
	// 8246CB68: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246CB6C: 419A0070  beq cr6, 0x8246cbdc
	if ctx.cr[6].eq {
	pc = 0x8246CBDC; continue 'dispatch;
	}
	// 8246CB70: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CB74: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8246CB78: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CB7C: 7FCAE838  and r10, r30, r29
	ctx.r[10].u64 = ctx.r[30].u64 & ctx.r[29].u64;
	// 8246CB80: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246CB84: 409A0048  bne cr6, 0x8246cbcc
	if !ctx.cr[6].eq {
	pc = 0x8246CBCC; continue 'dispatch;
	}
	// 8246CB88: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CB8C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CB90: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8246CB94: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CB98: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246CB9C: 409A0010  bne cr6, 0x8246cbac
	if !ctx.cr[6].eq {
	pc = 0x8246CBAC; continue 'dispatch;
	}
	// 8246CBA0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8246CBA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246CBA8: 480017A9  bl 0x8246e350
	ctx.lr = 0x8246CBAC;
	sub_8246E350(ctx, base);
	pc = 0x8246CBAC; continue 'dispatch;
            }
            0x8246CBAC => {
    //   block [0x8246CBAC..0x8246CBCC)
	// 8246CBAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CBB0: 7FBDF078  andc r29, r29, r30
	ctx.r[29].u64 = ctx.r[29].u64 & !ctx.r[30].u64;
	// 8246CBB4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CBB8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246CBBC: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8246CBC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CBC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246CBC8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x8246CBCC; continue 'dispatch;
            }
            0x8246CBCC => {
    //   block [0x8246CBCC..0x8246CBDC)
	// 8246CBCC: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 8246CBD0: 3B7BFFF8  addi r27, r27, -8
	ctx.r[27].s64 = ctx.r[27].s64 + -8;
	// 8246CBD4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8246CBD8: 4098FF90  bge cr6, 0x8246cb68
	if !ctx.cr[6].lt {
	pc = 0x8246CB68; continue 'dispatch;
	}
	pc = 0x8246CBDC; continue 'dispatch;
            }
            0x8246CBDC => {
    //   block [0x8246CBDC..0x8246CBF4)
	// 8246CBDC: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 8246CBE0: 93B80000  stw r29, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8246CBE4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246CBE8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8246CBEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246CBF0: 480C8508  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CBF8 size=72
    let mut pc: u32 = 0x8246CBF8;
    'dispatch: loop {
        match pc {
            0x8246CBF8 => {
    //   block [0x8246CBF8..0x8246CC40)
	// 8246CBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246CC00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246CC04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CC08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246CC0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246CC10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246CC14: 48000155  bl 0x8246cd68
	ctx.lr = 0x8246CC18;
	sub_8246CD68(ctx, base);
	// 8246CC18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8246CC1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246CC20: 4BFFFE41  bl 0x8246ca60
	ctx.lr = 0x8246CC24;
	sub_8246CA60(ctx, base);
	// 8246CC24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246CC28: 48000131  bl 0x8246cd58
	ctx.lr = 0x8246CC2C;
	sub_8246CD58(ctx, base);
	// 8246CC2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246CC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246CC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246CC38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246CC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CC40 size=172
    let mut pc: u32 = 0x8246CC40;
    'dispatch: loop {
        match pc {
            0x8246CC40 => {
    //   block [0x8246CC40..0x8246CC5C)
	// 8246CC40: 546A07FE  clrlwi r10, r3, 0x1f
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 8246CC44: 3D60EDB8  lis r11, -0x1248
	ctx.r[11].s64 = -306708480;
	// 8246CC48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246CC4C: 616B8320  ori r11, r11, 0x8320
	ctx.r[11].u64 = ctx.r[11].u64 | 33568;
	// 8246CC50: 546AF87E  srwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CC54: 419A0008  beq cr6, 0x8246cc5c
	if ctx.cr[6].eq {
	pc = 0x8246CC5C; continue 'dispatch;
	}
	// 8246CC58: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CC5C; continue 'dispatch;
            }
            0x8246CC5C => {
    //   block [0x8246CC5C..0x8246CC70)
	// 8246CC5C: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CC60: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CC64: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CC68: 419A0008  beq cr6, 0x8246cc70
	if ctx.cr[6].eq {
	pc = 0x8246CC70; continue 'dispatch;
	}
	// 8246CC6C: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CC70; continue 'dispatch;
            }
            0x8246CC70 => {
    //   block [0x8246CC70..0x8246CC84)
	// 8246CC70: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CC74: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CC78: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CC7C: 419A0008  beq cr6, 0x8246cc84
	if ctx.cr[6].eq {
	pc = 0x8246CC84; continue 'dispatch;
	}
	// 8246CC80: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CC84; continue 'dispatch;
            }
            0x8246CC84 => {
    //   block [0x8246CC84..0x8246CC98)
	// 8246CC84: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CC88: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CC8C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CC90: 419A0008  beq cr6, 0x8246cc98
	if ctx.cr[6].eq {
	pc = 0x8246CC98; continue 'dispatch;
	}
	// 8246CC94: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CC98; continue 'dispatch;
            }
            0x8246CC98 => {
    //   block [0x8246CC98..0x8246CCAC)
	// 8246CC98: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CC9C: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CCA0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CCA4: 419A0008  beq cr6, 0x8246ccac
	if ctx.cr[6].eq {
	pc = 0x8246CCAC; continue 'dispatch;
	}
	// 8246CCA8: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CCAC; continue 'dispatch;
            }
            0x8246CCAC => {
    //   block [0x8246CCAC..0x8246CCC0)
	// 8246CCAC: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CCB0: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CCB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CCB8: 419A0008  beq cr6, 0x8246ccc0
	if ctx.cr[6].eq {
	pc = 0x8246CCC0; continue 'dispatch;
	}
	// 8246CCBC: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CCC0; continue 'dispatch;
            }
            0x8246CCC0 => {
    //   block [0x8246CCC0..0x8246CCD4)
	// 8246CCC0: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CCC4: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CCC8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CCCC: 419A0008  beq cr6, 0x8246ccd4
	if ctx.cr[6].eq {
	pc = 0x8246CCD4; continue 'dispatch;
	}
	// 8246CCD0: 7D4A5A78  xor r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	pc = 0x8246CCD4; continue 'dispatch;
            }
            0x8246CCD4 => {
    //   block [0x8246CCD4..0x8246CCEC)
	// 8246CCD4: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CCD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CCDC: 419A0010  beq cr6, 0x8246ccec
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246CCEC);
		return;
	}
	// 8246CCE0: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246CCE4: 7D435A78  xor r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 8246CCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CCF8 size=92
    let mut pc: u32 = 0x8246CCF8;
    'dispatch: loop {
        match pc {
            0x8246CCF8 => {
    //   block [0x8246CCF8..0x8246CD20)
	// 8246CCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CCFC: 480C83B9  bl 0x825350b4
	ctx.lr = 0x8246CD00;
	sub_82535080(ctx, base);
	// 8246CD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CD04: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8246CD08: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8246CD0C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246CD10: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246CD14: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246CD18: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CD1C: 40990028  ble cr6, 0x8246cd44
	if !ctx.cr[6].gt {
	pc = 0x8246CD44; continue 'dispatch;
	}
	pc = 0x8246CD20; continue 'dispatch;
            }
            0x8246CD20 => {
    //   block [0x8246CD20..0x8246CD44)
	// 8246CD20: 7D7FE0AE  lbzx r11, r31, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246CD24: 7D6BF278  xor r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[30].u64;
	// 8246CD28: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8246CD2C: 4BFFFF15  bl 0x8246cc40
	ctx.lr = 0x8246CD30;
	sub_8246CC40(ctx, base);
	// 8246CD30: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8246CD34: 57CBC23E  srwi r11, r30, 8
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246CD38: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8246CD3C: 7C7E5A78  xor r30, r3, r11
	ctx.r[30].u64 = ctx.r[3].u64 ^ ctx.r[11].u64;
	// 8246CD40: 4198FFE0  blt cr6, 0x8246cd20
	if ctx.cr[6].lt {
	pc = 0x8246CD20; continue 'dispatch;
	}
	pc = 0x8246CD44; continue 'dispatch;
            }
            0x8246CD44 => {
    //   block [0x8246CD44..0x8246CD54)
	// 8246CD44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246CD48: 93DB0008  stw r30, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8246CD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246CD50: 480C83B4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CD58 size=12
    let mut pc: u32 = 0x8246CD58;
    'dispatch: loop {
        match pc {
            0x8246CD58 => {
    //   block [0x8246CD58..0x8246CD64)
	// 8246CD58: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CD5C: 7D6358F8  nor r3, r11, r11
	ctx.r[3].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 8246CD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CD68 size=32
    let mut pc: u32 = 0x8246CD68;
    'dispatch: loop {
        match pc {
            0x8246CD68 => {
    //   block [0x8246CD68..0x8246CD88)
	// 8246CD68: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246CD6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246CD70: 396B8700  addi r11, r11, -0x7900
	ctx.r[11].s64 = ctx.r[11].s64 + -30976;
	// 8246CD74: 7C8920F8  nor r9, r4, r4
	ctx.r[9].u64 = !(ctx.r[4].u64 | ctx.r[4].u64);
	// 8246CD78: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246CD7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246CD80: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8246CD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CD88 size=12
    let mut pc: u32 = 0x8246CD88;
    'dispatch: loop {
        match pc {
            0x8246CD88 => {
    //   block [0x8246CD88..0x8246CD94)
	// 8246CD88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246CD8C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246CD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CDB8 size=108
    let mut pc: u32 = 0x8246CDB8;
    'dispatch: loop {
        match pc {
            0x8246CDB8 => {
    //   block [0x8246CDB8..0x8246CDFC)
	// 8246CDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246CDC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246CDC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246CDC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CDCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246CDD0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CDD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246CDD8: 419A0024  beq cr6, 0x8246cdfc
	if ctx.cr[6].eq {
	pc = 0x8246CDFC; continue 'dispatch;
	}
	// 8246CDDC: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8246CDE0: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8246CDE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CDE8: 4E800421  bctrl
	ctx.lr = 0x8246CDEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CDEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CDF0: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8246CDF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CDF8: 4E800421  bctrl
	ctx.lr = 0x8246CDFC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246CDFC => {
    //   block [0x8246CDFC..0x8246CE24)
	// 8246CDFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246CE00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246CE04: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246CE08: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246CE0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246CE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246CE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246CE18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246CE1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246CE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CE28 size=372
    let mut pc: u32 = 0x8246CE28;
    'dispatch: loop {
        match pc {
            0x8246CE28 => {
    //   block [0x8246CE28..0x8246CE74)
	// 8246CE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CE2C: 480C827D  bl 0x825350a8
	ctx.lr = 0x8246CE30;
	sub_82535080(ctx, base);
	// 8246CE30: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CE34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246CE38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CE3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8246CE40: 40990154  ble cr6, 0x8246cf94
	if !ctx.cr[6].gt {
	pc = 0x8246CF94; continue 'dispatch;
	}
	// 8246CE44: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8246CE48: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246CE4C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8246CE50: 817E49AC  lwz r11, 0x49ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8246CE54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CE58: 4E800421  bctrl
	ctx.lr = 0x8246CE5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CE5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CE60: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246CE64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246CE68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246CE6C: 40990028  ble cr6, 0x8246ce94
	if !ctx.cr[6].gt {
	pc = 0x8246CE94; continue 'dispatch;
	}
	// 8246CE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
            }
            0x8246CE74 => {
    //   block [0x8246CE74..0x8246CE94)
	// 8246CE74: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CE78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246CE7C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8246CE80: 7D2BD12E  stwx r9, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[9].u32) };
	// 8246CE84: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8246CE88: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CE8C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246CE90: 4198FFE4  blt cr6, 0x8246ce74
	if ctx.cr[6].lt {
	pc = 0x8246CE74; continue 'dispatch;
	}
	pc = 0x8246CE94; continue 'dispatch;
            }
            0x8246CE94 => {
    //   block [0x8246CE94..0x8246CEB8)
	// 8246CE94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CE98: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8246CE9C: 4099001C  ble cr6, 0x8246ceb8
	if !ctx.cr[6].gt {
	pc = 0x8246CEB8; continue 'dispatch;
	}
	// 8246CEA0: 3D408247  lis r10, -0x7db9
	ctx.r[10].s64 = -2109276160;
	// 8246CEA4: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8246CEA8: 38CACD98  addi r6, r10, -0x3268
	ctx.r[6].s64 = ctx.r[10].s64 + -12904;
	// 8246CEAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246CEB0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246CEB4: 48000195  bl 0x8246d048
	ctx.lr = 0x8246CEB8;
	sub_8246D048(ctx, base);
	pc = 0x8246CEB8; continue 'dispatch;
            }
            0x8246CEB8 => {
    //   block [0x8246CEB8..0x8246CF0C)
	// 8246CEB8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CEBC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8246CEC0: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246CEC4: 817E49AC  lwz r11, 0x49ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8246CEC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CECC: 4E800421  bctrl
	ctx.lr = 0x8246CED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CED0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CED4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8246CED8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8246CEDC: 1C6B0054  mulli r3, r11, 0x54
	ctx.r[3].s32 = ((ctx.r[11].s32 as i64 * 84 as i64) as i32);
	ctx.r[3].s64 = ctx.r[3].s32 as i64;
	// 8246CEE0: 817E49AC  lwz r11, 0x49ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8246CEE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CEE8: 4E800421  bctrl
	ctx.lr = 0x8246CEEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CEEC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CEF0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8246CEF4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8246CEF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246CEFC: 4099005C  ble cr6, 0x8246cf58
	if !ctx.cr[6].gt {
	pc = 0x8246CF58; continue 'dispatch;
	}
	// 8246CF00: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 8246CF04: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8246CF08: 7F7AC850  subf r27, r26, r25
	ctx.r[27].s64 = ctx.r[25].s64 - ctx.r[26].s64;
            }
            0x8246CF0C => {
    //   block [0x8246CF0C..0x8246CF58)
	// 8246CF0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CF10: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8246CF14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246CF18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CF1C: 7D7BF12E  stwx r11, r27, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 8246CF20: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CF24: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CF28: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CF2C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8246CF30: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8246CF34: 1D6B0054  mulli r11, r11, 0x54
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 84 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246CF38: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246CF3C: 480C7C15  bl 0x82534b50
	ctx.lr = 0x8246CF40;
	sub_82534B50(ctx, base);
	// 8246CF40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CF44: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8246CF48: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8246CF4C: 3BBD0054  addi r29, r29, 0x54
	ctx.r[29].s64 = ctx.r[29].s64 + 84;
	// 8246CF50: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246CF54: 4198FFB8  blt cr6, 0x8246cf0c
	if ctx.cr[6].lt {
	pc = 0x8246CF0C; continue 'dispatch;
	}
	pc = 0x8246CF58; continue 'dispatch;
            }
            0x8246CF58 => {
    //   block [0x8246CF58..0x8246CF94)
	// 8246CF58: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8246CF5C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246CF60: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8246CF64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CF68: 4E800421  bctrl
	ctx.lr = 0x8246CF6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CF6C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CF70: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8246CF74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CF78: 4E800421  bctrl
	ctx.lr = 0x8246CF7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CF7C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246CF80: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 8246CF84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246CF88: 4E800421  bctrl
	ctx.lr = 0x8246CF8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246CF8C: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8246CF90: 931F0004  stw r24, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
            }
            0x8246CF94 => {
    //   block [0x8246CF94..0x8246CF9C)
	// 8246CF94: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246CF98: 480C8160  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246CFA0 size=164
    let mut pc: u32 = 0x8246CFA0;
    'dispatch: loop {
        match pc {
            0x8246CFA0 => {
    //   block [0x8246CFA0..0x8246CFF4)
	// 8246CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246CFA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246CFAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246CFB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246CFB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246CFB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246CFBC: 4BFFFE6D  bl 0x8246ce28
	ctx.lr = 0x8246CFC0;
	sub_8246CE28(ctx, base);
	// 8246CFC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246CFC4: 4BFFFE65  bl 0x8246ce28
	ctx.lr = 0x8246CFC8;
	sub_8246CE28(ctx, base);
	// 8246CFC8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CFCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CFD0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8246CFD4: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246CFD8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CFDC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8246CFE0: 7CE85A14  add r7, r8, r11
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8246CFE4: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8246CFE8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8246CFEC: 40980034  bge cr6, 0x8246d020
	if !ctx.cr[6].lt {
	pc = 0x8246D020; continue 'dispatch;
	}
	// 8246CFF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	pc = 0x8246CFF4; continue 'dispatch;
            }
            0x8246CFF4 => {
    //   block [0x8246CFF4..0x8246D014)
	// 8246CFF4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8246CFF8: 40980028  bge cr6, 0x8246d020
	if !ctx.cr[6].lt {
	pc = 0x8246D020; continue 'dispatch;
	}
	// 8246CFFC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D000: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D004: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246D008: 409A0030  bne cr6, 0x8246d038
	if !ctx.cr[6].eq {
	pc = 0x8246D038; continue 'dispatch;
	}
	// 8246D00C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8246D010: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	pc = 0x8246D014; continue 'dispatch;
            }
            0x8246D014 => {
    //   block [0x8246D014..0x8246D018)
	// 8246D014: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	pc = 0x8246D018; continue 'dispatch;
            }
            0x8246D018 => {
    //   block [0x8246D018..0x8246D020)
	// 8246D018: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8246D01C: 4198FFD8  blt cr6, 0x8246cff4
	if ctx.cr[6].lt {
	pc = 0x8246CFF4; continue 'dispatch;
	}
	pc = 0x8246D020; continue 'dispatch;
            }
            0x8246D020 => {
    //   block [0x8246D020..0x8246D038)
	// 8246D020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246D024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246D028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246D02C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246D030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246D034: 4E800020  blr
	return;
            }
            0x8246D038 => {
    //   block [0x8246D038..0x8246D044)
	// 8246D038: 4098FFDC  bge cr6, 0x8246d014
	if !ctx.cr[6].lt {
	pc = 0x8246D014; continue 'dispatch;
	}
	// 8246D03C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8246D040: 4BFFFFD8  b 0x8246d018
	pc = 0x8246D018; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D048 size=316
    let mut pc: u32 = 0x8246D048;
    'dispatch: loop {
        match pc {
            0x8246D048 => {
    //   block [0x8246D048..0x8246D064)
	// 8246D048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D04C: 480C805D  bl 0x825350a8
	ctx.lr = 0x8246D050;
	sub_82535080(ctx, base);
	// 8246D050: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D054: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246D058: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8246D05C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8246D060: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	pc = 0x8246D064; continue 'dispatch;
            }
            0x8246D064 => {
    //   block [0x8246D064..0x8246D07C)
	// 8246D064: 7D79C214  add r11, r25, r24
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[24].u64;
	// 8246D068: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8246D06C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8246D070: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 8246D074: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D078: 7F8BD02E  lwzx r28, r11, r26
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	pc = 0x8246D07C; continue 'dispatch;
            }
            0x8246D07C => {
    //   block [0x8246D07C..0x8246D0A4)
	// 8246D07C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D080: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8246D084: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8246D088: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246D08C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D090: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 8246D094: 4E800421  bctrl
	ctx.lr = 0x8246D098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246D098: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D09C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D0A0: 419A002C  beq cr6, 0x8246d0cc
	if ctx.cr[6].eq {
	pc = 0x8246D0CC; continue 'dispatch;
	}
            }
            0x8246D0A4 => {
    //   block [0x8246D0A4..0x8246D0CC)
	// 8246D0A4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8246D0A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8246D0AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8246D0B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246D0B4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D0B8: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 8246D0BC: 4E800421  bctrl
	ctx.lr = 0x8246D0C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246D0C0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D0C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D0C8: 409AFFDC  bne cr6, 0x8246d0a4
	if !ctx.cr[6].eq {
	pc = 0x8246D0A4; continue 'dispatch;
	}
            }
            0x8246D0CC => {
    //   block [0x8246D0CC..0x8246D0F4)
	// 8246D0CC: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D0D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246D0D4: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8246D0D8: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8246D0DC: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D0E0: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 8246D0E4: 4E800421  bctrl
	ctx.lr = 0x8246D0E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246D0E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D0EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D0F0: 419A002C  beq cr6, 0x8246d11c
	if ctx.cr[6].eq {
	pc = 0x8246D11C; continue 'dispatch;
	}
            }
            0x8246D0F4 => {
    //   block [0x8246D0F4..0x8246D11C)
	// 8246D0F4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 8246D0F8: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8246D0FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246D100: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8246D104: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D108: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 8246D10C: 4E800421  bctrl
	ctx.lr = 0x8246D110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246D110: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D114: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D118: 409AFFDC  bne cr6, 0x8246d0f4
	if !ctx.cr[6].eq {
	pc = 0x8246D0F4; continue 'dispatch;
	}
            }
            0x8246D11C => {
    //   block [0x8246D11C..0x8246D140)
	// 8246D11C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8246D120: 41980030  blt cr6, 0x8246d150
	if ctx.cr[6].lt {
	pc = 0x8246D150; continue 'dispatch;
	}
	// 8246D124: 419A001C  beq cr6, 0x8246d140
	if ctx.cr[6].eq {
	pc = 0x8246D140; continue 'dispatch;
	}
	// 8246D128: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D12C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D130: 7D0BD02E  lwzx r8, r11, r26
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8246D134: 7D2AD02E  lwzx r9, r10, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8246D138: 7D0AD12E  stwx r8, r10, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32), ctx.r[8].u32) };
	// 8246D13C: 7D2BD12E  stwx r9, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[9].u32) };
	pc = 0x8246D140; continue 'dispatch;
            }
            0x8246D140 => {
    //   block [0x8246D140..0x8246D150)
	// 8246D140: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8246D144: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8246D148: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246D14C: 4099FF30  ble cr6, 0x8246d07c
	if !ctx.cr[6].gt {
	pc = 0x8246D07C; continue 'dispatch;
	}
	pc = 0x8246D150; continue 'dispatch;
            }
            0x8246D150 => {
    //   block [0x8246D150..0x8246D16C)
	// 8246D150: 7F19F000  cmpw cr6, r25, r30
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246D154: 40980018  bge cr6, 0x8246d16c
	if !ctx.cr[6].lt {
	pc = 0x8246D16C; continue 'dispatch;
	}
	// 8246D158: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8246D15C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246D160: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8246D164: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246D168: 4BFFFEE1  bl 0x8246d048
	ctx.lr = 0x8246D16C;
	sub_8246D048(ctx, base);
	pc = 0x8246D16C; continue 'dispatch;
            }
            0x8246D16C => {
    //   block [0x8246D16C..0x8246D17C)
	// 8246D16C: 7F1DC000  cmpw cr6, r29, r24
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[24].s32, &mut ctx.xer);
	// 8246D170: 4098000C  bge cr6, 0x8246d17c
	if !ctx.cr[6].lt {
	pc = 0x8246D17C; continue 'dispatch;
	}
	// 8246D174: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 8246D178: 4BFFFEEC  b 0x8246d064
	pc = 0x8246D064; continue 'dispatch;
            }
            0x8246D17C => {
    //   block [0x8246D17C..0x8246D184)
	// 8246D17C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246D180: 480C7F78  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D188 size=68
    let mut pc: u32 = 0x8246D188;
    'dispatch: loop {
        match pc {
            0x8246D188 => {
    //   block [0x8246D188..0x8246D1B0)
	// 8246D188: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 8246D18C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8246D190: 54ABE8FE  srwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D194: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D198: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246D19C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 8246D1A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246D1A4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8246D1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D1AC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	pc = 0x8246D1B0; continue 'dispatch;
            }
            0x8246D1B0 => {
    //   block [0x8246D1B0..0x8246D1CC)
	// 8246D1B0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D1B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D1B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D1BC: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 8246D1C0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246D1C4: 409AFFEC  bne cr6, 0x8246d1b0
	if !ctx.cr[6].eq {
	pc = 0x8246D1B0; continue 'dispatch;
	}
	// 8246D1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D1D0 size=68
    let mut pc: u32 = 0x8246D1D0;
    'dispatch: loop {
        match pc {
            0x8246D1D0 => {
    //   block [0x8246D1D0..0x8246D1EC)
	// 8246D1D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D1D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D1D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D1DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D1E0: 40990024  ble cr6, 0x8246d204
	if !ctx.cr[6].gt {
	pc = 0x8246D204; continue 'dispatch;
	}
	// 8246D1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D1E8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	pc = 0x8246D1EC; continue 'dispatch;
            }
            0x8246D1EC => {
    //   block [0x8246D1EC..0x8246D204)
	// 8246D1EC: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D1F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D1F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D1F8: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 8246D1FC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246D200: 409AFFEC  bne cr6, 0x8246d1ec
	if !ctx.cr[6].eq {
	pc = 0x8246D1EC; continue 'dispatch;
	}
	pc = 0x8246D204; continue 'dispatch;
            }
            0x8246D204 => {
    //   block [0x8246D204..0x8246D214)
	// 8246D204: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D208: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D20C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D218 size=68
    let mut pc: u32 = 0x8246D218;
    'dispatch: loop {
        match pc {
            0x8246D218 => {
    //   block [0x8246D218..0x8246D240)
	// 8246D218: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 8246D21C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8246D220: 54ABE13E  srwi r11, r5, 4
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D224: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D228: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246D22C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 8246D230: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246D234: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8246D238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D23C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	pc = 0x8246D240; continue 'dispatch;
            }
            0x8246D240 => {
    //   block [0x8246D240..0x8246D25C)
	// 8246D240: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D244: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D24C: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 8246D250: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8246D254: 409AFFEC  bne cr6, 0x8246d240
	if !ctx.cr[6].eq {
	pc = 0x8246D240; continue 'dispatch;
	}
	// 8246D258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D260 size=32
    let mut pc: u32 = 0x8246D260;
    'dispatch: loop {
        match pc {
            0x8246D260 => {
    //   block [0x8246D260..0x8246D26C)
	// 8246D260: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8246D264: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8246D268: 40990010  ble cr6, 0x8246d278
	if !ctx.cr[6].gt {
	pc = 0x8246D278; continue 'dispatch;
	}
	pc = 0x8246D26C; continue 'dispatch;
            }
            0x8246D26C => {
    //   block [0x8246D26C..0x8246D278)
	// 8246D26C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D270: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8246D274: 4198FFF8  blt cr6, 0x8246d26c
	if ctx.cr[6].lt {
	pc = 0x8246D26C; continue 'dispatch;
	}
	pc = 0x8246D278; continue 'dispatch;
            }
            0x8246D278 => {
    //   block [0x8246D278..0x8246D280)
	// 8246D278: 55632834  slwi r3, r11, 5
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246D27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D280 size=188
    let mut pc: u32 = 0x8246D280;
    'dispatch: loop {
        match pc {
            0x8246D280 => {
    //   block [0x8246D280..0x8246D2B4)
	// 8246D280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246D288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246D28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246D290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D294: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246D298: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246D29C: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8246D2A0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D2A4: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D2A8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8246D2AC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8246D2B0: 40990010  ble cr6, 0x8246d2c0
	if !ctx.cr[6].gt {
	pc = 0x8246D2C0; continue 'dispatch;
	}
	pc = 0x8246D2B4; continue 'dispatch;
            }
            0x8246D2B4 => {
    //   block [0x8246D2B4..0x8246D2C0)
	// 8246D2B4: 57FF083C  slwi r31, r31, 1
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8246D2B8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246D2BC: 4198FFF8  blt cr6, 0x8246d2b4
	if ctx.cr[6].lt {
	pc = 0x8246D2B4; continue 'dispatch;
	}
	pc = 0x8246D2C0; continue 'dispatch;
            }
            0x8246D2C0 => {
    //   block [0x8246D2C0..0x8246D2FC)
	// 8246D2C0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D2C4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246D2C8: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246D2CC: 57E41838  slwi r4, r31, 3
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246D2D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246D2D4: 4BFF6D65  bl 0x82464038
	ctx.lr = 0x8246D2D8;
	sub_82464038(ctx, base);
	// 8246D2D8: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8246D2DC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246D2E0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8246D2E4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246D2E8: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D2EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D2F0: 40990024  ble cr6, 0x8246d314
	if !ctx.cr[6].gt {
	pc = 0x8246D314; continue 'dispatch;
	}
	// 8246D2F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D2F8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	pc = 0x8246D2FC; continue 'dispatch;
            }
            0x8246D2FC => {
    //   block [0x8246D2FC..0x8246D314)
	// 8246D2FC: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D300: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D308: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 8246D30C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246D310: 409AFFEC  bne cr6, 0x8246d2fc
	if !ctx.cr[6].eq {
	pc = 0x8246D2FC; continue 'dispatch;
	}
	pc = 0x8246D314; continue 'dispatch;
            }
            0x8246D314 => {
    //   block [0x8246D314..0x8246D33C)
	// 8246D314: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246D31C: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D320: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D324: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246D328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246D32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246D330: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246D334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246D338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D340 size=132
    let mut pc: u32 = 0x8246D340;
    'dispatch: loop {
        match pc {
            0x8246D340 => {
    //   block [0x8246D340..0x8246D388)
	// 8246D340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246D348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246D34C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D350: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D354: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246D358: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246D35C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 8246D360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246D364: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246D368: 4BFF6CD1  bl 0x82464038
	ctx.lr = 0x8246D36C;
	sub_82464038(ctx, base);
	// 8246D36C: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 8246D370: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246D374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D378: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8246D37C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8246D380: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8246D384: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x8246D388; continue 'dispatch;
            }
            0x8246D388 => {
    //   block [0x8246D388..0x8246D3C4)
	// 8246D388: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D38C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D394: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 8246D398: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246D39C: 409AFFEC  bne cr6, 0x8246d388
	if !ctx.cr[6].eq {
	pc = 0x8246D388; continue 'dispatch;
	}
	// 8246D3A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D3A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246D3A8: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D3AC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246D3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246D3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246D3BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246D3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D3C8 size=52
    let mut pc: u32 = 0x8246D3C8;
    'dispatch: loop {
        match pc {
            0x8246D3C8 => {
    //   block [0x8246D3C8..0x8246D3FC)
	// 8246D3C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D3CC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D3D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D3D4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8246D3D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D3DC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246D3E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D3E4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246D3E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D3EC: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D3F0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246D3F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246D3F8: 4BFF6CC0  b 0x824640b8
	sub_824640B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D400 size=212
    let mut pc: u32 = 0x8246D400;
    'dispatch: loop {
        match pc {
            0x8246D400 => {
    //   block [0x8246D400..0x8246D438)
	// 8246D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D404: 480C7CB9  bl 0x825350bc
	ctx.lr = 0x8246D408;
	sub_82535080(ctx, base);
	// 8246D408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D40C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246D410: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246D414: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8246D418: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D41C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D420: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D424: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246D428: 40990010  ble cr6, 0x8246d438
	if !ctx.cr[6].gt {
	pc = 0x8246D438; continue 'dispatch;
	}
	// 8246D42C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D430: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246D434: 4800032D  bl 0x8246d760
	ctx.lr = 0x8246D438;
	sub_8246D760(ctx, base);
	pc = 0x8246D438; continue 'dispatch;
            }
            0x8246D438 => {
    //   block [0x8246D438..0x8246D464)
	// 8246D438: 3D409E37  lis r10, -0x61c9
	ctx.r[10].s64 = -1640562688;
	// 8246D43C: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D440: 57CBE13E  srwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D444: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D448: 614A79B1  ori r10, r10, 0x79b1
	ctx.r[10].u64 = ctx.r[10].u64 | 31153;
	// 8246D44C: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246D450: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D454: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D458: 7CE9502E  lwzx r7, r9, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246D45C: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8246D460: 419A002C  beq cr6, 0x8246d48c
	if ctx.cr[6].eq {
	pc = 0x8246D48C; continue 'dispatch;
	}
	pc = 0x8246D464; continue 'dispatch;
            }
            0x8246D464 => {
    //   block [0x8246D464..0x8246D48C)
	// 8246D464: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246D468: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8246D46C: 419A0020  beq cr6, 0x8246d48c
	if ctx.cr[6].eq {
	pc = 0x8246D48C; continue 'dispatch;
	}
	// 8246D470: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D474: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D478: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D47C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D480: 7CEA382E  lwzx r7, r10, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8246D484: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8246D488: 409AFFDC  bne cr6, 0x8246d464
	if !ctx.cr[6].eq {
	pc = 0x8246D464; continue 'dispatch;
	}
	pc = 0x8246D48C; continue 'dispatch;
            }
            0x8246D48C => {
    //   block [0x8246D48C..0x8246D4D4)
	// 8246D48C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D490: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D494: 7D0A482E  lwzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246D498: 7D08F050  subf r8, r8, r30
	ctx.r[8].s64 = ctx.r[30].s64 - ctx.r[8].s64;
	// 8246D49C: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8246D4A0: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8246D4A4: 69080001  xori r8, r8, 1
	ctx.r[8].u64 = ctx.r[8].u64 ^ 1;
	// 8246D4A8: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 8246D4AC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8246D4B0: 7FCA492E  stwx r30, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 8246D4B4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D4B8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D4BC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246D4C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D4C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D4C8: 7FAB492E  stwx r29, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u32) };
	// 8246D4CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246D4D0: 480C7C3C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D4D8 size=84
    let mut pc: u32 = 0x8246D4D8;
    'dispatch: loop {
        match pc {
            0x8246D4D8 => {
    //   block [0x8246D4D8..0x8246D504)
	// 8246D4D8: 3D209E37  lis r9, -0x61c9
	ctx.r[9].s64 = -1640562688;
	// 8246D4DC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D4E0: 548BE13E  srwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D4E4: 612879B1  ori r8, r9, 0x79b1
	ctx.r[8].u64 = ctx.r[9].u64 | 31153;
	// 8246D4E8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D4EC: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246D4F0: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8246D4F4: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D4F8: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246D4FC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8246D500: 419A0024  beq cr6, 0x8246d524
	if ctx.cr[6].eq {
	pc = 0x8246D524; continue 'dispatch;
	}
	pc = 0x8246D504; continue 'dispatch;
            }
            0x8246D504 => {
    //   block [0x8246D504..0x8246D524)
	// 8246D504: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8246D508: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246D50C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8246D510: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8246D514: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D518: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246D51C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8246D520: 409AFFE4  bne cr6, 0x8246d504
	if !ctx.cr[6].eq {
	pc = 0x8246D504; continue 'dispatch;
	}
	pc = 0x8246D524; continue 'dispatch;
            }
            0x8246D524 => {
    //   block [0x8246D524..0x8246D52C)
	// 8246D524: 386A0001  addi r3, r10, 1
	ctx.r[3].s64 = ctx.r[10].s64 + 1;
	// 8246D528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D530 size=88
    let mut pc: u32 = 0x8246D530;
    'dispatch: loop {
        match pc {
            0x8246D530 => {
    //   block [0x8246D530..0x8246D560)
	// 8246D530: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246D534: 3D009E37  lis r8, -0x61c9
	ctx.r[8].s64 = -1640562688;
	// 8246D538: 548AE13E  srwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D53C: 610779B1  ori r7, r8, 0x79b1
	ctx.r[7].u64 = ctx.r[8].u64 | 31153;
	// 8246D540: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D544: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D548: 7D6A39D6  mullw r11, r10, r7
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246D54C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 8246D550: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D554: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8246D558: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8246D55C: 419A0024  beq cr6, 0x8246d580
	if ctx.cr[6].eq {
	pc = 0x8246D580; continue 'dispatch;
	}
	pc = 0x8246D560; continue 'dispatch;
            }
            0x8246D560 => {
    //   block [0x8246D560..0x8246D580)
	// 8246D560: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8246D564: 419A0024  beq cr6, 0x8246d588
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246D588);
		return;
	}
	// 8246D568: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D56C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 8246D570: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D574: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8246D578: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8246D57C: 409AFFE4  bne cr6, 0x8246d560
	if !ctx.cr[6].eq {
	pc = 0x8246D560; continue 'dispatch;
	}
	pc = 0x8246D580; continue 'dispatch;
            }
            0x8246D580 => {
    //   block [0x8246D580..0x8246D588)
	// 8246D580: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8246D584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D5A0 size=124
    let mut pc: u32 = 0x8246D5A0;
    'dispatch: loop {
        match pc {
            0x8246D5A0 => {
    //   block [0x8246D5A0..0x8246D5D4)
	// 8246D5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246D5A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246D5AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246D5B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D5B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246D5B8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246D5BC: 4BFFFF1D  bl 0x8246d4d8
	ctx.lr = 0x8246D5C0;
	sub_8246D4D8(ctx, base);
	// 8246D5C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246D5C8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246D5CC: 40990008  ble cr6, 0x8246d5d4
	if !ctx.cr[6].gt {
	pc = 0x8246D5D4; continue 'dispatch;
	}
	// 8246D5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x8246D5D4; continue 'dispatch;
            }
            0x8246D5D4 => {
    //   block [0x8246D5D4..0x8246D600)
	// 8246D5D4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246D5D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246D5DC: 419A0024  beq cr6, 0x8246d600
	if ctx.cr[6].eq {
	pc = 0x8246D600; continue 'dispatch;
	}
	// 8246D5E0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246D5E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D5E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246D5EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D5F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D5F4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246D5F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246D5FC: 48000008  b 0x8246d604
	pc = 0x8246D604; continue 'dispatch;
            }
            0x8246D600 => {
    //   block [0x8246D600..0x8246D604)
	// 8246D600: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x8246D604; continue 'dispatch;
            }
            0x8246D604 => {
    //   block [0x8246D604..0x8246D61C)
	// 8246D604: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246D608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246D60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246D610: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246D614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246D618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D620 size=320
    let mut pc: u32 = 0x8246D620;
    'dispatch: loop {
        match pc {
            0x8246D620 => {
    //   block [0x8246D620..0x8246D66C)
	// 8246D620: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8246D624: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8246D628: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D62C: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8246D630: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8246D638: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D63C: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D640: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D644: 7CAA492E  stwx r5, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[5].u32) };
	// 8246D648: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D64C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D650: 7D6A4214  add r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8246D654: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8246D658: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246D65C: 7CE7482E  lwzx r7, r7, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246D660: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8246D664: 419A0020  beq cr6, 0x8246d684
	if ctx.cr[6].eq {
	pc = 0x8246D684; continue 'dispatch;
	}
	// 8246D668: 5527003E  slwi r7, r9, 0
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	pc = 0x8246D66C; continue 'dispatch;
            }
            0x8246D66C => {
    //   block [0x8246D66C..0x8246D684)
	// 8246D66C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246D670: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8246D674: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8246D678: 7CC6382E  lwzx r6, r6, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8246D67C: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 8246D680: 409AFFEC  bne cr6, 0x8246d66c
	if !ctx.cr[6].eq {
	pc = 0x8246D66C; continue 'dispatch;
	}
	pc = 0x8246D684; continue 'dispatch;
            }
            0x8246D684 => {
    //   block [0x8246D684..0x8246D6AC)
	// 8246D684: 38E80001  addi r7, r8, 1
	ctx.r[7].s64 = ctx.r[8].s64 + 1;
	// 8246D688: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 8246D68C: 7CEB5038  and r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 8246D690: 7CDF5038  and r31, r6, r10
	ctx.r[31].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 8246D694: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246D698: 7D29382E  lwzx r9, r9, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8246D69C: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 8246D6A0: 419A00B4  beq cr6, 0x8246d754
	if ctx.cr[6].eq {
	pc = 0x8246D754; continue 'dispatch;
	}
	// 8246D6A4: 3D209E37  lis r9, -0x61c9
	ctx.r[9].s64 = -1640562688;
	// 8246D6A8: 612479B1  ori r4, r9, 0x79b1
	ctx.r[4].u64 = ctx.r[9].u64 | 31153;
	pc = 0x8246D6AC; continue 'dispatch;
            }
            0x8246D6AC => {
    //   block [0x8246D6AC..0x8246D6D0)
	// 8246D6AC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D6B0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8246D6B4: 7CC9382E  lwzx r6, r9, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8246D6B8: 54DEE13E  srwi r30, r6, 4
	ctx.r[30].u32 = ctx.r[6].u32.wrapping_shr(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8246D6BC: 7FDE21D6  mullw r30, r30, r4
	ctx.r[30].s32 = ((ctx.r[30].s32 as i64 * ctx.r[4].s32 as i64) as i32);
	ctx.r[30].s64 = ctx.r[30].s32 as i64;
	// 8246D6C0: 7FCA5038  and r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 & ctx.r[10].u64;
	// 8246D6C4: 4198000C  blt cr6, 0x8246d6d0
	if ctx.cr[6].lt {
	pc = 0x8246D6D0; continue 'dispatch;
	}
	// 8246D6C8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246D6CC: 41990068  bgt cr6, 0x8246d734
	if ctx.cr[6].gt {
	pc = 0x8246D734; continue 'dispatch;
	}
	pc = 0x8246D6D0; continue 'dispatch;
            }
            0x8246D6D0 => {
    //   block [0x8246D6D0..0x8246D6E8)
	// 8246D6D0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246D6D4: 40980014  bge cr6, 0x8246d6e8
	if !ctx.cr[6].lt {
	pc = 0x8246D6E8; continue 'dispatch;
	}
	// 8246D6D8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246D6DC: 41990058  bgt cr6, 0x8246d734
	if ctx.cr[6].gt {
	pc = 0x8246D734; continue 'dispatch;
	}
	// 8246D6E0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8246D6E4: 40990050  ble cr6, 0x8246d734
	if !ctx.cr[6].gt {
	pc = 0x8246D734; continue 'dispatch;
	}
	pc = 0x8246D6E8; continue 'dispatch;
            }
            0x8246D6E8 => {
    //   block [0x8246D6E8..0x8246D6F8)
	// 8246D6E8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246D6EC: 4099000C  ble cr6, 0x8246d6f8
	if !ctx.cr[6].gt {
	pc = 0x8246D6F8; continue 'dispatch;
	}
	// 8246D6F0: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8246D6F4: 41980040  blt cr6, 0x8246d734
	if ctx.cr[6].lt {
	pc = 0x8246D734; continue 'dispatch;
	}
	pc = 0x8246D6F8; continue 'dispatch;
            }
            0x8246D6F8 => {
    //   block [0x8246D6F8..0x8246D734)
	// 8246D6F8: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D6FC: 7CCA492E  stwx r6, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u32) };
	// 8246D700: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D704: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D708: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246D70C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8246D710: 39060001  addi r8, r6, 1
	ctx.r[8].s64 = ctx.r[6].s64 + 1;
	// 8246D714: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246D718: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8246D71C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D720: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8246D724: 7CC6482E  lwzx r6, r6, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246D728: 7CCA492E  stwx r6, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u32) };
	// 8246D72C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D730: 7CA7512E  stwx r5, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	pc = 0x8246D734; continue 'dispatch;
            }
            0x8246D734 => {
    //   block [0x8246D734..0x8246D754)
	// 8246D734: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D738: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D73C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D740: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8246D744: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246D748: 7D27482E  lwzx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246D74C: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 8246D750: 409AFF5C  bne cr6, 0x8246d6ac
	if !ctx.cr[6].eq {
	pc = 0x8246D6AC; continue 'dispatch;
	}
	pc = 0x8246D754; continue 'dispatch;
            }
            0x8246D754 => {
    //   block [0x8246D754..0x8246D760)
	// 8246D754: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246D758: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8246D75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D760 size=232
    let mut pc: u32 = 0x8246D760;
    'dispatch: loop {
        match pc {
            0x8246D760 => {
    //   block [0x8246D760..0x8246D7B8)
	// 8246D760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D764: 480C7941  bl 0x825350a4
	ctx.lr = 0x8246D768;
	sub_82535080(ctx, base);
	// 8246D768: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246D770: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D774: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 8246D778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246D77C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246D780: 57C41838  slwi r4, r30, 3
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246D784: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D788: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D78C: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D790: 55570000  rlwinm r23, r10, 0, 0, 0
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D794: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246D798: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 8246D79C: 4BFF689D  bl 0x82464038
	ctx.lr = 0x8246D7A0;
	sub_82464038(ctx, base);
	// 8246D7A0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246D7A4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246D7A8: 40990028  ble cr6, 0x8246d7d0
	if !ctx.cr[6].gt {
	pc = 0x8246D7D0; continue 'dispatch;
	}
	// 8246D7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D7B0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246D7B4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	pc = 0x8246D7B8; continue 'dispatch;
            }
            0x8246D7B8 => {
    //   block [0x8246D7B8..0x8246D7D0)
	// 8246D7B8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D7BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D7C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246D7C4: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 8246D7C8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246D7CC: 409AFFEC  bne cr6, 0x8246d7b8
	if !ctx.cr[6].eq {
	pc = 0x8246D7B8; continue 'dispatch;
	}
	pc = 0x8246D7D0; continue 'dispatch;
            }
            0x8246D7D0 => {
    //   block [0x8246D7D0..0x8246D7F8)
	// 8246D7D0: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8246D7D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D7D8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8246D7DC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246D7E0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246D7E4: 40990040  ble cr6, 0x8246d824
	if !ctx.cr[6].gt {
	pc = 0x8246D824; continue 'dispatch;
	}
	// 8246D7E8: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D7EC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8246D7F0: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8246D7F4: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	pc = 0x8246D7F8; continue 'dispatch;
            }
            0x8246D7F8 => {
    //   block [0x8246D7F8..0x8246D810)
	// 8246D7F8: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D7FC: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 8246D800: 419A0010  beq cr6, 0x8246d810
	if ctx.cr[6].eq {
	pc = 0x8246D810; continue 'dispatch;
	}
	// 8246D804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246D808: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D80C: 4BFFFBF5  bl 0x8246d400
	ctx.lr = 0x8246D810;
	sub_8246D400(ctx, base);
	pc = 0x8246D810; continue 'dispatch;
            }
            0x8246D810 => {
    //   block [0x8246D810..0x8246D824)
	// 8246D810: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8246D814: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8246D818: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8246D81C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8246D820: 409AFFD8  bne cr6, 0x8246d7f8
	if !ctx.cr[6].eq {
	pc = 0x8246D7F8; continue 'dispatch;
	}
	pc = 0x8246D824; continue 'dispatch;
            }
            0x8246D824 => {
    //   block [0x8246D824..0x8246D840)
	// 8246D824: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8246D828: 409A0018  bne cr6, 0x8246d840
	if !ctx.cr[6].eq {
	pc = 0x8246D840; continue 'dispatch;
	}
	// 8246D82C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246D830: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246D834: 57651838  slwi r5, r27, 3
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246D838: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246D83C: 4BFF687D  bl 0x824640b8
	ctx.lr = 0x8246D840;
	sub_824640B8(ctx, base);
	pc = 0x8246D840; continue 'dispatch;
            }
            0x8246D840 => {
    //   block [0x8246D840..0x8246D848)
	// 8246D840: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246D844: 480C78B0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D848 size=52
    let mut pc: u32 = 0x8246D848;
    'dispatch: loop {
        match pc {
            0x8246D848 => {
    //   block [0x8246D848..0x8246D87C)
	// 8246D848: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D84C: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D850: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D854: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8246D858: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D85C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246D860: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D864: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246D868: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D86C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D870: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246D874: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246D878: 4BFF6840  b 0x824640b8
	sub_824640B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246D880 size=220
    let mut pc: u32 = 0x8246D880;
    'dispatch: loop {
        match pc {
            0x8246D880 => {
    //   block [0x8246D880..0x8246D8B8)
	// 8246D880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D884: 480C7839  bl 0x825350bc
	ctx.lr = 0x8246D888;
	sub_82535080(ctx, base);
	// 8246D888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246D88C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246D890: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246D894: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8246D898: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D89C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D8A0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D8A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246D8A8: 40990010  ble cr6, 0x8246d8b8
	if !ctx.cr[6].gt {
	pc = 0x8246D8B8; continue 'dispatch;
	}
	// 8246D8AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D8B0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246D8B4: 4800025D  bl 0x8246db10
	ctx.lr = 0x8246D8B8;
	sub_8246DB10(ctx, base);
	pc = 0x8246D8B8; continue 'dispatch;
            }
            0x8246D8B8 => {
    //   block [0x8246D8B8..0x8246D8E8)
	// 8246D8B8: 392079B1  li r9, 0x79b1
	ctx.r[9].s64 = 31153;
	// 8246D8BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D8C0: 7BCAE102  rldicl r10, r30, 0x3c, 4
	ctx.r[10].u64 = ctx.r[30].u64 & 0x000000000000000Fu64;
	// 8246D8C4: 65279E37  oris r7, r9, 0x9e37
	ctx.r[7].u64 = ctx.r[9].u64 | 2654404608;
	// 8246D8C8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D8CC: 7D6807B4  extsw r8, r11
	ctx.r[8].s64 = ctx.r[11].s32 as i64;
	// 8246D8D0: 7D6A39D2  mulld r11, r10, r7
	ctx.r[11].s64 = ctx.r[10].s64 * ctx.r[7].s64;
	// 8246D8D4: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D8D8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D8DC: 7CE9502A  ldx r7, r9, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	// 8246D8E0: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 8246D8E4: 419A002C  beq cr6, 0x8246d910
	if ctx.cr[6].eq {
	pc = 0x8246D910; continue 'dispatch;
	}
	pc = 0x8246D8E8; continue 'dispatch;
            }
            0x8246D8E8 => {
    //   block [0x8246D8E8..0x8246D910)
	// 8246D8E8: 7D49502A  ldx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	// 8246D8EC: 7F2AF040  cmpld cr6, r10, r30
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[30].u64, &mut ctx.xer);
	// 8246D8F0: 419A0020  beq cr6, 0x8246d910
	if ctx.cr[6].eq {
	pc = 0x8246D910; continue 'dispatch;
	}
	// 8246D8F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D8F8: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D8FC: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D900: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D904: 7CEA382A  ldx r7, r10, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) };
	// 8246D908: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 8246D90C: 409AFFDC  bne cr6, 0x8246d8e8
	if !ctx.cr[6].eq {
	pc = 0x8246D8E8; continue 'dispatch;
	}
	pc = 0x8246D910; continue 'dispatch;
            }
            0x8246D910 => {
    //   block [0x8246D910..0x8246D92C)
	// 8246D910: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D914: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8246D918: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8246D91C: 7D68482A  ldx r11, r8, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246D920: 7F2BF040  cmpld cr6, r11, r30
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[30].u64, &mut ctx.xer);
	// 8246D924: 409A0008  bne cr6, 0x8246d92c
	if !ctx.cr[6].eq {
	pc = 0x8246D92C; continue 'dispatch;
	}
	// 8246D928: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	pc = 0x8246D92C; continue 'dispatch;
            }
            0x8246D92C => {
    //   block [0x8246D92C..0x8246D95C)
	// 8246D92C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D930: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8246D934: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D938: 7FC8492A  stdx r30, r8, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u64) };
	// 8246D93C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D940: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D944: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246D948: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D94C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D950: 7FAB492A  stdx r29, r11, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u64) };
	// 8246D954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246D958: 480C77B4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D960 size=88
    let mut pc: u32 = 0x8246D960;
    'dispatch: loop {
        match pc {
            0x8246D960 => {
    //   block [0x8246D960..0x8246D990)
	// 8246D960: 394079B1  li r10, 0x79b1
	ctx.r[10].s64 = 31153;
	// 8246D964: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D968: 788BE102  rldicl r11, r4, 0x3c, 4
	ctx.r[11].u64 = ctx.r[4].u64 & 0x000000000000000Fu64;
	// 8246D96C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D970: 654A9E37  oris r10, r10, 0x9e37
	ctx.r[10].u64 = ctx.r[10].u64 | 2654404608;
	// 8246D974: 7CE807B4  extsw r8, r7
	ctx.r[8].s64 = ctx.r[7].s32 as i64;
	// 8246D978: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8246D97C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D980: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D984: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246D988: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 8246D98C: 419A0024  beq cr6, 0x8246d9b0
	if ctx.cr[6].eq {
	pc = 0x8246D9B0; continue 'dispatch;
	}
	pc = 0x8246D990; continue 'dispatch;
            }
            0x8246D990 => {
    //   block [0x8246D990..0x8246D9B0)
	// 8246D990: 7F2A2040  cmpld cr6, r10, r4
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[4].u64, &mut ctx.xer);
	// 8246D994: 419A0024  beq cr6, 0x8246d9b8
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8246D9B8);
		return;
	}
	// 8246D998: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D99C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D9A0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D9A4: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246D9A8: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 8246D9AC: 409AFFE4  bne cr6, 0x8246d990
	if !ctx.cr[6].eq {
	pc = 0x8246D990; continue 'dispatch;
	}
	pc = 0x8246D9B0; continue 'dispatch;
            }
            0x8246D9B0 => {
    //   block [0x8246D9B0..0x8246D9B8)
	// 8246D9B0: 38670001  addi r3, r7, 1
	ctx.r[3].s64 = ctx.r[7].s64 + 1;
	// 8246D9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D9C0 size=332
    let mut pc: u32 = 0x8246D9C0;
    'dispatch: loop {
        match pc {
            0x8246D9C0 => {
    //   block [0x8246D9C0..0x8246DA10)
	// 8246D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D9C4: 480C76F9  bl 0x825350bc
	ctx.lr = 0x8246D9C8;
	sub_82535080(ctx, base);
	// 8246D9C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D9CC: 7C8507B4  extsw r5, r4
	ctx.r[5].s64 = ctx.r[4].s32 as i64;
	// 8246D9D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D9D4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8246D9D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D9DC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D9E0: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D9E4: 7FCB512A  stdx r30, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u64) };
	// 8246D9E8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D9EC: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D9F0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246D9F4: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8246D9F8: 7D495838  and r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8246D9FC: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246DA00: 7D4A402A  ldx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	// 8246DA04: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 8246DA08: 419A0020  beq cr6, 0x8246da28
	if ctx.cr[6].eq {
	pc = 0x8246DA28; continue 'dispatch;
	}
	// 8246DA0C: 550A003E  slwi r10, r8, 0
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	pc = 0x8246DA10; continue 'dispatch;
            }
            0x8246DA10 => {
    //   block [0x8246DA10..0x8246DA28)
	// 8246DA10: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8246DA14: 7D295838  and r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 8246DA18: 55271838  slwi r7, r9, 3
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246DA1C: 7CE7502A  ldx r7, r7, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	// 8246DA20: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 8246DA24: 409AFFEC  bne cr6, 0x8246da10
	if !ctx.cr[6].eq {
	pc = 0x8246DA10; continue 'dispatch;
	}
	pc = 0x8246DA28; continue 'dispatch;
            }
            0x8246DA28 => {
    //   block [0x8246DA28..0x8246DA54)
	// 8246DA28: 39450001  addi r10, r5, 1
	ctx.r[10].s64 = ctx.r[5].s64 + 1;
	// 8246DA2C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8246DA30: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8246DA34: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 8246DA38: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8246DA3C: 54C71838  slwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246DA40: 7D28382A  ldx r9, r8, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	// 8246DA44: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 8246DA48: 419A00C0  beq cr6, 0x8246db08
	if ctx.cr[6].eq {
	pc = 0x8246DB08; continue 'dispatch;
	}
	// 8246DA4C: 392079B1  li r9, 0x79b1
	ctx.r[9].s64 = 31153;
	// 8246DA50: 65249E37  oris r4, r9, 0x9e37
	ctx.r[4].u64 = ctx.r[9].u64 | 2654404608;
	pc = 0x8246DA54; continue 'dispatch;
            }
            0x8246DA54 => {
    //   block [0x8246DA54..0x8246DA78)
	// 8246DA54: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DA58: 7F2AF840  cmpld cr6, r10, r31
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[31].u64, &mut ctx.xer);
	// 8246DA5C: 7D09382A  ldx r8, r9, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) };
	// 8246DA60: 791DE102  rldicl r29, r8, 0x3c, 4
	ctx.r[29].u64 = ctx.r[8].u64 & 0x000000000000000Fu64;
	// 8246DA64: 7FBD21D2  mulld r29, r29, r4
	ctx.r[29].s64 = ctx.r[29].s64 * ctx.r[4].s64;
	// 8246DA68: 7FAB5838  and r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 & ctx.r[11].u64;
	// 8246DA6C: 4198000C  blt cr6, 0x8246da78
	if ctx.cr[6].lt {
	pc = 0x8246DA78; continue 'dispatch;
	}
	// 8246DA70: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA74: 4199006C  bgt cr6, 0x8246dae0
	if ctx.cr[6].gt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	pc = 0x8246DA78; continue 'dispatch;
            }
            0x8246DA78 => {
    //   block [0x8246DA78..0x8246DA90)
	// 8246DA78: 7F2A2840  cmpld cr6, r10, r5
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA7C: 40980014  bge cr6, 0x8246da90
	if !ctx.cr[6].lt {
	pc = 0x8246DA90; continue 'dispatch;
	}
	// 8246DA80: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA84: 4199005C  bgt cr6, 0x8246dae0
	if ctx.cr[6].gt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	// 8246DA88: 7F2B5040  cmpld cr6, r11, r10
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[10].u64, &mut ctx.xer);
	// 8246DA8C: 40990054  ble cr6, 0x8246dae0
	if !ctx.cr[6].gt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	pc = 0x8246DA90; continue 'dispatch;
            }
            0x8246DA90 => {
    //   block [0x8246DA90..0x8246DAA0)
	// 8246DA90: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA94: 4099000C  ble cr6, 0x8246daa0
	if !ctx.cr[6].gt {
	pc = 0x8246DAA0; continue 'dispatch;
	}
	// 8246DA98: 7F2BF840  cmpld cr6, r11, r31
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[31].u64, &mut ctx.xer);
	// 8246DA9C: 41980044  blt cr6, 0x8246dae0
	if ctx.cr[6].lt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	pc = 0x8246DAA0; continue 'dispatch;
            }
            0x8246DAA0 => {
    //   block [0x8246DAA0..0x8246DAE0)
	// 8246DAA0: 54AB003E  slwi r11, r5, 0
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DAA4: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8246DAA8: 557D1838  slwi r29, r11, 3
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8246DAAC: 7D1D492A  stdx r8, r29, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u64) };
	// 8246DAB0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DAB4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DAB8: 7CC93214  add r6, r9, r6
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 8246DABC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8246DAC0: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 8246DAC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246DAC8: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8246DACC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DAD0: 7D29402A  ldx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	// 8246DAD4: 7D2B412A  stdx r9, r11, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 8246DAD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DADC: 7FC7592A  stdx r30, r7, r11
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u64) };
	pc = 0x8246DAE0; continue 'dispatch;
            }
            0x8246DAE0 => {
    //   block [0x8246DAE0..0x8246DB08)
	// 8246DAE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DAE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246DAE8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DAEC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246DAF0: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8246DAF4: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8246DAF8: 54C71838  slwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246DAFC: 7D27482A  ldx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246DB00: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 8246DB04: 409AFF50  bne cr6, 0x8246da54
	if !ctx.cr[6].eq {
	pc = 0x8246DA54; continue 'dispatch;
	}
	pc = 0x8246DB08; continue 'dispatch;
            }
            0x8246DB08 => {
    //   block [0x8246DB08..0x8246DB0C)
	// 8246DB08: 480C7604  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DB10 size=232
    let mut pc: u32 = 0x8246DB10;
    'dispatch: loop {
        match pc {
            0x8246DB10 => {
    //   block [0x8246DB10..0x8246DB68)
	// 8246DB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DB14: 480C7591  bl 0x825350a4
	ctx.lr = 0x8246DB18;
	sub_82535080(ctx, base);
	// 8246DB18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DB1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246DB20: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DB24: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 8246DB28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246DB2C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246DB30: 57C42036  slwi r4, r30, 4
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246DB34: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DB38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DB3C: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DB40: 55570000  rlwinm r23, r10, 0, 0, 0
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8246DB44: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246DB48: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 8246DB4C: 4BFF64ED  bl 0x82464038
	ctx.lr = 0x8246DB50;
	sub_82464038(ctx, base);
	// 8246DB50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246DB54: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246DB58: 40990028  ble cr6, 0x8246db80
	if !ctx.cr[6].gt {
	pc = 0x8246DB80; continue 'dispatch;
	}
	// 8246DB5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246DB60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246DB64: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	pc = 0x8246DB68; continue 'dispatch;
            }
            0x8246DB68 => {
    //   block [0x8246DB68..0x8246DB80)
	// 8246DB68: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DB6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246DB70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246DB74: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 8246DB78: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8246DB7C: 409AFFEC  bne cr6, 0x8246db68
	if !ctx.cr[6].eq {
	pc = 0x8246DB68; continue 'dispatch;
	}
	pc = 0x8246DB80; continue 'dispatch;
            }
            0x8246DB80 => {
    //   block [0x8246DB80..0x8246DBA8)
	// 8246DB80: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8246DB84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246DB88: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8246DB8C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246DB90: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246DB94: 40990040  ble cr6, 0x8246dbd4
	if !ctx.cr[6].gt {
	pc = 0x8246DBD4; continue 'dispatch;
	}
	// 8246DB98: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DB9C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8246DBA0: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8246DBA4: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	pc = 0x8246DBA8; continue 'dispatch;
            }
            0x8246DBA8 => {
    //   block [0x8246DBA8..0x8246DBC0)
	// 8246DBA8: E89D0000  ld r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 8246DBAC: 2F24FFFF  cmpdi cr6, r4, -1
	ctx.cr[6].compare_i64(ctx.r[4].s64, -1, &mut ctx.xer);
	// 8246DBB0: 419A0010  beq cr6, 0x8246dbc0
	if ctx.cr[6].eq {
	pc = 0x8246DBC0; continue 'dispatch;
	}
	// 8246DBB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DBB8: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8246DBBC: 4BFFFCC5  bl 0x8246d880
	ctx.lr = 0x8246DBC0;
	sub_8246D880(ctx, base);
	pc = 0x8246DBC0; continue 'dispatch;
            }
            0x8246DBC0 => {
    //   block [0x8246DBC0..0x8246DBD4)
	// 8246DBC0: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8246DBC4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8246DBC8: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 8246DBCC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8246DBD0: 409AFFD8  bne cr6, 0x8246dba8
	if !ctx.cr[6].eq {
	pc = 0x8246DBA8; continue 'dispatch;
	}
	pc = 0x8246DBD4; continue 'dispatch;
            }
            0x8246DBD4 => {
    //   block [0x8246DBD4..0x8246DBF0)
	// 8246DBD4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8246DBD8: 409A0018  bne cr6, 0x8246dbf0
	if !ctx.cr[6].eq {
	pc = 0x8246DBF0; continue 'dispatch;
	}
	// 8246DBDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246DBE0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246DBE4: 57652036  slwi r5, r27, 4
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246DBE8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246DBEC: 4BFF64CD  bl 0x824640b8
	ctx.lr = 0x8246DBF0;
	sub_824640B8(ctx, base);
	pc = 0x8246DBF0; continue 'dispatch;
            }
            0x8246DBF0 => {
    //   block [0x8246DBF0..0x8246DBF8)
	// 8246DBF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246DBF4: 480C7500  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DBF8 size=116
    let mut pc: u32 = 0x8246DBF8;
    'dispatch: loop {
        match pc {
            0x8246DBF8 => {
    //   block [0x8246DBF8..0x8246DC28)
	// 8246DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246DC00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246DC04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DC08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246DC0C: 4BFFF8CD  bl 0x8246d4d8
	ctx.lr = 0x8246DC10;
	sub_8246D4D8(ctx, base);
	// 8246DC10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DC14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8246DC18: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DC1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246DC20: 40990008  ble cr6, 0x8246dc28
	if !ctx.cr[6].gt {
	pc = 0x8246DC28; continue 'dispatch;
	}
	// 8246DC24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8246DC28; continue 'dispatch;
            }
            0x8246DC28 => {
    //   block [0x8246DC28..0x8246DC54)
	// 8246DC28: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246DC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246DC30: 419A0024  beq cr6, 0x8246dc54
	if ctx.cr[6].eq {
	pc = 0x8246DC54; continue 'dispatch;
	}
	// 8246DC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DC38: 4BFFF9E9  bl 0x8246d620
	ctx.lr = 0x8246DC3C;
	sub_8246D620(ctx, base);
	// 8246DC3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246DC40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246DC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246DC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246DC4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246DC50: 4E800020  blr
	return;
            }
            0x8246DC54 => {
    //   block [0x8246DC54..0x8246DC6C)
	// 8246DC54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246DC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246DC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246DC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246DC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246DC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246DC70 size=40
    let mut pc: u32 = 0x8246DC70;
    'dispatch: loop {
        match pc {
            0x8246DC70 => {
    //   block [0x8246DC70..0x8246DC84)
	// 8246DC70: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246DC74: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8246DC78: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 8246DC7C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8246DC80: 40990010  ble cr6, 0x8246dc90
	if !ctx.cr[6].gt {
	pc = 0x8246DC90; continue 'dispatch;
	}
	pc = 0x8246DC84; continue 'dispatch;
            }
            0x8246DC84 => {
    //   block [0x8246DC84..0x8246DC90)
	// 8246DC84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DC88: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246DC8C: 4198FFF8  blt cr6, 0x8246dc84
	if ctx.cr[6].lt {
	pc = 0x8246DC84; continue 'dispatch;
	}
	pc = 0x8246DC90; continue 'dispatch;
            }
            0x8246DC90 => {
    //   block [0x8246DC90..0x8246DC98)
	// 8246DC90: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DC94: 4BFFFACC  b 0x8246d760
	sub_8246D760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DC98 size=412
    let mut pc: u32 = 0x8246DC98;
    'dispatch: loop {
        match pc {
            0x8246DC98 => {
    //   block [0x8246DC98..0x8246DCF0)
	// 8246DC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DC9C: 480C7419  bl 0x825350b4
	ctx.lr = 0x8246DCA0;
	sub_82535080(ctx, base);
	// 8246DCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DCA4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246DCA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246DCAC: 3B6B8728  addi r27, r11, -0x78d8
	ctx.r[27].s64 = ctx.r[11].s64 + -30936;
	// 8246DCB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246DCB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246DCB8: 4BFFC569  bl 0x8246a220
	ctx.lr = 0x8246DCBC;
	sub_8246A220(ctx, base);
	// 8246DCBC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246DCC0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246DCC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DCC8: 4BFFC339  bl 0x8246a000
	ctx.lr = 0x8246DCCC;
	sub_8246A000(ctx, base);
	// 8246DCCC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246DCD0: 419A0020  beq cr6, 0x8246dcf0
	if ctx.cr[6].eq {
	pc = 0x8246DCF0; continue 'dispatch;
	}
	// 8246DCD4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246DCD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DCDC: 388B8724  addi r4, r11, -0x78dc
	ctx.r[4].s64 = ctx.r[11].s64 + -30940;
	// 8246DCE0: 4BFFC5A1  bl 0x8246a280
	ctx.lr = 0x8246DCE4;
	sub_8246A280(ctx, base);
	// 8246DCE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246DCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246DCEC: 419A0008  beq cr6, 0x8246dcf4
	if ctx.cr[6].eq {
	pc = 0x8246DCF4; continue 'dispatch;
	}
	pc = 0x8246DCF0; continue 'dispatch;
            }
            0x8246DCF0 => {
    //   block [0x8246DCF0..0x8246DCF4)
	// 8246DCF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x8246DCF4; continue 'dispatch;
            }
            0x8246DCF4 => {
    //   block [0x8246DCF4..0x8246DD04)
	// 8246DCF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246DCF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246DCFC: 409A0094  bne cr6, 0x8246dd90
	if !ctx.cr[6].eq {
	pc = 0x8246DD90; continue 'dispatch;
	}
	// 8246DD00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x8246DD04; continue 'dispatch;
            }
            0x8246DD04 => {
    //   block [0x8246DD04..0x8246DD1C)
	// 8246DD04: 7D7EE8AE  lbzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8246DD08: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246DD0C: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 8246DD10: 419A000C  beq cr6, 0x8246dd1c
	if ctx.cr[6].eq {
	pc = 0x8246DD1C; continue 'dispatch;
	}
	// 8246DD14: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 8246DD18: 409A000C  bne cr6, 0x8246dd24
	if !ctx.cr[6].eq {
	pc = 0x8246DD24; continue 'dispatch;
	}
	pc = 0x8246DD1C; continue 'dispatch;
            }
            0x8246DD1C => {
    //   block [0x8246DD1C..0x8246DD24)
	// 8246DD1C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8246DD20: 4BFFFFE4  b 0x8246dd04
	pc = 0x8246DD04; continue 'dispatch;
            }
            0x8246DD24 => {
    //   block [0x8246DD24..0x8246DD64)
	// 8246DD24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246DD28: 4BFFC4F9  bl 0x8246a220
	ctx.lr = 0x8246DD2C;
	sub_8246A220(ctx, base);
	// 8246DD2C: 7FBEEA14  add r29, r30, r29
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 8246DD30: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246DD34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DD38: 4BFFC4E9  bl 0x8246a220
	ctx.lr = 0x8246DD3C;
	sub_8246A220(ctx, base);
	// 8246DD3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DD40: 7D43E214  add r10, r3, r28
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 8246DD44: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DD48: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 8246DD4C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246DD50: 40980024  bge cr6, 0x8246dd74
	if !ctx.cr[6].lt {
	pc = 0x8246DD74; continue 'dispatch;
	}
	// 8246DD54: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DD58: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DD5C: 41980008  blt cr6, 0x8246dd64
	if ctx.cr[6].lt {
	pc = 0x8246DD64; continue 'dispatch;
	}
	// 8246DD60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246DD64; continue 'dispatch;
            }
            0x8246DD64 => {
    //   block [0x8246DD64..0x8246DD74)
	// 8246DD64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DD68: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DD6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DD70: 48000559  bl 0x8246e2c8
	ctx.lr = 0x8246DD74;
	sub_8246E2C8(ctx, base);
	pc = 0x8246DD74; continue 'dispatch;
            }
            0x8246DD74 => {
    //   block [0x8246DD74..0x8246DD90)
	// 8246DD74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246DD78: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DD7C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246DD80: 4BFFC471  bl 0x8246a1f0
	ctx.lr = 0x8246DD84;
	sub_8246A1F0(ctx, base);
	// 8246DD84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DD88: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8246DD8C: 48000048  b 0x8246ddd4
	pc = 0x8246DDD4; continue 'dispatch;
            }
            0x8246DD90 => {
    //   block [0x8246DD90..0x8246DDBC)
	// 8246DD90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DD94: 4BFFC48D  bl 0x8246a220
	ctx.lr = 0x8246DD98;
	sub_8246A220(ctx, base);
	// 8246DD98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DD9C: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 8246DDA0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DDA4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246DDA8: 40980024  bge cr6, 0x8246ddcc
	if !ctx.cr[6].lt {
	pc = 0x8246DDCC; continue 'dispatch;
	}
	// 8246DDAC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DDB0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DDB4: 41980008  blt cr6, 0x8246ddbc
	if ctx.cr[6].lt {
	pc = 0x8246DDBC; continue 'dispatch;
	}
	// 8246DDB8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246DDBC; continue 'dispatch;
            }
            0x8246DDBC => {
    //   block [0x8246DDBC..0x8246DDCC)
	// 8246DDBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DDC0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DDC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DDC8: 48000501  bl 0x8246e2c8
	ctx.lr = 0x8246DDCC;
	sub_8246E2C8(ctx, base);
	pc = 0x8246DDCC; continue 'dispatch;
            }
            0x8246DDCC => {
    //   block [0x8246DDCC..0x8246DDD4)
	// 8246DDCC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DDD0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	pc = 0x8246DDD4; continue 'dispatch;
            }
            0x8246DDD4 => {
    //   block [0x8246DDD4..0x8246DE04)
	// 8246DDD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246DDD8: 4BFFC419  bl 0x8246a1f0
	ctx.lr = 0x8246DDDC;
	sub_8246A1F0(ctx, base);
	// 8246DDDC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DDE0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DDE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246DDE8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8246DDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246DDF0: 992AFFFF  stb r9, -1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 8246DDF4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DDF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246DDFC: 4099002C  ble cr6, 0x8246de28
	if !ctx.cr[6].gt {
	pc = 0x8246DE28; continue 'dispatch;
	}
	// 8246DE00: 3920005C  li r9, 0x5c
	ctx.r[9].s64 = 92;
	pc = 0x8246DE04; continue 'dispatch;
            }
            0x8246DE04 => {
    //   block [0x8246DE04..0x8246DE18)
	// 8246DE04: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DE08: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246DE0C: 2B08002F  cmplwi cr6, r8, 0x2f
	ctx.cr[6].compare_u32(ctx.r[8].u32, 47 as u32, &mut ctx.xer);
	// 8246DE10: 409A0008  bne cr6, 0x8246de18
	if !ctx.cr[6].eq {
	pc = 0x8246DE18; continue 'dispatch;
	}
	// 8246DE14: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	pc = 0x8246DE18; continue 'dispatch;
            }
            0x8246DE18 => {
    //   block [0x8246DE18..0x8246DE28)
	// 8246DE18: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DE1C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246DE20: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246DE24: 4198FFE0  blt cr6, 0x8246de04
	if ctx.cr[6].lt {
	pc = 0x8246DE04; continue 'dispatch;
	}
	pc = 0x8246DE28; continue 'dispatch;
            }
            0x8246DE28 => {
    //   block [0x8246DE28..0x8246DE34)
	// 8246DE28: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246DE30: 480C72D4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DE38 size=1168
    let mut pc: u32 = 0x8246DE38;
    'dispatch: loop {
        match pc {
            0x8246DE38 => {
    //   block [0x8246DE38..0x8246DE98)
	// 8246DE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DE3C: 480C726D  bl 0x825350a8
	ctx.lr = 0x8246DE40;
	sub_82535080(ctx, base);
	// 8246DE40: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DE44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246DE48: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 8246DE4C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246DE50: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8246DE54: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 8246DE58: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8246DE5C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8246DE60: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8246DE64: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8246DE68: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 8246DE6C: 419A0054  beq cr6, 0x8246dec0
	if ctx.cr[6].eq {
	pc = 0x8246DEC0; continue 'dispatch;
	}
	// 8246DE70: 4BFFC3B1  bl 0x8246a220
	ctx.lr = 0x8246DE74;
	sub_8246A220(ctx, base);
	// 8246DE74: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246DE78: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246DE7C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DE80: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246DE84: 40980024  bge cr6, 0x8246dea8
	if !ctx.cr[6].lt {
	pc = 0x8246DEA8; continue 'dispatch;
	}
	// 8246DE88: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DE8C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DE90: 41980008  blt cr6, 0x8246de98
	if ctx.cr[6].lt {
	pc = 0x8246DE98; continue 'dispatch;
	}
	// 8246DE94: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246DE98; continue 'dispatch;
            }
            0x8246DE98 => {
    //   block [0x8246DE98..0x8246DEA8)
	// 8246DE98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DE9C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DEA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246DEA4: 48000425  bl 0x8246e2c8
	ctx.lr = 0x8246DEA8;
	sub_8246E2C8(ctx, base);
	pc = 0x8246DEA8; continue 'dispatch;
            }
            0x8246DEA8 => {
    //   block [0x8246DEA8..0x8246DEC0)
	// 8246DEA8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246DEAC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246DEB0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246DEB4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8246DEB8: 4BFFC471  bl 0x8246a328
	ctx.lr = 0x8246DEBC;
	sub_8246A328(ctx, base);
	// 8246DEBC: 48000020  b 0x8246dedc
	pc = 0x8246DEDC; continue 'dispatch;
            }
            0x8246DEC0 => {
    //   block [0x8246DEC0..0x8246DEDC)
	// 8246DEC0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DEC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246DEC8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246DECC: 480003FD  bl 0x8246e2c8
	ctx.lr = 0x8246DED0;
	sub_8246E2C8(ctx, base);
	// 8246DED0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246DED4: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 8246DED8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	pc = 0x8246DEDC; continue 'dispatch;
            }
            0x8246DEDC => {
    //   block [0x8246DEDC..0x8246DF1C)
	// 8246DEDC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8246DEE0: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 8246DEE4: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 8246DEE8: 3B8BDEB0  addi r28, r11, -0x2150
	ctx.r[28].s64 = ctx.r[11].s64 + -8528;
	// 8246DEEC: 93610080  stw r27, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u32 ) };
	// 8246DEF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246DEF4: 4BFFC32D  bl 0x8246a220
	ctx.lr = 0x8246DEF8;
	sub_8246A220(ctx, base);
	// 8246DEF8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8246DEFC: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246DF00: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DF04: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246DF08: 40980024  bge cr6, 0x8246df2c
	if !ctx.cr[6].lt {
	pc = 0x8246DF2C; continue 'dispatch;
	}
	// 8246DF0C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DF10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DF14: 41980008  blt cr6, 0x8246df1c
	if ctx.cr[6].lt {
	pc = 0x8246DF1C; continue 'dispatch;
	}
	// 8246DF18: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246DF1C; continue 'dispatch;
            }
            0x8246DF1C => {
    //   block [0x8246DF1C..0x8246DF2C)
	// 8246DF1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DF20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DF24: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8246DF28: 480003A1  bl 0x8246e2c8
	ctx.lr = 0x8246DF2C;
	sub_8246E2C8(ctx, base);
	pc = 0x8246DF2C; continue 'dispatch;
            }
            0x8246DF2C => {
    //   block [0x8246DF2C..0x8246DF80)
	// 8246DF2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246DF30: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8246DF34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246DF38: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 8246DF3C: 4BFFC3ED  bl 0x8246a328
	ctx.lr = 0x8246DF40;
	sub_8246A328(ctx, base);
	// 8246DF40: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246DF44: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 8246DF48: 3BCB8734  addi r30, r11, -0x78cc
	ctx.r[30].s64 = ctx.r[11].s64 + -30924;
	// 8246DF4C: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8246DF50: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 8246DF54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246DF58: 4BFFC2C9  bl 0x8246a220
	ctx.lr = 0x8246DF5C;
	sub_8246A220(ctx, base);
	// 8246DF5C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246DF60: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246DF64: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DF68: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246DF6C: 40980024  bge cr6, 0x8246df90
	if !ctx.cr[6].lt {
	pc = 0x8246DF90; continue 'dispatch;
	}
	// 8246DF70: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DF74: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DF78: 41980008  blt cr6, 0x8246df80
	if ctx.cr[6].lt {
	pc = 0x8246DF80; continue 'dispatch;
	}
	// 8246DF7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246DF80; continue 'dispatch;
            }
            0x8246DF80 => {
    //   block [0x8246DF80..0x8246DF90)
	// 8246DF80: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DF84: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DF88: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8246DF8C: 4800033D  bl 0x8246e2c8
	ctx.lr = 0x8246DF90;
	sub_8246E2C8(ctx, base);
	pc = 0x8246DF90; continue 'dispatch;
            }
            0x8246DF90 => {
    //   block [0x8246DF90..0x8246DFE8)
	// 8246DF90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246DF94: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246DF98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246DF9C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 8246DFA0: 4BFFC389  bl 0x8246a328
	ctx.lr = 0x8246DFA4;
	sub_8246A328(ctx, base);
	// 8246DFA4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8246DFA8: 38C10078  addi r6, r1, 0x78
	ctx.r[6].s64 = ctx.r[1].s64 + 120;
	// 8246DFAC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8246DFB0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8246DFB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246DFB8: 4BFFD071  bl 0x8246b028
	ctx.lr = 0x8246DFBC;
	sub_8246B028(ctx, base);
	// 8246DFBC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246DFC0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246DFC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246DFC8: 409A0020  bne cr6, 0x8246dfe8
	if !ctx.cr[6].eq {
	pc = 0x8246DFE8; continue 'dispatch;
	}
	// 8246DFCC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DFD0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246DFD4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246DFD8: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246DFDC: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DFE0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246DFE4: 4BFF60D5  bl 0x824640b8
	ctx.lr = 0x8246DFE8;
	sub_824640B8(ctx, base);
	pc = 0x8246DFE8; continue 'dispatch;
            }
            0x8246DFE8 => {
    //   block [0x8246DFE8..0x8246E014)
	// 8246DFE8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8246DFEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246DFF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246DFF4: 409A0020  bne cr6, 0x8246e014
	if !ctx.cr[6].eq {
	pc = 0x8246E014; continue 'dispatch;
	}
	// 8246DFF8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DFFC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E000: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E004: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8246E008: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E00C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E010: 4BFF60A9  bl 0x824640b8
	ctx.lr = 0x8246E014;
	sub_824640B8(ctx, base);
	pc = 0x8246E014; continue 'dispatch;
            }
            0x8246E014 => {
    //   block [0x8246E014..0x8246E04C)
	// 8246E014: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246E018: 93A10098  stw r29, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[29].u32 ) };
	// 8246E01C: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 8246E020: 936100A0  stw r27, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 8246E024: 4BFFC1FD  bl 0x8246a220
	ctx.lr = 0x8246E028;
	sub_8246A220(ctx, base);
	// 8246E028: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 8246E02C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246E030: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E034: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E038: 40980024  bge cr6, 0x8246e05c
	if !ctx.cr[6].lt {
	pc = 0x8246E05C; continue 'dispatch;
	}
	// 8246E03C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E040: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E044: 41980008  blt cr6, 0x8246e04c
	if ctx.cr[6].lt {
	pc = 0x8246E04C; continue 'dispatch;
	}
	// 8246E048: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246E04C; continue 'dispatch;
            }
            0x8246E04C => {
    //   block [0x8246E04C..0x8246E05C)
	// 8246E04C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E050: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E054: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8246E058: 48000271  bl 0x8246e2c8
	ctx.lr = 0x8246E05C;
	sub_8246E2C8(ctx, base);
	pc = 0x8246E05C; continue 'dispatch;
            }
            0x8246E05C => {
    //   block [0x8246E05C..0x8246E0B0)
	// 8246E05C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E060: 80610098  lwz r3, 0x98(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 8246E064: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246E068: 93E1009C  stw r31, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 8246E06C: 4BFFC2BD  bl 0x8246a328
	ctx.lr = 0x8246E070;
	sub_8246A328(ctx, base);
	// 8246E070: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246E074: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 8246E078: 3BCB8730  addi r30, r11, -0x78d0
	ctx.r[30].s64 = ctx.r[11].s64 + -30928;
	// 8246E07C: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 8246E080: 93610090  stw r27, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[27].u32 ) };
	// 8246E084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246E088: 4BFFC199  bl 0x8246a220
	ctx.lr = 0x8246E08C;
	sub_8246A220(ctx, base);
	// 8246E08C: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8246E090: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246E094: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E098: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E09C: 40980024  bge cr6, 0x8246e0c0
	if !ctx.cr[6].lt {
	pc = 0x8246E0C0; continue 'dispatch;
	}
	// 8246E0A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E0A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E0A8: 41980008  blt cr6, 0x8246e0b0
	if ctx.cr[6].lt {
	pc = 0x8246E0B0; continue 'dispatch;
	}
	// 8246E0AC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246E0B0; continue 'dispatch;
            }
            0x8246E0B0 => {
    //   block [0x8246E0B0..0x8246E0C0)
	// 8246E0B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E0B4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E0B8: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8246E0BC: 4800020D  bl 0x8246e2c8
	ctx.lr = 0x8246E0C0;
	sub_8246E2C8(ctx, base);
	pc = 0x8246E0C0; continue 'dispatch;
            }
            0x8246E0C0 => {
    //   block [0x8246E0C0..0x8246E108)
	// 8246E0C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E0C4: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8246E0C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246E0CC: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 8246E0D0: 4BFFC259  bl 0x8246a328
	ctx.lr = 0x8246E0D4;
	sub_8246A328(ctx, base);
	// 8246E0D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8246E0D8: 38C10098  addi r6, r1, 0x98
	ctx.r[6].s64 = ctx.r[1].s64 + 152;
	// 8246E0DC: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 8246E0E0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8246E0E4: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8246E0E8: 4BFFCF41  bl 0x8246b028
	ctx.lr = 0x8246E0EC;
	sub_8246B028(ctx, base);
	// 8246E0EC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E0F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E0F4: 409A0014  bne cr6, 0x8246e108
	if !ctx.cr[6].eq {
	pc = 0x8246E108; continue 'dispatch;
	}
	// 8246E0F8: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246E0FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E100: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8246E104: 419A0008  beq cr6, 0x8246e10c
	if ctx.cr[6].eq {
	pc = 0x8246E10C; continue 'dispatch;
	}
	pc = 0x8246E108; continue 'dispatch;
            }
            0x8246E108 => {
    //   block [0x8246E108..0x8246E10C)
	// 8246E108: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	pc = 0x8246E10C; continue 'dispatch;
            }
            0x8246E10C => {
    //   block [0x8246E10C..0x8246E13C)
	// 8246E10C: 81410090  lwz r10, 0x90(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8246E110: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8246E114: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E118: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246E11C: 409A0020  bne cr6, 0x8246e13c
	if !ctx.cr[6].eq {
	pc = 0x8246E13C; continue 'dispatch;
	}
	// 8246E120: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E124: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E128: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E12C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8246E130: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E134: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E138: 4BFF5F81  bl 0x824640b8
	ctx.lr = 0x8246E13C;
	sub_824640B8(ctx, base);
	pc = 0x8246E13C; continue 'dispatch;
            }
            0x8246E13C => {
    //   block [0x8246E13C..0x8246E168)
	// 8246E13C: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 8246E140: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E144: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E148: 409A0020  bne cr6, 0x8246e168
	if !ctx.cr[6].eq {
	pc = 0x8246E168; continue 'dispatch;
	}
	// 8246E14C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E150: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E154: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E158: 80810098  lwz r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 8246E15C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E160: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E164: 4BFF5F55  bl 0x824640b8
	ctx.lr = 0x8246E168;
	sub_824640B8(ctx, base);
	pc = 0x8246E168; continue 'dispatch;
            }
            0x8246E168 => {
    //   block [0x8246E168..0x8246E1A8)
	// 8246E168: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246E16C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8246E170: 38AB872C  addi r5, r11, -0x78d4
	ctx.r[5].s64 = ctx.r[11].s64 + -30932;
	// 8246E174: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8246E178: 4BFFC7B9  bl 0x8246a930
	ctx.lr = 0x8246E17C;
	sub_8246A930(ctx, base);
	// 8246E17C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E184: 419A007C  beq cr6, 0x8246e200
	if ctx.cr[6].eq {
	pc = 0x8246E200; continue 'dispatch;
	}
	// 8246E188: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8246E18C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8246E190: 615EFFFF  ori r30, r10, 0xffff
	ctx.r[30].u64 = ctx.r[10].u64 | 65535;
	// 8246E194: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 8246E198: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8246E19C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246E1A0: 40980008  bge cr6, 0x8246e1a8
	if !ctx.cr[6].lt {
	pc = 0x8246E1A8; continue 'dispatch;
	}
	// 8246E1A4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x8246E1A8; continue 'dispatch;
            }
            0x8246E1A8 => {
    //   block [0x8246E1A8..0x8246E1E4)
	// 8246E1A8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E1AC: 3BFE0001  addi r31, r30, 1
	ctx.r[31].s64 = ctx.r[30].s64 + 1;
	// 8246E1B0: 38830002  addi r4, r3, 2
	ctx.r[4].s64 = ctx.r[3].s64 + 2;
	// 8246E1B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E1B8: 4BFFC179  bl 0x8246a330
	ctx.lr = 0x8246E1BC;
	sub_8246A330(ctx, base);
	// 8246E1BC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E1C0: 7FBE59AE  stbx r29, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	// 8246E1C4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246E1C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E1CC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E1D0: 40980024  bge cr6, 0x8246e1f4
	if !ctx.cr[6].lt {
	pc = 0x8246E1F4; continue 'dispatch;
	}
	// 8246E1D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E1D8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E1DC: 41980008  blt cr6, 0x8246e1e4
	if ctx.cr[6].lt {
	pc = 0x8246E1E4; continue 'dispatch;
	}
	// 8246E1E0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246E1E4; continue 'dispatch;
            }
            0x8246E1E4 => {
    //   block [0x8246E1E4..0x8246E1F4)
	// 8246E1E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E1E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E1EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246E1F0: 480000D9  bl 0x8246e2c8
	ctx.lr = 0x8246E1F4;
	sub_8246E2C8(ctx, base);
	pc = 0x8246E1F4; continue 'dispatch;
            }
            0x8246E1F4 => {
    //   block [0x8246E1F4..0x8246E200)
	// 8246E1F4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8246E1F8: 9B010050  stb r24, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u8 ) };
	// 8246E1FC: 48000008  b 0x8246e204
	pc = 0x8246E204; continue 'dispatch;
            }
            0x8246E200 => {
    //   block [0x8246E200..0x8246E204)
	// 8246E200: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	pc = 0x8246E204; continue 'dispatch;
            }
            0x8246E204 => {
    //   block [0x8246E204..0x8246E230)
	// 8246E204: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246E208: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E20C: 419A0084  beq cr6, 0x8246e290
	if ctx.cr[6].eq {
	pc = 0x8246E290; continue 'dispatch;
	}
	// 8246E210: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E214: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E218: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E21C: 40980024  bge cr6, 0x8246e240
	if !ctx.cr[6].lt {
	pc = 0x8246E240; continue 'dispatch;
	}
	// 8246E220: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E224: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E228: 41980008  blt cr6, 0x8246e230
	if ctx.cr[6].lt {
	pc = 0x8246E230; continue 'dispatch;
	}
	// 8246E22C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246E230; continue 'dispatch;
            }
            0x8246E230 => {
    //   block [0x8246E230..0x8246E240)
	// 8246E230: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E234: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E238: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8246E23C: 4800008D  bl 0x8246e2c8
	ctx.lr = 0x8246E240;
	sub_8246E2C8(ctx, base);
	pc = 0x8246E240; continue 'dispatch;
            }
            0x8246E240 => {
    //   block [0x8246E240..0x8246E284)
	// 8246E240: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E244: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E248: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E24C: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8246E250: 4BFFC0D9  bl 0x8246a328
	ctx.lr = 0x8246E254;
	sub_8246A328(ctx, base);
	// 8246E254: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246E258: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E25C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E260: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E264: 409A0020  bne cr6, 0x8246e284
	if !ctx.cr[6].eq {
	pc = 0x8246E284; continue 'dispatch;
	}
	// 8246E268: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E26C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E270: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E274: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E278: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E27C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E280: 4BFF5E39  bl 0x824640b8
	ctx.lr = 0x8246E284;
	sub_824640B8(ctx, base);
	pc = 0x8246E284; continue 'dispatch;
            }
            0x8246E284 => {
    //   block [0x8246E284..0x8246E290)
	// 8246E284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E288: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8246E28C: 480C6E6C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E290 => {
    //   block [0x8246E290..0x8246E2BC)
	// 8246E290: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246E294: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E298: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E29C: 409A0020  bne cr6, 0x8246e2bc
	if !ctx.cr[6].eq {
	pc = 0x8246E2BC; continue 'dispatch;
	}
	// 8246E2A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E2A4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E2A8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E2AC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E2B0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E2B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E2B8: 4BFF5E01  bl 0x824640b8
	ctx.lr = 0x8246E2BC;
	sub_824640B8(ctx, base);
	pc = 0x8246E2BC; continue 'dispatch;
            }
            0x8246E2BC => {
    //   block [0x8246E2BC..0x8246E2C8)
	// 8246E2BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246E2C0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8246E2C4: 480C6E34  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E2C8 size=136
    let mut pc: u32 = 0x8246E2C8;
    'dispatch: loop {
        match pc {
            0x8246E2C8 => {
    //   block [0x8246E2C8..0x8246E334)
	// 8246E2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E2CC: 480C6DE5  bl 0x825350b0
	ctx.lr = 0x8246E2D0;
	sub_82535080(ctx, base);
	// 8246E2D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E2D4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E2D8: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8246E2DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246E2E0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8246E2E4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246E2E8: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s32 = ((ctx.r[27].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 8246E2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E2F0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E2F4: 4BFF5D45  bl 0x82464038
	ctx.lr = 0x8246E2F8;
	sub_82464038(ctx, base);
	// 8246E2F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E2FC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246E300: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E304: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246E308: 480C6849  bl 0x82534b50
	ctx.lr = 0x8246E30C;
	sub_82534B50(ctx, base);
	// 8246E30C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E310: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E314: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E318: 409A001C  bne cr6, 0x8246e334
	if !ctx.cr[6].eq {
	pc = 0x8246E334; continue 'dispatch;
	}
	// 8246E31C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E320: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E324: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E328: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E32C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246E330: 4BFF5D89  bl 0x824640b8
	ctx.lr = 0x8246E334;
	sub_824640B8(ctx, base);
	pc = 0x8246E334; continue 'dispatch;
            }
            0x8246E334 => {
    //   block [0x8246E334..0x8246E350)
	// 8246E334: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E338: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8246E33C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E340: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 8246E344: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246E348: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246E34C: 480C6DB4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E350 size=152
    let mut pc: u32 = 0x8246E350;
    'dispatch: loop {
        match pc {
            0x8246E350 => {
    //   block [0x8246E350..0x8246E378)
	// 8246E350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E354: 480C6D5D  bl 0x825350b0
	ctx.lr = 0x8246E358;
	sub_82535080(ctx, base);
	// 8246E358: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246E364: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E368: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246E36C: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 8246E370: 409A0008  bne cr6, 0x8246e378
	if !ctx.cr[6].eq {
	pc = 0x8246E378; continue 'dispatch;
	}
	// 8246E374: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	pc = 0x8246E378; continue 'dispatch;
            }
            0x8246E378 => {
    //   block [0x8246E378..0x8246E3CC)
	// 8246E378: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E37C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8246E380: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246E384: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s32 = ((ctx.r[26].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 8246E388: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E38C: 4BFF5CAD  bl 0x82464038
	ctx.lr = 0x8246E390;
	sub_82464038(ctx, base);
	// 8246E390: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E394: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E398: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246E39C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8246E3A0: 480C67B1  bl 0x82534b50
	ctx.lr = 0x8246E3A4;
	sub_82534B50(ctx, base);
	// 8246E3A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E3A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E3AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E3B0: 409A001C  bne cr6, 0x8246e3cc
	if !ctx.cr[6].eq {
	pc = 0x8246E3CC; continue 'dispatch;
	}
	// 8246E3B4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E3B8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E3BC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E3C0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E3C4: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246E3C8: 4BFF5CF1  bl 0x824640b8
	ctx.lr = 0x8246E3CC;
	sub_824640B8(ctx, base);
	pc = 0x8246E3CC; continue 'dispatch;
            }
            0x8246E3CC => {
    //   block [0x8246E3CC..0x8246E3E8)
	// 8246E3CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E3D0: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8246E3D4: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E3D8: 7D6BD378  or r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[26].u64;
	// 8246E3DC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246E3E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246E3E4: 480C6D1C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E3E8 size=176
    let mut pc: u32 = 0x8246E3E8;
    'dispatch: loop {
        match pc {
            0x8246E3E8 => {
    //   block [0x8246E3E8..0x8246E424)
	// 8246E3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E3EC: 480C6CC9  bl 0x825350b4
	ctx.lr = 0x8246E3F0;
	sub_82535080(ctx, base);
	// 8246E3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E3F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E3F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246E3FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8246E400: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8246E404: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246E408: 419A001C  beq cr6, 0x8246e424
	if ctx.cr[6].eq {
	pc = 0x8246E424; continue 'dispatch;
	}
	// 8246E40C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E410: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8246E414: 40980010  bge cr6, 0x8246e424
	if !ctx.cr[6].lt {
	pc = 0x8246E424; continue 'dispatch;
	}
	// 8246E418: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246E41C: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 8246E420: 48000020  b 0x8246e440
	pc = 0x8246E440; continue 'dispatch;
            }
            0x8246E424 => {
    //   block [0x8246E424..0x8246E440)
	// 8246E424: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E428: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246E42C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246E430: 7C9DE1D6  mullw r4, r29, r28
	ctx.r[4].s32 = ((ctx.r[29].s32 as i64 * ctx.r[28].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 8246E434: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E438: 4BFF5C01  bl 0x82464038
	ctx.lr = 0x8246E43C;
	sub_82464038(ctx, base);
	// 8246E43C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	pc = 0x8246E440; continue 'dispatch;
            }
            0x8246E440 => {
    //   block [0x8246E440..0x8246E498)
	// 8246E440: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246E448: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E44C: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * ctx.r[29].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246E450: 4BFFBED9  bl 0x8246a328
	ctx.lr = 0x8246E454;
	sub_8246A328(ctx, base);
	// 8246E454: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E458: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E45C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E460: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E464: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E468: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E46C: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * ctx.r[29].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 8246E470: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E474: 4BFF5C45  bl 0x824640b8
	ctx.lr = 0x8246E478;
	sub_824640B8(ctx, base);
	// 8246E478: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E47C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8246E480: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E484: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 8246E488: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 8246E48C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246E490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E494: 480C6C70  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246E498 size=208
    let mut pc: u32 = 0x8246E498;
    'dispatch: loop {
        match pc {
            0x8246E498 => {
    //   block [0x8246E498..0x8246E4C8)
	// 8246E498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E49C: 480C6C1D  bl 0x825350b8
	ctx.lr = 0x8246E4A0;
	sub_82535080(ctx, base);
	// 8246E4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E4A8: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8246E4AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E4B0: 40980018  bge cr6, 0x8246e4c8
	if !ctx.cr[6].lt {
	pc = 0x8246E4C8; continue 'dispatch;
	}
	// 8246E4B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246E4B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E4BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E4C4: 480C6C44  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E4C8 => {
    //   block [0x8246E4C8..0x8246E4FC)
	// 8246E4C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E4CC: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8246E4D0: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E4D4: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246E4D8: 40990024  ble cr6, 0x8246e4fc
	if !ctx.cr[6].gt {
	pc = 0x8246E4FC; continue 'dispatch;
	}
	// 8246E4DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246E4E0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8246E4E4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E4E8: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8246E4EC: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8246E4F0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E4F8: 480C6C10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E4FC => {
    //   block [0x8246E4FC..0x8246E524)
	// 8246E4FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E500: 40990060  ble cr6, 0x8246e560
	if !ctx.cr[6].gt {
	pc = 0x8246E560; continue 'dispatch;
	}
	// 8246E504: 7FCB4E70  srawi r11, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 8246E508: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8246E50C: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E510: 7F8BF050  subf r28, r11, r30
	ctx.r[28].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8246E514: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8246E518: 23BC0200  subfic r29, r28, 0x200
	ctx.xer.ca = ctx.r[28].u32 <= 512 as u32;
	ctx.r[29].s64 = (512 as i64) - ctx.r[28].s64;
	// 8246E51C: 409A0008  bne cr6, 0x8246e524
	if !ctx.cr[6].eq {
	pc = 0x8246E524; continue 'dispatch;
	}
	// 8246E520: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x8246E524; continue 'dispatch;
            }
            0x8246E524 => {
    //   block [0x8246E524..0x8246E560)
	// 8246E524: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E528: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E52C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E530: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8246E534: 4BFFBDFD  bl 0x8246a330
	ctx.lr = 0x8246E538;
	sub_8246A330(ctx, base);
	// 8246E538: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 8246E53C: 7FCA4E70  srawi r10, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 8246E540: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 8246E544: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246E548: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8246E54C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246E550: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E554: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E558: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E55C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	pc = 0x8246E560; continue 'dispatch;
            }
            0x8246E560 => {
    //   block [0x8246E560..0x8246E568)
	// 8246E560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E564: 480C6BA4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E568 size=192
    let mut pc: u32 = 0x8246E568;
    'dispatch: loop {
        match pc {
            0x8246E568 => {
    //   block [0x8246E568..0x8246E5A8)
	// 8246E568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E56C: 480C6B51  bl 0x825350bc
	ctx.lr = 0x8246E570;
	sub_82535080(ctx, base);
	// 8246E570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E578: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246E57C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E580: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E584: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E58C: 4E800421  bctrl
	ctx.lr = 0x8246E590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E590: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E598: 409A0010  bne cr6, 0x8246e5a8
	if !ctx.cr[6].eq {
	pc = 0x8246E5A8; continue 'dispatch;
	}
	// 8246E59C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246E5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E5A4: 480C6B68  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E5A8 => {
    //   block [0x8246E5A8..0x8246E5C8)
	// 8246E5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E5AC: 4BFFFEED  bl 0x8246e498
	ctx.lr = 0x8246E5B0;
	sub_8246E498(ctx, base);
	// 8246E5B0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246E5B4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E5B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246E5BC: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E5C0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246E5C4: 40990048  ble cr6, 0x8246e60c
	if !ctx.cr[6].gt {
	pc = 0x8246E60C; continue 'dispatch;
	}
	pc = 0x8246E5C8; continue 'dispatch;
            }
            0x8246E5C8 => {
    //   block [0x8246E5C8..0x8246E60C)
	// 8246E5C8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E5CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E5D0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E5D4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E5D8: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E5DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E5E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E5E8: 4E800421  bctrl
	ctx.lr = 0x8246E5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E5EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E5F0: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 8246E5F4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246E5F8: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E5FC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E600: 409A0018  bne cr6, 0x8246e618
	if !ctx.cr[6].eq {
	pc = 0x8246E618; continue 'dispatch;
	}
	// 8246E604: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E608: 4198FFC0  blt cr6, 0x8246e5c8
	if ctx.cr[6].lt {
	pc = 0x8246E5C8; continue 'dispatch;
	}
            }
            0x8246E60C => {
    //   block [0x8246E60C..0x8246E618)
	// 8246E60C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246E610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E614: 480C6AF8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E618 => {
    //   block [0x8246E618..0x8246E628)
	// 8246E618: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 8246E61C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246E620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E624: 480C6AE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E628 size=196
    let mut pc: u32 = 0x8246E628;
    'dispatch: loop {
        match pc {
            0x8246E628 => {
    //   block [0x8246E628..0x8246E658)
	// 8246E628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E62C: 480C6A89  bl 0x825350b4
	ctx.lr = 0x8246E630;
	sub_82535080(ctx, base);
	// 8246E630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E638: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8246E63C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246E640: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8246E644: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E648: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E64C: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246E650: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E654: 4099005C  ble cr6, 0x8246e6b0
	if !ctx.cr[6].gt {
	pc = 0x8246E6B0; continue 'dispatch;
	}
	pc = 0x8246E658; continue 'dispatch;
            }
            0x8246E658 => {
    //   block [0x8246E658..0x8246E6B0)
	// 8246E658: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E65C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E660: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246E664: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E668: 4BFFBCC1  bl 0x8246a328
	ctx.lr = 0x8246E66C;
	sub_8246A328(ctx, base);
	// 8246E66C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E670: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E674: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8246E678: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246E67C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8246E680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E684: 814A0030  lwz r10, 0x30(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8246E688: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E68C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8246E690: 4E800421  bctrl
	ctx.lr = 0x8246E694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E694: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246E698: 409A0048  bne cr6, 0x8246e6e0
	if !ctx.cr[6].eq {
	pc = 0x8246E6E0; continue 'dispatch;
	}
	// 8246E69C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E6A0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E6A4: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246E6A8: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E6AC: 4199FFAC  bgt cr6, 0x8246e658
	if ctx.cr[6].gt {
	pc = 0x8246E658; continue 'dispatch;
	}
            }
            0x8246E6B0 => {
    //   block [0x8246E6B0..0x8246E6E0)
	// 8246E6B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E6B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246E6B8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E6BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246E6C0: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E6C4: 4BFFBC65  bl 0x8246a328
	ctx.lr = 0x8246E6C8;
	sub_8246A328(ctx, base);
	// 8246E6C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E6CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246E6D0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8246E6D4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E6DC: 480C6A28  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E6E0 => {
    //   block [0x8246E6E0..0x8246E6EC)
	// 8246E6E0: 7C7DD850  subf r3, r29, r27
	ctx.r[3].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 8246E6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E6E8: 480C6A1C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E6F0 size=132
    let mut pc: u32 = 0x8246E6F0;
    'dispatch: loop {
        match pc {
            0x8246E6F0 => {
    //   block [0x8246E6F0..0x8246E71C)
	// 8246E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E6F4: 480C69C9  bl 0x825350bc
	ctx.lr = 0x8246E6F8;
	sub_82535080(ctx, base);
	// 8246E6F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E700: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246E704: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8246E708: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E70C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E710: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E714: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E718: 40990038  ble cr6, 0x8246e750
	if !ctx.cr[6].gt {
	pc = 0x8246E750; continue 'dispatch;
	}
	pc = 0x8246E71C; continue 'dispatch;
            }
            0x8246E71C => {
    //   block [0x8246E71C..0x8246E750)
	// 8246E71C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E720: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8246E724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E728: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8246E72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E730: 4E800421  bctrl
	ctx.lr = 0x8246E734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E734: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246E738: 409A0030  bne cr6, 0x8246e768
	if !ctx.cr[6].eq {
	pc = 0x8246E768; continue 'dispatch;
	}
	// 8246E73C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E740: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E744: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E748: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E74C: 4199FFD0  bgt cr6, 0x8246e71c
	if ctx.cr[6].gt {
	pc = 0x8246E71C; continue 'dispatch;
	}
            }
            0x8246E750 => {
    //   block [0x8246E750..0x8246E768)
	// 8246E750: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246E758: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246E75C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E764: 480C69A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246E768 => {
    //   block [0x8246E768..0x8246E774)
	// 8246E768: 7C7EE850  subf r3, r30, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8246E76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E770: 480C699C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E778 size=108
    let mut pc: u32 = 0x8246E778;
    'dispatch: loop {
        match pc {
            0x8246E778 => {
    //   block [0x8246E778..0x8246E7C4)
	// 8246E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E784: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E788: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E790: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E794: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246E798: 409A002C  bne cr6, 0x8246e7c4
	if !ctx.cr[6].eq {
	pc = 0x8246E7C4; continue 'dispatch;
	}
	// 8246E79C: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E7A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246E7A4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E7A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E7AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E7B0: 4E800421  bctrl
	ctx.lr = 0x8246E7B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E7B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E7B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E7BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246E7C0: 419A0008  beq cr6, 0x8246e7c8
	if ctx.cr[6].eq {
	pc = 0x8246E7C8; continue 'dispatch;
	}
            }
            0x8246E7C4 => {
    //   block [0x8246E7C4..0x8246E7C8)
	// 8246E7C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x8246E7C8; continue 'dispatch;
            }
            0x8246E7C8 => {
    //   block [0x8246E7C8..0x8246E7E4)
	// 8246E7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E7CC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246E7D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E7DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E7E8 size=24
    let mut pc: u32 = 0x8246E7E8;
    'dispatch: loop {
        match pc {
            0x8246E7E8 => {
    //   block [0x8246E7E8..0x8246E800)
	// 8246E7E8: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246E7EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246E7F0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246E7F4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246E7F8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246E7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E800 size=36
    let mut pc: u32 = 0x8246E800;
    'dispatch: loop {
        match pc {
            0x8246E800 => {
    //   block [0x8246E800..0x8246E824)
	// 8246E800: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E804: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246E808: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 8246E80C: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246E810: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8246E814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246E818: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8246E81C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246E820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E828 size=28
    let mut pc: u32 = 0x8246E828;
    'dispatch: loop {
        match pc {
            0x8246E828 => {
    //   block [0x8246E828..0x8246E844)
	// 8246E828: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246E82C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8246E830: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E834: 41980010  blt cr6, 0x8246e844
	if ctx.cr[6].lt {
		sub_8246E844(ctx, base);
		return;
	}
	// 8246E838: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246E83C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8246E840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E844(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E844 size=8
    let mut pc: u32 = 0x8246E844;
    'dispatch: loop {
        match pc {
            0x8246E844 => {
    //   block [0x8246E844..0x8246E84C)
	// 8246E844: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246E848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E850 size=64
    let mut pc: u32 = 0x8246E850;
    'dispatch: loop {
        match pc {
            0x8246E850 => {
    //   block [0x8246E850..0x8246E890)
	// 8246E850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E858: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E85C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E860: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E868: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E86C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8246E870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E874: 4E800421  bctrl
	ctx.lr = 0x8246E878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E87C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246E880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E890 size=48
    let mut pc: u32 = 0x8246E890;
    'dispatch: loop {
        match pc {
            0x8246E890 => {
    //   block [0x8246E890..0x8246E8C0)
	// 8246E890: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246E894: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8246E898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246E89C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E8A0: 912B001C  stw r9, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 8246E8A4: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8246E8A8: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8246E8AC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8246E8B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E8B4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8246E8B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E8BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E8C0 size=108
    let mut pc: u32 = 0x8246E8C0;
    'dispatch: loop {
        match pc {
            0x8246E8C0 => {
    //   block [0x8246E8C0..0x8246E914)
	// 8246E8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E8C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E8CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E8D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E8D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E8D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E8DC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8246E8E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E8E4: 4E800421  bctrl
	ctx.lr = 0x8246E8E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E8E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246E8EC: 41980028  blt cr6, 0x8246e914
	if ctx.cr[6].lt {
	pc = 0x8246E914; continue 'dispatch;
	}
	// 8246E8F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E8F4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E8F8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E8FC: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E90C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E910: 4E800020  blr
	return;
            }
            0x8246E914 => {
    //   block [0x8246E914..0x8246E92C)
	// 8246E914: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246E918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246E91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E930 size=128
    let mut pc: u32 = 0x8246E930;
    'dispatch: loop {
        match pc {
            0x8246E930 => {
    //   block [0x8246E930..0x8246E9B0)
	// 8246E930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246E93C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E944: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E948: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246E94C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246E954: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246E958: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E95C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E960: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8246E964: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E968: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E970: 4E800421  bctrl
	ctx.lr = 0x8246E974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E974: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246E978: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8246E97C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246E980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E984: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8246E988: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246E98C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246E990: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E994: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E998: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E9A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246E9A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E9D0 size=112
    let mut pc: u32 = 0x8246E9D0;
    'dispatch: loop {
        match pc {
            0x8246E9D0 => {
    //   block [0x8246E9D0..0x8246EA2C)
	// 8246E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246E9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E9E8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8246E9EC: 394A873C  addi r10, r10, -0x78c4
	ctx.r[10].s64 = ctx.r[10].s64 + -30916;
	// 8246E9F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246E9F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246E9F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8246E9FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246EA00: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246EA04: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246EA08: 4BFFFF29  bl 0x8246e930
	ctx.lr = 0x8246EA0C;
	sub_8246E930(ctx, base);
	// 8246EA0C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EA14: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EA18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246EA1C: 419A0010  beq cr6, 0x8246ea2c
	if ctx.cr[6].eq {
	pc = 0x8246EA2C; continue 'dispatch;
	}
	// 8246EA20: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246EA24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246EA28: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x8246EA2C; continue 'dispatch;
            }
            0x8246EA2C => {
    //   block [0x8246EA2C..0x8246EA40)
	// 8246EA2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EA40 size=156
    let mut pc: u32 = 0x8246EA40;
    'dispatch: loop {
        match pc {
            0x8246EA40 => {
    //   block [0x8246EA40..0x8246EA9C)
	// 8246EA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EA48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EA4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EA54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246EA58: 396B873C  addi r11, r11, -0x78c4
	ctx.r[11].s64 = ctx.r[11].s64 + -30916;
	// 8246EA5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EA60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EA64: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EA68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EA6C: 419A0030  beq cr6, 0x8246ea9c
	if ctx.cr[6].eq {
	pc = 0x8246EA9C; continue 'dispatch;
	}
	// 8246EA70: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246EA74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246EA78: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246EA7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246EA80: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246EA84: 409A0018  bne cr6, 0x8246ea9c
	if !ctx.cr[6].eq {
	pc = 0x8246EA9C; continue 'dispatch;
	}
	// 8246EA88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EA8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246EA90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EA94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EA98: 4E800421  bctrl
	ctx.lr = 0x8246EA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246EA9C => {
    //   block [0x8246EA9C..0x8246EADC)
	// 8246EA9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EAA0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246EAA4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EAA8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246EAAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EAB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EAB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EAB8: 4E800421  bctrl
	ctx.lr = 0x8246EABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EABC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246EAC0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246EAC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EAD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EAE0 size=100
    let mut pc: u32 = 0x8246EAE0;
    'dispatch: loop {
        match pc {
            0x8246EAE0 => {
    //   block [0x8246EAE0..0x8246EB28)
	// 8246EAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246EAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EAF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EAF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246EAFC: 4BFFFF45  bl 0x8246ea40
	ctx.lr = 0x8246EB00;
	sub_8246EA40(ctx, base);
	// 8246EB00: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246EB04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EB08: 419A0020  beq cr6, 0x8246eb28
	if ctx.cr[6].eq {
	pc = 0x8246EB28; continue 'dispatch;
	}
	// 8246EB0C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EB10: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246EB14: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246EB18: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EB1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246EB20: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246EB24: 4BFF5595  bl 0x824640b8
	ctx.lr = 0x8246EB28;
	sub_824640B8(ctx, base);
	pc = 0x8246EB28; continue 'dispatch;
            }
            0x8246EB28 => {
    //   block [0x8246EB28..0x8246EB44)
	// 8246EB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EB2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EB38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246EB3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EB48 size=72
    let mut pc: u32 = 0x8246EB48;
    'dispatch: loop {
        match pc {
            0x8246EB48 => {
    //   block [0x8246EB48..0x8246EB7C)
	// 8246EB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EB50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EB5C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246EB60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246EB64: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EB68: 480C9989  bl 0x825384f0
	ctx.lr = 0x8246EB6C;
	sub_825384F0(ctx, base);
	// 8246EB6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246EB70: 4199000C  bgt cr6, 0x8246eb7c
	if ctx.cr[6].gt {
	pc = 0x8246EB7C; continue 'dispatch;
	}
	// 8246EB74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246EB78: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	pc = 0x8246EB7C; continue 'dispatch;
            }
            0x8246EB7C => {
    //   block [0x8246EB7C..0x8246EB90)
	// 8246EB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EB90 size=12
    let mut pc: u32 = 0x8246EB90;
    'dispatch: loop {
        match pc {
            0x8246EB90 => {
    //   block [0x8246EB90..0x8246EB9C)
	// 8246EB90: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EB94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246EB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EBA0 size=12
    let mut pc: u32 = 0x8246EBA0;
    'dispatch: loop {
        match pc {
            0x8246EBA0 => {
    //   block [0x8246EBA0..0x8246EBAC)
	// 8246EBA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246EBA4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246EBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EBB0 size=12
    let mut pc: u32 = 0x8246EBB0;
    'dispatch: loop {
        match pc {
            0x8246EBB0 => {
    //   block [0x8246EBB0..0x8246EBBC)
	// 8246EBB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246EBB4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246EBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EBC0 size=48
    let mut pc: u32 = 0x8246EBC0;
    'dispatch: loop {
        match pc {
            0x8246EBC0 => {
    //   block [0x8246EBC0..0x8246EBF0)
	// 8246EBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EBCC: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EBD0: 480C9A11  bl 0x825385e0
	ctx.lr = 0x8246EBD4;
	sub_825385E0(ctx, base);
	// 8246EBD4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8246EBD8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246EBDC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8246EBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EBF8 size=116
    let mut pc: u32 = 0x8246EBF8;
    'dispatch: loop {
        match pc {
            0x8246EBF8 => {
    //   block [0x8246EBF8..0x8246EC6C)
	// 8246EBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EC00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EC04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EC08: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246EC0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EC10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246EC14: 396B8778  addi r11, r11, -0x7888
	ctx.r[11].s64 = ctx.r[11].s64 + -30856;
	// 8246EC18: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8246EC1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246EC20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246EC24: 388A8770  addi r4, r10, -0x7890
	ctx.r[4].s64 = ctx.r[10].s64 + -30864;
	// 8246EC28: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246EC2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EC30: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8246EC34: 993F0010  stb r9, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 8246EC38: 480C9F81  bl 0x82538bb8
	ctx.lr = 0x8246EC3C;
	sub_82538BB8(ctx, base);
	// 8246EC3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246EC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EC44: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246EC48: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8246EC4C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8246EC50: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8246EC54: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 8246EC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EC70 size=80
    let mut pc: u32 = 0x8246EC70;
    'dispatch: loop {
        match pc {
            0x8246EC70 => {
    //   block [0x8246EC70..0x8246ECA0)
	// 8246EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EC78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EC7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EC80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EC84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246EC88: 396B8778  addi r11, r11, -0x7888
	ctx.r[11].s64 = ctx.r[11].s64 + -30856;
	// 8246EC8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EC90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246EC94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EC98: 419A0008  beq cr6, 0x8246eca0
	if ctx.cr[6].eq {
	pc = 0x8246ECA0; continue 'dispatch;
	}
	// 8246EC9C: 480C9FE5  bl 0x82538c80
	ctx.lr = 0x8246ECA0;
	sub_82538C80(ctx, base);
	pc = 0x8246ECA0; continue 'dispatch;
            }
            0x8246ECA0 => {
    //   block [0x8246ECA0..0x8246ECC0)
	// 8246ECA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246ECA4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246ECA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246ECAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246ECB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ECB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ECB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ECC0 size=72
    let mut pc: u32 = 0x8246ECC0;
    'dispatch: loop {
        match pc {
            0x8246ECC0 => {
    //   block [0x8246ECC0..0x8246ED08)
	// 8246ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246ECC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246ECCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246ECD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246ECD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ECD8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8246ECDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246ECE0: 4E800421  bctrl
	ctx.lr = 0x8246ECE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246ECE4: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8246ECE8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8246ECEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246ECF0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246ECF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246ECF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ECFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ED00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246ED04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246ED08 size=24
    let mut pc: u32 = 0x8246ED08;
    'dispatch: loop {
        match pc {
            0x8246ED08 => {
    //   block [0x8246ED08..0x8246ED20)
	// 8246ED08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ED0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8246ED10: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246ED14: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8246ED18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246ED1C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ED20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ED20 size=136
    let mut pc: u32 = 0x8246ED20;
    'dispatch: loop {
        match pc {
            0x8246ED20 => {
    //   block [0x8246ED20..0x8246ED58)
	// 8246ED20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ED24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246ED28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246ED2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246ED30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246ED34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246ED38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246ED3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246ED40: 396B8778  addi r11, r11, -0x7888
	ctx.r[11].s64 = ctx.r[11].s64 + -30856;
	// 8246ED44: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246ED48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246ED4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246ED50: 419A0008  beq cr6, 0x8246ed58
	if ctx.cr[6].eq {
	pc = 0x8246ED58; continue 'dispatch;
	}
	// 8246ED54: 480C9F2D  bl 0x82538c80
	ctx.lr = 0x8246ED58;
	sub_82538C80(ctx, base);
	pc = 0x8246ED58; continue 'dispatch;
            }
            0x8246ED58 => {
    //   block [0x8246ED58..0x8246ED8C)
	// 8246ED58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246ED5C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246ED60: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246ED64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246ED68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246ED6C: 419A0020  beq cr6, 0x8246ed8c
	if ctx.cr[6].eq {
	pc = 0x8246ED8C; continue 'dispatch;
	}
	// 8246ED70: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ED74: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246ED78: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246ED7C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ED80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246ED84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246ED88: 4BFF5331  bl 0x824640b8
	ctx.lr = 0x8246ED8C;
	sub_824640B8(ctx, base);
	pc = 0x8246ED8C; continue 'dispatch;
            }
            0x8246ED8C => {
    //   block [0x8246ED8C..0x8246EDA8)
	// 8246ED8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246ED90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246ED94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ED98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ED9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246EDA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EDA8 size=140
    let mut pc: u32 = 0x8246EDA8;
    'dispatch: loop {
        match pc {
            0x8246EDA8 => {
    //   block [0x8246EDA8..0x8246EDD4)
	// 8246EDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EDAC: 480C6311  bl 0x825350bc
	ctx.lr = 0x8246EDB0;
	sub_82535080(ctx, base);
	// 8246EDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EDB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246EDB8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EDBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EDC0: 419A0068  beq cr6, 0x8246ee28
	if ctx.cr[6].eq {
	pc = 0x8246EE28; continue 'dispatch;
	}
	// 8246EDC4: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EDC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246EDCC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246EDD0: 40990038  ble cr6, 0x8246ee08
	if !ctx.cr[6].gt {
	pc = 0x8246EE08; continue 'dispatch;
	}
	pc = 0x8246EDD4; continue 'dispatch;
            }
            0x8246EDD4 => {
    //   block [0x8246EDD4..0x8246EE08)
	// 8246EDD4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EDD8: 7CBFE850  subf r5, r31, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 8246EDDC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EDE0: 7C9F5A14  add r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 8246EDE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EDE8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EDEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EDF0: 4E800421  bctrl
	ctx.lr = 0x8246EDF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EDF4: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8246EDF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246EDFC: 419A0020  beq cr6, 0x8246ee1c
	if ctx.cr[6].eq {
	pc = 0x8246EE1C; continue 'dispatch;
	}
	// 8246EE00: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8246EE04: 4198FFD0  blt cr6, 0x8246edd4
	if ctx.cr[6].lt {
	pc = 0x8246EDD4; continue 'dispatch;
	}
            }
            0x8246EE08 => {
    //   block [0x8246EE08..0x8246EE1C)
	// 8246EE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246EE0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EE10: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EE14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EE18: 480C62F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246EE1C => {
    //   block [0x8246EE1C..0x8246EE28)
	// 8246EE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EE20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EE24: 480C62E8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246EE28 => {
    //   block [0x8246EE28..0x8246EE34)
	// 8246EE28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246EE2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EE30: 480C62DC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EE38 size=184
    let mut pc: u32 = 0x8246EE38;
    'dispatch: loop {
        match pc {
            0x8246EE38 => {
    //   block [0x8246EE38..0x8246EE68)
	// 8246EE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EE3C: 480C6275  bl 0x825350b0
	ctx.lr = 0x8246EE40;
	sub_82535080(ctx, base);
	// 8246EE40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EE44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EE48: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8246EE4C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246EE50: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8246EE54: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EE58: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EE5C: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246EE60: 7F1AF000  cmpw cr6, r26, r30
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246EE64: 40990050  ble cr6, 0x8246eeb4
	if !ctx.cr[6].gt {
	pc = 0x8246EEB4; continue 'dispatch;
	}
	pc = 0x8246EE68; continue 'dispatch;
            }
            0x8246EE68 => {
    //   block [0x8246EE68..0x8246EEB4)
	// 8246EE68: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EE6C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246EE70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246EE74: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246EE78: 4BFFB4B1  bl 0x8246a328
	ctx.lr = 0x8246EE7C;
	sub_8246A328(ctx, base);
	// 8246EE7C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EE80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EE84: 7F6BF214  add r27, r11, r30
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246EE88: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8246EE8C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8246EE90: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8246EE94: 4BFFFF15  bl 0x8246eda8
	ctx.lr = 0x8246EE98;
	sub_8246EDA8(ctx, base);
	// 8246EE98: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8246EE9C: 409A0048  bne cr6, 0x8246eee4
	if !ctx.cr[6].eq {
	pc = 0x8246EEE4; continue 'dispatch;
	}
	// 8246EEA0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EEA4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EEA8: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246EEAC: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246EEB0: 4199FFB8  bgt cr6, 0x8246ee68
	if ctx.cr[6].gt {
	pc = 0x8246EE68; continue 'dispatch;
	}
	pc = 0x8246EEB4; continue 'dispatch;
            }
            0x8246EEB4 => {
    //   block [0x8246EEB4..0x8246EEE4)
	// 8246EEB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EEB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246EEBC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EEC0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246EEC4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246EEC8: 4BFFB461  bl 0x8246a328
	ctx.lr = 0x8246EECC;
	sub_8246A328(ctx, base);
	// 8246EECC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EED0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246EED4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8246EED8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EEDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246EEE0: 480C6220  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x8246EEE4 => {
    //   block [0x8246EEE4..0x8246EEF0)
	// 8246EEE4: 7C7DD050  subf r3, r29, r26
	ctx.r[3].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 8246EEE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246EEEC: 480C6214  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EEF0 size=76
    let mut pc: u32 = 0x8246EEF0;
    'dispatch: loop {
        match pc {
            0x8246EEF0 => {
    //   block [0x8246EEF0..0x8246EF28)
	// 8246EEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EEFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EF00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EF04: 4BFFFEA5  bl 0x8246eda8
	ctx.lr = 0x8246EF08;
	sub_8246EDA8(ctx, base);
	// 8246EF08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EF0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EF10: 419A0018  beq cr6, 0x8246ef28
	if ctx.cr[6].eq {
	pc = 0x8246EF28; continue 'dispatch;
	}
	// 8246EF14: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246EF18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EF1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EF24: 4E800421  bctrl
	ctx.lr = 0x8246EF28;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246EF28 => {
    //   block [0x8246EF28..0x8246EF3C)
	// 8246EF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EF34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EF40 size=192
    let mut pc: u32 = 0x8246EF40;
    'dispatch: loop {
        match pc {
            0x8246EF40 => {
    //   block [0x8246EF40..0x8246EF8C)
	// 8246EF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EF44: 480C6179  bl 0x825350bc
	ctx.lr = 0x8246EF48;
	sub_82535080(ctx, base);
	// 8246EF48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EF4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EF50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246EF54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8246EF58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EF5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EF60: 419A002C  beq cr6, 0x8246ef8c
	if ctx.cr[6].eq {
	pc = 0x8246EF8C; continue 'dispatch;
	}
	// 8246EF64: 4BFFFE45  bl 0x8246eda8
	ctx.lr = 0x8246EF68;
	sub_8246EDA8(ctx, base);
	// 8246EF68: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EF6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246EF70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246EF74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EF78: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8246EF7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EF80: 4E800421  bctrl
	ctx.lr = 0x8246EF84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EF84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EF88: 480C6184  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246EF8C => {
    //   block [0x8246EF8C..0x8246EFB0)
	// 8246EF8C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8246EF90: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8246EF94: 41980028  blt cr6, 0x8246efbc
	if ctx.cr[6].lt {
	pc = 0x8246EFBC; continue 'dispatch;
	}
	// 8246EF98: 419A0018  beq cr6, 0x8246efb0
	if ctx.cr[6].eq {
	pc = 0x8246EFB0; continue 'dispatch;
	}
	// 8246EF9C: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 8246EFA0: 40980020  bge cr6, 0x8246efc0
	if !ctx.cr[6].lt {
	pc = 0x8246EFC0; continue 'dispatch;
	}
	// 8246EFA4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EFA8: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8246EFAC: 48000014  b 0x8246efc0
	pc = 0x8246EFC0; continue 'dispatch;
            }
            0x8246EFB0 => {
    //   block [0x8246EFB0..0x8246EFBC)
	// 8246EFB0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EFB4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246EFB8: 48000008  b 0x8246efc0
	pc = 0x8246EFC0; continue 'dispatch;
            }
            0x8246EFBC => {
    //   block [0x8246EFBC..0x8246EFC0)
	// 8246EFBC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246EFC0; continue 'dispatch;
            }
            0x8246EFC0 => {
    //   block [0x8246EFC0..0x8246EFE0)
	// 8246EFC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246EFC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246EFC8: 40980018  bge cr6, 0x8246efe0
	if !ctx.cr[6].lt {
	pc = 0x8246EFE0; continue 'dispatch;
	}
	// 8246EFCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246EFD0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246EFD4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EFD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EFDC: 480C6130  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246EFE0 => {
    //   block [0x8246EFE0..0x8246EFF4)
	// 8246EFE0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EFE4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246EFE8: 4099000C  ble cr6, 0x8246eff4
	if !ctx.cr[6].gt {
	pc = 0x8246EFF4; continue 'dispatch;
	}
	// 8246EFEC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8246EFF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x8246EFF4; continue 'dispatch;
            }
            0x8246EFF4 => {
    //   block [0x8246EFF4..0x8246F000)
	// 8246EFF4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EFFC: 480C6110  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F000 size=120
    let mut pc: u32 = 0x8246F000;
    'dispatch: loop {
        match pc {
            0x8246F000 => {
    //   block [0x8246F000..0x8246F03C)
	// 8246F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F00C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F014: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F01C: 419A003C  beq cr6, 0x8246f058
	if ctx.cr[6].eq {
	pc = 0x8246F058; continue 'dispatch;
	}
	// 8246F020: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246F024: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F028: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8246F02C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F030: 4E800421  bctrl
	ctx.lr = 0x8246F034;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F034: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246F038: 41980028  blt cr6, 0x8246f060
	if ctx.cr[6].lt {
	pc = 0x8246F060; continue 'dispatch;
	}
            }
            0x8246F03C => {
    //   block [0x8246F03C..0x8246F058)
	// 8246F03C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246F040: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246F044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F054: 4E800020  blr
	return;
            }
            0x8246F058 => {
    //   block [0x8246F058..0x8246F060)
	// 8246F058: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246F05C: 4BFFFFE0  b 0x8246f03c
	pc = 0x8246F03C; continue 'dispatch;
            }
            0x8246F060 => {
    //   block [0x8246F060..0x8246F078)
	// 8246F060: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246F064: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F078 size=128
    let mut pc: u32 = 0x8246F078;
    'dispatch: loop {
        match pc {
            0x8246F078 => {
    //   block [0x8246F078..0x8246F0C8)
	// 8246F078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246F08C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F090: 394A87AC  addi r10, r10, -0x7854
	ctx.r[10].s64 = ctx.r[10].s64 + -30804;
	// 8246F094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246F098: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F09C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8246F0A0: 7CCB0774  extsb r11, r6
	ctx.r[11].s64 = ctx.r[6].s8 as i64;
	// 8246F0A4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246F0A8: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 8246F0AC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8246F0B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246F0B4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8246F0B8: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8246F0BC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8246F0C0: 409A0008  bne cr6, 0x8246f0c8
	if !ctx.cr[6].eq {
	pc = 0x8246F0C8; continue 'dispatch;
	}
	// 8246F0C4: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	pc = 0x8246F0C8; continue 'dispatch;
            }
            0x8246F0C8 => {
    //   block [0x8246F0C8..0x8246F0E0)
	// 8246F0C8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8246F0CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246F0D0: 993F0018  stb r9, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 8246F0D4: 419A000C  beq cr6, 0x8246f0e0
	if ctx.cr[6].eq {
	pc = 0x8246F0E0; continue 'dispatch;
	}
	// 8246F0D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246F0DC: 4BFFB25D  bl 0x8246a338
	ctx.lr = 0x8246F0E0;
	sub_8246A338(ctx, base);
	pc = 0x8246F0E0; continue 'dispatch;
            }
            0x8246F0E0 => {
    //   block [0x8246F0E0..0x8246F0F8)
	// 8246F0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F0E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F0F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F0F8 size=120
    let mut pc: u32 = 0x8246F0F8;
    'dispatch: loop {
        match pc {
            0x8246F0F8 => {
    //   block [0x8246F0F8..0x8246F134)
	// 8246F0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F104: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F108: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F10C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246F114: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F118: 419A001C  beq cr6, 0x8246f134
	if ctx.cr[6].eq {
	pc = 0x8246F134; continue 'dispatch;
	}
	// 8246F11C: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246F120: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F124: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F12C: 4E800421  bctrl
	ctx.lr = 0x8246F130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F130: 48000020  b 0x8246f150
	pc = 0x8246F150; continue 'dispatch;
            }
            0x8246F134 => {
    //   block [0x8246F134..0x8246F150)
	// 8246F134: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246F138: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246F13C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246F140: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246F144: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246F148: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246F14C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	pc = 0x8246F150; continue 'dispatch;
            }
            0x8246F150 => {
    //   block [0x8246F150..0x8246F170)
	// 8246F150: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F158: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F15C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F170 size=100
    let mut pc: u32 = 0x8246F170;
    'dispatch: loop {
        match pc {
            0x8246F170 => {
    //   block [0x8246F170..0x8246F1AC)
	// 8246F170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F17C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F180: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246F18C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F190: 419A001C  beq cr6, 0x8246f1ac
	if ctx.cr[6].eq {
	pc = 0x8246F1AC; continue 'dispatch;
	}
	// 8246F194: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246F198: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F19C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246F1A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F1A4: 4E800421  bctrl
	ctx.lr = 0x8246F1A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F1A8: 4800000C  b 0x8246f1b4
	pc = 0x8246F1B4; continue 'dispatch;
            }
            0x8246F1AC => {
    //   block [0x8246F1AC..0x8246F1B4)
	// 8246F1AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246F1B0: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	pc = 0x8246F1B4; continue 'dispatch;
            }
            0x8246F1B4 => {
    //   block [0x8246F1B4..0x8246F1D4)
	// 8246F1B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F1BC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F1CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F1D8 size=180
    let mut pc: u32 = 0x8246F1D8;
    'dispatch: loop {
        match pc {
            0x8246F1D8 => {
    //   block [0x8246F1D8..0x8246F234)
	// 8246F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F1EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F1F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F1F4: 396B87AC  addi r11, r11, -0x7854
	ctx.r[11].s64 = ctx.r[11].s64 + -30804;
	// 8246F1F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F1FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246F200: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8246F204: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F208: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F20C: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 8246F210: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F218: 419A001C  beq cr6, 0x8246f234
	if ctx.cr[6].eq {
	pc = 0x8246F234; continue 'dispatch;
	}
	// 8246F21C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F220: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246F224: 419A0010  beq cr6, 0x8246f234
	if ctx.cr[6].eq {
	pc = 0x8246F234; continue 'dispatch;
	}
	// 8246F228: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246F22C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246F230: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x8246F234; continue 'dispatch;
            }
            0x8246F234 => {
    //   block [0x8246F234..0x8246F28C)
	// 8246F234: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F238: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F23C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246F240: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246F244: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8246F248: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F24C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F250: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F258: 4E800421  bctrl
	ctx.lr = 0x8246F25C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F25C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246F260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246F264: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8246F268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F26C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8246F270: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8246F274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F290 size=208
    let mut pc: u32 = 0x8246F290;
    'dispatch: loop {
        match pc {
            0x8246F290 => {
    //   block [0x8246F290..0x8246F2D0)
	// 8246F290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F29C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F2A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F2A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F2A8: 396B87AC  addi r11, r11, -0x7854
	ctx.r[11].s64 = ctx.r[11].s64 + -30804;
	// 8246F2AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F2B0: 4BFFFAF9  bl 0x8246eda8
	ctx.lr = 0x8246F2B4;
	sub_8246EDA8(ctx, base);
	// 8246F2B4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F2B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F2BC: 419A0014  beq cr6, 0x8246f2d0
	if ctx.cr[6].eq {
	pc = 0x8246F2D0; continue 'dispatch;
	}
	// 8246F2C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F2C4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246F2C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F2CC: 4E800421  bctrl
	ctx.lr = 0x8246F2D0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246F2D0 => {
    //   block [0x8246F2D0..0x8246F314)
	// 8246F2D0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F2D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F2D8: 419A003C  beq cr6, 0x8246f314
	if ctx.cr[6].eq {
	pc = 0x8246F314; continue 'dispatch;
	}
	// 8246F2DC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F2E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F2E4: 419A0030  beq cr6, 0x8246f314
	if ctx.cr[6].eq {
	pc = 0x8246F314; continue 'dispatch;
	}
	// 8246F2E8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246F2EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246F2F0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246F2F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246F2F8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246F2FC: 409A0018  bne cr6, 0x8246f314
	if !ctx.cr[6].eq {
	pc = 0x8246F314; continue 'dispatch;
	}
	// 8246F300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F304: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246F308: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F30C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F310: 4E800421  bctrl
	ctx.lr = 0x8246F314;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246F314 => {
    //   block [0x8246F314..0x8246F340)
	// 8246F314: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246F318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F31C: 419A0024  beq cr6, 0x8246f340
	if ctx.cr[6].eq {
	pc = 0x8246F340; continue 'dispatch;
	}
	// 8246F320: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F324: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F328: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F32C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F330: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F334: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F33C: 4E800421  bctrl
	ctx.lr = 0x8246F340;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246F340 => {
    //   block [0x8246F340..0x8246F360)
	// 8246F340: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F344: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F348: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F34C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F360 size=100
    let mut pc: u32 = 0x8246F360;
    'dispatch: loop {
        match pc {
            0x8246F360 => {
    //   block [0x8246F360..0x8246F3A8)
	// 8246F360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F36C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F378: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246F37C: 4BFFFF15  bl 0x8246f290
	ctx.lr = 0x8246F380;
	sub_8246F290(ctx, base);
	// 8246F380: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246F384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F388: 419A0020  beq cr6, 0x8246f3a8
	if ctx.cr[6].eq {
	pc = 0x8246F3A8; continue 'dispatch;
	}
	// 8246F38C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F390: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F394: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246F398: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F39C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246F3A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F3A4: 4BFF4D15  bl 0x824640b8
	ctx.lr = 0x8246F3A8;
	sub_824640B8(ctx, base);
	pc = 0x8246F3A8; continue 'dispatch;
            }
            0x8246F3A8 => {
    //   block [0x8246F3A8..0x8246F3C4)
	// 8246F3A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F3AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F3B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F3BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F3C8 size=56
    let mut pc: u32 = 0x8246F3C8;
    'dispatch: loop {
        match pc {
            0x8246F3C8 => {
    //   block [0x8246F3C8..0x8246F400)
	// 8246F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F3D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F3D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F3DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F3E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246F3E4: 480C97D5  bl 0x82538bb8
	ctx.lr = 0x8246F3E8;
	sub_82538BB8(ctx, base);
	// 8246F3E8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8246F3EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F400 size=76
    let mut pc: u32 = 0x8246F400;
    'dispatch: loop {
        match pc {
            0x8246F400 => {
    //   block [0x8246F400..0x8246F430)
	// 8246F400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F40C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F414: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F41C: 419A0014  beq cr6, 0x8246f430
	if ctx.cr[6].eq {
	pc = 0x8246F430; continue 'dispatch;
	}
	// 8246F420: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F428: 419A0008  beq cr6, 0x8246f430
	if ctx.cr[6].eq {
	pc = 0x8246F430; continue 'dispatch;
	}
	// 8246F42C: 480C9855  bl 0x82538c80
	ctx.lr = 0x8246F430;
	sub_82538C80(ctx, base);
	pc = 0x8246F430; continue 'dispatch;
            }
            0x8246F430 => {
    //   block [0x8246F430..0x8246F44C)
	// 8246F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246F434: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F450 size=132
    let mut pc: u32 = 0x8246F450;
    'dispatch: loop {
        match pc {
            0x8246F450 => {
    //   block [0x8246F450..0x8246F4A8)
	// 8246F450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F45C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F468: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F46C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F470: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8246F474: 419A0044  beq cr6, 0x8246f4b8
	if ctx.cr[6].eq {
	pc = 0x8246F4B8; continue 'dispatch;
	}
	// 8246F478: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246F47C: 480C9B9D  bl 0x82539018
	ctx.lr = 0x8246F480;
	sub_82539018(ctx, base);
	// 8246F480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246F484: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246F488: 41990028  bgt cr6, 0x8246f4b0
	if ctx.cr[6].gt {
	pc = 0x8246F4B0; continue 'dispatch;
	}
	// 8246F48C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F494: 419A0014  beq cr6, 0x8246f4a8
	if ctx.cr[6].eq {
	pc = 0x8246F4A8; continue 'dispatch;
	}
	// 8246F498: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F49C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F4A0: 419A0008  beq cr6, 0x8246f4a8
	if ctx.cr[6].eq {
	pc = 0x8246F4A8; continue 'dispatch;
	}
	// 8246F4A4: 480C97DD  bl 0x82538c80
	ctx.lr = 0x8246F4A8;
	sub_82538C80(ctx, base);
	pc = 0x8246F4A8; continue 'dispatch;
            }
            0x8246F4A8 => {
    //   block [0x8246F4A8..0x8246F4B0)
	// 8246F4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246F4AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8246F4B0; continue 'dispatch;
            }
            0x8246F4B0 => {
    //   block [0x8246F4B0..0x8246F4B8)
	// 8246F4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246F4B4: 48000008  b 0x8246f4bc
	pc = 0x8246F4BC; continue 'dispatch;
            }
            0x8246F4B8 => {
    //   block [0x8246F4B8..0x8246F4BC)
	// 8246F4B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8246F4BC; continue 'dispatch;
            }
            0x8246F4BC => {
    //   block [0x8246F4BC..0x8246F4D4)
	// 8246F4BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F4C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F4CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F4D8 size=16
    let mut pc: u32 = 0x8246F4D8;
    'dispatch: loop {
        match pc {
            0x8246F4D8 => {
    //   block [0x8246F4D8..0x8246F4E8)
	// 8246F4D8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F4DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F4E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246F4E4: 480C7BE4  b 0x825370c8
	sub_825370C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F4E8 size=4
    let mut pc: u32 = 0x8246F4E8;
    'dispatch: loop {
        match pc {
            0x8246F4E8 => {
    //   block [0x8246F4E8..0x8246F4EC)
	// 8246F4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F4F0 size=24
    let mut pc: u32 = 0x8246F4F0;
    'dispatch: loop {
        match pc {
            0x8246F4F0 => {
    //   block [0x8246F4F0..0x8246F508)
	// 8246F4F0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F4F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246F4F8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246F4FC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246F500: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F508 size=12
    let mut pc: u32 = 0x8246F508;
    'dispatch: loop {
        match pc {
            0x8246F508 => {
    //   block [0x8246F508..0x8246F514)
	// 8246F508: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246F50C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F518 size=12
    let mut pc: u32 = 0x8246F518;
    'dispatch: loop {
        match pc {
            0x8246F518 => {
    //   block [0x8246F518..0x8246F524)
	// 8246F518: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246F51C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F528 size=48
    let mut pc: u32 = 0x8246F528;
    'dispatch: loop {
        match pc {
            0x8246F528 => {
    //   block [0x8246F528..0x8246F558)
	// 8246F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F534: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F538: 480C90A9  bl 0x825385e0
	ctx.lr = 0x8246F53C;
	sub_825385E0(ctx, base);
	// 8246F53C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8246F540: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246F544: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8246F548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F560 size=104
    let mut pc: u32 = 0x8246F560;
    'dispatch: loop {
        match pc {
            0x8246F560 => {
    //   block [0x8246F560..0x8246F5A4)
	// 8246F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F56C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F570: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F578: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F57C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F580: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246F584: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8246F588: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F58C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F590: 993F000C  stb r9, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u8 ) };
	// 8246F594: 409A0010  bne cr6, 0x8246f5a4
	if !ctx.cr[6].eq {
	pc = 0x8246F5A4; continue 'dispatch;
	}
	// 8246F598: 480C7591  bl 0x82536b28
	ctx.lr = 0x8246F59C;
	sub_82536B28(ctx, base);
	// 8246F59C: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 8246F5A0: 4800000C  b 0x8246f5ac
	pc = 0x8246F5AC; continue 'dispatch;
            }
            0x8246F5A4 => {
    //   block [0x8246F5A4..0x8246F5AC)
	// 8246F5A4: 480C7585  bl 0x82536b28
	ctx.lr = 0x8246F5A8;
	sub_82536B28(ctx, base);
	// 8246F5A8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	pc = 0x8246F5AC; continue 'dispatch;
            }
            0x8246F5AC => {
    //   block [0x8246F5AC..0x8246F5C8)
	// 8246F5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F5B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246F5B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F5C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F5C8 size=108
    let mut pc: u32 = 0x8246F5C8;
    'dispatch: loop {
        match pc {
            0x8246F5C8 => {
    //   block [0x8246F5C8..0x8246F61C)
	// 8246F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F5D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F5D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F5D8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F5E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F5E4: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F5E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246F5EC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F5F0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F5F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F5F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F5FC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8246F600: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 8246F604: 419A0018  beq cr6, 0x8246f61c
	if ctx.cr[6].eq {
	pc = 0x8246F61C; continue 'dispatch;
	}
	// 8246F608: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F60C: 388B87FC  addi r4, r11, -0x7804
	ctx.r[4].s64 = ctx.r[11].s64 + -30724;
	// 8246F610: 480C95A9  bl 0x82538bb8
	ctx.lr = 0x8246F614;
	sub_82538BB8(ctx, base);
	// 8246F614: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246F618: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8246F61C; continue 'dispatch;
            }
            0x8246F61C => {
    //   block [0x8246F61C..0x8246F634)
	// 8246F61C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F638 size=100
    let mut pc: u32 = 0x8246F638;
    'dispatch: loop {
        match pc {
            0x8246F638 => {
    //   block [0x8246F638..0x8246F674)
	// 8246F638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F640: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F64C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F650: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F654: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F658: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F65C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F660: 419A0014  beq cr6, 0x8246f674
	if ctx.cr[6].eq {
	pc = 0x8246F674; continue 'dispatch;
	}
	// 8246F664: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F66C: 419A0008  beq cr6, 0x8246f674
	if ctx.cr[6].eq {
	pc = 0x8246F674; continue 'dispatch;
	}
	// 8246F670: 480C9611  bl 0x82538c80
	ctx.lr = 0x8246F674;
	sub_82538C80(ctx, base);
	pc = 0x8246F674; continue 'dispatch;
            }
            0x8246F674 => {
    //   block [0x8246F674..0x8246F69C)
	// 8246F674: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246F67C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F680: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246F684: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F6A0 size=156
    let mut pc: u32 = 0x8246F6A0;
    'dispatch: loop {
        match pc {
            0x8246F6A0 => {
    //   block [0x8246F6A0..0x8246F6E4)
	// 8246F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F6B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F6B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F6BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246F6C0: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F6C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F6C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F6CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F6D0: 419A0014  beq cr6, 0x8246f6e4
	if ctx.cr[6].eq {
	pc = 0x8246F6E4; continue 'dispatch;
	}
	// 8246F6D4: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F6D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F6DC: 419A0008  beq cr6, 0x8246f6e4
	if ctx.cr[6].eq {
	pc = 0x8246F6E4; continue 'dispatch;
	}
	// 8246F6E0: 480C95A1  bl 0x82538c80
	ctx.lr = 0x8246F6E4;
	sub_82538C80(ctx, base);
	pc = 0x8246F6E4; continue 'dispatch;
            }
            0x8246F6E4 => {
    //   block [0x8246F6E4..0x8246F720)
	// 8246F6E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246F6EC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F6F0: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246F6F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246F6F8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246F6FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F700: 419A0020  beq cr6, 0x8246f720
	if ctx.cr[6].eq {
	pc = 0x8246F720; continue 'dispatch;
	}
	// 8246F704: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F708: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F70C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246F710: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F714: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246F718: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F71C: 4BFF499D  bl 0x824640b8
	ctx.lr = 0x8246F720;
	sub_824640B8(ctx, base);
	pc = 0x8246F720; continue 'dispatch;
            }
            0x8246F720 => {
    //   block [0x8246F720..0x8246F73C)
	// 8246F720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F724: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F740 size=20
    let mut pc: u32 = 0x8246F740;
    'dispatch: loop {
        match pc {
            0x8246F740 => {
    //   block [0x8246F740..0x8246F754)
	// 8246F740: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F744: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8246F748: 386B8800  addi r3, r11, -0x7800
	ctx.r[3].s64 = ctx.r[11].s64 + -30720;
	// 8246F74C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8246F750: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F758 size=8
    let mut pc: u32 = 0x8246F758;
    'dispatch: loop {
        match pc {
            0x8246F758 => {
    //   block [0x8246F758..0x8246F760)
	// 8246F758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246F75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F760 size=24
    let mut pc: u32 = 0x8246F760;
    'dispatch: loop {
        match pc {
            0x8246F760 => {
    //   block [0x8246F760..0x8246F778)
	// 8246F760: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F764: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F768: 396B8820  addi r11, r11, -0x77e0
	ctx.r[11].s64 = ctx.r[11].s64 + -30688;
	// 8246F76C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F770: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F778 size=16
    let mut pc: u32 = 0x8246F778;
    'dispatch: loop {
        match pc {
            0x8246F778 => {
    //   block [0x8246F778..0x8246F788)
	// 8246F778: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F77C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F780: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F788 size=108
    let mut pc: u32 = 0x8246F788;
    'dispatch: loop {
        match pc {
            0x8246F788 => {
    //   block [0x8246F788..0x8246F7AC)
	// 8246F788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F78C: 480C592D  bl 0x825350b8
	ctx.lr = 0x8246F790;
	sub_82535080(ctx, base);
	// 8246F790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F794: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246F798: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246F79C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246F7A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246F7A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246F7A8: 40990034  ble cr6, 0x8246f7dc
	if !ctx.cr[6].gt {
	pc = 0x8246F7DC; continue 'dispatch;
	}
	pc = 0x8246F7AC; continue 'dispatch;
            }
            0x8246F7AC => {
    //   block [0x8246F7AC..0x8246F7DC)
	// 8246F7AC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F7B0: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8246F7B4: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8246F7B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F7BC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246F7C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F7C4: 4E800421  bctrl
	ctx.lr = 0x8246F7C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F7C8: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8246F7CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246F7D0: 419A0018  beq cr6, 0x8246f7e8
	if ctx.cr[6].eq {
	pc = 0x8246F7E8; continue 'dispatch;
	}
	// 8246F7D4: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246F7D8: 4198FFD4  blt cr6, 0x8246f7ac
	if ctx.cr[6].lt {
	pc = 0x8246F7AC; continue 'dispatch;
	}
            }
            0x8246F7DC => {
    //   block [0x8246F7DC..0x8246F7E8)
	// 8246F7DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246F7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F7E4: 480C5924  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246F7E8 => {
    //   block [0x8246F7E8..0x8246F7F4)
	// 8246F7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F7F0: 480C5918  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F7F8 size=64
    let mut pc: u32 = 0x8246F7F8;
    'dispatch: loop {
        match pc {
            0x8246F7F8 => {
    //   block [0x8246F7F8..0x8246F838)
	// 8246F7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F800: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F808: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F80C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F810: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F814: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F81C: 4E800421  bctrl
	ctx.lr = 0x8246F820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F838 size=108
    let mut pc: u32 = 0x8246F838;
    'dispatch: loop {
        match pc {
            0x8246F838 => {
    //   block [0x8246F838..0x8246F85C)
	// 8246F838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F83C: 480C587D  bl 0x825350b8
	ctx.lr = 0x8246F840;
	sub_82535080(ctx, base);
	// 8246F840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F844: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246F848: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246F84C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246F850: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246F854: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246F858: 40990034  ble cr6, 0x8246f88c
	if !ctx.cr[6].gt {
	pc = 0x8246F88C; continue 'dispatch;
	}
	pc = 0x8246F85C; continue 'dispatch;
            }
            0x8246F85C => {
    //   block [0x8246F85C..0x8246F88C)
	// 8246F85C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F860: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8246F864: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8246F868: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F86C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246F870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F874: 4E800421  bctrl
	ctx.lr = 0x8246F878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F878: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8246F87C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246F880: 419A0018  beq cr6, 0x8246f898
	if ctx.cr[6].eq {
	pc = 0x8246F898; continue 'dispatch;
	}
	// 8246F884: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246F888: 4198FFD4  blt cr6, 0x8246f85c
	if ctx.cr[6].lt {
	pc = 0x8246F85C; continue 'dispatch;
	}
            }
            0x8246F88C => {
    //   block [0x8246F88C..0x8246F898)
	// 8246F88C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246F890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F894: 480C5874  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246F898 => {
    //   block [0x8246F898..0x8246F8A4)
	// 8246F898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F8A0: 480C5868  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F8A8 size=64
    let mut pc: u32 = 0x8246F8A8;
    'dispatch: loop {
        match pc {
            0x8246F8A8 => {
    //   block [0x8246F8A8..0x8246F8E8)
	// 8246F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F8B8: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F8C0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F8C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F8C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F8CC: 4E800421  bctrl
	ctx.lr = 0x8246F8D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F8D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F8E8 size=68
    let mut pc: u32 = 0x8246F8E8;
    'dispatch: loop {
        match pc {
            0x8246F8E8 => {
    //   block [0x8246F8E8..0x8246F92C)
	// 8246F8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F8F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F8F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F8F8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F8FC: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8246F900: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8246F904: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F908: 4BFF4731  bl 0x82464038
	ctx.lr = 0x8246F90C;
	sub_82464038(ctx, base);
	// 8246F90C: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 8246F910: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8246F914: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8246F918: 480013D1  bl 0x82470ce8
	ctx.lr = 0x8246F91C;
	sub_82470CE8(ctx, base);
	// 8246F91C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F930 size=148
    let mut pc: u32 = 0x8246F930;
    'dispatch: loop {
        match pc {
            0x8246F930 => {
    //   block [0x8246F930..0x8246F9B8)
	// 8246F930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F934: 480C5789  bl 0x825350bc
	ctx.lr = 0x8246F938;
	sub_82535080(ctx, base);
	// 8246F938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F93C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246F944: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 8246F948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F94C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8246F950: 396B888C  addi r11, r11, -0x7774
	ctx.r[11].s64 = ctx.r[11].s64 + -30580;
	// 8246F954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8246F958: 394A8830  addi r10, r10, -0x77d0
	ctx.r[10].s64 = ctx.r[10].s64 + -30672;
	// 8246F95C: 39298864  addi r9, r9, -0x779c
	ctx.r[9].s64 = ctx.r[9].s64 + -30620;
	// 8246F960: 3FA08293  lis r29, -0x7d6d
	ctx.r[29].s64 = -2104295424;
	// 8246F964: B3DF0006  sth r30, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 8246F968: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F96C: B3DF000E  sth r30, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 8246F970: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246F974: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8246F978: B3DF001A  sth r30, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[30].u16 ) };
	// 8246F97C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8246F980: 911F001C  stw r8, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8246F984: 93FF0010  stw r31, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 8246F988: 93FF001C  stw r31, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 8246F98C: 897D91D8  lbz r11, -0x6e28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-28200 as u32) ) } as u64;
	// 8246F990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F994: 409A0024  bne cr6, 0x8246f9b8
	if !ctx.cr[6].eq {
	pc = 0x8246F9B8; continue 'dispatch;
	}
	// 8246F998: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246F99C: 814B4EA4  lwz r10, 0x4ea4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20132 as u32) ) } as u64;
	// 8246F9A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246F9A4: 419A0014  beq cr6, 0x8246f9b8
	if ctx.cr[6].eq {
	pc = 0x8246F9B8; continue 'dispatch;
	}
	// 8246F9A8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246F9AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F9B0: 4E800421  bctrl
	ctx.lr = 0x8246F9B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F9B4: 9BDD91D8  stb r30, -0x6e28(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(-28200 as u32), ctx.r[30].u8 ) };
            }
            0x8246F9B8 => {
    //   block [0x8246F9B8..0x8246F9C4)
	// 8246F9B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F9BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F9C0: 480C574C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F9C8 size=8
    let mut pc: u32 = 0x8246F9C8;
    'dispatch: loop {
        match pc {
            0x8246F9C8 => {
    //   block [0x8246F9C8..0x8246F9D0)
	// 8246F9C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246F9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F9D0 size=96
    let mut pc: u32 = 0x8246F9D0;
    'dispatch: loop {
        match pc {
            0x8246F9D0 => {
    //   block [0x8246F9D0..0x8246FA18)
	// 8246F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F9E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F9E8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F9EC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8246F9F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246F9F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F9F8: 419A0020  beq cr6, 0x8246fa18
	if ctx.cr[6].eq {
	pc = 0x8246FA18; continue 'dispatch;
	}
	// 8246F9FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FA00: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FA04: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246FA08: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246FA0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FA10: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FA14: 4BFF46A5  bl 0x824640b8
	ctx.lr = 0x8246FA18;
	sub_824640B8(ctx, base);
	pc = 0x8246FA18; continue 'dispatch;
            }
            0x8246FA18 => {
    //   block [0x8246FA18..0x8246FA30)
	// 8246FA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FA1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FA28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FA30 size=104
    let mut pc: u32 = 0x8246FA30;
    'dispatch: loop {
        match pc {
            0x8246FA30 => {
    //   block [0x8246FA30..0x8246FA80)
	// 8246FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FA38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FA3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FA40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246FA44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FA48: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246FA4C: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8246FA50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246FA54: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246FA58: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246FA5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FA60: 419A0020  beq cr6, 0x8246fa80
	if ctx.cr[6].eq {
	pc = 0x8246FA80; continue 'dispatch;
	}
	// 8246FA64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FA68: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FA6C: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8246FA70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246FA74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FA78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FA7C: 4BFF463D  bl 0x824640b8
	ctx.lr = 0x8246FA80;
	sub_824640B8(ctx, base);
	pc = 0x8246FA80; continue 'dispatch;
            }
            0x8246FA80 => {
    //   block [0x8246FA80..0x8246FA98)
	// 8246FA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FA84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FA98 size=8
    let mut pc: u32 = 0x8246FA98;
    'dispatch: loop {
        match pc {
            0x8246FA98 => {
    //   block [0x8246FA98..0x8246FAA0)
	// 8246FA98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FAA0 size=8
    let mut pc: u32 = 0x8246FAA0;
    'dispatch: loop {
        match pc {
            0x8246FAA0 => {
    //   block [0x8246FAA0..0x8246FAA8)
	// 8246FAA0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246FAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FAA8 size=12
    let mut pc: u32 = 0x8246FAA8;
    'dispatch: loop {
        match pc {
            0x8246FAA8 => {
    //   block [0x8246FAA8..0x8246FAB4)
	// 8246FAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FAAC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246FAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FAB8 size=108
    let mut pc: u32 = 0x8246FAB8;
    'dispatch: loop {
        match pc {
            0x8246FAB8 => {
    //   block [0x8246FAB8..0x8246FADC)
	// 8246FAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FABC: 480C5601  bl 0x825350bc
	ctx.lr = 0x8246FAC0;
	sub_82535080(ctx, base);
	// 8246FAC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FAC4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FAC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246FACC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246FAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246FAD4: 419A0048  beq cr6, 0x8246fb1c
	if ctx.cr[6].eq {
	pc = 0x8246FB1C; continue 'dispatch;
	}
	// 8246FAD8: 7FA42850  subf r29, r4, r5
	ctx.r[29].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	pc = 0x8246FADC; continue 'dispatch;
            }
            0x8246FADC => {
    //   block [0x8246FADC..0x8246FB0C)
	// 8246FADC: 7CBDF82E  lwzx r5, r29, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8246FAE0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246FAE4: 419A0038  beq cr6, 0x8246fb1c
	if ctx.cr[6].eq {
	pc = 0x8246FB1C; continue 'dispatch;
	}
	// 8246FAE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FAEC: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246FAF0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8246FAF4: 419A0018  beq cr6, 0x8246fb0c
	if ctx.cr[6].eq {
	pc = 0x8246FB0C; continue 'dispatch;
	}
	// 8246FAF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246FB00: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246FB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246FB08: 4E800421  bctrl
	ctx.lr = 0x8246FB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246FB0C => {
    //   block [0x8246FB0C..0x8246FB1C)
	// 8246FB0C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8246FB10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FB14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246FB18: 409AFFC4  bne cr6, 0x8246fadc
	if !ctx.cr[6].eq {
	pc = 0x8246FADC; continue 'dispatch;
	}
	pc = 0x8246FB1C; continue 'dispatch;
            }
            0x8246FB1C => {
    //   block [0x8246FB1C..0x8246FB24)
	// 8246FB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FB20: 480C55EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FB28 size=104
    let mut pc: u32 = 0x8246FB28;
    'dispatch: loop {
        match pc {
            0x8246FB28 => {
    //   block [0x8246FB28..0x8246FB90)
	// 8246FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FB38: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FB3C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FB40: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8246FB44: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8246FB48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FB4C: 4BFF44ED  bl 0x82464038
	ctx.lr = 0x8246FB50;
	sub_82464038(ctx, base);
	// 8246FB50: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246FB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FB58: 396B88BC  addi r11, r11, -0x7744
	ctx.r[11].s64 = ctx.r[11].s64 + -30532;
	// 8246FB5C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8246FB60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246FB64: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FB68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FB6C: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8246FB70: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246FB74: 4BFFD7CD  bl 0x8246d340
	ctx.lr = 0x8246FB78;
	sub_8246D340(ctx, base);
	// 8246FB78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FB90 size=216
    let mut pc: u32 = 0x8246FB90;
    'dispatch: loop {
        match pc {
            0x8246FB90 => {
    //   block [0x8246FB90..0x8246FBB8)
	// 8246FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FB94: 480C5529  bl 0x825350bc
	ctx.lr = 0x8246FB98;
	sub_82535080(ctx, base);
	// 8246FB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FB9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246FBA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246FBA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246FBA8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FBAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246FBB0: 41980024  blt cr6, 0x8246fbd4
	if ctx.cr[6].lt {
	pc = 0x8246FBD4; continue 'dispatch;
	}
	// 8246FBB4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x8246FBB8; continue 'dispatch;
            }
            0x8246FBB8 => {
    //   block [0x8246FBB8..0x8246FBD4)
	// 8246FBB8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FBBC: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 8246FBC0: 409A0014  bne cr6, 0x8246fbd4
	if !ctx.cr[6].eq {
	pc = 0x8246FBD4; continue 'dispatch;
	}
	// 8246FBC4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8246FBC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8246FBCC: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246FBD0: 4099FFE8  ble cr6, 0x8246fbb8
	if !ctx.cr[6].gt {
	pc = 0x8246FBB8; continue 'dispatch;
	}
	pc = 0x8246FBD4; continue 'dispatch;
            }
            0x8246FBD4 => {
    //   block [0x8246FBD4..0x8246FBE8)
	// 8246FBD4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FBD8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FBDC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246FBE0: 40990008  ble cr6, 0x8246fbe8
	if !ctx.cr[6].gt {
	pc = 0x8246FBE8; continue 'dispatch;
	}
	// 8246FBE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8246FBE8; continue 'dispatch;
            }
            0x8246FBE8 => {
    //   block [0x8246FBE8..0x8246FC3C)
	// 8246FBE8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FBEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246FBF0: 419A0070  beq cr6, 0x8246fc60
	if ctx.cr[6].eq {
	pc = 0x8246FC60; continue 'dispatch;
	}
	// 8246FBF4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FBF8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246FBFC: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 8246FC00: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FC04: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FC08: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FC0C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8246FC10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246FC14: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246FC18: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246FC1C: 4BFFD7E5  bl 0x8246d400
	ctx.lr = 0x8246FC20;
	sub_8246D400(ctx, base);
	// 8246FC20: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FC24: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8246FC28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246FC2C: 4199002C  bgt cr6, 0x8246fc58
	if ctx.cr[6].gt {
	pc = 0x8246FC58; continue 'dispatch;
	}
	// 8246FC30: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FC34: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246FC38: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	pc = 0x8246FC3C; continue 'dispatch;
            }
            0x8246FC3C => {
    //   block [0x8246FC3C..0x8246FC58)
	// 8246FC3C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FC40: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 8246FC44: 409A0014  bne cr6, 0x8246fc58
	if !ctx.cr[6].eq {
	pc = 0x8246FC58; continue 'dispatch;
	}
	// 8246FC48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246FC4C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246FC50: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246FC54: 4099FFE8  ble cr6, 0x8246fc3c
	if !ctx.cr[6].gt {
	pc = 0x8246FC3C; continue 'dispatch;
	}
	pc = 0x8246FC58; continue 'dispatch;
            }
            0x8246FC58 => {
    //   block [0x8246FC58..0x8246FC60)
	// 8246FC58: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8246FC5C: 4BFFFF78  b 0x8246fbd4
	pc = 0x8246FBD4; continue 'dispatch;
            }
            0x8246FC60 => {
    //   block [0x8246FC60..0x8246FC68)
	// 8246FC60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FC64: 480C54A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FC68 size=8
    let mut pc: u32 = 0x8246FC68;
    'dispatch: loop {
        match pc {
            0x8246FC68 => {
    //   block [0x8246FC68..0x8246FC70)
	// 8246FC68: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8246FC6C: 4BFFD794  b 0x8246d400
	sub_8246D400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FC70 size=16
    let mut pc: u32 = 0x8246FC70;
    'dispatch: loop {
        match pc {
            0x8246FC70 => {
    //   block [0x8246FC70..0x8246FC80)
	// 8246FC70: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FC74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8246FC78: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8246FC7C: 4BFFD8B4  b 0x8246d530
	sub_8246D530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FC80 size=116
    let mut pc: u32 = 0x8246FC80;
    'dispatch: loop {
        match pc {
            0x8246FC80 => {
    //   block [0x8246FC80..0x8246FCD8)
	// 8246FC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FC88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246FC8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FC90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FC98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246FC9C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FCA0: 4BFFD729  bl 0x8246d3c8
	ctx.lr = 0x8246FCA4;
	sub_8246D3C8(ctx, base);
	// 8246FCA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246FCA8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246FCAC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246FCB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246FCB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FCB8: 419A0020  beq cr6, 0x8246fcd8
	if ctx.cr[6].eq {
	pc = 0x8246FCD8; continue 'dispatch;
	}
	// 8246FCBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FCC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FCC4: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8246FCC8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246FCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FCD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FCD4: 4BFF43E5  bl 0x824640b8
	ctx.lr = 0x8246FCD8;
	sub_824640B8(ctx, base);
	pc = 0x8246FCD8; continue 'dispatch;
            }
            0x8246FCD8 => {
    //   block [0x8246FCD8..0x8246FCF4)
	// 8246FCD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FCDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FCE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246FCEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FCF8 size=144
    let mut pc: u32 = 0x8246FCF8;
    'dispatch: loop {
        match pc {
            0x8246FCF8 => {
    //   block [0x8246FCF8..0x8246FD20)
	// 8246FCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FCFC: 480C53B5  bl 0x825350b0
	ctx.lr = 0x8246FD00;
	sub_82535080(ctx, base);
	// 8246FD00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FD04: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246FD08: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246FD0C: 3BCB4F80  addi r30, r11, 0x4f80
	ctx.r[30].s64 = ctx.r[11].s64 + 20352;
	// 8246FD10: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8246FD14: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8246FD18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246FD1C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	pc = 0x8246FD20; continue 'dispatch;
            }
            0x8246FD20 => {
    //   block [0x8246FD20..0x8246FD64)
	// 8246FD20: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246FD24: 4BFF6B35  bl 0x82466858
	ctx.lr = 0x8246FD28;
	sub_82466858(ctx, base);
	// 8246FD28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8246FD2C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FD30: 4BFFA2A1  bl 0x82469fd0
	ctx.lr = 0x8246FD34;
	sub_82469FD0(ctx, base);
	// 8246FD34: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246FD38: 419A002C  beq cr6, 0x8246fd64
	if ctx.cr[6].eq {
	pc = 0x8246FD64; continue 'dispatch;
	}
	// 8246FD3C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8246FD40: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 8246FD44: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8246FD48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FD4C: 4198FFD4  blt cr6, 0x8246fd20
	if ctx.cr[6].lt {
	pc = 0x8246FD20; continue 'dispatch;
	}
	// 8246FD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FD54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246FD58: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246FD5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246FD60: 480C53A0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x8246FD64 => {
    //   block [0x8246FD64..0x8246FD88)
	// 8246FD64: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246FD68: 393E0004  addi r9, r30, 4
	ctx.r[9].s64 = ctx.r[30].s64 + 4;
	// 8246FD6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FD70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246FD74: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246FD78: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246FD7C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FD80: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246FD84: 480C537C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FD88 size=72
    let mut pc: u32 = 0x8246FD88;
    'dispatch: loop {
        match pc {
            0x8246FD88 => {
    //   block [0x8246FD88..0x8246FDD0)
	// 8246FD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FD90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FD94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FD98: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246FD9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FDA0: 396B88E8  addi r11, r11, -0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + -30488;
	// 8246FDA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FDA8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FDAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FDB0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246FDB4: 4BFFD58D  bl 0x8246d340
	ctx.lr = 0x8246FDB8;
	sub_8246D340(ctx, base);
	// 8246FDB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FDBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FDC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FDD0 size=128
    let mut pc: u32 = 0x8246FDD0;
    'dispatch: loop {
        match pc {
            0x8246FDD0 => {
    //   block [0x8246FDD0..0x8246FDE8)
	// 8246FDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FDD4: 480C52E9  bl 0x825350bc
	ctx.lr = 0x8246FDD8;
	sub_82535080(ctx, base);
	// 8246FDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FDDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246FDE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246FDE4: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	pc = 0x8246FDE8; continue 'dispatch;
            }
            0x8246FDE8 => {
    //   block [0x8246FDE8..0x8246FE44)
	// 8246FDE8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8246FDEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246FDF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246FDF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246FDF8: 4BFFFF01  bl 0x8246fcf8
	ctx.lr = 0x8246FDFC;
	sub_8246FCF8(ctx, base);
	// 8246FDFC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FE00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246FE04: 409A0040  bne cr6, 0x8246fe44
	if !ctx.cr[6].eq {
	pc = 0x8246FE44; continue 'dispatch;
	}
	// 8246FE08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246FE0C: 4BFF6A55  bl 0x82466860
	ctx.lr = 0x8246FE10;
	sub_82466860(ctx, base);
	// 8246FE10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246FE14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8246FE18: 409AFFD0  bne cr6, 0x8246fde8
	if !ctx.cr[6].eq {
	pc = 0x8246FDE8; continue 'dispatch;
	}
	// 8246FE1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FE20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246FE24: 4BFF6EA5  bl 0x82466cc8
	ctx.lr = 0x8246FE28;
	sub_82466CC8(ctx, base);
	// 8246FE28: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FE2C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FE30: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246FE34: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246FE38: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8246FE3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FE40: 480C52CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246FE44 => {
    //   block [0x8246FE44..0x8246FE50)
	// 8246FE44: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8246FE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FE4C: 480C52C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FE50 size=104
    let mut pc: u32 = 0x8246FE50;
    'dispatch: loop {
        match pc {
            0x8246FE50 => {
    //   block [0x8246FE50..0x8246FEB8)
	// 8246FE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FE58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FE5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FE60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FE64: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FE68: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8246FE6C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8246FE70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FE74: 4BFF41C5  bl 0x82464038
	ctx.lr = 0x8246FE78;
	sub_82464038(ctx, base);
	// 8246FE78: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246FE7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FE80: 396B88E8  addi r11, r11, -0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + -30488;
	// 8246FE84: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8246FE88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246FE8C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FE90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FE94: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8246FE98: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246FE9C: 4BFFD4A5  bl 0x8246d340
	ctx.lr = 0x8246FEA0;
	sub_8246D340(ctx, base);
	// 8246FEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FEB8 size=104
    let mut pc: u32 = 0x8246FEB8;
    'dispatch: loop {
        match pc {
            0x8246FEB8 => {
    //   block [0x8246FEB8..0x8246FEF4)
	// 8246FEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246FEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FED0: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 8246FED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246FED8: 4BFFD601  bl 0x8246d4d8
	ctx.lr = 0x8246FEDC;
	sub_8246D4D8(ctx, base);
	// 8246FEDC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FEE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8246FEE4: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FEE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246FEEC: 40990008  ble cr6, 0x8246fef4
	if !ctx.cr[6].gt {
	pc = 0x8246FEF4; continue 'dispatch;
	}
	// 8246FEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8246FEF4; continue 'dispatch;
            }
            0x8246FEF4 => {
    //   block [0x8246FEF4..0x8246FF08)
	// 8246FEF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FEF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246FEFC: 419A000C  beq cr6, 0x8246ff08
	if ctx.cr[6].eq {
	pc = 0x8246FF08; continue 'dispatch;
	}
	// 8246FF00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246FF04: 4BFFD71D  bl 0x8246d620
	ctx.lr = 0x8246FF08;
	sub_8246D620(ctx, base);
	pc = 0x8246FF08; continue 'dispatch;
            }
            0x8246FF08 => {
    //   block [0x8246FF08..0x8246FF20)
	// 8246FF08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FF14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246FF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FF20 size=124
    let mut pc: u32 = 0x8246FF20;
    'dispatch: loop {
        match pc {
            0x8246FF20 => {
    //   block [0x8246FF20..0x8246FF58)
	// 8246FF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FF24: 480C5195  bl 0x825350b8
	ctx.lr = 0x8246FF28;
	sub_82535080(ctx, base);
	// 8246FF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FF2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246FF30: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246FF34: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 8246FF38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8246FF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FF40: 4BFFD599  bl 0x8246d4d8
	ctx.lr = 0x8246FF44;
	sub_8246D4D8(ctx, base);
	// 8246FF44: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FF48: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FF4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246FF50: 40990008  ble cr6, 0x8246ff58
	if !ctx.cr[6].gt {
	pc = 0x8246FF58; continue 'dispatch;
	}
	// 8246FF54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8246FF58; continue 'dispatch;
            }
            0x8246FF58 => {
    //   block [0x8246FF58..0x8246FF84)
	// 8246FF58: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FF5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246FF60: 419A0024  beq cr6, 0x8246ff84
	if ctx.cr[6].eq {
	pc = 0x8246FF84; continue 'dispatch;
	}
	// 8246FF64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FF68: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FF6C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246FF70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246FF74: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246FF78: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8246FF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FF80: 480C5188  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8246FF84 => {
    //   block [0x8246FF84..0x8246FF9C)
	// 8246FF84: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8246FF88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246FF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FF90: 4BFFD471  bl 0x8246d400
	ctx.lr = 0x8246FF94;
	sub_8246D400(ctx, base);
	// 8246FF94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FF98: 480C5170  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FFA0 size=140
    let mut pc: u32 = 0x8246FFA0;
    'dispatch: loop {
        match pc {
            0x8246FFA0 => {
    //   block [0x8246FFA0..0x8246FFE0)
	// 8246FFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FFA4: 480C5115  bl 0x825350b8
	ctx.lr = 0x8246FFA8;
	sub_82535080(ctx, base);
	// 8246FFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FFAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246FFB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246FFB4: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 8246FFB8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246FFBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FFC0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8246FFC4: 4BFFD515  bl 0x8246d4d8
	ctx.lr = 0x8246FFC8;
	sub_8246D4D8(ctx, base);
	// 8246FFC8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FFCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246FFD0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246FFD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FFD8: 40990008  ble cr6, 0x8246ffe0
	if !ctx.cr[6].gt {
	pc = 0x8246FFE0; continue 'dispatch;
	}
	// 8246FFDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x8246FFE0; continue 'dispatch;
            }
            0x8246FFE0 => {
    //   block [0x8246FFE0..0x8247001C)
	// 8246FFE0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246FFE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246FFE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246FFEC: 419A0030  beq cr6, 0x8247001c
	if ctx.cr[6].eq {
	pc = 0x8247001C; continue 'dispatch;
	}
	// 8246FFF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FFF4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FFF8: 995D0000  stb r10, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246FFFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82470000: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82470004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82470008: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247000C: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82470010: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82470018: 480C50F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8247001C => {
    //   block [0x8247001C..0x8247002C)
	// 8247001C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82470020: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82470024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82470028: 480C50E0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470030 size=144
    let mut pc: u32 = 0x82470030;
    'dispatch: loop {
        match pc {
            0x82470030 => {
    //   block [0x82470030..0x8247004C)
	// 82470030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470034: 480C5085  bl 0x825350b8
	ctx.lr = 0x82470038;
	sub_82535080(ctx, base);
	// 82470038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247003C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82470040: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82470044: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 82470048: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	pc = 0x8247004C; continue 'dispatch;
            }
            0x8247004C => {
    //   block [0x8247004C..0x8247006C)
	// 8247004C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82470050: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82470054: 4BFFD485  bl 0x8246d4d8
	ctx.lr = 0x82470058;
	sub_8246D4D8(ctx, base);
	// 82470058: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8247005C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470060: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82470064: 40990008  ble cr6, 0x8247006c
	if !ctx.cr[6].gt {
	pc = 0x8247006C; continue 'dispatch;
	}
	// 82470068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x8247006C; continue 'dispatch;
            }
            0x8247006C => {
    //   block [0x8247006C..0x824700A0)
	// 8247006C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82470070: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82470074: 409A002C  bne cr6, 0x824700a0
	if !ctx.cr[6].eq {
	pc = 0x824700A0; continue 'dispatch;
	}
	// 82470078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247007C: 4BFF67E5  bl 0x82466860
	ctx.lr = 0x82470080;
	sub_82466860(ctx, base);
	// 82470080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470084: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82470088: 409AFFC4  bne cr6, 0x8247004c
	if !ctx.cr[6].eq {
	pc = 0x8247004C; continue 'dispatch;
	}
	// 8247008C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82470090: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470094: 4BFFFD3D  bl 0x8246fdd0
	ctx.lr = 0x82470098;
	sub_8246FDD0(ctx, base);
	// 82470098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8247009C: 480C506C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x824700A0 => {
    //   block [0x824700A0..0x824700C0)
	// 824700A0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824700A4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824700A8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824700AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824700B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824700B4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824700B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824700BC: 480C504C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824700C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824700C0 size=116
    let mut pc: u32 = 0x824700C0;
    'dispatch: loop {
        match pc {
            0x824700C0 => {
    //   block [0x824700C0..0x82470118)
	// 824700C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824700C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824700C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824700CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824700D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824700D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824700D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824700DC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824700E0: 4BFFD2E9  bl 0x8246d3c8
	ctx.lr = 0x824700E4;
	sub_8246D3C8(ctx, base);
	// 824700E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824700E8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824700EC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824700F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824700F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824700F8: 419A0020  beq cr6, 0x82470118
	if ctx.cr[6].eq {
	pc = 0x82470118; continue 'dispatch;
	}
	// 824700FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470100: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82470104: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82470108: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247010C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82470110: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82470114: 4BFF3FA5  bl 0x824640b8
	ctx.lr = 0x82470118;
	sub_824640B8(ctx, base);
	pc = 0x82470118; continue 'dispatch;
            }
            0x82470118 => {
    //   block [0x82470118..0x82470134)
	// 82470118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247011C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8247012C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470150 size=96
    let mut pc: u32 = 0x82470150;
    'dispatch: loop {
        match pc {
            0x82470150 => {
    //   block [0x82470150..0x82470198)
	// 82470150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247015C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247016C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470170: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82470174: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82470178: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247017C: 4BFF69AD  bl 0x82466b28
	ctx.lr = 0x82470180;
	sub_82466B28(ctx, base);
	// 82470180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470184: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82470188: 419A0010  beq cr6, 0x82470198
	if ctx.cr[6].eq {
	pc = 0x82470198; continue 'dispatch;
	}
	// 8247018C: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82470190: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470194: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82470198; continue 'dispatch;
            }
            0x82470198 => {
    //   block [0x82470198..0x824701B0)
	// 82470198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8247019C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824701A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824701A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824701A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824701AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824701B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824701B0 size=20
    let mut pc: u32 = 0x824701B0;
    'dispatch: loop {
        match pc {
            0x824701B0 => {
    //   block [0x824701B0..0x824701C4)
	// 824701B0: A1650012  lhz r11, 0x12(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(18 as u32) ) } as u64;
	// 824701B4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824701B8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824701BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824701C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824701C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824701C8 size=104
    let mut pc: u32 = 0x824701C8;
    'dispatch: loop {
        match pc {
            0x824701C8 => {
    //   block [0x824701C8..0x82470214)
	// 824701C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824701CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824701D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824701D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824701D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824701DC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824701E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824701E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824701E8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824701EC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824701F0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824701F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824701F8: 4BFF6931  bl 0x82466b28
	ctx.lr = 0x824701FC;
	sub_82466B28(ctx, base);
	// 824701FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470200: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82470204: 419A0010  beq cr6, 0x82470214
	if ctx.cr[6].eq {
	pc = 0x82470214; continue 'dispatch;
	}
	// 82470208: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 8247020C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470210: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82470214; continue 'dispatch;
            }
            0x82470214 => {
    //   block [0x82470214..0x82470230)
	// 82470214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8247021C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470224: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82470228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8247022C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470230 size=100
    let mut pc: u32 = 0x82470230;
    'dispatch: loop {
        match pc {
            0x82470230 => {
    //   block [0x82470230..0x82470278)
	// 82470230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247023C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247024C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470250: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82470254: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82470258: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247025C: 4BFF68CD  bl 0x82466b28
	ctx.lr = 0x82470260;
	sub_82466B28(ctx, base);
	// 82470260: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470264: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82470268: 419A0010  beq cr6, 0x82470278
	if ctx.cr[6].eq {
	pc = 0x82470278; continue 'dispatch;
	}
	// 8247026C: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82470270: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470274: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82470278; continue 'dispatch;
            }
            0x82470278 => {
    //   block [0x82470278..0x82470294)
	// 82470278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247027C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8247028C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470298 size=24
    let mut pc: u32 = 0x82470298;
    'dispatch: loop {
        match pc {
            0x82470298 => {
    //   block [0x82470298..0x824702B0)
	// 82470298: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247029C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824702A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824702A4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824702A8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824702AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702B0 size=8
    let mut pc: u32 = 0x824702B0;
    'dispatch: loop {
        match pc {
            0x824702B0 => {
    //   block [0x824702B0..0x824702B8)
	// 824702B0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702B8 size=8
    let mut pc: u32 = 0x824702B8;
    'dispatch: loop {
        match pc {
            0x824702B8 => {
    //   block [0x824702B8..0x824702C0)
	// 824702B8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702C0 size=16
    let mut pc: u32 = 0x824702C0;
    'dispatch: loop {
        match pc {
            0x824702C0 => {
    //   block [0x824702C0..0x824702D0)
	// 824702C0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702C4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824702C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824702CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702D0 size=16
    let mut pc: u32 = 0x824702D0;
    'dispatch: loop {
        match pc {
            0x824702D0 => {
    //   block [0x824702D0..0x824702E0)
	// 824702D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702D4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824702D8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824702DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702E0 size=12
    let mut pc: u32 = 0x824702E0;
    'dispatch: loop {
        match pc {
            0x824702E0 => {
    //   block [0x824702E0..0x824702EC)
	// 824702E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702E4: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824702E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702F0 size=16
    let mut pc: u32 = 0x824702F0;
    'dispatch: loop {
        match pc {
            0x824702F0 => {
    //   block [0x824702F0..0x82470300)
	// 824702F0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702F4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824702F8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824702FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470300 size=16
    let mut pc: u32 = 0x82470300;
    'dispatch: loop {
        match pc {
            0x82470300 => {
    //   block [0x82470300..0x82470310)
	// 82470300: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470304: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470308: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247030C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470310 size=16
    let mut pc: u32 = 0x82470310;
    'dispatch: loop {
        match pc {
            0x82470310 => {
    //   block [0x82470310..0x82470320)
	// 82470310: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470314: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470318: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470320 size=16
    let mut pc: u32 = 0x82470320;
    'dispatch: loop {
        match pc {
            0x82470320 => {
    //   block [0x82470320..0x82470330)
	// 82470320: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470324: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470328: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247032C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470330 size=16
    let mut pc: u32 = 0x82470330;
    'dispatch: loop {
        match pc {
            0x82470330 => {
    //   block [0x82470330..0x82470340)
	// 82470330: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470334: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470338: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247033C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470340 size=16
    let mut pc: u32 = 0x82470340;
    'dispatch: loop {
        match pc {
            0x82470340 => {
    //   block [0x82470340..0x82470350)
	// 82470340: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470344: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470348: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247034C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470350 size=12
    let mut pc: u32 = 0x82470350;
    'dispatch: loop {
        match pc {
            0x82470350 => {
    //   block [0x82470350..0x8247035C)
	// 82470350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470354: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82470358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470360 size=12
    let mut pc: u32 = 0x82470360;
    'dispatch: loop {
        match pc {
            0x82470360 => {
    //   block [0x82470360..0x8247036C)
	// 82470360: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470364: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82470368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470370 size=16
    let mut pc: u32 = 0x82470370;
    'dispatch: loop {
        match pc {
            0x82470370 => {
    //   block [0x82470370..0x82470380)
	// 82470370: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470374: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470378: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247037C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470380 size=24
    let mut pc: u32 = 0x82470380;
    'dispatch: loop {
        match pc {
            0x82470380 => {
    //   block [0x82470380..0x82470398)
	// 82470380: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470384: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470388: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8247038C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470390: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82470394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470398 size=16
    let mut pc: u32 = 0x82470398;
    'dispatch: loop {
        match pc {
            0x82470398 => {
    //   block [0x82470398..0x824703A8)
	// 82470398: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247039C: 548B3032  slwi r11, r4, 6
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703A0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703A8 size=24
    let mut pc: u32 = 0x824703A8;
    'dispatch: loop {
        match pc {
            0x824703A8 => {
    //   block [0x824703A8..0x824703C0)
	// 824703A8: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703AC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703B0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824703B4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703B8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703C0 size=16
    let mut pc: u32 = 0x824703C0;
    'dispatch: loop {
        match pc {
            0x824703C0 => {
    //   block [0x824703C0..0x824703D0)
	// 824703C0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703C4: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703D0 size=24
    let mut pc: u32 = 0x824703D0;
    'dispatch: loop {
        match pc {
            0x824703D0 => {
    //   block [0x824703D0..0x824703E8)
	// 824703D0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703D8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824703DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703E0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703E8 size=16
    let mut pc: u32 = 0x824703E8;
    'dispatch: loop {
        match pc {
            0x824703E8 => {
    //   block [0x824703E8..0x824703F8)
	// 824703E8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703EC: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703F0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703F8 size=8
    let mut pc: u32 = 0x824703F8;
    'dispatch: loop {
        match pc {
            0x824703F8 => {
    //   block [0x824703F8..0x82470400)
	// 824703F8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824703FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470400 size=88
    let mut pc: u32 = 0x82470400;
    'dispatch: loop {
        match pc {
            0x82470400 => {
    //   block [0x82470400..0x8247044C)
	// 82470400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470404: 480C4CB9  bl 0x825350bc
	ctx.lr = 0x82470408;
	sub_82535080(ctx, base);
	// 82470408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247040C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470414: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82470418: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247041C: 4BFFADBD  bl 0x8246b1d8
	ctx.lr = 0x82470420;
	sub_8246B1D8(ctx, base);
	// 82470420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82470424: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470428: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8247042C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470430: 4BFF66F9  bl 0x82466b28
	ctx.lr = 0x82470434;
	sub_82466B28(ctx, base);
	// 82470434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470438: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8247043C: 419A0010  beq cr6, 0x8247044c
	if ctx.cr[6].eq {
	pc = 0x8247044C; continue 'dispatch;
	}
	// 82470440: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82470444: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470448: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8247044C; continue 'dispatch;
            }
            0x8247044C => {
    //   block [0x8247044C..0x82470458)
	// 8247044C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470454: 480C4CB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470458 size=12
    let mut pc: u32 = 0x82470458;
    'dispatch: loop {
        match pc {
            0x82470458 => {
    //   block [0x82470458..0x82470464)
	// 82470458: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8247045C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82470460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470468 size=20
    let mut pc: u32 = 0x82470468;
    'dispatch: loop {
        match pc {
            0x82470468 => {
    //   block [0x82470468..0x8247047C)
	// 82470468: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247046C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470470: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470474: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82470478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470480 size=80
    let mut pc: u32 = 0x82470480;
    'dispatch: loop {
        match pc {
            0x82470480 => {
    //   block [0x82470480..0x824704D0)
	// 82470480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247048C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470494: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470498: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247049C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824704A0: 4BFFAD39  bl 0x8246b1d8
	ctx.lr = 0x824704A4;
	sub_8246B1D8(ctx, base);
	// 824704A4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824704A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824704AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824704B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824704B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824704B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824704BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824704C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824704C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824704C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824704CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824704D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824704D0 size=120
    let mut pc: u32 = 0x824704D0;
    'dispatch: loop {
        match pc {
            0x824704D0 => {
    //   block [0x824704D0..0x82470514)
	// 824704D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824704D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824704D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824704DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824704E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824704E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824704E8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824704EC: 4BFF9E95  bl 0x8246a380
	ctx.lr = 0x824704F0;
	sub_8246A380(ctx, base);
	// 824704F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824704F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824704F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824704FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82470500: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470504: 409A0010  bne cr6, 0x82470514
	if !ctx.cr[6].eq {
	pc = 0x82470514; continue 'dispatch;
	}
	// 82470508: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8247050C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470510: 4BFFDE41  bl 0x8246e350
	ctx.lr = 0x82470514;
	sub_8246E350(ctx, base);
	pc = 0x82470514; continue 'dispatch;
            }
            0x82470514 => {
    //   block [0x82470514..0x82470548)
	// 82470514: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470518: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247051C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470520: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82470524: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470528: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8247052C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82470530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247053C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82470540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470548 size=124
    let mut pc: u32 = 0x82470548;
    'dispatch: loop {
        match pc {
            0x82470548 => {
    //   block [0x82470548..0x82470590)
	// 82470548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247054C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82470554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247055C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470560: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82470564: 4BFF9E1D  bl 0x8246a380
	ctx.lr = 0x82470568;
	sub_8246A380(ctx, base);
	// 82470568: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8247056C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82470570: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82470574: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470578: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247057C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470580: 409A0010  bne cr6, 0x82470590
	if !ctx.cr[6].eq {
	pc = 0x82470590; continue 'dispatch;
	}
	// 82470584: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82470588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247058C: 4BFFDDC5  bl 0x8246e350
	ctx.lr = 0x82470590;
	sub_8246E350(ctx, base);
	pc = 0x82470590; continue 'dispatch;
            }
            0x82470590 => {
    //   block [0x82470590..0x824705C4)
	// 82470590: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470594: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470598: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247059C: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824705A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824705A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824705A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824705AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824705B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824705B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824705B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824705BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824705C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824705C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824705C8 size=488
    let mut pc: u32 = 0x824705C8;
    'dispatch: loop {
        match pc {
            0x824705C8 => {
    //   block [0x824705C8..0x82470618)
	// 824705C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824705CC: 480C4AE9  bl 0x825350b4
	ctx.lr = 0x824705D0;
	sub_82535080(ctx, base);
	// 824705D0: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824705D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824705D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824705DC: 4BFF9C45  bl 0x8246a220
	ctx.lr = 0x824705E0;
	sub_8246A220(ctx, base);
	// 824705E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824705E4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824705E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824705EC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824705F0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 824705F4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 824705F8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824705FC: 40990094  ble cr6, 0x82470690
	if !ctx.cr[6].gt {
	pc = 0x82470690; continue 'dispatch;
	}
	// 82470600: 3BDF0003  addi r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 3;
	// 82470604: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82470608: 4099001C  ble cr6, 0x82470624
	if !ctx.cr[6].gt {
	pc = 0x82470624; continue 'dispatch;
	}
	// 8247060C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82470610: 41980008  blt cr6, 0x82470618
	if ctx.cr[6].lt {
	pc = 0x82470618; continue 'dispatch;
	}
	// 82470614: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x82470618; continue 'dispatch;
            }
            0x82470618 => {
    //   block [0x82470618..0x82470624)
	// 82470618: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8247061C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470620: 4BFFDCA9  bl 0x8246e2c8
	ctx.lr = 0x82470624;
	sub_8246E2C8(ctx, base);
	pc = 0x82470624; continue 'dispatch;
            }
            0x82470624 => {
    //   block [0x82470624..0x82470660)
	// 82470624: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82470628: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8247062C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82470630: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82470634: 4BFF9CF5  bl 0x8246a328
	ctx.lr = 0x82470638;
	sub_8246A328(ctx, base);
	// 82470638: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8247063C: 7FABF9AE  stbx r29, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u8) };
	// 82470640: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470644: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82470648: 894BFFFF  lbz r10, -1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8247064C: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 82470650: 419A0010  beq cr6, 0x82470660
	if ctx.cr[6].eq {
	pc = 0x82470660; continue 'dispatch;
	}
	// 82470654: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 82470658: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8247065C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	pc = 0x82470660; continue 'dispatch;
            }
            0x82470660 => {
    //   block [0x82470660..0x82470690)
	// 82470660: 3920002A  li r9, 0x2a
	ctx.r[9].s64 = 42;
	// 82470664: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 82470668: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8247066C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82470670: 9BAB0001  stb r29, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 82470674: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470678: 816A4DA0  lwz r11, 0x4da0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19872 as u32) ) } as u64;
	// 8247067C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470680: 4E800421  bctrl
	ctx.lr = 0x82470684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470684: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82470688: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247068C: 409A000C  bne cr6, 0x82470698
	if !ctx.cr[6].eq {
	pc = 0x82470698; continue 'dispatch;
	}
            }
            0x82470690 => {
    //   block [0x82470690..0x82470698)
	// 82470690: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82470694: 48000008  b 0x8247069c
	pc = 0x8247069C; continue 'dispatch;
            }
            0x82470698 => {
    //   block [0x82470698..0x8247069C)
	// 82470698: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	pc = 0x8247069C; continue 'dispatch;
            }
            0x8247069C => {
    //   block [0x8247069C..0x824706DC)
	// 8247069C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824706A0: 480D67F9  bl 0x82546e98
	ctx.lr = 0x824706A4;
	sub_82546E98(ctx, base);
	// 824706A4: 546BE7FE  rlwinm r11, r3, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 824706A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824706AC: 409A003C  bne cr6, 0x824706e8
	if !ctx.cr[6].eq {
	pc = 0x824706E8; continue 'dispatch;
	}
	// 824706B0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824706B4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824706B8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824706BC: 409A0020  bne cr6, 0x824706dc
	if !ctx.cr[6].eq {
	pc = 0x824706DC; continue 'dispatch;
	}
	// 824706C0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824706C4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824706C8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824706CC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824706D0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824706D4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824706D8: 4BFF39E1  bl 0x824640b8
	ctx.lr = 0x824706DC;
	sub_824640B8(ctx, base);
	pc = 0x824706DC; continue 'dispatch;
            }
            0x824706DC => {
    //   block [0x824706DC..0x824706E8)
	// 824706DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824706E0: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824706E4: 480C4A20  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824706E8 => {
    //   block [0x824706E8..0x82470710)
	// 824706E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824706EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824706F0: 4BF5AE21  bl 0x823cb510
	ctx.lr = 0x824706F4;
	sub_823CB510(ctx, base);
	// 824706F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824706F8: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 824706FC: 419A0074  beq cr6, 0x82470770
	if ctx.cr[6].eq {
	pc = 0x82470770; continue 'dispatch;
	}
	// 82470700: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470704: 3BEB88FC  addi r31, r11, -0x7704
	ctx.r[31].s64 = ctx.r[11].s64 + -30468;
	// 82470708: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8247070C: 3BCB88F8  addi r30, r11, -0x7708
	ctx.r[30].s64 = ctx.r[11].s64 + -30472;
	pc = 0x82470710; continue 'dispatch;
            }
            0x82470710 => {
    //   block [0x82470710..0x82470758)
	// 82470710: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 82470714: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470718: 4BFF98B9  bl 0x82469fd0
	ctx.lr = 0x8247071C;
	sub_82469FD0(ctx, base);
	// 8247071C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470720: 419A003C  beq cr6, 0x8247075c
	if ctx.cr[6].eq {
	pc = 0x8247075C; continue 'dispatch;
	}
	// 82470724: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 82470728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8247072C: 4BFF98A5  bl 0x82469fd0
	ctx.lr = 0x82470730;
	sub_82469FD0(ctx, base);
	// 82470730: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470734: 419A0028  beq cr6, 0x8247075c
	if ctx.cr[6].eq {
	pc = 0x8247075C; continue 'dispatch;
	}
	// 82470738: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8247073C: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82470740: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82470744: 556BE7FE  rlwinm r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82470748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247074C: 419A000C  beq cr6, 0x82470758
	if ctx.cr[6].eq {
	pc = 0x82470758; continue 'dispatch;
	}
	// 82470750: 4BFFFD81  bl 0x824704d0
	ctx.lr = 0x82470754;
	sub_824704D0(ctx, base);
	// 82470754: 48000008  b 0x8247075c
	pc = 0x8247075C; continue 'dispatch;
            }
            0x82470758 => {
    //   block [0x82470758..0x8247075C)
	// 82470758: 4BFFFDF1  bl 0x82470548
	ctx.lr = 0x8247075C;
	sub_82470548(ctx, base);
	pc = 0x8247075C; continue 'dispatch;
            }
            0x8247075C => {
    //   block [0x8247075C..0x82470770)
	// 8247075C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82470760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470764: 4BF5AD55  bl 0x823cb4b8
	ctx.lr = 0x82470768;
	sub_823CB4B8(ctx, base);
	// 82470768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8247076C: 409AFFA4  bne cr6, 0x82470710
	if !ctx.cr[6].eq {
	pc = 0x82470710; continue 'dispatch;
	}
	pc = 0x82470770; continue 'dispatch;
            }
            0x82470770 => {
    //   block [0x82470770..0x824707A4)
	// 82470770: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470774: 4BF501E5  bl 0x823c0958
	ctx.lr = 0x82470778;
	sub_823C0958(ctx, base);
	// 82470778: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8247077C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82470780: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82470784: 409A0020  bne cr6, 0x824707a4
	if !ctx.cr[6].eq {
	pc = 0x824707A4; continue 'dispatch;
	}
	// 82470788: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247078C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82470790: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82470794: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470798: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247079C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824707A0: 4BFF3919  bl 0x824640b8
	ctx.lr = 0x824707A4;
	sub_824640B8(ctx, base);
	pc = 0x824707A4; continue 'dispatch;
            }
            0x824707A4 => {
    //   block [0x824707A4..0x824707B0)
	// 824707A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824707A8: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824707AC: 480C4958  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824707B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824707B0 size=104
    let mut pc: u32 = 0x824707B0;
    'dispatch: loop {
        match pc {
            0x824707B0 => {
    //   block [0x824707B0..0x824707D0)
	// 824707B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824707B4: 480C4909  bl 0x825350bc
	ctx.lr = 0x824707B8;
	sub_82535080(ctx, base);
	// 824707B8: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824707BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824707C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824707C4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 824707C8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824707CC: 419A0040  beq cr6, 0x8247080c
	if ctx.cr[6].eq {
	pc = 0x8247080C; continue 'dispatch;
	}
	pc = 0x824707D0; continue 'dispatch;
            }
            0x824707D0 => {
    //   block [0x824707D0..0x824707E0)
	// 824707D0: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 824707D4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824707D8: 41990008  bgt cr6, 0x824707e0
	if ctx.cr[6].gt {
	pc = 0x824707E0; continue 'dispatch;
	}
	// 824707DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	pc = 0x824707E0; continue 'dispatch;
            }
            0x824707E0 => {
    //   block [0x824707E0..0x8247080C)
	// 824707E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824707E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824707E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824707EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824707F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824707F4: 4E800421  bctrl
	ctx.lr = 0x824707F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824707F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824707FC: 419A0010  beq cr6, 0x8247080c
	if ctx.cr[6].eq {
	pc = 0x8247080C; continue 'dispatch;
	}
	// 82470800: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82470804: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82470808: 409AFFC8  bne cr6, 0x824707d0
	if !ctx.cr[6].eq {
	pc = 0x824707D0; continue 'dispatch;
	}
            }
            0x8247080C => {
    //   block [0x8247080C..0x82470818)
	// 8247080C: 7C7FE850  subf r3, r31, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82470810: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82470814: 480C48F8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470818 size=88
    let mut pc: u32 = 0x82470818;
    'dispatch: loop {
        match pc {
            0x82470818 => {
    //   block [0x82470818..0x8247085C)
	// 82470818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470824: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470828: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8247082C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82470830: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470838: 4E800421  bctrl
	ctx.lr = 0x8247083C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247083C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470840: 419A001C  beq cr6, 0x8247085c
	if ctx.cr[6].eq {
	pc = 0x8247085C; continue 'dispatch;
	}
	// 82470844: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470848: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8247084C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470858: 4E800020  blr
	return;
            }
            0x8247085C => {
    //   block [0x8247085C..0x82470870)
	// 8247085C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82470860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470870 size=8
    let mut pc: u32 = 0x82470870;
    'dispatch: loop {
        match pc {
            0x82470870 => {
    //   block [0x82470870..0x82470878)
	// 82470870: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470878 size=8
    let mut pc: u32 = 0x82470878;
    'dispatch: loop {
        match pc {
            0x82470878 => {
    //   block [0x82470878..0x82470880)
	// 82470878: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8247087C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470880 size=8
    let mut pc: u32 = 0x82470880;
    'dispatch: loop {
        match pc {
            0x82470880 => {
    //   block [0x82470880..0x82470888)
	// 82470880: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470888 size=8
    let mut pc: u32 = 0x82470888;
    'dispatch: loop {
        match pc {
            0x82470888 => {
    //   block [0x82470888..0x82470890)
	// 82470888: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8247088C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470890 size=12
    let mut pc: u32 = 0x82470890;
    'dispatch: loop {
        match pc {
            0x82470890 => {
    //   block [0x82470890..0x8247089C)
	// 82470890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82470894: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82470898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824708A0 size=12
    let mut pc: u32 = 0x824708A0;
    'dispatch: loop {
        match pc {
            0x824708A0 => {
    //   block [0x824708A0..0x824708AC)
	// 824708A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824708A4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824708A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824708B0 size=4
    let mut pc: u32 = 0x824708B0;
    'dispatch: loop {
        match pc {
            0x824708B0 => {
    //   block [0x824708B0..0x824708B4)
	// 824708B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824708B8 size=32
    let mut pc: u32 = 0x824708B8;
    'dispatch: loop {
        match pc {
            0x824708B8 => {
    //   block [0x824708B8..0x824708D8)
	// 824708B8: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 824708BC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 824708C0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824708C4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824708C8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824708CC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824708D0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824708D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824708D8 size=64
    let mut pc: u32 = 0x824708D8;
    'dispatch: loop {
        match pc {
            0x824708D8 => {
    //   block [0x824708D8..0x82470904)
	// 824708D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824708DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824708E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824708E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824708E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824708EC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824708F0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824708F4: 419A0010  beq cr6, 0x82470904
	if ctx.cr[6].eq {
	pc = 0x82470904; continue 'dispatch;
	}
	// 824708F8: 4BF80A79  bl 0x823f1370
	ctx.lr = 0x824708FC;
	sub_823F1370(ctx, base);
	// 824708FC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82470900: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x82470904; continue 'dispatch;
            }
            0x82470904 => {
    //   block [0x82470904..0x82470918)
	// 82470904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247090C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470918 size=92
    let mut pc: u32 = 0x82470918;
    'dispatch: loop {
        match pc {
            0x82470918 => {
    //   block [0x82470918..0x82470974)
	// 82470918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247091C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247092C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470930: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470938: 4E800421  bctrl
	ctx.lr = 0x8247093C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247093C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470940: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470944: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470948: 4BF80A11  bl 0x823f1358
	ctx.lr = 0x8247094C;
	sub_823F1358(ctx, base);
	// 8247094C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82470950: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82470954: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82470958: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8247095C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82470960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247096C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470978 size=100
    let mut pc: u32 = 0x82470978;
    'dispatch: loop {
        match pc {
            0x82470978 => {
    //   block [0x82470978..0x824709B0)
	// 82470978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470984: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247098C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470990: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470994: 419A0030  beq cr6, 0x824709c4
	if ctx.cr[6].eq {
	pc = 0x824709C4; continue 'dispatch;
	}
	// 82470998: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8247099C: 4BF80AA5  bl 0x823f1440
	ctx.lr = 0x824709A0;
	sub_823F1440(ctx, base);
	// 824709A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824709A4: 4099000C  ble cr6, 0x824709b0
	if !ctx.cr[6].gt {
	pc = 0x824709B0; continue 'dispatch;
	}
	// 824709A8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824709AC: 409A001C  bne cr6, 0x824709c8
	if !ctx.cr[6].eq {
	pc = 0x824709C8; continue 'dispatch;
	}
	pc = 0x824709B0; continue 'dispatch;
            }
            0x824709B0 => {
    //   block [0x824709B0..0x824709C4)
	// 824709B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824709B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824709B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824709BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824709C0: 4E800421  bctrl
	ctx.lr = 0x824709C4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824709C4 => {
    //   block [0x824709C4..0x824709C8)
	// 824709C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x824709C8; continue 'dispatch;
            }
            0x824709C8 => {
    //   block [0x824709C8..0x824709DC)
	// 824709C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824709CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824709D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824709D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824709D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824709E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824709E0 size=100
    let mut pc: u32 = 0x824709E0;
    'dispatch: loop {
        match pc {
            0x824709E0 => {
    //   block [0x824709E0..0x82470A18)
	// 824709E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824709E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824709E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824709EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824709F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824709F4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824709F8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824709FC: 419A0030  beq cr6, 0x82470a2c
	if ctx.cr[6].eq {
	pc = 0x82470A2C; continue 'dispatch;
	}
	// 82470A00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82470A04: 4BF80A75  bl 0x823f1478
	ctx.lr = 0x82470A08;
	sub_823F1478(ctx, base);
	// 82470A08: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470A0C: 4099000C  ble cr6, 0x82470a18
	if !ctx.cr[6].gt {
	pc = 0x82470A18; continue 'dispatch;
	}
	// 82470A10: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470A14: 409A001C  bne cr6, 0x82470a30
	if !ctx.cr[6].eq {
	pc = 0x82470A30; continue 'dispatch;
	}
	pc = 0x82470A18; continue 'dispatch;
            }
            0x82470A18 => {
    //   block [0x82470A18..0x82470A2C)
	// 82470A18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470A20: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470A28: 4E800421  bctrl
	ctx.lr = 0x82470A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82470A2C => {
    //   block [0x82470A2C..0x82470A30)
	// 82470A2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82470A30; continue 'dispatch;
            }
            0x82470A30 => {
    //   block [0x82470A30..0x82470A44)
	// 82470A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470A60 size=8
    let mut pc: u32 = 0x82470A60;
    'dispatch: loop {
        match pc {
            0x82470A60 => {
    //   block [0x82470A60..0x82470A68)
	// 82470A60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470A68 size=220
    let mut pc: u32 = 0x82470A68;
    'dispatch: loop {
        match pc {
            0x82470A68 => {
    //   block [0x82470A68..0x82470B28)
	// 82470A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470A74: 9421FBF0  stwu r1, -0x410(r1)
	ea = ctx.r[1].u32.wrapping_add(-1040 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470A78: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 82470A7C: 897F91E4  lbz r11, -0x6e1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-28188 as u32) ) } as u64;
	// 82470A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82470A84: 409A00AC  bne cr6, 0x82470b30
	if !ctx.cr[6].eq {
	pc = 0x82470B30; continue 'dispatch;
	}
	// 82470A88: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82470A8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82470A90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470A94: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82470A98: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82470A9C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82470AA0: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82470AA4: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82470AA8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82470AAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82470AB0: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82470AB4: 4BF80765  bl 0x823f1218
	ctx.lr = 0x82470AB8;
	sub_823F1218(ctx, base);
	// 82470AB8: 38810270  addi r4, r1, 0x270
	ctx.r[4].s64 = ctx.r[1].s64 + 624;
	// 82470ABC: 38600202  li r3, 0x202
	ctx.r[3].s64 = 514;
	// 82470AC0: 4BF80889  bl 0x823f1348
	ctx.lr = 0x82470AC4;
	sub_823F1348(ctx, base);
	// 82470AC4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470AC8: 409A0060  bne cr6, 0x82470b28
	if !ctx.cr[6].eq {
	pc = 0x82470B28; continue 'dispatch;
	}
	// 82470ACC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82470AD0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82470AD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470AD8: 4BFF8321  bl 0x82468df8
	ctx.lr = 0x82470ADC;
	sub_82468DF8(ctx, base);
	// 82470ADC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470AE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470AE4: 388B8928  addi r4, r11, -0x76d8
	ctx.r[4].s64 = ctx.r[11].s64 + -30424;
	// 82470AE8: 4BFF77B9  bl 0x824682a0
	ctx.lr = 0x82470AEC;
	sub_824682A0(ctx, base);
	// 82470AEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82470AF0: 3CA03218  lis r5, 0x3218
	ctx.r[5].s64 = 840433664;
	// 82470AF4: 39000048  li r8, 0x48
	ctx.r[8].s64 = 72;
	// 82470AF8: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82470AFC: 60A525F8  ori r5, r5, 0x25f8
	ctx.r[5].u64 = ctx.r[5].u64 | 9720;
	// 82470B00: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82470B04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470B08: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82470B0C: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 82470B10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470B14: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82470B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470B1C: 4E800421  bctrl
	ctx.lr = 0x82470B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470B20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470B24: 4BFF7DBD  bl 0x824688e0
	ctx.lr = 0x82470B28;
	sub_824688E0(ctx, base);
            }
            0x82470B28 => {
    //   block [0x82470B28..0x82470B30)
	// 82470B28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82470B2C: 997F91E4  stb r11, -0x6e1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-28188 as u32), ctx.r[11].u8 ) };
	pc = 0x82470B30; continue 'dispatch;
            }
            0x82470B30 => {
    //   block [0x82470B30..0x82470B44)
	// 82470B30: 38210410  addi r1, r1, 0x410
	ctx.r[1].s64 = ctx.r[1].s64 + 1040;
	// 82470B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470B48 size=96
    let mut pc: u32 = 0x82470B48;
    'dispatch: loop {
        match pc {
            0x82470B48 => {
    //   block [0x82470B48..0x82470B80)
	// 82470B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470B5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470B60: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 82470B64: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470B68: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470B6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470B70: 419A0010  beq cr6, 0x82470b80
	if ctx.cr[6].eq {
	pc = 0x82470B80; continue 'dispatch;
	}
	// 82470B74: 4BF807FD  bl 0x823f1370
	ctx.lr = 0x82470B78;
	sub_823F1370(ctx, base);
	// 82470B78: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82470B7C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x82470B80; continue 'dispatch;
            }
            0x82470B80 => {
    //   block [0x82470B80..0x82470BA8)
	// 82470B80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82470B84: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82470B88: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82470B8C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82470B90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82470BA8 size=316
    let mut pc: u32 = 0x82470BA8;
    'dispatch: loop {
        match pc {
            0x82470BA8 => {
    //   block [0x82470BA8..0x82470C10)
	// 82470BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470BAC: 480C4511  bl 0x825350bc
	ctx.lr = 0x82470BB0;
	sub_82535080(ctx, base);
	// 82470BB0: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470BB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82470BB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82470BBC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82470BC0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82470BC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82470BC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470BCC: 4BFF976D  bl 0x8246a338
	ctx.lr = 0x82470BD0;
	sub_8246A338(ctx, base);
	// 82470BD0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470BD4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82470BD8: B3C10052  sth r30, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[30].u16 ) };
	// 82470BDC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82470BE0: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 82470BE4: 216B0009  subfic r11, r11, 9
	ctx.xer.ca = ctx.r[11].u32 <= 9 as u32;
	ctx.r[11].s64 = (9 as i64) - ctx.r[11].s64;
	// 82470BE8: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82470BEC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82470BF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82470BF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82470BF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82470BFC: 419A0014  beq cr6, 0x82470c10
	if ctx.cr[6].eq {
	pc = 0x82470C10; continue 'dispatch;
	}
	// 82470C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470C04: 4BF808AD  bl 0x823f14b0
	ctx.lr = 0x82470C08;
	sub_823F14B0(ctx, base);
	// 82470C08: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82470C0C: 4800005C  b 0x82470c68
	pc = 0x82470C68; continue 'dispatch;
            }
            0x82470C10 => {
    //   block [0x82470C10..0x82470C68)
	// 82470C10: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82470C14: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82470C18: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470C1C: 4BFF81DD  bl 0x82468df8
	ctx.lr = 0x82470C20;
	sub_82468DF8(ctx, base);
	// 82470C20: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470C24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470C28: 388B8980  addi r4, r11, -0x7680
	ctx.r[4].s64 = ctx.r[11].s64 + -30336;
	// 82470C2C: 4BFF7675  bl 0x824682a0
	ctx.lr = 0x82470C30;
	sub_824682A0(ctx, base);
	// 82470C30: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82470C34: 390000C3  li r8, 0xc3
	ctx.r[8].s64 = 195;
	// 82470C38: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82470C3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470C40: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82470C44: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82470C48: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470C4C: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 82470C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470C54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82470C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470C5C: 4E800421  bctrl
	ctx.lr = 0x82470C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470C60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470C64: 4BFF7C7D  bl 0x824688e0
	ctx.lr = 0x82470C68;
	sub_824688E0(ctx, base);
            }
            0x82470C68 => {
    //   block [0x82470C68..0x82470CA4)
	// 82470C68: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470C6C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470C70: 409A0034  bne cr6, 0x82470ca4
	if !ctx.cr[6].eq {
	pc = 0x82470CA4; continue 'dispatch;
	}
	// 82470C74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470C78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470C7C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470C84: 4E800421  bctrl
	ctx.lr = 0x82470C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470C88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470C8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470C90: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470C94: 4BF806C5  bl 0x823f1358
	ctx.lr = 0x82470C98;
	sub_823F1358(ctx, base);
	// 82470C98: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470C9C: 907D0020  stw r3, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82470CA0: 419A002C  beq cr6, 0x82470ccc
	if ctx.cr[6].eq {
	pc = 0x82470CCC; continue 'dispatch;
	}
            }
            0x82470CA4 => {
    //   block [0x82470CA4..0x82470CCC)
	// 82470CA4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82470CA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82470CAC: 4BF80735  bl 0x823f13e0
	ctx.lr = 0x82470CB0;
	sub_823F13E0(ctx, base);
	// 82470CB0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470CB4: 409A0024  bne cr6, 0x82470cd8
	if !ctx.cr[6].eq {
	pc = 0x82470CD8; continue 'dispatch;
	}
	// 82470CB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470CBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470CC0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470CC8: 4E800421  bctrl
	ctx.lr = 0x82470CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82470CCC => {
    //   block [0x82470CCC..0x82470CD8)
	// 82470CCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470CD0: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82470CD4: 480C4438  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82470CD8 => {
    //   block [0x82470CD8..0x82470CE4)
	// 82470CD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82470CDC: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82470CE0: 480C442C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470CE8 size=124
    let mut pc: u32 = 0x82470CE8;
    'dispatch: loop {
        match pc {
            0x82470CE8 => {
    //   block [0x82470CE8..0x82470D4C)
	// 82470CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82470CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470D00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470D04: 4BFFEC2D  bl 0x8246f930
	ctx.lr = 0x82470D08;
	sub_8246F930(ctx, base);
	// 82470D08: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470D0C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82470D10: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82470D14: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 82470D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470D1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470D20: 409A002C  bne cr6, 0x82470d4c
	if !ctx.cr[6].eq {
	pc = 0x82470D4C; continue 'dispatch;
	}
	// 82470D24: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470D2C: 4E800421  bctrl
	ctx.lr = 0x82470D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470D30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470D34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470D38: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470D3C: 4BF8061D  bl 0x823f1358
	ctx.lr = 0x82470D40;
	sub_823F1358(ctx, base);
	// 82470D40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82470D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470D48: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
            }
            0x82470D4C => {
    //   block [0x82470D4C..0x82470D64)
	// 82470D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470D58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82470D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470D68 size=616
    let mut pc: u32 = 0x82470D68;
    'dispatch: loop {
        match pc {
            0x82470D68 => {
    //   block [0x82470D68..0x82470DF4)
	// 82470D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470D6C: 480C4345  bl 0x825350b0
	ctx.lr = 0x82470D70;
	sub_82535080(ctx, base);
	// 82470D70: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470D78: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82470D7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470D80: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470D88: 4E800421  bctrl
	ctx.lr = 0x82470D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470D90: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470D94: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470D98: 4BF805C1  bl 0x823f1358
	ctx.lr = 0x82470D9C;
	sub_823F1358(ctx, base);
	// 82470D9C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470DA0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82470DA4: 419A0064  beq cr6, 0x82470e08
	if ctx.cr[6].eq {
	pc = 0x82470E08; continue 'dispatch;
	}
	// 82470DA8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82470DAC: B38100A2  sth r28, 0xa2(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(162 as u32), ctx.r[28].u16 ) };
	// 82470DB0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82470DB4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82470DB8: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 82470DBC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82470DC0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82470DC4: B16100A0  sth r11, 0xa0(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u16 ) };
	// 82470DC8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82470DCC: 93A100A4  stw r29, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 82470DD0: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 82470DD4: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82470DD8: 4BF805D1  bl 0x823f13a8
	ctx.lr = 0x82470DDC;
	sub_823F13A8(ctx, base);
	// 82470DDC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82470DE0: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82470DE4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470DE8: 4BF805E1  bl 0x823f13c8
	ctx.lr = 0x82470DEC;
	sub_823F13C8(ctx, base);
	// 82470DEC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470DF0: 409A0024  bne cr6, 0x82470e14
	if !ctx.cr[6].eq {
	pc = 0x82470E14; continue 'dispatch;
	}
            }
            0x82470DF4 => {
    //   block [0x82470DF4..0x82470E08)
	// 82470DF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470DFC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470E04: 4E800421  bctrl
	ctx.lr = 0x82470E08;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82470E08 => {
    //   block [0x82470E08..0x82470E14)
	// 82470E08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470E0C: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82470E10: 480C42F0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x82470E14 => {
    //   block [0x82470E14..0x82470E50)
	// 82470E14: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82470E18: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470E1C: 4BF805DD  bl 0x823f13f8
	ctx.lr = 0x82470E20;
	sub_823F13F8(ctx, base);
	// 82470E20: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470E24: 419AFFD0  beq cr6, 0x82470df4
	if ctx.cr[6].eq {
	pc = 0x82470DF4; continue 'dispatch;
	}
	// 82470E28: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470E2C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82470E30: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82470E34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470E38: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82470E3C: 4BFF31FD  bl 0x82464038
	ctx.lr = 0x82470E40;
	sub_82464038(ctx, base);
	// 82470E40: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82470E44: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82470E48: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82470E4C: 9BA30000  stb r29, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	pc = 0x82470E50; continue 'dispatch;
            }
            0x82470E50 => {
    //   block [0x82470E50..0x82470E6C)
	// 82470E50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470E54: 4BF8043D  bl 0x823f1290
	ctx.lr = 0x82470E58;
	sub_823F1290(ctx, base);
	// 82470E58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470E5C: 419AFFF4  beq cr6, 0x82470e50
	if ctx.cr[6].eq {
	pc = 0x82470E50; continue 'dispatch;
	}
	// 82470E60: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82470E64: 409A0038  bne cr6, 0x82470e9c
	if !ctx.cr[6].eq {
	pc = 0x82470E9C; continue 'dispatch;
	}
	// 82470E68: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	pc = 0x82470E6C; continue 'dispatch;
            }
            0x82470E6C => {
    //   block [0x82470E6C..0x82470E9C)
	// 82470E6C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82470E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82470E74: 419A0058  beq cr6, 0x82470ecc
	if ctx.cr[6].eq {
	pc = 0x82470ECC; continue 'dispatch;
	}
	// 82470E78: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470E7C: 89010067  lbz r8, 0x67(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(103 as u32) ) } as u64;
	// 82470E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470E84: 88E10066  lbz r7, 0x66(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82470E88: 388B89C4  addi r4, r11, -0x763c
	ctx.r[4].s64 = ctx.r[11].s64 + -30268;
	// 82470E8C: 88C10065  lbz r6, 0x65(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(101 as u32) ) } as u64;
	// 82470E90: 88A10064  lbz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82470E94: 4BFF96CD  bl 0x8246a560
	ctx.lr = 0x82470E98;
	sub_8246A560(ctx, base);
	// 82470E98: 4800008C  b 0x82470f24
	pc = 0x82470F24; continue 'dispatch;
            }
            0x82470E9C => {
    //   block [0x82470E9C..0x82470ECC)
	// 82470E9C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82470EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82470EA4: 419AFFC8  beq cr6, 0x82470e6c
	if ctx.cr[6].eq {
	pc = 0x82470E6C; continue 'dispatch;
	}
	// 82470EA8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470EAC: 89010063  lbz r8, 0x63(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(99 as u32) ) } as u64;
	// 82470EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470EB4: 88E10062  lbz r7, 0x62(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 82470EB8: 388B89C4  addi r4, r11, -0x763c
	ctx.r[4].s64 = ctx.r[11].s64 + -30268;
	// 82470EBC: 88C10061  lbz r6, 0x61(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 82470EC0: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82470EC4: 4BFF969D  bl 0x8246a560
	ctx.lr = 0x82470EC8;
	sub_8246A560(ctx, base);
	// 82470EC8: 4800005C  b 0x82470f24
	pc = 0x82470F24; continue 'dispatch;
            }
            0x82470ECC => {
    //   block [0x82470ECC..0x82470F00)
	// 82470ECC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82470ED0: 3BCBCD5C  addi r30, r11, -0x32a4
	ctx.r[30].s64 = ctx.r[11].s64 + -12964;
	// 82470ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82470ED8: 4BFF9349  bl 0x8246a220
	ctx.lr = 0x82470EDC;
	sub_8246A220(ctx, base);
	// 82470EDC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82470EE0: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82470EE4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82470EE8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82470EEC: 40980024  bge cr6, 0x82470f10
	if !ctx.cr[6].lt {
	pc = 0x82470F10; continue 'dispatch;
	}
	// 82470EF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470EF4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470EF8: 41980008  blt cr6, 0x82470f00
	if ctx.cr[6].lt {
	pc = 0x82470F00; continue 'dispatch;
	}
	// 82470EFC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82470F00; continue 'dispatch;
            }
            0x82470F00 => {
    //   block [0x82470F00..0x82470F10)
	// 82470F00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82470F04: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82470F08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470F0C: 4BFFD3BD  bl 0x8246e2c8
	ctx.lr = 0x82470F10;
	sub_8246E2C8(ctx, base);
	pc = 0x82470F10; continue 'dispatch;
            }
            0x82470F10 => {
    //   block [0x82470F10..0x82470F24)
	// 82470F10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82470F14: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470F1C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82470F20: 4BFF9409  bl 0x8246a328
	ctx.lr = 0x82470F24;
	sub_8246A328(ctx, base);
	pc = 0x82470F24; continue 'dispatch;
            }
            0x82470F24 => {
    //   block [0x82470F24..0x82470FC4)
	// 82470F24: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82470F28: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82470F2C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82470F30: 4BFF7EC9  bl 0x82468df8
	ctx.lr = 0x82470F34;
	sub_82468DF8(ctx, base);
	// 82470F34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470F38: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82470F3C: 388B89B0  addi r4, r11, -0x7650
	ctx.r[4].s64 = ctx.r[11].s64 + -30288;
	// 82470F40: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470F44: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82470F48: 3BEB89A8  addi r31, r11, -0x7658
	ctx.r[31].s64 = ctx.r[11].s64 + -30296;
	// 82470F4C: 4BFF7355  bl 0x824682a0
	ctx.lr = 0x82470F50;
	sub_824682A0(ctx, base);
	// 82470F50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470F54: 4BFF7AA5  bl 0x824689f8
	ctx.lr = 0x82470F58;
	sub_824689F8(ctx, base);
	// 82470F58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82470F5C: 4BFF7345  bl 0x824682a0
	ctx.lr = 0x82470F60;
	sub_824682A0(ctx, base);
	// 82470F60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82470F64: 4BFF7475  bl 0x824683d8
	ctx.lr = 0x82470F68;
	sub_824683D8(ctx, base);
	// 82470F68: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82470F6C: 3900016C  li r8, 0x16c
	ctx.r[8].s64 = 364;
	// 82470F70: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82470F74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82470F78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82470F7C: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82470F80: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470F84: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 82470F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470F8C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82470F90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470F94: 4E800421  bctrl
	ctx.lr = 0x82470F98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470F98: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82470F9C: 4BFF7945  bl 0x824688e0
	ctx.lr = 0x82470FA0;
	sub_824688E0(ctx, base);
	// 82470FA0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82470FA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82470FA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82470FAC: 409A0018  bne cr6, 0x82470fc4
	if !ctx.cr[6].eq {
	pc = 0x82470FC4; continue 'dispatch;
	}
	// 82470FB0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82470FB4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82470FB8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82470FBC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470FC0: 4BFF30F9  bl 0x824640b8
	ctx.lr = 0x82470FC4;
	sub_824640B8(ctx, base);
            }
            0x82470FC4 => {
    //   block [0x82470FC4..0x82470FD0)
	// 82470FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82470FC8: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82470FCC: 480C4134  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470FD0 size=468
    let mut pc: u32 = 0x82470FD0;
    'dispatch: loop {
        match pc {
            0x82470FD0 => {
    //   block [0x82470FD0..0x8247111C)
	// 82470FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470FD4: 480C40E1  bl 0x825350b4
	ctx.lr = 0x82470FD8;
	sub_82535080(ctx, base);
	// 82470FD8: 9421FB20  stwu r1, -0x4e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1248 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470FDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470FE0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470FE4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82470FE8: 419A0134  beq cr6, 0x8247111c
	if ctx.cr[6].eq {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 82470FEC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82470FF0: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82470FF4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82470FF8: 916101A4  stw r11, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[11].u32 ) };
	// 82470FFC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82471000: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 82471004: 38C101A0  addi r6, r1, 0x1a0
	ctx.r[6].s64 = ctx.r[1].s64 + 416;
	// 82471008: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8247100C: 93C10090  stw r30, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82471010: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82471014: 93C101A0  stw r30, 0x1a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 82471018: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8247101C: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82471020: 4BF80401  bl 0x823f1420
	ctx.lr = 0x82471024;
	sub_823F1420(ctx, base);
	// 82471024: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471028: 409900F4  ble cr6, 0x8247111c
	if !ctx.cr[6].gt {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 8247102C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82471030: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471034: 4BF8048D  bl 0x823f14c0
	ctx.lr = 0x82471038;
	sub_823F14C0(ctx, base);
	// 82471038: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8247103C: 419A00E0  beq cr6, 0x8247111c
	if ctx.cr[6].eq {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 82471040: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82471044: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471048: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8247104C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82471050: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82471054: 4BF803B5  bl 0x823f1408
	ctx.lr = 0x82471058;
	sub_823F1408(ctx, base);
	// 82471058: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247105C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82471060: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82471064: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82471068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247106C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82471070: 4BFF2FC9  bl 0x82464038
	ctx.lr = 0x82471074;
	sub_82464038(ctx, base);
	// 82471074: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471078: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8247107C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82471080: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82471084: 388B89D0  addi r4, r11, -0x7630
	ctx.r[4].s64 = ctx.r[11].s64 + -30256;
	// 82471088: 9B630000  stb r27, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8247108C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82471090: A0C10082  lhz r6, 0x82(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(130 as u32) ) } as u64;
	// 82471094: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82471098: 4BFF94C9  bl 0x8246a560
	ctx.lr = 0x8247109C;
	sub_8246A560(ctx, base);
	// 8247109C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824710A0: 388102B0  addi r4, r1, 0x2b0
	ctx.r[4].s64 = ctx.r[1].s64 + 688;
	// 824710A4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824710A8: 4BFF7D51  bl 0x82468df8
	ctx.lr = 0x824710AC;
	sub_82468DF8(ctx, base);
	// 824710AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824710B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824710B4: 4BFF7945  bl 0x824689f8
	ctx.lr = 0x824710B8;
	sub_824689F8(ctx, base);
	// 824710B8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824710BC: 39000194  li r8, 0x194
	ctx.r[8].s64 = 404;
	// 824710C0: 38C102B0  addi r6, r1, 0x2b0
	ctx.r[6].s64 = ctx.r[1].s64 + 688;
	// 824710C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 824710C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824710CC: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824710D0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824710D4: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 824710D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824710DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824710E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824710E4: 4E800421  bctrl
	ctx.lr = 0x824710E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824710E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824710EC: 4BFF77F5  bl 0x824688e0
	ctx.lr = 0x824710F0;
	sub_824688E0(ctx, base);
	// 824710F0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 824710F4: 409A0034  bne cr6, 0x82471128
	if !ctx.cr[6].eq {
	pc = 0x82471128; continue 'dispatch;
	}
	// 824710F8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824710FC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82471100: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471104: 409A0018  bne cr6, 0x8247111c
	if !ctx.cr[6].eq {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 82471108: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8247110C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82471110: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471114: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82471118: 4BFF2FA1  bl 0x824640b8
	ctx.lr = 0x8247111C;
	sub_824640B8(ctx, base);
            }
            0x8247111C => {
    //   block [0x8247111C..0x82471128)
	// 8247111C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471120: 382104E0  addi r1, r1, 0x4e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1248;
	// 82471124: 480C3FE0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82471128 => {
    //   block [0x82471128..0x82471198)
	// 82471128: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8247112C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82471130: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82471134: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82471138: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8247113C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471140: 4BF80269  bl 0x823f13a8
	ctx.lr = 0x82471144;
	sub_823F13A8(ctx, base);
	// 82471144: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82471148: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8247114C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82471150: 4BFF2EE9  bl 0x82464038
	ctx.lr = 0x82471154;
	sub_82464038(ctx, base);
	// 82471154: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82471158: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 8247115C: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82471160: 4BFFE7D1  bl 0x8246f930
	ctx.lr = 0x82471164;
	sub_8246F930(ctx, base);
	// 82471164: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471168: 93FE0020  stw r31, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8247116C: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 82471170: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82471174: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82471178: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8247117C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471180: 409A0018  bne cr6, 0x82471198
	if !ctx.cr[6].eq {
	pc = 0x82471198; continue 'dispatch;
	}
	// 82471184: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82471188: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8247118C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471190: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82471194: 4BFF2F25  bl 0x824640b8
	ctx.lr = 0x82471198;
	sub_824640B8(ctx, base);
	pc = 0x82471198; continue 'dispatch;
            }
            0x82471198 => {
    //   block [0x82471198..0x824711A4)
	// 82471198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247119C: 382104E0  addi r1, r1, 0x4e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1248;
	// 824711A0: 480C3F64  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824711A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824711A8 size=152
    let mut pc: u32 = 0x824711A8;
    'dispatch: loop {
        match pc {
            0x824711A8 => {
    //   block [0x824711A8..0x824711E8)
	// 824711A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824711AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824711B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824711B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824711B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824711BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824711C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824711C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824711C8: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 824711CC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824711D0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824711D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824711D8: 419A0010  beq cr6, 0x824711e8
	if ctx.cr[6].eq {
	pc = 0x824711E8; continue 'dispatch;
	}
	// 824711DC: 4BF80195  bl 0x823f1370
	ctx.lr = 0x824711E0;
	sub_823F1370(ctx, base);
	// 824711E0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824711E4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x824711E8; continue 'dispatch;
            }
            0x824711E8 => {
    //   block [0x824711E8..0x82471224)
	// 824711E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824711EC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824711F0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824711F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824711F8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824711FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82471200: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82471204: 419A0020  beq cr6, 0x82471224
	if ctx.cr[6].eq {
	pc = 0x82471224; continue 'dispatch;
	}
	// 82471208: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247120C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82471210: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82471214: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471218: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8247121C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471220: 4BFF2E99  bl 0x824640b8
	ctx.lr = 0x82471224;
	sub_824640B8(ctx, base);
	pc = 0x82471224; continue 'dispatch;
            }
            0x82471224 => {
    //   block [0x82471224..0x82471240)
	// 82471224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8247122C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82471234: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82471238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8247123C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471240 size=48
    let mut pc: u32 = 0x82471240;
    'dispatch: loop {
        match pc {
            0x82471240 => {
    //   block [0x82471240..0x82471250)
	// 82471240: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82471244: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471248: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8247124C: 3963002C  addi r11, r3, 0x2c
	ctx.r[11].s64 = ctx.r[3].s64 + 44;
	pc = 0x82471250; continue 'dispatch;
            }
            0x82471250 => {
    //   block [0x82471250..0x82471270)
	// 82471250: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82471254: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82471258: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8247125C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82471260: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82471264: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82471268: 409AFFE8  bne cr6, 0x82471250
	if !ctx.cr[6].eq {
	pc = 0x82471250; continue 'dispatch;
	}
	// 8247126C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471270 size=16
    let mut pc: u32 = 0x82471270;
    'dispatch: loop {
        match pc {
            0x82471270 => {
    //   block [0x82471270..0x82471280)
	// 82471270: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82471274: 409A000C  bne cr6, 0x82471280
	if !ctx.cr[6].eq {
		sub_82471280(ctx, base);
		return;
	}
	// 82471278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8247127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82471280 size=96
    let mut pc: u32 = 0x82471280;
    'dispatch: loop {
        match pc {
            0x82471280 => {
    //   block [0x82471280..0x824712C4)
	// 82471280: 7C8B1670  srawi r11, r4, 2
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 2) as i64;
	// 82471284: 0CC50000  twi 6, r5, 0
	// 82471288: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8247128C: 7D2B2BD6  divw r9, r11, r5
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[5].s32;
	// 82471290: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82471294: 7D2929D6  mullw r9, r9, r5
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82471298: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8247129C: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824712A0: 7CAA5078  andc r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 & !ctx.r[10].u64;
	// 824712A4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824712A8: 0CAAFFFF  twi 5, r10, -1
	// 824712AC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824712B0: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824712B4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824712B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824712BC: 4098001C  bge cr6, 0x824712d8
	if !ctx.cr[6].lt {
	pc = 0x824712D8; continue 'dispatch;
	}
	// 824712C0: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	pc = 0x824712C4; continue 'dispatch;
            }
            0x824712C4 => {
    //   block [0x824712C4..0x824712D8)
	// 824712C4: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824712C8: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 824712CC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824712D0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824712D4: 4198FFF0  blt cr6, 0x824712c4
	if ctx.cr[6].lt {
	pc = 0x824712C4; continue 'dispatch;
	}
	pc = 0x824712D8; continue 'dispatch;
            }
            0x824712D8 => {
    //   block [0x824712D8..0x824712E0)
	// 824712D8: 7C6B2A14  add r3, r11, r5
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 824712DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824712E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824712E0 size=44
    let mut pc: u32 = 0x824712E0;
    'dispatch: loop {
        match pc {
            0x824712E0 => {
    //   block [0x824712E0..0x8247130C)
	// 824712E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824712E4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 824712E8: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824712EC: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 824712F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 824712F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824712F8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824712FC: 409A0010  bne cr6, 0x8247130c
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8247130C);
		return;
	}
	// 82471300: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471304: 386B8A38  addi r3, r11, -0x75c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30152;
	// 82471308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471328 size=128
    let mut pc: u32 = 0x82471328;
    'dispatch: loop {
        match pc {
            0x82471328 => {
    //   block [0x82471328..0x82471354)
	// 82471328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247132C: 480C3D8D  bl 0x825350b8
	ctx.lr = 0x82471330;
	sub_82535080(ctx, base);
	// 82471330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471334: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82471338: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8247133C: 3BAB5128  addi r29, r11, 0x5128
	ctx.r[29].s64 = ctx.r[11].s64 + 20776;
	// 82471340: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82471344: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247134C: 419A0044  beq cr6, 0x82471390
	if ctx.cr[6].eq {
	pc = 0x82471390; continue 'dispatch;
	}
	// 82471350: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82471354; continue 'dispatch;
            }
            0x82471354 => {
    //   block [0x82471354..0x82471390)
	// 82471354: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247135C: 4BFF8EC5  bl 0x8246a220
	ctx.lr = 0x82471360;
	sub_8246A220(ctx, base);
	// 82471360: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471364: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82471368: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8247136C: 4BFF8C95  bl 0x8246a000
	ctx.lr = 0x82471370;
	sub_8246A000(ctx, base);
	// 82471370: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471374: 419A0028  beq cr6, 0x8247139c
	if ctx.cr[6].eq {
	pc = 0x8247139C; continue 'dispatch;
	}
	// 82471378: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8247137C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471380: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82471384: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471388: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8247138C: 409AFFC8  bne cr6, 0x82471354
	if !ctx.cr[6].eq {
	pc = 0x82471354; continue 'dispatch;
	}
	pc = 0x82471390; continue 'dispatch;
            }
            0x82471390 => {
    //   block [0x82471390..0x8247139C)
	// 82471390: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82471398: 480C3D70  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8247139C => {
    //   block [0x8247139C..0x824713A8)
	// 8247139C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824713A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824713A4: 480C3D64  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824713A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824713A8 size=148
    let mut pc: u32 = 0x824713A8;
    'dispatch: loop {
        match pc {
            0x824713A8 => {
    //   block [0x824713A8..0x82471420)
	// 824713A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824713AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824713B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824713B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824713B8: 4BFF54A1  bl 0x82466858
	ctx.lr = 0x824713BC;
	sub_82466858(ctx, base);
	// 824713BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824713C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824713C4: 388B7468  addi r4, r11, 0x7468
	ctx.r[4].s64 = ctx.r[11].s64 + 29800;
	// 824713C8: 4BFF8C09  bl 0x82469fd0
	ctx.lr = 0x824713CC;
	sub_82469FD0(ctx, base);
	// 824713CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824713D0: 419A0050  beq cr6, 0x82471420
	if ctx.cr[6].eq {
	pc = 0x82471420; continue 'dispatch;
	}
	// 824713D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824713D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824713DC: 388B72F4  addi r4, r11, 0x72f4
	ctx.r[4].s64 = ctx.r[11].s64 + 29428;
	// 824713E0: 4BFF8BF1  bl 0x82469fd0
	ctx.lr = 0x824713E4;
	sub_82469FD0(ctx, base);
	// 824713E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824713E8: 419A0038  beq cr6, 0x82471420
	if ctx.cr[6].eq {
	pc = 0x82471420; continue 'dispatch;
	}
	// 824713EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824713F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824713F4: 388B7898  addi r4, r11, 0x7898
	ctx.r[4].s64 = ctx.r[11].s64 + 30872;
	// 824713F8: 4BFF8BD9  bl 0x82469fd0
	ctx.lr = 0x824713FC;
	sub_82469FD0(ctx, base);
	// 824713FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471400: 419A0020  beq cr6, 0x82471420
	if ctx.cr[6].eq {
	pc = 0x82471420; continue 'dispatch;
	}
	// 82471404: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82471408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247140C: 388B7888  addi r4, r11, 0x7888
	ctx.r[4].s64 = ctx.r[11].s64 + 30856;
	// 82471410: 4BFF8BC1  bl 0x82469fd0
	ctx.lr = 0x82471414;
	sub_82469FD0(ctx, base);
	// 82471414: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471418: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8247141C: 409A0008  bne cr6, 0x82471424
	if !ctx.cr[6].eq {
	pc = 0x82471424; continue 'dispatch;
	}
	pc = 0x82471420; continue 'dispatch;
            }
            0x82471420 => {
    //   block [0x82471420..0x82471424)
	// 82471420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82471424; continue 'dispatch;
            }
            0x82471424 => {
    //   block [0x82471424..0x8247143C)
	// 82471424: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82471428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8247142C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82471434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82471438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471440 size=8
    let mut pc: u32 = 0x82471440;
    'dispatch: loop {
        match pc {
            0x82471440 => {
    //   block [0x82471440..0x82471448)
	// 82471440: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471478 size=116
    let mut pc: u32 = 0x82471478;
    'dispatch: loop {
        match pc {
            0x82471478 => {
    //   block [0x82471478..0x824714A4)
	// 82471478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247147C: 480C3C3D  bl 0x825350b8
	ctx.lr = 0x82471480;
	sub_82535080(ctx, base);
	// 82471480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82471488: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8247148C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82471490: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471494: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471498: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247149C: 40990038  ble cr6, 0x824714d4
	if !ctx.cr[6].gt {
	pc = 0x824714D4; continue 'dispatch;
	}
	// 824714A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x824714A4; continue 'dispatch;
            }
            0x824714A4 => {
    //   block [0x824714A4..0x824714D4)
	// 824714A4: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 824714A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824714AC: 7C7F5A14  add r3, r31, r11
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 824714B0: 4BFF8B21  bl 0x82469fd0
	ctx.lr = 0x824714B4;
	sub_82469FD0(ctx, base);
	// 824714B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824714B8: 419A0028  beq cr6, 0x824714e0
	if ctx.cr[6].eq {
	pc = 0x824714E0; continue 'dispatch;
	}
	// 824714BC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 824714C0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824714C4: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 824714C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824714CC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824714D0: 4198FFD4  blt cr6, 0x824714a4
	if ctx.cr[6].lt {
	pc = 0x824714A4; continue 'dispatch;
	}
	pc = 0x824714D4; continue 'dispatch;
            }
            0x824714D4 => {
    //   block [0x824714D4..0x824714E0)
	// 824714D4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824714D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824714DC: 480C3C2C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x824714E0 => {
    //   block [0x824714E0..0x824714EC)
	// 824714E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824714E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824714E8: 480C3C20  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824714F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824714F0 size=140
    let mut pc: u32 = 0x824714F0;
    'dispatch: loop {
        match pc {
            0x824714F0 => {
    //   block [0x824714F0..0x82471530)
	// 824714F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824714F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824714F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824714FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82471500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82471508: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247150C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471510: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471514: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82471518: 41980018  blt cr6, 0x82471530
	if ctx.cr[6].lt {
	pc = 0x82471530; continue 'dispatch;
	}
	// 8247151C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82471520: 41980010  blt cr6, 0x82471530
	if ctx.cr[6].lt {
	pc = 0x82471530; continue 'dispatch;
	}
	// 82471524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471528: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247152C: 4800002C  b 0x82471558
	pc = 0x82471558; continue 'dispatch;
            }
            0x82471530 => {
    //   block [0x82471530..0x82471558)
	// 82471530: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471534: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247153C: 388B8A74  addi r4, r11, -0x758c
	ctx.r[4].s64 = ctx.r[11].s64 + -30092;
	// 82471540: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82471544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471548: 4E800421  bctrl
	ctx.lr = 0x8247154C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247154C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82471550: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82471554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
            }
            0x82471558 => {
    //   block [0x82471558..0x8247157C)
	// 82471558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247155C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471560: 4E800421  bctrl
	ctx.lr = 0x82471564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82471568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247156C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82471570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82471574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82471578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824715B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824715B8 size=312
    let mut pc: u32 = 0x824715B8;
    'dispatch: loop {
        match pc {
            0x824715B8 => {
    //   block [0x824715B8..0x824715E0)
	// 824715B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824715BC: 480C3AF5  bl 0x825350b0
	ctx.lr = 0x824715C0;
	sub_82535080(ctx, base);
	// 824715C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824715C4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824715C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824715CC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824715D0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824715D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824715D8: 4099010C  ble cr6, 0x824716e4
	if !ctx.cr[6].gt {
	pc = 0x824716E4; continue 'dispatch;
	}
	// 824715DC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	pc = 0x824715E0; continue 'dispatch;
            }
            0x824715E0 => {
    //   block [0x824715E0..0x82471680)
	// 824715E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824715E8: 7FDB5A14  add r30, r27, r11
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 824715EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715F0: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715F4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824715F8: 83A50000  lwz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82471600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471604: 4E800421  bctrl
	ctx.lr = 0x82471608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471608: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8247160C: 419A0080  beq cr6, 0x8247168c
	if ctx.cr[6].eq {
	pc = 0x8247168C; continue 'dispatch;
	}
	// 82471610: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471614: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471618: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8247161C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471620: 48003171  bl 0x82474790
	ctx.lr = 0x82471624;
	sub_82474790(ctx, base);
	// 82471624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82471628: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247162C: 4BFF569D  bl 0x82466cc8
	ctx.lr = 0x82471630;
	sub_82466CC8(ctx, base);
	// 82471630: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471638: 419A0048  beq cr6, 0x82471680
	if ctx.cr[6].eq {
	pc = 0x82471680; continue 'dispatch;
	}
	// 8247163C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471640: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82471644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82471648: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8247164C: 4BFFBEE5  bl 0x8246d530
	ctx.lr = 0x82471650;
	sub_8246D530(ctx, base);
	// 82471650: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471654: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82471658: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8247165C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471660: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471668: 4E800421  bctrl
	ctx.lr = 0x8247166C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247166C: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82471670: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471674: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471678: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8247167C: 48003115  bl 0x82474790
	ctx.lr = 0x82471680;
	sub_82474790(ctx, base);
            }
            0x82471680 => {
    //   block [0x82471680..0x8247168C)
	// 82471680: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82471684: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82471688: 48000050  b 0x824716d8
	pc = 0x824716D8; continue 'dispatch;
            }
            0x8247168C => {
    //   block [0x8247168C..0x824716A8)
	// 8247168C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471690: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82471694: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82471698: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8247169C: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824716A0: 40980038  bge cr6, 0x824716d8
	if !ctx.cr[6].lt {
	pc = 0x824716D8; continue 'dispatch;
	}
	// 824716A4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	pc = 0x824716A8; continue 'dispatch;
            }
            0x824716A8 => {
    //   block [0x824716A8..0x824716D8)
	// 824716A8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824716AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824716B0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824716B4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824716B8: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824716BC: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824716C0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824716C4: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824716C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824716CC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824716D0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824716D4: 4198FFD4  blt cr6, 0x824716a8
	if ctx.cr[6].lt {
	pc = 0x824716A8; continue 'dispatch;
	}
	pc = 0x824716D8; continue 'dispatch;
            }
            0x824716D8 => {
    //   block [0x824716D8..0x824716E4)
	// 824716D8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824716DC: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824716E0: 4198FF00  blt cr6, 0x824715e0
	if ctx.cr[6].lt {
	pc = 0x824715E0; continue 'dispatch;
	}
	pc = 0x824716E4; continue 'dispatch;
            }
            0x824716E4 => {
    //   block [0x824716E4..0x824716F0)
	// 824716E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824716E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824716EC: 480C3A14  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824716F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824716F0 size=52
    let mut pc: u32 = 0x824716F0;
    'dispatch: loop {
        match pc {
            0x824716F0 => {
    //   block [0x824716F0..0x82471704)
	// 824716F0: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824716F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824716F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824716FC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 82471700: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	pc = 0x82471704; continue 'dispatch;
            }
            0x82471704 => {
    //   block [0x82471704..0x82471724)
	// 82471704: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471708: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8247170C: 419A0018  beq cr6, 0x82471724
	if ctx.cr[6].eq {
		sub_82471724(ctx, base);
		return;
	}
	// 82471710: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82471714: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82471718: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8247171C: 4198FFE8  blt cr6, 0x82471704
	if ctx.cr[6].lt {
	pc = 0x82471704; continue 'dispatch;
	}
	// 82471720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471724 size=48
    let mut pc: u32 = 0x82471724;
    'dispatch: loop {
        match pc {
            0x82471724 => {
    //   block [0x82471724..0x82471754)
	// 82471724: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471728: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8247172C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471730: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82471734: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82471738: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247173C: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82471740: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82471744: 7D46582E  lwzx r10, r6, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471748: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8247174C: 7C67482E  lwzx r3, r7, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82471750: 4BFF2B70  b 0x824642c0
	sub_824642C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471758 size=32
    let mut pc: u32 = 0x82471758;
    'dispatch: loop {
        match pc {
            0x82471758 => {
    //   block [0x82471758..0x82471778)
	// 82471758: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8247175C: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471760: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82471764: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82471768: 419A0010  beq cr6, 0x82471778
	if ctx.cr[6].eq {
		sub_82471778(ctx, base);
		return;
	}
	// 8247176C: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471770: 7C655A14  add r3, r5, r11
	ctx.r[3].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82471774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471778 size=8
    let mut pc: u32 = 0x82471778;
    'dispatch: loop {
        match pc {
            0x82471778 => {
    //   block [0x82471778..0x82471780)
	// 82471778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8247177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471780 size=28
    let mut pc: u32 = 0x82471780;
    'dispatch: loop {
        match pc {
            0x82471780 => {
    //   block [0x82471780..0x8247179C)
	// 82471780: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82471784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471788: 419A0014  beq cr6, 0x8247179c
	if ctx.cr[6].eq {
		sub_8247179C(ctx, base);
		return;
	}
	// 8247178C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82471790: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82471794: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82471798: 4BFFBD98  b 0x8246d530
	sub_8246D530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8247179C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8247179C size=44
    let mut pc: u32 = 0x8247179C;
    'dispatch: loop {
        match pc {
            0x8247179C => {
    //   block [0x8247179C..0x824717C8)
	// 8247179C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824717A0: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824717A4: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824717A8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824717AC: 4198001C  blt cr6, 0x824717c8
	if ctx.cr[6].lt {
		sub_824717C8(ctx, base);
		return;
	}
	// 824717B0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824717B4: 41980014  blt cr6, 0x824717c8
	if ctx.cr[6].lt {
		sub_824717C8(ctx, base);
		return;
	}
	// 824717B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824717BC: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824717C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824717C4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824717C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824717C8 size=8
    let mut pc: u32 = 0x824717C8;
    'dispatch: loop {
        match pc {
            0x824717C8 => {
    //   block [0x824717C8..0x824717D0)
	// 824717C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824717CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824717D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824717D0 size=664
    let mut pc: u32 = 0x824717D0;
    'dispatch: loop {
        match pc {
            0x824717D0 => {
    //   block [0x824717D0..0x82471890)
	// 824717D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824717D4: 480C38C1  bl 0x82535094
	ctx.lr = 0x824717D8;
	sub_82535080(ctx, base);
	// 824717D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824717DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824717E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824717E4: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 824717E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824717EC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824717F0: 4098013C  bge cr6, 0x8247192c
	if !ctx.cr[6].lt {
	pc = 0x8247192C; continue 'dispatch;
	}
	// 824717F4: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824717F8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824717FC: 3BCB8A9C  addi r30, r11, -0x7564
	ctx.r[30].s64 = ctx.r[11].s64 + -30052;
	// 82471800: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471808: 4E800421  bctrl
	ctx.lr = 0x8247180C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247180C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82471810: 4BFF87C1  bl 0x82469fd0
	ctx.lr = 0x82471814;
	sub_82469FD0(ctx, base);
	// 82471814: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471818: 419A0114  beq cr6, 0x8247192c
	if ctx.cr[6].eq {
	pc = 0x8247192C; continue 'dispatch;
	}
	// 8247181C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471820: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471824: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82471828: 388B8A8C  addi r4, r11, -0x7574
	ctx.r[4].s64 = ctx.r[11].s64 + -30068;
	// 8247182C: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82471830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471834: 4E800421  bctrl
	ctx.lr = 0x82471838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471838: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8247183C: 41980224  blt cr6, 0x82471a60
	if ctx.cr[6].lt {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471840: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471844: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471848: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 8247184C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471850: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471854: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247185C: 419A0204  beq cr6, 0x82471a60
	if ctx.cr[6].eq {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471860: 813A0024  lwz r9, 0x24(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471864: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471868: 7D0A482E  lwzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8247186C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82471870: 419A01F0  beq cr6, 0x82471a60
	if ctx.cr[6].eq {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471874: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82471878: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8247187C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82471880: 7F080194  addze r24, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[24].s64 = tmp.s64;
	// 82471884: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82471888: 409901D8  ble cr6, 0x82471a60
	if !ctx.cr[6].gt {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 8247188C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
            }
            0x82471890 => {
    //   block [0x82471890..0x824718E8)
	// 82471890: 839B0000  lwz r28, 0(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471894: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82471898: 419A01C8  beq cr6, 0x82471a60
	if ctx.cr[6].eq {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 8247189C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824718A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824718A4: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824718A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824718AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824718B0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824718B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824718B8: 4E800421  bctrl
	ctx.lr = 0x824718BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824718BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824718C0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824718C4: 419A0050  beq cr6, 0x82471914
	if ctx.cr[6].eq {
	pc = 0x82471914; continue 'dispatch;
	}
	// 824718C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824718CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824718D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824718D4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824718D8: 409A0010  bne cr6, 0x824718e8
	if !ctx.cr[6].eq {
	pc = 0x824718E8; continue 'dispatch;
	}
	// 824718DC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 824718E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824718E4: 4BFFCA6D  bl 0x8246e350
	ctx.lr = 0x824718E8;
	sub_8246E350(ctx, base);
            }
            0x824718E8 => {
    //   block [0x824718E8..0x82471914)
	// 824718E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824718EC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824718F0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824718F4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 824718F8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824718FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471900: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471904: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82471908: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8247190C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82471910: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	pc = 0x82471914; continue 'dispatch;
            }
            0x82471914 => {
    //   block [0x82471914..0x8247192C)
	// 82471914: 3B390002  addi r25, r25, 2
	ctx.r[25].s64 = ctx.r[25].s64 + 2;
	// 82471918: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 8247191C: 7F19C000  cmpw cr6, r25, r24
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82471920: 4198FF70  blt cr6, 0x82471890
	if ctx.cr[6].lt {
	pc = 0x82471890; continue 'dispatch;
	}
	// 82471924: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82471928: 480C37BC  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            0x8247192C => {
    //   block [0x8247192C..0x82471950)
	// 8247192C: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471930: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82471934: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471938: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247193C: 40990124  ble cr6, 0x82471a60
	if !ctx.cr[6].gt {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471940: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82471944: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82471948: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8247194C: 3A8B7468  addi r20, r11, 0x7468
	ctx.r[20].s64 = ctx.r[11].s64 + 29800;
	pc = 0x82471950; continue 'dispatch;
            }
            0x82471950 => {
    //   block [0x82471950..0x82471990)
	// 82471950: 815A0024  lwz r10, 0x24(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471954: 7D6AB02E  lwzx r11, r10, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82471958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247195C: 419A00E8  beq cr6, 0x82471a44
	if ctx.cr[6].eq {
	pc = 0x82471A44; continue 'dispatch;
	}
	// 82471960: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471964: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82471968: 7F0AB02E  lwzx r24, r10, r22
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 8247196C: 7F2BAA14  add r25, r11, r21
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 82471970: 81790020  lwz r11, 0x20(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471974: 81590024  lwz r10, 0x24(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471978: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8247197C: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82471980: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82471984: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471988: 408100BC  ble 0x82471a44
	if !ctx.cr[0].gt {
	pc = 0x82471A44; continue 'dispatch;
	}
	// 8247198C: 3B6B0008  addi r27, r11, 8
	ctx.r[27].s64 = ctx.r[11].s64 + 8;
	pc = 0x82471990; continue 'dispatch;
            }
            0x82471990 => {
    //   block [0x82471990..0x824719F4)
	// 82471990: 839BFFF8  lwz r28, -8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471994: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82471998: 419A0088  beq cr6, 0x82471a20
	if ctx.cr[6].eq {
	pc = 0x82471A20; continue 'dispatch;
	}
	// 8247199C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824719A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824719A4: 83DBFFFC  lwz r30, -4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824719A8: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824719AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824719B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824719B4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824719B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824719BC: 4E800421  bctrl
	ctx.lr = 0x824719C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824719C0: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 824719C4: 4BFF860D  bl 0x82469fd0
	ctx.lr = 0x824719C8;
	sub_82469FD0(ctx, base);
	// 824719C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824719CC: 409A0054  bne cr6, 0x82471a20
	if !ctx.cr[6].eq {
	pc = 0x82471A20; continue 'dispatch;
	}
	// 824719D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824719D4: 7F9CC214  add r28, r28, r24
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[24].u64;
	// 824719D8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824719DC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824719E0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824719E4: 409A0010  bne cr6, 0x824719f4
	if !ctx.cr[6].eq {
	pc = 0x824719F4; continue 'dispatch;
	}
	// 824719E8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 824719EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824719F0: 4BFFC961  bl 0x8246e350
	ctx.lr = 0x824719F4;
	sub_8246E350(ctx, base);
            }
            0x824719F4 => {
    //   block [0x824719F4..0x82471A20)
	// 824719F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824719F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824719FC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82471A00: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82471A04: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82471A08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471A0C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471A10: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82471A14: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82471A18: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82471A1C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	pc = 0x82471A20; continue 'dispatch;
            }
            0x82471A20 => {
    //   block [0x82471A20..0x82471A44)
	// 82471A20: 81790024  lwz r11, 0x24(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471A24: 3AF70003  addi r23, r23, 3
	ctx.r[23].s64 = ctx.r[23].s64 + 3;
	// 82471A28: 81590020  lwz r10, 0x20(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471A2C: 3B7B000C  addi r27, r27, 0xc
	ctx.r[27].s64 = ctx.r[27].s64 + 12;
	// 82471A30: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82471A34: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82471A38: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82471A3C: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471A40: 4198FF50  blt cr6, 0x82471990
	if ctx.cr[6].lt {
	pc = 0x82471990; continue 'dispatch;
	}
	pc = 0x82471A44; continue 'dispatch;
            }
            0x82471A44 => {
    //   block [0x82471A44..0x82471A60)
	// 82471A44: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471A48: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82471A4C: 3AB50030  addi r21, r21, 0x30
	ctx.r[21].s64 = ctx.r[21].s64 + 48;
	// 82471A50: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82471A54: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471A58: 7F135800  cmpw cr6, r19, r11
	ctx.cr[6].compare_i32(ctx.r[19].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471A5C: 4198FEF4  blt cr6, 0x82471950
	if ctx.cr[6].lt {
	pc = 0x82471950; continue 'dispatch;
	}
	pc = 0x82471A60; continue 'dispatch;
            }
            0x82471A60 => {
    //   block [0x82471A60..0x82471A68)
	// 82471A60: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82471A64: 480C3680  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471A68 size=308
    let mut pc: u32 = 0x82471A68;
    'dispatch: loop {
        match pc {
            0x82471A68 => {
    //   block [0x82471A68..0x82471ABC)
	// 82471A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471A6C: 480C364D  bl 0x825350b8
	ctx.lr = 0x82471A70;
	sub_82535080(ctx, base);
	// 82471A70: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471A74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82471A78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82471A7C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82471A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82471A84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471A88: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471A90: 4E800421  bctrl
	ctx.lr = 0x82471A94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471A94: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471A98: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82471A9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471AA0: 419A001C  beq cr6, 0x82471abc
	if ctx.cr[6].eq {
	pc = 0x82471ABC; continue 'dispatch;
	}
	// 82471AA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471AA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471AAC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82471AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471AB4: 4E800421  bctrl
	ctx.lr = 0x82471AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471AB8: 48000008  b 0x82471ac0
	pc = 0x82471AC0; continue 'dispatch;
            }
            0x82471ABC => {
    //   block [0x82471ABC..0x82471AC0)
	// 82471ABC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	pc = 0x82471AC0; continue 'dispatch;
            }
            0x82471AC0 => {
    //   block [0x82471AC0..0x82471AF4)
	// 82471AC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82471AC4: 907D0070  stw r3, 0x70(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82471AC8: 409A002C  bne cr6, 0x82471af4
	if !ctx.cr[6].eq {
	pc = 0x82471AF4; continue 'dispatch;
	}
	// 82471ACC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471AD0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82471AD4: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82471AD8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82471ADC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471AE0: 4BFF2789  bl 0x82464268
	ctx.lr = 0x82471AE4;
	sub_82464268(ctx, base);
	// 82471AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82471AE8: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471AEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471AF0: 480023E1  bl 0x82473ed0
	ctx.lr = 0x82471AF4;
	sub_82473ED0(ctx, base);
	pc = 0x82471AF4; continue 'dispatch;
            }
            0x82471AF4 => {
    //   block [0x82471AF4..0x82471B8C)
	// 82471AF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471AF8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82471AFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471B00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471B04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471B0C: 4E800421  bctrl
	ctx.lr = 0x82471B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471B10: 2B030040  cmplwi cr6, r3, 0x40
	ctx.cr[6].compare_u32(ctx.r[3].u32, 64 as u32, &mut ctx.xer);
	// 82471B14: 409A0078  bne cr6, 0x82471b8c
	if !ctx.cr[6].eq {
	pc = 0x82471B8C; continue 'dispatch;
	}
	// 82471B18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82471B1C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82471B20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82471B24: 4BFF8815  bl 0x8246a338
	ctx.lr = 0x82471B28;
	sub_8246A338(ctx, base);
	// 82471B28: 3D6057E0  lis r11, 0x57e0
	ctx.r[11].s64 = 1474297856;
	// 82471B2C: 3D4010C0  lis r10, 0x10c0
	ctx.r[10].s64 = 281018368;
	// 82471B30: 9B810088  stb r28, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u8 ) };
	// 82471B34: 616BE057  ori r11, r11, 0xe057
	ctx.r[11].u64 = ctx.r[11].u64 | 57431;
	// 82471B38: 614AC010  ori r10, r10, 0xc010
	ctx.r[10].u64 = ctx.r[10].u64 | 49168;
	// 82471B3C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82471B40: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82471B44: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471B48: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471B4C: 409A0040  bne cr6, 0x82471b8c
	if !ctx.cr[6].eq {
	pc = 0x82471B8C; continue 'dispatch;
	}
	// 82471B50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471B54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82471B58: 409A0034  bne cr6, 0x82471b8c
	if !ctx.cr[6].eq {
	pc = 0x82471B8C; continue 'dispatch;
	}
	// 82471B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471B60: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471B64: 93FD001C  stw r31, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82471B68: 4BFFF779  bl 0x824712e0
	ctx.lr = 0x82471B6C;
	sub_824712E0(ctx, base);
	// 82471B6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82471B70: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82471B74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82471B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471B7C: 4E800421  bctrl
	ctx.lr = 0x82471B80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471B80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471B84: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82471B88: 480C3580  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82471B8C => {
    //   block [0x82471B8C..0x82471B9C)
	// 82471B8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82471B90: 939D001C  stw r28, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82471B94: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82471B98: 480C3570  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471BA0 size=356
    let mut pc: u32 = 0x82471BA0;
    'dispatch: loop {
        match pc {
            0x82471BA0 => {
    //   block [0x82471BA0..0x82471BF8)
	// 82471BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471BA4: 480C3515  bl 0x825350b8
	ctx.lr = 0x82471BA8;
	sub_82535080(ctx, base);
	// 82471BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471BAC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82471BB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82471BB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82471BB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82471BBC: 409A003C  bne cr6, 0x82471bf8
	if !ctx.cr[6].eq {
	pc = 0x82471BF8; continue 'dispatch;
	}
	// 82471BC0: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471BC4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82471BC8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471BCC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82471BD0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471BD4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82471BD8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471BDC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471BE0: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82471BE4: 4BFF2685  bl 0x82464268
	ctx.lr = 0x82471BE8;
	sub_82464268(ctx, base);
	// 82471BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82471BEC: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471BF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471BF4: 480022DD  bl 0x82473ed0
	ctx.lr = 0x82471BF8;
	sub_82473ED0(ctx, base);
	pc = 0x82471BF8; continue 'dispatch;
            }
            0x82471BF8 => {
    //   block [0x82471BF8..0x82471C6C)
	// 82471BF8: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471C00: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471C04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471C08: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471C0C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471C10: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471C14: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471C18: 557E2036  slwi r30, r11, 4
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82471C1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82471C20: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82471C24: 4E800421  bctrl
	ctx.lr = 0x82471C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471C28: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82471C2C: 409A00CC  bne cr6, 0x82471cf8
	if !ctx.cr[6].eq {
	pc = 0x82471CF8; continue 'dispatch;
	}
	// 82471C30: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471C34: 3BDD0024  addi r30, r29, 0x24
	ctx.r[30].s64 = ctx.r[29].s64 + 36;
	// 82471C38: 93FD0020  stw r31, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82471C3C: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471C40: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471C44: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82471C48: 40990060  ble cr6, 0x82471ca8
	if !ctx.cr[6].gt {
	pc = 0x82471CA8; continue 'dispatch;
	}
	// 82471C4C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471C50: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471C54: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82471C58: 40980024  bge cr6, 0x82471c7c
	if !ctx.cr[6].lt {
	pc = 0x82471C7C; continue 'dispatch;
	}
	// 82471C5C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471C60: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471C64: 41980008  blt cr6, 0x82471c6c
	if ctx.cr[6].lt {
	pc = 0x82471C6C; continue 'dispatch;
	}
	// 82471C68: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
            }
            0x82471C6C => {
    //   block [0x82471C6C..0x82471C7C)
	// 82471C6C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82471C70: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82471C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471C78: 4BFFC651  bl 0x8246e2c8
	ctx.lr = 0x82471C7C;
	sub_8246E2C8(ctx, base);
	pc = 0x82471C7C; continue 'dispatch;
            }
            0x82471C7C => {
    //   block [0x82471C7C..0x82471C90)
	// 82471C7C: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82471C80: 40980028  bge cr6, 0x82471ca8
	if !ctx.cr[6].lt {
	pc = 0x82471CA8; continue 'dispatch;
	}
	// 82471C84: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471C88: 7D7CF850  subf r11, r28, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82471C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x82471C90; continue 'dispatch;
            }
            0x82471C90 => {
    //   block [0x82471C90..0x82471CA8)
	// 82471C90: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471C94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82471C98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471C9C: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82471CA0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82471CA4: 409AFFEC  bne cr6, 0x82471c90
	if !ctx.cr[6].eq {
	pc = 0x82471C90; continue 'dispatch;
	}
	pc = 0x82471CA8; continue 'dispatch;
            }
            0x82471CA8 => {
    //   block [0x82471CA8..0x82471CD0)
	// 82471CA8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82471CAC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471CB0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82471CB4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82471CB8: 40980034  bge cr6, 0x82471cec
	if !ctx.cr[6].lt {
	pc = 0x82471CEC; continue 'dispatch;
	}
	// 82471CBC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471CC0: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471CC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471CC8: 40990024  ble cr6, 0x82471cec
	if !ctx.cr[6].gt {
	pc = 0x82471CEC; continue 'dispatch;
	}
	// 82471CCC: 396B002C  addi r11, r11, 0x2c
	ctx.r[11].s64 = ctx.r[11].s64 + 44;
	pc = 0x82471CD0; continue 'dispatch;
            }
            0x82471CD0 => {
    //   block [0x82471CD0..0x82471CEC)
	// 82471CD0: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82471CD4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82471CD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82471CDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82471CE0: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82471CE4: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82471CE8: 409AFFE8  bne cr6, 0x82471cd0
	if !ctx.cr[6].eq {
	pc = 0x82471CD0; continue 'dispatch;
	}
	pc = 0x82471CEC; continue 'dispatch;
            }
            0x82471CEC => {
    //   block [0x82471CEC..0x82471CF8)
	// 82471CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471CF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82471CF4: 480C3414  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82471CF8 => {
    //   block [0x82471CF8..0x82471D04)
	// 82471CF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82471CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82471D00: 480C3408  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471D08 size=100
    let mut pc: u32 = 0x82471D08;
    'dispatch: loop {
        match pc {
            0x82471D08 => {
    //   block [0x82471D08..0x82471D58)
	// 82471D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471D0C: 480C33B1  bl 0x825350bc
	ctx.lr = 0x82471D10;
	sub_82535080(ctx, base);
	// 82471D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471D14: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82471D18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82471D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471D20: 4BFFB7B9  bl 0x8246d4d8
	ctx.lr = 0x82471D24;
	sub_8246D4D8(ctx, base);
	// 82471D24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471D28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82471D2C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471D30: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82471D34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82471D38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471D3C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82471D40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471D44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471D48: 419A0010  beq cr6, 0x82471d58
	if ctx.cr[6].eq {
	pc = 0x82471D58; continue 'dispatch;
	}
	// 82471D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471D54: 4E800421  bctrl
	ctx.lr = 0x82471D58;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82471D58 => {
    //   block [0x82471D58..0x82471D6C)
	// 82471D58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82471D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471D60: 4BFFB8C1  bl 0x8246d620
	ctx.lr = 0x82471D64;
	sub_8246D620(ctx, base);
	// 82471D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82471D68: 480C33A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82471D70 size=556
    let mut pc: u32 = 0x82471D70;
    'dispatch: loop {
        match pc {
            0x82471D70 => {
    //   block [0x82471D70..0x82471DC0)
	// 82471D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471D74: 480C3329  bl 0x8253509c
	ctx.lr = 0x82471D78;
	sub_82535080(ctx, base);
	// 82471D78: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471D7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82471D80: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82471D84: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471D88: 48008B71  bl 0x8247a8f8
	ctx.lr = 0x82471D8C;
	sub_8247A8F8(ctx, base);
	// 82471D8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471D90: 409A0118  bne cr6, 0x82471ea8
	if !ctx.cr[6].eq {
	pc = 0x82471EA8; continue 'dispatch;
	}
	// 82471D94: 817C0078  lwz r11, 0x78(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(120 as u32) ) } as u64;
	// 82471D98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471D9C: 409A0118  bne cr6, 0x82471eb4
	if !ctx.cr[6].eq {
	pc = 0x82471EB4; continue 'dispatch;
	}
	// 82471DA0: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471DA4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82471DA8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471DAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82471DB0: 409900F8  ble cr6, 0x82471ea8
	if !ctx.cr[6].gt {
	pc = 0x82471EA8; continue 'dispatch;
	}
	// 82471DB4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82471DB8: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82471DBC: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	pc = 0x82471DC0; continue 'dispatch;
            }
            0x82471DC0 => {
    //   block [0x82471DC0..0x82471E00)
	// 82471DC0: 815C0024  lwz r10, 0x24(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471DC4: 7D6AC82E  lwzx r11, r10, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82471DC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471DCC: 419A00C0  beq cr6, 0x82471e8c
	if ctx.cr[6].eq {
	pc = 0x82471E8C; continue 'dispatch;
	}
	// 82471DD0: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471DD4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82471DD8: 7FAAC82E  lwzx r29, r10, r25
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82471DDC: 7F6BC214  add r27, r11, r24
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82471DE0: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471DE4: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471DE8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82471DEC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82471DF0: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82471DF4: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471DF8: 40810094  ble 0x82471e8c
	if !ctx.cr[0].gt {
	pc = 0x82471E8C; continue 'dispatch;
	}
	// 82471DFC: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	pc = 0x82471E00; continue 'dispatch;
            }
            0x82471E00 => {
    //   block [0x82471E00..0x82471E68)
	// 82471E00: 83FEFFF8  lwz r31, -8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471E04: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82471E08: 419A0060  beq cr6, 0x82471e68
	if ctx.cr[6].eq {
	pc = 0x82471E68; continue 'dispatch;
	}
	// 82471E0C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471E10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82471E14: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82471E18: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471E1C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82471E20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471E24: 4E800421  bctrl
	ctx.lr = 0x82471E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471E28: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471E2C: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82471E30: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471E34: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82471E38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471E3C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471E44: 4E800421  bctrl
	ctx.lr = 0x82471E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471E48: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471E4C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82471E50: 419A0018  beq cr6, 0x82471e68
	if ctx.cr[6].eq {
	pc = 0x82471E68; continue 'dispatch;
	}
	// 82471E54: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471E58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471E5C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82471E60: 92AB000C  stw r21, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 82471E64: 4BFFB59D  bl 0x8246d400
	ctx.lr = 0x82471E68;
	sub_8246D400(ctx, base);
            }
            0x82471E68 => {
    //   block [0x82471E68..0x82471E8C)
	// 82471E68: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471E6C: 3B5A0003  addi r26, r26, 3
	ctx.r[26].s64 = ctx.r[26].s64 + 3;
	// 82471E70: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471E74: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82471E78: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82471E7C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82471E80: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82471E84: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471E88: 4198FF78  blt cr6, 0x82471e00
	if ctx.cr[6].lt {
	pc = 0x82471E00; continue 'dispatch;
	}
	pc = 0x82471E8C; continue 'dispatch;
            }
            0x82471E8C => {
    //   block [0x82471E8C..0x82471EA8)
	// 82471E8C: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471E90: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82471E94: 3B180030  addi r24, r24, 0x30
	ctx.r[24].s64 = ctx.r[24].s64 + 48;
	// 82471E98: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82471E9C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471EA0: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471EA4: 4198FF1C  blt cr6, 0x82471dc0
	if ctx.cr[6].lt {
	pc = 0x82471DC0; continue 'dispatch;
	}
	pc = 0x82471EA8; continue 'dispatch;
            }
            0x82471EA8 => {
    //   block [0x82471EA8..0x82471EB4)
	// 82471EA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471EAC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82471EB0: 480C323C  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            0x82471EB4 => {
    //   block [0x82471EB4..0x82471ECC)
	// 82471EB4: 3BAB0028  addi r29, r11, 0x28
	ctx.r[29].s64 = ctx.r[11].s64 + 40;
	// 82471EB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82471EBC: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471EC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82471EC4: 41980024  blt cr6, 0x82471ee8
	if ctx.cr[6].lt {
	pc = 0x82471EE8; continue 'dispatch;
	}
	// 82471EC8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82471ECC; continue 'dispatch;
            }
            0x82471ECC => {
    //   block [0x82471ECC..0x82471EE8)
	// 82471ECC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471ED0: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82471ED4: 409A0014  bne cr6, 0x82471ee8
	if !ctx.cr[6].eq {
	pc = 0x82471EE8; continue 'dispatch;
	}
	// 82471ED8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82471EDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82471EE0: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471EE4: 4099FFE8  ble cr6, 0x82471ecc
	if !ctx.cr[6].gt {
	pc = 0x82471ECC; continue 'dispatch;
	}
	pc = 0x82471EE8; continue 'dispatch;
            }
            0x82471EE8 => {
    //   block [0x82471EE8..0x82471EEC)
	// 82471EE8: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	pc = 0x82471EEC; continue 'dispatch;
            }
            0x82471EEC => {
    //   block [0x82471EEC..0x82471EFC)
	// 82471EEC: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471EF0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82471EF4: 40990008  ble cr6, 0x82471efc
	if !ctx.cr[6].gt {
	pc = 0x82471EFC; continue 'dispatch;
	}
	// 82471EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82471EFC; continue 'dispatch;
            }
            0x82471EFC => {
    //   block [0x82471EFC..0x82471F5C)
	// 82471EFC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82471F00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82471F04: 419AFFA4  beq cr6, 0x82471ea8
	if ctx.cr[6].eq {
	pc = 0x82471EA8; continue 'dispatch;
	}
	// 82471F08: 7D49F214  add r10, r9, r30
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82471F0C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F10: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82471F14: 81370000  lwz r9, 0(r23)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82471F1C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82471F20: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471F24: 7FE8582E  lwzx r31, r8, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471F28: 81290010  lwz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471F2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471F30: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471F34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82471F38: 4E800421  bctrl
	ctx.lr = 0x82471F3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471F3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471F40: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82471F44: 419A0018  beq cr6, 0x82471f5c
	if ctx.cr[6].eq {
	pc = 0x82471F5C; continue 'dispatch;
	}
	// 82471F48: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471F4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471F50: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82471F54: 92AB000C  stw r21, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 82471F58: 4BFFB4A9  bl 0x8246d400
	ctx.lr = 0x82471F5C;
	sub_8246D400(ctx, base);
            }
            0x82471F5C => {
    //   block [0x82471F5C..0x82471F78)
	// 82471F5C: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471F60: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82471F64: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471F68: 4199002C  bgt cr6, 0x82471f94
	if ctx.cr[6].gt {
	pc = 0x82471F94; continue 'dispatch;
	}
	// 82471F6C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F70: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471F74: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	pc = 0x82471F78; continue 'dispatch;
            }
            0x82471F78 => {
    //   block [0x82471F78..0x82471F94)
	// 82471F78: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F7C: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82471F80: 409A0014  bne cr6, 0x82471f94
	if !ctx.cr[6].eq {
	pc = 0x82471F94; continue 'dispatch;
	}
	// 82471F84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82471F88: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82471F8C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471F90: 4099FFE8  ble cr6, 0x82471f78
	if !ctx.cr[6].gt {
	pc = 0x82471F78; continue 'dispatch;
	}
	pc = 0x82471F94; continue 'dispatch;
            }
            0x82471F94 => {
    //   block [0x82471F94..0x82471F9C)
	// 82471F94: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82471F98: 4BFFFF54  b 0x82471eec
	pc = 0x82471EEC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471FA0 size=376
    let mut pc: u32 = 0x82471FA0;
    'dispatch: loop {
        match pc {
            0x82471FA0 => {
    //   block [0x82471FA0..0x82471FE0)
	// 82471FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471FA4: 480C3109  bl 0x825350ac
	ctx.lr = 0x82471FA8;
	sub_82535080(ctx, base);
	// 82471FA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471FAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82471FB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82471FB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82471FB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82471FBC: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471FC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471FC4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471FC8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471FCC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471FD0: 409A0010  bne cr6, 0x82471fe0
	if !ctx.cr[6].eq {
	pc = 0x82471FE0; continue 'dispatch;
	}
	// 82471FD4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82471FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471FDC: 4BFFC375  bl 0x8246e350
	ctx.lr = 0x82471FE0;
	sub_8246E350(ctx, base);
	pc = 0x82471FE0; continue 'dispatch;
            }
            0x82471FE0 => {
    //   block [0x82471FE0..0x82472028)
	// 82471FE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471FE4: 3B3C000C  addi r25, r28, 0xc
	ctx.r[25].s64 = ctx.r[28].s64 + 12;
	// 82471FE8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471FEC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82471FF0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471FF4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82471FF8: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82471FFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472000: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82472004: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82472008: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8247200C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82472010: 4BFFB3F1  bl 0x8246d400
	ctx.lr = 0x82472014;
	sub_8246D400(ctx, base);
	// 82472014: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472018: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8247201C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472020: 409900EC  ble cr6, 0x8247210c
	if !ctx.cr[6].gt {
	pc = 0x8247210C; continue 'dispatch;
	}
	// 82472024: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	pc = 0x82472028; continue 'dispatch;
            }
            0x82472028 => {
    //   block [0x82472028..0x82472060)
	// 82472028: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247202C: 7FFA5A14  add r31, r26, r11
	ctx.r[31].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82472030: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472034: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472038: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8247203C: 419A0078  beq cr6, 0x824720b4
	if ctx.cr[6].eq {
	pc = 0x824720B4; continue 'dispatch;
	}
	// 82472040: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472044: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82472048: 4BFFB491  bl 0x8246d4d8
	ctx.lr = 0x8247204C;
	sub_8246D4D8(ctx, base);
	// 8247204C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82472050: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472054: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82472058: 40990008  ble cr6, 0x82472060
	if !ctx.cr[6].gt {
	pc = 0x82472060; continue 'dispatch;
	}
	// 8247205C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82472060; continue 'dispatch;
            }
            0x82472060 => {
    //   block [0x82472060..0x824720B4)
	// 82472060: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82472064: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472068: 409A004C  bne cr6, 0x824720b4
	if !ctx.cr[6].eq {
	pc = 0x824720B4; continue 'dispatch;
	}
	// 8247206C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472070: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472074: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82472078: 4BFFB4B9  bl 0x8246d530
	ctx.lr = 0x8247207C;
	sub_8246D530(ctx, base);
	// 8247207C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82472080: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82472084: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472088: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8247208C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82472090: 4BFFB371  bl 0x8246d400
	ctx.lr = 0x82472094;
	sub_8246D400(ctx, base);
	// 82472094: 80DC0020  lwz r6, 0x20(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472098: 80BC001C  lwz r5, 0x1c(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247209C: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 824720A0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720A4: 480026ED  bl 0x82474790
	ctx.lr = 0x824720A8;
	sub_82474790(ctx, base);
	// 824720A8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824720AC: 3B5A0008  addi r26, r26, 8
	ctx.r[26].s64 = ctx.r[26].s64 + 8;
	// 824720B0: 48000050  b 0x82472100
	pc = 0x82472100; continue 'dispatch;
            }
            0x824720B4 => {
    //   block [0x824720B4..0x824720D0)
	// 824720B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720B8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 824720BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824720C0: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824720C4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824720C8: 40980038  bge cr6, 0x82472100
	if !ctx.cr[6].lt {
	pc = 0x82472100; continue 'dispatch;
	}
	// 824720CC: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	pc = 0x824720D0; continue 'dispatch;
            }
            0x824720D0 => {
    //   block [0x824720D0..0x82472100)
	// 824720D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824720D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824720D8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824720DC: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824720E0: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824720E4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824720E8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824720EC: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720F0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824720F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824720FC: 4198FFD4  blt cr6, 0x824720d0
	if ctx.cr[6].lt {
	pc = 0x824720D0; continue 'dispatch;
	}
	pc = 0x82472100; continue 'dispatch;
            }
            0x82472100 => {
    //   block [0x82472100..0x8247210C)
	// 82472100: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472104: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472108: 4198FF20  blt cr6, 0x82472028
	if ctx.cr[6].lt {
	pc = 0x82472028; continue 'dispatch;
	}
	pc = 0x8247210C; continue 'dispatch;
            }
            0x8247210C => {
    //   block [0x8247210C..0x82472118)
	// 8247210C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82472110: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82472114: 480C2FE8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82472118 size=192
    let mut pc: u32 = 0x82472118;
    'dispatch: loop {
        match pc {
            0x82472118 => {
    //   block [0x82472118..0x824721D8)
	// 82472118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82472120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82472124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82472128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247212C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82472130: 480026B9  bl 0x824747e8
	ctx.lr = 0x82472134;
	sub_824747E8(ctx, base);
	// 82472134: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82472138: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247213C: 395F0024  addi r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 + 36;
	// 82472140: 392B8AB4  addi r9, r11, -0x754c
	ctx.r[9].s64 = ctx.r[11].s64 + -30028;
	// 82472144: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82472148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247214C: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 82472150: 60E70010  ori r7, r7, 0x10
	ctx.r[7].u64 = ctx.r[7].u64 | 16;
	// 82472154: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82472158: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8247215C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82472160: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82472164: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82472168: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8247216C: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82472170: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82472174: 90EA0008  stw r7, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82472178: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8247217C: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82472180: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82472184: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82472188: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8247218C: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82472190: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82472194: 7C66402E  lwzx r3, r6, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82472198: 4BFF1EA1  bl 0x82464038
	ctx.lr = 0x8247219C;
	sub_82464038(ctx, base);
	// 8247219C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824721A0: 3960004C  li r11, 0x4c
	ctx.r[11].s64 = 76;
	// 824721A4: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824721A8: 48008891  bl 0x8247aa38
	ctx.lr = 0x824721AC;
	sub_8247AA38(ctx, base);
	// 824721AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824721B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824721B4: 396B89FC  addi r11, r11, -0x7604
	ctx.r[11].s64 = ctx.r[11].s64 + -30212;
	// 824721B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824721BC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 824721C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824721C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824721C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824721CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824721D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824721D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824721D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824721D8 size=260
    let mut pc: u32 = 0x824721D8;
    'dispatch: loop {
        match pc {
            0x824721D8 => {
    //   block [0x824721D8..0x82472218)
	// 824721D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824721DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824721E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824721E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824721E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824721EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824721F0: 396B8AB4  addi r11, r11, -0x754c
	ctx.r[11].s64 = ctx.r[11].s64 + -30028;
	// 824721F4: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 824721F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824721FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82472200: 419A0018  beq cr6, 0x82472218
	if ctx.cr[6].eq {
	pc = 0x82472218; continue 'dispatch;
	}
	// 82472204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8247220C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472214: 4E800421  bctrl
	ctx.lr = 0x82472218;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82472218 => {
    //   block [0x82472218..0x8247222C)
	// 82472218: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8247221C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82472220: 419A000C  beq cr6, 0x8247222c
	if ctx.cr[6].eq {
	pc = 0x8247222C; continue 'dispatch;
	}
	// 82472224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82472228: 4800E2A9  bl 0x824804d0
	ctx.lr = 0x8247222C;
	sub_824804D0(ctx, base);
	pc = 0x8247222C; continue 'dispatch;
            }
            0x8247222C => {
    //   block [0x8247222C..0x82472268)
	// 8247222C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472230: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472234: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472238: 419A0030  beq cr6, 0x82472268
	if ctx.cr[6].eq {
	pc = 0x82472268; continue 'dispatch;
	}
	// 8247223C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472240: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82472244: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82472248: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247224C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82472250: 409A0018  bne cr6, 0x82472268
	if !ctx.cr[6].eq {
	pc = 0x82472268; continue 'dispatch;
	}
	// 82472254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472258: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8247225C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472264: 4E800421  bctrl
	ctx.lr = 0x82472268;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82472268 => {
    //   block [0x82472268..0x82472294)
	// 82472268: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8247226C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82472270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472274: 409A0020  bne cr6, 0x82472294
	if !ctx.cr[6].eq {
	pc = 0x82472294; continue 'dispatch;
	}
	// 82472278: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247227C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82472280: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82472284: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82472288: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8247228C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472290: 4BFF1E29  bl 0x824640b8
	ctx.lr = 0x82472294;
	sub_824640B8(ctx, base);
	pc = 0x82472294; continue 'dispatch;
            }
            0x82472294 => {
    //   block [0x82472294..0x824722C0)
	// 82472294: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82472298: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8247229C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824722A0: 409A0020  bne cr6, 0x824722c0
	if !ctx.cr[6].eq {
	pc = 0x824722C0; continue 'dispatch;
	}
	// 824722A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824722A8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824722AC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824722B0: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824722B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824722B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824722BC: 4BFF1DFD  bl 0x824640b8
	ctx.lr = 0x824722C0;
	sub_824640B8(ctx, base);
	pc = 0x824722C0; continue 'dispatch;
            }
            0x824722C0 => {
    //   block [0x824722C0..0x824722DC)
	// 824722C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824722C4: 48002575  bl 0x82474838
	ctx.lr = 0x824722C8;
	sub_82474838(ctx, base);
	// 824722C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824722CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824722D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824722D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824722D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824722E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824722E0 size=696
    let mut pc: u32 = 0x824722E0;
    'dispatch: loop {
        match pc {
            0x824722E0 => {
    //   block [0x824722E0..0x82472348)
	// 824722E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824722E4: 480C2DC5  bl 0x825350a8
	ctx.lr = 0x824722E8;
	sub_82535080(ctx, base);
	// 824722E8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824722EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824722F0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 824722F4: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824722F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824722FC: 409A00DC  bne cr6, 0x824723d8
	if !ctx.cr[6].eq {
	pc = 0x824723D8; continue 'dispatch;
	}
	// 82472300: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82472304: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82472308: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8247230C: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82472310: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82472314: 4BFFF4BD  bl 0x824717d0
	ctx.lr = 0x82472318;
	sub_824717D0(ctx, base);
	// 82472318: 3BDF007C  addi r30, r31, 0x7c
	ctx.r[30].s64 = ctx.r[31].s64 + 124;
	// 8247231C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472320: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472324: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82472328: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 8247232C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82472330: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82472334: 40980024  bge cr6, 0x82472358
	if !ctx.cr[6].lt {
	pc = 0x82472358; continue 'dispatch;
	}
	// 82472338: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247233C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472340: 41980008  blt cr6, 0x82472348
	if ctx.cr[6].lt {
	pc = 0x82472348; continue 'dispatch;
	}
	// 82472344: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82472348; continue 'dispatch;
            }
            0x82472348 => {
    //   block [0x82472348..0x82472358)
	// 82472348: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8247234C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82472350: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82472354: 4BFFBF75  bl 0x8246e2c8
	ctx.lr = 0x82472358;
	sub_8246E2C8(ctx, base);
	pc = 0x82472358; continue 'dispatch;
            }
            0x82472358 => {
    //   block [0x82472358..0x8247237C)
	// 82472358: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247235C: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82472360: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82472364: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82472368: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8247236C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472370: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472374: 4099002C  ble cr6, 0x824723a0
	if !ctx.cr[6].gt {
	pc = 0x824723A0; continue 'dispatch;
	}
	// 82472378: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	pc = 0x8247237C; continue 'dispatch;
            }
            0x8247237C => {
    //   block [0x8247237C..0x824723A0)
	// 8247237C: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82472380: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82472384: 7D0A402E  lwzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82472388: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 8247238C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82472390: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82472394: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472398: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8247239C: 4198FFE0  blt cr6, 0x8247237c
	if ctx.cr[6].lt {
	pc = 0x8247237C; continue 'dispatch;
	}
	pc = 0x824723A0; continue 'dispatch;
            }
            0x824723A0 => {
    //   block [0x824723A0..0x824723D8)
	// 824723A0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824723A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824723A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824723AC: 409A002C  bne cr6, 0x824723d8
	if !ctx.cr[6].eq {
	pc = 0x824723D8; continue 'dispatch;
	}
	// 824723B0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824723B4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824723B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824723BC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824723C0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824723C4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824723C8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824723CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824723D0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824723D4: 4BFF1CE5  bl 0x824640b8
	ctx.lr = 0x824723D8;
	sub_824640B8(ctx, base);
	pc = 0x824723D8; continue 'dispatch;
            }
            0x824723D8 => {
    //   block [0x824723D8..0x82472454)
	// 824723D8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824723DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824723E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824723E4: 83CB92F0  lwz r30, -0x6d10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27920 as u32) ) } as u64;
	// 824723E8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 824723EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824723F0: 4E800421  bctrl
	ctx.lr = 0x824723F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824723F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824723F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824723FC: 48007E7D  bl 0x8247a278
	ctx.lr = 0x82472400;
	sub_8247A278(ctx, base);
	// 82472400: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82472404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472408: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247240C: 419A0168  beq cr6, 0x82472574
	if ctx.cr[6].eq {
	pc = 0x82472574; continue 'dispatch;
	}
	// 82472410: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472414: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82472418: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8247241C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82472420: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472424: 4BFF1C15  bl 0x82464038
	ctx.lr = 0x82472428;
	sub_82464038(ctx, base);
	// 82472428: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8247242C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472430: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82472434: 48008CE5  bl 0x8247b118
	ctx.lr = 0x82472438;
	sub_8247B118(ctx, base);
	// 82472438: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8247243C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82472440: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82472444: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472448: 40990078  ble cr6, 0x824724c0
	if !ctx.cr[6].gt {
	pc = 0x824724C0; continue 'dispatch;
	}
	// 8247244C: 3B3F0008  addi r25, r31, 8
	ctx.r[25].s64 = ctx.r[31].s64 + 8;
	// 82472450: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
            }
            0x82472454 => {
    //   block [0x82472454..0x824724C0)
	// 82472454: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82472458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247245C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472460: 7FDD582E  lwzx r30, r29, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472464: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247246C: 4E800421  bctrl
	ctx.lr = 0x82472470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472470: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472474: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82472478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247247C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82472480: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82472484: 4800230D  bl 0x82474790
	ctx.lr = 0x82472488;
	sub_82474790(ctx, base);
	// 82472488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247248C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472490: 4BFF43C9  bl 0x82466858
	ctx.lr = 0x82472494;
	sub_82466858(ctx, base);
	// 82472494: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82472498: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247249C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824724A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824724A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824724A8: 4E800421  bctrl
	ctx.lr = 0x824724AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824724AC: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824724B0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824724B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824724B8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824724BC: 4198FF98  blt cr6, 0x82472454
	if ctx.cr[6].lt {
	pc = 0x82472454; continue 'dispatch;
	}
            }
            0x824724C0 => {
    //   block [0x824724C0..0x8247251C)
	// 824724C0: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824724C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824724C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 824724CC: 93010070  stw r24, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[24].u32 ) };
	// 824724D0: 3D208293  lis r9, -0x7d6d
	ctx.r[9].s64 = -2104295424;
	// 824724D4: 3D008293  lis r8, -0x7d6d
	ctx.r[8].s64 = -2104295424;
	// 824724D8: 396B8E24  addi r11, r11, -0x71dc
	ctx.r[11].s64 = ctx.r[11].s64 + -29148;
	// 824724DC: 394A8DC4  addi r10, r10, -0x723c
	ctx.r[10].s64 = ctx.r[10].s64 + -29244;
	// 824724E0: 80E7000C  lwz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 824724E4: 39298F14  addi r9, r9, -0x70ec
	ctx.r[9].s64 = ctx.r[9].s64 + -28908;
	// 824724E8: 39088EE4  addi r8, r8, -0x711c
	ctx.r[8].s64 = ctx.r[8].s64 + -28956;
	// 824724EC: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 824724F0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824724F4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824724F8: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 824724FC: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82472500: 409A001C  bne cr6, 0x8247251c
	if !ctx.cr[6].eq {
	pc = 0x8247251C; continue 'dispatch;
	}
	// 82472504: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82472508: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8247250C: 396B3A78  addi r11, r11, 0x3a78
	ctx.r[11].s64 = ctx.r[11].s64 + 14968;
	// 82472510: 394A3AD8  addi r10, r10, 0x3ad8
	ctx.r[10].s64 = ctx.r[10].s64 + 15064;
	// 82472514: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82472518: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	pc = 0x8247251C; continue 'dispatch;
            }
            0x8247251C => {
    //   block [0x8247251C..0x8247252C)
	// 8247251C: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82472520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472524: 419A0044  beq cr6, 0x82472568
	if ctx.cr[6].eq {
	pc = 0x82472568; continue 'dispatch;
	}
	// 82472528: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	pc = 0x8247252C; continue 'dispatch;
            }
            0x8247252C => {
    //   block [0x8247252C..0x82472568)
	// 8247252C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472530: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472534: 4BFF4325  bl 0x82466858
	ctx.lr = 0x82472538;
	sub_82466858(ctx, base);
	// 82472538: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8247253C: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472540: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82472544: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82472548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247254C: 4E800421  bctrl
	ctx.lr = 0x82472550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472550: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82472554: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82472558: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8247255C: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472560: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82472564: 409AFFC8  bne cr6, 0x8247252c
	if !ctx.cr[6].eq {
	pc = 0x8247252C; continue 'dispatch;
	}
            }
            0x82472568 => {
    //   block [0x82472568..0x82472574)
	// 82472568: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8247256C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82472570: 480C2B88  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82472574 => {
    //   block [0x82472574..0x82472590)
	// 82472574: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247257C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472580: 419A0010  beq cr6, 0x82472590
	if ctx.cr[6].eq {
	pc = 0x82472590; continue 'dispatch;
	}
	// 82472584: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472588: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8247258C: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82472590; continue 'dispatch;
            }
            0x82472590 => {
    //   block [0x82472590..0x82472598)
	// 82472590: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82472594: 480C2B64  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82472598 size=828
    let mut pc: u32 = 0x82472598;
    'dispatch: loop {
        match pc {
            0x82472598 => {
    //   block [0x82472598..0x82472604)
	// 82472598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247259C: 480C2AF9  bl 0x82535094
	ctx.lr = 0x824725A0;
	sub_82535080(ctx, base);
	// 824725A0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824725A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824725A8: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824725AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824725B0: 409A0318  bne cr6, 0x824728c8
	if !ctx.cr[6].eq {
	pc = 0x824728C8; continue 'dispatch;
	}
	// 824725B4: 4BFFFD2D  bl 0x824722e0
	ctx.lr = 0x824725B8;
	sub_824722E0(ctx, base);
	// 824725B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824725BC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 824725C0: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 824725C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824725C8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824725CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824725D0: 4E800421  bctrl
	ctx.lr = 0x824725D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824725D4: 4BFFED55  bl 0x82471328
	ctx.lr = 0x824725D8;
	sub_82471328(ctx, base);
	// 824725D8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 824725DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824725E0: 419A0024  beq cr6, 0x82472604
	if ctx.cr[6].eq {
	pc = 0x82472604; continue 'dispatch;
	}
	// 824725E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824725E8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824725EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824725F0: 388B8AF0  addi r4, r11, -0x7510
	ctx.r[4].s64 = ctx.r[11].s64 + -29968;
	// 824725F4: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 824725F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824725FC: 4E800421  bctrl
	ctx.lr = 0x82472600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472600: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
            }
            0x82472604 => {
    //   block [0x82472604..0x8247262C)
	// 82472604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82472608: 4BFFAD39  bl 0x8246d340
	ctx.lr = 0x8247260C;
	sub_8246D340(ctx, base);
	// 8247260C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472610: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82472614: 7E749B78  mr r20, r19
	ctx.r[20].u64 = ctx.r[19].u64;
	// 82472618: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8247261C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472620: 409900F8  ble cr6, 0x82472718
	if !ctx.cr[6].gt {
	pc = 0x82472718; continue 'dispatch;
	}
	// 82472624: 7E789B78  mr r24, r19
	ctx.r[24].u64 = ctx.r[19].u64;
	// 82472628: 7E759B78  mr r21, r19
	ctx.r[21].u64 = ctx.r[19].u64;
	pc = 0x8247262C; continue 'dispatch;
            }
            0x8247262C => {
    //   block [0x8247262C..0x8247266C)
	// 8247262C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472630: 7D78502E  lwzx r11, r24, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472638: 419A00C4  beq cr6, 0x824726fc
	if ctx.cr[6].eq {
	pc = 0x824726FC; continue 'dispatch;
	}
	// 8247263C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472640: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82472644: 7F58502E  lwzx r26, r24, r10
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472648: 7F6BAA14  add r27, r11, r21
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 8247264C: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472650: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472654: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82472658: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8247265C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82472660: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472664: 40810098  ble 0x824726fc
	if !ctx.cr[0].gt {
	pc = 0x824726FC; continue 'dispatch;
	}
	// 82472668: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	pc = 0x8247266C; continue 'dispatch;
            }
            0x8247266C => {
    //   block [0x8247266C..0x824726D8)
	// 8247266C: 833EFFF8  lwz r25, -8(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82472670: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 82472674: 419A0064  beq cr6, 0x824726d8
	if ctx.cr[6].eq {
	pc = 0x824726D8; continue 'dispatch;
	}
	// 82472678: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247267C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472680: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82472684: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472688: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247268C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472690: 4E800421  bctrl
	ctx.lr = 0x82472694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472694: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472698: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8247269C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824726A0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824726A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824726A8: 4E800421  bctrl
	ctx.lr = 0x824726AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824726AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824726B0: 7F16E800  cmpw cr6, r22, r29
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824726B4: 419A0024  beq cr6, 0x824726d8
	if ctx.cr[6].eq {
	pc = 0x824726D8; continue 'dispatch;
	}
	// 824726B8: 4BFFECF1  bl 0x824713a8
	ctx.lr = 0x824726BC;
	sub_824713A8(ctx, base);
	// 824726BC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 824726C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824726C4: 419A0014  beq cr6, 0x824726d8
	if ctx.cr[6].eq {
	pc = 0x824726D8; continue 'dispatch;
	}
	// 824726C8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824726CC: 7C99D214  add r4, r25, r26
	ctx.r[4].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 824726D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824726D4: 4BFFAD2D  bl 0x8246d400
	ctx.lr = 0x824726D8;
	sub_8246D400(ctx, base);
            }
            0x824726D8 => {
    //   block [0x824726D8..0x824726FC)
	// 824726D8: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 824726DC: 3BBD0003  addi r29, r29, 3
	ctx.r[29].s64 = ctx.r[29].s64 + 3;
	// 824726E0: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824726E4: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 824726E8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824726EC: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824726F0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824726F4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824726F8: 4198FF74  blt cr6, 0x8247266c
	if ctx.cr[6].lt {
	pc = 0x8247266C; continue 'dispatch;
	}
	pc = 0x824726FC; continue 'dispatch;
            }
            0x824726FC => {
    //   block [0x824726FC..0x82472718)
	// 824726FC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472700: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82472704: 3AB50030  addi r21, r21, 0x30
	ctx.r[21].s64 = ctx.r[21].s64 + 48;
	// 82472708: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 8247270C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82472710: 7F145800  cmpw cr6, r20, r11
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472714: 4198FF18  blt cr6, 0x8247262c
	if ctx.cr[6].lt {
	pc = 0x8247262C; continue 'dispatch;
	}
	pc = 0x82472718; continue 'dispatch;
            }
            0x82472718 => {
    //   block [0x82472718..0x82472748)
	// 82472718: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247271C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82472720: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472724: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82472728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247272C: 419A001C  beq cr6, 0x82472748
	if ctx.cr[6].eq {
	pc = 0x82472748; continue 'dispatch;
	}
	// 82472730: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82472734: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82472738: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8247273C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472740: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82472744: 48000010  b 0x82472754
	pc = 0x82472754; continue 'dispatch;
            }
            0x82472748 => {
    //   block [0x82472748..0x82472754)
	// 82472748: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8247274C: 4BFF16F5  bl 0x82463e40
	ctx.lr = 0x82472750;
	sub_82463E40(ctx, base);
	// 82472750: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82472754; continue 'dispatch;
            }
            0x82472754 => {
    //   block [0x82472754..0x82472770)
	// 82472754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472758: 419A0018  beq cr6, 0x82472770
	if ctx.cr[6].eq {
	pc = 0x82472770; continue 'dispatch;
	}
	// 8247275C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82472760: 926B0000  stw r19, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 82472764: 926B0004  stw r19, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 82472768: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8247276C: 48000008  b 0x82472774
	pc = 0x82472774; continue 'dispatch;
            }
            0x82472770 => {
    //   block [0x82472770..0x82472774)
	// 82472770: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	pc = 0x82472774; continue 'dispatch;
            }
            0x82472774 => {
    //   block [0x82472774..0x824727F8)
	// 82472774: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247277C: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82472780: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472784: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82472788: 4E800421  bctrl
	ctx.lr = 0x8247278C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247278C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82472790: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472798: 396B8A84  addi r11, r11, -0x757c
	ctx.r[11].s64 = ctx.r[11].s64 + -30076;
	// 8247279C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 824727A0: 83AA000C  lwz r29, 0xc(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824727A4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824727A8: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824727AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824727B0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824727B4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824727B8: 4BFFAB89  bl 0x8246d340
	ctx.lr = 0x824727BC;
	sub_8246D340(ctx, base);
	// 824727BC: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 824727C0: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824727C4: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 824727C8: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 824727CC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824727D0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824727D4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824727D8: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 824727DC: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824727E0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824727E4: 40980020  bge cr6, 0x82472804
	if !ctx.cr[6].lt {
	pc = 0x82472804; continue 'dispatch;
	}
	// 824727E8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824727EC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824727F0: 40980008  bge cr6, 0x824727f8
	if !ctx.cr[6].lt {
	pc = 0x824727F8; continue 'dispatch;
	}
	// 824727F4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
            }
            0x824727F8 => {
    //   block [0x824727F8..0x82472804)
	// 824727F8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824727FC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82472800: 4BFFBAC9  bl 0x8246e2c8
	ctx.lr = 0x82472804;
	sub_8246E2C8(ctx, base);
	pc = 0x82472804; continue 'dispatch;
            }
            0x82472804 => {
    //   block [0x82472804..0x8247283C)
	// 82472804: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472808: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8247280C: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472810: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82472814: 41980028  blt cr6, 0x8247283c
	if ctx.cr[6].lt {
	pc = 0x8247283C; continue 'dispatch;
	}
	// 82472818: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8247281C: 41980020  blt cr6, 0x8247283c
	if ctx.cr[6].lt {
	pc = 0x8247283C; continue 'dispatch;
	}
	// 82472820: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472828: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247282C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472830: 4E800421  bctrl
	ctx.lr = 0x82472834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472834: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82472838: 48000008  b 0x82472840
	pc = 0x82472840; continue 'dispatch;
            }
            0x8247283C => {
    //   block [0x8247283C..0x82472840)
	// 8247283C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	pc = 0x82472840; continue 'dispatch;
            }
            0x82472840 => {
    //   block [0x82472840..0x824728AC)
	// 82472840: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472844: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82472848: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8247284C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472850: 4E800421  bctrl
	ctx.lr = 0x82472854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472854: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247285C: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82472860: 4BFFEC91  bl 0x824714f0
	ctx.lr = 0x82472864;
	sub_824714F0(ctx, base);
	// 82472864: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472868: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8247286C: 48008E65  bl 0x8247b6d0
	ctx.lr = 0x82472870;
	sub_8247B6D0(ctx, base);
	// 82472870: A1770004  lhz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472874: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472878: 419A0034  beq cr6, 0x824728ac
	if ctx.cr[6].eq {
	pc = 0x824728AC; continue 'dispatch;
	}
	// 8247287C: A1770006  lhz r11, 6(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472880: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82472884: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82472888: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247288C: B1770006  sth r11, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82472890: 409A001C  bne cr6, 0x824728ac
	if !ctx.cr[6].eq {
	pc = 0x824728AC; continue 'dispatch;
	}
	// 82472894: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472898: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8247289C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824728A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824728A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824728A8: 4E800421  bctrl
	ctx.lr = 0x824728AC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824728AC => {
    //   block [0x824728AC..0x824728C8)
	// 824728AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 824728B0: 4BFFAB19  bl 0x8246d3c8
	ctx.lr = 0x824728B4;
	sub_8246D3C8(ctx, base);
	// 824728B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824728B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824728BC: 396B8A18  addi r11, r11, -0x75e8
	ctx.r[11].s64 = ctx.r[11].s64 + -30184;
	// 824728C0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824728C4: 4BFFAB05  bl 0x8246d3c8
	ctx.lr = 0x824728C8;
	sub_8246D3C8(ctx, base);
	pc = 0x824728C8; continue 'dispatch;
            }
            0x824728C8 => {
    //   block [0x824728C8..0x824728D4)
	// 824728C8: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824728CC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824728D0: 480C2814  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


