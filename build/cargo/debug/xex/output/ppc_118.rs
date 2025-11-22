pub fn sub_82DC5B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5B98 size=440
	// 82DC5B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5B9C: 4BEE3869  bl 0x82ca9404
	ctx.lr = 0x82DC5BA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DC5BA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5BA4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5BA8: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DC5BAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC5BB0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DC5BB4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DC5BB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5BBC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5BC0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC5BC4: 40980020  bge cr6, 0x82dc5be4
	if !ctx.cr[6].lt {
	pc = 0x82DC5BE4; continue 'dispatch;
	}
	// 82DC5BC8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC5BCC: 39293114  addi r9, r9, 0x3114
	ctx.r[9].s64 = ctx.r[9].s64 + 12564;
	// 82DC5BD0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC5BD4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC5BD8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC5BDC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC5BE0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC5BE4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DC5BE8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DC5BEC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DC5BF0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC5BF4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC5BF8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC5BFC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DC5C00: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC5C04: 4200FFF0  bdnz 0x82dc5bf4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC5BF4; continue 'dispatch;
	}
	// 82DC5C08: 3BE40030  addi r31, r4, 0x30
	ctx.r[31].s64 = ctx.r[4].s64 + 48;
}

pub fn sub_82DC5D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5D50 size=12
	// 82DC5D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5D54: 4BEE36B1  bl 0x82ca9404
	ctx.lr = 0x82DC5D58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DC5D58: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DC5ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5ED0 size=152
	// 82DC5ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5ED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC5EDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5EE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5EE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC5EE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC5EEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC5EF0: 388B3124  addi r4, r11, 0x3124
	ctx.r[4].s64 = ctx.r[11].s64 + 12580;
	// 82DC5EF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC5EF8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5EFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC5F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5F04: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5F0C: 4E800421  bctrl
	ctx.lr = 0x82DC5F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5F10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5F14: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DC5F18: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC5F1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC5F20: 388BB3B4  addi r4, r11, -0x4c4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19532;
	// 82DC5F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5F28: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5F30: 4E800421  bctrl
	ctx.lr = 0x82DC5F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5F34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5F3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC5F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5F44: 4E800421  bctrl
	ctx.lr = 0x82DC5F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC5F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC5F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC5F54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC5F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5F5C: 4E800020  blr
	return;
	// 82DC5F60: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DC5F64: 4E800020  blr
	return;
}

pub fn sub_82DC5F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5F68 size=272
	// 82DC5F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5F70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC5F7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC5F80: 396B0E48  addi r11, r11, 0xe48
	ctx.r[11].s64 = ctx.r[11].s64 + 3656;
	// 82DC5F84: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC5F88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC5F8C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5F90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC5F94: 419A0030  beq cr6, 0x82dc5fc4
	if ctx.cr[6].eq {
	pc = 0x82DC5FC4; continue 'dispatch;
	}
	// 82DC5F98: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC5F9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC5FA0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC5FA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC5FA8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC5FAC: 409A0018  bne cr6, 0x82dc5fc4
	if !ctx.cr[6].eq {
	pc = 0x82DC5FC4; continue 'dispatch;
	}
	// 82DC5FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5FB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC5FB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5FC0: 4E800421  bctrl
	ctx.lr = 0x82DC5FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5FC4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82DC5FC8: 4BFEBB79  bl 0x82db1b40
	ctx.lr = 0x82DC5FCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DB1B40);
	// 82DC5FCC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC5FD0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DC5FD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC5FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC5FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC5FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC5FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5FE8: 4E800020  blr
	return;
}

pub fn sub_82DC6078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6078 size=216
	// 82DC6078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC607C: 4BEE338D  bl 0x82ca9408
	ctx.lr = 0x82DC6080;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DC6080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6084: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6088: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DC608C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC6090: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DC6094: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DC6098: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC609C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC60A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC60A4: 40980020  bge cr6, 0x82dc60c4
	if !ctx.cr[6].lt {
	pc = 0x82DC60C4; continue 'dispatch;
	}
	// 82DC60A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC60AC: 3929312C  addi r9, r9, 0x312c
	ctx.r[9].s64 = ctx.r[9].s64 + 12588;
	// 82DC60B0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC60B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC60B8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC60BC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC60C0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC60C4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC60C8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DC60CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC60D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC60D4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC60D8: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC60DC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC60E0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC60E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC60E8: 4E800421  bctrl
	ctx.lr = 0x82DC60EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC60EC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC60F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC60F4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC60F8: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC60FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC6100: 419A0014  beq cr6, 0x82dc6114
	if ctx.cr[6].eq {
	pc = 0x82DC6114; continue 'dispatch;
	}
	// 82DC6104: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC6108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DC610C: 556B103A  slwi r11, r11, 2
	// 82DC6110: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82DC6114: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DC6118: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC611C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC6120: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC6124: 40980020  bge cr6, 0x82dc6144
	if !ctx.cr[6].lt {
	pc = 0x82DC6144; continue 'dispatch;
	}
	// 82DC6128: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DC612C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DC6130: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC6134: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC6138: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DC613C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC6140: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC6144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC6148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC614C: 4BEE330C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DC6150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6150 size=208
	// 82DC6150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC6158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC615C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC6160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6164: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6168: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DC616C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC6170: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6174: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC6178: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC617C: 40980020  bge cr6, 0x82dc619c
	if !ctx.cr[6].lt {
	pc = 0x82DC619C; continue 'dispatch;
	}
	// 82DC6180: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC6184: 3929312C  addi r9, r9, 0x312c
	ctx.r[9].s64 = ctx.r[9].s64 + 12588;
	// 82DC6188: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC618C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC6190: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC6194: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC6198: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC619C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC61A0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC61A4: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82DC61A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DC61AC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DC61B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC61B4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DC61B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DC61BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC61C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC61C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC61C8: 4E800421  bctrl
	ctx.lr = 0x82DC61CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC61CC: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC61D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC61D4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC61D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC61DC: 40980020  bge cr6, 0x82dc61fc
	if !ctx.cr[6].lt {
	pc = 0x82DC61FC; continue 'dispatch;
	}
	// 82DC61E0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DC61E4: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DC61E8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC61EC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC61F0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC61F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC61F8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC61FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC6200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC6204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC6208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC620C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC6210: 4E800020  blr
	return;
}

pub fn sub_82DC6220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6220 size=160
	// 82DC6220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6224: 4BEE31DD  bl 0x82ca9400
	ctx.lr = 0x82DC6228;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 82DC6228: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC622C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC6230: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DC6234: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6238: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC623C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC6240: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82DC6244: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DC6248: 4198004C  blt cr6, 0x82dc6294
	if ctx.cr[6].lt {
	pc = 0x82DC6294; continue 'dispatch;
	}
	// 82DC624C: 577F103A  slwi r31, r27, 2
	// 82DC6250: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC6254: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6258: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC625C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC6260: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC6264: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82DC6268: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC626C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6274: 4E800421  bctrl
	ctx.lr = 0x82DC6278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6278: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC627C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6280: 419A0028  beq cr6, 0x82dc62a8
	if ctx.cr[6].eq {
	pc = 0x82DC62A8; continue 'dispatch;
	}
	// 82DC6284: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82DC6288: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82DC628C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DC6290: 4098FFC0  bge cr6, 0x82dc6250
	if !ctx.cr[6].lt {
	pc = 0x82DC6250; continue 'dispatch;
	}
	// 82DC6294: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6298: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC629C: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC62A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC62A4: 4BEE31AC  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
	// 82DC62A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC62AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC62B0: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC62B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC62B8: 4BEE3198  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_82DC62C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC62C0 size=184
	// 82DC62C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC62C4: 4BEE3131  bl 0x82ca93f4
	ctx.lr = 0x82DC62C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
	// 82DC62C8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC62CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC62D0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DC62D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC62D8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC62DC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DC62E0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC62E4: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DC62E8: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DC62EC: 3B0BFFFF  addi r24, r11, -1
	ctx.r[24].s64 = ctx.r[11].s64 + -1;
	// 82DC62F0: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82DC62F4: 41980058  blt cr6, 0x82dc634c
	if ctx.cr[6].lt {
	pc = 0x82DC634C; continue 'dispatch;
	}
	// 82DC62F8: 571E103A  slwi r30, r24, 2
	// 82DC62FC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC6300: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82DC6304: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DC6308: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DC630C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6310: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC6314: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC6318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC631C: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82DC6320: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6324: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC632C: 4E800421  bctrl
	ctx.lr = 0x82DC6330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6330: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6338: 419A0028  beq cr6, 0x82dc6360
	if ctx.cr[6].eq {
	pc = 0x82DC6360; continue 'dispatch;
	}
	// 82DC633C: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 82DC6340: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DC6344: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82DC6348: 4098FFB4  bge cr6, 0x82dc62fc
	if !ctx.cr[6].lt {
	pc = 0x82DC62FC; continue 'dispatch;
	}
	// 82DC634C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6350: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DC6354: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6358: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DC635C: 4BEE30E8  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
	// 82DC6360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC6364: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DC6368: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC636C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DC6370: 4BEE30D4  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
}

pub fn sub_82DC6378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6378 size=176
	// 82DC6378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC637C: 4BEE307D  bl 0x82ca93f8
	ctx.lr = 0x82DC6380;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 82DC6380: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6384: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC6388: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DC638C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6390: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC6394: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DC6398: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC639C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DC63A0: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 82DC63A4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82DC63A8: 41980054  blt cr6, 0x82dc63fc
	if ctx.cr[6].lt {
	pc = 0x82DC63FC; continue 'dispatch;
	}
	// 82DC63AC: 573F103A  slwi r31, r25, 2
	// 82DC63B0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC63B4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DC63B8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DC63BC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC63C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC63C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC63C8: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC63CC: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82DC63D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC63D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC63D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC63DC: 4E800421  bctrl
	ctx.lr = 0x82DC63E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC63E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC63E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC63E8: 419A0028  beq cr6, 0x82dc6410
	if ctx.cr[6].eq {
	pc = 0x82DC6410; continue 'dispatch;
	}
	// 82DC63EC: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 82DC63F0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82DC63F4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82DC63F8: 4098FFB8  bge cr6, 0x82dc63b0
	if !ctx.cr[6].lt {
	pc = 0x82DC63B0; continue 'dispatch;
	}
	// 82DC63FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6400: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DC6404: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6408: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC640C: 4BEE303C  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
	// 82DC6410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC6414: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DC6418: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC641C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC6420: 4BEE3028  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
}

pub fn sub_82DC6428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6428 size=200
	// 82DC6428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC642C: 4BEE2FC1  bl 0x82ca93ec
	ctx.lr = 0x82DC6430;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93EC);
	// 82DC6430: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC6438: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82DC643C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6440: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC6444: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DC6448: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC644C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DC6450: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DC6454: 3ACBFFFF  addi r22, r11, -1
	ctx.r[22].s64 = ctx.r[11].s64 + -1;
	// 82DC6458: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 82DC645C: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82DC6460: 41980064  blt cr6, 0x82dc64c4
	if ctx.cr[6].lt {
	pc = 0x82DC64C4; continue 'dispatch;
	}
	// 82DC6464: 82E10124  lwz r23, 0x124(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82DC6468: 56DE103A  slwi r30, r22, 2
	// 82DC646C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC6470: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82DC6474: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82DC6478: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82DC647C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DC6480: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DC6484: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6488: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC648C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC6490: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DC6494: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82DC6498: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC649C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC64A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC64A4: 4E800421  bctrl
	ctx.lr = 0x82DC64A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC64A8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC64AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC64B0: 419A0028  beq cr6, 0x82dc64d8
	if ctx.cr[6].eq {
	pc = 0x82DC64D8; continue 'dispatch;
	}
	// 82DC64B4: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 82DC64B8: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DC64BC: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82DC64C0: 4098FFAC  bge cr6, 0x82dc646c
	if !ctx.cr[6].lt {
	pc = 0x82DC646C; continue 'dispatch;
	}
	// 82DC64C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC64C8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DC64CC: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC64D0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DC64D4: 4BEE2F68  b 0x82ca943c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA943C);
	return;
	// 82DC64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC64DC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DC64E0: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC64E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DC64E8: 4BEE2F54  b 0x82ca943c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA943C);
	return;
}

pub fn sub_82DC64F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC64F0 size=312
	// 82DC64F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC64F4: 4BEE2F0D  bl 0x82ca9400
	ctx.lr = 0x82DC64F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 82DC64F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC64FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC6500: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DC6504: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6508: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC650C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC6510: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82DC6514: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DC6518: 4198004C  blt cr6, 0x82dc6564
	if ctx.cr[6].lt {
	pc = 0x82DC6564; continue 'dispatch;
	}
	// 82DC651C: 577F103A  slwi r31, r27, 2
	// 82DC6520: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC6524: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6528: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC652C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC6530: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC6534: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 82DC6538: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC653C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6544: 4E800421  bctrl
	ctx.lr = 0x82DC6548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6548: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC654C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6550: 419A0028  beq cr6, 0x82dc6578
	if ctx.cr[6].eq {
	pc = 0x82DC6578; continue 'dispatch;
	}
	// 82DC6554: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82DC6558: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82DC655C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DC6560: 4098FFC0  bge cr6, 0x82dc6520
	if !ctx.cr[6].lt {
	pc = 0x82DC6520; continue 'dispatch;
	}
	// 82DC6564: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6568: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC656C: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6570: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC6574: 4BEE2EDC  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
	// 82DC6578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC657C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC6580: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6584: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC6588: 4BEE2EC8  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_82DC6628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6628 size=416
	// 82DC6628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC662C: 4BEE2DDD  bl 0x82ca9408
	ctx.lr = 0x82DC6630;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DC6630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6634: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC6638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC663C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DC6640: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82DC6644: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC6648: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC664C: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82DC6650: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DC6654: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82DC6658: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC665C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC6660: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DC6664: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DC6668: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DC666C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DC6670: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82DC6674: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DC6678: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC667C: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DC6680: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82DC6684: 396B1518  addi r11, r11, 0x1518
	ctx.r[11].s64 = ctx.r[11].s64 + 5400;
	// 82DC6688: 38E71544  addi r7, r7, 0x1544
	ctx.r[7].s64 = ctx.r[7].s64 + 5444;
	// 82DC668C: 38C61538  addi r6, r6, 0x1538
	ctx.r[6].s64 = ctx.r[6].s64 + 5432;
	// 82DC6690: 38A51524  addi r5, r5, 0x1524
	ctx.r[5].s64 = ctx.r[5].s64 + 5412;
	// 82DC6694: 3884150C  addi r4, r4, 0x150c
	ctx.r[4].s64 = ctx.r[4].s64 + 5388;
	// 82DC6698: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DC669C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82DC66A0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DC66A4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DC66A8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DC66AC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82DC66B0: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DC66B4: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82DC66B8: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82DC66BC: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DC66C0: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DC66C4: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DC66C8: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DC66CC: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82DC66D0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC66D4: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC66D8: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC66DC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DC66E0: 40980060  bge cr6, 0x82dc6740
	if !ctx.cr[6].lt {
	pc = 0x82DC6740; continue 'dispatch;
	}
	// 82DC66E4: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC66E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC66EC: 409A0020  bne cr6, 0x82dc670c
	if !ctx.cr[6].eq {
	pc = 0x82DC670C; continue 'dispatch;
	}
	// 82DC66F0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC66F4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC66F8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC66FC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6700: 5545103A  slwi r5, r10, 2
	// 82DC6704: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC6708: 4BF8EBC1  bl 0x82d552c8
	ctx.lr = 0x82DC670C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DC670C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6710: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC6714: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6718: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DC671C: 5524103A  slwi r4, r9, 2
	// 82DC6720: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC6724: 4BF8EB25  bl 0x82d55248
	ctx.lr = 0x82DC6728;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DC6728: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC672C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DC6730: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6734: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC6738: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DC673C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC6740: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6744: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6748: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC674C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DC6750: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6754: 40990020  ble cr6, 0x82dc6774
	if !ctx.cr[6].gt {
	pc = 0x82DC6774; continue 'dispatch;
	}
	// 82DC6758: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DC675C: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC6760: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DC6764: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC6768: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC676C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DC6770: 409AFFEC  bne cr6, 0x82dc675c
	if !ctx.cr[6].eq {
	pc = 0x82DC675C; continue 'dispatch;
	}
	// 82DC6774: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC6778: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DC677C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC6780: 40990038  ble cr6, 0x82dc67b8
	if !ctx.cr[6].gt {
	pc = 0x82DC67B8; continue 'dispatch;
	}
	// 82DC6784: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6788: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC678C: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6790: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DC6794: 419A0010  beq cr6, 0x82dc67a4
	if ctx.cr[6].eq {
	pc = 0x82DC67A4; continue 'dispatch;
	}
	// 82DC6798: A12B0006  lhz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC679C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DC67A0: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC67A4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC67A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC67AC: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DC67B0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC67B4: 4198FFD0  blt cr6, 0x82dc6784
	if ctx.cr[6].lt {
	pc = 0x82DC6784; continue 'dispatch;
	}
	// 82DC67B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC67BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC67C0: 4BEE2C98  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DC67C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC67C8 size=296
	// 82DC67C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC67CC: 4BEE2C41  bl 0x82ca940c
	ctx.lr = 0x82DC67D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC67D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC67D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC67D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC67DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC67E0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC67E4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DC67E8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DC67EC: 80DF0034  lwz r6, 0x34(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC67F0: 396B1544  addi r11, r11, 0x1544
	ctx.r[11].s64 = ctx.r[11].s64 + 5444;
	// 82DC67F4: 394A1538  addi r10, r10, 0x1538
	ctx.r[10].s64 = ctx.r[10].s64 + 5432;
	// 82DC67F8: 39291524  addi r9, r9, 0x1524
	ctx.r[9].s64 = ctx.r[9].s64 + 5412;
	// 82DC67FC: 39081518  addi r8, r8, 0x1518
	ctx.r[8].s64 = ctx.r[8].s64 + 5400;
	// 82DC6800: 38E7150C  addi r7, r7, 0x150c
	ctx.r[7].s64 = ctx.r[7].s64 + 5388;
	// 82DC6804: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC6808: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC680C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DC6810: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DC6814: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DC6818: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DC681C: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DC6820: 4099005C  ble cr6, 0x82dc687c
	if !ctx.cr[6].gt {
	pc = 0x82DC687C; continue 'dispatch;
	}
	// 82DC6824: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC6828: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC682C: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC6830: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6838: 419A0030  beq cr6, 0x82dc6868
	if ctx.cr[6].eq {
	pc = 0x82DC6868; continue 'dispatch;
	}
	// 82DC683C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC6840: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC6844: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC6848: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC684C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC6850: 409A0018  bne cr6, 0x82dc6868
	if !ctx.cr[6].eq {
	pc = 0x82DC6868; continue 'dispatch;
	}
	// 82DC6854: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6858: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC685C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6864: 4E800421  bctrl
	ctx.lr = 0x82DC6868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6868: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC686C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DC6870: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DC6874: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC6878: 4198FFB0  blt cr6, 0x82dc6828
	if ctx.cr[6].lt {
	pc = 0x82DC6828; continue 'dispatch;
	}
	// 82DC687C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC6880: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC6884: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC6888: 409A0020  bne cr6, 0x82dc68a8
	if !ctx.cr[6].eq {
	pc = 0x82DC68A8; continue 'dispatch;
	}
	// 82DC688C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6890: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC6894: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC6898: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC689C: 5565103A  slwi r5, r11, 2
	// 82DC68A0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC68A4: 4BF8EA25  bl 0x82d552c8
	ctx.lr = 0x82DC68A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DC68A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC68AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC68B0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC68B4: 396B8604  addi r11, r11, -0x79fc
	ctx.r[11].s64 = ctx.r[11].s64 + -31228;
	// 82DC68B8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DC68BC: 394A8624  addi r10, r10, -0x79dc
	ctx.r[10].s64 = ctx.r[10].s64 + -31196;
	// 82DC68C0: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 82DC68C4: 39298610  addi r9, r9, -0x79f0
	ctx.r[9].s64 = ctx.r[9].s64 + -31216;
	// 82DC68C8: 390885E8  addi r8, r8, -0x7a18
	ctx.r[8].s64 = ctx.r[8].s64 + -31256;
	// 82DC68CC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DC68D0: 38E739E0  addi r7, r7, 0x39e0
	ctx.r[7].s64 = ctx.r[7].s64 + 14816;
	// 82DC68D4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC68D8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DC68DC: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DC68E0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DC68E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC68E8: 4BEE2B74  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DC68F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC68F0 size=152
	// 82DC68F0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DC68F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC68F8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DC68FC: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82DC6900: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC6904: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC6908: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82DC690C: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DC6910: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DC6914: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC6918: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC691C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DC6920: 388B1518  addi r4, r11, 0x1518
	ctx.r[4].s64 = ctx.r[11].s64 + 5400;
	// 82DC6924: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC6928: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DC692C: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DC6930: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82DC6934: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82DC6938: 3BEB150C  addi r31, r11, 0x150c
	ctx.r[31].s64 = ctx.r[11].s64 + 5388;
	// 82DC693C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82DC6940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC6944: 38E71544  addi r7, r7, 0x1544
	ctx.r[7].s64 = ctx.r[7].s64 + 5444;
	// 82DC6948: 38C61538  addi r6, r6, 0x1538
	ctx.r[6].s64 = ctx.r[6].s64 + 5432;
	// 82DC694C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DC6950: 38A51524  addi r5, r5, 0x1524
	ctx.r[5].s64 = ctx.r[5].s64 + 5412;
	// 82DC6954: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82DC6958: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DC695C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DC6960: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DC6964: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DC6968: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82DC696C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82DC6970: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82DC6974: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DC6978: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC697C: 91430038  stw r10, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DC6980: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DC6984: 4E800020  blr
	return;
}

pub fn sub_82DC6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6988 size=248
	// 82DC6988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC698C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC6990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC6994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC6998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC699C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC69A0: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC69A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC69A8: 419A0010  beq cr6, 0x82dc69b8
	if ctx.cr[6].eq {
	pc = 0x82DC69B8; continue 'dispatch;
	}
	// 82DC69AC: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC69B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC69B4: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC69B8: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82DC69BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC69C0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC69C4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC69C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC69CC: 409A0010  bne cr6, 0x82dc69dc
	if !ctx.cr[6].eq {
	pc = 0x82DC69DC; continue 'dispatch;
	}
	// 82DC69D0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC69D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC69D8: 4BF905C1  bl 0x82d56f98
	ctx.lr = 0x82DC69DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D56F98);
	// 82DC69DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC69E0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC69E4: 556B103A  slwi r11, r11, 2
	// 82DC69E8: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82DC69EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC69F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC69F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC69F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC69FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC6A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC6A04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC6A08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC6A0C: 4E800020  blr
	return;
	// 82DC6A10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DC6A14: 4E800020  blr
	return;
	// 82DC6A18: 3D600842  lis r11, 0x842
	ctx.r[11].s64 = 138543104;
	// 82DC6A1C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6A20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DC6A24: 61691085  ori r9, r11, 0x1085
	ctx.r[9].u64 = ctx.r[11].u64 | 4229;
	// 82DC6A28: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 82DC6A2C: 7D665B96  divwu r11, r6, r11
	ctx.r[11].u32 = ctx.r[6].u32 / ctx.r[11].u32;
	// 82DC6A30: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC6A34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC6A38: 556B103A  slwi r11, r11, 2
	// 82DC6A3C: 7CEB282E  lwzx r7, r11, r5
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82DC6A40: 7D664816  mulhwu r11, r6, r9
	ctx.r[11].u64 = ((ctx.r[6].u32 as u64 * ctx.r[9].u32 as u64) >> 32);
	// 82DC6A44: 7D4B3050  subf r10, r11, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 82DC6A48: 554AF87E  srwi r10, r10, 1
	// 82DC6A4C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DC6A50: 556BE13E  srwi r11, r11, 4
	// 82DC6A54: 556A2834  slwi r10, r11, 5
	// 82DC6A58: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DC6A5C: 7D6B3050  subf r11, r11, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 82DC6A60: 7D0B5830  slw r11, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DC6A64: 7D6B3838  and r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 82DC6A68: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DC6A6C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DC6A70: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DC6A74: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6A78: 4E800020  blr
	return;
}

pub fn sub_82DC6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC6A80 size=144
	// 82DC6A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6A84: 4BEE2989  bl 0x82ca940c
	ctx.lr = 0x82DC6A88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC6A88: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82DC6A8C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6A90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC6A94: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC6A98: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DC6A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC6AA0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6AA4: 4BFF24F5  bl 0x82db8f98
	ctx.lr = 0x82DC6AA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DB8F98);
	// 82DC6AA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC6AAC: D3FF0020  stfs f31, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DC6AB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC6AB4: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82DC6AB8: 396B16C8  addi r11, r11, 0x16c8
	ctx.r[11].s64 = ctx.r[11].s64 + 5832;
	// 82DC6ABC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82DC6AC0: 394A16A4  addi r10, r10, 0x16a4
	ctx.r[10].s64 = ctx.r[10].s64 + 5796;
	// 82DC6AC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC6AC8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC6ACC: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6AD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6AD4: 419A0010  beq cr6, 0x82dc6ae4
	if ctx.cr[6].eq {
	pc = 0x82DC6AE4; continue 'dispatch;
	}
	// 82DC6AD8: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC6ADC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC6AE0: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC6AE4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC6AE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC6AEC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6AF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC6AF4: 419A0010  beq cr6, 0x82dc6b04
	if ctx.cr[6].eq {
	pc = 0x82DC6B04; continue 'dispatch;
	}
	// 82DC6AF8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC6AFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC6B00: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DC6B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC6B08: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82DC6B0C: 4BEE2950  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DC6B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6B10 size=248
	// 82DC6B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC6B18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC6B1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6B20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC6B24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC6B28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC6B2C: 396B16C8  addi r11, r11, 0x16c8
	ctx.r[11].s64 = ctx.r[11].s64 + 5832;
	// 82DC6B30: 394A16A4  addi r10, r10, 0x16a4
	ctx.r[10].s64 = ctx.r[10].s64 + 5796;
	// 82DC6B34: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC6B38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC6B3C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC6B40: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6B44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6B48: 419A0030  beq cr6, 0x82dc6b78
	if ctx.cr[6].eq {
	pc = 0x82DC6B78; continue 'dispatch;
	}
	// 82DC6B4C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC6B50: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC6B54: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC6B58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC6B5C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC6B60: 409A0018  bne cr6, 0x82dc6b78
	if !ctx.cr[6].eq {
	pc = 0x82DC6B78; continue 'dispatch;
	}
	// 82DC6B64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6B68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC6B6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6B70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6B74: 4E800421  bctrl
	ctx.lr = 0x82DC6B78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6B78: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC6B7C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6B80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6B84: 419A0030  beq cr6, 0x82dc6bb4
	if ctx.cr[6].eq {
	pc = 0x82DC6BB4; continue 'dispatch;
	}
	// 82DC6B88: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC6B8C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC6B90: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC6B94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC6B98: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC6B9C: 409A0018  bne cr6, 0x82dc6bb4
	if !ctx.cr[6].eq {
	pc = 0x82DC6BB4; continue 'dispatch;
	}
	// 82DC6BA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6BA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC6BA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6BB0: 4E800421  bctrl
	ctx.lr = 0x82DC6BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6BB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC6BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DC6BBC: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DC6BC0: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DC6BC4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DC6BC8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC6BCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC6BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC6BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC6BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC6BDC: 4E800020  blr
	return;
	// 82DC6BE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC6BE4: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 82DC6BE8: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC6BEC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC6BF0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DC6BF4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC6BF8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82DC6BFC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DC6C00: 4E800020  blr
	return;
}

pub fn sub_82DC6C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6C08 size=12
	// 82DC6C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6C0C: 4BEE27D9  bl 0x82ca93e4
	ctx.lr = 0x82DC6C10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
	// 82DC6C10: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DC6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC6F28 size=520
	// 82DC6F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6F2C: 4BEE24D9  bl 0x82ca9404
	ctx.lr = 0x82DC6F30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DC6F30: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DC6F34: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6F38: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC6F3C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC6F40: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DC6F44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DC6F48: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82DC6F4C: 3BCB0010  addi r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 16;
	// 82DC6F50: C00A0C64  lfs f0, 0xc64(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC6F54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC6F58: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DC6F5C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DC6F60: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DC6F64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC6F68: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DC6F6C: C1AA0C18  lfs f13, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC6F70: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC6F74: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DC6F78: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DC6F7C: C00A0BE4  lfs f0, 0xbe4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3044 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC6F80: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DC6F84: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DC6F88: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DC6F8C: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
}

pub fn sub_82DC7130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC7130 size=12
	// 82DC7130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC7134: 4BEE22D9  bl 0x82ca940c
	ctx.lr = 0x82DC7138;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC7138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DC7270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC7270 size=384
	// 82DC7270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC7274: 4BEE2195  bl 0x82ca9408
	ctx.lr = 0x82DC7278;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DC7278: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DC727C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC7280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC7284: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82DC7288: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DC728C: 3B8B6E60  addi r28, r11, 0x6e60
	ctx.r[28].s64 = ctx.r[11].s64 + 28256;
	// 82DC7290: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC7294: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC7298: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DC729C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DC72A0: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC72A4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DC72A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC72AC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC72B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC72B4: 4E800421  bctrl
	ctx.lr = 0x82DC72B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC72B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC72BC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DC72C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DC72C4: 4099007C  ble cr6, 0x82dc7340
	if !ctx.cr[6].gt {
	pc = 0x82DC7340; continue 'dispatch;
	}
	// 82DC72C8: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82DC72CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC72D0: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82DC72D4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DC72D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DC72DC: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC72E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC72E4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC72E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC72EC: 4E800421  bctrl
	ctx.lr = 0x82DC72F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC72F0: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DC72F4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DC72F8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DC72FC: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
}

pub fn sub_82DC73F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC73F0 size=12
	// 82DC73F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC73F4: 4BEE2011  bl 0x82ca9404
	ctx.lr = 0x82DC73F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DC73F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DC74E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC74E8 size=224
	// 82DC74E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC74EC: 4BEE1F21  bl 0x82ca940c
	ctx.lr = 0x82DC74F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC74F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC74F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC74F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC74FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC7500: 396B24A4  addi r11, r11, 0x24a4
	ctx.r[11].s64 = ctx.r[11].s64 + 9380;
	// 82DC7504: 394A2180  addi r10, r10, 0x2180
	ctx.r[10].s64 = ctx.r[10].s64 + 8576;
	// 82DC7508: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC750C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC7510: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DC7514: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC7518: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC751C: 4099005C  ble cr6, 0x82dc7578
	if !ctx.cr[6].gt {
	pc = 0x82DC7578; continue 'dispatch;
	}
	// 82DC7520: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC7524: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC7528: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC752C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC7530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC7534: 419A0030  beq cr6, 0x82dc7564
	if ctx.cr[6].eq {
	pc = 0x82DC7564; continue 'dispatch;
	}
	// 82DC7538: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC753C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC7540: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC7544: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC7548: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC754C: 409A0018  bne cr6, 0x82dc7564
	if !ctx.cr[6].eq {
	pc = 0x82DC7564; continue 'dispatch;
	}
	// 82DC7550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7554: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC7558: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC755C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC7560: 4E800421  bctrl
	ctx.lr = 0x82DC7564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC7564: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC7568: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DC756C: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82DC7570: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC7574: 4198FFB0  blt cr6, 0x82dc7524
	if ctx.cr[6].lt {
	pc = 0x82DC7524; continue 'dispatch;
	}
	// 82DC7578: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC757C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC7580: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC7584: 409A0020  bne cr6, 0x82dc75a4
	if !ctx.cr[6].eq {
	pc = 0x82DC75A4; continue 'dispatch;
	}
	// 82DC7588: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC758C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC7590: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC7594: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC7598: 55652036  slwi r5, r11, 4
	// 82DC759C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC75A0: 4BF8DD29  bl 0x82d552c8
	ctx.lr = 0x82DC75A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DC75A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC75A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DC75AC: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DC75B0: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DC75B4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DC75B8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC75BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC75C0: 4BEE1E9C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DC75C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC75C8 size=256
	// 82DC75C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC75CC: 4BEE1E39  bl 0x82ca9404
	ctx.lr = 0x82DC75D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DC75D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC75D4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DC75D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC75DC: 3BFB0018  addi r31, r27, 0x18
	ctx.r[31].s64 = ctx.r[27].s64 + 24;
	// 82DC75E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC75E4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC75E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC75EC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC75F0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC75F4: 40980024  bge cr6, 0x82dc7618
	if !ctx.cr[6].lt {
	pc = 0x82DC7618; continue 'dispatch;
	}
	// 82DC75F8: 556B083C  slwi r11, r11, 1
	// 82DC75FC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC7600: 41980008  blt cr6, 0x82dc7608
	if ctx.cr[6].lt {
	pc = 0x82DC7608; continue 'dispatch;
	}
	// 82DC7604: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC7608: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82DC760C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC7610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC7614: 4BF8F8FD  bl 0x82d56f10
	ctx.lr = 0x82DC7618;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D56F10);
	// 82DC7618: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DC761C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC7620: 40990094  ble cr6, 0x82dc76b4
	if !ctx.cr[6].gt {
	pc = 0x82DC76B4; continue 'dispatch;
	}
	// 82DC7624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DC7628: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC762C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DC7630: 7CBEE050  subf r5, r30, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 82DC7634: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC7638: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC763C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC7640: 419A0060  beq cr6, 0x82dc76a0
	if ctx.cr[6].eq {
	pc = 0x82DC76A0; continue 'dispatch;
	}
	// 82DC7644: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7648: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DC764C: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82DC7650: 419A000C  beq cr6, 0x82dc765c
	if ctx.cr[6].eq {
	pc = 0x82DC765C; continue 'dispatch;
	}
	// 82DC7654: 7D25402E  lwzx r9, r5, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DC7658: 48000008  b 0x82dc7660
	pc = 0x82DC7660; continue 'dispatch;
	// 82DC765C: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82DC7660: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7664: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC7668: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC766C: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7670: A12A0004  lhz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC7674: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DC7678: 419A0010  beq cr6, 0x82dc7688
	if ctx.cr[6].eq {
	pc = 0x82DC7688; continue 'dispatch;
	}
	// 82DC767C: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC7680: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DC7684: B12A0006  sth r9, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC7688: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC768C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC7690: 93AA000C  stw r29, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82DC7694: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7698: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC769C: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DC76A0: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82DC76A4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DC76A8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DC76AC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DC76B0: 409AFF88  bne cr6, 0x82dc7638
	if !ctx.cr[6].eq {
	pc = 0x82DC7638; continue 'dispatch;
	}
	// 82DC76B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DC76B8: 4BFFFBB9  bl 0x82dc7270
	ctx.lr = 0x82DC76BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC7270);
	// 82DC76BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC76C0: 4BEE1D94  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DC76C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC76C8 size=488
	// 82DC76C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC76CC: 4BEE1D21  bl 0x82ca93ec
	ctx.lr = 0x82DC76D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93EC);
	// 82DC76D0: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC76D4: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC76D8: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DC76DC: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82DC76E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC76E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC76E8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DC76EC: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DC76F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC76F4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC76F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC76FC: 40980020  bge cr6, 0x82dc771c
	if !ctx.cr[6].lt {
	pc = 0x82DC771C; continue 'dispatch;
	}
	// 82DC7700: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC7704: 39293170  addi r9, r9, 0x3170
	ctx.r[9].s64 = ctx.r[9].s64 + 12656;
	// 82DC7708: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC770C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC7710: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC7714: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC7718: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC771C: 817B0040  lwz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC7720: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 82DC7724: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DC7728: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC772C: 7ED9B378  mr r25, r22
	ctx.r[25].u64 = ctx.r[22].u64;
	// 82DC7730: 917B0040  stw r11, 0x40(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC7734: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC7738: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC773C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC7740: 409A0070  bne cr6, 0x82dc77b0
	if !ctx.cr[6].eq {
	pc = 0x82DC77B0; continue 'dispatch;
	}
	// 82DC7744: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC7748: 409900F4  ble cr6, 0x82dc783c
	if !ctx.cr[6].gt {
	pc = 0x82DC783C; continue 'dispatch;
	}
	// 82DC774C: 3B9E0010  addi r28, r30, 0x10
	ctx.r[28].s64 = ctx.r[30].s64 + 16;
	// 82DC7750: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7754: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DC7758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC775C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DC7760: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC7764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC7768: 4E800421  bctrl
	ctx.lr = 0x82DC776C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC776C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DC7770: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DC7774: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC7778: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DC777C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC7780: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC7784: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC7788: 4E800421  bctrl
	ctx.lr = 0x82DC778C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC778C: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82DC7790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC7794: 419A0008  beq cr6, 0x82dc779c
	if ctx.cr[6].eq {
	pc = 0x82DC779C; continue 'dispatch;
	}
	// 82DC7798: 7FF9FB78  mr r25, r31
	ctx.r[25].u64 = ctx.r[31].u64;
	// 82DC779C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC77A0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DC77A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC77A8: 4198FFA8  blt cr6, 0x82dc7750
	if ctx.cr[6].lt {
	pc = 0x82DC7750; continue 'dispatch;
	}
	// 82DC77AC: 48000090  b 0x82dc783c
	pc = 0x82DC783C; continue 'dispatch;
	// 82DC77B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC77B4: 40990088  ble cr6, 0x82dc783c
	if !ctx.cr[6].gt {
	pc = 0x82DC783C; continue 'dispatch;
	}
	// 82DC77B8: 3B5E0010  addi r26, r30, 0x10
	ctx.r[26].s64 = ctx.r[30].s64 + 16;
	// 82DC77BC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DC77C0: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC77C4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DC77C8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DC77CC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC77D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC77D4: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82DC77D8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC77DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC77E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC77E4: 4E800421  bctrl
	ctx.lr = 0x82DC77E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC77E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC77EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC77F0: 419A0038  beq cr6, 0x82dc7828
	if ctx.cr[6].eq {
	pc = 0x82DC7828; continue 'dispatch;
	}
	// 82DC77F4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC77F8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DC77FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC7800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC7804: 7C8BE02E  lwzx r4, r11, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DC7808: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC780C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC7810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC7814: 4E800421  bctrl
	ctx.lr = 0x82DC7818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC7818: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DC781C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC7820: 419A0008  beq cr6, 0x82dc7828
	if ctx.cr[6].eq {
	pc = 0x82DC7828; continue 'dispatch;
	}
	// 82DC7824: 7FF9FB78  mr r25, r31
	ctx.r[25].u64 = ctx.r[31].u64;
	// 82DC7828: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC782C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DC7830: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 82DC7834: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC7838: 4198FF88  blt cr6, 0x82dc77c0
	if ctx.cr[6].lt {
	pc = 0x82DC77C0; continue 'dispatch;
	}
	// 82DC783C: 817B0040  lwz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC7840: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 82DC7844: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC7848: 917B0040  stw r11, 0x40(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC784C: 419A0010  beq cr6, 0x82dc785c
	if ctx.cr[6].eq {
	pc = 0x82DC785C; continue 'dispatch;
	}
	// 82DC7850: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC7854: 556B103A  slwi r11, r11, 2
	// 82DC7858: 7F2BD92E  stwx r25, r11, r27
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32), ctx.r[25].u32) };
	// 82DC785C: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DC7860: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC7864: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC7868: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC786C: 40980020  bge cr6, 0x82dc788c
	if !ctx.cr[6].lt {
	pc = 0x82DC788C; continue 'dispatch;
	}
	// 82DC7870: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DC7874: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DC7878: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC787C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC7880: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DC7884: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC7888: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC788C: 7D79B050  subf r11, r25, r22
	ctx.r[11].s64 = ctx.r[22].s64 - ctx.r[25].s64;
	// 82DC7890: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DC7894: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DC7898: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DC789C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DC78A0: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC78A4: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DC78A8: 4BEE1B94  b 0x82ca943c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA943C);
	return;
}

pub fn sub_82DC78B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC78B0 size=264
	// 82DC78B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC78B4: 4BEE1B4D  bl 0x82ca9400
	ctx.lr = 0x82DC78B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 82DC78B8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DC78BC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC78C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC78C4: 11BF038C  vspltisw v13, -1
	for i in 0..4 {
		ctx.v[13].u32[i] = 4294967295;
	}
	// 82DC78C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC78CC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC78D0: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DC78D4: 396B4C30  addi r11, r11, 0x4c30
	ctx.r[11].s64 = ctx.r[11].s64 + 19504;
	// 82DC78D8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DC78DC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
}

pub fn sub_82DC79B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC79B8 size=2000
	// 82DC79B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC79BC: 4BEE1A15  bl 0x82ca93d0
	ctx.lr = 0x82DC79C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
	// 82DC79C0: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DC79C4: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DC79C8: 3981FF50  addi r12, r1, -0xb0
	ctx.r[12].s64 = ctx.r[1].s64 + -176;
	// 82DC79CC: 4823F009  bl 0x830069d4
	ctx.lr = 0x82DC79D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830069D4);
	// 82DC79D0: 9421FC10  stwu r1, -0x3f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1008 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC79D4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
}

pub fn sub_82DC8188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC8188 size=1008
	// 82DC8188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC818C: 4BEE1245  bl 0x82ca93d0
	ctx.lr = 0x82DC8190;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
	// 82DC8190: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DC8194: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DC8198: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC819C: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 82DC81A0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC81A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC81A8: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82DC81AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DC81B0: 3B5C0010  addi r26, r28, 0x10
	ctx.r[26].s64 = ctx.r[28].s64 + 16;
	// 82DC81B4: 3A4B0010  addi r18, r11, 0x10
	ctx.r[18].s64 = ctx.r[11].s64 + 16;
	// 82DC81B8: C00A0C64  lfs f0, 0xc64(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC81BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC81C0: D01C0000  stfs f0, 0(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DC81C4: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
}

pub fn sub_82DC8578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC8578 size=232
	// 82DC8578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC857C: 4BEE0E89  bl 0x82ca9404
	ctx.lr = 0x82DC8580;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DC8580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC8584: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC8588: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC858C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC8590: 388B3180  addi r4, r11, 0x3180
	ctx.r[4].s64 = ctx.r[11].s64 + 12672;
	// 82DC8594: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DC8598: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC859C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC85A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC85A4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC85A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC85AC: 4E800421  bctrl
	ctx.lr = 0x82DC85B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC85B0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC85B4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC85B8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC85BC: 409A0034  bne cr6, 0x82dc85f0
	if !ctx.cr[6].eq {
	pc = 0x82DC85F0; continue 'dispatch;
	}
	// 82DC85C0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC85C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC85C8: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC85CC: 55682036  slwi r8, r11, 4
	// 82DC85D0: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC85D4: 388A2BEC  addi r4, r10, 0x2bec
	ctx.r[4].s64 = ctx.r[10].s64 + 11244;
	// 82DC85D8: 54E72036  slwi r7, r7, 4
	// 82DC85DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC85E0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC85E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC85E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC85EC: 4E800421  bctrl
	ctx.lr = 0x82DC85F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC85F0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC85F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DC85F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC85FC: 40990048  ble cr6, 0x82dc8644
	if !ctx.cr[6].gt {
	pc = 0x82DC8644; continue 'dispatch;
	}
	// 82DC8600: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DC8604: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC8608: 3B6BB3B4  addi r27, r11, -0x4c4c
	ctx.r[27].s64 = ctx.r[11].s64 + -19532;
	// 82DC860C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC8610: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC8614: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC8618: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DC861C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC8620: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC8624: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DC8628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC862C: 4E800421  bctrl
	ctx.lr = 0x82DC8630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC8630: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC8634: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DC8638: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82DC863C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC8640: 4198FFCC  blt cr6, 0x82dc860c
	if ctx.cr[6].lt {
	pc = 0x82DC860C; continue 'dispatch;
	}
	// 82DC8644: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC8648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC864C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC8650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC8654: 4E800421  bctrl
	ctx.lr = 0x82DC8658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC8658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC865C: 4BEE0DF8  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DC8660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC8660 size=12
	// 82DC8660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC8664: 4BEE0DA9  bl 0x82ca940c
	ctx.lr = 0x82DC8668;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC8668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DC8728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC8728 size=128
	// 82DC8728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC8730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC8734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC8738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC873C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC8740: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC8744: 4BFFEDA5  bl 0x82dc74e8
	ctx.lr = 0x82DC8748;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC74E8);
	// 82DC8748: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC874C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC8750: 419A0020  beq cr6, 0x82dc8770
	if ctx.cr[6].eq {
	pc = 0x82DC8770; continue 'dispatch;
	}
	// 82DC8754: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC8758: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC875C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DC8760: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC8764: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC8768: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC876C: 4BF8CB5D  bl 0x82d552c8
	ctx.lr = 0x82DC8770;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DC8770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC8774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC8778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC877C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC8780: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC8784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC8788: 4E800020  blr
	return;
}

pub fn sub_82DC87A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC87A8 size=368
	// 82DC87A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC87AC: 4BEE0C51  bl 0x82ca93fc
	ctx.lr = 0x82DC87B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 82DC87B0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC87B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC87B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DC87BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC87C0: 39000016  li r8, 0x16
	ctx.r[8].s64 = 22;
	// 82DC87C4: 394B1B08  addi r10, r11, 0x1b08
	ctx.r[10].s64 = ctx.r[11].s64 + 6920;
	// 82DC87C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC87CC: B13E0006  sth r9, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC87D0: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 82DC87D4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DC87D8: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DC87DC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DC87E0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC87E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC87E8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC87EC: 3B3E001C  addi r25, r30, 0x1c
	ctx.r[25].s64 = ctx.r[30].s64 + 28;
	// 82DC87F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC87F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC87F8: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DC87FC: D03E001C  stfs f1, 0x1c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DC8800: 835F0004  lwz r26, 4(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC8804: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC8808: 7F9ADA14  add r28, r26, r27
	ctx.r[28].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 82DC880C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC8810: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DC8814: 40980024  bge cr6, 0x82dc8838
	if !ctx.cr[6].lt {
	pc = 0x82DC8838; continue 'dispatch;
	}
	// 82DC8818: 556B083C  slwi r11, r11, 1
	// 82DC881C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC8820: 41980008  blt cr6, 0x82dc8828
	if ctx.cr[6].lt {
	pc = 0x82DC8828; continue 'dispatch;
	}
	// 82DC8824: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82DC8828: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82DC882C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC8830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC8834: 4BF8E6DD  bl 0x82d56f10
	ctx.lr = 0x82DC8838;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D56F10);
	// 82DC8838: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC883C: 395BFFFF  addi r10, r27, -1
	ctx.r[10].s64 = ctx.r[27].s64 + -1;
	// 82DC8840: 574B2834  slwi r11, r26, 5
	// 82DC8844: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DC8848: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC884C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DC8850: 419800B8  blt cr6, 0x82dc8908
	if ctx.cr[6].lt {
	pc = 0x82DC8908; continue 'dispatch;
	}
	// 82DC8854: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DC8858: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DC885C: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
}

pub fn sub_82DC8918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC8918 size=176
	// 82DC8918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC891C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC8920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC8924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC8928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC892C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC8930: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC8934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC8938: 388B31A8  addi r4, r11, 0x31a8
	ctx.r[4].s64 = ctx.r[11].s64 + 12712;
	// 82DC893C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC8940: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC8944: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC8948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC894C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC8950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC8954: 4E800421  bctrl
	ctx.lr = 0x82DC8958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC8958: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC895C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC8960: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC8964: 409A0034  bne cr6, 0x82dc8998
	if !ctx.cr[6].eq {
	pc = 0x82DC8998; continue 'dispatch;
	}
	// 82DC8968: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC896C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC8970: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC8974: 55682834  slwi r8, r11, 5
	// 82DC8978: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC897C: 388A31A0  addi r4, r10, 0x31a0
	ctx.r[4].s64 = ctx.r[10].s64 + 12704;
	// 82DC8980: 54E72834  slwi r7, r7, 5
	// 82DC8984: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC8988: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC898C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC8990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC8994: 4E800421  bctrl
	ctx.lr = 0x82DC8998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC8998: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC899C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC89A0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC89A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC89A8: 4E800421  bctrl
	ctx.lr = 0x82DC89AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC89AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC89B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC89B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC89B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC89BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC89C0: 4E800020  blr
	return;
}

pub fn sub_82DC89C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC89C8 size=976
	// 82DC89C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC89CC: 4BEE0A31  bl 0x82ca93fc
	ctx.lr = 0x82DC89D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 82DC89D0: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC89D4: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82DC89D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC89DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC89E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC89E4: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82DC89E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC89EC: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC89F0: 2F1D0010  cmpwi cr6, r29, 0x10
	ctx.cr[6].compare_i32(ctx.r[29].s32, 16, &mut ctx.xer);
	// 82DC89F4: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82DC89F8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DC89FC: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82DC8A00: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82DC8A04: 40990020  ble cr6, 0x82dc8a24
	if !ctx.cr[6].gt {
	pc = 0x82DC8A24; continue 'dispatch;
	}
	// 82DC8A08: 2F1D0020  cmpwi cr6, r29, 0x20
	ctx.cr[6].compare_i32(ctx.r[29].s32, 32, &mut ctx.xer);
	// 82DC8A0C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DC8A10: 41980008  blt cr6, 0x82dc8a18
	if ctx.cr[6].lt {
	pc = 0x82DC8A18; continue 'dispatch;
	}
	// 82DC8A14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DC8A18: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82DC8A1C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82DC8A20: 4BF8E4F1  bl 0x82d56f10
	ctx.lr = 0x82DC8A24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D56F10);
	// 82DC8A24: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82DC8A28: 93A10094  stw r29, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82DC8A2C: EBBF0000  ld r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82DC8A30: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82DC8A34: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 82DC8A38: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82DC8A3C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DC8A40: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC8A44: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DC8A48: 83DE0010  lwz r30, 0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC8A4C: EB280000  ld r25, 0(r8)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DC8A50: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82DC8A54: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 82DC8A58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DC8A5C: EB6A0000  ld r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DC8A60: 556B083C  slwi r11, r11, 1
	// 82DC8A64: FBA70000  std r29, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 82DC8A68: FBE70008  std r31, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 82DC8A6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC8A70: FB240000  std r25, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82DC8A74: F9040008  std r8, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 82DC8A78: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DC8A7C: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82DC8A80: 55632036  slwi r3, r11, 4
	// 82DC8A84: FB660000  std r27, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DC8A88: EB490000  ld r26, 0(r9)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DC8A8C: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
}

pub fn sub_82DC8D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DC8D98 size=840
	// 82DC8D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC8D9C: 4BEE0651  bl 0x82ca93ec
	ctx.lr = 0x82DC8DA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93EC);
	// 82DC8DA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC8DA4: 39230040  addi r9, r3, 0x40
	ctx.r[9].s64 = ctx.r[3].s64 + 64;
	// 82DC8DA8: 390B3200  addi r8, r11, 0x3200
	ctx.r[8].s64 = ctx.r[11].s64 + 12800;
	// 82DC8DAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC8DB0: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82DC8DB4: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82DC8DB8: C00B0B98  lfs f0, 0xb98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2968 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC8DBC: 4198022C  blt cr6, 0x82dc8fe8
	if ctx.cr[6].lt {
	pc = 0x82DC8FE8; continue 'dispatch;
	}
	// 82DC8DC0: 3965FFFC  addi r11, r5, -4
	ctx.r[11].s64 = ctx.r[5].s64 + -4;
	// 82DC8DC4: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 82DC8DC8: 38E30020  addi r7, r3, 0x20
	ctx.r[7].s64 = ctx.r[3].s64 + 32;
	// 82DC8DCC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82DC8DD0: 556BF0BE  srwi r11, r11, 2
	// 82DC8DD4: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82DC8DD8: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82DC8DDC: 39460020  addi r10, r6, 0x20
	ctx.r[10].s64 = ctx.r[6].s64 + 32;
	// 82DC8DE0: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82DC8DE4: 57D6103A  slwi r22, r30, 2
	// 82DC8DE8: 3AE0FFE0  li r23, -0x20
	ctx.r[23].s64 = -32;
	// 82DC8DEC: 3B00FFF0  li r24, -0x10
	ctx.r[24].s64 = -16;
	// 82DC8DF0: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82DC8DF4: 3B61FF80  addi r27, r1, -0x80
	ctx.r[27].s64 = ctx.r[1].s64 + -128;
}

pub fn sub_82DC90E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC90E0 size=128
	// 82DC90E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC90E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC90E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC90EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC90F0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DC90F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DC90F8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DC90FC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DC9100: C00A0BFC  lfs f0, 0xbfc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC9104: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DC9108: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DC910C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
}

pub fn sub_82DC9160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC9160 size=520
	// 82DC9160: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DC9164: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
}

pub fn sub_82DC9368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DC9368 size=392
	// 82DC9368: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DC936C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9370: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DC9374: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC9378: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC937C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC9380: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC9384: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DC9388: 40980020  bge cr6, 0x82dc93a8
	if !ctx.cr[6].lt {
	pc = 0x82DC93A8; continue 'dispatch;
	}
	// 82DC938C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DC9390: 39083220  addi r8, r8, 0x3220
	ctx.r[8].s64 = ctx.r[8].s64 + 12832;
	// 82DC9394: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC9398: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DC939C: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DC93A0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC93A4: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DC93A8: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82DC93AC: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC93B0: C1860010  lfs f12, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DC93B4: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 82DC93B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC93BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
}

pub fn sub_82DC94F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC94F0 size=176
	// 82DC94F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC94F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC94F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC94FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC9500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9504: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC9508: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC950C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC9510: 388B3238  addi r4, r11, 0x3238
	ctx.r[4].s64 = ctx.r[11].s64 + 12856;
	// 82DC9514: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC9518: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC951C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC9520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC9524: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC9528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC952C: 4E800421  bctrl
	ctx.lr = 0x82DC9530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC9530: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC9534: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC9538: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC953C: 409A0034  bne cr6, 0x82dc9570
	if !ctx.cr[6].eq {
	pc = 0x82DC9570; continue 'dispatch;
	}
	// 82DC9540: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9544: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC9548: 80FE0018  lwz r7, 0x18(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC954C: 55682036  slwi r8, r11, 4
	// 82DC9550: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC9554: 388A005C  addi r4, r10, 0x5c
	ctx.r[4].s64 = ctx.r[10].s64 + 92;
	// 82DC9558: 54E72036  slwi r7, r7, 4
	// 82DC955C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC9560: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC9564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC9568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC956C: 4E800421  bctrl
	ctx.lr = 0x82DC9570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC9570: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC9578: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC957C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC9580: 4E800421  bctrl
	ctx.lr = 0x82DC9584;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC9584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC9588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC9590: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC9594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC9598: 4E800020  blr
	return;
}

pub fn sub_82DC95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC95A0 size=1272
	// 82DC95A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC95A4: 4BEDFE45  bl 0x82ca93e8
	ctx.lr = 0x82DC95A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E8);
	// 82DC95A8: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
}

pub fn sub_82DC9A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC9A98 size=280
	// 82DC9A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9A9C: 4BEDF971  bl 0x82ca940c
	ctx.lr = 0x82DC9AA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC9AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9AA4: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 82DC9AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC9AAC: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82DC9AB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DC9AB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC9AB8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82DC9ABC: 394B1E94  addi r10, r11, 0x1e94
	ctx.r[10].s64 = ctx.r[11].s64 + 7828;
	// 82DC9AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC9AC4: D03E0010  stfs f1, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DC9AC8: B13E0006  sth r9, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC9ACC: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 82DC9AD0: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82DC9AD4: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DC9AD8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DC9ADC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC9AE0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC9AE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC9AE8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC9AEC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DC9AF0: 80C10094  lwz r6, 0x94(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82DC9AF4: 80A10098  lwz r5, 0x98(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82DC9AF8: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DC9AFC: 4BFFFAA5  bl 0x82dc95a0
	ctx.lr = 0x82DC9B00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC95A0);
	// 82DC9B00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC9B04: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC9B08: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC9B0C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DC9B10: 40980060  bge cr6, 0x82dc9b70
	if !ctx.cr[6].lt {
	pc = 0x82DC9B70; continue 'dispatch;
	}
	// 82DC9B14: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC9B18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC9B1C: 409A0020  bne cr6, 0x82dc9b3c
	if !ctx.cr[6].eq {
	pc = 0x82DC9B3C; continue 'dispatch;
	}
	// 82DC9B20: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9B24: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC9B28: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC9B2C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9B30: 55452036  slwi r5, r10, 4
	// 82DC9B34: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC9B38: 4BF8B791  bl 0x82d552c8
	ctx.lr = 0x82DC9B3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DC9B3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9B40: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC9B44: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC9B48: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DC9B4C: 55242036  slwi r4, r9, 4
	// 82DC9B50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC9B54: 4BF8B6F5  bl 0x82d55248
	ctx.lr = 0x82DC9B58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DC9B58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC9B5C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DC9B60: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC9B64: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC9B68: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DC9B6C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC9B70: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC9B74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9B78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC9B7C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DC9B80: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9B84: 40990020  ble cr6, 0x82dc9ba4
	if !ctx.cr[6].gt {
	pc = 0x82DC9BA4; continue 'dispatch;
	}
	// 82DC9B88: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DC9B8C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
}

pub fn sub_82DC9BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC9BB0 size=80
	// 82DC9BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC9BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9BBC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DC9BC0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC9BC4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DC9BC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC9BCC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DC9BD0: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DC9BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC9BD8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DC9BDC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DC9BE0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DC9BE4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DC9BE8: 48004231  bl 0x82dcde18
	ctx.lr = 0x82DC9BEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCDE18);
	// 82DC9BEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC9BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC9BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC9BF8: 4E800020  blr
	return;
}

pub fn sub_82DC9C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC9C00 size=80
	// 82DC9C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC9C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9C0C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC9C10: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DC9C14: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DC9C18: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DC9C1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DC9C20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DC9C24: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DC9C28: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC9C2C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC9C30: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DC9C34: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DC9C38: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DC9C3C: 48003F85  bl 0x82dcdbc0
	ctx.lr = 0x82DC9C40;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCDBC0);
	// 82DC9C40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC9C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC9C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC9C4C: 4E800020  blr
	return;
}

pub fn sub_82DC9C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC9C50 size=232
	// 82DC9C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC9C58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC9C5C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9C60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC9C64: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DC9C68: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DC9C6C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DC9C70: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DC9C74: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DC9C78: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC9C7C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC9C80: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC9C84: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DC9C88: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC9C8C: 4200FFF0  bdnz 0x82dc9c7c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC9C7C; continue 'dispatch;
	}
	// 82DC9C90: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC9C94: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DC9C98: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DC9C9C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DC9CA0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DC9CA4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DC9D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC9D38 size=144
	// 82DC9D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC9D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC9D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9D48: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC9D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC9D50: 396B3250  addi r11, r11, 0x3250
	ctx.r[11].s64 = ctx.r[11].s64 + 12880;
	// 82DC9D54: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DC9D58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC9D5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC9D60: 419A004C  beq cr6, 0x82dc9dac
	if ctx.cr[6].eq {
	pc = 0x82DC9DAC; continue 'dispatch;
	}
	// 82DC9D64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9D68: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC9D6C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC9D70: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DC9D74: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC9D78: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DC9D7C: 41980018  blt cr6, 0x82dc9d94
	if ctx.cr[6].lt {
	pc = 0x82DC9D94; continue 'dispatch;
	}
	// 82DC9D80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DC9D84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC9D88: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DC9D8C: 4BF8B39D  bl 0x82d55128
	ctx.lr = 0x82DC9D90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55128);
	// 82DC9D90: 4800001C  b 0x82dc9dac
	pc = 0x82DC9DAC; continue 'dispatch;
	// 82DC9D94: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DC9D98: 812B0040  lwz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC9D9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC9DA0: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DC9DA4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC9DA8: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82DC9DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC9DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC9DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC9DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC9DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC9DC0: 4E800020  blr
	return;
}

pub fn sub_82DC9DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC9DC8 size=144
	// 82DC9DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC9DD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC9DD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9DD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC9DDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC9DE0: 396B3250  addi r11, r11, 0x3250
	ctx.r[11].s64 = ctx.r[11].s64 + 12880;
	// 82DC9DE4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DC9DE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC9DEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC9DF0: 419A004C  beq cr6, 0x82dc9e3c
	if ctx.cr[6].eq {
	pc = 0x82DC9E3C; continue 'dispatch;
	}
	// 82DC9DF4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9DF8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC9DFC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC9E00: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DC9E04: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC9E08: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DC9E0C: 41980018  blt cr6, 0x82dc9e24
	if ctx.cr[6].lt {
	pc = 0x82DC9E24; continue 'dispatch;
	}
	// 82DC9E10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DC9E14: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC9E18: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DC9E1C: 4BF8B30D  bl 0x82d55128
	ctx.lr = 0x82DC9E20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55128);
	// 82DC9E20: 4800001C  b 0x82dc9e3c
	pc = 0x82DC9E3C; continue 'dispatch;
	// 82DC9E24: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DC9E28: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DC9E2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC9E30: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82DC9E34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC9E38: 93EB0058  stw r31, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82DC9E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC9E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC9E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC9E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC9E4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC9E50: 4E800020  blr
	return;
}

pub fn sub_82DC9E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC9E58 size=144
	// 82DC9E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC9E60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC9E64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC9E68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC9E6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC9E70: 396B325C  addi r11, r11, 0x325c
	ctx.r[11].s64 = ctx.r[11].s64 + 12892;
	// 82DC9E74: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DC9E78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC9E7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC9E80: 419A004C  beq cr6, 0x82dc9ecc
	if ctx.cr[6].eq {
	pc = 0x82DC9ECC; continue 'dispatch;
	}
	// 82DC9E84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC9E88: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC9E8C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC9E90: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DC9E94: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC9E98: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DC9E9C: 41980018  blt cr6, 0x82dc9eb4
	if ctx.cr[6].lt {
	pc = 0x82DC9EB4; continue 'dispatch;
	}
	// 82DC9EA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DC9EA4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DC9EA8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DC9EAC: 4BF8B27D  bl 0x82d55128
	ctx.lr = 0x82DC9EB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55128);
	// 82DC9EB0: 4800001C  b 0x82dc9ecc
	pc = 0x82DC9ECC; continue 'dispatch;
	// 82DC9EB4: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DC9EB8: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DC9EBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC9EC0: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82DC9EC4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC9EC8: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82DC9ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC9ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC9ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC9ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC9EDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC9EE0: 4E800020  blr
	return;
}

pub fn sub_82DC9EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC9EE8 size=12
	// 82DC9EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC9EEC: 4BEDF521  bl 0x82ca940c
	ctx.lr = 0x82DC9EF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DC9EF0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCA030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA030 size=104
	// 82DCA030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCA038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCA03C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCA044: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DCA048: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DCA04C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCA050: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCA054: 4804744D  bl 0x82e114a0
	ctx.lr = 0x82DCA058;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E114A0);
	// 82DCA058: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA05C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DCA060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCA064: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA068: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCA06C: 4E800421  bctrl
	ctx.lr = 0x82DCA070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCA070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DCA074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCA078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCA07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCA080: 4E800020  blr
	return;
}

pub fn sub_82DCA098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA098 size=136
	// 82DCA098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA09C: 4BEDF369  bl 0x82ca9404
	ctx.lr = 0x82DCA0A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DCA0A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCA0A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DCA0AC: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82DCA0B0: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82DCA0B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DCA0B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DCA0BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
}

pub fn sub_82DCA120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA120 size=176
	// 82DCA120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCA128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCA12C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCA130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA134: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DCA138: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCA13C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCA140: 388B3294  addi r4, r11, 0x3294
	ctx.r[4].s64 = ctx.r[11].s64 + 12948;
	// 82DCA144: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DCA148: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA14C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DCA150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCA154: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCA158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCA15C: 4E800421  bctrl
	ctx.lr = 0x82DCA160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCA160: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82DCA164: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCA168: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DCA16C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DCA170: 409A0034  bne cr6, 0x82dca1a4
	if !ctx.cr[6].eq {
	pc = 0x82DCA1A4; continue 'dispatch;
	}
	// 82DCA174: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA178: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DCA17C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCA180: 5568103A  slwi r8, r11, 2
	// 82DCA184: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA188: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82DCA18C: 54E7103A  slwi r7, r7, 2
	// 82DCA190: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DCA194: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCA198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCA19C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCA1A0: 4E800421  bctrl
	ctx.lr = 0x82DCA1A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCA1A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DCA1A8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCA1AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCA1B0: 4803DE61  bl 0x82e08010
	ctx.lr = 0x82DCA1B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E08010);
	// 82DCA1B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCA1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCA1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCA1C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCA1C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCA1C8: 4E800020  blr
	return;
}

pub fn sub_82DCA1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA1D0 size=136
	// 82DCA1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCA1D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCA1DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA1E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DCA1E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCA1E8: 392A2C60  addi r9, r10, 0x2c60
	ctx.r[9].s64 = ctx.r[10].s64 + 11360;
	// 82DCA1EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DCA1F0: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DCA1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DCA1F8: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 82DCA1FC: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DCA200: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCA204: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82DCA208: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DCA20C: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82DCA210: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DCA214: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCA218: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82DCA21C: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA220: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
}

pub fn sub_82DCA258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA258 size=88
	// 82DCA258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA25C: 4BEDF1AD  bl 0x82ca9408
	ctx.lr = 0x82DCA260;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCA260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA264: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA268: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCA26C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCA270: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DCA274: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCA278: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCA27C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCA280: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCA284: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DCA288: 4BF8AFC1  bl 0x82d55248
	ctx.lr = 0x82DCA28C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCA28C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCA290: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DCA294: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DCA298: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DCA29C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DCA2A0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCA2A4: 4BFFFF2D  bl 0x82dca1d0
	ctx.lr = 0x82DCA2A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCA1D0);
	// 82DCA2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCA2AC: 4BEDF1AC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCA2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DCA2B0 size=28
	// 82DCA2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA2B4: 4BEDF12D  bl 0x82ca93e0
	ctx.lr = 0x82DCA2B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
	// 82DCA2B8: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82DCA2BC: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82DCA2C0: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
}

pub fn sub_82DCA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA890 size=304
	// 82DCA890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA894: 4BEDEB75  bl 0x82ca9408
	ctx.lr = 0x82DCA898;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCA898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA89C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DCA8A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DCA8A4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DCA8A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DCA8AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DCA8B0: 419A00BC  beq cr6, 0x82dca96c
	if ctx.cr[6].eq {
	pc = 0x82DCA96C; continue 'dispatch;
	}
	// 82DCA8B4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCA8B8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DCA8BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DCA8C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCA8C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA8C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCA8CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCA8D0: 4E800421  bctrl
	ctx.lr = 0x82DCA8D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCA8D4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82DCA8D8: 41980058  blt cr6, 0x82dca930
	if ctx.cr[6].lt {
	pc = 0x82DCA930; continue 'dispatch;
	}
	// 82DCA8DC: 419A0018  beq cr6, 0x82dca8f4
	if ctx.cr[6].eq {
	pc = 0x82DCA8F4; continue 'dispatch;
	}
	// 82DCA8E0: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82DCA8E4: 4198004C  blt cr6, 0x82dca930
	if ctx.cr[6].lt {
	pc = 0x82DCA930; continue 'dispatch;
	}
	// 82DCA8E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DCA8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCA8F0: 4BEDEB68  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 82DCA8F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA8F8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCA8FC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCA900: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DCA904: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCA908: 4BF8A941  bl 0x82d55248
	ctx.lr = 0x82DCA90C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCA90C: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DCA910: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DCA914: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DCA918: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DCA91C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCA920: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCA924: 48012E1D  bl 0x82ddd740
	ctx.lr = 0x82DCA928;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD740);
	// 82DCA928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCA92C: 4BEDEB2C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 82DCA930: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA934: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCA938: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCA93C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCA940: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCA944: 4BF8A905  bl 0x82d55248
	ctx.lr = 0x82DCA948;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCA948: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCA94C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DCA950: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DCA954: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DCA958: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCA95C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCA960: 4BFFF871  bl 0x82dca1d0
	ctx.lr = 0x82DCA964;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCA1D0);
	// 82DCA964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCA968: 4BEDEAF0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 82DCA96C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA970: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCA974: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCA978: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCA97C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCA980: 4BF8A8C9  bl 0x82d55248
	ctx.lr = 0x82DCA984;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCA984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCA988: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCA98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DCA990: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DCA994: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DCA998: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DCA99C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCA9A0: 4BFFF831  bl 0x82dca1d0
	ctx.lr = 0x82DCA9A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCA1D0);
	// 82DCA9A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCA9A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCA9AC: 396B32C8  addi r11, r11, 0x32c8
	ctx.r[11].s64 = ctx.r[11].s64 + 13000;
	// 82DCA9B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCA9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCA9B8: 4BEDEAA0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCA9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCA9C0 size=112
	// 82DCA9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCA9C4: 4BEDEA41  bl 0x82ca9404
	ctx.lr = 0x82DCA9C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DCA9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCA9CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCA9D0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCA9D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DCA9D8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DCA9DC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCA9E0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCA9E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCA9E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCA9EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DCA9F0: 4BF8A859  bl 0x82d55248
	ctx.lr = 0x82DCA9F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCA9F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCA9F8: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCA9FC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DCAA00: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DCAA04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DCAA08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCAA0C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCAA10: 4BFFF7C1  bl 0x82dca1d0
	ctx.lr = 0x82DCAA14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCA1D0);
	// 82DCAA14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCAA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCAA1C: 396B32C8  addi r11, r11, 0x32c8
	ctx.r[11].s64 = ctx.r[11].s64 + 13000;
	// 82DCAA20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCAA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCAA28: 4BEDEA2C  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DCAA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCAA30 size=208
	// 82DCAA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCAA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCAA38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCAA3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCAA40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCAA44: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCAA48: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCAA4C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCAA50: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCAA54: 3908A9C0  addi r8, r8, -0x5640
	ctx.r[8].s64 = ctx.r[8].s64 + -22080;
	// 82DCAA58: 39299BB0  addi r9, r9, -0x6450
	ctx.r[9].s64 = ctx.r[9].s64 + -25680;
	// 82DCAA5C: 394A9C00  addi r10, r10, -0x6400
	ctx.r[10].s64 = ctx.r[10].s64 + -25600;
	// 82DCAA60: 396BADC0  addi r11, r11, -0x5240
	ctx.r[11].s64 = ctx.r[11].s64 + -21056;
	// 82DCAA64: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DCAA68: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DCAA6C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DCAA70: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82DCAA74: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DCAA78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DCAA7C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCAA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCAA84: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DCAA88: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DCAA8C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DCAA90: 4BFF66D9  bl 0x82dc1168
	ctx.lr = 0x82DCAA94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCAA94: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCAA98: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DCAA9C: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCAAA0: 396BD980  addi r11, r11, -0x2680
	ctx.r[11].s64 = ctx.r[11].s64 + -9856;
	// 82DCAAA4: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCAAA8: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCAAAC: 3908A258  addi r8, r8, -0x5da8
	ctx.r[8].s64 = ctx.r[8].s64 + -23976;
	// 82DCAAB0: 3929DE18  addi r9, r9, -0x21e8
	ctx.r[9].s64 = ctx.r[9].s64 + -8680;
	// 82DCAAB4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DCAAB8: 394ADBC0  addi r10, r10, -0x2440
	ctx.r[10].s64 = ctx.r[10].s64 + -9280;
	// 82DCAABC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCAAC0: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82DCAAC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DCAAC8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DCAACC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DCAAD0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DCAAD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCAAD8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DCAADC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DCAAE0: 4BFF6689  bl 0x82dc1168
	ctx.lr = 0x82DCAAE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCAAE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DCAAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCAAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCAAF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCAAF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCAAF8: 4E800020  blr
	return;
}

pub fn sub_82DCAB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCAB00 size=208
	// 82DCAB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCAB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCAB08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCAB0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCAB10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCAB14: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCAB18: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCAB1C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCAB20: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCAB24: 3908A9C0  addi r8, r8, -0x5640
	ctx.r[8].s64 = ctx.r[8].s64 + -22080;
	// 82DCAB28: 39299BB0  addi r9, r9, -0x6450
	ctx.r[9].s64 = ctx.r[9].s64 + -25680;
	// 82DCAB2C: 394A9C00  addi r10, r10, -0x6400
	ctx.r[10].s64 = ctx.r[10].s64 + -25600;
	// 82DCAB30: 396BADC0  addi r11, r11, -0x5240
	ctx.r[11].s64 = ctx.r[11].s64 + -21056;
	// 82DCAB34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DCAB38: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DCAB3C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DCAB40: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82DCAB44: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DCAB48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DCAB4C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCAB50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCAB54: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DCAB58: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DCAB5C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DCAB60: 4BFF6609  bl 0x82dc1168
	ctx.lr = 0x82DCAB64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCAB64: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCAB68: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DCAB6C: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCAB70: 396BD980  addi r11, r11, -0x2680
	ctx.r[11].s64 = ctx.r[11].s64 + -9856;
	// 82DCAB74: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCAB78: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCAB7C: 3908A258  addi r8, r8, -0x5da8
	ctx.r[8].s64 = ctx.r[8].s64 + -23976;
	// 82DCAB80: 3929DE18  addi r9, r9, -0x21e8
	ctx.r[9].s64 = ctx.r[9].s64 + -8680;
	// 82DCAB84: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DCAB88: 394ADBC0  addi r10, r10, -0x2440
	ctx.r[10].s64 = ctx.r[10].s64 + -9280;
	// 82DCAB8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCAB90: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82DCAB94: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82DCAB98: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DCAB9C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DCABA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DCABA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCABA8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DCABAC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DCABB0: 4BFF65B9  bl 0x82dc1168
	ctx.lr = 0x82DCABB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCABB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DCABB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCABBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCABC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCABC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCABC8: 4E800020  blr
	return;
}

pub fn sub_82DCABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCABD0 size=12
	// 82DCABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCABD4: 4BEDE831  bl 0x82ca9404
	ctx.lr = 0x82DCABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DCABD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCACF0 size=208
	// 82DCACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCACF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCACFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCAD00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCAD04: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCAD08: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCAD0C: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCAD10: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCAD14: 3908ABD0  addi r8, r8, -0x5430
	ctx.r[8].s64 = ctx.r[8].s64 + -21552;
	// 82DCAD18: 39299BB0  addi r9, r9, -0x6450
	ctx.r[9].s64 = ctx.r[9].s64 + -25680;
	// 82DCAD1C: 394A9C00  addi r10, r10, -0x6400
	ctx.r[10].s64 = ctx.r[10].s64 + -25600;
	// 82DCAD20: 396BADC0  addi r11, r11, -0x5240
	ctx.r[11].s64 = ctx.r[11].s64 + -21056;
	// 82DCAD24: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DCAD28: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82DCAD2C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DCAD30: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82DCAD34: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DCAD38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DCAD3C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCAD40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCAD44: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DCAD48: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DCAD4C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DCAD50: 4BFF6419  bl 0x82dc1168
	ctx.lr = 0x82DCAD54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCAD54: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCAD58: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DCAD5C: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCAD60: 396BD980  addi r11, r11, -0x2680
	ctx.r[11].s64 = ctx.r[11].s64 + -9856;
	// 82DCAD64: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCAD68: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCAD6C: 3908A890  addi r8, r8, -0x5770
	ctx.r[8].s64 = ctx.r[8].s64 + -22384;
	// 82DCAD70: 3929DE18  addi r9, r9, -0x21e8
	ctx.r[9].s64 = ctx.r[9].s64 + -8680;
	// 82DCAD74: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DCAD78: 394ADBC0  addi r10, r10, -0x2440
	ctx.r[10].s64 = ctx.r[10].s64 + -9280;
	// 82DCAD7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCAD80: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82DCAD84: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 82DCAD88: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DCAD8C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DCAD90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DCAD94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCAD98: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DCAD9C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DCADA0: 4BFF63C9  bl 0x82dc1168
	ctx.lr = 0x82DCADA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCADA4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DCADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCADB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCADB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCADB8: 4E800020  blr
	return;
}

pub fn sub_82DCADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCADC0 size=232
	// 82DCADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCADC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCADCC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCADD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCADD4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DCADD8: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DCADDC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DCADE0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DCADE4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DCADE8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DCADEC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DCADF0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DCADF4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DCADF8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DCADFC: 4200FFF0  bdnz 0x82dcadec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DCADEC; continue 'dispatch;
	}
	// 82DCAE00: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCAE04: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DCAE08: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DCAE0C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DCAE10: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DCAE14: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DCAEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCAEA8 size=12
	// 82DCAEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCAEAC: 4BEDE55D  bl 0x82ca9408
	ctx.lr = 0x82DCAEB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCAEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCB018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB018 size=72
	// 82DCB018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB024: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB028: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DCB02C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DCB030: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DCB034: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DCB038: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DCB03C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCB040: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DCB044: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DCB048: 48002DD1  bl 0x82dcde18
	ctx.lr = 0x82DCB04C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCDE18);
	// 82DCB04C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB058: 4E800020  blr
	return;
}

pub fn sub_82DCB060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB060 size=88
	// 82DCB060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB06C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB070: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DCB074: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DCB078: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DCB07C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DCB080: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DCB084: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCB088: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCB08C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DCB090: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DCB094: 48002B2D  bl 0x82dcdbc0
	ctx.lr = 0x82DCB098;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCDBC0);
	// 82DCB098: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB0A4: 4E800020  blr
	return;
	// 82DCB0A8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB0AC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB0B0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB0B4: 4BFFEFE4  b 0x82dca098
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCA098);
	return;
}

pub fn sub_82DCB0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB0B8 size=192
	// 82DCB0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB0C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB0C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB0C8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DCB0CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB0D0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DCB0D4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB0D8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB0DC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB0E0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB0E4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DCB0E8: 4BFFF1C9  bl 0x82dca2b0
	ctx.lr = 0x82DCB0EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCA2B0);
	// 82DCB0EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB0F0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DCB0F4: 40980044  bge cr6, 0x82dcb138
	if !ctx.cr[6].lt {
	pc = 0x82DCB138; continue 'dispatch;
	}
	// 82DCB0F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB0FC: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DCB100: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
}

pub fn sub_82DCB178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB178 size=80
	// 82DCB178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB184: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB188: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB18C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB190: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB194: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DCB198: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DCB19C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCB1A0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DCB1A4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCB1A8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DCB1AC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DCB1B0: 48012041  bl 0x82ddd1f0
	ctx.lr = 0x82DCB1B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD1F0);
	// 82DCB1B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB1C0: 4E800020  blr
	return;
}

pub fn sub_82DCB1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB1C8 size=80
	// 82DCB1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB1D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB1D4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB1D8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DCB1DC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DCB1E0: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DCB1E4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB1E8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB1EC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DCB1F0: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCB1F4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB1F8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DCB1FC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DCB200: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCB204: 48011A5D  bl 0x82ddcc60
	ctx.lr = 0x82DCB208;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCC60);
	// 82DCB208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB214: 4E800020  blr
	return;
}

pub fn sub_82DCB218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB218 size=176
	// 82DCB218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB228: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DCB22C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB230: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DCB234: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB238: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB23C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB240: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB244: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DCB248: 480118F1  bl 0x82ddcb38
	ctx.lr = 0x82DCB24C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCB38);
	// 82DCB24C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB250: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DCB254: 40980044  bge cr6, 0x82dcb298
	if !ctx.cr[6].lt {
	pc = 0x82DCB298; continue 'dispatch;
	}
	// 82DCB258: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB25C: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DCB260: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
}

pub fn sub_82DCB2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB2C8 size=240
	// 82DCB2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB2D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB2D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB2D8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB2DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCB2E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB2E4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DCB2E8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DCB2EC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DCB2F0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DCB2F4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DCB2F8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DCB2FC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DCB300: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DCB304: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DCB308: 4200FFF0  bdnz 0x82dcb2f8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DCB2F8; continue 'dispatch;
	}
	// 82DCB30C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB310: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DCB314: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DCB318: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DCB31C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DCB320: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DCB3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB3B8 size=120
	// 82DCB3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB3C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB3C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB3C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB3CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCB3D0: 396B3340  addi r11, r11, 0x3340
	ctx.r[11].s64 = ctx.r[11].s64 + 13120;
	// 82DCB3D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCB3D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCB3DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DCB3E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DCB3E4: 409A002C  bne cr6, 0x82dcb410
	if !ctx.cr[6].eq {
	pc = 0x82DCB410; continue 'dispatch;
	}
	// 82DCB3E8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB3EC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DCB3F0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DCB3F4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCB3F8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DCB3FC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DCB400: 556A083C  slwi r10, r11, 1
	// 82DCB404: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCB408: 5565103A  slwi r5, r11, 2
	// 82DCB40C: 4BF89EBD  bl 0x82d552c8
	ctx.lr = 0x82DCB410;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DCB410: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DCB414: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DCB418: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCB41C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DCB420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCB42C: 4E800020  blr
	return;
}

pub fn sub_82DCB430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB430 size=64
	// 82DCB430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB43C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCB444: 480007B5  bl 0x82dcbbf8
	ctx.lr = 0x82DCB448;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCBBF8);
	// 82DCB448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB44C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCB450: 396B337C  addi r11, r11, 0x337c
	ctx.r[11].s64 = ctx.r[11].s64 + 13180;
	// 82DCB454: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCB458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DCB45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCB468: 4E800020  blr
	return;
}

pub fn sub_82DCB470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB470 size=392
	// 82DCB470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB474: 4BEDDF89  bl 0x82ca93fc
	ctx.lr = 0x82DCB478;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 82DCB478: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB47C: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB480: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DCB484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCB488: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DCB48C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DCB490: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DCB494: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DCB498: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DCB49C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCB4A0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCB4A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCB4A8: 40980020  bge cr6, 0x82dcb4c8
	if !ctx.cr[6].lt {
	pc = 0x82DCB4C8; continue 'dispatch;
	}
	// 82DCB4AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB4B0: 392933B4  addi r9, r9, 0x33b4
	ctx.r[9].s64 = ctx.r[9].s64 + 13236;
	// 82DCB4B4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCB4B8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCB4BC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DCB4C0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCB4C4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCB4C8: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82DCB4CC: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCB4D0: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCB4D4: 4BF8BE9D  bl 0x82d57370
	ctx.lr = 0x82DCB4D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
	// 82DCB4D8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB4DC: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DCB4E0: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 82DCB4E4: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82DCB4E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB4EC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DCB4F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCB4F4: 4E800421  bctrl
	ctx.lr = 0x82DCB4F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCB4F8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82DCB4FC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DCB500: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCB504: 4BF8AF75  bl 0x82d56478
	ctx.lr = 0x82DCB508;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D56478);
	// 82DCB508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DCB50C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DCB510: C1BF0004  lfs f13, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DCB514: 392100C0  addi r9, r1, 0xc0
	ctx.r[9].s64 = ctx.r[1].s64 + 192;
	// 82DCB518: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DCB51C: 93C100B4  stw r30, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82DCB520: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82DCB524: 93A100B8  stw r29, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[29].u32 ) };
	// 82DCB528: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DCB52C: C00B0BFC  lfs f0, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCB530: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DCB534: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DCB538: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82DCB53C: 396B4C40  addi r11, r11, 0x4c40
	ctx.r[11].s64 = ctx.r[11].s64 + 19520;
}

pub fn sub_82DCB5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB5F8 size=176
	// 82DCB5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB60C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB610: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DCB614: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB618: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCB61C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCB620: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCB624: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCB628: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DCB62C: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DCB630: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DCB634: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB638: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCB63C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCB640: 40980028  bge cr6, 0x82dcb668
	if !ctx.cr[6].lt {
	pc = 0x82DCB668; continue 'dispatch;
	}
	// 82DCB644: 4BF89C05  bl 0x82d55248
	ctx.lr = 0x82DCB648;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCB648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCB64C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCB650: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DCB654: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCB658: 480005A1  bl 0x82dcbbf8
	ctx.lr = 0x82DCB65C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCBBF8);
	// 82DCB65C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB660: 396B337C  addi r11, r11, 0x337c
	ctx.r[11].s64 = ctx.r[11].s64 + 13180;
	// 82DCB664: 48000024  b 0x82dcb688
	pc = 0x82DCB688; continue 'dispatch;
	// 82DCB668: 4BF89BE1  bl 0x82d55248
	ctx.lr = 0x82DCB66C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCB66C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCB670: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCB674: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DCB678: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCB67C: 4800057D  bl 0x82dcbbf8
	ctx.lr = 0x82DCB680;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCBBF8);
	// 82DCB680: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB684: 396B33C0  addi r11, r11, 0x33c0
	ctx.r[11].s64 = ctx.r[11].s64 + 13248;
	// 82DCB688: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCB68C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCB690: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB69C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCB6A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCB6A4: 4E800020  blr
	return;
}

pub fn sub_82DCB6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB6A8 size=12
	// 82DCB6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB6AC: 4BEDDD61  bl 0x82ca940c
	ctx.lr = 0x82DCB6B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DCB6B0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCB7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB7A8 size=80
	// 82DCB7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB7B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB7B4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB7B8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB7BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB7C0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB7C4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DCB7C8: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DCB7CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCB7D0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DCB7D4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCB7D8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DCB7DC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DCB7E0: 480029C9  bl 0x82dce1a8
	ctx.lr = 0x82DCB7E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE1A8);
	// 82DCB7E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB7F0: 4E800020  blr
	return;
}

pub fn sub_82DCB7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB7F8 size=80
	// 82DCB7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB804: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB808: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DCB80C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DCB810: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DCB814: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB818: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB81C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DCB820: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCB824: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB828: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DCB82C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DCB830: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCB834: 4800295D  bl 0x82dce190
	ctx.lr = 0x82DCB838;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE190);
	// 82DCB838: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB844: 4E800020  blr
	return;
}

pub fn sub_82DCB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCB848 size=176
	// 82DCB848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB858: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DCB85C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB860: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DCB864: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCB868: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB86C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCB870: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB874: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DCB878: 48001349  bl 0x82dccbc0
	ctx.lr = 0x82DCB87C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCCBC0);
	// 82DCB87C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB880: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DCB884: 40980044  bge cr6, 0x82dcb8c8
	if !ctx.cr[6].lt {
	pc = 0x82DCB8C8; continue 'dispatch;
	}
	// 82DCB888: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCB88C: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DCB890: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
}

pub fn sub_82DCB8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB8F8 size=104
	// 82DCB8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB90C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCB910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCB914: 4BFFFAA5  bl 0x82dcb3b8
	ctx.lr = 0x82DCB918;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCB3B8);
	// 82DCB918: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DCB91C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCB920: 419A0020  beq cr6, 0x82dcb940
	if ctx.cr[6].eq {
	pc = 0x82DCB940; continue 'dispatch;
	}
	// 82DCB924: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCB928: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCB92C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82DCB930: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCB934: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DCB938: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCB93C: 4BF8998D  bl 0x82d552c8
	ctx.lr = 0x82DCB940;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DCB940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCB944: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCB948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCB94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCB950: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCB954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCB958: 4E800020  blr
	return;
}

pub fn sub_82DCB960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCB960 size=272
	// 82DCB960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCB964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCB968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCB96C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCB970: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCB974: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCB978: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCB97C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DCB980: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DCB984: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DCB988: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DCB98C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DCB990: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DCB994: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DCB998: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DCB99C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DCB9A0: 4200FFF0  bdnz 0x82dcb990
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DCB990; continue 'dispatch;
	}
	// 82DCB9A4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCB9A8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DCB9AC: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DCB9B0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DCB9B4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DCB9B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DCBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCBA70 size=136
	// 82DCBA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCBA74: 4BEDD995  bl 0x82ca9408
	ctx.lr = 0x82DCBA78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCBA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCBA7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCBA80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DCBA84: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBA88: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBA8C: 5569083C  slwi r9, r11, 1
	// 82DCBA90: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DCBA94: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCBA98: 556B103A  slwi r11, r11, 2
	// 82DCBA9C: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCBAA0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DCBAA4: 419A0034  beq cr6, 0x82dcbad8
	if ctx.cr[6].eq {
	pc = 0x82DCBAD8; continue 'dispatch;
	}
	// 82DCBAA8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBAAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCBAB0: 419A001C  beq cr6, 0x82dcbacc
	if ctx.cr[6].eq {
	pc = 0x82DCBACC; continue 'dispatch;
	}
	// 82DCBAB4: 5563003E  slwi r3, r11, 0
	// 82DCBAB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DCBABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBAC0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DCBAC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBAC8: 4E800421  bctrl
	ctx.lr = 0x82DCBACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBACC: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DCBAD0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DCBAD4: 409AFFD4  bne cr6, 0x82dcbaa8
	if !ctx.cr[6].eq {
	pc = 0x82DCBAA8; continue 'dispatch;
	}
	// 82DCBAD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBADC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DCBAE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCBAE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBAE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBAEC: 4E800421  bctrl
	ctx.lr = 0x82DCBAF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCBAF4: 4BEDD964  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCBAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCBAF8 size=112
	// 82DCBAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCBAFC: 4BEDD911  bl 0x82ca940c
	ctx.lr = 0x82DCBB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DCBB00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCBB04: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBB08: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DCBB0C: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBB10: 5569083C  slwi r9, r11, 1
	// 82DCBB14: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DCBB18: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCBB1C: 556B103A  slwi r11, r11, 2
	// 82DCBB20: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCBB24: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DCBB28: 419A0034  beq cr6, 0x82dcbb5c
	if ctx.cr[6].eq {
	pc = 0x82DCBB5C; continue 'dispatch;
	}
	// 82DCBB2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBB30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCBB34: 419A001C  beq cr6, 0x82dcbb50
	if ctx.cr[6].eq {
	pc = 0x82DCBB50; continue 'dispatch;
	}
	// 82DCBB38: 5563003E  slwi r3, r11, 0
	// 82DCBB3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCBB40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBB44: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DCBB48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBB4C: 4E800421  bctrl
	ctx.lr = 0x82DCBB50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBB50: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DCBB54: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DCBB58: 409AFFD4  bne cr6, 0x82dcbb2c
	if !ctx.cr[6].eq {
	pc = 0x82DCBB2C; continue 'dispatch;
	}
	// 82DCBB5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCBB60: 4BEDD8FC  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DCBB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCBB68 size=240
	// 82DCBB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCBB6C: 4BEDD8A1  bl 0x82ca940c
	ctx.lr = 0x82DCBB70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DCBB70: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82DCBB74: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82DCBB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCBB7C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBB80: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DCBB84: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBB88: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82DCBB8C: 5569083C  slwi r9, r11, 1
	// 82DCBB90: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DCBB94: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCBB98: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DCBB9C: 556B103A  slwi r11, r11, 2
	// 82DCBBA0: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCBBA4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DCBBA8: 419A003C  beq cr6, 0x82dcbbe4
	if ctx.cr[6].eq {
	pc = 0x82DCBBE4; continue 'dispatch;
	}
	// 82DCBBAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBBB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCBBB4: 419A0024  beq cr6, 0x82dcbbd8
	if ctx.cr[6].eq {
	pc = 0x82DCBBD8; continue 'dispatch;
	}
	// 82DCBBB8: 5563003E  slwi r3, r11, 0
	// 82DCBBBC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82DCBBC0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DCBBC4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DCBBC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBBCC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DCBBD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBBD4: 4E800421  bctrl
	ctx.lr = 0x82DCBBD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBBD8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DCBBDC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DCBBE0: 409AFFCC  bne cr6, 0x82dcbbac
	if !ctx.cr[6].eq {
	pc = 0x82DCBBAC; continue 'dispatch;
	}
	// 82DCBBE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCBBE8: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82DCBBEC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82DCBBF0: 4BEDD86C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DCBC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCBC58 size=248
	// 82DCBC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCBC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCBC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCBC64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCBC68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBC6C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCBC70: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCBC74: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCBC78: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DCBC7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCBC80: 4BF895C9  bl 0x82d55248
	ctx.lr = 0x82DCBC84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCBC84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCBC88: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DCBC8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DCBC90: 392B3340  addi r9, r11, 0x3340
	ctx.r[9].s64 = ctx.r[11].s64 + 13120;
	// 82DCBC94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DCBC98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DCBC9C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DCBCA0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DCBCA4: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82DCBCA8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCBCAC: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCBCB0: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCBCB4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCBCB8: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82DCBCBC: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DCBCC0: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DCBCC4: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DCBCC8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
}

pub fn sub_82DCBD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCBD50 size=472
	// 82DCBD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCBD54: 4BEDD6A1  bl 0x82ca93f4
	ctx.lr = 0x82DCBD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
	// 82DCBD58: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCBD5C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DCBD60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCBD64: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DCBD68: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DCBD6C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DCBD70: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBD74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBD78: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCBD7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBD80: 4E800421  bctrl
	ctx.lr = 0x82DCBD84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBD84: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBD88: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DCBD8C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82DCBD90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DCBD94: 4099018C  ble cr6, 0x82dcbf20
	if !ctx.cr[6].gt {
	pc = 0x82DCBF20; continue 'dispatch;
	}
	// 82DCBD98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DCBD9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBDA0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DCBDA4: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBDA8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DCBDAC: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCBDB0: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCBDB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DCBDB8: 4E800421  bctrl
	ctx.lr = 0x82DCBDBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBDBC: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBDC0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBDC4: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82DCBDC8: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82DCBDCC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DCBDD0: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBDD4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DCBDD8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DCBDDC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DCBDE0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DCBDE4: 7D3E582E  lwzx r9, r30, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCBDE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DCBDEC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DCBDF0: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82DCBDF4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBDF8: 7D3E582E  lwzx r9, r30, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCBDFC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCBE00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBE04: 4E800421  bctrl
	ctx.lr = 0x82DCBE08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBE08: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBE0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCBE10: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBE14: 7F7E5A14  add r27, r30, r11
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DCBE18: 419A00B4  beq cr6, 0x82dcbecc
	if ctx.cr[6].eq {
	pc = 0x82DCBECC; continue 'dispatch;
	}
	// 82DCBE1C: 4BFF610D  bl 0x82dc1f28
	ctx.lr = 0x82DCBE20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1F28);
	// 82DCBE20: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBE24: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82DCBE28: 409A0074  bne cr6, 0x82dcbe9c
	if !ctx.cr[6].eq {
	pc = 0x82DCBE9C; continue 'dispatch;
	}
	// 82DCBE2C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBE30: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBE34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DCBE38: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DCBE3C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBE40: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBE44: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBE48: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBE4C: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82DCBE50: 409A0008  bne cr6, 0x82dcbe58
	if !ctx.cr[6].eq {
	pc = 0x82DCBE58; continue 'dispatch;
	}
	// 82DCBE54: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	// 82DCBE58: 556B2834  slwi r11, r11, 5
	// 82DCBE5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DCBE60: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCBE64: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DCBE68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DCBE6C: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DCBE70: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DCBE74: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCBE78: 556B103A  slwi r11, r11, 2
	// 82DCBE7C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCBE80: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DCBE84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBE88: 4E800421  bctrl
	ctx.lr = 0x82DCBE8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBE8C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBE90: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DCBE94: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DCBE98: 48000074  b 0x82dcbf0c
	pc = 0x82DCBF0C; continue 'dispatch;
	// 82DCBE9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBEA0: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DCBEA4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DCBEA8: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DCBEAC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DCBEB0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DCBEB4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBEB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBEBC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DCBEC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBEC4: 4E800421  bctrl
	ctx.lr = 0x82DCBEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBEC8: 48000044  b 0x82dcbf0c
	pc = 0x82DCBF0C; continue 'dispatch;
	// 82DCBECC: 4BFF605D  bl 0x82dc1f28
	ctx.lr = 0x82DCBED0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1F28);
	// 82DCBED0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBED4: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82DCBED8: 419A0034  beq cr6, 0x82dcbf0c
	if ctx.cr[6].eq {
	pc = 0x82DCBF0C; continue 'dispatch;
	}
	// 82DCBEDC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBEE0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82DCBEE4: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DCBEE8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBEEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBEF0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DCBEF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBEF8: 4E800421  bctrl
	ctx.lr = 0x82DCBEFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBEFC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBF00: 7F7E5A14  add r27, r30, r11
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DCBF04: 4BFF6025  bl 0x82dc1f28
	ctx.lr = 0x82DCBF08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1F28);
	// 82DCBF08: 907B0008  stw r3, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DCBF0C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBF10: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82DCBF14: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82DCBF18: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DCBF1C: 4198FE80  blt cr6, 0x82dcbd9c
	if ctx.cr[6].lt {
	pc = 0x82DCBD9C; continue 'dispatch;
	}
	// 82DCBF20: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DCBF24: 4BEDD520  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
}

pub fn sub_82DCBF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCBF28 size=248
	// 82DCBF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCBF2C: 4BEDD4DD  bl 0x82ca9408
	ctx.lr = 0x82DCBF30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCBF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCBF34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCBF38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCBF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCBF40: 388B3404  addi r4, r11, 0x3404
	ctx.r[4].s64 = ctx.r[11].s64 + 13316;
	// 82DCBF44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DCBF48: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBF4C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DCBF50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCBF54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCBF58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCBF5C: 4E800421  bctrl
	ctx.lr = 0x82DCBF60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBF60: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCBF64: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DCBF68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DCBF6C: 409A0048  bne cr6, 0x82dcbfb4
	if !ctx.cr[6].eq {
	pc = 0x82DCBFB4; continue 'dispatch;
	}
	// 82DCBF70: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBF74: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCBF78: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBF7C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DCBF80: 388933F8  addi r4, r9, 0x33f8
	ctx.r[4].s64 = ctx.r[9].s64 + 13304;
	// 82DCBF84: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBF88: 5569083C  slwi r9, r11, 1
	// 82DCBF8C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DCBF90: 83A80008  lwz r29, 8(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBF94: 5548083C  slwi r8, r10, 1
	// 82DCBF98: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCBF9C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DCBFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCBFA4: 5548103A  slwi r8, r10, 2
	// 82DCBFA8: 5567103A  slwi r7, r11, 2
	// 82DCBFAC: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82DCBFB0: 4E800421  bctrl
	ctx.lr = 0x82DCBFB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCBFB4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCBFB8: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCBFBC: 5569083C  slwi r9, r11, 1
	// 82DCBFC0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82DCBFC4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCBFC8: 556B103A  slwi r11, r11, 2
	// 82DCBFCC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCBFD0: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DCBFD4: 419A0040  beq cr6, 0x82dcc014
	if ctx.cr[6].eq {
	pc = 0x82DCC014; continue 'dispatch;
	}
	// 82DCBFD8: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82DCBFDC: 3B8B4D6C  addi r28, r11, 0x4d6c
	ctx.r[28].s64 = ctx.r[11].s64 + 19820;
	// 82DCBFE0: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCBFE4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DCBFE8: 419A0020  beq cr6, 0x82dcc008
	if ctx.cr[6].eq {
	pc = 0x82DCC008; continue 'dispatch;
	}
	// 82DCBFEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCBFF0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DCBFF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DCBFF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCBFFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCC000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCC004: 4E800421  bctrl
	ctx.lr = 0x82DCC008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCC008: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82DCC00C: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DCC010: 409AFFD0  bne cr6, 0x82dcbfe0
	if !ctx.cr[6].eq {
	pc = 0x82DCBFE0; continue 'dispatch;
	}
	// 82DCC014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCC018: 4BEDD440  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCC020 size=176
	// 82DCC020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCC024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCC028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCC02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCC030: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCC034: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCC038: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCC03C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCC040: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DCC044: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCC048: 4BF89201  bl 0x82d55248
	ctx.lr = 0x82DCC04C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCC04C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCC050: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DCC054: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82DCC058: 396B3340  addi r11, r11, 0x3340
	ctx.r[11].s64 = ctx.r[11].s64 + 13120;
	// 82DCC05C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DCC060: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DCC064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DCC068: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 82DCC06C: B0E30004  sth r7, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82DCC070: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCC074: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82DCC078: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DCC07C: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCC080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DCC084: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DCC088: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DCC08C: 392A3414  addi r9, r10, 0x3414
	ctx.r[9].s64 = ctx.r[10].s64 + 13332;
	// 82DCC090: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82DCC094: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
}

pub fn sub_82DCC0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCC0D0 size=328
	// 82DCC0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCC0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCC0D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCC0DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCC0E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCC0E4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DCC0E8: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCC0EC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCC0F0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82DCC0F4: C00B00A0  lfs f0, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCC0F8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCC0FC: C1AA00A0  lfs f13, 0xa0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DCC100: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCC104: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DCC108: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCC10C: 40980080  bge cr6, 0x82dcc18c
	if !ctx.cr[6].lt {
	pc = 0x82DCC18C; continue 'dispatch;
	}
	// 82DCC110: 4BF89139  bl 0x82d55248
	ctx.lr = 0x82DCC114;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCC114: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCC118: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DCC11C: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DCC120: 390B3340  addi r8, r11, 0x3340
	ctx.r[8].s64 = ctx.r[11].s64 + 13120;
	// 82DCC124: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DCC128: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DCC12C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DCC130: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DCC134: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82DCC138: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DCC13C: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCC140: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82DCC144: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCC148: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82DCC14C: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DCC150: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DCC154: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DCC158: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
}

pub fn sub_82DCC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCC218 size=2060
	// 82DCC218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCC21C: 4BEDD1C1  bl 0x82ca93dc
	ctx.lr = 0x82DCC220;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
	// 82DCC220: 9421FA80  stwu r1, -0x580(r1)
	ea = ctx.r[1].u32.wrapping_add(-1408 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCC224: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82DCC228: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 82DCC22C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCC230: 386104C0  addi r3, r1, 0x4c0
	ctx.r[3].s64 = ctx.r[1].s64 + 1216;
	// 82DCC234: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DCC238: 80B30008  lwz r5, 8(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCC23C: 7CF23B78  mr r18, r7
	ctx.r[18].u64 = ctx.r[7].u64;
	// 82DCC240: 80910008  lwz r4, 8(r17)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCC244: 4BF8B12D  bl 0x82d57370
	ctx.lr = 0x82DCC248;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
	// 82DCC248: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DCC24C: 80730000  lwz r3, 0(r19)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCC250: C0170004  lfs f0, 4(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCC254: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DCC258: 388104C0  addi r4, r1, 0x4c0
	ctx.r[4].s64 = ctx.r[1].s64 + 1216;
	// 82DCC25C: C1AB0BFC  lfs f13, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DCC260: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCC264: EC20037A  fmadds f1, f0, f13, f0
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82DCC268: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DCC26C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCC270: 4E800421  bctrl
	ctx.lr = 0x82DCC274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCC274: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DCC278: 896BB880  lbz r11, -0x4780(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-18304 as u32) ) } as u64;
	// 82DCC27C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCC280: 419A0064  beq cr6, 0x82dcc2e4
	if ctx.cr[6].eq {
	pc = 0x82DCC2E4; continue 'dispatch;
	}
	// 82DCC284: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DCC288: 113F038C  vspltisw v9, -1
	for i in 0..4 {
		ctx.v[9].u32[i] = 4294967295;
	}
	// 82DCC28C: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82DCC290: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DCC294: 390000E0  li r8, 0xe0
	ctx.r[8].s64 = 224;
}

pub fn sub_82DCCBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DCCBC0 size=3520
	// 82DCCBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCCBC4: 4BEDC80D  bl 0x82ca93d0
	ctx.lr = 0x82DCCBC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
	// 82DCCBC8: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DCCBCC: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DCCBD0: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
}

pub fn sub_82DCD980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCD980 size=12
	// 82DCD980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCD984: 4BEDBA69  bl 0x82ca93ec
	ctx.lr = 0x82DCD988;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93EC);
	// 82DCD988: 9421FB00  stwu r1, -0x500(r1)
	ea = ctx.r[1].u32.wrapping_add(-1280 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCDBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCDBC0 size=600
	// 82DCDBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCDBC4: 4BEDB82D  bl 0x82ca93f0
	ctx.lr = 0x82DCDBC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F0);
	// 82DCDBC8: 9421FAC0  stwu r1, -0x540(r1)
	ea = ctx.r[1].u32.wrapping_add(-1344 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCDBCC: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDBD0: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	// 82DCDBD4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DCDBD8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DCDBDC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DCDBE0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DCDBE4: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DCDBE8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCDBEC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCDBF0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCDBF4: 4098002C  bge cr6, 0x82dcdc20
	if !ctx.cr[6].lt {
	pc = 0x82DCDC20; continue 'dispatch;
	}
	// 82DCDBF8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCDBFC: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DCDC00: 3929345C  addi r9, r9, 0x345c
	ctx.r[9].s64 = ctx.r[9].s64 + 13404;
	// 82DCDC04: 390832AC  addi r8, r8, 0x32ac
	ctx.r[8].s64 = ctx.r[8].s64 + 12972;
	// 82DCDC08: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCDC0C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DCDC10: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCDC14: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DCDC18: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCDC1C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCDC20: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82DCDC24: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCDC28: 809A0008  lwz r4, 8(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCDC2C: 4BF89745  bl 0x82d57370
	ctx.lr = 0x82DCDC30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
	// 82DCDC30: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDC34: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DCDC38: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DCDC3C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82DCDC40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDC44: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DCDC48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCDC4C: 4E800421  bctrl
	ctx.lr = 0x82DCDC50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCDC50: 396100DC  addi r11, r1, 0xdc
	ctx.r[11].s64 = ctx.r[1].s64 + 220;
	// 82DCDC54: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDC58: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 82DCDC5C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DCDC60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DCDC64: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82DCDC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCDC6C: 916100D4  stw r11, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 82DCDC70: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DCDC74: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82DCDC78: 916100D8  stw r11, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 82DCDC7C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDC80: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DCDC84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCDC88: 4E800421  bctrl
	ctx.lr = 0x82DCDC8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCDC8C: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DCDC90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCDC94: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCDC98: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCDC9C: 40980020  bge cr6, 0x82dcdcbc
	if !ctx.cr[6].lt {
	pc = 0x82DCDCBC; continue 'dispatch;
	}
	// 82DCDCA0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCDCA4: 3929344C  addi r9, r9, 0x344c
	ctx.r[9].s64 = ctx.r[9].s64 + 13388;
	// 82DCDCA8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCDCAC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCDCB0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DCDCB4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCDCB8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCDCBC: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82DCDCC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DCDCC4: 83E100D0  lwz r31, 0xd0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82DCDCC8: 556B103A  slwi r11, r11, 2
	// 82DCDCCC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDCD0: 7F0BFA14  add r24, r11, r31
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DCDCD4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCDCD8: 836A000C  lwz r27, 0xc(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCDCDC: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82DCDCE0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DCDCE4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDCE8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCDCEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCDCF0: 4E800421  bctrl
	ctx.lr = 0x82DCDCF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCDCF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DCDCF8: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DCDCFC: 419A00B8  beq cr6, 0x82dcddb4
	if ctx.cr[6].eq {
	pc = 0x82DCDDB4; continue 'dispatch;
	}
	// 82DCDD00: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCDD04: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DCDD08: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD0C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DCDD10: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DCDD14: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DCDD18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DCDD1C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCDD24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCDD28: 4E800421  bctrl
	ctx.lr = 0x82DCDD2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCDD2C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCDD34: 419A0074  beq cr6, 0x82dcdda8
	if ctx.cr[6].eq {
	pc = 0x82DCDDA8; continue 'dispatch;
	}
	// 82DCDD38: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD3C: 38A102E0  addi r5, r1, 0x2e0
	ctx.r[5].s64 = ctx.r[1].s64 + 736;
	// 82DCDD40: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DCDD48: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCDD4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCDD50: 4E800421  bctrl
	ctx.lr = 0x82DCDD54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCDD54: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DCDD5C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCDD60: 391B000D  addi r8, r27, 0xd
	ctx.r[8].s64 = ctx.r[27].s64 + 13;
	// 82DCDD64: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82DCDD68: 55082834  slwi r8, r8, 5
	// 82DCDD6C: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82DCDD70: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DCDD74: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DCDD78: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DCDD7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCDD80: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DCDD84: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DCDD88: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DCDD8C: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DCDD90: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DCDD94: 556B103A  slwi r11, r11, 2
	// 82DCDD98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DCDD9C: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82DCDDA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCDDA4: 4E800421  bctrl
	ctx.lr = 0x82DCDDA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCDDA8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DCDDAC: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DCDDB0: 409AFF50  bne cr6, 0x82dcdd00
	if !ctx.cr[6].eq {
	pc = 0x82DCDD00; continue 'dispatch;
	}
	// 82DCDDB4: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DCDDB8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCDDBC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCDDC0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCDDC4: 40980020  bge cr6, 0x82dcdde4
	if !ctx.cr[6].lt {
	pc = 0x82DCDDE4; continue 'dispatch;
	}
	// 82DCDDC8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DCDDCC: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DCDDD0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCDDD4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCDDD8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DCDDDC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCDDE0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCDDE4: 816100D8  lwz r11, 0xd8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 82DCDDE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DCDDEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DCDDF0: 409A001C  bne cr6, 0x82dcde0c
	if !ctx.cr[6].eq {
	pc = 0x82DCDE0C; continue 'dispatch;
	}
	// 82DCDDF4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCDDF8: 808100D0  lwz r4, 0xd0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82DCDDFC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DCDE00: 5565103A  slwi r5, r11, 2
	// 82DCDE04: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DCDE08: 4BF874C1  bl 0x82d552c8
	ctx.lr = 0x82DCDE0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DCDE0C: 38210540  addi r1, r1, 0x540
	ctx.r[1].s64 = ctx.r[1].s64 + 1344;
	// 82DCDE10: 4BEDB630  b 0x82ca9440
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9440);
	return;
}

pub fn sub_82DCDE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCDE18 size=12
	// 82DCDE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCDE1C: 4BEDB5D5  bl 0x82ca93f0
	ctx.lr = 0x82DCDE20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F0);
	// 82DCDE20: 9421FAC0  stwu r1, -0x540(r1)
	ea = ctx.r[1].u32.wrapping_add(-1344 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCE078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE078 size=12
	// 82DCE078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE07C: 4BEDB391  bl 0x82ca940c
	ctx.lr = 0x82DCE080;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DCE080: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCE1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE1D0 size=232
	// 82DCE1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCE1D8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCE1DC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DCE1E0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DCE1E4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DCE1E8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DCE1EC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DCE1F0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DCE1F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DCE1F8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DCE1FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DCE200: 4200FFF0  bdnz 0x82dce1f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DCE1F0; continue 'dispatch;
	}
	// 82DCE204: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCE208: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DCE20C: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DCE210: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82DCE214: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DCE218: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DCE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE2B8 size=144
	// 82DCE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE2BC: 4BEDB14D  bl 0x82ca9408
	ctx.lr = 0x82DCE2C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCE2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCE2C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCE2C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DCE2CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCE2D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCE2D4: 419A0054  beq cr6, 0x82dce328
	if ctx.cr[6].eq {
	pc = 0x82DCE328; continue 'dispatch;
	}
	// 82DCE2D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCE2DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DCE2E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DCE2E4: 40990044  ble cr6, 0x82dce328
	if !ctx.cr[6].gt {
	pc = 0x82DCE328; continue 'dispatch;
	}
	// 82DCE2E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DCE2EC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCE2F0: 7C8BF22E  lhzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DCE2F4: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82DCE2F8: 419A001C  beq cr6, 0x82dce314
	if ctx.cr[6].eq {
	pc = 0x82DCE314; continue 'dispatch;
	}
	// 82DCE2FC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCE300: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DCE304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE308: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCE30C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCE310: 4E800421  bctrl
	ctx.lr = 0x82DCE314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCE314: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCE318: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DCE31C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DCE320: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DCE324: 4198FFC8  blt cr6, 0x82dce2ec
	if ctx.cr[6].lt {
	pc = 0x82DCE2EC; continue 'dispatch;
	}
	// 82DCE328: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE32C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DCE330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCE334: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCE33C: 4E800421  bctrl
	ctx.lr = 0x82DCE340;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCE340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCE344: 4BEDB114  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCE348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE348 size=224
	// 82DCE348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE34C: 4BEDB0BD  bl 0x82ca9408
	ctx.lr = 0x82DCE350;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCE350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCE354: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCE358: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DCE35C: 396B3490  addi r11, r11, 0x3490
	ctx.r[11].s64 = ctx.r[11].s64 + 13456;
	// 82DCE360: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DCE364: 3BFC000C  addi r31, r28, 0xc
	ctx.r[31].s64 = ctx.r[28].s64 + 12;
	// 82DCE368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DCE36C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DCE370: 90FC0008  stw r7, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DCE374: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCE378: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DCE37C: B13C0006  sth r9, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DCE380: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DCE384: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DCE388: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DCE38C: 419A008C  beq cr6, 0x82dce418
	if ctx.cr[6].eq {
	pc = 0x82DCE418; continue 'dispatch;
	}
	// 82DCE390: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE394: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE398: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DCE39C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCE3A0: 4E800421  bctrl
	ctx.lr = 0x82DCE3A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCE3A4: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCE3A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCE3AC: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DCE3B0: 40990064  ble cr6, 0x82dce414
	if !ctx.cr[6].gt {
	pc = 0x82DCE414; continue 'dispatch;
	}
	// 82DCE3B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCE3B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DCE3BC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DCE3C0: 40980024  bge cr6, 0x82dce3e4
	if !ctx.cr[6].lt {
	pc = 0x82DCE3E4; continue 'dispatch;
	}
	// 82DCE3C4: 556B083C  slwi r11, r11, 1
	// 82DCE3C8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DCE3CC: 41980008  blt cr6, 0x82dce3d4
	if ctx.cr[6].lt {
	pc = 0x82DCE3D4; continue 'dispatch;
	}
	// 82DCE3D0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DCE3D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DCE3D8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DCE3DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCE3E0: 4BF88B31  bl 0x82d56f10
	ctx.lr = 0x82DCE3E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D56F10);
	// 82DCE3E4: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DCE3E8: 4098002C  bge cr6, 0x82dce414
	if !ctx.cr[6].lt {
	pc = 0x82DCE414; continue 'dispatch;
	}
	// 82DCE3EC: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82DCE3F0: 57AA083C  slwi r10, r29, 1
	// 82DCE3F4: 7D7DF050  subf r11, r29, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82DCE3F8: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82DCE3FC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE400: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DCE404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCE408: 7D2A432E  sthx r9, r10, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u16) };
	// 82DCE40C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DCE410: 409AFFEC  bne cr6, 0x82dce3fc
	if !ctx.cr[6].eq {
	pc = 0x82DCE3FC; continue 'dispatch;
	}
	// 82DCE414: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DCE418: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DCE41C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCE420: 4BEDB038  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCE428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE428 size=88
	// 82DCE428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE42C: 4BEDAFDD  bl 0x82ca9408
	ctx.lr = 0x82DCE430;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DCE430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCE434: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE438: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCE43C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCE440: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DCE444: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCE448: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82DCE44C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCE450: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCE454: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DCE458: 4BF86DF1  bl 0x82d55248
	ctx.lr = 0x82DCE45C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCE45C: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82DCE460: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DCE464: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DCE468: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DCE46C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DCE470: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCE474: 4BFFFED5  bl 0x82dce348
	ctx.lr = 0x82DCE478;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE348);
	// 82DCE478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCE47C: 4BEDAFDC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DCE480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE480 size=12
	// 82DCE480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE484: 4BEDAF61  bl 0x82ca93e4
	ctx.lr = 0x82DCE488;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
	// 82DCE488: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DCE780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCE780 size=1040
	// 82DCE780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCE784: 4BEDAC5D  bl 0x82ca93e0
	ctx.lr = 0x82DCE788;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
	// 82DCE788: DBE1FF80  stfd f31, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82DCE78C: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCE790: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE794: 3A400008  li r18, 8
	ctx.r[18].s64 = 8;
	// 82DCE798: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82DCE79C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82DCE7A0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DCE7A4: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82DCE7A8: 7D58902E  lwzx r10, r24, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82DCE7AC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCE7B0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCE7B4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCE7B8: 4098002C  bge cr6, 0x82dce7e4
	if !ctx.cr[6].lt {
	pc = 0x82DCE7E4; continue 'dispatch;
	}
	// 82DCE7BC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCE7C0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DCE7C4: 39293500  addi r9, r9, 0x3500
	ctx.r[9].s64 = ctx.r[9].s64 + 13568;
	// 82DCE7C8: 39083544  addi r8, r8, 0x3544
	ctx.r[8].s64 = ctx.r[8].s64 + 13636;
	// 82DCE7CC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCE7D0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DCE7D4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCE7D8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DCE7DC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCE7E0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCE7E4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82DCE7E8: 83F50000  lwz r31, 0(r21)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE7EC: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE7F0: 80B50008  lwz r5, 8(r21)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCE7F4: 80970008  lwz r4, 8(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCE7F8: 4BF88B79  bl 0x82d57370
	ctx.lr = 0x82DCE7FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
	// 82DCE7FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCE804: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DCE808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCE80C: 4E800421  bctrl
	ctx.lr = 0x82DCE810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCE810: 3A600004  li r19, 4
	ctx.r[19].s64 = 4;
	// 82DCE814: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DCE818: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82DCE81C: 7C78982E  lwzx r3, r24, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82DCE820: 557E2036  slwi r30, r11, 4
	// 82DCE824: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DCE828: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DCE82C: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DCE830: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCE834: 41990010  bgt cr6, 0x82dce844
	if ctx.cr[6].gt {
	pc = 0x82DCE844; continue 'dispatch;
	}
	// 82DCE838: 7D765B78  mr r22, r11
	ctx.r[22].u64 = ctx.r[11].u64;
	// 82DCE83C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DCE840: 4800001C  b 0x82dce85c
	pc = 0x82DCE85C; continue 'dispatch;
	// 82DCE844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE848: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DCE84C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCE850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCE854: 4E800421  bctrl
	ctx.lr = 0x82DCE858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCE858: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DCE85C: 7D58902E  lwzx r10, r24, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82DCE860: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCE864: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCE868: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCE86C: 40980020  bge cr6, 0x82dce88c
	if !ctx.cr[6].lt {
	pc = 0x82DCE88C; continue 'dispatch;
	}
	// 82DCE870: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCE874: 39293534  addi r9, r9, 0x3534
	ctx.r[9].s64 = ctx.r[9].s64 + 13620;
	// 82DCE878: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCE87C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCE880: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DCE884: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCE888: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCE88C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCE890: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DCE894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCE898: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DCE89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCE8A0: 4E800421  bctrl
	ctx.lr = 0x82DCE8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCE8A4: 7D58902E  lwzx r10, r24, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82DCE8A8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCE8AC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCE8B0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCE8B4: 40980020  bge cr6, 0x82dce8d4
	if !ctx.cr[6].lt {
	pc = 0x82DCE8D4; continue 'dispatch;
	}
	// 82DCE8B8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCE8BC: 39293528  addi r9, r9, 0x3528
	ctx.r[9].s64 = ctx.r[9].s64 + 13608;
	// 82DCE8C0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCE8C4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCE8C8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DCE8CC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCE8D0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCE8D4: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82DCE8D8: 3B3DFFFF  addi r25, r29, -1
	ctx.r[25].s64 = ctx.r[29].s64 + -1;
	// 82DCE8DC: 7D23B050  subf r9, r3, r22
	ctx.r[9].s64 = ctx.r[22].s64 - ctx.r[3].s64;
}

pub fn sub_82DCEB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DCEB90 size=920
	// 82DCEB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCEB94: 4BEDA84D  bl 0x82ca93e0
	ctx.lr = 0x82DCEB98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
	// 82DCEB98: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
}

pub fn sub_82DCEF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCEF28 size=112
	// 82DCEF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCEF2C: 4BEDA4D9  bl 0x82ca9404
	ctx.lr = 0x82DCEF30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DCEF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCEF34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCEF38: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCEF3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DCEF40: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DCEF44: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DCEF48: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82DCEF4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCEF50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCEF54: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DCEF58: 4BF862F1  bl 0x82d55248
	ctx.lr = 0x82DCEF5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DCEF5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCEF60: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82DCEF64: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DCEF68: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DCEF6C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DCEF70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCEF74: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DCEF78: 4BFFF3D1  bl 0x82dce348
	ctx.lr = 0x82DCEF7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE348);
	// 82DCEF7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCEF80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCEF84: 396B3570  addi r11, r11, 0x3570
	ctx.r[11].s64 = ctx.r[11].s64 + 13680;
	// 82DCEF88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCEF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCEF90: 4BEDA4C4  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DCEF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCEF98 size=208
	// 82DCEF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCEF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCEFA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCEFA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCEFA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCEFAC: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCEFB0: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCEFB4: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCEFB8: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCEFBC: 3908EF28  addi r8, r8, -0x10d8
	ctx.r[8].s64 = ctx.r[8].s64 + -4312;
	// 82DCEFC0: 3929F550  addi r9, r9, -0xab0
	ctx.r[9].s64 = ctx.r[9].s64 + -2736;
	// 82DCEFC4: 394AF5A0  addi r10, r10, -0xa60
	ctx.r[10].s64 = ctx.r[10].s64 + -2656;
	// 82DCEFC8: 396BF5F0  addi r11, r11, -0xa10
	ctx.r[11].s64 = ctx.r[11].s64 + -2576;
	// 82DCEFCC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DCEFD0: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82DCEFD4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DCEFD8: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82DCEFDC: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DCEFE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DCEFE4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCEFE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCEFEC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DCEFF0: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DCEFF4: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DCEFF8: 4BFF2171  bl 0x82dc1168
	ctx.lr = 0x82DCEFFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCEFFC: 3D6082DD  lis r11, -0x7d23
	ctx.r[11].s64 = -2099445760;
	// 82DCF000: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DCF004: 3D0082DD  lis r8, -0x7d23
	ctx.r[8].s64 = -2099445760;
	// 82DCF008: 396BEB90  addi r11, r11, -0x1470
	ctx.r[11].s64 = ctx.r[11].s64 + -5232;
	// 82DCF00C: 3D2082DD  lis r9, -0x7d23
	ctx.r[9].s64 = -2099445760;
	// 82DCF010: 3D4082DD  lis r10, -0x7d23
	ctx.r[10].s64 = -2099445760;
	// 82DCF014: 3908E428  addi r8, r8, -0x1bd8
	ctx.r[8].s64 = ctx.r[8].s64 + -7128;
	// 82DCF018: 3929E480  addi r9, r9, -0x1b80
	ctx.r[9].s64 = ctx.r[9].s64 + -7040;
	// 82DCF01C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DCF020: 394AE780  addi r10, r10, -0x1880
	ctx.r[10].s64 = ctx.r[10].s64 + -6272;
	// 82DCF024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCF028: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82DCF02C: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82DCF030: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DCF034: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DCF038: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DCF03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCF040: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DCF044: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DCF048: 4BFF2121  bl 0x82dc1168
	ctx.lr = 0x82DCF04C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DCF04C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DCF050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF058: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCF05C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCF060: 4E800020  blr
	return;
}

pub fn sub_82DCF068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF068 size=72
	// 82DCF068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCF074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF078: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCF07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCF080: 396B2B14  addi r11, r11, 0x2b14
	ctx.r[11].s64 = ctx.r[11].s64 + 11028;
	// 82DCF084: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DCF088: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DCF08C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCF090: 419A000C  beq cr6, 0x82dcf09c
	if ctx.cr[6].eq {
	pc = 0x82DCF09C; continue 'dispatch;
	}
	// 82DCF094: 4BA7671D  bl 0x828457b0
	ctx.lr = 0x82DCF098;
	crate::recompiler::externs::call(&mut ctx, base, 0x828457B0);
	// 82DCF098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCF09C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DCF0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF0A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCF0AC: 4E800020  blr
	return;
}

pub fn sub_82DCF0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF0B0 size=240
	// 82DCF0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF0B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCF0BC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF0C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
}

pub fn sub_82DCF1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF1A0 size=944
	// 82DCF1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF1A4: 4BEDA22D  bl 0x82ca93d0
	ctx.lr = 0x82DCF1A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
	// 82DCF1A8: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DCF1AC: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF1B0: 81ED0000  lwz r15, 0(r13)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF1B4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DCF1B8: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DCF1BC: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82DCF1C0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DCF1C4: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82DCF1C8: 7D6F582E  lwzx r11, r15, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCF1CC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DCF1D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCF1D4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCF1D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCF1DC: 40980020  bge cr6, 0x82dcf1fc
	if !ctx.cr[6].lt {
	pc = 0x82DCF1FC; continue 'dispatch;
	}
	// 82DCF1E0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCF1E4: 392934C8  addi r9, r9, 0x34c8
	ctx.r[9].s64 = ctx.r[9].s64 + 13512;
	// 82DCF1E8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCF1EC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCF1F0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DCF1F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCF1F8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCF1FC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DCF200: 83F10000  lwz r31, 0(r17)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF204: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF208: 80B10008  lwz r5, 8(r17)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCF20C: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCF210: 4BF88161  bl 0x82d57370
	ctx.lr = 0x82DCF214;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
	// 82DCF214: 39C00004  li r14, 4
	ctx.r[14].s64 = 4;
	// 82DCF218: 83D60010  lwz r30, 0x10(r22)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DCF21C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82DCF220: 557C2036  slwi r28, r11, 4
	// 82DCF224: 7C6F702E  lwzx r3, r15, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82DCF228: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DCF22C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DCF230: 7D4BE214  add r10, r11, r28
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DCF234: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCF238: 41990010  bgt cr6, 0x82dcf248
	if ctx.cr[6].gt {
	pc = 0x82DCF248; continue 'dispatch;
	}
	// 82DCF23C: 7D725B78  mr r18, r11
	ctx.r[18].u64 = ctx.r[11].u64;
	// 82DCF240: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DCF244: 4800001C  b 0x82dcf260
	pc = 0x82DCF260; continue 'dispatch;
	// 82DCF248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF24C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DCF250: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCF254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCF258: 4E800421  bctrl
	ctx.lr = 0x82DCF25C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCF25C: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82DCF260: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF264: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82DCF268: 83B6000C  lwz r29, 0xc(r22)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCF26C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCF270: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DCF274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCF278: 4E800421  bctrl
	ctx.lr = 0x82DCF27C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCF27C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82DCF280: 3BFEFFFF  addi r31, r30, -1
	ctx.r[31].s64 = ctx.r[30].s64 + -1;
	// 82DCF284: 7D239050  subf r9, r3, r18
	ctx.r[9].s64 = ctx.r[18].s64 - ctx.r[3].s64;
}

pub fn sub_82DCF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF550 size=80
	// 82DCF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF55C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DCF560: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCF564: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DCF568: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DCF56C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DCF570: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DCF574: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCF578: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCF57C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCF580: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DCF584: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DCF588: 4BFFEEF9  bl 0x82dce480
	ctx.lr = 0x82DCF58C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE480);
	// 82DCF58C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCF590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF598: 4E800020  blr
	return;
}

pub fn sub_82DCF5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCF5A0 size=80
	// 82DCF5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF5A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF5AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCF5B0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DCF5B4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DCF5B8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DCF5BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DCF5C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DCF5C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCF5C8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCF5CC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DCF5D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DCF5D4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DCF5D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DCF5DC: 4BFFF1A5  bl 0x82dce780
	ctx.lr = 0x82DCF5E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE780);
	// 82DCF5E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCF5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF5EC: 4E800020  blr
	return;
}

pub fn sub_82DCF5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF5F0 size=232
	// 82DCF5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF5F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCF5FC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCF604: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DCF608: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DCF60C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DCF610: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DCF614: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DCF618: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DCF61C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DCF620: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DCF624: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DCF628: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DCF62C: 4200FFF0  bdnz 0x82dcf61c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DCF61C; continue 'dispatch;
	}
	// 82DCF630: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCF634: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DCF638: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DCF63C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DCF640: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DCF644: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DCF6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCF6D8 size=176
	// 82DCF6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF6E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCF6E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCF6E8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DCF6EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF6F0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DCF6F4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DCF6F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DCF6FC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DCF700: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF704: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DCF708: 4BFFFA99  bl 0x82dcf1a0
	ctx.lr = 0x82DCF70C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCF1A0);
	// 82DCF70C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF710: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DCF714: 40980044  bge cr6, 0x82dcf758
	if !ctx.cr[6].lt {
	pc = 0x82DCF758; continue 'dispatch;
	}
	// 82DCF718: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCF71C: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DCF720: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
}

pub fn sub_82DCF788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF788 size=72
	// 82DCF788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF794: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCF798: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DCF79C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DCF7A0: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DCF7A4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DCF7A8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DCF7AC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCF7B0: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82DCF7B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DCF7B8: 4BFFECC9  bl 0x82dce480
	ctx.lr = 0x82DCF7BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE480);
	// 82DCF7BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCF7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF7C8: 4E800020  blr
	return;
}

pub fn sub_82DCF7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCF7D0 size=72
	// 82DCF7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF7D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF7DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCF7E0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82DCF7E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DCF7E8: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DCF7EC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DCF7F0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DCF7F4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCF7F8: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCF7FC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DCF800: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DCF804: 4BFFEF7D  bl 0x82dce780
	ctx.lr = 0x82DCF808;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCE780);
	// 82DCF808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCF80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF814: 4E800020  blr
	return;
}

pub fn sub_82DCF818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF818 size=232
	// 82DCF818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF820: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF824: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DCF828: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DCF82C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DCF830: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DCF834: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DCF838: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DCF83C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DCF840: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DCF844: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DCF848: 4200FFF0  bdnz 0x82dcf838
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DCF838; continue 'dispatch;
	}
	// 82DCF84C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCF850: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DCF854: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DCF858: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82DCF85C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DCF860: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
}

pub fn sub_82DCF900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCF900 size=320
	// 82DCF900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCF904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCF908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DCF90C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCF910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCF914: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCF918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCF91C: 396B3490  addi r11, r11, 0x3490
	ctx.r[11].s64 = ctx.r[11].s64 + 13456;
	// 82DCF920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCF924: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCF928: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCF92C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DCF930: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DCF934: 409A0020  bne cr6, 0x82dcf954
	if !ctx.cr[6].eq {
	pc = 0x82DCF954; continue 'dispatch;
	}
	// 82DCF938: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF93C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DCF940: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DCF944: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCF948: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DCF94C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DCF950: 4BF85979  bl 0x82d552c8
	ctx.lr = 0x82DCF954;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DCF954: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DCF958: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DCF95C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DCF960: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DCF964: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCF968: 419A0020  beq cr6, 0x82dcf988
	if ctx.cr[6].eq {
	pc = 0x82DCF988; continue 'dispatch;
	}
	// 82DCF96C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCF970: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DCF974: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82DCF978: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCF97C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DCF980: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DCF984: 4BF85945  bl 0x82d552c8
	ctx.lr = 0x82DCF988;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
	// 82DCF988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCF98C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DCF990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCF994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCF998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DCF99C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCF9A0: 4E800020  blr
	return;
}

pub fn sub_82DCFA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCFA40 size=80
	// 82DCFA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCFA48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCFA4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCFA54: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DCFA58: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DCFA5C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCFA60: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DCFA64: 48041A3D  bl 0x82e114a0
	ctx.lr = 0x82DCFA68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E114A0);
	// 82DCFA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DCFA6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DCFA70: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DCFA74: 995F0084  stb r10, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u8 ) };
	// 82DCFA78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DCFA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCFA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCFA84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCFA88: 4E800020  blr
	return;
}

pub fn sub_82DCFA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCFA90 size=112
	// 82DCFA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCFA98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCFA9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFAA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCFAA4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DCFAA8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DCFAAC: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DCFAB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCFAB4: 419A0014  beq cr6, 0x82dcfac8
	if ctx.cr[6].eq {
	pc = 0x82DCFAC8; continue 'dispatch;
	}
	// 82DCFAB8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DCFABC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCFAC0: 48046949  bl 0x82e16408
	ctx.lr = 0x82DCFAC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E16408);
	// 82DCFAC4: 48000010  b 0x82dcfad4
	pc = 0x82DCFAD4; continue 'dispatch;
	// 82DCFAC8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCFACC: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DCFAD0: 480419D1  bl 0x82e114a0
	ctx.lr = 0x82DCFAD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E114A0);
	// 82DCFAD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFAD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DCFADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCFAE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFAE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCFAE8: 4E800421  bctrl
	ctx.lr = 0x82DCFAEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCFAEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DCFAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCFAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCFAF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCFAFC: 4E800020  blr
	return;
}

pub fn sub_82DCFB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCFB00 size=112
	// 82DCFB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DCFB08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DCFB0C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFB10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCFB14: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DCFB18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCFB1C: 409A0040  bne cr6, 0x82dcfb5c
	if !ctx.cr[6].eq {
	pc = 0x82DCFB5C; continue 'dispatch;
	}
	// 82DCFB20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCFB24: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFB28: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82DCFB2C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82DCFB30: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82DCFB34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DCFB38: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82DCFB3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFB40: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DCFB44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DCFB48: 4E800421  bctrl
	ctx.lr = 0x82DCFB4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DCFB4C: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82DCFB50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DCFB54: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DCFB58: 480429C9  bl 0x82e12520
	ctx.lr = 0x82DCFB5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E12520);
	// 82DCFB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DCFB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DCFB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DCFB68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DCFB6C: 4E800020  blr
	return;
}

pub fn sub_82DCFB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCFB70 size=144
	// 82DCFB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFB74: 4BED9891  bl 0x82ca9404
	ctx.lr = 0x82DCFB78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DCFB78: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFB7C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DCFB80: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DCFB84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DCFB88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DCFB8C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DCFB90: 48014879  bl 0x82de4408
	ctx.lr = 0x82DCFB94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE4408);
	// 82DCFB94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFB98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DCFB9C: 396B3644  addi r11, r11, 0x3644
	ctx.r[11].s64 = ctx.r[11].s64 + 13892;
	// 82DCFBA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DCFBA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DCFBA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DCFBAC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFBB0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DCFBB4: 995F0084  stb r10, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u8 ) };
	// 82DCFBB8: 993F0085  stb r9, 0x85(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(133 as u32), ctx.r[9].u8 ) };
	// 82DCFBBC: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFBC0: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFBC4: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCFBC8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DCFBCC: 4BF877A5  bl 0x82d57370
	ctx.lr = 0x82DCFBD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
	// 82DCFBD0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCFBD4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DCFBD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DCFBDC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DCFBE0: 48048A59  bl 0x82e18638
	ctx.lr = 0x82DCFBE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E18638);
	// 82DCFBE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DCFBE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCFBEC: C00B0EE0  lfs f0, 0xee0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCFBF0: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DCFBF4: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DCFBF8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DCFBFC: 4BED9858  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DCFC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCFC00 size=616
	// 82DCFC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFC04: 4BED97F9  bl 0x82ca93fc
	ctx.lr = 0x82DCFC08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 82DCFC08: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82DCFC0C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFC10: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFC14: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DCFC18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCFC1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DCFC20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DCFC24: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DCFC28: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DCFC2C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCFC30: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCFC34: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCFC38: 4098002C  bge cr6, 0x82dcfc64
	if !ctx.cr[6].lt {
	pc = 0x82DCFC64; continue 'dispatch;
	}
	// 82DCFC3C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCFC40: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DCFC44: 39293698  addi r9, r9, 0x3698
	ctx.r[9].s64 = ctx.r[9].s64 + 13976;
	// 82DCFC48: 3908368C  addi r8, r8, 0x368c
	ctx.r[8].s64 = ctx.r[8].s64 + 13964;
	// 82DCFC4C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCFC50: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DCFC54: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCFC58: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DCFC5C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCFC60: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCFC64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFC68: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DCFC6C: 396B360C  addi r11, r11, 0x360c
	ctx.r[11].s64 = ctx.r[11].s64 + 13836;
	// 82DCFC70: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCFC74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DCFC78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DCFC7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCFC80: 9B810054  stb r28, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u8 ) };
	// 82DCFC84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DCFC88: 48013EA1  bl 0x82de3b28
	ctx.lr = 0x82DCFC8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE3B28);
	// 82DCFC8C: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DCFC90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCFC94: 419A0074  beq cr6, 0x82dcfd08
	if ctx.cr[6].eq {
	pc = 0x82DCFD08; continue 'dispatch;
	}
	// 82DCFC98: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DCFC9C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCFCA0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCFCA4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCFCA8: 40980020  bge cr6, 0x82dcfcc8
	if !ctx.cr[6].lt {
	pc = 0x82DCFCC8; continue 'dispatch;
	}
	// 82DCFCAC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCFCB0: 39293680  addi r9, r9, 0x3680
	ctx.r[9].s64 = ctx.r[9].s64 + 13952;
	// 82DCFCB4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCFCB8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCFCBC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DCFCC0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCFCC4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCFCC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFCCC: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82DCFCD0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DCFCD4: 396B3268  addi r11, r11, 0x3268
	ctx.r[11].s64 = ctx.r[11].s64 + 12904;
	// 82DCFCD8: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 82DCFCDC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DCFCE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DCFCE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DCFCE8: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCFCEC: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DCFCF0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DCFCF4: 4800D095  bl 0x82ddcd88
	ctx.lr = 0x82DCFCF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCD88);
	// 82DCFCF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFCFC: 396B3250  addi r11, r11, 0x3250
	ctx.r[11].s64 = ctx.r[11].s64 + 12880;
	// 82DCFD00: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DCFD04: 48000104  b 0x82dcfe08
	pc = 0x82DCFE08; continue 'dispatch;
	// 82DCFD08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFD0C: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 82DCFD10: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DCFD14: 396B35C4  addi r11, r11, 0x35c4
	ctx.r[11].s64 = ctx.r[11].s64 + 13764;
	// 82DCFD18: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82DCFD1C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DCFD20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DCFD24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCFD28: C3EA0C64  lfs f31, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DCFD2C: D3E100AC  stfs f31, 0xac(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82DCFD30: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82DCFD34: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82DCFD38: 48013F99  bl 0x82de3cd0
	ctx.lr = 0x82DCFD3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE3CD0);
	// 82DCFD3C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82DCFD40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCFD44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFD48: 3B8B3250  addi r28, r11, 0x3250
	ctx.r[28].s64 = ctx.r[11].s64 + 12880;
	// 82DCFD4C: 419A00B8  beq cr6, 0x82dcfe04
	if ctx.cr[6].eq {
	pc = 0x82DCFE04; continue 'dispatch;
	}
	// 82DCFD50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFD54: C1A100AC  lfs f13, 0xac(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DCFD58: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DCFD5C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DCFD60: 40990048  ble cr6, 0x82dcfda8
	if !ctx.cr[6].gt {
	pc = 0x82DCFDA8; continue 'dispatch;
	}
	// 82DCFD64: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82DCFD68: 93C100E0  stw r30, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[30].u32 ) };
	// 82DCFD6C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFD70: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82DCFD74: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
}

pub fn sub_82DCFE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DCFE68 size=336
	// 82DCFE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFE6C: 4BED958D  bl 0x82ca93f8
	ctx.lr = 0x82DCFE70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 82DCFE70: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFE74: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFE78: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DCFE7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DCFE80: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DCFE84: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DCFE88: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82DCFE8C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DCFE90: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCFE94: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCFE98: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCFE9C: 4098002C  bge cr6, 0x82dcfec8
	if !ctx.cr[6].lt {
	pc = 0x82DCFEC8; continue 'dispatch;
	}
	// 82DCFEA0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCFEA4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DCFEA8: 39293698  addi r9, r9, 0x3698
	ctx.r[9].s64 = ctx.r[9].s64 + 13976;
	// 82DCFEAC: 3908368C  addi r8, r8, 0x368c
	ctx.r[8].s64 = ctx.r[8].s64 + 13964;
	// 82DCFEB0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCFEB4: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DCFEB8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCFEBC: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DCFEC0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCFEC4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCFEC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFECC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DCFED0: 396B360C  addi r11, r11, 0x360c
	ctx.r[11].s64 = ctx.r[11].s64 + 13836;
	// 82DCFED4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DCFED8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DCFEDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DCFEE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DCFEE4: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82DCFEE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DCFEEC: 48013C3D  bl 0x82de3b28
	ctx.lr = 0x82DCFEF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE3B28);
	// 82DCFEF0: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DCFEF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DCFEF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFEFC: 3BEB325C  addi r31, r11, 0x325c
	ctx.r[31].s64 = ctx.r[11].s64 + 12892;
	// 82DCFF00: 419A0060  beq cr6, 0x82dcff60
	if ctx.cr[6].eq {
	pc = 0x82DCFF60; continue 'dispatch;
	}
	// 82DCFF04: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DCFF08: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCFF0C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCFF10: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCFF14: 40980020  bge cr6, 0x82dcff34
	if !ctx.cr[6].lt {
	pc = 0x82DCFF34; continue 'dispatch;
	}
	// 82DCFF18: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCFF1C: 39293680  addi r9, r9, 0x3680
	ctx.r[9].s64 = ctx.r[9].s64 + 13952;
	// 82DCFF20: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCFF24: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCFF28: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DCFF2C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCFF30: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCFF34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DCFF38: 9B61005C  stb r27, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u8 ) };
	// 82DCFF3C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DCFF40: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82DCFF44: 396B3280  addi r11, r11, 0x3280
	ctx.r[11].s64 = ctx.r[11].s64 + 12928;
	// 82DCFF48: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DCFF4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DCFF50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DCFF54: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DCFF58: 4800D3C9  bl 0x82ddd320
	ctx.lr = 0x82DCFF5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD320);
	// 82DCFF5C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82DCFF60: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DCFF64: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DCFF68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCFF6C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCFF70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCFF74: 40980020  bge cr6, 0x82dcff94
	if !ctx.cr[6].lt {
	pc = 0x82DCFF94; continue 'dispatch;
	}
	// 82DCFF78: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DCFF7C: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DCFF80: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DCFF84: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DCFF88: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DCFF8C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DCFF90: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DCFF94: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DCFF98: 4BED94B0  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
}

pub fn sub_82DCFFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DCFFB8 size=336
	// 82DCFFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DCFFBC: 4BED9441  bl 0x82ca93fc
	ctx.lr = 0x82DCFFC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 82DCFFC0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DCFFC4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DCFFC8: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DCFFCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DCFFD0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DCFFD4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DCFFD8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DCFFDC: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DCFFE0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DCFFE4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DCFFE8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DCFFEC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DCFFF0: 4098002C  bge cr6, 0x82dd001c
	if !ctx.cr[6].lt {
	pc = 0x82DD001C; continue 'dispatch;
	}
	// 82DCFFF4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DCFFF8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DCFFFC: 392936AC  addi r9, r9, 0x36ac
	ctx.r[9].s64 = ctx.r[9].s64 + 13996;
	// 82DD0000: 3908368C  addi r8, r8, 0x368c
	ctx.r[8].s64 = ctx.r[8].s64 + 13964;
	// 82DD0004: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD0008: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DD000C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD0010: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DD0014: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD0018: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD001C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD0020: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DD0024: 396B0E8C  addi r11, r11, 0xe8c
	ctx.r[11].s64 = ctx.r[11].s64 + 3724;
	// 82DD0028: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DD002C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DD0030: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DD0034: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DD0038: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DD003C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD0040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DD0044: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DD0048: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82DD004C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DD0050: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82DD0054: 48014AAD  bl 0x82de4b00
	ctx.lr = 0x82DD0058;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE4B00);
	// 82DD0058: 89610058  lbz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DD005C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DD0060: 419A004C  beq cr6, 0x82dd00ac
	if ctx.cr[6].eq {
	pc = 0x82DD00AC; continue 'dispatch;
	}
	// 82DD0064: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DD0068: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD006C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0070: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD0074: 40980020  bge cr6, 0x82dd0094
	if !ctx.cr[6].lt {
	pc = 0x82DD0094; continue 'dispatch;
	}
	// 82DD0078: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DD007C: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DD0080: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD0084: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD0088: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD008C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD0090: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD0094: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DD0098: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DD009C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DD00A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DD00A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DD00A8: 48000D79  bl 0x82dd0e20
	ctx.lr = 0x82DD00AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD0E20);
	// 82DD00AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD00B0: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DD00B4: 396B3250  addi r11, r11, 0x3250
	ctx.r[11].s64 = ctx.r[11].s64 + 12880;
	// 82DD00B8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DD00BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DD00C0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD00C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DD00C8: 40980020  bge cr6, 0x82dd00e8
	if !ctx.cr[6].lt {
	pc = 0x82DD00E8; continue 'dispatch;
	}
	// 82DD00CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DD00D0: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DD00D4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DD00D8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DD00DC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DD00E0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DD00E4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DD00E8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DD00EC: 4BED9360  b 0x82ca944c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA944C);
	return;
	// 82DD00F0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DD00F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DD00F8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DD00FC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DD0100: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82DD0104: 4BFFFEB4  b 0x82dcffb8
	pc = 0x82DCFFB8; continue 'dispatch;
}

pub fn sub_82DD0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0108 size=352
	// 82DD0108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD010C: 4BED92F9  bl 0x82ca9404
	ctx.lr = 0x82DD0110;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 82DD0110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DD0114: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DD0118: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DD011C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DD0120: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DD0124: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DD0128: 419A0100  beq cr6, 0x82dd0228
	if ctx.cr[6].eq {
	pc = 0x82DD0228; continue 'dispatch;
	}
	// 82DD012C: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0130: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD0134: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0138: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD013C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0140: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DD0144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DD0148: 4E800421  bctrl
	ctx.lr = 0x82DD014C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DD014C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82DD0150: 41980088  blt cr6, 0x82dd01d8
	if ctx.cr[6].lt {
	pc = 0x82DD01D8; continue 'dispatch;
	}
	// 82DD0154: 419A0064  beq cr6, 0x82dd01b8
	if ctx.cr[6].eq {
	pc = 0x82DD01B8; continue 'dispatch;
	}
	// 82DD0158: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82DD015C: 41980010  blt cr6, 0x82dd016c
	if ctx.cr[6].lt {
	pc = 0x82DD016C; continue 'dispatch;
	}
	// 82DD0160: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DD0164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0168: 4BED92EC  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
	// 82DD016C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD0170: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0174: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0178: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DD017C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD0180: 4BF850C9  bl 0x82d55248
	ctx.lr = 0x82DD0184;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD0184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD0188: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82DD018C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DD0190: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0198: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD019C: 4801426D  bl 0x82de4408
	ctx.lr = 0x82DD01A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE4408);
	// 82DD01A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD01A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD01A8: 396B35D0  addi r11, r11, 0x35d0
	ctx.r[11].s64 = ctx.r[11].s64 + 13776;
	// 82DD01AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD01B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD01B4: 4BED92A0  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
	// 82DD01B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD01BC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD01C0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD01C4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD01C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD01CC: 4BF8507D  bl 0x82d55248
	ctx.lr = 0x82DD01D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD01D0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD01D4: 48000070  b 0x82dd0244
	pc = 0x82DD0244; continue 'dispatch;
	// 82DD01D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD01DC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD01E0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD01E4: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82DD01E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD01EC: 4BF8505D  bl 0x82d55248
	ctx.lr = 0x82DD01F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD01F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DD01F4: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82DD01F8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD01FC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD0200: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DD0204: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DD0208: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD020C: 4BFFF965  bl 0x82dcfb70
	ctx.lr = 0x82DD0210;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFB70);
	// 82DD0210: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DD0214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD0218: 396B36C4  addi r11, r11, 0x36c4
	ctx.r[11].s64 = ctx.r[11].s64 + 14020;
	// 82DD021C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DD0220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0224: 4BED9230  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
	// 82DD0228: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD022C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DD0230: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DD0234: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DD0238: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DD023C: 4BF8500D  bl 0x82d55248
	ctx.lr = 0x82DD0240;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD0240: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DD0244: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82DD0248: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DD024C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DD0250: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DD0254: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DD0258: 4800D4E9  bl 0x82ddd740
	ctx.lr = 0x82DD025C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD740);
	// 82DD025C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0260: 4BED91F4  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DD0268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0268 size=20
	// 82DD0268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD026C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DD0270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DD0274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DD0278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD02E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DD02E8 size=1448
	// 82DD02E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD02EC: 4BED90E5  bl 0x82ca93d0
	ctx.lr = 0x82DD02F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
	// 82DD02F0: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DD02F4: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DD02F8: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
}

pub fn sub_82DD0890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0890 size=344
	// 82DD0890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0894: 4BED8B71  bl 0x82ca9404
	ctx.lr = 0x82DD0898;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE4408);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD0954: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0958: 4800005C  b 0x82dd09b4
	pc = 0x82DD09B4; continue 'dispatch;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFB70);
	// 82DD0990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0994: 4BED8AC0  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD09B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD740);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DD09E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD09E8 size=544
	// 82DD09E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD09EC: 4BED8A19  bl 0x82ca9404
	ctx.lr = 0x82DD09F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
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
	// 82DD0A74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DD0A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0A7C: 4BED89D8  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DE4408);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD740);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFB70);
	// 82DD0B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0B54: 4BED8900  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD0B70: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DD0B74: 48000070  b 0x82dd0be4
	pc = 0x82DD0BE4; continue 'dispatch;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFB70);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
	// 82DD0BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD740);
	// 82DD0BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD0C00: 4BED8854  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DD0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0C08 size=12
	// 82DD0C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD0C0C: 4BED8801  bl 0x82ca940c
	ctx.lr = 0x82DD0C10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DD0C10: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD0D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0D80 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD320);
	// 82DD0DBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD0DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD0DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD0DC8: 4E800020  blr
	return;
}

pub fn sub_82DD0DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD0DD0 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCD88);
	// 82DD0E10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD0E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD0E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD0E1C: 4E800020  blr
	return;
}

pub fn sub_82DD0E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0E20 size=232
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

pub fn sub_82DD0F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0F08 size=144
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55128);
	// 82DD0F60: 4800001C  b 0x82dd0f7c
	pc = 0x82DD0F7C; continue 'dispatch;
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

pub fn sub_82DD0F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD0F98 size=20
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
}

pub fn sub_82DD10B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD10B8 size=376
	// 82DD10B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD10BC: 4BED8339  bl 0x82ca93f4
	ctx.lr = 0x82DD10C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
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
	// 82DD1198: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
	// 82DD1220: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DD1224: 9AFB0010  stb r23, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 82DD1228: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DD122C: 4BED8218  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
}

pub fn sub_82DD1230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1230 size=416
	// 82DD1230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1234: 4BED81C1  bl 0x82ca93f4
	ctx.lr = 0x82DD1238;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
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
	// 82DD1318: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
	// 82DD13B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DD13BC: 9AFB0010  stb r23, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 82DD13C0: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 82DD13C4: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82DD13C8: 4BED807C  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
}

pub fn sub_82DD13D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD13D0 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFE68);
	// 82DD140C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1418: 4E800020  blr
	return;
}

pub fn sub_82DD1420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1420 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFC00);
	// 82DD1460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD146C: 4E800020  blr
	return;
}

pub fn sub_82DD1470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1470 size=232
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

pub fn sub_82DD1558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1558 size=112
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E12520);
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

pub fn sub_82DD15C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD15C8 size=72
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFE68);
	// 82DD15FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1608: 4E800020  blr
	return;
}

pub fn sub_82DD1610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1610 size=72
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DCFC00);
	// 82DD1648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1654: 4E800020  blr
	return;
}

pub fn sub_82DD1658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1658 size=232
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

pub fn sub_82DD1740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1740 size=208
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

pub fn sub_82DD1810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1810 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD02E8);
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
}

pub fn sub_82DD18C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD18C0 size=120
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E114A0);
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

pub fn sub_82DD1938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1938 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E12520);
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

pub fn sub_82DD19A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD19A0 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCD88);
	// 82DD19E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD19E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD19E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD19EC: 4E800020  blr
	return;
}

pub fn sub_82DD19F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD19F0 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCD88);
	// 82DD1A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1A34: 4E800020  blr
	return;
	// 82DD1A38: 4BFFF3E8  b 0x82dd0e20
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD0E20);
	return;
}

pub fn sub_82DD1A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1A40 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD320);
	// 82DD1A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1A88: 4E800020  blr
	return;
}

pub fn sub_82DD1A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD1A90 size=488
	// 82DD1A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1A94: 4BED7969  bl 0x82ca93fc
	ctx.lr = 0x82DD1A98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
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

pub fn sub_82DD1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1C78 size=72
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD320);
	// 82DD1CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD1CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD1CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD1CB8: 4E800020  blr
	return;
}

pub fn sub_82DD1CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1CC0 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E116B0);
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

pub fn sub_82DD1D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1D28 size=12
	// 82DD1D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1D2C: 4BED76E1  bl 0x82ca940c
	ctx.lr = 0x82DD1D30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DD1D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD1DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1DF0 size=208
	// 82DD1DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD1DF4: 4BED7619  bl 0x82ca940c
	ctx.lr = 0x82DD1DF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E116B0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DD1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1EC0 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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

pub fn sub_82DD1F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD1F90 size=152
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
	// 82DD1FD0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DD1FD4: 4BF832F5  bl 0x82d552c8
	ctx.lr = 0x82DD1FD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
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

pub fn sub_82DD2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2028 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E12520);
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

pub fn sub_82DD2090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2090 size=96
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDCD88);
	// 82DD20DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD20E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD20E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD20E8: 4E800020  blr
	return;
}

pub fn sub_82DD20F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD20F0 size=88
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DDD320);
	// 82DD2138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD213C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2144: 4E800020  blr
	return;
}

pub fn sub_82DD2148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2148 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD1A90);
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
}

pub fn sub_82DD21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD21F8 size=16
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
}

pub fn sub_82DD2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2358 size=12
	// 82DD2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD235C: 4BED70AD  bl 0x82ca9408
	ctx.lr = 0x82DD2360;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82DD2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD23D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD23D8 size=16
	// 82DD23D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD23DC: 4BED7001  bl 0x82ca93dc
	ctx.lr = 0x82DD23E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
	// 82DD23E0: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82DD23E4: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD2658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2658 size=16
	// 82DD2658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD265C: 4BED6D81  bl 0x82ca93dc
	ctx.lr = 0x82DD2660;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
	// 82DD2660: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82DD2664: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD28D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD28D8 size=464
	// 82DD28D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD28DC: 4BED6B19  bl 0x82ca93f4
	ctx.lr = 0x82DD28E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
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
}

pub fn sub_82DD2AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2AA8 size=12
	// 82DD2AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2AAC: 4BED6961  bl 0x82ca940c
	ctx.lr = 0x82DD2AB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DD2AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD2B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2B68 size=88
	// 82DD2B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2B6C: 4BED689D  bl 0x82ca9408
	ctx.lr = 0x82DD2B70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD2AA8);
	// 82DD2BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD2BBC: 4BED689C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DD2BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2BC0 size=112
	// 82DD2BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2BC4: 4BED6841  bl 0x82ca9404
	ctx.lr = 0x82DD2BC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD2AA8);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_82DD2C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2C30 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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

pub fn sub_82DD2D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD2D00 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD28D8);
	// 82DD2D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD2D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2D48: 4E800020  blr
	return;
}

pub fn sub_82DD2D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2D50 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD2658);
	// 82DD2D90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD2D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD2D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD2D9C: 4E800020  blr
	return;
}

pub fn sub_82DD2DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD2DA0 size=728
	// 82DD2DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD2DA4: 4BED662D  bl 0x82ca93d0
	ctx.lr = 0x82DD2DA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D57370);
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
}

pub fn sub_82DD3078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD3078 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD23D8);
	// 82DD30B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD30BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD30C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD30C4: 4E800020  blr
	return;
}

pub fn sub_82DD30C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD30C8 size=72
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD28D8);
	// 82DD30FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD3100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD3104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD3108: 4E800020  blr
	return;
}

pub fn sub_82DD3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD3110 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD2DA0);
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
}

pub fn sub_82DD31C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD31C0 size=152
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D552C8);
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

pub fn sub_82DD3258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3258 size=240
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

pub fn sub_82DD3348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3348 size=136
	// 82DD3348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD334C: 4BED60C1  bl 0x82ca940c
	ctx.lr = 0x82DD3350;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C520);
	// 82DD33C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD33C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD33CC: 4BED6090  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DD33D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD33D0 size=120
	// 82DD33D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD33D4: 4BED6035  bl 0x82ca9408
	ctx.lr = 0x82DD33D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
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
	// 82DD3440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD3444: 4BED6014  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DD3448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3448 size=152
	// 82DD3448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD344C: 4BED5FC1  bl 0x82ca940c
	ctx.lr = 0x82DD3450;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C520);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DD34E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD34E0 size=584
	// 82DD34E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD34E4: 4BED5EFD  bl 0x82ca93e0
	ctx.lr = 0x82DD34E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
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

pub fn sub_82DD3728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3728 size=592
	// 82DD3728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD372C: 4BED5CB9  bl 0x82ca93e4
	ctx.lr = 0x82DD3730;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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

pub fn sub_82DD3978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3978 size=744
	// 82DD3978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD397C: 4BED5A59  bl 0x82ca93d4
	ctx.lr = 0x82DD3980;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D4);
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

pub fn sub_82DD3C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3C60 size=760
	// 82DD3C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD3C64: 4BED5775  bl 0x82ca93d8
	ctx.lr = 0x82DD3C68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D8);
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

pub fn sub_82DD3F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD3F58 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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

pub fn sub_82DD4028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4028 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD34E0);
	// 82DD4064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD4068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD406C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD4070: 4E800020  blr
	return;
}

pub fn sub_82DD4078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4078 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD3728);
	// 82DD40B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD40B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD40BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD40C0: 4E800020  blr
	return;
}

pub fn sub_82DD40C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD40C8 size=904
	// 82DD40C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD40CC: 4BED5309  bl 0x82ca93d4
	ctx.lr = 0x82DD40D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D4);
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

pub fn sub_82DD4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD4450 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD3978);
	// 82DD4490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD4494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD4498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD449C: 4E800020  blr
	return;
}

pub fn sub_82DD44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD44A0 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD3C60);
	// 82DD44E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD44E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD44E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD44EC: 4E800020  blr
	return;
}

pub fn sub_82DD44F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD44F0 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD40C8);
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
}

pub fn sub_82DD45A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD45A0 size=12
	// 82DD45A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD45A4: 4BED4E69  bl 0x82ca940c
	ctx.lr = 0x82DD45A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DD45A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD4620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4620 size=120
	// 82DD4620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4624: 4BED4DE5  bl 0x82ca9408
	ctx.lr = 0x82DD4628;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
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
	// 82DD4690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD4694: 4BED4DC4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DD4698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4698 size=144
	// 82DD4698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD469C: 4BED4D71  bl 0x82ca940c
	ctx.lr = 0x82DD46A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C5F8);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DD4728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4728 size=456
	// 82DD4728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD472C: 4BED4CB9  bl 0x82ca93e4
	ctx.lr = 0x82DD4730;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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

pub fn sub_82DD48F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD48F0 size=592
	// 82DD48F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD48F4: 4BED4AF5  bl 0x82ca93e8
	ctx.lr = 0x82DD48F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E8);
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

pub fn sub_82DD4B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4B40 size=688
	// 82DD4B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4B44: 4BED489D  bl 0x82ca93e0
	ctx.lr = 0x82DD4B48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
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

pub fn sub_82DD4DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD4DF0 size=712
	// 82DD4DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD4DF4: 4BED45F1  bl 0x82ca93e4
	ctx.lr = 0x82DD4DF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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
	// 82DD4E44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD4E48: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82DD4E4C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD4E50: 480477A9  bl 0x82e1c5f8
	ctx.lr = 0x82DD4E54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C5F8);
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

pub fn sub_82DD50B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD50B8 size=632
	// 82DD50B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD50BC: 4BED4321  bl 0x82ca93dc
	ctx.lr = 0x82DD50C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
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

pub fn sub_82DD5330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5330 size=792
	// 82DD5330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5334: 4BED40AD  bl 0x82ca93e0
	ctx.lr = 0x82DD5338;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
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
	// 82DD5384: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD5388: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82DD538C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD5390: 48047269  bl 0x82e1c5f8
	ctx.lr = 0x82DD5394;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C5F8);
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

pub fn sub_82DD5648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5648 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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

pub fn sub_82DD5718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5718 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1280);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1280);
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

pub fn sub_82DD57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD57E8 size=232
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

pub fn sub_82DD58D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD58D0 size=792
	// 82DD58D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD58D4: 4BED3B09  bl 0x82ca93dc
	ctx.lr = 0x82DD58D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
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

pub fn sub_82DD5BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5BE8 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD50B8);
	// 82DD5C24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5C30: 4E800020  blr
	return;
}

pub fn sub_82DD5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5C38 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD5330);
	// 82DD5C74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5C80: 4E800020  blr
	return;
}

pub fn sub_82DD5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD5C88 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD4B40);
	// 82DD5CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5CD4: 4E800020  blr
	return;
}

pub fn sub_82DD5CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD5CD8 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD4DF0);
	// 82DD5D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD5D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD5D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD5D24: 4E800020  blr
	return;
}

pub fn sub_82DD5D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD5D28 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD58D0);
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
}

pub fn sub_82DD5DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5DD8 size=120
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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

pub fn sub_82DD5E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5E50 size=120
	// 82DD5E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5E54: 4BED35B5  bl 0x82ca9408
	ctx.lr = 0x82DD5E58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
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
	// 82DD5EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DD5EC4: 4BED3594  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82DD5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD5EC8 size=12
	// 82DD5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD5ECC: 4BED3519  bl 0x82ca93e4
	ctx.lr = 0x82DD5ED0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
	// 82DD5ED0: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD6180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD6180 size=464
	// 82DD6180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6184: 4BED326D  bl 0x82ca93f0
	ctx.lr = 0x82DD6188;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F0);
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

pub fn sub_82DD6350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6350 size=12
	// 82DD6350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6354: 4BED3099  bl 0x82ca93ec
	ctx.lr = 0x82DD6358;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93EC);
	// 82DD6358: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD6510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6510 size=600
	// 82DD6510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6514: 4BED2ED1  bl 0x82ca93e4
	ctx.lr = 0x82DD6518;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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

pub fn sub_82DD6768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6768 size=600
	// 82DD6768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD676C: 4BED2C79  bl 0x82ca93e4
	ctx.lr = 0x82DD6770;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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

pub fn sub_82DD69C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD69C0 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
	// 82DD6A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD6A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD6A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD6A24: 4E800020  blr
	return;
}

pub fn sub_82DD6A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD6A28 size=800
	// 82DD6A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6A2C: 4BED29B1  bl 0x82ca93dc
	ctx.lr = 0x82DD6A30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
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

pub fn sub_82DD6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6D48 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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

pub fn sub_82DD6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6DB0 size=104
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

pub fn sub_82DD6E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6E18 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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

pub fn sub_82DD6E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD6E80 size=384
	// 82DD6E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD6E84: 4BED2579  bl 0x82ca93fc
	ctx.lr = 0x82DD6E88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D563F0);
	// 82DD6EF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DD6EF8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
}

pub fn sub_82DD7000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7000 size=888
	// 82DD7000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7004: 4BED23F1  bl 0x82ca93f4
	ctx.lr = 0x82DD7008;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D563F0);
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

pub fn sub_82DD7378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7378 size=784
	// 82DD7378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD737C: 4BED2081  bl 0x82ca93fc
	ctx.lr = 0x82DD7380;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D563F0);
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

pub fn sub_82DD7688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7688 size=784
	// 82DD7688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD768C: 4BED1D71  bl 0x82ca93fc
	ctx.lr = 0x82DD7690;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D563F0);
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

pub fn sub_82DD7998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7998 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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

pub fn sub_82DD7A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7A68 size=72
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD6E80);
	// 82DD7A9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7AA8: 4E800020  blr
	return;
}

pub fn sub_82DD7AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7AB0 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD6E80);
	// 82DD7AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7AF8: 4E800020  blr
	return;
}

pub fn sub_82DD7B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD7B00 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD7378);
	// 82DD7B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7B4C: 4E800020  blr
	return;
}

pub fn sub_82DD7B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD7B50 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD7688);
	// 82DD7B90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD7B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD7B9C: 4E800020  blr
	return;
}

pub fn sub_82DD7BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD7BA0 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD7000);
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
}

pub fn sub_82DD7C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7C50 size=120
	// 82DD7C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7C54: 4BED17B9  bl 0x82ca940c
	ctx.lr = 0x82DD7C58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C520);
	// 82DD7CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DD7CBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD7CC0: 4BED179C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82DD7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7CC8 size=104
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

pub fn sub_82DD7D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7D30 size=12
	// 82DD7D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7D34: 4BED16D9  bl 0x82ca940c
	ctx.lr = 0x82DD7D38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82DD7D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7DB0 size=304
	// 82DD7DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7DB4: 4BED1649  bl 0x82ca93fc
	ctx.lr = 0x82DD7DB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
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

pub fn sub_82DD7EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD7EE0 size=12
	// 82DD7EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD7EE4: 4BED1515  bl 0x82ca93f8
	ctx.lr = 0x82DD7EE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 82DD7EE8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82DD8020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8020 size=376
	// 82DD8020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8024: 4BED13C5  bl 0x82ca93e8
	ctx.lr = 0x82DD8028;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E8);
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

pub fn sub_82DD8198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8198 size=496
	// 82DD8198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD819C: 4BED1245  bl 0x82ca93e0
	ctx.lr = 0x82DD81A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E0);
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

pub fn sub_82DD8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8388 size=656
	// 82DD8388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD838C: 4BED1059  bl 0x82ca93e4
	ctx.lr = 0x82DD8390;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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
	// 82DD83DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DD83E0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DD83E4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DD83E8: 48044139  bl 0x82e1c520
	ctx.lr = 0x82DD83EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E1C520);
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

pub fn sub_82DD8618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8618 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1168);
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

pub fn sub_82DD86E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD86E8 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1280);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DC1280);
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

pub fn sub_82DD87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD87B8 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD7DB0);
	// 82DD87F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD87F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD87FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8800: 4E800020  blr
	return;
}

pub fn sub_82DD8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8808 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD7EE0);
	// 82DD8844: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD884C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8850: 4E800020  blr
	return;
}

pub fn sub_82DD8858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8858 size=624
	// 82DD8858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD885C: 4BED0B79  bl 0x82ca93d4
	ctx.lr = 0x82DD8860;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D4);
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

pub fn sub_82DD8AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD8AC8 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD8198);
	// 82DD8B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8B14: 4E800020  blr
	return;
}

pub fn sub_82DD8B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD8B18 size=80
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD8388);
	// 82DD8B58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DD8B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DD8B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DD8B64: 4E800020  blr
	return;
}

pub fn sub_82DD8B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DD8B68 size=176
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82DD8858);
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
}

pub fn sub_82DD8C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8C18 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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

pub fn sub_82DD8C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8C80 size=104
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

pub fn sub_82DD8CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8CE8 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55248);
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

pub fn sub_82DD8D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8D50 size=480
	// 82DD8D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8D54: 4BED0691  bl 0x82ca93e4
	ctx.lr = 0x82DD8D58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
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

pub fn sub_82DD8F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DD8F30 size=632
	// 82DD8F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DD8F34: 4BED04B9  bl 0x82ca93ec
	ctx.lr = 0x82DD8F38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93EC);
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

