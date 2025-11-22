pub fn sub_82467F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467F68 size=112
    let mut pc: u32 = 0x82467F68;
    'dispatch: loop {
        match pc {
            0x82467F68 => {
    //   block [0x82467F68..0x82467FD8)
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
    //   block [0x82467FE8..0x82468088)
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
    //   block [0x824680B0..0x82468120)
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
    //   block [0x824681C8..0x8246824C)
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
	// 824681F0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824681F4: 3BCB830C  addi r30, r11, -0x7cf4
	ctx.r[30].s64 = ctx.r[11].s64 + -31988;
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
    //   block [0x824682A0..0x82468308)
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


pub fn sub_82468678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468678 size=20
    let mut pc: u32 = 0x82468678;
    'dispatch: loop {
        match pc {
            0x82468678 => {
    //   block [0x82468678..0x8246868C)
	// 82468678: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246867C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468680: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82468684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468688: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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
    // ---- function 0x82468748 size=32
    let mut pc: u32 = 0x82468748;
    'dispatch: loop {
        match pc {
            0x82468748 => {
    //   block [0x82468748..0x82468768)
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468768 size=12
    let mut pc: u32 = 0x82468768;
    'dispatch: loop {
        match pc {
            0x82468768 => {
    //   block [0x82468768..0x82468774)
	// 82468768: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246876C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82468770: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468774 size=16
    let mut pc: u32 = 0x82468774;
    'dispatch: loop {
        match pc {
            0x82468774 => {
    //   block [0x82468774..0x82468784)
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
    //   block [0x824688E0..0x82468964)
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
    //   block [0x82468968..0x824689F4)
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
    //   block [0x82468A48..0x82468ABC)
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
	// 82468A88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82468A8C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82468A90: 48005839  bl 0x8246e2c8
	ctx.lr = 0x82468A94;
	sub_8246E2C8(ctx, base);
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
    //   block [0x82468AE0..0x82468BB4)
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
	// 82468B3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82468B40: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82468B44: 48005785  bl 0x8246e2c8
	ctx.lr = 0x82468B48;
	sub_8246E2C8(ctx, base);
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
    //   block [0x82468BB8..0x82468CA4)
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
	// 82468BF4: 7F8B2214  add r28, r11, r4
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82468BF8: 48000008  b 0x82468c00
	pc = 0x82468C00; continue 'dispatch;
	// 82468BFC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
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
	// 82468C48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82468C4C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82468C50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468C54: 48005675  bl 0x8246e2c8
	ctx.lr = 0x82468C58;
	sub_8246E2C8(ctx, base);
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
	// 82468C7C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82468C80: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468C84: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82468C88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82468C8C: 939B000C  stw r28, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82468C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468C94: 480CC470  b 0x82535104
	sub_825350D0(ctx, base);
	return;
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
    //   block [0x82468CA8..0x82468D60)
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
    //   block [0x82468D60..0x82468DF8)
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
    //   block [0x82468DF8..0x82468E84)
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
	// 82468E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
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
    //   block [0x82469010..0x82469074)
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
    //   block [0x82469078..0x824690F8)
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
	ctx.r[4].s64 = (ctx.r[6].s32 as i64) * (ctx.r[31].s32 as i64);
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
	ctx.r[7].s64 = (ctx.r[27].s32 as i64) * (ctx.r[31].s32 as i64);
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
    //   block [0x82469168..0x8246923C)
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


pub fn sub_82469240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469240 size=8
    let mut pc: u32 = 0x82469240;
    'dispatch: loop {
        match pc {
            0x82469240 => {
    //   block [0x82469240..0x82469248)
	// 82469240: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82469244: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469248 size=12
    let mut pc: u32 = 0x82469248;
    'dispatch: loop {
        match pc {
            0x82469248 => {
    //   block [0x82469248..0x82469254)
	// 82469248: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8246924C: 806B91E0  lwz r3, -0x6e20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28192 as u32) ) } as u64;
	// 82469250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469258 size=32
    let mut pc: u32 = 0x82469258;
    'dispatch: loop {
        match pc {
            0x82469258 => {
    //   block [0x82469258..0x82469278)
	// 82469258: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8246925C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82469260: 816B91DC  lwz r11, -0x6e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82469264: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82469268: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246926C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469274: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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
    //   block [0x82469278..0x8246938C)
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
	// 8246933C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469340: 4BFFD519  bl 0x82466858
	ctx.lr = 0x82469344;
	sub_82466858(ctx, base);
	// 82469344: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
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
    //   block [0x82469390..0x82469404)
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
	// 824693EC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
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
    //   block [0x82469408..0x8246949C)
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
    //   block [0x824694A0..0x82469574)
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
    //   block [0x82469578..0x82469654)
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
    //   block [0x82469654..0x824696E0)
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
	// 824696A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824696A4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824696A8: 409901F8  ble cr6, 0x824698a0
	if !ctx.cr[6].gt {
	pc = 0x824698A0; continue 'dispatch;
	}
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
    //   block [0x82469870..0x824698A0)
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
    //   block [0x824698A0..0x824698B4)
	// 824698A0: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 824698A4: 7F17A800  cmpw cr6, r23, r21
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[21].s32, &mut ctx.xer);
	// 824698A8: 4198FD38  blt cr6, 0x824695e0
	if ctx.cr[6].lt {
	pc = 0x824695E0; continue 'dispatch;
	}
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
    //   block [0x824698B8..0x8246991C)
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
	// 824698E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
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


pub fn sub_82469920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469920 size=40
    let mut pc: u32 = 0x82469920;
    'dispatch: loop {
        match pc {
            0x82469920 => {
    //   block [0x82469920..0x82469948)
	// 82469920: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82469924: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82469928: 409A000C  bne cr6, 0x82469934
	if !ctx.cr[6].eq {
	pc = 0x82469934; continue 'dispatch;
	}
	// 8246992C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82469930: 80EB91E0  lwz r7, -0x6e20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28192 as u32) ) } as u64;
	// 82469934: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82469938: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8246993C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82469940: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82469944: 4BFFF824  b 0x82469168
	sub_82469168(ctx, base);
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
    //   block [0x82469948..0x824699B4)
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
    //   block [0x824699B8..0x82469A84)
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
    //   block [0x82469A88..0x82469AE0)
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
    //   block [0x82469AE0..0x82469B38)
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


pub fn sub_82469B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469B38 size=12
    let mut pc: u32 = 0x82469B38;
    'dispatch: loop {
        match pc {
            0x82469B38 => {
    //   block [0x82469B38..0x82469B44)
	// 82469B38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469B3C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82469B40: 4E800020  blr
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
    //   block [0x82469B48..0x82469BE8)
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
    //   block [0x82469BE8..0x82469CA0)
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
	// 82469C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469C84: 4BFFFEC5  bl 0x82469b48
	ctx.lr = 0x82469C88;
	sub_82469B48(ctx, base);
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
    //   block [0x82469CA0..0x82469D00)
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
    //   block [0x82469D00..0x82469D80)
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
	// 82469D40: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82469D44: 4BFFA0FD  bl 0x82463e40
	ctx.lr = 0x82469D48;
	sub_82463E40(ctx, base);
	// 82469D48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
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


pub fn sub_82469D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469D80 size=12
    let mut pc: u32 = 0x82469D80;
    'dispatch: loop {
        match pc {
            0x82469D80 => {
    //   block [0x82469D80..0x82469D8C)
	// 82469D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82469D84: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82469D88: 4E800020  blr
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
    //   block [0x82469D90..0x82469DD0)
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
    //   block [0x82469E28..0x82469E7C)
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


pub fn sub_82469EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469EA8 size=8
    let mut pc: u32 = 0x82469EA8;
    'dispatch: loop {
        match pc {
            0x82469EA8 => {
    //   block [0x82469EA8..0x82469EB0)
	// 82469EA8: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82469EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469EB0 size=8
    let mut pc: u32 = 0x82469EB0;
    'dispatch: loop {
        match pc {
            0x82469EB0 => {
    //   block [0x82469EB0..0x82469EB8)
	// 82469EB0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469EB4: 4E800020  blr
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
    //   block [0x82469EB8..0x82469EF8)
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
    // ---- function 0x82469EF8 size=12
    let mut pc: u32 = 0x82469EF8;
    'dispatch: loop {
        match pc {
            0x82469EF8 => {
    //   block [0x82469EF8..0x82469F04)
	// 82469EF8: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82469EFC: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82469F00: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469F04 size=8
    let mut pc: u32 = 0x82469F04;
    'dispatch: loop {
        match pc {
            0x82469F04 => {
    //   block [0x82469F04..0x82469F0C)
	// 82469F04: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 82469F08: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469F0C size=12
    let mut pc: u32 = 0x82469F0C;
    'dispatch: loop {
        match pc {
            0x82469F0C => {
    //   block [0x82469F0C..0x82469F18)
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
    // ---- function 0x82469F18 size=12
    let mut pc: u32 = 0x82469F18;
    'dispatch: loop {
        match pc {
            0x82469F18 => {
    //   block [0x82469F18..0x82469F24)
	// 82469F18: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82469F1C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82469F20: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469F24 size=8
    let mut pc: u32 = 0x82469F24;
    'dispatch: loop {
        match pc {
            0x82469F24 => {
    //   block [0x82469F24..0x82469F2C)
	// 82469F24: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82469F28: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469F2C size=12
    let mut pc: u32 = 0x82469F2C;
    'dispatch: loop {
        match pc {
            0x82469F2C => {
    //   block [0x82469F2C..0x82469F38)
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
    // ---- function 0x82469F38 size=4
    let mut pc: u32 = 0x82469F38;
    'dispatch: loop {
        match pc {
            0x82469F38 => {
    //   block [0x82469F38..0x82469F3C)
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
    // ---- function 0x82469FD0 size=24
    let mut pc: u32 = 0x82469FD0;
    'dispatch: loop {
        match pc {
            0x82469FD0 => {
    //   block [0x82469FD0..0x82469FE8)
	// 82469FD0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469FE8 size=20
    let mut pc: u32 = 0x82469FE8;
    'dispatch: loop {
        match pc {
            0x82469FE8 => {
    //   block [0x82469FE8..0x82469FFC)
	// 82469FE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82469FEC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82469FF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82469FF4: 419AFFE0  beq cr6, 0x82469fd4
	if ctx.cr[6].eq {
		sub_82469FD0(ctx, base);
		return;
	}
	// 82469FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A000 size=4
    let mut pc: u32 = 0x8246A000;
    'dispatch: loop {
        match pc {
            0x8246A000 => {
    //   block [0x8246A000..0x8246A004)
	// 8246A000: 480C9190  b 0x82533190
	sub_82533190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A008 size=60
    let mut pc: u32 = 0x8246A008;
    'dispatch: loop {
        match pc {
            0x8246A008 => {
    //   block [0x8246A008..0x8246A044)
	// 8246A008: 7CE41850  subf r7, r4, r3
	ctx.r[7].s64 = ctx.r[3].s64 - ctx.r[4].s64;
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
		sub_8246A0D8(ctx, base);
		return;
	}
	// 8246A024: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8246A028: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A02C: 41980018  blt cr6, 0x8246a044
	if ctx.cr[6].lt {
		sub_8246A044(ctx, base);
		return;
	}
	// 8246A030: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A034: 41990010  bgt cr6, 0x8246a044
	if ctx.cr[6].gt {
		sub_8246A044(ctx, base);
		return;
	}
	// 8246A038: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A03C: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 8246A040: 48000008  b 0x8246a048
	sub_8246A044(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A044(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A044 size=40
    let mut pc: u32 = 0x8246A044;
    'dispatch: loop {
        match pc {
            0x8246A044 => {
    //   block [0x8246A044..0x8246A06C)
	// 8246A044: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8246A048: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A04C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A050: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A054: 41980018  blt cr6, 0x8246a06c
	if ctx.cr[6].lt {
		sub_8246A06C(ctx, base);
		return;
	}
	// 8246A058: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A05C: 41990010  bgt cr6, 0x8246a06c
	if ctx.cr[6].gt {
		sub_8246A06C(ctx, base);
		return;
	}
	// 8246A060: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A064: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A068: 48000008  b 0x8246a070
	sub_8246A06C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A06C size=80
    let mut pc: u32 = 0x8246A06C;
    'dispatch: loop {
        match pc {
            0x8246A06C => {
    //   block [0x8246A06C..0x8246A0BC)
	// 8246A06C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
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
	// 8246A09C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A0A0: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A0A4: 41980018  blt cr6, 0x8246a0bc
	if ctx.cr[6].lt {
		sub_8246A0BC(ctx, base);
		return;
	}
	// 8246A0A8: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A0AC: 41990010  bgt cr6, 0x8246a0bc
	if ctx.cr[6].gt {
		sub_8246A0BC(ctx, base);
		return;
	}
	// 8246A0B0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A0B4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A0B8: 48000008  b 0x8246a0c0
	sub_8246A0BC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A0BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A0BC size=28
    let mut pc: u32 = 0x8246A0BC;
    'dispatch: loop {
        match pc {
            0x8246A0BC => {
    //   block [0x8246A0BC..0x8246A0D8)
	// 8246A0BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
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
	sub_8246A008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A0D8 size=8
    let mut pc: u32 = 0x8246A0D8;
    'dispatch: loop {
        match pc {
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
    // ---- function 0x8246A0F0 size=76
    let mut pc: u32 = 0x8246A0F0;
    'dispatch: loop {
        match pc {
            0x8246A0F0 => {
    //   block [0x8246A0F0..0x8246A13C)
	// 8246A0F0: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 8246A0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8246A0F8: 7C841850  subf r4, r4, r3
	ctx.r[4].s64 = ctx.r[3].s64 - ctx.r[4].s64;
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
		sub_8246A13C(ctx, base);
		return;
	}
	// 8246A128: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A12C: 41990010  bgt cr6, 0x8246a13c
	if ctx.cr[6].gt {
		sub_8246A13C(ctx, base);
		return;
	}
	// 8246A130: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A134: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 8246A138: 48000008  b 0x8246a140
	sub_8246A13C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A13C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A13C size=40
    let mut pc: u32 = 0x8246A13C;
    'dispatch: loop {
        match pc {
            0x8246A13C => {
    //   block [0x8246A13C..0x8246A164)
	// 8246A13C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8246A140: 89270000  lbz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A144: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A148: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A14C: 41980018  blt cr6, 0x8246a164
	if ctx.cr[6].lt {
		sub_8246A164(ctx, base);
		return;
	}
	// 8246A150: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A154: 41990010  bgt cr6, 0x8246a164
	if ctx.cr[6].gt {
		sub_8246A164(ctx, base);
		return;
	}
	// 8246A158: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A15C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A160: 48000008  b 0x8246a168
	sub_8246A164(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A164 size=80
    let mut pc: u32 = 0x8246A164;
    'dispatch: loop {
        match pc {
            0x8246A164 => {
    //   block [0x8246A164..0x8246A1B4)
	// 8246A164: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8246A168: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A16C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 8246A170: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A174: 41980060  blt cr6, 0x8246a1d4
	if ctx.cr[6].lt {
		sub_8246A1D4(ctx, base);
		return;
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
	// 8246A194: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A198: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A19C: 41980018  blt cr6, 0x8246a1b4
	if ctx.cr[6].lt {
		sub_8246A1B4(ctx, base);
		return;
	}
	// 8246A1A0: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A1A4: 41990010  bgt cr6, 0x8246a1b4
	if ctx.cr[6].gt {
		sub_8246A1B4(ctx, base);
		return;
	}
	// 8246A1A8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A1AC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A1B0: 48000008  b 0x8246a1b8
	sub_8246A1B4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A1B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A1B4 size=32
    let mut pc: u32 = 0x8246A1B4;
    'dispatch: loop {
        match pc {
            0x8246A1B4 => {
    //   block [0x8246A1B4..0x8246A1D4)
	// 8246A1B4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
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
	sub_8246A0F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A1D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A1D4 size=8
    let mut pc: u32 = 0x8246A1D4;
    'dispatch: loop {
        match pc {
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
    //   block [0x8246A1F0..0x8246A20C)
	// 8246A1F0: 7D441850  subf r10, r4, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[4].s64;
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
    // ---- function 0x8246A210 size=8
    let mut pc: u32 = 0x8246A210;
    'dispatch: loop {
        match pc {
            0x8246A210 => {
    //   block [0x8246A210..0x8246A218)
	// 8246A210: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8246A214: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A218 size=8
    let mut pc: u32 = 0x8246A218;
    'dispatch: loop {
        match pc {
            0x8246A218 => {
    //   block [0x8246A218..0x8246A220)
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
    //   block [0x8246A220..0x8246A244)
	// 8246A220: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
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
    // ---- function 0x8246A248 size=12
    let mut pc: u32 = 0x8246A248;
    'dispatch: loop {
        match pc {
            0x8246A248 => {
    //   block [0x8246A248..0x8246A254)
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
    // ---- function 0x8246A280 size=4
    let mut pc: u32 = 0x8246A280;
    'dispatch: loop {
        match pc {
            0x8246A280 => {
    //   block [0x8246A280..0x8246A284)
	// 8246A280: 480CA438  b 0x825346b8
	sub_825346B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A288 size=4
    let mut pc: u32 = 0x8246A288;
    'dispatch: loop {
        match pc {
            0x8246A288 => {
    //   block [0x8246A288..0x8246A28C)
	// 8246A288: 480C90E8  b 0x82533370
	sub_82533370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A290 size=4
    let mut pc: u32 = 0x8246A290;
    'dispatch: loop {
        match pc {
            0x8246A290 => {
    //   block [0x8246A290..0x8246A294)
	// 8246A290: 480CDE40  b 0x825380d0
	sub_825380D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A298 size=12
    let mut pc: u32 = 0x8246A298;
    'dispatch: loop {
        match pc {
            0x8246A298 => {
    //   block [0x8246A298..0x8246A2A4)
	// 8246A298: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A2A0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A2A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A2A4 size=60
    let mut pc: u32 = 0x8246A2A4;
    'dispatch: loop {
        match pc {
            0x8246A2A4 => {
    //   block [0x8246A2A4..0x8246A2E0)
	// 8246A2A4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
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
    // ---- function 0x8246A2E0 size=12
    let mut pc: u32 = 0x8246A2E0;
    'dispatch: loop {
        match pc {
            0x8246A2E0 => {
    //   block [0x8246A2E0..0x8246A2EC)
	// 8246A2E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A2E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A2E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A2EC size=60
    let mut pc: u32 = 0x8246A2EC;
    'dispatch: loop {
        match pc {
            0x8246A2EC => {
    //   block [0x8246A2EC..0x8246A328)
	// 8246A2EC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
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
    // ---- function 0x8246A328 size=4
    let mut pc: u32 = 0x8246A328;
    'dispatch: loop {
        match pc {
            0x8246A328 => {
    //   block [0x8246A328..0x8246A32C)
	// 8246A328: 480CA828  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A330 size=4
    let mut pc: u32 = 0x8246A330;
    'dispatch: loop {
        match pc {
            0x8246A330 => {
    //   block [0x8246A330..0x8246A334)
	// 8246A330: 480CB040  b 0x82535370
	sub_82535370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A338 size=4
    let mut pc: u32 = 0x8246A338;
    'dispatch: loop {
        match pc {
            0x8246A338 => {
    //   block [0x8246A338..0x8246A33C)
	// 8246A338: 480CAE98  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A340 size=20
    let mut pc: u32 = 0x8246A340;
    'dispatch: loop {
        match pc {
            0x8246A340 => {
    //   block [0x8246A340..0x8246A354)
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A354 size=20
    let mut pc: u32 = 0x8246A354;
    'dispatch: loop {
        match pc {
            0x8246A354 => {
    //   block [0x8246A354..0x8246A368)
	// 8246A354: 7D2B2A14  add r9, r11, r5
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8246A358: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A35C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A360: 7C674051  subf. r3, r7, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246A364: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A368 size=20
    let mut pc: u32 = 0x8246A368;
    'dispatch: loop {
        match pc {
            0x8246A368 => {
    //   block [0x8246A368..0x8246A37C)
	// 8246A368: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A36C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246A370: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246A374: 409AFFE4  bne cr6, 0x8246a358
	if !ctx.cr[6].eq {
		sub_8246A354(ctx, base);
		return;
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
    //   block [0x8246A380..0x8246A400)
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
    //   block [0x8246A400..0x8246A484)
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
    //   block [0x8246A488..0x8246A560)
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
    //   block [0x8246A560..0x8246A6B8)
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
	// 8246A5B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A5B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A5BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A5C0: 48003D09  bl 0x8246e2c8
	ctx.lr = 0x8246A5C4;
	sub_8246E2C8(ctx, base);
	// 8246A5C4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8246A5C8: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 8246A5CC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246A5D0: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
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
	// 8246A624: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A628: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A630: 48003C99  bl 0x8246e2c8
	ctx.lr = 0x8246A634;
	sub_8246E2C8(ctx, base);
	// 8246A634: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8246A638: 4BFFFF9C  b 0x8246a5d4
	pc = 0x8246A5D4; continue 'dispatch;
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
    //   block [0x8246A6B8..0x8246A700)
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
    //   block [0x8246A708..0x8246A754)
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
    //   block [0x8246A760..0x8246A7E0)
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
	// 8246A7AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A7B0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A7B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A7B8: 48003B11  bl 0x8246e2c8
	ctx.lr = 0x8246A7BC;
	sub_8246E2C8(ctx, base);
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
    //   block [0x8246A7E0..0x8246A87C)
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
	// 8246A848: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A84C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A854: 48003A75  bl 0x8246e2c8
	ctx.lr = 0x8246A858;
	sub_8246E2C8(ctx, base);
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
    // ---- function 0x8246A880 size=16
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A890 size=68
    let mut pc: u32 = 0x8246A890;
    'dispatch: loop {
        match pc {
            0x8246A890 => {
    //   block [0x8246A890..0x8246A8D4)
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
    // ---- function 0x8246A8D8 size=16
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A8E8 size=68
    let mut pc: u32 = 0x8246A8E8;
    'dispatch: loop {
        match pc {
            0x8246A8E8 => {
    //   block [0x8246A8E8..0x8246A92C)
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
    //   block [0x8246A930..0x8246A980)
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
    //   block [0x8246A9B4..0x8246A9F8)
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
    //   block [0x8246A9F8..0x8246AA40)
	// 8246A9F8: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8246A9FC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8246AA00: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
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
    //   block [0x8246AA40..0x8246AA8C)
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
    // ---- function 0x8246AA90 size=20
    let mut pc: u32 = 0x8246AA90;
    'dispatch: loop {
        match pc {
            0x8246AA90 => {
    //   block [0x8246AA90..0x8246AAA4)
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AAA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246AAA4 size=44
    let mut pc: u32 = 0x8246AAA4;
    'dispatch: loop {
        match pc {
            0x8246AAA4 => {
    //   block [0x8246AAA4..0x8246AAD0)
	// 8246AAA4: 7CA80774  extsb r8, r5
	ctx.r[8].s64 = ctx.r[5].s8 as i64;
	// 8246AAA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
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
		sub_8246AAD0(ctx, base);
		return;
	}
	// 8246AAC0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8246AAC4: 7CCA59AE  stbx r6, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u8) };
	// 8246AAC8: 99230000  stb r9, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246AACC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246AAD0 size=24
    let mut pc: u32 = 0x8246AAD0;
    'dispatch: loop {
        match pc {
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
		sub_8246AAA4(ctx, base);
		return;
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
    //   block [0x8246AAE8..0x8246ABD0)
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
    //   block [0x8246AC50..0x8246ACEC)
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
    //   block [0x8246ACF0..0x8246ADB8)
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
	// 8246AD60: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
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
    //   block [0x8246ADB8..0x8246AE80)
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
	// 8246AE28: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
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
    //   block [0x8246AE80..0x8246AF38)
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


pub fn sub_8246AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246AF38 size=240
    let mut pc: u32 = 0x8246AF38;
    'dispatch: loop {
        match pc {
            0x8246AF38 => {
    //   block [0x8246AF38..0x8246B028)
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
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
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
	// 8246AFE0: 480C9B71  bl 0x82534b50
	ctx.lr = 0x8246AFE4;
	sub_82534B50(ctx, base);
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
    //   block [0x8246B028..0x8246B1B4)
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
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
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
	// 8246B0C4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
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
	// 8246B124: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8246B128: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8246B12C: 48000030  b 0x8246b15c
	pc = 0x8246B15C; continue 'dispatch;
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
	// 8246B14C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B150: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246B158: 48003171  bl 0x8246e2c8
	ctx.lr = 0x8246B15C;
	sub_8246E2C8(ctx, base);
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
	// 8246B178: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246B17C: 9B590000  stb r26, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8246B180: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246B184: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
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


pub fn sub_8246B200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B200 size=8
    let mut pc: u32 = 0x8246B200;
    'dispatch: loop {
        match pc {
            0x8246B200 => {
    //   block [0x8246B200..0x8246B208)
	// 8246B200: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246B204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B208 size=16
    let mut pc: u32 = 0x8246B208;
    'dispatch: loop {
        match pc {
            0x8246B208 => {
    //   block [0x8246B208..0x8246B218)
	// 8246B208: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246B20C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246B210: 419A0008  beq cr6, 0x8246b218
	if ctx.cr[6].eq {
		sub_8246B218(ctx, base);
		return;
	}
	// 8246B214: 4BFFB5C4  b 0x824667d8
	sub_824667D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B218 size=8
    let mut pc: u32 = 0x8246B218;
    'dispatch: loop {
        match pc {
            0x8246B218 => {
    //   block [0x8246B218..0x8246B220)
	// 8246B218: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246B21C: 4E800020  blr
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
    //   block [0x8246B2D8..0x8246B314)
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
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8246B310: 48000064  b 0x8246b374
	pc = 0x8246B374; continue 'dispatch;
            }
            0x8246B314 => {
    //   block [0x8246B314..0x8246B354)
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
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8246B350: 48000024  b 0x8246b374
	pc = 0x8246B374; continue 'dispatch;
            }
            0x8246B354 => {
    //   block [0x8246B354..0x8246B374)
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
	// 8246B368: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B36C: 4BFFB94D  bl 0x82466cb8
	ctx.lr = 0x8246B370;
	sub_82466CB8(ctx, base);
	// 8246B370: 7D63F9D6  mullw r11, r3, r31
	ctx.r[11].s64 = (ctx.r[3].s32 as i64) * (ctx.r[31].s32 as i64);
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


pub fn sub_8246B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B390 size=20
    let mut pc: u32 = 0x8246B390;
    'dispatch: loop {
        match pc {
            0x8246B390 => {
    //   block [0x8246B390..0x8246B3A4)
	// 8246B390: A1640010  lhz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246B394: 556BBA7E  srwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246B398: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8246B39C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246B3A0: 4E800020  blr
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
    //   block [0x8246B3A8..0x8246B494)
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
	// 8246B3CC: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
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
	// 8246B430: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246B434: 48000024  b 0x8246b458
	pc = 0x8246B458; continue 'dispatch;
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
		sub_8246B4E8(ctx, base);
		return;
	}
	// 8246B4AC: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 8246B4B0: 419A0038  beq cr6, 0x8246b4e8
	if ctx.cr[6].eq {
		sub_8246B4E8(ctx, base);
		return;
	}
	// 8246B4B4: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 8246B4B8: 419A0028  beq cr6, 0x8246b4e0
	if ctx.cr[6].eq {
		sub_8246B4E0(ctx, base);
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


pub fn sub_8246B4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B4E0 size=8
    let mut pc: u32 = 0x8246B4E0;
    'dispatch: loop {
        match pc {
            0x8246B4E0 => {
    //   block [0x8246B4E0..0x8246B4E8)
	// 8246B4E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246B4E4: 4BFFB7D4  b 0x82466cb8
	sub_82466CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246B4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246B4E8 size=8
    let mut pc: u32 = 0x8246B4E8;
    'dispatch: loop {
        match pc {
            0x8246B4E8 => {
    //   block [0x8246B4E8..0x8246B4F0)
	// 8246B4E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246B4EC: 4E800020  blr
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
    //   block [0x8246B4F0..0x8246B564)
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
	// 8246B530: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B534: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 8246B538: 48000014  b 0x8246b54c
	pc = 0x8246B54C; continue 'dispatch;
	// 8246B53C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246B540: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8246B544: 48000008  b 0x8246b54c
	pc = 0x8246B54C; continue 'dispatch;
	// 8246B548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
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
    //   block [0x8246B568..0x8246B5CC)
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
	// 8246B5A8: B3DF0000  sth r30, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8246B5AC: 48000008  b 0x8246b5b4
	pc = 0x8246B5B4; continue 'dispatch;
	// 8246B5B0: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
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
    //   block [0x8246B5D0..0x8246B6EC)
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
	// 8246B64C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B650: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B654: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B658: 48002C71  bl 0x8246e2c8
	ctx.lr = 0x8246B65C;
	sub_8246E2C8(ctx, base);
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
	// 8246B698: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B69C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246B6A0: 48002C29  bl 0x8246e2c8
	ctx.lr = 0x8246B6A4;
	sub_8246E2C8(ctx, base);
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
    //   block [0x8246B6F0..0x8246BA8C)
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
	// 8246B72C: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
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
	// 8246B804: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B808: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B80C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B810: 48002AB9  bl 0x8246e2c8
	ctx.lr = 0x8246B814;
	sub_8246E2C8(ctx, base);
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
	// 8246B84C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B850: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B854: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B858: 48002A71  bl 0x8246e2c8
	ctx.lr = 0x8246B85C;
	sub_8246E2C8(ctx, base);
	// 8246B85C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246B860: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8246B864: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8246B868: 480001DC  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
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
	// 8246B8A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246B8A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246B8A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246B8AC: 48002A1D  bl 0x8246e2c8
	ctx.lr = 0x8246B8B0;
	sub_8246E2C8(ctx, base);
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
	// 8246B948: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B94C: 388B8524  addi r4, r11, -0x7adc
	ctx.r[4].s64 = ctx.r[11].s64 + -31452;
	// 8246B950: 4BFFEC11  bl 0x8246a560
	ctx.lr = 0x8246B954;
	sub_8246A560(ctx, base);
	// 8246B954: 480000F0  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
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
	// 8246B970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246B974: 38AA850C  addi r5, r10, -0x7af4
	ctx.r[5].s64 = ctx.r[10].s64 + -31476;
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
	// 8246B9B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246B9B8: 388B84EC  addi r4, r11, -0x7b14
	ctx.r[4].s64 = ctx.r[11].s64 + -31508;
	// 8246B9BC: 4BFFEBA5  bl 0x8246a560
	ctx.lr = 0x8246B9C0;
	sub_8246A560(ctx, base);
	// 8246B9C0: 48000084  b 0x8246ba44
	pc = 0x8246BA44; continue 'dispatch;
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
    //   block [0x8246BA90..0x8246BDBC)
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
	// 8246BBD8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BBDC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246BBE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BBE4: 480026E5  bl 0x8246e2c8
	ctx.lr = 0x8246BBE8;
	sub_8246E2C8(ctx, base);
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
	// 8246BC74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BC78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246BC7C: 4800264D  bl 0x8246e2c8
	ctx.lr = 0x8246BC80;
	sub_8246E2C8(ctx, base);
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
	// 8246BCC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BCC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246BCCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BCD0: 480025F9  bl 0x8246e2c8
	ctx.lr = 0x8246BCD4;
	sub_8246E2C8(ctx, base);
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
	// 8246BD14: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246BD18: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8246BD1C: 3BAB4BD0  addi r29, r11, 0x4bd0
	ctx.r[29].s64 = ctx.r[11].s64 + 19408;
	// 8246BD20: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
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
	// 8246BD78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246BD7C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246BD80: 480C9384  b 0x82535104
	sub_825350D0(ctx, base);
	return;
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
    //   block [0x8246BDC0..0x8246C024)
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
	// 8246BE24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BE28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246BE2C: 4800249D  bl 0x8246e2c8
	ctx.lr = 0x8246BE30;
	sub_8246E2C8(ctx, base);
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
	// 8246BE84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BE88: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246BE8C: 480C927C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
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
	// 8246BEE8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BEEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246BEF0: 480023D9  bl 0x8246e2c8
	ctx.lr = 0x8246BEF4;
	sub_8246E2C8(ctx, base);
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
	// 8246BF48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246BF4C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246BF50: 480C91B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
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
	// 8246BFAC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246BFB0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8246BFB4: 48002315  bl 0x8246e2c8
	ctx.lr = 0x8246BFB8;
	sub_8246E2C8(ctx, base);
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
	// 8246C00C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246C010: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246C014: 480C90F4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
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


pub fn sub_8246C0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C0A8 size=20
    let mut pc: u32 = 0x8246C0A8;
    'dispatch: loop {
        match pc {
            0x8246C0A8 => {
    //   block [0x8246C0A8..0x8246C0BC)
	// 8246C0A8: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C0AC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C0B0: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246C0B4: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246C0B8: 4E800020  blr
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
    //   block [0x8246C0C0..0x8246C288)
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
	ctx.r[5].s64 = (ctx.r[28].s32 as i64) * (ctx.r[6].s32 as i64);
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
	// 8246C100: 7FFC31D6  mullw r31, r28, r6
	ctx.r[31].s64 = (ctx.r[28].s32 as i64) * (ctx.r[6].s32 as i64);
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


pub fn sub_8246C2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C2A0 size=8
    let mut pc: u32 = 0x8246C2A0;
    'dispatch: loop {
        match pc {
            0x8246C2A0 => {
    //   block [0x8246C2A0..0x8246C2A8)
	// 8246C2A0: 9883000C  stb r4, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u8 ) };
	// 8246C2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C2A8 size=12
    let mut pc: u32 = 0x8246C2A8;
    'dispatch: loop {
        match pc {
            0x8246C2A8 => {
    //   block [0x8246C2A8..0x8246C2B4)
	// 8246C2A8: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246C2AC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246C2B0: 4E800020  blr
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
    //   block [0x8246C300..0x8246C38C)
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
    // ---- function 0x8246C390 size=40
    let mut pc: u32 = 0x8246C390;
    'dispatch: loop {
        match pc {
            0x8246C390 => {
    //   block [0x8246C390..0x8246C3B8)
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C3B8 size=16
    let mut pc: u32 = 0x8246C3B8;
    'dispatch: loop {
        match pc {
            0x8246C3B8 => {
    //   block [0x8246C3B8..0x8246C3C8)
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
    //   block [0x8246C538..0x8246C5B4)
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
    //   block [0x8246C738..0x8246C7E4)
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
	// 8246C794: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 8246C798: 40980008  bge cr6, 0x8246c7a0
	if !ctx.cr[6].lt {
	pc = 0x8246C7A0; continue 'dispatch;
	}
	// 8246C79C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
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
    //   block [0x8246C7E8..0x8246C894)
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
	// 8246C844: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 8246C848: 40980008  bge cr6, 0x8246c850
	if !ctx.cr[6].lt {
	pc = 0x8246C850; continue 'dispatch;
	}
	// 8246C84C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
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
    //   block [0x8246C918..0x8246C97C)
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
    //   block [0x8246C980..0x8246C9BC)
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
	// 8246C998: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246C99C: 7F072000  cmpw cr6, r7, r4
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8246C9A0: 419A001C  beq cr6, 0x8246c9bc
	if ctx.cr[6].eq {
		sub_8246C9BC(ctx, base);
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
	// 8246C9B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246C9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246C9BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246C9BC size=24
    let mut pc: u32 = 0x8246C9BC;
    'dispatch: loop {
        match pc {
            0x8246C9BC => {
    //   block [0x8246C9BC..0x8246C9D4)
	// 8246C9BC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246C9C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246C9C4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8246C9C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246C9CC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246C9D0: 4E800020  blr
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
    //   block [0x8246C9D8..0x8246CA5C)
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
	// 8246CA34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246CA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246CA3C: 480C86C8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
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
    //   block [0x8246CA60..0x8246CB14)
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


pub fn sub_8246CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CB18 size=16
    let mut pc: u32 = 0x8246CB18;
    'dispatch: loop {
        match pc {
            0x8246CB18 => {
    //   block [0x8246CB18..0x8246CB28)
	// 8246CB18: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246CB1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246CB20: 419A0008  beq cr6, 0x8246cb28
	if ctx.cr[6].eq {
		sub_8246CB28(ctx, base);
		return;
	}
	// 8246CB24: 4BFF9CB4  b 0x824667d8
	sub_824667D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CB28 size=8
    let mut pc: u32 = 0x8246CB28;
    'dispatch: loop {
        match pc {
            0x8246CB28 => {
    //   block [0x8246CB28..0x8246CB30)
	// 8246CB28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246CB2C: 4E800020  blr
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
    //   block [0x8246CB30..0x8246CBF4)
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
    //   block [0x8246CC40..0x8246CCEC)
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
	// 8246CCD4: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8246CCD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246CCDC: 419A0010  beq cr6, 0x8246ccec
	if ctx.cr[6].eq {
		sub_8246CCEC(ctx, base);
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


pub fn sub_8246CCEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246CCEC size=8
    let mut pc: u32 = 0x8246CCEC;
    'dispatch: loop {
        match pc {
            0x8246CCEC => {
    //   block [0x8246CCEC..0x8246CCF4)
	// 8246CCEC: 5543F87E  srwi r3, r10, 1
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246CCF0: 4E800020  blr
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
    //   block [0x8246CCF8..0x8246CD54)
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


pub fn sub_8246CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8246CD98 size=28
    let mut pc: u32 = 0x8246CD98;
    'dispatch: loop {
        match pc {
            0x8246CD98 => {
    //   block [0x8246CD98..0x8246CDB4)
	// 8246CD98: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CD9C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246CDA0: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246CDA4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8246CDA8: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8246CDAC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246CDB0: 4E800020  blr
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
    //   block [0x8246CDB8..0x8246CE24)
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
    //   block [0x8246CE28..0x8246CF9C)
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
	ctx.r[3].s64 = ctx.r[11].s64 * 84;
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
	ctx.r[11].s64 = ctx.r[11].s64 * 84;
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
    //   block [0x8246CFA0..0x8246D044)
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
	// 8246D014: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246D018: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8246D01C: 4198FFD8  blt cr6, 0x8246cff4
	if ctx.cr[6].lt {
	pc = 0x8246CFF4; continue 'dispatch;
	}
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
    //   block [0x8246D048..0x8246D184)
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
    // ---- function 0x8246D188 size=32
    let mut pc: u32 = 0x8246D188;
    'dispatch: loop {
        match pc {
            0x8246D188 => {
    //   block [0x8246D188..0x8246D1A8)
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D1A8 size=36
    let mut pc: u32 = 0x8246D1A8;
    'dispatch: loop {
        match pc {
            0x8246D1A8 => {
    //   block [0x8246D1A8..0x8246D1CC)
	// 8246D1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D1AC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
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
    //   block [0x8246D1D0..0x8246D214)
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
    // ---- function 0x8246D218 size=32
    let mut pc: u32 = 0x8246D218;
    'dispatch: loop {
        match pc {
            0x8246D218 => {
    //   block [0x8246D218..0x8246D238)
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
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D238 size=36
    let mut pc: u32 = 0x8246D238;
    'dispatch: loop {
        match pc {
            0x8246D238 => {
    //   block [0x8246D238..0x8246D25C)
	// 8246D238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246D23C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
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
    //   block [0x8246D260..0x8246D280)
	// 8246D260: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8246D264: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8246D268: 40990010  ble cr6, 0x8246d278
	if !ctx.cr[6].gt {
	pc = 0x8246D278; continue 'dispatch;
	}
	// 8246D26C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D270: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8246D274: 4198FFF8  blt cr6, 0x8246d26c
	if ctx.cr[6].lt {
	pc = 0x8246D26C; continue 'dispatch;
	}
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
    //   block [0x8246D280..0x8246D33C)
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
	// 8246D2B4: 57FF083C  slwi r31, r31, 1
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8246D2B8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246D2BC: 4198FFF8  blt cr6, 0x8246d2b4
	if ctx.cr[6].lt {
	pc = 0x8246D2B4; continue 'dispatch;
	}
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
    //   block [0x8246D340..0x8246D3C4)
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
    // ---- function 0x8246D3C8 size=16
    let mut pc: u32 = 0x8246D3C8;
    'dispatch: loop {
        match pc {
            0x8246D3C8 => {
    //   block [0x8246D3C8..0x8246D3D8)
	// 8246D3C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D3CC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D3D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D3D4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D3D8 size=36
    let mut pc: u32 = 0x8246D3D8;
    'dispatch: loop {
        match pc {
            0x8246D3D8 => {
    //   block [0x8246D3D8..0x8246D3FC)
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


pub fn sub_8246D3FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D3FC size=4
    let mut pc: u32 = 0x8246D3FC;
    'dispatch: loop {
        match pc {
            0x8246D3FC => {
    //   block [0x8246D3FC..0x8246D400)
	// 8246D3FC: 4E800020  blr
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
    //   block [0x8246D400..0x8246D4D4)
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
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
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
    // ---- function 0x8246D4D8 size=52
    let mut pc: u32 = 0x8246D4D8;
    'dispatch: loop {
        match pc {
            0x8246D4D8 => {
    //   block [0x8246D4D8..0x8246D50C)
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
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
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
		sub_8246D50C(ctx, base);
		return;
	}
	// 8246D504: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8246D508: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D50C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D50C size=32
    let mut pc: u32 = 0x8246D50C;
    'dispatch: loop {
        match pc {
            0x8246D50C => {
    //   block [0x8246D50C..0x8246D52C)
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
		sub_8246D4D8(ctx, base);
		return;
	}
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
    //   block [0x8246D530..0x8246D588)
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
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[7].s32 as i64);
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
	// 8246D560: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8246D564: 419A0024  beq cr6, 0x8246d588
	if ctx.cr[6].eq {
		sub_8246D588(ctx, base);
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
	// 8246D580: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8246D584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D588 size=20
    let mut pc: u32 = 0x8246D588;
    'dispatch: loop {
        match pc {
            0x8246D588 => {
    //   block [0x8246D588..0x8246D59C)
	// 8246D588: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8246D58C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D590: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D594: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8246D598: 4E800020  blr
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
    //   block [0x8246D5A0..0x8246D61C)
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
	// 8246D600: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
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
    //   block [0x8246D620..0x8246D760)
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
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[4].s32 as i64);
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
    //   block [0x8246D760..0x8246D848)
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
    // ---- function 0x8246D848 size=16
    let mut pc: u32 = 0x8246D848;
    'dispatch: loop {
        match pc {
            0x8246D848 => {
    //   block [0x8246D848..0x8246D858)
	// 8246D848: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D84C: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246D850: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246D854: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D858 size=36
    let mut pc: u32 = 0x8246D858;
    'dispatch: loop {
        match pc {
            0x8246D858 => {
    //   block [0x8246D858..0x8246D87C)
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


pub fn sub_8246D87C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D87C size=4
    let mut pc: u32 = 0x8246D87C;
    'dispatch: loop {
        match pc {
            0x8246D87C => {
    //   block [0x8246D87C..0x8246D880)
	// 8246D87C: 4E800020  blr
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
    //   block [0x8246D880..0x8246D95C)
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


