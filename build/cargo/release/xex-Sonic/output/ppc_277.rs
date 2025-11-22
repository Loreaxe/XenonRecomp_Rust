pub fn sub_831660F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831660F0 size=12
    let mut pc: u32 = 0x831660F0;
    'dispatch: loop {
        match pc {
            0x831660F0 => {
    //   block [0x831660F0..0x831660FC)
	// 831660F0: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831660F4: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831660F8: 4BFFFB58  b 0x83165c50
	sub_83165C50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166100 size=12
    let mut pc: u32 = 0x83166100;
    'dispatch: loop {
        match pc {
            0x83166100 => {
    //   block [0x83166100..0x8316610C)
	// 83166100: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83166104: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166108: 4BFFF818  b 0x83165920
	sub_83165920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166110 size=12
    let mut pc: u32 = 0x83166110;
    'dispatch: loop {
        match pc {
            0x83166110 => {
    //   block [0x83166110..0x8316611C)
	// 83166110: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83166114: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166118: 4BFFF650  b 0x83165768
	sub_83165768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166120 size=96
    let mut pc: u32 = 0x83166120;
    'dispatch: loop {
        match pc {
            0x83166120 => {
    //   block [0x83166120..0x83166180)
	// 83166120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316612C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166134: 3BE3FCAC  addi r31, r3, -0x354
	ctx.r[31].s64 = ctx.r[3].s64 + -852;
	// 83166138: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316613C: 387F0354  addi r3, r31, 0x354
	ctx.r[3].s64 = ctx.r[31].s64 + 852;
	// 83166140: 4BFFED31  bl 0x83164e70
	ctx.lr = 0x83166144;
	sub_83164E70(ctx, base);
	// 83166144: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166148: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316614C: 396B6C54  addi r11, r11, 0x6c54
	ctx.r[11].s64 = ctx.r[11].s64 + 27732;
	// 83166150: 917F0354  stw r11, 0x354(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(852 as u32), ctx.r[11].u32 ) };
	// 83166154: 41820010  beq 0x83166164
	if ctx.cr[0].eq {
	pc = 0x83166164; continue 'dispatch;
	}
	// 83166158: 38800358  li r4, 0x358
	ctx.r[4].s64 = 856;
	// 8316615C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166160: 4BFF9B21  bl 0x8315fc80
	ctx.lr = 0x83166164;
	sub_8315FC80(ctx, base);
	// 83166164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166174: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83166178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316617C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166180 size=220
    let mut pc: u32 = 0x83166180;
    'dispatch: loop {
        match pc {
            0x83166180 => {
    //   block [0x83166180..0x8316625C)
	// 83166180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166184: 48041FE9  bl 0x831a816c
	ctx.lr = 0x83166188;
	sub_831A8130(ctx, base);
	// 83166188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316618C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83166190: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166194: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83166198: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316619C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831661A0: 409A0014  bne cr6, 0x831661b4
	if !ctx.cr[6].eq {
	pc = 0x831661B4; continue 'dispatch;
	}
	// 831661A4: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 831661A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831661AC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831661B0: 480000A4  b 0x83166254
	pc = 0x83166254; continue 'dispatch;
	// 831661B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831661B8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831661BC: 38AB726C  addi r5, r11, 0x726c
	ctx.r[5].s64 = ctx.r[11].s64 + 29292;
	// 831661C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831661C4: 38600358  li r3, 0x358
	ctx.r[3].s64 = 856;
	// 831661C8: 4BFF9B31  bl 0x8315fcf8
	ctx.lr = 0x831661CC;
	sub_8315FCF8(ctx, base);
	// 831661CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831661D0: 41820010  beq 0x831661e0
	if ctx.cr[0].eq {
	pc = 0x831661E0; continue 'dispatch;
	}
	// 831661D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831661D8: 4BFFEB99  bl 0x83164d70
	ctx.lr = 0x831661DC;
	sub_83164D70(ctx, base);
	// 831661DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831661E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831661E4: 409A0028  bne cr6, 0x8316620c
	if !ctx.cr[6].eq {
	pc = 0x8316620C; continue 'dispatch;
	}
	// 831661E8: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 831661EC: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831661F0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831661F4: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 831661F8: 388A7260  addi r4, r10, 0x7260
	ctx.r[4].s64 = ctx.r[10].s64 + 29280;
	// 831661FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166200: 4BFF9941  bl 0x8315fb40
	ctx.lr = 0x83166204;
	sub_8315FB40(ctx, base);
	// 83166204: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166208: 4800004C  b 0x83166254
	pc = 0x83166254; continue 'dispatch;
	// 8316620C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166214: 4BFFF5BD  bl 0x831657d0
	ctx.lr = 0x83166218;
	sub_831657D0(ctx, base);
	// 83166218: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316621C: 40820034  bne 0x83166250
	if !ctx.cr[0].eq {
	pc = 0x83166250; continue 'dispatch;
	}
	// 83166220: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83166224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83166228: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316622C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83166230: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83166234: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83166238: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316623C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83166240: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83166244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166248: 4E800421  bctrl
	ctx.lr = 0x8316624C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316624C: 4BFFFFB8  b 0x83166204
	pc = 0x83166204; continue 'dispatch;
	// 83166250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166258: 48041F64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166260 size=260
    let mut pc: u32 = 0x83166260;
    'dispatch: loop {
        match pc {
            0x83166260 => {
    //   block [0x83166260..0x83166364)
	// 83166260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166264: 48041F09  bl 0x831a816c
	ctx.lr = 0x83166268;
	sub_831A8130(ctx, base);
	// 83166268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316626C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166270: 817F033C  lwz r11, 0x33c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 83166274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83166278: 409A0028  bne cr6, 0x831662a0
	if !ctx.cr[6].eq {
	pc = 0x831662A0; continue 'dispatch;
	}
	// 8316627C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166284: 388B72D8  addi r4, r11, 0x72d8
	ctx.r[4].s64 = ctx.r[11].s64 + 29400;
	// 83166288: 4BFF9891  bl 0x8315fb18
	ctx.lr = 0x8316628C;
	sub_8315FB18(ctx, base);
	// 8316628C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83166290: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166294: 409A00C4  bne cr6, 0x83166358
	if !ctx.cr[6].eq {
	pc = 0x83166358; continue 'dispatch;
	}
	// 83166298: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316629C: 480000B8  b 0x83166354
	pc = 0x83166354; continue 'dispatch;
	// 831662A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831662A4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831662A8: 48008A59  bl 0x8316ed00
	ctx.lr = 0x831662AC;
	sub_8316ED00(ctx, base);
	// 831662AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831662B0: 41820084  beq 0x83166334
	if ctx.cr[0].eq {
	pc = 0x83166334; continue 'dispatch;
	}
	// 831662B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831662B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831662BC: 409A0078  bne cr6, 0x83166334
	if !ctx.cr[6].eq {
	pc = 0x83166334; continue 'dispatch;
	}
	// 831662C0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831662C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831662C8: 3BAB8328  addi r29, r11, -0x7cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -31960;
	// 831662CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831662D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831662D4: 480094ED  bl 0x8316f7c0
	ctx.lr = 0x831662D8;
	sub_8316F7C0(ctx, base);
	// 831662D8: 807F033C  lwz r3, 0x33c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 831662DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831662E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831662E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831662E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831662EC: 4E800421  bctrl
	ctx.lr = 0x831662F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831662F0: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831662F4: 41820034  beq 0x83166328
	if ctx.cr[0].eq {
	pc = 0x83166328; continue 'dispatch;
	}
	// 831662F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831662FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166300: 409A0028  bne cr6, 0x83166328
	if !ctx.cr[6].eq {
	pc = 0x83166328; continue 'dispatch;
	}
	// 83166304: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83166308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316630C: 480094F5  bl 0x8316f800
	ctx.lr = 0x83166310;
	sub_8316F800(ctx, base);
	// 83166310: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83166314: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83166318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316631C: 4BFFF715  bl 0x83165a30
	ctx.lr = 0x83166320;
	sub_83165A30(ctx, base);
	// 83166320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83166324: 48000038  b 0x8316635c
	pc = 0x8316635C; continue 'dispatch;
	// 83166328: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316632C: 388B72B0  addi r4, r11, 0x72b0
	ctx.r[4].s64 = ctx.r[11].s64 + 29360;
	// 83166330: 4800000C  b 0x8316633c
	pc = 0x8316633C; continue 'dispatch;
	// 83166334: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166338: 388B7284  addi r4, r11, 0x7284
	ctx.r[4].s64 = ctx.r[11].s64 + 29316;
	// 8316633C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166340: 4BFF97D9  bl 0x8315fb18
	ctx.lr = 0x83166344;
	sub_8315FB18(ctx, base);
	// 83166344: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83166348: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316634C: 409A000C  bne cr6, 0x83166358
	if !ctx.cr[6].eq {
	pc = 0x83166358; continue 'dispatch;
	}
	// 83166350: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 83166354: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83166358: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316635C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83166360: 48041E5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166368 size=132
    let mut pc: u32 = 0x83166368;
    'dispatch: loop {
        match pc {
            0x83166368 => {
    //   block [0x83166368..0x831663EC)
	// 83166368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316636C: 48041DF9  bl 0x831a8164
	ctx.lr = 0x83166370;
	sub_831A8130(ctx, base);
	// 83166370: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166374: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83166378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316637C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166380: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83166384: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83166388: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316638C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166390: 4BFFEFB9  bl 0x83165348
	ctx.lr = 0x83166394;
	sub_83165348(ctx, base);
	// 83166394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166398: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316639C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831663A0: 4BFFF1D9  bl 0x83165578
	ctx.lr = 0x831663A4;
	sub_83165578(ctx, base);
	// 831663A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831663A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831663AC: 4BFFF00D  bl 0x831653b8
	ctx.lr = 0x831663B0;
	sub_831653B8(ctx, base);
	// 831663B0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 831663B4: 409A002C  bne cr6, 0x831663e0
	if !ctx.cr[6].eq {
	pc = 0x831663E0; continue 'dispatch;
	}
	// 831663B8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831663BC: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831663C0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831663C4: 57A6043E  clrlwi r6, r29, 0x10
	ctx.r[6].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 831663C8: 388A7328  addi r4, r10, 0x7328
	ctx.r[4].s64 = ctx.r[10].s64 + 29480;
	// 831663CC: 38BEFDD4  addi r5, r30, -0x22c
	ctx.r[5].s64 = ctx.r[30].s64 + -556;
	// 831663D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831663D4: 4BFF9765  bl 0x8315fb38
	ctx.lr = 0x831663D8;
	sub_8315FB38(ctx, base);
	// 831663D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831663DC: 48000008  b 0x831663e4
	pc = 0x831663E4; continue 'dispatch;
	// 831663E0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831663E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831663E8: 48041DCC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831663F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831663F0 size=132
    let mut pc: u32 = 0x831663F0;
    'dispatch: loop {
        match pc {
            0x831663F0 => {
    //   block [0x831663F0..0x83166474)
	// 831663F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831663F4: 48041D71  bl 0x831a8164
	ctx.lr = 0x831663F8;
	sub_831A8130(ctx, base);
	// 831663F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831663FC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83166400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166408: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316640C: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83166410: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166418: 4BFFEF31  bl 0x83165348
	ctx.lr = 0x8316641C;
	sub_83165348(ctx, base);
	// 8316641C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166420: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83166424: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166428: 4BFFF151  bl 0x83165578
	ctx.lr = 0x8316642C;
	sub_83165578(ctx, base);
	// 8316642C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83166430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166434: 4BFFEF85  bl 0x831653b8
	ctx.lr = 0x83166438;
	sub_831653B8(ctx, base);
	// 83166438: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8316643C: 409A002C  bne cr6, 0x83166468
	if !ctx.cr[6].eq {
	pc = 0x83166468; continue 'dispatch;
	}
	// 83166440: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83166444: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83166448: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316644C: 57A6043E  clrlwi r6, r29, 0x10
	ctx.r[6].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 83166450: 388A7370  addi r4, r10, 0x7370
	ctx.r[4].s64 = ctx.r[10].s64 + 29552;
	// 83166454: 38BEFDD4  addi r5, r30, -0x22c
	ctx.r[5].s64 = ctx.r[30].s64 + -556;
	// 83166458: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316645C: 4BFF96DD  bl 0x8315fb38
	ctx.lr = 0x83166460;
	sub_8315FB38(ctx, base);
	// 83166460: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83166464: 48000008  b 0x8316646c
	pc = 0x8316646C; continue 'dispatch;
	// 83166468: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316646C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83166470: 48041D44  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166478 size=104
    let mut pc: u32 = 0x83166478;
    'dispatch: loop {
        match pc {
            0x83166478 => {
    //   block [0x83166478..0x831664E0)
	// 83166478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316647C: 48041CE1  bl 0x831a815c
	ctx.lr = 0x83166480;
	sub_831A8130(ctx, base);
	// 83166480: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166488: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316648C: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83166490: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83166494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166498: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8316649C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 831664A0: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 831664A4: 4BFFEEA5  bl 0x83165348
	ctx.lr = 0x831664A8;
	sub_83165348(ctx, base);
	// 831664A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831664AC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 831664B0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 831664B4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831664B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831664BC: 4BFFF6B5  bl 0x83165b70
	ctx.lr = 0x831664C0;
	sub_83165B70(ctx, base);
	// 831664C0: 815EFD08  lwz r10, -0x2f8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 831664C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831664C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831664CC: 917EFD08  stw r11, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[11].u32 ) };
	// 831664D0: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831664D4: 4BFFEEE5  bl 0x831653b8
	ctx.lr = 0x831664D8;
	sub_831653B8(ctx, base);
	// 831664D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831664DC: 48041CD0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831664E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831664E0 size=96
    let mut pc: u32 = 0x831664E0;
    'dispatch: loop {
        match pc {
            0x831664E0 => {
    //   block [0x831664E0..0x83166540)
	// 831664E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831664E4: 48041C81  bl 0x831a8164
	ctx.lr = 0x831664E8;
	sub_831A8130(ctx, base);
	// 831664E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831664EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831664F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831664F4: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 831664F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831664FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166500: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83166504: 4BFFEE45  bl 0x83165348
	ctx.lr = 0x83166508;
	sub_83165348(ctx, base);
	// 83166508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316650C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83166510: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83166514: 4BFFF78D  bl 0x83165ca0
	ctx.lr = 0x83166518;
	sub_83165CA0(ctx, base);
	// 83166518: 817EFD08  lwz r11, -0x2f8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 8316651C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83166520: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83166524: 915EFD08  stw r10, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 83166528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316652C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166530: 4BFFEE89  bl 0x831653b8
	ctx.lr = 0x83166534;
	sub_831653B8(ctx, base);
	// 83166534: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83166538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316653C: 48041C78  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166540 size=96
    let mut pc: u32 = 0x83166540;
    'dispatch: loop {
        match pc {
            0x83166540 => {
    //   block [0x83166540..0x831665A0)
	// 83166540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166544: 48041C21  bl 0x831a8164
	ctx.lr = 0x83166548;
	sub_831A8130(ctx, base);
	// 83166548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316654C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166550: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83166554: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83166558: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8316655C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166560: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83166564: 4BFFEDE5  bl 0x83165348
	ctx.lr = 0x83166568;
	sub_83165348(ctx, base);
	// 83166568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316656C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83166570: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83166574: 4BFFF7BD  bl 0x83165d30
	ctx.lr = 0x83166578;
	sub_83165D30(ctx, base);
	// 83166578: 817EFD08  lwz r11, -0x2f8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 8316657C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83166580: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83166584: 915EFD08  stw r10, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 83166588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316658C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166590: 4BFFEE29  bl 0x831653b8
	ctx.lr = 0x83166594;
	sub_831653B8(ctx, base);
	// 83166594: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83166598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316659C: 48041C18  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831665A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831665A0 size=96
    let mut pc: u32 = 0x831665A0;
    'dispatch: loop {
        match pc {
            0x831665A0 => {
    //   block [0x831665A0..0x83166600)
	// 831665A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831665A4: 48041BC1  bl 0x831a8164
	ctx.lr = 0x831665A8;
	sub_831A8130(ctx, base);
	// 831665A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831665AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831665B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831665B4: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 831665B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831665BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831665C0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831665C4: 4BFFED85  bl 0x83165348
	ctx.lr = 0x831665C8;
	sub_83165348(ctx, base);
	// 831665C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831665CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831665D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831665D4: 4BFFF815  bl 0x83165de8
	ctx.lr = 0x831665D8;
	sub_83165DE8(ctx, base);
	// 831665D8: 817EFD08  lwz r11, -0x2f8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 831665DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831665E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831665E4: 915EFD08  stw r10, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 831665E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831665EC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831665F0: 4BFFEDC9  bl 0x831653b8
	ctx.lr = 0x831665F4;
	sub_831653B8(ctx, base);
	// 831665F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831665F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831665FC: 48041BB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166600 size=80
    let mut pc: u32 = 0x83166600;
    'dispatch: loop {
        match pc {
            0x83166600 => {
    //   block [0x83166600..0x83166650)
	// 83166600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166604: 48041B65  bl 0x831a8168
	ctx.lr = 0x83166608;
	sub_831A8130(ctx, base);
	// 83166608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316660C: 3BE3FCAC  addi r31, r3, -0x354
	ctx.r[31].s64 = ctx.r[3].s64 + -852;
	// 83166610: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83166614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166618: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316661C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83166620: 4BFFED29  bl 0x83165348
	ctx.lr = 0x83166624;
	sub_83165348(ctx, base);
	// 83166624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166628: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8316662C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83166630: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166634: 4BFFF8D5  bl 0x83165f08
	ctx.lr = 0x83166638;
	sub_83165F08(ctx, base);
	// 83166638: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316663C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166640: 4BFFED79  bl 0x831653b8
	ctx.lr = 0x83166644;
	sub_831653B8(ctx, base);
	// 83166644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83166648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316664C: 48041B6C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166650 size=68
    let mut pc: u32 = 0x83166650;
    'dispatch: loop {
        match pc {
            0x83166650 => {
    //   block [0x83166650..0x83166694)
	// 83166650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166654: 48041B19  bl 0x831a816c
	ctx.lr = 0x83166658;
	sub_831A8130(ctx, base);
	// 83166658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316665C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166660: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83166664: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83166668: 4BFFECE1  bl 0x83165348
	ctx.lr = 0x8316666C;
	sub_83165348(ctx, base);
	// 8316666C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166670: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83166674: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166678: 4BFFF979  bl 0x83165ff0
	ctx.lr = 0x8316667C;
	sub_83165FF0(ctx, base);
	// 8316667C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166684: 4BFFED35  bl 0x831653b8
	ctx.lr = 0x83166688;
	sub_831653B8(ctx, base);
	// 83166688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316668C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166690: 48041B2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166698 size=108
    let mut pc: u32 = 0x83166698;
    'dispatch: loop {
        match pc {
            0x83166698 => {
    //   block [0x83166698..0x83166704)
	// 83166698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316669C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831666A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831666A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831666A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831666AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831666B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831666B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831666B8: 4BFFFAC9  bl 0x83166180
	ctx.lr = 0x831666BC;
	sub_83166180(ctx, base);
	// 831666BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831666C0: 41820024  beq 0x831666e4
	if ctx.cr[0].eq {
	pc = 0x831666E4; continue 'dispatch;
	}
	// 831666C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831666C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831666CC: 409A0018  bne cr6, 0x831666e4
	if !ctx.cr[6].eq {
	pc = 0x831666E4; continue 'dispatch;
	}
	// 831666D0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831666D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831666D8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831666DC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 831666E0: 48000010  b 0x831666f0
	pc = 0x831666F0; continue 'dispatch;
	// 831666E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831666E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831666EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831666F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831666F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831666F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831666FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166708 size=12
    let mut pc: u32 = 0x83166708;
    'dispatch: loop {
        match pc {
            0x83166708 => {
    //   block [0x83166708..0x83166714)
	// 83166708: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316670C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166710: 4BFFFC58  b 0x83166368
	sub_83166368(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166718 size=12
    let mut pc: u32 = 0x83166718;
    'dispatch: loop {
        match pc {
            0x83166718 => {
    //   block [0x83166718..0x83166724)
	// 83166718: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316671C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166720: 4BFFFE80  b 0x831665a0
	sub_831665A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166728 size=12
    let mut pc: u32 = 0x83166728;
    'dispatch: loop {
        match pc {
            0x83166728 => {
    //   block [0x83166728..0x83166734)
	// 83166728: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316672C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166730: 4BFFFDB0  b 0x831664e0
	sub_831664E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166738 size=12
    let mut pc: u32 = 0x83166738;
    'dispatch: loop {
        match pc {
            0x83166738 => {
    //   block [0x83166738..0x83166744)
	// 83166738: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316673C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166740: 4BFFFD38  b 0x83166478
	sub_83166478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166748 size=12
    let mut pc: u32 = 0x83166748;
    'dispatch: loop {
        match pc {
            0x83166748 => {
    //   block [0x83166748..0x83166754)
	// 83166748: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316674C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166750: 4BFFFCA0  b 0x831663f0
	sub_831663F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166758 size=12
    let mut pc: u32 = 0x83166758;
    'dispatch: loop {
        match pc {
            0x83166758 => {
    //   block [0x83166758..0x83166764)
	// 83166758: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316675C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166760: 4BFFFDE0  b 0x83166540
	sub_83166540(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166768 size=12
    let mut pc: u32 = 0x83166768;
    'dispatch: loop {
        match pc {
            0x83166768 => {
    //   block [0x83166768..0x83166774)
	// 83166768: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8316676C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166770: 4BFFFE90  b 0x83166600
	sub_83166600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166778 size=80
    let mut pc: u32 = 0x83166778;
    'dispatch: loop {
        match pc {
            0x83166778 => {
    //   block [0x83166778..0x831667C8)
	// 83166778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316677C: 480419ED  bl 0x831a8168
	ctx.lr = 0x83166780;
	sub_831A8130(ctx, base);
	// 83166780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166784: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166788: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316678C: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83166790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166794: 4BFFEBB5  bl 0x83165348
	ctx.lr = 0x83166798;
	sub_83165348(ctx, base);
	// 83166798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316679C: 4BFFFAC5  bl 0x83166260
	ctx.lr = 0x831667A0;
	sub_83166260(ctx, base);
	// 831667A0: 817EFD08  lwz r11, -0x2f8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-760 as u32) ) } as u64;
	// 831667A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831667A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831667AC: 915EFD08  stw r10, -0x2f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-760 as u32), ctx.r[10].u32 ) };
	// 831667B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831667B4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831667B8: 4BFFEC01  bl 0x831653b8
	ctx.lr = 0x831667BC;
	sub_831653B8(ctx, base);
	// 831667BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831667C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831667C4: 480419F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831667C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831667C8 size=132
    let mut pc: u32 = 0x831667C8;
    'dispatch: loop {
        match pc {
            0x831667C8 => {
    //   block [0x831667C8..0x8316684C)
	// 831667C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831667CC: 48041999  bl 0x831a8164
	ctx.lr = 0x831667D0;
	sub_831A8130(ctx, base);
	// 831667D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831667D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831667D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831667DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831667E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831667E4: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 831667E8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831667EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831667F0: 4BFFEB59  bl 0x83165348
	ctx.lr = 0x831667F4;
	sub_83165348(ctx, base);
	// 831667F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831667F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831667FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166800: 4BFFF7F1  bl 0x83165ff0
	ctx.lr = 0x83166804;
	sub_83165FF0(ctx, base);
	// 83166804: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83166808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316680C: 4BFFEBAD  bl 0x831653b8
	ctx.lr = 0x83166810;
	sub_831653B8(ctx, base);
	// 83166810: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83166814: 409A002C  bne cr6, 0x83166840
	if !ctx.cr[6].eq {
	pc = 0x83166840; continue 'dispatch;
	}
	// 83166818: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316681C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83166820: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166824: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83166828: 388A73B8  addi r4, r10, 0x73b8
	ctx.r[4].s64 = ctx.r[10].s64 + 29624;
	// 8316682C: 38BEFDD4  addi r5, r30, -0x22c
	ctx.r[5].s64 = ctx.r[30].s64 + -556;
	// 83166830: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83166834: 4BFF9305  bl 0x8315fb38
	ctx.lr = 0x83166838;
	sub_8315FB38(ctx, base);
	// 83166838: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316683C: 48000008  b 0x83166844
	pc = 0x83166844; continue 'dispatch;
	// 83166840: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83166844: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83166848: 4804196C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166850 size=132
    let mut pc: u32 = 0x83166850;
    'dispatch: loop {
        match pc {
            0x83166850 => {
    //   block [0x83166850..0x831668D4)
	// 83166850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166854: 48041911  bl 0x831a8164
	ctx.lr = 0x83166858;
	sub_831A8130(ctx, base);
	// 83166858: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316685C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83166860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166864: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166868: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316686C: 3BFEFCAC  addi r31, r30, -0x354
	ctx.r[31].s64 = ctx.r[30].s64 + -852;
	// 83166870: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166878: 4BFFEAD1  bl 0x83165348
	ctx.lr = 0x8316687C;
	sub_83165348(ctx, base);
	// 8316687C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166880: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83166884: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166888: 4BFFF769  bl 0x83165ff0
	ctx.lr = 0x8316688C;
	sub_83165FF0(ctx, base);
	// 8316688C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83166890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166894: 4BFFEB25  bl 0x831653b8
	ctx.lr = 0x83166898;
	sub_831653B8(ctx, base);
	// 83166898: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8316689C: 409A002C  bne cr6, 0x831668c8
	if !ctx.cr[6].eq {
	pc = 0x831668C8; continue 'dispatch;
	}
	// 831668A0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831668A4: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831668A8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831668AC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 831668B0: 388A7400  addi r4, r10, 0x7400
	ctx.r[4].s64 = ctx.r[10].s64 + 29696;
	// 831668B4: 38BEFDD4  addi r5, r30, -0x22c
	ctx.r[5].s64 = ctx.r[30].s64 + -556;
	// 831668B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831668BC: 4BFF927D  bl 0x8315fb38
	ctx.lr = 0x831668C0;
	sub_8315FB38(ctx, base);
	// 831668C0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831668C4: 48000008  b 0x831668cc
	pc = 0x831668CC; continue 'dispatch;
	// 831668C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831668CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831668D0: 480418E4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831668D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831668D8 size=12
    let mut pc: u32 = 0x831668D8;
    'dispatch: loop {
        match pc {
            0x831668D8 => {
    //   block [0x831668D8..0x831668E4)
	// 831668D8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831668DC: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831668E0: 4BFFFE98  b 0x83166778
	sub_83166778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831668E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831668E8 size=12
    let mut pc: u32 = 0x831668E8;
    'dispatch: loop {
        match pc {
            0x831668E8 => {
    //   block [0x831668E8..0x831668F4)
	// 831668E8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831668EC: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831668F0: 4BFFFF60  b 0x83166850
	sub_83166850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831668F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831668F8 size=12
    let mut pc: u32 = 0x831668F8;
    'dispatch: loop {
        match pc {
            0x831668F8 => {
    //   block [0x831668F8..0x83166904)
	// 831668F8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 831668FC: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83166900: 4BFFFEC8  b 0x831667c8
	sub_831667C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166908 size=72
    let mut pc: u32 = 0x83166908;
    'dispatch: loop {
        match pc {
            0x83166908 => {
    //   block [0x83166908..0x83166950)
	// 83166908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316690C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316691C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166920: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83166924: 396B7444  addi r11, r11, 0x7444
	ctx.r[11].s64 = ctx.r[11].s64 + 29764;
	// 83166928: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316692C: 4182000C  beq 0x83166938
	if ctx.cr[0].eq {
	pc = 0x83166938; continue 'dispatch;
	}
	// 83166930: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83166934: 4BFF934D  bl 0x8315fc80
	ctx.lr = 0x83166938;
	sub_8315FC80(ctx, base);
	// 83166938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316693C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83166940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316694C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166950 size=124
    let mut pc: u32 = 0x83166950;
    'dispatch: loop {
        match pc {
            0x83166950 => {
    //   block [0x83166950..0x831669CC)
	// 83166950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316695C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166968: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316696C: 4BFF90B5  bl 0x8315fa20
	ctx.lr = 0x83166970;
	sub_8315FA20(ctx, base);
	// 83166970: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166974: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83166978: 386B8408  addi r3, r11, -0x7bf8
	ctx.r[3].s64 = ctx.r[11].s64 + -31736;
	// 8316697C: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 83166980: 480037C9  bl 0x8316a148
	ctx.lr = 0x83166984;
	sub_8316A148(ctx, base);
	// 83166984: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166988: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316698C: 906B8404  stw r3, -0x7bfc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-31740 as u32), ctx.r[3].u32 ) };
	// 83166990: 41820010  beq 0x831669a0
	if ctx.cr[0].eq {
	pc = 0x831669A0; continue 'dispatch;
	}
	// 83166994: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166998: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316699C: 419A001C  beq cr6, 0x831669b8
	if ctx.cr[6].eq {
	pc = 0x831669B8; continue 'dispatch;
	}
	// 831669A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831669A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831669A8: 388B6CA8  addi r4, r11, 0x6ca8
	ctx.r[4].s64 = ctx.r[11].s64 + 27816;
	// 831669AC: 4BFF916D  bl 0x8315fb18
	ctx.lr = 0x831669B0;
	sub_8315FB18(ctx, base);
	// 831669B0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831669B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831669B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831669BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831669C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831669C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831669C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831669D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831669D0 size=156
    let mut pc: u32 = 0x831669D0;
    'dispatch: loop {
        match pc {
            0x831669D0 => {
    //   block [0x831669D0..0x83166A6C)
	// 831669D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831669D4: 48041791  bl 0x831a8164
	ctx.lr = 0x831669D8;
	sub_831A8130(ctx, base);
	// 831669D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831669DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831669E0: 3FC0833A  lis r30, -0x7cc6
	ctx.r[30].s64 = -2093350912;
	// 831669E4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831669E8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831669EC: 3B8B81F0  addi r28, r11, -0x7e10
	ctx.r[28].s64 = ctx.r[11].s64 + -32272;
	// 831669F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831669F4: 807E8404  lwz r3, -0x7bfc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31740 as u32) ) } as u64;
	// 831669F8: 48003711  bl 0x8316a108
	ctx.lr = 0x831669FC;
	sub_8316A108(ctx, base);
	// 831669FC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83166A00: 41820044  beq 0x83166a44
	if ctx.cr[0].eq {
	pc = 0x83166A44; continue 'dispatch;
	}
	// 83166A04: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166A08: 3B6B8328  addi r27, r11, -0x7cd8
	ctx.r[27].s64 = ctx.r[11].s64 + -31960;
	// 83166A0C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83166A10: 807E8404  lwz r3, -0x7bfc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31740 as u32) ) } as u64;
	// 83166A14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83166A18: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83166A1C: 4800387D  bl 0x8316a298
	ctx.lr = 0x83166A20;
	sub_8316A298(ctx, base);
	// 83166A20: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83166A24: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 83166A28: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83166A2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166A34: 4E800421  bctrl
	ctx.lr = 0x83166A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166A38: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83166A3C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83166A40: 409AFFCC  bne cr6, 0x83166a0c
	if !ctx.cr[6].eq {
	pc = 0x83166A0C; continue 'dispatch;
	}
	// 83166A44: 807E8404  lwz r3, -0x7bfc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-31740 as u32) ) } as u64;
	// 83166A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83166A4C: 419A0014  beq cr6, 0x83166a60
	if ctx.cr[6].eq {
	pc = 0x83166A60; continue 'dispatch;
	}
	// 83166A50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83166A54: 48003645  bl 0x8316a098
	ctx.lr = 0x83166A58;
	sub_8316A098(ctx, base);
	// 83166A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166A5C: 917E8404  stw r11, -0x7bfc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-31740 as u32), ctx.r[11].u32 ) };
	// 83166A60: 4BFF8BB9  bl 0x8315f618
	ctx.lr = 0x83166A64;
	sub_8315F618(ctx, base);
	// 83166A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83166A68: 4804174C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166A70 size=336
    let mut pc: u32 = 0x83166A70;
    'dispatch: loop {
        match pc {
            0x83166A70 => {
    //   block [0x83166A70..0x83166BC0)
	// 83166A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166A74: 480416F5  bl 0x831a8168
	ctx.lr = 0x83166A78;
	sub_831A8130(ctx, base);
	// 83166A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166A7C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83166A80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83166A84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83166A88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166A8C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83166A90: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83166A94: 38AB74F4  addi r5, r11, 0x74f4
	ctx.r[5].s64 = ctx.r[11].s64 + 29940;
	// 83166A98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83166A9C: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 83166AA0: 4BFF9259  bl 0x8315fcf8
	ctx.lr = 0x83166AA4;
	sub_8315FCF8(ctx, base);
	// 83166AA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83166AA8: 41820060  beq 0x83166b08
	if ctx.cr[0].eq {
	pc = 0x83166B08; continue 'dispatch;
	}
	// 83166AAC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166AB0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83166AB4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83166AB8: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 83166ABC: 396B747C  addi r11, r11, 0x747c
	ctx.r[11].s64 = ctx.r[11].s64 + 29820;
	// 83166AC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83166AC4: 3D200010  lis r9, 0x10
	ctx.r[9].s64 = 1048576;
	// 83166AC8: 93E30010  stw r31, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83166ACC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166AD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83166AD4: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 83166AD8: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83166ADC: F9230020  std r9, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u64 ) };
	// 83166AE0: FBE30028  std r31, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[31].u64 ) };
	// 83166AE4: FBE30030  std r31, 0x30(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[31].u64 ) };
	// 83166AE8: FBE30038  std r31, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[31].u64 ) };
	// 83166AEC: 93E30040  stw r31, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 83166AF0: 93E30044  stw r31, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[31].u32 ) };
	// 83166AF4: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 83166AF8: 93E3004C  stw r31, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[31].u32 ) };
	// 83166AFC: 93E30050  stw r31, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 83166B00: 93E30054  stw r31, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83166B04: 48000008  b 0x83166b0c
	pc = 0x83166B0C; continue 'dispatch;
	// 83166B08: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 83166B0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83166B10: 409A001C  bne cr6, 0x83166b2c
	if !ctx.cr[6].eq {
	pc = 0x83166B2C; continue 'dispatch;
	}
	// 83166B14: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166B18: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83166B1C: 388B74E8  addi r4, r11, 0x74e8
	ctx.r[4].s64 = ctx.r[11].s64 + 29928;
	// 83166B20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166B24: 4BFF901D  bl 0x8315fb40
	ctx.lr = 0x83166B28;
	sub_8315FB40(ctx, base);
	// 83166B28: 4800008C  b 0x83166bb4
	pc = 0x83166BB4; continue 'dispatch;
	// 83166B2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166B30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83166B34: 4BFF93C5  bl 0x8315fef8
	ctx.lr = 0x83166B38;
	sub_8315FEF8(ctx, base);
	// 83166B38: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83166B3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83166B40: 41820040  beq 0x83166b80
	if ctx.cr[0].eq {
	pc = 0x83166B80; continue 'dispatch;
	}
	// 83166B44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166B48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166B4C: 409A0034  bne cr6, 0x83166b80
	if !ctx.cr[6].eq {
	pc = 0x83166B80; continue 'dispatch;
	}
	// 83166B50: 93FE0048  stw r31, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 83166B54: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166B58: 93FE004C  stw r31, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[31].u32 ) };
	// 83166B5C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166B60: 93FE0050  stw r31, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 83166B64: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83166B68: 93FE0054  stw r31, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83166B6C: 38AA81F0  addi r5, r10, -0x7e10
	ctx.r[5].s64 = ctx.r[10].s64 + -32272;
	// 83166B70: 806B8404  lwz r3, -0x7bfc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31740 as u32) ) } as u64;
	// 83166B74: 480036DD  bl 0x8316a250
	ctx.lr = 0x83166B78;
	sub_8316A250(ctx, base);
	// 83166B78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83166B7C: 4800003C  b 0x83166bb8
	pc = 0x83166BB8; continue 'dispatch;
	// 83166B80: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166B84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166B88: 388B74B4  addi r4, r11, 0x74b4
	ctx.r[4].s64 = ctx.r[11].s64 + 29876;
	// 83166B8C: 4BFF8F8D  bl 0x8315fb18
	ctx.lr = 0x83166B90;
	sub_8315FB18(ctx, base);
	// 83166B90: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83166B94: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166B98: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166B9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83166BA0: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83166BA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166BA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166BB0: 4E800421  bctrl
	ctx.lr = 0x83166BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166BB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83166BBC: 480415FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166BC0 size=172
    let mut pc: u32 = 0x83166BC0;
    'dispatch: loop {
        match pc {
            0x83166BC0 => {
    //   block [0x83166BC0..0x83166C6C)
	// 83166BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166BC4: 480415A9  bl 0x831a816c
	ctx.lr = 0x83166BC8;
	sub_831A8130(ctx, base);
	// 83166BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166BCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83166BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166BD4: 93A40000  stw r29, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83166BD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83166BDC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83166BE0: 409A0008  bne cr6, 0x83166be8
	if !ctx.cr[6].eq {
	pc = 0x83166BE8; continue 'dispatch;
	}
	// 83166BE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83166BE8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166BEC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166BF0: 3BCB81F0  addi r30, r11, -0x7e10
	ctx.r[30].s64 = ctx.r[11].s64 + -32272;
	// 83166BF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83166BF8: 806A8404  lwz r3, -0x7bfc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31740 as u32) ) } as u64;
	// 83166BFC: 4800369D  bl 0x8316a298
	ctx.lr = 0x83166C00;
	sub_8316A298(ctx, base);
	// 83166C00: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83166C04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83166C08: 419A0020  beq cr6, 0x83166c28
	if ctx.cr[6].eq {
	pc = 0x83166C28; continue 'dispatch;
	}
	// 83166C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166C10: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166C14: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83166C18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166C1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166C20: 4E800421  bctrl
	ctx.lr = 0x83166C24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166C24: 93BF004C  stw r29, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[29].u32 ) };
	// 83166C28: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83166C2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83166C30: 419A001C  beq cr6, 0x83166c4c
	if ctx.cr[6].eq {
	pc = 0x83166C4C; continue 'dispatch;
	}
	// 83166C34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166C38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166C3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166C44: 4E800421  bctrl
	ctx.lr = 0x83166C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166C48: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83166C4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166C50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83166C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166C58: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83166C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166C60: 4E800421  bctrl
	ctx.lr = 0x83166C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166C64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166C68: 48041554  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166C70 size=188
    let mut pc: u32 = 0x83166C70;
    'dispatch: loop {
        match pc {
            0x83166C70 => {
    //   block [0x83166C70..0x83166D2C)
	// 83166C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166C74: 480414F5  bl 0x831a8168
	ctx.lr = 0x83166C78;
	sub_831A8130(ctx, base);
	// 83166C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166C7C: 3FC08219  lis r30, -0x7de7
	ctx.r[30].s64 = -2112290816;
	// 83166C80: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83166C84: 3BBE7508  addi r29, r30, 0x7508
	ctx.r[29].s64 = ctx.r[30].s64 + 29960;
	// 83166C88: 938A0000  stw r28, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83166C8C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83166C90: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 83166C94: 83DE7508  lwz r30, 0x7508(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(29960 as u32) ) } as u64;
	// 83166C98: 38A30058  addi r5, r3, 0x58
	ctx.r[5].s64 = ctx.r[3].s64 + 88;
	// 83166C9C: 93C30058  stw r30, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 83166CA0: 39030066  addi r8, r3, 0x66
	ctx.r[8].s64 = ctx.r[3].s64 + 102;
	// 83166CA4: A3DD0004  lhz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83166CA8: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 83166CAC: B3C3005C  sth r30, 0x5c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[30].u16 ) };
	// 83166CB0: 8BDD0006  lbz r30, 6(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 83166CB4: 3BA0000A  li r29, 0xa
	ctx.r[29].s64 = 10;
	// 83166CB8: 9BC3005E  stb r30, 0x5e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(94 as u32), ctx.r[30].u8 ) };
	// 83166CBC: 7FCBEB96  divwu r30, r11, r29
	ctx.r[30].u32 = ctx.r[11].u32 / ctx.r[29].u32;
	// 83166CC0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83166CC4: 1FDE000A  mulli r30, r30, 0xa
	ctx.r[30].s64 = ctx.r[30].s64 * 10;
	// 83166CC8: 7FDE5850  subf r30, r30, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 83166CCC: 7D6BEB96  divwu r11, r11, r29
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[29].u32;
	// 83166CD0: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 83166CD4: 9BC80000  stb r30, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 83166CD8: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83166CDC: 4082FFE0  bne 0x83166cbc
	if !ctx.cr[0].eq {
	pc = 0x83166CBC; continue 'dispatch;
	}
	// 83166CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166CE4: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 83166CE8: 9B830067  stb r28, 0x67(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(103 as u32), ctx.r[28].u8 ) };
	// 83166CEC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166CF0: 409A0020  bne cr6, 0x83166d10
	if !ctx.cr[6].eq {
	pc = 0x83166D10; continue 'dispatch;
	}
	// 83166CF4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83166CF8: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 83166CFC: 390A8328  addi r8, r10, -0x7cd8
	ctx.r[8].s64 = ctx.r[10].s64 + -31960;
	// 83166D00: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83166D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166D08: 4E800421  bctrl
	ctx.lr = 0x83166D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166D0C: 48000018  b 0x83166d24
	pc = 0x83166D24; continue 'dispatch;
	// 83166D10: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83166D14: 394A8328  addi r10, r10, -0x7cd8
	ctx.r[10].s64 = ctx.r[10].s64 + -31960;
	// 83166D18: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 83166D1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166D20: 4E800421  bctrl
	ctx.lr = 0x83166D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83166D28: 48041490  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166D30 size=36
    let mut pc: u32 = 0x83166D30;
    'dispatch: loop {
        match pc {
            0x83166D30 => {
    //   block [0x83166D30..0x83166D54)
	// 83166D30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166D34: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 83166D38: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 83166D3C: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 83166D40: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 83166D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83166D48: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83166D4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166D50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166D58 size=180
    let mut pc: u32 = 0x83166D58;
    'dispatch: loop {
        match pc {
            0x83166D58 => {
    //   block [0x83166D58..0x83166E0C)
	// 83166D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83166D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166D74: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166D78: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83166D7C: 419A0008  beq cr6, 0x83166d84
	if ctx.cr[6].eq {
	pc = 0x83166D84; continue 'dispatch;
	}
	// 83166D80: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83166D84: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83166D88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83166D8C: 409A000C  bne cr6, 0x83166d98
	if !ctx.cr[6].eq {
	pc = 0x83166D98; continue 'dispatch;
	}
	// 83166D90: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83166D94: 48000060  b 0x83166df4
	pc = 0x83166DF4; continue 'dispatch;
	// 83166D98: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166D9C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83166DA0: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 83166DA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166DA8: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 83166DAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166DB0: 4E800421  bctrl
	ctx.lr = 0x83166DB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166DB4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83166DB8: 4182001C  beq 0x83166dd4
	if ctx.cr[0].eq {
	pc = 0x83166DD4; continue 'dispatch;
	}
	// 83166DBC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83166DC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166DC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166DC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83166DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166DD0: 4E800421  bctrl
	ctx.lr = 0x83166DD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166DD4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83166DD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83166DDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166DE0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83166DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166DE8: 4E800421  bctrl
	ctx.lr = 0x83166DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166DEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83166DF0: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83166DF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166E00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83166E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166E10 size=112
    let mut pc: u32 = 0x83166E10;
    'dispatch: loop {
        match pc {
            0x83166E10 => {
    //   block [0x83166E10..0x83166E80)
	// 83166E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166E18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166E1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166E24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166E28: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83166E2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166E30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83166E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166E38: 4E800421  bctrl
	ctx.lr = 0x83166E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166E3C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166E40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166E44: 419A0028  beq cr6, 0x83166e6c
	if ctx.cr[6].eq {
	pc = 0x83166E6C; continue 'dispatch;
	}
	// 83166E48: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166E4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166E50: 388B7510  addi r4, r11, 0x7510
	ctx.r[4].s64 = ctx.r[11].s64 + 29968;
	// 83166E54: 4BFF8CC5  bl 0x8315fb18
	ctx.lr = 0x83166E58;
	sub_8315FB18(ctx, base);
	// 83166E58: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83166E5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166E60: 409A000C  bne cr6, 0x83166e6c
	if !ctx.cr[6].eq {
	pc = 0x83166E6C; continue 'dispatch;
	}
	// 83166E64: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166E68: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83166E6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166E80 size=112
    let mut pc: u32 = 0x83166E80;
    'dispatch: loop {
        match pc {
            0x83166E80 => {
    //   block [0x83166E80..0x83166EF0)
	// 83166E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166E8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166E94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166E98: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83166E9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83166EA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83166EA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83166EA8: 4E800421  bctrl
	ctx.lr = 0x83166EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83166EAC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166EB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166EB4: 419A0028  beq cr6, 0x83166edc
	if ctx.cr[6].eq {
	pc = 0x83166EDC; continue 'dispatch;
	}
	// 83166EB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166EBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166EC0: 388B7540  addi r4, r11, 0x7540
	ctx.r[4].s64 = ctx.r[11].s64 + 30016;
	// 83166EC4: 4BFF8C55  bl 0x8315fb18
	ctx.lr = 0x83166EC8;
	sub_8315FB18(ctx, base);
	// 83166EC8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83166ECC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166ED0: 409A000C  bne cr6, 0x83166edc
	if !ctx.cr[6].eq {
	pc = 0x83166EDC; continue 'dispatch;
	}
	// 83166ED4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166ED8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83166EDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166EE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166EF0 size=84
    let mut pc: u32 = 0x83166EF0;
    'dispatch: loop {
        match pc {
            0x83166EF0 => {
    //   block [0x83166EF0..0x83166F44)
	// 83166EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166EF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166EFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166F00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83166F04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166F08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83166F0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166F10: 4BFFFB61  bl 0x83166a70
	ctx.lr = 0x83166F14;
	sub_83166A70(ctx, base);
	// 83166F14: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166F18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83166F1C: 4182000C  beq 0x83166f28
	if ctx.cr[0].eq {
	pc = 0x83166F28; continue 'dispatch;
	}
	// 83166F20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166F24: 419A000C  beq cr6, 0x83166f30
	if ctx.cr[6].eq {
	pc = 0x83166F30; continue 'dispatch;
	}
	// 83166F28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83166F30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166F48 size=72
    let mut pc: u32 = 0x83166F48;
    'dispatch: loop {
        match pc {
            0x83166F48 => {
    //   block [0x83166F48..0x83166F90)
	// 83166F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166F50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166F54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166F58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166F5C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83166F60: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83166F64: 396B7444  addi r11, r11, 0x7444
	ctx.r[11].s64 = ctx.r[11].s64 + 29764;
	// 83166F68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166F6C: 4182000C  beq 0x83166f78
	if ctx.cr[0].eq {
	pc = 0x83166F78; continue 'dispatch;
	}
	// 83166F70: 38800068  li r4, 0x68
	ctx.r[4].s64 = 104;
	// 83166F74: 4BFF8D0D  bl 0x8315fc80
	ctx.lr = 0x83166F78;
	sub_8315FC80(ctx, base);
	// 83166F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83166F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83166F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166F88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83166F90 size=96
    let mut pc: u32 = 0x83166F90;
    'dispatch: loop {
        match pc {
            0x83166F90 => {
    //   block [0x83166F90..0x83166FF0)
	// 83166F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83166F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83166F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83166F9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83166FA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83166FA4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166FAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166FB0: 816A8400  lwz r11, -0x7c00(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31744 as u32) ) } as u64;
	// 83166FB4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83166FB8: 916A8400  stw r11, -0x7c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31744 as u32), ctx.r[11].u32 ) };
	// 83166FBC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83166FC0: 409A001C  bne cr6, 0x83166fdc
	if !ctx.cr[6].eq {
	pc = 0x83166FDC; continue 'dispatch;
	}
	// 83166FC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83166FC8: 4BFFF989  bl 0x83166950
	ctx.lr = 0x83166FCC;
	sub_83166950(ctx, base);
	// 83166FCC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83166FD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83166FD4: 419A0008  beq cr6, 0x83166fdc
	if ctx.cr[6].eq {
	pc = 0x83166FDC; continue 'dispatch;
	}
	// 83166FD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83166FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83166FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83166FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83166FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83166FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83166FF0 size=28
    let mut pc: u32 = 0x83166FF0;
    'dispatch: loop {
        match pc {
            0x83166FF0 => {
    //   block [0x83166FF0..0x8316700C)
	// 83166FF0: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83166FF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83166FF8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83166FFC: 816A8400  lwz r11, -0x7c00(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31744 as u32) ) } as u64;
	// 83167000: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167004: 916A8400  stw r11, -0x7c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31744 as u32), ctx.r[11].u32 ) };
	// 83167008: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316700C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316700C size=12
    let mut pc: u32 = 0x8316700C;
    'dispatch: loop {
        match pc {
            0x8316700C => {
    //   block [0x8316700C..0x83167018)
	// 8316700C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83167010: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 83167014: 4BFFF9BC  b 0x831669d0
	sub_831669D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167018 size=4
    let mut pc: u32 = 0x83167018;
    'dispatch: loop {
        match pc {
            0x83167018 => {
    //   block [0x83167018..0x8316701C)
	// 83167018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167020 size=64
    let mut pc: u32 = 0x83167020;
    'dispatch: loop {
        match pc {
            0x83167020 => {
    //   block [0x83167020..0x83167060)
	// 83167020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167024: 48041149  bl 0x831a816c
	ctx.lr = 0x83167028;
	sub_831A8130(ctx, base);
	// 83167028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316702C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83167034: 4BFFFDDD  bl 0x83166e10
	ctx.lr = 0x83167038;
	sub_83166E10(ctx, base);
	// 83167038: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316703C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83167040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167044: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167048: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316704C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167050: 4BFFFE31  bl 0x83166e80
	ctx.lr = 0x83167054;
	sub_83166E80(ctx, base);
	// 83167054: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83167058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316705C: 48041160  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167060 size=104
    let mut pc: u32 = 0x83167060;
    'dispatch: loop {
        match pc {
            0x83167060 => {
    //   block [0x83167060..0x831670C8)
	// 83167060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316706C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167074: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167078: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 8316707C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83167080: 419A0034  beq cr6, 0x831670b4
	if ctx.cr[6].eq {
	pc = 0x831670B4; continue 'dispatch;
	}
	// 83167084: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167088: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316708C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83167090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167094: 4E800421  bctrl
	ctx.lr = 0x83167098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167098: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316709C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831670A0: 419A0014  beq cr6, 0x831670b4
	if ctx.cr[6].eq {
	pc = 0x831670B4; continue 'dispatch;
	}
	// 831670A4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831670A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831670AC: 409A0008  bne cr6, 0x831670b4
	if !ctx.cr[6].eq {
	pc = 0x831670B4; continue 'dispatch;
	}
	// 831670B0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831670B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831670B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831670BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831670C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831670C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831670C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831670C8 size=64
    let mut pc: u32 = 0x831670C8;
    'dispatch: loop {
        match pc {
            0x831670C8 => {
    //   block [0x831670C8..0x83167108)
	// 831670C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831670CC: 480410A1  bl 0x831a816c
	ctx.lr = 0x831670D0;
	sub_831A8130(ctx, base);
	// 831670D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831670D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831670D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831670DC: 4BFFFD35  bl 0x83166e10
	ctx.lr = 0x831670E0;
	sub_83166E10(ctx, base);
	// 831670E0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831670E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831670E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831670EC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831670F0: EBBF0020  ld r29, 0x20(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 831670F4: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831670F8: 4BFFFD89  bl 0x83166e80
	ctx.lr = 0x831670FC;
	sub_83166E80(ctx, base);
	// 831670FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83167100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83167104: 480410B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167108 size=104
    let mut pc: u32 = 0x83167108;
    'dispatch: loop {
        match pc {
            0x83167108 => {
    //   block [0x83167108..0x83167170)
	// 83167108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316710C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83167114: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316711C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167120: F89F0020  std r4, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u64 ) };
	// 83167124: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83167128: 419A0034  beq cr6, 0x8316715c
	if ctx.cr[6].eq {
	pc = 0x8316715C; continue 'dispatch;
	}
	// 8316712C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167130: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83167134: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83167138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316713C: 4E800421  bctrl
	ctx.lr = 0x83167140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167140: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83167144: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167148: 419A0014  beq cr6, 0x8316715c
	if ctx.cr[6].eq {
	pc = 0x8316715C; continue 'dispatch;
	}
	// 8316714C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167150: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83167154: 409A0008  bne cr6, 0x8316715c
	if !ctx.cr[6].eq {
	pc = 0x8316715C; continue 'dispatch;
	}
	// 83167158: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8316715C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83167160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83167164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83167168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167170 size=844
    let mut pc: u32 = 0x83167170;
    'dispatch: loop {
        match pc {
            0x83167170 => {
    //   block [0x83167170..0x831674BC)
	// 83167170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167174: 48040FE1  bl 0x831a8154
	ctx.lr = 0x83167178;
	sub_831A8130(ctx, base);
	// 83167178: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316717C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83167180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167184: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83167188: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8316718C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83167190: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 83167194: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 83167198: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8316719C: 419A02F4  beq cr6, 0x83167490
	if ctx.cr[6].eq {
	pc = 0x83167490; continue 'dispatch;
	}
	// 831671A0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831671A4: 419A02EC  beq cr6, 0x83167490
	if ctx.cr[6].eq {
	pc = 0x83167490; continue 'dispatch;
	}
	// 831671A8: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831671AC: 419A02E4  beq cr6, 0x83167490
	if ctx.cr[6].eq {
	pc = 0x83167490; continue 'dispatch;
	}
	// 831671B0: 2F3A0000  cmpdi cr6, r26, 0
	ctx.cr[6].compare_i64(ctx.r[26].s64, 0, &mut ctx.xer);
	// 831671B4: 419802D0  blt cr6, 0x83167484
	if ctx.cr[6].lt {
	pc = 0x83167484; continue 'dispatch;
	}
	// 831671B8: 2F390000  cmpdi cr6, r25, 0
	ctx.cr[6].compare_i64(ctx.r[25].s64, 0, &mut ctx.xer);
	// 831671BC: 419802C8  blt cr6, 0x83167484
	if ctx.cr[6].lt {
	pc = 0x83167484; continue 'dispatch;
	}
	// 831671C0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831671C4: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 831671C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831671CC: 409A0024  bne cr6, 0x831671f0
	if !ctx.cr[6].eq {
	pc = 0x831671F0; continue 'dispatch;
	}
	// 831671D0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831671D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831671D8: 388B76D0  addi r4, r11, 0x76d0
	ctx.r[4].s64 = ctx.r[11].s64 + 30416;
	// 831671DC: 4BFF893D  bl 0x8315fb18
	ctx.lr = 0x831671E0;
	sub_8315FB18(ctx, base);
	// 831671E0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831671E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831671E8: 409A0008  bne cr6, 0x831671f0
	if !ctx.cr[6].eq {
	pc = 0x831671F0; continue 'dispatch;
	}
	// 831671EC: 92FF0014  stw r23, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[23].u32 ) };
	// 831671F0: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831671F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831671F8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831671FC: 93BF0048  stw r29, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 83167200: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83167204: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83167208: FBDF0028  std r30, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u64 ) };
	// 8316720C: 3BAB8328  addi r29, r11, -0x7cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -31960;
	// 83167210: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 83167214: 419A001C  beq cr6, 0x83167230
	if ctx.cr[6].eq {
	pc = 0x83167230; continue 'dispatch;
	}
	// 83167218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316721C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83167220: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167228: 4E800421  bctrl
	ctx.lr = 0x8316722C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316722C: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 83167230: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83167234: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83167238: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316723C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167240: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167244: 4E800421  bctrl
	ctx.lr = 0x83167248;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167248: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8316724C: 41820214  beq 0x83167460
	if ctx.cr[0].eq {
	pc = 0x83167460; continue 'dispatch;
	}
	// 83167250: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83167254: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167258: 409A0208  bne cr6, 0x83167460
	if !ctx.cr[6].eq {
	pc = 0x83167460; continue 'dispatch;
	}
	// 8316725C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167260: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83167264: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316726C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83167270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167274: 4E800421  bctrl
	ctx.lr = 0x83167278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167278: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316727C: E89F0020  ld r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 83167280: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83167284: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167288: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316728C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167290: 4E800421  bctrl
	ctx.lr = 0x83167294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167294: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167298: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316729C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831672A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831672A4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831672A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831672AC: 4E800421  bctrl
	ctx.lr = 0x831672B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831672B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831672B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831672B8: 419A0044  beq cr6, 0x831672fc
	if ctx.cr[6].eq {
	pc = 0x831672FC; continue 'dispatch;
	}
	// 831672BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831672C0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831672C4: 388B7698  addi r4, r11, 0x7698
	ctx.r[4].s64 = ctx.r[11].s64 + 30360;
	// 831672C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831672CC: 4BFF885D  bl 0x8315fb28
	ctx.lr = 0x831672D0;
	sub_8315FB28(ctx, base);
	// 831672D0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831672D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831672D8: 409A0008  bne cr6, 0x831672e0
	if !ctx.cr[6].eq {
	pc = 0x831672E0; continue 'dispatch;
	}
	// 831672DC: 92FF0014  stw r23, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[23].u32 ) };
	// 831672E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831672E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831672E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831672EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831672F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831672F4: 4E800421  bctrl
	ctx.lr = 0x831672F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831672F8: 480001BC  b 0x831674b4
	pc = 0x831674B4; continue 'dispatch;
	// 831672FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167300: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83167304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83167308: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316730C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167310: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167318: 4E800421  bctrl
	ctx.lr = 0x8316731C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316731C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83167320: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167324: 419A0014  beq cr6, 0x83167338
	if ctx.cr[6].eq {
	pc = 0x83167338; continue 'dispatch;
	}
	// 83167328: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316732C: 5745003E  slwi r5, r26, 0
	ctx.r[5].u32 = ctx.r[26].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83167330: 388B7664  addi r4, r11, 0x7664
	ctx.r[4].s64 = ctx.r[11].s64 + 30308;
	// 83167334: 4BFFFF94  b 0x831672c8
	pc = 0x831672C8; continue 'dispatch;
	// 83167338: 2F3B0000  cmpdi cr6, r27, 0
	ctx.cr[6].compare_i64(ctx.r[27].s64, 0, &mut ctx.xer);
	// 8316733C: 4098000C  bge cr6, 0x83167348
	if !ctx.cr[6].lt {
	pc = 0x83167348; continue 'dispatch;
	}
	// 83167340: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 83167344: 48000090  b 0x831673d4
	pc = 0x831673D4; continue 'dispatch;
	// 83167348: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316734C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83167350: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 83167354: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83167358: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8316735C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167360: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 83167364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167368: 4E800421  bctrl
	ctx.lr = 0x8316736C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316736C: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83167370: 7F2BD800  cmpd cr6, r11, r27
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[27].s64, &mut ctx.xer);
	// 83167374: 40980060  bge cr6, 0x831673d4
	if !ctx.cr[6].lt {
	pc = 0x831673D4; continue 'dispatch;
	}
	// 83167378: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316737C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83167380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167384: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83167388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316738C: 4E800421  bctrl
	ctx.lr = 0x83167390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167390: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167394: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83167398: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8316739C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 831673A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 831673A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831673A8: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 831673AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831673B0: 4E800421  bctrl
	ctx.lr = 0x831673B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831673B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831673B8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 831673BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831673C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 831673C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831673C8: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 831673CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831673D0: 4E800421  bctrl
	ctx.lr = 0x831673D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831673D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831673D8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831673DC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831673E0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 831673E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831673E8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831673EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831673F0: 4E800421  bctrl
	ctx.lr = 0x831673F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831673F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831673F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831673FC: 419A0014  beq cr6, 0x83167410
	if ctx.cr[6].eq {
	pc = 0x83167410; continue 'dispatch;
	}
	// 83167400: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167404: 5785003E  slwi r5, r28, 0
	ctx.r[5].u32 = ctx.r[28].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83167408: 388B7628  addi r4, r11, 0x7628
	ctx.r[4].s64 = ctx.r[11].s64 + 30248;
	// 8316740C: 4BFFFEBC  b 0x831672c8
	pc = 0x831672C8; continue 'dispatch;
	// 83167410: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167414: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83167418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316741C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83167420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83167424: 4E800421  bctrl
	ctx.lr = 0x83167428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167428: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316742C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167430: 419A0018  beq cr6, 0x83167448
	if ctx.cr[6].eq {
	pc = 0x83167448; continue 'dispatch;
	}
	// 83167434: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167438: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316743C: 388B75F8  addi r4, r11, 0x75f8
	ctx.r[4].s64 = ctx.r[11].s64 + 30200;
	// 83167440: 4BFF86D9  bl 0x8315fb18
	ctx.lr = 0x83167444;
	sub_8315FB18(ctx, base);
	// 83167444: 4BFFFE8C  b 0x831672d0
	pc = 0x831672D0; continue 'dispatch;
	// 83167448: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316744C: FB7F0030  std r27, 0x30(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[27].u64 ) };
	// 83167450: FB3F0038  std r25, 0x38(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[25].u64 ) };
	// 83167454: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83167458: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8316745C: 48000058  b 0x831674b4
	pc = 0x831674B4; continue 'dispatch;
	// 83167460: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167464: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167468: 388B75C0  addi r4, r11, 0x75c0
	ctx.r[4].s64 = ctx.r[11].s64 + 30144;
	// 8316746C: 4BFF86AD  bl 0x8315fb18
	ctx.lr = 0x83167470;
	sub_8315FB18(ctx, base);
	// 83167470: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167474: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167478: 409A003C  bne cr6, 0x831674b4
	if !ctx.cr[6].eq {
	pc = 0x831674B4; continue 'dispatch;
	}
	// 8316747C: 92FF0014  stw r23, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[23].u32 ) };
	// 83167480: 48000034  b 0x831674b4
	pc = 0x831674B4; continue 'dispatch;
	// 83167484: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167488: 388B7594  addi r4, r11, 0x7594
	ctx.r[4].s64 = ctx.r[11].s64 + 30100;
	// 8316748C: 4800000C  b 0x83167498
	pc = 0x83167498; continue 'dispatch;
	// 83167490: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167494: 388B7570  addi r4, r11, 0x7570
	ctx.r[4].s64 = ctx.r[11].s64 + 30064;
	// 83167498: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316749C: 4BFF867D  bl 0x8315fb18
	ctx.lr = 0x831674A0;
	sub_8315FB18(ctx, base);
	// 831674A0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831674A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831674A8: 409A000C  bne cr6, 0x831674b4
	if !ctx.cr[6].eq {
	pc = 0x831674B4; continue 'dispatch;
	}
	// 831674AC: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 831674B0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831674B4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 831674B8: 48040CEC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831674C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831674C0 size=432
    let mut pc: u32 = 0x831674C0;
    'dispatch: loop {
        match pc {
            0x831674C0 => {
    //   block [0x831674C0..0x83167670)
	// 831674C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831674C4: 48040C99  bl 0x831a815c
	ctx.lr = 0x831674C8;
	sub_831A8130(ctx, base);
	// 831674C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831674CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831674D0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831674D4: 7F59D378  mr r25, r26
	ctx.r[25].u64 = ctx.r[26].u64;
	// 831674D8: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831674DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831674E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831674E4: 419A0184  beq cr6, 0x83167668
	if ctx.cr[6].eq {
	pc = 0x83167668; continue 'dispatch;
	}
	// 831674E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831674EC: 409A017C  bne cr6, 0x83167668
	if !ctx.cr[6].eq {
	pc = 0x83167668; continue 'dispatch;
	}
	// 831674F0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831674F4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831674F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 831674FC: 3BCA8328  addi r30, r10, -0x7cd8
	ctx.r[30].s64 = ctx.r[10].s64 + -31960;
	// 83167500: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83167504: 81690034  lwz r11, 0x34(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 83167508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316750C: 4E800421  bctrl
	ctx.lr = 0x83167510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167510: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167514: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83167518: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316751C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83167520: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167524: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 83167528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316752C: 4E800421  bctrl
	ctx.lr = 0x83167530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167530: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167534: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83167538: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316753C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83167540: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167544: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 83167548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316754C: 4E800421  bctrl
	ctx.lr = 0x83167550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167550: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167554: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83167558: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316755C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83167560: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167564: 816A005C  lwz r11, 0x5c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(92 as u32) ) } as u64;
	// 83167568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316756C: 4E800421  bctrl
	ctx.lr = 0x83167570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83167570: F87F0028  std r3, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u64 ) };
	// 83167574: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 83167578: 419A00EC  beq cr6, 0x83167664
	if ctx.cr[6].eq {
	pc = 0x83167664; continue 'dispatch;
	}
	// 8316757C: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 83167580: 419A00E4  beq cr6, 0x83167664
	if ctx.cr[6].eq {
	pc = 0x83167664; continue 'dispatch;
	}
	// 83167584: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 83167588: 419A00DC  beq cr6, 0x83167664
	if ctx.cr[6].eq {
	pc = 0x83167664; continue 'dispatch;
	}
	// 8316758C: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 83167590: 409A0020  bne cr6, 0x831675b0
	if !ctx.cr[6].eq {
	pc = 0x831675B0; continue 'dispatch;
	}
	// 83167594: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83167598: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316759C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831675A0: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 831675A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831675A8: 4E800421  bctrl
	ctx.lr = 0x831675AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831675AC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 831675B0: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831675B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831675B8: 419A001C  beq cr6, 0x831675d4
	if ctx.cr[6].eq {
	pc = 0x831675D4; continue 'dispatch;
	}
	// 831675BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831675C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831675C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831675C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831675CC: 4E800421  bctrl
	ctx.lr = 0x831675D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831675D0: 935F004C  stw r26, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[26].u32 ) };
	// 831675D4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 831675D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831675DC: 419A0010  beq cr6, 0x831675ec
	if ctx.cr[6].eq {
	pc = 0x831675EC; continue 'dispatch;
	}
	// 831675E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831675E4: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 831675E8: 48000080  b 0x83167668
	pc = 0x83167668; continue 'dispatch;
	// 831675EC: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 831675F0: 419A0058  beq cr6, 0x83167648
	if ctx.cr[6].eq {
	pc = 0x83167648; continue 'dispatch;
	}
	// 831675F4: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 831675F8: 419A0050  beq cr6, 0x83167648
	if ctx.cr[6].eq {
	pc = 0x83167648; continue 'dispatch;
	}
	// 831675FC: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 83167600: 419A0048  beq cr6, 0x83167648
	if ctx.cr[6].eq {
	pc = 0x83167648; continue 'dispatch;
	}
	// 83167604: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 83167608: 409A005C  bne cr6, 0x83167664
	if !ctx.cr[6].eq {
	pc = 0x83167664; continue 'dispatch;
	}
	// 8316760C: E97F0030  ld r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	// 83167610: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83167614: 40980044  bge cr6, 0x83167658
	if !ctx.cr[6].lt {
	pc = 0x83167658; continue 'dispatch;
	}
	// 83167618: E97F0038  ld r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 8316761C: 7F2BC800  cmpd cr6, r11, r25
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[25].s64, &mut ctx.xer);
	// 83167620: 40980038  bge cr6, 0x83167658
	if !ctx.cr[6].lt {
	pc = 0x83167658; continue 'dispatch;
	}
	// 83167624: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167628: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316762C: 388B7720  addi r4, r11, 0x7720
	ctx.r[4].s64 = ctx.r[11].s64 + 30496;
	// 83167630: 4BFF84E9  bl 0x8315fb18
	ctx.lr = 0x83167634;
	sub_8315FB18(ctx, base);
	// 83167634: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167638: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316763C: 409A000C  bne cr6, 0x83167648
	if !ctx.cr[6].eq {
	pc = 0x83167648; continue 'dispatch;
	}
	// 83167640: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 83167644: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167648: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316764C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83167650: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83167654: 48000014  b 0x83167668
	pc = 0x83167668; continue 'dispatch;
	// 83167658: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316765C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83167660: 4BFFFFF0  b 0x83167650
	pc = 0x83167650; continue 'dispatch;
	// 83167664: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83167668: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316766C: 48040B40  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167670 size=64
    let mut pc: u32 = 0x83167670;
    'dispatch: loop {
        match pc {
            0x83167670 => {
    //   block [0x83167670..0x831676B0)
	// 83167670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167674: 48040AF9  bl 0x831a816c
	ctx.lr = 0x83167678;
	sub_831A8130(ctx, base);
	// 83167678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316767C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83167684: 4BFFF78D  bl 0x83166e10
	ctx.lr = 0x83167688;
	sub_83166E10(ctx, base);
	// 83167688: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316768C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83167690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167694: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167698: 83BF0054  lwz r29, 0x54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316769C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831676A0: 4BFFF7E1  bl 0x83166e80
	ctx.lr = 0x831676A4;
	sub_83166E80(ctx, base);
	// 831676A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831676A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831676AC: 48040B10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831676B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831676B0 size=88
    let mut pc: u32 = 0x831676B0;
    'dispatch: loop {
        match pc {
            0x831676B0 => {
    //   block [0x831676B0..0x83167708)
	// 831676B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831676B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831676B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831676BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831676C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831676C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831676C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831676CC: 4BFFF745  bl 0x83166e10
	ctx.lr = 0x831676D0;
	sub_83166E10(ctx, base);
	// 831676D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831676D4: 4BFFF685  bl 0x83166d58
	ctx.lr = 0x831676D8;
	sub_83166D58(ctx, base);
	// 831676D8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831676DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831676E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831676E4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831676E8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831676EC: 4BFFF795  bl 0x83166e80
	ctx.lr = 0x831676F0;
	sub_83166E80(ctx, base);
	// 831676F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831676F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831676F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831676FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83167700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83167704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167708 size=80
    let mut pc: u32 = 0x83167708;
    'dispatch: loop {
        match pc {
            0x83167708 => {
    //   block [0x83167708..0x83167758)
	// 83167708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316770C: 48040A61  bl 0x831a816c
	ctx.lr = 0x83167710;
	sub_831A8130(ctx, base);
	// 83167710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167718: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316771C: 4BFFF6F5  bl 0x83166e10
	ctx.lr = 0x83167720;
	sub_83166E10(ctx, base);
	// 83167720: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83167724: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83167728: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316772C: 419A0008  beq cr6, 0x83167734
	if ctx.cr[6].eq {
	pc = 0x83167734; continue 'dispatch;
	}
	// 83167730: EBDF0028  ld r30, 0x28(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 83167734: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316773C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167740: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83167744: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83167748: 4BFFF739  bl 0x83166e80
	ctx.lr = 0x8316774C;
	sub_83166E80(ctx, base);
	// 8316774C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83167754: 48040A68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167758 size=72
    let mut pc: u32 = 0x83167758;
    'dispatch: loop {
        match pc {
            0x83167758 => {
    //   block [0x83167758..0x831677A0)
	// 83167758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316775C: 48040A11  bl 0x831a816c
	ctx.lr = 0x83167760;
	sub_831A8130(ctx, base);
	// 83167760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167768: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316776C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83167770: 4BFFF6A1  bl 0x83166e10
	ctx.lr = 0x83167774;
	sub_83166E10(ctx, base);
	// 83167774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167778: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316777C: 4BFFF8E5  bl 0x83167060
	ctx.lr = 0x83167780;
	sub_83167060(ctx, base);
	// 83167780: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167784: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83167788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316778C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167790: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167794: 4BFFF6ED  bl 0x83166e80
	ctx.lr = 0x83167798;
	sub_83166E80(ctx, base);
	// 83167798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316779C: 48040A20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831677A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831677A0 size=72
    let mut pc: u32 = 0x831677A0;
    'dispatch: loop {
        match pc {
            0x831677A0 => {
    //   block [0x831677A0..0x831677E8)
	// 831677A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831677A4: 480409C9  bl 0x831a816c
	ctx.lr = 0x831677A8;
	sub_831A8130(ctx, base);
	// 831677A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831677AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831677B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831677B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831677B8: 4BFFF659  bl 0x83166e10
	ctx.lr = 0x831677BC;
	sub_83166E10(ctx, base);
	// 831677BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831677C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831677C4: 4BFFF945  bl 0x83167108
	ctx.lr = 0x831677C8;
	sub_83167108(ctx, base);
	// 831677C8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831677CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831677D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831677D4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831677D8: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831677DC: 4BFFF6A5  bl 0x83166e80
	ctx.lr = 0x831677E0;
	sub_83166E80(ctx, base);
	// 831677E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831677E4: 480409D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831677E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831677E8 size=116
    let mut pc: u32 = 0x831677E8;
    'dispatch: loop {
        match pc {
            0x831677E8 => {
    //   block [0x831677E8..0x8316785C)
	// 831677E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831677EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831677F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831677F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831677F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831677FC: 2F270000  cmpdi cr6, r7, 0
	ctx.cr[6].compare_i64(ctx.r[7].s64, 0, &mut ctx.xer);
	// 83167800: 4098002C  bge cr6, 0x8316782c
	if !ctx.cr[6].lt {
	pc = 0x8316782C; continue 'dispatch;
	}
	// 83167804: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167808: 388B779C  addi r4, r11, 0x779c
	ctx.r[4].s64 = ctx.r[11].s64 + 30620;
	// 8316780C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167810: 4BFF8309  bl 0x8315fb18
	ctx.lr = 0x83167814;
	sub_8315FB18(ctx, base);
	// 83167814: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167818: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316781C: 409A002C  bne cr6, 0x83167848
	if !ctx.cr[6].eq {
	pc = 0x83167848; continue 'dispatch;
	}
	// 83167820: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 83167824: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167828: 48000020  b 0x83167848
	pc = 0x83167848; continue 'dispatch;
	// 8316782C: 7F274800  cmpd cr6, r7, r9
	ctx.cr[6].compare_i64(ctx.r[7].s64, ctx.r[9].s64, &mut ctx.xer);
	// 83167830: 40990010  ble cr6, 0x83167840
	if !ctx.cr[6].gt {
	pc = 0x83167840; continue 'dispatch;
	}
	// 83167834: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167838: 388B775C  addi r4, r11, 0x775c
	ctx.r[4].s64 = ctx.r[11].s64 + 30556;
	// 8316783C: 4BFFFFD0  b 0x8316780c
	pc = 0x8316780C; continue 'dispatch;
	// 83167840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167844: 4BFFF92D  bl 0x83167170
	ctx.lr = 0x83167848;
	sub_83167170(ctx, base);
	// 83167848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316784C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83167850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83167854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83167858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167860 size=72
    let mut pc: u32 = 0x83167860;
    'dispatch: loop {
        match pc {
            0x83167860 => {
    //   block [0x83167860..0x831678A8)
	// 83167860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167864: 48040909  bl 0x831a816c
	ctx.lr = 0x83167868;
	sub_831A8130(ctx, base);
	// 83167868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316786C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167870: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83167874: 4BFFF59D  bl 0x83166e10
	ctx.lr = 0x83167878;
	sub_83166E10(ctx, base);
	// 83167878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316787C: 4BFFFC45  bl 0x831674c0
	ctx.lr = 0x83167880;
	sub_831674C0(ctx, base);
	// 83167880: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167884: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83167888: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316788C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83167890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167894: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83167898: 4BFFF5E9  bl 0x83166e80
	ctx.lr = 0x8316789C;
	sub_83166E80(ctx, base);
	// 8316789C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831678A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831678A4: 48040918  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831678A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831678A8 size=104
    let mut pc: u32 = 0x831678A8;
    'dispatch: loop {
        match pc {
            0x831678A8 => {
    //   block [0x831678A8..0x83167910)
	// 831678A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831678AC: 480408B5  bl 0x831a8160
	ctx.lr = 0x831678B0;
	sub_831A8130(ctx, base);
	// 831678B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831678B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831678B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831678BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831678C0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831678C4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 831678C8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 831678CC: 4BFFF545  bl 0x83166e10
	ctx.lr = 0x831678D0;
	sub_83166E10(ctx, base);
	// 831678D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831678D4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 831678D8: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 831678DC: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 831678E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831678E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831678E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831678EC: 4BFFF885  bl 0x83167170
	ctx.lr = 0x831678F0;
	sub_83167170(ctx, base);
	// 831678F0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831678F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831678F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831678FC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167900: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167904: 4BFFF57D  bl 0x83166e80
	ctx.lr = 0x83167908;
	sub_83166E80(ctx, base);
	// 83167908: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316790C: 480408A4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167910 size=112
    let mut pc: u32 = 0x83167910;
    'dispatch: loop {
        match pc {
            0x83167910 => {
    //   block [0x83167910..0x83167980)
	// 83167910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167914: 48040845  bl 0x831a8158
	ctx.lr = 0x83167918;
	sub_831A8130(ctx, base);
	// 83167918: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316791C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83167924: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83167928: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316792C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83167930: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 83167934: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 83167938: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 8316793C: 4BFFF4D5  bl 0x83166e10
	ctx.lr = 0x83167940;
	sub_83166E10(ctx, base);
	// 83167940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167944: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 83167948: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 8316794C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83167950: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83167954: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83167958: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316795C: 4BFFFE8D  bl 0x831677e8
	ctx.lr = 0x83167960;
	sub_831677E8(ctx, base);
	// 83167960: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83167968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316796C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83167970: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167974: 4BFFF50D  bl 0x83166e80
	ctx.lr = 0x83167978;
	sub_83166E80(ctx, base);
	// 83167978: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8316797C: 4804082C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167980 size=68
    let mut pc: u32 = 0x83167980;
    'dispatch: loop {
        match pc {
            0x83167980 => {
    //   block [0x83167980..0x831679C4)
	// 83167980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316798C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167994: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167998: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316799C: 396B77C8  addi r11, r11, 0x77c8
	ctx.r[11].s64 = ctx.r[11].s64 + 30664;
	// 831679A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831679A4: 41820008  beq 0x831679ac
	if ctx.cr[0].eq {
	pc = 0x831679AC; continue 'dispatch;
	}
	// 831679A8: 4B1588C1  bl 0x822c0268
	ctx.lr = 0x831679AC;
	sub_822C0268(ctx, base);
	// 831679AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831679B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831679B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831679B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831679BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831679C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831679C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831679C8 size=216
    let mut pc: u32 = 0x831679C8;
    'dispatch: loop {
        match pc {
            0x831679C8 => {
    //   block [0x831679C8..0x83167AA0)
	// 831679C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831679CC: 4804079D  bl 0x831a8168
	ctx.lr = 0x831679D0;
	sub_831A8130(ctx, base);
	// 831679D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831679D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831679D8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 831679DC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831679E0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 831679E4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 831679E8: 817F0250  lwz r11, 0x250(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 831679EC: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 831679F0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831679F4: 4198000C  blt cr6, 0x83167a00
	if ctx.cr[6].lt {
	pc = 0x83167A00; continue 'dispatch;
	}
	// 831679F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831679FC: 4800009C  b 0x83167a98
	pc = 0x83167A98; continue 'dispatch;
	// 83167A00: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A04: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 83167A08: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167A0C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167A10: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83167A14: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A18: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167A1C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167A20: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83167A24: 48044ACD  bl 0x831ac4f0
	ctx.lr = 0x83167A28;
	sub_831AC4F0(ctx, base);
	// 83167A28: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83167A30: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167A34: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167A38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83167A3C: 994B010B  stb r10, 0x10b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(267 as u32), ctx.r[10].u8 ) };
	// 83167A40: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A44: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167A48: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167A4C: FBCB0110  std r30, 0x110(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[30].u64 ) };
	// 83167A50: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A54: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167A58: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167A5C: FBAB0118  std r29, 0x118(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(280 as u32), ctx.r[29].u64 ) };
	// 83167A60: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83167A68: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167A6C: 7F8BF92E  stwx r28, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 83167A70: 817F0250  lwz r11, 0x250(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 83167A74: 815F024C  lwz r10, 0x24c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 83167A78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83167A7C: 7D490E70  srawi r9, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83167A80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83167A84: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83167A88: 917F0250  stw r11, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[11].u32 ) };
	// 83167A8C: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83167A90: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83167A94: 917F024C  stw r11, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[11].u32 ) };
	// 83167A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83167A9C: 4804071C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167AA0 size=184
    let mut pc: u32 = 0x83167AA0;
    'dispatch: loop {
        match pc {
            0x83167AA0 => {
    //   block [0x83167AA0..0x83167B58)
	// 83167AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167AA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83167AAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83167AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167AB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167AB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83167ABC: 817F0250  lwz r11, 0x250(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 83167AC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83167AC4: 409A000C  bne cr6, 0x83167ad0
	if !ctx.cr[6].eq {
	pc = 0x83167AD0; continue 'dispatch;
	}
	// 83167AC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167ACC: 48000074  b 0x83167b40
	pc = 0x83167B40; continue 'dispatch;
	// 83167AD0: 817F0248  lwz r11, 0x248(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 83167AD4: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 83167AD8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83167ADC: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167AE0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167AE4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83167AE8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83167AEC: 817F0248  lwz r11, 0x248(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 83167AF0: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167AF4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167AF8: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 83167AFC: 480449F5  bl 0x831ac4f0
	ctx.lr = 0x83167B00;
	sub_831AC4F0(ctx, base);
	// 83167B00: 817F0248  lwz r11, 0x248(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 83167B04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83167B08: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167B0C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167B10: E96B0110  ld r11, 0x110(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(272 as u32) ) };
	// 83167B14: F97E0108  std r11, 0x108(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(264 as u32), ctx.r[11].u64 ) };
	// 83167B18: 817F0248  lwz r11, 0x248(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 83167B1C: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167B20: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83167B24: E96B0118  ld r11, 0x118(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(280 as u32) ) };
	// 83167B28: F97E0110  std r11, 0x110(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(272 as u32), ctx.r[11].u64 ) };
	// 83167B2C: 817F0248  lwz r11, 0x248(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 83167B30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83167B34: 1D6B0120  mulli r11, r11, 0x120
	ctx.r[11].s64 = ctx.r[11].s64 * 288;
	// 83167B38: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83167B3C: 917E0118  stw r11, 0x118(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(280 as u32), ctx.r[11].u32 ) };
	// 83167B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83167B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83167B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83167B4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83167B50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83167B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83167B58 size=92
    let mut pc: u32 = 0x83167B58;
    'dispatch: loop {
        match pc {
            0x83167B58 => {
    //   block [0x83167B58..0x83167BB4)
	// 83167B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167B60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83167B64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167B68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167B6C: 4BFFFF35  bl 0x83167aa0
	ctx.lr = 0x83167B70;
	sub_83167AA0(ctx, base);
	// 83167B70: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83167B74: 409A002C  bne cr6, 0x83167ba0
	if !ctx.cr[6].eq {
	pc = 0x83167BA0; continue 'dispatch;
	}
	// 83167B78: 815F0248  lwz r10, 0x248(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 83167B7C: 817F0250  lwz r11, 0x250(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 83167B80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83167B84: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83167B88: 7D490E70  srawi r9, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83167B8C: 917F0250  stw r11, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[11].u32 ) };
	// 83167B90: 7D690194  addze r11, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83167B94: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83167B98: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83167B9C: 917F0248  stw r11, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[11].u32 ) };
	// 83167BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83167BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83167BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83167BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83167BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167BB8 size=36
    let mut pc: u32 = 0x83167BB8;
    'dispatch: loop {
        match pc {
            0x83167BB8 => {
    //   block [0x83167BB8..0x83167BDC)
	// 83167BB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83167BBC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83167BC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83167BC4: 419A0018  beq cr6, 0x83167bdc
	if ctx.cr[6].eq {
		sub_83167BDC(ctx, base);
		return;
	}
	// 83167BC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83167BD0: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83167BD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83167BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167BDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167BDC size=12
    let mut pc: u32 = 0x83167BDC;
    'dispatch: loop {
        match pc {
            0x83167BDC => {
    //   block [0x83167BDC..0x83167BE8)
	// 83167BDC: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83167BE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83167BE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167BE8 size=36
    let mut pc: u32 = 0x83167BE8;
    'dispatch: loop {
        match pc {
            0x83167BE8 => {
    //   block [0x83167BE8..0x83167C0C)
	// 83167BE8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83167BF0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83167BF4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83167BF8: E9430016  lwa r10, 0x14(r3)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83167BFC: E92B0038  ld r9, 0x38(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 83167C00: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83167C04: F94B0038  std r10, 0x38(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u64 ) };
	// 83167C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C10 size=20
    let mut pc: u32 = 0x83167C10;
    'dispatch: loop {
        match pc {
            0x83167C10 => {
    //   block [0x83167C10..0x83167C24)
	// 83167C10: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83167C14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167C18: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83167C1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83167C20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C24 size=12
    let mut pc: u32 = 0x83167C24;
    'dispatch: loop {
        match pc {
            0x83167C24 => {
    //   block [0x83167C24..0x83167C30)
	// 83167C24: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167C28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83167C2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C30 size=4
    let mut pc: u32 = 0x83167C30;
    'dispatch: loop {
        match pc {
            0x83167C30 => {
    //   block [0x83167C30..0x83167C34)
	// 83167C30: 48000008  b 0x83167c38
	sub_83167C34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C34 size=32
    let mut pc: u32 = 0x83167C34;
    'dispatch: loop {
        match pc {
            0x83167C34 => {
    //   block [0x83167C34..0x83167C54)
	// 83167C34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167C38: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167C3C: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167C40: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83167C44: 409AFFF0  bne cr6, 0x83167c34
	if !ctx.cr[6].eq {
	pc = 0x83167C34; continue 'dispatch;
	}
	// 83167C48: 814A0044  lwz r10, 0x44(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 83167C4C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83167C50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C54 size=16
    let mut pc: u32 = 0x83167C54;
    'dispatch: loop {
        match pc {
            0x83167C54 => {
    //   block [0x83167C54..0x83167C64)
	// 83167C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83167C58: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167C5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C68 size=24
    let mut pc: u32 = 0x83167C68;
    'dispatch: loop {
        match pc {
            0x83167C68 => {
    //   block [0x83167C68..0x83167C80)
	// 83167C68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83167C6C: 409A0014  bne cr6, 0x83167c80
	if !ctx.cr[6].eq {
		sub_83167C80(ctx, base);
		return;
	}
	// 83167C70: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167C74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167C78: 388B77E8  addi r4, r11, 0x77e8
	ctx.r[4].s64 = ctx.r[11].s64 + 30696;
	// 83167C7C: 4BFF7E9C  b 0x8315fb18
	sub_8315FB18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C80 size=24
    let mut pc: u32 = 0x83167C80;
    'dispatch: loop {
        match pc {
            0x83167C80 => {
    //   block [0x83167C80..0x83167C98)
	// 83167C80: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83167C88: 409A0014  bne cr6, 0x83167c9c
	if !ctx.cr[6].eq {
		sub_83167C98(ctx, base);
		return;
	}
	// 83167C8C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 83167C90: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83167C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83167C98 size=28
    let mut pc: u32 = 0x83167C98;
    'dispatch: loop {
        match pc {
            0x83167C98 => {
    //   block [0x83167C98..0x83167CB4)
	// 83167C98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167C9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167CA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83167CA4: 409AFFF4  bne cr6, 0x83167c98
	if !ctx.cr[6].eq {
	pc = 0x83167C98; continue 'dispatch;
	}
	// 83167CA8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83167CAC: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83167CB8 size=424
    let mut pc: u32 = 0x83167CB8;
    'dispatch: loop {
        match pc {
            0x83167CB8 => {
    //   block [0x83167CB8..0x83167E60)
	// 83167CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167CBC: 480404A9  bl 0x831a8164
	ctx.lr = 0x83167CC0;
	sub_831A8130(ctx, base);
	// 83167CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167CC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83167CCC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83167CD0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167CD4: 57642834  slwi r4, r27, 5
	ctx.r[4].u32 = ctx.r[27].u32.wrapping_shl(5);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83167CD8: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83167CDC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83167CE0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83167CE4: 38AB7850  addi r5, r11, 0x7850
	ctx.r[5].s64 = ctx.r[11].s64 + 30800;
	// 83167CE8: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83167CEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167CF0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 83167CF4: 4BFF794D  bl 0x8315f640
	ctx.lr = 0x83167CF8;
	sub_8315F640(ctx, base);
	// 83167CF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83167CFC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83167D00: 40820020  bne 0x83167d20
	if !ctx.cr[0].eq {
	pc = 0x83167D20; continue 'dispatch;
	}
	// 83167D04: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167D08: 388B7840  addi r4, r11, 0x7840
	ctx.r[4].s64 = ctx.r[11].s64 + 30784;
	// 83167D0C: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83167D10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167D14: 4BFF7E2D  bl 0x8315fb40
	ctx.lr = 0x83167D18;
	sub_8315FB40(ctx, base);
	// 83167D18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167D1C: 4800013C  b 0x83167e58
	pc = 0x83167E58; continue 'dispatch;
	// 83167D20: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83167D24: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83167D28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83167D2C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83167D30: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83167D34: 38A97834  addi r5, r9, 0x7834
	ctx.r[5].s64 = ctx.r[9].s64 + 30772;
	// 83167D38: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83167D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83167D40: 7C8B41D6  mullw r4, r11, r8
	ctx.r[4].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 83167D44: 4BFF78FD  bl 0x8315f640
	ctx.lr = 0x83167D48;
	sub_8315F640(ctx, base);
	// 83167D48: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83167D4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83167D50: 40820010  bne 0x83167d60
	if !ctx.cr[0].eq {
	pc = 0x83167D60; continue 'dispatch;
	}
	// 83167D54: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167D58: 388B7824  addi r4, r11, 0x7824
	ctx.r[4].s64 = ctx.r[11].s64 + 30756;
	// 83167D5C: 4BFFFFB0  b 0x83167d0c
	pc = 0x83167D0C; continue 'dispatch;
	// 83167D60: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83167D64: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83167D68: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83167D6C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83167D70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83167D74: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83167D78: 409900CC  ble cr6, 0x83167e44
	if !ctx.cr[6].gt {
	pc = 0x83167E44; continue 'dispatch;
	}
	// 83167D7C: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 83167D80: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83167D84: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83167D88: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83167D8C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83167D90: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167D94: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83167D98: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167D9C: 7D4AE9D6  mullw r10, r10, r29
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[29].s32 as i64);
	// 83167DA0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83167DA4: 7D3E4214  add r9, r30, r8
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[8].u64;
	// 83167DA8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83167DAC: 91690018  stw r11, 0x18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83167DB0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83167DB4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167DB8: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83167DBC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167DC0: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 83167DC4: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83167DC8: 48040419  bl 0x831a81e0
	ctx.lr = 0x83167DCC;
	sub_831A81E0(ctx, base);
	// 83167DCC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167DD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83167DD4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83167DD8: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83167DDC: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167DE0: 48040401  bl 0x831a81e0
	ctx.lr = 0x83167DE4;
	sub_831A81E0(ctx, base);
	// 83167DE4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167DE8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83167DEC: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83167DF0: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83167DF4: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83167DF8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83167DFC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167E00: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83167E04: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83167E08: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167E0C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83167E10: 938B0014  stw r28, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 83167E14: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167E18: 409A000C  bne cr6, 0x83167e24
	if !ctx.cr[6].eq {
	pc = 0x83167E24; continue 'dispatch;
	}
	// 83167E1C: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 83167E20: 48000010  b 0x83167e30
	pc = 0x83167E30; continue 'dispatch;
	// 83167E24: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83167E28: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 83167E2C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83167E30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83167E34: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83167E38: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 83167E3C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83167E40: 4198FF44  blt cr6, 0x83167d84
	if ctx.cr[6].lt {
	pc = 0x83167D84; continue 'dispatch;
	}
	// 83167E44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83167E48: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 83167E4C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83167E50: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83167E54: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83167E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83167E5C: 48040358  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167E60 size=92
    let mut pc: u32 = 0x83167E60;
    'dispatch: loop {
        match pc {
            0x83167E60 => {
    //   block [0x83167E60..0x83167EBC)
	// 83167E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83167E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83167E74: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83167E78: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83167E7C: 419A000C  beq cr6, 0x83167e88
	if ctx.cr[6].eq {
	pc = 0x83167E88; continue 'dispatch;
	}
	// 83167E80: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167E84: 4BFF78E5  bl 0x8315f768
	ctx.lr = 0x83167E88;
	sub_8315F768(ctx, base);
	// 83167E88: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83167E8C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83167E90: 419A000C  beq cr6, 0x83167e9c
	if ctx.cr[6].eq {
	pc = 0x83167E9C; continue 'dispatch;
	}
	// 83167E94: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167E98: 4BFF78D1  bl 0x8315f768
	ctx.lr = 0x83167E9C;
	sub_8315F768(ctx, base);
	// 83167E9C: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 83167EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83167EA4: 4BFF7DDD  bl 0x8315fc80
	ctx.lr = 0x83167EA8;
	sub_8315FC80(ctx, base);
	// 83167EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83167EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83167EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83167EB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83167EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83167EC0 size=176
    let mut pc: u32 = 0x83167EC0;
    'dispatch: loop {
        match pc {
            0x83167EC0 => {
    //   block [0x83167EC0..0x83167F70)
	// 83167EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83167EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83167ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83167ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167ED4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83167ED8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83167EDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167EE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83167EE4: 419A0018  beq cr6, 0x83167efc
	if ctx.cr[6].eq {
	pc = 0x83167EFC; continue 'dispatch;
	}
	// 83167EE8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167EEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83167EF0: 388B7870  addi r4, r11, 0x7870
	ctx.r[4].s64 = ctx.r[11].s64 + 30832;
	// 83167EF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167EF8: 4BFF7C49  bl 0x8315fb40
	ctx.lr = 0x83167EFC;
	sub_8315FB40(ctx, base);
	// 83167EFC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167F00: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83167F04: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83167F08: 394A7868  addi r10, r10, 0x7868
	ctx.r[10].s64 = ctx.r[10].s64 + 30824;
	// 83167F0C: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83167F10: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167F14: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167F18: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83167F1C: 40820014  bne 0x83167f30
	if !ctx.cr[0].eq {
	pc = 0x83167F30; continue 'dispatch;
	}
	// 83167F20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83167F24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83167F28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83167F2C: 409AFFE4  bne cr6, 0x83167f10
	if !ctx.cr[6].eq {
	pc = 0x83167F10; continue 'dispatch;
	}
	// 83167F30: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83167F34: 41820018  beq 0x83167f4c
	if ctx.cr[0].eq {
	pc = 0x83167F4C; continue 'dispatch;
	}
	// 83167F38: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83167F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83167F40: 388B785C  addi r4, r11, 0x785c
	ctx.r[4].s64 = ctx.r[11].s64 + 30812;
	// 83167F44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83167F48: 4BFF7BF9  bl 0x8315fb40
	ctx.lr = 0x83167F4C;
	sub_8315FB40(ctx, base);
	// 83167F4C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83167F50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83167F54: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 83167F58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83167F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83167F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83167F64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83167F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83167F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83167F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83167F70 size=648
    let mut pc: u32 = 0x83167F70;
    'dispatch: loop {
        match pc {
            0x83167F70 => {
    //   block [0x83167F70..0x831681F8)
	// 83167F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83167F74: 480401F5  bl 0x831a8168
	ctx.lr = 0x83167F78;
	sub_831A8130(ctx, base);
	// 83167F78: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 83167F7C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 83167F80: 9421FB60  stwu r1, -0x4a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1184 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83167F84: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83167F88: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 83167F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83167F90: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 83167F94: 80DC0004  lwz r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83167F98: C3C808A4  lfs f30, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 83167F9C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83167FA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83167FA4: 41820020  beq 0x83167fc4
	if ctx.cr[0].eq {
	pc = 0x83167FC4; continue 'dispatch;
	}
	// 83167FA8: C00A001C  lfs f0, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83167FAC: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 83167FB0: 419A0008  beq cr6, 0x83167fb8
	if ctx.cr[6].eq {
	pc = 0x83167FB8; continue 'dispatch;
	}
	// 83167FB4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83167FB8: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83167FBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83167FC0: 409AFFE8  bne cr6, 0x83167fa8
	if !ctx.cr[6].eq {
	pc = 0x83167FA8; continue 'dispatch;
	}
	// 83167FC4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83167FC8: 40990220  ble cr6, 0x831681e8
	if !ctx.cr[6].gt {
	pc = 0x831681E8; continue 'dispatch;
	}
	// 83167FCC: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 83167FD0: FC00F090  fmr f0, f30
	ctx.f[0].f64 = ctx.f[30].f64;
	// 83167FD4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83167FD8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83167FDC: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83167FE0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83167FE4: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83167FE8: 409900BC  ble cr6, 0x831680a4
	if !ctx.cr[6].gt {
	pc = 0x831680A4; continue 'dispatch;
	}
	// 83167FEC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 83167FF0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 83167FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83167FF8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83167FFC: 41820014  beq 0x83168010
	if ctx.cr[0].eq {
	pc = 0x83168010; continue 'dispatch;
	}
	// 83168000: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83168004: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83168008: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8316800C: 4200FFF8  bdnz 0x83168004
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83168004; continue 'dispatch;
	}
	// 83168010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83168014: 48000014  b 0x83168028
	pc = 0x83168028; continue 'dispatch;
	// 83168018: C1AB001C  lfs f13, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8316801C: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 83168020: 409A0010  bne cr6, 0x83168030
	if !ctx.cr[6].eq {
	pc = 0x83168030; continue 'dispatch;
	}
	// 83168024: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168028: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316802C: 409AFFEC  bne cr6, 0x83168018
	if !ctx.cr[6].eq {
	pc = 0x83168018; continue 'dispatch;
	}
	// 83168030: 38E10260  addi r7, r1, 0x260
	ctx.r[7].s64 = ctx.r[1].s64 + 608;
	// 83168034: C1AB001C  lfs f13, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83168038: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8316803C: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 83168040: 7D6A392E  stwx r11, r10, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[11].u32) };
	// 83168044: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83168048: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316804C: 4082FFDC  bne 0x83168028
	if !ctx.cr[0].eq {
	pc = 0x83168028; continue 'dispatch;
	}
	// 83168050: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83168054: 40990050  ble cr6, 0x831680a4
	if !ctx.cr[6].gt {
	pc = 0x831680A4; continue 'dispatch;
	}
	// 83168058: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8316805C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83168060: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83168064: C1A808A8  lfs f13, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83168068: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8316806C: 39010260  addi r8, r1, 0x260
	ctx.r[8].s64 = ctx.r[1].s64 + 608;
	// 83168070: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 83168074: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83168078: 7D0B402E  lwzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8316807C: C1A8001C  lfs f13, 0x1c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83168080: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 83168084: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 83168088: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8316808C: D9A10050  stfd f13, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[13].u64 ) };
	// 83168090: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83168094: 7CC83050  subf r6, r8, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 83168098: 7D0B392E  stwx r8, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u32) };
	// 8316809C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831680A0: 4082FFCC  bne 0x8316806c
	if !ctx.cr[0].eq {
	pc = 0x8316806C; continue 'dispatch;
	}
	// 831680A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 831680A8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831680AC: C3EBB41C  lfs f31, -0x4be4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19428 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 831680B0: 40990098  ble cr6, 0x83168148
	if !ctx.cr[6].gt {
	pc = 0x83168148; continue 'dispatch;
	}
	// 831680B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831680B8: C18B5D80  lfs f12, 0x5d80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23936 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831680BC: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	// 831680C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831680C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831680C8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831680CC: 40990060  ble cr6, 0x8316812c
	if !ctx.cr[6].gt {
	pc = 0x8316812C; continue 'dispatch;
	}
	// 831680D0: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 831680D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831680D8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 831680DC: 38810260  addi r4, r1, 0x260
	ctx.r[4].s64 = ctx.r[1].s64 + 608;
	// 831680E0: 7CAB282E  lwzx r5, r11, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 831680E4: 7C8B202E  lwzx r4, r11, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 831680E8: 7CA539D6  mullw r5, r5, r7
	ctx.r[5].s64 = (ctx.r[5].s32 as i64) * (ctx.r[7].s32 as i64);
	// 831680EC: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831680F0: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831680F4: 7CA507B4  extsw r5, r5
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 831680F8: F8A10050  std r5, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u64 ) };
	// 831680FC: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83168100: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 83168104: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83168108: EC0B0024  fdivs f0, f11, f0
	ctx.f[0].f64 = ((ctx.f[11].f64 / ctx.f[0].f64) as f32) as f64;
	// 8316810C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83168110: 4098000C  bge cr6, 0x8316811c
	if !ctx.cr[6].lt {
	pc = 0x8316811C; continue 'dispatch;
	}
	// 83168114: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 83168118: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 8316811C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83168120: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83168124: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83168128: 4198FFB0  blt cr6, 0x831680d8
	if ctx.cr[6].lt {
	pc = 0x831680D8; continue 'dispatch;
	}
	// 8316812C: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83168130: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 83168134: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83168138: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8316813C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83168140: 7D0A592E  stwx r8, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 83168144: 4181FF78  bgt 0x831680bc
	if ctx.cr[0].gt {
	pc = 0x831680BC; continue 'dispatch;
	}
	// 83168148: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8316814C: 4099009C  ble cr6, 0x831681e8
	if !ctx.cr[6].gt {
	pc = 0x831681E8; continue 'dispatch;
	}
	// 83168150: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168154: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83168158: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 8316815C: 3BAB787C  addi r29, r11, 0x787c
	ctx.r[29].s64 = ctx.r[11].s64 + 30844;
	// 83168160: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 83168164: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83168168: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316816C: 41990014  bgt cr6, 0x83168180
	if ctx.cr[6].gt {
	pc = 0x83168180; continue 'dispatch;
	}
	// 83168170: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83168174: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83168178: 4BFF79A1  bl 0x8315fb18
	ctx.lr = 0x8316817C;
	sub_8315FB18(ctx, base);
	// 8316817C: 48000060  b 0x831681dc
	pc = 0x831681DC; continue 'dispatch;
	// 83168180: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83168184: 41980048  blt cr6, 0x831681cc
	if ctx.cr[6].lt {
	pc = 0x831681CC; continue 'dispatch;
	}
	// 83168188: 39210260  addi r9, r1, 0x260
	ctx.r[9].s64 = ctx.r[1].s64 + 608;
	// 8316818C: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168190: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 83168194: 7D480E70  srawi r8, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83168198: 7D4B51D6  mullw r10, r11, r10
	ctx.r[10].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8316819C: 7D7F482E  lwzx r11, r31, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831681A0: C00B001C  lfs f0, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831681A4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831681A8: 7D680194  addze r11, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831681AC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831681B0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 831681B4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 831681B8: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831681BC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 831681C0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 831681C4: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 831681C8: 48000008  b 0x831681d0
	pc = 0x831681D0; continue 'dispatch;
	// 831681CC: FC00F090  fmr f0, f30
	ctx.f[0].f64 = ctx.f[30].f64;
	// 831681D0: 39610260  addi r11, r1, 0x260
	ctx.r[11].s64 = ctx.r[1].s64 + 608;
	// 831681D4: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831681D8: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 831681DC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831681E0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831681E4: 4082FF7C  bne 0x83168160
	if !ctx.cr[0].eq {
	pc = 0x83168160; continue 'dispatch;
	}
	// 831681E8: 382104A0  addi r1, r1, 0x4a0
	ctx.r[1].s64 = ctx.r[1].s64 + 1184;
	// 831681EC: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 831681F0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 831681F4: 4803FFC4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831681F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831681F8 size=68
    let mut pc: u32 = 0x831681F8;
    'dispatch: loop {
        match pc {
            0x831681F8 => {
    //   block [0x831681F8..0x8316823C)
	// 831681F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831681FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168208: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8316820C: 807F8470  lwz r3, -0x7b90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-31632 as u32) ) } as u64;
	// 83168210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168214: 419A0010  beq cr6, 0x83168224
	if ctx.cr[6].eq {
	pc = 0x83168224; continue 'dispatch;
	}
	// 83168218: 4BFFFC49  bl 0x83167e60
	ctx.lr = 0x8316821C;
	sub_83167E60(ctx, base);
	// 8316821C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83168220: 917F8470  stw r11, -0x7b90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-31632 as u32), ctx.r[11].u32 ) };
	// 83168224: 4BFF73F5  bl 0x8315f618
	ctx.lr = 0x83168228;
	sub_8315F618(ctx, base);
	// 83168228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316822C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168240 size=156
    let mut pc: u32 = 0x83168240;
    'dispatch: loop {
        match pc {
            0x83168240 => {
    //   block [0x83168240..0x831682DC)
	// 83168240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316824C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168258: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8316825C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83168260: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 83168264: 387F0270  addi r3, r31, 0x270
	ctx.r[3].s64 = ctx.r[31].s64 + 624;
	// 83168268: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8316826C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83168270: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83168274: 38A00120  li r5, 0x120
	ctx.r[5].s64 = 288;
	// 83168278: 93DF0250  stw r30, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[30].u32 ) };
	// 8316827C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83168280: 93DF0254  stw r30, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[30].u32 ) };
	// 83168284: 93DF0258  stw r30, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[30].u32 ) };
	// 83168288: 93DF0260  stw r30, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u32 ) };
	// 8316828C: 93DF0264  stw r30, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[30].u32 ) };
	// 83168290: 93DF0268  stw r30, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u32 ) };
	// 83168294: 93DF026C  stw r30, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[30].u32 ) };
	// 83168298: 4803FF49  bl 0x831a81e0
	ctx.lr = 0x8316829C;
	sub_831A81E0(ctx, base);
	// 8316829C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831682A0: 93DF0390  stw r30, 0x390(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(912 as u32), ctx.r[30].u32 ) };
	// 831682A4: 93DF0394  stw r30, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[30].u32 ) };
	// 831682A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831682AC: 93DF0398  stw r30, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[30].u32 ) };
	// 831682B0: 816B8470  lwz r11, -0x7b90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31632 as u32) ) } as u64;
	// 831682B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831682B8: 917F039C  stw r11, 0x39c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(924 as u32), ctx.r[11].u32 ) };
	// 831682BC: 93DF03A0  stw r30, 0x3a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(928 as u32), ctx.r[30].u32 ) };
	// 831682C0: 93DF03A4  stw r30, 0x3a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(932 as u32), ctx.r[30].u32 ) };
	// 831682C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831682C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831682CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831682D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831682D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831682D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831682E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831682E0 size=8
    let mut pc: u32 = 0x831682E0;
    'dispatch: loop {
        match pc {
            0x831682E0 => {
    //   block [0x831682E0..0x831682E8)
	// 831682E0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 831682E4: 4BFFF6E4  b 0x831679c8
	sub_831679C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831682E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831682E8 size=8
    let mut pc: u32 = 0x831682E8;
    'dispatch: loop {
        match pc {
            0x831682E8 => {
    //   block [0x831682E8..0x831682F0)
	// 831682E8: 80630264  lwz r3, 0x264(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(612 as u32) ) } as u64;
	// 831682EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831682F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831682F0 size=24
    let mut pc: u32 = 0x831682F0;
    'dispatch: loop {
        match pc {
            0x831682F0 => {
    //   block [0x831682F0..0x83168308)
	// 831682F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831682F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831682F8: 91630268  stw r11, 0x268(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(616 as u32), ctx.r[11].u32 ) };
	// 831682FC: 91430264  stw r10, 0x264(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(612 as u32), ctx.r[10].u32 ) };
	// 83168300: 916303A4  stw r11, 0x3a4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(932 as u32), ctx.r[11].u32 ) };
	// 83168304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168308 size=12
    let mut pc: u32 = 0x83168308;
    'dispatch: loop {
        match pc {
            0x83168308 => {
    //   block [0x83168308..0x83168314)
	// 83168308: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316830C: 916303A0  stw r11, 0x3a0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(928 as u32), ctx.r[11].u32 ) };
	// 83168310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168318 size=108
    let mut pc: u32 = 0x83168318;
    'dispatch: loop {
        match pc {
            0x83168318 => {
    //   block [0x83168318..0x83168384)
	// 83168318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316832C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168330: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168334: 419A0024  beq cr6, 0x83168358
	if ctx.cr[6].eq {
	pc = 0x83168358; continue 'dispatch;
	}
	// 83168338: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316833C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83168340: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83168344: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316834C: 4E800421  bctrl
	ctx.lr = 0x83168350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83168350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83168354: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83168358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316835C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83168360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83168364: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83168368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316836C: 4E800421  bctrl
	ctx.lr = 0x83168370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83168370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83168374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316837C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168388 size=192
    let mut pc: u32 = 0x83168388;
    'dispatch: loop {
        match pc {
            0x83168388 => {
    //   block [0x83168388..0x83168448)
	// 83168388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316838C: 4803FDE1  bl 0x831a816c
	ctx.lr = 0x83168390;
	sub_831A8130(ctx, base);
	// 83168390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168398: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316839C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831683A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831683A4: 419A001C  beq cr6, 0x831683c0
	if ctx.cr[6].eq {
	pc = 0x831683C0; continue 'dispatch;
	}
	// 831683A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831683AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831683B0: 388B78EC  addi r4, r11, 0x78ec
	ctx.r[4].s64 = ctx.r[11].s64 + 30956;
	// 831683B4: 4BFF7765  bl 0x8315fb18
	ctx.lr = 0x831683B8;
	sub_8315FB18(ctx, base);
	// 831683B8: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 831683BC: 48000080  b 0x8316843c
	pc = 0x8316843C; continue 'dispatch;
	// 831683C0: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831683C4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831683C8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831683CC: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 831683D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831683D4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831683D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831683DC: 4E800421  bctrl
	ctx.lr = 0x831683E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831683E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831683E4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 831683E8: 4082001C  bne 0x83168404
	if !ctx.cr[0].eq {
	pc = 0x83168404; continue 'dispatch;
	}
	// 831683EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831683F0: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 831683F4: 388B78DC  addi r4, r11, 0x78dc
	ctx.r[4].s64 = ctx.r[11].s64 + 30940;
	// 831683F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831683FC: 4BFF7745  bl 0x8315fb40
	ctx.lr = 0x83168400;
	sub_8315FB40(ctx, base);
	// 83168400: 4BFFFFB8  b 0x831683b8
	pc = 0x831683B8; continue 'dispatch;
	// 83168404: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168408: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316840C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 83168410: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83168414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83168418: 4E800421  bctrl
	ctx.lr = 0x8316841C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316841C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168420: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83168424: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83168428: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316842C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83168434: 4E800421  bctrl
	ctx.lr = 0x83168438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83168438: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316843C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83168440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83168444: 4803FD78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168448 size=216
    let mut pc: u32 = 0x83168448;
    'dispatch: loop {
        match pc {
            0x83168448 => {
    //   block [0x83168448..0x83168520)
	// 83168448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83168454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316845C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168460: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168464: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83168468: 409900A0  ble cr6, 0x83168508
	if !ctx.cr[6].gt {
	pc = 0x83168508; continue 'dispatch;
	}
	// 8316846C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83168470: 40990040  ble cr6, 0x831684b0
	if !ctx.cr[6].gt {
	pc = 0x831684B0; continue 'dispatch;
	}
	// 83168474: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83168478: 409A0090  bne cr6, 0x83168508
	if !ctx.cr[6].eq {
	pc = 0x83168508; continue 'dispatch;
	}
	// 8316847C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168480: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83168484: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168488: 419A0020  beq cr6, 0x831684a8
	if ctx.cr[6].eq {
	pc = 0x831684A8; continue 'dispatch;
	}
	// 8316848C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168490: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83168494: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 83168498: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316849C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831684A0: 4E800421  bctrl
	ctx.lr = 0x831684A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831684A4: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 831684A8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 831684AC: 4800005C  b 0x83168508
	pc = 0x83168508; continue 'dispatch;
	// 831684B0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831684B4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831684B8: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 831684BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831684C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831684C4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 831684C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831684CC: 4E800421  bctrl
	ctx.lr = 0x831684D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831684D0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831684D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831684D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831684DC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 831684E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831684E4: 4E800421  bctrl
	ctx.lr = 0x831684E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831684E8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831684EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831684F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831684F4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831684F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831684FC: 4E800421  bctrl
	ctx.lr = 0x83168500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83168500: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83168504: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83168508: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316850C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168514: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83168518: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316851C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168520 size=112
    let mut pc: u32 = 0x83168520;
    'dispatch: loop {
        match pc {
            0x83168520 => {
    //   block [0x83168520..0x83168590)
	// 83168520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316852C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168534: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168538: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8316853C: 419A001C  beq cr6, 0x83168558
	if ctx.cr[6].eq {
	pc = 0x83168558; continue 'dispatch;
	}
	// 83168540: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168548: 388B7910  addi r4, r11, 0x7910
	ctx.r[4].s64 = ctx.r[11].s64 + 30992;
	// 8316854C: 4BFF75CD  bl 0x8315fb18
	ctx.lr = 0x83168550;
	sub_8315FB18(ctx, base);
	// 83168550: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83168554: 48000024  b 0x83168578
	pc = 0x83168578; continue 'dispatch;
	// 83168558: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316855C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83168560: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 83168564: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168568: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316856C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83168570: 4E800421  bctrl
	ctx.lr = 0x83168574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83168574: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83168578: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316857C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83168580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168590 size=112
    let mut pc: u32 = 0x83168590;
    'dispatch: loop {
        match pc {
            0x83168590 => {
    //   block [0x83168590..0x83168600)
	// 83168590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316859C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831685A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831685A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831685A8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831685AC: 419A0020  beq cr6, 0x831685cc
	if ctx.cr[6].eq {
	pc = 0x831685CC; continue 'dispatch;
	}
	// 831685B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831685B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831685B8: 388B7930  addi r4, r11, 0x7930
	ctx.r[4].s64 = ctx.r[11].s64 + 31024;
	// 831685BC: 4BFF755D  bl 0x8315fb18
	ctx.lr = 0x831685C0;
	sub_8315FB18(ctx, base);
	// 831685C0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 831685C4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831685C8: 48000024  b 0x831685ec
	pc = 0x831685EC; continue 'dispatch;
	// 831685CC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831685D0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831685D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831685D8: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 831685DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831685E0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 831685E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831685E8: 4E800421  bctrl
	ctx.lr = 0x831685EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831685EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831685F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831685F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831685F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831685FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168600 size=384
    let mut pc: u32 = 0x83168600;
    'dispatch: loop {
        match pc {
            0x83168600 => {
    //   block [0x83168600..0x83168780)
	// 83168600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316860C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168618: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316861C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83168620: 419A00F0  beq cr6, 0x83168710
	if ctx.cr[6].eq {
	pc = 0x83168710; continue 'dispatch;
	}
	// 83168624: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83168628: 419A0088  beq cr6, 0x831686b0
	if ctx.cr[6].eq {
	pc = 0x831686B0; continue 'dispatch;
	}
	// 8316862C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83168630: 409A0138  bne cr6, 0x83168768
	if !ctx.cr[6].eq {
	pc = 0x83168768; continue 'dispatch;
	}
	// 83168634: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168638: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316863C: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 83168640: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83168644: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168648: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8316864C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83168650: 4E800421  bctrl
	ctx.lr = 0x83168654;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83168654: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83168658: 4182004C  beq 0x831686a4
	if ctx.cr[0].eq {
	pc = 0x831686A4; continue 'dispatch;
	}
	// 8316865C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83168660: 419A0014  beq cr6, 0x83168674
	if ctx.cr[6].eq {
	pc = 0x83168674; continue 'dispatch;
	}
	// 83168664: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83168668: 419A0030  beq cr6, 0x83168698
	if ctx.cr[6].eq {
	pc = 0x83168698; continue 'dispatch;
	}
	// 8316866C: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83168670: 409A00F8  bne cr6, 0x83168768
	if !ctx.cr[6].eq {
	pc = 0x83168768; continue 'dispatch;
	}
	// 83168674: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168678: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316867C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168680: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83168688: 4E800421  bctrl
	ctx.lr = 0x8316868C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316868C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83168690: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83168694: 480000D0  b 0x83168764
	pc = 0x83168764; continue 'dispatch;
	// 83168698: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316869C: 388B7A34  addi r4, r11, 0x7a34
	ctx.r[4].s64 = ctx.r[11].s64 + 31284;
	// 831686A0: 480000B8  b 0x83168758
	pc = 0x83168758; continue 'dispatch;
	// 831686A4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831686A8: 388B7A08  addi r4, r11, 0x7a08
	ctx.r[4].s64 = ctx.r[11].s64 + 31240;
	// 831686AC: 480000AC  b 0x83168758
	pc = 0x83168758; continue 'dispatch;
	// 831686B0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831686B4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831686B8: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 831686BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831686C0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 831686C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831686C8: 4E800421  bctrl
	ctx.lr = 0x831686CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831686CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831686D0: 41820034  beq 0x83168704
	if ctx.cr[0].eq {
	pc = 0x83168704; continue 'dispatch;
	}
	// 831686D4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831686D8: 419A0024  beq cr6, 0x831686fc
	if ctx.cr[6].eq {
	pc = 0x831686FC; continue 'dispatch;
	}
	// 831686DC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831686E0: 419A0010  beq cr6, 0x831686f0
	if ctx.cr[6].eq {
	pc = 0x831686F0; continue 'dispatch;
	}
	// 831686E4: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831686E8: 409A0080  bne cr6, 0x83168768
	if !ctx.cr[6].eq {
	pc = 0x83168768; continue 'dispatch;
	}
	// 831686EC: 48000074  b 0x83168760
	pc = 0x83168760; continue 'dispatch;
	// 831686F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831686F4: 388B79D8  addi r4, r11, 0x79d8
	ctx.r[4].s64 = ctx.r[11].s64 + 31192;
	// 831686F8: 48000060  b 0x83168758
	pc = 0x83168758; continue 'dispatch;
	// 831686FC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83168700: 48000064  b 0x83168764
	pc = 0x83168764; continue 'dispatch;
	// 83168704: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168708: 388B79AC  addi r4, r11, 0x79ac
	ctx.r[4].s64 = ctx.r[11].s64 + 31148;
	// 8316870C: 4800004C  b 0x83168758
	pc = 0x83168758; continue 'dispatch;
	// 83168710: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168714: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83168718: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316871C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168720: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83168724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83168728: 4E800421  bctrl
	ctx.lr = 0x8316872C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316872C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83168730: 41820020  beq 0x83168750
	if ctx.cr[0].eq {
	pc = 0x83168750; continue 'dispatch;
	}
	// 83168734: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83168738: 419AFFC4  beq cr6, 0x831686fc
	if ctx.cr[6].eq {
	pc = 0x831686FC; continue 'dispatch;
	}
	// 8316873C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83168740: 409AFFA4  bne cr6, 0x831686e4
	if !ctx.cr[6].eq {
	pc = 0x831686E4; continue 'dispatch;
	}
	// 83168744: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168748: 388B797C  addi r4, r11, 0x797c
	ctx.r[4].s64 = ctx.r[11].s64 + 31100;
	// 8316874C: 4800000C  b 0x83168758
	pc = 0x83168758; continue 'dispatch;
	// 83168750: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168754: 388B7950  addi r4, r11, 0x7950
	ctx.r[4].s64 = ctx.r[11].s64 + 31056;
	// 83168758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316875C: 4BFF73BD  bl 0x8315fb18
	ctx.lr = 0x83168760;
	sub_8315FB18(ctx, base);
	// 83168760: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83168764: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83168768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316876C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168774: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83168778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316877C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168780 size=12
    let mut pc: u32 = 0x83168780;
    'dispatch: loop {
        match pc {
            0x83168780 => {
    //   block [0x83168780..0x8316878C)
	// 83168780: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83168788: 48000010  b 0x83168798
	sub_8316878C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316878C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316878C size=32
    let mut pc: u32 = 0x8316878C;
    'dispatch: loop {
        match pc {
            0x8316878C => {
    //   block [0x8316878C..0x831687AC)
	// 8316878C: E94B0016  lwa r10, 0x14(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168790: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168794: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83168798: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316879C: 409AFFF0  bne cr6, 0x8316878c
	if !ctx.cr[6].eq {
	pc = 0x8316878C; continue 'dispatch;
	}
	// 831687A0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831687A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831687A8: 48000010  b 0x831687b8
	sub_831687AC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831687AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831687AC size=32
    let mut pc: u32 = 0x831687AC;
    'dispatch: loop {
        match pc {
            0x831687AC => {
    //   block [0x831687AC..0x831687CC)
	// 831687AC: E94B0016  lwa r10, 0x14(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 831687B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831687B4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831687B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831687BC: 409AFFF0  bne cr6, 0x831687ac
	if !ctx.cr[6].eq {
	pc = 0x831687AC; continue 'dispatch;
	}
	// 831687C0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 831687C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831687C8: 48000010  b 0x831687d8
	sub_831687CC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831687CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831687CC size=48
    let mut pc: u32 = 0x831687CC;
    'dispatch: loop {
        match pc {
            0x831687CC => {
    //   block [0x831687CC..0x831687FC)
	// 831687CC: E90B0016  lwa r8, 0x14(r11)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 831687D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831687D4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 831687D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831687DC: 409AFFF0  bne cr6, 0x831687cc
	if !ctx.cr[6].eq {
	pc = 0x831687CC; continue 'dispatch;
	}
	// 831687E0: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831687E4: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 831687E8: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 831687EC: 409A0010  bne cr6, 0x831687fc
	if !ctx.cr[6].eq {
		sub_831687FC(ctx, base);
		return;
	}
	// 831687F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831687F4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831687F8: 4800003C  b 0x83168834
	sub_8316882C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831687FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831687FC size=48
    let mut pc: u32 = 0x831687FC;
    'dispatch: loop {
        match pc {
            0x831687FC => {
    //   block [0x831687FC..0x8316882C)
	// 831687FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83168800: C003001C  lfs f0, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83168804: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83168808: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8316880C: 419A0020  beq cr6, 0x8316882c
	if ctx.cr[6].eq {
		sub_8316882C(ctx, base);
		return;
	}
	// 83168810: 796B1F24  sldi r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(3);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 83168814: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 83168818: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316881C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83168820: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83168824: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 83168828: 4800000C  b 0x83168834
	sub_8316882C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316882C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8316882C size=20
    let mut pc: u32 = 0x8316882C;
    'dispatch: loop {
        match pc {
            0x8316882C => {
    //   block [0x8316882C..0x83168840)
	// 8316882C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168830: C00B5D80  lfs f0, 0x5d80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23936 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83168834: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83168838: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8316883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168840 size=12
    let mut pc: u32 = 0x83168840;
    'dispatch: loop {
        match pc {
            0x83168840 => {
    //   block [0x83168840..0x8316884C)
	// 83168840: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83168844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83168848: 48000010  b 0x83168858
	sub_8316884C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316884C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316884C size=52
    let mut pc: u32 = 0x8316884C;
    'dispatch: loop {
        match pc {
            0x8316884C => {
    //   block [0x8316884C..0x83168880)
	// 8316884C: E92B0016  lwa r9, 0x14(r11)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168850: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168854: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83168858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316885C: 409AFFF0  bne cr6, 0x8316884c
	if !ctx.cr[6].eq {
	pc = 0x8316884C; continue 'dispatch;
	}
	// 83168860: E9630038  ld r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	// 83168864: E9230028  ld r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 83168868: E9030030  ld r8, 0x30(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	// 8316886C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83168870: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83168874: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83168878: 7F2B4000  cmpd cr6, r11, r8
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[8].s64, &mut ctx.xer);
	// 8316887C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168880 size=8
    let mut pc: u32 = 0x83168880;
    'dispatch: loop {
        match pc {
            0x83168880 => {
    //   block [0x83168880..0x83168888)
	// 83168880: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168888 size=12
    let mut pc: u32 = 0x83168888;
    'dispatch: loop {
        match pc {
            0x83168888 => {
    //   block [0x83168888..0x83168894)
	// 83168888: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316888C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83168890: 48000010  b 0x831688a0
	sub_83168894(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168894 size=52
    let mut pc: u32 = 0x83168894;
    'dispatch: loop {
        match pc {
            0x83168894 => {
    //   block [0x83168894..0x831688C8)
	// 83168894: E92B0016  lwa r9, 0x14(r11)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168898: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316889C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831688A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831688A4: 409AFFF0  bne cr6, 0x83168894
	if !ctx.cr[6].eq {
	pc = 0x83168894; continue 'dispatch;
	}
	// 831688A8: E9630038  ld r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	// 831688AC: E9230028  ld r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 831688B0: E9030030  ld r8, 0x30(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	// 831688B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831688B8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 831688BC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831688C0: 7F2B4000  cmpd cr6, r11, r8
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[8].s64, &mut ctx.xer);
	// 831688C4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831688C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831688C8 size=8
    let mut pc: u32 = 0x831688C8;
    'dispatch: loop {
        match pc {
            0x831688C8 => {
    //   block [0x831688C8..0x831688D0)
	// 831688C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831688CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831688D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831688D0 size=12
    let mut pc: u32 = 0x831688D0;
    'dispatch: loop {
        match pc {
            0x831688D0 => {
    //   block [0x831688D0..0x831688DC)
	// 831688D0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831688D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831688D8: 48000010  b 0x831688e8
	sub_831688DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831688DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831688DC size=32
    let mut pc: u32 = 0x831688DC;
    'dispatch: loop {
        match pc {
            0x831688DC => {
    //   block [0x831688DC..0x831688FC)
	// 831688DC: E94B0016  lwa r10, 0x14(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 831688E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831688E4: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 831688E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831688EC: 409AFFF0  bne cr6, 0x831688dc
	if !ctx.cr[6].eq {
	pc = 0x831688DC; continue 'dispatch;
	}
	// 831688F0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831688F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831688F8: 48000010  b 0x83168908
	sub_831688FC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831688FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831688FC size=32
    let mut pc: u32 = 0x831688FC;
    'dispatch: loop {
        match pc {
            0x831688FC => {
    //   block [0x831688FC..0x8316891C)
	// 831688FC: E94B0016  lwa r10, 0x14(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168900: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168904: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83168908: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316890C: 409AFFF0  bne cr6, 0x831688fc
	if !ctx.cr[6].eq {
	pc = 0x831688FC; continue 'dispatch;
	}
	// 83168910: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83168914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83168918: 48000010  b 0x83168928
	sub_8316891C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316891C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316891C size=32
    let mut pc: u32 = 0x8316891C;
    'dispatch: loop {
        match pc {
            0x8316891C => {
    //   block [0x8316891C..0x8316893C)
	// 8316891C: E94B0016  lwa r10, 0x14(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168920: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168924: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83168928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316892C: 409AFFF0  bne cr6, 0x8316891c
	if !ctx.cr[6].eq {
	pc = 0x8316891C; continue 'dispatch;
	}
	// 83168930: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83168938: 48000010  b 0x83168948
	sub_8316893C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316893C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8316893C size=52
    let mut pc: u32 = 0x8316893C;
    'dispatch: loop {
        match pc {
            0x8316893C => {
    //   block [0x8316893C..0x83168970)
	// 8316893C: E90B0016  lwa r8, 0x14(r11)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168940: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168944: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 83168948: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316894C: 409AFFF0  bne cr6, 0x8316893c
	if !ctx.cr[6].eq {
	pc = 0x8316893C; continue 'dispatch;
	}
	// 83168950: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83168954: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 83168958: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8316895C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83168960: 409A0010  bne cr6, 0x83168970
	if !ctx.cr[6].eq {
		sub_83168970(ctx, base);
		return;
	}
	// 83168964: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83168968: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8316896C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83168970 size=48
    let mut pc: u32 = 0x83168970;
    'dispatch: loop {
        match pc {
            0x83168970 => {
    //   block [0x83168970..0x831689A0)
	// 83168970: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83168974: C003001C  lfs f0, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83168978: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8316897C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83168980: 419A0020  beq cr6, 0x831689a0
	if ctx.cr[6].eq {
		sub_831689A0(ctx, base);
		return;
	}
	// 83168984: 796B1F24  sldi r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(3);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 83168988: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8316898C: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168990: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83168994: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83168998: EC2D0024  fdivs f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8316899C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831689A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831689A0 size=12
    let mut pc: u32 = 0x831689A0;
    'dispatch: loop {
        match pc {
            0x831689A0 => {
    //   block [0x831689A0..0x831689AC)
	// 831689A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831689A4: C02B5D80  lfs f1, 0x5d80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23936 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831689A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831689B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831689B0 size=172
    let mut pc: u32 = 0x831689B0;
    'dispatch: loop {
        match pc {
            0x831689B0 => {
    //   block [0x831689B0..0x83168A5C)
	// 831689B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831689B4: 4803F7B9  bl 0x831a816c
	ctx.lr = 0x831689B8;
	sub_831A8130(ctx, base);
	// 831689B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831689BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831689C0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831689C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831689C8: 409A0014  bne cr6, 0x831689dc
	if !ctx.cr[6].eq {
	pc = 0x831689DC; continue 'dispatch;
	}
	// 831689CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831689D0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831689D4: 4800001C  b 0x831689f0
	pc = 0x831689F0; continue 'dispatch;
	// 831689D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831689DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831689E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831689E4: 409AFFF4  bne cr6, 0x831689d8
	if !ctx.cr[6].eq {
	pc = 0x831689D8; continue 'dispatch;
	}
	// 831689E8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831689EC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831689F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831689F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831689F8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 831689FC: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 83168A00: 4BFFF1B9  bl 0x83167bb8
	ctx.lr = 0x83168A04;
	sub_83167BB8(ctx, base);
	// 83168A04: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 83168A08: 4800001C  b 0x83168a24
	pc = 0x83168A24; continue 'dispatch;
	// 83168A0C: FBA30008  std r29, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 83168A10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83168A14: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168A18: 4BFFF4A9  bl 0x83167ec0
	ctx.lr = 0x83168A1C;
	sub_83167EC0(ctx, base);
	// 83168A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83168A20: 4BFFF199  bl 0x83167bb8
	ctx.lr = 0x83168A24;
	sub_83167BB8(ctx, base);
	// 83168A24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83168A28: 4082FFE4  bne 0x83168a0c
	if !ctx.cr[0].eq {
	pc = 0x83168A0C; continue 'dispatch;
	}
	// 83168A2C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168A30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83168A34: 419A0020  beq cr6, 0x83168a54
	if ctx.cr[6].eq {
	pc = 0x83168A54; continue 'dispatch;
	}
	// 83168A38: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168A3C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83168A40: 93C40000  stw r30, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83168A44: FBA40008  std r29, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 83168A48: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168A4C: 4BFFF475  bl 0x83167ec0
	ctx.lr = 0x83168A50;
	sub_83167EC0(ctx, base);
	// 83168A50: 4BFFFFDC  b 0x83168a2c
	pc = 0x83168A2C; continue 'dispatch;
	// 83168A54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83168A58: 4803F764  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168A60 size=136
    let mut pc: u32 = 0x83168A60;
    'dispatch: loop {
        match pc {
            0x83168A60 => {
    //   block [0x83168A60..0x83168AE8)
	// 83168A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168A64: 4803F705  bl 0x831a8168
	ctx.lr = 0x83168A68;
	sub_831A8130(ctx, base);
	// 83168A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168A6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83168A70: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168A74: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83168A78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83168A7C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83168A80: 38AB7A70  addi r5, r11, 0x7a70
	ctx.r[5].s64 = ctx.r[11].s64 + 31344;
	// 83168A84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83168A88: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83168A8C: 4BFF726D  bl 0x8315fcf8
	ctx.lr = 0x83168A90;
	sub_8315FCF8(ctx, base);
	// 83168A90: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83168A94: 4082001C  bne 0x83168ab0
	if !ctx.cr[0].eq {
	pc = 0x83168AB0; continue 'dispatch;
	}
	// 83168A98: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168A9C: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83168AA0: 388B7A64  addi r4, r11, 0x7a64
	ctx.r[4].s64 = ctx.r[11].s64 + 31332;
	// 83168AA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168AA8: 4BFF7099  bl 0x8315fb40
	ctx.lr = 0x83168AAC;
	sub_8315FB40(ctx, base);
	// 83168AAC: 48000030  b 0x83168adc
	pc = 0x83168ADC; continue 'dispatch;
	// 83168AB0: 397D0040  addi r11, r29, 0x40
	ctx.r[11].s64 = ctx.r[29].s64 + 64;
	// 83168AB4: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 83168AB8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83168ABC: 7CBC5BD6  divw r5, r28, r11
	ctx.r[5].s32 = ctx.r[28].s32 / ctx.r[11].s32;
	// 83168AC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83168AC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83168AC8: 4BFFF1F1  bl 0x83167cb8
	ctx.lr = 0x83168ACC;
	sub_83167CB8(ctx, base);
	// 83168ACC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83168AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83168AD4: 4082000C  bne 0x83168ae0
	if !ctx.cr[0].eq {
	pc = 0x83168AE0; continue 'dispatch;
	}
	// 83168AD8: 4BFFF389  bl 0x83167e60
	ctx.lr = 0x83168ADC;
	sub_83167E60(ctx, base);
	// 83168ADC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83168AE4: 4803F6D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83168AE8 size=152
    let mut pc: u32 = 0x83168AE8;
    'dispatch: loop {
        match pc {
            0x83168AE8 => {
    //   block [0x83168AE8..0x83168B80)
	// 83168AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168AF4: 80A30018  lwz r5, 0x18(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168AF8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83168AFC: 419A0018  beq cr6, 0x83168b14
	if ctx.cr[6].eq {
	pc = 0x83168B14; continue 'dispatch;
	}
	// 83168B00: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83168B08: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83168B0C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83168B10: 4800005C  b 0x83168b6c
	pc = 0x83168B6C; continue 'dispatch;
	// 83168B14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83168B18: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83168B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83168B20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168B24: C18B08A4  lfs f12, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83168B28: 419A0044  beq cr6, 0x83168b6c
	if ctx.cr[6].eq {
	pc = 0x83168B6C; continue 'dispatch;
	}
	// 83168B2C: 4BFFFC55  bl 0x83168780
	ctx.lr = 0x83168B30;
	sub_83168780(ctx, base);
	// 83168B30: FF0C0800  fcmpu cr6, f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[1].f64);
	// 83168B34: 40980010  bge cr6, 0x83168b44
	if !ctx.cr[6].lt {
	pc = 0x83168B44; continue 'dispatch;
	}
	// 83168B38: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83168B3C: 4BFFFC45  bl 0x83168780
	ctx.lr = 0x83168B40;
	sub_83168780(ctx, base);
	// 83168B40: FD800890  fmr f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = ctx.f[1].f64;
	// 83168B44: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168B48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168B4C: 409AFFE0  bne cr6, 0x83168b2c
	if !ctx.cr[6].eq {
	pc = 0x83168B2C; continue 'dispatch;
	}
	// 83168B50: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 83168B54: 419A0018  beq cr6, 0x83168b6c
	if ctx.cr[6].eq {
	pc = 0x83168B6C; continue 'dispatch;
	}
	// 83168B58: 7F062040  cmplw cr6, r6, r4
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83168B5C: 419A0010  beq cr6, 0x83168b6c
	if ctx.cr[6].eq {
	pc = 0x83168B6C; continue 'dispatch;
	}
	// 83168B60: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83168B64: 4BFFF0AD  bl 0x83167c10
	ctx.lr = 0x83168B68;
	sub_83167C10(ctx, base);
	// 83168B68: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83168B6C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83168B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83168B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168B80 size=176
    let mut pc: u32 = 0x83168B80;
    'dispatch: loop {
        match pc {
            0x83168B80 => {
    //   block [0x83168B80..0x83168C30)
	// 83168B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168B90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168B94: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83168B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83168B9C: 419A0080  beq cr6, 0x83168c1c
	if ctx.cr[6].eq {
	pc = 0x83168C1C; continue 'dispatch;
	}
	// 83168BA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83168BA4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83168BA8: 409A0010  bne cr6, 0x83168bb8
	if !ctx.cr[6].eq {
	pc = 0x83168BB8; continue 'dispatch;
	}
	// 83168BAC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168BB0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83168BB4: 48000038  b 0x83168bec
	pc = 0x83168BEC; continue 'dispatch;
	// 83168BB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168BBC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83168BC0: 419A0030  beq cr6, 0x83168bf0
	if ctx.cr[6].eq {
	pc = 0x83168BF0; continue 'dispatch;
	}
	// 83168BC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168BC8: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83168BCC: 419A0018  beq cr6, 0x83168be4
	if ctx.cr[6].eq {
	pc = 0x83168BE4; continue 'dispatch;
	}
	// 83168BD0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168BD4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83168BD8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83168BDC: 409AFFE8  bne cr6, 0x83168bc4
	if !ctx.cr[6].eq {
	pc = 0x83168BC4; continue 'dispatch;
	}
	// 83168BE0: 48000010  b 0x83168bf0
	pc = 0x83168BF0; continue 'dispatch;
	// 83168BE4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168BE8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83168BEC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83168BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83168BF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83168BF8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83168BFC: 409A0018  bne cr6, 0x83168c14
	if !ctx.cr[6].eq {
	pc = 0x83168C14; continue 'dispatch;
	}
	// 83168C00: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83168C08: 388B7A84  addi r4, r11, 0x7a84
	ctx.r[4].s64 = ctx.r[11].s64 + 31364;
	// 83168C0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168C10: 4BFF6F31  bl 0x8315fb40
	ctx.lr = 0x83168C14;
	sub_8315FB40(ctx, base);
	// 83168C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83168C18: 4BFFF359  bl 0x83167f70
	ctx.lr = 0x83168C1C;
	sub_83167F70(ctx, base);
	// 83168C1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83168C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168C28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168C30 size=20
    let mut pc: u32 = 0x83168C30;
    'dispatch: loop {
        match pc {
            0x83168C30 => {
    //   block [0x83168C30..0x83168C44)
	// 83168C30: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83168C34: 816A8474  lwz r11, -0x7b8c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31628 as u32) ) } as u64;
	// 83168C38: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83168C3C: 916A8474  stw r11, -0x7b8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31628 as u32), ctx.r[11].u32 ) };
	// 83168C40: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168C44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168C44 size=8
    let mut pc: u32 = 0x83168C44;
    'dispatch: loop {
        match pc {
            0x83168C44 => {
    //   block [0x83168C44..0x83168C4C)
	// 83168C44: 4BFFF5B4  b 0x831681f8
	sub_831681F8(ctx, base);
	return;
	// 83168C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168C50 size=116
    let mut pc: u32 = 0x83168C50;
    'dispatch: loop {
        match pc {
            0x83168C50 => {
    //   block [0x83168C50..0x83168CC4)
	// 83168C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83168C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168C68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83168C6C: 93DF0264  stw r30, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[30].u32 ) };
	// 83168C70: 93DF0398  stw r30, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[30].u32 ) };
	// 83168C74: 93DF03A0  stw r30, 0x3a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(928 as u32), ctx.r[30].u32 ) };
	// 83168C78: 93DF0258  stw r30, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[30].u32 ) };
	// 83168C7C: 93DF0250  stw r30, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[30].u32 ) };
	// 83168C80: 93DF0254  stw r30, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[30].u32 ) };
	// 83168C84: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 83168C88: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83168C8C: 409A0010  bne cr6, 0x83168c9c
	if !ctx.cr[6].eq {
	pc = 0x83168C9C; continue 'dispatch;
	}
	// 83168C90: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83168C94: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83168C98: 916A8478  stw r11, -0x7b88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31624 as u32), ctx.r[11].u32 ) };
	// 83168C9C: 93DF0268  stw r30, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u32 ) };
	// 83168CA0: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168CA4: 4BFFFD0D  bl 0x831689b0
	ctx.lr = 0x83168CA8;
	sub_831689B0(ctx, base);
	// 83168CA8: 93DF03A4  stw r30, 0x3a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(932 as u32), ctx.r[30].u32 ) };
	// 83168CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83168CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168CB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83168CBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83168CC8 size=16
    let mut pc: u32 = 0x83168CC8;
    'dispatch: loop {
        match pc {
            0x83168CC8 => {
    //   block [0x83168CC8..0x83168CD8)
	// 83168CC8: 81630260  lwz r11, 0x260(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168CCC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168CD0: D02B001C  stfs f1, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83168CD4: 4BFFF29C  b 0x83167f70
	sub_83167F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83168CD8 size=8
    let mut pc: u32 = 0x83168CD8;
    'dispatch: loop {
        match pc {
            0x83168CD8 => {
    //   block [0x83168CD8..0x83168CE0)
	// 83168CD8: 80630260  lwz r3, 0x260(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168CDC: 4BFFFAA4  b 0x83168780
	sub_83168780(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83168CE0 size=452
    let mut pc: u32 = 0x83168CE0;
    'dispatch: loop {
        match pc {
            0x83168CE0 => {
    //   block [0x83168CE0..0x83168EA4)
	// 83168CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168CE4: 4803F489  bl 0x831a816c
	ctx.lr = 0x83168CE8;
	sub_831A8130(ctx, base);
	// 83168CE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168CEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83168CF0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83168CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168CF8: 419A01A0  beq cr6, 0x83168e98
	if ctx.cr[6].eq {
	pc = 0x83168E98; continue 'dispatch;
	}
	// 83168CFC: 817E0390  lwz r11, 0x390(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(912 as u32) ) } as u64;
	// 83168D00: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83168D04: 419A0194  beq cr6, 0x83168e98
	if ctx.cr[6].eq {
	pc = 0x83168E98; continue 'dispatch;
	}
	// 83168D08: 817E0264  lwz r11, 0x264(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(612 as u32) ) } as u64;
	// 83168D0C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83168D10: 409A0188  bne cr6, 0x83168e98
	if !ctx.cr[6].eq {
	pc = 0x83168E98; continue 'dispatch;
	}
	// 83168D14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83168D18: 4BFF8F01  bl 0x83161c18
	ctx.lr = 0x83168D1C;
	sub_83161C18(ctx, base);
	// 83168D1C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83168D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83168D24: 419A00B0  beq cr6, 0x83168dd4
	if ctx.cr[6].eq {
	pc = 0x83168DD4; continue 'dispatch;
	}
	// 83168D28: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168D2C: 3BAB7A90  addi r29, r11, 0x7a90
	ctx.r[29].s64 = ctx.r[11].s64 + 31376;
	// 83168D30: 817E0260  lwz r11, 0x260(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168D34: 83EB0018  lwz r31, 0x18(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168D38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83168D3C: 409A0010  bne cr6, 0x83168d4c
	if !ctx.cr[6].eq {
	pc = 0x83168D4C; continue 'dispatch;
	}
	// 83168D40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83168D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168D48: 4BFF6DD1  bl 0x8315fb18
	ctx.lr = 0x83168D4C;
	sub_8315FB18(ctx, base);
	// 83168D4C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83168D50: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83168D54: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83168D58: 40990010  ble cr6, 0x83168d68
	if !ctx.cr[6].gt {
	pc = 0x83168D68; continue 'dispatch;
	}
	// 83168D5C: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 83168D60: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83168D64: 48000058  b 0x83168dbc
	pc = 0x83168DBC; continue 'dispatch;
	// 83168D68: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83168D6C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83168D70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83168D74: 4BFF8E45  bl 0x83161bb8
	ctx.lr = 0x83168D78;
	sub_83161BB8(ctx, base);
	// 83168D78: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83168D7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83168D80: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83168D84: 4BFF8E05  bl 0x83161b88
	ctx.lr = 0x83168D88;
	sub_83161B88(ctx, base);
	// 83168D88: 817E0260  lwz r11, 0x260(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168D8C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168D90: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83168D94: 419A0028  beq cr6, 0x83168dbc
	if ctx.cr[6].eq {
	pc = 0x83168DBC; continue 'dispatch;
	}
	// 83168D98: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83168DA0: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83168DA4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83168DA8: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83168DAC: 817E0260  lwz r11, 0x260(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168DB0: F9440008  std r10, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83168DB4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83168DB8: 4BFFF109  bl 0x83167ec0
	ctx.lr = 0x83168DBC;
	sub_83167EC0(ctx, base);
	// 83168DBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83168DC0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83168DC4: 4BFF8E55  bl 0x83161c18
	ctx.lr = 0x83168DC8;
	sub_83161C18(ctx, base);
	// 83168DC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83168DCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83168DD0: 409AFF60  bne cr6, 0x83168d30
	if !ctx.cr[6].eq {
	pc = 0x83168D30; continue 'dispatch;
	}
	// 83168DD4: 807E0260  lwz r3, 0x260(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168DD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83168DDC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168DE0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83168DE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83168DE8: 4182005C  beq 0x83168e44
	if ctx.cr[0].eq {
	pc = 0x83168E44; continue 'dispatch;
	}
	// 83168DEC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168DF0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83168DF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83168DF8: 409AFFF4  bne cr6, 0x83168dec
	if !ctx.cr[6].eq {
	pc = 0x83168DEC; continue 'dispatch;
	}
	// 83168DFC: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 83168E00: 41980044  blt cr6, 0x83168e44
	if ctx.cr[6].lt {
	pc = 0x83168E44; continue 'dispatch;
	}
	// 83168E04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83168E08: 41820018  beq 0x83168e20
	if ctx.cr[0].eq {
	pc = 0x83168E20; continue 'dispatch;
	}
	// 83168E0C: E92B0016  lwa r9, 0x14(r11)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83168E10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168E14: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83168E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83168E1C: 409AFFF0  bne cr6, 0x83168e0c
	if !ctx.cr[6].eq {
	pc = 0x83168E0C; continue 'dispatch;
	}
	// 83168E20: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83168E24: C1A3001C  lfs f13, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83168E28: C00B17A0  lfs f0, 0x17a0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83168E2C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 83168E30: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 83168E34: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 83168E38: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83168E3C: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 83168E40: 40980058  bge cr6, 0x83168e98
	if !ctx.cr[6].lt {
	pc = 0x83168E98; continue 'dispatch;
	}
	// 83168E44: 4BFFED75  bl 0x83167bb8
	ctx.lr = 0x83168E48;
	sub_83167BB8(ctx, base);
	// 83168E48: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83168E4C: 4182004C  beq 0x83168e98
	if ctx.cr[0].eq {
	pc = 0x83168E98; continue 'dispatch;
	}
	// 83168E50: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83168E54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83168E58: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168E5C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83168E60: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83168E64: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83168E68: 4BFF8DC1  bl 0x83161c28
	ctx.lr = 0x83168E6C;
	sub_83161C28(ctx, base);
	// 83168E6C: 815E0260  lwz r10, 0x260(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(608 as u32) ) } as u64;
	// 83168E70: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 83168E74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83168E78: 409A0010  bne cr6, 0x83168e88
	if !ctx.cr[6].eq {
	pc = 0x83168E88; continue 'dispatch;
	}
	// 83168E7C: 93EA0018  stw r31, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 83168E80: 48000018  b 0x83168e98
	pc = 0x83168E98; continue 'dispatch;
	// 83168E84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168E88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168E8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83168E90: 409AFFF4  bne cr6, 0x83168e84
	if !ctx.cr[6].eq {
	pc = 0x83168E84; continue 'dispatch;
	}
	// 83168E94: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83168E98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168E9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83168EA0: 4803F31C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168EA8 size=144
    let mut pc: u32 = 0x83168EA8;
    'dispatch: loop {
        match pc {
            0x83168EA8 => {
    //   block [0x83168EA8..0x83168F38)
	// 83168EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168EB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168EBC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168EC0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83168EC4: 38CB7ADC  addi r6, r11, 0x7adc
	ctx.r[6].s64 = ctx.r[11].s64 + 31452;
	// 83168EC8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83168ECC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83168ED0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168ED4: 4BFF6D45  bl 0x8315fc18
	ctx.lr = 0x83168ED8;
	sub_8315FC18(ctx, base);
	// 83168ED8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83168EDC: 41820028  beq 0x83168f04
	if ctx.cr[0].eq {
	pc = 0x83168F04; continue 'dispatch;
	}
	// 83168EE0: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83168EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83168EE8: 394A78BC  addi r10, r10, 0x78bc
	ctx.r[10].s64 = ctx.r[10].s64 + 30908;
	// 83168EEC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83168EF0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168EF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83168EF8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83168EFC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83168F00: 48000008  b 0x83168f08
	pc = 0x83168F08; continue 'dispatch;
	// 83168F04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168F08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83168F0C: 409A0018  bne cr6, 0x83168f24
	if !ctx.cr[6].eq {
	pc = 0x83168F24; continue 'dispatch;
	}
	// 83168F10: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168F14: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83168F18: 388B7AD0  addi r4, r11, 0x7ad0
	ctx.r[4].s64 = ctx.r[11].s64 + 31440;
	// 83168F1C: 4BFF6C25  bl 0x8315fb40
	ctx.lr = 0x83168F20;
	sub_8315FB40(ctx, base);
	// 83168F20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83168F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83168F38 size=72
    let mut pc: u32 = 0x83168F38;
    'dispatch: loop {
        match pc {
            0x83168F38 => {
    //   block [0x83168F38..0x83168F80)
	// 83168F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83168F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83168F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168F48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168F4C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168F50: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83168F54: 396B77C8  addi r11, r11, 0x77c8
	ctx.r[11].s64 = ctx.r[11].s64 + 30664;
	// 83168F58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83168F5C: 4182000C  beq 0x83168f68
	if ctx.cr[0].eq {
	pc = 0x83168F68; continue 'dispatch;
	}
	// 83168F60: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83168F64: 4BFF6D1D  bl 0x8315fc80
	ctx.lr = 0x83168F68;
	sub_8315FC80(ctx, base);
	// 83168F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83168F6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83168F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83168F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83168F78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83168F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83168F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83168F80 size=188
    let mut pc: u32 = 0x83168F80;
    'dispatch: loop {
        match pc {
            0x83168F80 => {
    //   block [0x83168F80..0x8316903C)
	// 83168F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83168F84: 4803F1E5  bl 0x831a8168
	ctx.lr = 0x83168F88;
	sub_831A8130(ctx, base);
	// 83168F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83168F8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83168F90: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168F94: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83168F98: 38AB7AF4  addi r5, r11, 0x7af4
	ctx.r[5].s64 = ctx.r[11].s64 + 31476;
	// 83168F9C: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83168FA0: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83168FA4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83168FA8: 4BFF6D51  bl 0x8315fcf8
	ctx.lr = 0x83168FAC;
	sub_8315FCF8(ctx, base);
	// 83168FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83168FB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83168FB4: 409A001C  bne cr6, 0x83168fd0
	if !ctx.cr[6].eq {
	pc = 0x83168FD0; continue 'dispatch;
	}
	// 83168FB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83168FBC: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83168FC0: 388B5294  addi r4, r11, 0x5294
	ctx.r[4].s64 = ctx.r[11].s64 + 21140;
	// 83168FC4: 4BFF6B7D  bl 0x8315fb40
	ctx.lr = 0x83168FC8;
	sub_8315FB40(ctx, base);
	// 83168FC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83168FCC: 48000068  b 0x83169034
	pc = 0x83169034; continue 'dispatch;
	// 83168FD0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83168FD4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83168FD8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83168FDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83168FE0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83168FE4: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83168FE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83168FEC: 93FD001C  stw r31, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 83168FF0: 4BFFEF81  bl 0x83167f70
	ctx.lr = 0x83168FF4;
	sub_83167F70(ctx, base);
	// 83168FF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83168FF8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83168FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169000: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83169004: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83169008: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8316900C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83169010: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83169014: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83169018: FBDF0028  std r30, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u64 ) };
	// 8316901C: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 83169020: FBDF0030  std r30, 0x30(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u64 ) };
	// 83169024: FBDF0038  std r30, 0x38(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u64 ) };
	// 83169028: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 8316902C: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 83169030: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 83169034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83169038: 4803F180  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169040 size=68
    let mut pc: u32 = 0x83169040;
    'dispatch: loop {
        match pc {
            0x83169040 => {
    //   block [0x83169040..0x83169084)
	// 83169040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316904C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169050: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169054: 4BFFF95D  bl 0x831689b0
	ctx.lr = 0x83169058;
	sub_831689B0(ctx, base);
	// 83169058: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316905C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83169060: 4BFFFB21  bl 0x83168b80
	ctx.lr = 0x83169064;
	sub_83168B80(ctx, base);
	// 83169064: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 83169068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316906C: 4BFF6C15  bl 0x8315fc80
	ctx.lr = 0x83169070;
	sub_8315FC80(ctx, base);
	// 83169070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83169074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316907C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169088 size=232
    let mut pc: u32 = 0x83169088;
    'dispatch: loop {
        match pc {
            0x83169088 => {
    //   block [0x83169088..0x83169170)
	// 83169088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316908C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316909C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831690A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831690A4: 4BFFFA45  bl 0x83168ae8
	ctx.lr = 0x831690A8;
	sub_83168AE8(ctx, base);
	// 831690A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831690AC: 4082000C  bne 0x831690b8
	if !ctx.cr[0].eq {
	pc = 0x831690B8; continue 'dispatch;
	}
	// 831690B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831690B4: 480000A8  b 0x8316915c
	pc = 0x8316915C; continue 'dispatch;
	// 831690B8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831690BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831690C0: 48000010  b 0x831690d0
	pc = 0x831690D0; continue 'dispatch;
	// 831690C4: E94B0016  lwa r10, 0x14(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 831690C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831690CC: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 831690D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831690D4: 409AFFF0  bne cr6, 0x831690c4
	if !ctx.cr[6].eq {
	pc = 0x831690C4; continue 'dispatch;
	}
	// 831690D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831690DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831690E0: 48000010  b 0x831690f0
	pc = 0x831690F0; continue 'dispatch;
	// 831690E4: E90B0016  lwa r8, 0x14(r11)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 831690E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831690EC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 831690F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831690F4: 409AFFF0  bne cr6, 0x831690e4
	if !ctx.cr[6].eq {
	pc = 0x831690E4; continue 'dispatch;
	}
	// 831690F8: E97F0038  ld r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 831690FC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83169100: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83169104: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83169108: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 8316910C: E97F0030  ld r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	// 83169110: E95F0028  ld r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 83169114: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 83169118: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8316911C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83169120: 7D695851  subf. r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169124: 4082001C  bne 0x83169140
	if !ctx.cr[0].eq {
	pc = 0x83169140; continue 'dispatch;
	}
	// 83169128: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316912C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83169130: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83169134: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83169138: 4BFFED89  bl 0x83167ec0
	ctx.lr = 0x8316913C;
	sub_83167EC0(ctx, base);
	// 8316913C: 4BFFFF74  b 0x831690b0
	pc = 0x831690B0; continue 'dispatch;
	// 83169140: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83169144: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83169148: 40980008  bge cr6, 0x83169150
	if !ctx.cr[6].lt {
	pc = 0x83169150; continue 'dispatch;
	}
	// 8316914C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83169150: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83169154: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83169158: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8316915C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83169160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316916C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169170 size=116
    let mut pc: u32 = 0x83169170;
    'dispatch: loop {
        match pc {
            0x83169170 => {
    //   block [0x83169170..0x831691E4)
	// 83169170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169174: 4803EFF9  bl 0x831a816c
	ctx.lr = 0x83169178;
	sub_831A8130(ctx, base);
	// 83169178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316917C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169180: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83169184: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83169188: 4BFF6899  bl 0x8315fa20
	ctx.lr = 0x8316918C;
	sub_8315FA20(ctx, base);
	// 8316918C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83169190: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83169194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169198: 4BFFF8C9  bl 0x83168a60
	ctx.lr = 0x8316919C;
	sub_83168A60(ctx, base);
	// 8316919C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831691A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831691A4: 906B8470  stw r3, -0x7b90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-31632 as u32), ctx.r[3].u32 ) };
	// 831691A8: 40820020  bne 0x831691c8
	if !ctx.cr[0].eq {
	pc = 0x831691C8; continue 'dispatch;
	}
	// 831691AC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831691B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831691B4: 388B7B04  addi r4, r11, 0x7b04
	ctx.r[4].s64 = ctx.r[11].s64 + 31492;
	// 831691B8: 4BFF6961  bl 0x8315fb18
	ctx.lr = 0x831691BC;
	sub_8315FB18(ctx, base);
	// 831691BC: 4BFFF03D  bl 0x831681f8
	ctx.lr = 0x831691C0;
	sub_831681F8(ctx, base);
	// 831691C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831691C4: 48000018  b 0x831691dc
	pc = 0x831691DC; continue 'dispatch;
	// 831691C8: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 831691CC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 831691D0: 396B8EA8  addi r11, r11, -0x7158
	ctx.r[11].s64 = ctx.r[11].s64 + -29016;
	// 831691D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831691D8: 916A8468  stw r11, -0x7b98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31640 as u32), ctx.r[11].u32 ) };
	// 831691DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831691E0: 4803EFDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831691E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831691E8 size=128
    let mut pc: u32 = 0x831691E8;
    'dispatch: loop {
        match pc {
            0x831691E8 => {
    //   block [0x831691E8..0x83169268)
	// 831691E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831691EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831691F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831691F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831691F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831691FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169200: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83169204: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 83169208: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316920C: 419A0018  beq cr6, 0x83169224
	if ctx.cr[6].eq {
	pc = 0x83169224; continue 'dispatch;
	}
	// 83169210: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169214: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316921C: 4E800421  bctrl
	ctx.lr = 0x83169220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83169220: 93DF026C  stw r30, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[30].u32 ) };
	// 83169224: 809F0394  lwz r4, 0x394(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83169228: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316922C: 419A0010  beq cr6, 0x8316923c
	if ctx.cr[6].eq {
	pc = 0x8316923C; continue 'dispatch;
	}
	// 83169230: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169234: 4BFF6535  bl 0x8315f768
	ctx.lr = 0x83169238;
	sub_8315F768(ctx, base);
	// 83169238: 93DF0394  stw r30, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[30].u32 ) };
	// 8316923C: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 83169240: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83169244: 419A000C  beq cr6, 0x83169250
	if ctx.cr[6].eq {
	pc = 0x83169250; continue 'dispatch;
	}
	// 83169248: 4BFFFDF9  bl 0x83169040
	ctx.lr = 0x8316924C;
	sub_83169040(ctx, base);
	// 8316924C: 93DF0260  stw r30, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u32 ) };
	// 83169250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83169254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316925C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83169260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169268 size=160
    let mut pc: u32 = 0x83169268;
    'dispatch: loop {
        match pc {
            0x83169268 => {
    //   block [0x83169268..0x83169308)
	// 83169268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316927C: 817F0264  lwz r11, 0x264(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(612 as u32) ) } as u64;
	// 83169280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169284: 419A006C  beq cr6, 0x831692f0
	if ctx.cr[6].eq {
	pc = 0x831692F0; continue 'dispatch;
	}
	// 83169288: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 8316928C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169290: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83169294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169298: 4E800421  bctrl
	ctx.lr = 0x8316929C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316929C: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 831692A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831692A4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831692A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831692AC: 4E800421  bctrl
	ctx.lr = 0x831692B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831692B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831692B4: 41820034  beq 0x831692e8
	if ctx.cr[0].eq {
	pc = 0x831692E8; continue 'dispatch;
	}
	// 831692B8: 817F03A4  lwz r11, 0x3a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(932 as u32) ) } as u64;
	// 831692BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831692C0: 409A0020  bne cr6, 0x831692e0
	if !ctx.cr[6].eq {
	pc = 0x831692E0; continue 'dispatch;
	}
	// 831692C4: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 831692C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831692CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831692D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831692D4: 4E800421  bctrl
	ctx.lr = 0x831692D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831692D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831692DC: 917F03A4  stw r11, 0x3a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(932 as u32), ctx.r[11].u32 ) };
	// 831692E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831692E4: 48000010  b 0x831692f4
	pc = 0x831692F4; continue 'dispatch;
	// 831692E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831692EC: 4BFFF965  bl 0x83168c50
	ctx.lr = 0x831692F0;
	sub_83168C50(ctx, base);
	// 831692F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831692F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831692F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831692FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83169308 size=972
    let mut pc: u32 = 0x83169308;
    'dispatch: loop {
        match pc {
            0x83169308 => {
    //   block [0x83169308..0x831696D4)
	// 83169308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316930C: 4803EE49  bl 0x831a8154
	ctx.lr = 0x83169310;
	sub_831A8130(ctx, base);
	// 83169310: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 83169314: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169318: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316931C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83169320: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 83169324: 817F0390  lwz r11, 0x390(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(912 as u32) ) } as u64;
	// 83169328: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316932C: 409A000C  bne cr6, 0x83169338
	if !ctx.cr[6].eq {
	pc = 0x83169338; continue 'dispatch;
	}
	// 83169330: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169334: 48000394  b 0x831696c8
	pc = 0x831696C8; continue 'dispatch;
	// 83169338: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 8316933C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169340: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83169344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169348: 4E800421  bctrl
	ctx.lr = 0x8316934C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316934C: 817F0264  lwz r11, 0x264(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(612 as u32) ) } as u64;
	// 83169350: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83169354: 409A0370  bne cr6, 0x831696c4
	if !ctx.cr[6].eq {
	pc = 0x831696C4; continue 'dispatch;
	}
	// 83169358: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 8316935C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169360: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83169364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169368: 4E800421  bctrl
	ctx.lr = 0x8316936C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316936C: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 83169370: 409A0014  bne cr6, 0x83169384
	if !ctx.cr[6].eq {
	pc = 0x83169384; continue 'dispatch;
	}
	// 83169374: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83169378: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316937C: 917F0264  stw r11, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[11].u32 ) };
	// 83169380: 48000348  b 0x831696c8
	pc = 0x831696C8; continue 'dispatch;
	// 83169384: 80FF0268  lwz r7, 0x268(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 83169388: 3AE00002  li r23, 2
	ctx.r[23].s64 = 2;
	// 8316938C: 2F070005  cmpwi cr6, r7, 5
	ctx.cr[6].compare_i32(ctx.r[7].s32, 5, &mut ctx.xer);
	// 83169390: 409A0024  bne cr6, 0x831693b4
	if !ctx.cr[6].eq {
	pc = 0x831693B4; continue 'dispatch;
	}
	// 83169394: 817F03A0  lwz r11, 0x3a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(928 as u32) ) } as u64;
	// 83169398: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316939C: 409A0018  bne cr6, 0x831693b4
	if !ctx.cr[6].eq {
	pc = 0x831693B4; continue 'dispatch;
	}
	// 831693A0: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 831693A4: 4BFFF4E5  bl 0x83168888
	ctx.lr = 0x831693A8;
	sub_83168888(ctx, base);
	// 831693A8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831693AC: 409A0008  bne cr6, 0x831693b4
	if !ctx.cr[6].eq {
	pc = 0x831693B4; continue 'dispatch;
	}
	// 831693B0: 92FF0264  stw r23, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[23].u32 ) };
	// 831693B4: 817F03A4  lwz r11, 0x3a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(932 as u32) ) } as u64;
	// 831693B8: 3F20833A  lis r25, -0x7cc6
	ctx.r[25].s64 = -2093350912;
	// 831693BC: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 831693C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831693C4: 409A0288  bne cr6, 0x8316964c
	if !ctx.cr[6].eq {
	pc = 0x8316964C; continue 'dispatch;
	}
	// 831693C8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831693CC: 419A000C  beq cr6, 0x831693d8
	if ctx.cr[6].eq {
	pc = 0x831693D8; continue 'dispatch;
	}
	// 831693D0: 2F070005  cmpwi cr6, r7, 5
	ctx.cr[6].compare_i32(ctx.r[7].s32, 5, &mut ctx.xer);
	// 831693D4: 409A00E0  bne cr6, 0x831694b4
	if !ctx.cr[6].eq {
	pc = 0x831694B4; continue 'dispatch;
	}
	// 831693D8: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 831693DC: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 831693E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831693E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831693E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831693EC: 4E800421  bctrl
	ctx.lr = 0x831693F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831693F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831693F4: 41820008  beq 0x831693fc
	if ctx.cr[0].eq {
	pc = 0x831693FC; continue 'dispatch;
	}
	// 831693F8: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 831693FC: 3BDF0270  addi r30, r31, 0x270
	ctx.r[30].s64 = ctx.r[31].s64 + 624;
	// 83169400: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 83169404: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83169408: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316940C: 4BFFE695  bl 0x83167aa0
	ctx.lr = 0x83169410;
	sub_83167AA0(ctx, base);
	// 83169410: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83169414: 409A00A0  bne cr6, 0x831694b4
	if !ctx.cr[6].eq {
	pc = 0x831694B4; continue 'dispatch;
	}
	// 83169418: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8316941C: 409A0098  bne cr6, 0x831694b4
	if !ctx.cr[6].eq {
	pc = 0x831694B4; continue 'dispatch;
	}
	// 83169420: 817F0260  lwz r11, 0x260(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 83169424: E93F0380  ld r9, 0x380(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(896 as u32) ) };
	// 83169428: E95F0378  ld r10, 0x378(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(888 as u32) ) };
	// 8316942C: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83169430: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83169434: 409A0080  bne cr6, 0x831694b4
	if !ctx.cr[6].eq {
	pc = 0x831694B4; continue 'dispatch;
	}
	// 83169438: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316943C: 38BF0274  addi r5, r31, 0x274
	ctx.r[5].s64 = ctx.r[31].s64 + 628;
	// 83169440: F94B0028  std r10, 0x28(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u64 ) };
	// 83169444: F92B0030  std r9, 0x30(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[9].u64 ) };
	// 83169448: 90AB0024  stw r5, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 8316944C: F94B0038  std r10, 0x38(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u64 ) };
	// 83169450: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 83169454: 934B000C  stw r26, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 83169458: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316945C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83169460: 409A0018  bne cr6, 0x83169478
	if !ctx.cr[6].eq {
	pc = 0x83169478; continue 'dispatch;
	}
	// 83169464: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316946C: 388B7B38  addi r4, r11, 0x7b38
	ctx.r[4].s64 = ctx.r[11].s64 + 31544;
	// 83169470: 4BFF66A9  bl 0x8315fb18
	ctx.lr = 0x83169474;
	sub_8315FB18(ctx, base);
	// 83169474: 4BFFFF00  b 0x83169374
	pc = 0x83169374; continue 'dispatch;
	// 83169478: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 8316947C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169480: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83169484: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169488: 4E800421  bctrl
	ctx.lr = 0x8316948C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316948C: 817F0260  lwz r11, 0x260(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 83169490: 934B0044  stw r26, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[26].u32 ) };
	// 83169494: 931F0268  stw r24, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[24].u32 ) };
	// 83169498: 817F0388  lwz r11, 0x388(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(904 as u32) ) } as u64;
	// 8316949C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831694A0: 409A0010  bne cr6, 0x831694b0
	if !ctx.cr[6].eq {
	pc = 0x831694B0; continue 'dispatch;
	}
	// 831694A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831694A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831694AC: 4BFFE6AD  bl 0x83167b58
	ctx.lr = 0x831694B0;
	sub_83167B58(ctx, base);
	// 831694B0: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 831694B4: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 831694B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831694BC: 409A0024  bne cr6, 0x831694e0
	if !ctx.cr[6].eq {
	pc = 0x831694E0; continue 'dispatch;
	}
	// 831694C0: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 831694C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831694C8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831694CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831694D0: 4E800421  bctrl
	ctx.lr = 0x831694D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831694D4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831694D8: 409A0008  bne cr6, 0x831694e0
	if !ctx.cr[6].eq {
	pc = 0x831694E0; continue 'dispatch;
	}
	// 831694DC: 92FF0268  stw r23, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[23].u32 ) };
	// 831694E0: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 831694E4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831694E8: 409A0058  bne cr6, 0x83169540
	if !ctx.cr[6].eq {
	pc = 0x83169540; continue 'dispatch;
	}
	// 831694EC: 80FF0260  lwz r7, 0x260(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 831694F0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831694F4: 4BFFF34D  bl 0x83168840
	ctx.lr = 0x831694F8;
	sub_83168840(ctx, base);
	// 831694F8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831694FC: 409A0044  bne cr6, 0x83169540
	if !ctx.cr[6].eq {
	pc = 0x83169540; continue 'dispatch;
	}
	// 83169500: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83169504: 4BFFF27D  bl 0x83168780
	ctx.lr = 0x83169508;
	sub_83168780(ctx, base);
	// 83169508: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8316950C: C00B9524  lfs f0, -0x6adc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83169510: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 83169514: 41990028  bgt cr6, 0x8316953c
	if ctx.cr[6].gt {
	pc = 0x8316953C; continue 'dispatch;
	}
	// 83169518: 817F0260  lwz r11, 0x260(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 8316951C: 930B0044  stw r24, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[24].u32 ) };
	// 83169520: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 83169524: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169528: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316952C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169530: 4E800421  bctrl
	ctx.lr = 0x83169534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83169534: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83169538: 917F0268  stw r11, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[11].u32 ) };
	// 8316953C: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 83169540: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 83169544: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83169548: 409A0104  bne cr6, 0x8316964c
	if !ctx.cr[6].eq {
	pc = 0x8316964C; continue 'dispatch;
	}
	// 8316954C: 81798478  lwz r11, -0x7b88(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-31624 as u32) ) } as u64;
	// 83169550: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169554: 409A00F8  bne cr6, 0x8316964c
	if !ctx.cr[6].eq {
	pc = 0x8316964C; continue 'dispatch;
	}
	// 83169558: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 8316955C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83169560: C3E30050  lfs f31, 0x50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83169564: 48000088  b 0x831695ec
	pc = 0x831695EC; continue 'dispatch;
	// 83169568: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 8316956C: 4BFFFB1D  bl 0x83169088
	ctx.lr = 0x83169570;
	sub_83169088(ctx, base);
	// 83169570: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83169574: 41820084  beq 0x831695f8
	if ctx.cr[0].eq {
	pc = 0x831695F8; continue 'dispatch;
	}
	// 83169578: 817F0398  lwz r11, 0x398(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 8316957C: 815F0394  lwz r10, 0x394(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83169580: 81240018  lwz r9, 0x18(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 83169584: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83169588: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316958C: 396B001F  addi r11, r11, 0x1f
	ctx.r[11].s64 = ctx.r[11].s64 + 31;
	// 83169590: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 83169594: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 83169598: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 8316959C: 813F0398  lwz r9, 0x398(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 831695A0: 7D0A0194  addze r8, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831695A4: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831695A8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831695AC: 550A2834  slwi r10, r8, 5
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831695B0: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 831695B4: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 831695B8: 817F0398  lwz r11, 0x398(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 831695BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831695C0: 409A0008  bne cr6, 0x831695c8
	if !ctx.cr[6].eq {
	pc = 0x831695C8; continue 'dispatch;
	}
	// 831695C4: EBC40008  ld r30, 8(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 831695C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831695CC: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 831695D0: 917F0398  stw r11, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[11].u32 ) };
	// 831695D4: 4BFFE695  bl 0x83167c68
	ctx.lr = 0x831695D8;
	sub_83167C68(ctx, base);
	// 831695D8: 817F0398  lwz r11, 0x398(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 831695DC: 815F039C  lwz r10, 0x39c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(924 as u32) ) } as u64;
	// 831695E0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831695E4: 40980014  bge cr6, 0x831695f8
	if !ctx.cr[6].lt {
	pc = 0x831695F8; continue 'dispatch;
	}
	// 831695E8: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 831695EC: 4BFFF2E5  bl 0x831688d0
	ctx.lr = 0x831695F0;
	sub_831688D0(ctx, base);
	// 831695F0: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 831695F4: 4099FF74  ble cr6, 0x83169568
	if !ctx.cr[6].gt {
	pc = 0x83169568; continue 'dispatch;
	}
	// 831695F8: 817F0398  lwz r11, 0x398(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 831695FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169600: 419A004C  beq cr6, 0x8316964c
	if ctx.cr[6].eq {
	pc = 0x8316964C; continue 'dispatch;
	}
	// 83169604: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83169608: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316960C: 91798478  stw r11, -0x7b88(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(-31624 as u32), ctx.r[11].u32 ) };
	// 83169610: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 83169614: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169618: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316961C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169620: 4E800421  bctrl
	ctx.lr = 0x83169624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83169624: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 83169628: 80BF0398  lwz r5, 0x398(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 8316962C: 809F0394  lwz r4, 0x394(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83169630: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169634: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83169638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316963C: 4E800421  bctrl
	ctx.lr = 0x83169640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83169640: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83169644: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 83169648: 917F0268  stw r11, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[11].u32 ) };
	// 8316964C: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 83169650: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83169654: 409A0070  bne cr6, 0x831696c4
	if !ctx.cr[6].eq {
	pc = 0x831696C4; continue 'dispatch;
	}
	// 83169658: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 8316965C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169660: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83169664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169668: 4E800421  bctrl
	ctx.lr = 0x8316966C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316966C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83169670: 409A0054  bne cr6, 0x831696c4
	if !ctx.cr[6].eq {
	pc = 0x831696C4; continue 'dispatch;
	}
	// 83169674: 817F0260  lwz r11, 0x260(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 83169678: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316967C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83169680: 409A0014  bne cr6, 0x83169694
	if !ctx.cr[6].eq {
	pc = 0x83169694; continue 'dispatch;
	}
	// 83169684: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83169688: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8316968C: 4800001C  b 0x831696a8
	pc = 0x831696A8; continue 'dispatch;
	// 83169690: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169694: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169698: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316969C: 409AFFF4  bne cr6, 0x83169690
	if !ctx.cr[6].eq {
	pc = 0x83169690; continue 'dispatch;
	}
	// 831696A0: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831696A4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831696A8: 934B0014  stw r26, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 831696AC: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 831696B0: 934B0040  stw r26, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[26].u32 ) };
	// 831696B4: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 831696B8: 935F0398  stw r26, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[26].u32 ) };
	// 831696BC: 92FF0268  stw r23, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[23].u32 ) };
	// 831696C0: 91798478  stw r11, -0x7b88(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(-31624 as u32), ctx.r[11].u32 ) };
	// 831696C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831696C8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831696CC: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 831696D0: 4803EAD4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831696D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831696D8 size=276
    let mut pc: u32 = 0x831696D8;
    'dispatch: loop {
        match pc {
            0x831696D8 => {
    //   block [0x831696D8..0x831697EC)
	// 831696D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831696DC: 4803EA91  bl 0x831a816c
	ctx.lr = 0x831696E0;
	sub_831A8130(ctx, base);
	// 831696E0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 831696E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831696E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831696EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831696F0: 83FD001C  lwz r31, 0x1c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 831696F4: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 831696F8: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831696FC: 41820028  beq 0x83169724
	if ctx.cr[0].eq {
	pc = 0x83169724; continue 'dispatch;
	}
	// 83169700: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83169704: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83169708: C1AA001C  lfs f13, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8316970C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 83169710: 419A0008  beq cr6, 0x83169718
	if ctx.cr[6].eq {
	pc = 0x83169718; continue 'dispatch;
	}
	// 83169714: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169718: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316971C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83169720: 409AFFE8  bne cr6, 0x83169708
	if !ctx.cr[6].eq {
	pc = 0x83169708; continue 'dispatch;
	}
	// 83169724: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83169728: C01D0020  lfs f0, 0x20(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8316972C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83169730: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83169734: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83169738: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8316973C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83169740: EFE00372  fmuls f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83169744: 41820070  beq 0x831697b4
	if ctx.cr[0].eq {
	pc = 0x831697B4; continue 'dispatch;
	}
	// 83169748: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8316974C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83169750: 4BFFF591  bl 0x83168ce0
	ctx.lr = 0x83169754;
	sub_83168CE0(ctx, base);
	// 83169754: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83169758: 4082FFF4  bne 0x8316974c
	if !ctx.cr[0].eq {
	pc = 0x8316974C; continue 'dispatch;
	}
	// 8316975C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169760: 4BFFF021  bl 0x83168780
	ctx.lr = 0x83169764;
	sub_83168780(ctx, base);
	// 83169764: 817E0268  lwz r11, 0x268(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(616 as u32) ) } as u64;
	// 83169768: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8316976C: C01F0050  lfs f0, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83169770: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83169774: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83169778: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316977C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83169780: 409A0018  bne cr6, 0x83169798
	if !ctx.cr[6].eq {
	pc = 0x83169798; continue 'dispatch;
	}
	// 83169784: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 83169788: 40990010  ble cr6, 0x83169798
	if !ctx.cr[6].gt {
	pc = 0x83169798; continue 'dispatch;
	}
	// 8316978C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169790: 409A0008  bne cr6, 0x83169798
	if !ctx.cr[6].eq {
	pc = 0x83169798; continue 'dispatch;
	}
	// 83169794: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83169798: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8316979C: 4098000C  bge cr6, 0x831697a8
	if !ctx.cr[6].lt {
	pc = 0x831697A8; continue 'dispatch;
	}
	// 831697A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831697A4: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 831697A8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831697AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831697B0: 409AFF98  bne cr6, 0x83169748
	if !ctx.cr[6].eq {
	pc = 0x83169748; continue 'dispatch;
	}
	// 831697B4: 83FD001C  lwz r31, 0x1c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 831697B8: 4800001C  b 0x831697d4
	pc = 0x831697D4; continue 'dispatch;
	// 831697BC: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831697C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831697C4: 409A000C  bne cr6, 0x831697d0
	if !ctx.cr[6].eq {
	pc = 0x831697D0; continue 'dispatch;
	}
	// 831697C8: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831697CC: 4BFFFB3D  bl 0x83169308
	ctx.lr = 0x831697D0;
	sub_83169308(ctx, base);
	// 831697D0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831697D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831697D8: 409AFFE4  bne cr6, 0x831697bc
	if !ctx.cr[6].eq {
	pc = 0x831697BC; continue 'dispatch;
	}
	// 831697DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831697E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831697E4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831697E8: 4803E9D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831697F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831697F0 size=88
    let mut pc: u32 = 0x831697F0;
    'dispatch: loop {
        match pc {
            0x831697F0 => {
    //   block [0x831697F0..0x83169848)
	// 831697F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831697F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831697F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831697FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169800: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 83169804: 817F8474  lwz r11, -0x7b8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-31628 as u32) ) } as u64;
	// 83169808: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316980C: 917F8474  stw r11, -0x7b8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-31628 as u32), ctx.r[11].u32 ) };
	// 83169810: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83169814: 419A000C  beq cr6, 0x83169820
	if ctx.cr[6].eq {
	pc = 0x83169820; continue 'dispatch;
	}
	// 83169818: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316981C: 48000018  b 0x83169834
	pc = 0x83169834; continue 'dispatch;
	// 83169820: 4BFFF951  bl 0x83169170
	ctx.lr = 0x83169824;
	sub_83169170(ctx, base);
	// 83169824: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83169828: 4082000C  bne 0x83169834
	if !ctx.cr[0].eq {
	pc = 0x83169834; continue 'dispatch;
	}
	// 8316982C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83169830: 917F8474  stw r11, -0x7b8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-31628 as u32), ctx.r[11].u32 ) };
	// 83169834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83169838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316983C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169848 size=76
    let mut pc: u32 = 0x83169848;
    'dispatch: loop {
        match pc {
            0x83169848 => {
    //   block [0x83169848..0x83169894)
	// 83169848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316984C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316985C: 4BFFF98D  bl 0x831691e8
	ctx.lr = 0x83169860;
	sub_831691E8(ctx, base);
	// 83169860: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83169864: 419A001C  beq cr6, 0x83169880
	if ctx.cr[6].eq {
	pc = 0x83169880; continue 'dispatch;
	}
	// 83169868: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8316986C: 388003A8  li r4, 0x3a8
	ctx.r[4].s64 = 936;
	// 83169870: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 83169874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169878: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316987C: 4BFF6405  bl 0x8315fc80
	ctx.lr = 0x83169880;
	sub_8315FC80(ctx, base);
	// 83169880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83169884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316988C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169898 size=244
    let mut pc: u32 = 0x83169898;
    'dispatch: loop {
        match pc {
            0x83169898 => {
    //   block [0x83169898..0x8316998C)
	// 83169898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316989C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831698A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831698A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831698A8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831698AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831698B0: 806B8470  lwz r3, -0x7b90(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31632 as u32) ) } as u64;
	// 831698B4: 4BFFF6CD  bl 0x83168f80
	ctx.lr = 0x831698B8;
	sub_83168F80(ctx, base);
	// 831698B8: 907F0260  stw r3, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[3].u32 ) };
	// 831698BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831698C0: 40820024  bne 0x831698e4
	if !ctx.cr[0].eq {
	pc = 0x831698E4; continue 'dispatch;
	}
	// 831698C4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831698C8: 388B7BB4  addi r4, r11, 0x7bb4
	ctx.r[4].s64 = ctx.r[11].s64 + 31668;
	// 831698CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831698D0: 4BFF6249  bl 0x8315fb18
	ctx.lr = 0x831698D4;
	sub_8315FB18(ctx, base);
	// 831698D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831698D8: 4BFFF911  bl 0x831691e8
	ctx.lr = 0x831698DC;
	sub_831691E8(ctx, base);
	// 831698DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831698E0: 48000098  b 0x83169978
	pc = 0x83169978; continue 'dispatch;
	// 831698E4: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 831698E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831698EC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831698F0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 831698F4: 815F039C  lwz r10, 0x39c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(924 as u32) ) } as u64;
	// 831698F8: 38AB7BA4  addi r5, r11, 0x7ba4
	ctx.r[5].s64 = ctx.r[11].s64 + 31652;
	// 831698FC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83169900: 55442036  slwi r4, r10, 4
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83169904: 4BFF5DFD  bl 0x8315f700
	ctx.lr = 0x83169908;
	sub_8315F700(ctx, base);
	// 83169908: 907F0394  stw r3, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[3].u32 ) };
	// 8316990C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83169910: 40820018  bne 0x83169928
	if !ctx.cr[0].eq {
	pc = 0x83169928; continue 'dispatch;
	}
	// 83169914: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169918: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8316991C: 388B7B98  addi r4, r11, 0x7b98
	ctx.r[4].s64 = ctx.r[11].s64 + 31640;
	// 83169920: 4BFF6221  bl 0x8315fb40
	ctx.lr = 0x83169924;
	sub_8315FB40(ctx, base);
	// 83169924: 4BFFFFB0  b 0x831698d4
	pc = 0x831698D4; continue 'dispatch;
	// 83169928: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316992C: 814B8468  lwz r10, -0x7b98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31640 as u32) ) } as u64;
	// 83169930: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83169934: 419A0028  beq cr6, 0x8316995c
	if ctx.cr[6].eq {
	pc = 0x8316995C; continue 'dispatch;
	}
	// 83169938: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316993C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83169940: 816B8468  lwz r11, -0x7b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31640 as u32) ) } as u64;
	// 83169944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83169948: 808A846C  lwz r4, -0x7b94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31636 as u32) ) } as u64;
	// 8316994C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 83169950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169954: 4E800421  bctrl
	ctx.lr = 0x83169958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83169958: 907F026C  stw r3, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[3].u32 ) };
	// 8316995C: 817F026C  lwz r11, 0x26c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 83169960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83169964: 409A0010  bne cr6, 0x83169974
	if !ctx.cr[6].eq {
	pc = 0x83169974; continue 'dispatch;
	}
	// 83169968: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316996C: 388B7B6C  addi r4, r11, 0x7b6c
	ctx.r[4].s64 = ctx.r[11].s64 + 31596;
	// 83169970: 4BFFFF5C  b 0x831698cc
	pc = 0x831698CC; continue 'dispatch;
	// 83169974: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83169978: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316997C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169990 size=72
    let mut pc: u32 = 0x83169990;
    'dispatch: loop {
        match pc {
            0x83169990 => {
    //   block [0x83169990..0x831699D8)
	// 83169990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316999C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831699A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831699A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831699A8: 917F0390  stw r11, 0x390(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(912 as u32), ctx.r[11].u32 ) };
	// 831699AC: 4BFFF8BD  bl 0x83169268
	ctx.lr = 0x831699B0;
	sub_83169268(ctx, base);
	// 831699B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831699B4: 41820010  beq 0x831699c4
	if ctx.cr[0].eq {
	pc = 0x831699C4; continue 'dispatch;
	}
	// 831699B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831699BC: 4BFFFE8D  bl 0x83169848
	ctx.lr = 0x831699C0;
	sub_83169848(ctx, base);
	// 831699C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831699C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831699C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831699CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831699D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831699D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831699D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831699D8 size=20
    let mut pc: u32 = 0x831699D8;
    'dispatch: loop {
        match pc {
            0x831699D8 => {
    //   block [0x831699D8..0x831699EC)
	// 831699D8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831699DC: 806B8470  lwz r3, -0x7b90(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31632 as u32) ) } as u64;
	// 831699E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831699E4: 409A0008  bne cr6, 0x831699ec
	if !ctx.cr[6].eq {
		sub_831699EC(ctx, base);
		return;
	}
	// 831699E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831699EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831699EC size=4
    let mut pc: u32 = 0x831699EC;
    'dispatch: loop {
        match pc {
            0x831699EC => {
    //   block [0x831699EC..0x831699F0)
	// 831699EC: 4BFFFCEC  b 0x831696d8
	sub_831696D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831699F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831699F0 size=248
    let mut pc: u32 = 0x831699F0;
    'dispatch: loop {
        match pc {
            0x831699F0 => {
    //   block [0x831699F0..0x83169AE8)
	// 831699F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831699F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831699F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831699FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169A04: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83169A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169A0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83169A10: 816B8470  lwz r11, -0x7b90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31632 as u32) ) } as u64;
	// 83169A14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83169A18: 409A0018  bne cr6, 0x83169a30
	if !ctx.cr[6].eq {
	pc = 0x83169A30; continue 'dispatch;
	}
	// 83169A1C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169A20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169A24: 388B7C0C  addi r4, r11, 0x7c0c
	ctx.r[4].s64 = ctx.r[11].s64 + 31756;
	// 83169A28: 4BFF60F1  bl 0x8315fb18
	ctx.lr = 0x83169A2C;
	sub_8315FB18(ctx, base);
	// 83169A2C: 480000A0  b 0x83169acc
	pc = 0x83169ACC; continue 'dispatch;
	// 83169A30: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83169A34: 409A001C  bne cr6, 0x83169a50
	if !ctx.cr[6].eq {
	pc = 0x83169A50; continue 'dispatch;
	}
	// 83169A38: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169A3C: 38A0FFFE  li r5, -2
	ctx.r[5].s64 = -2;
	// 83169A40: 388B7C00  addi r4, r11, 0x7c00
	ctx.r[4].s64 = ctx.r[11].s64 + 31744;
	// 83169A44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169A48: 4BFF60F9  bl 0x8315fb40
	ctx.lr = 0x83169A4C;
	sub_8315FB40(ctx, base);
	// 83169A4C: 48000080  b 0x83169acc
	pc = 0x83169ACC; continue 'dispatch;
	// 83169A50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169A54: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83169A58: 38CB7BF0  addi r6, r11, 0x7bf0
	ctx.r[6].s64 = ctx.r[11].s64 + 31728;
	// 83169A5C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83169A60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83169A64: 386003A8  li r3, 0x3a8
	ctx.r[3].s64 = 936;
	// 83169A68: 4BFF61B1  bl 0x8315fc18
	ctx.lr = 0x83169A6C;
	sub_8315FC18(ctx, base);
	// 83169A6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83169A70: 41820018  beq 0x83169a88
	if ctx.cr[0].eq {
	pc = 0x83169A88; continue 'dispatch;
	}
	// 83169A74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83169A78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83169A7C: 4BFFE7C5  bl 0x83168240
	ctx.lr = 0x83169A80;
	sub_83168240(ctx, base);
	// 83169A80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169A84: 48000008  b 0x83169a8c
	pc = 0x83169A8C; continue 'dispatch;
	// 83169A88: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83169A8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83169A90: 409A0014  bne cr6, 0x83169aa4
	if !ctx.cr[6].eq {
	pc = 0x83169AA4; continue 'dispatch;
	}
	// 83169A94: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169A98: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83169A9C: 388B7BE4  addi r4, r11, 0x7be4
	ctx.r[4].s64 = ctx.r[11].s64 + 31716;
	// 83169AA0: 4BFFFFA4  b 0x83169a44
	pc = 0x83169A44; continue 'dispatch;
	// 83169AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169AA8: 4BFFFDF1  bl 0x83169898
	ctx.lr = 0x83169AAC;
	sub_83169898(ctx, base);
	// 83169AAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83169AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169AB4: 4082001C  bne 0x83169ad0
	if !ctx.cr[0].eq {
	pc = 0x83169AD0; continue 'dispatch;
	}
	// 83169AB8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83169ABC: 388003A8  li r4, 0x3a8
	ctx.r[4].s64 = 936;
	// 83169AC0: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 83169AC4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83169AC8: 4BFF61B9  bl 0x8315fc80
	ctx.lr = 0x83169ACC;
	sub_8315FC80(ctx, base);
	// 83169ACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83169AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169ADC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83169AE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169AE8 size=84
    let mut pc: u32 = 0x83169AE8;
    'dispatch: loop {
        match pc {
            0x83169AE8 => {
    //   block [0x83169AE8..0x83169B3C)
	// 83169AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169AF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83169AF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169AF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169B00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83169B04: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83169B08: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 83169B0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83169B10: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83169B14: 4803E6CD  bl 0x831a81e0
	ctx.lr = 0x83169B18;
	sub_831A81E0(ctx, base);
	// 83169B18: 93DF0108  stw r30, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[30].u32 ) };
	// 83169B1C: 93DF010C  stw r30, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[30].u32 ) };
	// 83169B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83169B24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83169B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169B30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83169B34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169B40 size=304
    let mut pc: u32 = 0x83169B40;
    'dispatch: loop {
        match pc {
            0x83169B40 => {
    //   block [0x83169B40..0x83169C70)
	// 83169B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169B44: 4803E621  bl 0x831a8164
	ctx.lr = 0x83169B48;
	sub_831A8130(ctx, base);
	// 83169B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169B4C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83169B50: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83169B54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83169B58: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83169B5C: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 83169B60: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83169B64: 409A0020  bne cr6, 0x83169b84
	if !ctx.cr[6].eq {
	pc = 0x83169B84; continue 'dispatch;
	}
	// 83169B68: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 83169B6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83169B70: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83169B74: 4803E66D  bl 0x831a81e0
	ctx.lr = 0x83169B78;
	sub_831A81E0(ctx, base);
	// 83169B78: 93BE0108  stw r29, 0x108(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(264 as u32), ctx.r[29].u32 ) };
	// 83169B7C: 93BE010C  stw r29, 0x10c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(268 as u32), ctx.r[29].u32 ) };
	// 83169B80: 480000E8  b 0x83169c68
	pc = 0x83169C68; continue 'dispatch;
	// 83169B84: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83169B88: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169B8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169B90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83169B94: 409AFFF4  bne cr6, 0x83169b88
	if !ctx.cr[6].eq {
	pc = 0x83169B88; continue 'dispatch;
	}
	// 83169B98: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 83169B9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83169BA0: 557F003F  rotlwi. r31, r11, 0
	ctx.r[31].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83169BA4: 4081FFC4  ble 0x83169b68
	if !ctx.cr[0].gt {
	pc = 0x83169B68; continue 'dispatch;
	}
	// 83169BA8: 7D7F2214  add r11, r31, r4
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83169BAC: 896BFFFF  lbz r11, -1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 83169BB0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83169BB4: 2F0B005C  cmpwi cr6, r11, 0x5c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 92, &mut ctx.xer);
	// 83169BB8: 419A0014  beq cr6, 0x83169bcc
	if ctx.cr[6].eq {
	pc = 0x83169BCC; continue 'dispatch;
	}
	// 83169BBC: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 83169BC0: 419A000C  beq cr6, 0x83169bcc
	if ctx.cr[6].eq {
	pc = 0x83169BCC; continue 'dispatch;
	}
	// 83169BC4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83169BC8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83169BCC: 2F1F0104  cmpwi cr6, r31, 0x104
	ctx.cr[6].compare_i32(ctx.r[31].s32, 260, &mut ctx.xer);
	// 83169BD0: 40990020  ble cr6, 0x83169bf0
	if !ctx.cr[6].gt {
	pc = 0x83169BF0; continue 'dispatch;
	}
	// 83169BD4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169BD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169BDC: 388B7C38  addi r4, r11, 0x7c38
	ctx.r[4].s64 = ctx.r[11].s64 + 31800;
	// 83169BE0: 4BFF5F39  bl 0x8315fb18
	ctx.lr = 0x83169BE4;
	sub_8315FB18(ctx, base);
	// 83169BE4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83169BE8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83169BEC: 4800007C  b 0x83169c68
	pc = 0x83169C68; continue 'dispatch;
	// 83169BF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83169BF4: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 83169BF8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83169BFC: 388B5C58  addi r4, r11, 0x5c58
	ctx.r[4].s64 = ctx.r[11].s64 + 23640;
	// 83169C00: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83169C04: 4803EED5  bl 0x831a8ad8
	ctx.lr = 0x83169C08;
	sub_831A8AD8(ctx, base);
	// 83169C08: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83169C0C: 419A0014  beq cr6, 0x83169c20
	if ctx.cr[6].eq {
	pc = 0x83169C20; continue 'dispatch;
	}
	// 83169C10: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83169C14: 3940005C  li r10, 0x5c
	ctx.r[10].s64 = 92;
	// 83169C18: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 83169C1C: 9BAB0004  stb r29, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 83169C20: 93FE0108  stw r31, 0x108(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(264 as u32), ctx.r[31].u32 ) };
	// 83169C24: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83169C28: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83169C2C: 93BE010C  stw r29, 0x10c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(268 as u32), ctx.r[29].u32 ) };
	// 83169C30: 40990038  ble cr6, 0x83169c68
	if !ctx.cr[6].gt {
	pc = 0x83169C68; continue 'dispatch;
	}
	// 83169C34: 7D5C58AE  lbzx r10, r28, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83169C38: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83169C3C: 2F0A003A  cmpwi cr6, r10, 0x3a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 58, &mut ctx.xer);
	// 83169C40: 419A0024  beq cr6, 0x83169c64
	if ctx.cr[6].eq {
	pc = 0x83169C64; continue 'dispatch;
	}
	// 83169C44: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 83169C48: 419A0020  beq cr6, 0x83169c68
	if ctx.cr[6].eq {
	pc = 0x83169C68; continue 'dispatch;
	}
	// 83169C4C: 2F0A002F  cmpwi cr6, r10, 0x2f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 47, &mut ctx.xer);
	// 83169C50: 419A0018  beq cr6, 0x83169c68
	if ctx.cr[6].eq {
	pc = 0x83169C68; continue 'dispatch;
	}
	// 83169C54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169C58: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83169C5C: 4198FFD8  blt cr6, 0x83169c34
	if ctx.cr[6].lt {
	pc = 0x83169C34; continue 'dispatch;
	}
	// 83169C60: 48000008  b 0x83169c68
	pc = 0x83169C68; continue 'dispatch;
	// 83169C64: 917E010C  stw r11, 0x10c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(268 as u32), ctx.r[11].u32 ) };
	// 83169C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83169C6C: 4803E548  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83169C70 size=16
    let mut pc: u32 = 0x83169C70;
    'dispatch: loop {
        match pc {
            0x83169C70 => {
    //   block [0x83169C70..0x83169C80)
	// 83169C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83169C74: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83169C78: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83169C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83169C80 size=16
    let mut pc: u32 = 0x83169C80;
    'dispatch: loop {
        match pc {
            0x83169C80 => {
    //   block [0x83169C80..0x83169C90)
	// 83169C80: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83169C84: 409A000C  bne cr6, 0x83169c90
	if !ctx.cr[6].eq {
		sub_83169C90(ctx, base);
		return;
	}
	// 83169C88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83169C90 size=124
    let mut pc: u32 = 0x83169C90;
    'dispatch: loop {
        match pc {
            0x83169C90 => {
    //   block [0x83169C90..0x83169D0C)
	// 83169C90: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169C94: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83169C98: 2F0B005C  cmpwi cr6, r11, 0x5c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 92, &mut ctx.xer);
	// 83169C9C: 419A0080  beq cr6, 0x83169d1c
	if ctx.cr[6].eq {
		sub_83169D1C(ctx, base);
		return;
	}
	// 83169CA0: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 83169CA4: 419A0078  beq cr6, 0x83169d1c
	if ctx.cr[6].eq {
		sub_83169D1C(ctx, base);
		return;
	}
	// 83169CA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83169CAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83169CB0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 83169CB4: 4099FFD4  ble cr6, 0x83169c88
	if !ctx.cr[6].gt {
		sub_83169C80(ctx, base);
		return;
	}
	// 83169CB8: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 83169CBC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83169CC0: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 83169CC4: 419A000C  beq cr6, 0x83169cd0
	if ctx.cr[6].eq {
	pc = 0x83169CD0; continue 'dispatch;
	}
	// 83169CC8: 2F0A002F  cmpwi cr6, r10, 0x2f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 47, &mut ctx.xer);
	// 83169CCC: 409A0008  bne cr6, 0x83169cd4
	if !ctx.cr[6].eq {
	pc = 0x83169CD4; continue 'dispatch;
	}
	// 83169CD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83169CD4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83169CD8: 409A000C  bne cr6, 0x83169ce4
	if !ctx.cr[6].eq {
	pc = 0x83169CE4; continue 'dispatch;
	}
	// 83169CDC: 2F0A003A  cmpwi cr6, r10, 0x3a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 58, &mut ctx.xer);
	// 83169CE0: 419A002C  beq cr6, 0x83169d0c
	if ctx.cr[6].eq {
		sub_83169D0C(ctx, base);
		return;
	}
	// 83169CE4: 7D0B2214  add r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83169CE8: 8908FFFF  lbz r8, -1(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(-1 as u32) ) } as u64;
	// 83169CEC: 2B08002E  cmplwi cr6, r8, 0x2e
	ctx.cr[6].compare_u32(ctx.r[8].u32, 46 as u32, &mut ctx.xer);
	// 83169CF0: 409A000C  bne cr6, 0x83169cfc
	if !ctx.cr[6].eq {
	pc = 0x83169CFC; continue 'dispatch;
	}
	// 83169CF4: 2F0A002F  cmpwi cr6, r10, 0x2f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 47, &mut ctx.xer);
	// 83169CF8: 419A001C  beq cr6, 0x83169d14
	if ctx.cr[6].eq {
		sub_83169D14(ctx, base);
		return;
	}
	// 83169CFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169D00: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83169D04: 4198FFB4  blt cr6, 0x83169cb8
	if ctx.cr[6].lt {
	pc = 0x83169CB8; continue 'dispatch;
	}
	// 83169D08: 4BFFFF80  b 0x83169c88
	sub_83169C80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169D0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83169D0C size=8
    let mut pc: u32 = 0x83169D0C;
    'dispatch: loop {
        match pc {
            0x83169D0C => {
    //   block [0x83169D0C..0x83169D14)
	// 83169D0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83169D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83169D14 size=8
    let mut pc: u32 = 0x83169D14;
    'dispatch: loop {
        match pc {
            0x83169D14 => {
    //   block [0x83169D14..0x83169D1C)
	// 83169D14: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83169D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169D1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83169D1C size=8
    let mut pc: u32 = 0x83169D1C;
    'dispatch: loop {
        match pc {
            0x83169D1C => {
    //   block [0x83169D1C..0x83169D24)
	// 83169D1C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83169D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169D28 size=104
    let mut pc: u32 = 0x83169D28;
    'dispatch: loop {
        match pc {
            0x83169D28 => {
    //   block [0x83169D28..0x83169D90)
	// 83169D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169D38: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83169D3C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 83169D40: 7F053800  cmpw cr6, r5, r7
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83169D44: 40990020  ble cr6, 0x83169d64
	if !ctx.cr[6].gt {
	pc = 0x83169D64; continue 'dispatch;
	}
	// 83169D48: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169D4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169D50: 388B7C5C  addi r4, r11, 0x7c5c
	ctx.r[4].s64 = ctx.r[11].s64 + 31836;
	// 83169D54: 4BFF5DC5  bl 0x8315fb18
	ctx.lr = 0x83169D58;
	sub_8315FB18(ctx, base);
	// 83169D58: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83169D5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83169D60: 4800001C  b 0x83169d7c
	pc = 0x83169D7C; continue 'dispatch;
	// 83169D64: 7D4B3050  subf r10, r11, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 83169D68: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169D6C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83169D70: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 83169D74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169D78: 4082FFF0  bne 0x83169d68
	if !ctx.cr[0].eq {
	pc = 0x83169D68; continue 'dispatch;
	}
	// 83169D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83169D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169D90 size=168
    let mut pc: u32 = 0x83169D90;
    'dispatch: loop {
        match pc {
            0x83169D90 => {
    //   block [0x83169D90..0x83169E38)
	// 83169D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169D94: 4803E3D5  bl 0x831a8168
	ctx.lr = 0x83169D98;
	sub_831A8130(ctx, base);
	// 83169D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169DA0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83169DA4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83169DA8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83169DAC: 817F010C  lwz r11, 0x10c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 83169DB0: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83169DB4: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83169DB8: 40990020  ble cr6, 0x83169dd8
	if !ctx.cr[6].gt {
	pc = 0x83169DD8; continue 'dispatch;
	}
	// 83169DBC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169DC4: 388B7C98  addi r4, r11, 0x7c98
	ctx.r[4].s64 = ctx.r[11].s64 + 31896;
	// 83169DC8: 4BFF5D51  bl 0x8315fb18
	ctx.lr = 0x83169DCC;
	sub_8315FB18(ctx, base);
	// 83169DCC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83169DD0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83169DD4: 4800005C  b 0x83169e30
	pc = 0x83169E30; continue 'dispatch;
	// 83169DD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169DDC: 40990024  ble cr6, 0x83169e00
	if !ctx.cr[6].gt {
	pc = 0x83169E00; continue 'dispatch;
	}
	// 83169DE0: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 83169DE4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83169DE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83169DEC: 48042705  bl 0x831ac4f0
	ctx.lr = 0x83169DF0;
	sub_831AC4F0(ctx, base);
	// 83169DF0: 817F010C  lwz r11, 0x10c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 83169DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83169DF8: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83169DFC: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 83169E00: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 83169E04: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169E08: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83169E0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83169E10: 409AFFF4  bne cr6, 0x83169e04
	if !ctx.cr[6].eq {
	pc = 0x83169E04; continue 'dispatch;
	}
	// 83169E14: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 83169E18: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169E1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83169E20: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83169E24: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83169E28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169E2C: 409AFFEC  bne cr6, 0x83169e18
	if !ctx.cr[6].eq {
	pc = 0x83169E18; continue 'dispatch;
	}
	// 83169E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83169E34: 4803E384  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169E38 size=160
    let mut pc: u32 = 0x83169E38;
    'dispatch: loop {
        match pc {
            0x83169E38 => {
    //   block [0x83169E38..0x83169ED8)
	// 83169E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169E48: 81630108  lwz r11, 0x108(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) } as u64;
	// 83169E4C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 83169E50: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 83169E54: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83169E58: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83169E5C: 40990020  ble cr6, 0x83169e7c
	if !ctx.cr[6].gt {
	pc = 0x83169E7C; continue 'dispatch;
	}
	// 83169E60: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169E64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169E68: 388B7CD4  addi r4, r11, 0x7cd4
	ctx.r[4].s64 = ctx.r[11].s64 + 31956;
	// 83169E6C: 4BFF5CAD  bl 0x8315fb18
	ctx.lr = 0x83169E70;
	sub_8315FB18(ctx, base);
	// 83169E70: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83169E74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83169E78: 4800004C  b 0x83169ec4
	pc = 0x83169EC4; continue 'dispatch;
	// 83169E7C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 83169E80: 7D2B3050  subf r9, r11, r6
	ctx.r[9].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 83169E84: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169E88: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83169E8C: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 83169E90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169E94: 4082FFF0  bne 0x83169e84
	if !ctx.cr[0].eq {
	pc = 0x83169E84; continue 'dispatch;
	}
	// 83169E98: 89660000  lbz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169E9C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 83169EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83169EA4: 409AFFF4  bne cr6, 0x83169e98
	if !ctx.cr[6].eq {
	pc = 0x83169E98; continue 'dispatch;
	}
	// 83169EA8: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 83169EAC: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169EB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83169EB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83169EB8: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83169EBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169EC0: 409AFFEC  bne cr6, 0x83169eac
	if !ctx.cr[6].eq {
	pc = 0x83169EAC; continue 'dispatch;
	}
	// 83169EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83169EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83169ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83169ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83169ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169ED8 size=224
    let mut pc: u32 = 0x83169ED8;
    'dispatch: loop {
        match pc {
            0x83169ED8 => {
    //   block [0x83169ED8..0x83169FB8)
	// 83169ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169EDC: 4803E291  bl 0x831a816c
	ctx.lr = 0x83169EE0;
	sub_831A8130(ctx, base);
	// 83169EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169EE4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83169EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83169EEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83169EF0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83169EF4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83169EF8: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83169EFC: 419A009C  beq cr6, 0x83169f98
	if ctx.cr[6].eq {
	pc = 0x83169F98; continue 'dispatch;
	}
	// 83169F00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83169F04: 419A0094  beq cr6, 0x83169f98
	if ctx.cr[6].eq {
	pc = 0x83169F98; continue 'dispatch;
	}
	// 83169F08: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83169F0C: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83169F10: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169F14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83169F18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83169F1C: 409AFFF4  bne cr6, 0x83169f10
	if !ctx.cr[6].eq {
	pc = 0x83169F10; continue 'dispatch;
	}
	// 83169F20: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 83169F24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83169F28: 5565003F  rotlwi. r5, r11, 0
	ctx.r[5].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83169F2C: 4080001C  bge 0x83169f48
	if !ctx.cr[0].lt {
	pc = 0x83169F48; continue 'dispatch;
	}
	// 83169F30: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169F34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169F38: 388B7D34  addi r4, r11, 0x7d34
	ctx.r[4].s64 = ctx.r[11].s64 + 32052;
	// 83169F3C: 4BFF5BDD  bl 0x8315fb18
	ctx.lr = 0x83169F40;
	sub_8315FB18(ctx, base);
	// 83169F40: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83169F44: 48000068  b 0x83169fac
	pc = 0x83169FAC; continue 'dispatch;
	// 83169F48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83169F4C: 4BFFFD35  bl 0x83169c80
	ctx.lr = 0x83169F50;
	sub_83169C80(ctx, base);
	// 83169F50: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 83169F54: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83169F58: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 83169F5C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83169F60: 419A0020  beq cr6, 0x83169f80
	if ctx.cr[6].eq {
	pc = 0x83169F80; continue 'dispatch;
	}
	// 83169F64: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83169F68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83169F6C: 419A000C  beq cr6, 0x83169f78
	if ctx.cr[6].eq {
	pc = 0x83169F78; continue 'dispatch;
	}
	// 83169F70: 4BFFFEC9  bl 0x83169e38
	ctx.lr = 0x83169F74;
	sub_83169E38(ctx, base);
	// 83169F74: 48000014  b 0x83169f88
	pc = 0x83169F88; continue 'dispatch;
	// 83169F78: 4BFFFE19  bl 0x83169d90
	ctx.lr = 0x83169F7C;
	sub_83169D90(ctx, base);
	// 83169F7C: 4800000C  b 0x83169f88
	pc = 0x83169F88; continue 'dispatch;
	// 83169F80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83169F84: 4BFFFDA5  bl 0x83169d28
	ctx.lr = 0x83169F88;
	sub_83169D28(ctx, base);
	// 83169F88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83169F8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169F90: 419A0020  beq cr6, 0x83169fb0
	if ctx.cr[6].eq {
	pc = 0x83169FB0; continue 'dispatch;
	}
	// 83169F94: 48000018  b 0x83169fac
	pc = 0x83169FAC; continue 'dispatch;
	// 83169F98: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169F9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169FA0: 388B7D10  addi r4, r11, 0x7d10
	ctx.r[4].s64 = ctx.r[11].s64 + 32016;
	// 83169FA4: 4BFF5B75  bl 0x8315fb18
	ctx.lr = 0x83169FA8;
	sub_8315FB18(ctx, base);
	// 83169FA8: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 83169FAC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83169FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83169FB4: 4803E208  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83169FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83169FB8 size=112
    let mut pc: u32 = 0x83169FB8;
    'dispatch: loop {
        match pc {
            0x83169FB8 => {
    //   block [0x83169FB8..0x8316A028)
	// 83169FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83169FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83169FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83169FC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83169FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83169FCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83169FD0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83169FD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83169FD8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83169FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83169FE0: 4E800421  bctrl
	ctx.lr = 0x83169FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83169FE4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83169FE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83169FEC: 419A0028  beq cr6, 0x8316a014
	if ctx.cr[6].eq {
	pc = 0x8316A014; continue 'dispatch;
	}
	// 83169FF0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83169FF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83169FF8: 388B7D5C  addi r4, r11, 0x7d5c
	ctx.r[4].s64 = ctx.r[11].s64 + 32092;
	// 83169FFC: 4BFF5B1D  bl 0x8315fb18
	ctx.lr = 0x8316A000;
	sub_8315FB18(ctx, base);
	// 8316A000: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316A004: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A008: 409A000C  bne cr6, 0x8316a014
	if !ctx.cr[6].eq {
	pc = 0x8316A014; continue 'dispatch;
	}
	// 8316A00C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A010: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316A014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A028 size=112
    let mut pc: u32 = 0x8316A028;
    'dispatch: loop {
        match pc {
            0x8316A028 => {
    //   block [0x8316A028..0x8316A098)
	// 8316A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A034: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A03C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316A040: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316A044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A048: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316A04C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A050: 4E800421  bctrl
	ctx.lr = 0x8316A054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A054: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A058: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A05C: 419A0028  beq cr6, 0x8316a084
	if ctx.cr[6].eq {
	pc = 0x8316A084; continue 'dispatch;
	}
	// 8316A060: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A064: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A068: 388B7D8C  addi r4, r11, 0x7d8c
	ctx.r[4].s64 = ctx.r[11].s64 + 32140;
	// 8316A06C: 4BFF5AAD  bl 0x8315fb18
	ctx.lr = 0x8316A070;
	sub_8315FB18(ctx, base);
	// 8316A070: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316A074: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A078: 409A000C  bne cr6, 0x8316a084
	if !ctx.cr[6].eq {
	pc = 0x8316A084; continue 'dispatch;
	}
	// 8316A07C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A080: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316A084: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A098 size=112
    let mut pc: u32 = 0x8316A098;
    'dispatch: loop {
        match pc {
            0x8316A098 => {
    //   block [0x8316A098..0x8316A108)
	// 8316A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316A0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A0B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316A0B4: 93C40000  stw r30, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316A0B8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316A0BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316A0C0: 419A0020  beq cr6, 0x8316a0e0
	if ctx.cr[6].eq {
	pc = 0x8316A0E0; continue 'dispatch;
	}
	// 8316A0C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A0C8: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A0CC: 388A81F0  addi r4, r10, -0x7e10
	ctx.r[4].s64 = ctx.r[10].s64 + -32272;
	// 8316A0D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A0D8: 4E800421  bctrl
	ctx.lr = 0x8316A0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A0DC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8316A0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A0E4: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316A0E8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316A0EC: 4BF5D9F5  bl 0x830c7ae0
	ctx.lr = 0x8316A0F0;
	sub_830C7AE0(ctx, base);
	// 8316A0F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A0FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316A100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A108 size=64
    let mut pc: u32 = 0x8316A108;
    'dispatch: loop {
        match pc {
            0x8316A108 => {
    //   block [0x8316A108..0x8316A148)
	// 8316A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A10C: 4803E061  bl 0x831a816c
	ctx.lr = 0x8316A110;
	sub_831A8130(ctx, base);
	// 8316A110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A118: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316A11C: 4BFFFE9D  bl 0x83169fb8
	ctx.lr = 0x8316A120;
	sub_83169FB8(ctx, base);
	// 8316A120: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316A124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316A128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A12C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316A130: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A134: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316A138: 4BFFFEF1  bl 0x8316a028
	ctx.lr = 0x8316A13C;
	sub_8316A028(ctx, base);
	// 8316A13C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316A140: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A144: 4803E078  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A148 size=264
    let mut pc: u32 = 0x8316A148;
    'dispatch: loop {
        match pc {
            0x8316A148 => {
    //   block [0x8316A148..0x8316A250)
	// 8316A148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A14C: 4803E021  bl 0x831a816c
	ctx.lr = 0x8316A150;
	sub_831A8130(ctx, base);
	// 8316A150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A154: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316A158: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316A15C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316A160: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316A164: 409A0020  bne cr6, 0x8316a184
	if !ctx.cr[6].eq {
	pc = 0x8316A184; continue 'dispatch;
	}
	// 8316A168: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A16C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A170: 388B7E18  addi r4, r11, 0x7e18
	ctx.r[4].s64 = ctx.r[11].s64 + 32280;
	// 8316A174: 4BFF59A5  bl 0x8315fb18
	ctx.lr = 0x8316A178;
	sub_8315FB18(ctx, base);
	// 8316A178: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8316A17C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316A180: 480000C4  b 0x8316a244
	pc = 0x8316A244; continue 'dispatch;
	// 8316A184: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8316A188: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8316A18C: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 8316A190: 4BFF5B79  bl 0x8315fd08
	ctx.lr = 0x8316A194;
	sub_8315FD08(ctx, base);
	// 8316A194: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8316A198: 41820030  beq 0x8316a1c8
	if ctx.cr[0].eq {
	pc = 0x8316A1C8; continue 'dispatch;
	}
	// 8316A19C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8316A1A0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316A1A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316A1A8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8316A1AC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8316A1B0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8316A1B4: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8316A1B8: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8316A1BC: 4803E025  bl 0x831a81e0
	ctx.lr = 0x8316A1C0;
	sub_831A81E0(ctx, base);
	// 8316A1C0: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8316A1C4: 48000008  b 0x8316a1cc
	pc = 0x8316A1CC; continue 'dispatch;
	// 8316A1C8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8316A1CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316A1D0: 409A001C  bne cr6, 0x8316a1ec
	if !ctx.cr[6].eq {
	pc = 0x8316A1EC; continue 'dispatch;
	}
	// 8316A1D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A1D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A1DC: 388B7DF0  addi r4, r11, 0x7df0
	ctx.r[4].s64 = ctx.r[11].s64 + 32240;
	// 8316A1E0: 4BFF5939  bl 0x8315fb18
	ctx.lr = 0x8316A1E4;
	sub_8315FB18(ctx, base);
	// 8316A1E4: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 8316A1E8: 4BFFFF94  b 0x8316a17c
	pc = 0x8316A17C; continue 'dispatch;
	// 8316A1EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316A1F0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8316A1F4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8316A1F8: 4BFF5B41  bl 0x8315fd38
	ctx.lr = 0x8316A1FC;
	sub_8315FD38(ctx, base);
	// 8316A1FC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8316A200: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316A204: 41820018  beq 0x8316a21c
	if ctx.cr[0].eq {
	pc = 0x8316A21C; continue 'dispatch;
	}
	// 8316A208: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A20C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A210: 409A000C  bne cr6, 0x8316a21c
	if !ctx.cr[6].eq {
	pc = 0x8316A21C; continue 'dispatch;
	}
	// 8316A214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A218: 48000030  b 0x8316a248
	pc = 0x8316A248; continue 'dispatch;
	// 8316A21C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A220: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A224: 388B7DBC  addi r4, r11, 0x7dbc
	ctx.r[4].s64 = ctx.r[11].s64 + 32188;
	// 8316A228: 4BFF58F1  bl 0x8315fb18
	ctx.lr = 0x8316A22C;
	sub_8315FB18(ctx, base);
	// 8316A22C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316A230: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A234: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316A238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A23C: 388A81F0  addi r4, r10, -0x7e10
	ctx.r[4].s64 = ctx.r[10].s64 + -32272;
	// 8316A240: 4BFFFE59  bl 0x8316a098
	ctx.lr = 0x8316A244;
	sub_8316A098(ctx, base);
	// 8316A244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316A24C: 4803DF70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A250 size=72
    let mut pc: u32 = 0x8316A250;
    'dispatch: loop {
        match pc {
            0x8316A250 => {
    //   block [0x8316A250..0x8316A298)
	// 8316A250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A254: 4803DF19  bl 0x831a816c
	ctx.lr = 0x8316A258;
	sub_831A8130(ctx, base);
	// 8316A258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A25C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A260: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316A264: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316A268: 4BFFFD51  bl 0x83169fb8
	ctx.lr = 0x8316A26C;
	sub_83169FB8(ctx, base);
	// 8316A26C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A270: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316A274: 4BFE11ED  bl 0x8314b460
	ctx.lr = 0x8316A278;
	sub_8314B460(ctx, base);
	// 8316A278: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316A27C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316A280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A284: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316A288: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316A28C: 4BFFFD9D  bl 0x8316a028
	ctx.lr = 0x8316A290;
	sub_8316A028(ctx, base);
	// 8316A290: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A294: 4803DF28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A298 size=72
    let mut pc: u32 = 0x8316A298;
    'dispatch: loop {
        match pc {
            0x8316A298 => {
    //   block [0x8316A298..0x8316A2E0)
	// 8316A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A29C: 4803DED1  bl 0x831a816c
	ctx.lr = 0x8316A2A0;
	sub_831A8130(ctx, base);
	// 8316A2A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A2A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A2A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316A2AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316A2B0: 4BFFFD09  bl 0x83169fb8
	ctx.lr = 0x8316A2B4;
	sub_83169FB8(ctx, base);
	// 8316A2B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A2B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316A2BC: 4BFD5ECD  bl 0x83140188
	ctx.lr = 0x8316A2C0;
	sub_83140188(ctx, base);
	// 8316A2C0: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316A2C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316A2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A2CC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316A2D0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316A2D4: 4BFFFD55  bl 0x8316a028
	ctx.lr = 0x8316A2D8;
	sub_8316A028(ctx, base);
	// 8316A2D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A2DC: 4803DEE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A2E0 size=72
    let mut pc: u32 = 0x8316A2E0;
    'dispatch: loop {
        match pc {
            0x8316A2E0 => {
    //   block [0x8316A2E0..0x8316A328)
	// 8316A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A2E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A2EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A2F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A2F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A2F8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316A2FC: 396B7E40  addi r11, r11, 0x7e40
	ctx.r[11].s64 = ctx.r[11].s64 + 32320;
	// 8316A300: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316A304: 4182000C  beq 0x8316a310
	if ctx.cr[0].eq {
	pc = 0x8316A310; continue 'dispatch;
	}
	// 8316A308: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8316A30C: 4BFF5975  bl 0x8315fc80
	ctx.lr = 0x8316A310;
	sub_8315FC80(ctx, base);
	// 8316A310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A314: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A320: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A328 size=164
    let mut pc: u32 = 0x8316A328;
    'dispatch: loop {
        match pc {
            0x8316A328 => {
    //   block [0x8316A328..0x8316A3CC)
	// 8316A328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A32C: 4803DE39  bl 0x831a8164
	ctx.lr = 0x8316A330;
	sub_831A8130(ctx, base);
	// 8316A330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A334: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8316A338: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316A33C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A340: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8316A344: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316A348: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 8316A34C: 3BC9949C  addi r30, r9, -0x6b64
	ctx.r[30].s64 = ctx.r[9].s64 + -27492;
	// 8316A350: 3B8A8328  addi r28, r10, -0x7cd8
	ctx.r[28].s64 = ctx.r[10].s64 + -31960;
	// 8316A354: 3BAB948C  addi r29, r11, -0x6b74
	ctx.r[29].s64 = ctx.r[11].s64 + -27508;
	// 8316A358: 7C7FE82E  lwzx r3, r31, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8316A35C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316A360: 419A0028  beq cr6, 0x8316a388
	if ctx.cr[6].eq {
	pc = 0x8316A388; continue 'dispatch;
	}
	// 8316A364: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8316A368: 7C9FF02E  lwzx r4, r31, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8316A36C: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 8316A370: 48005699  bl 0x8316fa08
	ctx.lr = 0x8316A374;
	sub_8316FA08(ctx, base);
	// 8316A374: 7F7FF12E  stwx r27, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 8316A378: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316A37C: 7C7FE82E  lwzx r3, r31, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8316A380: 48005879  bl 0x8316fbf8
	ctx.lr = 0x8316A384;
	sub_8316FBF8(ctx, base);
	// 8316A384: 7F7FE92E  stwx r27, r31, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[27].u32) };
	// 8316A388: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8316A38C: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 8316A390: 4198FFC8  blt cr6, 0x8316a358
	if ctx.cr[6].lt {
	pc = 0x8316A358; continue 'dispatch;
	}
	// 8316A394: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8316A398: 807F9488  lwz r3, -0x6b78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-27512 as u32) ) } as u64;
	// 8316A39C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316A3A0: 419A0010  beq cr6, 0x8316a3b0
	if ctx.cr[6].eq {
	pc = 0x8316A3B0; continue 'dispatch;
	}
	// 8316A3A4: 4BFF5505  bl 0x8315f8a8
	ctx.lr = 0x8316A3A8;
	sub_8315F8A8(ctx, base);
	// 8316A3A8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8316A3AC: 917F9488  stw r11, -0x6b78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-27512 as u32), ctx.r[11].u32 ) };
	// 8316A3B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316A3B4: 48005FE5  bl 0x83170398
	ctx.lr = 0x8316A3B8;
	sub_83170398(ctx, base);
	// 8316A3B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316A3BC: 480057F5  bl 0x8316fbb0
	ctx.lr = 0x8316A3C0;
	sub_8316FBB0(ctx, base);
	// 8316A3C0: 4BFF5259  bl 0x8315f618
	ctx.lr = 0x8316A3C4;
	sub_8315F618(ctx, base);
	// 8316A3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316A3C8: 4803DDEC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A3D0 size=124
    let mut pc: u32 = 0x8316A3D0;
    'dispatch: loop {
        match pc {
            0x8316A3D0 => {
    //   block [0x8316A3D0..0x8316A44C)
	// 8316A3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A3D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316A3DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A3E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A3E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A3E8: 807F0248  lwz r3, 0x248(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 8316A3EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316A3F0: 419A0014  beq cr6, 0x8316a404
	if ctx.cr[6].eq {
	pc = 0x8316A404; continue 'dispatch;
	}
	// 8316A3F4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316A3F8: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8316A3FC: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 8316A400: 4BFFA7B1  bl 0x83164bb0
	ctx.lr = 0x8316A404;
	sub_83164BB0(ctx, base);
	// 8316A404: 807F0250  lwz r3, 0x250(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 8316A408: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316A40C: 93DF029C  stw r30, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[30].u32 ) };
	// 8316A410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316A414: 419A0020  beq cr6, 0x8316a434
	if ctx.cr[6].eq {
	pc = 0x8316A434; continue 'dispatch;
	}
	// 8316A418: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A41C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A420: 388A81F0  addi r4, r10, -0x7e10
	ctx.r[4].s64 = ctx.r[10].s64 + -32272;
	// 8316A424: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A42C: 4E800421  bctrl
	ctx.lr = 0x8316A430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A430: 93DF0250  stw r30, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[30].u32 ) };
	// 8316A434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316A444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A450 size=64
    let mut pc: u32 = 0x8316A450;
    'dispatch: loop {
        match pc {
            0x8316A450 => {
    //   block [0x8316A450..0x8316A490)
	// 8316A450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A45C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A460: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A464: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A468: 388B7EA8  addi r4, r11, 0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + 32424;
	// 8316A46C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8316A470: 4BFF56A9  bl 0x8315fb18
	ctx.lr = 0x8316A474;
	sub_8315FB18(ctx, base);
	// 8316A474: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316A478: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316A47C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A488: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A490 size=88
    let mut pc: u32 = 0x8316A490;
    'dispatch: loop {
        match pc {
            0x8316A490 => {
    //   block [0x8316A490..0x8316A4E8)
	// 8316A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A49C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A4A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A4A4: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 8316A4A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316A4AC: 409A0024  bne cr6, 0x8316a4d0
	if !ctx.cr[6].eq {
	pc = 0x8316A4D0; continue 'dispatch;
	}
	// 8316A4B0: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316A4B4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A4B8: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316A4BC: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316A4C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316A4C4: 38898328  addi r4, r9, -0x7cd8
	ctx.r[4].s64 = ctx.r[9].s64 + -31960;
	// 8316A4C8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316A4CC: 48005ADD  bl 0x8316ffa8
	ctx.lr = 0x8316A4D0;
	sub_8316FFA8(ctx, base);
	// 8316A4D0: 807F0144  lwz r3, 0x144(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 8316A4D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A4E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A4E8 size=88
    let mut pc: u32 = 0x8316A4E8;
    'dispatch: loop {
        match pc {
            0x8316A4E8 => {
    //   block [0x8316A4E8..0x8316A540)
	// 8316A4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A4F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A4F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A4F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A4FC: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8316A500: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316A504: 409A0024  bne cr6, 0x8316a528
	if !ctx.cr[6].eq {
	pc = 0x8316A528; continue 'dispatch;
	}
	// 8316A508: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316A50C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A510: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316A514: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316A518: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316A51C: 38898328  addi r4, r9, -0x7cd8
	ctx.r[4].s64 = ctx.r[9].s64 + -31960;
	// 8316A520: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316A524: 48005A85  bl 0x8316ffa8
	ctx.lr = 0x8316A528;
	sub_8316FFA8(ctx, base);
	// 8316A528: 807F0148  lwz r3, 0x148(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8316A52C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A540 size=88
    let mut pc: u32 = 0x8316A540;
    'dispatch: loop {
        match pc {
            0x8316A540 => {
    //   block [0x8316A540..0x8316A598)
	// 8316A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A54C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A554: 817F01F0  lwz r11, 0x1f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 8316A558: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316A55C: 409A0024  bne cr6, 0x8316a580
	if !ctx.cr[6].eq {
	pc = 0x8316A580; continue 'dispatch;
	}
	// 8316A560: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316A564: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A568: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316A56C: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316A570: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316A574: 38898328  addi r4, r9, -0x7cd8
	ctx.r[4].s64 = ctx.r[9].s64 + -31960;
	// 8316A578: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316A57C: 48005A2D  bl 0x8316ffa8
	ctx.lr = 0x8316A580;
	sub_8316FFA8(ctx, base);
	// 8316A580: 807F01F0  lwz r3, 0x1f0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 8316A584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A598 size=108
    let mut pc: u32 = 0x8316A598;
    'dispatch: loop {
        match pc {
            0x8316A598 => {
    //   block [0x8316A598..0x8316A604)
	// 8316A598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A59C: 4803DBC5  bl 0x831a8160
	ctx.lr = 0x8316A5A0;
	sub_831A8130(ctx, base);
	// 8316A5A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A5A4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316A5A8: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 8316A5AC: 3B4B948C  addi r26, r11, -0x6b74
	ctx.r[26].s64 = ctx.r[11].s64 + -27508;
	// 8316A5B0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316A5B4: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 8316A5B8: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 8316A5BC: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 8316A5C0: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A5C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316A5C8: 419A0024  beq cr6, 0x8316a5ec
	if ctx.cr[6].eq {
	pc = 0x8316A5EC; continue 'dispatch;
	}
	// 8316A5CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316A5D0: 809DBCDC  lwz r4, -0x4324(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-17188 as u32) ) } as u64;
	// 8316A5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A5D8: 48005561  bl 0x8316fb38
	ctx.lr = 0x8316A5DC;
	sub_8316FB38(ctx, base);
	// 8316A5DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316A5E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A5E4: 809CBCE0  lwz r4, -0x4320(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-17184 as u32) ) } as u64;
	// 8316A5E8: 48005579  bl 0x8316fb60
	ctx.lr = 0x8316A5EC;
	sub_8316FB60(ctx, base);
	// 8316A5EC: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 8316A5F0: 397A0010  addi r11, r26, 0x10
	ctx.r[11].s64 = ctx.r[26].s64 + 16;
	// 8316A5F4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8316A5F8: 4198FFC8  blt cr6, 0x8316a5c0
	if ctx.cr[6].lt {
	pc = 0x8316A5C0; continue 'dispatch;
	}
	// 8316A5FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316A600: 4803DBB0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A608 size=176
    let mut pc: u32 = 0x8316A608;
    'dispatch: loop {
        match pc {
            0x8316A608 => {
    //   block [0x8316A608..0x8316A6B8)
	// 8316A608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A60C: 4803DB61  bl 0x831a816c
	ctx.lr = 0x8316A610;
	sub_831A8130(ctx, base);
	// 8316A610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A614: 83C30178  lwz r30, 0x178(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(376 as u32) ) } as u64;
	// 8316A618: 3BE30160  addi r31, r3, 0x160
	ctx.r[31].s64 = ctx.r[3].s64 + 352;
	// 8316A61C: 80E30188  lwz r7, 0x188(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(392 as u32) ) } as u64;
	// 8316A620: 80C30184  lwz r6, 0x184(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(388 as u32) ) } as u64;
	// 8316A624: 80A30180  lwz r5, 0x180(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(384 as u32) ) } as u64;
	// 8316A628: 8083017C  lwz r4, 0x17c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(380 as u32) ) } as u64;
	// 8316A62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316A630: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A634: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316A638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A63C: 4E800421  bctrl
	ctx.lr = 0x8316A640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A640: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8316A644: 40820010  bne 0x8316a654
	if !ctx.cr[0].eq {
	pc = 0x8316A654; continue 'dispatch;
	}
	// 8316A648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316A64C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8316A650: 48000060  b 0x8316a6b0
	pc = 0x8316A6B0; continue 'dispatch;
	// 8316A654: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A658: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316A65C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316A660: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316A664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A668: 4E800421  bctrl
	ctx.lr = 0x8316A66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A66C: F87F0030  std r3, 0x30(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u64 ) };
	// 8316A670: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316A674: 4098002C  bge cr6, 0x8316a6a0
	if !ctx.cr[6].lt {
	pc = 0x8316A6A0; continue 'dispatch;
	}
	// 8316A678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316A67C: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316A680: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316A684: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8316A688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316A68C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A690: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316A694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A698: 4E800421  bctrl
	ctx.lr = 0x8316A69C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A69C: 48000014  b 0x8316a6b0
	pc = 0x8316A6B0; continue 'dispatch;
	// 8316A6A0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316A6A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8316A6A8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316A6AC: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 8316A6B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A6B4: 4803DB08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A6B8 size=132
    let mut pc: u32 = 0x8316A6B8;
    'dispatch: loop {
        match pc {
            0x8316A6B8 => {
    //   block [0x8316A6B8..0x8316A73C)
	// 8316A6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A6C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A6C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A6C8: 3BE301A0  addi r31, r3, 0x1a0
	ctx.r[31].s64 = ctx.r[3].s64 + 416;
	// 8316A6CC: 806301B8  lwz r3, 0x1b8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(440 as u32) ) } as u64;
	// 8316A6D0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316A6D4: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A6D8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8316A6DC: 409A0010  bne cr6, 0x8316a6ec
	if !ctx.cr[6].eq {
	pc = 0x8316A6EC; continue 'dispatch;
	}
	// 8316A6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316A6E4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8316A6E8: 48000040  b 0x8316a728
	pc = 0x8316A728; continue 'dispatch;
	// 8316A6EC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316A6F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A6F4: 419A0010  beq cr6, 0x8316a704
	if ctx.cr[6].eq {
	pc = 0x8316A704; continue 'dispatch;
	}
	// 8316A6F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316A6FC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8316A700: 4800001C  b 0x8316a71c
	pc = 0x8316A71C; continue 'dispatch;
	// 8316A704: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A708: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316A70C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316A710: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A714: 4E800421  bctrl
	ctx.lr = 0x8316A718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A718: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8316A71C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316A720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316A724: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316A728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316A740 size=52
    let mut pc: u32 = 0x8316A740;
    'dispatch: loop {
        match pc {
            0x8316A740 => {
    //   block [0x8316A740..0x8316A774)
	// 8316A740: 816302C0  lwz r11, 0x2c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316A744: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316A748: 81230244  lwz r9, 0x244(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(580 as u32) ) } as u64;
	// 8316A74C: 388301D0  addi r4, r3, 0x1d0
	ctx.r[4].s64 = ctx.r[3].s64 + 464;
	// 8316A750: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316A754: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316A758: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8316A75C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316A760: 409A0014  bne cr6, 0x8316a774
	if !ctx.cr[6].eq {
		sub_8316A774(ctx, base);
		return;
	}
	// 8316A764: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8316A768: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8316A76C: F9440060  std r10, 0x60(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8316A770: 4800007C  b 0x8316a7ec
	sub_8316A7DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316A774 size=36
    let mut pc: u32 = 0x8316A774;
    'dispatch: loop {
        match pc {
            0x8316A774 => {
    //   block [0x8316A774..0x8316A798)
	// 8316A774: 80E40070  lwz r7, 0x70(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 8316A778: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8316A77C: 409A001C  bne cr6, 0x8316a798
	if !ctx.cr[6].eq {
		sub_8316A798(ctx, base);
		return;
	}
	// 8316A780: 8164006C  lwz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316A784: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A788: 409A0010  bne cr6, 0x8316a798
	if !ctx.cr[6].eq {
		sub_8316A798(ctx, base);
		return;
	}
	// 8316A78C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316A790: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 8316A794: 4800556C  b 0x8316fd00
	sub_8316FD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316A798 size=68
    let mut pc: u32 = 0x8316A798;
    'dispatch: loop {
        match pc {
            0x8316A798 => {
    //   block [0x8316A798..0x8316A7DC)
	// 8316A798: E9640058  ld r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) };
	// 8316A79C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316A7A0: E9240060  ld r9, 0x60(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) };
	// 8316A7A4: 81040034  lwz r8, 0x34(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 8316A7A8: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8316A7AC: 80C40030  lwz r6, 0x30(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316A7B0: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 8316A7B4: F9440058  std r10, 0x58(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8316A7B8: F9240060  std r9, 0x60(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 8316A7BC: 91640034  stw r11, 0x34(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8316A7C0: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 8316A7C4: F9440038  std r10, 0x38(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), ctx.r[10].u64 ) };
	// 8316A7C8: 40980014  bge cr6, 0x8316a7dc
	if !ctx.cr[6].lt {
		sub_8316A7DC(ctx, base);
		return;
	}
	// 8316A7CC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316A7D0: 91440070  stw r10, 0x70(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 8316A7D4: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 8316A7D8: 48005528  b 0x8316fd00
	sub_8316FD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A7DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316A7DC size=24
    let mut pc: u32 = 0x8316A7DC;
    'dispatch: loop {
        match pc {
            0x8316A7DC => {
    //   block [0x8316A7DC..0x8316A7F4)
	// 8316A7DC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8316A7E0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316A7E4: 409A0008  bne cr6, 0x8316a7ec
	if !ctx.cr[6].eq {
	pc = 0x8316A7EC; continue 'dispatch;
	}
	// 8316A7E8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316A7EC: 91640020  stw r11, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8316A7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A7F8 size=84
    let mut pc: u32 = 0x8316A7F8;
    'dispatch: loop {
        match pc {
            0x8316A7F8 => {
    //   block [0x8316A7F8..0x8316A84C)
	// 8316A7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A800: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A80C: 4BFFFC85  bl 0x8316a490
	ctx.lr = 0x8316A810;
	sub_8316A490(ctx, base);
	// 8316A810: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316A814: 419A000C  beq cr6, 0x8316a820
	if ctx.cr[6].eq {
	pc = 0x8316A820; continue 'dispatch;
	}
	// 8316A818: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A81C: 4800001C  b 0x8316a838
	pc = 0x8316A838; continue 'dispatch;
	// 8316A820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A824: 4BFFFCC5  bl 0x8316a4e8
	ctx.lr = 0x8316A828;
	sub_8316A4E8(ctx, base);
	// 8316A828: 3963FFFE  addi r11, r3, -2
	ctx.r[11].s64 = ctx.r[3].s64 + -2;
	// 8316A82C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316A830: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316A834: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8316A838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A850 size=100
    let mut pc: u32 = 0x8316A850;
    'dispatch: loop {
        match pc {
            0x8316A850 => {
    //   block [0x8316A850..0x8316A8B4)
	// 8316A850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A858: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A85C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A860: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A864: 4BFFFC2D  bl 0x8316a490
	ctx.lr = 0x8316A868;
	sub_8316A490(ctx, base);
	// 8316A868: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316A86C: 409A000C  bne cr6, 0x8316a878
	if !ctx.cr[6].eq {
	pc = 0x8316A878; continue 'dispatch;
	}
	// 8316A870: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A874: 4800002C  b 0x8316a8a0
	pc = 0x8316A8A0; continue 'dispatch;
	// 8316A878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A87C: 4BFFFC6D  bl 0x8316a4e8
	ctx.lr = 0x8316A880;
	sub_8316A4E8(ctx, base);
	// 8316A880: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316A884: 419AFFEC  beq cr6, 0x8316a870
	if ctx.cr[6].eq {
	pc = 0x8316A870; continue 'dispatch;
	}
	// 8316A888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316A88C: 4BFFFCB5  bl 0x8316a540
	ctx.lr = 0x8316A890;
	sub_8316A540(ctx, base);
	// 8316A890: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8316A894: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316A898: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316A89C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8316A8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316A8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A8AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A8B8 size=112
    let mut pc: u32 = 0x8316A8B8;
    'dispatch: loop {
        match pc {
            0x8316A8B8 => {
    //   block [0x8316A8B8..0x8316A928)
	// 8316A8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A8C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A8C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A8C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A8CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316A8D0: 807F0250  lwz r3, 0x250(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 8316A8D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A8D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316A8DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A8E0: 4E800421  bctrl
	ctx.lr = 0x8316A8E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A8E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A8E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A8EC: 419A0028  beq cr6, 0x8316a914
	if ctx.cr[6].eq {
	pc = 0x8316A914; continue 'dispatch;
	}
	// 8316A8F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A8F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A8F8: 388B7EEC  addi r4, r11, 0x7eec
	ctx.r[4].s64 = ctx.r[11].s64 + 32492;
	// 8316A8FC: 4BFF521D  bl 0x8315fb18
	ctx.lr = 0x8316A900;
	sub_8315FB18(ctx, base);
	// 8316A900: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316A904: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A908: 409A000C  bne cr6, 0x8316a914
	if !ctx.cr[6].eq {
	pc = 0x8316A914; continue 'dispatch;
	}
	// 8316A90C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A910: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316A914: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A928 size=112
    let mut pc: u32 = 0x8316A928;
    'dispatch: loop {
        match pc {
            0x8316A928 => {
    //   block [0x8316A928..0x8316A998)
	// 8316A928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A934: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A93C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316A940: 807F0250  lwz r3, 0x250(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 8316A944: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316A948: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316A94C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316A950: 4E800421  bctrl
	ctx.lr = 0x8316A954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316A954: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A958: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A95C: 419A0028  beq cr6, 0x8316a984
	if ctx.cr[6].eq {
	pc = 0x8316A984; continue 'dispatch;
	}
	// 8316A960: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A964: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316A968: 388B7F1C  addi r4, r11, 0x7f1c
	ctx.r[4].s64 = ctx.r[11].s64 + 32540;
	// 8316A96C: 4BFF51AD  bl 0x8315fb18
	ctx.lr = 0x8316A970;
	sub_8315FB18(ctx, base);
	// 8316A970: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316A974: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316A978: 409A000C  bne cr6, 0x8316a984
	if !ctx.cr[6].eq {
	pc = 0x8316A984; continue 'dispatch;
	}
	// 8316A97C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316A980: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316A984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316A988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316A98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316A990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316A994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316A998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316A998 size=296
    let mut pc: u32 = 0x8316A998;
    'dispatch: loop {
        match pc {
            0x8316A998 => {
    //   block [0x8316A998..0x8316AAC0)
	// 8316A998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316A99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316A9A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316A9A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316A9A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316A9AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316A9B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316A9B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316A9B8: 396B7F50  addi r11, r11, 0x7f50
	ctx.r[11].s64 = ctx.r[11].s64 + 32592;
	// 8316A9BC: 387F0254  addi r3, r31, 0x254
	ctx.r[3].s64 = ctx.r[31].s64 + 596;
	// 8316A9C0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8316A9C4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8316A9C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316A9CC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8316A9D0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8316A9D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316A9D8: 93DF0248  stw r30, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[30].u32 ) };
	// 8316A9DC: 93DF024C  stw r30, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[30].u32 ) };
	// 8316A9E0: 93DF0250  stw r30, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[30].u32 ) };
	// 8316A9E4: 4803D7FD  bl 0x831a81e0
	ctx.lr = 0x8316A9E8;
	sub_831A81E0(ctx, base);
	// 8316A9E8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8316A9EC: 93DF0294  stw r30, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[30].u32 ) };
	// 8316A9F0: 3D400010  lis r10, 0x10
	ctx.r[10].s64 = 1048576;
	// 8316A9F4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8316A9F8: 917F0298  stw r11, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[11].u32 ) };
	// 8316A9FC: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8316AA00: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8316AA04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AA08: 4803D7D9  bl 0x831a81e0
	ctx.lr = 0x8316AA0C;
	sub_831A81E0(ctx, base);
	// 8316AA0C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316AA10: 387F02A0  addi r3, r31, 0x2a0
	ctx.r[3].s64 = ctx.r[31].s64 + 672;
	// 8316AA14: 93DF02E4  stw r30, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[30].u32 ) };
	// 8316AA18: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 8316AA1C: 917F011C  stw r11, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 8316AA20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AA24: 4803D7BD  bl 0x831a81e0
	ctx.lr = 0x8316AA28;
	sub_831A81E0(ctx, base);
	// 8316AA28: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 8316AA2C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8316AA30: 93DF029C  stw r30, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[30].u32 ) };
	// 8316AA34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AA38: 93DF02E8  stw r30, 0x2e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(744 as u32), ctx.r[30].u32 ) };
	// 8316AA3C: 4803D7A5  bl 0x831a81e0
	ctx.lr = 0x8316AA40;
	sub_831A81E0(ctx, base);
	// 8316AA40: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316AA44: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 8316AA48: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 8316AA4C: 796B0040  clrldi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8316AA50: FBDF02F0  std r30, 0x2f0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[30].u64 ) };
	// 8316AA54: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8316AA58: FBDF02F8  std r30, 0x2f8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[30].u64 ) };
	// 8316AA5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AA60: FBDF0300  std r30, 0x300(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[30].u64 ) };
	// 8316AA64: F97F0308  std r11, 0x308(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[11].u64 ) };
	// 8316AA68: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 8316AA6C: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 8316AA70: 4803D771  bl 0x831a81e0
	ctx.lr = 0x8316AA74;
	sub_831A81E0(ctx, base);
	// 8316AA74: 387F0160  addi r3, r31, 0x160
	ctx.r[3].s64 = ctx.r[31].s64 + 352;
	// 8316AA78: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8316AA7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AA80: 4803D761  bl 0x831a81e0
	ctx.lr = 0x8316AA84;
	sub_831A81E0(ctx, base);
	// 8316AA84: 387F01A0  addi r3, r31, 0x1a0
	ctx.r[3].s64 = ctx.r[31].s64 + 416;
	// 8316AA88: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8316AA8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AA90: 4803D751  bl 0x831a81e0
	ctx.lr = 0x8316AA94;
	sub_831A81E0(ctx, base);
	// 8316AA94: 387F01D0  addi r3, r31, 0x1d0
	ctx.r[3].s64 = ctx.r[31].s64 + 464;
	// 8316AA98: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 8316AA9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AAA0: 4803D741  bl 0x831a81e0
	ctx.lr = 0x8316AAA4;
	sub_831A81E0(ctx, base);
	// 8316AAA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316AAA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316AAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316AAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316AAB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316AAB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316AABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AAC0 size=72
    let mut pc: u32 = 0x8316AAC0;
    'dispatch: loop {
        match pc {
            0x8316AAC0 => {
    //   block [0x8316AAC0..0x8316AB08)
	// 8316AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316AAC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316AACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AAD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316AAD4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316AAD8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316AADC: 396B7E40  addi r11, r11, 0x7e40
	ctx.r[11].s64 = ctx.r[11].s64 + 32320;
	// 8316AAE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316AAE4: 4182000C  beq 0x8316aaf0
	if ctx.cr[0].eq {
	pc = 0x8316AAF0; continue 'dispatch;
	}
	// 8316AAE8: 38800310  li r4, 0x310
	ctx.r[4].s64 = 784;
	// 8316AAEC: 4BFF5195  bl 0x8315fc80
	ctx.lr = 0x8316AAF0;
	sub_8315FC80(ctx, base);
	// 8316AAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316AAF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316AAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316AAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316AB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316AB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316AB08 size=56
    let mut pc: u32 = 0x8316AB08;
    'dispatch: loop {
        match pc {
            0x8316AB08 => {
    //   block [0x8316AB08..0x8316AB40)
	// 8316AB08: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316AB0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316AB10: 38EB8480  addi r7, r11, -0x7b80
	ctx.r[7].s64 = ctx.r[11].s64 + -31616;
	// 8316AB14: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316AB18: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8316AB1C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316AB20: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8316AB24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8316AB28: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8316AB2C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316AB30: 4082FFE8  bne 0x8316ab18
	if !ctx.cr[0].eq {
	pc = 0x8316AB18; continue 'dispatch;
	}
	// 8316AB34: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8316AB38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316AB3C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316AB40 size=12
    let mut pc: u32 = 0x8316AB40;
    'dispatch: loop {
        match pc {
            0x8316AB40 => {
    //   block [0x8316AB40..0x8316AB4C)
	// 8316AB40: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316AB44: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 8316AB48: 4BFFF7E0  b 0x8316a328
	sub_8316A328(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AB4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316AB4C size=4
    let mut pc: u32 = 0x8316AB4C;
    'dispatch: loop {
        match pc {
            0x8316AB4C => {
    //   block [0x8316AB4C..0x8316AB50)
	// 8316AB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AB50 size=192
    let mut pc: u32 = 0x8316AB50;
    'dispatch: loop {
        match pc {
            0x8316AB50 => {
    //   block [0x8316AB50..0x8316AC10)
	// 8316AB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316AB58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316AB5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316AB60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AB64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316AB68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316AB6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316AB70: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8316AB74: 387F0254  addi r3, r31, 0x254
	ctx.r[3].s64 = ctx.r[31].s64 + 596;
	// 8316AB78: 4BFF51C1  bl 0x8315fd38
	ctx.lr = 0x8316AB7C;
	sub_8315FD38(ctx, base);
	// 8316AB7C: 907F0250  stw r3, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[3].u32 ) };
	// 8316AB80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316AB84: 41820058  beq 0x8316abdc
	if ctx.cr[0].eq {
	pc = 0x8316ABDC; continue 'dispatch;
	}
	// 8316AB88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316AB8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316AB90: 409A004C  bne cr6, 0x8316abdc
	if !ctx.cr[6].eq {
	pc = 0x8316ABDC; continue 'dispatch;
	}
	// 8316AB94: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316AB98: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316AB9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316ABA0: 816B8484  lwz r11, -0x7b7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31612 as u32) ) } as u64;
	// 8316ABA4: 91498328  stw r10, -0x7cd8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-31960 as u32), ctx.r[10].u32 ) };
	// 8316ABA8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316ABAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316ABB0: 40820008  bne 0x8316abb8
	if !ctx.cr[0].eq {
	pc = 0x8316ABB8; continue 'dispatch;
	}
	// 8316ABB4: 48005F6D  bl 0x83170b20
	ctx.lr = 0x8316ABB8;
	sub_83170B20(ctx, base);
	// 8316ABB8: 907F029C  stw r3, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[3].u32 ) };
	// 8316ABBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316ABC0: 409A0010  bne cr6, 0x8316abd0
	if !ctx.cr[6].eq {
	pc = 0x8316ABD0; continue 'dispatch;
	}
	// 8316ABC4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316ABC8: 388B7FF0  addi r4, r11, 0x7ff0
	ctx.r[4].s64 = ctx.r[11].s64 + 32752;
	// 8316ABCC: 48000018  b 0x8316abe4
	pc = 0x8316ABE4; continue 'dispatch;
	// 8316ABD0: 93DF024C  stw r30, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[30].u32 ) };
	// 8316ABD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316ABD8: 48000020  b 0x8316abf8
	pc = 0x8316ABF8; continue 'dispatch;
	// 8316ABDC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316ABE0: 388B7FBC  addi r4, r11, 0x7fbc
	ctx.r[4].s64 = ctx.r[11].s64 + 32700;
	// 8316ABE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316ABE8: 4BFF4F31  bl 0x8315fb18
	ctx.lr = 0x8316ABEC;
	sub_8315FB18(ctx, base);
	// 8316ABEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316ABF0: 4BFFF7E1  bl 0x8316a3d0
	ctx.lr = 0x8316ABF4;
	sub_8316A3D0(ctx, base);
	// 8316ABF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316ABF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316ABFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316AC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316AC04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316AC08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316AC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AC10 size=64
    let mut pc: u32 = 0x8316AC10;
    'dispatch: loop {
        match pc {
            0x8316AC10 => {
    //   block [0x8316AC10..0x8316AC50)
	// 8316AC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AC14: 4803D559  bl 0x831a816c
	ctx.lr = 0x8316AC18;
	sub_831A8130(ctx, base);
	// 8316AC18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AC1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316AC20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316AC24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316AC28: 4BFFFC91  bl 0x8316a8b8
	ctx.lr = 0x8316AC2C;
	sub_8316A8B8(ctx, base);
	// 8316AC2C: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316AC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316AC34: 93DF0248  stw r30, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[30].u32 ) };
	// 8316AC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316AC3C: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316AC40: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316AC44: 4BFFFCE5  bl 0x8316a928
	ctx.lr = 0x8316AC48;
	sub_8316A928(ctx, base);
	// 8316AC48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316AC4C: 4803D570  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AC50 size=64
    let mut pc: u32 = 0x8316AC50;
    'dispatch: loop {
        match pc {
            0x8316AC50 => {
    //   block [0x8316AC50..0x8316AC90)
	// 8316AC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AC54: 4803D519  bl 0x831a816c
	ctx.lr = 0x8316AC58;
	sub_831A8130(ctx, base);
	// 8316AC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AC5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316AC60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316AC64: 4BFFFC55  bl 0x8316a8b8
	ctx.lr = 0x8316AC68;
	sub_8316A8B8(ctx, base);
	// 8316AC68: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316AC6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316AC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316AC74: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316AC78: 83BF0298  lwz r29, 0x298(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) } as u64;
	// 8316AC7C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316AC80: 4BFFFCA9  bl 0x8316a928
	ctx.lr = 0x8316AC84;
	sub_8316A928(ctx, base);
	// 8316AC84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316AC88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316AC8C: 4803D530  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AC90 size=64
    let mut pc: u32 = 0x8316AC90;
    'dispatch: loop {
        match pc {
            0x8316AC90 => {
    //   block [0x8316AC90..0x8316ACD0)
	// 8316AC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AC94: 4803D4D9  bl 0x831a816c
	ctx.lr = 0x8316AC98;
	sub_831A8130(ctx, base);
	// 8316AC98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AC9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316ACA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316ACA4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316ACA8: 4BFFFC11  bl 0x8316a8b8
	ctx.lr = 0x8316ACAC;
	sub_8316A8B8(ctx, base);
	// 8316ACAC: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316ACB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316ACB4: 93DF0298  stw r30, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[30].u32 ) };
	// 8316ACB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316ACBC: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316ACC0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316ACC4: 4BFFFC65  bl 0x8316a928
	ctx.lr = 0x8316ACC8;
	sub_8316A928(ctx, base);
	// 8316ACC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316ACCC: 4803D4F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316ACD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316ACD0 size=64
    let mut pc: u32 = 0x8316ACD0;
    'dispatch: loop {
        match pc {
            0x8316ACD0 => {
    //   block [0x8316ACD0..0x8316AD10)
	// 8316ACD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316ACD4: 4803D499  bl 0x831a816c
	ctx.lr = 0x8316ACD8;
	sub_831A8130(ctx, base);
	// 8316ACD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316ACDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316ACE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316ACE4: 4BFFFBD5  bl 0x8316a8b8
	ctx.lr = 0x8316ACE8;
	sub_8316A8B8(ctx, base);
	// 8316ACE8: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316ACEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316ACF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316ACF4: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316ACF8: EBBF0010  ld r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8316ACFC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316AD00: 4BFFFC29  bl 0x8316a928
	ctx.lr = 0x8316AD04;
	sub_8316A928(ctx, base);
	// 8316AD04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316AD08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316AD0C: 4803D4B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AD10 size=64
    let mut pc: u32 = 0x8316AD10;
    'dispatch: loop {
        match pc {
            0x8316AD10 => {
    //   block [0x8316AD10..0x8316AD50)
	// 8316AD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AD14: 4803D459  bl 0x831a816c
	ctx.lr = 0x8316AD18;
	sub_831A8130(ctx, base);
	// 8316AD18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AD1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316AD20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316AD24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316AD28: 4BFFFB91  bl 0x8316a8b8
	ctx.lr = 0x8316AD2C;
	sub_8316A8B8(ctx, base);
	// 8316AD2C: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316AD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316AD34: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 8316AD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316AD3C: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316AD40: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316AD44: 4BFFFBE5  bl 0x8316a928
	ctx.lr = 0x8316AD48;
	sub_8316A928(ctx, base);
	// 8316AD48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316AD4C: 4803D470  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AD50 size=668
    let mut pc: u32 = 0x8316AD50;
    'dispatch: loop {
        match pc {
            0x8316AD50 => {
    //   block [0x8316AD50..0x8316AFEC)
	// 8316AD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AD54: 4803D405  bl 0x831a8158
	ctx.lr = 0x8316AD58;
	sub_831A8130(ctx, base);
	// 8316AD58: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AD5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316AD60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8316AD64: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8316AD68: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8316AD6C: 4BFFFAE5  bl 0x8316a850
	ctx.lr = 0x8316AD70;
	sub_8316A850(ctx, base);
	// 8316AD70: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316AD74: 40820010  bne 0x8316ad84
	if !ctx.cr[0].eq {
	pc = 0x8316AD84; continue 'dispatch;
	}
	// 8316AD78: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316AD7C: 388B80D8  addi r4, r11, -0x7f28
	ctx.r[4].s64 = ctx.r[11].s64 + -32552;
	// 8316AD80: 48000248  b 0x8316afc8
	pc = 0x8316AFC8; continue 'dispatch;
	// 8316AD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316AD88: 4BFFF709  bl 0x8316a490
	ctx.lr = 0x8316AD8C;
	sub_8316A490(ctx, base);
	// 8316AD8C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316AD90: 419A0230  beq cr6, 0x8316afc0
	if ctx.cr[6].eq {
	pc = 0x8316AFC0; continue 'dispatch;
	}
	// 8316AD94: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316AD98: 409A0020  bne cr6, 0x8316adb8
	if !ctx.cr[6].eq {
	pc = 0x8316ADB8; continue 'dispatch;
	}
	// 8316AD9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316ADA0: 4BFFF749  bl 0x8316a4e8
	ctx.lr = 0x8316ADA4;
	sub_8316A4E8(ctx, base);
	// 8316ADA4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316ADA8: 419A0010  beq cr6, 0x8316adb8
	if ctx.cr[6].eq {
	pc = 0x8316ADB8; continue 'dispatch;
	}
	// 8316ADAC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316ADB0: 388B8098  addi r4, r11, -0x7f68
	ctx.r[4].s64 = ctx.r[11].s64 + -32616;
	// 8316ADB4: 48000214  b 0x8316afc8
	pc = 0x8316AFC8; continue 'dispatch;
	// 8316ADB8: 807F0248  lwz r3, 0x248(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 8316ADBC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316ADC0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316ADC4: 837F029C  lwz r27, 0x29c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 8316ADC8: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 8316ADCC: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8316ADD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316ADD4: 3B4B8328  addi r26, r11, -0x7cd8
	ctx.r[26].s64 = ctx.r[11].s64 + -31960;
	// 8316ADD8: 419A001C  beq cr6, 0x8316adf4
	if ctx.cr[6].eq {
	pc = 0x8316ADF4; continue 'dispatch;
	}
	// 8316ADDC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8316ADE0: 38C00104  li r6, 0x104
	ctx.r[6].s64 = 260;
	// 8316ADE4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316ADE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316ADEC: 4BFF9CB5  bl 0x83164aa0
	ctx.lr = 0x8316ADF0;
	sub_83164AA0(ctx, base);
	// 8316ADF0: 48000038  b 0x8316ae28
	pc = 0x8316AE28; continue 'dispatch;
	// 8316ADF4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8316ADF8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8316ADFC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AE00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316AE04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316AE08: 409AFFF4  bne cr6, 0x8316adfc
	if !ctx.cr[6].eq {
	pc = 0x8316ADFC; continue 'dispatch;
	}
	// 8316AE0C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8316AE10: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AE14: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316AE18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316AE1C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8316AE20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316AE24: 409AFFEC  bne cr6, 0x8316ae10
	if !ctx.cr[6].eq {
	pc = 0x8316AE10; continue 'dispatch;
	}
	// 8316AE28: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AE2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316AE30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316AE34: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316AE38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316AE40: 4E800421  bctrl
	ctx.lr = 0x8316AE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316AE44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316AE48: 40820010  bne 0x8316ae58
	if !ctx.cr[0].eq {
	pc = 0x8316AE58; continue 'dispatch;
	}
	// 8316AE4C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316AE50: 388B806C  addi r4, r11, -0x7f94
	ctx.r[4].s64 = ctx.r[11].s64 + -32660;
	// 8316AE54: 48000174  b 0x8316afc8
	pc = 0x8316AFC8; continue 'dispatch;
	// 8316AE58: 387F02A0  addi r3, r31, 0x2a0
	ctx.r[3].s64 = ctx.r[31].s64 + 672;
	// 8316AE5C: FBBF02F0  std r29, 0x2f0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[29].u64 ) };
	// 8316AE60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316AE64: FBBF02F8  std r29, 0x2f8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[29].u64 ) };
	// 8316AE68: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 8316AE6C: 93BF0148  stw r29, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[29].u32 ) };
	// 8316AE70: 933F011C  stw r25, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[25].u32 ) };
	// 8316AE74: 3B7F02F0  addi r27, r31, 0x2f0
	ctx.r[27].s64 = ctx.r[31].s64 + 752;
	// 8316AE78: 931F02E4  stw r24, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[24].u32 ) };
	// 8316AE7C: 4803D695  bl 0x831a8510
	ctx.lr = 0x8316AE80;
	sub_831A8510(ctx, base);
	// 8316AE80: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8316AE84: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AE88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316AE8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8316AE90: 409AFFF4  bne cr6, 0x8316ae84
	if !ctx.cr[6].eq {
	pc = 0x8316AE84; continue 'dispatch;
	}
	// 8316AE94: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8316AE98: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8316AE9C: 556B003F  rotlwi. r11, r11, 0
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316AEA0: 40810028  ble 0x8316aec8
	if !ctx.cr[0].gt {
	pc = 0x8316AEC8; continue 'dispatch;
	}
	// 8316AEA4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8316AEA8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AEAC: 2B09002F  cmplwi cr6, r9, 0x2f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 47 as u32, &mut ctx.xer);
	// 8316AEB0: 409A000C  bne cr6, 0x8316aebc
	if !ctx.cr[6].eq {
	pc = 0x8316AEBC; continue 'dispatch;
	}
	// 8316AEB4: 3920005C  li r9, 0x5c
	ctx.r[9].s64 = 92;
	// 8316AEB8: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8316AEBC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316AEC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316AEC4: 4082FFE4  bne 0x8316aea8
	if !ctx.cr[0].eq {
	pc = 0x8316AEA8; continue 'dispatch;
	}
	// 8316AEC8: 3BBF02E8  addi r29, r31, 0x2e8
	ctx.r[29].s64 = ctx.r[31].s64 + 744;
	// 8316AECC: 80BF02E4  lwz r5, 0x2e4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8316AED0: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 8316AED4: 809F011C  lwz r4, 0x11c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 8316AED8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8316AEDC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8316AEE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316AEE4: 48005665  bl 0x83170548
	ctx.lr = 0x8316AEE8;
	sub_83170548(ctx, base);
	// 8316AEE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316AEEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316AEF0: 41820058  beq 0x8316af48
	if ctx.cr[0].eq {
	pc = 0x8316AF48; continue 'dispatch;
	}
	// 8316AEF4: 3B9F0120  addi r28, r31, 0x120
	ctx.r[28].s64 = ctx.r[31].s64 + 288;
	// 8316AEF8: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8316AEFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316AF00: 4803D2E1  bl 0x831a81e0
	ctx.lr = 0x8316AF04;
	sub_831A81E0(ctx, base);
	// 8316AF04: 93DF0128  stw r30, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[30].u32 ) };
	// 8316AF08: 817F011C  lwz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 8316AF0C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316AF10: 917F012C  stw r11, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 8316AF14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316AF18: 817F02E4  lwz r11, 0x2e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8316AF1C: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 8316AF20: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316AF24: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 8316AF28: E97B0000  ld r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 8316AF2C: F97F0138  std r11, 0x138(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u64 ) };
	// 8316AF30: 48005529  bl 0x83170458
	ctx.lr = 0x8316AF34;
	sub_83170458(ctx, base);
	// 8316AF34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316AF38: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8316AF3C: 917F0140  stw r11, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 8316AF40: 915F0144  stw r10, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 8316AF44: 480000A0  b 0x8316afe4
	pc = 0x8316AFE4; continue 'dispatch;
	// 8316AF48: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316AF4C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316AF50: 3B9F0160  addi r28, r31, 0x160
	ctx.r[28].s64 = ctx.r[31].s64 + 352;
	// 8316AF54: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316AF58: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316AF5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316AF60: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8316AF64: 7F6B502E  lwzx r27, r11, r10
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316AF68: 4803D279  bl 0x831a81e0
	ctx.lr = 0x8316AF6C;
	sub_831A81E0(ctx, base);
	// 8316AF6C: 815F029C  lwz r10, 0x29c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 8316AF70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316AF74: 93DF0180  stw r30, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[30].u32 ) };
	// 8316AF78: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8316AF7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316AF80: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316AF84: 915F0178  stw r10, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[10].u32 ) };
	// 8316AF88: 815F011C  lwz r10, 0x11c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 8316AF8C: 915F0184  stw r10, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[10].u32 ) };
	// 8316AF90: 815F02E4  lwz r10, 0x2e4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8316AF94: 915F0188  stw r10, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 8316AF98: 93BF0198  stw r29, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[29].u32 ) };
	// 8316AF9C: 815F024C  lwz r10, 0x24c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 8316AFA0: 915F017C  stw r10, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[10].u32 ) };
	// 8316AFA4: 815F0298  lwz r10, 0x298(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) } as u64;
	// 8316AFA8: 915F0168  stw r10, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[10].u32 ) };
	// 8316AFAC: 917F016C  stw r11, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 8316AFB0: 93FF0174  stw r31, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[31].u32 ) };
	// 8316AFB4: 917F0144  stw r11, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 8316AFB8: 48005059  bl 0x83170010
	ctx.lr = 0x8316AFBC;
	sub_83170010(ctx, base);
	// 8316AFBC: 48000028  b 0x8316afe4
	pc = 0x8316AFE4; continue 'dispatch;
	// 8316AFC0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316AFC4: 388B8018  addi r4, r11, -0x7fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -32744;
	// 8316AFC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316AFCC: 4BFF4B4D  bl 0x8315fb18
	ctx.lr = 0x8316AFD0;
	sub_8315FB18(ctx, base);
	// 8316AFD0: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316AFD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316AFD8: 409A000C  bne cr6, 0x8316afe4
	if !ctx.cr[6].eq {
	pc = 0x8316AFE4; continue 'dispatch;
	}
	// 8316AFDC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316AFE0: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316AFE4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8316AFE8: 4803D1C0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316AFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316AFF0 size=636
    let mut pc: u32 = 0x8316AFF0;
    'dispatch: loop {
        match pc {
            0x8316AFF0 => {
    //   block [0x8316AFF0..0x8316B26C)
	// 8316AFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316AFF4: 4803D16D  bl 0x831a8160
	ctx.lr = 0x8316AFF8;
	sub_831A8130(ctx, base);
	// 8316AFF8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316AFFC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8316B000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B004: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8316B008: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8316B00C: 409A0010  bne cr6, 0x8316b01c
	if !ctx.cr[6].eq {
	pc = 0x8316B01C; continue 'dispatch;
	}
	// 8316B010: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B014: 388B8344  addi r4, r11, -0x7cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -31932;
	// 8316B018: 48000230  b 0x8316b248
	pc = 0x8316B248; continue 'dispatch;
	// 8316B01C: 2F3C0000  cmpdi cr6, r28, 0
	ctx.cr[6].compare_i64(ctx.r[28].s64, 0, &mut ctx.xer);
	// 8316B020: 40980010  bge cr6, 0x8316b030
	if !ctx.cr[6].lt {
	pc = 0x8316B030; continue 'dispatch;
	}
	// 8316B024: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B028: 388B8324  addi r4, r11, -0x7cdc
	ctx.r[4].s64 = ctx.r[11].s64 + -31964;
	// 8316B02C: 4800021C  b 0x8316b248
	pc = 0x8316B248; continue 'dispatch;
	// 8316B030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B034: 4BFFF45D  bl 0x8316a490
	ctx.lr = 0x8316B038;
	sub_8316A490(ctx, base);
	// 8316B038: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316B03C: 40820010  bne 0x8316b04c
	if !ctx.cr[0].eq {
	pc = 0x8316B04C; continue 'dispatch;
	}
	// 8316B040: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B044: 388B82E4  addi r4, r11, -0x7d1c
	ctx.r[4].s64 = ctx.r[11].s64 + -32028;
	// 8316B048: 48000200  b 0x8316b248
	pc = 0x8316B248; continue 'dispatch;
	// 8316B04C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B050: 4BFFF499  bl 0x8316a4e8
	ctx.lr = 0x8316B054;
	sub_8316A4E8(ctx, base);
	// 8316B054: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B058: 419A01E8  beq cr6, 0x8316b240
	if ctx.cr[6].eq {
	pc = 0x8316B240; continue 'dispatch;
	}
	// 8316B05C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316B060: 419A01E0  beq cr6, 0x8316b240
	if ctx.cr[6].eq {
	pc = 0x8316B240; continue 'dispatch;
	}
	// 8316B064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B068: 4BFFF4D9  bl 0x8316a540
	ctx.lr = 0x8316B06C;
	sub_8316A540(ctx, base);
	// 8316B06C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B070: 409A0018  bne cr6, 0x8316b088
	if !ctx.cr[6].eq {
	pc = 0x8316B088; continue 'dispatch;
	}
	// 8316B074: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B078: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B07C: 388B8290  addi r4, r11, -0x7d70
	ctx.r[4].s64 = ctx.r[11].s64 + -32112;
	// 8316B080: 4BFF4A99  bl 0x8315fb18
	ctx.lr = 0x8316B084;
	sub_8315FB18(ctx, base);
	// 8316B084: 480001E0  b 0x8316b264
	pc = 0x8316B264; continue 'dispatch;
	// 8316B088: 817F02DC  lwz r11, 0x2dc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 8316B08C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B090: 419A01A4  beq cr6, 0x8316b234
	if ctx.cr[6].eq {
	pc = 0x8316B234; continue 'dispatch;
	}
	// 8316B094: 815F02D0  lwz r10, 0x2d0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as u64;
	// 8316B098: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316B09C: 419A0198  beq cr6, 0x8316b234
	if ctx.cr[6].eq {
	pc = 0x8316B234; continue 'dispatch;
	}
	// 8316B0A0: 815F02C4  lwz r10, 0x2c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(708 as u32) ) } as u64;
	// 8316B0A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316B0A8: 409A0010  bne cr6, 0x8316b0b8
	if !ctx.cr[6].eq {
	pc = 0x8316B0B8; continue 'dispatch;
	}
	// 8316B0AC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B0B0: 388B8260  addi r4, r11, -0x7da0
	ctx.r[4].s64 = ctx.r[11].s64 + -32160;
	// 8316B0B4: 48000194  b 0x8316b248
	pc = 0x8316B248; continue 'dispatch;
	// 8316B0B8: 7D5A5BD6  divw r10, r26, r11
	ctx.r[10].s32 = ctx.r[26].s32 / ctx.r[11].s32;
	// 8316B0BC: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8316B0C0: 7D6BD051  subf. r11, r11, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B0C4: 41820014  beq 0x8316b0d8
	if ctx.cr[0].eq {
	pc = 0x8316B0D8; continue 'dispatch;
	}
	// 8316B0C8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B0CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B0D0: 388B8220  addi r4, r11, -0x7de0
	ctx.r[4].s64 = ctx.r[11].s64 + -32224;
	// 8316B0D4: 4BFF4A45  bl 0x8315fb18
	ctx.lr = 0x8316B0D8;
	sub_8315FB18(ctx, base);
	// 8316B0D8: E97F02D2  lwa r11, 0x2d0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as i32) as i64;
	// 8316B0DC: 7D5C5BD2  divd r10, r28, r11
	ctx.r[10].s64 = ctx.r[28].s64 / ctx.r[11].s64;
	// 8316B0E0: 7D6A59D2  mulld r11, r10, r11
	ctx.r[11].s64 = ctx.r[10].s64 * ctx.r[11].s64;
	// 8316B0E4: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 8316B0E8: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316B0EC: 419A0014  beq cr6, 0x8316b100
	if ctx.cr[6].eq {
	pc = 0x8316B100; continue 'dispatch;
	}
	// 8316B0F0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B0F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B0F8: 388B81D0  addi r4, r11, -0x7e30
	ctx.r[4].s64 = ctx.r[11].s64 + -32304;
	// 8316B0FC: 4BFF4A1D  bl 0x8315fb18
	ctx.lr = 0x8316B100;
	sub_8315FB18(ctx, base);
	// 8316B100: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316B104: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316B108: 3BDF01D0  addi r30, r31, 0x1d0
	ctx.r[30].s64 = ctx.r[31].s64 + 464;
	// 8316B10C: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316B110: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B114: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 8316B118: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316B11C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316B120: 7F6B502E  lwzx r27, r11, r10
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316B124: 4803D0BD  bl 0x831a81e0
	ctx.lr = 0x8316B128;
	sub_831A81E0(ctx, base);
	// 8316B128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B12C: 4BFFF365  bl 0x8316a490
	ctx.lr = 0x8316B130;
	sub_8316A490(ctx, base);
	// 8316B130: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8316B134: 419A00F4  beq cr6, 0x8316b228
	if ctx.cr[6].eq {
	pc = 0x8316B228; continue 'dispatch;
	}
	// 8316B138: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8316B13C: 419A00EC  beq cr6, 0x8316b228
	if ctx.cr[6].eq {
	pc = 0x8316B228; continue 'dispatch;
	}
	// 8316B140: E97F0300  ld r11, 0x300(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(768 as u32) ) };
	// 8316B144: E95F02F8  ld r10, 0x2f8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) };
	// 8316B148: E93F02DA  lwa r9, 0x2d8(r31)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as i32) as i64;
	// 8316B14C: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316B150: 7D7D4BD2  divd r11, r29, r9
	ctx.r[11].s64 = ctx.r[29].s64 / ctx.r[9].s64;
	// 8316B154: 7D6B49D2  mulld r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[9].s64;
	// 8316B158: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8316B15C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316B160: 419A0014  beq cr6, 0x8316b174
	if ctx.cr[6].eq {
	pc = 0x8316B174; continue 'dispatch;
	}
	// 8316B164: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B168: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B16C: 388B81A4  addi r4, r11, -0x7e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -32348;
	// 8316B170: 4BFF49A9  bl 0x8315fb18
	ctx.lr = 0x8316B174;
	sub_8315FB18(ctx, base);
	// 8316B174: E95F0308  ld r10, 0x308(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) };
	// 8316B178: E97F0300  ld r11, 0x300(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(768 as u32) ) };
	// 8316B17C: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 8316B180: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316B184: 7F3C4800  cmpd cr6, r28, r9
	ctx.cr[6].compare_i64(ctx.r[28].s64, ctx.r[9].s64, &mut ctx.xer);
	// 8316B188: 40980008  bge cr6, 0x8316b190
	if !ctx.cr[6].lt {
	pc = 0x8316B190; continue 'dispatch;
	}
	// 8316B18C: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8316B190: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 8316B194: 4199000C  bgt cr6, 0x8316b1a0
	if ctx.cr[6].gt {
	pc = 0x8316B1A0; continue 'dispatch;
	}
	// 8316B198: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316B19C: 48000090  b 0x8316b22c
	pc = 0x8316B22C; continue 'dispatch;
	// 8316B1A0: 811F029C  lwz r8, 0x29c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 8316B1A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8316B1A8: F93F0158  std r9, 0x158(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u64 ) };
	// 8316B1AC: 397F0150  addi r11, r31, 0x150
	ctx.r[11].s64 = ctx.r[31].s64 + 336;
	// 8316B1B0: 935F0150  stw r26, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[26].u32 ) };
	// 8316B1B4: 393F02A0  addi r9, r31, 0x2a0
	ctx.r[9].s64 = ctx.r[31].s64 + 672;
	// 8316B1B8: 38FF02E8  addi r7, r31, 0x2e8
	ctx.r[7].s64 = ctx.r[31].s64 + 744;
	// 8316B1BC: 38DF02F0  addi r6, r31, 0x2f0
	ctx.r[6].s64 = ctx.r[31].s64 + 752;
	// 8316B1C0: 911E0018  stw r8, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8316B1C4: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8316B1C8: 80BF024C  lwz r5, 0x24c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 8316B1CC: 3C60833A  lis r3, -0x7cc6
	ctx.r[3].s64 = -2093350912;
	// 8316B1D0: 90BE001C  stw r5, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8316B1D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316B1D8: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8316B1DC: 38A38328  addi r5, r3, -0x7cd8
	ctx.r[5].s64 = ctx.r[3].s64 + -31960;
	// 8316B1E0: 913E0024  stw r9, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8316B1E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316B1E8: 90FE0028  stw r7, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 8316B1EC: 915E0030  stw r10, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8316B1F0: FBBE0040  std r29, 0x40(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u64 ) };
	// 8316B1F4: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8316B1F8: F97E0048  std r11, 0x48(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 8316B1FC: 90DE0068  stw r6, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 8316B200: 817F0298  lwz r11, 0x298(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) } as u64;
	// 8316B204: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316B208: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8316B20C: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 8316B210: E97F02F8  ld r11, 0x2f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) };
	// 8316B214: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8316B218: F97F02F8  std r11, 0x2f8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[11].u64 ) };
	// 8316B21C: 915E0020  stw r10, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8316B220: 48004DF1  bl 0x83170010
	ctx.lr = 0x8316B224;
	sub_83170010(ctx, base);
	// 8316B224: 48000040  b 0x8316b264
	pc = 0x8316B264; continue 'dispatch;
	// 8316B228: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8316B22C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8316B230: 48000034  b 0x8316b264
	pc = 0x8316B264; continue 'dispatch;
	// 8316B234: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B238: 388B8168  addi r4, r11, -0x7e98
	ctx.r[4].s64 = ctx.r[11].s64 + -32408;
	// 8316B23C: 4800000C  b 0x8316b248
	pc = 0x8316B248; continue 'dispatch;
	// 8316B240: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B244: 388B8128  addi r4, r11, -0x7ed8
	ctx.r[4].s64 = ctx.r[11].s64 + -32472;
	// 8316B248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B24C: 4BFF48CD  bl 0x8315fb18
	ctx.lr = 0x8316B250;
	sub_8315FB18(ctx, base);
	// 8316B250: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316B254: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B258: 409A000C  bne cr6, 0x8316b264
	if !ctx.cr[6].eq {
	pc = 0x8316B264; continue 'dispatch;
	}
	// 8316B25C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316B260: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316B264: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316B268: 4803CF48  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B270 size=724
    let mut pc: u32 = 0x8316B270;
    'dispatch: loop {
        match pc {
            0x8316B270 => {
    //   block [0x8316B270..0x8316B544)
	// 8316B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B274: 4803CEE1  bl 0x831a8154
	ctx.lr = 0x8316B278;
	sub_831A8130(ctx, base);
	// 8316B278: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B27C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8316B280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B284: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8316B288: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 8316B28C: 409A0010  bne cr6, 0x8316b29c
	if !ctx.cr[6].eq {
	pc = 0x8316B29C; continue 'dispatch;
	}
	// 8316B290: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B294: 388B85D4  addi r4, r11, -0x7a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -31276;
	// 8316B298: 48000288  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B29C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8316B2A0: 40980010  bge cr6, 0x8316b2b0
	if !ctx.cr[6].lt {
	pc = 0x8316B2B0; continue 'dispatch;
	}
	// 8316B2A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B2A8: 388B85AC  addi r4, r11, -0x7a54
	ctx.r[4].s64 = ctx.r[11].s64 + -31316;
	// 8316B2AC: 48000274  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B2B4: 4BFFF1DD  bl 0x8316a490
	ctx.lr = 0x8316B2B8;
	sub_8316A490(ctx, base);
	// 8316B2B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316B2BC: 40820010  bne 0x8316b2cc
	if !ctx.cr[0].eq {
	pc = 0x8316B2CC; continue 'dispatch;
	}
	// 8316B2C0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B2C4: 388B856C  addi r4, r11, -0x7a94
	ctx.r[4].s64 = ctx.r[11].s64 + -31380;
	// 8316B2C8: 48000258  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B2CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B2D0: 4BFFF219  bl 0x8316a4e8
	ctx.lr = 0x8316B2D4;
	sub_8316A4E8(ctx, base);
	// 8316B2D4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B2D8: 419A0240  beq cr6, 0x8316b518
	if ctx.cr[6].eq {
	pc = 0x8316B518; continue 'dispatch;
	}
	// 8316B2DC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316B2E0: 419A0238  beq cr6, 0x8316b518
	if ctx.cr[6].eq {
	pc = 0x8316B518; continue 'dispatch;
	}
	// 8316B2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B2E8: 4BFFF259  bl 0x8316a540
	ctx.lr = 0x8316B2EC;
	sub_8316A540(ctx, base);
	// 8316B2EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B2F0: 409A0018  bne cr6, 0x8316b308
	if !ctx.cr[6].eq {
	pc = 0x8316B308; continue 'dispatch;
	}
	// 8316B2F4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B2F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B2FC: 388B8518  addi r4, r11, -0x7ae8
	ctx.r[4].s64 = ctx.r[11].s64 + -31464;
	// 8316B300: 4BFF4819  bl 0x8315fb18
	ctx.lr = 0x8316B304;
	sub_8315FB18(ctx, base);
	// 8316B304: 48000238  b 0x8316b53c
	pc = 0x8316B53C; continue 'dispatch;
	// 8316B308: 817F02DC  lwz r11, 0x2dc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 8316B30C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B310: 419A01FC  beq cr6, 0x8316b50c
	if ctx.cr[6].eq {
	pc = 0x8316B50C; continue 'dispatch;
	}
	// 8316B314: 817F02D0  lwz r11, 0x2d0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as u64;
	// 8316B318: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B31C: 419A01F0  beq cr6, 0x8316b50c
	if ctx.cr[6].eq {
	pc = 0x8316B50C; continue 'dispatch;
	}
	// 8316B320: 817F02C4  lwz r11, 0x2c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(708 as u32) ) } as u64;
	// 8316B324: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B328: 409A0010  bne cr6, 0x8316b338
	if !ctx.cr[6].eq {
	pc = 0x8316B338; continue 'dispatch;
	}
	// 8316B32C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B330: 388B84E8  addi r4, r11, -0x7b18
	ctx.r[4].s64 = ctx.r[11].s64 + -31512;
	// 8316B334: 480001EC  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B338: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8316B33C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8316B340: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8316B344: 40990088  ble cr6, 0x8316b3cc
	if !ctx.cr[6].gt {
	pc = 0x8316B3CC; continue 'dispatch;
	}
	// 8316B348: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B34C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8316B350: 7EFDBB78  mr r29, r23
	ctx.r[29].u64 = ctx.r[23].u64;
	// 8316B354: 3B4B8498  addi r26, r11, -0x7b68
	ctx.r[26].s64 = ctx.r[11].s64 + -31592;
	// 8316B358: 3B2A8454  addi r25, r10, -0x7bac
	ctx.r[25].s64 = ctx.r[10].s64 + -31660;
	// 8316B35C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316B360: EBDD0008  ld r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	// 8316B364: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B368: 419A0104  beq cr6, 0x8316b46c
	if ctx.cr[6].eq {
	pc = 0x8316B46C; continue 'dispatch;
	}
	// 8316B36C: 2F3E0000  cmpdi cr6, r30, 0
	ctx.cr[6].compare_i64(ctx.r[30].s64, 0, &mut ctx.xer);
	// 8316B370: 41980108  blt cr6, 0x8316b478
	if ctx.cr[6].lt {
	pc = 0x8316B478; continue 'dispatch;
	}
	// 8316B374: 815F02DC  lwz r10, 0x2dc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 8316B378: 7D2B53D6  divw r9, r11, r10
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8316B37C: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8316B380: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B384: 41820010  beq 0x8316b394
	if ctx.cr[0].eq {
	pc = 0x8316B394; continue 'dispatch;
	}
	// 8316B388: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8316B38C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B390: 4BFF4789  bl 0x8315fb18
	ctx.lr = 0x8316B394;
	sub_8315FB18(ctx, base);
	// 8316B394: E97F02D2  lwa r11, 0x2d0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as i32) as i64;
	// 8316B398: 7D5E5BD2  divd r10, r30, r11
	ctx.r[10].s64 = ctx.r[30].s64 / ctx.r[11].s64;
	// 8316B39C: 7D6A59D2  mulld r11, r10, r11
	ctx.r[11].s64 = ctx.r[10].s64 * ctx.r[11].s64;
	// 8316B3A0: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8316B3A4: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316B3A8: 419A0010  beq cr6, 0x8316b3b8
	if ctx.cr[6].eq {
	pc = 0x8316B3B8; continue 'dispatch;
	}
	// 8316B3AC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316B3B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B3B4: 4BFF4765  bl 0x8315fb18
	ctx.lr = 0x8316B3B8;
	sub_8315FB18(ctx, base);
	// 8316B3B8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8316B3BC: 7F7EDA14  add r27, r30, r27
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 8316B3C0: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 8316B3C4: 7F1CC000  cmpw cr6, r28, r24
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[24].s32, &mut ctx.xer);
	// 8316B3C8: 4198FF94  blt cr6, 0x8316b35c
	if ctx.cr[6].lt {
	pc = 0x8316B35C; continue 'dispatch;
	}
	// 8316B3CC: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316B3D0: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316B3D4: 3BDF01D0  addi r30, r31, 0x1d0
	ctx.r[30].s64 = ctx.r[31].s64 + 464;
	// 8316B3D8: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316B3DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B3E0: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 8316B3E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316B3E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316B3EC: 7F8B502E  lwzx r28, r11, r10
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316B3F0: 4803CDF1  bl 0x831a81e0
	ctx.lr = 0x8316B3F4;
	sub_831A81E0(ctx, base);
	// 8316B3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B3F8: 4BFFF099  bl 0x8316a490
	ctx.lr = 0x8316B3FC;
	sub_8316A490(ctx, base);
	// 8316B3FC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8316B400: 419A0100  beq cr6, 0x8316b500
	if ctx.cr[6].eq {
	pc = 0x8316B500; continue 'dispatch;
	}
	// 8316B404: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8316B408: 419A00F8  beq cr6, 0x8316b500
	if ctx.cr[6].eq {
	pc = 0x8316B500; continue 'dispatch;
	}
	// 8316B40C: E97F0300  ld r11, 0x300(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(768 as u32) ) };
	// 8316B410: E95F02F8  ld r10, 0x2f8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) };
	// 8316B414: E93F02DA  lwa r9, 0x2d8(r31)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as i32) as i64;
	// 8316B418: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316B41C: 7D7D4BD2  divd r11, r29, r9
	ctx.r[11].s64 = ctx.r[29].s64 / ctx.r[9].s64;
	// 8316B420: 7D6B49D2  mulld r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[9].s64;
	// 8316B424: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8316B428: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316B42C: 419A0014  beq cr6, 0x8316b440
	if ctx.cr[6].eq {
	pc = 0x8316B440; continue 'dispatch;
	}
	// 8316B430: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B434: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B438: 388B8428  addi r4, r11, -0x7bd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31704;
	// 8316B43C: 4BFF46DD  bl 0x8315fb18
	ctx.lr = 0x8316B440;
	sub_8315FB18(ctx, base);
	// 8316B440: E95F0308  ld r10, 0x308(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) };
	// 8316B444: E97F0300  ld r11, 0x300(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(768 as u32) ) };
	// 8316B448: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 8316B44C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316B450: 7F3B5800  cmpd cr6, r27, r11
	ctx.cr[6].compare_i64(ctx.r[27].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8316B454: 40980008  bge cr6, 0x8316b45c
	if !ctx.cr[6].lt {
	pc = 0x8316B45C; continue 'dispatch;
	}
	// 8316B458: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8316B45C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316B460: 41990024  bgt cr6, 0x8316b484
	if ctx.cr[6].gt {
	pc = 0x8316B484; continue 'dispatch;
	}
	// 8316B464: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316B468: 4800009C  b 0x8316b504
	pc = 0x8316B504; continue 'dispatch;
	// 8316B46C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B470: 388B8404  addi r4, r11, -0x7bfc
	ctx.r[4].s64 = ctx.r[11].s64 + -31740;
	// 8316B474: 480000AC  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B478: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B47C: 388B83E4  addi r4, r11, -0x7c1c
	ctx.r[4].s64 = ctx.r[11].s64 + -31772;
	// 8316B480: 480000A0  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B484: 817F029C  lwz r11, 0x29c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 8316B488: 395F02A0  addi r10, r31, 0x2a0
	ctx.r[10].s64 = ctx.r[31].s64 + 672;
	// 8316B48C: 393F02E8  addi r9, r31, 0x2e8
	ctx.r[9].s64 = ctx.r[31].s64 + 744;
	// 8316B490: 391F02F0  addi r8, r31, 0x2f0
	ctx.r[8].s64 = ctx.r[31].s64 + 752;
	// 8316B494: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8316B498: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8316B49C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8316B4A0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316B4A4: 80BF024C  lwz r5, 0x24c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 8316B4A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316B4AC: 90BE001C  stw r5, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8316B4B0: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 8316B4B4: 915E0024  stw r10, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8316B4B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316B4BC: 913E0028  stw r9, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8316B4C0: 92FE002C  stw r23, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[23].u32 ) };
	// 8316B4C4: 931E0030  stw r24, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 8316B4C8: FBBE0040  std r29, 0x40(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u64 ) };
	// 8316B4CC: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8316B4D0: F97E0048  std r11, 0x48(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 8316B4D4: 911E0068  stw r8, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 8316B4D8: 817F0298  lwz r11, 0x298(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) } as u64;
	// 8316B4DC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316B4E0: 90FE000C  stw r7, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8316B4E4: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 8316B4E8: E97F02F8  ld r11, 0x2f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) };
	// 8316B4EC: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8316B4F0: F97F02F8  std r11, 0x2f8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[11].u64 ) };
	// 8316B4F4: 90DE0020  stw r6, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8316B4F8: 48004B19  bl 0x83170010
	ctx.lr = 0x8316B4FC;
	sub_83170010(ctx, base);
	// 8316B4FC: 48000040  b 0x8316b53c
	pc = 0x8316B53C; continue 'dispatch;
	// 8316B500: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8316B504: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8316B508: 48000034  b 0x8316b53c
	pc = 0x8316B53C; continue 'dispatch;
	// 8316B50C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B510: 388B83A8  addi r4, r11, -0x7c58
	ctx.r[4].s64 = ctx.r[11].s64 + -31832;
	// 8316B514: 4800000C  b 0x8316b520
	pc = 0x8316B520; continue 'dispatch;
	// 8316B518: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B51C: 388B8368  addi r4, r11, -0x7c98
	ctx.r[4].s64 = ctx.r[11].s64 + -31896;
	// 8316B520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B524: 4BFF45F5  bl 0x8315fb18
	ctx.lr = 0x8316B528;
	sub_8315FB18(ctx, base);
	// 8316B528: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316B52C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B530: 409A000C  bne cr6, 0x8316b53c
	if !ctx.cr[6].eq {
	pc = 0x8316B53C; continue 'dispatch;
	}
	// 8316B534: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316B538: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316B53C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8316B540: 4803CC64  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B548 size=112
    let mut pc: u32 = 0x8316B548;
    'dispatch: loop {
        match pc {
            0x8316B548 => {
    //   block [0x8316B548..0x8316B5B8)
	// 8316B548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316B550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316B554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B55C: 4BFFEF35  bl 0x8316a490
	ctx.lr = 0x8316B560;
	sub_8316A490(ctx, base);
	// 8316B560: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B564: 419A000C  beq cr6, 0x8316b570
	if ctx.cr[6].eq {
	pc = 0x8316B570; continue 'dispatch;
	}
	// 8316B568: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B56C: 48000038  b 0x8316b5a4
	pc = 0x8316B5A4; continue 'dispatch;
	// 8316B570: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316B574: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316B578: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316B57C: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316B580: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B584: 38A98328  addi r5, r9, -0x7cd8
	ctx.r[5].s64 = ctx.r[9].s64 + -31960;
	// 8316B588: 389F0160  addi r4, r31, 0x160
	ctx.r[4].s64 = ctx.r[31].s64 + 352;
	// 8316B58C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316B590: 48004B19  bl 0x831700a8
	ctx.lr = 0x8316B594;
	sub_831700A8(ctx, base);
	// 8316B594: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316B598: 4182000C  beq 0x8316b5a4
	if ctx.cr[0].eq {
	pc = 0x8316B5A4; continue 'dispatch;
	}
	// 8316B59C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316B5A0: 917F0144  stw r11, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 8316B5A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316B5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316B5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316B5B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316B5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B5B8 size=124
    let mut pc: u32 = 0x8316B5B8;
    'dispatch: loop {
        match pc {
            0x8316B5B8 => {
    //   block [0x8316B5B8..0x8316B634)
	// 8316B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316B5C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316B5C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B5C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B5CC: 4BFFEF1D  bl 0x8316a4e8
	ctx.lr = 0x8316B5D0;
	sub_8316A4E8(ctx, base);
	// 8316B5D0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B5D4: 419A000C  beq cr6, 0x8316b5e0
	if ctx.cr[6].eq {
	pc = 0x8316B5E0; continue 'dispatch;
	}
	// 8316B5D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B5DC: 48000044  b 0x8316b620
	pc = 0x8316B620; continue 'dispatch;
	// 8316B5E0: 817F01C4  lwz r11, 0x1c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 8316B5E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B5E8: 409AFFF0  bne cr6, 0x8316b5d8
	if !ctx.cr[6].eq {
	pc = 0x8316B5D8; continue 'dispatch;
	}
	// 8316B5EC: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316B5F0: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316B5F4: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316B5F8: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316B5FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B600: 38A98328  addi r5, r9, -0x7cd8
	ctx.r[5].s64 = ctx.r[9].s64 + -31960;
	// 8316B604: 389F01A0  addi r4, r31, 0x1a0
	ctx.r[4].s64 = ctx.r[31].s64 + 416;
	// 8316B608: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316B60C: 48004A9D  bl 0x831700a8
	ctx.lr = 0x8316B610;
	sub_831700A8(ctx, base);
	// 8316B610: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316B614: 4182000C  beq 0x8316b620
	if ctx.cr[0].eq {
	pc = 0x8316B620; continue 'dispatch;
	}
	// 8316B618: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316B61C: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 8316B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316B62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316B630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B638 size=128
    let mut pc: u32 = 0x8316B638;
    'dispatch: loop {
        match pc {
            0x8316B638 => {
    //   block [0x8316B638..0x8316B6B8)
	// 8316B638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316B640: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316B644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B64C: 4BFFEEF5  bl 0x8316a540
	ctx.lr = 0x8316B650;
	sub_8316A540(ctx, base);
	// 8316B650: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316B654: 409A004C  bne cr6, 0x8316b6a0
	if !ctx.cr[6].eq {
	pc = 0x8316B6A0; continue 'dispatch;
	}
	// 8316B658: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316B65C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316B660: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316B664: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316B668: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B66C: 3BFF01D0  addi r31, r31, 0x1d0
	ctx.r[31].s64 = ctx.r[31].s64 + 464;
	// 8316B670: 38A98328  addi r5, r9, -0x7cd8
	ctx.r[5].s64 = ctx.r[9].s64 + -31960;
	// 8316B674: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316B678: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316B67C: 48004A2D  bl 0x831700a8
	ctx.lr = 0x8316B680;
	sub_831700A8(ctx, base);
	// 8316B680: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316B684: 41820014  beq 0x8316b698
	if ctx.cr[0].eq {
	pc = 0x8316B698; continue 'dispatch;
	}
	// 8316B688: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316B68C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316B690: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8316B694: 48000010  b 0x8316b6a4
	pc = 0x8316B6A4; continue 'dispatch;
	// 8316B698: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316B69C: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8316B6A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B6A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316B6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316B6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316B6B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316B6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B6B8 size=72
    let mut pc: u32 = 0x8316B6B8;
    'dispatch: loop {
        match pc {
            0x8316B6B8 => {
    //   block [0x8316B6B8..0x8316B700)
	// 8316B6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B6BC: 4803CAB1  bl 0x831a816c
	ctx.lr = 0x8316B6C0;
	sub_831A8130(ctx, base);
	// 8316B6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B6C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B6C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316B6CC: 4BFFF1ED  bl 0x8316a8b8
	ctx.lr = 0x8316B6D0;
	sub_8316A8B8(ctx, base);
	// 8316B6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B6D4: 4BFFEDBD  bl 0x8316a490
	ctx.lr = 0x8316B6D8;
	sub_8316A490(ctx, base);
	// 8316B6D8: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316B6DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316B6E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316B6E4: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316B6E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B6EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316B6F0: 4BFFF239  bl 0x8316a928
	ctx.lr = 0x8316B6F4;
	sub_8316A928(ctx, base);
	// 8316B6F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316B6F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316B6FC: 4803CAC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B700 size=72
    let mut pc: u32 = 0x8316B700;
    'dispatch: loop {
        match pc {
            0x8316B700 => {
    //   block [0x8316B700..0x8316B748)
	// 8316B700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B704: 4803CA69  bl 0x831a816c
	ctx.lr = 0x8316B708;
	sub_831A8130(ctx, base);
	// 8316B708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B70C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B710: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316B714: 4BFFF1A5  bl 0x8316a8b8
	ctx.lr = 0x8316B718;
	sub_8316A8B8(ctx, base);
	// 8316B718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B71C: 4BFFEDCD  bl 0x8316a4e8
	ctx.lr = 0x8316B720;
	sub_8316A4E8(ctx, base);
	// 8316B720: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316B724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316B728: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316B72C: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316B730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B734: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316B738: 4BFFF1F1  bl 0x8316a928
	ctx.lr = 0x8316B73C;
	sub_8316A928(ctx, base);
	// 8316B73C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316B740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316B744: 4803CA78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B748 size=72
    let mut pc: u32 = 0x8316B748;
    'dispatch: loop {
        match pc {
            0x8316B748 => {
    //   block [0x8316B748..0x8316B790)
	// 8316B748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B74C: 4803CA21  bl 0x831a816c
	ctx.lr = 0x8316B750;
	sub_831A8130(ctx, base);
	// 8316B750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B758: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316B75C: 4BFFF15D  bl 0x8316a8b8
	ctx.lr = 0x8316B760;
	sub_8316A8B8(ctx, base);
	// 8316B760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B764: 4BFFEDDD  bl 0x8316a540
	ctx.lr = 0x8316B768;
	sub_8316A540(ctx, base);
	// 8316B768: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316B76C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316B770: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316B774: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316B778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B77C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316B780: 4BFFF1A9  bl 0x8316a928
	ctx.lr = 0x8316B784;
	sub_8316A928(ctx, base);
	// 8316B784: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316B788: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316B78C: 4803CA30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B790 size=100
    let mut pc: u32 = 0x8316B790;
    'dispatch: loop {
        match pc {
            0x8316B790 => {
    //   block [0x8316B790..0x8316B7F4)
	// 8316B790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316B798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316B79C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B7A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316B7A4: 4BFFF115  bl 0x8316a8b8
	ctx.lr = 0x8316B7A8;
	sub_8316A8B8(ctx, base);
	// 8316B7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B7AC: 4BFFF17D  bl 0x8316a928
	ctx.lr = 0x8316B7B0;
	sub_8316A928(ctx, base);
	// 8316B7B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B7B4: 4BFFEF05  bl 0x8316a6b8
	ctx.lr = 0x8316B7B8;
	sub_8316A6B8(ctx, base);
	// 8316B7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B7BC: 4BFFF0FD  bl 0x8316a8b8
	ctx.lr = 0x8316B7C0;
	sub_8316A8B8(ctx, base);
	// 8316B7C0: 817F01C8  lwz r11, 0x1c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 8316B7C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316B7C8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8316B7CC: 419A0008  beq cr6, 0x8316b7d4
	if ctx.cr[6].eq {
	pc = 0x8316B7D4; continue 'dispatch;
	}
	// 8316B7D0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316B7D4: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 8316B7D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316B7DC: 4BFFF14D  bl 0x8316a928
	ctx.lr = 0x8316B7E0;
	sub_8316A928(ctx, base);
	// 8316B7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316B7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316B7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316B7EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316B7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316B7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316B7F8 size=632
    let mut pc: u32 = 0x8316B7F8;
    'dispatch: loop {
        match pc {
            0x8316B7F8 => {
    //   block [0x8316B7F8..0x8316BA70)
	// 8316B7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316B7FC: 4803C935  bl 0x831a8130
	ctx.lr = 0x8316B800;
	sub_831A8130(ctx, base);
	// 8316B800: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316B804: 81630204  lwz r11, 0x204(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(516 as u32) ) } as u64;
	// 8316B808: 39E00001  li r15, 1
	ctx.r[15].s64 = 1;
	// 8316B80C: 80C30238  lwz r6, 0x238(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(568 as u32) ) } as u64;
	// 8316B810: 39C00000  li r14, 0
	ctx.r[14].s64 = 0;
	// 8316B814: 814301FC  lwz r10, 0x1fc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 8316B818: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8316B81C: E9030220  ld r8, 0x220(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(544 as u32) ) };
	// 8316B820: 3BE301D0  addi r31, r3, 0x1d0
	ctx.r[31].s64 = ctx.r[3].s64 + 464;
	// 8316B824: E8E30210  ld r7, 0x210(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(528 as u32) ) };
	// 8316B828: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8316B82C: 816301F4  lwz r11, 0x1f4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(500 as u32) ) } as u64;
	// 8316B830: 7DD07378  mr r16, r14
	ctx.r[16].u64 = ctx.r[14].u64;
	// 8316B834: 80A301F8  lwz r5, 0x1f8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(504 as u32) ) } as u64;
	// 8316B838: 7D274214  add r9, r7, r8
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 8316B83C: 91E30244  stw r15, 0x244(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(580 as u32), ctx.r[15].u32 ) };
	// 8316B840: E9060000  ld r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8316B844: 7F694050  subf r27, r9, r8
	ctx.r[27].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 8316B848: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316B84C: 2F3B0000  cmpdi cr6, r27, 0
	ctx.cr[6].compare_i64(ctx.r[27].s64, 0, &mut ctx.xer);
	// 8316B850: EA2A0008  ld r17, 8(r10)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8316B854: 82A301E8  lwz r21, 0x1e8(r3)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(488 as u32) ) } as u64;
	// 8316B858: 824301EC  lwz r18, 0x1ec(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(492 as u32) ) } as u64;
	// 8316B85C: 82850000  lwz r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316B860: EAE30218  ld r23, 0x218(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(536 as u32) ) };
	// 8316B864: 834B0030  lwz r26, 0x30(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316B868: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316B86C: 80CB003C  lwz r6, 0x3c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8316B870: 409901F4  ble cr6, 0x8316ba64
	if !ctx.cr[6].gt {
	pc = 0x8316BA64; continue 'dispatch;
	}
	// 8316B874: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8316B878: E8FF0058  ld r7, 0x58(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 8316B87C: 7CA953D2  divd r5, r9, r10
	ctx.r[5].s64 = ctx.r[9].s64 / ctx.r[10].s64;
	// 8316B880: 7EC78850  subf r22, r7, r17
	ctx.r[22].s64 = ctx.r[17].s64 - ctx.r[7].s64;
	// 8316B884: 7E6551D2  mulld r19, r5, r10
	ctx.r[19].s64 = ctx.r[5].s64 * ctx.r[10].s64;
	// 8316B888: 7F334850  subf r25, r19, r9
	ctx.r[25].s64 = ctx.r[9].s64 - ctx.r[19].s64;
	// 8316B88C: 2F360000  cmpdi cr6, r22, 0
	ctx.cr[6].compare_i64(ctx.r[22].s64, 0, &mut ctx.xer);
	// 8316B890: 409901D4  ble cr6, 0x8316ba64
	if !ctx.cr[6].gt {
	pc = 0x8316BA64; continue 'dispatch;
	}
	// 8316B894: E95F0038  ld r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 8316B898: 54CB003E  slwi r11, r6, 0
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B89C: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 8316B8A0: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8316B8A4: 7CEB00D0  neg r7, r11
	ctx.r[7].s64 = -ctx.r[11].s64;
	// 8316B8A8: 7F0A4214  add r24, r10, r8
	ctx.r[24].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8316B8AC: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 8316B8B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8316B8B4: 7D7D3838  and r29, r11, r7
	ctx.r[29].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 8316B8B8: 7F18E800  cmpw cr6, r24, r29
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8316B8BC: 419A0010  beq cr6, 0x8316b8cc
	if ctx.cr[6].eq {
	pc = 0x8316B8CC; continue 'dispatch;
	}
	// 8316B8C0: 7D78E850  subf r11, r24, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[24].s64;
	// 8316B8C4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8316B8C8: 7D2BB050  subf r9, r11, r22
	ctx.r[9].s64 = ctx.r[22].s64 - ctx.r[11].s64;
	// 8316B8CC: 7F5E07B4  extsw r30, r26
	ctx.r[30].s64 = ctx.r[26].s32 as i64;
	// 8316B8D0: 7D69F3D2  divd r11, r9, r30
	ctx.r[11].s64 = ctx.r[9].s64 / ctx.r[30].s64;
	// 8316B8D4: 7F8BF1D2  mulld r28, r11, r30
	ctx.r[28].s64 = ctx.r[11].s64 * ctx.r[30].s64;
	// 8316B8D8: 7D79E050  subf r11, r25, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[25].s64;
	// 8316B8DC: 7F2BF000  cmpd cr6, r11, r30
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[30].s64, &mut ctx.xer);
	// 8316B8E0: 40980050  bge cr6, 0x8316b930
	if !ctx.cr[6].lt {
	pc = 0x8316B930; continue 'dispatch;
	}
	// 8316B8E4: 397AFFFF  addi r11, r26, -1
	ctx.r[11].s64 = ctx.r[26].s64 + -1;
	// 8316B8E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8316B8EC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8316B8F0: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 8316B8F4: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8316B8F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316B8FC: 7D6BF3D2  divd r11, r11, r30
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[30].s64;
	// 8316B900: 7F8BF1D2  mulld r28, r11, r30
	ctx.r[28].s64 = ctx.r[11].s64 * ctx.r[30].s64;
	// 8316B904: 7F8407B4  extsw r4, r28
	ctx.r[4].s64 = ctx.r[28].s32 as i64;
	// 8316B908: 4BFF3D99  bl 0x8315f6a0
	ctx.lr = 0x8316B90C;
	sub_8315F6A0(ctx, base);
	// 8316B90C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8316B910: 4082001C  bne 0x8316b92c
	if !ctx.cr[0].eq {
	pc = 0x8316B92C; continue 'dispatch;
	}
	// 8316B914: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316B918: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316B91C: 388B85F8  addi r4, r11, -0x7a08
	ctx.r[4].s64 = ctx.r[11].s64 + -31240;
	// 8316B920: 4BFF41F9  bl 0x8315fb18
	ctx.lr = 0x8316B924;
	sub_8315FB18(ctx, base);
	// 8316B924: 91DF0074  stw r14, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[14].u32 ) };
	// 8316B928: 48000140  b 0x8316ba68
	pc = 0x8316BA68; continue 'dispatch;
	// 8316B92C: 7DF07B78  mr r16, r15
	ctx.r[16].u64 = ctx.r[15].u64;
	// 8316B930: 7D79DA14  add r11, r25, r27
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[27].u64;
	// 8316B934: 7F2BE000  cmpd cr6, r11, r28
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[28].s64, &mut ctx.xer);
	// 8316B938: 40980008  bge cr6, 0x8316b940
	if !ctx.cr[6].lt {
	pc = 0x8316B940; continue 'dispatch;
	}
	// 8316B93C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8316B940: 397AFFFF  addi r11, r26, -1
	ctx.r[11].s64 = ctx.r[26].s64 + -1;
	// 8316B944: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8316B948: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8316B94C: 7D6BF3D2  divd r11, r11, r30
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[30].s64;
	// 8316B950: 7FCBF1D2  mulld r30, r11, r30
	ctx.r[30].s64 = ctx.r[11].s64 * ctx.r[30].s64;
	// 8316B954: 7F3EB800  cmpd cr6, r30, r23
	ctx.cr[6].compare_i64(ctx.r[30].s64, ctx.r[23].s64, &mut ctx.xer);
	// 8316B958: 41980008  blt cr6, 0x8316b960
	if ctx.cr[6].lt {
	pc = 0x8316B960; continue 'dispatch;
	}
	// 8316B95C: 7EFEBB78  mr r30, r23
	ctx.r[30].u64 = ctx.r[23].u64;
	// 8316B960: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316B964: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8316B968: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8316B96C: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 8316B970: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8316B974: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8316B978: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316B97C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316B980: 4E800421  bctrl
	ctx.lr = 0x8316B984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316B984: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8316B988: 2F3B0000  cmpdi cr6, r27, 0
	ctx.cr[6].compare_i64(ctx.r[27].s64, 0, &mut ctx.xer);
	// 8316B98C: 4098001C  bge cr6, 0x8316b9a8
	if !ctx.cr[6].lt {
	pc = 0x8316B9A8; continue 'dispatch;
	}
	// 8316B990: 2F100000  cmpwi cr6, r16, 0
	ctx.cr[6].compare_i32(ctx.r[16].s32, 0, &mut ctx.xer);
	// 8316B994: 419AFF90  beq cr6, 0x8316b924
	if ctx.cr[6].eq {
	pc = 0x8316B924; continue 'dispatch;
	}
	// 8316B998: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316B99C: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 8316B9A0: 4BFF3DC9  bl 0x8315f768
	ctx.lr = 0x8316B9A4;
	sub_8315F768(ctx, base);
	// 8316B9A4: 4BFFFF80  b 0x8316b924
	pc = 0x8316B924; continue 'dispatch;
	// 8316B9A8: 7F3ED800  cmpd cr6, r30, r27
	ctx.cr[6].compare_i64(ctx.r[30].s64, ctx.r[27].s64, &mut ctx.xer);
	// 8316B9AC: 4099001C  ble cr6, 0x8316b9c8
	if !ctx.cr[6].gt {
	pc = 0x8316B9C8; continue 'dispatch;
	}
	// 8316B9B0: 576B003E  slwi r11, r27, 0
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316B9B4: 57CA003E  slwi r10, r30, 0
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8316B9B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316B9BC: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8316B9C0: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8316B9C4: 4803C81D  bl 0x831a81e0
	ctx.lr = 0x8316B9C8;
	sub_831A81E0(ctx, base);
	// 8316B9C8: 7F99D850  subf r28, r25, r27
	ctx.r[28].s64 = ctx.r[27].s64 - ctx.r[25].s64;
	// 8316B9CC: 7F18E840  cmplw cr6, r24, r29
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8316B9D0: 409A000C  bne cr6, 0x8316b9dc
	if !ctx.cr[6].eq {
	pc = 0x8316B9DC; continue 'dispatch;
	}
	// 8316B9D4: 2F390000  cmpdi cr6, r25, 0
	ctx.cr[6].compare_i64(ctx.r[25].s64, 0, &mut ctx.xer);
	// 8316B9D8: 4099003C  ble cr6, 0x8316ba14
	if !ctx.cr[6].gt {
	pc = 0x8316BA14; continue 'dispatch;
	}
	// 8316B9DC: 7F36E000  cmpd cr6, r22, r28
	ctx.cr[6].compare_i64(ctx.r[22].s64, ctx.r[28].s64, &mut ctx.xer);
	// 8316B9E0: 40980008  bge cr6, 0x8316b9e8
	if !ctx.cr[6].lt {
	pc = 0x8316B9E8; continue 'dispatch;
	}
	// 8316B9E4: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 8316B9E8: 7DCB7378  mr r11, r14
	ctx.r[11].u64 = ctx.r[14].u64;
	// 8316B9EC: 2F3C0000  cmpdi cr6, r28, 0
	ctx.cr[6].compare_i64(ctx.r[28].s64, 0, &mut ctx.xer);
	// 8316B9F0: 40990024  ble cr6, 0x8316ba14
	if !ctx.cr[6].gt {
	pc = 0x8316BA14; continue 'dispatch;
	}
	// 8316B9F4: 7F2907B4  extsw r9, r25
	ctx.r[9].s64 = ctx.r[25].s32 as i64;
	// 8316B9F8: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 8316B9FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316BA00: 7D095214  add r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8316BA04: 7F2BE000  cmpd cr6, r11, r28
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[28].s64, &mut ctx.xer);
	// 8316BA08: 7D08E8AE  lbzx r8, r8, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8316BA0C: 7D0AC1AE  stbx r8, r10, r24
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32), ctx.r[8].u8) };
	// 8316BA10: 4198FFE8  blt cr6, 0x8316b9f8
	if ctx.cr[6].lt {
	pc = 0x8316B9F8; continue 'dispatch;
	}
	// 8316BA14: 2F100000  cmpwi cr6, r16, 0
	ctx.cr[6].compare_i32(ctx.r[16].s32, 0, &mut ctx.xer);
	// 8316BA18: 419A0010  beq cr6, 0x8316ba28
	if ctx.cr[6].eq {
	pc = 0x8316BA28; continue 'dispatch;
	}
	// 8316BA1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316BA20: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 8316BA24: 4BFF3D45  bl 0x8315f768
	ctx.lr = 0x8316BA28;
	sub_8315F768(ctx, base);
	// 8316BA28: E95F0038  ld r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 8316BA2C: E91F0050  ld r8, 0x50(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	// 8316BA30: 7D395050  subf r9, r25, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[25].s64;
	// 8316BA34: E97F0058  ld r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 8316BA38: 7D594050  subf r10, r25, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[25].s64;
	// 8316BA3C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8316BA40: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 8316BA44: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8316BA48: F97F0058  std r11, 0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8316BA4C: F93F0038  std r9, 0x38(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u64 ) };
	// 8316BA50: 7F2B8800  cmpd cr6, r11, r17
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[17].s64, &mut ctx.xer);
	// 8316BA54: F95F0050  std r10, 0x50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8316BA58: 4098000C  bge cr6, 0x8316ba64
	if !ctx.cr[6].lt {
	pc = 0x8316BA64; continue 'dispatch;
	}
	// 8316BA5C: 7F3ED800  cmpd cr6, r30, r27
	ctx.cr[6].compare_i64(ctx.r[30].s64, ctx.r[27].s64, &mut ctx.xer);
	// 8316BA60: 419A0008  beq cr6, 0x8316ba68
	if ctx.cr[6].eq {
	pc = 0x8316BA68; continue 'dispatch;
	}
	// 8316BA64: 91FF0070  stw r15, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[15].u32 ) };
	// 8316BA68: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8316BA6C: 4803C714  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BA70 size=188
    let mut pc: u32 = 0x8316BA70;
    'dispatch: loop {
        match pc {
            0x8316BA70 => {
    //   block [0x8316BA70..0x8316BB2C)
	// 8316BA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316BA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316BA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316BA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BA84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BA88: 817F019C  lwz r11, 0x19c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(412 as u32) ) } as u64;
	// 8316BA8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BA90: 409A0010  bne cr6, 0x8316baa0
	if !ctx.cr[6].eq {
	pc = 0x8316BAA0; continue 'dispatch;
	}
	// 8316BA94: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8316BA98: 917F0144  stw r11, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 8316BA9C: 48000078  b 0x8316bb14
	pc = 0x8316BB14; continue 'dispatch;
	// 8316BAA0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316BAA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BAA8: 917F0144  stw r11, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 8316BAAC: E97F0190  ld r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) };
	// 8316BAB0: F97F02F0  std r11, 0x2f0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[11].u64 ) };
	// 8316BAB4: 4BFFEA35  bl 0x8316a4e8
	ctx.lr = 0x8316BAB8;
	sub_8316A4E8(ctx, base);
	// 8316BAB8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316BABC: 419A0058  beq cr6, 0x8316bb14
	if ctx.cr[6].eq {
	pc = 0x8316BB14; continue 'dispatch;
	}
	// 8316BAC0: 3BDF0120  addi r30, r31, 0x120
	ctx.r[30].s64 = ctx.r[31].s64 + 288;
	// 8316BAC4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8316BAC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316BACC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316BAD0: 4803C711  bl 0x831a81e0
	ctx.lr = 0x8316BAD4;
	sub_831A81E0(ctx, base);
	// 8316BAD4: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 8316BAD8: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316BADC: 917F0128  stw r11, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 8316BAE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316BAE4: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 8316BAE8: 817F011C  lwz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 8316BAEC: 917F012C  stw r11, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 8316BAF0: 817F02E4  lwz r11, 0x2e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8316BAF4: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 8316BAF8: 817F02E8  lwz r11, 0x2e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(744 as u32) ) } as u64;
	// 8316BAFC: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 8316BB00: E97F02F0  ld r11, 0x2f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) };
	// 8316BB04: F97F0138  std r11, 0x138(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u64 ) };
	// 8316BB08: 48004951  bl 0x83170458
	ctx.lr = 0x8316BB0C;
	sub_83170458(ctx, base);
	// 8316BB0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316BB10: 917F0140  stw r11, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 8316BB14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316BB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316BB20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316BB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316BB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BB30 size=72
    let mut pc: u32 = 0x8316BB30;
    'dispatch: loop {
        match pc {
            0x8316BB30 => {
    //   block [0x8316BB30..0x8316BB78)
	// 8316BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BB34: 4803C639  bl 0x831a816c
	ctx.lr = 0x8316BB38;
	sub_831A8130(ctx, base);
	// 8316BB38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BB3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BB40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316BB44: 4BFFED75  bl 0x8316a8b8
	ctx.lr = 0x8316BB48;
	sub_8316A8B8(ctx, base);
	// 8316BB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BB4C: 4BFFECAD  bl 0x8316a7f8
	ctx.lr = 0x8316BB50;
	sub_8316A7F8(ctx, base);
	// 8316BB50: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316BB58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316BB5C: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316BB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BB64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316BB68: 4BFFEDC1  bl 0x8316a928
	ctx.lr = 0x8316BB6C;
	sub_8316A928(ctx, base);
	// 8316BB6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316BB70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BB74: 4803C648  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BB78 size=128
    let mut pc: u32 = 0x8316BB78;
    'dispatch: loop {
        match pc {
            0x8316BB78 => {
    //   block [0x8316BB78..0x8316BBF8)
	// 8316BB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316BB80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316BB84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BB88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BB8C: 4BFFE905  bl 0x8316a490
	ctx.lr = 0x8316BB90;
	sub_8316A490(ctx, base);
	// 8316BB90: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316BB94: 419A0030  beq cr6, 0x8316bbc4
	if ctx.cr[6].eq {
	pc = 0x8316BBC4; continue 'dispatch;
	}
	// 8316BB98: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316BB9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BBA0: 388B8648  addi r4, r11, -0x79b8
	ctx.r[4].s64 = ctx.r[11].s64 + -31160;
	// 8316BBA4: 4BFF3F75  bl 0x8315fb18
	ctx.lr = 0x8316BBA8;
	sub_8315FB18(ctx, base);
	// 8316BBA8: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BBAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BBB0: 409A000C  bne cr6, 0x8316bbbc
	if !ctx.cr[6].eq {
	pc = 0x8316BBBC; continue 'dispatch;
	}
	// 8316BBB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316BBB8: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316BBBC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316BBC0: 48000024  b 0x8316bbe4
	pc = 0x8316BBE4; continue 'dispatch;
	// 8316BBC4: E93F0300  ld r9, 0x300(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(768 as u32) ) };
	// 8316BBC8: E95F0308  ld r10, 0x308(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) };
	// 8316BBCC: E97F02F0  ld r11, 0x2f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) };
	// 8316BBD0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8316BBD4: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8316BBD8: 40980008  bge cr6, 0x8316bbe0
	if !ctx.cr[6].lt {
	pc = 0x8316BBE0; continue 'dispatch;
	}
	// 8316BBDC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8316BBE0: 7C695850  subf r3, r9, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8316BBE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316BBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316BBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316BBF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316BBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BBF8 size=100
    let mut pc: u32 = 0x8316BBF8;
    'dispatch: loop {
        match pc {
            0x8316BBF8 => {
    //   block [0x8316BBF8..0x8316BC5C)
	// 8316BBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316BC00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316BC04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BC08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BC0C: 4BFFE885  bl 0x8316a490
	ctx.lr = 0x8316BC10;
	sub_8316A490(ctx, base);
	// 8316BC10: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316BC14: 419A0030  beq cr6, 0x8316bc44
	if ctx.cr[6].eq {
	pc = 0x8316BC44; continue 'dispatch;
	}
	// 8316BC18: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316BC1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BC20: 388B8690  addi r4, r11, -0x7970
	ctx.r[4].s64 = ctx.r[11].s64 + -31088;
	// 8316BC24: 4BFF3EF5  bl 0x8315fb18
	ctx.lr = 0x8316BC28;
	sub_8315FB18(ctx, base);
	// 8316BC28: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BC30: 409A000C  bne cr6, 0x8316bc3c
	if !ctx.cr[6].eq {
	pc = 0x8316BC3C; continue 'dispatch;
	}
	// 8316BC34: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316BC38: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316BC3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BC40: 48000008  b 0x8316bc48
	pc = 0x8316BC48; continue 'dispatch;
	// 8316BC44: 807F02E8  lwz r3, 0x2e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(744 as u32) ) } as u64;
	// 8316BC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316BC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316BC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316BC54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316BC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BC60 size=164
    let mut pc: u32 = 0x8316BC60;
    'dispatch: loop {
        match pc {
            0x8316BC60 => {
    //   block [0x8316BC60..0x8316BD04)
	// 8316BC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BC64: 4803C505  bl 0x831a8168
	ctx.lr = 0x8316BC68;
	sub_831A8130(ctx, base);
	// 8316BC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BC6C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8316BC70: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8316BC74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316BC78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BC7C: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 8316BC80: EBBC02F8  ld r29, 0x2f8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(760 as u32) ) };
	// 8316BC84: 409A0040  bne cr6, 0x8316bcc4
	if !ctx.cr[6].eq {
	pc = 0x8316BCC4; continue 'dispatch;
	}
	// 8316BC88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316BC8C: 4BFFFEED  bl 0x8316bb78
	ctx.lr = 0x8316BC90;
	sub_8316BB78(ctx, base);
	// 8316BC90: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316BC94: 40980030  bge cr6, 0x8316bcc4
	if !ctx.cr[6].lt {
	pc = 0x8316BCC4; continue 'dispatch;
	}
	// 8316BC98: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316BC9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BCA0: 388B86D8  addi r4, r11, -0x7928
	ctx.r[4].s64 = ctx.r[11].s64 + -31016;
	// 8316BCA4: 4BFF3E75  bl 0x8315fb18
	ctx.lr = 0x8316BCA8;
	sub_8315FB18(ctx, base);
	// 8316BCA8: 817C0294  lwz r11, 0x294(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BCAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BCB0: 409A000C  bne cr6, 0x8316bcbc
	if !ctx.cr[6].eq {
	pc = 0x8316BCBC; continue 'dispatch;
	}
	// 8316BCB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316BCB8: 917C0294  stw r11, 0x294(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316BCBC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316BCC0: 4800003C  b 0x8316bcfc
	pc = 0x8316BCFC; continue 'dispatch;
	// 8316BCC4: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 8316BCC8: 41980020  blt cr6, 0x8316bce8
	if ctx.cr[6].lt {
	pc = 0x8316BCE8; continue 'dispatch;
	}
	// 8316BCCC: 419A0014  beq cr6, 0x8316bce0
	if ctx.cr[6].eq {
	pc = 0x8316BCE0; continue 'dispatch;
	}
	// 8316BCD0: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 8316BCD4: 40980020  bge cr6, 0x8316bcf4
	if !ctx.cr[6].lt {
	pc = 0x8316BCF4; continue 'dispatch;
	}
	// 8316BCD8: 7C63F214  add r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 8316BCDC: 48000010  b 0x8316bcec
	pc = 0x8316BCEC; continue 'dispatch;
	// 8316BCE0: 7C7DF214  add r3, r29, r30
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 8316BCE4: 48000008  b 0x8316bcec
	pc = 0x8316BCEC; continue 'dispatch;
	// 8316BCE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316BCEC: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316BCF0: 40980008  bge cr6, 0x8316bcf8
	if !ctx.cr[6].lt {
	pc = 0x8316BCF8; continue 'dispatch;
	}
	// 8316BCF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BCF8: F87C02F8  std r3, 0x2f8(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(760 as u32), ctx.r[3].u64 ) };
	// 8316BCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316BD00: 4803C4B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BD08 size=64
    let mut pc: u32 = 0x8316BD08;
    'dispatch: loop {
        match pc {
            0x8316BD08 => {
    //   block [0x8316BD08..0x8316BD48)
	// 8316BD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BD0C: 4803C461  bl 0x831a816c
	ctx.lr = 0x8316BD10;
	sub_831A8130(ctx, base);
	// 8316BD10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BD14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BD18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316BD1C: 4BFFEB9D  bl 0x8316a8b8
	ctx.lr = 0x8316BD20;
	sub_8316A8B8(ctx, base);
	// 8316BD20: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BD24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316BD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BD2C: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316BD30: EBBF02F8  ld r29, 0x2f8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) };
	// 8316BD34: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316BD38: 4BFFEBF1  bl 0x8316a928
	ctx.lr = 0x8316BD3C;
	sub_8316A928(ctx, base);
	// 8316BD3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316BD40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BD44: 4803C478  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BD48 size=96
    let mut pc: u32 = 0x8316BD48;
    'dispatch: loop {
        match pc {
            0x8316BD48 => {
    //   block [0x8316BD48..0x8316BDA8)
	// 8316BD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BD4C: 4803C41D  bl 0x831a8168
	ctx.lr = 0x8316BD50;
	sub_831A8130(ctx, base);
	// 8316BD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BD54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BD58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316BD5C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316BD60: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316BD64: 4BFFEB55  bl 0x8316a8b8
	ctx.lr = 0x8316BD68;
	sub_8316A8B8(ctx, base);
	// 8316BD68: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316BD6C: 419A000C  beq cr6, 0x8316bd78
	if ctx.cr[6].eq {
	pc = 0x8316BD78; continue 'dispatch;
	}
	// 8316BD70: E97F0300  ld r11, 0x300(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(768 as u32) ) };
	// 8316BD74: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8316BD78: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8316BD7C: 419A000C  beq cr6, 0x8316bd88
	if ctx.cr[6].eq {
	pc = 0x8316BD88; continue 'dispatch;
	}
	// 8316BD80: E97F0308  ld r11, 0x308(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) };
	// 8316BD84: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8316BD88: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316BD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BD94: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316BD98: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316BD9C: 4BFFEB8D  bl 0x8316a928
	ctx.lr = 0x8316BDA0;
	sub_8316A928(ctx, base);
	// 8316BDA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316BDA4: 4803C414  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BDA8 size=144
    let mut pc: u32 = 0x8316BDA8;
    'dispatch: loop {
        match pc {
            0x8316BDA8 => {
    //   block [0x8316BDA8..0x8316BE38)
	// 8316BDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316BDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316BDB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BDB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BDBC: 2F240000  cmpdi cr6, r4, 0
	ctx.cr[6].compare_i64(ctx.r[4].s64, 0, &mut ctx.xer);
	// 8316BDC0: 41980040  blt cr6, 0x8316be00
	if ctx.cr[6].lt {
	pc = 0x8316BE00; continue 'dispatch;
	}
	// 8316BDC4: 2F250000  cmpdi cr6, r5, 0
	ctx.cr[6].compare_i64(ctx.r[5].s64, 0, &mut ctx.xer);
	// 8316BDC8: 41980038  blt cr6, 0x8316be00
	if ctx.cr[6].lt {
	pc = 0x8316BE00; continue 'dispatch;
	}
	// 8316BDCC: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 8316BDD0: 7F2B2000  cmpd cr6, r11, r4
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[4].s64, &mut ctx.xer);
	// 8316BDD4: 41980020  blt cr6, 0x8316bdf4
	if ctx.cr[6].lt {
	pc = 0x8316BDF4; continue 'dispatch;
	}
	// 8316BDD8: 7F2B2800  cmpd cr6, r11, r5
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[5].s64, &mut ctx.xer);
	// 8316BDDC: 41980018  blt cr6, 0x8316bdf4
	if ctx.cr[6].lt {
	pc = 0x8316BDF4; continue 'dispatch;
	}
	// 8316BDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316BDE4: F89F0300  std r4, 0x300(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[4].u64 ) };
	// 8316BDE8: F8BF0308  std r5, 0x308(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[5].u64 ) };
	// 8316BDEC: F97F02F8  std r11, 0x2f8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[11].u64 ) };
	// 8316BDF0: 48000034  b 0x8316be24
	pc = 0x8316BE24; continue 'dispatch;
	// 8316BDF4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316BDF8: 388B8754  addi r4, r11, -0x78ac
	ctx.r[4].s64 = ctx.r[11].s64 + -30892;
	// 8316BDFC: 4800000C  b 0x8316be08
	pc = 0x8316BE08; continue 'dispatch;
	// 8316BE00: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316BE04: 388B8720  addi r4, r11, -0x78e0
	ctx.r[4].s64 = ctx.r[11].s64 + -30944;
	// 8316BE08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316BE0C: 4BFF3D0D  bl 0x8315fb18
	ctx.lr = 0x8316BE10;
	sub_8315FB18(ctx, base);
	// 8316BE10: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BE14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BE18: 409A000C  bne cr6, 0x8316be24
	if !ctx.cr[6].eq {
	pc = 0x8316BE24; continue 'dispatch;
	}
	// 8316BE1C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316BE20: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316BE24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316BE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316BE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316BE30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316BE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BE38 size=72
    let mut pc: u32 = 0x8316BE38;
    'dispatch: loop {
        match pc {
            0x8316BE38 => {
    //   block [0x8316BE38..0x8316BE80)
	// 8316BE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BE3C: 4803C331  bl 0x831a816c
	ctx.lr = 0x8316BE40;
	sub_831A8130(ctx, base);
	// 8316BE40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BE44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316BE48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316BE4C: 4BFFEA6D  bl 0x8316a8b8
	ctx.lr = 0x8316BE50;
	sub_8316A8B8(ctx, base);
	// 8316BE50: E97E0228  ld r11, 0x228(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(552 as u32) ) };
	// 8316BE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8316BE58: E95E0230  ld r10, 0x230(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(560 as u32) ) };
	// 8316BE5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316BE60: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316BE64: 817E0294  lwz r11, 0x294(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BE68: 913E0294  stw r9, 0x294(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(660 as u32), ctx.r[9].u32 ) };
	// 8316BE6C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316BE70: 4BFFEAB9  bl 0x8316a928
	ctx.lr = 0x8316BE74;
	sub_8316A928(ctx, base);
	// 8316BE74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BE7C: 4803C340  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BE80 size=72
    let mut pc: u32 = 0x8316BE80;
    'dispatch: loop {
        match pc {
            0x8316BE80 => {
    //   block [0x8316BE80..0x8316BEC8)
	// 8316BE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BE84: 4803C2E9  bl 0x831a816c
	ctx.lr = 0x8316BE88;
	sub_831A8130(ctx, base);
	// 8316BE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BE8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BE90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316BE94: 4BFFEA25  bl 0x8316a8b8
	ctx.lr = 0x8316BE98;
	sub_8316A8B8(ctx, base);
	// 8316BE98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BE9C: 4BFFE9B5  bl 0x8316a850
	ctx.lr = 0x8316BEA0;
	sub_8316A850(ctx, base);
	// 8316BEA0: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316BEA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316BEA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316BEAC: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316BEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BEB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316BEB8: 4BFFEA71  bl 0x8316a928
	ctx.lr = 0x8316BEBC;
	sub_8316A928(ctx, base);
	// 8316BEBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316BEC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BEC4: 4803C2F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BEC8 size=188
    let mut pc: u32 = 0x8316BEC8;
    'dispatch: loop {
        match pc {
            0x8316BEC8 => {
    //   block [0x8316BEC8..0x8316BF84)
	// 8316BEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316BED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316BED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316BED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BEE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316BEE4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316BEE8: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316BEEC: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 8316BEF0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316BEF4: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316BEF8: 48000014  b 0x8316bf0c
	pc = 0x8316BF0C; continue 'dispatch;
	// 8316BEFC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8316BF00: 4BA609F9  bl 0x82bcc8f8
	ctx.lr = 0x8316BF04;
	sub_82BCC8F8(ctx, base);
	// 8316BF04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316BF08: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316BF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BF10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316BF14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316BF18: 4E800421  bctrl
	ctx.lr = 0x8316BF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316BF1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316BF20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BF24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316BF28: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8316BF2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316BF30: 4E800421  bctrl
	ctx.lr = 0x8316BF34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316BF34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316BF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BF3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316BF40: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316BF44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316BF48: 4E800421  bctrl
	ctx.lr = 0x8316BF4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316BF4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316BF50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316BF54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316BF58: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8316BF5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316BF60: 4E800421  bctrl
	ctx.lr = 0x8316BF64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316BF64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316BF68: 4182FF94  beq 0x8316befc
	if ctx.cr[0].eq {
	pc = 0x8316BEFC; continue 'dispatch;
	}
	// 8316BF6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316BF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316BF78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316BF7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316BF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316BF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316BF88 size=152
    let mut pc: u32 = 0x8316BF88;
    'dispatch: loop {
        match pc {
            0x8316BF88 => {
    //   block [0x8316BF88..0x8316C020)
	// 8316BF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316BF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316BF90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316BF94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316BF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316BF9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316BFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316BFA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316BFA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316BFAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316BFB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316BFB4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316BFB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316BFBC: 4E800421  bctrl
	ctx.lr = 0x8316BFC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316BFC0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316BFC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BFC8: 419A0034  beq cr6, 0x8316bffc
	if ctx.cr[6].eq {
	pc = 0x8316BFFC; continue 'dispatch;
	}
	// 8316BFCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316BFD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316BFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316BFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316BFDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316BFE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316BFE4: 4E800020  blr
	return;
	// 8316BFE8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316BFEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316BFF0: 409AFFDC  bne cr6, 0x8316bfcc
	if !ctx.cr[6].eq {
	pc = 0x8316BFCC; continue 'dispatch;
	}
	// 8316BFF4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8316BFF8: 4BA60901  bl 0x82bcc8f8
	ctx.lr = 0x8316BFFC;
	sub_82BCC8F8(ctx, base);
	// 8316BFFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C004: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316C008: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8316C00C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C010: 4E800421  bctrl
	ctx.lr = 0x8316C014;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C014: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316C018: 419AFFD0  beq cr6, 0x8316bfe8
	if ctx.cr[6].eq {
	pc = 0x8316BFE8; continue 'dispatch;
	}
	// 8316C01C: 4BFFFFB4  b 0x8316bfd0
	pc = 0x8316BFD0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C020 size=200
    let mut pc: u32 = 0x8316C020;
    'dispatch: loop {
        match pc {
            0x8316C020 => {
    //   block [0x8316C020..0x8316C0E8)
	// 8316C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C024: 4803C149  bl 0x831a816c
	ctx.lr = 0x8316C028;
	sub_831A8130(ctx, base);
	// 8316C028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C02C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316C030: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316C034: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8316C038: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316C03C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8316C040: 409A0020  bne cr6, 0x8316c060
	if !ctx.cr[6].eq {
	pc = 0x8316C060; continue 'dispatch;
	}
	// 8316C044: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C04C: 388B87B8  addi r4, r11, -0x7848
	ctx.r[4].s64 = ctx.r[11].s64 + -30792;
	// 8316C050: 4BFF3AC9  bl 0x8315fb18
	ctx.lr = 0x8316C054;
	sub_8315FB18(ctx, base);
	// 8316C054: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8316C058: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C05C: 48000080  b 0x8316c0dc
	pc = 0x8316C0DC; continue 'dispatch;
	// 8316C060: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C064: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8316C068: 38AB87A4  addi r5, r11, -0x785c
	ctx.r[5].s64 = ctx.r[11].s64 + -30812;
	// 8316C06C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C070: 38600310  li r3, 0x310
	ctx.r[3].s64 = 784;
	// 8316C074: 4BFF3C85  bl 0x8315fcf8
	ctx.lr = 0x8316C078;
	sub_8315FCF8(ctx, base);
	// 8316C078: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316C07C: 4182000C  beq 0x8316c088
	if ctx.cr[0].eq {
	pc = 0x8316C088; continue 'dispatch;
	}
	// 8316C080: 4BFFE919  bl 0x8316a998
	ctx.lr = 0x8316C084;
	sub_8316A998(ctx, base);
	// 8316C084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C088: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316C08C: 409A001C  bne cr6, 0x8316c0a8
	if !ctx.cr[6].eq {
	pc = 0x8316C0A8; continue 'dispatch;
	}
	// 8316C090: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C094: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C098: 388B877C  addi r4, r11, -0x7884
	ctx.r[4].s64 = ctx.r[11].s64 + -30852;
	// 8316C09C: 4BFF3A7D  bl 0x8315fb18
	ctx.lr = 0x8316C0A0;
	sub_8315FB18(ctx, base);
	// 8316C0A0: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 8316C0A4: 4BFFFFB4  b 0x8316c058
	pc = 0x8316C058; continue 'dispatch;
	// 8316C0A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C0AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C0B0: 4BFFEAA1  bl 0x8316ab50
	ctx.lr = 0x8316C0B4;
	sub_8316AB50(ctx, base);
	// 8316C0B4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316C0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C0BC: 40820024  bne 0x8316c0e0
	if !ctx.cr[0].eq {
	pc = 0x8316C0E0; continue 'dispatch;
	}
	// 8316C0C0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316C0C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8316C0C8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C0CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C0D0: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316C0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C0D8: 4E800421  bctrl
	ctx.lr = 0x8316C0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C0DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C0E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C0E4: 4803C0D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C0E8 size=160
    let mut pc: u32 = 0x8316C0E8;
    'dispatch: loop {
        match pc {
            0x8316C0E8 => {
    //   block [0x8316C0E8..0x8316C188)
	// 8316C0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316C0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C104: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C108: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C10C: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 8316C110: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C114: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C118: 816A0068  lwz r11, 0x68(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316C11C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C120: 4E800421  bctrl
	ctx.lr = 0x8316C124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C124: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C12C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C130: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8316C134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C138: 4E800421  bctrl
	ctx.lr = 0x8316C13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C13C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316C140: 41820010  beq 0x8316c150
	if ctx.cr[0].eq {
	pc = 0x8316C150; continue 'dispatch;
	}
	// 8316C144: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C14C: 4BFFFE3D  bl 0x8316bf88
	ctx.lr = 0x8316C150;
	sub_8316BF88(ctx, base);
	// 8316C150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C154: 4BFFE27D  bl 0x8316a3d0
	ctx.lr = 0x8316C158;
	sub_8316A3D0(ctx, base);
	// 8316C158: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C15C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8316C160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C164: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316C168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C16C: 4E800421  bctrl
	ctx.lr = 0x8316C170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C170: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316C178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316C17C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316C180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316C184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C188 size=88
    let mut pc: u32 = 0x8316C188;
    'dispatch: loop {
        match pc {
            0x8316C188 => {
    //   block [0x8316C188..0x8316C1E0)
	// 8316C188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C18C: 4803BFD9  bl 0x831a8164
	ctx.lr = 0x8316C190;
	sub_831A8130(ctx, base);
	// 8316C190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C198: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C19C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316C1A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316C1A4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8316C1A8: 4BFFE711  bl 0x8316a8b8
	ctx.lr = 0x8316C1AC;
	sub_8316A8B8(ctx, base);
	// 8316C1AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C1B0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8316C1B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316C1B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C1BC: 4BFFEB95  bl 0x8316ad50
	ctx.lr = 0x8316C1C0;
	sub_8316AD50(ctx, base);
	// 8316C1C0: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C1C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C1CC: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316C1D0: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C1D4: 4BFFE755  bl 0x8316a928
	ctx.lr = 0x8316C1D8;
	sub_8316A928(ctx, base);
	// 8316C1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316C1DC: 4803BFD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C1E0 size=416
    let mut pc: u32 = 0x8316C1E0;
    'dispatch: loop {
        match pc {
            0x8316C1E0 => {
    //   block [0x8316C1E0..0x8316C380)
	// 8316C1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C1E4: 4803BF7D  bl 0x831a8160
	ctx.lr = 0x8316C1E8;
	sub_831A8130(ctx, base);
	// 8316C1E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C1F0: 4BFFE2A1  bl 0x8316a490
	ctx.lr = 0x8316C1F4;
	sub_8316A490(ctx, base);
	// 8316C1F4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316C1F8: 40820010  bne 0x8316c208
	if !ctx.cr[0].eq {
	pc = 0x8316C208; continue 'dispatch;
	}
	// 8316C1FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C200: 388B8838  addi r4, r11, -0x77c8
	ctx.r[4].s64 = ctx.r[11].s64 + -30664;
	// 8316C204: 48000158  b 0x8316c35c
	pc = 0x8316C35C; continue 'dispatch;
	// 8316C208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C20C: 4BFFE2DD  bl 0x8316a4e8
	ctx.lr = 0x8316C210;
	sub_8316A4E8(ctx, base);
	// 8316C210: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316C214: 419A0140  beq cr6, 0x8316c354
	if ctx.cr[6].eq {
	pc = 0x8316C354; continue 'dispatch;
	}
	// 8316C218: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316C21C: 419A0138  beq cr6, 0x8316c354
	if ctx.cr[6].eq {
	pc = 0x8316C354; continue 'dispatch;
	}
	// 8316C220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C224: 4BFFE26D  bl 0x8316a490
	ctx.lr = 0x8316C228;
	sub_8316A490(ctx, base);
	// 8316C228: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8316C22C: 419A011C  beq cr6, 0x8316c348
	if ctx.cr[6].eq {
	pc = 0x8316C348; continue 'dispatch;
	}
	// 8316C230: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8316C234: 419A0114  beq cr6, 0x8316c348
	if ctx.cr[6].eq {
	pc = 0x8316C348; continue 'dispatch;
	}
	// 8316C238: 817F02C0  lwz r11, 0x2c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(704 as u32) ) } as u64;
	// 8316C23C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316C240: 3BDF01A0  addi r30, r31, 0x1a0
	ctx.r[30].s64 = ctx.r[31].s64 + 416;
	// 8316C244: 394A948C  addi r10, r10, -0x6b74
	ctx.r[10].s64 = ctx.r[10].s64 + -27508;
	// 8316C248: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316C24C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8316C250: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316C254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316C258: 7F6B502E  lwzx r27, r11, r10
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8316C25C: 4803BF85  bl 0x831a81e0
	ctx.lr = 0x8316C260;
	sub_831A81E0(ctx, base);
	// 8316C260: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C264: 3BBF02E8  addi r29, r31, 0x2e8
	ctx.r[29].s64 = ctx.r[31].s64 + 744;
	// 8316C268: 3B8B8328  addi r28, r11, -0x7cd8
	ctx.r[28].s64 = ctx.r[11].s64 + -31960;
	// 8316C26C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8316C270: 817F029C  lwz r11, 0x29c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 8316C274: 93BF01C0  stw r29, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[29].u32 ) };
	// 8316C278: 917F01B8  stw r11, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[11].u32 ) };
	// 8316C27C: 817F024C  lwz r11, 0x24c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 8316C280: 917F01BC  stw r11, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[11].u32 ) };
	// 8316C284: 817F0140  lwz r11, 0x140(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 8316C288: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C28C: 419A0054  beq cr6, 0x8316c2e0
	if ctx.cr[6].eq {
	pc = 0x8316C2E0; continue 'dispatch;
	}
	// 8316C290: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316C294: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 8316C298: 48004231  bl 0x831704c8
	ctx.lr = 0x8316C29C;
	sub_831704C8(ctx, base);
	// 8316C29C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316C2A0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C2A4: 4800436D  bl 0x83170610
	ctx.lr = 0x8316C2A8;
	sub_83170610(ctx, base);
	// 8316C2A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316C2AC: 41820008  beq 0x8316c2b4
	if ctx.cr[0].eq {
	pc = 0x8316C2B4; continue 'dispatch;
	}
	// 8316C2B0: 935E0024  stw r26, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 8316C2B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C2B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C2BC: 917F0140  stw r11, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 8316C2C0: 4BFFE281  bl 0x8316a540
	ctx.lr = 0x8316C2C4;
	sub_8316A540(ctx, base);
	// 8316C2C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316C2C8: 419A0018  beq cr6, 0x8316c2e0
	if ctx.cr[6].eq {
	pc = 0x8316C2E0; continue 'dispatch;
	}
	// 8316C2CC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316C2D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C2D4: 419A000C  beq cr6, 0x8316c2e0
	if ctx.cr[6].eq {
	pc = 0x8316C2E0; continue 'dispatch;
	}
	// 8316C2D8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316C2DC: 48000070  b 0x8316c34c
	pc = 0x8316C34C; continue 'dispatch;
	// 8316C2E0: 817F0298  lwz r11, 0x298(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) } as u64;
	// 8316C2E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8316C2E8: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 8316C2EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C2F0: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8316C2F4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316C2F8: 4BFFE249  bl 0x8316a540
	ctx.lr = 0x8316C2FC;
	sub_8316A540(ctx, base);
	// 8316C2FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316C300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C304: 4BFFE4F5  bl 0x8316a7f8
	ctx.lr = 0x8316C308;
	sub_8316A7F8(ctx, base);
	// 8316C308: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8316C30C: 419A0024  beq cr6, 0x8316c330
	if ctx.cr[6].eq {
	pc = 0x8316C330; continue 'dispatch;
	}
	// 8316C310: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8316C314: 419A001C  beq cr6, 0x8316c330
	if ctx.cr[6].eq {
	pc = 0x8316C330; continue 'dispatch;
	}
	// 8316C318: 817F02E4  lwz r11, 0x2e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8316C31C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C320: 409A0010  bne cr6, 0x8316c330
	if !ctx.cr[6].eq {
	pc = 0x8316C330; continue 'dispatch;
	}
	// 8316C324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C328: 4BFFF469  bl 0x8316b790
	ctx.lr = 0x8316C32C;
	sub_8316B790(ctx, base);
	// 8316C32C: 4800004C  b 0x8316c378
	pc = 0x8316C378; continue 'dispatch;
	// 8316C330: 935F0148  stw r26, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[26].u32 ) };
	// 8316C334: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8316C338: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C33C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316C340: 48003CD1  bl 0x83170010
	ctx.lr = 0x8316C344;
	sub_83170010(ctx, base);
	// 8316C344: 48000034  b 0x8316c378
	pc = 0x8316C378; continue 'dispatch;
	// 8316C348: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8316C34C: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 8316C350: 48000028  b 0x8316c378
	pc = 0x8316C378; continue 'dispatch;
	// 8316C354: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C358: 388B87E0  addi r4, r11, -0x7820
	ctx.r[4].s64 = ctx.r[11].s64 + -30752;
	// 8316C35C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C360: 4BFF37B9  bl 0x8315fb18
	ctx.lr = 0x8316C364;
	sub_8315FB18(ctx, base);
	// 8316C364: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C368: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C36C: 409A000C  bne cr6, 0x8316c378
	if !ctx.cr[6].eq {
	pc = 0x8316C378; continue 'dispatch;
	}
	// 8316C370: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316C374: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316C378: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316C37C: 4803BE34  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C380 size=80
    let mut pc: u32 = 0x8316C380;
    'dispatch: loop {
        match pc {
            0x8316C380 => {
    //   block [0x8316C380..0x8316C3D0)
	// 8316C380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C384: 4803BDE5  bl 0x831a8168
	ctx.lr = 0x8316C388;
	sub_831A8130(ctx, base);
	// 8316C388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C390: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C394: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316C398: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316C39C: 4BFFE51D  bl 0x8316a8b8
	ctx.lr = 0x8316C3A0;
	sub_8316A8B8(ctx, base);
	// 8316C3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C3A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316C3A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C3AC: 4BFFEC45  bl 0x8316aff0
	ctx.lr = 0x8316C3B0;
	sub_8316AFF0(ctx, base);
	// 8316C3B0: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C3B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C3B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C3BC: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316C3C0: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C3C4: 4BFFE565  bl 0x8316a928
	ctx.lr = 0x8316C3C8;
	sub_8316A928(ctx, base);
	// 8316C3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316C3CC: 4803BDEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C3D0 size=80
    let mut pc: u32 = 0x8316C3D0;
    'dispatch: loop {
        match pc {
            0x8316C3D0 => {
    //   block [0x8316C3D0..0x8316C420)
	// 8316C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C3D4: 4803BD95  bl 0x831a8168
	ctx.lr = 0x8316C3D8;
	sub_831A8130(ctx, base);
	// 8316C3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C3E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C3E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316C3E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316C3EC: 4BFFE4CD  bl 0x8316a8b8
	ctx.lr = 0x8316C3F0;
	sub_8316A8B8(ctx, base);
	// 8316C3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C3F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316C3F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C3FC: 4BFFEE75  bl 0x8316b270
	ctx.lr = 0x8316C400;
	sub_8316B270(ctx, base);
	// 8316C400: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C404: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C40C: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316C410: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C414: 4BFFE515  bl 0x8316a928
	ctx.lr = 0x8316C418;
	sub_8316A928(ctx, base);
	// 8316C418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316C41C: 4803BD9C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C420 size=72
    let mut pc: u32 = 0x8316C420;
    'dispatch: loop {
        match pc {
            0x8316C420 => {
    //   block [0x8316C420..0x8316C468)
	// 8316C420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C424: 4803BD49  bl 0x831a816c
	ctx.lr = 0x8316C428;
	sub_831A8130(ctx, base);
	// 8316C428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C42C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C430: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C434: 4BFFE485  bl 0x8316a8b8
	ctx.lr = 0x8316C438;
	sub_8316A8B8(ctx, base);
	// 8316C438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C43C: 4BFFF10D  bl 0x8316b548
	ctx.lr = 0x8316C440;
	sub_8316B548(ctx, base);
	// 8316C440: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C444: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C448: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316C44C: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316C450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C454: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C458: 4BFFE4D1  bl 0x8316a928
	ctx.lr = 0x8316C45C;
	sub_8316A928(ctx, base);
	// 8316C45C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316C460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C464: 4803BD58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C468 size=72
    let mut pc: u32 = 0x8316C468;
    'dispatch: loop {
        match pc {
            0x8316C468 => {
    //   block [0x8316C468..0x8316C4B0)
	// 8316C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C46C: 4803BD01  bl 0x831a816c
	ctx.lr = 0x8316C470;
	sub_831A8130(ctx, base);
	// 8316C470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C478: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C47C: 4BFFE43D  bl 0x8316a8b8
	ctx.lr = 0x8316C480;
	sub_8316A8B8(ctx, base);
	// 8316C480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C484: 4BFFF135  bl 0x8316b5b8
	ctx.lr = 0x8316C488;
	sub_8316B5B8(ctx, base);
	// 8316C488: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C48C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C490: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316C494: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316C498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C49C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C4A0: 4BFFE489  bl 0x8316a928
	ctx.lr = 0x8316C4A4;
	sub_8316A928(ctx, base);
	// 8316C4A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316C4A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C4AC: 4803BD10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C4B0 size=72
    let mut pc: u32 = 0x8316C4B0;
    'dispatch: loop {
        match pc {
            0x8316C4B0 => {
    //   block [0x8316C4B0..0x8316C4F8)
	// 8316C4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C4B4: 4803BCB9  bl 0x831a816c
	ctx.lr = 0x8316C4B8;
	sub_831A8130(ctx, base);
	// 8316C4B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C4BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C4C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C4C4: 4BFFE3F5  bl 0x8316a8b8
	ctx.lr = 0x8316C4C8;
	sub_8316A8B8(ctx, base);
	// 8316C4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C4CC: 4BFFF16D  bl 0x8316b638
	ctx.lr = 0x8316C4D0;
	sub_8316B638(ctx, base);
	// 8316C4D0: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C4D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C4D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316C4DC: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316C4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C4E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C4E8: 4BFFE441  bl 0x8316a928
	ctx.lr = 0x8316C4EC;
	sub_8316A928(ctx, base);
	// 8316C4EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316C4F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C4F4: 4803BCC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C4F8 size=84
    let mut pc: u32 = 0x8316C4F8;
    'dispatch: loop {
        match pc {
            0x8316C4F8 => {
    //   block [0x8316C4F8..0x8316C54C)
	// 8316C4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C500: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C504: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C50C: 4BFFE3AD  bl 0x8316a8b8
	ctx.lr = 0x8316C510;
	sub_8316A8B8(ctx, base);
	// 8316C510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C514: 4BFFE415  bl 0x8316a928
	ctx.lr = 0x8316C518;
	sub_8316A928(ctx, base);
	// 8316C518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C51C: 4BFFE0ED  bl 0x8316a608
	ctx.lr = 0x8316C520;
	sub_8316A608(ctx, base);
	// 8316C520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C524: 4BFFE395  bl 0x8316a8b8
	ctx.lr = 0x8316C528;
	sub_8316A8B8(ctx, base);
	// 8316C528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C52C: 4BFFF545  bl 0x8316ba70
	ctx.lr = 0x8316C530;
	sub_8316BA70(ctx, base);
	// 8316C530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C534: 4BFFE3F5  bl 0x8316a928
	ctx.lr = 0x8316C538;
	sub_8316A928(ctx, base);
	// 8316C538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316C544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316C548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C550 size=84
    let mut pc: u32 = 0x8316C550;
    'dispatch: loop {
        match pc {
            0x8316C550 => {
    //   block [0x8316C550..0x8316C5A4)
	// 8316C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C55C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C564: 4BFFE355  bl 0x8316a8b8
	ctx.lr = 0x8316C568;
	sub_8316A8B8(ctx, base);
	// 8316C568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C56C: 4BFFE3BD  bl 0x8316a928
	ctx.lr = 0x8316C570;
	sub_8316A928(ctx, base);
	// 8316C570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C574: 4BFFF285  bl 0x8316b7f8
	ctx.lr = 0x8316C578;
	sub_8316B7F8(ctx, base);
	// 8316C578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C57C: 4BFFE33D  bl 0x8316a8b8
	ctx.lr = 0x8316C580;
	sub_8316A8B8(ctx, base);
	// 8316C580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C584: 4BFFE1BD  bl 0x8316a740
	ctx.lr = 0x8316C588;
	sub_8316A740(ctx, base);
	// 8316C588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C58C: 4BFFE39D  bl 0x8316a928
	ctx.lr = 0x8316C590;
	sub_8316A928(ctx, base);
	// 8316C590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316C594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316C598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316C59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316C5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C5A8 size=72
    let mut pc: u32 = 0x8316C5A8;
    'dispatch: loop {
        match pc {
            0x8316C5A8 => {
    //   block [0x8316C5A8..0x8316C5F0)
	// 8316C5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C5AC: 4803BBC1  bl 0x831a816c
	ctx.lr = 0x8316C5B0;
	sub_831A8130(ctx, base);
	// 8316C5B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C5B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C5B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C5BC: 4BFFE2FD  bl 0x8316a8b8
	ctx.lr = 0x8316C5C0;
	sub_8316A8B8(ctx, base);
	// 8316C5C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C5C4: 4BFFF5B5  bl 0x8316bb78
	ctx.lr = 0x8316C5C8;
	sub_8316BB78(ctx, base);
	// 8316C5C8: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C5CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C5D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316C5D4: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316C5D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C5DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C5E0: 4BFFE349  bl 0x8316a928
	ctx.lr = 0x8316C5E4;
	sub_8316A928(ctx, base);
	// 8316C5E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316C5E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C5EC: 4803BBD0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C5F0 size=72
    let mut pc: u32 = 0x8316C5F0;
    'dispatch: loop {
        match pc {
            0x8316C5F0 => {
    //   block [0x8316C5F0..0x8316C638)
	// 8316C5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C5F4: 4803BB79  bl 0x831a816c
	ctx.lr = 0x8316C5F8;
	sub_831A8130(ctx, base);
	// 8316C5F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C5FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C600: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C604: 4BFFE2B5  bl 0x8316a8b8
	ctx.lr = 0x8316C608;
	sub_8316A8B8(ctx, base);
	// 8316C608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C60C: 4BFFF5ED  bl 0x8316bbf8
	ctx.lr = 0x8316C610;
	sub_8316BBF8(ctx, base);
	// 8316C610: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C614: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C618: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316C61C: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316C620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C624: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C628: 4BFFE301  bl 0x8316a928
	ctx.lr = 0x8316C62C;
	sub_8316A928(ctx, base);
	// 8316C62C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316C630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C634: 4803BB88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C638 size=88
    let mut pc: u32 = 0x8316C638;
    'dispatch: loop {
        match pc {
            0x8316C638 => {
    //   block [0x8316C638..0x8316C690)
	// 8316C638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C63C: 4803BB2D  bl 0x831a8168
	ctx.lr = 0x8316C640;
	sub_831A8130(ctx, base);
	// 8316C640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C648: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C64C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316C650: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316C654: 4BFFE265  bl 0x8316a8b8
	ctx.lr = 0x8316C658;
	sub_8316A8B8(ctx, base);
	// 8316C658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C65C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316C660: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C664: 4BFFF5FD  bl 0x8316bc60
	ctx.lr = 0x8316C668;
	sub_8316BC60(ctx, base);
	// 8316C668: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C66C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316C670: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316C674: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8316C678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C67C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C680: 4BFFE2A9  bl 0x8316a928
	ctx.lr = 0x8316C684;
	sub_8316A928(ctx, base);
	// 8316C684: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316C688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316C68C: 4803BB2C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C690 size=80
    let mut pc: u32 = 0x8316C690;
    'dispatch: loop {
        match pc {
            0x8316C690 => {
    //   block [0x8316C690..0x8316C6E0)
	// 8316C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C694: 4803BAD5  bl 0x831a8168
	ctx.lr = 0x8316C698;
	sub_831A8130(ctx, base);
	// 8316C698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C69C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C6A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C6A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316C6A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316C6AC: 4BFFE20D  bl 0x8316a8b8
	ctx.lr = 0x8316C6B0;
	sub_8316A8B8(ctx, base);
	// 8316C6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C6B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316C6B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316C6BC: 4BFFF6ED  bl 0x8316bda8
	ctx.lr = 0x8316C6C0;
	sub_8316BDA8(ctx, base);
	// 8316C6C0: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C6C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C6C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C6CC: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316C6D0: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C6D4: 4BFFE255  bl 0x8316a928
	ctx.lr = 0x8316C6D8;
	sub_8316A928(ctx, base);
	// 8316C6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316C6DC: 4803BADC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316C6E0 size=16
    let mut pc: u32 = 0x8316C6E0;
    'dispatch: loop {
        match pc {
            0x8316C6E0 => {
    //   block [0x8316C6E0..0x8316C6F0)
	// 8316C6E0: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 8316C6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8316C6E8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8316C6EC: 4BFFFA9C  b 0x8316c188
	sub_8316C188(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C6F0 size=88
    let mut pc: u32 = 0x8316C6F0;
    'dispatch: loop {
        match pc {
            0x8316C6F0 => {
    //   block [0x8316C6F0..0x8316C748)
	// 8316C6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C6F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316C6FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C708: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316C70C: 4BFFE1AD  bl 0x8316a8b8
	ctx.lr = 0x8316C710;
	sub_8316A8B8(ctx, base);
	// 8316C710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C714: 4BFFFACD  bl 0x8316c1e0
	ctx.lr = 0x8316C718;
	sub_8316C1E0(ctx, base);
	// 8316C718: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8316C71C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316C724: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 8316C728: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C72C: 4BFFE1FD  bl 0x8316a928
	ctx.lr = 0x8316C730;
	sub_8316A928(ctx, base);
	// 8316C730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316C738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316C73C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316C740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316C744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316C748 size=32
    let mut pc: u32 = 0x8316C748;
    'dispatch: loop {
        match pc {
            0x8316C748 => {
    //   block [0x8316C748..0x8316C768)
	// 8316C748: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316C74C: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316C750: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316C754: 419A001C  beq cr6, 0x8316c770
	if ctx.cr[6].eq {
		sub_8316C770(ctx, base);
		return;
	}
	// 8316C758: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8316C75C: 419A0010  beq cr6, 0x8316c76c
	if ctx.cr[6].eq {
		sub_8316C76C(ctx, base);
		return;
	}
	// 8316C760: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8316C764: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316C768 size=4
    let mut pc: u32 = 0x8316C768;
    'dispatch: loop {
        match pc {
            0x8316C768 => {
    //   block [0x8316C768..0x8316C76C)
	// 8316C768: 4BFFFDE8  b 0x8316c550
	sub_8316C550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C76C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316C76C size=4
    let mut pc: u32 = 0x8316C76C;
    'dispatch: loop {
        match pc {
            0x8316C76C => {
    //   block [0x8316C76C..0x8316C770)
	// 8316C76C: 4BFFF024  b 0x8316b790
	sub_8316B790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316C770 size=8
    let mut pc: u32 = 0x8316C770;
    'dispatch: loop {
        match pc {
            0x8316C770 => {
    //   block [0x8316C770..0x8316C778)
	// 8316C770: 4BFFFD88  b 0x8316c4f8
	sub_8316C4F8(ctx, base);
	return;
	// 8316C774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C778 size=264
    let mut pc: u32 = 0x8316C778;
    'dispatch: loop {
        match pc {
            0x8316C778 => {
    //   block [0x8316C778..0x8316C880)
	// 8316C778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C77C: 4803B9E5  bl 0x831a8160
	ctx.lr = 0x8316C780;
	sub_831A8130(ctx, base);
	// 8316C780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C784: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8316C788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C78C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C790: 4BFF3291  bl 0x8315fa20
	ctx.lr = 0x8316C794;
	sub_8315FA20(ctx, base);
	// 8316C794: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C798: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 8316C79C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316C7A0: 480033E9  bl 0x8316fb88
	ctx.lr = 0x8316C7A4;
	sub_8316FB88(ctx, base);
	// 8316C7A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316C7A8: 48003B79  bl 0x83170320
	ctx.lr = 0x8316C7AC;
	sub_83170320(ctx, base);
	// 8316C7AC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C7B0: 38801000  li r4, 0x1000
	ctx.r[4].s64 = 4096;
	// 8316C7B4: 386B8488  addi r3, r11, -0x7b78
	ctx.r[3].s64 = ctx.r[11].s64 + -31608;
	// 8316C7B8: 4BFF3011  bl 0x8315f7c8
	ctx.lr = 0x8316C7BC;
	sub_8315F7C8(ctx, base);
	// 8316C7BC: 3FA0833A  lis r29, -0x7cc6
	ctx.r[29].s64 = -2093350912;
	// 8316C7C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316C7C4: 907D9488  stw r3, -0x6b78(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-27512 as u32), ctx.r[3].u32 ) };
	// 8316C7C8: 40820020  bne 0x8316c7e8
	if !ctx.cr[0].eq {
	pc = 0x8316C7E8; continue 'dispatch;
	}
	// 8316C7CC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C7D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C7D4: 388B88A0  addi r4, r11, -0x7760
	ctx.r[4].s64 = ctx.r[11].s64 + -30560;
	// 8316C7D8: 4BFF3341  bl 0x8315fb18
	ctx.lr = 0x8316C7DC;
	sub_8315FB18(ctx, base);
	// 8316C7DC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316C7E0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C7E4: 48000068  b 0x8316c84c
	pc = 0x8316C84C; continue 'dispatch;
	// 8316C7E8: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316C7EC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C7F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8316C7F4: 3B8A949C  addi r28, r10, -0x6b64
	ctx.r[28].s64 = ctx.r[10].s64 + -27492;
	// 8316C7F8: 3B6B948C  addi r27, r11, -0x6b74
	ctx.r[27].s64 = ctx.r[11].s64 + -27508;
	// 8316C7FC: 48000008  b 0x8316c804
	pc = 0x8316C804; continue 'dispatch;
	// 8316C800: 807D9488  lwz r3, -0x6b78(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-27512 as u32) ) } as u64;
	// 8316C804: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316C808: 48003619  bl 0x8316fe20
	ctx.lr = 0x8316C80C;
	sub_8316FE20(ctx, base);
	// 8316C80C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316C810: 7C7FD92E  stwx r3, r31, r27
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32), ctx.r[3].u32) };
	// 8316C814: 41820040  beq 0x8316c854
	if ctx.cr[0].eq {
	pc = 0x8316C854; continue 'dispatch;
	}
	// 8316C818: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316C81C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C820: 409A0034  bne cr6, 0x8316c854
	if !ctx.cr[6].eq {
	pc = 0x8316C854; continue 'dispatch;
	}
	// 8316C824: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 8316C828: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8316C82C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8316C830: 388BC748  addi r4, r11, -0x38b8
	ctx.r[4].s64 = ctx.r[11].s64 + -14520;
	// 8316C834: 4800312D  bl 0x8316f960
	ctx.lr = 0x8316C838;
	sub_8316F960(ctx, base);
	// 8316C838: 7C7FE12E  stwx r3, r31, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 8316C83C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8316C840: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 8316C844: 4198FFBC  blt cr6, 0x8316c800
	if ctx.cr[6].lt {
	pc = 0x8316C800; continue 'dispatch;
	}
	// 8316C848: 4BFFDD51  bl 0x8316a598
	ctx.lr = 0x8316C84C;
	sub_8316A598(ctx, base);
	// 8316C84C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316C850: 4803B960  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8316C854: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C858: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C85C: 388B8878  addi r4, r11, -0x7788
	ctx.r[4].s64 = ctx.r[11].s64 + -30600;
	// 8316C860: 4BFF32B9  bl 0x8315fb18
	ctx.lr = 0x8316C864;
	sub_8315FB18(ctx, base);
	// 8316C864: 807D9488  lwz r3, -0x6b78(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-27512 as u32) ) } as u64;
	// 8316C868: 4BFF3041  bl 0x8315f8a8
	ctx.lr = 0x8316C86C;
	sub_8315F8A8(ctx, base);
	// 8316C86C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C870: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8316C874: 917D9488  stw r11, -0x6b78(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-27512 as u32), ctx.r[11].u32 ) };
	// 8316C878: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316C87C: 4BFFFFD0  b 0x8316c84c
	pc = 0x8316C84C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C880 size=120
    let mut pc: u32 = 0x8316C880;
    'dispatch: loop {
        match pc {
            0x8316C880 => {
    //   block [0x8316C880..0x8316C8F8)
	// 8316C880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C88C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316C894: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C898: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316C89C: 38EB8480  addi r7, r11, -0x7b80
	ctx.r[7].s64 = ctx.r[11].s64 + -31616;
	// 8316C8A0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316C8A4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8316C8A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316C8AC: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8316C8B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316C8B4: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8316C8B8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316C8BC: 4082FFE8  bne 0x8316c8a4
	if !ctx.cr[0].eq {
	pc = 0x8316C8A4; continue 'dispatch;
	}
	// 8316C8C0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8316C8C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316C8C8: 409A001C  bne cr6, 0x8316c8e4
	if !ctx.cr[6].eq {
	pc = 0x8316C8E4; continue 'dispatch;
	}
	// 8316C8CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8316C8D0: 4BFFFEA9  bl 0x8316c778
	ctx.lr = 0x8316C8D4;
	sub_8316C778(ctx, base);
	// 8316C8D4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316C8D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C8DC: 419A0008  beq cr6, 0x8316c8e4
	if ctx.cr[6].eq {
	pc = 0x8316C8E4; continue 'dispatch;
	}
	// 8316C8E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C8E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316C8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316C8F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316C8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C8F8 size=224
    let mut pc: u32 = 0x8316C8F8;
    'dispatch: loop {
        match pc {
            0x8316C8F8 => {
    //   block [0x8316C8F8..0x8316C9D8)
	// 8316C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316C904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C90C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316C910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C914: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C918: 4BFF3109  bl 0x8315fa20
	ctx.lr = 0x8316C91C;
	sub_8315FA20(ctx, base);
	// 8316C91C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C920: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 8316C924: 386B94B8  addi r3, r11, -0x6b48
	ctx.r[3].s64 = ctx.r[11].s64 + -27464;
	// 8316C928: 4BFF2EA1  bl 0x8315f7c8
	ctx.lr = 0x8316C92C;
	sub_8315F7C8(ctx, base);
	// 8316C92C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316C930: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316C934: 906B94B4  stw r3, -0x6b4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27468 as u32), ctx.r[3].u32 ) };
	// 8316C938: 40820010  bne 0x8316c948
	if !ctx.cr[0].eq {
	pc = 0x8316C948; continue 'dispatch;
	}
	// 8316C93C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C940: 388B88F8  addi r4, r11, -0x7708
	ctx.r[4].s64 = ctx.r[11].s64 + -30472;
	// 8316C944: 4800006C  b 0x8316c9b0
	pc = 0x8316C9B0; continue 'dispatch;
	// 8316C948: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316C94C: 48099175  bl 0x83205ac0
	ctx.lr = 0x8316C950;
	sub_83205AC0(ctx, base);
	// 8316C950: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8316C954: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316C958: 907F98B8  stw r3, -0x6748(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26440 as u32), ctx.r[3].u32 ) };
	// 8316C95C: 4182004C  beq 0x8316c9a8
	if ctx.cr[0].eq {
	pc = 0x8316C9A8; continue 'dispatch;
	}
	// 8316C960: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316C964: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316C968: 409A0040  bne cr6, 0x8316c9a8
	if !ctx.cr[6].eq {
	pc = 0x8316C9A8; continue 'dispatch;
	}
	// 8316C96C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C970: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316C974: 3880FFF1  li r4, -0xf
	ctx.r[4].s64 = -15;
	// 8316C978: 38AA81F0  addi r5, r10, -0x7e10
	ctx.r[5].s64 = ctx.r[10].s64 + -32272;
	// 8316C97C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316C980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C984: 4E800421  bctrl
	ctx.lr = 0x8316C988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C988: 807F98B8  lwz r3, -0x6748(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26440 as u32) ) } as u64;
	// 8316C98C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316C990: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8316C994: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316C998: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316C99C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316C9A0: 4E800421  bctrl
	ctx.lr = 0x8316C9A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316C9A4: 4800001C  b 0x8316c9c0
	pc = 0x8316C9C0; continue 'dispatch;
	// 8316C9A8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316C9AC: 388B88C8  addi r4, r11, -0x7738
	ctx.r[4].s64 = ctx.r[11].s64 + -30520;
	// 8316C9B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316C9B4: 4BFF3165  bl 0x8315fb18
	ctx.lr = 0x8316C9B8;
	sub_8315FB18(ctx, base);
	// 8316C9B8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316C9BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C9C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316C9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316C9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316C9CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316C9D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316C9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316C9D8 size=124
    let mut pc: u32 = 0x8316C9D8;
    'dispatch: loop {
        match pc {
            0x8316C9D8 => {
    //   block [0x8316C9D8..0x8316CA54)
	// 8316C9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316C9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316C9E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316C9E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316C9E8: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8316C9EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316C9F0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316C9F4: 807F98B8  lwz r3, -0x6748(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26440 as u32) ) } as u64;
	// 8316C9F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316C9FC: 419A0024  beq cr6, 0x8316ca20
	if ctx.cr[6].eq {
	pc = 0x8316CA20; continue 'dispatch;
	}
	// 8316CA00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CA04: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316CA08: 388A81F0  addi r4, r10, -0x7e10
	ctx.r[4].s64 = ctx.r[10].s64 + -32272;
	// 8316CA0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CA10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CA14: 4E800421  bctrl
	ctx.lr = 0x8316CA18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316CA1C: 917F98B8  stw r11, -0x6748(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26440 as u32), ctx.r[11].u32 ) };
	// 8316CA20: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 8316CA24: 807F94B4  lwz r3, -0x6b4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-27468 as u32) ) } as u64;
	// 8316CA28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316CA2C: 419A0010  beq cr6, 0x8316ca3c
	if ctx.cr[6].eq {
	pc = 0x8316CA3C; continue 'dispatch;
	}
	// 8316CA30: 4BFF2E79  bl 0x8315f8a8
	ctx.lr = 0x8316CA34;
	sub_8315F8A8(ctx, base);
	// 8316CA34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316CA38: 917F94B4  stw r11, -0x6b4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-27468 as u32), ctx.r[11].u32 ) };
	// 8316CA3C: 4BFF2BDD  bl 0x8315f618
	ctx.lr = 0x8316CA40;
	sub_8315F618(ctx, base);
	// 8316CA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316CA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316CA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316CA4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316CA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CA58 size=148
    let mut pc: u32 = 0x8316CA58;
    'dispatch: loop {
        match pc {
            0x8316CA58 => {
    //   block [0x8316CA58..0x8316CAEC)
	// 8316CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CA5C: 4803B711  bl 0x831a816c
	ctx.lr = 0x8316CA60;
	sub_831A8130(ctx, base);
	// 8316CA60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CA64: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316CA68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8316CA6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316CA70: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CA74: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8316CA78: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8316CA7C: 38AB8944  addi r5, r11, -0x76bc
	ctx.r[5].s64 = ctx.r[11].s64 + -30396;
	// 8316CA80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316CA84: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8316CA88: 4BFF3271  bl 0x8315fcf8
	ctx.lr = 0x8316CA8C;
	sub_8315FCF8(ctx, base);
	// 8316CA8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316CA90: 41820028  beq 0x8316cab8
	if ctx.cr[0].eq {
	pc = 0x8316CAB8; continue 'dispatch;
	}
	// 8316CA94: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316CA98: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8316CA9C: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8316CAA0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316CAA4: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8316CAA8: 93E30010  stw r31, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 8316CAAC: FBE30018  std r31, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 8316CAB0: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8316CAB4: 48000008  b 0x8316cabc
	pc = 0x8316CABC; continue 'dispatch;
	// 8316CAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316CABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316CAC0: 409A0020  bne cr6, 0x8316cae0
	if !ctx.cr[6].eq {
	pc = 0x8316CAE0; continue 'dispatch;
	}
	// 8316CAC4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CAC8: 388B8920  addi r4, r11, -0x76e0
	ctx.r[4].s64 = ctx.r[11].s64 + -30432;
	// 8316CACC: 4BFF304D  bl 0x8315fb18
	ctx.lr = 0x8316CAD0;
	sub_8315FB18(ctx, base);
	// 8316CAD0: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 8316CAD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316CAD8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CADC: 48000008  b 0x8316cae4
	pc = 0x8316CAE4; continue 'dispatch;
	// 8316CAE0: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316CAE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316CAE8: 4803B6D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CAF0 size=140
    let mut pc: u32 = 0x8316CAF0;
    'dispatch: loop {
        match pc {
            0x8316CAF0 => {
    //   block [0x8316CAF0..0x8316CB7C)
	// 8316CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316CAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316CAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CB00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CB04: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CB08: 48099221  bl 0x83205d28
	ctx.lr = 0x8316CB0C;
	sub_83205D28(ctx, base);
	// 8316CB0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316CB10: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8316CB14: 40820010  bne 0x8316cb24
	if !ctx.cr[0].eq {
	pc = 0x8316CB24; continue 'dispatch;
	}
	// 8316CB18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316CB1C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8316CB20: 48000048  b 0x8316cb68
	pc = 0x8316CB68; continue 'dispatch;
	// 8316CB24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CB28: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316CB2C: 80FF0010  lwz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316CB30: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316CB34: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8316CB38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316CB3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CB40: 4E800421  bctrl
	ctx.lr = 0x8316CB44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CB44: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316CB48: F87F0018  std r3, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u64 ) };
	// 8316CB4C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316CB50: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CB54: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CB58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CB5C: 4E800421  bctrl
	ctx.lr = 0x8316CB60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316CB64: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8316CB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316CB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316CB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316CB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316CB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CB80 size=120
    let mut pc: u32 = 0x8316CB80;
    'dispatch: loop {
        match pc {
            0x8316CB80 => {
    //   block [0x8316CB80..0x8316CBF8)
	// 8316CB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316CB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CB8C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316CB94: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316CB98: 816B98B8  lwz r11, -0x6748(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26440 as u32) ) } as u64;
	// 8316CB9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316CBA0: 419A0044  beq cr6, 0x8316cbe4
	if ctx.cr[6].eq {
	pc = 0x8316CBE4; continue 'dispatch;
	}
	// 8316CBA4: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316CBA8: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8316CBAC: 419A0038  beq cr6, 0x8316cbe4
	if ctx.cr[6].eq {
	pc = 0x8316CBE4; continue 'dispatch;
	}
	// 8316CBB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CBB4: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 8316CBB8: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316CBBC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316CBC0: 38A981F0  addi r5, r9, -0x7e10
	ctx.r[5].s64 = ctx.r[9].s64 + -32272;
	// 8316CBC4: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316CBC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CBCC: 4E800421  bctrl
	ctx.lr = 0x8316CBD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CBD0: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8316CBD4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316CBD8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316CBDC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8316CBE0: 48000008  b 0x8316cbe8
	pc = 0x8316CBE8; continue 'dispatch;
	// 8316CBE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316CBE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316CBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316CBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316CBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CBF8 size=8
    let mut pc: u32 = 0x8316CBF8;
    'dispatch: loop {
        match pc {
            0x8316CBF8 => {
    //   block [0x8316CBF8..0x8316CC00)
	// 8316CBF8: 90830028  stw r4, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 8316CBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CC00 size=120
    let mut pc: u32 = 0x8316CC00;
    'dispatch: loop {
        match pc {
            0x8316CC00 => {
    //   block [0x8316CC00..0x8316CC78)
	// 8316CC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316CC08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316CC0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CC10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CC14: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CC18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316CC1C: 38EB94B0  addi r7, r11, -0x6b50
	ctx.r[7].s64 = ctx.r[11].s64 + -27472;
	// 8316CC20: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316CC24: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8316CC28: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316CC2C: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8316CC30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316CC34: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8316CC38: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316CC3C: 4082FFE8  bne 0x8316cc24
	if !ctx.cr[0].eq {
	pc = 0x8316CC24; continue 'dispatch;
	}
	// 8316CC40: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8316CC44: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316CC48: 409A001C  bne cr6, 0x8316cc64
	if !ctx.cr[6].eq {
	pc = 0x8316CC64; continue 'dispatch;
	}
	// 8316CC4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8316CC50: 4BFFFCA9  bl 0x8316c8f8
	ctx.lr = 0x8316CC54;
	sub_8316C8F8(ctx, base);
	// 8316CC54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316CC58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316CC5C: 419A0008  beq cr6, 0x8316cc64
	if ctx.cr[6].eq {
	pc = 0x8316CC64; continue 'dispatch;
	}
	// 8316CC60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CC64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316CC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316CC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316CC70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CC78 size=56
    let mut pc: u32 = 0x8316CC78;
    'dispatch: loop {
        match pc {
            0x8316CC78 => {
    //   block [0x8316CC78..0x8316CCB0)
	// 8316CC78: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316CC80: 38EB94B0  addi r7, r11, -0x6b50
	ctx.r[7].s64 = ctx.r[11].s64 + -27472;
	// 8316CC84: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316CC88: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8316CC8C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316CC90: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8316CC94: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8316CC98: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8316CC9C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316CCA0: 4082FFE8  bne 0x8316cc88
	if !ctx.cr[0].eq {
	pc = 0x8316CC88; continue 'dispatch;
	}
	// 8316CCA4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8316CCA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316CCAC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CCB0 size=12
    let mut pc: u32 = 0x8316CCB0;
    'dispatch: loop {
        match pc {
            0x8316CCB0 => {
    //   block [0x8316CCB0..0x8316CCBC)
	// 8316CCB0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CCB4: 386B8328  addi r3, r11, -0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -31960;
	// 8316CCB8: 4BFFFD20  b 0x8316c9d8
	sub_8316C9D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CCBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CCBC size=4
    let mut pc: u32 = 0x8316CCBC;
    'dispatch: loop {
        match pc {
            0x8316CCBC => {
    //   block [0x8316CCBC..0x8316CCC0)
	// 8316CCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CCC0 size=16
    let mut pc: u32 = 0x8316CCC0;
    'dispatch: loop {
        match pc {
            0x8316CCC0 => {
    //   block [0x8316CCC0..0x8316CCD0)
	// 8316CCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316CCC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316CCC8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CCCC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CCD0 size=8
    let mut pc: u32 = 0x8316CCD0;
    'dispatch: loop {
        match pc {
            0x8316CCD0 => {
    //   block [0x8316CCD0..0x8316CCD8)
	// 8316CCD0: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8316CCD4: 4BFF2FAC  b 0x8315fc80
	sub_8315FC80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316CCD8 size=4
    let mut pc: u32 = 0x8316CCD8;
    'dispatch: loop {
        match pc {
            0x8316CCD8 => {
    //   block [0x8316CCD8..0x8316CCDC)
	// 8316CCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CCE0 size=184
    let mut pc: u32 = 0x8316CCE0;
    'dispatch: loop {
        match pc {
            0x8316CCE0 => {
    //   block [0x8316CCE0..0x8316CD98)
	// 8316CCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316CCE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316CCEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316CCF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CCF4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8316CCF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CCFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316CD00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CD04: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316CD08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316CD0C: 419A0014  beq cr6, 0x8316cd20
	if ctx.cr[6].eq {
	pc = 0x8316CD20; continue 'dispatch;
	}
	// 8316CD10: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316CD14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316CD18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CD1C: 48000064  b 0x8316cd80
	pc = 0x8316CD80; continue 'dispatch;
	// 8316CD20: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CD24: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8316CD28: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8316CD2C: 3D408317  lis r10, -0x7ce9
	ctx.r[10].s64 = -2095644672;
	// 8316CD30: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 8316CD34: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8316CD38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8316CD3C: 388ACAF0  addi r4, r10, -0x3510
	ctx.r[4].s64 = ctx.r[10].s64 + -13584;
	// 8316CD40: 806B98B8  lwz r3, -0x6748(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26440 as u32) ) } as u64;
	// 8316CD44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CD48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316CD4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CD50: 4E800421  bctrl
	ctx.lr = 0x8316CD54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CD54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316CD58: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8316CD5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316CD60: 419A0010  beq cr6, 0x8316cd70
	if ctx.cr[6].eq {
	pc = 0x8316CD70; continue 'dispatch;
	}
	// 8316CD64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CD68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316CD6C: 48000014  b 0x8316cd80
	pc = 0x8316CD80; continue 'dispatch;
	// 8316CD70: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8316CD74: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316CD78: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316CD7C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8316CD80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316CD8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316CD90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316CD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CD98 size=132
    let mut pc: u32 = 0x8316CD98;
    'dispatch: loop {
        match pc {
            0x8316CD98 => {
    //   block [0x8316CD98..0x8316CE1C)
	// 8316CD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CD9C: 4803B3C5  bl 0x831a8160
	ctx.lr = 0x8316CDA0;
	sub_831A8130(ctx, base);
	// 8316CDA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CDA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CDA8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CDAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316CDB0: 3B4B81F0  addi r26, r11, -0x7e10
	ctx.r[26].s64 = ctx.r[11].s64 + -32272;
	// 8316CDB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316CDB8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316CDBC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316CDC0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8316CDC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316CDC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CDCC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316CDD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CDD4: 4E800421  bctrl
	ctx.lr = 0x8316CDD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CDD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316CDDC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8316CDE0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8316CDE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316CDE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316CDEC: 4BFFFEF5  bl 0x8316cce0
	ctx.lr = 0x8316CDF0;
	sub_8316CCE0(ctx, base);
	// 8316CDF0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316CDF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CDF8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316CDFC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316CE00: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CE04: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316CE08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316CE0C: 4E800421  bctrl
	ctx.lr = 0x8316CE10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316CE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316CE14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316CE18: 4803B398  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CE20 size=68
    let mut pc: u32 = 0x8316CE20;
    'dispatch: loop {
        match pc {
            0x8316CE20 => {
    //   block [0x8316CE20..0x8316CE64)
	// 8316CE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316CE28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316CE2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CE30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CE34: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CE38: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316CE3C: 396B8958  addi r11, r11, -0x76a8
	ctx.r[11].s64 = ctx.r[11].s64 + -30376;
	// 8316CE40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316CE44: 41820008  beq 0x8316ce4c
	if ctx.cr[0].eq {
	pc = 0x8316CE4C; continue 'dispatch;
	}
	// 8316CE48: 4B153421  bl 0x822c0268
	ctx.lr = 0x8316CE4C;
	sub_822C0268(ctx, base);
	// 8316CE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316CE50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316CE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316CE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316CE5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316CE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316CE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316CE68 size=480
    let mut pc: u32 = 0x8316CE68;
    'dispatch: loop {
        match pc {
            0x8316CE68 => {
    //   block [0x8316CE68..0x8316D048)
	// 8316CE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316CE6C: 4803B2ED  bl 0x831a8158
	ctx.lr = 0x8316CE70;
	sub_831A8130(ctx, base);
	// 8316CE70: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316CE74: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CE78: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8316CE7C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8316CE80: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8316CE84: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8316CE88: 38AB8A50  addi r5, r11, -0x75b0
	ctx.r[5].s64 = ctx.r[11].s64 + -30128;
	// 8316CE8C: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 8316CE90: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8316CE94: 4BFF27AD  bl 0x8315f640
	ctx.lr = 0x8316CE98;
	sub_8315F640(ctx, base);
	// 8316CE98: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8316CE9C: 40820010  bne 0x8316ceac
	if !ctx.cr[0].eq {
	pc = 0x8316CEAC; continue 'dispatch;
	}
	// 8316CEA0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CEA4: 388B8A28  addi r4, r11, -0x75d8
	ctx.r[4].s64 = ctx.r[11].s64 + -30168;
	// 8316CEA8: 4800018C  b 0x8316d034
	pc = 0x8316D034; continue 'dispatch;
	// 8316CEAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8316CEB0: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 8316CEB4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8316CEB8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316CEBC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316CEC0: 4BFFA031  bl 0x83166ef0
	ctx.lr = 0x8316CEC4;
	sub_83166EF0(ctx, base);
	// 8316CEC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316CEC8: 41820030  beq 0x8316cef8
	if ctx.cr[0].eq {
	pc = 0x8316CEF8; continue 'dispatch;
	}
	// 8316CECC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CED0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316CED4: 409A0024  bne cr6, 0x8316cef8
	if !ctx.cr[6].eq {
	pc = 0x8316CEF8; continue 'dispatch;
	}
	// 8316CED8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8316CEDC: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8316CEE0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8316CEE4: 93FDFFFC  stw r31, -4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), ctx.r[31].u32 ) };
	// 8316CEE8: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 8316CEEC: 2B1E0008  cmplwi cr6, r30, 8
	ctx.cr[6].compare_u32(ctx.r[30].u32, 8 as u32, &mut ctx.xer);
	// 8316CEF0: 4198FFC8  blt cr6, 0x8316ceb8
	if ctx.cr[6].lt {
	pc = 0x8316CEB8; continue 'dispatch;
	}
	// 8316CEF4: 48000034  b 0x8316cf28
	pc = 0x8316CF28; continue 'dispatch;
	// 8316CEF8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316CEFC: 409A001C  bne cr6, 0x8316cf18
	if !ctx.cr[6].eq {
	pc = 0x8316CF18; continue 'dispatch;
	}
	// 8316CF00: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316CF04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316CF08: 4BFF2861  bl 0x8315f768
	ctx.lr = 0x8316CF0C;
	sub_8315F768(ctx, base);
	// 8316CF0C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CF10: 388B8A04  addi r4, r11, -0x75fc
	ctx.r[4].s64 = ctx.r[11].s64 + -30204;
	// 8316CF14: 48000120  b 0x8316d034
	pc = 0x8316D034; continue 'dispatch;
	// 8316CF18: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CF1C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316CF20: 388B89E0  addi r4, r11, -0x7620
	ctx.r[4].s64 = ctx.r[11].s64 + -30240;
	// 8316CF24: 4BFF2BF5  bl 0x8315fb18
	ctx.lr = 0x8316CF28;
	sub_8315FB18(ctx, base);
	// 8316CF28: 2B1E0008  cmplwi cr6, r30, 8
	ctx.cr[6].compare_u32(ctx.r[30].u32, 8 as u32, &mut ctx.xer);
	// 8316CF2C: 40980030  bge cr6, 0x8316cf5c
	if !ctx.cr[6].lt {
	pc = 0x8316CF5C; continue 'dispatch;
	}
	// 8316CF30: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 * 12;
	// 8316CF34: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8316CF38: 215E0008  subfic r10, r30, 8
	ctx.xer.ca = ctx.r[30].u32 <= 8 as u32;
	ctx.r[10].s64 = (8 as i64) - ctx.r[30].s64;
	// 8316CF3C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8316CF40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8316CF44: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8316CF48: 93EBFFFC  stw r31, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[31].u32 ) };
	// 8316CF4C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316CF50: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8316CF54: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8316CF58: 4082FFE8  bne 0x8316cf40
	if !ctx.cr[0].eq {
	pc = 0x8316CF40; continue 'dispatch;
	}
	// 8316CF5C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316CF60: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8316CF64: 38AB89C8  addi r5, r11, -0x7638
	ctx.r[5].s64 = ctx.r[11].s64 + -30264;
	// 8316CF68: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316CF6C: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 8316CF70: 4BFF2D89  bl 0x8315fcf8
	ctx.lr = 0x8316CF74;
	sub_8315FCF8(ctx, base);
	// 8316CF74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316CF78: 41820034  beq 0x8316cfac
	if ctx.cr[0].eq {
	pc = 0x8316CFAC; continue 'dispatch;
	}
	// 8316CF7C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8316CF80: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8316CF84: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316CF88: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8316CF8C: 394A897C  addi r10, r10, -0x7684
	ctx.r[10].s64 = ctx.r[10].s64 + -30340;
	// 8316CF90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8316CF94: 93E3001C  stw r31, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 8316CF98: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8316CF9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316CFA0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316CFA4: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8316CFA8: F9630028  std r11, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 8316CFAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316CFB0: 419A0038  beq cr6, 0x8316cfe8
	if ctx.cr[6].eq {
	pc = 0x8316CFE8; continue 'dispatch;
	}
	// 8316CFB4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316CFB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316CFBC: 409A002C  bne cr6, 0x8316cfe8
	if !ctx.cr[6].eq {
	pc = 0x8316CFE8; continue 'dispatch;
	}
	// 8316CFC0: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 8316CFC4: 931F0018  stw r24, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8316CFC8: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8316CFCC: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8316CFD0: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 8316CFD4: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316CFD8: 4BFE7621  bl 0x831545f8
	ctx.lr = 0x8316CFDC;
	sub_831545F8(ctx, base);
	// 8316CFDC: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 8316CFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316CFE4: 4800005C  b 0x8316d040
	pc = 0x8316D040; continue 'dispatch;
	// 8316CFE8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316CFEC: 419A0034  beq cr6, 0x8316d020
	if ctx.cr[6].eq {
	pc = 0x8316D020; continue 'dispatch;
	}
	// 8316CFF0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316CFF4: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 8316CFF8: 3BAB8328  addi r29, r11, -0x7cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -31960;
	// 8316CFFC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D000: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316D004: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D008: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D00C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D010: 4E800421  bctrl
	ctx.lr = 0x8316D014;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D014: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8316D018: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8316D01C: 4082FFE0  bne 0x8316cffc
	if !ctx.cr[0].eq {
	pc = 0x8316CFFC; continue 'dispatch;
	}
	// 8316D020: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316D024: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8316D028: 4BFF2741  bl 0x8315f768
	ctx.lr = 0x8316D02C;
	sub_8315F768(ctx, base);
	// 8316D02C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D030: 388B89A0  addi r4, r11, -0x7660
	ctx.r[4].s64 = ctx.r[11].s64 + -30304;
	// 8316D034: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316D038: 4BFF2AE1  bl 0x8315fb18
	ctx.lr = 0x8316D03C;
	sub_8315FB18(ctx, base);
	// 8316D03C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316D040: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8316D044: 4803B164  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D048 size=200
    let mut pc: u32 = 0x8316D048;
    'dispatch: loop {
        match pc {
            0x8316D048 => {
    //   block [0x8316D048..0x8316D110)
	// 8316D048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D04C: 4803B11D  bl 0x831a8168
	ctx.lr = 0x8316D050;
	sub_831A8130(ctx, base);
	// 8316D050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D058: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8316D05C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316D060: 409A001C  bne cr6, 0x8316d07c
	if !ctx.cr[6].eq {
	pc = 0x8316D07C; continue 'dispatch;
	}
	// 8316D064: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D068: 388B8A74  addi r4, r11, -0x758c
	ctx.r[4].s64 = ctx.r[11].s64 + -30092;
	// 8316D06C: 4BFF2AAD  bl 0x8315fb18
	ctx.lr = 0x8316D070;
	sub_8315FB18(ctx, base);
	// 8316D070: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D074: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D078: 48000090  b 0x8316d108
	pc = 0x8316D108; continue 'dispatch;
	// 8316D07C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316D080: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D084: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8316D088: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 8316D08C: 4BFF8A95  bl 0x83165b20
	ctx.lr = 0x8316D090;
	sub_83165B20(ctx, base);
	// 8316D090: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D094: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316D098: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D09C: 40990048  ble cr6, 0x8316d0e4
	if !ctx.cr[6].gt {
	pc = 0x8316D0E4; continue 'dispatch;
	}
	// 8316D0A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316D0A4: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316D0A8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8316D0AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316D0B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8316D0B4: 419A001C  beq cr6, 0x8316d0d0
	if ctx.cr[6].eq {
	pc = 0x8316D0D0; continue 'dispatch;
	}
	// 8316D0B8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8316D0BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316D0C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D0C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D0C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D0CC: 4E800421  bctrl
	ctx.lr = 0x8316D0D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D0D0: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D0D4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8316D0D8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 8316D0DC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8316D0E0: 4198FFC4  blt cr6, 0x8316d0a4
	if ctx.cr[6].lt {
	pc = 0x8316D0A4; continue 'dispatch;
	}
	// 8316D0E4: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316D0E8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316D0EC: 4BFF267D  bl 0x8315f768
	ctx.lr = 0x8316D0F0;
	sub_8315F768(ctx, base);
	// 8316D0F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D0F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8316D0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D0FC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D104: 4E800421  bctrl
	ctx.lr = 0x8316D108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316D10C: 4803B0AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D110 size=32
    let mut pc: u32 = 0x8316D110;
    'dispatch: loop {
        match pc {
            0x8316D110 => {
    //   block [0x8316D110..0x8316D130)
	// 8316D110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316D114: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D118: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316D11C: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 8316D120: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 8316D124: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8316D128: F9630028  std r11, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 8316D12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D130 size=16
    let mut pc: u32 = 0x8316D130;
    'dispatch: loop {
        match pc {
            0x8316D130 => {
    //   block [0x8316D130..0x8316D140)
	// 8316D130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D134: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D138: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


