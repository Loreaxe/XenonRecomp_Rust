pub fn sub_82DD0890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0890 size=344
    let mut pc: u32 = 0x82DD0890;
    'dispatch: loop {
        match pc {
            0x82DD0890 => {
    //   block [0x82DD0890..0x82DD08F0)
	// 82DD0890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0894: 4BED8B71  bl 0x82ca9404
	ctx.lr = 0x82DD0898;
	sub_82CA93D0(ctx, base);
	// 82DD0898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD089C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DD08A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD08A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DD08A8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DD08AC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DD08B0: 419A00E8  beq cr6, 0x82dd0998
	if ctx.cr[6].eq {
	pc = 0x82DD0998; continue 'dispatch;
	}
	// 82DD08B4: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD08B8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD08BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD08C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD08C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD08C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD08CC: 4E800421  bctrl
	ctx.lr = 0x82DD08D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD08D0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82DD08D4: 41980088  blt cr6, 0x82dd095c
	if ctx.cr[6].lt {
	pc = 0x82DD095C; continue 'dispatch;
	}
	// 82DD08D8: 419A0064  beq cr6, 0x82dd093c
	if ctx.cr[6].eq {
	pc = 0x82DD093C; continue 'dispatch;
	}
	// 82DD08DC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82DD08E0: 41980010  blt cr6, 0x82dd08f0
	if ctx.cr[6].lt {
	pc = 0x82DD08F0; continue 'dispatch;
	}
	// 82DD08E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DD08E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD08EC: 4BED8B68  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD08F0 => {
    //   block [0x82DD08F0..0x82DD093C)
	// 82DD08F0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD08F4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD08F8: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD08FC: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DD0900: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0904: 4BF84945  bl 0x82d55248
	ctx.lr = 0x82DD0908;
	sub_82D55248(ctx, base);
	// 82DD0908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD090C: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82DD0910: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DD0914: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0918: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD091C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0920: 48013AE9  bl 0x82de4408
	ctx.lr = 0x82DD0924;
	sub_82DE4408(ctx, base);
	// 82DD0924: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD0928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD092C: 396B35D0  addi r11, r11, 0x35d0
	ctx.r[11].s64 = ctx.r[11].s64 + 13776;
	// 82DD0930: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD0934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0938: 4BED8B1C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD093C => {
    //   block [0x82DD093C..0x82DD095C)
	// 82DD093C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0940: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0944: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0948: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD094C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0950: 4BF848F9  bl 0x82d55248
	ctx.lr = 0x82DD0954;
	sub_82D55248(ctx, base);
	// 82DD0954: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0958: 4800005C  b 0x82dd09b4
	pc = 0x82DD09B4; continue 'dispatch;
            }
            0x82DD095C => {
    //   block [0x82DD095C..0x82DD0998)
	// 82DD095C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0960: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0964: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0968: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82DD096C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0970: 4BF848D9  bl 0x82d55248
	ctx.lr = 0x82DD0974;
	sub_82D55248(ctx, base);
	// 82DD0974: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82DD0978: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD097C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD0980: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0984: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0988: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD098C: 4BFFF1E5  bl 0x82dcfb70
	ctx.lr = 0x82DD0990;
	sub_82DCFB70(ctx, base);
	// 82DD0990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0994: 4BED8AC0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD0998 => {
    //   block [0x82DD0998..0x82DD09B4)
	// 82DD0998: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD099C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD09A0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD09A4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD09A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD09AC: 4BF8489D  bl 0x82d55248
	ctx.lr = 0x82DD09B0;
	sub_82D55248(ctx, base);
	// 82DD09B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	pc = 0x82DD09B4; continue 'dispatch;
            }
            0x82DD09B4 => {
    //   block [0x82DD09B4..0x82DD09E8)
	// 82DD09B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD09B8: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DD09BC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD09C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD09C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD09C8: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD09CC: 4800CD75  bl 0x82ddd740
	ctx.lr = 0x82DD09D0;
	sub_82DDD740(ctx, base);
	// 82DD09D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD09D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD09D8: 396B3304  addi r11, r11, 0x3304
	ctx.r[11].s64 = ctx.r[11].s64 + 13060;
	// 82DD09DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD09E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD09E4: 4BED8A70  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD09E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD09E8 size=540
    let mut pc: u32 = 0x82DD09E8;
    'dispatch: loop {
        match pc {
            0x82DD09E8 => {
    //   block [0x82DD09E8..0x82DD0A74)
	// 82DD09E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD09EC: 4BED8A19  bl 0x82ca9404
	ctx.lr = 0x82DD09F0;
	sub_82CA93D0(ctx, base);
	// 82DD09F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD09F4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DD09F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD09FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DD0A00: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DD0A04: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DD0A08: 419A01C0  beq cr6, 0x82dd0bc8
	if ctx.cr[6].eq {
	pc = 0x82DD0BC8; continue 'dispatch;
	}
	// 82DD0A0C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0A10: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DD0A14: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0A18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0A1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0A20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0A28: 4E800421  bctrl
	ctx.lr = 0x82DD0A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD0A2C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82DD0A30: 41980148  blt cr6, 0x82dd0b78
	if ctx.cr[6].lt {
	pc = 0x82DD0B78; continue 'dispatch;
	}
	// 82DD0A34: 419A0124  beq cr6, 0x82dd0b58
	if ctx.cr[6].eq {
	pc = 0x82DD0B58; continue 'dispatch;
	}
	// 82DD0A38: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82DD0A3C: 40980038  bge cr6, 0x82dd0a74
	if !ctx.cr[6].lt {
	pc = 0x82DD0A74; continue 'dispatch;
	}
	// 82DD0A40: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0A44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DD0A48: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD0A4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD0A50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0A54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0A58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0A5C: 4E800421  bctrl
	ctx.lr = 0x82DD0A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD0A60: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82DD0A64: 419800B8  blt cr6, 0x82dd0b1c
	if ctx.cr[6].lt {
	pc = 0x82DD0B1C; continue 'dispatch;
	}
	// 82DD0A68: 419A0064  beq cr6, 0x82dd0acc
	if ctx.cr[6].eq {
	pc = 0x82DD0ACC; continue 'dispatch;
	}
	// 82DD0A6C: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82DD0A70: 41980010  blt cr6, 0x82dd0a80
	if ctx.cr[6].lt {
	pc = 0x82DD0A80; continue 'dispatch;
	}
            }
            0x82DD0A74 => {
    //   block [0x82DD0A74..0x82DD0A80)
	// 82DD0A74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DD0A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0A7C: 4BED89D8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD0A80 => {
    //   block [0x82DD0A80..0x82DD0ACC)
	// 82DD0A80: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0A84: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0A88: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0A8C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DD0A90: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0A94: 4BF847B5  bl 0x82d55248
	ctx.lr = 0x82DD0A98;
	sub_82D55248(ctx, base);
	// 82DD0A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD0A9C: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82DD0AA0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DD0AA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0AA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0AAC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0AB0: 48013959  bl 0x82de4408
	ctx.lr = 0x82DD0AB4;
	sub_82DE4408(ctx, base);
	// 82DD0AB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD0AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD0ABC: 396B35D0  addi r11, r11, 0x35d0
	ctx.r[11].s64 = ctx.r[11].s64 + 13776;
	// 82DD0AC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD0AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0AC8: 4BED898C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD0ACC => {
    //   block [0x82DD0ACC..0x82DD0B1C)
	// 82DD0ACC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0AD0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0AD4: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0AD8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD0ADC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0AE0: 4BF84769  bl 0x82d55248
	ctx.lr = 0x82DD0AE4;
	sub_82D55248(ctx, base);
	// 82DD0AE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DD0AE8: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DD0AEC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0AF0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DD0AF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD0AF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD0AFC: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0B00: 4800CC41  bl 0x82ddd740
	ctx.lr = 0x82DD0B04;
	sub_82DDD740(ctx, base);
	// 82DD0B04: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD0B08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DD0B0C: 396B3304  addi r11, r11, 0x3304
	ctx.r[11].s64 = ctx.r[11].s64 + 13060;
	// 82DD0B10: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD0B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0B18: 4BED893C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD0B1C => {
    //   block [0x82DD0B1C..0x82DD0B58)
	// 82DD0B1C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0B20: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0B24: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0B28: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82DD0B2C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0B30: 4BF84719  bl 0x82d55248
	ctx.lr = 0x82DD0B34;
	sub_82D55248(ctx, base);
	// 82DD0B34: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82DD0B38: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0B3C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DD0B40: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0B44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0B48: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0B4C: 4BFFF025  bl 0x82dcfb70
	ctx.lr = 0x82DD0B50;
	sub_82DCFB70(ctx, base);
	// 82DD0B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0B54: 4BED8900  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD0B58 => {
    //   block [0x82DD0B58..0x82DD0B78)
	// 82DD0B58: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0B5C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0B60: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0B64: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD0B68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0B6C: 4BF846DD  bl 0x82d55248
	ctx.lr = 0x82DD0B70;
	sub_82D55248(ctx, base);
	// 82DD0B70: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0B74: 48000070  b 0x82dd0be4
	pc = 0x82DD0BE4; continue 'dispatch;
            }
            0x82DD0B78 => {
    //   block [0x82DD0B78..0x82DD0BC8)
	// 82DD0B78: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0B7C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0B80: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0B84: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82DD0B88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0B8C: 4BF846BD  bl 0x82d55248
	ctx.lr = 0x82DD0B90;
	sub_82D55248(ctx, base);
	// 82DD0B90: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DD0B94: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82DD0B98: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0B9C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DD0BA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD0BA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD0BA8: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0BAC: 4BFFEFC5  bl 0x82dcfb70
	ctx.lr = 0x82DD0BB0;
	sub_82DCFB70(ctx, base);
	// 82DD0BB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD0BB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DD0BB8: 396B36C4  addi r11, r11, 0x36c4
	ctx.r[11].s64 = ctx.r[11].s64 + 14020;
	// 82DD0BBC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD0BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0BC4: 4BED8890  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD0BC8 => {
    //   block [0x82DD0BC8..0x82DD0BE4)
	// 82DD0BC8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0BCC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0BD0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0BD4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD0BD8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0BDC: 4BF8466D  bl 0x82d55248
	ctx.lr = 0x82DD0BE0;
	sub_82D55248(ctx, base);
	// 82DD0BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	pc = 0x82DD0BE4; continue 'dispatch;
            }
            0x82DD0BE4 => {
    //   block [0x82DD0BE4..0x82DD0C04)
	// 82DD0BE4: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DD0BE8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DD0BEC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0BF4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0BF8: 4800CB49  bl 0x82ddd740
	ctx.lr = 0x82DD0BFC;
	sub_82DDD740(ctx, base);
	// 82DD0BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0C00: 4BED8854  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0C08 size=256
    let mut pc: u32 = 0x82DD0C08;
    'dispatch: loop {
        match pc {
            0x82DD0C08 => {
    //   block [0x82DD0C08..0x82DD0D08)
	// 82DD0C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0C0C: 4BED8801  bl 0x82ca940c
	ctx.lr = 0x82DD0C10;
	sub_82CA93D0(ctx, base);
	// 82DD0C10: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0C14: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD0C18: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD0C1C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD0C20: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD0C24: 39080108  addi r8, r8, 0x108
	ctx.r[8].s64 = ctx.r[8].s64 + 264;
	// 82DD0C28: 392913D0  addi r9, r9, 0x13d0
	ctx.r[9].s64 = ctx.r[9].s64 + 5072;
	// 82DD0C2C: 394A1420  addi r10, r10, 0x1420
	ctx.r[10].s64 = ctx.r[10].s64 + 5152;
	// 82DD0C30: 396B1470  addi r11, r11, 0x1470
	ctx.r[11].s64 = ctx.r[11].s64 + 5232;
	// 82DD0C34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD0C38: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DD0C3C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD0C40: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 82DD0C44: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD0C48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD0C4C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD0C50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD0C54: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD0C58: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DD0C5C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD0C60: 4BFF0509  bl 0x82dc1168
	ctx.lr = 0x82DD0C64;
	sub_82DC1168(ctx, base);
	// 82DD0C64: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD0C68: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD0C6C: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD0C70: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD0C74: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD0C78: 39080890  addi r8, r8, 0x890
	ctx.r[8].s64 = ctx.r[8].s64 + 2192;
	// 82DD0C7C: 3929FE68  addi r9, r9, -0x198
	ctx.r[9].s64 = ctx.r[9].s64 + -408;
	// 82DD0C80: 394AFC00  addi r10, r10, -0x400
	ctx.r[10].s64 = ctx.r[10].s64 + -1024;
	// 82DD0C84: 396BFFB8  addi r11, r11, -0x48
	ctx.r[11].s64 = ctx.r[11].s64 + -72;
	// 82DD0C88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DD0C8C: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82DD0C90: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD0C94: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DD0C98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD0C9C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD0CA0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD0CA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD0CA8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD0CAC: 9BA10080  stb r29, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u8 ) };
	// 82DD0CB0: 4BFF04B9  bl 0x82dc1168
	ctx.lr = 0x82DD0CB4;
	sub_82DC1168(ctx, base);
	// 82DD0CB4: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD0CB8: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD0CBC: 9BA100A0  stb r29, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[29].u8 ) };
	// 82DD0CC0: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD0CC4: 9BC100A1  stb r30, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[30].u8 ) };
	// 82DD0CC8: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD0CCC: 390809E8  addi r8, r8, 0x9e8
	ctx.r[8].s64 = ctx.r[8].s64 + 2536;
	// 82DD0CD0: 3929FE68  addi r9, r9, -0x198
	ctx.r[9].s64 = ctx.r[9].s64 + -408;
	// 82DD0CD4: 394AFC00  addi r10, r10, -0x400
	ctx.r[10].s64 = ctx.r[10].s64 + -1024;
	// 82DD0CD8: 396BFFB8  addi r11, r11, -0x48
	ctx.r[11].s64 = ctx.r[11].s64 + -72;
	// 82DD0CDC: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82DD0CE0: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 82DD0CE4: 91010090  stw r8, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82DD0CE8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82DD0CEC: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82DD0CF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD0CF4: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82DD0CF8: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82DD0CFC: 4BFF046D  bl 0x82dc1168
	ctx.lr = 0x82DD0D00;
	sub_82DC1168(ctx, base);
	// 82DD0D00: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DD0D04: 4BED8758  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD0D08 size=20
    let mut pc: u32 = 0x82DD0D08;
    'dispatch: loop {
        match pc {
            0x82DD0D08 => {
    //   block [0x82DD0D08..0x82DD0D1C)
	// 82DD0D08: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0D0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0D10: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DD0D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0D18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD0D20 size=20
    let mut pc: u32 = 0x82DD0D20;
    'dispatch: loop {
        match pc {
            0x82DD0D20 => {
    //   block [0x82DD0D20..0x82DD0D34)
	// 82DD0D20: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0D28: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD0D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0D30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD0D38 size=20
    let mut pc: u32 = 0x82DD0D38;
    'dispatch: loop {
        match pc {
            0x82DD0D38 => {
    //   block [0x82DD0D38..0x82DD0D4C)
	// 82DD0D38: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0D3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0D40: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD0D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0D48: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD0D50 size=20
    let mut pc: u32 = 0x82DD0D50;
    'dispatch: loop {
        match pc {
            0x82DD0D50 => {
    //   block [0x82DD0D50..0x82DD0D64)
	// 82DD0D50: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0D54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0D58: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DD0D5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0D60: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD0D68 size=20
    let mut pc: u32 = 0x82DD0D68;
    'dispatch: loop {
        match pc {
            0x82DD0D68 => {
    //   block [0x82DD0D68..0x82DD0D7C)
	// 82DD0D68: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0D6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0D70: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DD0D74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0D78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0D80 size=76
    let mut pc: u32 = 0x82DD0D80;
    'dispatch: loop {
        match pc {
            0x82DD0D80 => {
    //   block [0x82DD0D80..0x82DD0DCC)
	// 82DD0D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD0D88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0D8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD0D90: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD0D94: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD0D98: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD0D9C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD0DA0: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD0DA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD0DA8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD0DAC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD0DB0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD0DB4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD0DB8: 4800C569  bl 0x82ddd320
	ctx.lr = 0x82DD0DBC;
	sub_82DDD320(ctx, base);
	// 82DD0DBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD0DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD0DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD0DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD0DD0 size=80
    let mut pc: u32 = 0x82DD0DD0;
    'dispatch: loop {
        match pc {
            0x82DD0DD0 => {
    //   block [0x82DD0DD0..0x82DD0E20)
	// 82DD0DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD0DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0DDC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD0DE0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD0DE4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD0DE8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD0DEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD0DF0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD0DF4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD0DF8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD0DFC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD0E00: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD0E04: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD0E08: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD0E0C: 4800BF7D  bl 0x82ddcd88
	ctx.lr = 0x82DD0E10;
	sub_82DDCD88(ctx, base);
	// 82DD0E10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD0E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD0E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD0E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0E20 size=232
    let mut pc: u32 = 0x82DD0E20;
    'dispatch: loop {
        match pc {
            0x82DD0E20 => {
    //   block [0x82DD0E20..0x82DD0E4C)
	// 82DD0E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD0E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD0E2C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD0E34: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD0E38: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DD0E3C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DD0E40: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DD0E44: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DD0E48: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD0E4C; continue 'dispatch;
            }
            0x82DD0E4C => {
    //   block [0x82DD0E4C..0x82DD0F08)
	// 82DD0E4C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD0E50: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DD0E54: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DD0E58: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DD0E5C: 4200FFF0  bdnz 0x82dd0e4c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD0E4C; continue 'dispatch;
	}
	// 82DD0E60: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD0E64: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DD0E68: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DD0E6C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DD0E70: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD0E74: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0F08 size=140
    let mut pc: u32 = 0x82DD0F08;
    'dispatch: loop {
        match pc {
            0x82DD0F08 => {
    //   block [0x82DD0F08..0x82DD0F64)
	// 82DD0F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD0F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD0F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0F18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD0F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD0F20: 396B325C  addi r11, r11, 0x325c
	ctx.r[11].s64 = ctx.r[11].s64 + 12892;
	// 82DD0F24: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DD0F28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DD0F2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD0F30: 419A004C  beq cr6, 0x82dd0f7c
	if ctx.cr[6].eq {
	pc = 0x82DD0F7C; continue 'dispatch;
	}
	// 82DD0F34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0F38: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0F3C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0F40: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DD0F44: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DD0F48: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DD0F4C: 41980018  blt cr6, 0x82dd0f64
	if ctx.cr[6].lt {
	pc = 0x82DD0F64; continue 'dispatch;
	}
	// 82DD0F50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DD0F54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD0F58: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DD0F5C: 4BF841CD  bl 0x82d55128
	ctx.lr = 0x82DD0F60;
	sub_82D55128(ctx, base);
	// 82DD0F60: 4800001C  b 0x82dd0f7c
	pc = 0x82DD0F7C; continue 'dispatch;
            }
            0x82DD0F64 => {
    //   block [0x82DD0F64..0x82DD0F7C)
	// 82DD0F64: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DD0F68: 812B0040  lwz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DD0F6C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DD0F70: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DD0F74: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD0F78: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	pc = 0x82DD0F7C; continue 'dispatch;
            }
            0x82DD0F7C => {
    //   block [0x82DD0F7C..0x82DD0F94)
	// 82DD0F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD0F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD0F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD0F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD0F8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD0F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD0F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD0F98 size=288
    let mut pc: u32 = 0x82DD0F98;
    'dispatch: loop {
        match pc {
            0x82DD0F98 => {
    //   block [0x82DD0F98..0x82DD0FD8)
	// 82DD0F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD0FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD0FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD0FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0FAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD0FB0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD0FB4: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82DD0FB8: C1BE1030  lfs f13, 0x1030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DD0FBC: C01F3030  lfs f0, 0x3030(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD0FC0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DD0FC4: 419A00BC  beq cr6, 0x82dd1080
	if ctx.cr[6].eq {
	pc = 0x82DD1080; continue 'dispatch;
	}
	// 82DD0FC8: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0FCC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD0FD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD0FD4: 419A0014  beq cr6, 0x82dd0fe8
	if ctx.cr[6].eq {
	pc = 0x82DD0FE8; continue 'dispatch;
	}
	pc = 0x82DD0FD8; continue 'dispatch;
            }
            0x82DD0FD8 => {
    //   block [0x82DD0FD8..0x82DD0FE8)
	// 82DD0FD8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD0FDC: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0FE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD0FE4: 409AFFF4  bne cr6, 0x82dd0fd8
	if !ctx.cr[6].eq {
	pc = 0x82DD0FD8; continue 'dispatch;
	}
	pc = 0x82DD0FE8; continue 'dispatch;
            }
            0x82DD0FE8 => {
    //   block [0x82DD0FE8..0x82DD0FF4)
	// 82DD0FE8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0FEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD0FF0: 419A0014  beq cr6, 0x82dd1004
	if ctx.cr[6].eq {
	pc = 0x82DD1004; continue 'dispatch;
	}
	pc = 0x82DD0FF4; continue 'dispatch;
            }
            0x82DD0FF4 => {
    //   block [0x82DD0FF4..0x82DD1004)
	// 82DD0FF4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD0FF8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0FFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD1000: 409AFFF4  bne cr6, 0x82dd0ff4
	if !ctx.cr[6].eq {
	pc = 0x82DD0FF4; continue 'dispatch;
	}
	pc = 0x82DD1004; continue 'dispatch;
            }
            0x82DD1004 => {
    //   block [0x82DD1004..0x82DD1080)
	// 82DD1004: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1008: 38FF3050  addi r7, r31, 0x3050
	ctx.r[7].s64 = ctx.r[31].s64 + 12368;
	// 82DD100C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DD1010: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD1014: 4E800421  bctrl
	ctx.lr = 0x82DD1018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD1018: 397E1010  addi r11, r30, 0x1010
	ctx.r[11].s64 = ctx.r[30].s64 + 4112;
	// 82DD101C: 395F3010  addi r10, r31, 0x3010
	ctx.r[10].s64 = ctx.r[31].s64 + 12304;
	// 82DD1020: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82DD1024: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 82DD1028: 390A0030  addi r8, r10, 0x30
	ctx.r[8].s64 = ctx.r[10].s64 + 48;
            }
            0x82DD1080 => {
    //   block [0x82DD1080..0x82DD10B8)
	// 82DD1080: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1084: 807F3080  lwz r3, 0x3080(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12416 as u32) ) } as u64;
	// 82DD1088: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DD108C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD1090: 419A0010  beq cr6, 0x82dd10a0
	if ctx.cr[6].eq {
	pc = 0x82DD10A0; continue 'dispatch;
	}
	// 82DD1094: 38A01008  li r5, 0x1008
	ctx.r[5].s64 = 4104;
	// 82DD1098: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82DD109C: 4BED83E5  bl 0x82ca9480
	ctx.lr = 0x82DD10A0;
	sub_82CA9480(ctx, base);
	// 82DD10A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD10A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD10A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD10AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD10B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD10B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD10B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD10B8 size=376
    let mut pc: u32 = 0x82DD10B8;
    'dispatch: loop {
        match pc {
            0x82DD10B8 => {
    //   block [0x82DD10B8..0x82DD1110)
	// 82DD10B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD10BC: 4BED8339  bl 0x82ca93f4
	ctx.lr = 0x82DD10C0;
	sub_82CA93D0(ctx, base);
	// 82DD10C0: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD10C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD10C8: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 82DD10CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DD10D0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DD10D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DD10D8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD10DC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD10E0: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DD10E4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82DD10E8: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82DD10EC: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD10F0: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 82DD10F4: 409A006C  bne cr6, 0x82dd1160
	if !ctx.cr[6].eq {
	pc = 0x82DD1160; continue 'dispatch;
	}
	// 82DD10F8: 891F0008  lbz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD10FC: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82DD1100: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1104: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82DD1108: 40990024  ble cr6, 0x82dd112c
	if !ctx.cr[6].gt {
	pc = 0x82DD112C; continue 'dispatch;
	}
	// 82DD110C: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	pc = 0x82DD1110; continue 'dispatch;
            }
            0x82DD1110 => {
    //   block [0x82DD1110..0x82DD112C)
	// 82DD1110: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1114: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DD1118: 409A0108  bne cr6, 0x82dd1220
	if !ctx.cr[6].eq {
	pc = 0x82DD1220; continue 'dispatch;
	}
	// 82DD111C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DD1120: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DD1124: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DD1128: 4198FFE8  blt cr6, 0x82dd1110
	if ctx.cr[6].lt {
	pc = 0x82DD1110; continue 'dispatch;
	}
	pc = 0x82DD112C; continue 'dispatch;
            }
            0x82DD112C => {
    //   block [0x82DD112C..0x82DD1160)
	// 82DD112C: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD1130: 38690014  addi r3, r9, 0x14
	ctx.r[3].s64 = ctx.r[9].s64 + 20;
	// 82DD1134: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DD1138: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD113C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD1140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD1144: 4E800421  bctrl
	ctx.lr = 0x82DD1148;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD1148: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD114C: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82DD1150: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82DD1154: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82DD1158: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DD115C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
            }
            0x82DD1160 => {
    //   block [0x82DD1160..0x82DD119C)
	// 82DD1160: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1164: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD1168: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 82DD116C: 409A0080  bne cr6, 0x82dd11ec
	if !ctx.cr[6].eq {
	pc = 0x82DD11EC; continue 'dispatch;
	}
	// 82DD1170: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD1174: 897F0009  lbz r11, 9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82DD1178: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82DD117C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DD1180: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82DD1184: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82DD1188: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DD118C: 7FCAF8AE  lbzx r30, r10, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DD1190: 40980028  bge cr6, 0x82dd11b8
	if !ctx.cr[6].lt {
	pc = 0x82DD11B8; continue 'dispatch;
	}
	// 82DD1194: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DD1198: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	pc = 0x82DD119C; continue 'dispatch;
            }
            0x82DD119C => {
    //   block [0x82DD119C..0x82DD11B8)
	// 82DD119C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD11A0: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DD11A4: 409A007C  bne cr6, 0x82dd1220
	if !ctx.cr[6].eq {
	pc = 0x82DD1220; continue 'dispatch;
	}
	// 82DD11A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DD11AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DD11B0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DD11B4: 4198FFE8  blt cr6, 0x82dd119c
	if ctx.cr[6].lt {
	pc = 0x82DD119C; continue 'dispatch;
	}
	pc = 0x82DD11B8; continue 'dispatch;
            }
            0x82DD11B8 => {
    //   block [0x82DD11B8..0x82DD11EC)
	// 82DD11B8: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD11BC: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 82DD11C0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DD11C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD11C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD11CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD11D0: 4E800421  bctrl
	ctx.lr = 0x82DD11D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD11D4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD11D8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82DD11DC: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 82DD11E0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82DD11E4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DD11E8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
            }
            0x82DD11EC => {
    //   block [0x82DD11EC..0x82DD1220)
	// 82DD11EC: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD11F0: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82DD11F4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DD11F8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DD11FC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DD1200: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD1204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1208: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD120C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD1210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD1214: 4E800421  bctrl
	ctx.lr = 0x82DD1218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD1218: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DD121C: 4BED8228  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD1220 => {
    //   block [0x82DD1220..0x82DD1230)
	// 82DD1220: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DD1224: 9AFB0010  stb r23, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 82DD1228: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DD122C: 4BED8218  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1230 size=412
    let mut pc: u32 = 0x82DD1230;
    'dispatch: loop {
        match pc {
            0x82DD1230 => {
    //   block [0x82DD1230..0x82DD1290)
	// 82DD1230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1234: 4BED81C1  bl 0x82ca93f4
	ctx.lr = 0x82DD1238;
	sub_82CA93D0(ctx, base);
	// 82DD1238: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82DD123C: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1240: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD1244: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DD1248: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 82DD124C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DD1250: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DD1254: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DD1258: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD125C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD1260: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82DD1264: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82DD1268: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82DD126C: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD1270: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 82DD1274: 409A006C  bne cr6, 0x82dd12e0
	if !ctx.cr[6].eq {
	pc = 0x82DD12E0; continue 'dispatch;
	}
	// 82DD1278: 891F0008  lbz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD127C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82DD1280: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1284: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82DD1288: 40990024  ble cr6, 0x82dd12ac
	if !ctx.cr[6].gt {
	pc = 0x82DD12AC; continue 'dispatch;
	}
	// 82DD128C: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	pc = 0x82DD1290; continue 'dispatch;
            }
            0x82DD1290 => {
    //   block [0x82DD1290..0x82DD12AC)
	// 82DD1290: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1294: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DD1298: 409A0120  bne cr6, 0x82dd13b8
	if !ctx.cr[6].eq {
	pc = 0x82DD13B8; continue 'dispatch;
	}
	// 82DD129C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DD12A0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DD12A4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DD12A8: 4198FFE8  blt cr6, 0x82dd1290
	if ctx.cr[6].lt {
	pc = 0x82DD1290; continue 'dispatch;
	}
	pc = 0x82DD12AC; continue 'dispatch;
            }
            0x82DD12AC => {
    //   block [0x82DD12AC..0x82DD12E0)
	// 82DD12AC: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD12B0: 38690014  addi r3, r9, 0x14
	ctx.r[3].s64 = ctx.r[9].s64 + 20;
	// 82DD12B4: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DD12B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD12BC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD12C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD12C4: 4E800421  bctrl
	ctx.lr = 0x82DD12C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD12C8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD12CC: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82DD12D0: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
	// 82DD12D4: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82DD12D8: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82DD12DC: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
            }
            0x82DD12E0 => {
    //   block [0x82DD12E0..0x82DD131C)
	// 82DD12E0: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD12E4: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD12E8: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 82DD12EC: 409A0080  bne cr6, 0x82dd136c
	if !ctx.cr[6].eq {
	pc = 0x82DD136C; continue 'dispatch;
	}
	// 82DD12F0: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD12F4: 897F0009  lbz r11, 9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82DD12F8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82DD12FC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DD1300: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82DD1304: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82DD1308: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DD130C: 7FCAF8AE  lbzx r30, r10, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DD1310: 40980028  bge cr6, 0x82dd1338
	if !ctx.cr[6].lt {
	pc = 0x82DD1338; continue 'dispatch;
	}
	// 82DD1314: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DD1318: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	pc = 0x82DD131C; continue 'dispatch;
            }
            0x82DD131C => {
    //   block [0x82DD131C..0x82DD1338)
	// 82DD131C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1320: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DD1324: 409A0094  bne cr6, 0x82dd13b8
	if !ctx.cr[6].eq {
	pc = 0x82DD13B8; continue 'dispatch;
	}
	// 82DD1328: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DD132C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DD1330: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DD1334: 4198FFE8  blt cr6, 0x82dd131c
	if ctx.cr[6].lt {
	pc = 0x82DD131C; continue 'dispatch;
	}
	pc = 0x82DD1338; continue 'dispatch;
            }
            0x82DD1338 => {
    //   block [0x82DD1338..0x82DD136C)
	// 82DD1338: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD133C: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 82DD1340: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DD1344: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD1348: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD134C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD1350: 4E800421  bctrl
	ctx.lr = 0x82DD1354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD1354: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD1358: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82DD135C: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 82DD1360: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82DD1364: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DD1368: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
            }
            0x82DD136C => {
    //   block [0x82DD136C..0x82DD13B8)
	// 82DD136C: 8161033C  lwz r11, 0x33c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(828 as u32) ) } as u64;
	// 82DD1370: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DD1374: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD1378: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82DD137C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DD1380: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DD1384: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DD1388: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD138C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD1390: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD1394: 81610334  lwz r11, 0x334(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(820 as u32) ) } as u64;
	// 82DD1398: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DD139C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD13A0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DD13A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD13A8: 4E800421  bctrl
	ctx.lr = 0x82DD13AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD13AC: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 82DD13B0: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82DD13B4: 4BED8090  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DD13B8 => {
    //   block [0x82DD13B8..0x82DD13CC)
	// 82DD13B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD13BC: 9AFB0010  stb r23, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 82DD13C0: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 82DD13C4: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82DD13C8: 4BED807C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD13D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD13D0 size=76
    let mut pc: u32 = 0x82DD13D0;
    'dispatch: loop {
        match pc {
            0x82DD13D0 => {
    //   block [0x82DD13D0..0x82DD141C)
	// 82DD13D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD13D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD13D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD13DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD13E0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD13E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD13E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD13EC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD13F0: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD13F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD13F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD13FC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD1400: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD1404: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD1408: 4BFFEA61  bl 0x82dcfe68
	ctx.lr = 0x82DD140C;
	sub_82DCFE68(ctx, base);
	// 82DD140C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1420 size=80
    let mut pc: u32 = 0x82DD1420;
    'dispatch: loop {
        match pc {
            0x82DD1420 => {
    //   block [0x82DD1420..0x82DD1470)
	// 82DD1420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD142C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD1430: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD1434: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD1438: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD143C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD1440: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD1444: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD1448: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD144C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD1450: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD1454: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD1458: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD145C: 4BFFE7A5  bl 0x82dcfc00
	ctx.lr = 0x82DD1460;
	sub_82DCFC00(ctx, base);
	// 82DD1460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD146C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1470 size=232
    let mut pc: u32 = 0x82DD1470;
    'dispatch: loop {
        match pc {
            0x82DD1470 => {
    //   block [0x82DD1470..0x82DD149C)
	// 82DD1470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD147C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD1484: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD1488: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DD148C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DD1490: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DD1494: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DD1498: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD149C; continue 'dispatch;
            }
            0x82DD149C => {
    //   block [0x82DD149C..0x82DD1558)
	// 82DD149C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD14A0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DD14A4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DD14A8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DD14AC: 4200FFF0  bdnz 0x82dd149c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD149C; continue 'dispatch;
	}
	// 82DD14B0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD14B4: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DD14B8: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DD14BC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DD14C0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD14C4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1558 size=112
    let mut pc: u32 = 0x82DD1558;
    'dispatch: loop {
        match pc {
            0x82DD1558 => {
    //   block [0x82DD1558..0x82DD15B4)
	// 82DD1558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD1564: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD156C: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DD1570: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD1574: 409A0040  bne cr6, 0x82dd15b4
	if !ctx.cr[6].eq {
	pc = 0x82DD15B4; continue 'dispatch;
	}
	// 82DD1578: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD157C: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1580: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82DD1584: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82DD1588: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82DD158C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DD1590: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82DD1594: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1598: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD159C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD15A0: 4E800421  bctrl
	ctx.lr = 0x82DD15A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD15A4: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82DD15A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD15AC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DD15B0: 48040F71  bl 0x82e12520
	ctx.lr = 0x82DD15B4;
	sub_82E12520(ctx, base);
            }
            0x82DD15B4 => {
    //   block [0x82DD15B4..0x82DD15C8)
	// 82DD15B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD15B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD15BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD15C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD15C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD15C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD15C8 size=68
    let mut pc: u32 = 0x82DD15C8;
    'dispatch: loop {
        match pc {
            0x82DD15C8 => {
    //   block [0x82DD15C8..0x82DD160C)
	// 82DD15C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD15CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD15D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD15D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD15D8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD15DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD15E0: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DD15E4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD15E8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD15EC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD15F0: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DD15F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD15F8: 4BFFE871  bl 0x82dcfe68
	ctx.lr = 0x82DD15FC;
	sub_82DCFE68(ctx, base);
	// 82DD15FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1610 size=72
    let mut pc: u32 = 0x82DD1610;
    'dispatch: loop {
        match pc {
            0x82DD1610 => {
    //   block [0x82DD1610..0x82DD1658)
	// 82DD1610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD161C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1620: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD1624: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD1628: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DD162C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD1630: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD1634: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD1638: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD163C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD1640: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD1644: 4BFFE5BD  bl 0x82dcfc00
	ctx.lr = 0x82DD1648;
	sub_82DCFC00(ctx, base);
	// 82DD1648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1658 size=228
    let mut pc: u32 = 0x82DD1658;
    'dispatch: loop {
        match pc {
            0x82DD1658 => {
    //   block [0x82DD1658..0x82DD1678)
	// 82DD1658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1660: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1664: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD1668: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DD166C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DD1670: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DD1674: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD1678; continue 'dispatch;
            }
            0x82DD1678 => {
    //   block [0x82DD1678..0x82DD173C)
	// 82DD1678: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD167C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DD1680: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DD1684: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DD1688: 4200FFF0  bdnz 0x82dd1678
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD1678; continue 'dispatch;
	}
	// 82DD168C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD1690: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DD1694: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DD1698: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82DD169C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD16A0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1740 size=208
    let mut pc: u32 = 0x82DD1740;
    'dispatch: loop {
        match pc {
            0x82DD1740 => {
    //   block [0x82DD1740..0x82DD1810)
	// 82DD1740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD174C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1750: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DD1754: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD175C: 39443010  addi r10, r4, 0x3010
	ctx.r[10].s64 = ctx.r[4].s64 + 12304;
	// 82DD1760: 397F1010  addi r11, r31, 0x1010
	ctx.r[11].s64 = ctx.r[31].s64 + 4112;
	// 82DD1764: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DD1768: C1A90C64  lfs f13, 0xc64(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DD176C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DD1770: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD1774: 38EA0040  addi r7, r10, 0x40
	ctx.r[7].s64 = ctx.r[10].s64 + 64;
	// 82DD1778: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82DD177C: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DD1780: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 82DD1784: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82DD1788: C0090C18  lfs f0, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD178C: 392A0030  addi r9, r10, 0x30
	ctx.r[9].s64 = ctx.r[10].s64 + 48;
	// 82DD1790: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82DD1794: D00B0044  stfs f0, 0x44(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1810 size=176
    let mut pc: u32 = 0x82DD1810;
    'dispatch: loop {
        match pc {
            0x82DD1810 => {
    //   block [0x82DD1810..0x82DD1890)
	// 82DD1810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD181C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD1820: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD1824: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1828: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD182C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD1830: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD1834: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD1838: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD183C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD1840: 4BFFEAA9  bl 0x82dd02e8
	ctx.lr = 0x82DD1844;
	sub_82DD02E8(ctx, base);
	// 82DD1844: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1848: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD184C: 40980044  bge cr6, 0x82dd1890
	if !ctx.cr[6].lt {
	pc = 0x82DD1890; continue 'dispatch;
	}
	// 82DD1850: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1854: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD1858: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD1890; continue 'dispatch;
            }
            0x82DD1890 => {
    //   block [0x82DD1890..0x82DD18C0)
	// 82DD1890: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD1894: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD1898: 419A000C  beq cr6, 0x82dd18a4
	if ctx.cr[6].eq {
	pc = 0x82DD18A4; continue 'dispatch;
	}
	// 82DD189C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD18A0: 4BFF96E9  bl 0x82dcaf88
	ctx.lr = 0x82DD18A4;
	sub_82DCAF88(ctx, base);
	// 82DD18A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD18A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD18AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD18B0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD18B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD18B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD18BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD18C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD18C0 size=84
    let mut pc: u32 = 0x82DD18C0;
    'dispatch: loop {
        match pc {
            0x82DD18C0 => {
    //   block [0x82DD18C0..0x82DD1914)
	// 82DD18C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD18C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD18C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD18CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD18D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD18D4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DD18D8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DD18DC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD18E0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD18E4: 4803FBBD  bl 0x82e114a0
	ctx.lr = 0x82DD18E8;
	sub_82E114A0(ctx, base);
	// 82DD18E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD18EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD18F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD18F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD18F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD18FC: 4E800421  bctrl
	ctx.lr = 0x82DD1900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD1900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD1904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD190C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD1910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1938 size=100
    let mut pc: u32 = 0x82DD1938;
    'dispatch: loop {
        match pc {
            0x82DD1938 => {
    //   block [0x82DD1938..0x82DD199C)
	// 82DD1938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD1944: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD194C: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1950: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82DD1954: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82DD1958: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82DD195C: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82DD1960: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD1964: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DD1968: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD196C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD1970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD1974: 4E800421  bctrl
	ctx.lr = 0x82DD1978;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD1978: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82DD197C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD1980: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DD1984: 48040B9D  bl 0x82e12520
	ctx.lr = 0x82DD1988;
	sub_82E12520(ctx, base);
	// 82DD1988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD198C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD1998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD19A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD19A0 size=80
    let mut pc: u32 = 0x82DD19A0;
    'dispatch: loop {
        match pc {
            0x82DD19A0 => {
    //   block [0x82DD19A0..0x82DD19F0)
	// 82DD19A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD19A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD19A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD19AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD19B0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD19B4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD19B8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD19BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD19C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD19C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD19C8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD19CC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD19D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD19D4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD19D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD19DC: 4800B3AD  bl 0x82ddcd88
	ctx.lr = 0x82DD19E0;
	sub_82DDCD88(ctx, base);
	// 82DD19E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD19E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD19E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD19EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD19F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD19F0 size=72
    let mut pc: u32 = 0x82DD19F0;
    'dispatch: loop {
        match pc {
            0x82DD19F0 => {
    //   block [0x82DD19F0..0x82DD1A38)
	// 82DD19F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD19F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD19F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD19FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1A00: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD1A04: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD1A08: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DD1A0C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD1A10: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD1A14: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD1A18: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD1A1C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD1A20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD1A24: 4800B365  bl 0x82ddcd88
	ctx.lr = 0x82DD1A28;
	sub_82DDCD88(ctx, base);
	// 82DD1A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1A40 size=76
    let mut pc: u32 = 0x82DD1A40;
    'dispatch: loop {
        match pc {
            0x82DD1A40 => {
    //   block [0x82DD1A40..0x82DD1A8C)
	// 82DD1A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1A4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD1A50: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD1A54: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD1A58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD1A5C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD1A60: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD1A64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD1A68: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD1A6C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD1A70: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD1A74: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD1A78: 4800B8A9  bl 0x82ddd320
	ctx.lr = 0x82DD1A7C;
	sub_82DDD320(ctx, base);
	// 82DD1A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1A90 size=464
    let mut pc: u32 = 0x82DD1A90;
    'dispatch: loop {
        match pc {
            0x82DD1A90 => {
    //   block [0x82DD1A90..0x82DD1C60)
	// 82DD1A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1A94: 4BED7969  bl 0x82ca93fc
	ctx.lr = 0x82DD1A98;
	sub_82CA93D0(ctx, base);
	// 82DD1A98: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1A9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DD1AA0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD1AA4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD1AA8: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82DD1AAC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DD1AB0: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82DD1AB4: 90C10068  stw r6, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 82DD1AB8: C0060058  lfs f0, 0x58(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD1ABC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD1AC0: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD1AC4: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD1AC8: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82DD1ACC: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 82DD1AD0: 83E50000  lwz r31, 0(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1AD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DD1AD8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82DD1ADC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD1AE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82DD1AE4: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD1C60 size=24
    let mut pc: u32 = 0x82DD1C60;
    'dispatch: loop {
        match pc {
            0x82DD1C60 => {
    //   block [0x82DD1C60..0x82DD1C78)
	// 82DD1C60: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD1C64: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD1C68: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD1C6C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DD1C70: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82DD1C74: 4BFFF1AC  b 0x82dd0e20
	sub_82DD0E20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1C78 size=68
    let mut pc: u32 = 0x82DD1C78;
    'dispatch: loop {
        match pc {
            0x82DD1C78 => {
    //   block [0x82DD1C78..0x82DD1CBC)
	// 82DD1C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1C80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1C84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1C88: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD1C8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD1C90: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DD1C94: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD1C98: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD1C9C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD1CA0: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DD1CA4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD1CA8: 4800B679  bl 0x82ddd320
	ctx.lr = 0x82DD1CAC;
	sub_82DDD320(ctx, base);
	// 82DD1CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1CC0 size=104
    let mut pc: u32 = 0x82DD1CC0;
    'dispatch: loop {
        match pc {
            0x82DD1CC0 => {
    //   block [0x82DD1CC0..0x82DD1D28)
	// 82DD1CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD1CCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1CD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1CD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD1CD8: 396B372C  addi r11, r11, 0x372c
	ctx.r[11].s64 = ctx.r[11].s64 + 14124;
	// 82DD1CDC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD1CE0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DD1CE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD1CE8: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DD1CEC: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DD1CF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD1CF4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD1CF8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD1CFC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DD1D00: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DD1D04: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1D08: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DD1D0C: 4803F9A5  bl 0x82e116b0
	ctx.lr = 0x82DD1D10;
	sub_82E116B0(ctx, base);
	// 82DD1D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD1D14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD1D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1D20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD1D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1D28 size=128
    let mut pc: u32 = 0x82DD1D28;
    'dispatch: loop {
        match pc {
            0x82DD1D28 => {
    //   block [0x82DD1D28..0x82DD1DA8)
	// 82DD1D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1D2C: 4BED76E1  bl 0x82ca940c
	ctx.lr = 0x82DD1D30;
	sub_82CA93D0(ctx, base);
	// 82DD1D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1D34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1D38: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD1D3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DD1D40: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD1D44: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82DD1D48: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DD1D4C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD1D50: 4BF834F9  bl 0x82d55248
	ctx.lr = 0x82DD1D54;
	sub_82D55248(ctx, base);
	// 82DD1D54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1D58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD1D5C: 396B372C  addi r11, r11, 0x372c
	ctx.r[11].s64 = ctx.r[11].s64 + 14124;
	// 82DD1D60: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82DD1D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DD1D68: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DD1D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD1D70: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82DD1D74: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DD1D78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD1D7C: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DD1D80: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DD1D84: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD1D88: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DD1D8C: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DD1D90: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1D94: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DD1D98: 4803F919  bl 0x82e116b0
	ctx.lr = 0x82DD1D9C;
	sub_82E116B0(ctx, base);
	// 82DD1D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD1DA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1DA4: 4BED76B8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD1DA8 size=68
    let mut pc: u32 = 0x82DD1DA8;
    'dispatch: loop {
        match pc {
            0x82DD1DA8 => {
    //   block [0x82DD1DA8..0x82DD1DEC)
	// 82DD1DA8: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD1DAC: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD1DB0: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD1DB4: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD1DB8: 39081D28  addi r8, r8, 0x1d28
	ctx.r[8].s64 = ctx.r[8].s64 + 7464;
	// 82DD1DBC: 39290D80  addi r9, r9, 0xd80
	ctx.r[9].s64 = ctx.r[9].s64 + 3456;
	// 82DD1DC0: 394A0DD0  addi r10, r10, 0xdd0
	ctx.r[10].s64 = ctx.r[10].s64 + 3536;
	// 82DD1DC4: 396B0E20  addi r11, r11, 0xe20
	ctx.r[11].s64 = ctx.r[11].s64 + 3616;
	// 82DD1DC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DD1DCC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DD1DD0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD1DD4: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD1DD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DD1DDC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DD1DE0: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 82DD1DE4: 98C30011  stb r6, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[6].u8 ) };
	// 82DD1DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1DF0 size=140
    let mut pc: u32 = 0x82DD1DF0;
    'dispatch: loop {
        match pc {
            0x82DD1DF0 => {
    //   block [0x82DD1DF0..0x82DD1E7C)
	// 82DD1DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1DF4: 4BED7619  bl 0x82ca940c
	ctx.lr = 0x82DD1DF8;
	sub_82CA93D0(ctx, base);
	// 82DD1DF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1DFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1E00: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD1E04: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DD1E08: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD1E0C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82DD1E10: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DD1E14: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD1E18: 4BF83431  bl 0x82d55248
	ctx.lr = 0x82DD1E1C;
	sub_82D55248(ctx, base);
	// 82DD1E1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD1E24: 396B372C  addi r11, r11, 0x372c
	ctx.r[11].s64 = ctx.r[11].s64 + 14124;
	// 82DD1E28: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82DD1E2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DD1E30: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DD1E34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD1E38: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82DD1E3C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DD1E40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD1E44: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DD1E48: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DD1E4C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD1E50: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DD1E54: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DD1E58: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1E5C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DD1E60: 4803F851  bl 0x82e116b0
	ctx.lr = 0x82DD1E64;
	sub_82E116B0(ctx, base);
	// 82DD1E64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD1E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD1E6C: 396B376C  addi r11, r11, 0x376c
	ctx.r[11].s64 = ctx.r[11].s64 + 14188;
	// 82DD1E70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD1E74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1E78: 4BED75E4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD1E80 size=64
    let mut pc: u32 = 0x82DD1E80;
    'dispatch: loop {
        match pc {
            0x82DD1E80 => {
    //   block [0x82DD1E80..0x82DD1EC0)
	// 82DD1E80: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD1E84: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD1E88: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD1E8C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD1E90: 38EBD050  addi r7, r11, -0x2fb0
	ctx.r[7].s64 = ctx.r[11].s64 + -12208;
	// 82DD1E94: 39081DF0  addi r8, r8, 0x1df0
	ctx.r[8].s64 = ctx.r[8].s64 + 7664;
	// 82DD1E98: 3929D320  addi r9, r9, -0x2ce0
	ctx.r[9].s64 = ctx.r[9].s64 + -11488;
	// 82DD1E9C: 394ACD88  addi r10, r10, -0x3278
	ctx.r[10].s64 = ctx.r[10].s64 + -12920;
	// 82DD1EA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD1EA4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DD1EA8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD1EAC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD1EB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DD1EB4: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DD1EB8: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82DD1EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1EC0 size=204
    let mut pc: u32 = 0x82DD1EC0;
    'dispatch: loop {
        match pc {
            0x82DD1EC0 => {
    //   block [0x82DD1EC0..0x82DD1F8C)
	// 82DD1EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD1ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD1ED0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1ED4: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD1ED8: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD1EDC: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD1EE0: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD1EE4: 39081DF0  addi r8, r8, 0x1df0
	ctx.r[8].s64 = ctx.r[8].s64 + 7664;
	// 82DD1EE8: 3929D320  addi r9, r9, -0x2ce0
	ctx.r[9].s64 = ctx.r[9].s64 + -11488;
	// 82DD1EEC: 394ACD88  addi r10, r10, -0x3278
	ctx.r[10].s64 = ctx.r[10].s64 + -12920;
	// 82DD1EF0: 396BD050  addi r11, r11, -0x2fb0
	ctx.r[11].s64 = ctx.r[11].s64 + -12208;
	// 82DD1EF4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD1EF8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DD1EFC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD1F00: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82DD1F04: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD1F08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD1F0C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD1F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD1F14: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD1F18: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DD1F1C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD1F20: 4BFEF249  bl 0x82dc1168
	ctx.lr = 0x82DD1F24;
	sub_82DC1168(ctx, base);
	// 82DD1F24: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD1F28: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD1F2C: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD1F30: 396B0E20  addi r11, r11, 0xe20
	ctx.r[11].s64 = ctx.r[11].s64 + 3616;
	// 82DD1F34: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD1F38: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD1F3C: 39081D28  addi r8, r8, 0x1d28
	ctx.r[8].s64 = ctx.r[8].s64 + 7464;
	// 82DD1F40: 39290D80  addi r9, r9, 0xd80
	ctx.r[9].s64 = ctx.r[9].s64 + 3456;
	// 82DD1F44: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD1F48: 394A0DD0  addi r10, r10, 0xdd0
	ctx.r[10].s64 = ctx.r[10].s64 + 3536;
	// 82DD1F4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD1F50: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82DD1F54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DD1F58: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD1F5C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD1F60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD1F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD1F68: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD1F6C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DD1F70: 4BFEF1F9  bl 0x82dc1168
	ctx.lr = 0x82DD1F74;
	sub_82DC1168(ctx, base);
	// 82DD1F74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD1F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1F80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD1F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD1F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD1F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1F90 size=152
    let mut pc: u32 = 0x82DD1F90;
    'dispatch: loop {
        match pc {
            0x82DD1F90 => {
    //   block [0x82DD1F90..0x82DD1FD8)
	// 82DD1F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD1F98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD1F9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD1FA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD1FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD1FA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD1FAC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DD1FB0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DD1FB4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DD1FB8: 409A0020  bne cr6, 0x82dd1fd8
	if !ctx.cr[6].eq {
	pc = 0x82DD1FD8; continue 'dispatch;
	}
	// 82DD1FBC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1FC0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DD1FC4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DD1FC8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD1FCC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DD1FD0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DD1FD4: 4BF832F5  bl 0x82d552c8
	ctx.lr = 0x82DD1FD8;
	sub_82D552C8(ctx, base);
	pc = 0x82DD1FD8; continue 'dispatch;
            }
            0x82DD1FD8 => {
    //   block [0x82DD1FD8..0x82DD200C)
	// 82DD1FD8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DD1FDC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DD1FE0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DD1FE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DD1FE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD1FEC: 419A0020  beq cr6, 0x82dd200c
	if ctx.cr[6].eq {
	pc = 0x82DD200C; continue 'dispatch;
	}
	// 82DD1FF0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD1FF4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD1FF8: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82DD1FFC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DD2004: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD2008: 4BF832C1  bl 0x82d552c8
	ctx.lr = 0x82DD200C;
	sub_82D552C8(ctx, base);
	pc = 0x82DD200C; continue 'dispatch;
            }
            0x82DD200C => {
    //   block [0x82DD200C..0x82DD2028)
	// 82DD200C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD2010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD2014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD201C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD2020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD2024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2028 size=100
    let mut pc: u32 = 0x82DD2028;
    'dispatch: loop {
        match pc {
            0x82DD2028 => {
    //   block [0x82DD2028..0x82DD208C)
	// 82DD2028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD2034: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD203C: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2040: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82DD2044: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82DD2048: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82DD204C: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82DD2050: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD2054: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DD2058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD205C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD2060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD2064: 4E800421  bctrl
	ctx.lr = 0x82DD2068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD2068: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82DD206C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD2070: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DD2074: 480404AD  bl 0x82e12520
	ctx.lr = 0x82DD2078;
	sub_82E12520(ctx, base);
	// 82DD2078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD207C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD2088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2090 size=92
    let mut pc: u32 = 0x82DD2090;
    'dispatch: loop {
        match pc {
            0x82DD2090 => {
    //   block [0x82DD2090..0x82DD20EC)
	// 82DD2090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD209C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD20A0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD20A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD20A8: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DD20AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD20B0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD20B4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD20B8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DD20BC: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD20C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD20C4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD20C8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DD20CC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DD20D0: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DD20D4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DD20D8: 4800ACB1  bl 0x82ddcd88
	ctx.lr = 0x82DD20DC;
	sub_82DDCD88(ctx, base);
	// 82DD20DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD20E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD20E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD20E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD20F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD20F0 size=88
    let mut pc: u32 = 0x82DD20F0;
    'dispatch: loop {
        match pc {
            0x82DD20F0 => {
    //   block [0x82DD20F0..0x82DD2148)
	// 82DD20F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD20F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD20F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD20FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD2100: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD2104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD2108: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DD210C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD2110: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD2114: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD2118: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DD211C: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DD2120: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD2124: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DD2128: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DD212C: 99410064  stb r10, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u8 ) };
	// 82DD2130: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DD2134: 4800B1ED  bl 0x82ddd320
	ctx.lr = 0x82DD2138;
	sub_82DDD320(ctx, base);
	// 82DD2138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD213C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2148 size=176
    let mut pc: u32 = 0x82DD2148;
    'dispatch: loop {
        match pc {
            0x82DD2148 => {
    //   block [0x82DD2148..0x82DD21C8)
	// 82DD2148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD214C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD2154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD2158: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD215C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2160: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD2164: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD2168: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD216C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD2170: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2174: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD2178: 4BFFF919  bl 0x82dd1a90
	ctx.lr = 0x82DD217C;
	sub_82DD1A90(ctx, base);
	// 82DD217C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2180: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD2184: 40980044  bge cr6, 0x82dd21c8
	if !ctx.cr[6].lt {
	pc = 0x82DD21C8; continue 'dispatch;
	}
	// 82DD2188: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD218C: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD2190: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD21C8; continue 'dispatch;
            }
            0x82DD21C8 => {
    //   block [0x82DD21C8..0x82DD21F8)
	// 82DD21C8: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD21CC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD21D0: 419A000C  beq cr6, 0x82dd21dc
	if ctx.cr[6].eq {
	pc = 0x82DD21DC; continue 'dispatch;
	}
	// 82DD21D4: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD21D8: 4BFF8DB1  bl 0x82dcaf88
	ctx.lr = 0x82DD21DC;
	sub_82DCAF88(ctx, base);
	// 82DD21DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD21E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD21E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD21E8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD21EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD21F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD21F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD21F8 size=352
    let mut pc: u32 = 0x82DD21F8;
    'dispatch: loop {
        match pc {
            0x82DD21F8 => {
    //   block [0x82DD21F8..0x82DD2220)
	// 82DD21F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD21FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD2204: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2208: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD220C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DD2210: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 82DD2214: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DD2218: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DD221C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD2220; continue 'dispatch;
            }
            0x82DD2220 => {
    //   block [0x82DD2220..0x82DD2358)
	// 82DD2220: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD2224: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DD2228: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DD222C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DD2230: 4200FFF0  bdnz 0x82dd2220
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD2220; continue 'dispatch;
	}
	// 82DD2234: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2238: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DD223C: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DD2240: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82DD2244: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD2248: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2358 size=128
    let mut pc: u32 = 0x82DD2358;
    'dispatch: loop {
        match pc {
            0x82DD2358 => {
    //   block [0x82DD2358..0x82DD2380)
	// 82DD2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD235C: 4BED70AD  bl 0x82ca9408
	ctx.lr = 0x82DD2360;
	sub_82CA93D0(ctx, base);
	// 82DD2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD2368: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD236C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD2370: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DD2374: 40990044  ble cr6, 0x82dd23b8
	if !ctx.cr[6].gt {
	pc = 0x82DD23B8; continue 'dispatch;
	}
	// 82DD2378: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD237C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	pc = 0x82DD2380; continue 'dispatch;
            }
            0x82DD2380 => {
    //   block [0x82DD2380..0x82DD23A8)
	// 82DD2380: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2384: 7C8BF22E  lhzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DD2388: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82DD238C: 419A001C  beq cr6, 0x82dd23a8
	if ctx.cr[6].eq {
	pc = 0x82DD23A8; continue 'dispatch;
	}
	// 82DD2390: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD2394: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD2398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD239C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD23A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD23A4: 4E800421  bctrl
	ctx.lr = 0x82DD23A8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD23A8 => {
    //   block [0x82DD23A8..0x82DD23B8)
	// 82DD23A8: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DD23AC: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DD23B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DD23B4: 409AFFCC  bne cr6, 0x82dd2380
	if !ctx.cr[6].eq {
	pc = 0x82DD2380; continue 'dispatch;
	}
	pc = 0x82DD23B8; continue 'dispatch;
            }
            0x82DD23B8 => {
    //   block [0x82DD23B8..0x82DD23D8)
	// 82DD23B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD23BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD23C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD23C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD23C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD23CC: 4E800421  bctrl
	ctx.lr = 0x82DD23D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD23D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD23D4: 4BED7084  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD23D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD23D8 size=640
    let mut pc: u32 = 0x82DD23D8;
    'dispatch: loop {
        match pc {
            0x82DD23D8 => {
    //   block [0x82DD23D8..0x82DD242C)
	// 82DD23D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD23DC: 4BED7001  bl 0x82ca93dc
	ctx.lr = 0x82DD23E0;
	sub_82CA93D0(ctx, base);
	// 82DD23E0: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82DD23E4: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD23E8: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD23EC: 3A400008  li r18, 8
	ctx.r[18].s64 = 8;
	// 82DD23F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD23F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DD23F8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD23FC: 7D52882E  lwzx r10, r18, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82DD2400: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2404: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2408: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD240C: 40980020  bge cr6, 0x82dd242c
	if !ctx.cr[6].lt {
	pc = 0x82DD242C; continue 'dispatch;
	}
	// 82DD2410: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2414: 392937C0  addi r9, r9, 0x37c0
	ctx.r[9].s64 = ctx.r[9].s64 + 14272;
	// 82DD2418: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD241C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD2420: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD2424: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD2428: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD242C; continue 'dispatch;
            }
            0x82DD242C => {
    //   block [0x82DD242C..0x82DD261C)
	// 82DD242C: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82DD2430: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2434: 82DC0000  lwz r22, 0(r28)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2438: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD243C: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD2440: 4BF84F31  bl 0x82d57370
	ctx.lr = 0x82DD2444;
	sub_82D57370(ctx, base);
	// 82DD2444: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DD2448: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD244C: 83FB0010  lwz r31, 0x10(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD2450: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82DD2454: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DD2458: 938100B4  stw r28, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 82DD245C: 92A10080  stw r21, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 82DD2460: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 82DD2464: 409901B8  ble cr6, 0x82dd261c
	if !ctx.cr[6].gt {
	pc = 0x82DD261C; continue 'dispatch;
	}
	// 82DD2468: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD246C: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82DD2470: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DD2474: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 82DD2478: 3B4B4C40  addi r26, r11, 0x4c40
	ctx.r[26].s64 = ctx.r[11].s64 + 19520;
	// 82DD247C: C3EA0C14  lfs f31, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD2480: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 82DD2484: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82DD2488: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82DD248C: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82DD2490: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 82DD2494: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 82DD2498: 928100D4  stw r20, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 82DD249C: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 82DD24A0: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 82DD24A4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DD24A8: 92E100E0  stw r23, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[23].u32 ) };
	pc = 0x82DD261C; continue 'dispatch;
            }
            0x82DD261C => {
    //   block [0x82DD261C..0x82DD2658)
	// 82DD261C: 7D52882E  lwzx r10, r18, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82DD2620: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2624: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2628: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD262C: 40980020  bge cr6, 0x82dd264c
	if !ctx.cr[6].lt {
	pc = 0x82DD264C; continue 'dispatch;
	}
	// 82DD2630: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DD2634: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DD2638: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD263C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD2640: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD2644: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD2648: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD264C: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 82DD2650: CBE1FF78  lfd f31, -0x88(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82DD2654: 4BED6DD8  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2658 size=640
    let mut pc: u32 = 0x82DD2658;
    'dispatch: loop {
        match pc {
            0x82DD2658 => {
    //   block [0x82DD2658..0x82DD26AC)
	// 82DD2658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD265C: 4BED6D81  bl 0x82ca93dc
	ctx.lr = 0x82DD2660;
	sub_82CA93D0(ctx, base);
	// 82DD2660: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82DD2664: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2668: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD266C: 3A400008  li r18, 8
	ctx.r[18].s64 = 8;
	// 82DD2670: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD2674: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD2678: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DD267C: 7D52882E  lwzx r10, r18, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82DD2680: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2684: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2688: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD268C: 40980020  bge cr6, 0x82dd26ac
	if !ctx.cr[6].lt {
	pc = 0x82DD26AC; continue 'dispatch;
	}
	// 82DD2690: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2694: 392937C0  addi r9, r9, 0x37c0
	ctx.r[9].s64 = ctx.r[9].s64 + 14272;
	// 82DD2698: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD269C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD26A0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD26A4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD26A8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD26AC; continue 'dispatch;
            }
            0x82DD26AC => {
    //   block [0x82DD26AC..0x82DD289C)
	// 82DD26AC: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82DD26B0: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD26B4: 82DC0000  lwz r22, 0(r28)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD26B8: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD26BC: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD26C0: 4BF84CB1  bl 0x82d57370
	ctx.lr = 0x82DD26C4;
	sub_82D57370(ctx, base);
	// 82DD26C4: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DD26C8: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD26CC: 83FB0010  lwz r31, 0x10(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD26D0: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82DD26D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DD26D8: 938100B4  stw r28, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 82DD26DC: 92A10080  stw r21, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 82DD26E0: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 82DD26E4: 409901B8  ble cr6, 0x82dd289c
	if !ctx.cr[6].gt {
	pc = 0x82DD289C; continue 'dispatch;
	}
	// 82DD26E8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD26EC: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82DD26F0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DD26F4: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 82DD26F8: 3B4B4C40  addi r26, r11, 0x4c40
	ctx.r[26].s64 = ctx.r[11].s64 + 19520;
	// 82DD26FC: C3EA0C14  lfs f31, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD2700: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 82DD2704: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82DD2708: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82DD270C: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82DD2710: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 82DD2714: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 82DD2718: 928100D4  stw r20, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 82DD271C: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 82DD2720: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 82DD2724: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DD2728: 92E100E0  stw r23, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[23].u32 ) };
	pc = 0x82DD289C; continue 'dispatch;
            }
            0x82DD289C => {
    //   block [0x82DD289C..0x82DD28D8)
	// 82DD289C: 7D52882E  lwzx r10, r18, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82DD28A0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD28A4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD28A8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD28AC: 40980020  bge cr6, 0x82dd28cc
	if !ctx.cr[6].lt {
	pc = 0x82DD28CC; continue 'dispatch;
	}
	// 82DD28B0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DD28B4: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DD28B8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD28BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD28C0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD28C4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD28C8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD28CC: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 82DD28D0: CBE1FF78  lfd f31, -0x88(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82DD28D4: 4BED6B58  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD28D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD28D8 size=436
    let mut pc: u32 = 0x82DD28D8;
    'dispatch: loop {
        match pc {
            0x82DD28D8 => {
    //   block [0x82DD28D8..0x82DD2928)
	// 82DD28D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD28DC: 4BED6B19  bl 0x82ca93f4
	ctx.lr = 0x82DD28E0;
	sub_82CA93D0(ctx, base);
	// 82DD28E0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD28E4: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD28E8: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DD28EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DD28F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD28F4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DD28F8: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DD28FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2900: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2904: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD2908: 40980020  bge cr6, 0x82dd2928
	if !ctx.cr[6].lt {
	pc = 0x82DD2928; continue 'dispatch;
	}
	// 82DD290C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2910: 392937D0  addi r9, r9, 0x37d0
	ctx.r[9].s64 = ctx.r[9].s64 + 14288;
	// 82DD2914: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD2918: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD291C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD2920: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD2924: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD2928; continue 'dispatch;
            }
            0x82DD2928 => {
    //   block [0x82DD2928..0x82DD2A54)
	// 82DD2928: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82DD292C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2930: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2934: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD2938: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD293C: 4BF84A35  bl 0x82d57370
	ctx.lr = 0x82DD2940;
	sub_82D57370(ctx, base);
	// 82DD2940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD2944: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD2948: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD294C: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD2950: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DD2954: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DD2958: C00A0C14  lfs f0, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD295C: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DD2960: 91610110  stw r11, 0x110(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 82DD2964: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DD2968: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DD296C: D00100E0  stfs f0, 0xe0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 82DD2970: 914100E4  stw r10, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82DD2974: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82DD2978: 419A00DC  beq cr6, 0x82dd2a54
	if ctx.cr[6].eq {
	pc = 0x82DD2A54; continue 'dispatch;
	}
	// 82DD297C: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82DD2980: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2984: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DD2988: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD298C: 409A00C8  bne cr6, 0x82dd2a54
	if !ctx.cr[6].eq {
	pc = 0x82DD2A54; continue 'dispatch;
	}
	// 82DD2990: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	pc = 0x82DD2A54; continue 'dispatch;
            }
            0x82DD2A54 => {
    //   block [0x82DD2A54..0x82DD2A8C)
	// 82DD2A54: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DD2A58: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2A5C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2A60: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD2A64: 40980020  bge cr6, 0x82dd2a84
	if !ctx.cr[6].lt {
	pc = 0x82DD2A84; continue 'dispatch;
	}
	// 82DD2A68: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DD2A6C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DD2A70: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD2A74: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD2A78: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD2A7C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD2A80: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD2A84: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 82DD2A88: 4BED69BC  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD2A90 size=20
    let mut pc: u32 = 0x82DD2A90;
    'dispatch: loop {
        match pc {
            0x82DD2A90 => {
    //   block [0x82DD2A90..0x82DD2AA4)
	// 82DD2A90: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD2A94: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD2A98: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD2A9C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DD2AA0: 4BFFFE38  b 0x82dd28d8
	sub_82DD28D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2AA8 size=192
    let mut pc: u32 = 0x82DD2AA8;
    'dispatch: loop {
        match pc {
            0x82DD2AA8 => {
    //   block [0x82DD2AA8..0x82DD2B18)
	// 82DD2AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2AAC: 4BED6961  bl 0x82ca940c
	ctx.lr = 0x82DD2AB0;
	sub_82CA93D0(ctx, base);
	// 82DD2AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2AB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD2AB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DD2ABC: 396B37EC  addi r11, r11, 0x37ec
	ctx.r[11].s64 = ctx.r[11].s64 + 14316;
	// 82DD2AC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DD2AC4: 3BFD000C  addi r31, r29, 0xc
	ctx.r[31].s64 = ctx.r[29].s64 + 12;
	// 82DD2AC8: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DD2ACC: 90FD0008  stw r7, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DD2AD0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD2AD4: 61290004  ori r9, r9, 4
	ctx.r[9].u64 = ctx.r[9].u64 | 4;
	// 82DD2AD8: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DD2ADC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82DD2AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD2AE4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DD2AE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD2AEC: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DD2AF0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DD2AF4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2AF8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DD2AFC: 83CA0014  lwz r30, 0x14(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD2B00: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DD2B04: 40980024  bge cr6, 0x82dd2b28
	if !ctx.cr[6].lt {
	pc = 0x82DD2B28; continue 'dispatch;
	}
	// 82DD2B08: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DD2B0C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DD2B10: 41980008  blt cr6, 0x82dd2b18
	if ctx.cr[6].lt {
	pc = 0x82DD2B18; continue 'dispatch;
	}
	// 82DD2B14: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82DD2B18; continue 'dispatch;
            }
            0x82DD2B18 => {
    //   block [0x82DD2B18..0x82DD2B28)
	// 82DD2B18: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DD2B1C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD2B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD2B24: 4BF843ED  bl 0x82d56f10
	ctx.lr = 0x82DD2B28;
	sub_82D56F10(ctx, base);
	pc = 0x82DD2B28; continue 'dispatch;
            }
            0x82DD2B28 => {
    //   block [0x82DD2B28..0x82DD2B44)
	// 82DD2B28: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DD2B2C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DD2B30: 4099002C  ble cr6, 0x82dd2b5c
	if !ctx.cr[6].gt {
	pc = 0x82DD2B5C; continue 'dispatch;
	}
	// 82DD2B34: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82DD2B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD2B3C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DD2B40: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	pc = 0x82DD2B44; continue 'dispatch;
            }
            0x82DD2B44 => {
    //   block [0x82DD2B44..0x82DD2B5C)
	// 82DD2B44: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2B48: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DD2B4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD2B50: 7D28532E  sthx r9, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 82DD2B54: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DD2B58: 409AFFEC  bne cr6, 0x82dd2b44
	if !ctx.cr[6].eq {
	pc = 0x82DD2B44; continue 'dispatch;
	}
	pc = 0x82DD2B5C; continue 'dispatch;
            }
            0x82DD2B5C => {
    //   block [0x82DD2B5C..0x82DD2B68)
	// 82DD2B5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DD2B60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD2B64: 4BED68F8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2B68 size=88
    let mut pc: u32 = 0x82DD2B68;
    'dispatch: loop {
        match pc {
            0x82DD2B68 => {
    //   block [0x82DD2B68..0x82DD2BC0)
	// 82DD2B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2B6C: 4BED689D  bl 0x82ca9408
	ctx.lr = 0x82DD2B70;
	sub_82CA93D0(ctx, base);
	// 82DD2B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2B74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2B78: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD2B7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD2B80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DD2B84: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD2B88: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DD2B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD2B90: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD2B94: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DD2B98: 4BF826B1  bl 0x82d55248
	ctx.lr = 0x82DD2B9C;
	sub_82D55248(ctx, base);
	// 82DD2B9C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82DD2BA0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DD2BA4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DD2BA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD2BAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DD2BB0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD2BB4: 4BFFFEF5  bl 0x82dd2aa8
	ctx.lr = 0x82DD2BB8;
	sub_82DD2AA8(ctx, base);
	// 82DD2BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD2BBC: 4BED689C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2BC0 size=108
    let mut pc: u32 = 0x82DD2BC0;
    'dispatch: loop {
        match pc {
            0x82DD2BC0 => {
    //   block [0x82DD2BC0..0x82DD2C2C)
	// 82DD2BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2BC4: 4BED6841  bl 0x82ca9404
	ctx.lr = 0x82DD2BC8;
	sub_82CA93D0(ctx, base);
	// 82DD2BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2BCC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2BD0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD2BD4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DD2BD8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DD2BDC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD2BE0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DD2BE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD2BE8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD2BEC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DD2BF0: 4BF82659  bl 0x82d55248
	ctx.lr = 0x82DD2BF4;
	sub_82D55248(ctx, base);
	// 82DD2BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD2BF8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82DD2BFC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD2C00: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD2C04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD2C08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD2C0C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD2C10: 4BFFFE99  bl 0x82dd2aa8
	ctx.lr = 0x82DD2C14;
	sub_82DD2AA8(ctx, base);
	// 82DD2C14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD2C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD2C1C: 396B3828  addi r11, r11, 0x3828
	ctx.r[11].s64 = ctx.r[11].s64 + 14376;
	// 82DD2C20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD2C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD2C28: 4BED682C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2C30 size=204
    let mut pc: u32 = 0x82DD2C30;
    'dispatch: loop {
        match pc {
            0x82DD2C30 => {
    //   block [0x82DD2C30..0x82DD2CFC)
	// 82DD2C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD2C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD2C40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2C44: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD2C48: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD2C4C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD2C50: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD2C54: 39082BC0  addi r8, r8, 0x2bc0
	ctx.r[8].s64 = ctx.r[8].s64 + 11200;
	// 82DD2C58: 39292D00  addi r9, r9, 0x2d00
	ctx.r[9].s64 = ctx.r[9].s64 + 11520;
	// 82DD2C5C: 394A2D50  addi r10, r10, 0x2d50
	ctx.r[10].s64 = ctx.r[10].s64 + 11600;
	// 82DD2C60: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD2C64: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD2C68: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DD2C6C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD2C70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DD2C74: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD2C78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD2C7C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD2C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD2C84: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD2C88: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DD2C8C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD2C90: 4BFEE4D9  bl 0x82dc1168
	ctx.lr = 0x82DD2C94;
	sub_82DC1168(ctx, base);
	// 82DD2C94: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD2C98: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD2C9C: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD2CA0: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD2CA4: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD2CA8: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD2CAC: 39082B68  addi r8, r8, 0x2b68
	ctx.r[8].s64 = ctx.r[8].s64 + 11112;
	// 82DD2CB0: 392928D8  addi r9, r9, 0x28d8
	ctx.r[9].s64 = ctx.r[9].s64 + 10456;
	// 82DD2CB4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD2CB8: 394A2658  addi r10, r10, 0x2658
	ctx.r[10].s64 = ctx.r[10].s64 + 9816;
	// 82DD2CBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD2CC0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DD2CC4: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DD2CC8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD2CCC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD2CD0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD2CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD2CD8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD2CDC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DD2CE0: 4BFEE489  bl 0x82dc1168
	ctx.lr = 0x82DD2CE4;
	sub_82DC1168(ctx, base);
	// 82DD2CE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD2CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2CF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD2CF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD2CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2D00 size=76
    let mut pc: u32 = 0x82DD2D00;
    'dispatch: loop {
        match pc {
            0x82DD2D00 => {
    //   block [0x82DD2D00..0x82DD2D4C)
	// 82DD2D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2D0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD2D10: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2D14: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD2D18: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD2D1C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD2D20: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD2D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD2D28: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD2D2C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD2D30: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD2D34: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD2D38: 4BFFFBA1  bl 0x82dd28d8
	ctx.lr = 0x82DD2D3C;
	sub_82DD28D8(ctx, base);
	// 82DD2D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD2D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2D50 size=80
    let mut pc: u32 = 0x82DD2D50;
    'dispatch: loop {
        match pc {
            0x82DD2D50 => {
    //   block [0x82DD2D50..0x82DD2DA0)
	// 82DD2D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD2D58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2D5C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2D60: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD2D64: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD2D68: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD2D6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD2D70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD2D74: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD2D78: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD2D7C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD2D80: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD2D84: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD2D88: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD2D8C: 4BFFF8CD  bl 0x82dd2658
	ctx.lr = 0x82DD2D90;
	sub_82DD2658(ctx, base);
	// 82DD2D90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD2D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD2DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2DA0 size=724
    let mut pc: u32 = 0x82DD2DA0;
    'dispatch: loop {
        match pc {
            0x82DD2DA0 => {
    //   block [0x82DD2DA0..0x82DD2E00)
	// 82DD2DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2DA4: 4BED662D  bl 0x82ca93d0
	ctx.lr = 0x82DD2DA8;
	sub_82CA93D0(ctx, base);
	// 82DD2DA8: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DD2DAC: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD2DB0: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2DB4: 39C00008  li r14, 8
	ctx.r[14].s64 = 8;
	// 82DD2DB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD2DBC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DD2DC0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DD2DC4: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82DD2DC8: 7D6EF82E  lwzx r11, r14, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DD2DCC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82DD2DD0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DD2DD4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD2DD8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD2DDC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD2DE0: 40980020  bge cr6, 0x82dd2e00
	if !ctx.cr[6].lt {
	pc = 0x82DD2E00; continue 'dispatch;
	}
	// 82DD2DE4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD2DE8: 392937C0  addi r9, r9, 0x37c0
	ctx.r[9].s64 = ctx.r[9].s64 + 14272;
	// 82DD2DEC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD2DF0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD2DF4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD2DF8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD2DFC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD2E00; continue 'dispatch;
            }
            0x82DD2E00 => {
    //   block [0x82DD2E00..0x82DD3038)
	// 82DD2E00: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82DD2E04: 83190000  lwz r24, 0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2E08: 829B0000  lwz r20, 0(r27)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD2E0C: 80B90008  lwz r5, 8(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD2E10: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD2E14: 4BF8455D  bl 0x82d57370
	ctx.lr = 0x82DD2E18;
	sub_82D57370(ctx, base);
	// 82DD2E18: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82DD2E1C: 81780014  lwz r11, 0x14(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD2E20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DD2E24: 92610080  stw r19, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[19].u32 ) };
	// 82DD2E28: 92610084  stw r19, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[19].u32 ) };
	// 82DD2E2C: 83580010  lwz r26, 0x10(r24)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DD2E30: 40990208  ble cr6, 0x82dd3038
	if !ctx.cr[6].gt {
	pc = 0x82DD3038; continue 'dispatch;
	}
	// 82DD2E34: 7D6F5B78  mr r15, r11
	ctx.r[15].u64 = ctx.r[11].u64;
	// 82DD2E38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DD2E3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD2E40: 3AEB4C40  addi r23, r11, 0x4c40
	ctx.r[23].s64 = ctx.r[11].s64 + 19520;
	// 82DD2E44: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82DD2E48: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82DD2E4C: 3A200010  li r17, 0x10
	ctx.r[17].s64 = 16;
	// 82DD2E50: C3EA0C14  lfs f31, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD2E54: 3A40FFFF  li r18, -1
	ctx.r[18].s64 = -1;
	// 82DD2E58: 3AA0FFFF  li r21, -1
	ctx.r[21].s64 = -1;
	// 82DD2E5C: 6176FFFF  ori r22, r11, 0xffff
	ctx.r[22].u64 = ctx.r[11].u64 | 65535;
	// 82DD2E60: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	pc = 0x82DD3038; continue 'dispatch;
            }
            0x82DD3038 => {
    //   block [0x82DD3038..0x82DD3074)
	// 82DD3038: 7D4EF82E  lwzx r10, r14, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DD303C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD3040: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD3044: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD3048: 40980020  bge cr6, 0x82dd3068
	if !ctx.cr[6].lt {
	pc = 0x82DD3068; continue 'dispatch;
	}
	// 82DD304C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DD3050: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DD3054: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD3058: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD305C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD3060: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD3064: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD3068: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82DD306C: CBE1FF60  lfd f31, -0xa0(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	// 82DD3070: 4BED63B0  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD3078 size=80
    let mut pc: u32 = 0x82DD3078;
    'dispatch: loop {
        match pc {
            0x82DD3078 => {
    //   block [0x82DD3078..0x82DD30C8)
	// 82DD3078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD307C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD3080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3084: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD3088: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD308C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD3090: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD3094: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD3098: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD309C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD30A0: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD30A4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD30A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD30AC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD30B0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD30B4: 4BFFF325  bl 0x82dd23d8
	ctx.lr = 0x82DD30B8;
	sub_82DD23D8(ctx, base);
	// 82DD30B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD30BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD30C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD30C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD30C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD30C8 size=68
    let mut pc: u32 = 0x82DD30C8;
    'dispatch: loop {
        match pc {
            0x82DD30C8 => {
    //   block [0x82DD30C8..0x82DD310C)
	// 82DD30C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD30CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD30D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD30D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD30D8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD30DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD30E0: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DD30E4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD30E8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD30EC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD30F0: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DD30F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD30F8: 4BFFF7E1  bl 0x82dd28d8
	ctx.lr = 0x82DD30FC;
	sub_82DD28D8(ctx, base);
	// 82DD30FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD3100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD3104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD3108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD3110 size=176
    let mut pc: u32 = 0x82DD3110;
    'dispatch: loop {
        match pc {
            0x82DD3110 => {
    //   block [0x82DD3110..0x82DD3190)
	// 82DD3110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD3114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD3118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD311C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD3120: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD3124: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3128: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD312C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD3130: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD3134: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD3138: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD313C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD3140: 4BFFFC61  bl 0x82dd2da0
	ctx.lr = 0x82DD3144;
	sub_82DD2DA0(ctx, base);
	// 82DD3144: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3148: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD314C: 40980044  bge cr6, 0x82dd3190
	if !ctx.cr[6].lt {
	pc = 0x82DD3190; continue 'dispatch;
	}
	// 82DD3150: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD3154: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD3158: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD3190; continue 'dispatch;
            }
            0x82DD3190 => {
    //   block [0x82DD3190..0x82DD31C0)
	// 82DD3190: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD3194: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD3198: 419A000C  beq cr6, 0x82dd31a4
	if ctx.cr[6].eq {
	pc = 0x82DD31A4; continue 'dispatch;
	}
	// 82DD319C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD31A0: 4BFF7DE9  bl 0x82dcaf88
	ctx.lr = 0x82DD31A4;
	sub_82DCAF88(ctx, base);
	// 82DD31A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD31A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD31AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD31B0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD31B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD31B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD31BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD31C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD31C0 size=152
    let mut pc: u32 = 0x82DD31C0;
    'dispatch: loop {
        match pc {
            0x82DD31C0 => {
    //   block [0x82DD31C0..0x82DD3208)
	// 82DD31C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD31C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD31C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD31CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD31D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD31D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD31D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD31DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD31E0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DD31E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DD31E8: 409A0020  bne cr6, 0x82dd3208
	if !ctx.cr[6].eq {
	pc = 0x82DD3208; continue 'dispatch;
	}
	// 82DD31EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD31F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DD31F4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DD31F8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD31FC: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DD3200: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DD3204: 4BF820C5  bl 0x82d552c8
	ctx.lr = 0x82DD3208;
	sub_82D552C8(ctx, base);
	pc = 0x82DD3208; continue 'dispatch;
            }
            0x82DD3208 => {
    //   block [0x82DD3208..0x82DD323C)
	// 82DD3208: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DD320C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DD3210: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DD3214: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DD3218: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD321C: 419A0020  beq cr6, 0x82dd323c
	if ctx.cr[6].eq {
	pc = 0x82DD323C; continue 'dispatch;
	}
	// 82DD3220: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3224: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD3228: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82DD322C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD3230: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DD3234: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD3238: 4BF82091  bl 0x82d552c8
	ctx.lr = 0x82DD323C;
	sub_82D552C8(ctx, base);
	pc = 0x82DD323C; continue 'dispatch;
            }
            0x82DD323C => {
    //   block [0x82DD323C..0x82DD3258)
	// 82DD323C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD3240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD3244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD3248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD324C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD3250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD3254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3258 size=240
    let mut pc: u32 = 0x82DD3258;
    'dispatch: loop {
        match pc {
            0x82DD3258 => {
    //   block [0x82DD3258..0x82DD3288)
	// 82DD3258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD325C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD3260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD3264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD3268: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD326C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD3270: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD3274: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DD3278: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DD327C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DD3280: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DD3284: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD3288; continue 'dispatch;
            }
            0x82DD3288 => {
    //   block [0x82DD3288..0x82DD3348)
	// 82DD3288: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD328C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DD3290: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DD3294: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DD3298: 4200FFF0  bdnz 0x82dd3288
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD3288; continue 'dispatch;
	}
	// 82DD329C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD32A0: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DD32A4: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DD32A8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD32AC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD32B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3348 size=136
    let mut pc: u32 = 0x82DD3348;
    'dispatch: loop {
        match pc {
            0x82DD3348 => {
    //   block [0x82DD3348..0x82DD33A8)
	// 82DD3348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD334C: 4BED60C1  bl 0x82ca940c
	ctx.lr = 0x82DD3350;
	sub_82CA93D0(ctx, base);
	// 82DD3350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3354: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3358: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD335C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DD3360: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD3364: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82DD3368: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DD336C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD3370: 4BF81ED9  bl 0x82d55248
	ctx.lr = 0x82DD3374;
	sub_82D55248(ctx, base);
	// 82DD3374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD3378: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD337C: 394B3874  addi r10, r11, 0x3874
	ctx.r[10].s64 = ctx.r[11].s64 + 14452;
	// 82DD3380: 38E0002C  li r7, 0x2c
	ctx.r[7].s64 = 44;
	// 82DD3384: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DD3388: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD338C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DD3390: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DD3394: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82DD3398: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD339C: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82DD33A0: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DD33A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD33A8; continue 'dispatch;
            }
            0x82DD33A8 => {
    //   block [0x82DD33A8..0x82DD33D0)
	// 82DD33A8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82DD33AC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DD33B0: 4200FFF8  bdnz 0x82dd33a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD33A8; continue 'dispatch;
	}
	// 82DD33B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD33B8: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82DD33BC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD33C0: 48049161  bl 0x82e1c520
	ctx.lr = 0x82DD33C4;
	sub_82E1C520(ctx, base);
	// 82DD33C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD33C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD33CC: 4BED6090  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD33D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD33D0 size=120
    let mut pc: u32 = 0x82DD33D0;
    'dispatch: loop {
        match pc {
            0x82DD33D0 => {
    //   block [0x82DD33D0..0x82DD33EC)
	// 82DD33D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD33D4: 4BED6035  bl 0x82ca9408
	ctx.lr = 0x82DD33D8;
	sub_82CA93D0(ctx, base);
	// 82DD33D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD33DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD33E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD33E4: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82DD33E8: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	pc = 0x82DD33EC; continue 'dispatch;
            }
            0x82DD33EC => {
    //   block [0x82DD33EC..0x82DD3410)
	// 82DD33EC: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD33F0: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82DD33F4: 419A001C  beq cr6, 0x82dd3410
	if ctx.cr[6].eq {
	pc = 0x82DD3410; continue 'dispatch;
	}
	// 82DD33F8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD33FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD3400: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3404: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD3408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD340C: 4E800421  bctrl
	ctx.lr = 0x82DD3410;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD3410 => {
    //   block [0x82DD3410..0x82DD3440)
	// 82DD3410: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DD3414: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DD3418: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DD341C: 409AFFD0  bne cr6, 0x82dd33ec
	if !ctx.cr[6].eq {
	pc = 0x82DD33EC; continue 'dispatch;
	}
	// 82DD3420: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DD3424: 419A001C  beq cr6, 0x82dd3440
	if ctx.cr[6].eq {
	pc = 0x82DD3440; continue 'dispatch;
	}
	// 82DD3428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD342C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD3430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD3434: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD343C: 4E800421  bctrl
	ctx.lr = 0x82DD3440;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD3440 => {
    //   block [0x82DD3440..0x82DD3448)
	// 82DD3440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD3444: 4BED6014  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3448 size=148
    let mut pc: u32 = 0x82DD3448;
    'dispatch: loop {
        match pc {
            0x82DD3448 => {
    //   block [0x82DD3448..0x82DD34A8)
	// 82DD3448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD344C: 4BED5FC1  bl 0x82ca940c
	ctx.lr = 0x82DD3450;
	sub_82CA93D0(ctx, base);
	// 82DD3450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3454: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3458: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD345C: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD3460: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82DD3464: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DD3468: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DD346C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD3470: 4BF81DD9  bl 0x82d55248
	ctx.lr = 0x82DD3474;
	sub_82D55248(ctx, base);
	// 82DD3474: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD3478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD347C: 394B3874  addi r10, r11, 0x3874
	ctx.r[10].s64 = ctx.r[11].s64 + 14452;
	// 82DD3480: 38E0002C  li r7, 0x2c
	ctx.r[7].s64 = 44;
	// 82DD3484: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DD3488: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD348C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DD3490: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DD3494: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82DD3498: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD349C: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82DD34A0: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DD34A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD34A8; continue 'dispatch;
            }
            0x82DD34A8 => {
    //   block [0x82DD34A8..0x82DD34DC)
	// 82DD34A8: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82DD34AC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DD34B0: 4200FFF8  bdnz 0x82dd34a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD34A8; continue 'dispatch;
	}
	// 82DD34B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD34B8: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82DD34BC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD34C0: 48049061  bl 0x82e1c520
	ctx.lr = 0x82DD34C4;
	sub_82E1C520(ctx, base);
	// 82DD34C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD34C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD34CC: 396B38B0  addi r11, r11, 0x38b0
	ctx.r[11].s64 = ctx.r[11].s64 + 14512;
	// 82DD34D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD34D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD34D8: 4BED5F84  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD34E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD34E0 size=584
    let mut pc: u32 = 0x82DD34E0;
    'dispatch: loop {
        match pc {
            0x82DD34E0 => {
    //   block [0x82DD34E0..0x82DD3728)
	// 82DD34E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD34E4: 4BED5EFD  bl 0x82ca93e0
	ctx.lr = 0x82DD34E8;
	sub_82CA93D0(ctx, base);
	// 82DD34E8: DBE1FF80  stfd f31, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82DD34EC: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD34F0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DD34F4: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DD34F8: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DD34FC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DD3500: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82DD3504: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD3508: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82DD350C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82DD3510: 82F90000  lwz r23, 0(r25)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3514: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD3518: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD351C: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3520: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD3524: EB4B0000  ld r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD3528: 3BF70020  addi r31, r23, 0x20
	ctx.r[31].s64 = ctx.r[23].s64 + 32;
	// 82DD352C: EA6B0008  ld r19, 8(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD3530: 3B8100A0  addi r28, r1, 0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + 160;
	// 82DD3534: EB060000  ld r24, 0(r6)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD3538: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82DD353C: EA460008  ld r18, 8(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD3540: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DD3544: EAA50000  ld r21, 0(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD3548: 7CDFE050  subf r6, r31, r28
	ctx.r[6].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82DD354C: FB4A0000  std r26, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82DD3550: FA6A0008  std r19, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 82DD3554: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DD3558: FB090000  std r24, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82DD355C: FA490008  std r18, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[18].u64 ) };
	// 82DD3560: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD3564: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3728 size=588
    let mut pc: u32 = 0x82DD3728;
    'dispatch: loop {
        match pc {
            0x82DD3728 => {
    //   block [0x82DD3728..0x82DD3974)
	// 82DD3728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD372C: 4BED5CB9  bl 0x82ca93e4
	ctx.lr = 0x82DD3730;
	sub_82CA93D0(ctx, base);
	// 82DD3730: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82DD3734: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3738: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DD373C: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD3740: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DD3744: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DD3748: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD374C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD3750: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD3754: 83190000  lwz r24, 0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3758: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DD375C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD3760: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD3764: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD3768: EB8B0000  ld r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD376C: 38780020  addi r3, r24, 0x20
	ctx.r[3].s64 = ctx.r[24].s64 + 32;
	// 82DD3770: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD3774: 3BA100A0  addi r29, r1, 0xa0
	ctx.r[29].s64 = ctx.r[1].s64 + 160;
	// 82DD3778: EB460000  ld r26, 0(r6)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD377C: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82DD3780: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD3784: EAC50000  ld r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD3788: FB8A0000  std r28, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 82DD378C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD3790: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DD3794: FB490000  std r26, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82DD3798: 7D43E850  subf r10, r3, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 82DD379C: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD37A0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD37A4: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD37A8: FAC80000  std r22, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD37AC: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD37B0: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3978 size=744
    let mut pc: u32 = 0x82DD3978;
    'dispatch: loop {
        match pc {
            0x82DD3978 => {
    //   block [0x82DD3978..0x82DD39C8)
	// 82DD3978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD397C: 4BED5A59  bl 0x82ca93d4
	ctx.lr = 0x82DD3980;
	sub_82CA93D0(ctx, base);
	// 82DD3980: DBE1FF68  stfd f31, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 82DD3984: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3988: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD398C: 3AC00008  li r22, 8
	ctx.r[22].s64 = 8;
	// 82DD3990: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DD3994: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DD3998: 7D76A82E  lwzx r11, r22, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DD399C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD39A0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD39A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD39A8: 40980020  bge cr6, 0x82dd39c8
	if !ctx.cr[6].lt {
	pc = 0x82DD39C8; continue 'dispatch;
	}
	// 82DD39AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD39B0: 392938FC  addi r9, r9, 0x38fc
	ctx.r[9].s64 = ctx.r[9].s64 + 14588;
	// 82DD39B4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD39B8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD39BC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD39C0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD39C4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD39C8; continue 'dispatch;
            }
            0x82DD39C8 => {
    //   block [0x82DD39C8..0x82DD3C60)
	// 82DD39C8: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD39CC: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DD39D0: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82DD39D4: 83250000  lwz r25, 0(r5)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD39D8: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD39DC: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD39E0: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 82DD39E4: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD39E8: EA8B0000  ld r20, 0(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD39EC: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 82DD39F0: EA0B0008  ld r16, 8(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD39F4: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DD39F8: EA660000  ld r19, 0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD39FC: 3BB90020  addi r29, r25, 0x20
	ctx.r[29].s64 = ctx.r[25].s64 + 32;
	// 82DD3A00: E9E60008  ld r15, 8(r6)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD3A04: 3B4100F0  addi r26, r1, 0xf0
	ctx.r[26].s64 = ctx.r[1].s64 + 240;
	// 82DD3A08: EA5F0000  ld r18, 0(r31)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82DD3A0C: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 82DD3A10: FA8A0000  std r20, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD3A14: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82DD3A18: FA0A0008  std r16, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[16].u64 ) };
	// 82DD3A1C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DD3A20: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82DD3A24: 7CDDD050  subf r6, r29, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 82DD3A28: F9E90008  std r15, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[15].u64 ) };
	// 82DD3A2C: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82DD3A30: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3C60 size=756
    let mut pc: u32 = 0x82DD3C60;
    'dispatch: loop {
        match pc {
            0x82DD3C60 => {
    //   block [0x82DD3C60..0x82DD3CB8)
	// 82DD3C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD3C64: 4BED5775  bl 0x82ca93d8
	ctx.lr = 0x82DD3C68;
	sub_82CA93D0(ctx, base);
	// 82DD3C68: DBE1FF70  stfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82DD3C6C: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3C70: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3C74: 3AC00008  li r22, 8
	ctx.r[22].s64 = 8;
	// 82DD3C78: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DD3C7C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DD3C80: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82DD3C84: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DD3C88: 7D76A82E  lwzx r11, r22, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DD3C8C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD3C90: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD3C94: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD3C98: 40980020  bge cr6, 0x82dd3cb8
	if !ctx.cr[6].lt {
	pc = 0x82DD3CB8; continue 'dispatch;
	}
	// 82DD3C9C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD3CA0: 392938FC  addi r9, r9, 0x38fc
	ctx.r[9].s64 = ctx.r[9].s64 + 14588;
	// 82DD3CA4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD3CA8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD3CAC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD3CB0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD3CB4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD3CB8; continue 'dispatch;
            }
            0x82DD3CB8 => {
    //   block [0x82DD3CB8..0x82DD3F54)
	// 82DD3CB8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD3CBC: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82DD3CC0: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD3CC4: 833A0000  lwz r25, 0(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3CC8: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD3CCC: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD3CD0: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD3CD4: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD3CD8: EB6B0000  ld r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD3CDC: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD3CE0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD3CE4: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DD3CE8: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD3CEC: 38790020  addi r3, r25, 0x20
	ctx.r[3].s64 = ctx.r[25].s64 + 32;
	// 82DD3CF0: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD3CF4: 3BA10100  addi r29, r1, 0x100
	ctx.r[29].s64 = ctx.r[1].s64 + 256;
	// 82DD3CF8: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD3CFC: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82DD3D00: FB6A0000  std r27, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DD3D04: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD3D08: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DD3D0C: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD3D10: 7D43E850  subf r10, r3, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 82DD3D14: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD3D18: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82DD3D1C: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD3D20: FA680000  std r19, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82DD3D24: EA440000  ld r18, 0(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD3D28: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD3F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3F58 size=204
    let mut pc: u32 = 0x82DD3F58;
    'dispatch: loop {
        match pc {
            0x82DD3F58 => {
    //   block [0x82DD3F58..0x82DD4024)
	// 82DD3F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD3F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD3F60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD3F64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD3F68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD3F6C: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD3F70: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD3F74: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD3F78: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD3F7C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD3F80: 39083448  addi r8, r8, 0x3448
	ctx.r[8].s64 = ctx.r[8].s64 + 13384;
	// 82DD3F84: 39294078  addi r9, r9, 0x4078
	ctx.r[9].s64 = ctx.r[9].s64 + 16504;
	// 82DD3F88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD3F8C: 394A44A0  addi r10, r10, 0x44a0
	ctx.r[10].s64 = ctx.r[10].s64 + 17568;
	// 82DD3F90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD3F94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD3F98: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 82DD3F9C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD3FA0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82DD3FA4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD3FA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD3FAC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD3FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD3FB4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD3FB8: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD3FBC: 4BFED1AD  bl 0x82dc1168
	ctx.lr = 0x82DD3FC0;
	sub_82DC1168(ctx, base);
	// 82DD3FC0: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD3FC4: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD3FC8: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD3FCC: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD3FD0: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD3FD4: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD3FD8: 39083348  addi r8, r8, 0x3348
	ctx.r[8].s64 = ctx.r[8].s64 + 13128;
	// 82DD3FDC: 39293728  addi r9, r9, 0x3728
	ctx.r[9].s64 = ctx.r[9].s64 + 14120;
	// 82DD3FE0: 394A3C60  addi r10, r10, 0x3c60
	ctx.r[10].s64 = ctx.r[10].s64 + 15456;
	// 82DD3FE4: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD3FE8: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82DD3FEC: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 82DD3FF0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD3FF4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD3FF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD3FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD4000: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD4004: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD4008: 4BFED161  bl 0x82dc1168
	ctx.lr = 0x82DD400C;
	sub_82DC1168(ctx, base);
	// 82DD400C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD4010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD4014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD4018: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD401C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD4020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4028 size=76
    let mut pc: u32 = 0x82DD4028;
    'dispatch: loop {
        match pc {
            0x82DD4028 => {
    //   block [0x82DD4028..0x82DD4074)
	// 82DD4028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD402C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD4030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD4034: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD4038: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD403C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD4040: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD4044: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD4048: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD404C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD4050: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD4054: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD4058: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD405C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD4060: 4BFFF481  bl 0x82dd34e0
	ctx.lr = 0x82DD4064;
	sub_82DD34E0(ctx, base);
	// 82DD4064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD4068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD406C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD4070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4078 size=76
    let mut pc: u32 = 0x82DD4078;
    'dispatch: loop {
        match pc {
            0x82DD4078 => {
    //   block [0x82DD4078..0x82DD40C4)
	// 82DD4078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD407C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD4080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD4084: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD4088: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD408C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD4090: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD4094: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD4098: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD409C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD40A0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD40A4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD40A8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD40AC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD40B0: 4BFFF679  bl 0x82dd3728
	ctx.lr = 0x82DD40B4;
	sub_82DD3728(ctx, base);
	// 82DD40B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD40B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD40BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD40C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD40C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD40C8 size=904
    let mut pc: u32 = 0x82DD40C8;
    'dispatch: loop {
        match pc {
            0x82DD40C8 => {
    //   block [0x82DD40C8..0x82DD4124)
	// 82DD40C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD40CC: 4BED5309  bl 0x82ca93d4
	ctx.lr = 0x82DD40D0;
	sub_82CA93D0(ctx, base);
	// 82DD40D0: DBE1FF68  stfd f31, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 82DD40D4: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD40D8: 820D0000  lwz r16, 0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD40DC: 3A200008  li r17, 8
	ctx.r[17].s64 = 8;
	// 82DD40E0: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82DD40E4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DD40E8: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82DD40EC: 7CD23378  mr r18, r6
	ctx.r[18].u64 = ctx.r[6].u64;
	// 82DD40F0: 7D71802E  lwzx r11, r17, r16
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82DD40F4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82DD40F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD40FC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD4100: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD4104: 40980020  bge cr6, 0x82dd4124
	if !ctx.cr[6].lt {
	pc = 0x82DD4124; continue 'dispatch;
	}
	// 82DD4108: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD410C: 392938E8  addi r9, r9, 0x38e8
	ctx.r[9].s64 = ctx.r[9].s64 + 14568;
	// 82DD4110: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD4114: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD4118: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD411C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD4120: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD4124; continue 'dispatch;
            }
            0x82DD4124 => {
    //   block [0x82DD4124..0x82DD4450)
	// 82DD4124: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD4128: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DD412C: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82DD4130: 82F60000  lwz r23, 0(r22)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4134: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD4138: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD413C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD4140: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DD4144: EB6B0000  ld r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD4148: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD414C: EA6B0008  ld r19, 8(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD4150: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DD4154: EB260000  ld r25, 0(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD4158: 38770020  addi r3, r23, 0x20
	ctx.r[3].s64 = ctx.r[23].s64 + 32;
	// 82DD415C: E9E60008  ld r15, 8(r6)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD4160: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82DD4164: EB050000  ld r24, 0(r5)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD4168: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82DD416C: FB6A0000  std r27, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DD4170: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DD4174: FA6A0008  std r19, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 82DD4178: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DD417C: FB290000  std r25, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82DD4180: 7CC3E850  subf r6, r3, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 82DD4184: F9E90008  std r15, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[15].u64 ) };
	// 82DD4188: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD418C: FB080000  std r24, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD4450 size=80
    let mut pc: u32 = 0x82DD4450;
    'dispatch: loop {
        match pc {
            0x82DD4450 => {
    //   block [0x82DD4450..0x82DD44A0)
	// 82DD4450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD4458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD445C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD4460: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD4464: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD4468: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD446C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD4470: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD4474: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD4478: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD447C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD4480: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD4484: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD4488: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD448C: 4BFFF4ED  bl 0x82dd3978
	ctx.lr = 0x82DD4490;
	sub_82DD3978(ctx, base);
	// 82DD4490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD4494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD4498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD449C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD44A0 size=80
    let mut pc: u32 = 0x82DD44A0;
    'dispatch: loop {
        match pc {
            0x82DD44A0 => {
    //   block [0x82DD44A0..0x82DD44F0)
	// 82DD44A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD44A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD44A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD44AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD44B0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD44B4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD44B8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD44BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD44C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD44C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD44C8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD44CC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD44D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD44D4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD44D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD44DC: 4BFFF785  bl 0x82dd3c60
	ctx.lr = 0x82DD44E0;
	sub_82DD3C60(ctx, base);
	// 82DD44E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD44E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD44E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD44EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD44F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD44F0 size=176
    let mut pc: u32 = 0x82DD44F0;
    'dispatch: loop {
        match pc {
            0x82DD44F0 => {
    //   block [0x82DD44F0..0x82DD4570)
	// 82DD44F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD44F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD44F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD44FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD4500: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD4504: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD4508: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD450C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD4510: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD4514: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD4518: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD451C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD4520: 4BFFFBA9  bl 0x82dd40c8
	ctx.lr = 0x82DD4524;
	sub_82DD40C8(ctx, base);
	// 82DD4524: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4528: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD452C: 40980044  bge cr6, 0x82dd4570
	if !ctx.cr[6].lt {
	pc = 0x82DD4570; continue 'dispatch;
	}
	// 82DD4530: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD4534: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD4538: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD4570; continue 'dispatch;
            }
            0x82DD4570 => {
    //   block [0x82DD4570..0x82DD45A0)
	// 82DD4570: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD4574: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD4578: 419A000C  beq cr6, 0x82dd4584
	if ctx.cr[6].eq {
	pc = 0x82DD4584; continue 'dispatch;
	}
	// 82DD457C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD4580: 4BFF6A09  bl 0x82dcaf88
	ctx.lr = 0x82DD4584;
	sub_82DCAF88(ctx, base);
	// 82DD4584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD4588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD458C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD4590: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD4594: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD4598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD459C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD45A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD45A0 size=128
    let mut pc: u32 = 0x82DD45A0;
    'dispatch: loop {
        match pc {
            0x82DD45A0 => {
    //   block [0x82DD45A0..0x82DD4620)
	// 82DD45A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD45A4: 4BED4E69  bl 0x82ca940c
	ctx.lr = 0x82DD45A8;
	sub_82CA93D0(ctx, base);
	// 82DD45A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD45AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD45B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD45B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD45B8: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD45BC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82DD45C0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DD45C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD45C8: 4BF80C81  bl 0x82d55248
	ctx.lr = 0x82DD45CC;
	sub_82D55248(ctx, base);
	// 82DD45CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD45D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD45D4: 394B3934  addi r10, r11, 0x3934
	ctx.r[10].s64 = ctx.r[11].s64 + 14644;
	// 82DD45D8: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82DD45DC: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 82DD45E0: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82DD45E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DD45E8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82DD45EC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD45F0: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82DD45F4: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DD45F8: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82DD45FC: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DD4600: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82DD4604: B17F0010  sth r11, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82DD4608: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD460C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD4610: 48047FE9  bl 0x82e1c5f8
	ctx.lr = 0x82DD4614;
	sub_82E1C5F8(ctx, base);
	// 82DD4614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD4618: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD461C: 4BED4E40  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4620 size=120
    let mut pc: u32 = 0x82DD4620;
    'dispatch: loop {
        match pc {
            0x82DD4620 => {
    //   block [0x82DD4620..0x82DD463C)
	// 82DD4620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4624: 4BED4DE5  bl 0x82ca9408
	ctx.lr = 0x82DD4628;
	sub_82CA93D0(ctx, base);
	// 82DD4628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD462C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD4630: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD4634: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82DD4638: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	pc = 0x82DD463C; continue 'dispatch;
            }
            0x82DD463C => {
    //   block [0x82DD463C..0x82DD4660)
	// 82DD463C: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4640: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82DD4644: 419A001C  beq cr6, 0x82dd4660
	if ctx.cr[6].eq {
	pc = 0x82DD4660; continue 'dispatch;
	}
	// 82DD4648: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD464C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD4650: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4654: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD4658: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD465C: 4E800421  bctrl
	ctx.lr = 0x82DD4660;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD4660 => {
    //   block [0x82DD4660..0x82DD4690)
	// 82DD4660: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DD4664: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DD4668: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DD466C: 409AFFD0  bne cr6, 0x82dd463c
	if !ctx.cr[6].eq {
	pc = 0x82DD463C; continue 'dispatch;
	}
	// 82DD4670: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DD4674: 419A001C  beq cr6, 0x82dd4690
	if ctx.cr[6].eq {
	pc = 0x82DD4690; continue 'dispatch;
	}
	// 82DD4678: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD467C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD4680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD4684: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD468C: 4E800421  bctrl
	ctx.lr = 0x82DD4690;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD4690 => {
    //   block [0x82DD4690..0x82DD4698)
	// 82DD4690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD4694: 4BED4DC4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4698 size=140
    let mut pc: u32 = 0x82DD4698;
    'dispatch: loop {
        match pc {
            0x82DD4698 => {
    //   block [0x82DD4698..0x82DD4724)
	// 82DD4698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD469C: 4BED4D71  bl 0x82ca940c
	ctx.lr = 0x82DD46A0;
	sub_82CA93D0(ctx, base);
	// 82DD46A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD46A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD46A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD46AC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD46B0: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82DD46B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD46B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DD46BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD46C0: 4BF80B89  bl 0x82d55248
	ctx.lr = 0x82DD46C4;
	sub_82D55248(ctx, base);
	// 82DD46C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD46C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD46CC: 394B3934  addi r10, r11, 0x3934
	ctx.r[10].s64 = ctx.r[11].s64 + 14644;
	// 82DD46D0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82DD46D4: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 82DD46D8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82DD46DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DD46E0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82DD46E4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD46E8: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82DD46EC: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DD46F0: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82DD46F4: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DD46F8: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82DD46FC: B17F0010  sth r11, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82DD4700: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4704: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD4708: 48047EF1  bl 0x82e1c5f8
	ctx.lr = 0x82DD470C;
	sub_82E1C5F8(ctx, base);
	// 82DD470C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD4710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD4714: 396B3970  addi r11, r11, 0x3970
	ctx.r[11].s64 = ctx.r[11].s64 + 14704;
	// 82DD4718: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD471C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD4720: 4BED4D3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4728 size=456
    let mut pc: u32 = 0x82DD4728;
    'dispatch: loop {
        match pc {
            0x82DD4728 => {
    //   block [0x82DD4728..0x82DD48F0)
	// 82DD4728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD472C: 4BED4CB9  bl 0x82ca93e4
	ctx.lr = 0x82DD4730;
	sub_82CA93D0(ctx, base);
	// 82DD4730: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD4734: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD4738: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82DD473C: 3BE10080  addi r31, r1, 0x80
	ctx.r[31].s64 = ctx.r[1].s64 + 128;
	// 82DD4740: 82A30000  lwz r21, 0(r3)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4744: 3B8B0010  addi r28, r11, 0x10
	ctx.r[28].s64 = ctx.r[11].s64 + 16;
	// 82DD4748: 82C40000  lwz r22, 0(r4)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD474C: 3B6B0020  addi r27, r11, 0x20
	ctx.r[27].s64 = ctx.r[11].s64 + 32;
	// 82DD4750: 3B4B0030  addi r26, r11, 0x30
	ctx.r[26].s64 = ctx.r[11].s64 + 48;
	// 82DD4754: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD4758: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DD475C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD4760: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 82DD4764: E8FC0000  ld r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 82DD4768: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DD476C: EB9C0008  ld r28, 8(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	// 82DD4770: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82DD4774: EA9B0000  ld r20, 0(r27)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 82DD4778: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82DD477C: F91F0000  std r8, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DD4780: 3AE100A0  addi r23, r1, 0xa0
	ctx.r[23].s64 = ctx.r[1].s64 + 160;
	// 82DD4784: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD4788: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DD478C: F8FE0000  std r7, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82DD4790: 3B150030  addi r24, r21, 0x30
	ctx.r[24].s64 = ctx.r[21].s64 + 48;
	// 82DD4794: FB9E0008  std r28, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u64 ) };
	// 82DD4798: EB7B0008  ld r27, 8(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	// 82DD479C: FA830000  std r20, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD48F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD48F0 size=588
    let mut pc: u32 = 0x82DD48F0;
    'dispatch: loop {
        match pc {
            0x82DD48F0 => {
    //   block [0x82DD48F0..0x82DD4B3C)
	// 82DD48F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD48F4: 4BED4AF5  bl 0x82ca93e8
	ctx.lr = 0x82DD48F8;
	sub_82CA93D0(ctx, base);
	// 82DD48F8: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD48FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD4900: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DD4904: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4908: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD490C: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82DD4910: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4914: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD4918: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DD491C: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD4920: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD4924: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD4928: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD492C: EAC50000  ld r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD4930: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DD4934: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD4938: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD493C: EAA30000  ld r21, 0(r3)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82DD4940: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD4944: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD4948: 3B810100  addi r28, r1, 0x100
	ctx.r[28].s64 = ctx.r[1].s64 + 256;
	// 82DD494C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD4950: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD4954: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD4958: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 82DD495C: F8A90008  std r5, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82DD4960: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82DD4964: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4B40 size=688
    let mut pc: u32 = 0x82DD4B40;
    'dispatch: loop {
        match pc {
            0x82DD4B40 => {
    //   block [0x82DD4B40..0x82DD4B8C)
	// 82DD4B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4B44: 4BED489D  bl 0x82ca93e0
	ctx.lr = 0x82DD4B48;
	sub_82CA93D0(ctx, base);
	// 82DD4B48: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD4B4C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4B50: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DD4B54: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DD4B58: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82DD4B5C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DD4B60: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD4B64: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD4B68: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD4B6C: 40980020  bge cr6, 0x82dd4b8c
	if !ctx.cr[6].lt {
	pc = 0x82DD4B8C; continue 'dispatch;
	}
	// 82DD4B70: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD4B74: 392939B8  addi r9, r9, 0x39b8
	ctx.r[9].s64 = ctx.r[9].s64 + 14776;
	// 82DD4B78: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD4B7C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD4B80: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD4B84: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD4B88: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD4B8C; continue 'dispatch;
            }
            0x82DD4B8C => {
    //   block [0x82DD4B8C..0x82DD4DF0)
	// 82DD4B8C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD4B90: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD4B94: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82DD4B98: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD4B9C: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82DD4BA0: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD4BA4: 83440000  lwz r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4BA8: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DD4BAC: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DD4BB0: 83650000  lwz r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4BB4: EAAB0000  ld r21, 0(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD4BB8: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD4BBC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD4BC0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD4BC4: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD4BC8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD4BCC: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD4BD0: 3B810130  addi r28, r1, 0x130
	ctx.r[28].s64 = ctx.r[1].s64 + 304;
	// 82DD4BD4: EA640000  ld r19, 0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD4BD8: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 82DD4BDC: FAAA0000  std r21, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD4BE0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD4BE4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD4BE8: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD4BEC: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD4BF0: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DD4BF4: FA680000  std r19, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD4DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4DF0 size=712
    let mut pc: u32 = 0x82DD4DF0;
    'dispatch: loop {
        match pc {
            0x82DD4DF0 => {
    //   block [0x82DD4DF0..0x82DD4E44)
	// 82DD4DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4DF4: 4BED45F1  bl 0x82ca93e4
	ctx.lr = 0x82DD4DF8;
	sub_82CA93D0(ctx, base);
	// 82DD4DF8: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD4DFC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4E00: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DD4E04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD4E08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DD4E0C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82DD4E10: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD4E14: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DD4E18: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD4E1C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD4E20: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD4E24: 40980020  bge cr6, 0x82dd4e44
	if !ctx.cr[6].lt {
	pc = 0x82DD4E44; continue 'dispatch;
	}
	// 82DD4E28: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD4E2C: 392939B8  addi r9, r9, 0x39b8
	ctx.r[9].s64 = ctx.r[9].s64 + 14776;
	// 82DD4E30: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD4E34: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD4E38: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD4E3C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD4E40: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD4E44; continue 'dispatch;
            }
            0x82DD4E44 => {
    //   block [0x82DD4E44..0x82DD50B8)
	// 82DD4E44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4E48: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82DD4E4C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD4E50: 480477A9  bl 0x82e1c5f8
	ctx.lr = 0x82DD4E54;
	sub_82E1C5F8(ctx, base);
	// 82DD4E54: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD4E58: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82DD4E5C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DD4E60: 93E100B4  stw r31, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 82DD4E64: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD4E68: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD4E6C: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4E70: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD4E74: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4E78: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD4E7C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DD4E80: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD4E84: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82DD4E88: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD4E8C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82DD4E90: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD4E94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD4E98: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD4E9C: 3BA10150  addi r29, r1, 0x150
	ctx.r[29].s64 = ctx.r[1].s64 + 336;
	// 82DD4EA0: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD4EA4: 3BDB0030  addi r30, r27, 0x30
	ctx.r[30].s64 = ctx.r[27].s64 + 48;
	// 82DD4EA8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD4EAC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82DD4EB0: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD4EB4: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD4EB8: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD4EBC: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD50B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD50B8 size=632
    let mut pc: u32 = 0x82DD50B8;
    'dispatch: loop {
        match pc {
            0x82DD50B8 => {
    //   block [0x82DD50B8..0x82DD5108)
	// 82DD50B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD50BC: 4BED4321  bl 0x82ca93dc
	ctx.lr = 0x82DD50C0;
	sub_82CA93D0(ctx, base);
	// 82DD50C0: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD50C4: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD50C8: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	// 82DD50CC: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DD50D0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DD50D4: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 82DD50D8: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DD50DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD50E0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD50E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD50E8: 40980020  bge cr6, 0x82dd5108
	if !ctx.cr[6].lt {
	pc = 0x82DD5108; continue 'dispatch;
	}
	// 82DD50EC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD50F0: 392939B8  addi r9, r9, 0x39b8
	ctx.r[9].s64 = ctx.r[9].s64 + 14776;
	// 82DD50F4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD50F8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD50FC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD5100: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD5104: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD5108; continue 'dispatch;
            }
            0x82DD5108 => {
    //   block [0x82DD5108..0x82DD5330)
	// 82DD5108: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD510C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD5110: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD5114: 83380000  lwz r25, 0(r24)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5118: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82DD511C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5120: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DD5124: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DD5128: EA8B0000  ld r20, 0(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD512C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD5130: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD5134: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD5138: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD513C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD5140: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD5144: 3B8100A0  addi r28, r1, 0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + 160;
	// 82DD5148: EA440000  ld r18, 0(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD514C: 3BB90030  addi r29, r25, 0x30
	ctx.r[29].s64 = ctx.r[25].s64 + 48;
	// 82DD5150: FA8A0000  std r20, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD5154: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD5158: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD515C: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82DD5160: F8A90008  std r5, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82DD5164: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DD5168: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5330 size=652
    let mut pc: u32 = 0x82DD5330;
    'dispatch: loop {
        match pc {
            0x82DD5330 => {
    //   block [0x82DD5330..0x82DD5384)
	// 82DD5330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5334: 4BED40AD  bl 0x82ca93e0
	ctx.lr = 0x82DD5338;
	sub_82CA93D0(ctx, base);
	// 82DD5338: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD533C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5340: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DD5344: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DD5348: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DD534C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82DD5350: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82DD5354: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DD5358: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD535C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD5360: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD5364: 40980020  bge cr6, 0x82dd5384
	if !ctx.cr[6].lt {
	pc = 0x82DD5384; continue 'dispatch;
	}
	// 82DD5368: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD536C: 392939B8  addi r9, r9, 0x39b8
	ctx.r[9].s64 = ctx.r[9].s64 + 14776;
	// 82DD5370: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD5374: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD5378: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD537C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD5380: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD5384; continue 'dispatch;
            }
            0x82DD5384 => {
    //   block [0x82DD5384..0x82DD55BC)
	// 82DD5384: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5388: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82DD538C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD5390: 48047269  bl 0x82e1c5f8
	ctx.lr = 0x82DD5394;
	sub_82E1C5F8(ctx, base);
	// 82DD5394: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD5398: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DD539C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD53A0: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD53A4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD53A8: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD53AC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD53B0: EAAB0000  ld r21, 0(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD53B4: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD53B8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD53BC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DD53C0: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD53C4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD53C8: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD53CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD53D0: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD53D4: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82DD53D8: FAAA0000  std r21, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD53DC: 3BDB0030  addi r30, r27, 0x30
	ctx.r[30].s64 = ctx.r[27].s64 + 48;
	// 82DD53E0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD53E4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DD53E8: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD53EC: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD53F0: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD53F4: FA680000  std r19, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD55C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD55C0 size=64
    let mut pc: u32 = 0x82DD55C0;
    'dispatch: loop {
        match pc {
            0x82DD55C0 => {
    //   block [0x82DD55C0..0x82DD5600)
	// 82DD55C0: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD55C4: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD55C8: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD55CC: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD55D0: 38EB4B00  addi r7, r11, 0x4b00
	ctx.r[7].s64 = ctx.r[11].s64 + 19200;
	// 82DD55D4: 390845A0  addi r8, r8, 0x45a0
	ctx.r[8].s64 = ctx.r[8].s64 + 17824;
	// 82DD55D8: 39295330  addi r9, r9, 0x5330
	ctx.r[9].s64 = ctx.r[9].s64 + 21296;
	// 82DD55DC: 394A4DF0  addi r10, r10, 0x4df0
	ctx.r[10].s64 = ctx.r[10].s64 + 19952;
	// 82DD55E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD55E4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DD55E8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD55EC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD55F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DD55F4: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DD55F8: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82DD55FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD5600 size=68
    let mut pc: u32 = 0x82DD5600;
    'dispatch: loop {
        match pc {
            0x82DD5600 => {
    //   block [0x82DD5600..0x82DD5644)
	// 82DD5600: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD5604: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD5608: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD560C: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD5610: 39084698  addi r8, r8, 0x4698
	ctx.r[8].s64 = ctx.r[8].s64 + 18072;
	// 82DD5614: 39295C38  addi r9, r9, 0x5c38
	ctx.r[9].s64 = ctx.r[9].s64 + 23608;
	// 82DD5618: 394A5CD8  addi r10, r10, 0x5cd8
	ctx.r[10].s64 = ctx.r[10].s64 + 23768;
	// 82DD561C: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD5620: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DD5624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DD5628: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD562C: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD5630: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DD5634: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DD5638: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 82DD563C: 98C30011  stb r6, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[6].u8 ) };
	// 82DD5640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5648 size=204
    let mut pc: u32 = 0x82DD5648;
    'dispatch: loop {
        match pc {
            0x82DD5648 => {
    //   block [0x82DD5648..0x82DD5714)
	// 82DD5648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD564C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD5654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD5658: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD565C: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD5660: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD5664: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD5668: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD566C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD5670: 39084698  addi r8, r8, 0x4698
	ctx.r[8].s64 = ctx.r[8].s64 + 18072;
	// 82DD5674: 39295C38  addi r9, r9, 0x5c38
	ctx.r[9].s64 = ctx.r[9].s64 + 23608;
	// 82DD5678: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD567C: 394A5CD8  addi r10, r10, 0x5cd8
	ctx.r[10].s64 = ctx.r[10].s64 + 23768;
	// 82DD5680: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD5684: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD5688: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82DD568C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD5690: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82DD5694: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD5698: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD569C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD56A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD56A4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD56A8: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD56AC: 4BFEBABD  bl 0x82dc1168
	ctx.lr = 0x82DD56B0;
	sub_82DC1168(ctx, base);
	// 82DD56B0: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD56B4: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD56B8: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD56BC: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD56C0: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD56C4: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD56C8: 390845A0  addi r8, r8, 0x45a0
	ctx.r[8].s64 = ctx.r[8].s64 + 17824;
	// 82DD56CC: 39295330  addi r9, r9, 0x5330
	ctx.r[9].s64 = ctx.r[9].s64 + 21296;
	// 82DD56D0: 394A4DF0  addi r10, r10, 0x4df0
	ctx.r[10].s64 = ctx.r[10].s64 + 19952;
	// 82DD56D4: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD56D8: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82DD56DC: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82DD56E0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD56E4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD56E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD56EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD56F0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD56F4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD56F8: 4BFEBA71  bl 0x82dc1168
	ctx.lr = 0x82DD56FC;
	sub_82DC1168(ctx, base);
	// 82DD56FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD5700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5708: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD570C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD5710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5718 size=204
    let mut pc: u32 = 0x82DD5718;
    'dispatch: loop {
        match pc {
            0x82DD5718 => {
    //   block [0x82DD5718..0x82DD57E4)
	// 82DD5718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD571C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD5724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD5728: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD572C: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD5730: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD5734: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD5738: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD573C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD5740: 39084698  addi r8, r8, 0x4698
	ctx.r[8].s64 = ctx.r[8].s64 + 18072;
	// 82DD5744: 39295C38  addi r9, r9, 0x5c38
	ctx.r[9].s64 = ctx.r[9].s64 + 23608;
	// 82DD5748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD574C: 394A5CD8  addi r10, r10, 0x5cd8
	ctx.r[10].s64 = ctx.r[10].s64 + 23768;
	// 82DD5750: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD5754: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD5758: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82DD575C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD5760: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82DD5764: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD5768: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD576C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD5770: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD5774: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD5778: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD577C: 4BFEBB05  bl 0x82dc1280
	ctx.lr = 0x82DD5780;
	sub_82DC1280(ctx, base);
	// 82DD5780: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD5784: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD5788: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD578C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD5790: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD5794: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD5798: 390845A0  addi r8, r8, 0x45a0
	ctx.r[8].s64 = ctx.r[8].s64 + 17824;
	// 82DD579C: 39295330  addi r9, r9, 0x5330
	ctx.r[9].s64 = ctx.r[9].s64 + 21296;
	// 82DD57A0: 394A4DF0  addi r10, r10, 0x4df0
	ctx.r[10].s64 = ctx.r[10].s64 + 19952;
	// 82DD57A4: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD57A8: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82DD57AC: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82DD57B0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD57B4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD57B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD57BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD57C0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD57C4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD57C8: 4BFEBAB9  bl 0x82dc1280
	ctx.lr = 0x82DD57CC;
	sub_82DC1280(ctx, base);
	// 82DD57CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD57D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD57D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD57D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD57DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD57E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD57E8 size=232
    let mut pc: u32 = 0x82DD57E8;
    'dispatch: loop {
        match pc {
            0x82DD57E8 => {
    //   block [0x82DD57E8..0x82DD5814)
	// 82DD57E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD57EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD57F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD57F4: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD57F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD57FC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD5800: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DD5804: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DD5808: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DD580C: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DD5810: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DD5814; continue 'dispatch;
            }
            0x82DD5814 => {
    //   block [0x82DD5814..0x82DD58D0)
	// 82DD5814: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD5818: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DD581C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DD5820: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DD5824: 4200FFF0  bdnz 0x82dd5814
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DD5814; continue 'dispatch;
	}
	// 82DD5828: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD582C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DD5830: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DD5834: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DD5838: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD583C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD58D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD58D0 size=792
    let mut pc: u32 = 0x82DD58D0;
    'dispatch: loop {
        match pc {
            0x82DD58D0 => {
    //   block [0x82DD58D0..0x82DD5928)
	// 82DD58D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD58D4: 4BED3B09  bl 0x82ca93dc
	ctx.lr = 0x82DD58D8;
	sub_82CA93D0(ctx, base);
	// 82DD58D8: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD58DC: 828D0000  lwz r20, 0(r13)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD58E0: 3AA00008  li r21, 8
	ctx.r[21].s64 = 8;
	// 82DD58E4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DD58E8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DD58EC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DD58F0: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82DD58F4: 7D75A02E  lwzx r11, r21, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82DD58F8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DD58FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD5900: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD5904: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD5908: 40980020  bge cr6, 0x82dd5928
	if !ctx.cr[6].lt {
	pc = 0x82DD5928; continue 'dispatch;
	}
	// 82DD590C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD5910: 392939A8  addi r9, r9, 0x39a8
	ctx.r[9].s64 = ctx.r[9].s64 + 14760;
	// 82DD5914: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD5918: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD591C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD5920: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD5924: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD5928; continue 'dispatch;
            }
            0x82DD5928 => {
    //   block [0x82DD5928..0x82DD5BE8)
	// 82DD5928: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD592C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD5930: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD5934: 83980000  lwz r28, 0(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5938: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD593C: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5940: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD5944: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD5948: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD594C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD5950: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD5954: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD5958: EA660000  ld r19, 0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD595C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD5960: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD5964: 3BC100A0  addi r30, r1, 0xa0
	ctx.r[30].s64 = ctx.r[1].s64 + 160;
	// 82DD5968: EA450000  ld r18, 0(r5)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD596C: 3BFC0030  addi r31, r28, 0x30
	ctx.r[31].s64 = ctx.r[28].s64 + 48;
	// 82DD5970: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD5974: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD5978: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD597C: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82DD5980: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD5984: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD5988: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5BE8 size=76
    let mut pc: u32 = 0x82DD5BE8;
    'dispatch: loop {
        match pc {
            0x82DD5BE8 => {
    //   block [0x82DD5BE8..0x82DD5C34)
	// 82DD5BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5BF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5BF4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD5BF8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD5BFC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD5C00: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD5C04: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD5C08: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD5C0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD5C10: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD5C14: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD5C18: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD5C1C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD5C20: 4BFFF499  bl 0x82dd50b8
	ctx.lr = 0x82DD5C24;
	sub_82DD50B8(ctx, base);
	// 82DD5C24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5C38 size=76
    let mut pc: u32 = 0x82DD5C38;
    'dispatch: loop {
        match pc {
            0x82DD5C38 => {
    //   block [0x82DD5C38..0x82DD5C84)
	// 82DD5C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5C44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD5C48: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD5C4C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD5C50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD5C54: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD5C58: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD5C5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD5C60: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD5C64: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD5C68: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD5C6C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD5C70: 4BFFF6C1  bl 0x82dd5330
	ctx.lr = 0x82DD5C74;
	sub_82DD5330(ctx, base);
	// 82DD5C74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD5C88 size=80
    let mut pc: u32 = 0x82DD5C88;
    'dispatch: loop {
        match pc {
            0x82DD5C88 => {
    //   block [0x82DD5C88..0x82DD5CD8)
	// 82DD5C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5C90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5C94: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD5C98: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD5C9C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD5CA0: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD5CA4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD5CA8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD5CAC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD5CB0: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD5CB4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD5CB8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD5CBC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD5CC0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD5CC4: 4BFFEE7D  bl 0x82dd4b40
	ctx.lr = 0x82DD5CC8;
	sub_82DD4B40(ctx, base);
	// 82DD5CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD5CD8 size=80
    let mut pc: u32 = 0x82DD5CD8;
    'dispatch: loop {
        match pc {
            0x82DD5CD8 => {
    //   block [0x82DD5CD8..0x82DD5D28)
	// 82DD5CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5CE4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD5CE8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD5CEC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD5CF0: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD5CF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD5CF8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD5CFC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD5D00: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD5D04: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD5D08: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD5D0C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD5D10: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD5D14: 4BFFF0DD  bl 0x82dd4df0
	ctx.lr = 0x82DD5D18;
	sub_82DD4DF0(ctx, base);
	// 82DD5D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD5D28 size=176
    let mut pc: u32 = 0x82DD5D28;
    'dispatch: loop {
        match pc {
            0x82DD5D28 => {
    //   block [0x82DD5D28..0x82DD5DA8)
	// 82DD5D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5D30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD5D34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD5D38: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD5D3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5D40: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD5D44: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD5D48: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD5D4C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD5D50: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5D54: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD5D58: 4BFFFB79  bl 0x82dd58d0
	ctx.lr = 0x82DD5D5C;
	sub_82DD58D0(ctx, base);
	// 82DD5D5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5D60: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD5D64: 40980044  bge cr6, 0x82dd5da8
	if !ctx.cr[6].lt {
	pc = 0x82DD5DA8; continue 'dispatch;
	}
	// 82DD5D68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD5D6C: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD5D70: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD5DA8; continue 'dispatch;
            }
            0x82DD5DA8 => {
    //   block [0x82DD5DA8..0x82DD5DD8)
	// 82DD5DA8: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD5DAC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD5DB0: 419A000C  beq cr6, 0x82dd5dbc
	if ctx.cr[6].eq {
	pc = 0x82DD5DBC; continue 'dispatch;
	}
	// 82DD5DB4: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD5DB8: 4BFF51D1  bl 0x82dcaf88
	ctx.lr = 0x82DD5DBC;
	sub_82DCAF88(ctx, base);
	// 82DD5DBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5DC8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD5DCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD5DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD5DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5DD8 size=116
    let mut pc: u32 = 0x82DD5DD8;
    'dispatch: loop {
        match pc {
            0x82DD5DD8 => {
    //   block [0x82DD5DD8..0x82DD5E4C)
	// 82DD5DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD5DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD5DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5DE8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5DEC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD5DF0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD5DF4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DD5DF8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD5DFC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD5E00: 4BF7F449  bl 0x82d55248
	ctx.lr = 0x82DD5E04;
	sub_82D55248(ctx, base);
	// 82DD5E04: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD5E08: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DD5E0C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82DD5E10: 394B2C9C  addi r10, r11, 0x2c9c
	ctx.r[10].s64 = ctx.r[11].s64 + 11420;
	// 82DD5E14: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82DD5E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DD5E1C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82DD5E20: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DD5E24: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DD5E28: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DD5E2C: B163000C  sth r11, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82DD5E30: B163000E  sth r11, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82DD5E34: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82DD5E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD5E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD5E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5E50 size=120
    let mut pc: u32 = 0x82DD5E50;
    'dispatch: loop {
        match pc {
            0x82DD5E50 => {
    //   block [0x82DD5E50..0x82DD5E6C)
	// 82DD5E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5E54: 4BED35B5  bl 0x82ca9408
	ctx.lr = 0x82DD5E58;
	sub_82CA93D0(ctx, base);
	// 82DD5E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD5E60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD5E64: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82DD5E68: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	pc = 0x82DD5E6C; continue 'dispatch;
            }
            0x82DD5E6C => {
    //   block [0x82DD5E6C..0x82DD5E90)
	// 82DD5E6C: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5E70: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82DD5E74: 419A001C  beq cr6, 0x82dd5e90
	if ctx.cr[6].eq {
	pc = 0x82DD5E90; continue 'dispatch;
	}
	// 82DD5E78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD5E7C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD5E80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5E84: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD5E88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD5E8C: 4E800421  bctrl
	ctx.lr = 0x82DD5E90;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD5E90 => {
    //   block [0x82DD5E90..0x82DD5EC0)
	// 82DD5E90: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DD5E94: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DD5E98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DD5E9C: 409AFFD0  bne cr6, 0x82dd5e6c
	if !ctx.cr[6].eq {
	pc = 0x82DD5E6C; continue 'dispatch;
	}
	// 82DD5EA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DD5EA4: 419A001C  beq cr6, 0x82dd5ec0
	if ctx.cr[6].eq {
	pc = 0x82DD5EC0; continue 'dispatch;
	}
	// 82DD5EA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5EAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD5EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD5EB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD5EBC: 4E800421  bctrl
	ctx.lr = 0x82DD5EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD5EC0 => {
    //   block [0x82DD5EC0..0x82DD5EC8)
	// 82DD5EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD5EC4: 4BED3594  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5EC8 size=672
    let mut pc: u32 = 0x82DD5EC8;
    'dispatch: loop {
        match pc {
            0x82DD5EC8 => {
    //   block [0x82DD5EC8..0x82DD5F18)
	// 82DD5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5ECC: 4BED3519  bl 0x82ca93e4
	ctx.lr = 0x82DD5ED0;
	sub_82CA93D0(ctx, base);
	// 82DD5ED0: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD5ED4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5ED8: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DD5EDC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DD5EE0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DD5EE4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD5EE8: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DD5EEC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD5EF0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD5EF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD5EF8: 40980020  bge cr6, 0x82dd5f18
	if !ctx.cr[6].lt {
	pc = 0x82DD5F18; continue 'dispatch;
	}
	// 82DD5EFC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD5F00: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82DD5F04: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD5F08: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD5F0C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD5F10: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD5F14: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD5F18; continue 'dispatch;
            }
            0x82DD5F18 => {
    //   block [0x82DD5F18..0x82DD6168)
	// 82DD5F18: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD5F1C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DD5F20: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DD5F24: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5F28: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD5F2C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5F30: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD5F34: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD5F38: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD5F3C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DD5F40: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD5F44: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82DD5F48: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD5F4C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD5F50: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD5F54: 3BC100D0  addi r30, r1, 0xd0
	ctx.r[30].s64 = ctx.r[1].s64 + 208;
	// 82DD5F58: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD5F5C: 3BFA0030  addi r31, r26, 0x30
	ctx.r[31].s64 = ctx.r[26].s64 + 48;
	// 82DD5F60: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD5F64: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD5F68: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82DD5F6C: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD5F70: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD5F74: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD5F78: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD6168 size=20
    let mut pc: u32 = 0x82DD6168;
    'dispatch: loop {
        match pc {
            0x82DD6168 => {
    //   block [0x82DD6168..0x82DD617C)
	// 82DD6168: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD616C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD6170: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD6174: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DD6178: 4BFFFD50  b 0x82dd5ec8
	sub_82DD5EC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD6180 size=460
    let mut pc: u32 = 0x82DD6180;
    'dispatch: loop {
        match pc {
            0x82DD6180 => {
    //   block [0x82DD6180..0x82DD634C)
	// 82DD6180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6184: 4BED326D  bl 0x82ca93f0
	ctx.lr = 0x82DD6188;
	sub_82CA93D0(ctx, base);
	// 82DD6188: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD618C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DD6190: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD6194: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD6198: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD619C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DD61A0: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD61A4: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 82DD61A8: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 82DD61AC: D007001C  stfs f0, 0x1c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DD61B0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DD61B4: D007003C  stfs f0, 0x3c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82DD61B8: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DD61BC: D007005C  stfs f0, 0x5c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DD61C0: EB2B0000  ld r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD61C4: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD61C8: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD61CC: EB030000  ld r24, 0(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82DD61D0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DD61D4: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82DD61D8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DD61DC: EAFF0000  ld r23, 0(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82DD61E0: 3B8100B0  addi r28, r1, 0xb0
	ctx.r[28].s64 = ctx.r[1].s64 + 176;
	// 82DD61E4: FB2A0000  std r25, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82DD61E8: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 82DD61EC: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD61F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DD61F4: FB090000  std r24, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82DD61F8: F8690008  std r3, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[3].u64 ) };
	// 82DD61FC: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82DD6200: FAE80000  std r23, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6350 size=448
    let mut pc: u32 = 0x82DD6350;
    'dispatch: loop {
        match pc {
            0x82DD6350 => {
    //   block [0x82DD6350..0x82DD6510)
	// 82DD6350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6354: 4BED3099  bl 0x82ca93ec
	ctx.lr = 0x82DD6358;
	sub_82CA93D0(ctx, base);
	// 82DD6358: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD635C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD6360: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82DD6364: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DD6368: 83030000  lwz r24, 0(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD636C: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82DD6370: 83240000  lwz r25, 0(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6374: 3BCB0020  addi r30, r11, 0x20
	ctx.r[30].s64 = ctx.r[11].s64 + 32;
	// 82DD6378: 3BAB0030  addi r29, r11, 0x30
	ctx.r[29].s64 = ctx.r[11].s64 + 48;
	// 82DD637C: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD6380: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DD6384: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD6388: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DD638C: EAFF0000  ld r23, 0(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82DD6390: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DD6394: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82DD6398: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82DD639C: EADE0000  ld r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82DD63A0: 3B4100B0  addi r26, r1, 0xb0
	ctx.r[26].s64 = ctx.r[1].s64 + 176;
	// 82DD63A4: F8CA0000  std r6, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82DD63A8: 3B780030  addi r27, r24, 0x30
	ctx.r[27].s64 = ctx.r[24].s64 + 48;
	// 82DD63AC: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD63B0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DD63B4: FAE90000  std r23, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD63B8: FBE90008  std r31, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 82DD63BC: EBDE0008  ld r30, 8(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82DD63C0: FAC70000  std r22, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6510 size=596
    let mut pc: u32 = 0x82DD6510;
    'dispatch: loop {
        match pc {
            0x82DD6510 => {
    //   block [0x82DD6510..0x82DD6558)
	// 82DD6510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6514: 4BED2ED1  bl 0x82ca93e4
	ctx.lr = 0x82DD6518;
	sub_82CA93D0(ctx, base);
	// 82DD6518: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD651C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6520: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DD6524: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DD6528: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DD652C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD6530: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD6534: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD6538: 40980020  bge cr6, 0x82dd6558
	if !ctx.cr[6].lt {
	pc = 0x82DD6558; continue 'dispatch;
	}
	// 82DD653C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD6540: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82DD6544: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD6548: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD654C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD6550: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD6554: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD6558; continue 'dispatch;
            }
            0x82DD6558 => {
    //   block [0x82DD6558..0x82DD6764)
	// 82DD6558: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD655C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD6560: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82DD6564: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD6568: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82DD656C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD6570: 83440000  lwz r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6574: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DD6578: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82DD657C: 83650000  lwz r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6580: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD6584: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD6588: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD658C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD6590: EA830000  ld r20, 0(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82DD6594: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD6598: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82DD659C: 3B8100F0  addi r28, r1, 0xf0
	ctx.r[28].s64 = ctx.r[1].s64 + 240;
	// 82DD65A0: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD65A4: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 82DD65A8: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD65AC: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD65B0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD65B4: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DD65B8: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD65BC: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD65C0: EA7F0000  ld r19, 0(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6768 size=596
    let mut pc: u32 = 0x82DD6768;
    'dispatch: loop {
        match pc {
            0x82DD6768 => {
    //   block [0x82DD6768..0x82DD67B0)
	// 82DD6768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD676C: 4BED2C79  bl 0x82ca93e4
	ctx.lr = 0x82DD6770;
	sub_82CA93D0(ctx, base);
	// 82DD6770: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD6774: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6778: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DD677C: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD6780: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DD6784: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD6788: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD678C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD6790: 40980020  bge cr6, 0x82dd67b0
	if !ctx.cr[6].lt {
	pc = 0x82DD67B0; continue 'dispatch;
	}
	// 82DD6794: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD6798: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82DD679C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD67A0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD67A4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD67A8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD67AC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD67B0; continue 'dispatch;
            }
            0x82DD67B0 => {
    //   block [0x82DD67B0..0x82DD69BC)
	// 82DD67B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD67B4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD67B8: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82DD67BC: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD67C0: 908100B4  stw r4, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[4].u32 ) };
	// 82DD67C4: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD67C8: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD67CC: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DD67D0: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD67D4: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD67D8: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD67DC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD67E0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD67E4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD67E8: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD67EC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DD67F0: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD67F4: 3B8100F0  addi r28, r1, 0xf0
	ctx.r[28].s64 = ctx.r[1].s64 + 240;
	// 82DD67F8: EA830000  ld r20, 0(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82DD67FC: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 82DD6800: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD6804: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD6808: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD680C: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD6810: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD6814: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82DD6818: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD69C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD69C0 size=104
    let mut pc: u32 = 0x82DD69C0;
    'dispatch: loop {
        match pc {
            0x82DD69C0 => {
    //   block [0x82DD69C0..0x82DD6A28)
	// 82DD69C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD69C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD69C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD69CC: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD69D0: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD69D4: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD69D8: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD69DC: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD69E0: 39085DD8  addi r8, r8, 0x5dd8
	ctx.r[8].s64 = ctx.r[8].s64 + 24024;
	// 82DD69E4: 39295EC8  addi r9, r9, 0x5ec8
	ctx.r[9].s64 = ctx.r[9].s64 + 24264;
	// 82DD69E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD69EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD69F0: 394A6768  addi r10, r10, 0x6768
	ctx.r[10].s64 = ctx.r[10].s64 + 26472;
	// 82DD69F4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82DD69F8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82DD69FC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD6A00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD6A04: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD6A08: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD6A0C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD6A10: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82DD6A14: 4BFEA755  bl 0x82dc1168
	ctx.lr = 0x82DD6A18;
	sub_82DC1168(ctx, base);
	// 82DD6A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD6A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD6A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD6A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD6A28 size=796
    let mut pc: u32 = 0x82DD6A28;
    'dispatch: loop {
        match pc {
            0x82DD6A28 => {
    //   block [0x82DD6A28..0x82DD6A80)
	// 82DD6A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6A2C: 4BED29B1  bl 0x82ca93dc
	ctx.lr = 0x82DD6A30;
	sub_82CA93D0(ctx, base);
	// 82DD6A30: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD6A34: 828D0000  lwz r20, 0(r13)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6A38: 3AA00008  li r21, 8
	ctx.r[21].s64 = 8;
	// 82DD6A3C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DD6A40: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DD6A44: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DD6A48: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD6A4C: 7D75A02E  lwzx r11, r21, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82DD6A50: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DD6A54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD6A58: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD6A5C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD6A60: 40980020  bge cr6, 0x82dd6a80
	if !ctx.cr[6].lt {
	pc = 0x82DD6A80; continue 'dispatch;
	}
	// 82DD6A64: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD6A68: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82DD6A6C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD6A70: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD6A74: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD6A78: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD6A7C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD6A80; continue 'dispatch;
            }
            0x82DD6A80 => {
    //   block [0x82DD6A80..0x82DD6D44)
	// 82DD6A80: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD6A84: C0170004  lfs f0, 4(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD6A88: D00100EC  stfs f0, 0xec(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 82DD6A8C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD6A90: D001010C  stfs f0, 0x10c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 82DD6A94: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD6A98: D001012C  stfs f0, 0x12c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 82DD6A9C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD6AA0: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD6AA4: 83990000  lwz r28, 0(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6AA8: EB0B0000  ld r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD6AAC: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD6AB0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD6AB4: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DD6AB8: EA660000  ld r19, 0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD6ABC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD6AC0: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD6AC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD6AC8: EA450000  ld r18, 0(r5)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD6ACC: 3BC100C0  addi r30, r1, 0xc0
	ctx.r[30].s64 = ctx.r[1].s64 + 192;
	// 82DD6AD0: FB0A0000  std r24, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82DD6AD4: 3BFC0030  addi r31, r28, 0x30
	ctx.r[31].s64 = ctx.r[28].s64 + 48;
	// 82DD6AD8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD6ADC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD6AE0: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82DD6AE4: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD6AE8: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD6AEC: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6D48 size=104
    let mut pc: u32 = 0x82DD6D48;
    'dispatch: loop {
        match pc {
            0x82DD6D48 => {
    //   block [0x82DD6D48..0x82DD6DB0)
	// 82DD6D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD6D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD6D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD6D58: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6D5C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD6D60: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD6D64: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DD6D68: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD6D6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD6D70: 4BF7E4D9  bl 0x82d55248
	ctx.lr = 0x82DD6D74;
	sub_82D55248(ctx, base);
	// 82DD6D74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD6D78: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DD6D7C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DD6D80: 396B3A04  addi r11, r11, 0x3a04
	ctx.r[11].s64 = ctx.r[11].s64 + 14852;
	// 82DD6D84: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD6D88: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD6D8C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD6D90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD6D94: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD6D98: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD6D9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD6DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD6DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD6DA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD6DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6DB0 size=104
    let mut pc: u32 = 0x82DD6DB0;
    'dispatch: loop {
        match pc {
            0x82DD6DB0 => {
    //   block [0x82DD6DB0..0x82DD6DEC)
	// 82DD6DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD6DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD6DBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD6DC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD6DC4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DD6DC8: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD6DCC: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82DD6DD0: 419A001C  beq cr6, 0x82dd6dec
	if ctx.cr[6].eq {
	pc = 0x82DD6DEC; continue 'dispatch;
	}
	// 82DD6DD4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD6DD8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD6DDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6DE0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD6DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD6DE8: 4E800421  bctrl
	ctx.lr = 0x82DD6DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD6DEC => {
    //   block [0x82DD6DEC..0x82DD6E18)
	// 82DD6DEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6DF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD6DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD6DF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD6E00: 4E800421  bctrl
	ctx.lr = 0x82DD6E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD6E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD6E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD6E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD6E10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD6E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6E18 size=104
    let mut pc: u32 = 0x82DD6E18;
    'dispatch: loop {
        match pc {
            0x82DD6E18 => {
    //   block [0x82DD6E18..0x82DD6E80)
	// 82DD6E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD6E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD6E24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD6E28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6E2C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD6E30: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD6E34: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DD6E38: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD6E3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD6E40: 4BF7E409  bl 0x82d55248
	ctx.lr = 0x82DD6E44;
	sub_82D55248(ctx, base);
	// 82DD6E44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD6E48: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DD6E4C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DD6E50: 396B3A40  addi r11, r11, 0x3a40
	ctx.r[11].s64 = ctx.r[11].s64 + 14912;
	// 82DD6E54: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD6E58: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD6E5C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD6E60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD6E64: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD6E68: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD6E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD6E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD6E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD6E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD6E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6E80 size=360
    let mut pc: u32 = 0x82DD6E80;
    'dispatch: loop {
        match pc {
            0x82DD6E80 => {
    //   block [0x82DD6E80..0x82DD6ED4)
	// 82DD6E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6E84: 4BED2579  bl 0x82ca93fc
	ctx.lr = 0x82DD6E88;
	sub_82CA93D0(ctx, base);
	// 82DD6E88: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD6E8C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6E90: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD6E94: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DD6E98: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD6E9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DD6EA0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DD6EA4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6EA8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD6EAC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD6EB0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD6EB4: 40980020  bge cr6, 0x82dd6ed4
	if !ctx.cr[6].lt {
	pc = 0x82DD6ED4; continue 'dispatch;
	}
	// 82DD6EB8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD6EBC: 39293A78  addi r9, r9, 0x3a78
	ctx.r[9].s64 = ctx.r[9].s64 + 14968;
	// 82DD6EC0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD6EC4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD6EC8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD6ECC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD6ED0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD6ED4; continue 'dispatch;
            }
            0x82DD6ED4 => {
    //   block [0x82DD6ED4..0x82DD6FE8)
	// 82DD6ED4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD6ED8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DD6EDC: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6EE0: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 82DD6EE4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD6EE8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD6EEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD6EF0: 4BF7F501  bl 0x82d563f0
	ctx.lr = 0x82DD6EF4;
	sub_82D563F0(ctx, base);
	// 82DD6EF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD6EF8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD6FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD6FE8 size=20
    let mut pc: u32 = 0x82DD6FE8;
    'dispatch: loop {
        match pc {
            0x82DD6FE8 => {
    //   block [0x82DD6FE8..0x82DD6FFC)
	// 82DD6FE8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD6FEC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD6FF0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD6FF4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DD6FF8: 4BFFFE88  b 0x82dd6e80
	sub_82DD6E80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7000 size=888
    let mut pc: u32 = 0x82DD7000;
    'dispatch: loop {
        match pc {
            0x82DD7000 => {
    //   block [0x82DD7000..0x82DD705C)
	// 82DD7000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7004: 4BED23F1  bl 0x82ca93f4
	ctx.lr = 0x82DD7008;
	sub_82CA93D0(ctx, base);
	// 82DD7008: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD700C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7010: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD7014: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DD7018: 7EEB5214  add r23, r11, r10
	ctx.r[23].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD701C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DD7020: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DD7024: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DD7028: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DD702C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7030: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD7034: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD7038: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD703C: 40980020  bge cr6, 0x82dd705c
	if !ctx.cr[6].lt {
	pc = 0x82DD705C; continue 'dispatch;
	}
	// 82DD7040: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD7044: 39293A78  addi r9, r9, 0x3a78
	ctx.r[9].s64 = ctx.r[9].s64 + 14968;
	// 82DD7048: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD704C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD7050: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD7054: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD7058: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD705C; continue 'dispatch;
            }
            0x82DD705C => {
    //   block [0x82DD705C..0x82DD7378)
	// 82DD705C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7060: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DD7064: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7068: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 82DD706C: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 82DD7070: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7074: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD7078: 4BF7F379  bl 0x82d563f0
	ctx.lr = 0x82DD707C;
	sub_82D563F0(ctx, base);
	// 82DD707C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD7080: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 82DD7084: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7088: 396B14A0  addi r11, r11, 0x14a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5280;
	// 82DD708C: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 82DD7090: 811B0000  lwz r8, 0(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7378 size=784
    let mut pc: u32 = 0x82DD7378;
    'dispatch: loop {
        match pc {
            0x82DD7378 => {
    //   block [0x82DD7378..0x82DD73D0)
	// 82DD7378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD737C: 4BED2081  bl 0x82ca93fc
	ctx.lr = 0x82DD7380;
	sub_82CA93D0(ctx, base);
	// 82DD7380: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7384: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7388: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD738C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD7390: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD7394: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DD7398: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DD739C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD73A0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD73A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD73A8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD73AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD73B0: 40980020  bge cr6, 0x82dd73d0
	if !ctx.cr[6].lt {
	pc = 0x82DD73D0; continue 'dispatch;
	}
	// 82DD73B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD73B8: 39293A78  addi r9, r9, 0x3a78
	ctx.r[9].s64 = ctx.r[9].s64 + 14968;
	// 82DD73BC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD73C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD73C4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD73C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD73CC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD73D0; continue 'dispatch;
            }
            0x82DD73D0 => {
    //   block [0x82DD73D0..0x82DD7688)
	// 82DD73D0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD73D4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DD73D8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD73DC: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 82DD73E0: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 82DD73E4: 93C100A0  stw r30, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 82DD73E8: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 82DD73EC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD73F0: 4BF7F001  bl 0x82d563f0
	ctx.lr = 0x82DD73F4;
	sub_82D563F0(ctx, base);
	// 82DD73F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD73F8: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 82DD73FC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7400: 396B14A0  addi r11, r11, 0x14a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5280;
	// 82DD7404: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 82DD7408: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7688 size=784
    let mut pc: u32 = 0x82DD7688;
    'dispatch: loop {
        match pc {
            0x82DD7688 => {
    //   block [0x82DD7688..0x82DD76E0)
	// 82DD7688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD768C: 4BED1D71  bl 0x82ca93fc
	ctx.lr = 0x82DD7690;
	sub_82CA93D0(ctx, base);
	// 82DD7690: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7694: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7698: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD769C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD76A0: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD76A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DD76A8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DD76AC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DD76B0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD76B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD76B8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD76BC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD76C0: 40980020  bge cr6, 0x82dd76e0
	if !ctx.cr[6].lt {
	pc = 0x82DD76E0; continue 'dispatch;
	}
	// 82DD76C4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD76C8: 39293A78  addi r9, r9, 0x3a78
	ctx.r[9].s64 = ctx.r[9].s64 + 14968;
	// 82DD76CC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD76D0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD76D4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD76D8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD76DC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD76E0; continue 'dispatch;
            }
            0x82DD76E0 => {
    //   block [0x82DD76E0..0x82DD7998)
	// 82DD76E0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD76E4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DD76E8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD76EC: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 82DD76F0: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 82DD76F4: 93C100A0  stw r30, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 82DD76F8: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 82DD76FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DD7700: 4BF7ECF1  bl 0x82d563f0
	ctx.lr = 0x82DD7704;
	sub_82D563F0(ctx, base);
	// 82DD7704: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD7708: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 82DD770C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7710: 396B14A0  addi r11, r11, 0x14a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5280;
	// 82DD7714: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 82DD7718: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7998 size=204
    let mut pc: u32 = 0x82DD7998;
    'dispatch: loop {
        match pc {
            0x82DD7998 => {
    //   block [0x82DD7998..0x82DD7A64)
	// 82DD7998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD799C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD79A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD79A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD79A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD79AC: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD79B0: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD79B4: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD79B8: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD79BC: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD79C0: 39086E18  addi r8, r8, 0x6e18
	ctx.r[8].s64 = ctx.r[8].s64 + 28184;
	// 82DD79C4: 39297AB0  addi r9, r9, 0x7ab0
	ctx.r[9].s64 = ctx.r[9].s64 + 31408;
	// 82DD79C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD79CC: 394A7B50  addi r10, r10, 0x7b50
	ctx.r[10].s64 = ctx.r[10].s64 + 31568;
	// 82DD79D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD79D4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD79D8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DD79DC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD79E0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82DD79E4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD79E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD79EC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD79F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD79F4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD79F8: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD79FC: 4BFE976D  bl 0x82dc1168
	ctx.lr = 0x82DD7A00;
	sub_82DC1168(ctx, base);
	// 82DD7A00: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD7A04: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD7A08: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD7A0C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD7A10: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DD7A14: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD7A18: 39086D48  addi r8, r8, 0x6d48
	ctx.r[8].s64 = ctx.r[8].s64 + 27976;
	// 82DD7A1C: 39296E80  addi r9, r9, 0x6e80
	ctx.r[9].s64 = ctx.r[9].s64 + 28288;
	// 82DD7A20: 394A7688  addi r10, r10, 0x7688
	ctx.r[10].s64 = ctx.r[10].s64 + 30344;
	// 82DD7A24: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD7A28: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82DD7A2C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DD7A30: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD7A34: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD7A38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD7A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD7A40: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD7A44: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD7A48: 4BFE9721  bl 0x82dc1168
	ctx.lr = 0x82DD7A4C;
	sub_82DC1168(ctx, base);
	// 82DD7A4C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD7A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7A58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD7A5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD7A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7A68 size=68
    let mut pc: u32 = 0x82DD7A68;
    'dispatch: loop {
        match pc {
            0x82DD7A68 => {
    //   block [0x82DD7A68..0x82DD7AAC)
	// 82DD7A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD7A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7A74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD7A78: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD7A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD7A80: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DD7A84: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD7A88: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD7A8C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD7A90: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DD7A94: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD7A98: 4BFFF3E9  bl 0x82dd6e80
	ctx.lr = 0x82DD7A9C;
	sub_82DD6E80(ctx, base);
	// 82DD7A9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7AB0 size=76
    let mut pc: u32 = 0x82DD7AB0;
    'dispatch: loop {
        match pc {
            0x82DD7AB0 => {
    //   block [0x82DD7AB0..0x82DD7AFC)
	// 82DD7AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD7AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7ABC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD7AC0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD7AC4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD7AC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD7ACC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD7AD0: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD7AD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD7AD8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD7ADC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD7AE0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD7AE4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD7AE8: 4BFFF399  bl 0x82dd6e80
	ctx.lr = 0x82DD7AEC;
	sub_82DD6E80(ctx, base);
	// 82DD7AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD7B00 size=80
    let mut pc: u32 = 0x82DD7B00;
    'dispatch: loop {
        match pc {
            0x82DD7B00 => {
    //   block [0x82DD7B00..0x82DD7B50)
	// 82DD7B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD7B08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7B0C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD7B10: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD7B14: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD7B18: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD7B1C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD7B20: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD7B24: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD7B28: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD7B2C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD7B30: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD7B34: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD7B38: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD7B3C: 4BFFF83D  bl 0x82dd7378
	ctx.lr = 0x82DD7B40;
	sub_82DD7378(ctx, base);
	// 82DD7B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD7B50 size=80
    let mut pc: u32 = 0x82DD7B50;
    'dispatch: loop {
        match pc {
            0x82DD7B50 => {
    //   block [0x82DD7B50..0x82DD7BA0)
	// 82DD7B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD7B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7B5C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD7B60: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD7B64: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD7B68: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD7B6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD7B70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD7B74: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD7B78: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD7B7C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD7B80: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD7B84: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD7B88: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD7B8C: 4BFFFAFD  bl 0x82dd7688
	ctx.lr = 0x82DD7B90;
	sub_82DD7688(ctx, base);
	// 82DD7B90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD7BA0 size=176
    let mut pc: u32 = 0x82DD7BA0;
    'dispatch: loop {
        match pc {
            0x82DD7BA0 => {
    //   block [0x82DD7BA0..0x82DD7C20)
	// 82DD7BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD7BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD7BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD7BB0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD7BB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7BB8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD7BBC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD7BC0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD7BC4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD7BC8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7BCC: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD7BD0: 4BFFF431  bl 0x82dd7000
	ctx.lr = 0x82DD7BD4;
	sub_82DD7000(ctx, base);
	// 82DD7BD4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7BD8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD7BDC: 40980044  bge cr6, 0x82dd7c20
	if !ctx.cr[6].lt {
	pc = 0x82DD7C20; continue 'dispatch;
	}
	// 82DD7BE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD7BE4: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD7BE8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD7C20; continue 'dispatch;
            }
            0x82DD7C20 => {
    //   block [0x82DD7C20..0x82DD7C50)
	// 82DD7C20: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD7C24: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD7C28: 419A000C  beq cr6, 0x82dd7c34
	if ctx.cr[6].eq {
	pc = 0x82DD7C34; continue 'dispatch;
	}
	// 82DD7C2C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD7C30: 4BFF3359  bl 0x82dcaf88
	ctx.lr = 0x82DD7C34;
	sub_82DCAF88(ctx, base);
	// 82DD7C34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7C40: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD7C44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD7C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD7C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7C50 size=116
    let mut pc: u32 = 0x82DD7C50;
    'dispatch: loop {
        match pc {
            0x82DD7C50 => {
    //   block [0x82DD7C50..0x82DD7CC4)
	// 82DD7C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7C54: 4BED17B9  bl 0x82ca940c
	ctx.lr = 0x82DD7C58;
	sub_82CA93D0(ctx, base);
	// 82DD7C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7C5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7C60: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD7C64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD7C68: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD7C6C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DD7C70: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DD7C74: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD7C78: 4BF7D5D1  bl 0x82d55248
	ctx.lr = 0x82DD7C7C;
	sub_82D55248(ctx, base);
	// 82DD7C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD7C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD7C84: 396B3AA4  addi r11, r11, 0x3aa4
	ctx.r[11].s64 = ctx.r[11].s64 + 15012;
	// 82DD7C88: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DD7C8C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD7C90: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD7C94: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82DD7C98: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82DD7C9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD7CA0: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD7CA4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD7CA8: B11F000C  sth r8, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD7CAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7CB0: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD7CB4: 4804486D  bl 0x82e1c520
	ctx.lr = 0x82DD7CB8;
	sub_82E1C520(ctx, base);
	// 82DD7CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD7CBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7CC0: 4BED179C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7CC8 size=104
    let mut pc: u32 = 0x82DD7CC8;
    'dispatch: loop {
        match pc {
            0x82DD7CC8 => {
    //   block [0x82DD7CC8..0x82DD7D04)
	// 82DD7CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD7CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD7CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD7CDC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DD7CE0: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD7CE4: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82DD7CE8: 419A001C  beq cr6, 0x82dd7d04
	if ctx.cr[6].eq {
	pc = 0x82DD7D04; continue 'dispatch;
	}
	// 82DD7CEC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7CF0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD7CF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7CF8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD7CFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD7D00: 4E800421  bctrl
	ctx.lr = 0x82DD7D04;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD7D04 => {
    //   block [0x82DD7D04..0x82DD7D30)
	// 82DD7D04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7D08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD7D0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD7D10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD7D18: 4E800421  bctrl
	ctx.lr = 0x82DD7D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD7D1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD7D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD7D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7D30 size=128
    let mut pc: u32 = 0x82DD7D30;
    'dispatch: loop {
        match pc {
            0x82DD7D30 => {
    //   block [0x82DD7D30..0x82DD7DB0)
	// 82DD7D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7D34: 4BED16D9  bl 0x82ca940c
	ctx.lr = 0x82DD7D38;
	sub_82CA93D0(ctx, base);
	// 82DD7D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7D3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7D40: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD7D44: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD7D48: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DD7D4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD7D50: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DD7D54: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD7D58: 4BF7D4F1  bl 0x82d55248
	ctx.lr = 0x82DD7D5C;
	sub_82D55248(ctx, base);
	// 82DD7D5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD7D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD7D64: 396B3AA4  addi r11, r11, 0x3aa4
	ctx.r[11].s64 = ctx.r[11].s64 + 15012;
	// 82DD7D68: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DD7D6C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD7D70: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD7D74: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82DD7D78: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82DD7D7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD7D80: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD7D84: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD7D88: B11F000C  sth r8, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD7D8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7D90: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD7D94: 4804478D  bl 0x82e1c520
	ctx.lr = 0x82DD7D98;
	sub_82E1C520(ctx, base);
	// 82DD7D98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD7D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD7DA0: 396B3AE0  addi r11, r11, 0x3ae0
	ctx.r[11].s64 = ctx.r[11].s64 + 15072;
	// 82DD7DA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD7DA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7DAC: 4BED16B0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7DB0 size=304
    let mut pc: u32 = 0x82DD7DB0;
    'dispatch: loop {
        match pc {
            0x82DD7DB0 => {
    //   block [0x82DD7DB0..0x82DD7E00)
	// 82DD7DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7DB4: 4BED1649  bl 0x82ca93fc
	ctx.lr = 0x82DD7DB8;
	sub_82CA93D0(ctx, base);
	// 82DD7DB8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7DBC: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7DC0: 3B600008  li r27, 8
	ctx.r[27].s64 = 8;
	// 82DD7DC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DD7DC8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DD7DCC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD7DD0: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DD7DD4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD7DD8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD7DDC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD7DE0: 40980020  bge cr6, 0x82dd7e00
	if !ctx.cr[6].lt {
	pc = 0x82DD7E00; continue 'dispatch;
	}
	// 82DD7DE4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD7DE8: 39293B18  addi r9, r9, 0x3b18
	ctx.r[9].s64 = ctx.r[9].s64 + 15128;
	// 82DD7DEC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD7DF0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD7DF4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD7DF8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD7DFC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD7E00; continue 'dispatch;
            }
            0x82DD7E00 => {
    //   block [0x82DD7E00..0x82DD7EE0)
	// 82DD7E00: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7E04: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DD7E08: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7E0C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DD7E10: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DD7E14: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DD7E1C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7E20: 38A30010  addi r5, r3, 0x10
	ctx.r[5].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD7EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7EE0 size=320
    let mut pc: u32 = 0x82DD7EE0;
    'dispatch: loop {
        match pc {
            0x82DD7EE0 => {
    //   block [0x82DD7EE0..0x82DD7F30)
	// 82DD7EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7EE4: 4BED1515  bl 0x82ca93f8
	ctx.lr = 0x82DD7EE8;
	sub_82CA93D0(ctx, base);
	// 82DD7EE8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD7EEC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7EF0: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DD7EF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DD7EF8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD7EFC: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DD7F00: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DD7F04: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD7F08: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD7F0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD7F10: 40980020  bge cr6, 0x82dd7f30
	if !ctx.cr[6].lt {
	pc = 0x82DD7F30; continue 'dispatch;
	}
	// 82DD7F14: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD7F18: 39293B18  addi r9, r9, 0x3b18
	ctx.r[9].s64 = ctx.r[9].s64 + 15128;
	// 82DD7F1C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD7F20: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD7F24: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD7F28: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD7F2C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD7F30; continue 'dispatch;
            }
            0x82DD7F30 => {
    //   block [0x82DD7F30..0x82DD8020)
	// 82DD7F30: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7F34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD7F38: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD7F3C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82DD7F40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DD7F44: 480445DD  bl 0x82e1c520
	ctx.lr = 0x82DD7F48;
	sub_82E1C520(ctx, base);
	// 82DD7F48: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7F4C: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD7F50: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DD7F54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DD7F58: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82DD7F5C: 394A4C30  addi r10, r10, 0x4c30
	ctx.r[10].s64 = ctx.r[10].s64 + 19504;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8020 size=376
    let mut pc: u32 = 0x82DD8020;
    'dispatch: loop {
        match pc {
            0x82DD8020 => {
    //   block [0x82DD8020..0x82DD8198)
	// 82DD8020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8024: 4BED13C5  bl 0x82ca93e8
	ctx.lr = 0x82DD8028;
	sub_82CA93D0(ctx, base);
	// 82DD8028: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD802C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD8030: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DD8034: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD8038: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD803C: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8040: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DD8044: 83030000  lwz r24, 0(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8048: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82DD804C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD8050: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD8054: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82DD8058: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD805C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD8060: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD8064: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD8068: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD806C: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DD8070: EAC60000  ld r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD8074: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DD8078: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD807C: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82DD8080: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD8084: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82DD8088: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD808C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD8090: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD8094: 3B8100D0  addi r28, r1, 0xd0
	ctx.r[28].s64 = ctx.r[1].s64 + 208;
	// 82DD8098: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD809C: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
	// 82DD80A0: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD80A4: EA830000  ld r20, 0(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8198 size=492
    let mut pc: u32 = 0x82DD8198;
    'dispatch: loop {
        match pc {
            0x82DD8198 => {
    //   block [0x82DD8198..0x82DD81E4)
	// 82DD8198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD819C: 4BED1245  bl 0x82ca93e0
	ctx.lr = 0x82DD81A0;
	sub_82CA93D0(ctx, base);
	// 82DD81A0: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD81A4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD81A8: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DD81AC: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD81B0: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82DD81B4: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DD81B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD81BC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD81C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD81C4: 40980020  bge cr6, 0x82dd81e4
	if !ctx.cr[6].lt {
	pc = 0x82DD81E4; continue 'dispatch;
	}
	// 82DD81C8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD81CC: 39293B18  addi r9, r9, 0x3b18
	ctx.r[9].s64 = ctx.r[9].s64 + 15128;
	// 82DD81D0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD81D4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD81D8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD81DC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD81E0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD81E4; continue 'dispatch;
            }
            0x82DD81E4 => {
    //   block [0x82DD81E4..0x82DD8384)
	// 82DD81E4: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD81E8: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 82DD81EC: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD81F0: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD81F4: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82DD81F8: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DD81FC: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82DD8200: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82DD8204: 83850000  lwz r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8208: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD820C: 83440000  lwz r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8210: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD8214: EAAB0000  ld r21, 0(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD8218: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD821C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD8220: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DD8224: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82DD8228: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD822C: 3BA10110  addi r29, r1, 0x110
	ctx.r[29].s64 = ctx.r[1].s64 + 272;
	// 82DD8230: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD8234: 3BDC0040  addi r30, r28, 0x40
	ctx.r[30].s64 = ctx.r[28].s64 + 64;
	// 82DD8238: FAAA0000  std r21, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD823C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD8240: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82DD8244: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD8248: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD824C: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD8250: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8388 size=516
    let mut pc: u32 = 0x82DD8388;
    'dispatch: loop {
        match pc {
            0x82DD8388 => {
    //   block [0x82DD8388..0x82DD83DC)
	// 82DD8388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD838C: 4BED1059  bl 0x82ca93e4
	ctx.lr = 0x82DD8390;
	sub_82CA93D0(ctx, base);
	// 82DD8390: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8394: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8398: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DD839C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD83A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DD83A4: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82DD83A8: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD83AC: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DD83B0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD83B4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD83B8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD83BC: 40980020  bge cr6, 0x82dd83dc
	if !ctx.cr[6].lt {
	pc = 0x82DD83DC; continue 'dispatch;
	}
	// 82DD83C0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD83C4: 39293B18  addi r9, r9, 0x3b18
	ctx.r[9].s64 = ctx.r[9].s64 + 15128;
	// 82DD83C8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD83CC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD83D0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD83D4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD83D8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD83DC; continue 'dispatch;
            }
            0x82DD83DC => {
    //   block [0x82DD83DC..0x82DD858C)
	// 82DD83DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD83E0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD83E4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD83E8: 48044139  bl 0x82e1c520
	ctx.lr = 0x82DD83EC;
	sub_82E1C520(ctx, base);
	// 82DD83EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD83F0: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD83F4: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
	// 82DD83F8: 93C100C0  stw r30, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82DD83FC: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD8400: 93E100C4  stw r31, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 82DD8404: 3B860030  addi r28, r6, 0x30
	ctx.r[28].s64 = ctx.r[6].s64 + 48;
	// 82DD8408: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD840C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8410: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD8414: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD8418: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD841C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82DD8420: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD8424: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 82DD8428: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD842C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DD8430: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD8434: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DD8438: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD843C: 3BFD0040  addi r31, r29, 0x40
	ctx.r[31].s64 = ctx.r[29].s64 + 64;
	// 82DD8440: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD8444: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 82DD8448: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD844C: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD8450: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD8454: EA640000  ld r19, 0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD8590 size=64
    let mut pc: u32 = 0x82DD8590;
    'dispatch: loop {
        match pc {
            0x82DD8590 => {
    //   block [0x82DD8590..0x82DD85D0)
	// 82DD8590: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD8594: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD8598: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD859C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD85A0: 38EB4B00  addi r7, r11, 0x4b00
	ctx.r[7].s64 = ctx.r[11].s64 + 19200;
	// 82DD85A4: 39087C50  addi r8, r8, 0x7c50
	ctx.r[8].s64 = ctx.r[8].s64 + 31824;
	// 82DD85A8: 39297EE0  addi r9, r9, 0x7ee0
	ctx.r[9].s64 = ctx.r[9].s64 + 32480;
	// 82DD85AC: 394A8388  addi r10, r10, -0x7c78
	ctx.r[10].s64 = ctx.r[10].s64 + -31864;
	// 82DD85B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD85B4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DD85B8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD85BC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD85C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DD85C4: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DD85C8: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82DD85CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD85D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD85D0 size=68
    let mut pc: u32 = 0x82DD85D0;
    'dispatch: loop {
        match pc {
            0x82DD85D0 => {
    //   block [0x82DD85D0..0x82DD8614)
	// 82DD85D0: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD85D4: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD85D8: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD85DC: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD85E0: 39087D30  addi r8, r8, 0x7d30
	ctx.r[8].s64 = ctx.r[8].s64 + 32048;
	// 82DD85E4: 39298808  addi r9, r9, -0x77f8
	ctx.r[9].s64 = ctx.r[9].s64 + -30712;
	// 82DD85E8: 394A8B18  addi r10, r10, -0x74e8
	ctx.r[10].s64 = ctx.r[10].s64 + -29928;
	// 82DD85EC: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD85F0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DD85F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DD85F8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD85FC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD8600: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DD8604: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DD8608: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 82DD860C: 98C30011  stb r6, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[6].u8 ) };
	// 82DD8610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8618 size=204
    let mut pc: u32 = 0x82DD8618;
    'dispatch: loop {
        match pc {
            0x82DD8618 => {
    //   block [0x82DD8618..0x82DD86E4)
	// 82DD8618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD861C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8620: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD8624: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD8628: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD862C: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD8630: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD8634: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD8638: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD863C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD8640: 39087D30  addi r8, r8, 0x7d30
	ctx.r[8].s64 = ctx.r[8].s64 + 32048;
	// 82DD8644: 39298808  addi r9, r9, -0x77f8
	ctx.r[9].s64 = ctx.r[9].s64 + -30712;
	// 82DD8648: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD864C: 394A8B18  addi r10, r10, -0x74e8
	ctx.r[10].s64 = ctx.r[10].s64 + -29928;
	// 82DD8650: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD8654: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD8658: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DD865C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD8660: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82DD8664: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD8668: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD866C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD8670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD8674: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD8678: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD867C: 4BFE8AED  bl 0x82dc1168
	ctx.lr = 0x82DD8680;
	sub_82DC1168(ctx, base);
	// 82DD8680: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD8684: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD8688: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD868C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD8690: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD8694: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD8698: 39087C50  addi r8, r8, 0x7c50
	ctx.r[8].s64 = ctx.r[8].s64 + 31824;
	// 82DD869C: 39297EE0  addi r9, r9, 0x7ee0
	ctx.r[9].s64 = ctx.r[9].s64 + 32480;
	// 82DD86A0: 394A8388  addi r10, r10, -0x7c78
	ctx.r[10].s64 = ctx.r[10].s64 + -31864;
	// 82DD86A4: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD86A8: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82DD86AC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DD86B0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD86B4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD86B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD86BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD86C0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD86C4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD86C8: 4BFE8AA1  bl 0x82dc1168
	ctx.lr = 0x82DD86CC;
	sub_82DC1168(ctx, base);
	// 82DD86CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD86D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD86D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD86D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD86DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD86E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD86E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD86E8 size=204
    let mut pc: u32 = 0x82DD86E8;
    'dispatch: loop {
        match pc {
            0x82DD86E8 => {
    //   block [0x82DD86E8..0x82DD87B4)
	// 82DD86E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD86EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD86F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD86F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD86F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD86FC: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD8700: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD8704: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD8708: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD870C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD8710: 39087D30  addi r8, r8, 0x7d30
	ctx.r[8].s64 = ctx.r[8].s64 + 32048;
	// 82DD8714: 39298808  addi r9, r9, -0x77f8
	ctx.r[9].s64 = ctx.r[9].s64 + -30712;
	// 82DD8718: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD871C: 394A8B18  addi r10, r10, -0x74e8
	ctx.r[10].s64 = ctx.r[10].s64 + -29928;
	// 82DD8720: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD8724: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD8728: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DD872C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD8730: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82DD8734: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD8738: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD873C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD8740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD8744: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD8748: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD874C: 4BFE8B35  bl 0x82dc1280
	ctx.lr = 0x82DD8750;
	sub_82DC1280(ctx, base);
	// 82DD8750: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DD8754: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD8758: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DD875C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD8760: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD8764: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD8768: 39087C50  addi r8, r8, 0x7c50
	ctx.r[8].s64 = ctx.r[8].s64 + 31824;
	// 82DD876C: 39297EE0  addi r9, r9, 0x7ee0
	ctx.r[9].s64 = ctx.r[9].s64 + 32480;
	// 82DD8770: 394A8388  addi r10, r10, -0x7c78
	ctx.r[10].s64 = ctx.r[10].s64 + -31864;
	// 82DD8774: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD8778: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82DD877C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DD8780: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD8784: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD8788: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD878C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD8790: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD8794: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD8798: 4BFE8AE9  bl 0x82dc1280
	ctx.lr = 0x82DD879C;
	sub_82DC1280(ctx, base);
	// 82DD879C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD87A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD87A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD87A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD87AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD87B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD87B8 size=76
    let mut pc: u32 = 0x82DD87B8;
    'dispatch: loop {
        match pc {
            0x82DD87B8 => {
    //   block [0x82DD87B8..0x82DD8804)
	// 82DD87B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD87BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD87C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD87C4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD87C8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD87CC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD87D0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD87D4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD87D8: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD87DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD87E0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD87E4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD87E8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD87EC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD87F0: 4BFFF5C1  bl 0x82dd7db0
	ctx.lr = 0x82DD87F4;
	sub_82DD7DB0(ctx, base);
	// 82DD87F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD87F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD87FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8808 size=76
    let mut pc: u32 = 0x82DD8808;
    'dispatch: loop {
        match pc {
            0x82DD8808 => {
    //   block [0x82DD8808..0x82DD8854)
	// 82DD8808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD880C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8814: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD8818: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD881C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD8820: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD8824: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD8828: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD882C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD8830: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD8834: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD8838: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD883C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD8840: 4BFFF6A1  bl 0x82dd7ee0
	ctx.lr = 0x82DD8844;
	sub_82DD7EE0(ctx, base);
	// 82DD8844: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD884C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8858 size=620
    let mut pc: u32 = 0x82DD8858;
    'dispatch: loop {
        match pc {
            0x82DD8858 => {
    //   block [0x82DD8858..0x82DD88B0)
	// 82DD8858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD885C: 4BED0B79  bl 0x82ca93d4
	ctx.lr = 0x82DD8860;
	sub_82CA93D0(ctx, base);
	// 82DD8860: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8864: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8868: 3A800008  li r20, 8
	ctx.r[20].s64 = 8;
	// 82DD886C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DD8870: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD8874: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DD8878: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82DD887C: 7D74982E  lwzx r11, r20, r19
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82DD8880: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD8884: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD8888: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD888C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD8890: 40980020  bge cr6, 0x82dd88b0
	if !ctx.cr[6].lt {
	pc = 0x82DD88B0; continue 'dispatch;
	}
	// 82DD8894: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD8898: 39293B18  addi r9, r9, 0x3b18
	ctx.r[9].s64 = ctx.r[9].s64 + 15128;
	// 82DD889C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD88A0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD88A4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD88A8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD88AC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD88B0; continue 'dispatch;
            }
            0x82DD88B0 => {
    //   block [0x82DD88B0..0x82DD8AC4)
	// 82DD88B0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD88B4: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DD88B8: 80DC0008  lwz r6, 8(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD88BC: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82DD88C0: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD88C4: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD88C8: 3B060030  addi r24, r6, 0x30
	ctx.r[24].s64 = ctx.r[6].s64 + 48;
	// 82DD88CC: 83590000  lwz r26, 0(r25)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD88D0: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD88D4: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD88D8: EA4B0000  ld r18, 0(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD88DC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD88E0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD88E4: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD88E8: EA050000  ld r16, 0(r5)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD88EC: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82DD88F0: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD88F4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DD88F8: EA260000  ld r17, 0(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD88FC: 3BC100E0  addi r30, r1, 0xe0
	ctx.r[30].s64 = ctx.r[1].s64 + 224;
	// 82DD8900: FA4A0000  std r18, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82DD8904: 3BFD0040  addi r31, r29, 0x40
	ctx.r[31].s64 = ctx.r[29].s64 + 64;
	// 82DD8908: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD890C: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 82DD8910: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD8914: FA080000  std r16, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[16].u64 ) };
	// 82DD8918: FA290000  std r17, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82DD891C: E9E40000  ld r15, 0(r4)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD8AC8 size=80
    let mut pc: u32 = 0x82DD8AC8;
    'dispatch: loop {
        match pc {
            0x82DD8AC8 => {
    //   block [0x82DD8AC8..0x82DD8B18)
	// 82DD8AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8AD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8AD4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD8AD8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD8ADC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD8AE0: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD8AE4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD8AE8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD8AEC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD8AF0: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD8AF4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD8AF8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD8AFC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD8B00: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD8B04: 4BFFF695  bl 0x82dd8198
	ctx.lr = 0x82DD8B08;
	sub_82DD8198(ctx, base);
	// 82DD8B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD8B18 size=80
    let mut pc: u32 = 0x82DD8B18;
    'dispatch: loop {
        match pc {
            0x82DD8B18 => {
    //   block [0x82DD8B18..0x82DD8B68)
	// 82DD8B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8B20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8B24: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD8B28: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD8B2C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD8B30: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD8B34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD8B38: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD8B3C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD8B40: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD8B44: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD8B48: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD8B4C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD8B50: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD8B54: 4BFFF835  bl 0x82dd8388
	ctx.lr = 0x82DD8B58;
	sub_82DD8388(ctx, base);
	// 82DD8B58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD8B68 size=176
    let mut pc: u32 = 0x82DD8B68;
    'dispatch: loop {
        match pc {
            0x82DD8B68 => {
    //   block [0x82DD8B68..0x82DD8BE8)
	// 82DD8B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD8B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD8B78: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD8B7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8B80: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD8B84: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD8B88: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD8B8C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD8B90: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8B94: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD8B98: 4BFFFCC1  bl 0x82dd8858
	ctx.lr = 0x82DD8B9C;
	sub_82DD8858(ctx, base);
	// 82DD8B9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8BA0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD8BA4: 40980044  bge cr6, 0x82dd8be8
	if !ctx.cr[6].lt {
	pc = 0x82DD8BE8; continue 'dispatch;
	}
	// 82DD8BA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD8BAC: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD8BB0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD8BE8; continue 'dispatch;
            }
            0x82DD8BE8 => {
    //   block [0x82DD8BE8..0x82DD8C18)
	// 82DD8BE8: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD8BEC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD8BF0: 419A000C  beq cr6, 0x82dd8bfc
	if ctx.cr[6].eq {
	pc = 0x82DD8BFC; continue 'dispatch;
	}
	// 82DD8BF4: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD8BF8: 4BFF2391  bl 0x82dcaf88
	ctx.lr = 0x82DD8BFC;
	sub_82DCAF88(ctx, base);
	// 82DD8BFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8C08: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD8C0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD8C10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD8C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8C18 size=104
    let mut pc: u32 = 0x82DD8C18;
    'dispatch: loop {
        match pc {
            0x82DD8C18 => {
    //   block [0x82DD8C18..0x82DD8C80)
	// 82DD8C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD8C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8C28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8C2C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD8C30: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD8C34: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DD8C38: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD8C3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD8C40: 4BF7C609  bl 0x82d55248
	ctx.lr = 0x82DD8C44;
	sub_82D55248(ctx, base);
	// 82DD8C44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD8C48: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DD8C4C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DD8C50: 396B3B44  addi r11, r11, 0x3b44
	ctx.r[11].s64 = ctx.r[11].s64 + 15172;
	// 82DD8C54: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD8C58: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD8C5C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD8C60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD8C64: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD8C68: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD8C6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD8C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8C78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD8C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8C80 size=104
    let mut pc: u32 = 0x82DD8C80;
    'dispatch: loop {
        match pc {
            0x82DD8C80 => {
    //   block [0x82DD8C80..0x82DD8CBC)
	// 82DD8C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8C88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD8C8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8C90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD8C94: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DD8C98: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD8C9C: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82DD8CA0: 419A001C  beq cr6, 0x82dd8cbc
	if ctx.cr[6].eq {
	pc = 0x82DD8CBC; continue 'dispatch;
	}
	// 82DD8CA4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD8CA8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD8CAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8CB0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD8CB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD8CB8: 4E800421  bctrl
	ctx.lr = 0x82DD8CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DD8CBC => {
    //   block [0x82DD8CBC..0x82DD8CE8)
	// 82DD8CBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8CC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD8CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD8CC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8CCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD8CD0: 4E800421  bctrl
	ctx.lr = 0x82DD8CD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD8CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD8CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8CE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD8CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8CE8 size=104
    let mut pc: u32 = 0x82DD8CE8;
    'dispatch: loop {
        match pc {
            0x82DD8CE8 => {
    //   block [0x82DD8CE8..0x82DD8D50)
	// 82DD8CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD8CF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD8CF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8CF8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8CFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD8D00: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD8D04: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DD8D08: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD8D0C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD8D10: 4BF7C539  bl 0x82d55248
	ctx.lr = 0x82DD8D14;
	sub_82D55248(ctx, base);
	// 82DD8D14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD8D18: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DD8D1C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DD8D20: 396B3B80  addi r11, r11, 0x3b80
	ctx.r[11].s64 = ctx.r[11].s64 + 15232;
	// 82DD8D24: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD8D28: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD8D2C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD8D30: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD8D34: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD8D38: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD8D3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD8D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8D48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD8D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8D50 size=456
    let mut pc: u32 = 0x82DD8D50;
    'dispatch: loop {
        match pc {
            0x82DD8D50 => {
    //   block [0x82DD8D50..0x82DD8DA4)
	// 82DD8D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8D54: 4BED0691  bl 0x82ca93e4
	ctx.lr = 0x82DD8D58;
	sub_82CA93D0(ctx, base);
	// 82DD8D58: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8D5C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8D60: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD8D64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DD8D68: 7F0B5214  add r24, r11, r10
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD8D6C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DD8D70: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DD8D74: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8D78: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD8D7C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD8D80: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD8D84: 40980020  bge cr6, 0x82dd8da4
	if !ctx.cr[6].lt {
	pc = 0x82DD8DA4; continue 'dispatch;
	}
	// 82DD8D88: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD8D8C: 39293BB8  addi r9, r9, 0x3bb8
	ctx.r[9].s64 = ctx.r[9].s64 + 15288;
	// 82DD8D90: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD8D94: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD8D98: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD8D9C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD8DA0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD8DA4; continue 'dispatch;
            }
            0x82DD8DA4 => {
    //   block [0x82DD8DA4..0x82DD8F18)
	// 82DD8DA4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD8DA8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD8DAC: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD8DB0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD8DB4: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD8DB8: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8DBC: 3B460030  addi r26, r6, 0x30
	ctx.r[26].s64 = ctx.r[6].s64 + 48;
	// 82DD8DC0: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8DC4: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD8DC8: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD8DCC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD8DD0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD8DD4: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD8DD8: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD8DDC: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82DD8DE0: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD8DE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD8DE8: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD8DEC: 3BC100C0  addi r30, r1, 0xc0
	ctx.r[30].s64 = ctx.r[1].s64 + 192;
	// 82DD8DF0: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD8DF4: 3BFB0030  addi r31, r27, 0x30
	ctx.r[31].s64 = ctx.r[27].s64 + 48;
	// 82DD8DF8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD8DFC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD8E00: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD8E04: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82DD8E08: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD8E0C: EA640000  ld r19, 0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD8F18 size=20
    let mut pc: u32 = 0x82DD8F18;
    'dispatch: loop {
        match pc {
            0x82DD8F18 => {
    //   block [0x82DD8F18..0x82DD8F2C)
	// 82DD8F18: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD8F1C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD8F20: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD8F24: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DD8F28: 4BFFFE28  b 0x82dd8d50
	sub_82DD8D50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD8F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8F30 size=632
    let mut pc: u32 = 0x82DD8F30;
    'dispatch: loop {
        match pc {
            0x82DD8F30 => {
    //   block [0x82DD8F30..0x82DD91A8)
	// 82DD8F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8F34: 4BED04B9  bl 0x82ca93ec
	ctx.lr = 0x82DD8F38;
	sub_82CA93D0(ctx, base);
	// 82DD8F38: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD8F3C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD8F40: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DD8F44: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD8F48: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DD8F4C: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8F50: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD8F54: 3B260030  addi r25, r6, 0x30
	ctx.r[25].s64 = ctx.r[6].s64 + 48;
	// 82DD8F58: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD8F5C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD8F60: EB0B0000  ld r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD8F64: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD8F68: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD8F6C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD8F70: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD8F74: EAC50000  ld r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD8F78: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD8F7C: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 82DD8F80: EAE60000  ld r23, 0(r6)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD8F84: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82DD8F88: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD8F8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD8F90: FB0A0000  std r24, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82DD8F94: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82DD8F98: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD8F9C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD8FA0: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DD8FA4: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82DD8FA8: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DD8FAC: FAE90000  std r23, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD8FB0: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD91A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD91A8 size=768
    let mut pc: u32 = 0x82DD91A8;
    'dispatch: loop {
        match pc {
            0x82DD91A8 => {
    //   block [0x82DD91A8..0x82DD91F8)
	// 82DD91A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD91AC: 4BED023D  bl 0x82ca93e8
	ctx.lr = 0x82DD91B0;
	sub_82CA93D0(ctx, base);
	// 82DD91B0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD91B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD91B8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD91BC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DD91C0: 7F0B5214  add r24, r11, r10
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD91C4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DD91C8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD91CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD91D0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD91D4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD91D8: 40980020  bge cr6, 0x82dd91f8
	if !ctx.cr[6].lt {
	pc = 0x82DD91F8; continue 'dispatch;
	}
	// 82DD91DC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD91E0: 39293BB8  addi r9, r9, 0x3bb8
	ctx.r[9].s64 = ctx.r[9].s64 + 15288;
	// 82DD91E4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD91E8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD91EC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD91F0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD91F4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD91F8; continue 'dispatch;
            }
            0x82DD91F8 => {
    //   block [0x82DD91F8..0x82DD94A8)
	// 82DD91F8: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD91FC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD9200: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9204: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 82DD9208: 908100A0  stw r4, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[4].u32 ) };
	// 82DD920C: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82DD9210: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DD9214: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82DD9218: 83E50000  lwz r31, 0(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD921C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD9220: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9224: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD9228: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD922C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD9230: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD9234: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DD9238: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD923C: EAC60000  ld r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD9240: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82DD9244: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD9248: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82DD924C: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD9250: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD9254: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD9258: EAA50000  ld r21, 0(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD925C: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD9260: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DD9264: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD94A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD94A8 size=768
    let mut pc: u32 = 0x82DD94A8;
    'dispatch: loop {
        match pc {
            0x82DD94A8 => {
    //   block [0x82DD94A8..0x82DD94F8)
	// 82DD94A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD94AC: 4BECFF3D  bl 0x82ca93e8
	ctx.lr = 0x82DD94B0;
	sub_82CA93D0(ctx, base);
	// 82DD94B0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD94B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD94B8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD94BC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DD94C0: 7F0B5214  add r24, r11, r10
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD94C4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DD94C8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD94CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD94D0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD94D4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD94D8: 40980020  bge cr6, 0x82dd94f8
	if !ctx.cr[6].lt {
	pc = 0x82DD94F8; continue 'dispatch;
	}
	// 82DD94DC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD94E0: 39293BB8  addi r9, r9, 0x3bb8
	ctx.r[9].s64 = ctx.r[9].s64 + 15288;
	// 82DD94E4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD94E8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD94EC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD94F0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD94F4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD94F8; continue 'dispatch;
            }
            0x82DD94F8 => {
    //   block [0x82DD94F8..0x82DD97A8)
	// 82DD94F8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD94FC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD9500: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9504: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 82DD9508: 906100A0  stw r3, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[3].u32 ) };
	// 82DD950C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD9510: 908100A4  stw r4, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 82DD9514: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82DD9518: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD951C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD9520: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD9524: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD9528: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD952C: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82DD9530: EAA50000  ld r21, 0(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD9534: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DD9538: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD953C: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82DD9540: EAC60000  ld r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD9544: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82DD9548: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82DD954C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD9550: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD9554: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD9558: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82DD955C: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82DD9560: EA840000  ld r20, 0(r4)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD97A8 size=204
    let mut pc: u32 = 0x82DD97A8;
    'dispatch: loop {
        match pc {
            0x82DD97A8 => {
    //   block [0x82DD97A8..0x82DD9874)
	// 82DD97A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD97AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD97B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD97B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD97B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD97BC: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DD97C0: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DD97C4: 396B57E8  addi r11, r11, 0x57e8
	ctx.r[11].s64 = ctx.r[11].s64 + 22504;
	// 82DD97C8: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD97CC: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD97D0: 39088CE8  addi r8, r8, -0x7318
	ctx.r[8].s64 = ctx.r[8].s64 + -29464;
	// 82DD97D4: 392998C0  addi r9, r9, -0x6740
	ctx.r[9].s64 = ctx.r[9].s64 + -26432;
	// 82DD97D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DD97DC: 394A9CD0  addi r10, r10, -0x6330
	ctx.r[10].s64 = ctx.r[10].s64 + -25392;
	// 82DD97E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DD97E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DD97E8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DD97EC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DD97F0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82DD97F4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DD97F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DD97FC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD9800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD9804: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DD9808: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DD980C: 4BFE795D  bl 0x82dc1168
	ctx.lr = 0x82DD9810;
	sub_82DC1168(ctx, base);
	// 82DD9810: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DD9814: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82DD9818: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DD981C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DD9820: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DD9824: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DD9828: 39088C18  addi r8, r8, -0x73e8
	ctx.r[8].s64 = ctx.r[8].s64 + -29672;
	// 82DD982C: 39298D50  addi r9, r9, -0x72b0
	ctx.r[9].s64 = ctx.r[9].s64 + -29360;
	// 82DD9830: 394A94A8  addi r10, r10, -0x6b58
	ctx.r[10].s64 = ctx.r[10].s64 + -27480;
	// 82DD9834: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DD9838: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82DD983C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DD9840: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DD9844: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD9848: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DD984C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD9850: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DD9854: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DD9858: 4BFE7911  bl 0x82dc1168
	ctx.lr = 0x82DD985C;
	sub_82DC1168(ctx, base);
	// 82DD985C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DD9860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9868: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD986C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD9870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD9878 size=68
    let mut pc: u32 = 0x82DD9878;
    'dispatch: loop {
        match pc {
            0x82DD9878 => {
    //   block [0x82DD9878..0x82DD98BC)
	// 82DD9878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD987C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9884: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD9888: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DD988C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DD9890: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DD9894: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DD9898: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD989C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD98A0: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DD98A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD98A8: 4BFFF4A9  bl 0x82dd8d50
	ctx.lr = 0x82DD98AC;
	sub_82DD8D50(ctx, base);
	// 82DD98AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD98B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD98B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD98B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD98C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD98C0 size=76
    let mut pc: u32 = 0x82DD98C0;
    'dispatch: loop {
        match pc {
            0x82DD98C0 => {
    //   block [0x82DD98C0..0x82DD990C)
	// 82DD98C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD98C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD98C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD98CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD98D0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD98D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD98D8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD98DC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD98E0: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DD98E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD98E8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD98EC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD98F0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD98F4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DD98F8: 4BFFF459  bl 0x82dd8d50
	ctx.lr = 0x82DD98FC;
	sub_82DD8D50(ctx, base);
	// 82DD98FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD9900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD9910 size=876
    let mut pc: u32 = 0x82DD9910;
    'dispatch: loop {
        match pc {
            0x82DD9910 => {
    //   block [0x82DD9910..0x82DD996C)
	// 82DD9910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9914: 4BECFAC5  bl 0x82ca93d8
	ctx.lr = 0x82DD9918;
	sub_82CA93D0(ctx, base);
	// 82DD9918: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD991C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9920: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD9924: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DD9928: 7E8B5214  add r20, r11, r10
	ctx.r[20].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD992C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DD9930: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DD9934: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82DD9938: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DD993C: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9940: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD9944: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD9948: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD994C: 40980020  bge cr6, 0x82dd996c
	if !ctx.cr[6].lt {
	pc = 0x82DD996C; continue 'dispatch;
	}
	// 82DD9950: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD9954: 39293BB8  addi r9, r9, 0x3bb8
	ctx.r[9].s64 = ctx.r[9].s64 + 15288;
	// 82DD9958: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD995C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD9960: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD9964: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD9968: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD996C; continue 'dispatch;
            }
            0x82DD996C => {
    //   block [0x82DD996C..0x82DD9C7C)
	// 82DD996C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9970: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DD9974: 80DA0008  lwz r6, 8(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9978: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DD997C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DD9980: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9984: 3B060030  addi r24, r6, 0x30
	ctx.r[24].s64 = ctx.r[6].s64 + 48;
	// 82DD9988: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD998C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DD9990: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9994: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DD9998: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DD999C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DD99A0: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82DD99A4: EA250000  ld r17, 0(r5)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DD99A8: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82DD99AC: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DD99B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD99B4: EA460000  ld r18, 0(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DD99B8: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82DD99BC: FA6A0000  std r19, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82DD99C0: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82DD99C4: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DD99C8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DD99CC: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DD99D0: FA280000  std r17, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82DD99D4: FA490000  std r18, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82DD99D8: EA040000  ld r16, 0(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD9C80 size=80
    let mut pc: u32 = 0x82DD9C80;
    'dispatch: loop {
        match pc {
            0x82DD9C80 => {
    //   block [0x82DD9C80..0x82DD9CD0)
	// 82DD9C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9C8C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD9C90: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD9C94: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DD9C98: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD9C9C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD9CA0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD9CA4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD9CA8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD9CAC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD9CB0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD9CB4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD9CB8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD9CBC: 4BFFF4ED  bl 0x82dd91a8
	ctx.lr = 0x82DD9CC0;
	sub_82DD91A8(ctx, base);
	// 82DD9CC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD9CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD9CD0 size=80
    let mut pc: u32 = 0x82DD9CD0;
    'dispatch: loop {
        match pc {
            0x82DD9CD0 => {
    //   block [0x82DD9CD0..0x82DD9D20)
	// 82DD9CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9CDC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD9CE0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DD9CE4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DD9CE8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DD9CEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DD9CF0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD9CF4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD9CF8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD9CFC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD9D00: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD9D04: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DD9D08: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DD9D0C: 4BFFF79D  bl 0x82dd94a8
	ctx.lr = 0x82DD9D10;
	sub_82DD94A8(ctx, base);
	// 82DD9D10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD9D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD9D20 size=176
    let mut pc: u32 = 0x82DD9D20;
    'dispatch: loop {
        match pc {
            0x82DD9D20 => {
    //   block [0x82DD9D20..0x82DD9DA0)
	// 82DD9D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9D28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD9D2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD9D30: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DD9D34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9D38: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DD9D3C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DD9D40: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD9D44: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DD9D48: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9D4C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DD9D50: 4BFFFBC1  bl 0x82dd9910
	ctx.lr = 0x82DD9D54;
	sub_82DD9910(ctx, base);
	// 82DD9D54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9D58: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DD9D5C: 40980044  bge cr6, 0x82dd9da0
	if !ctx.cr[6].lt {
	pc = 0x82DD9DA0; continue 'dispatch;
	}
	// 82DD9D60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD9D64: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DD9D68: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DD9DA0; continue 'dispatch;
            }
            0x82DD9DA0 => {
    //   block [0x82DD9DA0..0x82DD9DD0)
	// 82DD9DA0: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD9DA4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DD9DA8: 419A000C  beq cr6, 0x82dd9db4
	if ctx.cr[6].eq {
	pc = 0x82DD9DB4; continue 'dispatch;
	}
	// 82DD9DAC: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DD9DB0: 4BFF11D9  bl 0x82dcaf88
	ctx.lr = 0x82DD9DB4;
	sub_82DCAF88(ctx, base);
	// 82DD9DB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD9DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9DC0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DD9DC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DD9DC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD9DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD9DD0 size=104
    let mut pc: u32 = 0x82DD9DD0;
    'dispatch: loop {
        match pc {
            0x82DD9DD0 => {
    //   block [0x82DD9DD0..0x82DD9E38)
	// 82DD9DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9DD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD9DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9DE0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9DE4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD9DE8: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD9DEC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DD9DF0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DD9DF4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD9DF8: 4BF7B451  bl 0x82d55248
	ctx.lr = 0x82DD9DFC;
	sub_82D55248(ctx, base);
	// 82DD9DFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD9E00: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DD9E04: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DD9E08: 396B3BCC  addi r11, r11, 0x3bcc
	ctx.r[11].s64 = ctx.r[11].s64 + 15308;
	// 82DD9E0C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DD9E10: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DD9E14: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DD9E18: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD9E1C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DD9E20: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82DD9E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD9E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD9E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD9E38 size=112
    let mut pc: u32 = 0x82DD9E38;
    'dispatch: loop {
        match pc {
            0x82DD9E38 => {
    //   block [0x82DD9E38..0x82DD9E7C)
	// 82DD9E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD9E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9E48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD9E4C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DD9E50: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD9E54: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82DD9E58: 419A0024  beq cr6, 0x82dd9e7c
	if ctx.cr[6].eq {
	pc = 0x82DD9E7C; continue 'dispatch;
	}
	// 82DD9E5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9E60: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DD9E64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9E68: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DD9E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD9E70: 4E800421  bctrl
	ctx.lr = 0x82DD9E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD9E74: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DD9E78: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
            }
            0x82DD9E7C => {
    //   block [0x82DD9E7C..0x82DD9EA8)
	// 82DD9E7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9E80: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DD9E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD9E88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD9E90: 4E800421  bctrl
	ctx.lr = 0x82DD9E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD9E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DD9E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD9E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD9EA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DD9EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD9EA8 size=248
    let mut pc: u32 = 0x82DD9EA8;
    'dispatch: loop {
        match pc {
            0x82DD9EA8 => {
    //   block [0x82DD9EA8..0x82DD9EFC)
	// 82DD9EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD9EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD9EB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9EB8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9EBC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD9EC0: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DD9EC4: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD9EC8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DD9ECC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9ED0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD9ED4: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD9ED8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DD9EDC: 40980020  bge cr6, 0x82dd9efc
	if !ctx.cr[6].lt {
	pc = 0x82DD9EFC; continue 'dispatch;
	}
	// 82DD9EE0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DD9EE4: 39083C04  addi r8, r8, 0x3c04
	ctx.r[8].s64 = ctx.r[8].s64 + 15364;
	// 82DD9EE8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DD9EEC: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DD9EF0: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DD9EF4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD9EF8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DD9EFC; continue 'dispatch;
            }
            0x82DD9EFC => {
    //   block [0x82DD9EFC..0x82DD9FA0)
	// 82DD9EFC: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9F00: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82DD9F04: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9F08: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DD9FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD9FA0 size=504
    let mut pc: u32 = 0x82DD9FA0;
    'dispatch: loop {
        match pc {
            0x82DD9FA0 => {
    //   block [0x82DD9FA0..0x82DD9FF0)
	// 82DD9FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD9FA4: 4BECF465  bl 0x82ca9408
	ctx.lr = 0x82DD9FA8;
	sub_82CA93D0(ctx, base);
	// 82DD9FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD9FAC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9FB0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DD9FB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD9FB8: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DD9FBC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DD9FC0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD9FC4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD9FC8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD9FCC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD9FD0: 40980020  bge cr6, 0x82dd9ff0
	if !ctx.cr[6].lt {
	pc = 0x82DD9FF0; continue 'dispatch;
	}
	// 82DD9FD4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD9FD8: 39293C04  addi r9, r9, 0x3c04
	ctx.r[9].s64 = ctx.r[9].s64 + 15364;
	// 82DD9FDC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD9FE0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD9FE4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DD9FE8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD9FEC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DD9FF0; continue 'dispatch;
            }
            0x82DD9FF0 => {
    //   block [0x82DD9FF0..0x82DDA198)
	// 82DD9FF0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9FF4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DD9FF8: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DD9FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDA000: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 82DDA004: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA008: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDA198 size=436
    let mut pc: u32 = 0x82DDA198;
    'dispatch: loop {
        match pc {
            0x82DDA198 => {
    //   block [0x82DDA198..0x82DDA1E4)
	// 82DDA198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDA1A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDA1A4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDA1A8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA1AC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DDA1B0: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDA1B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA1B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDA1BC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDA1C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDA1C4: 40980020  bge cr6, 0x82dda1e4
	if !ctx.cr[6].lt {
	pc = 0x82DDA1E4; continue 'dispatch;
	}
	// 82DDA1C8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDA1CC: 39293C04  addi r9, r9, 0x3c04
	ctx.r[9].s64 = ctx.r[9].s64 + 15364;
	// 82DDA1D0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDA1D4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDA1D8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDA1DC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDA1E0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDA1E4; continue 'dispatch;
            }
            0x82DDA1E4 => {
    //   block [0x82DDA1E4..0x82DDA34C)
	// 82DDA1E4: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDA1E8: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82DDA1EC: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDA1F0: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 82DDA1F4: 90810080  stw r4, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u32 ) };
	// 82DDA1F8: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82DDA1FC: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA200: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDA204: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDA350 size=440
    let mut pc: u32 = 0x82DDA350;
    'dispatch: loop {
        match pc {
            0x82DDA350 => {
    //   block [0x82DDA350..0x82DDA3A0)
	// 82DDA350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDA358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDA35C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDA360: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA364: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DDA368: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDA36C: 7FEA4A14  add r31, r10, r9
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDA370: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA374: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDA378: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDA37C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DDA380: 40980020  bge cr6, 0x82dda3a0
	if !ctx.cr[6].lt {
	pc = 0x82DDA3A0; continue 'dispatch;
	}
	// 82DDA384: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDA388: 39083C04  addi r8, r8, 0x3c04
	ctx.r[8].s64 = ctx.r[8].s64 + 15364;
	// 82DDA38C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DDA390: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DDA394: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82DDA398: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DDA39C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DDA3A0; continue 'dispatch;
            }
            0x82DDA3A0 => {
    //   block [0x82DDA3A0..0x82DDA508)
	// 82DDA3A0: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDA3A4: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82DDA3A8: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDA3AC: 392A0030  addi r9, r10, 0x30
	ctx.r[9].s64 = ctx.r[10].s64 + 48;
	// 82DDA3B0: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DDA3B4: 90810084  stw r4, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 82DDA3B8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA3BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDA3C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDA508 size=20
    let mut pc: u32 = 0x82DDA508;
    'dispatch: loop {
        match pc {
            0x82DDA508 => {
    //   block [0x82DDA508..0x82DDA51C)
	// 82DDA508: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDA50C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDA510: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DDA514: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DDA518: 4BFFF990  b 0x82dd9ea8
	sub_82DD9EA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDA520 size=52
    let mut pc: u32 = 0x82DDA520;
    'dispatch: loop {
        match pc {
            0x82DDA520 => {
    //   block [0x82DDA520..0x82DDA554)
	// 82DDA520: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDA524: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDA528: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDA52C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDA530: 39089DD0  addi r8, r8, -0x6230
	ctx.r[8].s64 = ctx.r[8].s64 + -25136;
	// 82DDA534: 39299EA8  addi r9, r9, -0x6158
	ctx.r[9].s64 = ctx.r[9].s64 + -24920;
	// 82DDA538: 394AA350  addi r10, r10, -0x5cb0
	ctx.r[10].s64 = ctx.r[10].s64 + -23728;
	// 82DDA53C: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DDA540: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DDA544: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDA548: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDA54C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DDA550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDA558 size=104
    let mut pc: u32 = 0x82DDA558;
    'dispatch: loop {
        match pc {
            0x82DDA558 => {
    //   block [0x82DDA558..0x82DDA5C0)
	// 82DDA558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDA560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDA564: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDA568: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDA56C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDA570: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDA574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DDA578: 39089DD0  addi r8, r8, -0x6230
	ctx.r[8].s64 = ctx.r[8].s64 + -25136;
	// 82DDA57C: 39299EA8  addi r9, r9, -0x6158
	ctx.r[9].s64 = ctx.r[9].s64 + -24920;
	// 82DDA580: 394AA350  addi r10, r10, -0x5cb0
	ctx.r[10].s64 = ctx.r[10].s64 + -23728;
	// 82DDA584: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DDA588: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DDA58C: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82DDA590: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DDA594: 98E10061  stb r7, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[7].u8 ) };
	// 82DDA598: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDA59C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDA5A0: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDA5A4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDA5A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDA5AC: 4BFE6BBD  bl 0x82dc1168
	ctx.lr = 0x82DDA5B0;
	sub_82DC1168(ctx, base);
	// 82DDA5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDA5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDA5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDA5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDA5C0 size=104
    let mut pc: u32 = 0x82DDA5C0;
    'dispatch: loop {
        match pc {
            0x82DDA5C0 => {
    //   block [0x82DDA5C0..0x82DDA628)
	// 82DDA5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDA5C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDA5CC: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDA5D0: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDA5D4: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDA5D8: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDA5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DDA5E0: 39089DD0  addi r8, r8, -0x6230
	ctx.r[8].s64 = ctx.r[8].s64 + -25136;
	// 82DDA5E4: 39299EA8  addi r9, r9, -0x6158
	ctx.r[9].s64 = ctx.r[9].s64 + -24920;
	// 82DDA5E8: 394AA350  addi r10, r10, -0x5cb0
	ctx.r[10].s64 = ctx.r[10].s64 + -23728;
	// 82DDA5EC: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DDA5F0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DDA5F4: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82DDA5F8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DDA5FC: 98E10061  stb r7, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[7].u8 ) };
	// 82DDA600: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDA604: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDA608: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDA60C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDA610: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDA614: 4BFE6C6D  bl 0x82dc1280
	ctx.lr = 0x82DDA618;
	sub_82DC1280(ctx, base);
	// 82DDA618: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDA61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDA620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDA624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDA628 size=120
    let mut pc: u32 = 0x82DDA628;
    'dispatch: loop {
        match pc {
            0x82DDA628 => {
    //   block [0x82DDA628..0x82DDA650)
	// 82DDA628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA62C: 4BECEDDD  bl 0x82ca9408
	ctx.lr = 0x82DDA630;
	sub_82CA93D0(ctx, base);
	// 82DDA630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDA634: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDA638: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDA63C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DDA640: 897D0031  lbz r11, 0x31(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(49 as u32) ) } as u64;
	// 82DDA644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDA648: 419A0038  beq cr6, 0x82dda680
	if ctx.cr[6].eq {
	pc = 0x82DDA680; continue 'dispatch;
	}
	// 82DDA64C: 3BFD0012  addi r31, r29, 0x12
	ctx.r[31].s64 = ctx.r[29].s64 + 18;
	pc = 0x82DDA650; continue 'dispatch;
            }
            0x82DDA650 => {
    //   block [0x82DDA650..0x82DDA680)
	// 82DDA650: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDA654: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDA658: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA65C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA660: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDA664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDA668: 4E800421  bctrl
	ctx.lr = 0x82DDA66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDA66C: 897D0031  lbz r11, 0x31(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(49 as u32) ) } as u64;
	// 82DDA670: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DDA674: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DDA678: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDA67C: 4198FFD4  blt cr6, 0x82dda650
	if ctx.cr[6].lt {
	pc = 0x82DDA650; continue 'dispatch;
	}
            }
            0x82DDA680 => {
    //   block [0x82DDA680..0x82DDA6A0)
	// 82DDA680: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA684: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDA688: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDA68C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDA694: 4E800421  bctrl
	ctx.lr = 0x82DDA698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDA698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDA69C: 4BECEDBC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDA6A0 size=184
    let mut pc: u32 = 0x82DDA6A0;
    'dispatch: loop {
        match pc {
            0x82DDA6A0 => {
    //   block [0x82DDA6A0..0x82DDA738)
	// 82DDA6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDA6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDA6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDA6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDA6B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA6B8: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDA6BC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDA6C0: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DDA6C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDA6C8: C1AB3C50  lfs f13, 0x3c50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15440 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DDA6CC: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DDA6D0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DDA6D4: 40980064  bge cr6, 0x82dda738
	if !ctx.cr[6].lt {
	pc = 0x82DDA738; continue 'dispatch;
	}
	// 82DDA6D8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA6DC: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DDA6E0: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DDA6E4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DDA6E8: 40980050  bge cr6, 0x82dda738
	if !ctx.cr[6].lt {
	pc = 0x82DDA738; continue 'dispatch;
	}
	// 82DDA6EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDA6F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDA6F4: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDA6F8: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82DDA6FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDA700: 4BF7AB49  bl 0x82d55248
	ctx.lr = 0x82DDA704;
	sub_82D55248(ctx, base);
	// 82DDA704: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDA708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDA70C: 396B3C18  addi r11, r11, 0x3c18
	ctx.r[11].s64 = ctx.r[11].s64 + 15384;
	// 82DDA710: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82DDA714: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDA718: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DDA71C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DDA720: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDA724: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DDA728: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DDA72C: 4800A7AD  bl 0x82de4ed8
	ctx.lr = 0x82DDA730;
	sub_82DE4ED8(ctx, base);
	// 82DDA730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDA734: 4800000C  b 0x82dda740
	pc = 0x82DDA740; continue 'dispatch;
            }
            0x82DDA738 => {
    //   block [0x82DDA738..0x82DDA740)
	// 82DDA738: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDA73C: 48009D15  bl 0x82de4450
	ctx.lr = 0x82DDA740;
	sub_82DE4450(ctx, base);
	pc = 0x82DDA740; continue 'dispatch;
            }
            0x82DDA740 => {
    //   block [0x82DDA740..0x82DDA758)
	// 82DDA740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDA744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDA748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDA74C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDA750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDA754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDA758 size=508
    let mut pc: u32 = 0x82DDA758;
    'dispatch: loop {
        match pc {
            0x82DDA758 => {
    //   block [0x82DDA758..0x82DDA954)
	// 82DDA758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA75C: 4BECECA1  bl 0x82ca93fc
	ctx.lr = 0x82DDA760;
	sub_82CA93D0(ctx, base);
	// 82DDA760: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDA958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDA958 size=552
    let mut pc: u32 = 0x82DDA958;
    'dispatch: loop {
        match pc {
            0x82DDA958 => {
    //   block [0x82DDA958..0x82DDAB80)
	// 82DDA958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDA95C: 4BECEAA5  bl 0x82ca9400
	ctx.lr = 0x82DDA960;
	sub_82CA93D0(ctx, base);
	// 82DDA960: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDAB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDAB80 size=20
    let mut pc: u32 = 0x82DDAB80;
    'dispatch: loop {
        match pc {
            0x82DDAB80 => {
    //   block [0x82DDAB80..0x82DDAB94)
	// 82DDAB80: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDAB84: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDAB88: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DDAB8C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DDAB90: 4BFFFDC8  b 0x82dda958
	sub_82DDA958(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDAB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDAB98 size=544
    let mut pc: u32 = 0x82DDAB98;
    'dispatch: loop {
        match pc {
            0x82DDAB98 => {
    //   block [0x82DDAB98..0x82DDADB8)
	// 82DDAB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDAB9C: 4BECE865  bl 0x82ca9400
	ctx.lr = 0x82DDABA0;
	sub_82CA93D0(ctx, base);
	// 82DDABA0: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDADB8 size=20
    let mut pc: u32 = 0x82DDADB8;
    'dispatch: loop {
        match pc {
            0x82DDADB8 => {
    //   block [0x82DDADB8..0x82DDADCC)
	// 82DDADB8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDADBC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDADC0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DDADC4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DDADC8: 4BFFFDD0  b 0x82ddab98
	sub_82DDAB98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDADD0 size=64
    let mut pc: u32 = 0x82DDADD0;
    'dispatch: loop {
        match pc {
            0x82DDADD0 => {
    //   block [0x82DDADD0..0x82DDAE10)
	// 82DDADD0: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDADD4: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDADD8: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDADDC: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDADE0: 38EB4B00  addi r7, r11, 0x4b00
	ctx.r[7].s64 = ctx.r[11].s64 + 19200;
	// 82DDADE4: 3908A6A0  addi r8, r8, -0x5960
	ctx.r[8].s64 = ctx.r[8].s64 + -22880;
	// 82DDADE8: 3929AB98  addi r9, r9, -0x5468
	ctx.r[9].s64 = ctx.r[9].s64 + -21608;
	// 82DDADEC: 394AA958  addi r10, r10, -0x56a8
	ctx.r[10].s64 = ctx.r[10].s64 + -22184;
	// 82DDADF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDADF4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DDADF8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DDADFC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDAE00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DDAE04: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DDAE08: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82DDAE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDAE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDAE10 size=104
    let mut pc: u32 = 0x82DDAE10;
    'dispatch: loop {
        match pc {
            0x82DDAE10 => {
    //   block [0x82DDAE10..0x82DDAE78)
	// 82DDAE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDAE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDAE18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDAE1C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDAE20: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDAE24: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DDAE28: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDAE2C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDAE30: 3908A6A0  addi r8, r8, -0x5960
	ctx.r[8].s64 = ctx.r[8].s64 + -22880;
	// 82DDAE34: 3929AB98  addi r9, r9, -0x5468
	ctx.r[9].s64 = ctx.r[9].s64 + -21608;
	// 82DDAE38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDAE3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDAE40: 394AA958  addi r10, r10, -0x56a8
	ctx.r[10].s64 = ctx.r[10].s64 + -22184;
	// 82DDAE44: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82DDAE48: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82DDAE4C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDAE50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDAE54: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDAE58: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DDAE5C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDAE60: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82DDAE64: 4BFE6305  bl 0x82dc1168
	ctx.lr = 0x82DDAE68;
	sub_82DC1168(ctx, base);
	// 82DDAE68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDAE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDAE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDAE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDAE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDAE78 size=96
    let mut pc: u32 = 0x82DDAE78;
    'dispatch: loop {
        match pc {
            0x82DDAE78 => {
    //   block [0x82DDAE78..0x82DDAEC0)
	// 82DDAE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDAE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDAE80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDAE84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDAE88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DDAE8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDAE90: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DDAE94: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DDAE98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDAE9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDAEA0: 419A0020  beq cr6, 0x82ddaec0
	if ctx.cr[6].eq {
	pc = 0x82DDAEC0; continue 'dispatch;
	}
	// 82DDAEA4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDAEA8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDAEAC: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82DDAEB0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDAEB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDAEB8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDAEBC: 4BF7A40D  bl 0x82d552c8
	ctx.lr = 0x82DDAEC0;
	sub_82D552C8(ctx, base);
	pc = 0x82DDAEC0; continue 'dispatch;
            }
            0x82DDAEC0 => {
    //   block [0x82DDAEC0..0x82DDAED8)
	// 82DDAEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDAEC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDAEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDAECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDAED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDAED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDAED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDAED8 size=160
    let mut pc: u32 = 0x82DDAED8;
    'dispatch: loop {
        match pc {
            0x82DDAED8 => {
    //   block [0x82DDAED8..0x82DDAF78)
	// 82DDAED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDAEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDAEE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDAEE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDAEE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDAEEC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDAEF0: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82DDAEF4: 394300C0  addi r10, r3, 0xc0
	ctx.r[10].s64 = ctx.r[3].s64 + 192;
	// 82DDAEF8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DDAEFC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DDAF00: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDAF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDAF78 size=148
    let mut pc: u32 = 0x82DDAF78;
    'dispatch: loop {
        match pc {
            0x82DDAF78 => {
    //   block [0x82DDAF78..0x82DDAFE4)
	// 82DDAF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDAF7C: 4BECE48D  bl 0x82ca9408
	ctx.lr = 0x82DDAF80;
	sub_82CA93D0(ctx, base);
	// 82DDAF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDAF84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDAF88: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDAF8C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDAF90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDAF94: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDAF98: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DDAF9C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDAFA0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDAFA4: 419A0040  beq cr6, 0x82ddafe4
	if ctx.cr[6].eq {
	pc = 0x82DDAFE4; continue 'dispatch;
	}
	// 82DDAFA8: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DDAFAC: 4BF7A29D  bl 0x82d55248
	ctx.lr = 0x82DDAFB0;
	sub_82D55248(ctx, base);
	// 82DDAFB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDAFB4: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82DDAFB8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDAFBC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDAFC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDAFC4: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDAFC8: 48009441  bl 0x82de4408
	ctx.lr = 0x82DDAFCC;
	sub_82DE4408(ctx, base);
	// 82DDAFCC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDAFD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDAFD4: 396B35D0  addi r11, r11, 0x35d0
	ctx.r[11].s64 = ctx.r[11].s64 + 13776;
	// 82DDAFD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDAFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDAFE0: 4BECE478  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DDAFE4 => {
    //   block [0x82DDAFE4..0x82DDB00C)
	// 82DDAFE4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82DDAFE8: 4BF7A261  bl 0x82d55248
	ctx.lr = 0x82DDAFEC;
	sub_82D55248(ctx, base);
	// 82DDAFEC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82DDAFF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DDAFF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDAFF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDAFFC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDB000: 48008761  bl 0x82de3760
	ctx.lr = 0x82DDB004;
	sub_82DE3760(ctx, base);
	// 82DDB004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDB008: 4BECE450  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDB010 size=108
    let mut pc: u32 = 0x82DDB010;
    'dispatch: loop {
        match pc {
            0x82DDB010 => {
    //   block [0x82DDB010..0x82DDB07C)
	// 82DDB010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDB014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDB018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDB01C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDB020: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDB024: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DDB028: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDB02C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDB030: 3908AF78  addi r8, r8, -0x5088
	ctx.r[8].s64 = ctx.r[8].s64 + -20616;
	// 82DDB034: 39293B28  addi r9, r9, 0x3b28
	ctx.r[9].s64 = ctx.r[9].s64 + 15144;
	// 82DDB038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDB03C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDB040: 394A3CD0  addi r10, r10, 0x3cd0
	ctx.r[10].s64 = ctx.r[10].s64 + 15568;
	// 82DDB044: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DDB048: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DDB04C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDB050: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDB054: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDB058: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DDB05C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDB060: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDB064: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82DDB068: 4BFE6101  bl 0x82dc1168
	ctx.lr = 0x82DDB06C;
	sub_82DC1168(ctx, base);
	// 82DDB06C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDB070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDB074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDB078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDB080 size=1496
    let mut pc: u32 = 0x82DDB080;
    'dispatch: loop {
        match pc {
            0x82DDB080 => {
    //   block [0x82DDB080..0x82DDB0DC)
	// 82DDB080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDB084: 4BECE35D  bl 0x82ca93e0
	ctx.lr = 0x82DDB088;
	sub_82CA93D0(ctx, base);
	// 82DDB088: 9421FB30  stwu r1, -0x4d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1232 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDB08C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB090: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DDB094: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DDB098: 7ECB5214  add r22, r11, r10
	ctx.r[22].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDB09C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDB0A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDB0A4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDB0A8: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DDB0AC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB0B0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDB0B4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB0B8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDB0BC: 40980020  bge cr6, 0x82ddb0dc
	if !ctx.cr[6].lt {
	pc = 0x82DDB0DC; continue 'dispatch;
	}
	// 82DDB0C0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDB0C4: 39293C70  addi r9, r9, 0x3c70
	ctx.r[9].s64 = ctx.r[9].s64 + 15472;
	// 82DDB0C8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDB0CC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDB0D0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDB0D4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDB0D8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDB0DC; continue 'dispatch;
            }
            0x82DDB0DC => {
    //   block [0x82DDB0DC..0x82DDB14C)
	// 82DDB0DC: C03C0050  lfs f1, 0x50(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDB0E0: C01B0018  lfs f0, 0x18(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDB0E4: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82DDB0E8: 419A0178  beq cr6, 0x82ddb260
	if ctx.cr[6].eq {
	pc = 0x82DDB260; continue 'dispatch;
	}
	// 82DDB0EC: 817C0060  lwz r11, 0x60(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDB0F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDB0F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDB0F8: 409A005C  bne cr6, 0x82ddb154
	if !ctx.cr[6].eq {
	pc = 0x82DDB154; continue 'dispatch;
	}
	// 82DDB0FC: C01C0054  lfs f0, 0x54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDB100: D01B0018  stfs f0, 0x18(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DDB104: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82DDB108: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDB10C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDB110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDB114: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DDB118: 48009481  bl 0x82de4598
	ctx.lr = 0x82DDB11C;
	sub_82DE4598(ctx, base);
	// 82DDB11C: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB120: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDB124: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB128: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDB12C: 40980020  bge cr6, 0x82ddb14c
	if !ctx.cr[6].lt {
	pc = 0x82DDB14C; continue 'dispatch;
	}
	// 82DDB130: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDB134: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDB138: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDB13C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDB140: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDB144: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDB148: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDB14C; continue 'dispatch;
            }
            0x82DDB14C => {
    //   block [0x82DDB14C..0x82DDB154)
	// 82DDB14C: 382104D0  addi r1, r1, 0x4d0
	ctx.r[1].s64 = ctx.r[1].s64 + 1232;
	// 82DDB150: 4BECE2E0  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DDB154 => {
    //   block [0x82DDB154..0x82DDB260)
	// 82DDB154: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDB158: 38A10410  addi r5, r1, 0x410
	ctx.r[5].s64 = ctx.r[1].s64 + 1040;
	// 82DDB15C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB160: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82DDB164: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDB168: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82DDB16C: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82DDB170: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DDB174: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82DDB178: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB17C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DDB180: 39610410  addi r11, r1, 0x410
	ctx.r[11].s64 = ctx.r[1].s64 + 1040;
	// 82DDB184: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82DDB188: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDB18C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82DDB190: 396102B0  addi r11, r1, 0x2b0
	ctx.r[11].s64 = ctx.r[1].s64 + 688;
	// 82DDB194: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82DDB198: 482EF409  bl 0x830ca5a0
	ctx.lr = 0x82DDB19C;
	sub_830CA5A0(ctx, base);
	// 82DDB19C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDB1A0: 38A102B0  addi r5, r1, 0x2b0
	ctx.r[5].s64 = ctx.r[1].s64 + 688;
	// 82DDB1A4: C03C0050  lfs f1, 0x50(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDB1A8: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82DDB1AC: 482EF3F5  bl 0x830ca5a0
	ctx.lr = 0x82DDB1B0;
	sub_830CA5A0(ctx, base);
	// 82DDB1B0: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 82DDB1B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDB1B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB1BC: 835D0000  lwz r26, 0(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB1C0: 38C10150  addi r6, r1, 0x150
	ctx.r[6].s64 = ctx.r[1].s64 + 336;
	// 82DDB1C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDB1C8: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82DDB1CC: 91610140  stw r11, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 82DDB1D0: 91610144  stw r11, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 82DDB1D4: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DDB1D8: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDB1DC: 91410134  stw r10, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[10].u32 ) };
	// 82DDB1E0: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDB1E4: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DDB1E8: 90A10130  stw r5, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[5].u32 ) };
	// 82DDB1EC: 91410138  stw r10, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[10].u32 ) };
	// 82DDB1F0: 9161013C  stw r11, 0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 82DDB1F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB1F8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDB1FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB200: 4E800421  bctrl
	ctx.lr = 0x82DDB204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDB204: 81610130  lwz r11, 0x130(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 82DDB208: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB20C: 38C10210  addi r6, r1, 0x210
	ctx.r[6].s64 = ctx.r[1].s64 + 528;
	// 82DDB210: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDB214: 80A10134  lwz r5, 0x134(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82DDB218: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDB21C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DDB220: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDB224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB228: 4E800421  bctrl
	ctx.lr = 0x82DDB22C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDB22C: 817C0060  lwz r11, 0x60(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDB230: 38FB0020  addi r7, r27, 0x20
	ctx.r[7].s64 = ctx.r[27].s64 + 32;
	// 82DDB234: 38C10130  addi r6, r1, 0x130
	ctx.r[6].s64 = ctx.r[1].s64 + 304;
	// 82DDB238: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DDB23C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDB240: C02B0000  lfs f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDB244: 4800860D  bl 0x82de3850
	ctx.lr = 0x82DDB248;
	sub_82DE3850(ctx, base);
	// 82DDB248: 81610144  lwz r11, 0x144(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 82DDB24C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDB250: 419A0010  beq cr6, 0x82ddb260
	if ctx.cr[6].eq {
	pc = 0x82DDB260; continue 'dispatch;
	}
	// 82DDB254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDB258: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 82DDB25C: 4803EA25  bl 0x82e19c80
	ctx.lr = 0x82DDB260;
	sub_82E19C80(ctx, base);
            }
            0x82DDB260 => {
    //   block [0x82DDB260..0x82DDB658)
	// 82DDB260: C01C0054  lfs f0, 0x54(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDB264: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82DDB268: D01B0018  stfs f0, 0x18(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DDB26C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDB270: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDB274: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDB278: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
	// 82DDB27C: C01C0058  lfs f0, 0x58(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDB280: 38EA0040  addi r7, r10, 0x40
	ctx.r[7].s64 = ctx.r[10].s64 + 64;
	// 82DDB284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDB288: C1AB00A0  lfs f13, 0xa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DDB28C: 391B0020  addi r8, r27, 0x20
	ctx.r[8].s64 = ctx.r[27].s64 + 32;
	// 82DDB290: C18B009C  lfs f12, 0x9c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DDB294: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDB298: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDB658 size=144
    let mut pc: u32 = 0x82DDB658;
    'dispatch: loop {
        match pc {
            0x82DDB658 => {
    //   block [0x82DDB658..0x82DDB694)
	// 82DDB658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDB65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDB660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDB664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDB668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDB66C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDB670: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82DDB674: 409A0020  bne cr6, 0x82ddb694
	if !ctx.cr[6].eq {
	pc = 0x82DDB694; continue 'dispatch;
	}
	// 82DDB678: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDB67C: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDB680: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB688: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDB68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB690: 4E800421  bctrl
	ctx.lr = 0x82DDB694;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDB694 => {
    //   block [0x82DDB694..0x82DDB6BC)
	// 82DDB694: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDB698: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82DDB69C: 409A0020  bne cr6, 0x82ddb6bc
	if !ctx.cr[6].eq {
	pc = 0x82DDB6BC; continue 'dispatch;
	}
	// 82DDB6A0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDB6A4: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB6A8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDB6AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB6B0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDB6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB6B8: 4E800421  bctrl
	ctx.lr = 0x82DDB6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDB6BC => {
    //   block [0x82DDB6BC..0x82DDB6E8)
	// 82DDB6BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB6C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDB6C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDB6C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB6CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB6D0: 4E800421  bctrl
	ctx.lr = 0x82DDB6D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDB6D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDB6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDB6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDB6E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDB6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB6E8 size=4
    let mut pc: u32 = 0x82DDB6E8;
    'dispatch: loop {
        match pc {
            0x82DDB6E8 => {
    //   block [0x82DDB6E8..0x82DDB6EC)
	// 82DDB6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB6F0 size=20
    let mut pc: u32 = 0x82DDB6F0;
    'dispatch: loop {
        match pc {
            0x82DDB6F0 => {
    //   block [0x82DDB6F0..0x82DDB704)
	// 82DDB6F0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB6F4: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82DDB6F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDB6FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB700: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB708 size=28
    let mut pc: u32 = 0x82DDB708;
    'dispatch: loop {
        match pc {
            0x82DDB708 => {
    //   block [0x82DDB708..0x82DDB724)
	// 82DDB708: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB70C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DDB710: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DDB714: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82DDB718: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDB71C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB720: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB728 size=4
    let mut pc: u32 = 0x82DDB728;
    'dispatch: loop {
        match pc {
            0x82DDB728 => {
    //   block [0x82DDB728..0x82DDB72C)
	// 82DDB728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB730 size=4
    let mut pc: u32 = 0x82DDB730;
    'dispatch: loop {
        match pc {
            0x82DDB730 => {
    //   block [0x82DDB730..0x82DDB734)
	// 82DDB730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB738 size=4
    let mut pc: u32 = 0x82DDB738;
    'dispatch: loop {
        match pc {
            0x82DDB738 => {
    //   block [0x82DDB738..0x82DDB73C)
	// 82DDB738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB740 size=4
    let mut pc: u32 = 0x82DDB740;
    'dispatch: loop {
        match pc {
            0x82DDB740 => {
    //   block [0x82DDB740..0x82DDB744)
	// 82DDB740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDB748 size=124
    let mut pc: u32 = 0x82DDB748;
    'dispatch: loop {
        match pc {
            0x82DDB748 => {
    //   block [0x82DDB748..0x82DDB770)
	// 82DDB748: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDB74C: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DDB750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDB754: 394B3C7C  addi r10, r11, 0x3c7c
	ctx.r[10].s64 = ctx.r[11].s64 + 15484;
	// 82DDB758: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDB75C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DDB760: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDB764: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB768: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDB76C: 419A0014  beq cr6, 0x82ddb780
	if ctx.cr[6].eq {
	pc = 0x82DDB780; continue 'dispatch;
	}
	pc = 0x82DDB770; continue 'dispatch;
            }
            0x82DDB770 => {
    //   block [0x82DDB770..0x82DDB780)
	// 82DDB770: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DDB774: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB778: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDB77C: 409AFFF4  bne cr6, 0x82ddb770
	if !ctx.cr[6].eq {
	pc = 0x82DDB770; continue 'dispatch;
	}
	pc = 0x82DDB780; continue 'dispatch;
            }
            0x82DDB780 => {
    //   block [0x82DDB780..0x82DDB794)
	// 82DDB780: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DDB784: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DDB788: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB78C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB790: 419A0014  beq cr6, 0x82ddb7a4
	if ctx.cr[6].eq {
	pc = 0x82DDB7A4; continue 'dispatch;
	}
	pc = 0x82DDB794; continue 'dispatch;
            }
            0x82DDB794 => {
    //   block [0x82DDB794..0x82DDB7A4)
	// 82DDB794: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DDB798: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB79C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB7A0: 409AFFF4  bne cr6, 0x82ddb794
	if !ctx.cr[6].eq {
	pc = 0x82DDB794; continue 'dispatch;
	}
	pc = 0x82DDB7A4; continue 'dispatch;
            }
            0x82DDB7A4 => {
    //   block [0x82DDB7A4..0x82DDB7C4)
	// 82DDB7A4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDB7A8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB7AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB7B0: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DDB7B4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB7B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB7BC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DDB7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDB7C8 size=304
    let mut pc: u32 = 0x82DDB7C8;
    'dispatch: loop {
        match pc {
            0x82DDB7C8 => {
    //   block [0x82DDB7C8..0x82DDB834)
	// 82DDB7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDB7CC: 4BECDC39  bl 0x82ca9404
	ctx.lr = 0x82DDB7D0;
	sub_82CA93D0(ctx, base);
	// 82DDB7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDB7D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB7D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDB7DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDB7E0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDB7E4: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDB7E8: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82DDB7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDB7F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDB7F4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDB7F8: 4BF79A51  bl 0x82d55248
	ctx.lr = 0x82DDB7FC;
	sub_82D55248(ctx, base);
	// 82DDB7FC: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82DDB800: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDB804: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDB808: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDB80C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDB810: 4BFFFF39  bl 0x82ddb748
	ctx.lr = 0x82DDB814;
	sub_82DDB748(ctx, base);
	// 82DDB814: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDB818: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDB81C: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82DDB820: 409A0060  bne cr6, 0x82ddb880
	if !ctx.cr[6].eq {
	pc = 0x82DDB880; continue 'dispatch;
	}
	// 82DDB824: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB828: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB82C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB830: 419A0014  beq cr6, 0x82ddb844
	if ctx.cr[6].eq {
	pc = 0x82DDB844; continue 'dispatch;
	}
	pc = 0x82DDB834; continue 'dispatch;
            }
            0x82DDB834 => {
    //   block [0x82DDB834..0x82DDB844)
	// 82DDB834: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDB838: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB83C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB840: 409AFFF4  bne cr6, 0x82ddb834
	if !ctx.cr[6].eq {
	pc = 0x82DDB834; continue 'dispatch;
	}
	pc = 0x82DDB844; continue 'dispatch;
            }
            0x82DDB844 => {
    //   block [0x82DDB844..0x82DDB854)
	// 82DDB844: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDB84C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB850: 419A0014  beq cr6, 0x82ddb864
	if ctx.cr[6].eq {
	pc = 0x82DDB864; continue 'dispatch;
	}
	pc = 0x82DDB854; continue 'dispatch;
            }
            0x82DDB854 => {
    //   block [0x82DDB854..0x82DDB864)
	// 82DDB854: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDB858: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB85C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB860: 409AFFF4  bne cr6, 0x82ddb854
	if !ctx.cr[6].eq {
	pc = 0x82DDB854; continue 'dispatch;
	}
	pc = 0x82DDB864; continue 'dispatch;
            }
            0x82DDB864 => {
    //   block [0x82DDB864..0x82DDB880)
	// 82DDB864: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB868: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDB86C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDB870: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDB874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB878: 4E800421  bctrl
	ctx.lr = 0x82DDB87C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDB87C: 93DC0014  stw r30, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
            }
            0x82DDB880 => {
    //   block [0x82DDB880..0x82DDB8A0)
	// 82DDB880: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDB884: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82DDB888: 409A0064  bne cr6, 0x82ddb8ec
	if !ctx.cr[6].eq {
	pc = 0x82DDB8EC; continue 'dispatch;
	}
	// 82DDB88C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB890: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDB894: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB898: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB89C: 419A0014  beq cr6, 0x82ddb8b0
	if ctx.cr[6].eq {
	pc = 0x82DDB8B0; continue 'dispatch;
	}
	pc = 0x82DDB8A0; continue 'dispatch;
            }
            0x82DDB8A0 => {
    //   block [0x82DDB8A0..0x82DDB8B0)
	// 82DDB8A0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDB8A4: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB8A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB8AC: 409AFFF4  bne cr6, 0x82ddb8a0
	if !ctx.cr[6].eq {
	pc = 0x82DDB8A0; continue 'dispatch;
	}
	pc = 0x82DDB8B0; continue 'dispatch;
            }
            0x82DDB8B0 => {
    //   block [0x82DDB8B0..0x82DDB8C0)
	// 82DDB8B0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB8B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDB8B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB8BC: 419A0014  beq cr6, 0x82ddb8d0
	if ctx.cr[6].eq {
	pc = 0x82DDB8D0; continue 'dispatch;
	}
	pc = 0x82DDB8C0; continue 'dispatch;
            }
            0x82DDB8C0 => {
    //   block [0x82DDB8C0..0x82DDB8D0)
	// 82DDB8C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDB8C4: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB8C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB8CC: 409AFFF4  bne cr6, 0x82ddb8c0
	if !ctx.cr[6].eq {
	pc = 0x82DDB8C0; continue 'dispatch;
	}
	pc = 0x82DDB8D0; continue 'dispatch;
            }
            0x82DDB8D0 => {
    //   block [0x82DDB8D0..0x82DDB8EC)
	// 82DDB8D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB8D4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDB8D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DDB8DC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDB8E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB8E4: 4E800421  bctrl
	ctx.lr = 0x82DDB8E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDB8E8: 93DC0018  stw r30, 0x18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
            }
            0x82DDB8EC => {
    //   block [0x82DDB8EC..0x82DDB8F8)
	// 82DDB8EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDB8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDB8F4: 4BECDB60  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDB8F8 size=236
    let mut pc: u32 = 0x82DDB8F8;
    'dispatch: loop {
        match pc {
            0x82DDB8F8 => {
    //   block [0x82DDB8F8..0x82DDB930)
	// 82DDB8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDB8FC: 4BECDB11  bl 0x82ca940c
	ctx.lr = 0x82DDB900;
	sub_82CA93D0(ctx, base);
	// 82DDB900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDB904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDB908: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDB90C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDB910: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB914: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB918: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82DDB91C: 409A0058  bne cr6, 0x82ddb974
	if !ctx.cr[6].eq {
	pc = 0x82DDB974; continue 'dispatch;
	}
	// 82DDB920: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB924: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDB928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB92C: 419A0014  beq cr6, 0x82ddb940
	if ctx.cr[6].eq {
	pc = 0x82DDB940; continue 'dispatch;
	}
	pc = 0x82DDB930; continue 'dispatch;
            }
            0x82DDB930 => {
    //   block [0x82DDB930..0x82DDB940)
	// 82DDB930: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDB934: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB93C: 409AFFF4  bne cr6, 0x82ddb930
	if !ctx.cr[6].eq {
	pc = 0x82DDB930; continue 'dispatch;
	}
	pc = 0x82DDB940; continue 'dispatch;
            }
            0x82DDB940 => {
    //   block [0x82DDB940..0x82DDB950)
	// 82DDB940: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB944: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDB948: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB94C: 419A0014  beq cr6, 0x82ddb960
	if ctx.cr[6].eq {
	pc = 0x82DDB960; continue 'dispatch;
	}
	pc = 0x82DDB950; continue 'dispatch;
            }
            0x82DDB950 => {
    //   block [0x82DDB950..0x82DDB960)
	// 82DDB950: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDB954: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB95C: 409AFFF4  bne cr6, 0x82ddb950
	if !ctx.cr[6].eq {
	pc = 0x82DDB950; continue 'dispatch;
	}
	pc = 0x82DDB960; continue 'dispatch;
            }
            0x82DDB960 => {
    //   block [0x82DDB960..0x82DDB974)
	// 82DDB960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB964: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDB968: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDB96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB970: 4E800421  bctrl
	ctx.lr = 0x82DDB974;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDB974 => {
    //   block [0x82DDB974..0x82DDB994)
	// 82DDB974: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB978: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB97C: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82DDB980: 409A0058  bne cr6, 0x82ddb9d8
	if !ctx.cr[6].eq {
	pc = 0x82DDB9D8; continue 'dispatch;
	}
	// 82DDB984: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB988: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDB98C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB990: 419A0014  beq cr6, 0x82ddb9a4
	if ctx.cr[6].eq {
	pc = 0x82DDB9A4; continue 'dispatch;
	}
	pc = 0x82DDB994; continue 'dispatch;
            }
            0x82DDB994 => {
    //   block [0x82DDB994..0x82DDB9A4)
	// 82DDB994: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDB998: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB99C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB9A0: 409AFFF4  bne cr6, 0x82ddb994
	if !ctx.cr[6].eq {
	pc = 0x82DDB994; continue 'dispatch;
	}
	pc = 0x82DDB9A4; continue 'dispatch;
            }
            0x82DDB9A4 => {
    //   block [0x82DDB9A4..0x82DDB9B4)
	// 82DDB9A4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB9A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDB9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB9B0: 419A0014  beq cr6, 0x82ddb9c4
	if ctx.cr[6].eq {
	pc = 0x82DDB9C4; continue 'dispatch;
	}
	pc = 0x82DDB9B4; continue 'dispatch;
            }
            0x82DDB9B4 => {
    //   block [0x82DDB9B4..0x82DDB9C4)
	// 82DDB9B4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDB9B8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDB9BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDB9C0: 409AFFF4  bne cr6, 0x82ddb9b4
	if !ctx.cr[6].eq {
	pc = 0x82DDB9B4; continue 'dispatch;
	}
	pc = 0x82DDB9C4; continue 'dispatch;
            }
            0x82DDB9C4 => {
    //   block [0x82DDB9C4..0x82DDB9D8)
	// 82DDB9C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDB9C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDB9CC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDB9D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDB9D4: 4E800421  bctrl
	ctx.lr = 0x82DDB9D8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDB9D8 => {
    //   block [0x82DDB9D8..0x82DDB9E4)
	// 82DDB9D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DDB9DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDB9E0: 4BECDA7C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDB9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDB9E8 size=140
    let mut pc: u32 = 0x82DDB9E8;
    'dispatch: loop {
        match pc {
            0x82DDB9E8 => {
    //   block [0x82DDB9E8..0x82DDBA74)
	// 82DDB9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDB9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDB9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDB9F4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDB9F8: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDB9FC: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDBA00: 396BB740  addi r11, r11, -0x48c0
	ctx.r[11].s64 = ctx.r[11].s64 + -18624;
	// 82DDBA04: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDBA08: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDBA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DDBA10: 3908B7C8  addi r8, r8, -0x4838
	ctx.r[8].s64 = ctx.r[8].s64 + -18488;
	// 82DDBA14: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDBA18: 3929B708  addi r9, r9, -0x48f8
	ctx.r[9].s64 = ctx.r[9].s64 + -18680;
	// 82DDBA1C: 394AB730  addi r10, r10, -0x48d0
	ctx.r[10].s64 = ctx.r[10].s64 + -18640;
	// 82DDBA20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DDBA24: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DDBA28: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82DDBA2C: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 82DDBA30: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDBA34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDBA38: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDBA3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDBA40: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDBA44: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82DDBA48: 4BFE5721  bl 0x82dc1168
	ctx.lr = 0x82DDBA4C;
	sub_82DC1168(ctx, base);
	// 82DDBA4C: 38C0001D  li r6, 0x1d
	ctx.r[6].s64 = 29;
	// 82DDBA50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DDBA54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDBA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDBA5C: 4BFE570D  bl 0x82dc1168
	ctx.lr = 0x82DDBA60;
	sub_82DC1168(ctx, base);
	// 82DDBA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDBA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDBA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDBA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDBA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBA78 size=84
    let mut pc: u32 = 0x82DDBA78;
    'dispatch: loop {
        match pc {
            0x82DDBA78 => {
    //   block [0x82DDBA78..0x82DDBACC)
	// 82DDBA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDBA80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDBA84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBA88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDBA8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBA90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBA94: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDBA98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBA9C: 4E800421  bctrl
	ctx.lr = 0x82DDBAA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBAA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBAA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDBAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDBAAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBAB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBAB4: 4E800421  bctrl
	ctx.lr = 0x82DDBAB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DDBABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDBAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDBAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDBAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBAD0 size=144
    let mut pc: u32 = 0x82DDBAD0;
    'dispatch: loop {
        match pc {
            0x82DDBAD0 => {
    //   block [0x82DDBAD0..0x82DDBB60)
	// 82DDBAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDBAD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDBADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDBAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBAE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDBAE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDBAEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDBAF0: 388B3CDC  addi r4, r11, 0x3cdc
	ctx.r[4].s64 = ctx.r[11].s64 + 15580;
	// 82DDBAF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDBAF8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBAFC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DDBB00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDBB04: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDBB08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBB0C: 4E800421  bctrl
	ctx.lr = 0x82DDBB10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBB10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBB14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDBB18: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBB1C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DDBB20: 388B3CD0  addi r4, r11, 0x3cd0
	ctx.r[4].s64 = ctx.r[11].s64 + 15568;
	// 82DDBB24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDBB28: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBB2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBB30: 4E800421  bctrl
	ctx.lr = 0x82DDBB34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBB34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBB38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDBB3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDBB40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBB44: 4E800421  bctrl
	ctx.lr = 0x82DDBB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBB48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDBB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDBB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDBB54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDBB58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDBB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB60 size=20
    let mut pc: u32 = 0x82DDBB60;
    'dispatch: loop {
        match pc {
            0x82DDBB60 => {
    //   block [0x82DDBB60..0x82DDBB74)
	// 82DDBB60: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBB64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBB68: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDBB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBB70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB78 size=20
    let mut pc: u32 = 0x82DDBB78;
    'dispatch: loop {
        match pc {
            0x82DDBB78 => {
    //   block [0x82DDBB78..0x82DDBB8C)
	// 82DDBB78: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBB7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBB80: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDBB84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBB88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBB90 size=20
    let mut pc: u32 = 0x82DDBB90;
    'dispatch: loop {
        match pc {
            0x82DDBB90 => {
    //   block [0x82DDBB90..0x82DDBBA4)
	// 82DDBB90: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBB94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBB98: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDBB9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBBA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBBA8 size=20
    let mut pc: u32 = 0x82DDBBA8;
    'dispatch: loop {
        match pc {
            0x82DDBBA8 => {
    //   block [0x82DDBBA8..0x82DDBBBC)
	// 82DDBBA8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBBAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBBB0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DDBBB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBBB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDBBC0 size=20
    let mut pc: u32 = 0x82DDBBC0;
    'dispatch: loop {
        match pc {
            0x82DDBBC0 => {
    //   block [0x82DDBBC0..0x82DDBBD4)
	// 82DDBBC0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBBC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBBC8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDBBCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBBD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBBD8 size=264
    let mut pc: u32 = 0x82DDBBD8;
    'dispatch: loop {
        match pc {
            0x82DDBBD8 => {
    //   block [0x82DDBBD8..0x82DDBC94)
	// 82DDBBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBBDC: 4BECD81D  bl 0x82ca93f8
	ctx.lr = 0x82DDBBE0;
	sub_82CA93D0(ctx, base);
	// 82DDBBE0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBBE4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBBE8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDBBEC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DDBBF0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDBBF4: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDBBF8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DDBBFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDBC00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDBC04: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DDBC08: 4BF79641  bl 0x82d55248
	ctx.lr = 0x82DDBC0C;
	sub_82D55248(ctx, base);
	// 82DDBC0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDBC10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDBC14: 396B3CF0  addi r11, r11, 0x3cf0
	ctx.r[11].s64 = ctx.r[11].s64 + 15600;
	// 82DDBC18: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DDBC1C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDBC20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDBC24: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82DDBC28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDBC2C: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DDBC30: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DDBC34: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBC38: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDBC3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDBC40: 835B0014  lwz r26, 0x14(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDBC44: 4B4C1EE5  bl 0x8229db28
	ctx.lr = 0x82DDBC48;
	sub_8229DB28(ctx, base);
	// 82DDBC48: 38BB0030  addi r5, r27, 0x30
	ctx.r[5].s64 = ctx.r[27].s64 + 48;
	// 82DDBC4C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDBC50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDBC54: 4BF7B6AD  bl 0x82d57300
	ctx.lr = 0x82DDBC58;
	sub_82D57300(ctx, base);
	// 82DDBC58: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDBC5C: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBC60: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDBC64: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBC68: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82DDBC6C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDBC70: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDBC74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDBC78: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82DDBC7C: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82DDBC80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDBC84: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBC88: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBC8C: 409A0008  bne cr6, 0x82ddbc94
	if !ctx.cr[6].eq {
	pc = 0x82DDBC94; continue 'dispatch;
	}
	// 82DDBC90: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x82DDBC94; continue 'dispatch;
            }
            0x82DDBC94 => {
    //   block [0x82DDBC94..0x82DDBCE0)
	// 82DDBC94: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDBC98: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82DDBC9C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DDBCA0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDBCA4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DDBCA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDBCAC: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDBCB0: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDBCB4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDBCB8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDBCBC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDBCC0: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDBCC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBCC8: 4E800421  bctrl
	ctx.lr = 0x82DDBCCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBCCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDBCD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDBCD4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DDBCD8: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82DDBCDC: 4BECD76C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBCE0 size=576
    let mut pc: u32 = 0x82DDBCE0;
    'dispatch: loop {
        match pc {
            0x82DDBCE0 => {
    //   block [0x82DDBCE0..0x82DDBD3C)
	// 82DDBCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBCE4: 4BECD70D  bl 0x82ca93f0
	ctx.lr = 0x82DDBCE8;
	sub_82CA93D0(ctx, base);
	// 82DDBCE8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBCEC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBCF0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DDBCF4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDBCF8: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDBCFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDBD00: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DDBD04: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DDBD08: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DDBD0C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBD10: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDBD14: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBD18: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDBD1C: 40980020  bge cr6, 0x82ddbd3c
	if !ctx.cr[6].lt {
	pc = 0x82DDBD3C; continue 'dispatch;
	}
	// 82DDBD20: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDBD24: 39293D28  addi r9, r9, 0x3d28
	ctx.r[9].s64 = ctx.r[9].s64 + 15656;
	// 82DDBD28: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDBD2C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDBD30: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDBD34: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDBD38: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDBD3C; continue 'dispatch;
            }
            0x82DDBD3C => {
    //   block [0x82DDBD3C..0x82DDBF20)
	// 82DDBD3C: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBD40: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DDBD44: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDBD48: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82DDBD4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDBD50: 4BF7B5B1  bl 0x82d57300
	ctx.lr = 0x82DDBD54;
	sub_82D57300(ctx, base);
	// 82DDBD54: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDBD58: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82DDBD5C: 396A0040  addi r11, r10, 0x40
	ctx.r[11].s64 = ctx.r[10].s64 + 64;
	// 82DDBD60: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82DDBD64: 393D0020  addi r9, r29, 0x20
	ctx.r[9].s64 = ctx.r[29].s64 + 32;
	// 82DDBD68: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82DDBD6C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBF20 size=132
    let mut pc: u32 = 0x82DDBF20;
    'dispatch: loop {
        match pc {
            0x82DDBF20 => {
    //   block [0x82DDBF20..0x82DDBFA4)
	// 82DDBF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBF24: 4BECD4D9  bl 0x82ca93fc
	ctx.lr = 0x82DDBF28;
	sub_82CA93D0(ctx, base);
	// 82DDBF28: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBF2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDBF30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDBF34: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDBF38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDBF3C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDBF40: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBF44: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDBF48: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDBF4C: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82DDBF50: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDBF54: 4BF7B3AD  bl 0x82d57300
	ctx.lr = 0x82DDBF58;
	sub_82D57300(ctx, base);
	// 82DDBF58: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDBF5C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDBF60: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82DDBF64: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDBF68: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDBF6C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDBF70: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDBF74: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDBF78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDBF7C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDBF80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDBF84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDBF88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDBF8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBF90: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDBF94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDBF98: 4E800421  bctrl
	ctx.lr = 0x82DDBF9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDBF9C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDBFA0: 4BECD4AC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDBFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDBFA8 size=168
    let mut pc: u32 = 0x82DDBFA8;
    'dispatch: loop {
        match pc {
            0x82DDBFA8 => {
    //   block [0x82DDBFA8..0x82DDC050)
	// 82DDBFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDBFAC: 4BECD455  bl 0x82ca9400
	ctx.lr = 0x82DDBFB0;
	sub_82CA93D0(ctx, base);
	// 82DDBFB0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDBFB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDBFB8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDBFBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDBFC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDBFC4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDBFC8: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBFCC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDBFD0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDBFD4: 38BC0030  addi r5, r28, 0x30
	ctx.r[5].s64 = ctx.r[28].s64 + 48;
	// 82DDBFD8: 4BF7B329  bl 0x82d57300
	ctx.lr = 0x82DDBFDC;
	sub_82D57300(ctx, base);
	// 82DDBFDC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDBFE0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDBFE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDBFE8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBFEC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDBFF0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDBFF4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDBFF8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDBFFC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDC000: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDC004: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDC00C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82DDC010: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DDC014: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC018: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC01C: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDC020: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC024: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC028: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDC02C: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDC030: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC034: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC038: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDC03C: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82DDC040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC044: 4E800421  bctrl
	ctx.lr = 0x82DDC048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC048: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDC04C: 4BECD404  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC050 size=124
    let mut pc: u32 = 0x82DDC050;
    'dispatch: loop {
        match pc {
            0x82DDC050 => {
    //   block [0x82DDC050..0x82DDC0CC)
	// 82DDC050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC054: 4BECD3AD  bl 0x82ca9400
	ctx.lr = 0x82DDC058;
	sub_82CA93D0(ctx, base);
	// 82DDC058: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC05C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDC060: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC064: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDC068: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC06C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC070: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC074: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDC078: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC07C: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDC080: 4BF7B281  bl 0x82d57300
	ctx.lr = 0x82DDC084;
	sub_82D57300(ctx, base);
	// 82DDC084: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDC088: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDC08C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDC090: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC094: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC098: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDC09C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDC0A0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDC0A4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC0A8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDC0AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC0B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDC0B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC0B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDC0BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC0C0: 4E800421  bctrl
	ctx.lr = 0x82DDC0C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC0C4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDC0C8: 4BECD388  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC0D0 size=160
    let mut pc: u32 = 0x82DDC0D0;
    'dispatch: loop {
        match pc {
            0x82DDC0D0 => {
    //   block [0x82DDC0D0..0x82DDC170)
	// 82DDC0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC0D4: 4BECD331  bl 0x82ca9404
	ctx.lr = 0x82DDC0D8;
	sub_82CA93D0(ctx, base);
	// 82DDC0D8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC0DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC0E0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDC0E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC0E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC0EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC0F0: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC0F4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC0F8: 38BC0030  addi r5, r28, 0x30
	ctx.r[5].s64 = ctx.r[28].s64 + 48;
	// 82DDC0FC: 4BF7B205  bl 0x82d57300
	ctx.lr = 0x82DDC100;
	sub_82D57300(ctx, base);
	// 82DDC100: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDC104: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDC108: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDC10C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC110: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC114: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC118: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDC11C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDC120: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDC124: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDC128: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC12C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82DDC130: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DDC134: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC138: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC13C: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDC140: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC144: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC148: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDC14C: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDC150: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC154: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC158: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDC15C: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82DDC160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC164: 4E800421  bctrl
	ctx.lr = 0x82DDC168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC168: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DDC16C: 4BECD2E8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC170 size=124
    let mut pc: u32 = 0x82DDC170;
    'dispatch: loop {
        match pc {
            0x82DDC170 => {
    //   block [0x82DDC170..0x82DDC1EC)
	// 82DDC170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC174: 4BECD28D  bl 0x82ca9400
	ctx.lr = 0x82DDC178;
	sub_82CA93D0(ctx, base);
	// 82DDC178: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC17C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDC180: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC184: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDC188: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC18C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC190: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC194: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDC198: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC19C: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDC1A0: 4BF7B161  bl 0x82d57300
	ctx.lr = 0x82DDC1A4;
	sub_82D57300(ctx, base);
	// 82DDC1A4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDC1A8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDC1AC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDC1B0: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC1B4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC1B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDC1BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDC1C0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDC1C4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC1C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDC1CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC1D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDC1D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC1D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC1DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC1E0: 4E800421  bctrl
	ctx.lr = 0x82DDC1E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC1E4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDC1E8: 4BECD268  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC1F0 size=160
    let mut pc: u32 = 0x82DDC1F0;
    'dispatch: loop {
        match pc {
            0x82DDC1F0 => {
    //   block [0x82DDC1F0..0x82DDC290)
	// 82DDC1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC1F4: 4BECD211  bl 0x82ca9404
	ctx.lr = 0x82DDC1F8;
	sub_82CA93D0(ctx, base);
	// 82DDC1F8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC1FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC200: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDC204: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC208: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC20C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC210: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC214: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC218: 38BC0030  addi r5, r28, 0x30
	ctx.r[5].s64 = ctx.r[28].s64 + 48;
	// 82DDC21C: 4BF7B0E5  bl 0x82d57300
	ctx.lr = 0x82DDC220;
	sub_82D57300(ctx, base);
	// 82DDC220: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDC224: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDC228: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DDC22C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC230: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC234: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC238: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDC23C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDC240: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDC244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDC248: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC24C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82DDC250: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DDC254: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC258: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC25C: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDC260: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC264: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC268: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDC26C: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDC270: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDC274: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDC278: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDC27C: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82DDC280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC284: 4E800421  bctrl
	ctx.lr = 0x82DDC288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC288: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DDC28C: 4BECD1C8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC290 size=124
    let mut pc: u32 = 0x82DDC290;
    'dispatch: loop {
        match pc {
            0x82DDC290 => {
    //   block [0x82DDC290..0x82DDC30C)
	// 82DDC290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC294: 4BECD16D  bl 0x82ca9400
	ctx.lr = 0x82DDC298;
	sub_82CA93D0(ctx, base);
	// 82DDC298: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC29C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDC2A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC2A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDC2A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC2AC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC2B0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC2B4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDC2B8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC2BC: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDC2C0: 4BF7B041  bl 0x82d57300
	ctx.lr = 0x82DDC2C4;
	sub_82D57300(ctx, base);
	// 82DDC2C4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDC2C8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDC2CC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDC2D0: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC2D4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC2D8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDC2DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDC2E0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDC2E4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC2E8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDC2EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC2F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDC2F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC2F8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDC2FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC300: 4E800421  bctrl
	ctx.lr = 0x82DDC304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC304: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDC308: 4BECD148  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC310 size=276
    let mut pc: u32 = 0x82DDC310;
    'dispatch: loop {
        match pc {
            0x82DDC310 => {
    //   block [0x82DDC310..0x82DDC3CC)
	// 82DDC310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC314: 4BECD0E5  bl 0x82ca93f8
	ctx.lr = 0x82DDC318;
	sub_82CA93D0(ctx, base);
	// 82DDC318: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC31C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC320: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDC324: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC328: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDC32C: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDC330: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DDC334: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DDC338: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDC33C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DDC340: 4BF78F09  bl 0x82d55248
	ctx.lr = 0x82DDC344;
	sub_82D55248(ctx, base);
	// 82DDC344: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDC348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC34C: 396B3CF0  addi r11, r11, 0x3cf0
	ctx.r[11].s64 = ctx.r[11].s64 + 15600;
	// 82DDC350: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DDC354: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDC358: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC35C: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82DDC360: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDC364: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82DDC368: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DDC36C: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC370: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC374: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDC378: 835B0014  lwz r26, 0x14(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC37C: 4B4C17AD  bl 0x8229db28
	ctx.lr = 0x82DDC380;
	sub_8229DB28(ctx, base);
	// 82DDC380: 38BB0030  addi r5, r27, 0x30
	ctx.r[5].s64 = ctx.r[27].s64 + 48;
	// 82DDC384: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDC388: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC38C: 4BF7AF75  bl 0x82d57300
	ctx.lr = 0x82DDC390;
	sub_82D57300(ctx, base);
	// 82DDC390: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDC394: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC398: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDC39C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC3A0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82DDC3A4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDC3A8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDC3AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC3B0: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82DDC3B4: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82DDC3B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDC3BC: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC3C0: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC3C4: 409A0008  bne cr6, 0x82ddc3cc
	if !ctx.cr[6].eq {
	pc = 0x82DDC3CC; continue 'dispatch;
	}
	// 82DDC3C8: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x82DDC3CC; continue 'dispatch;
            }
            0x82DDC3CC => {
    //   block [0x82DDC3CC..0x82DDC424)
	// 82DDC3CC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC3D0: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82DDC3D4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DDC3D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDC3DC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DDC3E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDC3E4: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDC3E8: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDC3EC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDC3F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDC3F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDC3F8: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDC3FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC400: 4E800421  bctrl
	ctx.lr = 0x82DDC404;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC404: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDC408: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DDC40C: 396B3D38  addi r11, r11, 0x3d38
	ctx.r[11].s64 = ctx.r[11].s64 + 15672;
	// 82DDC410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDC414: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DDC418: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDC41C: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82DDC420: 4BECD028  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC428 size=204
    let mut pc: u32 = 0x82DDC428;
    'dispatch: loop {
        match pc {
            0x82DDC428 => {
    //   block [0x82DDC428..0x82DDC4F4)
	// 82DDC428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC430: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDC434: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDC438: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC43C: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDC440: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDC444: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDC448: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDC44C: 3908C310  addi r8, r8, -0x3cf0
	ctx.r[8].s64 = ctx.r[8].s64 + -15600;
	// 82DDC450: 3929C4F8  addi r9, r9, -0x3b08
	ctx.r[9].s64 = ctx.r[9].s64 + -15112;
	// 82DDC454: 394AC548  addi r10, r10, -0x3ab8
	ctx.r[10].s64 = ctx.r[10].s64 + -15032;
	// 82DDC458: 396BC598  addi r11, r11, -0x3a68
	ctx.r[11].s64 = ctx.r[11].s64 + -14952;
	// 82DDC45C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DDC460: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82DDC464: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDC468: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DDC46C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDC470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDC474: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDC478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC47C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDC480: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DDC484: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DDC488: 4BFE4CE1  bl 0x82dc1168
	ctx.lr = 0x82DDC48C;
	sub_82DC1168(ctx, base);
	// 82DDC48C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDC490: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DDC494: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDC498: 396BBFA8  addi r11, r11, -0x4058
	ctx.r[11].s64 = ctx.r[11].s64 + -16472;
	// 82DDC49C: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDC4A0: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDC4A4: 3908BBD8  addi r8, r8, -0x4428
	ctx.r[8].s64 = ctx.r[8].s64 + -17448;
	// 82DDC4A8: 3929C1F0  addi r9, r9, -0x3e10
	ctx.r[9].s64 = ctx.r[9].s64 + -15888;
	// 82DDC4AC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DDC4B0: 394AC0D0  addi r10, r10, -0x3f30
	ctx.r[10].s64 = ctx.r[10].s64 + -16176;
	// 82DDC4B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC4B8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DDC4BC: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 82DDC4C0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DDC4C4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DDC4C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DDC4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDC4D0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DDC4D4: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DDC4D8: 4BFE4C91  bl 0x82dc1168
	ctx.lr = 0x82DDC4DC;
	sub_82DC1168(ctx, base);
	// 82DDC4DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDC4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC4E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDC4EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDC4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC4F8 size=76
    let mut pc: u32 = 0x82DDC4F8;
    'dispatch: loop {
        match pc {
            0x82DDC4F8 => {
    //   block [0x82DDC4F8..0x82DDC544)
	// 82DDC4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC504: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDC508: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDC50C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDC510: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDC514: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DDC518: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DDC51C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC520: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDC524: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDC528: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDC52C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DDC530: 4BFFFCC1  bl 0x82ddc1f0
	ctx.lr = 0x82DDC534;
	sub_82DDC1F0(ctx, base);
	// 82DDC534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDC538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDC548 size=80
    let mut pc: u32 = 0x82DDC548;
    'dispatch: loop {
        match pc {
            0x82DDC548 => {
    //   block [0x82DDC548..0x82DDC598)
	// 82DDC548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC554: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDC558: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDC55C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DDC560: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DDC564: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDC568: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDC56C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDC570: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC574: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDC578: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDC57C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDC580: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDC584: 4BFFFB4D  bl 0x82ddc0d0
	ctx.lr = 0x82DDC588;
	sub_82DDC0D0(ctx, base);
	// 82DDC588: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDC58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC598 size=232
    let mut pc: u32 = 0x82DDC598;
    'dispatch: loop {
        match pc {
            0x82DDC598 => {
    //   block [0x82DDC598..0x82DDC5C4)
	// 82DDC598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC5A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDC5A4: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC5A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDC5AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDC5B0: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DDC5B4: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DDC5B8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DDC5BC: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DDC5C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DDC5C4; continue 'dispatch;
            }
            0x82DDC5C4 => {
    //   block [0x82DDC5C4..0x82DDC680)
	// 82DDC5C4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDC5C8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDC5CC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DDC5D0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDC5D4: 4200FFF0  bdnz 0x82ddc5c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DDC5C4; continue 'dispatch;
	}
	// 82DDC5D8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDC5DC: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DDC5E0: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DDC5E4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DDC5E8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDC5EC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC680 size=140
    let mut pc: u32 = 0x82DDC680;
    'dispatch: loop {
        match pc {
            0x82DDC680 => {
    //   block [0x82DDC680..0x82DDC70C)
	// 82DDC680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC684: 4BECCD7D  bl 0x82ca9400
	ctx.lr = 0x82DDC688;
	sub_82CA93D0(ctx, base);
	// 82DDC688: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC68C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDC690: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DDC694: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDC698: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DDC69C: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DDC6A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC6A4: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC6A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DDC6AC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDC6B0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC6B4: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDC6B8: 9B410054  stb r26, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u8 ) };
	// 82DDC6BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC6C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDC6C4: 4BF7AC3D  bl 0x82d57300
	ctx.lr = 0x82DDC6C8;
	sub_82D57300(ctx, base);
	// 82DDC6C8: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82DDC6CC: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82DDC6D0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDC6D4: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC6D8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC6DC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDC6E0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDC6E4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDC6E8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC6EC: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82DDC6F0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDC6F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC6F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC6FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC700: 4E800421  bctrl
	ctx.lr = 0x82DDC704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC704: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DDC708: 4BECCD48  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDC710 size=148
    let mut pc: u32 = 0x82DDC710;
    'dispatch: loop {
        match pc {
            0x82DDC710 => {
    //   block [0x82DDC710..0x82DDC7A4)
	// 82DDC710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC714: 4BECCCF1  bl 0x82ca9404
	ctx.lr = 0x82DDC718;
	sub_82CA93D0(ctx, base);
	// 82DDC718: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC71C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDC720: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DDC724: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDC728: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDC72C: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DDC730: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC734: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC738: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DDC73C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDC740: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC744: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDC748: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC74C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDC750: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC754: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDC758: 4BF7ABA9  bl 0x82d57300
	ctx.lr = 0x82DDC75C;
	sub_82D57300(ctx, base);
	// 82DDC75C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82DDC760: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82DDC764: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC768: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDC76C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC770: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDC774: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDC778: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDC77C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC780: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDC784: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DDC78C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC790: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDC794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC798: 4E800421  bctrl
	ctx.lr = 0x82DDC79C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC79C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDC7A0: 4BECCCB4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDC7A8 size=176
    let mut pc: u32 = 0x82DDC7A8;
    'dispatch: loop {
        match pc {
            0x82DDC7A8 => {
    //   block [0x82DDC7A8..0x82DDC828)
	// 82DDC7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC7B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDC7B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDC7B8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DDC7BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC7C0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DDC7C4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDC7C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDC7CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDC7D0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC7D4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DDC7D8: 4BFFF509  bl 0x82ddbce0
	ctx.lr = 0x82DDC7DC;
	sub_82DDBCE0(ctx, base);
	// 82DDC7DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC7E0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDC7E4: 40980044  bge cr6, 0x82ddc828
	if !ctx.cr[6].lt {
	pc = 0x82DDC828; continue 'dispatch;
	}
	// 82DDC7E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDC7EC: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DDC7F0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DDC828; continue 'dispatch;
            }
            0x82DDC828 => {
    //   block [0x82DDC828..0x82DDC858)
	// 82DDC828: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDC82C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDC830: 419A000C  beq cr6, 0x82ddc83c
	if ctx.cr[6].eq {
	pc = 0x82DDC83C; continue 'dispatch;
	}
	// 82DDC834: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DDC838: 4BFEE751  bl 0x82dcaf88
	ctx.lr = 0x82DDC83C;
	sub_82DCAF88(ctx, base);
	// 82DDC83C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDC840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDC844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDC848: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DDC84C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDC850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDC854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC858 size=124
    let mut pc: u32 = 0x82DDC858;
    'dispatch: loop {
        match pc {
            0x82DDC858 => {
    //   block [0x82DDC858..0x82DDC8D4)
	// 82DDC858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC85C: 4BECCBA5  bl 0x82ca9400
	ctx.lr = 0x82DDC860;
	sub_82CA93D0(ctx, base);
	// 82DDC860: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC864: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDC868: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC86C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDC870: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDC874: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDC878: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC87C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDC880: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDC884: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DDC888: 4BF7AA79  bl 0x82d57300
	ctx.lr = 0x82DDC88C;
	sub_82D57300(ctx, base);
	// 82DDC88C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DDC890: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDC894: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDC898: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC89C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDC8A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDC8A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDC8A8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDC8AC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDC8B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDC8B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDC8B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDC8BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDC8C0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDC8C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDC8C8: 4E800421  bctrl
	ctx.lr = 0x82DDC8CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDC8CC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDC8D0: 4BECCB80  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC8D8 size=240
    let mut pc: u32 = 0x82DDC8D8;
    'dispatch: loop {
        match pc {
            0x82DDC8D8 => {
    //   block [0x82DDC8D8..0x82DDC908)
	// 82DDC8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDC8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDC8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDC8E8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC8EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDC8F0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDC8F4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DDC8F8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DDC8FC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DDC900: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DDC904: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DDC908; continue 'dispatch;
            }
            0x82DDC908 => {
    //   block [0x82DDC908..0x82DDC9C8)
	// 82DDC908: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDC90C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDC910: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DDC914: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDC918: 4200FFF0  bdnz 0x82ddc908
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DDC908; continue 'dispatch;
	}
	// 82DDC91C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDC920: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DDC924: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DDC928: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDC92C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDC930: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDC9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDC9C8 size=124
    let mut pc: u32 = 0x82DDC9C8;
    'dispatch: loop {
        match pc {
            0x82DDC9C8 => {
    //   block [0x82DDC9C8..0x82DDC9F0)
	// 82DDC9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDC9CC: 4BECCA3D  bl 0x82ca9408
	ctx.lr = 0x82DDC9D0;
	sub_82CA93D0(ctx, base);
	// 82DDC9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDC9D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDC9D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDC9DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDC9E0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDC9E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDC9E8: 4099003C  ble cr6, 0x82ddca24
	if !ctx.cr[6].gt {
	pc = 0x82DDCA24; continue 'dispatch;
	}
	// 82DDC9EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DDC9F0; continue 'dispatch;
            }
            0x82DDC9F0 => {
    //   block [0x82DDC9F0..0x82DDCA24)
	// 82DDC9F0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDC9F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDC9F8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDC9FC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCA00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCA04: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDCA08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCA0C: 4E800421  bctrl
	ctx.lr = 0x82DDCA10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCA10: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCA14: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDCA18: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DDCA1C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDCA20: 4198FFD0  blt cr6, 0x82ddc9f0
	if ctx.cr[6].lt {
	pc = 0x82DDC9F0; continue 'dispatch;
	}
            }
            0x82DDCA24 => {
    //   block [0x82DDCA24..0x82DDCA44)
	// 82DDCA24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCA28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDCA2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDCA30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCA34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCA38: 4E800421  bctrl
	ctx.lr = 0x82DDCA3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDCA40: 4BECCA18  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCA48 size=100
    let mut pc: u32 = 0x82DDCA48;
    'dispatch: loop {
        match pc {
            0x82DDCA48 => {
    //   block [0x82DDCA48..0x82DDCA70)
	// 82DDCA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCA4C: 4BECC9BD  bl 0x82ca9408
	ctx.lr = 0x82DDCA50;
	sub_82CA93D0(ctx, base);
	// 82DDCA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCA54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDCA58: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDCA5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDCA60: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCA64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDCA68: 4099003C  ble cr6, 0x82ddcaa4
	if !ctx.cr[6].gt {
	pc = 0x82DDCAA4; continue 'dispatch;
	}
	// 82DDCA6C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DDCA70; continue 'dispatch;
            }
            0x82DDCA70 => {
    //   block [0x82DDCA70..0x82DDCAA4)
	// 82DDCA70: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCA74: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDCA78: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDCA7C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCA80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCA84: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDCA88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCA8C: 4E800421  bctrl
	ctx.lr = 0x82DDCA90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCA90: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCA94: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDCA98: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DDCA9C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDCAA0: 4198FFD0  blt cr6, 0x82ddca70
	if ctx.cr[6].lt {
	pc = 0x82DDCA70; continue 'dispatch;
	}
            }
            0x82DDCAA4 => {
    //   block [0x82DDCAA4..0x82DDCAAC)
	// 82DDCAA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDCAA8: 4BECC9B0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCAB0 size=132
    let mut pc: u32 = 0x82DDCAB0;
    'dispatch: loop {
        match pc {
            0x82DDCAB0 => {
    //   block [0x82DDCAB0..0x82DDCAE8)
	// 82DDCAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCAB4: 4BECC955  bl 0x82ca9408
	ctx.lr = 0x82DDCAB8;
	sub_82CA93D0(ctx, base);
	// 82DDCAB8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82DDCABC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DDCAC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCAC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDCAC8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DDCACC: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82DDCAD0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDCAD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDCAD8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCADC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDCAE0: 40990044  ble cr6, 0x82ddcb24
	if !ctx.cr[6].gt {
	pc = 0x82DDCB24; continue 'dispatch;
	}
	// 82DDCAE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DDCAE8; continue 'dispatch;
            }
            0x82DDCAE8 => {
    //   block [0x82DDCAE8..0x82DDCB24)
	// 82DDCAE8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCAEC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDCAF0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82DDCAF4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDCAF8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DDCAFC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCB00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCB04: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDCB08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCB0C: 4E800421  bctrl
	ctx.lr = 0x82DDCB10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCB10: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCB14: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDCB18: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DDCB1C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDCB20: 4198FFC8  blt cr6, 0x82ddcae8
	if ctx.cr[6].lt {
	pc = 0x82DDCAE8; continue 'dispatch;
	}
            }
            0x82DDCB24 => {
    //   block [0x82DDCB24..0x82DDCB34)
	// 82DDCB24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DDCB28: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DDCB2C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82DDCB30: 4BECC928  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCB38 size=292
    let mut pc: u32 = 0x82DDCB38;
    'dispatch: loop {
        match pc {
            0x82DDCB38 => {
    //   block [0x82DDCB38..0x82DDCB90)
	// 82DDCB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCB3C: 4BECC8B9  bl 0x82ca93f4
	ctx.lr = 0x82DDCB40;
	sub_82CA93D0(ctx, base);
	// 82DDCB40: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCB44: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCB48: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DDCB4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDCB50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDCB54: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDCB58: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDCB5C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCB60: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DDCB64: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCB68: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCB6C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCB70: 40980020  bge cr6, 0x82ddcb90
	if !ctx.cr[6].lt {
	pc = 0x82DDCB90; continue 'dispatch;
	}
	// 82DDCB74: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDCB78: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDCB7C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCB80: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCB84: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDCB88: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCB8C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCB90; continue 'dispatch;
            }
            0x82DDCB90 => {
    //   block [0x82DDCB90..0x82DDCBC8)
	// 82DDCB90: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCB94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCB98: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCB9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCBA0: 4E800421  bctrl
	ctx.lr = 0x82DDCBA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCBA4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCBA8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCBAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDCBB0: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82DDCBB4: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCBB8: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DDCBBC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDCBC0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDCBC4: 41980060  blt cr6, 0x82ddcc24
	if ctx.cr[6].lt {
	pc = 0x82DDCC24; continue 'dispatch;
	}
            }
            0x82DDCBC8 => {
    //   block [0x82DDCBC8..0x82DDCC24)
	// 82DDCBC8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCBCC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DDCBD0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCBD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDCBD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDCBDC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCBE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCBE4: 4E800421  bctrl
	ctx.lr = 0x82DDCBE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCBE8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DDCBEC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82DDCBF0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DDCBF4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCBF8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDCBFC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDCC00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDCC04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCC08: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDCC0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCC10: 4E800421  bctrl
	ctx.lr = 0x82DDCC14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCC14: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DDCC18: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DDCC1C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDCC20: 4098FFA8  bge cr6, 0x82ddcbc8
	if !ctx.cr[6].lt {
	pc = 0x82DDCBC8; continue 'dispatch;
	}
            }
            0x82DDCC24 => {
    //   block [0x82DDCC24..0x82DDCC54)
	// 82DDCC24: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCC28: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCC2C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCC30: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCC34: 40980020  bge cr6, 0x82ddcc54
	if !ctx.cr[6].lt {
	pc = 0x82DDCC54; continue 'dispatch;
	}
	// 82DDCC38: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDCC3C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDCC40: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCC44: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCC48: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDCC4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCC50: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCC54; continue 'dispatch;
            }
            0x82DDCC54 => {
    //   block [0x82DDCC54..0x82DDCC5C)
	// 82DDCC54: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82DDCC58: 4BECC7EC  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCC60 size=292
    let mut pc: u32 = 0x82DDCC60;
    'dispatch: loop {
        match pc {
            0x82DDCC60 => {
    //   block [0x82DDCC60..0x82DDCCB8)
	// 82DDCC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCC64: 4BECC791  bl 0x82ca93f4
	ctx.lr = 0x82DDCC68;
	sub_82CA93D0(ctx, base);
	// 82DDCC68: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCC6C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCC70: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DDCC74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDCC78: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDCC7C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDCC80: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDCC84: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCC88: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DDCC8C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCC90: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCC94: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCC98: 40980020  bge cr6, 0x82ddccb8
	if !ctx.cr[6].lt {
	pc = 0x82DDCCB8; continue 'dispatch;
	}
	// 82DDCC9C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDCCA0: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDCCA4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCCA8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCCAC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDCCB0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCCB4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCCB8; continue 'dispatch;
            }
            0x82DDCCB8 => {
    //   block [0x82DDCCB8..0x82DDCCF0)
	// 82DDCCB8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCCBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCCC0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCCC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCCC8: 4E800421  bctrl
	ctx.lr = 0x82DDCCCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCCCC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCCD0: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCCD4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDCCD8: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82DDCCDC: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCCE0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DDCCE4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDCCE8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDCCEC: 41980060  blt cr6, 0x82ddcd4c
	if ctx.cr[6].lt {
	pc = 0x82DDCD4C; continue 'dispatch;
	}
            }
            0x82DDCCF0 => {
    //   block [0x82DDCCF0..0x82DDCD4C)
	// 82DDCCF0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCCF4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DDCCF8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCCFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDCD00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDCD04: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCD08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCD0C: 4E800421  bctrl
	ctx.lr = 0x82DDCD10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCD10: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DDCD14: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82DDCD18: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DDCD1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCD20: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDCD24: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDCD28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDCD2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCD30: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCD34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCD38: 4E800421  bctrl
	ctx.lr = 0x82DDCD3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCD3C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DDCD40: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DDCD44: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDCD48: 4098FFA8  bge cr6, 0x82ddccf0
	if !ctx.cr[6].lt {
	pc = 0x82DDCCF0; continue 'dispatch;
	}
            }
            0x82DDCD4C => {
    //   block [0x82DDCD4C..0x82DDCD7C)
	// 82DDCD4C: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCD50: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCD54: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCD58: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCD5C: 40980020  bge cr6, 0x82ddcd7c
	if !ctx.cr[6].lt {
	pc = 0x82DDCD7C; continue 'dispatch;
	}
	// 82DDCD60: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDCD64: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDCD68: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCD6C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCD70: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDCD74: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCD78: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCD7C; continue 'dispatch;
            }
            0x82DDCD7C => {
    //   block [0x82DDCD7C..0x82DDCD84)
	// 82DDCD7C: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82DDCD80: 4BECC6C4  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCD88 size=408
    let mut pc: u32 = 0x82DDCD88;
    'dispatch: loop {
        match pc {
            0x82DDCD88 => {
    //   block [0x82DDCD88..0x82DDCDDC)
	// 82DDCD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCD8C: 4BECC669  bl 0x82ca93f4
	ctx.lr = 0x82DDCD90;
	sub_82CA93D0(ctx, base);
	// 82DDCD90: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCD94: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCD98: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DDCD9C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDCDA0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDCDA4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDCDA8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DDCDAC: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCDB0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCDB4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCDB8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCDBC: 40980020  bge cr6, 0x82ddcddc
	if !ctx.cr[6].lt {
	pc = 0x82DDCDDC; continue 'dispatch;
	}
	// 82DDCDC0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDCDC4: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDCDC8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCDCC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCDD0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDCDD4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCDD8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCDDC; continue 'dispatch;
            }
            0x82DDCDDC => {
    //   block [0x82DDCDDC..0x82DDCE24)
	// 82DDCDDC: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCDE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCDE4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCDE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCDEC: 4E800421  bctrl
	ctx.lr = 0x82DDCDF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCDF0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCDF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDCDF8: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82DDCDFC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDCE00: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE04: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCE08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE0C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCE10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCE14: 4E800421  bctrl
	ctx.lr = 0x82DDCE18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCE18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDCE1C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DDCE20: 419A00C8  beq cr6, 0x82ddcee8
	if ctx.cr[6].eq {
	pc = 0x82DDCEE8; continue 'dispatch;
	}
            }
            0x82DDCE24 => {
    //   block [0x82DDCE24..0x82DDCEC4)
	// 82DDCE24: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCE28: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82DDCE2C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DDCE30: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDCE34: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDCE38: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDCE3C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDCE44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCE48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCE4C: 4E800421  bctrl
	ctx.lr = 0x82DDCE50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCE50: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDCE58: 419A006C  beq cr6, 0x82ddcec4
	if ctx.cr[6].eq {
	pc = 0x82DDCEC4; continue 'dispatch;
	}
	// 82DDCE5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE60: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DDCE64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDCE68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDCE6C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCE70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCE74: 4E800421  bctrl
	ctx.lr = 0x82DDCE78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCE78: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DDCE7C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82DDCE80: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82DDCE84: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCE88: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDCE8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCE90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDCE94: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDCE98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDCE9C: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDCEA0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDCEA4: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DDCEA8: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDCEAC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDCEB0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDCEB4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDCEB8: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82DDCEBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCEC0: 4E800421  bctrl
	ctx.lr = 0x82DDCEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDCEC4 => {
    //   block [0x82DDCEC4..0x82DDCEE8)
	// 82DDCEC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCEC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDCECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDCED0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCED8: 4E800421  bctrl
	ctx.lr = 0x82DDCEDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCEDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDCEE0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DDCEE4: 409AFF40  bne cr6, 0x82ddce24
	if !ctx.cr[6].eq {
	pc = 0x82DDCE24; continue 'dispatch;
	}
            }
            0x82DDCEE8 => {
    //   block [0x82DDCEE8..0x82DDCF18)
	// 82DDCEE8: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDCEEC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCEF0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCEF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCEF8: 40980020  bge cr6, 0x82ddcf18
	if !ctx.cr[6].lt {
	pc = 0x82DDCF18; continue 'dispatch;
	}
	// 82DDCEFC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDCF00: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDCF04: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCF08: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCF0C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDCF10: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCF14: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCF18; continue 'dispatch;
            }
            0x82DDCF18 => {
    //   block [0x82DDCF18..0x82DDCF20)
	// 82DDCF18: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DDCF1C: 4BECC528  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDCF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDCF20 size=300
    let mut pc: u32 = 0x82DDCF20;
    'dispatch: loop {
        match pc {
            0x82DDCF20 => {
    //   block [0x82DDCF20..0x82DDCF7C)
	// 82DDCF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDCF24: 4BECC4CD  bl 0x82ca93f0
	ctx.lr = 0x82DDCF28;
	sub_82CA93D0(ctx, base);
	// 82DDCF28: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDCF2C: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCF30: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	// 82DDCF34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDCF38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDCF3C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDCF40: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDCF44: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DDCF48: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DDCF4C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82DDCF50: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCF54: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCF58: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDCF5C: 40980020  bge cr6, 0x82ddcf7c
	if !ctx.cr[6].lt {
	pc = 0x82DDCF7C; continue 'dispatch;
	}
	// 82DDCF60: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDCF64: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDCF68: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDCF6C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDCF70: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDCF74: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDCF78: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDCF7C; continue 'dispatch;
            }
            0x82DDCF7C => {
    //   block [0x82DDCF7C..0x82DDCFB4)
	// 82DDCF7C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCF80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCF84: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCF88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCF8C: 4E800421  bctrl
	ctx.lr = 0x82DDCF90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCF90: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDCF94: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDCF98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDCF9C: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82DDCFA0: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDCFA4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DDCFA8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDCFAC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDCFB0: 41980064  blt cr6, 0x82ddd014
	if ctx.cr[6].lt {
	pc = 0x82DDD014; continue 'dispatch;
	}
            }
            0x82DDCFB4 => {
    //   block [0x82DDCFB4..0x82DDD014)
	// 82DDCFB4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCFB8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DDCFBC: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCFC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DDCFC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDCFC8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCFCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDCFD0: 4E800421  bctrl
	ctx.lr = 0x82DDCFD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDCFD4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DDCFD8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82DDCFDC: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82DDCFE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDCFE4: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DDCFE8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDCFEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDCFF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDCFF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDCFF8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDCFFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD000: 4E800421  bctrl
	ctx.lr = 0x82DDD004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD004: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DDD008: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DDD00C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDD010: 4098FFA4  bge cr6, 0x82ddcfb4
	if !ctx.cr[6].lt {
	pc = 0x82DDCFB4; continue 'dispatch;
	}
            }
            0x82DDD014 => {
    //   block [0x82DDD014..0x82DDD044)
	// 82DDD014: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DDD018: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD01C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD020: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD024: 40980020  bge cr6, 0x82ddd044
	if !ctx.cr[6].lt {
	pc = 0x82DDD044; continue 'dispatch;
	}
	// 82DDD028: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDD02C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDD030: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD034: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD038: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDD03C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD040: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD044; continue 'dispatch;
            }
            0x82DDD044 => {
    //   block [0x82DDD044..0x82DDD04C)
	// 82DDD044: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DDD048: 4BECC3F8  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD050 size=416
    let mut pc: u32 = 0x82DDD050;
    'dispatch: loop {
        match pc {
            0x82DDD050 => {
    //   block [0x82DDD050..0x82DDD0A8)
	// 82DDD050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD054: 4BECC39D  bl 0x82ca93f0
	ctx.lr = 0x82DDD058;
	sub_82CA93D0(ctx, base);
	// 82DDD058: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD05C: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD060: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	// 82DDD064: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DDD068: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDD06C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDD070: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDD074: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DDD078: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DDD07C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD080: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD084: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD088: 40980020  bge cr6, 0x82ddd0a8
	if !ctx.cr[6].lt {
	pc = 0x82DDD0A8; continue 'dispatch;
	}
	// 82DDD08C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDD090: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDD094: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD098: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD09C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDD0A0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD0A4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD0A8; continue 'dispatch;
            }
            0x82DDD0A8 => {
    //   block [0x82DDD0A8..0x82DDD0F0)
	// 82DDD0A8: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD0AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD0B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD0B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD0B8: 4E800421  bctrl
	ctx.lr = 0x82DDD0BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD0BC: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD0C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD0C4: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82DDD0C8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDD0CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD0D0: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD0D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD0D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD0DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD0E0: 4E800421  bctrl
	ctx.lr = 0x82DDD0E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD0E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD0E8: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DDD0EC: 419A00CC  beq cr6, 0x82ddd1b8
	if ctx.cr[6].eq {
	pc = 0x82DDD1B8; continue 'dispatch;
	}
            }
            0x82DDD0F0 => {
    //   block [0x82DDD0F0..0x82DDD194)
	// 82DDD0F0: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD0F4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82DDD0F8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DDD0FC: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDD100: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDD104: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDD108: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD10C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDD110: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD118: 4E800421  bctrl
	ctx.lr = 0x82DDD11C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD11C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD124: 419A0070  beq cr6, 0x82ddd194
	if ctx.cr[6].eq {
	pc = 0x82DDD194; continue 'dispatch;
	}
	// 82DDD128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD12C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DDD130: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDD134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD138: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD13C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD140: 4E800421  bctrl
	ctx.lr = 0x82DDD144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD144: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DDD148: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82DDD14C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DDD150: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD154: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDD158: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD15C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDD160: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDD164: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD168: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDD16C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDD170: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDD174: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DDD178: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDD17C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDD180: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDD184: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDD188: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82DDD18C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD190: 4E800421  bctrl
	ctx.lr = 0x82DDD194;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDD194 => {
    //   block [0x82DDD194..0x82DDD1B8)
	// 82DDD194: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD198: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDD19C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD1A0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD1A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD1A8: 4E800421  bctrl
	ctx.lr = 0x82DDD1AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD1AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD1B0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DDD1B4: 409AFF3C  bne cr6, 0x82ddd0f0
	if !ctx.cr[6].eq {
	pc = 0x82DDD0F0; continue 'dispatch;
	}
            }
            0x82DDD1B8 => {
    //   block [0x82DDD1B8..0x82DDD1E8)
	// 82DDD1B8: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DDD1BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD1C0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD1C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD1C8: 40980020  bge cr6, 0x82ddd1e8
	if !ctx.cr[6].lt {
	pc = 0x82DDD1E8; continue 'dispatch;
	}
	// 82DDD1CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDD1D0: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDD1D4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD1D8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD1DC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDD1E0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD1E4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD1E8; continue 'dispatch;
            }
            0x82DDD1E8 => {
    //   block [0x82DDD1E8..0x82DDD1F0)
	// 82DDD1E8: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82DDD1EC: 4BECC254  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD1F0 size=304
    let mut pc: u32 = 0x82DDD1F0;
    'dispatch: loop {
        match pc {
            0x82DDD1F0 => {
    //   block [0x82DDD1F0..0x82DDD248)
	// 82DDD1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD1F4: 4BECC201  bl 0x82ca93f4
	ctx.lr = 0x82DDD1F8;
	sub_82CA93D0(ctx, base);
	// 82DDD1F8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD1FC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD200: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DDD204: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DDD20C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DDD210: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DDD214: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDD218: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82DDD21C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD220: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD224: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD228: 40980020  bge cr6, 0x82ddd248
	if !ctx.cr[6].lt {
	pc = 0x82DDD248; continue 'dispatch;
	}
	// 82DDD22C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDD230: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDD234: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD238: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD23C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDD240: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD244: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD248; continue 'dispatch;
            }
            0x82DDD248 => {
    //   block [0x82DDD248..0x82DDD280)
	// 82DDD248: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD24C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD250: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD258: 4E800421  bctrl
	ctx.lr = 0x82DDD25C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD25C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD260: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD264: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDD268: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82DDD26C: 83DE000C  lwz r30, 0xc(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD270: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DDD274: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DDD278: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDD27C: 4198006C  blt cr6, 0x82ddd2e8
	if ctx.cr[6].lt {
	pc = 0x82DDD2E8; continue 'dispatch;
	}
            }
            0x82DDD280 => {
    //   block [0x82DDD280..0x82DDD2E8)
	// 82DDD280: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD284: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DDD288: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD28C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDD290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD294: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD29C: 4E800421  bctrl
	ctx.lr = 0x82DDD2A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD2A0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DDD2A4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DDD2A8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DDD2AC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD2B0: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82DDD2B4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DDD2B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDD2BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD2C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD2C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD2C8: 4E800421  bctrl
	ctx.lr = 0x82DDD2CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD2CC: 897C0004  lbz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD2D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD2D4: 409A0014  bne cr6, 0x82ddd2e8
	if !ctx.cr[6].eq {
	pc = 0x82DDD2E8; continue 'dispatch;
	}
	// 82DDD2D8: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82DDD2DC: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DDD2E0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DDD2E4: 4098FF9C  bge cr6, 0x82ddd280
	if !ctx.cr[6].lt {
	pc = 0x82DDD280; continue 'dispatch;
	}
            }
            0x82DDD2E8 => {
    //   block [0x82DDD2E8..0x82DDD318)
	// 82DDD2E8: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDD2EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD2F0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD2F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD2F8: 40980020  bge cr6, 0x82ddd318
	if !ctx.cr[6].lt {
	pc = 0x82DDD318; continue 'dispatch;
	}
	// 82DDD2FC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDD300: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDD304: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD308: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD30C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDD310: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD314: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD318; continue 'dispatch;
            }
            0x82DDD318 => {
    //   block [0x82DDD318..0x82DDD320)
	// 82DDD318: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82DDD31C: 4BECC128  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD320 size=420
    let mut pc: u32 = 0x82DDD320;
    'dispatch: loop {
        match pc {
            0x82DDD320 => {
    //   block [0x82DDD320..0x82DDD374)
	// 82DDD320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD324: 4BECC0D1  bl 0x82ca93f4
	ctx.lr = 0x82DDD328;
	sub_82CA93D0(ctx, base);
	// 82DDD328: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD32C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD330: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DDD334: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DDD338: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDD33C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDD340: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDD344: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDD348: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD34C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD350: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD354: 40980020  bge cr6, 0x82ddd374
	if !ctx.cr[6].lt {
	pc = 0x82DDD374; continue 'dispatch;
	}
	// 82DDD358: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDD35C: 39293D70  addi r9, r9, 0x3d70
	ctx.r[9].s64 = ctx.r[9].s64 + 15728;
	// 82DDD360: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD364: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD368: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DDD36C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD370: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD374; continue 'dispatch;
            }
            0x82DDD374 => {
    //   block [0x82DDD374..0x82DDD3BC)
	// 82DDD374: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD378: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD37C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD384: 4E800421  bctrl
	ctx.lr = 0x82DDD388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD388: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD390: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82DDD394: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDD398: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD39C: 834B000C  lwz r26, 0xc(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD3A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD3A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD3A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD3AC: 4E800421  bctrl
	ctx.lr = 0x82DDD3B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD3B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD3B4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DDD3B8: 419A00D4  beq cr6, 0x82ddd48c
	if ctx.cr[6].eq {
	pc = 0x82DDD48C; continue 'dispatch;
	}
            }
            0x82DDD3BC => {
    //   block [0x82DDD3BC..0x82DDD468)
	// 82DDD3BC: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD3C0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82DDD3C4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DDD3C8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DDD3CC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDD3D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDD3D4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD3D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDD3DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD3E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD3E4: 4E800421  bctrl
	ctx.lr = 0x82DDD3E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD3E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD3EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD3F0: 419A0078  beq cr6, 0x82ddd468
	if ctx.cr[6].eq {
	pc = 0x82DDD468; continue 'dispatch;
	}
	// 82DDD3F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD3F8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DDD3FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDD400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD404: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD40C: 4E800421  bctrl
	ctx.lr = 0x82DDD410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD410: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DDD414: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82DDD418: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDD41C: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD420: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDD424: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD428: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD42C: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDD430: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDD434: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDD438: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDD43C: 7D4AD0AE  lbzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DDD440: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDD444: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDD448: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDD44C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDD450: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82DDD454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD458: 4E800421  bctrl
	ctx.lr = 0x82DDD45C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD45C: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD464: 409A0028  bne cr6, 0x82ddd48c
	if !ctx.cr[6].eq {
	pc = 0x82DDD48C; continue 'dispatch;
	}
            }
            0x82DDD468 => {
    //   block [0x82DDD468..0x82DDD48C)
	// 82DDD468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD46C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDD470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD474: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD47C: 4E800421  bctrl
	ctx.lr = 0x82DDD480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD484: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82DDD488: 409AFF34  bne cr6, 0x82ddd3bc
	if !ctx.cr[6].eq {
	pc = 0x82DDD3BC; continue 'dispatch;
	}
            }
            0x82DDD48C => {
    //   block [0x82DDD48C..0x82DDD4BC)
	// 82DDD48C: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDD490: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD494: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD498: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDD49C: 40980020  bge cr6, 0x82ddd4bc
	if !ctx.cr[6].lt {
	pc = 0x82DDD4BC; continue 'dispatch;
	}
	// 82DDD4A0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDD4A4: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DDD4A8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDD4AC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDD4B0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDD4B4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD4B8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDD4BC; continue 'dispatch;
            }
            0x82DDD4BC => {
    //   block [0x82DDD4BC..0x82DDD4C4)
	// 82DDD4BC: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DDD4C0: 4BECBF84  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD4C8 size=632
    let mut pc: u32 = 0x82DDD4C8;
    'dispatch: loop {
        match pc {
            0x82DDD4C8 => {
    //   block [0x82DDD4C8..0x82DDD544)
	// 82DDD4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD4CC: 4BECBF25  bl 0x82ca93f0
	ctx.lr = 0x82DDD4D0;
	sub_82CA93D0(ctx, base);
	// 82DDD4D0: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD4D4: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82DDD4D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDD4DC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDD4E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDD4E4: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DDD4E8: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD4EC: 80770000  lwz r3, 0(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD4F0: 92E1006C  stw r23, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[23].u32 ) };
	// 82DDD4F4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDD4F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD4FC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD504: 4E800421  bctrl
	ctx.lr = 0x82DDD508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD508: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DDD50C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD510: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD518: 4E800421  bctrl
	ctx.lr = 0x82DDD51C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD51C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD520: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD524: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DDD528: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD52C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD530: 4E800421  bctrl
	ctx.lr = 0x82DDD534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD534: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDD538: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDD53C: 409901FC  ble cr6, 0x82ddd738
	if !ctx.cr[6].gt {
	pc = 0x82DDD738; continue 'dispatch;
	}
	// 82DDD540: 7FF6FB78  mr r22, r31
	ctx.r[22].u64 = ctx.r[31].u64;
            }
            0x82DDD544 => {
    //   block [0x82DDD544..0x82DDD558)
	// 82DDD544: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD548: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDD54C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDD550: 40990024  ble cr6, 0x82ddd574
	if !ctx.cr[6].gt {
	pc = 0x82DDD574; continue 'dispatch;
	}
	// 82DDD554: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	pc = 0x82DDD558; continue 'dispatch;
            }
            0x82DDD558 => {
    //   block [0x82DDD558..0x82DDD574)
	// 82DDD558: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD55C: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DDD560: 419A0018  beq cr6, 0x82ddd578
	if ctx.cr[6].eq {
	pc = 0x82DDD578; continue 'dispatch;
	}
	// 82DDD564: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDD568: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDD56C: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DDD570: 4198FFE8  blt cr6, 0x82ddd558
	if ctx.cr[6].lt {
	pc = 0x82DDD558; continue 'dispatch;
	}
	pc = 0x82DDD574; continue 'dispatch;
            }
            0x82DDD574 => {
    //   block [0x82DDD574..0x82DDD578)
	// 82DDD574: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	pc = 0x82DDD578; continue 'dispatch;
            }
            0x82DDD578 => {
    //   block [0x82DDD578..0x82DDD600)
	// 82DDD578: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD57C: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82DDD580: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82DDD584: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82DDD588: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDD58C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDD590: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDD598: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD59C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD5A0: 4E800421  bctrl
	ctx.lr = 0x82DDD5A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD5A4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD5A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD5AC: 419A010C  beq cr6, 0x82ddd6b8
	if ctx.cr[6].eq {
	pc = 0x82DDD6B8; continue 'dispatch;
	}
	// 82DDD5B0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD5B4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DDD5B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DDD5BC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DDD5C0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD5C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD5C8: 4E800421  bctrl
	ctx.lr = 0x82DDD5CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD5CC: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DDD5D0: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82DDD5D4: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DDD5D8: 409A00AC  bne cr6, 0x82ddd684
	if !ctx.cr[6].eq {
	pc = 0x82DDD684; continue 'dispatch;
	}
	// 82DDD5DC: 3BFD000C  addi r31, r29, 0xc
	ctx.r[31].s64 = ctx.r[29].s64 + 12;
	// 82DDD5E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD5E4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD5E8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DDD5EC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDD5F0: 409A0010  bne cr6, 0x82ddd600
	if !ctx.cr[6].eq {
	pc = 0x82DDD600; continue 'dispatch;
	}
	// 82DDD5F4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82DDD5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD5FC: 4BF7999D  bl 0x82d56f98
	ctx.lr = 0x82DDD600;
	sub_82D56F98(ctx, base);
            }
            0x82DDD600 => {
    //   block [0x82DDD600..0x82DDD648)
	// 82DDD600: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD604: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD608: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDD60C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DDD610: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDD614: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DDD618: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82DDD61C: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD620: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD624: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDD628: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDD62C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD630: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD634: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82DDD638: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD63C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD640: 409A0008  bne cr6, 0x82ddd648
	if !ctx.cr[6].eq {
	pc = 0x82DDD648; continue 'dispatch;
	}
	// 82DDD644: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x82DDD648; continue 'dispatch;
            }
            0x82DDD648 => {
    //   block [0x82DDD648..0x82DDD684)
	// 82DDD648: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD64C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DDD650: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DDD654: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DDD658: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDD65C: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDD660: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDD664: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDD668: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD66C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDD670: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDD674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD678: 4E800421  bctrl
	ctx.lr = 0x82DDD67C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD67C: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DDD680: 48000090  b 0x82ddd710
	pc = 0x82DDD710; continue 'dispatch;
            }
            0x82DDD684 => {
    //   block [0x82DDD684..0x82DDD6B8)
	// 82DDD684: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD688: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD68C: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDD690: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDD694: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDD698: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DDD69C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDD6A0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD6A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD6A8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDD6AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD6B0: 4E800421  bctrl
	ctx.lr = 0x82DDD6B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD6B4: 4800005C  b 0x82ddd710
	pc = 0x82DDD710; continue 'dispatch;
            }
            0x82DDD6B8 => {
    //   block [0x82DDD6B8..0x82DDD710)
	// 82DDD6B8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DDD6BC: 419A0054  beq cr6, 0x82ddd710
	if ctx.cr[6].eq {
	pc = 0x82DDD710; continue 'dispatch;
	}
	// 82DDD6C0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD6C4: 57FF1838  slwi r31, r31, 3
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DDD6C8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82DDD6CC: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82DDD6D0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD6D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD6D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDD6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD6E0: 4E800421  bctrl
	ctx.lr = 0x82DDD6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD6E4: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD6E8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD6EC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DDD6F0: 7D0BFA14  add r8, r11, r31
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DDD6F4: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DDD6F8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DDD6FC: 915D0010  stw r10, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDD700: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD704: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDD708: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD70C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
            }
            0x82DDD710 => {
    //   block [0x82DDD710..0x82DDD738)
	// 82DDD710: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD714: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DDD718: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DDD71C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD724: 4E800421  bctrl
	ctx.lr = 0x82DDD728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD728: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 82DDD72C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDD730: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82DDD734: 409AFE10  bne cr6, 0x82ddd544
	if !ctx.cr[6].eq {
	pc = 0x82DDD544; continue 'dispatch;
	}
            }
            0x82DDD738 => {
    //   block [0x82DDD738..0x82DDD740)
	// 82DDD738: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82DDD73C: 4BECBD04  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD740 size=500
    let mut pc: u32 = 0x82DDD740;
    'dispatch: loop {
        match pc {
            0x82DDD740 => {
    //   block [0x82DDD740..0x82DDD7F0)
	// 82DDD740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD744: 4BECBCAD  bl 0x82ca93f0
	ctx.lr = 0x82DDD748;
	sub_82CA93D0(ctx, base);
	// 82DDD748: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD74C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDD750: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DDD754: 396B3D88  addi r11, r11, 0x3d88
	ctx.r[11].s64 = ctx.r[11].s64 + 15752;
	// 82DDD758: 3BB7000C  addi r29, r23, 0xc
	ctx.r[29].s64 = ctx.r[23].s64 + 12;
	// 82DDD75C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DDD760: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DDD764: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDD768: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDD76C: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 82DDD770: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DDD774: 61080004  ori r8, r8, 4
	ctx.r[8].u64 = ctx.r[8].u64 | 4;
	// 82DDD778: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DDD77C: 93370008  stw r25, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DDD780: B1570006  sth r10, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DDD784: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDD788: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDD78C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DDD790: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD794: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DDD798: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD79C: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD7A0: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82DDD7A4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DDD7A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD7AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD7B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD7B4: 4E800421  bctrl
	ctx.lr = 0x82DDD7B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD7B8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DDD7BC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD7C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD7C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD7C8: 4E800421  bctrl
	ctx.lr = 0x82DDD7CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD7CC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD7D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD7D4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DDD7D8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DDD7DC: 40980024  bge cr6, 0x82ddd800
	if !ctx.cr[6].lt {
	pc = 0x82DDD800; continue 'dispatch;
	}
	// 82DDD7E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD7E4: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDD7E8: 41980008  blt cr6, 0x82ddd7f0
	if ctx.cr[6].lt {
	pc = 0x82DDD7F0; continue 'dispatch;
	}
	// 82DDD7EC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
            }
            0x82DDD7F0 => {
    //   block [0x82DDD7F0..0x82DDD800)
	// 82DDD7F0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DDD7F4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDD7F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDD7FC: 4BF79715  bl 0x82d56f10
	ctx.lr = 0x82DDD800;
	sub_82D56F10(ctx, base);
	pc = 0x82DDD800; continue 'dispatch;
            }
            0x82DDD800 => {
    //   block [0x82DDD800..0x82DDD824)
	// 82DDD800: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD804: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD808: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD80C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD810: 4E800421  bctrl
	ctx.lr = 0x82DDD814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD814: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDD818: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DDD81C: 4099010C  ble cr6, 0x82ddd928
	if !ctx.cr[6].gt {
	pc = 0x82DDD928; continue 'dispatch;
	}
	// 82DDD820: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
            }
            0x82DDD824 => {
    //   block [0x82DDD824..0x82DDD8C0)
	// 82DDD824: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD828: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DDD82C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD830: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD834: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDD838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD83C: 4E800421  bctrl
	ctx.lr = 0x82DDD840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD840: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDD844: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DDD848: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82DDD84C: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82DDD850: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DDD854: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDD858: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DDD85C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD860: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDD864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDD868: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD86C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD870: 4E800421  bctrl
	ctx.lr = 0x82DDD874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD874: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDD87C: 419A0084  beq cr6, 0x82ddd900
	if ctx.cr[6].eq {
	pc = 0x82DDD900; continue 'dispatch;
	}
	// 82DDD880: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDD884: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD888: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82DDD88C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD890: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDD894: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DDD898: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDD89C: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD8A0: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDD8A4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD8A8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDD8AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD8B0: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82DDD8B4: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD8B8: 409A0008  bne cr6, 0x82ddd8c0
	if !ctx.cr[6].eq {
	pc = 0x82DDD8C0; continue 'dispatch;
	}
	// 82DDD8BC: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
            }
            0x82DDD8C0 => {
    //   block [0x82DDD8C0..0x82DDD900)
	// 82DDD8C0: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD8C4: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82DDD8C8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DDD8CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDD8D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DDD8D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDD8D8: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDD8DC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDD8E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDD8E4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDD8E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDD8EC: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDD8F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD8F4: 4E800421  bctrl
	ctx.lr = 0x82DDD8F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD8F8: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DDD8FC: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
            }
            0x82DDD900 => {
    //   block [0x82DDD900..0x82DDD928)
	// 82DDD900: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD904: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDD908: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DDD90C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDD910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDD914: 4E800421  bctrl
	ctx.lr = 0x82DDD918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDD918: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 82DDD91C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDD920: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82DDD924: 409AFF00  bne cr6, 0x82ddd824
	if !ctx.cr[6].eq {
	pc = 0x82DDD824; continue 'dispatch;
	}
            }
            0x82DDD928 => {
    //   block [0x82DDD928..0x82DDD934)
	// 82DDD928: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DDD92C: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82DDD930: 4BECBB10  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD938 size=88
    let mut pc: u32 = 0x82DDD938;
    'dispatch: loop {
        match pc {
            0x82DDD938 => {
    //   block [0x82DDD938..0x82DDD990)
	// 82DDD938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD93C: 4BECBACD  bl 0x82ca9408
	ctx.lr = 0x82DDD940;
	sub_82CA93D0(ctx, base);
	// 82DDD940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD944: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD948: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDD94C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDD950: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDD954: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDD958: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DDD95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD960: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDD964: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDD968: 4BF778E1  bl 0x82d55248
	ctx.lr = 0x82DDD96C;
	sub_82D55248(ctx, base);
	// 82DDD96C: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DDD970: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DDD974: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDD978: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDD97C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDD980: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDD984: 4BFFFDBD  bl 0x82ddd740
	ctx.lr = 0x82DDD988;
	sub_82DDD740(ctx, base);
	// 82DDD988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDD98C: 4BECBACC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDD990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDD990 size=108
    let mut pc: u32 = 0x82DDD990;
    'dispatch: loop {
        match pc {
            0x82DDD990 => {
    //   block [0x82DDD990..0x82DDD9FC)
	// 82DDD990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDD994: 4BECBA71  bl 0x82ca9404
	ctx.lr = 0x82DDD998;
	sub_82CA93D0(ctx, base);
	// 82DDD998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDD99C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDD9A0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDD9A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDD9A8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDD9AC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDD9B0: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DDD9B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDD9B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDD9BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDD9C0: 4BF77889  bl 0x82d55248
	ctx.lr = 0x82DDD9C4;
	sub_82D55248(ctx, base);
	// 82DDD9C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDD9C8: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DDD9CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DDD9D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDD9D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDD9D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDD9DC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDD9E0: 4BFFFD61  bl 0x82ddd740
	ctx.lr = 0x82DDD9E4;
	sub_82DDD740(ctx, base);
	// 82DDD9E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDD9E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDD9EC: 396B3304  addi r11, r11, 0x3304
	ctx.r[11].s64 = ctx.r[11].s64 + 13060;
	// 82DDD9F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDD9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDD9F8: 4BECBA5C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDA00 size=204
    let mut pc: u32 = 0x82DDDA00;
    'dispatch: loop {
        match pc {
            0x82DDDA00 => {
    //   block [0x82DDDA00..0x82DDDACC)
	// 82DDDA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDDA08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDDA0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDDA10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDA14: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDDA18: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DDDA1C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DDDA20: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DDDA24: 3908D990  addi r8, r8, -0x2670
	ctx.r[8].s64 = ctx.r[8].s64 + -9840;
	// 82DDDA28: 39290D80  addi r9, r9, 0xd80
	ctx.r[9].s64 = ctx.r[9].s64 + 3456;
	// 82DDDA2C: 394A0DD0  addi r10, r10, 0xdd0
	ctx.r[10].s64 = ctx.r[10].s64 + 3536;
	// 82DDDA30: 396B0E20  addi r11, r11, 0xe20
	ctx.r[11].s64 = ctx.r[11].s64 + 3616;
	// 82DDDA34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DDDA38: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82DDDA3C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDDA40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DDDA44: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDDA48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDDA4C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDDA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDA54: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDDA58: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DDDA5C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DDDA60: 4BFE3709  bl 0x82dc1168
	ctx.lr = 0x82DDDA64;
	sub_82DC1168(ctx, base);
	// 82DDDA64: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDDA68: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DDDA6C: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDDA70: 396BD050  addi r11, r11, -0x2fb0
	ctx.r[11].s64 = ctx.r[11].s64 + -12208;
	// 82DDDA74: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDDA78: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDDA7C: 3908D938  addi r8, r8, -0x26c8
	ctx.r[8].s64 = ctx.r[8].s64 + -9928;
	// 82DDDA80: 3929D320  addi r9, r9, -0x2ce0
	ctx.r[9].s64 = ctx.r[9].s64 + -11488;
	// 82DDDA84: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DDDA88: 394ACD88  addi r10, r10, -0x3278
	ctx.r[10].s64 = ctx.r[10].s64 + -12920;
	// 82DDDA8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDDA90: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DDDA94: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DDDA98: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DDDA9C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DDDAA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DDDAA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDDAA8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DDDAAC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DDDAB0: 4BFE36B9  bl 0x82dc1168
	ctx.lr = 0x82DDDAB4;
	sub_82DC1168(ctx, base);
	// 82DDDAB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDDAB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDDABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDDAC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDDAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDDAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDDAD0 size=124
    let mut pc: u32 = 0x82DDDAD0;
    'dispatch: loop {
        match pc {
            0x82DDDAD0 => {
    //   block [0x82DDDAD0..0x82DDDAF8)
	// 82DDDAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDAD4: 4BECB935  bl 0x82ca9408
	ctx.lr = 0x82DDDAD8;
	sub_82CA93D0(ctx, base);
	// 82DDDAD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDADC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DDDAE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DDDAE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDDAE8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDDAEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DDDAF0: 4099003C  ble cr6, 0x82dddb2c
	if !ctx.cr[6].gt {
	pc = 0x82DDDB2C; continue 'dispatch;
	}
	// 82DDDAF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DDDAF8; continue 'dispatch;
            }
            0x82DDDAF8 => {
    //   block [0x82DDDAF8..0x82DDDB2C)
	// 82DDDAF8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDAFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DDDB00: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDDB04: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDDB08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDB0C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDDB10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDDB14: 4E800421  bctrl
	ctx.lr = 0x82DDDB18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDDB18: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDDB1C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DDDB20: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DDDB24: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDDB28: 4198FFD0  blt cr6, 0x82dddaf8
	if ctx.cr[6].lt {
	pc = 0x82DDDAF8; continue 'dispatch;
	}
            }
            0x82DDDB2C => {
    //   block [0x82DDDB2C..0x82DDDB4C)
	// 82DDDB2C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDB30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDDB34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDDB38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDB3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDDB40: 4E800421  bctrl
	ctx.lr = 0x82DDDB44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDDB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDDB48: 4BECB910  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDDB50 size=308
    let mut pc: u32 = 0x82DDDB50;
    'dispatch: loop {
        match pc {
            0x82DDDB50 => {
    //   block [0x82DDDB50..0x82DDDC7C)
	// 82DDDB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDB54: 4BECB89D  bl 0x82ca93f0
	ctx.lr = 0x82DDDB58;
	sub_82CA93D0(ctx, base);
	// 82DDDB58: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDB5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDDB60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDB64: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDDB68: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDDB6C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDDB70: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDB74: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDDB78: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDDB7C: 4B4BFFAD  bl 0x8229db28
	ctx.lr = 0x82DDDB80;
	sub_8229DB28(ctx, base);
	// 82DDDB80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDDB84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDDB88: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDDB8C: 4BFD835D  bl 0x82db5ee8
	ctx.lr = 0x82DDDB90;
	sub_82DB5EE8(ctx, base);
	// 82DDDB90: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDDB94: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDB98: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82DDDB9C: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DDDBA0: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DDDBA4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDDBA8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDDBAC: 419800D0  blt cr6, 0x82dddc7c
	if ctx.cr[6].lt {
	pc = 0x82DDDC7C; continue 'dispatch;
	}
	// 82DDDBB0: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82DDDBB4: 3AE00040  li r23, 0x40
	ctx.r[23].s64 = 64;
	// 82DDDBB8: 3B000050  li r24, 0x50
	ctx.r[24].s64 = 80;
	// 82DDDBBC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDBC0: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DDDBC4: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82DDDBC8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DDDBCC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDDBD0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	pc = 0x82DDDC7C; continue 'dispatch;
            }
            0x82DDDC7C => {
    //   block [0x82DDDC7C..0x82DDDC84)
	// 82DDDC7C: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82DDDC80: 4BECB7C0  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDDC88 size=268
    let mut pc: u32 = 0x82DDDC88;
    'dispatch: loop {
        match pc {
            0x82DDDC88 => {
    //   block [0x82DDDC88..0x82DDDD8C)
	// 82DDDC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDC8C: 4BECB76D  bl 0x82ca93f8
	ctx.lr = 0x82DDDC90;
	sub_82CA93D0(ctx, base);
	// 82DDDC90: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDC94: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDDC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDC9C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDDCA0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDDCA4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDDCA8: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDCAC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDDCB0: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDDCB4: 4B4BFE75  bl 0x8229db28
	ctx.lr = 0x82DDDCB8;
	sub_8229DB28(ctx, base);
	// 82DDDCB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDDCBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDDCC0: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDDCC4: 4BFD8225  bl 0x82db5ee8
	ctx.lr = 0x82DDDCC8;
	sub_82DB5EE8(ctx, base);
	// 82DDDCC8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDDCCC: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDCD0: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82DDDCD4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DDDCD8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DDDCDC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDDCE0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDDCE4: 419800A8  blt cr6, 0x82dddd8c
	if ctx.cr[6].lt {
	pc = 0x82DDDD8C; continue 'dispatch;
	}
	// 82DDDCE8: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82DDDCEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDCF0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DDDCF4: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82DDDCF8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DDDCFC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DDDD00: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x82DDDD8C; continue 'dispatch;
            }
            0x82DDDD8C => {
    //   block [0x82DDDD8C..0x82DDDD94)
	// 82DDDD8C: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82DDDD90: 4BECB6B8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDDD98 size=292
    let mut pc: u32 = 0x82DDDD98;
    'dispatch: loop {
        match pc {
            0x82DDDD98 => {
    //   block [0x82DDDD98..0x82DDDEB4)
	// 82DDDD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDD9C: 4BECB659  bl 0x82ca93f4
	ctx.lr = 0x82DDDDA0;
	sub_82CA93D0(ctx, base);
	// 82DDDDA0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDDA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDDDA8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDDDAC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DDDDB0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDDDB4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DDDDB8: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDDBC: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDDDC0: 4B4BFD69  bl 0x8229db28
	ctx.lr = 0x82DDDDC4;
	sub_8229DB28(ctx, base);
	// 82DDDDC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDDDC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDDDCC: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDDDD0: 4BFD8119  bl 0x82db5ee8
	ctx.lr = 0x82DDDDD4;
	sub_82DB5EE8(ctx, base);
	// 82DDDDD4: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DDDDD8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDDDC: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82DDDDE0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDDDE4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDDDE8: 815B0010  lwz r10, 0x10(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDDDEC: 830B000C  lwz r24, 0xc(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDDF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDDDF4: 409900C0  ble cr6, 0x82dddeb4
	if !ctx.cr[6].gt {
	pc = 0x82DDDEB4; continue 'dispatch;
	}
	// 82DDDDF8: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82DDDDFC: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82DDDE00: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82DDDEB4; continue 'dispatch;
            }
            0x82DDDEB4 => {
    //   block [0x82DDDEB4..0x82DDDEBC)
	// 82DDDEB4: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82DDDEB8: 4BECB58C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDDEC0 size=272
    let mut pc: u32 = 0x82DDDEC0;
    'dispatch: loop {
        match pc {
            0x82DDDEC0 => {
    //   block [0x82DDDEC0..0x82DDDFC8)
	// 82DDDEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDEC4: 4BECB531  bl 0x82ca93f4
	ctx.lr = 0x82DDDEC8;
	sub_82CA93D0(ctx, base);
	// 82DDDEC8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDECC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDDED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDDED4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDDED8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDDEDC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDDEE0: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDEE4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DDDEE8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDDEEC: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82DDDEF0: 4B4BFC39  bl 0x8229db28
	ctx.lr = 0x82DDDEF4;
	sub_8229DB28(ctx, base);
	// 82DDDEF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDDEF8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDDEFC: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDDF00: 4BFD7FE9  bl 0x82db5ee8
	ctx.lr = 0x82DDDF04;
	sub_82DB5EE8(ctx, base);
	// 82DDDF04: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDDF08: 831F000C  lwz r24, 0xc(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDDF0C: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82DDDF10: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DDDF14: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DDDF18: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DDDF1C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDDF20: 419800A8  blt cr6, 0x82dddfc8
	if ctx.cr[6].lt {
	pc = 0x82DDDFC8; continue 'dispatch;
	}
	// 82DDDF24: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82DDDF28: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82DDDF2C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDDF30: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DDDF34: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82DDDFC8; continue 'dispatch;
            }
            0x82DDDFC8 => {
    //   block [0x82DDDFC8..0x82DDDFD0)
	// 82DDDFC8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82DDDFCC: 4BECB478  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDDFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDDFD0 size=300
    let mut pc: u32 = 0x82DDDFD0;
    'dispatch: loop {
        match pc {
            0x82DDDFD0 => {
    //   block [0x82DDDFD0..0x82DDE0F4)
	// 82DDDFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDDFD4: 4BECB41D  bl 0x82ca93f0
	ctx.lr = 0x82DDDFD8;
	sub_82CA93D0(ctx, base);
	// 82DDDFD8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDDFDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DDDFE0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDDFE4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DDDFE8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDDFEC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DDDFF0: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDDFF4: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DDDFF8: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDDFFC: 4B4BFB2D  bl 0x8229db28
	ctx.lr = 0x82DDE000;
	sub_8229DB28(ctx, base);
	// 82DDE000: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDE004: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDE008: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDE00C: 4BFD7EDD  bl 0x82db5ee8
	ctx.lr = 0x82DDE010;
	sub_82DB5EE8(ctx, base);
	// 82DDE010: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DDE014: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE018: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82DDE01C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDE020: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE024: 815B0010  lwz r10, 0x10(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDE028: 82EB000C  lwz r23, 0xc(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDE02C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDE030: 409900C4  ble cr6, 0x82dde0f4
	if !ctx.cr[6].gt {
	pc = 0x82DDE0F4; continue 'dispatch;
	}
	// 82DDE034: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82DDE038: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82DDE03C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82DDE0F4; continue 'dispatch;
            }
            0x82DDE0F4 => {
    //   block [0x82DDE0F4..0x82DDE0FC)
	// 82DDE0F4: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82DDE0F8: 4BECB348  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDE100 size=280
    let mut pc: u32 = 0x82DDE100;
    'dispatch: loop {
        match pc {
            0x82DDE100 => {
    //   block [0x82DDE100..0x82DDE210)
	// 82DDE100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE104: 4BECB2F5  bl 0x82ca93f8
	ctx.lr = 0x82DDE108;
	sub_82CA93D0(ctx, base);
	// 82DDE108: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE10C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE114: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDE118: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DDE11C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDE120: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE124: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82DDE128: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDE12C: 4B4BF9FD  bl 0x8229db28
	ctx.lr = 0x82DDE130;
	sub_8229DB28(ctx, base);
	// 82DDE130: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDE134: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDE138: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDE13C: 4BFD7DAD  bl 0x82db5ee8
	ctx.lr = 0x82DDE140;
	sub_82DB5EE8(ctx, base);
	// 82DDE140: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDE144: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDE148: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 82DDE14C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82DDE150: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DDE154: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DDE158: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDE15C: 419800B4  blt cr6, 0x82dde210
	if ctx.cr[6].lt {
	pc = 0x82DDE210; continue 'dispatch;
	}
	// 82DDE160: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82DDE164: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE168: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DDE16C: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82DDE170: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DDE174: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DDE178: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x82DDE210; continue 'dispatch;
            }
            0x82DDE210 => {
    //   block [0x82DDE210..0x82DDE218)
	// 82DDE210: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82DDE214: 4BECB234  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDE218 size=304
    let mut pc: u32 = 0x82DDE218;
    'dispatch: loop {
        match pc {
            0x82DDE218 => {
    //   block [0x82DDE218..0x82DDE340)
	// 82DDE218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE21C: 4BECB1D9  bl 0x82ca93f4
	ctx.lr = 0x82DDE220;
	sub_82CA93D0(ctx, base);
	// 82DDE220: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE224: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DDE228: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDE22C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DDE230: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDE234: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DDE238: 82FB0000  lwz r23, 0(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE23C: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDE240: 4B4BF8E9  bl 0x8229db28
	ctx.lr = 0x82DDE244;
	sub_8229DB28(ctx, base);
	// 82DDE244: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDE248: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDE24C: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDE250: 4BFD7C99  bl 0x82db5ee8
	ctx.lr = 0x82DDE254;
	sub_82DB5EE8(ctx, base);
	// 82DDE254: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DDE258: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE25C: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82DDE260: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DDE264: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE268: 81570010  lwz r10, 0x10(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDE26C: 832B000C  lwz r25, 0xc(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDE270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDE274: 409900CC  ble cr6, 0x82dde340
	if !ctx.cr[6].gt {
	pc = 0x82DDE340; continue 'dispatch;
	}
	// 82DDE278: 3BF70020  addi r31, r23, 0x20
	ctx.r[31].s64 = ctx.r[23].s64 + 32;
	// 82DDE27C: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82DDE280: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82DDE340; continue 'dispatch;
            }
            0x82DDE340 => {
    //   block [0x82DDE340..0x82DDE348)
	// 82DDE340: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82DDE344: 4BECB100  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDE348 size=488
    let mut pc: u32 = 0x82DDE348;
    'dispatch: loop {
        match pc {
            0x82DDE348 => {
    //   block [0x82DDE348..0x82DDE3C8)
	// 82DDE348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE34C: 4BECB099  bl 0x82ca93e4
	ctx.lr = 0x82DDE350;
	sub_82CA93D0(ctx, base);
	// 82DDE350: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE354: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDE358: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82DDE35C: 396B3DC4  addi r11, r11, 0x3dc4
	ctx.r[11].s64 = ctx.r[11].s64 + 15812;
	// 82DDE360: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DDE364: 3BB3000C  addi r29, r19, 0xc
	ctx.r[29].s64 = ctx.r[19].s64 + 12;
	// 82DDE368: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DDE36C: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DDE370: 91730000  stw r11, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE374: 61290004  ori r9, r9, 4
	ctx.r[9].u64 = ctx.r[9].u64 | 4;
	// 82DDE378: B1530006  sth r10, 6(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DDE37C: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 82DDE380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDE384: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DDE388: 92F30008  stw r23, 8(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82DDE38C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DDE390: 913D0008  stw r9, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DDE394: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDE398: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE39C: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDE3A0: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DDE3A4: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE3A8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DDE3AC: 831F0010  lwz r24, 0x10(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDE3B0: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82DDE3B4: 40980024  bge cr6, 0x82dde3d8
	if !ctx.cr[6].lt {
	pc = 0x82DDE3D8; continue 'dispatch;
	}
	// 82DDE3B8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDE3BC: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DDE3C0: 41980008  blt cr6, 0x82dde3c8
	if ctx.cr[6].lt {
	pc = 0x82DDE3C8; continue 'dispatch;
	}
	// 82DDE3C4: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	pc = 0x82DDE3C8; continue 'dispatch;
            }
            0x82DDE3C8 => {
    //   block [0x82DDE3C8..0x82DDE3D8)
	// 82DDE3C8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DDE3CC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDE3D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DDE3D4: 4BF78B3D  bl 0x82d56f10
	ctx.lr = 0x82DDE3D8;
	sub_82D56F10(ctx, base);
	pc = 0x82DDE3D8; continue 'dispatch;
            }
            0x82DDE3D8 => {
    //   block [0x82DDE3D8..0x82DDE524)
	// 82DDE3D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDE3DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDE3E0: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DDE3E4: 4BFD7B05  bl 0x82db5ee8
	ctx.lr = 0x82DDE3E8;
	sub_82DB5EE8(ctx, base);
	// 82DDE3E8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DDE3EC: 80990008  lwz r4, 8(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDE3F0: 4B4BF739  bl 0x8229db28
	ctx.lr = 0x82DDE3F4;
	sub_8229DB28(ctx, base);
	// 82DDE3F4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82DDE3F8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82DDE3FC: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82DDE400: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DDE404: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82DDE408: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDE40C: 40990118  ble cr6, 0x82dde524
	if !ctx.cr[6].gt {
	pc = 0x82DDE524; continue 'dispatch;
	}
	// 82DDE410: 3A800030  li r20, 0x30
	ctx.r[20].s64 = 48;
	// 82DDE414: 3AA00040  li r21, 0x40
	ctx.r[21].s64 = 64;
	// 82DDE418: 3AC00050  li r22, 0x50
	ctx.r[22].s64 = 80;
	// 82DDE41C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82DDE524; continue 'dispatch;
            }
            0x82DDE524 => {
    //   block [0x82DDE524..0x82DDE530)
	// 82DDE524: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82DDE528: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82DDE52C: 4BECAF08  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE530 size=88
    let mut pc: u32 = 0x82DDE530;
    'dispatch: loop {
        match pc {
            0x82DDE530 => {
    //   block [0x82DDE530..0x82DDE588)
	// 82DDE530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE534: 4BECAED5  bl 0x82ca9408
	ctx.lr = 0x82DDE538;
	sub_82CA93D0(ctx, base);
	// 82DDE538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE53C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE540: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDE544: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE548: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDE54C: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDE550: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DDE554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE558: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE55C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDE560: 4BF76CE9  bl 0x82d55248
	ctx.lr = 0x82DDE564;
	sub_82D55248(ctx, base);
	// 82DDE564: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DDE568: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DDE56C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDE570: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDE574: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDE578: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDE57C: 4BFFFDCD  bl 0x82dde348
	ctx.lr = 0x82DDE580;
	sub_82DDE348(ctx, base);
	// 82DDE580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDE584: 4BECAED4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE588 size=108
    let mut pc: u32 = 0x82DDE588;
    'dispatch: loop {
        match pc {
            0x82DDE588 => {
    //   block [0x82DDE588..0x82DDE5F4)
	// 82DDE588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE58C: 4BECAE79  bl 0x82ca9404
	ctx.lr = 0x82DDE590;
	sub_82CA93D0(ctx, base);
	// 82DDE590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE594: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE598: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDE59C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDE5A0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDE5A4: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDE5A8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DDE5AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDE5B0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDE5B4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDE5B8: 4BF76C91  bl 0x82d55248
	ctx.lr = 0x82DDE5BC;
	sub_82D55248(ctx, base);
	// 82DDE5BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE5C0: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DDE5C4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DDE5C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDE5CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDE5D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDE5D4: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDE5D8: 4BFFFD71  bl 0x82dde348
	ctx.lr = 0x82DDE5DC;
	sub_82DDE348(ctx, base);
	// 82DDE5DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDE5E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE5E4: 396B3E00  addi r11, r11, 0x3e00
	ctx.r[11].s64 = ctx.r[11].s64 + 15872;
	// 82DDE5E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE5EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDE5F0: 4BECAE64  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE5F8 size=204
    let mut pc: u32 = 0x82DDE5F8;
    'dispatch: loop {
        match pc {
            0x82DDE5F8 => {
    //   block [0x82DDE5F8..0x82DDE6C4)
	// 82DDE5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE608: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE60C: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDE610: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDE614: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDE618: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDE61C: 3908E588  addi r8, r8, -0x1a78
	ctx.r[8].s64 = ctx.r[8].s64 + -6776;
	// 82DDE620: 3929E6C8  addi r9, r9, -0x1938
	ctx.r[9].s64 = ctx.r[9].s64 + -6456;
	// 82DDE624: 394AE718  addi r10, r10, -0x18e8
	ctx.r[10].s64 = ctx.r[10].s64 + -6376;
	// 82DDE628: 396BE768  addi r11, r11, -0x1898
	ctx.r[11].s64 = ctx.r[11].s64 + -6296;
	// 82DDE62C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DDE630: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 82DDE634: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDE638: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DDE63C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDE640: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDE644: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE64C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDE650: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DDE654: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DDE658: 4BFE2B11  bl 0x82dc1168
	ctx.lr = 0x82DDE65C;
	sub_82DC1168(ctx, base);
	// 82DDE65C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDE660: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DDE664: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDE668: 396BDFD0  addi r11, r11, -0x2030
	ctx.r[11].s64 = ctx.r[11].s64 + -8240;
	// 82DDE66C: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDE670: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDE674: 3908E530  addi r8, r8, -0x1ad0
	ctx.r[8].s64 = ctx.r[8].s64 + -6864;
	// 82DDE678: 3929E218  addi r9, r9, -0x1de8
	ctx.r[9].s64 = ctx.r[9].s64 + -7656;
	// 82DDE67C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DDE680: 394ADD98  addi r10, r10, -0x2268
	ctx.r[10].s64 = ctx.r[10].s64 + -8808;
	// 82DDE684: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDE688: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DDE68C: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 82DDE690: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DDE694: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DDE698: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DDE69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDE6A0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DDE6A4: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DDE6A8: 4BFE2AC1  bl 0x82dc1168
	ctx.lr = 0x82DDE6AC;
	sub_82DC1168(ctx, base);
	// 82DDE6AC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDE6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE6B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE6BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE6C8 size=76
    let mut pc: u32 = 0x82DDE6C8;
    'dispatch: loop {
        match pc {
            0x82DDE6C8 => {
    //   block [0x82DDE6C8..0x82DDE714)
	// 82DDE6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE6D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE6D8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDE6DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDE6E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDE6E4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DDE6E8: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DDE6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDE6F0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDE6F4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE6F8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDE6FC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DDE700: 4BFFFB19  bl 0x82dde218
	ctx.lr = 0x82DDE704;
	sub_82DDE218(ctx, base);
	// 82DDE704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDE718 size=80
    let mut pc: u32 = 0x82DDE718;
    'dispatch: loop {
        match pc {
            0x82DDE718 => {
    //   block [0x82DDE718..0x82DDE768)
	// 82DDE718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE724: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDE728: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDE72C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DDE730: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DDE734: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDE738: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDE73C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDE740: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDE744: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDE748: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDE74C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDE750: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE754: 4BFFF645  bl 0x82dddd98
	ctx.lr = 0x82DDE758;
	sub_82DDDD98(ctx, base);
	// 82DDE758: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE768 size=232
    let mut pc: u32 = 0x82DDE768;
    'dispatch: loop {
        match pc {
            0x82DDE768 => {
    //   block [0x82DDE768..0x82DDE794)
	// 82DDE768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE774: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE77C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDE780: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DDE784: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DDE788: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DDE78C: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DDE790: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DDE794; continue 'dispatch;
            }
            0x82DDE794 => {
    //   block [0x82DDE794..0x82DDE850)
	// 82DDE794: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDE798: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDE79C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DDE7A0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDE7A4: 4200FFF0  bdnz 0x82dde794
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DDE794; continue 'dispatch;
	}
	// 82DDE7A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDE7AC: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DDE7B0: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DDE7B4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DDE7B8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDE7BC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE850 size=76
    let mut pc: u32 = 0x82DDE850;
    'dispatch: loop {
        match pc {
            0x82DDE850 => {
    //   block [0x82DDE850..0x82DDE89C)
	// 82DDE850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE85C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDE860: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDE864: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDE868: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDE86C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DDE870: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DDE874: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDE878: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDE87C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE880: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDE884: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DDE888: 4BFFF879  bl 0x82dde100
	ctx.lr = 0x82DDE88C;
	sub_82DDE100(ctx, base);
	// 82DDE88C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDE8A0 size=80
    let mut pc: u32 = 0x82DDE8A0;
    'dispatch: loop {
        match pc {
            0x82DDE8A0 => {
    //   block [0x82DDE8A0..0x82DDE8F0)
	// 82DDE8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE8A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE8AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDE8B0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDE8B4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DDE8B8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DDE8BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDE8C0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDE8C4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDE8C8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDE8CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDE8D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDE8D4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDE8D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDE8DC: 4BFFF3AD  bl 0x82dddc88
	ctx.lr = 0x82DDE8E0;
	sub_82DDDC88(ctx, base);
	// 82DDE8E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDE8F0 size=176
    let mut pc: u32 = 0x82DDE8F0;
    'dispatch: loop {
        match pc {
            0x82DDE8F0 => {
    //   block [0x82DDE8F0..0x82DDE970)
	// 82DDE8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE8F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE8FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE900: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DDE904: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE908: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DDE90C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDE910: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDE914: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDE918: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE91C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DDE920: 4BFFF231  bl 0x82dddb50
	ctx.lr = 0x82DDE924;
	sub_82DDDB50(ctx, base);
	// 82DDE924: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE928: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDE92C: 40980044  bge cr6, 0x82dde970
	if !ctx.cr[6].lt {
	pc = 0x82DDE970; continue 'dispatch;
	}
	// 82DDE930: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDE934: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DDE938: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DDE970; continue 'dispatch;
            }
            0x82DDE970 => {
    //   block [0x82DDE970..0x82DDE9A0)
	// 82DDE970: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDE974: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDE978: 419A000C  beq cr6, 0x82dde984
	if ctx.cr[6].eq {
	pc = 0x82DDE984; continue 'dispatch;
	}
	// 82DDE97C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DDE980: 4BFEC609  bl 0x82dcaf88
	ctx.lr = 0x82DDE984;
	sub_82DCAF88(ctx, base);
	// 82DDE984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDE988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDE98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDE990: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DDE994: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDE998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDE99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDE9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDE9A0 size=152
    let mut pc: u32 = 0x82DDE9A0;
    'dispatch: loop {
        match pc {
            0x82DDE9A0 => {
    //   block [0x82DDE9A0..0x82DDE9E8)
	// 82DDE9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDE9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDE9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDE9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDE9B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDE9B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDE9B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDE9BC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDE9C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DDE9C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DDE9C8: 409A0020  bne cr6, 0x82dde9e8
	if !ctx.cr[6].eq {
	pc = 0x82DDE9E8; continue 'dispatch;
	}
	// 82DDE9CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDE9D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DDE9D4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DDE9D8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDE9DC: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DDE9E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DDE9E4: 4BF768E5  bl 0x82d552c8
	ctx.lr = 0x82DDE9E8;
	sub_82D552C8(ctx, base);
	pc = 0x82DDE9E8; continue 'dispatch;
            }
            0x82DDE9E8 => {
    //   block [0x82DDE9E8..0x82DDEA1C)
	// 82DDE9E8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DDE9EC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DDE9F0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DDE9F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDE9F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDE9FC: 419A0020  beq cr6, 0x82ddea1c
	if ctx.cr[6].eq {
	pc = 0x82DDEA1C; continue 'dispatch;
	}
	// 82DDEA00: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEA04: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDEA08: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82DDEA0C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDEA10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDEA14: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDEA18: 4BF768B1  bl 0x82d552c8
	ctx.lr = 0x82DDEA1C;
	sub_82D552C8(ctx, base);
	pc = 0x82DDEA1C; continue 'dispatch;
            }
            0x82DDEA1C => {
    //   block [0x82DDEA1C..0x82DDEA38)
	// 82DDEA1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDEA20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDEA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDEA2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDEA30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDEA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEA38 size=240
    let mut pc: u32 = 0x82DDEA38;
    'dispatch: loop {
        match pc {
            0x82DDEA38 => {
    //   block [0x82DDEA38..0x82DDEA68)
	// 82DDEA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDEA40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDEA44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDEA48: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEA4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDEA50: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDEA54: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DDEA58: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DDEA5C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DDEA60: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DDEA64: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DDEA68; continue 'dispatch;
            }
            0x82DDEA68 => {
    //   block [0x82DDEA68..0x82DDEB28)
	// 82DDEA68: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDEA6C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDEA70: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DDEA74: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDEA78: 4200FFF0  bdnz 0x82ddea68
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DDEA68; continue 'dispatch;
	}
	// 82DDEA7C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDEA80: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DDEA84: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DDEA88: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDEA8C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDEA90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEB28 size=136
    let mut pc: u32 = 0x82DDEB28;
    'dispatch: loop {
        match pc {
            0x82DDEB28 => {
    //   block [0x82DDEB28..0x82DDEB80)
	// 82DDEB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDEB30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDEB34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDEB38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEB3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEB40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDEB44: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEB48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEB4C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDEB50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEB54: 4E800421  bctrl
	ctx.lr = 0x82DDEB58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEB58: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEB5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DDEB60: 419A0020  beq cr6, 0x82ddeb80
	if ctx.cr[6].eq {
	pc = 0x82DDEB80; continue 'dispatch;
	}
	// 82DDEB64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEB68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDEB6C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDEB70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEB74: 4E800421  bctrl
	ctx.lr = 0x82DDEB78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDEB7C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
            }
            0x82DDEB80 => {
    //   block [0x82DDEB80..0x82DDEBB0)
	// 82DDEB80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEB84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DDEB88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDEB8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEB90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEB94: 4E800421  bctrl
	ctx.lr = 0x82DDEB98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEB98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDEBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDEBA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDEBA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDEBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


