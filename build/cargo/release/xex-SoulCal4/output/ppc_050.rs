pub fn sub_82422138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422138 size=56
    let mut pc: u32 = 0x82422138;
    'dispatch: loop {
        match pc {
            0x82422138 => {
    //   block [0x82422138..0x82422170)
	// 82422138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242213C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242214C: 4BFFEEE5  bl 0x82421030
	ctx.lr = 0x82422150;
	sub_82421030(ctx, base);
	// 82422150: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82422154: 4BFFEF1D  bl 0x82421070
	ctx.lr = 0x82422158;
	sub_82421070(ctx, base);
	// 82422158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242215C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242216C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422170 size=64
    let mut pc: u32 = 0x82422170;
    'dispatch: loop {
        match pc {
            0x82422170 => {
    //   block [0x82422170..0x824221B0)
	// 82422170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242217C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242218C: 4BFFEEA5  bl 0x82421030
	ctx.lr = 0x82422190;
	sub_82421030(ctx, base);
	// 82422190: 9BDF0048  stb r30, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 82422194: 4BFFEEDD  bl 0x82421070
	ctx.lr = 0x82422198;
	sub_82421070(ctx, base);
	// 82422198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824221A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824221A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824221A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824221AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824221B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824221B0 size=80
    let mut pc: u32 = 0x824221B0;
    'dispatch: loop {
        match pc {
            0x824221B0 => {
    //   block [0x824221B0..0x82422200)
	// 824221B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824221B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824221B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824221BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824221C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824221C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824221C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824221CC: 4BFFEE65  bl 0x82421030
	ctx.lr = 0x824221D0;
	sub_82421030(ctx, base);
	// 824221D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824221D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824221D8: 4BFFF589  bl 0x82421760
	ctx.lr = 0x824221DC;
	sub_82421760(ctx, base);
	// 824221DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824221E0: 4BFFEE91  bl 0x82421070
	ctx.lr = 0x824221E4;
	sub_82421070(ctx, base);
	// 824221E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824221E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824221EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824221F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824221F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824221F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824221FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422200 size=104
    let mut pc: u32 = 0x82422200;
    'dispatch: loop {
        match pc {
            0x82422200 => {
    //   block [0x82422200..0x82422268)
	// 82422200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422204: 48112EB1  bl 0x825350b4
	ctx.lr = 0x82422208;
	sub_82535080(ctx, base);
	// 82422208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242220C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422210: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82422214: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82422218: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8242221C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82422220: 4BFFEE11  bl 0x82421030
	ctx.lr = 0x82422224;
	sub_82421030(ctx, base);
	// 82422224: 4800CD2D  bl 0x8242ef50
	ctx.lr = 0x82422228;
	sub_8242EF50(ctx, base);
	// 82422228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8242222C: 397E07FF  addi r11, r30, 0x7ff
	ctx.r[11].s64 = ctx.r[30].s64 + 2047;
	// 82422230: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82422234: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 82422238: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8242223C: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82422240: 995F0049  stb r10, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[10].u8 ) };
	// 82422244: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 82422248: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 8242224C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82422250: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 82422254: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82422258: 4800CCF9  bl 0x8242ef50
	ctx.lr = 0x8242225C;
	sub_8242EF50(ctx, base);
	// 8242225C: 4BFFEE15  bl 0x82421070
	ctx.lr = 0x82422260;
	sub_82421070(ctx, base);
	// 82422260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82422264: 48112EA0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422268 size=128
    let mut pc: u32 = 0x82422268;
    'dispatch: loop {
        match pc {
            0x82422268 => {
    //   block [0x82422268..0x824222E8)
	// 82422268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242227C: 4BFFEDB5  bl 0x82421030
	ctx.lr = 0x82422280;
	sub_82421030(ctx, base);
	// 82422280: 4800CCD1  bl 0x8242ef50
	ctx.lr = 0x82422284;
	sub_8242EF50(ctx, base);
	// 82422284: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82422288: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242228C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82422290: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82422294: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82422298: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8242229C: 419A0008  beq cr6, 0x824222a4
	if ctx.cr[6].eq {
	pc = 0x824222A4; continue 'dispatch;
	}
	// 824222A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824222A4: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 824222A8: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 824222AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824222B0: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 824222B4: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 824222B8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824222BC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824222C0: 995F004B  stb r10, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 824222C4: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 824222C8: 4800CC89  bl 0x8242ef50
	ctx.lr = 0x824222CC;
	sub_8242EF50(ctx, base);
	// 824222CC: 4BFFEDA5  bl 0x82421070
	ctx.lr = 0x824222D0;
	sub_82421070(ctx, base);
	// 824222D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824222D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824222D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824222DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824222E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824222E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824222E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824222E8 size=132
    let mut pc: u32 = 0x824222E8;
    'dispatch: loop {
        match pc {
            0x824222E8 => {
    //   block [0x824222E8..0x8242236C)
	// 824222E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824222EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824222F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824222F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824222F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824222FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422300: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82422304: 4BFFED2D  bl 0x82421030
	ctx.lr = 0x82422308;
	sub_82421030(ctx, base);
	// 82422308: 4800CC49  bl 0x8242ef50
	ctx.lr = 0x8242230C;
	sub_8242EF50(ctx, base);
	// 8242230C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82422310: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82422314: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82422318: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8242231C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82422320: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82422324: 419A0008  beq cr6, 0x8242232c
	if ctx.cr[6].eq {
	pc = 0x8242232C; continue 'dispatch;
	}
	// 82422328: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8242232C: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82422330: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82422334: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82422338: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8242233C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82422340: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82422344: 995F004B  stb r10, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 82422348: 4800CC09  bl 0x8242ef50
	ctx.lr = 0x8242234C;
	sub_8242EF50(ctx, base);
	// 8242234C: 4BFFED25  bl 0x82421070
	ctx.lr = 0x82422350;
	sub_82421070(ctx, base);
	// 82422350: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82422354: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82422358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242235C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422360: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82422364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422370 size=124
    let mut pc: u32 = 0x82422370;
    'dispatch: loop {
        match pc {
            0x82422370 => {
    //   block [0x82422370..0x824223EC)
	// 82422370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242237C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422384: 4BFFECAD  bl 0x82421030
	ctx.lr = 0x82422388;
	sub_82421030(ctx, base);
	// 82422388: 4800CBC9  bl 0x8242ef50
	ctx.lr = 0x8242238C;
	sub_8242EF50(ctx, base);
	// 8242238C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82422390: 4BFFD8C9  bl 0x8241fc58
	ctx.lr = 0x82422394;
	sub_8241FC58(ctx, base);
	// 82422394: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82422398: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8242239C: 409A002C  bne cr6, 0x824223c8
	if !ctx.cr[6].eq {
	pc = 0x824223C8; continue 'dispatch;
	}
	// 824223A0: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 824223A4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824223A8: 409A0020  bne cr6, 0x824223c8
	if !ctx.cr[6].eq {
	pc = 0x824223C8; continue 'dispatch;
	}
	// 824223AC: 895F004B  lbz r10, 0x4b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 824223B0: 997F004C  stb r11, 0x4c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u8 ) };
	// 824223B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824223B8: 409A0018  bne cr6, 0x824223d0
	if !ctx.cr[6].eq {
	pc = 0x824223D0; continue 'dispatch;
	}
	// 824223BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824223C0: 997F004B  stb r11, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 824223C4: 4800000C  b 0x824223d0
	pc = 0x824223D0; continue 'dispatch;
	// 824223C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824223CC: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824223D0: 4800CB81  bl 0x8242ef50
	ctx.lr = 0x824223D4;
	sub_8242EF50(ctx, base);
	// 824223D4: 4BFFEC9D  bl 0x82421070
	ctx.lr = 0x824223D8;
	sub_82421070(ctx, base);
	// 824223D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824223DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824223E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824223E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824223E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824223F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824223F0 size=108
    let mut pc: u32 = 0x824223F0;
    'dispatch: loop {
        match pc {
            0x824223F0 => {
    //   block [0x824223F0..0x8242245C)
	// 824223F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824223F4: 48112CC9  bl 0x825350bc
	ctx.lr = 0x824223F8;
	sub_82535080(ctx, base);
	// 824223F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824223FC: 4BFFEC35  bl 0x82421030
	ctx.lr = 0x82422400;
	sub_82421030(ctx, base);
	// 82422400: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82422404: 3BAB8E0C  addi r29, r11, -0x71f4
	ctx.r[29].s64 = ctx.r[11].s64 + -29172;
	// 82422408: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242240C: 4800394D  bl 0x82425d58
	ctx.lr = 0x82422410;
	sub_82425D58(ctx, base);
	// 82422410: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82422414: 4182003C  beq 0x82422450
	if ctx.cr[0].eq {
	pc = 0x82422450; continue 'dispatch;
	}
	// 82422418: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8242241C: 3BCBA280  addi r30, r11, -0x5d80
	ctx.r[30].s64 = ctx.r[11].s64 + -23936;
	// 82422420: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82422424: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422428: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242242C: 409A000C  bne cr6, 0x82422438
	if !ctx.cr[6].eq {
	pc = 0x82422438; continue 'dispatch;
	}
	// 82422430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422434: 4BFFF98D  bl 0x82421dc0
	ctx.lr = 0x82422438;
	sub_82421DC0(ctx, base);
	// 82422438: 3BFF0068  addi r31, r31, 0x68
	ctx.r[31].s64 = ctx.r[31].s64 + 104;
	// 8242243C: 397E2080  addi r11, r30, 0x2080
	ctx.r[11].s64 = ctx.r[30].s64 + 8320;
	// 82422440: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82422444: 4198FFE0  blt cr6, 0x82422424
	if ctx.cr[6].lt {
	pc = 0x82422424; continue 'dispatch;
	}
	// 82422448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242244C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82422450: 4BFFEC21  bl 0x82421070
	ctx.lr = 0x82422454;
	sub_82421070(ctx, base);
	// 82422454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82422458: 48112CB4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422460 size=120
    let mut pc: u32 = 0x82422460;
    'dispatch: loop {
        match pc {
            0x82422460 => {
    //   block [0x82422460..0x824224D8)
	// 82422460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422464: 48112C59  bl 0x825350bc
	ctx.lr = 0x82422468;
	sub_82535080(ctx, base);
	// 82422468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242246C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82422474: 4BFFEBBD  bl 0x82421030
	ctx.lr = 0x82422478;
	sub_82421030(ctx, base);
	// 82422478: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8242247C: 4800CAD5  bl 0x8242ef50
	ctx.lr = 0x82422480;
	sub_8242EF50(ctx, base);
	// 82422480: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422484: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82422488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242248C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82422490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422494: 4E800421  bctrl
	ctx.lr = 0x82422498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422498: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242249C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824224A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824224A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824224A8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824224AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824224B0: 4E800421  bctrl
	ctx.lr = 0x824224B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824224B4: 7D7D1A14  add r11, r29, r3
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[3].u64;
	// 824224B8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824224BC: 4800CA95  bl 0x8242ef50
	ctx.lr = 0x824224C0;
	sub_8242EF50(ctx, base);
	// 824224C0: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 824224C4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824224C8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824224CC: 4BFFEBA5  bl 0x82421070
	ctx.lr = 0x824224D0;
	sub_82421070(ctx, base);
	// 824224D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824224D4: 48112C38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824224D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824224D8 size=104
    let mut pc: u32 = 0x824224D8;
    'dispatch: loop {
        match pc {
            0x824224D8 => {
    //   block [0x824224D8..0x82422540)
	// 824224D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824224DC: 48112BD9  bl 0x825350b4
	ctx.lr = 0x824224E0;
	sub_82535080(ctx, base);
	// 824224E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824224E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824224E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824224EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824224F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824224F4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 824224F8: 4BFFEB39  bl 0x82421030
	ctx.lr = 0x824224FC;
	sub_82421030(ctx, base);
	// 824224FC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82422500: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82422504: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82422508: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242250C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422510: 4BFFFCF1  bl 0x82422200
	ctx.lr = 0x82422514;
	sub_82422200(ctx, base);
	// 82422514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422518: 4BFFF8A9  bl 0x82421dc0
	ctx.lr = 0x8242251C;
	sub_82421DC0(ctx, base);
	// 8242251C: 48000008  b 0x82422524
	pc = 0x82422524; continue 'dispatch;
	// 82422520: 4BFF87B9  bl 0x8241acd8
	ctx.lr = 0x82422524;
	sub_8241ACD8(ctx, base);
	// 82422524: 480001AD  bl 0x824226d0
	ctx.lr = 0x82422528;
	sub_824226D0(ctx, base);
	// 82422528: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 8242252C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422530: 409AFFF0  bne cr6, 0x82422520
	if !ctx.cr[6].eq {
	pc = 0x82422520; continue 'dispatch;
	}
	// 82422534: 4BFFEB3D  bl 0x82421070
	ctx.lr = 0x82422538;
	sub_82421070(ctx, base);
	// 82422538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242253C: 48112BC8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422540 size=92
    let mut pc: u32 = 0x82422540;
    'dispatch: loop {
        match pc {
            0x82422540 => {
    //   block [0x82422540..0x8242259C)
	// 82422540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242254C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422554: 4BFFEADD  bl 0x82421030
	ctx.lr = 0x82422558;
	sub_82421030(ctx, base);
	// 82422558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242255C: 4BFFFE15  bl 0x82422370
	ctx.lr = 0x82422560;
	sub_82422370(ctx, base);
	// 82422560: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82422564: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82422568: 409A0010  bne cr6, 0x82422578
	if !ctx.cr[6].eq {
	pc = 0x82422578; continue 'dispatch;
	}
	// 8242256C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82422570: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422574: 419A0010  beq cr6, 0x82422584
	if ctx.cr[6].eq {
	pc = 0x82422584; continue 'dispatch;
	}
	// 82422578: 48000159  bl 0x824226d0
	ctx.lr = 0x8242257C;
	sub_824226D0(ctx, base);
	// 8242257C: 4BFF875D  bl 0x8241acd8
	ctx.lr = 0x82422580;
	sub_8241ACD8(ctx, base);
	// 82422580: 4BFFFFE0  b 0x82422560
	pc = 0x82422560; continue 'dispatch;
	// 82422584: 4BFFEAED  bl 0x82421070
	ctx.lr = 0x82422588;
	sub_82421070(ctx, base);
	// 82422588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242258C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824225A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824225A0 size=96
    let mut pc: u32 = 0x824225A0;
    'dispatch: loop {
        match pc {
            0x824225A0 => {
    //   block [0x824225A0..0x82422600)
	// 824225A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824225A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824225A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824225AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824225B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824225B4: 4BFFEA7D  bl 0x82421030
	ctx.lr = 0x824225B8;
	sub_82421030(ctx, base);
	// 824225B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824225BC: 4BFFFDB5  bl 0x82422370
	ctx.lr = 0x824225C0;
	sub_82422370(ctx, base);
	// 824225C0: 4800C991  bl 0x8242ef50
	ctx.lr = 0x824225C4;
	sub_8242EF50(ctx, base);
	// 824225C4: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 824225C8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824225CC: 409A0008  bne cr6, 0x824225d4
	if !ctx.cr[6].eq {
	pc = 0x824225D4; continue 'dispatch;
	}
	// 824225D0: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 824225D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824225D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824225DC: 997F0049  stb r11, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 824225E0: 4BFFF7E1  bl 0x82421dc0
	ctx.lr = 0x824225E4;
	sub_82421DC0(ctx, base);
	// 824225E4: 4800C96D  bl 0x8242ef50
	ctx.lr = 0x824225E8;
	sub_8242EF50(ctx, base);
	// 824225E8: 4BFFEA89  bl 0x82421070
	ctx.lr = 0x824225EC;
	sub_82421070(ctx, base);
	// 824225EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824225F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824225F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824225F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824225FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422600 size=108
    let mut pc: u32 = 0x82422600;
    'dispatch: loop {
        match pc {
            0x82422600 => {
    //   block [0x82422600..0x8242266C)
	// 82422600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422608: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242260C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422614: 4BFFEA1D  bl 0x82421030
	ctx.lr = 0x82422618;
	sub_82421030(ctx, base);
	// 82422618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242261C: 4BFFFF25  bl 0x82422540
	ctx.lr = 0x82422620;
	sub_82422540(ctx, base);
	// 82422620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422624: 4BFFFF7D  bl 0x824225a0
	ctx.lr = 0x82422628;
	sub_824225A0(ctx, base);
	// 82422628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242262C: 4BFFF795  bl 0x82421dc0
	ctx.lr = 0x82422630;
	sub_82421DC0(ctx, base);
	// 82422630: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 82422634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422638: 409A0010  bne cr6, 0x82422648
	if !ctx.cr[6].eq {
	pc = 0x82422648; continue 'dispatch;
	}
	// 8242263C: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 82422640: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422644: 419A0010  beq cr6, 0x82422654
	if ctx.cr[6].eq {
	pc = 0x82422654; continue 'dispatch;
	}
	// 82422648: 48000089  bl 0x824226d0
	ctx.lr = 0x8242264C;
	sub_824226D0(ctx, base);
	// 8242264C: 4BFF868D  bl 0x8241acd8
	ctx.lr = 0x82422650;
	sub_8241ACD8(ctx, base);
	// 82422650: 4BFFFFE0  b 0x82422630
	pc = 0x82422630; continue 'dispatch;
	// 82422654: 4BFFEA1D  bl 0x82421070
	ctx.lr = 0x82422658;
	sub_82421070(ctx, base);
	// 82422658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422670 size=96
    let mut pc: u32 = 0x82422670;
    'dispatch: loop {
        match pc {
            0x82422670 => {
    //   block [0x82422670..0x824226D0)
	// 82422670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422680: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422684: 4BFFE9AD  bl 0x82421030
	ctx.lr = 0x82422688;
	sub_82421030(ctx, base);
	// 82422688: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242268C: 419A002C  beq cr6, 0x824226b8
	if ctx.cr[6].eq {
	pc = 0x824226B8; continue 'dispatch;
	}
	// 82422690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422694: 4BFFFEAD  bl 0x82422540
	ctx.lr = 0x82422698;
	sub_82422540(ctx, base);
	// 82422698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242269C: 4BFFFF65  bl 0x82422600
	ctx.lr = 0x824226A0;
	sub_82422600(ctx, base);
	// 824226A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824226A4: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 824226A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824226AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824226B0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824226B4: 48112B1D  bl 0x825351d0
	ctx.lr = 0x824226B8;
	sub_825351D0(ctx, base);
	// 824226B8: 4BFFE9B9  bl 0x82421070
	ctx.lr = 0x824226BC;
	sub_82421070(ctx, base);
	// 824226BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824226C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824226C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824226C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824226CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824226D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824226D0 size=244
    let mut pc: u32 = 0x824226D0;
    'dispatch: loop {
        match pc {
            0x824226D0 => {
    //   block [0x824226D0..0x824227C4)
	// 824226D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824226D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824226D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824226DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824226E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824226E4: 4BFFE94D  bl 0x82421030
	ctx.lr = 0x824226E8;
	sub_82421030(ctx, base);
	// 824226E8: 4800C869  bl 0x8242ef50
	ctx.lr = 0x824226EC;
	sub_8242EF50(ctx, base);
	// 824226EC: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 824226F0: 817F8E1C  lwz r11, -0x71e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-29156 as u32) ) } as u64;
	// 824226F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824226F8: 419A000C  beq cr6, 0x82422704
	if ctx.cr[6].eq {
	pc = 0x82422704; continue 'dispatch;
	}
	// 824226FC: 4800C855  bl 0x8242ef50
	ctx.lr = 0x82422700;
	sub_8242EF50(ctx, base);
	// 82422700: 480000A8  b 0x824227a8
	pc = 0x824227A8; continue 'dispatch;
	// 82422704: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82422708: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 8242270C: 4800C845  bl 0x8242ef50
	ctx.lr = 0x82422710;
	sub_8242EF50(ctx, base);
	// 82422710: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82422714: 3BCB8E30  addi r30, r11, -0x71d0
	ctx.r[30].s64 = ctx.r[11].s64 + -29136;
	// 82422718: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8242271C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422720: 419A0018  beq cr6, 0x82422738
	if ctx.cr[6].eq {
	pc = 0x82422738; continue 'dispatch;
	}
	// 82422724: 397EFFF4  addi r11, r30, -0xc
	ctx.r[11].s64 = ctx.r[30].s64 + -12;
	// 82422728: 807EFFF8  lwz r3, -8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242272C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422734: 4E800421  bctrl
	ctx.lr = 0x82422738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422738: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242273C: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 82422740: 4BFFF919  bl 0x82422058
	ctx.lr = 0x82422744;
	sub_82422058(ctx, base);
	// 82422744: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82422748: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 8242274C: 4BFFFCA5  bl 0x824223f0
	ctx.lr = 0x82422750;
	sub_824223F0(ctx, base);
	// 82422750: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82422754: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 82422758: 4BFF4D11  bl 0x82417468
	ctx.lr = 0x8242275C;
	sub_82417468(ctx, base);
	// 8242275C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82422760: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 82422764: 4BFFFC8D  bl 0x824223f0
	ctx.lr = 0x82422768;
	sub_824223F0(ctx, base);
	// 82422768: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8242276C: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 82422770: 4BFFF8E9  bl 0x82422058
	ctx.lr = 0x82422774;
	sub_82422058(ctx, base);
	// 82422774: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82422778: 917F8E1C  stw r11, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[11].u32 ) };
	// 8242277C: 4BFFEEBD  bl 0x82421638
	ctx.lr = 0x82422780;
	sub_82421638(ctx, base);
	// 82422780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82422784: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82422788: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242278C: 915F8E1C  stw r10, -0x71e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-29156 as u32), ctx.r[10].u32 ) };
	// 82422790: 419A0018  beq cr6, 0x824227a8
	if ctx.cr[6].eq {
	pc = 0x824227A8; continue 'dispatch;
	}
	// 82422794: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 82422798: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242279C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824227A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824227A4: 4E800421  bctrl
	ctx.lr = 0x824227A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824227A8: 4BFFE8C9  bl 0x82421070
	ctx.lr = 0x824227AC;
	sub_82421070(ctx, base);
	// 824227AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824227B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824227B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824227B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824227BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824227C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824227C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824227C8 size=12
    let mut pc: u32 = 0x824227C8;
    'dispatch: loop {
        match pc {
            0x824227C8 => {
    //   block [0x824227C8..0x824227D4)
	// 824227C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824227CC: 386B1850  addi r3, r11, 0x1850
	ctx.r[3].s64 = ctx.r[11].s64 + 6224;
	// 824227D0: 4800AFA8  b 0x8242d778
	sub_8242D778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824227D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824227D8 size=28
    let mut pc: u32 = 0x824227D8;
    'dispatch: loop {
        match pc {
            0x824227D8 => {
    //   block [0x824227D8..0x824227F4)
	// 824227D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824227DC: 409A0018  bne cr6, 0x824227f4
	if !ctx.cr[6].eq {
		sub_824227F4(ctx, base);
		return;
	}
	// 824227E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824227E4: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 824227E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824227EC: 386B18EC  addi r3, r11, 0x18ec
	ctx.r[3].s64 = ctx.r[11].s64 + 6380;
	// 824227F0: 48008758  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824227F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824227F4 size=32
    let mut pc: u32 = 0x824227F4;
    'dispatch: loop {
        match pc {
            0x824227F4 => {
    //   block [0x824227F4..0x82422814)
	// 824227F4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824227F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824227FC: 409A0018  bne cr6, 0x82422814
	if !ctx.cr[6].eq {
		sub_82422814(ctx, base);
		return;
	}
	// 82422800: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422804: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422808: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242280C: 386B18E0  addi r3, r11, 0x18e0
	ctx.r[3].s64 = ctx.r[11].s64 + 6368;
	// 82422810: 48008738  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82422814 size=44
    let mut pc: u32 = 0x82422814;
    'dispatch: loop {
        match pc {
            0x82422814 => {
    //   block [0x82422814..0x82422840)
	// 82422814: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82422818: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242281C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82422820: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82422824: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82422828: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8242282C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82422830: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82422834: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82422838: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8242283C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82422840 size=32
    let mut pc: u32 = 0x82422840;
    'dispatch: loop {
        match pc {
            0x82422840 => {
    //   block [0x82422840..0x82422860)
	// 82422840: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82422844: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82422848: 409A0018  bne cr6, 0x82422860
	if !ctx.cr[6].eq {
		sub_82422860(ctx, base);
		return;
	}
	// 8242284C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422850: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82422854: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422858: 386B1940  addi r3, r11, 0x1940
	ctx.r[3].s64 = ctx.r[11].s64 + 6464;
	// 8242285C: 480086EC  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82422860 size=32
    let mut pc: u32 = 0x82422860;
    'dispatch: loop {
        match pc {
            0x82422860 => {
    //   block [0x82422860..0x82422880)
	// 82422860: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422864: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82422868: 409A0018  bne cr6, 0x82422880
	if !ctx.cr[6].eq {
		sub_82422880(ctx, base);
		return;
	}
	// 8242286C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422870: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422874: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422878: 386B1934  addi r3, r11, 0x1934
	ctx.r[3].s64 = ctx.r[11].s64 + 6452;
	// 8242287C: 480086CC  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82422880 size=32
    let mut pc: u32 = 0x82422880;
    'dispatch: loop {
        match pc {
            0x82422880 => {
    //   block [0x82422880..0x824228A0)
	// 82422880: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82422884: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82422888: 40820018  bne 0x824228a0
	if !ctx.cr[0].eq {
		sub_824228A0(ctx, base);
		return;
	}
	// 8242288C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422890: 388B191C  addi r4, r11, 0x191c
	ctx.r[4].s64 = ctx.r[11].s64 + 6428;
	// 82422894: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422898: 386B1910  addi r3, r11, 0x1910
	ctx.r[3].s64 = ctx.r[11].s64 + 6416;
	// 8242289C: 480086AC  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824228A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824228A0 size=140
    let mut pc: u32 = 0x824228A0;
    'dispatch: loop {
        match pc {
            0x824228A0 => {
    //   block [0x824228A0..0x8242292C)
	// 824228A0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824228A4: 409A0088  bne cr6, 0x8242292c
	if !ctx.cr[6].eq {
		sub_8242292C(ctx, base);
		return;
	}
	// 824228A8: 81030024  lwz r8, 0x24(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 824228AC: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824228B0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824228B4: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 824228B8: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 824228BC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824228C0: 41980008  blt cr6, 0x824228c8
	if ctx.cr[6].lt {
	pc = 0x824228C8; continue 'dispatch;
	}
	// 824228C4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 824228C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824228CC: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824228D0: 40980008  bge cr6, 0x824228d8
	if !ctx.cr[6].lt {
	pc = 0x824228D8; continue 'dispatch;
	}
	// 824228D4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824228D8: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824228DC: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824228E0: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824228E4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824228E8: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824228EC: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824228F0: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824228F4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824228F8: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824228FC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82422900: 7CEA4BD6  divw r7, r10, r9
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82422904: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82422908: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8242290C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82422910: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422914: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82422918: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8242291C: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422920: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82422924: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82422928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242292C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242292C size=140
    let mut pc: u32 = 0x8242292C;
    'dispatch: loop {
        match pc {
            0x8242292C => {
    //   block [0x8242292C..0x824229B8)
	// 8242292C: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82422930: 409A0088  bne cr6, 0x824229b8
	if !ctx.cr[6].eq {
		sub_824229B8(ctx, base);
		return;
	}
	// 82422934: 81030024  lwz r8, 0x24(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82422938: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242293C: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82422940: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82422944: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82422948: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242294C: 41980008  blt cr6, 0x82422954
	if ctx.cr[6].lt {
	pc = 0x82422954; continue 'dispatch;
	}
	// 82422950: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82422954: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82422958: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8242295C: 40980008  bge cr6, 0x82422964
	if !ctx.cr[6].lt {
	pc = 0x82422964; continue 'dispatch;
	}
	// 82422960: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82422964: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82422968: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242296C: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82422970: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82422974: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82422978: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242297C: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82422980: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82422984: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82422988: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8242298C: 7CEA4BD6  divw r7, r10, r9
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82422990: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82422994: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82422998: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8242299C: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 824229A0: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 824229A4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824229A8: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 824229AC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824229B0: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824229B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824229B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824229B8 size=24
    let mut pc: u32 = 0x824229B8;
    'dispatch: loop {
        match pc {
            0x824229B8 => {
    //   block [0x824229B8..0x824229D0)
	// 824229B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824229BC: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824229C0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824229C4: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 824229C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824229CC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824229D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824229D0 size=16
    let mut pc: u32 = 0x824229D0;
    'dispatch: loop {
        match pc {
            0x824229D0 => {
    //   block [0x824229D0..0x824229E0)
	// 824229D0: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 824229D4: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 824229D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824229DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824229E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824229E0 size=420
    let mut pc: u32 = 0x824229E0;
    'dispatch: loop {
        match pc {
            0x824229E0 => {
    //   block [0x824229E0..0x82422B84)
	// 824229E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824229E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824229E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824229EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824229F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824229F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824229F8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824229FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82422A00: 409A001C  bne cr6, 0x82422a1c
	if !ctx.cr[6].eq {
	pc = 0x82422A1C; continue 'dispatch;
	}
	// 82422A04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422A08: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82422A0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422A10: 386B1958  addi r3, r11, 0x1958
	ctx.r[3].s64 = ctx.r[11].s64 + 6488;
	// 82422A14: 48008535  bl 0x8242af48
	ctx.lr = 0x82422A18;
	sub_8242AF48(ctx, base);
	// 82422A18: 48000154  b 0x82422b6c
	pc = 0x82422B6C; continue 'dispatch;
	// 82422A1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422A20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82422A24: 409A0018  bne cr6, 0x82422a3c
	if !ctx.cr[6].eq {
	pc = 0x82422A3C; continue 'dispatch;
	}
	// 82422A28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422A2C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422A30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422A34: 386B194C  addi r3, r11, 0x194c
	ctx.r[3].s64 = ctx.r[11].s64 + 6476;
	// 82422A38: 4BFFFFDC  b 0x82422a14
	pc = 0x82422A14; continue 'dispatch;
	// 82422A3C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422A40: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82422A44: 40810128  ble 0x82422b6c
	if !ctx.cr[0].gt {
	pc = 0x82422B6C; continue 'dispatch;
	}
	// 82422A48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422A4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82422A50: 419A011C  beq cr6, 0x82422b6c
	if ctx.cr[6].eq {
	pc = 0x82422B6C; continue 'dispatch;
	}
	// 82422A54: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82422A58: 409A00C4  bne cr6, 0x82422b1c
	if !ctx.cr[6].eq {
	pc = 0x82422B1C; continue 'dispatch;
	}
	// 82422A5C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82422A60: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82422A64: 41820014  beq 0x82422a78
	if ctx.cr[0].eq {
	pc = 0x82422A78; continue 'dispatch;
	}
	// 82422A68: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82422A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82422A70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422A74: 4E800421  bctrl
	ctx.lr = 0x82422A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422A78: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422A7C: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82422A80: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82422A84: 7D682050  subf r11, r8, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 82422A88: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82422A8C: 4098002C  bge cr6, 0x82422ab8
	if !ctx.cr[6].lt {
	pc = 0x82422AB8; continue 'dispatch;
	}
	// 82422A90: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422A94: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82422A98: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82422A9C: 41980008  blt cr6, 0x82422aa4
	if ctx.cr[6].lt {
	pc = 0x82422AA4; continue 'dispatch;
	}
	// 82422AA0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82422AA4: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82422AA8: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82422AAC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82422AB0: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82422AB4: 4811209D  bl 0x82534b50
	ctx.lr = 0x82422AB8;
	sub_82534B50(ctx, base);
	// 82422AB8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422ABC: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82422AC0: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422AC4: 7D235850  subf r9, r3, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82422AC8: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82422ACC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82422AD0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82422AD4: 40990024  ble cr6, 0x82422af8
	if !ctx.cr[6].gt {
	pc = 0x82422AF8; continue 'dispatch;
	}
	// 82422AD8: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82422ADC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82422AE0: 41980008  blt cr6, 0x82422ae8
	if ctx.cr[6].lt {
	pc = 0x82422AE8; continue 'dispatch;
	}
	// 82422AE4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82422AE8: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82422AEC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82422AF0: 7C8A4A14  add r4, r10, r9
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82422AF4: 4811205D  bl 0x82534b50
	ctx.lr = 0x82422AF8;
	sub_82534B50(ctx, base);
	// 82422AF8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422AFC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82422B00: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82422B04: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82422B08: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82422B0C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422B10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82422B14: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82422B18: 48000054  b 0x82422b6c
	pc = 0x82422B6C; continue 'dispatch;
	// 82422B1C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82422B20: 409A0024  bne cr6, 0x82422b44
	if !ctx.cr[6].eq {
	pc = 0x82422B44; continue 'dispatch;
	}
	// 82422B24: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82422B28: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82422B2C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82422B30: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82422B34: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422B38: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82422B3C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82422B40: 4800002C  b 0x82422b6c
	pc = 0x82422B6C; continue 'dispatch;
	// 82422B44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82422B48: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82422B4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82422B50: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82422B54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82422B58: 41820014  beq 0x82422b6c
	if ctx.cr[0].eq {
	pc = 0x82422B6C; continue 'dispatch;
	}
	// 82422B5C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82422B60: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82422B64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422B68: 4E800421  bctrl
	ctx.lr = 0x82422B6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422B6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82422B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422B78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82422B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422B88 size=476
    let mut pc: u32 = 0x82422B88;
    'dispatch: loop {
        match pc {
            0x82422B88 => {
    //   block [0x82422B88..0x82422D64)
	// 82422B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82422B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422BA0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82422BA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82422BA8: 409A001C  bne cr6, 0x82422bc4
	if !ctx.cr[6].eq {
	pc = 0x82422BC4; continue 'dispatch;
	}
	// 82422BAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422BB0: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82422BB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422BB8: 386B197C  addi r3, r11, 0x197c
	ctx.r[3].s64 = ctx.r[11].s64 + 6524;
	// 82422BBC: 4800838D  bl 0x8242af48
	ctx.lr = 0x82422BC0;
	sub_8242AF48(ctx, base);
	// 82422BC0: 4800018C  b 0x82422d4c
	pc = 0x82422D4C; continue 'dispatch;
	// 82422BC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82422BCC: 409A0018  bne cr6, 0x82422be4
	if !ctx.cr[6].eq {
	pc = 0x82422BE4; continue 'dispatch;
	}
	// 82422BD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422BD4: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422BD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422BDC: 386B1970  addi r3, r11, 0x1970
	ctx.r[3].s64 = ctx.r[11].s64 + 6512;
	// 82422BE0: 4BFFFFDC  b 0x82422bbc
	pc = 0x82422BBC; continue 'dispatch;
	// 82422BE4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82422BE8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82422BEC: 40820018  bne 0x82422c04
	if !ctx.cr[0].eq {
	pc = 0x82422C04; continue 'dispatch;
	}
	// 82422BF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422BF4: 388B191C  addi r4, r11, 0x191c
	ctx.r[4].s64 = ctx.r[11].s64 + 6428;
	// 82422BF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422BFC: 386B1964  addi r3, r11, 0x1964
	ctx.r[3].s64 = ctx.r[11].s64 + 6500;
	// 82422C00: 4BFFFFBC  b 0x82422bbc
	pc = 0x82422BBC; continue 'dispatch;
	// 82422C04: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422C08: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82422C0C: 40810140  ble 0x82422d4c
	if !ctx.cr[0].gt {
	pc = 0x82422D4C; continue 'dispatch;
	}
	// 82422C10: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82422C14: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82422C18: 41820134  beq 0x82422d4c
	if ctx.cr[0].eq {
	pc = 0x82422D4C; continue 'dispatch;
	}
	// 82422C1C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82422C20: 409A0080  bne cr6, 0x82422ca0
	if !ctx.cr[6].eq {
	pc = 0x82422CA0; continue 'dispatch;
	}
	// 82422C24: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82422C28: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82422C2C: 7D294050  subf r9, r9, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82422C30: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82422C34: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82422C38: 7D0A5BD6  divw r8, r10, r11
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 82422C3C: 7CE95BD6  divw r7, r9, r11
	ctx.r[7].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 82422C40: 7D0859D6  mullw r8, r8, r11
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82422C44: 7D6759D6  mullw r11, r7, r11
	ctx.r[11].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82422C48: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82422C4C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82422C50: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82422C54: 409A001C  bne cr6, 0x82422c70
	if !ctx.cr[6].eq {
	pc = 0x82422C70; continue 'dispatch;
	}
	// 82422C58: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82422C5C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82422C60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422C64: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82422C68: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82422C6C: 48000020  b 0x82422c8c
	pc = 0x82422C8C; continue 'dispatch;
	// 82422C70: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82422C74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82422C78: 41820014  beq 0x82422c8c
	if ctx.cr[0].eq {
	pc = 0x82422C8C; continue 'dispatch;
	}
	// 82422C7C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82422C80: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82422C84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422C88: 4E800421  bctrl
	ctx.lr = 0x82422C8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422C8C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82422C90: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422C94: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82422C98: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82422C9C: 480000B0  b 0x82422d4c
	pc = 0x82422D4C; continue 'dispatch;
	// 82422CA0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82422CA4: 409A0080  bne cr6, 0x82422d24
	if !ctx.cr[6].eq {
	pc = 0x82422D24; continue 'dispatch;
	}
	// 82422CA8: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82422CAC: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82422CB0: 7D294050  subf r9, r9, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82422CB4: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82422CB8: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82422CBC: 7D0A5BD6  divw r8, r10, r11
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 82422CC0: 7CE95BD6  divw r7, r9, r11
	ctx.r[7].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 82422CC4: 7D0859D6  mullw r8, r8, r11
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82422CC8: 7D6759D6  mullw r11, r7, r11
	ctx.r[11].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82422CCC: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82422CD0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82422CD4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82422CD8: 409A001C  bne cr6, 0x82422cf4
	if !ctx.cr[6].eq {
	pc = 0x82422CF4; continue 'dispatch;
	}
	// 82422CDC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82422CE0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82422CE4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422CE8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82422CEC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82422CF0: 48000020  b 0x82422d10
	pc = 0x82422D10; continue 'dispatch;
	// 82422CF4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82422CF8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82422CFC: 41820014  beq 0x82422d10
	if ctx.cr[0].eq {
	pc = 0x82422D10; continue 'dispatch;
	}
	// 82422D00: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82422D04: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82422D08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422D0C: 4E800421  bctrl
	ctx.lr = 0x82422D10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422D10: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82422D14: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422D18: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82422D1C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82422D20: 4800002C  b 0x82422d4c
	pc = 0x82422D4C; continue 'dispatch;
	// 82422D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82422D28: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82422D2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82422D30: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82422D34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82422D38: 41820014  beq 0x82422d4c
	if ctx.cr[0].eq {
	pc = 0x82422D4C; continue 'dispatch;
	}
	// 82422D3C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82422D40: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82422D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82422D48: 4E800421  bctrl
	ctx.lr = 0x82422D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82422D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82422D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422D58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82422D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422D68 size=100
    let mut pc: u32 = 0x82422D68;
    'dispatch: loop {
        match pc {
            0x82422D68 => {
    //   block [0x82422D68..0x82422DCC)
	// 82422D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422D70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82422D74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422D7C: 4800AA05  bl 0x8242d780
	ctx.lr = 0x82422D80;
	sub_8242D780(ctx, base);
	// 82422D80: 4800AAF9  bl 0x8242d878
	ctx.lr = 0x82422D84;
	sub_8242D878(ctx, base);
	// 82422D84: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 82422D88: 83FE8E34  lwz r31, -0x71cc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-29132 as u32) ) } as u64;
	// 82422D8C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82422D90: 409A0018  bne cr6, 0x82422da8
	if !ctx.cr[6].eq {
	pc = 0x82422DA8; continue 'dispatch;
	}
	// 82422D94: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82422D98: 38A06C00  li r5, 0x6c00
	ctx.r[5].s64 = 27648;
	// 82422D9C: 386B3680  addi r3, r11, 0x3680
	ctx.r[3].s64 = ctx.r[11].s64 + 13952;
	// 82422DA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82422DA4: 4811242D  bl 0x825351d0
	ctx.lr = 0x82422DA8;
	sub_825351D0(ctx, base);
	// 82422DA8: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82422DAC: 917E8E34  stw r11, -0x71cc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-29132 as u32), ctx.r[11].u32 ) };
	// 82422DB0: 4800AB09  bl 0x8242d8b8
	ctx.lr = 0x82422DB4;
	sub_8242D8B8(ctx, base);
	// 82422DB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82422DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422DC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82422DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422DD0 size=80
    let mut pc: u32 = 0x82422DD0;
    'dispatch: loop {
        match pc {
            0x82422DD0 => {
    //   block [0x82422DD0..0x82422E20)
	// 82422DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422DDC: 4800AA9D  bl 0x8242d878
	ctx.lr = 0x82422DE0;
	sub_8242D878(ctx, base);
	// 82422DE0: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82422DE4: 816A8E34  lwz r11, -0x71cc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29132 as u32) ) } as u64;
	// 82422DE8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82422DEC: 916A8E34  stw r11, -0x71cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29132 as u32), ctx.r[11].u32 ) };
	// 82422DF0: 40820018  bne 0x82422e08
	if !ctx.cr[0].eq {
	pc = 0x82422E08; continue 'dispatch;
	}
	// 82422DF4: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82422DF8: 38A06C00  li r5, 0x6c00
	ctx.r[5].s64 = 27648;
	// 82422DFC: 386B3680  addi r3, r11, 0x3680
	ctx.r[3].s64 = ctx.r[11].s64 + 13952;
	// 82422E00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82422E04: 481123CD  bl 0x825351d0
	ctx.lr = 0x82422E08;
	sub_825351D0(ctx, base);
	// 82422E08: 4800AAB1  bl 0x8242d8b8
	ctx.lr = 0x82422E0C;
	sub_8242D8B8(ctx, base);
	// 82422E0C: 4800A9F5  bl 0x8242d800
	ctx.lr = 0x82422E10;
	sub_8242D800(ctx, base);
	// 82422E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422E20 size=136
    let mut pc: u32 = 0x82422E20;
    'dispatch: loop {
        match pc {
            0x82422E20 => {
    //   block [0x82422E20..0x82422EA8)
	// 82422E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422E34: 4800AA45  bl 0x8242d878
	ctx.lr = 0x82422E38;
	sub_8242D878(ctx, base);
	// 82422E38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82422E3C: 409A001C  bne cr6, 0x82422e58
	if !ctx.cr[6].eq {
	pc = 0x82422E58; continue 'dispatch;
	}
	// 82422E40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422E44: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82422E48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422E4C: 386B18A4  addi r3, r11, 0x18a4
	ctx.r[3].s64 = ctx.r[11].s64 + 6308;
	// 82422E50: 480080F9  bl 0x8242af48
	ctx.lr = 0x82422E54;
	sub_8242AF48(ctx, base);
	// 82422E54: 4800003C  b 0x82422e90
	pc = 0x82422E90; continue 'dispatch;
	// 82422E58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422E5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82422E60: 409A0018  bne cr6, 0x82422e78
	if !ctx.cr[6].eq {
	pc = 0x82422E78; continue 'dispatch;
	}
	// 82422E64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422E68: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422E6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422E70: 386B1878  addi r3, r11, 0x1878
	ctx.r[3].s64 = ctx.r[11].s64 + 6264;
	// 82422E74: 4BFFFFDC  b 0x82422e50
	pc = 0x82422E50; continue 'dispatch;
	// 82422E78: 38A00048  li r5, 0x48
	ctx.r[5].s64 = 72;
	// 82422E7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82422E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422E84: 4811234D  bl 0x825351d0
	ctx.lr = 0x82422E88;
	sub_825351D0(ctx, base);
	// 82422E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82422E8C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82422E90: 4800AA29  bl 0x8242d8b8
	ctx.lr = 0x82422E94;
	sub_8242D8B8(ctx, base);
	// 82422E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422EA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422EA8 size=124
    let mut pc: u32 = 0x82422EA8;
    'dispatch: loop {
        match pc {
            0x82422EA8 => {
    //   block [0x82422EA8..0x82422F24)
	// 82422EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422EB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422EBC: 4800A9BD  bl 0x8242d878
	ctx.lr = 0x82422EC0;
	sub_8242D878(ctx, base);
	// 82422EC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82422EC4: 409A0020  bne cr6, 0x82422ee4
	if !ctx.cr[6].eq {
	pc = 0x82422EE4; continue 'dispatch;
	}
	// 82422EC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422ECC: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82422ED0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422ED4: 386B18BC  addi r3, r11, 0x18bc
	ctx.r[3].s64 = ctx.r[11].s64 + 6332;
	// 82422ED8: 48008071  bl 0x8242af48
	ctx.lr = 0x82422EDC;
	sub_8242AF48(ctx, base);
	// 82422EDC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82422EE0: 48000028  b 0x82422f08
	pc = 0x82422F08; continue 'dispatch;
	// 82422EE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422EE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82422EEC: 409A0018  bne cr6, 0x82422f04
	if !ctx.cr[6].eq {
	pc = 0x82422F04; continue 'dispatch;
	}
	// 82422EF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422EF4: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422EF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422EFC: 386B18B0  addi r3, r11, 0x18b0
	ctx.r[3].s64 = ctx.r[11].s64 + 6320;
	// 82422F00: 4BFFFFD8  b 0x82422ed8
	pc = 0x82422ED8; continue 'dispatch;
	// 82422F04: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82422F08: 4800A9B1  bl 0x8242d8b8
	ctx.lr = 0x82422F0C;
	sub_8242D8B8(ctx, base);
	// 82422F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422F28 size=112
    let mut pc: u32 = 0x82422F28;
    'dispatch: loop {
        match pc {
            0x82422F28 => {
    //   block [0x82422F28..0x82422F98)
	// 82422F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422F2C: 48112191  bl 0x825350bc
	ctx.lr = 0x82422F30;
	sub_82535080(ctx, base);
	// 82422F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422F34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422F38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82422F3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82422F40: 4800A939  bl 0x8242d878
	ctx.lr = 0x82422F44;
	sub_8242D878(ctx, base);
	// 82422F44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82422F48: 409A001C  bne cr6, 0x82422f64
	if !ctx.cr[6].eq {
	pc = 0x82422F64; continue 'dispatch;
	}
	// 82422F4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422F50: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82422F54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422F58: 386B18D4  addi r3, r11, 0x18d4
	ctx.r[3].s64 = ctx.r[11].s64 + 6356;
	// 82422F5C: 48007FED  bl 0x8242af48
	ctx.lr = 0x82422F60;
	sub_8242AF48(ctx, base);
	// 82422F60: 4800002C  b 0x82422f8c
	pc = 0x82422F8C; continue 'dispatch;
	// 82422F64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82422F68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82422F6C: 409A0018  bne cr6, 0x82422f84
	if !ctx.cr[6].eq {
	pc = 0x82422F84; continue 'dispatch;
	}
	// 82422F70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422F74: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82422F78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422F7C: 386B18C8  addi r3, r11, 0x18c8
	ctx.r[3].s64 = ctx.r[11].s64 + 6344;
	// 82422F80: 4BFFFFDC  b 0x82422f5c
	pc = 0x82422F5C; continue 'dispatch;
	// 82422F84: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82422F88: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82422F8C: 4800A92D  bl 0x8242d8b8
	ctx.lr = 0x82422F90;
	sub_8242D8B8(ctx, base);
	// 82422F90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82422F94: 48112178  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422F98 size=56
    let mut pc: u32 = 0x82422F98;
    'dispatch: loop {
        match pc {
            0x82422F98 => {
    //   block [0x82422F98..0x82422FD0)
	// 82422F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422FA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422FAC: 4800A8CD  bl 0x8242d878
	ctx.lr = 0x82422FB0;
	sub_8242D878(ctx, base);
	// 82422FB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82422FB4: 4BFFF825  bl 0x824227d8
	ctx.lr = 0x82422FB8;
	sub_824227D8(ctx, base);
	// 82422FB8: 4800A901  bl 0x8242d8b8
	ctx.lr = 0x82422FBC;
	sub_8242D8B8(ctx, base);
	// 82422FBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82422FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82422FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82422FC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82422FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82422FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82422FD0 size=192
    let mut pc: u32 = 0x82422FD0;
    'dispatch: loop {
        match pc {
            0x82422FD0 => {
    //   block [0x82422FD0..0x82423090)
	// 82422FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82422FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82422FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82422FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82422FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82422FE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82422FE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82422FEC: 4800A88D  bl 0x8242d878
	ctx.lr = 0x82422FF0;
	sub_8242D878(ctx, base);
	// 82422FF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82422FF4: 409A001C  bne cr6, 0x82423010
	if !ctx.cr[6].eq {
	pc = 0x82423010; continue 'dispatch;
	}
	// 82422FF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82422FFC: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82423000: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423004: 386B1904  addi r3, r11, 0x1904
	ctx.r[3].s64 = ctx.r[11].s64 + 6404;
	// 82423008: 48007F41  bl 0x8242af48
	ctx.lr = 0x8242300C;
	sub_8242AF48(ctx, base);
	// 8242300C: 48000060  b 0x8242306c
	pc = 0x8242306C; continue 'dispatch;
	// 82423010: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423014: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82423018: 409A0018  bne cr6, 0x82423030
	if !ctx.cr[6].eq {
	pc = 0x82423030; continue 'dispatch;
	}
	// 8242301C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423020: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82423024: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423028: 386B18F8  addi r3, r11, 0x18f8
	ctx.r[3].s64 = ctx.r[11].s64 + 6392;
	// 8242302C: 4BFFFFDC  b 0x82423008
	pc = 0x82423008; continue 'dispatch;
	// 82423030: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82423034: 409A000C  bne cr6, 0x82423040
	if !ctx.cr[6].eq {
	pc = 0x82423040; continue 'dispatch;
	}
	// 82423038: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242303C: 48000034  b 0x82423070
	pc = 0x82423070; continue 'dispatch;
	// 82423040: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82423044: 409A000C  bne cr6, 0x82423050
	if !ctx.cr[6].eq {
	pc = 0x82423050; continue 'dispatch;
	}
	// 82423048: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8242304C: 48000024  b 0x82423070
	pc = 0x82423070; continue 'dispatch;
	// 82423050: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82423054: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82423058: 41820014  beq 0x8242306c
	if ctx.cr[0].eq {
	pc = 0x8242306C; continue 'dispatch;
	}
	// 8242305C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82423060: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82423064: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423068: 4E800421  bctrl
	ctx.lr = 0x8242306C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242306C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82423070: 4800A849  bl 0x8242d8b8
	ctx.lr = 0x82423074;
	sub_8242D8B8(ctx, base);
	// 82423074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242307C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423084: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82423088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242308C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423090 size=64
    let mut pc: u32 = 0x82423090;
    'dispatch: loop {
        match pc {
            0x82423090 => {
    //   block [0x82423090..0x824230D0)
	// 82423090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423094: 48112025  bl 0x825350b8
	ctx.lr = 0x82423098;
	sub_82535080(ctx, base);
	// 82423098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242309C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824230A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824230A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824230A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824230AC: 4800A7CD  bl 0x8242d878
	ctx.lr = 0x824230B0;
	sub_8242D878(ctx, base);
	// 824230B0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824230B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824230B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824230BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824230C0: 4BFFF781  bl 0x82422840
	ctx.lr = 0x824230C4;
	sub_82422840(ctx, base);
	// 824230C4: 4800A7F5  bl 0x8242d8b8
	ctx.lr = 0x824230C8;
	sub_8242D8B8(ctx, base);
	// 824230C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824230CC: 4811203C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824230D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824230D0 size=56
    let mut pc: u32 = 0x824230D0;
    'dispatch: loop {
        match pc {
            0x824230D0 => {
    //   block [0x824230D0..0x82423108)
	// 824230D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824230D4: 48111FE9  bl 0x825350bc
	ctx.lr = 0x824230D8;
	sub_82535080(ctx, base);
	// 824230D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824230DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824230E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824230E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824230E8: 4800A791  bl 0x8242d878
	ctx.lr = 0x824230EC;
	sub_8242D878(ctx, base);
	// 824230EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824230F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824230F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824230F8: 4BFFF8E9  bl 0x824229e0
	ctx.lr = 0x824230FC;
	sub_824229E0(ctx, base);
	// 824230FC: 4800A7BD  bl 0x8242d8b8
	ctx.lr = 0x82423100;
	sub_8242D8B8(ctx, base);
	// 82423100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82423104: 48112008  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423108 size=56
    let mut pc: u32 = 0x82423108;
    'dispatch: loop {
        match pc {
            0x82423108 => {
    //   block [0x82423108..0x82423140)
	// 82423108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242310C: 48111FB1  bl 0x825350bc
	ctx.lr = 0x82423110;
	sub_82535080(ctx, base);
	// 82423110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82423118: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242311C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82423120: 4800A759  bl 0x8242d878
	ctx.lr = 0x82423124;
	sub_8242D878(ctx, base);
	// 82423124: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82423128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242312C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423130: 4BFFFA59  bl 0x82422b88
	ctx.lr = 0x82423134;
	sub_82422B88(ctx, base);
	// 82423134: 4800A785  bl 0x8242d8b8
	ctx.lr = 0x82423138;
	sub_8242D8B8(ctx, base);
	// 82423138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242313C: 48111FD0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423140 size=252
    let mut pc: u32 = 0x82423140;
    'dispatch: loop {
        match pc {
            0x82423140 => {
    //   block [0x82423140..0x8242323C)
	// 82423140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423144: 48111F75  bl 0x825350b8
	ctx.lr = 0x82423148;
	sub_82535080(ctx, base);
	// 82423148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242314C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82423150: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82423154: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82423158: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8242315C: 4800A71D  bl 0x8242d878
	ctx.lr = 0x82423160;
	sub_8242D878(ctx, base);
	// 82423160: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82423164: 409A0020  bne cr6, 0x82423184
	if !ctx.cr[6].eq {
	pc = 0x82423184; continue 'dispatch;
	}
	// 82423168: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242316C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82423170: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423174: 386B1994  addi r3, r11, 0x1994
	ctx.r[3].s64 = ctx.r[11].s64 + 6548;
	// 82423178: 48007DD1  bl 0x8242af48
	ctx.lr = 0x8242317C;
	sub_8242AF48(ctx, base);
	// 8242317C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82423180: 480000AC  b 0x8242322c
	pc = 0x8242322C; continue 'dispatch;
	// 82423184: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423188: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242318C: 409A0018  bne cr6, 0x824231a4
	if !ctx.cr[6].eq {
	pc = 0x824231A4; continue 'dispatch;
	}
	// 82423190: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423194: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82423198: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242319C: 386B1988  addi r3, r11, 0x1988
	ctx.r[3].s64 = ctx.r[11].s64 + 6536;
	// 824231A0: 4BFFFFD8  b 0x82423178
	pc = 0x82423178; continue 'dispatch;
	// 824231A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824231A8: 409A0018  bne cr6, 0x824231c0
	if !ctx.cr[6].eq {
	pc = 0x824231C0; continue 'dispatch;
	}
	// 824231AC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824231B0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824231B4: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824231B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824231BC: 4800001C  b 0x824231d8
	pc = 0x824231D8; continue 'dispatch;
	// 824231C0: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 824231C4: 409A0038  bne cr6, 0x824231fc
	if !ctx.cr[6].eq {
	pc = 0x824231FC; continue 'dispatch;
	}
	// 824231C8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824231CC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824231D0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824231D4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824231D8: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824231DC: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824231E0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824231E4: 40980008  bge cr6, 0x824231ec
	if !ctx.cr[6].lt {
	pc = 0x824231EC; continue 'dispatch;
	}
	// 824231E8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824231EC: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824231F0: 4198002C  blt cr6, 0x8242321c
	if ctx.cr[6].lt {
	pc = 0x8242321C; continue 'dispatch;
	}
	// 824231F4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 824231F8: 48000024  b 0x8242321c
	pc = 0x8242321C; continue 'dispatch;
	// 824231FC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82423200: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82423204: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82423208: 41820014  beq 0x8242321c
	if ctx.cr[0].eq {
	pc = 0x8242321C; continue 'dispatch;
	}
	// 8242320C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82423210: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82423214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423218: 4E800421  bctrl
	ctx.lr = 0x8242321C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242321C: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82423220: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82423224: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82423228: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8242322C: 4800A68D  bl 0x8242d8b8
	ctx.lr = 0x82423230;
	sub_8242D8B8(ctx, base);
	// 82423230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82423238: 48111ED0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423240 size=124
    let mut pc: u32 = 0x82423240;
    'dispatch: loop {
        match pc {
            0x82423240 => {
    //   block [0x82423240..0x824232BC)
	// 82423240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242324C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82423254: 4800A625  bl 0x8242d878
	ctx.lr = 0x82423258;
	sub_8242D878(ctx, base);
	// 82423258: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242325C: 409A0020  bne cr6, 0x8242327c
	if !ctx.cr[6].eq {
	pc = 0x8242327C; continue 'dispatch;
	}
	// 82423260: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423264: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82423268: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242326C: 386B19AC  addi r3, r11, 0x19ac
	ctx.r[3].s64 = ctx.r[11].s64 + 6572;
	// 82423270: 48007CD9  bl 0x8242af48
	ctx.lr = 0x82423274;
	sub_8242AF48(ctx, base);
	// 82423274: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82423278: 48000028  b 0x824232a0
	pc = 0x824232A0; continue 'dispatch;
	// 8242327C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82423284: 409A0018  bne cr6, 0x8242329c
	if !ctx.cr[6].eq {
	pc = 0x8242329C; continue 'dispatch;
	}
	// 82423288: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242328C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82423290: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423294: 386B19A0  addi r3, r11, 0x19a0
	ctx.r[3].s64 = ctx.r[11].s64 + 6560;
	// 82423298: 4BFFFFD8  b 0x82423270
	pc = 0x82423270; continue 'dispatch;
	// 8242329C: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824232A0: 4800A619  bl 0x8242d8b8
	ctx.lr = 0x824232A4;
	sub_8242D8B8(ctx, base);
	// 824232A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824232A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824232AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824232B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824232B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824232B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824232C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824232C0 size=124
    let mut pc: u32 = 0x824232C0;
    'dispatch: loop {
        match pc {
            0x824232C0 => {
    //   block [0x824232C0..0x8242333C)
	// 824232C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824232C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824232C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824232CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824232D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824232D4: 4800A5A5  bl 0x8242d878
	ctx.lr = 0x824232D8;
	sub_8242D878(ctx, base);
	// 824232D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824232DC: 409A0020  bne cr6, 0x824232fc
	if !ctx.cr[6].eq {
	pc = 0x824232FC; continue 'dispatch;
	}
	// 824232E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824232E4: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 824232E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824232EC: 386B19C4  addi r3, r11, 0x19c4
	ctx.r[3].s64 = ctx.r[11].s64 + 6596;
	// 824232F0: 48007C59  bl 0x8242af48
	ctx.lr = 0x824232F4;
	sub_8242AF48(ctx, base);
	// 824232F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824232F8: 48000028  b 0x82423320
	pc = 0x82423320; continue 'dispatch;
	// 824232FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423300: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82423304: 409A0018  bne cr6, 0x8242331c
	if !ctx.cr[6].eq {
	pc = 0x8242331C; continue 'dispatch;
	}
	// 82423308: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242330C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82423310: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423314: 386B19B8  addi r3, r11, 0x19b8
	ctx.r[3].s64 = ctx.r[11].s64 + 6584;
	// 82423318: 4BFFFFD8  b 0x824232f0
	pc = 0x824232F0; continue 'dispatch;
	// 8242331C: 83FF0020  lwz r31, 0x20(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423320: 4800A599  bl 0x8242d8b8
	ctx.lr = 0x82423324;
	sub_8242D8B8(ctx, base);
	// 82423324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242332C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82423338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423340 size=124
    let mut pc: u32 = 0x82423340;
    'dispatch: loop {
        match pc {
            0x82423340 => {
    //   block [0x82423340..0x824233BC)
	// 82423340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242334C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82423354: 4800A525  bl 0x8242d878
	ctx.lr = 0x82423358;
	sub_8242D878(ctx, base);
	// 82423358: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242335C: 409A0020  bne cr6, 0x8242337c
	if !ctx.cr[6].eq {
	pc = 0x8242337C; continue 'dispatch;
	}
	// 82423360: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423364: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82423368: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242336C: 386B19DC  addi r3, r11, 0x19dc
	ctx.r[3].s64 = ctx.r[11].s64 + 6620;
	// 82423370: 48007BD9  bl 0x8242af48
	ctx.lr = 0x82423374;
	sub_8242AF48(ctx, base);
	// 82423374: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82423378: 48000028  b 0x824233a0
	pc = 0x824233A0; continue 'dispatch;
	// 8242337C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423380: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82423384: 409A0018  bne cr6, 0x8242339c
	if !ctx.cr[6].eq {
	pc = 0x8242339C; continue 'dispatch;
	}
	// 82423388: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242338C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82423390: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423394: 386B19D0  addi r3, r11, 0x19d0
	ctx.r[3].s64 = ctx.r[11].s64 + 6608;
	// 82423398: 4BFFFFD8  b 0x82423370
	pc = 0x82423370; continue 'dispatch;
	// 8242339C: 83FF0024  lwz r31, 0x24(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824233A0: 4800A519  bl 0x8242d8b8
	ctx.lr = 0x824233A4;
	sub_8242D8B8(ctx, base);
	// 824233A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824233A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824233AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824233B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824233B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824233B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824233C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824233C0 size=132
    let mut pc: u32 = 0x824233C0;
    'dispatch: loop {
        match pc {
            0x824233C0 => {
    //   block [0x824233C0..0x82423444)
	// 824233C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824233C4: 48111CF9  bl 0x825350bc
	ctx.lr = 0x824233C8;
	sub_82535080(ctx, base);
	// 824233C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824233CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824233D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824233D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824233D8: 4800A4A1  bl 0x8242d878
	ctx.lr = 0x824233DC;
	sub_8242D878(ctx, base);
	// 824233DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824233E0: 409A0020  bne cr6, 0x82423400
	if !ctx.cr[6].eq {
	pc = 0x82423400; continue 'dispatch;
	}
	// 824233E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824233E8: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 824233EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824233F0: 386B19F4  addi r3, r11, 0x19f4
	ctx.r[3].s64 = ctx.r[11].s64 + 6644;
	// 824233F4: 48007B55  bl 0x8242af48
	ctx.lr = 0x824233F8;
	sub_8242AF48(ctx, base);
	// 824233F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824233FC: 48000038  b 0x82423434
	pc = 0x82423434; continue 'dispatch;
	// 82423400: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423404: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82423408: 409A0018  bne cr6, 0x82423420
	if !ctx.cr[6].eq {
	pc = 0x82423420; continue 'dispatch;
	}
	// 8242340C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423410: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82423414: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423418: 386B19E8  addi r3, r11, 0x19e8
	ctx.r[3].s64 = ctx.r[11].s64 + 6632;
	// 8242341C: 4BFFFFD8  b 0x824233f4
	pc = 0x824233F4; continue 'dispatch;
	// 82423420: 397E0005  addi r11, r30, 5
	ctx.r[11].s64 = ctx.r[30].s64 + 5;
	// 82423424: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82423428: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8242342C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82423430: 7FEBF82E  lwzx r31, r11, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82423434: 4800A485  bl 0x8242d8b8
	ctx.lr = 0x82423438;
	sub_8242D8B8(ctx, base);
	// 82423438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242343C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82423440: 48111CCC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423448 size=196
    let mut pc: u32 = 0x82423448;
    'dispatch: loop {
        match pc {
            0x82423448 => {
    //   block [0x82423448..0x8242350C)
	// 82423448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242344C: 48111C6D  bl 0x825350b8
	ctx.lr = 0x82423450;
	sub_82535080(ctx, base);
	// 82423450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423454: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82423458: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8242345C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82423460: 4800A419  bl 0x8242d878
	ctx.lr = 0x82423464;
	sub_8242D878(ctx, base);
	// 82423464: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82423468: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8242346C: 396B3680  addi r11, r11, 0x3680
	ctx.r[11].s64 = ctx.r[11].s64 + 13952;
	// 82423470: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82423474: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82423478: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242347C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82423480: 419A0018  beq cr6, 0x82423498
	if ctx.cr[6].eq {
	pc = 0x82423498; continue 'dispatch;
	}
	// 82423484: 39290048  addi r9, r9, 0x48
	ctx.r[9].s64 = ctx.r[9].s64 + 72;
	// 82423488: 390B6C04  addi r8, r11, 0x6c04
	ctx.r[8].s64 = ctx.r[11].s64 + 27652;
	// 8242348C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82423490: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82423494: 4198FFE4  blt cr6, 0x82423478
	if ctx.cr[6].lt {
	pc = 0x82423478; continue 'dispatch;
	}
	// 82423498: 2F0A0180  cmpwi cr6, r10, 0x180
	ctx.cr[6].compare_i32(ctx.r[10].s32, 384, &mut ctx.xer);
	// 8242349C: 409A000C  bne cr6, 0x824234a8
	if !ctx.cr[6].eq {
	pc = 0x824234A8; continue 'dispatch;
	}
	// 824234A0: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 824234A4: 48000058  b 0x824234fc
	pc = 0x824234FC; continue 'dispatch;
	// 824234A8: 1D0A0048  mulli r8, r10, 0x48
	ctx.r[8].s64 = ctx.r[10].s64 * 72;
	// 824234AC: 7FE85A14  add r31, r8, r11
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824234B0: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 824234B4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824234B8: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 824234BC: 394A3C94  addi r10, r10, 0x3c94
	ctx.r[10].s64 = ctx.r[10].s64 + 15508;
	// 824234C0: 39291840  addi r9, r9, 0x1840
	ctx.r[9].s64 = ctx.r[9].s64 + 6208;
	// 824234C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824234C8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 824234CC: 396B27C8  addi r11, r11, 0x27c8
	ctx.r[11].s64 = ctx.r[11].s64 + 10184;
	// 824234D0: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 824234D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824234D8: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 824234DC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824234E0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824234E4: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824234E8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824234EC: 93FF003C  stw r31, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 824234F0: 90FF0040  stw r7, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[7].u32 ) };
	// 824234F4: 90FF0044  stw r7, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 824234F8: 4BFFF2E1  bl 0x824227d8
	ctx.lr = 0x824234FC;
	sub_824227D8(ctx, base);
	// 824234FC: 4800A3BD  bl 0x8242d8b8
	ctx.lr = 0x82423500;
	sub_8242D8B8(ctx, base);
	// 82423500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82423508: 48111C00  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423510 size=20
    let mut pc: u32 = 0x82423510;
    'dispatch: loop {
        match pc {
            0x82423510 => {
    //   block [0x82423510..0x82423524)
	// 82423510: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423514: 386B1A00  addi r3, r11, 0x1a00
	ctx.r[3].s64 = ctx.r[11].s64 + 6656;
	// 82423518: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242351C: 906B8E38  stw r3, -0x71c8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-29128 as u32), ctx.r[3].u32 ) };
	// 82423520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423528 size=48
    let mut pc: u32 = 0x82423528;
    'dispatch: loop {
        match pc {
            0x82423528 => {
    //   block [0x82423528..0x82423558)
	// 82423528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242352C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423538: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242353C: 48111D35  bl 0x82535270
	ctx.lr = 0x82423540;
	sub_82535270(ctx, base);
	// 82423540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82423548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242354C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82423554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423558 size=48
    let mut pc: u32 = 0x82423558;
    'dispatch: loop {
        match pc {
            0x82423558 => {
    //   block [0x82423558..0x82423588)
	// 82423558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242355C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242356C: 48111EC5  bl 0x82535430
	ctx.lr = 0x82423570;
	sub_82535430(ctx, base);
	// 82423570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82423578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242357C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82423584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423588 size=48
    let mut pc: u32 = 0x82423588;
    'dispatch: loop {
        match pc {
            0x82423588 => {
    //   block [0x82423588..0x824235B8)
	// 82423588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242358C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242359C: 4811211D  bl 0x825356b8
	ctx.lr = 0x824235A0;
	sub_825356B8(ctx, base);
	// 824235A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824235A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824235A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824235AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824235B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824235B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824235B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824235B8 size=48
    let mut pc: u32 = 0x824235B8;
    'dispatch: loop {
        match pc {
            0x824235B8 => {
    //   block [0x824235B8..0x824235E8)
	// 824235B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824235BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824235C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824235C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824235C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824235CC: 48111FD5  bl 0x825355a0
	ctx.lr = 0x824235D0;
	sub_825355A0(ctx, base);
	// 824235D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824235D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824235D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824235DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824235E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824235E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824235E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824235E8 size=68
    let mut pc: u32 = 0x824235E8;
    'dispatch: loop {
        match pc {
            0x824235E8 => {
    //   block [0x824235E8..0x8242362C)
	// 824235E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824235EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824235F0: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 824235F4: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 824235F8: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 824235FC: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82423600: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82423604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423608: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8242360C: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82423610: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82423614: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82423618: 48114511  bl 0x82537b28
	ctx.lr = 0x8242361C;
	sub_82537B28(ctx, base);
	// 8242361C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82423620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423630 size=4
    let mut pc: u32 = 0x82423630;
    'dispatch: loop {
        match pc {
            0x82423630 => {
    //   block [0x82423630..0x82423634)
	// 82423630: 481144F8  b 0x82537b28
	sub_82537B28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423638 size=4
    let mut pc: u32 = 0x82423638;
    'dispatch: loop {
        match pc {
            0x82423638 => {
    //   block [0x82423638..0x8242363C)
	// 82423638: 4BFFD9F8  b 0x82421030
	sub_82421030(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423640 size=96
    let mut pc: u32 = 0x82423640;
    'dispatch: loop {
        match pc {
            0x82423640 => {
    //   block [0x82423640..0x824236A0)
	// 82423640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242364C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423654: 4800B8FD  bl 0x8242ef50
	ctx.lr = 0x82423658;
	sub_8242EF50(ctx, base);
	// 82423658: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 8242365C: 83FE8E3C  lwz r31, -0x71c4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-29124 as u32) ) } as u64;
	// 82423660: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82423664: 409A0018  bne cr6, 0x8242367c
	if !ctx.cr[6].eq {
	pc = 0x8242367C; continue 'dispatch;
	}
	// 82423668: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 8242366C: 38A00CB0  li r5, 0xcb0
	ctx.r[5].s64 = 3248;
	// 82423670: 386B29C0  addi r3, r11, 0x29c0
	ctx.r[3].s64 = ctx.r[11].s64 + 10688;
	// 82423674: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423678: 48111B59  bl 0x825351d0
	ctx.lr = 0x8242367C;
	sub_825351D0(ctx, base);
	// 8242367C: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82423680: 917E8E3C  stw r11, -0x71c4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-29124 as u32), ctx.r[11].u32 ) };
	// 82423684: 4800B8CD  bl 0x8242ef50
	ctx.lr = 0x82423688;
	sub_8242EF50(ctx, base);
	// 82423688: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242368C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423694: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82423698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242369C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824236A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824236A0 size=76
    let mut pc: u32 = 0x824236A0;
    'dispatch: loop {
        match pc {
            0x824236A0 => {
    //   block [0x824236A0..0x824236EC)
	// 824236A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824236A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824236A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824236AC: 4800B8A5  bl 0x8242ef50
	ctx.lr = 0x824236B0;
	sub_8242EF50(ctx, base);
	// 824236B0: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 824236B4: 816A8E3C  lwz r11, -0x71c4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29124 as u32) ) } as u64;
	// 824236B8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824236BC: 916A8E3C  stw r11, -0x71c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29124 as u32), ctx.r[11].u32 ) };
	// 824236C0: 40820018  bne 0x824236d8
	if !ctx.cr[0].eq {
	pc = 0x824236D8; continue 'dispatch;
	}
	// 824236C4: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 824236C8: 38A00CB0  li r5, 0xcb0
	ctx.r[5].s64 = 3248;
	// 824236CC: 386B29C0  addi r3, r11, 0x29c0
	ctx.r[3].s64 = ctx.r[11].s64 + 10688;
	// 824236D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824236D4: 48111AFD  bl 0x825351d0
	ctx.lr = 0x824236D8;
	sub_825351D0(ctx, base);
	// 824236D8: 4800B879  bl 0x8242ef50
	ctx.lr = 0x824236DC;
	sub_8242EF50(ctx, base);
	// 824236DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824236E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824236E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824236E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824236F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824236F0 size=72
    let mut pc: u32 = 0x824236F0;
    'dispatch: loop {
        match pc {
            0x824236F0 => {
    //   block [0x824236F0..0x82423738)
	// 824236F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824236F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824236F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824236FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82423704: 4800B84D  bl 0x8242ef50
	ctx.lr = 0x82423708;
	sub_8242EF50(ctx, base);
	// 82423708: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242370C: 419A0014  beq cr6, 0x82423720
	if ctx.cr[6].eq {
	pc = 0x82423720; continue 'dispatch;
	}
	// 82423710: 38A0032C  li r5, 0x32c
	ctx.r[5].s64 = 812;
	// 82423714: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242371C: 48111AB5  bl 0x825351d0
	ctx.lr = 0x82423720;
	sub_825351D0(ctx, base);
	// 82423720: 4800B831  bl 0x8242ef50
	ctx.lr = 0x82423724;
	sub_8242EF50(ctx, base);
	// 82423724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82423728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242372C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82423734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423738 size=12
    let mut pc: u32 = 0x82423738;
    'dispatch: loop {
        match pc {
            0x82423738 => {
    //   block [0x82423738..0x82423744)
	// 82423738: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242373C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82423740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423748 size=72
    let mut pc: u32 = 0x82423748;
    'dispatch: loop {
        match pc {
            0x82423748 => {
    //   block [0x82423748..0x82423790)
	// 82423748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242375C: 4800B7F5  bl 0x8242ef50
	ctx.lr = 0x82423760;
	sub_8242EF50(ctx, base);
	// 82423760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82423764: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82423768: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8242376C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82423770: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82423774: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82423778: 4800B7D9  bl 0x8242ef50
	ctx.lr = 0x8242377C;
	sub_8242EF50(ctx, base);
	// 8242377C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82423780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242378C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423790 size=12
    let mut pc: u32 = 0x82423790;
    'dispatch: loop {
        match pc {
            0x82423790 => {
    //   block [0x82423790..0x8242379C)
	// 82423790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82423794: 99630001  stb r11, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82423798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237A0 size=8
    let mut pc: u32 = 0x824237A0;
    'dispatch: loop {
        match pc {
            0x824237A0 => {
    //   block [0x824237A0..0x824237A8)
	// 824237A0: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 824237A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237A8 size=8
    let mut pc: u32 = 0x824237A8;
    'dispatch: loop {
        match pc {
            0x824237A8 => {
    //   block [0x824237A8..0x824237B0)
	// 824237A8: 98830003  stb r4, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[4].u8 ) };
	// 824237AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237B0 size=8
    let mut pc: u32 = 0x824237B0;
    'dispatch: loop {
        match pc {
            0x824237B0 => {
    //   block [0x824237B0..0x824237B8)
	// 824237B0: 98830004  stb r4, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u8 ) };
	// 824237B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237B8 size=12
    let mut pc: u32 = 0x824237B8;
    'dispatch: loop {
        match pc {
            0x824237B8 => {
    //   block [0x824237B8..0x824237C4)
	// 824237B8: 896300AC  lbz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 824237BC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 824237C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237C8 size=32
    let mut pc: u32 = 0x824237C8;
    'dispatch: loop {
        match pc {
            0x824237C8 => {
    //   block [0x824237C8..0x824237E8)
	// 824237C8: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 824237CC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824237D0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824237D4: 41980014  blt cr6, 0x824237e8
	if ctx.cr[6].lt {
		sub_824237E8(ctx, base);
		return;
	}
	// 824237D8: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824237DC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824237E0: 806B00B4  lwz r3, 0xb4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 824237E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237E8 size=8
    let mut pc: u32 = 0x824237E8;
    'dispatch: loop {
        match pc {
            0x824237E8 => {
    //   block [0x824237E8..0x824237F0)
	// 824237E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824237EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824237F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824237F0 size=20
    let mut pc: u32 = 0x824237F0;
    'dispatch: loop {
        match pc {
            0x824237F0 => {
    //   block [0x824237F0..0x82423804)
	// 824237F0: 3964000B  addi r11, r4, 0xb
	ctx.r[11].s64 = ctx.r[4].s64 + 11;
	// 824237F4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824237F8: 7D6B18AE  lbzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824237FC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82423800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423808 size=8
    let mut pc: u32 = 0x82423808;
    'dispatch: loop {
        match pc {
            0x82423808 => {
    //   block [0x82423808..0x82423810)
	// 82423808: 806302B0  lwz r3, 0x2b0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(688 as u32) ) } as u64;
	// 8242380C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423810 size=32
    let mut pc: u32 = 0x82423810;
    'dispatch: loop {
        match pc {
            0x82423810 => {
    //   block [0x82423810..0x82423830)
	// 82423810: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82423814: 419A0014  beq cr6, 0x82423828
	if ctx.cr[6].eq {
	pc = 0x82423828; continue 'dispatch;
	}
	// 82423818: 816302B8  lwz r11, 0x2b8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(696 as u32) ) } as u64;
	// 8242381C: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82423820: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82423824: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82423828: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8242382C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423830 size=24
    let mut pc: u32 = 0x82423830;
    'dispatch: loop {
        match pc {
            0x82423830 => {
    //   block [0x82423830..0x82423848)
	// 82423830: 816302B8  lwz r11, 0x2b8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(696 as u32) ) } as u64;
	// 82423834: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82423838: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8242383C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423840: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82423844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423848 size=8
    let mut pc: u32 = 0x82423848;
    'dispatch: loop {
        match pc {
            0x82423848 => {
    //   block [0x82423848..0x82423850)
	// 82423848: 80630320  lwz r3, 0x320(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(800 as u32) ) } as u64;
	// 8242384C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82423850 size=12
    let mut pc: u32 = 0x82423850;
    'dispatch: loop {
        match pc {
            0x82423850 => {
    //   block [0x82423850..0x8242385C)
	// 82423850: 90830324  stw r4, 0x324(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(804 as u32), ctx.r[4].u32 ) };
	// 82423854: 90A30328  stw r5, 0x328(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(808 as u32), ctx.r[5].u32 ) };
	// 82423858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423860 size=272
    let mut pc: u32 = 0x82423860;
    'dispatch: loop {
        match pc {
            0x82423860 => {
    //   block [0x82423860..0x82423970)
	// 82423860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242386C: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 82423870: 4098000C  bge cr6, 0x8242387c
	if !ctx.cr[6].lt {
	pc = 0x8242387C; continue 'dispatch;
	}
	// 82423874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82423878: 480000E8  b 0x82423960
	pc = 0x82423960; continue 'dispatch;
	// 8242387C: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423880: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82423884: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82423888: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8242388C: 89030002  lbz r8, 2(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82423890: 88E30003  lbz r7, 3(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82423894: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82423898: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242389C: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 824238A0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824238A4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 824238A8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824238AC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824238B0: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824238B4: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 824238B8: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824238BC: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824238C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824238C4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 824238C8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824238CC: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 824238D0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824238D4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 824238D8: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824238DC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824238E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824238E4: 99450008  stb r10, 8(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 824238E8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824238EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824238F0: 99450009  stb r10, 9(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 824238F4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824238F8: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824238FC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82423900: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82423904: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82423908: B145000A  sth r10, 0xa(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8242390C: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82423910: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423914: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82423918: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8242391C: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82423920: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82423924: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82423928: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242392C: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82423930: 552A402E  slwi r10, r9, 8
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82423934: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82423938: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8242393C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82423940: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82423944: 419A0018  beq cr6, 0x8242395c
	if ctx.cr[6].eq {
	pc = 0x8242395C; continue 'dispatch;
	}
	// 82423948: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242394C: 386B1A5C  addi r3, r11, 0x1a5c
	ctx.r[3].s64 = ctx.r[11].s64 + 6748;
	// 82423950: 48000D89  bl 0x824246d8
	ctx.lr = 0x82423954;
	sub_824246D8(ctx, base);
	// 82423954: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82423958: 48000008  b 0x82423960
	pc = 0x82423960; continue 'dispatch;
	// 8242395C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82423960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82423964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423970 size=1140
    let mut pc: u32 = 0x82423970;
    'dispatch: loop {
        match pc {
            0x82423970 => {
    //   block [0x82423970..0x82423DE4)
	// 82423970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423974: 4811172D  bl 0x825350a0
	ctx.lr = 0x82423978;
	sub_82535080(ctx, base);
	// 82423978: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242397C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82423980: 3AC0FFFD  li r22, -3
	ctx.r[22].s64 = -3;
	// 82423984: 3B5B0020  addi r26, r27, 0x20
	ctx.r[26].s64 = ctx.r[27].s64 + 32;
	// 82423988: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8242398C: 897B00AC  lbz r11, 0xac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(172 as u32) ) } as u64;
	// 82423990: 83FB001C  lwz r31, 0x1c(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423994: 7D780775  extsb. r24, r11
	ctx.r[24].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82423998: 408102AC  ble 0x82423c44
	if !ctx.cr[0].gt {
	pc = 0x82423C44; continue 'dispatch;
	}
	// 8242399C: 3D604149  lis r11, 0x4149
	ctx.r[11].s64 = 1095303168;
	// 824239A0: 61775845  ori r23, r11, 0x5845
	ctx.r[23].u64 = ctx.r[11].u64 | 22597;
	// 824239A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824239A8: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 824239AC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824239B0: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 824239B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824239B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824239BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824239C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824239C4: 4E800421  bctrl
	ctx.lr = 0x824239C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824239C8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824239CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824239D0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824239D4: 409900D8  ble cr6, 0x82423aac
	if !ctx.cr[6].gt {
	pc = 0x82423AAC; continue 'dispatch;
	}
	// 824239D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824239DC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 824239E0: 892BFFFE  lbz r9, -2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 824239E4: 890BFFFF  lbz r8, -1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 824239E8: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 824239EC: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824239F0: 88CB0001  lbz r6, 1(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824239F4: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 824239F8: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824239FC: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82423A00: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82423A04: 7D293378  or r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[6].u64;
	// 82423A08: 7D374851  subf. r9, r23, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[23].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82423A0C: 41820024  beq 0x82423a30
	if ctx.cr[0].eq {
	pc = 0x82423A30; continue 'dispatch;
	}
	// 82423A10: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82423A14: 419A000C  beq cr6, 0x82423a20
	if ctx.cr[6].eq {
	pc = 0x82423A20; continue 'dispatch;
	}
	// 82423A18: 2B09000B  cmplwi cr6, r9, 0xb
	ctx.cr[6].compare_u32(ctx.r[9].u32, 11 as u32, &mut ctx.xer);
	// 82423A1C: 419A0014  beq cr6, 0x82423a30
	if ctx.cr[6].eq {
	pc = 0x82423A30; continue 'dispatch;
	}
	// 82423A20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82423A24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82423A28: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82423A2C: 4198FFB4  blt cr6, 0x824239e0
	if ctx.cr[6].lt {
	pc = 0x824239E0; continue 'dispatch;
	}
	// 82423A30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82423A34: 40990078  ble cr6, 0x82423aac
	if !ctx.cr[6].gt {
	pc = 0x82423AAC; continue 'dispatch;
	}
	// 82423A38: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 82423A3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423A40: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82423A44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82423A48: 480054C1  bl 0x82428f08
	ctx.lr = 0x82423A4C;
	sub_82428F08(ctx, base);
	// 82423A4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423A50: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82423A54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423A5C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423A64: 4E800421  bctrl
	ctx.lr = 0x82423A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423A68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423A6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423A70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423A78: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423A7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423A80: 4E800421  bctrl
	ctx.lr = 0x82423A84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423A84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423A88: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 82423A8C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82423A90: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 82423A94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423A98: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82423A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423AA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423AA4: 4E800421  bctrl
	ctx.lr = 0x82423AA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423AA8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82423AAC: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 82423AB0: 419801A0  blt cr6, 0x82423c50
	if ctx.cr[6].lt {
	pc = 0x82423C50; continue 'dispatch;
	}
	// 82423AB4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82423AB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82423ABC: 4BFFFDA5  bl 0x82423860
	ctx.lr = 0x82423AC0;
	sub_82423860(ctx, base);
	// 82423AC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82423AC4: 4182018C  beq 0x82423c50
	if ctx.cr[0].eq {
	pc = 0x82423C50; continue 'dispatch;
	}
	// 82423AC8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82423ACC: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82423AD0: 419A01A0  beq cr6, 0x82423c70
	if ctx.cr[6].eq {
	pc = 0x82423C70; continue 'dispatch;
	}
	// 82423AD4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82423AD8: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82423ADC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82423AE0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82423AE4: 4198016C  blt cr6, 0x82423c50
	if ctx.cr[6].lt {
	pc = 0x82423C50; continue 'dispatch;
	}
	// 82423AE8: 895B0002  lbz r10, 2(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(2 as u32) ) } as u64;
	// 82423AEC: 89610078  lbz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82423AF0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82423AF4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82423AF8: 40980268  bge cr6, 0x82423d60
	if !ctx.cr[6].lt {
	pc = 0x82423D60; continue 'dispatch;
	}
	// 82423AFC: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82423B00: ABA1007A  lha r29, 0x7a(r1)
	ctx.r[29].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(122 as u32) ) } as i16) as i64;
	// 82423B04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423B08: 7C7CD02E  lwzx r3, r28, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82423B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423B10: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82423B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423B18: 4E800421  bctrl
	ctx.lr = 0x82423B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423B1C: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82423B20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423B24: 41980130  blt cr6, 0x82423c54
	if ctx.cr[6].lt {
	pc = 0x82423C54; continue 'dispatch;
	}
	// 82423B28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82423B2C: 83C1007C  lwz r30, 0x7c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82423B30: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82423B34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82423B38: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
	// 82423B3C: 480053CD  bl 0x82428f08
	ctx.lr = 0x82423B40;
	sub_82428F08(ctx, base);
	// 82423B40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423B44: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82423B48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423B50: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423B58: 4E800421  bctrl
	ctx.lr = 0x82423B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423B5C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82423B60: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423B64: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82423B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82423B6C: 4800539D  bl 0x82428f08
	ctx.lr = 0x82423B70;
	sub_82428F08(ctx, base);
	// 82423B70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423B74: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423B78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423B80: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423B88: 4E800421  bctrl
	ctx.lr = 0x82423B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423B8C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82423B90: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82423B94: 419801FC  blt cr6, 0x82423d90
	if ctx.cr[6].lt {
	pc = 0x82423D90; continue 'dispatch;
	}
	// 82423B98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82423B9C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82423BA0: 40990068  ble cr6, 0x82423c08
	if !ctx.cr[6].gt {
	pc = 0x82423C08; continue 'dispatch;
	}
	// 82423BA4: 7C7CD02E  lwzx r3, r28, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82423BA8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82423BAC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82423BB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423BB4: 7CBE5850  subf r5, r30, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82423BB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423BBC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82423BC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423BC4: 4E800421  bctrl
	ctx.lr = 0x82423BC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423BC8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82423BCC: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82423BD0: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82423BD4: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82423BD8: 48110F79  bl 0x82534b50
	ctx.lr = 0x82423BDC;
	sub_82534B50(ctx, base);
	// 82423BDC: 7C7CD02E  lwzx r3, r28, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82423BE0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82423BE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423BE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423BEC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423BF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423BF4: 4E800421  bctrl
	ctx.lr = 0x82423BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423BF8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82423BFC: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82423C00: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82423C04: 4198FFA0  blt cr6, 0x82423ba4
	if ctx.cr[6].lt {
	pc = 0x82423BA4; continue 'dispatch;
	}
	// 82423C08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423C0C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82423C10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423C18: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423C1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423C20: 4E800421  bctrl
	ctx.lr = 0x82423C24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423C24: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82423C28: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82423C2C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82423C30: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82423C34: 7F19C000  cmpw cr6, r25, r24
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82423C38: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82423C3C: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82423C40: 4198FD64  blt cr6, 0x824239a4
	if ctx.cr[6].lt {
	pc = 0x824239A4; continue 'dispatch;
	}
	// 82423C44: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82423C48: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82423C4C: 481114A4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 82423C50: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423C54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423C58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423C60: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423C64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423C68: 4E800421  bctrl
	ctx.lr = 0x82423C6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423C6C: 4BFFFFD8  b 0x82423c44
	pc = 0x82423C44; continue 'dispatch;
	// 82423C70: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82423C74: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82423C78: 2F090008  cmpwi cr6, r9, 8
	ctx.cr[6].compare_i32(ctx.r[9].s32, 8, &mut ctx.xer);
	// 82423C7C: 4099005C  ble cr6, 0x82423cd8
	if !ctx.cr[6].gt {
	pc = 0x82423CD8; continue 'dispatch;
	}
	// 82423C80: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82423C84: 396B000A  addi r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 + 10;
	// 82423C88: 894BFFFE  lbz r10, -2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82423C8C: 890BFFFF  lbz r8, -1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82423C90: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82423C94: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423C98: 88CB0001  lbz r6, 1(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82423C9C: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82423CA0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82423CA4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82423CA8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82423CAC: 7D4A3378  or r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 82423CB0: 7D575051  subf. r10, r23, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[23].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82423CB4: 41820024  beq 0x82423cd8
	if ctx.cr[0].eq {
	pc = 0x82423CD8; continue 'dispatch;
	}
	// 82423CB8: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82423CBC: 419A000C  beq cr6, 0x82423cc8
	if ctx.cr[6].eq {
	pc = 0x82423CC8; continue 'dispatch;
	}
	// 82423CC0: 2B0A000B  cmplwi cr6, r10, 0xb
	ctx.cr[6].compare_u32(ctx.r[10].u32, 11 as u32, &mut ctx.xer);
	// 82423CC4: 419A0014  beq cr6, 0x82423cd8
	if ctx.cr[6].eq {
	pc = 0x82423CD8; continue 'dispatch;
	}
	// 82423CC8: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82423CCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82423CD0: 7F044800  cmpw cr6, r4, r9
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82423CD4: 4198FFB4  blt cr6, 0x82423c88
	if ctx.cr[6].lt {
	pc = 0x82423C88; continue 'dispatch;
	}
	// 82423CD8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82423CDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423CE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82423CE4: 48005225  bl 0x82428f08
	ctx.lr = 0x82423CE8;
	sub_82428F08(ctx, base);
	// 82423CE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423CEC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423CF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423CF8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423CFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423D00: 4E800421  bctrl
	ctx.lr = 0x82423D04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423D04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423D08: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82423D0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423D14: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423D1C: 4E800421  bctrl
	ctx.lr = 0x82423D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423D20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423D24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423D2C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82423D30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423D34: 4E800421  bctrl
	ctx.lr = 0x82423D38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423D38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82423D3C: 4181000C  bgt 0x82423d48
	if ctx.cr[0].gt {
	pc = 0x82423D48; continue 'dispatch;
	}
	// 82423D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82423D44: 997B0004  stb r11, 4(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82423D48: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82423D4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82423D50: 409AFEF4  bne cr6, 0x82423c44
	if !ctx.cr[6].eq {
	pc = 0x82423C44; continue 'dispatch;
	}
	// 82423D54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82423D58: 997B0001  stb r11, 1(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82423D5C: 4BFFFEE8  b 0x82423c44
	pc = 0x82423C44; continue 'dispatch;
	// 82423D60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423D64: 386B1AAC  addi r3, r11, 0x1aac
	ctx.r[3].s64 = ctx.r[11].s64 + 6828;
	// 82423D68: 48000971  bl 0x824246d8
	ctx.lr = 0x82423D6C;
	sub_824246D8(ctx, base);
	// 82423D6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423D70: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423D74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423D7C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423D80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423D84: 4E800421  bctrl
	ctx.lr = 0x82423D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423D88: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82423D8C: 4BFFFFCC  b 0x82423d58
	pc = 0x82423D58; continue 'dispatch;
	// 82423D90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82423D94: 386B1A88  addi r3, r11, 0x1a88
	ctx.r[3].s64 = ctx.r[11].s64 + 6792;
	// 82423D98: 48000941  bl 0x824246d8
	ctx.lr = 0x82423D9C;
	sub_824246D8(ctx, base);
	// 82423D9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423DA0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82423DA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423DAC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423DB4: 4E800421  bctrl
	ctx.lr = 0x82423DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423DB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423DBC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423DC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423DC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423DD0: 4E800421  bctrl
	ctx.lr = 0x82423DD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423DD4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82423DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82423DDC: 997B0001  stb r11, 1(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82423DE0: 4BFFFE68  b 0x82423c48
	pc = 0x82423C48; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423DE8 size=256
    let mut pc: u32 = 0x82423DE8;
    'dispatch: loop {
        match pc {
            0x82423DE8 => {
    //   block [0x82423DE8..0x82423EE8)
	// 82423DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82423DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423DFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82423E00: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 82423E04: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82423E08: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 82423E0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423E10: 83FE001C  lwz r31, 0x1c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423E18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423E1C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82423E20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423E24: 4E800421  bctrl
	ctx.lr = 0x82423E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423E28: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423E2C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82423E30: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82423E34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423E38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423E3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82423E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423E44: 4E800421  bctrl
	ctx.lr = 0x82423E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423E48: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82423E4C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82423E50: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82423E54: 48110CFD  bl 0x82534b50
	ctx.lr = 0x82423E58;
	sub_82534B50(ctx, base);
	// 82423E58: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423E5C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82423E60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423E64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423E68: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423E70: 4E800421  bctrl
	ctx.lr = 0x82423E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423E74: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82423E78: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423E7C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82423E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82423E84: 48005085  bl 0x82428f08
	ctx.lr = 0x82423E88;
	sub_82428F08(ctx, base);
	// 82423E88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423E8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82423E90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423E98: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82423E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423EA0: 4E800421  bctrl
	ctx.lr = 0x82423EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423EA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423EA8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82423EAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82423EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423EB4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82423EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82423EBC: 4E800421  bctrl
	ctx.lr = 0x82423EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82423EC0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82423EC4: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82423EC8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82423ECC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82423ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82423ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82423ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82423EDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82423EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82423EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82423EE8 size=224
    let mut pc: u32 = 0x82423EE8;
    'dispatch: loop {
        match pc {
            0x82423EE8 => {
    //   block [0x82423EE8..0x82423FC8)
	// 82423EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423EEC: 481111C9  bl 0x825350b4
	ctx.lr = 0x82423EF0;
	sub_82535080(ctx, base);
	// 82423EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423EF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82423EF8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82423EFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82423F00: 4800B051  bl 0x8242ef50
	ctx.lr = 0x82423F04;
	sub_8242EF50(ctx, base);
	// 82423F04: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82423F08: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82423F0C: 396B29C0  addi r11, r11, 0x29c0
	ctx.r[11].s64 = ctx.r[11].s64 + 10688;
	// 82423F10: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82423F14: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82423F18: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423F1C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82423F20: 419A0018  beq cr6, 0x82423f38
	if ctx.cr[6].eq {
	pc = 0x82423F38; continue 'dispatch;
	}
	// 82423F24: 3929032C  addi r9, r9, 0x32c
	ctx.r[9].s64 = ctx.r[9].s64 + 812;
	// 82423F28: 390B0CB0  addi r8, r11, 0xcb0
	ctx.r[8].s64 = ctx.r[11].s64 + 3248;
	// 82423F2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82423F30: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82423F34: 4198FFE4  blt cr6, 0x82423f18
	if ctx.cr[6].lt {
	pc = 0x82423F18; continue 'dispatch;
	}
	// 82423F38: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82423F3C: 409A000C  bne cr6, 0x82423f48
	if !ctx.cr[6].eq {
	pc = 0x82423F48; continue 'dispatch;
	}
	// 82423F40: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82423F44: 48000074  b 0x82423fb8
	pc = 0x82423FB8; continue 'dispatch;
	// 82423F48: 1D4A032C  mulli r10, r10, 0x32c
	ctx.r[10].s64 = ctx.r[10].s64 * 812;
	// 82423F4C: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82423F50: 38A0032C  li r5, 0x32c
	ctx.r[5].s64 = 812;
	// 82423F54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82423F58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423F5C: 48111275  bl 0x825351d0
	ctx.lr = 0x82423F60;
	sub_825351D0(ctx, base);
	// 82423F60: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82423F64: 9B7F0001  stb r27, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[27].u8 ) };
	// 82423F68: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82423F6C: 40990028  ble cr6, 0x82423f94
	if !ctx.cr[6].gt {
	pc = 0x82423F94; continue 'dispatch;
	}
	// 82423F70: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82423F74: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 82423F78: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82423F7C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423F80: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82423F84: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82423F88: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82423F8C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82423F90: 4082FFEC  bne 0x82423f7c
	if !ctx.cr[0].eq {
	pc = 0x82423F7C; continue 'dispatch;
	}
	// 82423F94: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82423F98: 9BDF0002  stb r30, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82423F9C: 393F02BC  addi r9, r31, 0x2bc
	ctx.r[9].s64 = ctx.r[31].s64 + 700;
	// 82423FA0: 9B7F0003  stb r27, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[27].u8 ) };
	// 82423FA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82423FA8: 9B7F0004  stb r27, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u8 ) };
	// 82423FAC: 915F02B4  stw r10, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 82423FB0: 913F02B8  stw r9, 0x2b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(696 as u32), ctx.r[9].u32 ) };
	// 82423FB4: 991F0000  stb r8, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82423FB8: 4800AF99  bl 0x8242ef50
	ctx.lr = 0x82423FBC;
	sub_8242EF50(ctx, base);
	// 82423FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82423FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82423FC4: 48111140  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82423FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82423FC8 size=1036
    let mut pc: u32 = 0x82423FC8;
    'dispatch: loop {
        match pc {
            0x82423FC8 => {
    //   block [0x82423FC8..0x824243D4)
	// 82423FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82423FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82423FD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82423FD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82423FD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82423FDC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82423FE0: 2F041800  cmpwi cr6, r4, 0x1800
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6144, &mut ctx.xer);
	// 82423FE4: 4098000C  bge cr6, 0x82423ff0
	if !ctx.cr[6].lt {
	pc = 0x82423FF0; continue 'dispatch;
	}
	// 82423FE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82423FEC: 480003D0  b 0x824243bc
	pc = 0x824243BC; continue 'dispatch;
	// 82423FF0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82423FF4: 3D404149  lis r10, 0x4149
	ctx.r[10].s64 = 1095303168;
	// 82423FF8: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82423FFC: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82424000: 89030002  lbz r8, 2(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82424004: 88E30003  lbz r7, 3(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82424008: 614A5846  ori r10, r10, 0x5846
	ctx.r[10].u64 = ctx.r[10].u64 | 22598;
	// 8242400C: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82424010: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82424014: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82424018: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242401C: 7D6B3B78  or r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 82424020: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82424024: 419A0048  beq cr6, 0x8242406c
	if ctx.cr[6].eq {
	pc = 0x8242406C; continue 'dispatch;
	}
	// 82424028: 38A00280  li r5, 0x280
	ctx.r[5].s64 = 640;
	// 8242402C: 83DF0218  lwz r30, 0x218(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 82424030: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82424034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82424038: 48111199  bl 0x825351d0
	ctx.lr = 0x8242403C;
	sub_825351D0(ctx, base);
	// 8242403C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82424040: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82424044: 93DF0218  stw r30, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[30].u32 ) };
	// 82424048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242404C: 917F0210  stw r11, 0x210(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(528 as u32), ctx.r[11].u32 ) };
	// 82424050: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 82424054: B0FF0260  sth r7, 0x260(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[7].u16 ) };
	// 82424058: B17F0262  sth r11, 0x262(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(610 as u32), ctx.r[11].u16 ) };
	// 8242405C: B0FF0264  sth r7, 0x264(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[7].u16 ) };
	// 82424060: B0FF0266  sth r7, 0x266(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(614 as u32), ctx.r[7].u16 ) };
	// 82424064: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82424068: 48000354  b 0x824243bc
	pc = 0x824243BC; continue 'dispatch;
	// 8242406C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82424070: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82424074: 394A1A54  addi r10, r10, 0x1a54
	ctx.r[10].s64 = ctx.r[10].s64 + 6740;
	// 82424078: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242407C: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82424080: 5527403E  rotlwi r7, r9, 8
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82424084: 88CB0002  lbz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82424088: 88AB0003  lbz r5, 3(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242408C: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82424090: 7CEB4378  or r11, r7, r8
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 82424094: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82424098: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 8242409C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824240A0: 7D662B78  or r6, r11, r5
	ctx.r[6].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 824240A4: 7D661A14  add r11, r6, r3
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 824240A8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 824240AC: 390B0006  addi r8, r11, 6
	ctx.r[8].s64 = ctx.r[11].s64 + 6;
	// 824240B0: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824240B4: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824240B8: 7CE53851  subf. r7, r5, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824240BC: 40820014  bne 0x824240d0
	if !ctx.cr[0].eq {
	pc = 0x824240D0; continue 'dispatch;
	}
	// 824240C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824240C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824240C8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824240CC: 409AFFE4  bne cr6, 0x824240b0
	if !ctx.cr[6].eq {
	pc = 0x824240B0; continue 'dispatch;
	}
	// 824240D0: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824240D4: 4082FF14  bne 0x82423fe8
	if !ctx.cr[0].eq {
	pc = 0x82423FE8; continue 'dispatch;
	}
	// 824240D8: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824240DC: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 824240E0: 813F0214  lwz r9, 0x214(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(532 as u32) ) } as u64;
	// 824240E4: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 824240E8: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824240EC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824240F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824240F4: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 824240F8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824240FC: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82424100: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82424104: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82424108: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8242410C: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82424110: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82424114: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82424118: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8242411C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82424120: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82424124: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424128: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242412C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82424130: 890B0002  lbz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82424134: 88EB0003  lbz r7, 3(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82424138: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242413C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82424140: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82424144: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82424148: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8242414C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82424150: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82424154: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82424158: 40980014  bge cr6, 0x8242416c
	if !ctx.cr[6].lt {
	pc = 0x8242416C; continue 'dispatch;
	}
	// 8242415C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82424160: 393F021C  addi r9, r31, 0x21c
	ctx.r[9].s64 = ctx.r[31].s64 + 540;
	// 82424164: 915F0214  stw r10, 0x214(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(532 as u32), ctx.r[10].u32 ) };
	// 82424168: 913F0218  stw r9, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[9].u32 ) };
	// 8242416C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424170: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82424174: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82424178: 811F0214  lwz r8, 0x214(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(532 as u32) ) } as u64;
	// 8242417C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82424180: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82424184: 915F0210  stw r10, 0x210(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(528 as u32), ctx.r[10].u32 ) };
	// 82424188: 40990018  ble cr6, 0x824241a0
	if !ctx.cr[6].gt {
	pc = 0x824241A0; continue 'dispatch;
	}
	// 8242418C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424190: 386B1AD0  addi r3, r11, 0x1ad0
	ctx.r[3].s64 = ctx.r[11].s64 + 6864;
	// 82424194: 48000545  bl 0x824246d8
	ctx.lr = 0x82424198;
	sub_824246D8(ctx, base);
	// 82424198: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8242419C: 48000220  b 0x824243bc
	pc = 0x824243BC; continue 'dispatch;
	// 824241A0: 815F0210  lwz r10, 0x210(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(528 as u32) ) } as u64;
	// 824241A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824241A8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824241AC: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 824241B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824241B4: 409900F8  ble cr6, 0x824242ac
	if !ctx.cr[6].gt {
	pc = 0x824242AC; continue 'dispatch;
	}
	// 824241B8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 824241BC: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824241C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824241C4: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824241C8: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 824241CC: 888B0002  lbz r4, 2(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824241D0: 886B0003  lbz r3, 3(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824241D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824241D8: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 824241DC: 80BF0218  lwz r5, 0x218(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 824241E0: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824241E4: 7D082378  or r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 824241E8: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824241EC: 7D081B78  or r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[3].u64;
	// 824241F0: 7D05512E  stwx r8, r5, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 824241F4: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824241F8: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824241FC: 54A5403E  rotlwi r5, r5, 8
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 82424200: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82424204: 8BCB0003  lbz r30, 3(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82424208: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242420C: 7CA52378  or r5, r5, r4
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[4].u64;
	// 82424210: 811F0218  lwz r8, 0x218(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 82424214: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82424218: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8242421C: 7CA51B78  or r5, r5, r3
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 82424220: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82424224: 7CA5F378  or r5, r5, r30
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[30].u64;
	// 82424228: 90A80004  stw r5, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8242422C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424230: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82424234: 54A5403E  rotlwi r5, r5, 8
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 82424238: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242423C: 8BCB0003  lbz r30, 3(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82424240: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82424244: 7CA52378  or r5, r5, r4
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[4].u64;
	// 82424248: 811F0218  lwz r8, 0x218(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 8242424C: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82424250: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82424254: 7CA51B78  or r5, r5, r3
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 82424258: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8242425C: 7CA5F378  or r5, r5, r30
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[30].u64;
	// 82424260: 90A80008  stw r5, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82424264: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424268: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242426C: 54A5403E  rotlwi r5, r5, 8
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 82424270: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82424274: 8BCB0003  lbz r30, 3(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82424278: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8242427C: 7CA52378  or r5, r5, r4
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[4].u64;
	// 82424280: 811F0218  lwz r8, 0x218(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 82424284: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82424288: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8242428C: 7CA51B78  or r5, r5, r3
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 82424290: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82424294: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82424298: 7CA5F378  or r5, r5, r30
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[30].u64;
	// 8242429C: 90A8000C  stw r5, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 824242A0: 811F0210  lwz r8, 0x210(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(528 as u32) ) } as u64;
	// 824242A4: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824242A8: 4198FF14  blt cr6, 0x824241bc
	if ctx.cr[6].lt {
	pc = 0x824241BC; continue 'dispatch;
	}
	// 824242AC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824242B0: 394B0008  addi r10, r11, 8
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	// 824242B4: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 824242B8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 824242BC: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 824242C0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824242C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824242C8: 4099007C  ble cr6, 0x82424344
	if !ctx.cr[6].gt {
	pc = 0x82424344; continue 'dispatch;
	}
	// 824242CC: 397F0262  addi r11, r31, 0x262
	ctx.r[11].s64 = ctx.r[31].s64 + 610;
	// 824242D0: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824242D4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824242D8: 88AA0001  lbz r5, 1(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 824242DC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 824242E0: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 824242E4: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 824242E8: B10BFFFE  sth r8, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[8].u16 ) };
	// 824242EC: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824242F0: 88AA0001  lbz r5, 1(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 824242F4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 824242F8: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 824242FC: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 82424300: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82424304: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424308: 88AA0001  lbz r5, 1(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242430C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82424310: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82424314: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 82424318: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 8242431C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424320: 88AA0001  lbz r5, 1(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82424324: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82424328: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 8242432C: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 82424330: B10B0004  sth r8, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82424334: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82424338: 811F025C  lwz r8, 0x25c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(604 as u32) ) } as u64;
	// 8242433C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82424340: 4198FF90  blt cr6, 0x824242d0
	if ctx.cr[6].lt {
	pc = 0x824242D0; continue 'dispatch;
	}
	// 82424344: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424348: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 8242434C: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82424350: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82424354: 991F000C  stb r8, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82424358: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242435C: 4081005C  ble 0x824243b8
	if !ctx.cr[0].gt {
	pc = 0x824243B8; continue 'dispatch;
	}
	// 82424360: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82424364: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424368: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8242436C: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82424370: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82424374: 88AB0002  lbz r5, 2(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82424378: 888B0003  lbz r4, 3(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 8242437C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82424380: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82424384: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82424388: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 8242438C: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82424390: 7D082378  or r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 82424394: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82424398: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242439C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824243A0: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 824243A4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824243A8: 891F000C  lbz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824243AC: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 824243B0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824243B4: 4198FFB0  blt cr6, 0x82424364
	if ctx.cr[6].lt {
	pc = 0x82424364; continue 'dispatch;
	}
	// 824243B8: 38660008  addi r3, r6, 8
	ctx.r[3].s64 = ctx.r[6].s64 + 8;
	// 824243BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824243C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824243C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824243C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824243CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824243D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824243D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824243D8 size=460
    let mut pc: u32 = 0x824243D8;
    'dispatch: loop {
        match pc {
            0x824243D8 => {
    //   block [0x824243D8..0x824245A4)
	// 824243D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824243DC: 48110CD9  bl 0x825350b4
	ctx.lr = 0x824243E0;
	sub_82535080(ctx, base);
	// 824243E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824243E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824243E8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824243EC: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 824243F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824243F4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 824243F8: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824243FC: 3BBF00A0  addi r29, r31, 0xa0
	ctx.r[29].s64 = ctx.r[31].s64 + 160;
	// 82424400: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82424404: 48110DCD  bl 0x825351d0
	ctx.lr = 0x82424408;
	sub_825351D0(ctx, base);
	// 82424408: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242440C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82424410: 38A01800  li r5, 0x1800
	ctx.r[5].s64 = 6144;
	// 82424414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82424418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242441C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82424420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424424: 4E800421  bctrl
	ctx.lr = 0x82424428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82424428: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242442C: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82424430: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82424434: 4BFFFB95  bl 0x82423fc8
	ctx.lr = 0x82424438;
	sub_82423FC8(ctx, base);
	// 82424438: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8242443C: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82424440: 409A0024  bne cr6, 0x82424464
	if !ctx.cr[6].eq {
	pc = 0x82424464; continue 'dispatch;
	}
	// 82424444: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242444C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82424450: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82424454: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242445C: 4E800421  bctrl
	ctx.lr = 0x82424460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82424460: 480000F0  b 0x82424550
	pc = 0x82424550; continue 'dispatch;
	// 82424464: 2F1CFFFE  cmpwi cr6, r28, -2
	ctx.cr[6].compare_i32(ctx.r[28].s32, -2, &mut ctx.xer);
	// 82424468: 409A002C  bne cr6, 0x82424494
	if !ctx.cr[6].eq {
	pc = 0x82424494; continue 'dispatch;
	}
	// 8242446C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424470: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82424474: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82424478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242447C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424484: 4E800421  bctrl
	ctx.lr = 0x82424488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82424488: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8242448C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82424490: 480000C0  b 0x82424550
	pc = 0x82424550; continue 'dispatch;
	// 82424494: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82424498: 409A00C0  bne cr6, 0x82424558
	if !ctx.cr[6].eq {
	pc = 0x82424558; continue 'dispatch;
	}
	// 8242449C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824244A0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824244A4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824244A8: 48009841  bl 0x8242dce8
	ctx.lr = 0x824244AC;
	sub_8242DCE8(ctx, base);
	// 824244AC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824244B0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824244B4: 419A0014  beq cr6, 0x824244c8
	if ctx.cr[6].eq {
	pc = 0x824244C8; continue 'dispatch;
	}
	// 824244B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824244BC: 386B1B38  addi r3, r11, 0x1b38
	ctx.r[3].s64 = ctx.r[11].s64 + 6968;
	// 824244C0: 48000219  bl 0x824246d8
	ctx.lr = 0x824244C4;
	sub_824246D8(ctx, base);
	// 824244C4: 4BFFFFA8  b 0x8242446c
	pc = 0x8242446C; continue 'dispatch;
	// 824244C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824244CC: 815D0218  lwz r10, 0x218(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(536 as u32) ) } as u64;
	// 824244D0: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824244D4: 917D0210  stw r11, 0x210(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(528 as u32), ctx.r[11].u32 ) };
	// 824244D8: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824244DC: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824244E0: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824244E4: 995D0010  stb r10, 0x10(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 824244E8: 913D0014  stw r9, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 824244EC: 917F0320  stw r11, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[11].u32 ) };
	// 824244F0: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 824244F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824244F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824244FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82424500: 48004A09  bl 0x82428f08
	ctx.lr = 0x82424504;
	sub_82428F08(ctx, base);
	// 82424504: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424508: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242450C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82424510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82424514: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82424518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242451C: 4E800421  bctrl
	ctx.lr = 0x82424520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82424520: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424524: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82424528: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8242452C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82424530: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424534: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424538: 4E800421  bctrl
	ctx.lr = 0x8242453C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242453C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82424540: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82424544: 7D7C5A14  add r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82424548: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8242454C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82424550: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82424554: 48110BB0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82424558: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8242455C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82424560: 419A0010  beq cr6, 0x82424570
	if ctx.cr[6].eq {
	pc = 0x82424570; continue 'dispatch;
	}
	// 82424564: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424568: 386B1B14  addi r3, r11, 0x1b14
	ctx.r[3].s64 = ctx.r[11].s64 + 6932;
	// 8242456C: 4BFFFF54  b 0x824244c0
	pc = 0x824244C0; continue 'dispatch;
	// 82424570: 817F0324  lwz r11, 0x324(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(804 as u32) ) } as u64;
	// 82424574: 937F0320  stw r27, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[27].u32 ) };
	// 82424578: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242457C: 4182FF74  beq 0x824244f0
	if ctx.cr[0].eq {
	pc = 0x824244F0; continue 'dispatch;
	}
	// 82424580: 807F0328  lwz r3, 0x328(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(808 as u32) ) } as u64;
	// 82424584: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82424588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242458C: 4E800421  bctrl
	ctx.lr = 0x82424590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82424590: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82424594: 4080FF5C  bge 0x824244f0
	if !ctx.cr[0].lt {
	pc = 0x824244F0; continue 'dispatch;
	}
	// 82424598: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8242459C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824245A0: 4BFFFEA4  b 0x82424444
	pc = 0x82424444; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824245A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824245A8 size=160
    let mut pc: u32 = 0x824245A8;
    'dispatch: loop {
        match pc {
            0x824245A8 => {
    //   block [0x824245A8..0x82424648)
	// 824245A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824245AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824245B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824245B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824245B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824245BC: 4800A995  bl 0x8242ef50
	ctx.lr = 0x824245C0;
	sub_8242EF50(ctx, base);
	// 824245C0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824245C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824245C8: 409A0068  bne cr6, 0x82424630
	if !ctx.cr[6].eq {
	pc = 0x82424630; continue 'dispatch;
	}
	// 824245CC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824245D0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824245D4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824245D8: 409A0010  bne cr6, 0x824245e8
	if !ctx.cr[6].eq {
	pc = 0x824245E8; continue 'dispatch;
	}
	// 824245DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824245E0: 4BFFFDF9  bl 0x824243d8
	ctx.lr = 0x824245E4;
	sub_824243D8(ctx, base);
	// 824245E4: 4800004C  b 0x82424630
	pc = 0x82424630; continue 'dispatch;
	// 824245E8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824245EC: 409A0044  bne cr6, 0x82424630
	if !ctx.cr[6].eq {
	pc = 0x82424630; continue 'dispatch;
	}
	// 824245F0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824245F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824245F8: 409A0038  bne cr6, 0x82424630
	if !ctx.cr[6].eq {
	pc = 0x82424630; continue 'dispatch;
	}
	// 824245FC: 817F0320  lwz r11, 0x320(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 82424600: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82424604: 41980014  blt cr6, 0x82424618
	if ctx.cr[6].lt {
	pc = 0x82424618; continue 'dispatch;
	}
	// 82424608: 409A0028  bne cr6, 0x82424630
	if !ctx.cr[6].eq {
	pc = 0x82424630; continue 'dispatch;
	}
	// 8242460C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82424610: 4BFFF7D9  bl 0x82423de8
	ctx.lr = 0x82424614;
	sub_82423DE8(ctx, base);
	// 82424614: 4800001C  b 0x82424630
	pc = 0x82424630; continue 'dispatch;
	// 82424618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242461C: 4BFFF355  bl 0x82423970
	ctx.lr = 0x82424620;
	sub_82423970(ctx, base);
	// 82424620: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82424624: 409A000C  bne cr6, 0x82424630
	if !ctx.cr[6].eq {
	pc = 0x82424630; continue 'dispatch;
	}
	// 82424628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242462C: 4BFFF345  bl 0x82423970
	ctx.lr = 0x82424630;
	sub_82423970(ctx, base);
	// 82424630: 4800A921  bl 0x8242ef50
	ctx.lr = 0x82424634;
	sub_8242EF50(ctx, base);
	// 82424634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82424638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242463C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424648 size=140
    let mut pc: u32 = 0x82424648;
    'dispatch: loop {
        match pc {
            0x82424648 => {
    //   block [0x82424648..0x824246D4)
	// 82424648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242464C: 48110A6D  bl 0x825350b8
	ctx.lr = 0x82424650;
	sub_82535080(ctx, base);
	// 82424650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424654: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82424658: 3BCB8E4C  addi r30, r11, -0x71b4
	ctx.r[30].s64 = ctx.r[11].s64 + -29108;
	// 8242465C: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82424660: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82424664: 419A0018  beq cr6, 0x8242467c
	if ctx.cr[6].eq {
	pc = 0x8242467C; continue 'dispatch;
	}
	// 82424668: 397EFFF4  addi r11, r30, -0xc
	ctx.r[11].s64 = ctx.r[30].s64 + -12;
	// 8242466C: 807EFFF8  lwz r3, -8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424670: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424678: 4E800421  bctrl
	ctx.lr = 0x8242467C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242467C: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82424680: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82424684: 3BAB29C0  addi r29, r11, 0x29c0
	ctx.r[29].s64 = ctx.r[11].s64 + 10688;
	// 82424688: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8242468C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82424690: 4BFFFF19  bl 0x824245a8
	ctx.lr = 0x82424694;
	sub_824245A8(ctx, base);
	// 82424694: 3BFF032C  addi r31, r31, 0x32c
	ctx.r[31].s64 = ctx.r[31].s64 + 812;
	// 82424698: 397D0CB0  addi r11, r29, 0xcb0
	ctx.r[11].s64 = ctx.r[29].s64 + 3248;
	// 8242469C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824246A0: 4198FFEC  blt cr6, 0x8242468c
	if ctx.cr[6].lt {
	pc = 0x8242468C; continue 'dispatch;
	}
	// 824246A4: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824246A8: 4082FFE0  bne 0x82424688
	if !ctx.cr[0].eq {
	pc = 0x82424688; continue 'dispatch;
	}
	// 824246AC: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824246B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824246B4: 419A0018  beq cr6, 0x824246cc
	if ctx.cr[6].eq {
	pc = 0x824246CC; continue 'dispatch;
	}
	// 824246B8: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 824246BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824246C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824246C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824246C8: 4E800421  bctrl
	ctx.lr = 0x824246CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824246CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824246D0: 48110A38  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824246D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824246D8 size=4
    let mut pc: u32 = 0x824246D8;
    'dispatch: loop {
        match pc {
            0x824246D8 => {
    //   block [0x824246D8..0x824246DC)
	// 824246D8: 4BFFCBC0  b 0x82421298
	sub_82421298(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824246E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824246E0 size=108
    let mut pc: u32 = 0x824246E0;
    'dispatch: loop {
        match pc {
            0x824246E0 => {
    //   block [0x824246E0..0x8242474C)
	// 824246E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824246E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824246E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824246EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824246F0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824246F4: 3BEB8E50  addi r31, r11, -0x71b0
	ctx.r[31].s64 = ctx.r[11].s64 + -29104;
	// 824246F8: 391F0580  addi r8, r31, 0x580
	ctx.r[8].s64 = ctx.r[31].s64 + 1408;
	// 824246FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82424700: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82424704: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82424708: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242470C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82424710: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82424714: 4082FFE8  bne 0x824246fc
	if !ctx.cr[0].eq {
	pc = 0x824246FC; continue 'dispatch;
	}
	// 82424718: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8242471C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82424720: 409A0018  bne cr6, 0x82424738
	if !ctx.cr[6].eq {
	pc = 0x82424738; continue 'dispatch;
	}
	// 82424724: 48005195  bl 0x824298b8
	ctx.lr = 0x82424728;
	sub_824298B8(ctx, base);
	// 82424728: 38A00580  li r5, 0x580
	ctx.r[5].s64 = 1408;
	// 8242472C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82424730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82424734: 48110A9D  bl 0x825351d0
	ctx.lr = 0x82424738;
	sub_825351D0(ctx, base);
	// 82424738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242473C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424750 size=220
    let mut pc: u32 = 0x82424750;
    'dispatch: loop {
        match pc {
            0x82424750 => {
    //   block [0x82424750..0x8242482C)
	// 82424750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424754: 48110965  bl 0x825350b8
	ctx.lr = 0x82424758;
	sub_82535080(ctx, base);
	// 82424758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242475C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82424760: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82424764: 394B8E50  addi r10, r11, -0x71b0
	ctx.r[10].s64 = ctx.r[11].s64 + -29104;
	// 82424768: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242476C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82424770: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82424774: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424778: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8242477C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82424780: 419A0018  beq cr6, 0x82424798
	if ctx.cr[6].eq {
	pc = 0x82424798; continue 'dispatch;
	}
	// 82424784: 396B002C  addi r11, r11, 0x2c
	ctx.r[11].s64 = ctx.r[11].s64 + 44;
	// 82424788: 390A0580  addi r8, r10, 0x580
	ctx.r[8].s64 = ctx.r[10].s64 + 1408;
	// 8242478C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82424790: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82424794: 4198FFE0  blt cr6, 0x82424774
	if ctx.cr[6].lt {
	pc = 0x82424774; continue 'dispatch;
	}
	// 82424798: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8242479C: 419A0084  beq cr6, 0x82424820
	if ctx.cr[6].eq {
	pc = 0x82424820; continue 'dispatch;
	}
	// 824247A0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824247A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824247A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824247AC: 48110A25  bl 0x825351d0
	ctx.lr = 0x824247B0;
	sub_825351D0(ctx, base);
	// 824247B0: 3BDD0024  addi r30, r29, 0x24
	ctx.r[30].s64 = ctx.r[29].s64 + 36;
	// 824247B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824247B8: 480060B9  bl 0x8242a870
	ctx.lr = 0x824247BC;
	sub_8242A870(ctx, base);
	// 824247BC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824247C0: 41800060  blt 0x82424820
	if ctx.cr[0].lt {
	pc = 0x82424820; continue 'dispatch;
	}
	// 824247C4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824247C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824247CC: 41820054  beq 0x82424820
	if ctx.cr[0].eq {
	pc = 0x82424820; continue 'dispatch;
	}
	// 824247D0: 38C02020  li r6, 0x2020
	ctx.r[6].s64 = 8224;
	// 824247D4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824247D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824247DC: 48005E7D  bl 0x8242a658
	ctx.lr = 0x824247E0;
	sub_8242A658(ctx, base);
	// 824247E0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824247E4: 40990028  ble cr6, 0x8242480c
	if !ctx.cr[6].gt {
	pc = 0x8242480C; continue 'dispatch;
	}
	// 824247E8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 824247EC: 393D001C  addi r9, r29, 0x1c
	ctx.r[9].s64 = ctx.r[29].s64 + 28;
	// 824247F0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824247F4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824247F8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824247FC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82424800: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82424804: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82424808: 4082FFEC  bne 0x824247f4
	if !ctx.cr[0].eq {
	pc = 0x824247F4; continue 'dispatch;
	}
	// 8242480C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82424810: 93FD0008  stw r31, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82424814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82424818: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242481C: 48000008  b 0x82424824
	pc = 0x82424824; continue 'dispatch;
	// 82424820: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82424824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82424828: 481108E0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424830 size=12
    let mut pc: u32 = 0x82424830;
    'dispatch: loop {
        match pc {
            0x82424830 => {
    //   block [0x82424830..0x8242483C)
	// 82424830: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424834: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424838: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242483C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242483C size=8
    let mut pc: u32 = 0x8242483C;
    'dispatch: loop {
        match pc {
            0x8242483C => {
    //   block [0x8242483C..0x82424844)
	// 8242483C: 48005504  b 0x82429d40
	sub_82429D40(ctx, base);
	return;
	// 82424840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424848 size=12
    let mut pc: u32 = 0x82424848;
    'dispatch: loop {
        match pc {
            0x82424848 => {
    //   block [0x82424848..0x82424854)
	// 82424848: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242484C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424850: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424854(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424854 size=8
    let mut pc: u32 = 0x82424854;
    'dispatch: loop {
        match pc {
            0x82424854 => {
    //   block [0x82424854..0x8242485C)
	// 82424854: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82424858: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242485C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242485C size=8
    let mut pc: u32 = 0x8242485C;
    'dispatch: loop {
        match pc {
            0x8242485C => {
    //   block [0x8242485C..0x82424864)
	// 8242485C: 480054E4  b 0x82429d40
	sub_82429D40(ctx, base);
	return;
	// 82424860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424868 size=12
    let mut pc: u32 = 0x82424868;
    'dispatch: loop {
        match pc {
            0x82424868 => {
    //   block [0x82424868..0x82424874)
	// 82424868: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242486C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424870: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424874 size=12
    let mut pc: u32 = 0x82424874;
    'dispatch: loop {
        match pc {
            0x82424874 => {
    //   block [0x82424874..0x82424880)
	// 82424874: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82424878: 409A0008  bne cr6, 0x82424880
	if !ctx.cr[6].eq {
		sub_82424880(ctx, base);
		return;
	}
	// 8242487C: 48005424  b 0x82429ca0
	sub_82429CA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424880 size=8
    let mut pc: u32 = 0x82424880;
    'dispatch: loop {
        match pc {
            0x82424880 => {
    //   block [0x82424880..0x82424888)
	// 82424880: 480054C0  b 0x82429d40
	sub_82429D40(ctx, base);
	return;
	// 82424884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424888 size=80
    let mut pc: u32 = 0x82424888;
    'dispatch: loop {
        match pc {
            0x82424888 => {
    //   block [0x82424888..0x824248D8)
	// 82424888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242488C: 48110831  bl 0x825350bc
	ctx.lr = 0x82424890;
	sub_82535080(ctx, base);
	// 82424890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82424898: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242489C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824248A0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824248A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824248A8: 41820028  beq 0x824248d0
	if ctx.cr[0].eq {
	pc = 0x824248D0; continue 'dispatch;
	}
	// 824248AC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824248B0: 48005A21  bl 0x8242a2d0
	ctx.lr = 0x824248B4;
	sub_8242A2D0(ctx, base);
	// 824248B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824248B8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824248BC: 4800598D  bl 0x8242a248
	ctx.lr = 0x824248C0;
	sub_8242A248(ctx, base);
	// 824248C0: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 824248C4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824248C8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824248CC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824248D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824248D4: 48110838  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824248D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824248D8 size=56
    let mut pc: u32 = 0x824248D8;
    'dispatch: loop {
        match pc {
            0x824248D8 => {
    //   block [0x824248D8..0x82424910)
	// 824248D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824248DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824248E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824248E4: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824248E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824248EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824248F0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824248F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824248F8: 4E800421  bctrl
	ctx.lr = 0x824248FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824248FC: 5463F87E  srwi r3, r3, 1
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82424900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82424904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242490C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424910 size=12
    let mut pc: u32 = 0x82424910;
    'dispatch: loop {
        match pc {
            0x82424910 => {
    //   block [0x82424910..0x8242491C)
	// 82424910: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 82424914: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 82424918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424920 size=80
    let mut pc: u32 = 0x82424920;
    'dispatch: loop {
        match pc {
            0x82424920 => {
    //   block [0x82424920..0x82424970)
	// 82424920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242492C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424930: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424934: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82424938: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242493C: 4182001C  beq 0x82424958
	if ctx.cr[0].eq {
	pc = 0x82424958; continue 'dispatch;
	}
	// 82424940: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82424944: 48005885  bl 0x8242a1c8
	ctx.lr = 0x82424948;
	sub_8242A1C8(ctx, base);
	// 82424948: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242494C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82424950: 409A0008  bne cr6, 0x82424958
	if !ctx.cr[6].eq {
	pc = 0x82424958; continue 'dispatch;
	}
	// 82424954: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82424958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242495C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242496C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424970 size=96
    let mut pc: u32 = 0x82424970;
    'dispatch: loop {
        match pc {
            0x82424970 => {
    //   block [0x82424970..0x824249D0)
	// 82424970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424974: 48110749  bl 0x825350bc
	ctx.lr = 0x82424978;
	sub_82535080(ctx, base);
	// 82424978: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242497C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82424980: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82424984: 3BCB8E50  addi r30, r11, -0x71b0
	ctx.r[30].s64 = ctx.r[11].s64 + -29104;
	// 82424988: 3BFE0024  addi r31, r30, 0x24
	ctx.r[31].s64 = ctx.r[30].s64 + 36;
	// 8242498C: 817FFFDC  lwz r11, -0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-36 as u32) ) } as u64;
	// 82424990: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424994: 419A0018  beq cr6, 0x824249ac
	if ctx.cr[6].eq {
	pc = 0x824249AC; continue 'dispatch;
	}
	// 82424998: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242499C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824249A0: 41820008  beq 0x824249a8
	if ctx.cr[0].eq {
	pc = 0x824249A8; continue 'dispatch;
	}
	// 824249A4: 48006235  bl 0x8242abd8
	ctx.lr = 0x824249A8;
	sub_8242ABD8(ctx, base);
	// 824249A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824249AC: 3BFF002C  addi r31, r31, 0x2c
	ctx.r[31].s64 = ctx.r[31].s64 + 44;
	// 824249B0: 397E05A4  addi r11, r30, 0x5a4
	ctx.r[11].s64 = ctx.r[30].s64 + 1444;
	// 824249B4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824249B8: 4198FFD4  blt cr6, 0x8242498c
	if ctx.cr[6].lt {
	pc = 0x8242498C; continue 'dispatch;
	}
	// 824249BC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824249C0: 40990008  ble cr6, 0x824249c8
	if !ctx.cr[6].gt {
	pc = 0x824249C8; continue 'dispatch;
	}
	// 824249C4: 48004BBD  bl 0x82429580
	ctx.lr = 0x824249C8;
	sub_82429580(ctx, base);
	// 824249C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824249CC: 48110740  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824249D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824249D0 size=12
    let mut pc: u32 = 0x824249D0;
    'dispatch: loop {
        match pc {
            0x824249D0 => {
    //   block [0x824249D0..0x824249DC)
	// 824249D0: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 824249D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824249D8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824249DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824249DC size=8
    let mut pc: u32 = 0x824249DC;
    'dispatch: loop {
        match pc {
            0x824249DC => {
    //   block [0x824249DC..0x824249E4)
	// 824249DC: 48005A5C  b 0x8242a438
	sub_8242A438(ctx, base);
	return;
	// 824249E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824249E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824249E8 size=12
    let mut pc: u32 = 0x824249E8;
    'dispatch: loop {
        match pc {
            0x824249E8 => {
    //   block [0x824249E8..0x824249F4)
	// 824249E8: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 824249EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824249F0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824249F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824249F4 size=8
    let mut pc: u32 = 0x824249F4;
    'dispatch: loop {
        match pc {
            0x824249F4 => {
    //   block [0x824249F4..0x824249FC)
	// 824249F4: 480059BC  b 0x8242a3b0
	sub_8242A3B0(ctx, base);
	return;
	// 824249F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82424A00 size=124
    let mut pc: u32 = 0x82424A00;
    'dispatch: loop {
        match pc {
            0x82424A00 => {
    //   block [0x82424A00..0x82424A7C)
	// 82424A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424A0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82424A10: 1D440064  mulli r10, r4, 0x64
	ctx.r[10].s64 = ctx.r[4].s64 * 100;
	// 82424A14: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424A18: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82424A1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424A20: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82424A24: 41820048  beq 0x82424a6c
	if ctx.cr[0].eq {
	pc = 0x82424A6C; continue 'dispatch;
	}
	// 82424A28: 7C8A07B4  extsw r10, r4
	ctx.r[10].s64 = ctx.r[4].s32 as i64;
	// 82424A2C: 7CA907B4  extsw r9, r5
	ctx.r[9].s64 = ctx.r[5].s32 as i64;
	// 82424A30: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82424A34: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82424A38: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 82424A3C: C00B2698  lfs f0, 0x2698(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82424A40: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82424A44: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82424A48: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82424A4C: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82424A50: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82424A54: FD606818  frsp f11, f13
	ctx.f[11].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82424A58: C1AB20A8  lfs f13, 0x20a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8360 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82424A5C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82424A60: EDAB0372  fmuls f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 82424A64: EC2C683A  fmadds f1, f12, f0, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82424A68: 48005AE9  bl 0x8242a550
	ctx.lr = 0x82424A6C;
	sub_8242A550(ctx, base);
	// 82424A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424A80 size=32
    let mut pc: u32 = 0x82424A80;
    'dispatch: loop {
        match pc {
            0x82424A80 => {
    //   block [0x82424A80..0x82424AA0)
	// 82424A80: 39600064  li r11, 0x64
	ctx.r[11].s64 = 100;
	// 82424A84: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82424A88: 419A0010  beq cr6, 0x82424a98
	if ctx.cr[6].eq {
	pc = 0x82424A98; continue 'dispatch;
	}
	// 82424A8C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82424A90: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 82424A94: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82424A98: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82424A9C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424AA0 size=24
    let mut pc: u32 = 0x82424AA0;
    'dispatch: loop {
        match pc {
            0x82424AA0 => {
    //   block [0x82424AA0..0x82424AB8)
	// 82424AA0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82424AA4: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 82424AA8: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 82424AAC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82424AB0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82424AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82424AB8 size=104
    let mut pc: u32 = 0x82424AB8;
    'dispatch: loop {
        match pc {
            0x82424AB8 => {
    //   block [0x82424AB8..0x82424B20)
	// 82424AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82424AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424ACC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82424AD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82424AD4: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424AD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424ADC: 41820028  beq 0x82424b04
	if ctx.cr[0].eq {
	pc = 0x82424B04; continue 'dispatch;
	}
	// 82424AE0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 82424AE4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82424AE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424AEC: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82424AF0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82424AF4: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82424AF8: C00B8E30  lfs f0, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82424AFC: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82424B00: 48004D21  bl 0x82429820
	ctx.lr = 0x82424B04;
	sub_82429820(ctx, base);
	// 82424B04: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82424B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424B14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82424B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82424B20 size=328
    let mut pc: u32 = 0x82424B20;
    'dispatch: loop {
        match pc {
            0x82424B20 => {
    //   block [0x82424B20..0x82424C68)
	// 82424B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424B28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82424B2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424B30: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82424B34: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82424B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424B3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82424B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82424B44: 397E0005  addi r11, r30, 5
	ctx.r[11].s64 = ctx.r[30].s64 + 5;
	// 82424B48: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82424B4C: 7CABF92E  stwx r5, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 82424B50: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82424B54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424B58: 409A00F0  bne cr6, 0x82424c48
	if !ctx.cr[6].eq {
	pc = 0x82424C48; continue 'dispatch;
	}
	// 82424B5C: 3565000F  addic. r11, r5, 0xf
	ctx.xer.ca = (ctx.r[5].u32 > (!(15 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + 15;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424B60: 2125FFF1  subfic r9, r5, -0xf
	ctx.xer.ca = ctx.r[5].u32 <= -15 as u32;
	ctx.r[9].s64 = (-15 as i64) - ctx.r[5].s64;
	// 82424B64: 41800008  blt 0x82424b6c
	if ctx.cr[0].lt {
	pc = 0x82424B6C; continue 'dispatch;
	}
	// 82424B68: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82424B6C: 3565FFF1  addic. r11, r5, -0xf
	ctx.xer.ca = (ctx.r[5].u32 > (!(-15 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -15;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424B70: 2145000F  subfic r10, r5, 0xf
	ctx.xer.ca = ctx.r[5].u32 <= 15 as u32;
	ctx.r[10].s64 = (15 as i64) - ctx.r[5].s64;
	// 82424B74: 41800008  blt 0x82424b7c
	if ctx.cr[0].lt {
	pc = 0x82424B7C; continue 'dispatch;
	}
	// 82424B78: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82424B7C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82424B80: 40980008  bge cr6, 0x82424b88
	if !ctx.cr[6].lt {
	pc = 0x82424B88; continue 'dispatch;
	}
	// 82424B84: 7CA500D0  neg r5, r5
	ctx.r[5].s64 = -ctx.r[5].s64;
	// 82424B88: 2F09000F  cmpwi cr6, r9, 0xf
	ctx.cr[6].compare_i32(ctx.r[9].s32, 15, &mut ctx.xer);
	// 82424B8C: 41980008  blt cr6, 0x82424b94
	if ctx.cr[6].lt {
	pc = 0x82424B94; continue 'dispatch;
	}
	// 82424B90: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 82424B94: 2F0A000F  cmpwi cr6, r10, 0xf
	ctx.cr[6].compare_i32(ctx.r[10].s32, 15, &mut ctx.xer);
	// 82424B98: 41980008  blt cr6, 0x82424ba0
	if ctx.cr[6].lt {
	pc = 0x82424BA0; continue 'dispatch;
	}
	// 82424B9C: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 82424BA0: 2F05000F  cmpwi cr6, r5, 0xf
	ctx.cr[6].compare_i32(ctx.r[5].s32, 15, &mut ctx.xer);
	// 82424BA4: 41980008  blt cr6, 0x82424bac
	if ctx.cr[6].lt {
	pc = 0x82424BAC; continue 'dispatch;
	}
	// 82424BA8: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 82424BAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424BB0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424BB4: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82424BB8: 396B1B60  addi r11, r11, 0x1b60
	ctx.r[11].s64 = ctx.r[11].s64 + 7008;
	// 82424BBC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82424BC0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82424BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82424BC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82424BCC: 7FC85C2E  lfsx f30, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82424BD0: 7FEA5C2E  lfsx f31, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82424BD4: 7C295C2E  lfsx f1, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82424BD8: 48005A21  bl 0x8242a5f8
	ctx.lr = 0x82424BDC;
	sub_8242A5F8(ctx, base);
	// 82424BDC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82424BE0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424BE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82424BE8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82424BEC: 48005A0D  bl 0x8242a5f8
	ctx.lr = 0x82424BF0;
	sub_8242A5F8(ctx, base);
	// 82424BF0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82424BF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82424BF8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424BFC: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82424C00: 480059F9  bl 0x8242a5f8
	ctx.lr = 0x82424C04;
	sub_8242A5F8(ctx, base);
	// 82424C04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424C08: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82424C0C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424C10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82424C14: C3EB1BB8  lfs f31, 0x1bb8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82424C18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82424C1C: 480059DD  bl 0x8242a5f8
	ctx.lr = 0x82424C20;
	sub_8242A5F8(ctx, base);
	// 82424C20: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82424C24: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424C28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82424C2C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82424C30: 480059C9  bl 0x8242a5f8
	ctx.lr = 0x82424C34;
	sub_8242A5F8(ctx, base);
	// 82424C34: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82424C38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82424C3C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424C40: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82424C44: 480059B5  bl 0x8242a5f8
	ctx.lr = 0x82424C48;
	sub_8242A5F8(ctx, base);
	// 82424C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82424C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424C54: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82424C58: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82424C5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82424C60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424C68 size=12
    let mut pc: u32 = 0x82424C68;
    'dispatch: loop {
        match pc {
            0x82424C68 => {
    //   block [0x82424C68..0x82424C74)
	// 82424C68: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424C6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424C70: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424C74 size=8
    let mut pc: u32 = 0x82424C74;
    'dispatch: loop {
        match pc {
            0x82424C74 => {
    //   block [0x82424C74..0x82424C7C)
	// 82424C74: 480054DC  b 0x8242a150
	sub_8242A150(ctx, base);
	return;
	// 82424C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424C80 size=16
    let mut pc: u32 = 0x82424C80;
    'dispatch: loop {
        match pc {
            0x82424C80 => {
    //   block [0x82424C80..0x82424C90)
	// 82424C80: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424C84: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82424C88: 419A0008  beq cr6, 0x82424c90
	if ctx.cr[6].eq {
		sub_82424C90(ctx, base);
		return;
	}
	// 82424C8C: 4800514C  b 0x82429dd8
	sub_82429DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424C90 size=4
    let mut pc: u32 = 0x82424C90;
    'dispatch: loop {
        match pc {
            0x82424C90 => {
    //   block [0x82424C90..0x82424C94)
	// 82424C90: 480051C0  b 0x82429e50
	sub_82429E50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424C98 size=16
    let mut pc: u32 = 0x82424C98;
    'dispatch: loop {
        match pc {
            0x82424C98 => {
    //   block [0x82424C98..0x82424CA8)
	// 82424C98: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424C9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424CA0: 41820008  beq 0x82424ca8
	if ctx.cr[0].eq {
		sub_82424CA8(ctx, base);
		return;
	}
	// 82424CA4: 48005AD4  b 0x8242a778
	sub_8242A778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424CA8 size=8
    let mut pc: u32 = 0x82424CA8;
    'dispatch: loop {
        match pc {
            0x82424CA8 => {
    //   block [0x82424CA8..0x82424CB0)
	// 82424CA8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82424CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82424CB0 size=112
    let mut pc: u32 = 0x82424CB0;
    'dispatch: loop {
        match pc {
            0x82424CB0 => {
    //   block [0x82424CB0..0x82424D20)
	// 82424CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424CBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82424CC4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424CC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424CCC: 41820038  beq 0x82424d04
	if ctx.cr[0].eq {
	pc = 0x82424D04; continue 'dispatch;
	}
	// 82424CD0: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 82424CD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424CD8: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82424CDC: 396B1BA0  addi r11, r11, 0x1ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 7072;
	// 82424CE0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82424CE4: 7CA9582E  lwzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82424CE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424CEC: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82424CF0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82424CF4: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82424CF8: C00B8E30  lfs f0, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82424CFC: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82424D00: 480058F9  bl 0x8242a5f8
	ctx.lr = 0x82424D04;
	sub_8242A5F8(ctx, base);
	// 82424D04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82424D08: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82424D0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424D18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424D20 size=88
    let mut pc: u32 = 0x82424D20;
    'dispatch: loop {
        match pc {
            0x82424D20 => {
    //   block [0x82424D20..0x82424D78)
	// 82424D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424D2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82424D34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82424D38: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424D3C: 480055DD  bl 0x8242a318
	ctx.lr = 0x82424D40;
	sub_8242A318(ctx, base);
	// 82424D40: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424D44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82424D48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424D4C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424D54: 4E800421  bctrl
	ctx.lr = 0x82424D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82424D58: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82424D5C: 546BF87E  srwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82424D60: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82424D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424D70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424D78 size=8
    let mut pc: u32 = 0x82424D78;
    'dispatch: loop {
        match pc {
            0x82424D78 => {
    //   block [0x82424D78..0x82424D80)
	// 82424D78: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424D7C: 4800574C  b 0x8242a4c8
	sub_8242A4C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424D80 size=96
    let mut pc: u32 = 0x82424D80;
    'dispatch: loop {
        match pc {
            0x82424D80 => {
    //   block [0x82424D80..0x82424DE0)
	// 82424D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82424D94: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424D98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424D9C: 41820008  beq 0x82424da4
	if ctx.cr[0].eq {
	pc = 0x82424DA4; continue 'dispatch;
	}
	// 82424DA0: 48004FA1  bl 0x82429d40
	ctx.lr = 0x82424DA4;
	sub_82429D40(ctx, base);
	// 82424DA4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82424DA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424DAC: 41820010  beq 0x82424dbc
	if ctx.cr[0].eq {
	pc = 0x82424DBC; continue 'dispatch;
	}
	// 82424DB0: 48005B19  bl 0x8242a8c8
	ctx.lr = 0x82424DB4;
	sub_8242A8C8(ctx, base);
	// 82424DB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82424DB8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82424DBC: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82424DC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82424DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82424DC8: 48110409  bl 0x825351d0
	ctx.lr = 0x82424DCC;
	sub_825351D0(ctx, base);
	// 82424DCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82424DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424DD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424DE0 size=156
    let mut pc: u32 = 0x82424DE0;
    'dispatch: loop {
        match pc {
            0x82424DE0 => {
    //   block [0x82424DE0..0x82424E7C)
	// 82424DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424DE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82424DEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424DF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424DF4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82424DF8: 3BCB8E50  addi r30, r11, -0x71b0
	ctx.r[30].s64 = ctx.r[11].s64 + -29104;
	// 82424DFC: 391E0580  addi r8, r30, 0x580
	ctx.r[8].s64 = ctx.r[30].s64 + 1408;
	// 82424E00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82424E04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82424E08: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82424E0C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82424E10: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82424E14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82424E18: 4082FFE8  bne 0x82424e00
	if !ctx.cr[0].eq {
	pc = 0x82424E00; continue 'dispatch;
	}
	// 82424E1C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82424E20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424E24: 409A0040  bne cr6, 0x82424e64
	if !ctx.cr[6].eq {
	pc = 0x82424E64; continue 'dispatch;
	}
	// 82424E28: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82424E2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424E30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424E34: 419A000C  beq cr6, 0x82424e40
	if ctx.cr[6].eq {
	pc = 0x82424E40; continue 'dispatch;
	}
	// 82424E38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82424E3C: 4BFFFF45  bl 0x82424d80
	ctx.lr = 0x82424E40;
	sub_82424D80(ctx, base);
	// 82424E40: 3BFF002C  addi r31, r31, 0x2c
	ctx.r[31].s64 = ctx.r[31].s64 + 44;
	// 82424E44: 397E0580  addi r11, r30, 0x580
	ctx.r[11].s64 = ctx.r[30].s64 + 1408;
	// 82424E48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82424E4C: 4198FFE0  blt cr6, 0x82424e2c
	if ctx.cr[6].lt {
	pc = 0x82424E2C; continue 'dispatch;
	}
	// 82424E50: 48004419  bl 0x82429268
	ctx.lr = 0x82424E54;
	sub_82429268(ctx, base);
	// 82424E54: 38A00580  li r5, 0x580
	ctx.r[5].s64 = 1408;
	// 82424E58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82424E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82424E60: 48110371  bl 0x825351d0
	ctx.lr = 0x82424E64;
	sub_825351D0(ctx, base);
	// 82424E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424E70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82424E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82424E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424E80 size=12
    let mut pc: u32 = 0x82424E80;
    'dispatch: loop {
        match pc {
            0x82424E80 => {
    //   block [0x82424E80..0x82424E8C)
	// 82424E80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424E84: 386B1BCC  addi r3, r11, 0x1bcc
	ctx.r[3].s64 = ctx.r[11].s64 + 7116;
	// 82424E88: 480088F0  b 0x8242d778
	sub_8242D778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424E90 size=28
    let mut pc: u32 = 0x82424E90;
    'dispatch: loop {
        match pc {
            0x82424E90 => {
    //   block [0x82424E90..0x82424EAC)
	// 82424E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82424E94: 409A0018  bne cr6, 0x82424eac
	if !ctx.cr[6].eq {
		sub_82424EAC(ctx, base);
		return;
	}
	// 82424E98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424E9C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82424EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424EA4: 386B1C8C  addi r3, r11, 0x1c8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7308;
	// 82424EA8: 480060A0  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424EAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424EAC size=32
    let mut pc: u32 = 0x82424EAC;
    'dispatch: loop {
        match pc {
            0x82424EAC => {
    //   block [0x82424EAC..0x82424ECC)
	// 82424EAC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82424EB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82424EB4: 409A0018  bne cr6, 0x82424ecc
	if !ctx.cr[6].eq {
		sub_82424ECC(ctx, base);
		return;
	}
	// 82424EB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424EBC: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82424EC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82424EC4: 386B1C80  addi r3, r11, 0x1c80
	ctx.r[3].s64 = ctx.r[11].s64 + 7296;
	// 82424EC8: 48006080  b 0x8242af48
	sub_8242AF48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424ECC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424ECC size=12
    let mut pc: u32 = 0x82424ECC;
    'dispatch: loop {
        match pc {
            0x82424ECC => {
    //   block [0x82424ECC..0x82424ED8)
	// 82424ECC: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82424ED0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82424ED4: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424ED8 size=12
    let mut pc: u32 = 0x82424ED8;
    'dispatch: loop {
        match pc {
            0x82424ED8 => {
    //   block [0x82424ED8..0x82424EE4)
	// 82424ED8: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424EDC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82424EE0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424EE4 size=20
    let mut pc: u32 = 0x82424EE4;
    'dispatch: loop {
        match pc {
            0x82424EE4 => {
    //   block [0x82424EE4..0x82424EF8)
	// 82424EE4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82424EE8: 409A0020  bne cr6, 0x82424f08
	if !ctx.cr[6].eq {
		sub_82424F08(ctx, base);
		return;
	}
	// 82424EEC: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424EF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424EF4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424EF8 size=16
    let mut pc: u32 = 0x82424EF8;
    'dispatch: loop {
        match pc {
            0x82424EF8 => {
    //   block [0x82424EF8..0x82424F08)
	// 82424EF8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82424EFC: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82424F00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424F04: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424F08 size=80
    let mut pc: u32 = 0x82424F08;
    'dispatch: loop {
        match pc {
            0x82424F08 => {
    //   block [0x82424F08..0x82424F58)
	// 82424F08: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82424F0C: 409A0068  bne cr6, 0x82424f74
	if !ctx.cr[6].eq {
		sub_82424F74(ctx, base);
		return;
	}
	// 82424F10: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82424F14: 7D0B5051  subf. r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82424F18: 41810008  bgt 0x82424f20
	if ctx.cr[0].gt {
	pc = 0x82424F20; continue 'dispatch;
	}
	// 82424F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82424F20: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82424F24: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82424F28: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82424F2C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82424F30: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82424F34: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82424F38: 41980008  blt cr6, 0x82424f40
	if ctx.cr[6].lt {
	pc = 0x82424F40; continue 'dispatch;
	}
	// 82424F3C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82424F40: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82424F44: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82424F48: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82424F4C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82424F50: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82424F54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424F58 size=12
    let mut pc: u32 = 0x82424F58;
    'dispatch: loop {
        match pc {
            0x82424F58 => {
    //   block [0x82424F58..0x82424F64)
	// 82424F58: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424F5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424F60: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424F64 size=16
    let mut pc: u32 = 0x82424F64;
    'dispatch: loop {
        match pc {
            0x82424F64 => {
    //   block [0x82424F64..0x82424F74)
	// 82424F64: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82424F68: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82424F6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424F70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424F74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424F74 size=24
    let mut pc: u32 = 0x82424F74;
    'dispatch: loop {
        match pc {
            0x82424F74 => {
    //   block [0x82424F74..0x82424F8C)
	// 82424F74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82424F78: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82424F7C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82424F80: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82424F84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82424F88: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424F8C size=16
    let mut pc: u32 = 0x82424F8C;
    'dispatch: loop {
        match pc {
            0x82424F8C => {
    //   block [0x82424F8C..0x82424F9C)
	// 82424F8C: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82424F90: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82424F94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82424F98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424F9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82424F9C size=4
    let mut pc: u32 = 0x82424F9C;
    'dispatch: loop {
        match pc {
            0x82424F9C => {
    //   block [0x82424F9C..0x82424FA0)
	// 82424F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82424FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82424FA0 size=100
    let mut pc: u32 = 0x82424FA0;
    'dispatch: loop {
        match pc {
            0x82424FA0 => {
    //   block [0x82424FA0..0x82425004)
	// 82424FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82424FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82424FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82424FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82424FB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82424FB4: 480087CD  bl 0x8242d780
	ctx.lr = 0x82424FB8;
	sub_8242D780(ctx, base);
	// 82424FB8: 480088C1  bl 0x8242d878
	ctx.lr = 0x82424FBC;
	sub_8242D878(ctx, base);
	// 82424FBC: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 82424FC0: 83FE93D4  lwz r31, -0x6c2c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-27692 as u32) ) } as u64;
	// 82424FC4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82424FC8: 409A0018  bne cr6, 0x82424fe0
	if !ctx.cr[6].eq {
	pc = 0x82424FE0; continue 'dispatch;
	}
	// 82424FCC: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82424FD0: 38A006C0  li r5, 0x6c0
	ctx.r[5].s64 = 1728;
	// 82424FD4: 386B2300  addi r3, r11, 0x2300
	ctx.r[3].s64 = ctx.r[11].s64 + 8960;
	// 82424FD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82424FDC: 481101F5  bl 0x825351d0
	ctx.lr = 0x82424FE0;
	sub_825351D0(ctx, base);
	// 82424FE0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82424FE4: 917E93D4  stw r11, -0x6c2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-27692 as u32), ctx.r[11].u32 ) };
	// 82424FE8: 480088D1  bl 0x8242d8b8
	ctx.lr = 0x82424FEC;
	sub_8242D8B8(ctx, base);
	// 82424FEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82424FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82424FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82424FF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82424FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425008 size=80
    let mut pc: u32 = 0x82425008;
    'dispatch: loop {
        match pc {
            0x82425008 => {
    //   block [0x82425008..0x82425058)
	// 82425008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242500C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425014: 48008865  bl 0x8242d878
	ctx.lr = 0x82425018;
	sub_8242D878(ctx, base);
	// 82425018: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242501C: 816A93D4  lwz r11, -0x6c2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27692 as u32) ) } as u64;
	// 82425020: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425024: 916A93D4  stw r11, -0x6c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27692 as u32), ctx.r[11].u32 ) };
	// 82425028: 40820018  bne 0x82425040
	if !ctx.cr[0].eq {
	pc = 0x82425040; continue 'dispatch;
	}
	// 8242502C: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82425030: 38A006C0  li r5, 0x6c0
	ctx.r[5].s64 = 1728;
	// 82425034: 386B2300  addi r3, r11, 0x2300
	ctx.r[3].s64 = ctx.r[11].s64 + 8960;
	// 82425038: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242503C: 48110195  bl 0x825351d0
	ctx.lr = 0x82425040;
	sub_825351D0(ctx, base);
	// 82425040: 48008879  bl 0x8242d8b8
	ctx.lr = 0x82425044;
	sub_8242D8B8(ctx, base);
	// 82425044: 480087BD  bl 0x8242d800
	ctx.lr = 0x82425048;
	sub_8242D800(ctx, base);
	// 82425048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242504C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425058 size=136
    let mut pc: u32 = 0x82425058;
    'dispatch: loop {
        match pc {
            0x82425058 => {
    //   block [0x82425058..0x824250E0)
	// 82425058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242505C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242506C: 4800880D  bl 0x8242d878
	ctx.lr = 0x82425070;
	sub_8242D878(ctx, base);
	// 82425070: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82425074: 409A001C  bne cr6, 0x82425090
	if !ctx.cr[6].eq {
	pc = 0x82425090; continue 'dispatch;
	}
	// 82425078: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242507C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82425080: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425084: 386B1BE4  addi r3, r11, 0x1be4
	ctx.r[3].s64 = ctx.r[11].s64 + 7140;
	// 82425088: 48005EC1  bl 0x8242af48
	ctx.lr = 0x8242508C;
	sub_8242AF48(ctx, base);
	// 8242508C: 4800003C  b 0x824250c8
	pc = 0x824250C8; continue 'dispatch;
	// 82425090: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425098: 409A0018  bne cr6, 0x824250b0
	if !ctx.cr[6].eq {
	pc = 0x824250B0; continue 'dispatch;
	}
	// 8242509C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824250A0: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 824250A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824250A8: 386B1BD8  addi r3, r11, 0x1bd8
	ctx.r[3].s64 = ctx.r[11].s64 + 7128;
	// 824250AC: 4BFFFFDC  b 0x82425088
	pc = 0x82425088; continue 'dispatch;
	// 824250B0: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 824250B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824250B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824250BC: 48110115  bl 0x825351d0
	ctx.lr = 0x824250C0;
	sub_825351D0(ctx, base);
	// 824250C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824250C4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824250C8: 480087F1  bl 0x8242d8b8
	ctx.lr = 0x824250CC;
	sub_8242D8B8(ctx, base);
	// 824250CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824250D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824250D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824250D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824250DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824250E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824250E0 size=124
    let mut pc: u32 = 0x824250E0;
    'dispatch: loop {
        match pc {
            0x824250E0 => {
    //   block [0x824250E0..0x8242515C)
	// 824250E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824250E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824250E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824250EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824250F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824250F4: 48008785  bl 0x8242d878
	ctx.lr = 0x824250F8;
	sub_8242D878(ctx, base);
	// 824250F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824250FC: 409A0020  bne cr6, 0x8242511c
	if !ctx.cr[6].eq {
	pc = 0x8242511C; continue 'dispatch;
	}
	// 82425100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425104: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82425108: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242510C: 386B1BFC  addi r3, r11, 0x1bfc
	ctx.r[3].s64 = ctx.r[11].s64 + 7164;
	// 82425110: 48005E39  bl 0x8242af48
	ctx.lr = 0x82425114;
	sub_8242AF48(ctx, base);
	// 82425114: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82425118: 48000028  b 0x82425140
	pc = 0x82425140; continue 'dispatch;
	// 8242511C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425120: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425124: 409A0018  bne cr6, 0x8242513c
	if !ctx.cr[6].eq {
	pc = 0x8242513C; continue 'dispatch;
	}
	// 82425128: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242512C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82425130: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425134: 386B1BF0  addi r3, r11, 0x1bf0
	ctx.r[3].s64 = ctx.r[11].s64 + 7152;
	// 82425138: 4BFFFFD8  b 0x82425110
	pc = 0x82425110; continue 'dispatch;
	// 8242513C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82425140: 48008779  bl 0x8242d8b8
	ctx.lr = 0x82425144;
	sub_8242D8B8(ctx, base);
	// 82425144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242514C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425160 size=112
    let mut pc: u32 = 0x82425160;
    'dispatch: loop {
        match pc {
            0x82425160 => {
    //   block [0x82425160..0x824251D0)
	// 82425160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425164: 4810FF59  bl 0x825350bc
	ctx.lr = 0x82425168;
	sub_82535080(ctx, base);
	// 82425168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242516C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82425174: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82425178: 48008701  bl 0x8242d878
	ctx.lr = 0x8242517C;
	sub_8242D878(ctx, base);
	// 8242517C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82425180: 409A001C  bne cr6, 0x8242519c
	if !ctx.cr[6].eq {
	pc = 0x8242519C; continue 'dispatch;
	}
	// 82425184: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425188: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242518C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425190: 386B1C14  addi r3, r11, 0x1c14
	ctx.r[3].s64 = ctx.r[11].s64 + 7188;
	// 82425194: 48005DB5  bl 0x8242af48
	ctx.lr = 0x82425198;
	sub_8242AF48(ctx, base);
	// 82425198: 4800002C  b 0x824251c4
	pc = 0x824251C4; continue 'dispatch;
	// 8242519C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824251A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824251A4: 409A0018  bne cr6, 0x824251bc
	if !ctx.cr[6].eq {
	pc = 0x824251BC; continue 'dispatch;
	}
	// 824251A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824251AC: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 824251B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824251B4: 386B1C08  addi r3, r11, 0x1c08
	ctx.r[3].s64 = ctx.r[11].s64 + 7176;
	// 824251B8: 4BFFFFDC  b 0x82425194
	pc = 0x82425194; continue 'dispatch;
	// 824251BC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 824251C0: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 824251C4: 480086F5  bl 0x8242d8b8
	ctx.lr = 0x824251C8;
	sub_8242D8B8(ctx, base);
	// 824251C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824251CC: 4810FF40  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824251D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824251D0 size=128
    let mut pc: u32 = 0x824251D0;
    'dispatch: loop {
        match pc {
            0x824251D0 => {
    //   block [0x824251D0..0x82425250)
	// 824251D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824251D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824251D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824251DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824251E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824251E4: 48008695  bl 0x8242d878
	ctx.lr = 0x824251E8;
	sub_8242D878(ctx, base);
	// 824251E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824251EC: 409A001C  bne cr6, 0x82425208
	if !ctx.cr[6].eq {
	pc = 0x82425208; continue 'dispatch;
	}
	// 824251F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824251F4: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 824251F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824251FC: 386B1C2C  addi r3, r11, 0x1c2c
	ctx.r[3].s64 = ctx.r[11].s64 + 7212;
	// 82425200: 48005D49  bl 0x8242af48
	ctx.lr = 0x82425204;
	sub_8242AF48(ctx, base);
	// 82425204: 48000034  b 0x82425238
	pc = 0x82425238; continue 'dispatch;
	// 82425208: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242520C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425210: 409A0018  bne cr6, 0x82425228
	if !ctx.cr[6].eq {
	pc = 0x82425228; continue 'dispatch;
	}
	// 82425214: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425218: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242521C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425220: 386B1C20  addi r3, r11, 0x1c20
	ctx.r[3].s64 = ctx.r[11].s64 + 7200;
	// 82425224: 4BFFFFDC  b 0x82425200
	pc = 0x82425200; continue 'dispatch;
	// 82425228: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242522C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82425230: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82425234: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82425238: 48008681  bl 0x8242d8b8
	ctx.lr = 0x8242523C;
	sub_8242D8B8(ctx, base);
	// 8242523C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82425240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242524C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425250 size=184
    let mut pc: u32 = 0x82425250;
    'dispatch: loop {
        match pc {
            0x82425250 => {
    //   block [0x82425250..0x82425308)
	// 82425250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242525C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425268: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8242526C: 4800860D  bl 0x8242d878
	ctx.lr = 0x82425270;
	sub_8242D878(ctx, base);
	// 82425270: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82425274: 409A001C  bne cr6, 0x82425290
	if !ctx.cr[6].eq {
	pc = 0x82425290; continue 'dispatch;
	}
	// 82425278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242527C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82425280: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425284: 386B1C44  addi r3, r11, 0x1c44
	ctx.r[3].s64 = ctx.r[11].s64 + 7236;
	// 82425288: 48005CC1  bl 0x8242af48
	ctx.lr = 0x8242528C;
	sub_8242AF48(ctx, base);
	// 8242528C: 48000058  b 0x824252e4
	pc = 0x824252E4; continue 'dispatch;
	// 82425290: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425294: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425298: 409A0018  bne cr6, 0x824252b0
	if !ctx.cr[6].eq {
	pc = 0x824252B0; continue 'dispatch;
	}
	// 8242529C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824252A0: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 824252A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824252A8: 386B1C38  addi r3, r11, 0x1c38
	ctx.r[3].s64 = ctx.r[11].s64 + 7224;
	// 824252AC: 4BFFFFDC  b 0x82425288
	pc = 0x82425288; continue 'dispatch;
	// 824252B0: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 824252B4: 409A000C  bne cr6, 0x824252c0
	if !ctx.cr[6].eq {
	pc = 0x824252C0; continue 'dispatch;
	}
	// 824252B8: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824252BC: 4800002C  b 0x824252e8
	pc = 0x824252E8; continue 'dispatch;
	// 824252C0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824252C4: 419A0020  beq cr6, 0x824252e4
	if ctx.cr[6].eq {
	pc = 0x824252E4; continue 'dispatch;
	}
	// 824252C8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824252CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824252D0: 41820014  beq 0x824252e4
	if ctx.cr[0].eq {
	pc = 0x824252E4; continue 'dispatch;
	}
	// 824252D4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824252D8: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 824252DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824252E0: 4E800421  bctrl
	ctx.lr = 0x824252E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824252E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824252E8: 480085D1  bl 0x8242d8b8
	ctx.lr = 0x824252EC;
	sub_8242D8B8(ctx, base);
	// 824252EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824252F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824252F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824252F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824252FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82425300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425308 size=248
    let mut pc: u32 = 0x82425308;
    'dispatch: loop {
        match pc {
            0x82425308 => {
    //   block [0x82425308..0x82425400)
	// 82425308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242530C: 4810FDAD  bl 0x825350b8
	ctx.lr = 0x82425310;
	sub_82535080(ctx, base);
	// 82425310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425318: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8242531C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82425320: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82425324: 48008555  bl 0x8242d878
	ctx.lr = 0x82425328;
	sub_8242D878(ctx, base);
	// 82425328: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242532C: 409A001C  bne cr6, 0x82425348
	if !ctx.cr[6].eq {
	pc = 0x82425348; continue 'dispatch;
	}
	// 82425330: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425334: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82425338: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242533C: 386B1C5C  addi r3, r11, 0x1c5c
	ctx.r[3].s64 = ctx.r[11].s64 + 7260;
	// 82425340: 48005C09  bl 0x8242af48
	ctx.lr = 0x82425344;
	sub_8242AF48(ctx, base);
	// 82425344: 480000B0  b 0x824253f4
	pc = 0x824253F4; continue 'dispatch;
	// 82425348: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242534C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425350: 409A0018  bne cr6, 0x82425368
	if !ctx.cr[6].eq {
	pc = 0x82425368; continue 'dispatch;
	}
	// 82425354: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425358: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 8242535C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425360: 386B1C50  addi r3, r11, 0x1c50
	ctx.r[3].s64 = ctx.r[11].s64 + 7248;
	// 82425364: 4BFFFFDC  b 0x82425340
	pc = 0x82425340; continue 'dispatch;
	// 82425368: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8242536C: 409A0014  bne cr6, 0x82425380
	if !ctx.cr[6].eq {
	pc = 0x82425380; continue 'dispatch;
	}
	// 82425370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82425374: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82425378: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242537C: 48000078  b 0x824253f4
	pc = 0x824253F4; continue 'dispatch;
	// 82425380: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82425384: 409A0048  bne cr6, 0x824253cc
	if !ctx.cr[6].eq {
	pc = 0x824253CC; continue 'dispatch;
	}
	// 82425388: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242538C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82425390: 41980008  blt cr6, 0x82425398
	if ctx.cr[6].lt {
	pc = 0x82425398; continue 'dispatch;
	}
	// 82425394: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82425398: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242539C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824253A0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824253A4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824253A8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824253AC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824253B0: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824253B4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824253B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824253BC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824253C0: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824253C4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824253C8: 4800002C  b 0x824253f4
	pc = 0x824253F4; continue 'dispatch;
	// 824253CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824253D0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824253D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824253D8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824253DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824253E0: 41820014  beq 0x824253f4
	if ctx.cr[0].eq {
	pc = 0x824253F4; continue 'dispatch;
	}
	// 824253E4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824253E8: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 824253EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824253F0: 4E800421  bctrl
	ctx.lr = 0x824253F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824253F4: 480084C5  bl 0x8242d8b8
	ctx.lr = 0x824253F8;
	sub_8242D8B8(ctx, base);
	// 824253F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824253FC: 4810FD0C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425400 size=184
    let mut pc: u32 = 0x82425400;
    'dispatch: loop {
        match pc {
            0x82425400 => {
    //   block [0x82425400..0x824254B8)
	// 82425400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425404: 4810FCB9  bl 0x825350bc
	ctx.lr = 0x82425408;
	sub_82535080(ctx, base);
	// 82425408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242540C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82425410: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82425414: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82425418: 48008461  bl 0x8242d878
	ctx.lr = 0x8242541C;
	sub_8242D878(ctx, base);
	// 8242541C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82425420: 409A001C  bne cr6, 0x8242543c
	if !ctx.cr[6].eq {
	pc = 0x8242543C; continue 'dispatch;
	}
	// 82425424: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425428: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 8242542C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425430: 386B1C74  addi r3, r11, 0x1c74
	ctx.r[3].s64 = ctx.r[11].s64 + 7284;
	// 82425434: 48005B15  bl 0x8242af48
	ctx.lr = 0x82425438;
	sub_8242AF48(ctx, base);
	// 82425438: 48000074  b 0x824254ac
	pc = 0x824254AC; continue 'dispatch;
	// 8242543C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425440: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425444: 409A0018  bne cr6, 0x8242545c
	if !ctx.cr[6].eq {
	pc = 0x8242545C; continue 'dispatch;
	}
	// 82425448: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242544C: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82425450: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425454: 386B1C68  addi r3, r11, 0x1c68
	ctx.r[3].s64 = ctx.r[11].s64 + 7272;
	// 82425458: 4BFFFFDC  b 0x82425434
	pc = 0x82425434; continue 'dispatch;
	// 8242545C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425460: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425464: 40990048  ble cr6, 0x824254ac
	if !ctx.cr[6].gt {
	pc = 0x824254AC; continue 'dispatch;
	}
	// 82425468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242546C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82425470: 419A003C  beq cr6, 0x824254ac
	if ctx.cr[6].eq {
	pc = 0x824254AC; continue 'dispatch;
	}
	// 82425474: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82425478: 419A0034  beq cr6, 0x824254ac
	if ctx.cr[6].eq {
	pc = 0x824254AC; continue 'dispatch;
	}
	// 8242547C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82425480: 419A002C  beq cr6, 0x824254ac
	if ctx.cr[6].eq {
	pc = 0x824254AC; continue 'dispatch;
	}
	// 82425484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82425488: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242548C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82425490: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82425494: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82425498: 41820014  beq 0x824254ac
	if ctx.cr[0].eq {
	pc = 0x824254AC; continue 'dispatch;
	}
	// 8242549C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 824254A0: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 824254A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824254A8: 4E800421  bctrl
	ctx.lr = 0x824254AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824254AC: 4800840D  bl 0x8242d8b8
	ctx.lr = 0x824254B0;
	sub_8242D8B8(ctx, base);
	// 824254B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824254B4: 4810FC58  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824254B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824254B8 size=56
    let mut pc: u32 = 0x824254B8;
    'dispatch: loop {
        match pc {
            0x824254B8 => {
    //   block [0x824254B8..0x824254F0)
	// 824254B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824254BC: 4810FC01  bl 0x825350bc
	ctx.lr = 0x824254C0;
	sub_82535080(ctx, base);
	// 824254C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824254C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824254C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824254CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824254D0: 480083A9  bl 0x8242d878
	ctx.lr = 0x824254D4;
	sub_8242D878(ctx, base);
	// 824254D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824254D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824254DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824254E0: 4BFFF9B1  bl 0x82424e90
	ctx.lr = 0x824254E4;
	sub_82424E90(ctx, base);
	// 824254E4: 480083D5  bl 0x8242d8b8
	ctx.lr = 0x824254E8;
	sub_8242D8B8(ctx, base);
	// 824254E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824254EC: 4810FC20  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824254F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824254F0 size=200
    let mut pc: u32 = 0x824254F0;
    'dispatch: loop {
        match pc {
            0x824254F0 => {
    //   block [0x824254F0..0x824255B8)
	// 824254F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824254F4: 4810FBC5  bl 0x825350b8
	ctx.lr = 0x824254F8;
	sub_82535080(ctx, base);
	// 824254F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824254FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82425504: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82425508: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8242550C: 4800836D  bl 0x8242d878
	ctx.lr = 0x82425510;
	sub_8242D878(ctx, base);
	// 82425510: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82425514: 409A0020  bne cr6, 0x82425534
	if !ctx.cr[6].eq {
	pc = 0x82425534; continue 'dispatch;
	}
	// 82425518: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242551C: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 82425520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425524: 386B1CA4  addi r3, r11, 0x1ca4
	ctx.r[3].s64 = ctx.r[11].s64 + 7332;
	// 82425528: 48005A21  bl 0x8242af48
	ctx.lr = 0x8242552C;
	sub_8242AF48(ctx, base);
	// 8242552C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82425530: 48000078  b 0x824255a8
	pc = 0x824255A8; continue 'dispatch;
	// 82425534: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425538: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242553C: 409A0018  bne cr6, 0x82425554
	if !ctx.cr[6].eq {
	pc = 0x82425554; continue 'dispatch;
	}
	// 82425540: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425544: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82425548: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242554C: 386B1C98  addi r3, r11, 0x1c98
	ctx.r[3].s64 = ctx.r[11].s64 + 7320;
	// 82425550: 4BFFFFD8  b 0x82425528
	pc = 0x82425528; continue 'dispatch;
	// 82425554: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82425558: 419A0040  beq cr6, 0x82425598
	if ctx.cr[6].eq {
	pc = 0x82425598; continue 'dispatch;
	}
	// 8242555C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82425560: 409A0018  bne cr6, 0x82425578
	if !ctx.cr[6].eq {
	pc = 0x82425578; continue 'dispatch;
	}
	// 82425564: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82425568: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8242556C: 4198002C  blt cr6, 0x82425598
	if ctx.cr[6].lt {
	pc = 0x82425598; continue 'dispatch;
	}
	// 82425570: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82425574: 48000024  b 0x82425598
	pc = 0x82425598; continue 'dispatch;
	// 82425578: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242557C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82425580: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82425584: 41820014  beq 0x82425598
	if ctx.cr[0].eq {
	pc = 0x82425598; continue 'dispatch;
	}
	// 82425588: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242558C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 82425590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425594: 4E800421  bctrl
	ctx.lr = 0x82425598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82425598: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8242559C: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824255A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824255A4: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824255A8: 48008311  bl 0x8242d8b8
	ctx.lr = 0x824255AC;
	sub_8242D8B8(ctx, base);
	// 824255AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824255B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824255B4: 4810FB54  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824255B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824255B8 size=124
    let mut pc: u32 = 0x824255B8;
    'dispatch: loop {
        match pc {
            0x824255B8 => {
    //   block [0x824255B8..0x82425634)
	// 824255B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824255BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824255C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824255C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824255C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824255CC: 480082AD  bl 0x8242d878
	ctx.lr = 0x824255D0;
	sub_8242D878(ctx, base);
	// 824255D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824255D4: 409A0020  bne cr6, 0x824255f4
	if !ctx.cr[6].eq {
	pc = 0x824255F4; continue 'dispatch;
	}
	// 824255D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824255DC: 388B185C  addi r4, r11, 0x185c
	ctx.r[4].s64 = ctx.r[11].s64 + 6236;
	// 824255E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824255E4: 386B1CBC  addi r3, r11, 0x1cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 7356;
	// 824255E8: 48005961  bl 0x8242af48
	ctx.lr = 0x824255EC;
	sub_8242AF48(ctx, base);
	// 824255EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824255F0: 48000028  b 0x82425618
	pc = 0x82425618; continue 'dispatch;
	// 824255F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824255F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824255FC: 409A0018  bne cr6, 0x82425614
	if !ctx.cr[6].eq {
	pc = 0x82425614; continue 'dispatch;
	}
	// 82425600: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425604: 388B1884  addi r4, r11, 0x1884
	ctx.r[4].s64 = ctx.r[11].s64 + 6276;
	// 82425608: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242560C: 386B1CB0  addi r3, r11, 0x1cb0
	ctx.r[3].s64 = ctx.r[11].s64 + 7344;
	// 82425610: 4BFFFFD8  b 0x824255e8
	pc = 0x824255E8; continue 'dispatch;
	// 82425614: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82425618: 480082A1  bl 0x8242d8b8
	ctx.lr = 0x8242561C;
	sub_8242D8B8(ctx, base);
	// 8242561C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82425624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242562C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425638 size=184
    let mut pc: u32 = 0x82425638;
    'dispatch: loop {
        match pc {
            0x82425638 => {
    //   block [0x82425638..0x824256F0)
	// 82425638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242563C: 4810FA81  bl 0x825350bc
	ctx.lr = 0x82425640;
	sub_82535080(ctx, base);
	// 82425640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425644: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82425648: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8242564C: 4800822D  bl 0x8242d878
	ctx.lr = 0x82425650;
	sub_8242D878(ctx, base);
	// 82425650: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82425654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82425658: 396B2300  addi r11, r11, 0x2300
	ctx.r[11].s64 = ctx.r[11].s64 + 8960;
	// 8242565C: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82425660: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425664: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82425668: 419A0018  beq cr6, 0x82425680
	if ctx.cr[6].eq {
	pc = 0x82425680; continue 'dispatch;
	}
	// 8242566C: 39290024  addi r9, r9, 0x24
	ctx.r[9].s64 = ctx.r[9].s64 + 36;
	// 82425670: 390B06C4  addi r8, r11, 0x6c4
	ctx.r[8].s64 = ctx.r[11].s64 + 1732;
	// 82425674: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82425678: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8242567C: 4198FFE4  blt cr6, 0x82425660
	if ctx.cr[6].lt {
	pc = 0x82425660; continue 'dispatch;
	}
	// 82425680: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82425684: 409A000C  bne cr6, 0x82425690
	if !ctx.cr[6].eq {
	pc = 0x82425690; continue 'dispatch;
	}
	// 82425688: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8242568C: 48000054  b 0x824256e0
	pc = 0x824256E0; continue 'dispatch;
	// 82425690: 1D0A0024  mulli r8, r10, 0x24
	ctx.r[8].s64 = ctx.r[10].s64 * 36;
	// 82425694: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 82425698: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8242569C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824256A0: 3D008242  lis r8, -0x7dbe
	ctx.r[8].s64 = -2109603840;
	// 824256A4: 394A3CC4  addi r10, r10, 0x3cc4
	ctx.r[10].s64 = ctx.r[10].s64 + 15556;
	// 824256A8: 39291BBC  addi r9, r9, 0x1bbc
	ctx.r[9].s64 = ctx.r[9].s64 + 7100;
	// 824256AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 824256B0: 39084E80  addi r8, r8, 0x4e80
	ctx.r[8].s64 = ctx.r[8].s64 + 20096;
	// 824256B4: 93AB0014  stw r29, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 824256B8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824256BC: 93EB0018  stw r31, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 824256C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824256C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824256C8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824256CC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 824256D0: 910B001C  stw r8, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 824256D4: 916B0020  stw r11, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824256D8: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 824256DC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824256E0: 480081D9  bl 0x8242d8b8
	ctx.lr = 0x824256E4;
	sub_8242D8B8(ctx, base);
	// 824256E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824256E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824256EC: 4810FA20  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824256F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824256F0 size=112
    let mut pc: u32 = 0x824256F0;
    'dispatch: loop {
        match pc {
            0x824256F0 => {
    //   block [0x824256F0..0x82425760)
	// 824256F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824256F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824256F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824256FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425700: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425708: 396B93DC  addi r11, r11, -0x6c24
	ctx.r[11].s64 = ctx.r[11].s64 + -27684;
	// 8242570C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425710: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82425714: 419A0038  beq cr6, 0x8242574c
	if ctx.cr[6].eq {
	pc = 0x8242574C; continue 'dispatch;
	}
	// 82425718: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242571C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82425720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425724: 4E800421  bctrl
	ctx.lr = 0x82425728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82425728: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242572C: 814B97AC  lwz r10, -0x6854(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26708 as u32) ) } as u64;
	// 82425730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82425734: 409A000C  bne cr6, 0x82425740
	if !ctx.cr[6].eq {
	pc = 0x82425740; continue 'dispatch;
	}
	// 82425738: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242573C: 93EA97B0  stw r31, -0x6850(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26704 as u32), ctx.r[31].u32 ) };
	// 82425740: 814B97AC  lwz r10, -0x6854(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26708 as u32) ) } as u64;
	// 82425744: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82425748: 914B97AC  stw r10, -0x6854(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26708 as u32), ctx.r[10].u32 ) };
	// 8242574C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82425750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425760 size=8
    let mut pc: u32 = 0x82425760;
    'dispatch: loop {
        match pc {
            0x82425760 => {
    //   block [0x82425760..0x82425768)
	// 82425760: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425764: 4BFFFF8C  b 0x824256f0
	sub_824256F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425768 size=180
    let mut pc: u32 = 0x82425768;
    'dispatch: loop {
        match pc {
            0x82425768 => {
    //   block [0x82425768..0x8242581C)
	// 82425768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82425774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425778: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 8242577C: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82425780: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82425784: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82425788: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 8242578C: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82425790: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82425794: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425798: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 8242579C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824257A0: 3BEB2260  addi r31, r11, 0x2260
	ctx.r[31].s64 = ctx.r[11].s64 + 8800;
	// 824257A4: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 824257A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824257AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824257B0: 4810FA21  bl 0x825351d0
	ctx.lr = 0x824257B4;
	sub_825351D0(ctx, base);
	// 824257B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824257B8: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 824257BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824257C0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 824257C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824257C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824257CC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824257D0: 4BFFDE61  bl 0x82423630
	ctx.lr = 0x824257D4;
	sub_82423630(ctx, base);
	// 824257D4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824257D8: 396B9640  addi r11, r11, -0x69c0
	ctx.r[11].s64 = ctx.r[11].s64 + -27072;
	// 824257DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824257E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824257E4: 419A0018  beq cr6, 0x824257fc
	if ctx.cr[6].eq {
	pc = 0x824257FC; continue 'dispatch;
	}
	// 824257E8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824257EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824257F0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824257F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824257F8: 4E800421  bctrl
	ctx.lr = 0x824257FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824257FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425800: 4BFF7409  bl 0x8241cc08
	ctx.lr = 0x82425804;
	sub_8241CC08(ctx, base);
	// 82425804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82425808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242580C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425810: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82425814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425820 size=152
    let mut pc: u32 = 0x82425820;
    'dispatch: loop {
        match pc {
            0x82425820 => {
    //   block [0x82425820..0x824258B8)
	// 82425820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242582C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425830: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82425834: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82425838: 409A0024  bne cr6, 0x8242585c
	if !ctx.cr[6].eq {
	pc = 0x8242585C; continue 'dispatch;
	}
	// 8242583C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425840: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82425844: 396B9640  addi r11, r11, -0x69c0
	ctx.r[11].s64 = ctx.r[11].s64 + -27072;
	// 82425848: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242584C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425854: 4E800421  bctrl
	ctx.lr = 0x82425858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82425858: 4800004C  b 0x824258a4
	pc = 0x824258A4; continue 'dispatch;
	// 8242585C: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82425860: 38C0007F  li r6, 0x7f
	ctx.r[6].s64 = 127;
	// 82425864: 3BEB2260  addi r31, r11, 0x2260
	ctx.r[31].s64 = ctx.r[11].s64 + 8800;
	// 82425868: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 8242586C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425870: 4BFFDCE9  bl 0x82423558
	ctx.lr = 0x82425874;
	sub_82423558(ctx, base);
	// 82425874: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425878: 396B9640  addi r11, r11, -0x69c0
	ctx.r[11].s64 = ctx.r[11].s64 + -27072;
	// 8242587C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425880: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82425884: 419A0018  beq cr6, 0x8242589c
	if ctx.cr[6].eq {
	pc = 0x8242589C; continue 'dispatch;
	}
	// 82425888: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242588C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82425890: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82425894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425898: 4E800421  bctrl
	ctx.lr = 0x8242589C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242589C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824258A0: 4BFF7369  bl 0x8241cc08
	ctx.lr = 0x824258A4;
	sub_8241CC08(ctx, base);
	// 824258A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824258A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824258AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824258B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824258B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824258B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824258B8 size=20
    let mut pc: u32 = 0x824258B8;
    'dispatch: loop {
        match pc {
            0x824258B8 => {
    //   block [0x824258B8..0x824258CC)
	// 824258B8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824258BC: 396B93DC  addi r11, r11, -0x6c24
	ctx.r[11].s64 = ctx.r[11].s64 + -27684;
	// 824258C0: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824258C4: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824258C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824258D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824258D0 size=20
    let mut pc: u32 = 0x824258D0;
    'dispatch: loop {
        match pc {
            0x824258D0 => {
    //   block [0x824258D0..0x824258E4)
	// 824258D0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824258D4: 396B93E4  addi r11, r11, -0x6c1c
	ctx.r[11].s64 = ctx.r[11].s64 + -27676;
	// 824258D8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824258DC: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824258E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824258E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824258E8 size=160
    let mut pc: u32 = 0x824258E8;
    'dispatch: loop {
        match pc {
            0x824258E8 => {
    //   block [0x824258E8..0x82425988)
	// 824258E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824258EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824258F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824258F4: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 824258F8: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 824258FC: 386B2240  addi r3, r11, 0x2240
	ctx.r[3].s64 = ctx.r[11].s64 + 8768;
	// 82425900: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82425904: 4810F8CD  bl 0x825351d0
	ctx.lr = 0x82425908;
	sub_825351D0(ctx, base);
	// 82425908: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242590C: 3CA08312  lis r5, -0x7cee
	ctx.r[5].s64 = -2095972352;
	// 82425910: 394B93D8  addi r10, r11, -0x6c28
	ctx.r[10].s64 = ctx.r[11].s64 + -27688;
	// 82425914: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82425918: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 8242591C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82425920: 38EA0260  addi r7, r10, 0x260
	ctx.r[7].s64 = ctx.r[10].s64 + 608;
	// 82425924: 38CA0014  addi r6, r10, 0x14
	ctx.r[6].s64 = ctx.r[10].s64 + 20;
	// 82425928: 38A522E0  addi r5, r5, 0x22e0
	ctx.r[5].s64 = ctx.r[5].s64 + 8928;
	// 8242592C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82425930: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82425934: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82425938: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242593C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82425940: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82425944: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82425948: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8242594C: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82425950: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82425954: 39250018  addi r9, r5, 0x18
	ctx.r[9].s64 = ctx.r[5].s64 + 24;
	// 82425958: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 8242595C: 7F044800  cmpw cr6, r4, r9
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82425960: 4198FFF0  blt cr6, 0x82425950
	if ctx.cr[6].lt {
	pc = 0x82425950; continue 'dispatch;
	}
	// 82425964: 386A0270  addi r3, r10, 0x270
	ctx.r[3].s64 = ctx.r[10].s64 + 624;
	// 82425968: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242596C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82425970: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82425974: 4810F85D  bl 0x825351d0
	ctx.lr = 0x82425978;
	sub_825351D0(ctx, base);
	// 82425978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242597C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425988 size=128
    let mut pc: u32 = 0x82425988;
    'dispatch: loop {
        match pc {
            0x82425988 => {
    //   block [0x82425988..0x82425A08)
	// 82425988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242598C: 4810F72D  bl 0x825350b8
	ctx.lr = 0x82425990;
	sub_82535080(ctx, base);
	// 82425990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425994: 3F80828A  lis r28, -0x7d76
	ctx.r[28].s64 = -2104885248;
	// 82425998: 817C97A8  lwz r11, -0x6858(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26712 as u32) ) } as u64;
	// 8242599C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824259A0: 409A0044  bne cr6, 0x824259e4
	if !ctx.cr[6].eq {
	pc = 0x824259E4; continue 'dispatch;
	}
	// 824259A4: 4BFFFF45  bl 0x824258e8
	ctx.lr = 0x824259A8;
	sub_824258E8(ctx, base);
	// 824259A8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824259AC: 3BAB96A8  addi r29, r11, -0x6958
	ctx.r[29].s64 = ctx.r[11].s64 + -26968;
	// 824259B0: 3BDDFFA0  addi r30, r29, -0x60
	ctx.r[30].s64 = ctx.r[29].s64 + -96;
	// 824259B4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 824259B8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 824259BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824259C0: 48007499  bl 0x8242ce58
	ctx.lr = 0x824259C4;
	sub_8242CE58(ctx, base);
	// 824259C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824259C8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824259CC: 4182002C  beq 0x824259f8
	if ctx.cr[0].eq {
	pc = 0x824259F8; continue 'dispatch;
	}
	// 824259D0: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 824259D4: 397D0100  addi r11, r29, 0x100
	ctx.r[11].s64 = ctx.r[29].s64 + 256;
	// 824259D8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824259DC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824259E0: 4198FFD8  blt cr6, 0x824259b8
	if ctx.cr[6].lt {
	pc = 0x824259B8; continue 'dispatch;
	}
	// 824259E4: 817C97A8  lwz r11, -0x6858(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-26712 as u32) ) } as u64;
	// 824259E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824259EC: 917C97A8  stw r11, -0x6858(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-26712 as u32), ctx.r[11].u32 ) };
	// 824259F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824259F4: 4810F714  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824259F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824259FC: 386B1D00  addi r3, r11, 0x1d00
	ctx.r[3].s64 = ctx.r[11].s64 + 7424;
	// 82425A00: 4BFFFE21  bl 0x82425820
	ctx.lr = 0x82425A04;
	sub_82425820(ctx, base);
	// 82425A04: 4BFFFFEC  b 0x824259f0
	pc = 0x824259F0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425A08 size=128
    let mut pc: u32 = 0x82425A08;
    'dispatch: loop {
        match pc {
            0x82425A08 => {
    //   block [0x82425A08..0x82425A88)
	// 82425A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82425A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425A1C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425A20: 814B97A8  lwz r10, -0x6858(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26712 as u32) ) } as u64;
	// 82425A24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82425A28: 914B97A8  stw r10, -0x6858(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26712 as u32), ctx.r[10].u32 ) };
	// 82425A2C: 816B97A8  lwz r11, -0x6858(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26712 as u32) ) } as u64;
	// 82425A30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425A34: 409A003C  bne cr6, 0x82425a70
	if !ctx.cr[6].eq {
	pc = 0x82425A70; continue 'dispatch;
	}
	// 82425A38: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425A3C: 3BCB9640  addi r30, r11, -0x69c0
	ctx.r[30].s64 = ctx.r[11].s64 + -27072;
	// 82425A40: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82425A44: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425A48: 480074F1  bl 0x8242cf38
	ctx.lr = 0x82425A4C;
	sub_8242CF38(ctx, base);
	// 82425A4C: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82425A50: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82425A54: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82425A58: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82425A5C: 4198FFE8  blt cr6, 0x82425a44
	if ctx.cr[6].lt {
	pc = 0x82425A44; continue 'dispatch;
	}
	// 82425A60: 4BFFFE89  bl 0x824258e8
	ctx.lr = 0x82425A64;
	sub_824258E8(ctx, base);
	// 82425A64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82425A68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82425A6C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82425A70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82425A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425A7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82425A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425A88 size=24
    let mut pc: u32 = 0x82425A88;
    'dispatch: loop {
        match pc {
            0x82425A88 => {
    //   block [0x82425A88..0x82425AA0)
	// 82425A88: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425A8C: 814B93D8  lwz r10, -0x6c28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27688 as u32) ) } as u64;
	// 82425A90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82425A94: 419A000C  beq cr6, 0x82425aa0
	if ctx.cr[6].eq {
		sub_82425AA0(ctx, base);
		return;
	}
	// 82425A98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82425A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425AA0 size=12
    let mut pc: u32 = 0x82425AA0;
    'dispatch: loop {
        match pc {
            0x82425AA0 => {
    //   block [0x82425AA0..0x82425AAC)
	// 82425AA0: 906B93D8  stw r3, -0x6c28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27688 as u32), ctx.r[3].u32 ) };
	// 82425AA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425AB0 size=108
    let mut pc: u32 = 0x82425AB0;
    'dispatch: loop {
        match pc {
            0x82425AB0 => {
    //   block [0x82425AB0..0x82425B1C)
	// 82425AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425AC4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82425AC8: 4198000C  blt cr6, 0x82425ad4
	if ctx.cr[6].lt {
	pc = 0x82425AD4; continue 'dispatch;
	}
	// 82425ACC: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 82425AD0: 41980010  blt cr6, 0x82425ae0
	if ctx.cr[6].lt {
	pc = 0x82425AE0; continue 'dispatch;
	}
	// 82425AD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425AD8: 386B1D98  addi r3, r11, 0x1d98
	ctx.r[3].s64 = ctx.r[11].s64 + 7576;
	// 82425ADC: 4BFFFD45  bl 0x82425820
	ctx.lr = 0x82425AE0;
	sub_82425820(ctx, base);
	// 82425AE0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425AE4: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82425AE8: 396B9648  addi r11, r11, -0x69b8
	ctx.r[11].s64 = ctx.r[11].s64 + -27064;
	// 82425AEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82425AF0: 480074D9  bl 0x8242cfc8
	ctx.lr = 0x82425AF4;
	sub_8242CFC8(ctx, base);
	// 82425AF4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82425AF8: 40800010  bge 0x82425b08
	if !ctx.cr[0].lt {
	pc = 0x82425B08; continue 'dispatch;
	}
	// 82425AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425B00: 386B1D38  addi r3, r11, 0x1d38
	ctx.r[3].s64 = ctx.r[11].s64 + 7480;
	// 82425B04: 4BFF7105  bl 0x8241cc08
	ctx.lr = 0x82425B08;
	sub_8241CC08(ctx, base);
	// 82425B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82425B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425B20 size=108
    let mut pc: u32 = 0x82425B20;
    'dispatch: loop {
        match pc {
            0x82425B20 => {
    //   block [0x82425B20..0x82425B8C)
	// 82425B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425B30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425B34: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82425B38: 4198000C  blt cr6, 0x82425b44
	if ctx.cr[6].lt {
	pc = 0x82425B44; continue 'dispatch;
	}
	// 82425B3C: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 82425B40: 41980010  blt cr6, 0x82425b50
	if ctx.cr[6].lt {
	pc = 0x82425B50; continue 'dispatch;
	}
	// 82425B44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425B48: 386B1E48  addi r3, r11, 0x1e48
	ctx.r[3].s64 = ctx.r[11].s64 + 7752;
	// 82425B4C: 4BFFFCD5  bl 0x82425820
	ctx.lr = 0x82425B50;
	sub_82425820(ctx, base);
	// 82425B50: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425B54: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82425B58: 396B9648  addi r11, r11, -0x69b8
	ctx.r[11].s64 = ctx.r[11].s64 + -27064;
	// 82425B5C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82425B60: 48007501  bl 0x8242d060
	ctx.lr = 0x82425B64;
	sub_8242D060(ctx, base);
	// 82425B64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82425B68: 40800010  bge 0x82425b78
	if !ctx.cr[0].lt {
	pc = 0x82425B78; continue 'dispatch;
	}
	// 82425B6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425B70: 386B1DE8  addi r3, r11, 0x1de8
	ctx.r[3].s64 = ctx.r[11].s64 + 7656;
	// 82425B74: 4BFF7095  bl 0x8241cc08
	ctx.lr = 0x82425B78;
	sub_8241CC08(ctx, base);
	// 82425B78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82425B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425B90 size=152
    let mut pc: u32 = 0x82425B90;
    'dispatch: loop {
        match pc {
            0x82425B90 => {
    //   block [0x82425B90..0x82425C28)
	// 82425B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82425B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425BA4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425BA8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82425BAC: 3BCB93E4  addi r30, r11, -0x6c1c
	ctx.r[30].s64 = ctx.r[11].s64 + -27676;
	// 82425BB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425BB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82425BB8: 419A0058  beq cr6, 0x82425c10
	if ctx.cr[6].eq {
	pc = 0x82425C10; continue 'dispatch;
	}
	// 82425BBC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425BC0: 814B97AC  lwz r10, -0x6854(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26708 as u32) ) } as u64;
	// 82425BC4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82425BC8: 914B97AC  stw r10, -0x6854(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26708 as u32), ctx.r[10].u32 ) };
	// 82425BCC: 816B97AC  lwz r11, -0x6854(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26708 as u32) ) } as u64;
	// 82425BD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82425BD4: 409A002C  bne cr6, 0x82425c00
	if !ctx.cr[6].eq {
	pc = 0x82425C00; continue 'dispatch;
	}
	// 82425BD8: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 82425BDC: 817F97B0  lwz r11, -0x6850(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26704 as u32) ) } as u64;
	// 82425BE0: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82425BE4: 419A0014  beq cr6, 0x82425bf8
	if ctx.cr[6].eq {
	pc = 0x82425BF8; continue 'dispatch;
	}
	// 82425BE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425BEC: 809F97B0  lwz r4, -0x6850(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26704 as u32) ) } as u64;
	// 82425BF0: 386B1E98  addi r3, r11, 0x1e98
	ctx.r[3].s64 = ctx.r[11].s64 + 7832;
	// 82425BF4: 4BFFFB75  bl 0x82425768
	ctx.lr = 0x82425BF8;
	sub_82425768(ctx, base);
	// 82425BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82425BFC: 917F97B0  stw r11, -0x6850(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26704 as u32), ctx.r[11].u32 ) };
	// 82425C00: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425C04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425C08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425C0C: 4E800421  bctrl
	ctx.lr = 0x82425C10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82425C10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82425C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425C1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82425C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425C28 size=8
    let mut pc: u32 = 0x82425C28;
    'dispatch: loop {
        match pc {
            0x82425C28 => {
    //   block [0x82425C28..0x82425C30)
	// 82425C28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425C2C: 4BFFFF64  b 0x82425b90
	sub_82425B90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425C30 size=72
    let mut pc: u32 = 0x82425C30;
    'dispatch: loop {
        match pc {
            0x82425C30 => {
    //   block [0x82425C30..0x82425C78)
	// 82425C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425C34: 4810F489  bl 0x825350bc
	ctx.lr = 0x82425C38;
	sub_82535080(ctx, base);
	// 82425C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425C40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425C44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82425C48: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82425C4C: 4BFFFAA5  bl 0x824256f0
	ctx.lr = 0x82425C50;
	sub_824256F0(ctx, base);
	// 82425C50: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82425C54: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82425C58: 394A9668  addi r10, r10, -0x6998
	ctx.r[10].s64 = ctx.r[10].s64 + -27032;
	// 82425C5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425C60: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82425C64: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82425C68: 7FAB492E  stwx r29, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u32) };
	// 82425C6C: 4BFFFF25  bl 0x82425b90
	ctx.lr = 0x82425C70;
	sub_82425B90(ctx, base);
	// 82425C70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82425C74: 4810F498  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425C78 size=156
    let mut pc: u32 = 0x82425C78;
    'dispatch: loop {
        match pc {
            0x82425C78 => {
    //   block [0x82425C78..0x82425D14)
	// 82425C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425C7C: 4810F435  bl 0x825350b0
	ctx.lr = 0x82425C80;
	sub_82535080(ctx, base);
	// 82425C80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425C88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82425C8C: 4BFFFE25  bl 0x82425ab0
	ctx.lr = 0x82425C90;
	sub_82425AB0(ctx, base);
	// 82425C90: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425C94: 1D5F0048  mulli r10, r31, 0x48
	ctx.r[10].s64 = ctx.r[31].s64 * 72;
	// 82425C98: 396B93F8  addi r11, r11, -0x6c08
	ctx.r[11].s64 = ctx.r[11].s64 + -27656;
	// 82425C9C: 3B400006  li r26, 6
	ctx.r[26].s64 = 6;
	// 82425CA0: 7F6A5A14  add r27, r10, r11
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82425CA4: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82425CA8: 3B8B2240  addi r28, r11, 0x2240
	ctx.r[28].s64 = ctx.r[11].s64 + 8768;
	// 82425CAC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425CB0: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82425CB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82425CB8: 41820024  beq 0x82425cdc
	if ctx.cr[0].eq {
	pc = 0x82425CDC; continue 'dispatch;
	}
	// 82425CBC: 57FD103A  slwi r29, r31, 2
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82425CC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82425CC4: 7D5DE12E  stwx r10, r29, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82425CC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425CCC: 4E800421  bctrl
	ctx.lr = 0x82425CD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82425CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82425CD4: 7C7EF378  or r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 82425CD8: 7D7DE12E  stwx r11, r29, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 82425CDC: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82425CE0: 3B7B000C  addi r27, r27, 0xc
	ctx.r[27].s64 = ctx.r[27].s64 + 12;
	// 82425CE4: 4082FFC8  bne 0x82425cac
	if !ctx.cr[0].eq {
	pc = 0x82425CAC; continue 'dispatch;
	}
	// 82425CE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425CEC: 4BFFFE35  bl 0x82425b20
	ctx.lr = 0x82425CF0;
	sub_82425B20(ctx, base);
	// 82425CF0: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82425CF4: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82425CF8: 396B22E0  addi r11, r11, 0x22e0
	ctx.r[11].s64 = ctx.r[11].s64 + 8928;
	// 82425CFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82425D00: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82425D04: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82425D08: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82425D0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82425D10: 4810F3F0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D18 size=8
    let mut pc: u32 = 0x82425D18;
    'dispatch: loop {
        match pc {
            0x82425D18 => {
    //   block [0x82425D18..0x82425D20)
	// 82425D18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82425D1C: 4BFFFF5C  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D20 size=8
    let mut pc: u32 = 0x82425D20;
    'dispatch: loop {
        match pc {
            0x82425D20 => {
    //   block [0x82425D20..0x82425D28)
	// 82425D20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425D24: 4BFFFF54  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D28 size=8
    let mut pc: u32 = 0x82425D28;
    'dispatch: loop {
        match pc {
            0x82425D28 => {
    //   block [0x82425D28..0x82425D30)
	// 82425D28: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82425D2C: 4BFFFF4C  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D30 size=8
    let mut pc: u32 = 0x82425D30;
    'dispatch: loop {
        match pc {
            0x82425D30 => {
    //   block [0x82425D30..0x82425D38)
	// 82425D30: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82425D34: 4BFFFF44  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D38 size=8
    let mut pc: u32 = 0x82425D38;
    'dispatch: loop {
        match pc {
            0x82425D38 => {
    //   block [0x82425D38..0x82425D40)
	// 82425D38: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82425D3C: 4BFFFF3C  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D40 size=8
    let mut pc: u32 = 0x82425D40;
    'dispatch: loop {
        match pc {
            0x82425D40 => {
    //   block [0x82425D40..0x82425D48)
	// 82425D40: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82425D44: 4BFFFF34  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D48 size=8
    let mut pc: u32 = 0x82425D48;
    'dispatch: loop {
        match pc {
            0x82425D48 => {
    //   block [0x82425D48..0x82425D50)
	// 82425D48: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82425D4C: 4BFFFF2C  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82425D50 size=8
    let mut pc: u32 = 0x82425D50;
    'dispatch: loop {
        match pc {
            0x82425D50 => {
    //   block [0x82425D50..0x82425D58)
	// 82425D50: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82425D54: 4BFFFF24  b 0x82425c78
	sub_82425C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425D58 size=124
    let mut pc: u32 = 0x82425D58;
    'dispatch: loop {
        match pc {
            0x82425D58 => {
    //   block [0x82425D58..0x82425DD4)
	// 82425D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425D68: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425D70: 814B93D8  lwz r10, -0x6c28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27688 as u32) ) } as u64;
	// 82425D74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82425D78: 419A0018  beq cr6, 0x82425d90
	if ctx.cr[6].eq {
	pc = 0x82425D90; continue 'dispatch;
	}
	// 82425D7C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82425D80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82425D84: 4E800421  bctrl
	ctx.lr = 0x82425D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82425D88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425D8C: 48000030  b 0x82425dbc
	pc = 0x82425DBC; continue 'dispatch;
	// 82425D90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425D94: 4BFFF95D  bl 0x824256f0
	ctx.lr = 0x82425D98;
	sub_824256F0(ctx, base);
	// 82425D98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82425DA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82425DA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425DA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82425DAC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82425DB0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82425DB4: 697F0001  xori r31, r11, 1
	ctx.r[31].u64 = ctx.r[11].u64 ^ 1;
	// 82425DB8: 4BFFFDD9  bl 0x82425b90
	ctx.lr = 0x82425DBC;
	sub_82425B90(ctx, base);
	// 82425DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82425DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425DD8 size=212
    let mut pc: u32 = 0x82425DD8;
    'dispatch: loop {
        match pc {
            0x82425DD8 => {
    //   block [0x82425DD8..0x82425EAC)
	// 82425DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425DDC: 4810F2D9  bl 0x825350b4
	ctx.lr = 0x82425DE0;
	sub_82535080(ctx, base);
	// 82425DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425DE8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425DEC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82425DF0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82425DF4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82425DF8: 4BFFF8F9  bl 0x824256f0
	ctx.lr = 0x82425DFC;
	sub_824256F0(ctx, base);
	// 82425DFC: 2B1F0007  cmplwi cr6, r31, 7
	ctx.cr[6].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	// 82425E00: 41990088  bgt cr6, 0x82425e88
	if ctx.cr[6].gt {
	pc = 0x82425E88; continue 'dispatch;
	}
	// 82425E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425E08: 4BFFFCA9  bl 0x82425ab0
	ctx.lr = 0x82425E0C;
	sub_82425AB0(ctx, base);
	// 82425E0C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425E10: 1D5F0048  mulli r10, r31, 0x48
	ctx.r[10].s64 = ctx.r[31].s64 * 72;
	// 82425E14: 396B93F8  addi r11, r11, -0x6c08
	ctx.r[11].s64 = ctx.r[11].s64 + -27656;
	// 82425E18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82425E1C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82425E20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425E24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82425E28: 419A0018  beq cr6, 0x82425e40
	if ctx.cr[6].eq {
	pc = 0x82425E40; continue 'dispatch;
	}
	// 82425E2C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82425E30: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82425E34: 2F1E0006  cmpwi cr6, r30, 6
	ctx.cr[6].compare_i32(ctx.r[30].s32, 6, &mut ctx.xer);
	// 82425E38: 4198FFE8  blt cr6, 0x82425e20
	if ctx.cr[6].lt {
	pc = 0x82425E20; continue 'dispatch;
	}
	// 82425E3C: 48000028  b 0x82425e64
	pc = 0x82425E64; continue 'dispatch;
	// 82425E40: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82425E44: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82425E48: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82425E4C: 419A000C  beq cr6, 0x82425e58
	if ctx.cr[6].eq {
	pc = 0x82425E58; continue 'dispatch;
	}
	// 82425E50: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82425E54: 48000010  b 0x82425e64
	pc = 0x82425E64; continue 'dispatch;
	// 82425E58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82425E5C: 394A1F38  addi r10, r10, 0x1f38
	ctx.r[10].s64 = ctx.r[10].s64 + 7992;
	// 82425E60: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82425E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425E68: 4BFFFCB9  bl 0x82425b20
	ctx.lr = 0x82425E6C;
	sub_82425B20(ctx, base);
	// 82425E6C: 2F1E0006  cmpwi cr6, r30, 6
	ctx.cr[6].compare_i32(ctx.r[30].s32, 6, &mut ctx.xer);
	// 82425E70: 409A0010  bne cr6, 0x82425e80
	if !ctx.cr[6].eq {
	pc = 0x82425E80; continue 'dispatch;
	}
	// 82425E74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425E78: 386B1F08  addi r3, r11, 0x1f08
	ctx.r[3].s64 = ctx.r[11].s64 + 7944;
	// 82425E7C: 48000014  b 0x82425e90
	pc = 0x82425E90; continue 'dispatch;
	// 82425E80: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82425E84: 48000014  b 0x82425e98
	pc = 0x82425E98; continue 'dispatch;
	// 82425E88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425E8C: 386B1EE0  addi r3, r11, 0x1ee0
	ctx.r[3].s64 = ctx.r[11].s64 + 7904;
	// 82425E90: 4BFFF991  bl 0x82425820
	ctx.lr = 0x82425E94;
	sub_82425820(ctx, base);
	// 82425E94: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82425E98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425E9C: 4BFFFCF5  bl 0x82425b90
	ctx.lr = 0x82425EA0;
	sub_82425B90(ctx, base);
	// 82425EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82425EA8: 4810F25C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425EB0 size=172
    let mut pc: u32 = 0x82425EB0;
    'dispatch: loop {
        match pc {
            0x82425EB0 => {
    //   block [0x82425EB0..0x82425F5C)
	// 82425EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82425EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82425EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82425EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82425EC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425ECC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82425ED0: 4BFFF821  bl 0x824256f0
	ctx.lr = 0x82425ED4;
	sub_824256F0(ctx, base);
	// 82425ED4: 2B1E0005  cmplwi cr6, r30, 5
	ctx.cr[6].compare_u32(ctx.r[30].u32, 5 as u32, &mut ctx.xer);
	// 82425ED8: 41990058  bgt cr6, 0x82425f30
	if ctx.cr[6].gt {
	pc = 0x82425F30; continue 'dispatch;
	}
	// 82425EDC: 2B1F0007  cmplwi cr6, r31, 7
	ctx.cr[6].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	// 82425EE0: 41990044  bgt cr6, 0x82425f24
	if ctx.cr[6].gt {
	pc = 0x82425F24; continue 'dispatch;
	}
	// 82425EE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425EE8: 4BFFFBC9  bl 0x82425ab0
	ctx.lr = 0x82425EEC;
	sub_82425AB0(ctx, base);
	// 82425EEC: 1D7F0006  mulli r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 * 6;
	// 82425EF0: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82425EF4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425EF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82425EFC: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 82425F00: 396B93F8  addi r11, r11, -0x6c08
	ctx.r[11].s64 = ctx.r[11].s64 + -27656;
	// 82425F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82425F08: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82425F0C: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 82425F10: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82425F14: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82425F18: 7D2A392E  stwx r9, r10, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[9].u32) };
	// 82425F1C: 4BFFFC05  bl 0x82425b20
	ctx.lr = 0x82425F20;
	sub_82425B20(ctx, base);
	// 82425F20: 4800001C  b 0x82425f3c
	pc = 0x82425F3C; continue 'dispatch;
	// 82425F24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425F28: 386B1F60  addi r3, r11, 0x1f60
	ctx.r[3].s64 = ctx.r[11].s64 + 8032;
	// 82425F2C: 4800000C  b 0x82425f38
	pc = 0x82425F38; continue 'dispatch;
	// 82425F30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425F34: 386B1F40  addi r3, r11, 0x1f40
	ctx.r[3].s64 = ctx.r[11].s64 + 8000;
	// 82425F38: 4BFFF8E9  bl 0x82425820
	ctx.lr = 0x82425F3C;
	sub_82425820(ctx, base);
	// 82425F3C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425F40: 4BFFFC51  bl 0x82425b90
	ctx.lr = 0x82425F44;
	sub_82425B90(ctx, base);
	// 82425F44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82425F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82425F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82425F50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82425F54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82425F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82425F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82425F60 size=200
    let mut pc: u32 = 0x82425F60;
    'dispatch: loop {
        match pc {
            0x82425F60 => {
    //   block [0x82425F60..0x82426028)
	// 82425F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82425F64: 4810F151  bl 0x825350b4
	ctx.lr = 0x82425F68;
	sub_82535080(ctx, base);
	// 82425F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82425F6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82425F70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82425F74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82425F78: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82425F7C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82425F80: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82425F84: 4BFFF76D  bl 0x824256f0
	ctx.lr = 0x82425F88;
	sub_824256F0(ctx, base);
	// 82425F88: 2B1F0005  cmplwi cr6, r31, 5
	ctx.cr[6].compare_u32(ctx.r[31].u32, 5 as u32, &mut ctx.xer);
	// 82425F8C: 41990080  bgt cr6, 0x8242600c
	if ctx.cr[6].gt {
	pc = 0x8242600C; continue 'dispatch;
	}
	// 82425F90: 2B1E0007  cmplwi cr6, r30, 7
	ctx.cr[6].compare_u32(ctx.r[30].u32, 7 as u32, &mut ctx.xer);
	// 82425F94: 4199006C  bgt cr6, 0x82426000
	if ctx.cr[6].gt {
	pc = 0x82426000; continue 'dispatch;
	}
	// 82425F98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82425F9C: 4BFFFB15  bl 0x82425ab0
	ctx.lr = 0x82425FA0;
	sub_82425AB0(ctx, base);
	// 82425FA0: 1D7E0006  mulli r11, r30, 6
	ctx.r[11].s64 = ctx.r[30].s64 * 6;
	// 82425FA4: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82425FA8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82425FAC: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 82425FB0: 396B93F8  addi r11, r11, -0x6c08
	ctx.r[11].s64 = ctx.r[11].s64 + -27656;
	// 82425FB4: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82425FB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82425FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82425FC0: 419A0010  beq cr6, 0x82425fd0
	if ctx.cr[6].eq {
	pc = 0x82425FD0; continue 'dispatch;
	}
	// 82425FC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425FC8: 386B1FD4  addi r3, r11, 0x1fd4
	ctx.r[3].s64 = ctx.r[11].s64 + 8148;
	// 82425FCC: 4BFFF855  bl 0x82425820
	ctx.lr = 0x82425FD0;
	sub_82425820(ctx, base);
	// 82425FD0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82425FD4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82425FD8: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82425FDC: 419A000C  beq cr6, 0x82425fe8
	if ctx.cr[6].eq {
	pc = 0x82425FE8; continue 'dispatch;
	}
	// 82425FE0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82425FE4: 48000010  b 0x82425ff4
	pc = 0x82425FF4; continue 'dispatch;
	// 82425FE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82425FEC: 396B1F38  addi r11, r11, 0x1f38
	ctx.r[11].s64 = ctx.r[11].s64 + 7992;
	// 82425FF0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82425FF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82425FF8: 4BFFFB29  bl 0x82425b20
	ctx.lr = 0x82425FFC;
	sub_82425B20(ctx, base);
	// 82425FFC: 4800001C  b 0x82426018
	pc = 0x82426018; continue 'dispatch;
	// 82426000: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82426004: 386B1FAC  addi r3, r11, 0x1fac
	ctx.r[3].s64 = ctx.r[11].s64 + 8108;
	// 82426008: 4800000C  b 0x82426014
	pc = 0x82426014; continue 'dispatch;
	// 8242600C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82426010: 386B1F88  addi r3, r11, 0x1f88
	ctx.r[3].s64 = ctx.r[11].s64 + 8072;
	// 82426014: 4BFFF80D  bl 0x82425820
	ctx.lr = 0x82426018;
	sub_82425820(ctx, base);
	// 82426018: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8242601C: 4BFFFB75  bl 0x82425b90
	ctx.lr = 0x82426020;
	sub_82425B90(ctx, base);
	// 82426020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82426024: 4810F0E0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82426028 size=88
    let mut pc: u32 = 0x82426028;
    'dispatch: loop {
        match pc {
            0x82426028 => {
    //   block [0x82426028..0x82426080)
	// 82426028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242602C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82426030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82426034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426038: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8242603C: 817F97B4  lwz r11, -0x684c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26700 as u32) ) } as u64;
	// 82426040: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426044: 409A001C  bne cr6, 0x82426060
	if !ctx.cr[6].eq {
	pc = 0x82426060; continue 'dispatch;
	}
	// 82426048: 48007F19  bl 0x8242df60
	ctx.lr = 0x8242604C;
	sub_8242DF60(ctx, base);
	// 8242604C: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82426050: 38A01680  li r5, 0x1680
	ctx.r[5].s64 = 5760;
	// 82426054: 386B0BC0  addi r3, r11, 0xbc0
	ctx.r[3].s64 = ctx.r[11].s64 + 3008;
	// 82426058: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242605C: 4810F175  bl 0x825351d0
	ctx.lr = 0x82426060;
	sub_825351D0(ctx, base);
	// 82426060: 817F97B4  lwz r11, -0x684c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26700 as u32) ) } as u64;
	// 82426064: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82426068: 917F97B4  stw r11, -0x684c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26700 as u32), ctx.r[11].u32 ) };
	// 8242606C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82426070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82426074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82426078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242607C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426080 size=28
    let mut pc: u32 = 0x82426080;
    'dispatch: loop {
        match pc {
            0x82426080 => {
    //   block [0x82426080..0x8242609C)
	// 82426080: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82426084: 814B97B4  lwz r10, -0x684c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26700 as u32) ) } as u64;
	// 82426088: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8242608C: 914B97B4  stw r10, -0x684c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26700 as u32), ctx.r[10].u32 ) };
	// 82426090: 816B97B4  lwz r11, -0x684c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26700 as u32) ) } as u64;
	// 82426094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426098: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242609C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242609C size=20
    let mut pc: u32 = 0x8242609C;
    'dispatch: loop {
        match pc {
            0x8242609C => {
    //   block [0x8242609C..0x824260B0)
	// 8242609C: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 824260A0: 38A01680  li r5, 0x1680
	ctx.r[5].s64 = 5760;
	// 824260A4: 386B0BC0  addi r3, r11, 0xbc0
	ctx.r[3].s64 = ctx.r[11].s64 + 3008;
	// 824260A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824260AC: 4810F124  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824260B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824260B0 size=4
    let mut pc: u32 = 0x824260B0;
    'dispatch: loop {
        match pc {
            0x824260B0 => {
    //   block [0x824260B0..0x824260B4)
	// 824260B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824260B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824260B8 size=96
    let mut pc: u32 = 0x824260B8;
    'dispatch: loop {
        match pc {
            0x824260B8 => {
    //   block [0x824260B8..0x82426118)
	// 824260B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824260BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824260C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824260C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824260C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824260CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824260D0: 419A0034  beq cr6, 0x82426104
	if ctx.cr[6].eq {
	pc = 0x82426104; continue 'dispatch;
	}
	// 824260D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824260D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824260DC: 41820010  beq 0x824260ec
	if ctx.cr[0].eq {
	pc = 0x824260EC; continue 'dispatch;
	}
	// 824260E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824260E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824260E8: 48007F19  bl 0x8242e000
	ctx.lr = 0x824260EC;
	sub_8242E000(ctx, base);
	// 824260EC: 48008E65  bl 0x8242ef50
	ctx.lr = 0x824260F0;
	sub_8242EF50(ctx, base);
	// 824260F0: 38A000B4  li r5, 0xb4
	ctx.r[5].s64 = 180;
	// 824260F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824260F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824260FC: 4810F0D5  bl 0x825351d0
	ctx.lr = 0x82426100;
	sub_825351D0(ctx, base);
	// 82426100: 48008E51  bl 0x8242ef50
	ctx.lr = 0x82426104;
	sub_8242EF50(ctx, base);
	// 82426104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82426108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242610C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82426110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82426114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426118 size=16
    let mut pc: u32 = 0x82426118;
    'dispatch: loop {
        match pc {
            0x82426118 => {
    //   block [0x82426118..0x82426128)
	// 82426118: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242611C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426120: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82426124: 48008D74  b 0x8242ee98
	sub_8242EE98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426128 size=16
    let mut pc: u32 = 0x82426128;
    'dispatch: loop {
        match pc {
            0x82426128 => {
    //   block [0x82426128..0x82426138)
	// 82426128: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242612C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426130: 908B0038  stw r4, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 82426134: 48008D84  b 0x8242eeb8
	sub_8242EEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426138 size=8
    let mut pc: u32 = 0x82426138;
    'dispatch: loop {
        match pc {
            0x82426138 => {
    //   block [0x82426138..0x82426140)
	// 82426138: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242613C: 48008DEC  b 0x8242ef28
	sub_8242EF28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426140 size=72
    let mut pc: u32 = 0x82426140;
    'dispatch: loop {
        match pc {
            0x82426140 => {
    //   block [0x82426140..0x82426188)
	// 82426140: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82426144: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82426148: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8242614C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82426150: 916300A0  stw r11, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82426154: 91430038  stw r10, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82426158: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8242615C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82426160: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82426164: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82426168: 9123003C  stw r9, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 8242616C: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82426170: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82426174: 99630003  stb r11, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82426178: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 8242617C: 916300AC  stw r11, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82426180: 99430001  stb r10, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82426184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82426188 size=56
    let mut pc: u32 = 0x82426188;
    'dispatch: loop {
        match pc {
            0x82426188 => {
    //   block [0x82426188..0x824261C0)
	// 82426188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242618C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82426190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82426194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242619C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824261A0: 48008309  bl 0x8242e4a8
	ctx.lr = 0x824261A4;
	sub_8242E4A8(ctx, base);
	// 824261A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824261A8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824261AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824261B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824261B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824261B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824261BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824261C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824261C0 size=632
    let mut pc: u32 = 0x824261C0;
    'dispatch: loop {
        match pc {
            0x824261C0 => {
    //   block [0x824261C0..0x82426438)
	// 824261C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824261C4: 4810EEE9  bl 0x825350ac
	ctx.lr = 0x824261C8;
	sub_82535080(ctx, base);
	// 824261C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824261CC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824261D0: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 824261D4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824261D8: 60A5C800  ori r5, r5, 0xc800
	ctx.r[5].u64 = ctx.r[5].u64 | 51200;
	// 824261DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824261E0: 837A0008  lwz r27, 8(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824261E4: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824261E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824261EC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824261F0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824261F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824261F8: 4E800421  bctrl
	ctx.lr = 0x824261FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824261FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82426200: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426204: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426208: 40990020  ble cr6, 0x82426228
	if !ctx.cr[6].gt {
	pc = 0x82426228; continue 'dispatch;
	}
	// 8242620C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82426210: 7D4A20AE  lbzx r10, r10, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82426214: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82426218: 409A0010  bne cr6, 0x82426228
	if !ctx.cr[6].eq {
	pc = 0x82426228; continue 'dispatch;
	}
	// 8242621C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82426220: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426224: 4198FFE8  blt cr6, 0x8242620c
	if ctx.cr[6].lt {
	pc = 0x8242620C; continue 'dispatch;
	}
	// 82426228: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8242622C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82426234: 48002CD5  bl 0x82428f08
	ctx.lr = 0x82426238;
	sub_82428F08(ctx, base);
	// 82426238: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242623C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426240: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426244: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82426248: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242624C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426250: 4E800421  bctrl
	ctx.lr = 0x82426254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426254: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82426258: 2F050010  cmpwi cr6, r5, 0x10
	ctx.cr[6].compare_i32(ctx.r[5].s32, 16, &mut ctx.xer);
	// 8242625C: 419801B8  blt cr6, 0x82426414
	if ctx.cr[6].lt {
	pc = 0x82426414; continue 'dispatch;
	}
	// 82426260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82426264: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82426268: 48008A49  bl 0x8242ecb0
	ctx.lr = 0x8242626C;
	sub_8242ECB0(ctx, base);
	// 8242626C: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82426270: 418201A4  beq 0x82426414
	if ctx.cr[0].eq {
	pc = 0x82426414; continue 'dispatch;
	}
	// 82426274: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82426278: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242627C: 41990198  bgt cr6, 0x82426414
	if ctx.cr[6].gt {
	pc = 0x82426414; continue 'dispatch;
	}
	// 82426280: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82426284: 4098001C  bge cr6, 0x824262a0
	if !ctx.cr[6].lt {
	pc = 0x824262A0; continue 'dispatch;
	}
	// 82426288: A17F009A  lhz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 8242628C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82426290: 419A0118  beq cr6, 0x824263a8
	if ctx.cr[6].eq {
	pc = 0x824263A8; continue 'dispatch;
	}
	// 82426294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82426298: 48007DC9  bl 0x8242e060
	ctx.lr = 0x8242629C;
	sub_8242E060(ctx, base);
	// 8242629C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824262A0: 817A0050  lwz r11, 0x50(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(80 as u32) ) } as u64;
	// 824262A4: 933A00A0  stw r25, 0xa0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(160 as u32), ctx.r[25].u32 ) };
	// 824262A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824262AC: 419A0050  beq cr6, 0x824262fc
	if ctx.cr[6].eq {
	pc = 0x824262FC; continue 'dispatch;
	}
	// 824262B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824262B4: 48007E6D  bl 0x8242e120
	ctx.lr = 0x824262B8;
	sub_8242E120(ctx, base);
	// 824262B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824262BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824262C0: 48007E69  bl 0x8242e128
	ctx.lr = 0x824262C4;
	sub_8242E128(ctx, base);
	// 824262C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824262C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824262CC: 4BF8866D  bl 0x823ae938
	ctx.lr = 0x824262D0;
	sub_823AE938(ctx, base);
	// 824262D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824262D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824262D8: 48007F01  bl 0x8242e1d8
	ctx.lr = 0x824262DC;
	sub_8242E1D8(ctx, base);
	// 824262DC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 824262E0: 807A0054  lwz r3, 0x54(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(84 as u32) ) } as u64;
	// 824262E4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824262E8: 817A0050  lwz r11, 0x50(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(80 as u32) ) } as u64;
	// 824262EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824262F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824262F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824262F8: 4E800421  bctrl
	ctx.lr = 0x824262FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824262FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82426300: 48007E21  bl 0x8242e120
	ctx.lr = 0x82426304;
	sub_8242E120(ctx, base);
	// 82426304: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82426308: 409A000C  bne cr6, 0x82426314
	if !ctx.cr[6].eq {
	pc = 0x82426314; continue 'dispatch;
	}
	// 8242630C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82426310: 997A0003  stb r11, 3(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82426314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82426318: 48007E09  bl 0x8242e120
	ctx.lr = 0x8242631C;
	sub_8242E120(ctx, base);
	// 8242631C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82426320: 409A0020  bne cr6, 0x82426340
	if !ctx.cr[6].eq {
	pc = 0x82426340; continue 'dispatch;
	}
	// 82426324: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82426328: 2F050040  cmpwi cr6, r5, 0x40
	ctx.cr[6].compare_i32(ctx.r[5].s32, 64, &mut ctx.xer);
	// 8242632C: 41980008  blt cr6, 0x82426334
	if ctx.cr[6].lt {
	pc = 0x82426334; continue 'dispatch;
	}
	// 82426330: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82426334: 387A0060  addi r3, r26, 0x60
	ctx.r[3].s64 = ctx.r[26].s64 + 96;
	// 82426338: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242633C: 4810E815  bl 0x82534b50
	ctx.lr = 0x82426340;
	sub_82534B50(ctx, base);
	// 82426340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82426344: 48007DDD  bl 0x8242e120
	ctx.lr = 0x82426348;
	sub_8242E120(ctx, base);
	// 82426348: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 8242634C: 419A00A4  beq cr6, 0x824263f0
	if ctx.cr[6].eq {
	pc = 0x824263F0; continue 'dispatch;
	}
	// 82426350: 2F03000B  cmpwi cr6, r3, 0xb
	ctx.cr[6].compare_i32(ctx.r[3].s32, 11, &mut ctx.xer);
	// 82426354: 419A009C  beq cr6, 0x824263f0
	if ctx.cr[6].eq {
	pc = 0x824263F0; continue 'dispatch;
	}
	// 82426358: 2F03000C  cmpwi cr6, r3, 0xc
	ctx.cr[6].compare_i32(ctx.r[3].s32, 12, &mut ctx.xer);
	// 8242635C: 419A0094  beq cr6, 0x824263f0
	if ctx.cr[6].eq {
	pc = 0x824263F0; continue 'dispatch;
	}
	// 82426360: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 82426364: 419A008C  beq cr6, 0x824263f0
	if ctx.cr[6].eq {
	pc = 0x824263F0; continue 'dispatch;
	}
	// 82426368: 2F03000F  cmpwi cr6, r3, 0xf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 15, &mut ctx.xer);
	// 8242636C: 419A0084  beq cr6, 0x824263f0
	if ctx.cr[6].eq {
	pc = 0x824263F0; continue 'dispatch;
	}
	// 82426370: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82426374: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426378: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8242637C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82426380: 48002B89  bl 0x82428f08
	ctx.lr = 0x82426384;
	sub_82428F08(ctx, base);
	// 82426384: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426388: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8242638C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426390: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82426394: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82426398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242639C: 4E800421  bctrl
	ctx.lr = 0x824263A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824263A0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824263A4: 48000050  b 0x824263f4
	pc = 0x824263F4; continue 'dispatch;
	// 824263A8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824263AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824263B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824263B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824263B8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824263BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824263C0: 4E800421  bctrl
	ctx.lr = 0x824263C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824263C4: 48007B8D  bl 0x8242df50
	ctx.lr = 0x824263C8;
	sub_8242DF50(ctx, base);
	// 824263C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824263CC: 40820018  bne 0x824263e4
	if !ctx.cr[0].eq {
	pc = 0x824263E4; continue 'dispatch;
	}
	// 824263D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824263D4: 388B202C  addi r4, r11, 0x202c
	ctx.r[4].s64 = ctx.r[11].s64 + 8236;
	// 824263D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824263DC: 386B200C  addi r3, r11, 0x200c
	ctx.r[3].s64 = ctx.r[11].s64 + 8204;
	// 824263E0: 4BFFAF41  bl 0x82421320
	ctx.lr = 0x824263E4;
	sub_82421320(ctx, base);
	// 824263E4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 824263E8: 997A0001  stb r11, 1(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824263EC: 48000044  b 0x82426430
	pc = 0x82426430; continue 'dispatch;
	// 824263F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824263F4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824263F8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824263FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426400: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426408: 4E800421  bctrl
	ctx.lr = 0x8242640C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242640C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82426410: 4BFFFFD8  b 0x824263e8
	pc = 0x824263E8; continue 'dispatch;
	// 82426414: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426418: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8242641C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426420: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426424: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242642C: 4E800421  bctrl
	ctx.lr = 0x82426430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426430: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82426434: 4810ECC8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82426438 size=232
    let mut pc: u32 = 0x82426438;
    'dispatch: loop {
        match pc {
            0x82426438 => {
    //   block [0x82426438..0x82426520)
	// 82426438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242643C: 4810EC6D  bl 0x825350a8
	ctx.lr = 0x82426440;
	sub_82535080(ctx, base);
	// 82426440: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82426448: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8242644C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82426450: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82426454: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82426458: 835F000C  lwz r26, 0xc(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8242645C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82426460: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426464: 48007CC5  bl 0x8242e128
	ctx.lr = 0x82426468;
	sub_8242E128(ctx, base);
	// 82426468: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242646C: 40810044  ble 0x824264b0
	if !ctx.cr[0].gt {
	pc = 0x824264B0; continue 'dispatch;
	}
	// 82426470: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82426474: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426478: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8242647C: 38A04000  li r5, 0x4000
	ctx.r[5].s64 = 16384;
	// 82426480: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426488: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242648C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426490: 4E800421  bctrl
	ctx.lr = 0x82426494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426494: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426498: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8242649C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824264A0: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824264A4: 48007C85  bl 0x8242e128
	ctx.lr = 0x824264A8;
	sub_8242E128(ctx, base);
	// 824264A8: 7F1C1800  cmpw cr6, r28, r3
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[3].s32, &mut ctx.xer);
	// 824264AC: 4198FFC8  blt cr6, 0x82426474
	if ctx.cr[6].lt {
	pc = 0x82426474; continue 'dispatch;
	}
	// 824264B0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824264B4: 4BFFCD8D  bl 0x82423240
	ctx.lr = 0x824264B8;
	sub_82423240(ctx, base);
	// 824264B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824264BC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 824264C0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824264C4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824264C8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824264CC: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824264D0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824264D4: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 824264D8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 824264DC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824264E0: 40980008  bge cr6, 0x824264e8
	if !ctx.cr[6].lt {
	pc = 0x824264E8; continue 'dispatch;
	}
	// 824264E4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824264E8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824264EC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 824264F0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824264F4: 41800010  blt 0x82426504
	if ctx.cr[0].lt {
	pc = 0x82426504; continue 'dispatch;
	}
	// 824264F8: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824264FC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82426500: 4800000C  b 0x8242650c
	pc = 0x8242650C; continue 'dispatch;
	// 82426504: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82426508: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8242650C: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82426510: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426514: 48007C05  bl 0x8242e118
	ctx.lr = 0x82426518;
	sub_8242E118(ctx, base);
	// 82426518: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8242651C: 4810EBDC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82426520 size=304
    let mut pc: u32 = 0x82426520;
    'dispatch: loop {
        match pc {
            0x82426520 => {
    //   block [0x82426520..0x82426650)
	// 82426520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426524: 4810EB91  bl 0x825350b4
	ctx.lr = 0x82426528;
	sub_82535080(ctx, base);
	// 82426528: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242652C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82426530: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82426534: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82426538: 815E00A8  lwz r10, 0xa8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 8242653C: 555C083C  slwi r28, r10, 1
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82426540: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426544: 40810070  ble 0x824265b4
	if !ctx.cr[0].gt {
	pc = 0x824265B4; continue 'dispatch;
	}
	// 82426548: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 8242654C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426550: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 82426554: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82426558: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8242655C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426560: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426564: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242656C: 4E800421  bctrl
	ctx.lr = 0x82426570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426570: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82426574: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426578: 41980008  blt cr6, 0x82426580
	if ctx.cr[6].lt {
	pc = 0x82426580; continue 'dispatch;
	}
	// 8242657C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82426580: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426584: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242658C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426590: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426594: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426598: 4E800421  bctrl
	ctx.lr = 0x8242659C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242659C: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 824265A0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824265A4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824265A8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824265AC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824265B0: 4198FF9C  blt cr6, 0x8242654c
	if ctx.cr[6].lt {
	pc = 0x8242654C; continue 'dispatch;
	}
	// 824265B4: 7F8B0E70  srawi r11, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 824265B8: 7F6B0194  addze r27, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[27].s64 = tmp.s64;
	// 824265BC: 577C083D  rlwinm. r28, r27, 1, 0, 0x1e
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824265C0: 40810088  ble 0x82426648
	if !ctx.cr[0].gt {
	pc = 0x82426648; continue 'dispatch;
	}
	// 824265C4: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 824265C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824265CC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824265D0: 4081006C  ble 0x8242663c
	if !ctx.cr[0].gt {
	pc = 0x8242663C; continue 'dispatch;
	}
	// 824265D4: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 824265D8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824265DC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824265E0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824265E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824265E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824265EC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824265F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824265F4: 4E800421  bctrl
	ctx.lr = 0x824265F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824265F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824265FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426600: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82426604: 4810EBCD  bl 0x825351d0
	ctx.lr = 0x82426608;
	sub_825351D0(ctx, base);
	// 82426608: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242660C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426610: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426614: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426618: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242661C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426620: 4E800421  bctrl
	ctx.lr = 0x82426624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426624: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82426628: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8242662C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82426630: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82426634: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426638: 4198FFA0  blt cr6, 0x824265d8
	if ctx.cr[6].lt {
	pc = 0x824265D8; continue 'dispatch;
	}
	// 8242663C: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82426640: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82426644: 917E00A8  stw r11, 0xa8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82426648: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8242664C: 4810EAB8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82426650 size=288
    let mut pc: u32 = 0x82426650;
    'dispatch: loop {
        match pc {
            0x82426650 => {
    //   block [0x82426650..0x82426770)
	// 82426650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426654: 4810EA61  bl 0x825350b4
	ctx.lr = 0x82426658;
	sub_82535080(ctx, base);
	// 82426658: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242665C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82426660: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82426664: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82426668: 815E00AC  lwz r10, 0xac(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 8242666C: 555C083C  slwi r28, r10, 1
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82426670: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426674: 40810070  ble 0x824266e4
	if !ctx.cr[0].gt {
	pc = 0x824266E4; continue 'dispatch;
	}
	// 82426678: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 8242667C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426680: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 82426684: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82426688: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8242668C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426690: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426694: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242669C: 4E800421  bctrl
	ctx.lr = 0x824266A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824266A0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824266A4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824266A8: 41980008  blt cr6, 0x824266b0
	if ctx.cr[6].lt {
	pc = 0x824266B0; continue 'dispatch;
	}
	// 824266AC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 824266B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824266B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824266B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824266BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824266C0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824266C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824266C8: 4E800421  bctrl
	ctx.lr = 0x824266CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824266CC: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 824266D0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824266D4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824266D8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824266DC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824266E0: 4198FF9C  blt cr6, 0x8242667c
	if ctx.cr[6].lt {
	pc = 0x8242667C; continue 'dispatch;
	}
	// 824266E4: 7F8B0E70  srawi r11, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 824266E8: 7F6B0194  addze r27, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[27].s64 = tmp.s64;
	// 824266EC: 577C083D  rlwinm. r28, r27, 1, 0, 0x1e
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824266F0: 40810078  ble 0x82426768
	if !ctx.cr[0].gt {
	pc = 0x82426768; continue 'dispatch;
	}
	// 824266F4: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 824266F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824266FC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426700: 4081005C  ble 0x8242675c
	if !ctx.cr[0].gt {
	pc = 0x8242675C; continue 'dispatch;
	}
	// 82426704: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82426708: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242670C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82426710: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82426714: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426718: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242671C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426724: 4E800421  bctrl
	ctx.lr = 0x82426728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426728: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242672C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426730: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426734: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426738: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8242673C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426740: 4E800421  bctrl
	ctx.lr = 0x82426744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426744: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82426748: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8242674C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82426750: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82426754: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426758: 4198FFB0  blt cr6, 0x82426708
	if ctx.cr[6].lt {
	pc = 0x82426708; continue 'dispatch;
	}
	// 8242675C: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82426760: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82426764: 917E00AC  stw r11, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82426768: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8242676C: 4810E998  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426770 size=8
    let mut pc: u32 = 0x82426770;
    'dispatch: loop {
        match pc {
            0x82426770 => {
    //   block [0x82426770..0x82426778)
	// 82426770: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82426774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426778 size=8
    let mut pc: u32 = 0x82426778;
    'dispatch: loop {
        match pc {
            0x82426778 => {
    //   block [0x82426778..0x82426780)
	// 82426778: 90830034  stw r4, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 8242677C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426780 size=8
    let mut pc: u32 = 0x82426780;
    'dispatch: loop {
        match pc {
            0x82426780 => {
    //   block [0x82426780..0x82426788)
	// 82426780: 908300A4  stw r4, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 82426784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426788 size=8
    let mut pc: u32 = 0x82426788;
    'dispatch: loop {
        match pc {
            0x82426788 => {
    //   block [0x82426788..0x82426790)
	// 82426788: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242678C: 48007B44  b 0x8242e2d0
	sub_8242E2D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426790 size=12
    let mut pc: u32 = 0x82426790;
    'dispatch: loop {
        match pc {
            0x82426790 => {
    //   block [0x82426790..0x8242679C)
	// 82426790: 90830050  stw r4, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82426794: 90A30054  stw r5, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82426798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267A0 size=12
    let mut pc: u32 = 0x824267A0;
    'dispatch: loop {
        match pc {
            0x824267A0 => {
    //   block [0x824267A0..0x824267AC)
	// 824267A0: 90830048  stw r4, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 824267A4: 90A3004C  stw r5, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 824267A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267B0 size=8
    let mut pc: u32 = 0x824267B0;
    'dispatch: loop {
        match pc {
            0x824267B0 => {
    //   block [0x824267B0..0x824267B8)
	// 824267B0: 9083003C  stw r4, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 824267B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267B8 size=8
    let mut pc: u32 = 0x824267B8;
    'dispatch: loop {
        match pc {
            0x824267B8 => {
    //   block [0x824267B8..0x824267C0)
	// 824267B8: 90830040  stw r4, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 824267BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267C0 size=8
    let mut pc: u32 = 0x824267C0;
    'dispatch: loop {
        match pc {
            0x824267C0 => {
    //   block [0x824267C0..0x824267C8)
	// 824267C0: 90830044  stw r4, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 824267C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267C8 size=8
    let mut pc: u32 = 0x824267C8;
    'dispatch: loop {
        match pc {
            0x824267C8 => {
    //   block [0x824267C8..0x824267D0)
	// 824267C8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267CC: 48007954  b 0x8242e120
	sub_8242E120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267D0 size=8
    let mut pc: u32 = 0x824267D0;
    'dispatch: loop {
        match pc {
            0x824267D0 => {
    //   block [0x824267D0..0x824267D8)
	// 824267D0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267D4: 4BF88164  b 0x823ae938
	sub_823AE938(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267D8 size=8
    let mut pc: u32 = 0x824267D8;
    'dispatch: loop {
        match pc {
            0x824267D8 => {
    //   block [0x824267D8..0x824267E0)
	// 824267D8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267DC: 4800794C  b 0x8242e128
	sub_8242E128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267E0 size=8
    let mut pc: u32 = 0x824267E0;
    'dispatch: loop {
        match pc {
            0x824267E0 => {
    //   block [0x824267E0..0x824267E8)
	// 824267E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267E4: 48007994  b 0x8242e178
	sub_8242E178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267E8 size=8
    let mut pc: u32 = 0x824267E8;
    'dispatch: loop {
        match pc {
            0x824267E8 => {
    //   block [0x824267E8..0x824267F0)
	// 824267E8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267EC: 480079E4  b 0x8242e1d0
	sub_8242E1D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267F0 size=8
    let mut pc: u32 = 0x824267F0;
    'dispatch: loop {
        match pc {
            0x824267F0 => {
    //   block [0x824267F0..0x824267F8)
	// 824267F0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267F4: 480079E4  b 0x8242e1d8
	sub_8242E1D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824267F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824267F8 size=8
    let mut pc: u32 = 0x824267F8;
    'dispatch: loop {
        match pc {
            0x824267F8 => {
    //   block [0x824267F8..0x82426800)
	// 824267F8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824267FC: 480079E4  b 0x8242e1e0
	sub_8242E1E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426800 size=8
    let mut pc: u32 = 0x82426800;
    'dispatch: loop {
        match pc {
            0x82426800 => {
    //   block [0x82426800..0x82426808)
	// 82426800: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426804: 480079E4  b 0x8242e1e8
	sub_8242E1E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426808 size=12
    let mut pc: u32 = 0x82426808;
    'dispatch: loop {
        match pc {
            0x82426808 => {
    //   block [0x82426808..0x82426814)
	// 82426808: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242680C: 409A0008  bne cr6, 0x82426814
	if !ctx.cr[6].eq {
		sub_82426814(ctx, base);
		return;
	}
	// 82426810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426814 size=8
    let mut pc: u32 = 0x82426814;
    'dispatch: loop {
        match pc {
            0x82426814 => {
    //   block [0x82426814..0x8242681C)
	// 82426814: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426818: 480079D8  b 0x8242e1f0
	sub_8242E1F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426820 size=8
    let mut pc: u32 = 0x82426820;
    'dispatch: loop {
        match pc {
            0x82426820 => {
    //   block [0x82426820..0x82426828)
	// 82426820: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426824: 480079DC  b 0x8242e200
	sub_8242E200(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426828 size=8
    let mut pc: u32 = 0x82426828;
    'dispatch: loop {
        match pc {
            0x82426828 => {
    //   block [0x82426828..0x82426830)
	// 82426828: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242682C: 480079DC  b 0x8242e208
	sub_8242E208(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82426830 size=96
    let mut pc: u32 = 0x82426830;
    'dispatch: loop {
        match pc {
            0x82426830 => {
    //   block [0x82426830..0x82426890)
	// 82426830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82426838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242683C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82426844: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426848: 480079C9  bl 0x8242e210
	ctx.lr = 0x8242684C;
	sub_8242E210(ctx, base);
	// 8242684C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426850: 40810028  ble 0x82426878
	if !ctx.cr[0].gt {
	pc = 0x82426878; continue 'dispatch;
	}
	// 82426854: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82426858: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8242685C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82426860: 419A000C  beq cr6, 0x8242686c
	if ctx.cr[6].eq {
	pc = 0x8242686C; continue 'dispatch;
	}
	// 82426864: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82426868: 409A0010  bne cr6, 0x82426878
	if !ctx.cr[6].eq {
	pc = 0x82426878; continue 'dispatch;
	}
	// 8242686C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426870: 480079A9  bl 0x8242e218
	ctx.lr = 0x82426874;
	sub_8242E218(ctx, base);
	// 82426874: 48000008  b 0x8242687c
	pc = 0x8242687C; continue 'dispatch;
	// 82426878: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242687C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82426880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82426884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82426888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242688C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82426890 size=112
    let mut pc: u32 = 0x82426890;
    'dispatch: loop {
        match pc {
            0x82426890 => {
    //   block [0x82426890..0x82426900)
	// 82426890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82426898: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242689C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824268A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824268A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824268A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824268AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824268B0: 48007961  bl 0x8242e210
	ctx.lr = 0x824268B4;
	sub_8242E210(ctx, base);
	// 824268B4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824268B8: 4081002C  ble 0x824268e4
	if !ctx.cr[0].gt {
	pc = 0x824268E4; continue 'dispatch;
	}
	// 824268BC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824268C0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824268C4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824268C8: 419A000C  beq cr6, 0x824268d4
	if ctx.cr[6].eq {
	pc = 0x824268D4; continue 'dispatch;
	}
	// 824268CC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824268D0: 409A0014  bne cr6, 0x824268e4
	if !ctx.cr[6].eq {
	pc = 0x824268E4; continue 'dispatch;
	}
	// 824268D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824268D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824268DC: 48007945  bl 0x8242e220
	ctx.lr = 0x824268E0;
	sub_8242E220(ctx, base);
	// 824268E0: 48000008  b 0x824268e8
	pc = 0x824268E8; continue 'dispatch;
	// 824268E4: 3860FF80  li r3, -0x80
	ctx.r[3].s64 = -128;
	// 824268E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824268EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824268F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824268F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824268F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824268FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426900 size=8
    let mut pc: u32 = 0x82426900;
    'dispatch: loop {
        match pc {
            0x82426900 => {
    //   block [0x82426900..0x82426908)
	// 82426900: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82426904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426908 size=8
    let mut pc: u32 = 0x82426908;
    'dispatch: loop {
        match pc {
            0x82426908 => {
    //   block [0x82426908..0x82426910)
	// 82426908: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242690C: 48007924  b 0x8242e230
	sub_8242E230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82426910 size=8
    let mut pc: u32 = 0x82426910;
    'dispatch: loop {
        match pc {
            0x82426910 => {
    //   block [0x82426910..0x82426918)
	// 82426910: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426914: 4800796C  b 0x8242e280
	sub_8242E280(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82426918 size=344
    let mut pc: u32 = 0x82426918;
    'dispatch: loop {
        match pc {
            0x82426918 => {
    //   block [0x82426918..0x82426A70)
	// 82426918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242691C: 4810E791  bl 0x825350ac
	ctx.lr = 0x82426920;
	sub_82535080(ctx, base);
	// 82426920: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426924: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82426928: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 8242692C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82426930: 396B0BC0  addi r11, r11, 0xbc0
	ctx.r[11].s64 = ctx.r[11].s64 + 3008;
	// 82426934: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82426938: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242693C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82426940: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82426944: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82426948: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242694C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82426950: 419A0018  beq cr6, 0x82426968
	if ctx.cr[6].eq {
	pc = 0x82426968; continue 'dispatch;
	}
	// 82426954: 392900B4  addi r9, r9, 0xb4
	ctx.r[9].s64 = ctx.r[9].s64 + 180;
	// 82426958: 390B1680  addi r8, r11, 0x1680
	ctx.r[8].s64 = ctx.r[11].s64 + 5760;
	// 8242695C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82426960: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82426964: 4198FFE4  blt cr6, 0x82426948
	if ctx.cr[6].lt {
	pc = 0x82426948; continue 'dispatch;
	}
	// 82426968: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8242696C: 409A000C  bne cr6, 0x82426978
	if !ctx.cr[6].eq {
	pc = 0x82426978; continue 'dispatch;
	}
	// 82426970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82426974: 480000F4  b 0x82426a68
	pc = 0x82426A68; continue 'dispatch;
	// 82426978: 1D4A00B4  mulli r10, r10, 0xb4
	ctx.r[10].s64 = ctx.r[10].s64 * 180;
	// 8242697C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426980: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82426984: 4BFFC8BD  bl 0x82423240
	ctx.lr = 0x82426988;
	sub_82423240(ctx, base);
	// 82426988: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8242698C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426990: 4BFFC931  bl 0x824232c0
	ctx.lr = 0x82426994;
	sub_824232C0(ctx, base);
	// 82426994: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82426998: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242699C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824269A0: 7FCB0194  addze r30, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[30].s64 = tmp.s64;
	// 824269A4: 4BFFC99D  bl 0x82423340
	ctx.lr = 0x824269A8;
	sub_82423340(ctx, base);
	// 824269A8: 7C6B0E70  srawi r11, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 824269AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824269B0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824269B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824269B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824269BC: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824269C0: 48007F11  bl 0x8242e8d0
	ctx.lr = 0x824269C4;
	sub_8242E8D0(ctx, base);
	// 824269C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824269C8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 824269CC: 4182FFA4  beq 0x82426970
	if ctx.cr[0].eq {
	pc = 0x82426970; continue 'dispatch;
	}
	// 824269D0: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 824269D4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824269D8: 388B6438  addi r4, r11, 0x6438
	ctx.r[4].s64 = ctx.r[11].s64 + 25656;
	// 824269DC: 4800772D  bl 0x8242e108
	ctx.lr = 0x824269E0;
	sub_8242E108(ctx, base);
	// 824269E0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824269E4: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 824269E8: 9BBF0002  stb r29, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 824269EC: 40990024  ble cr6, 0x82426a10
	if !ctx.cr[6].gt {
	pc = 0x82426A10; continue 'dispatch;
	}
	// 824269F0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 824269F4: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 824269F8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824269FC: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82426A00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82426A04: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82426A08: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82426A0C: 4082FFEC  bne 0x824269f8
	if !ctx.cr[0].eq {
	pc = 0x824269F8; continue 'dispatch;
	}
	// 82426A10: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82426A14: 9B3F0001  stb r25, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[25].u8 ) };
	// 82426A18: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82426A1C: 933F00A0  stw r25, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[25].u32 ) };
	// 82426A20: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82426A24: 933F002C  stw r25, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[25].u32 ) };
	// 82426A28: 933F0030  stw r25, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[25].u32 ) };
	// 82426A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82426A30: 933F0034  stw r25, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[25].u32 ) };
	// 82426A34: 933F0040  stw r25, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[25].u32 ) };
	// 82426A38: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82426A3C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82426A40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82426A44: 933F0044  stw r25, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[25].u32 ) };
	// 82426A48: 9B3F0003  stb r25, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[25].u8 ) };
	// 82426A4C: 933F00A8  stw r25, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[25].u32 ) };
	// 82426A50: 933F00AC  stw r25, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[25].u32 ) };
	// 82426A54: 933F0048  stw r25, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[25].u32 ) };
	// 82426A58: 933F004C  stw r25, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[25].u32 ) };
	// 82426A5C: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82426A60: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82426A64: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82426A68: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82426A6C: 4810E690  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82426A70 size=952
    let mut pc: u32 = 0x82426A70;
    'dispatch: loop {
        match pc {
            0x82426A70 => {
    //   block [0x82426A70..0x82426E28)
	// 82426A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426A74: 4810E639  bl 0x825350ac
	ctx.lr = 0x82426A78;
	sub_82535080(ctx, base);
	// 82426A78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426A7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82426A80: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82426A84: 833F0004  lwz r25, 4(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426A88: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82426A8C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426A90: 41800028  blt 0x82426ab8
	if ctx.cr[0].lt {
	pc = 0x82426AB8; continue 'dispatch;
	}
	// 82426A94: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82426A98: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426A9C: 4198001C  blt cr6, 0x82426ab8
	if ctx.cr[6].lt {
	pc = 0x82426AB8; continue 'dispatch;
	}
	// 82426AA0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82426AA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82426AA8: 41820010  beq 0x82426ab8
	if ctx.cr[0].eq {
	pc = 0x82426AB8; continue 'dispatch;
	}
	// 82426AAC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82426AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426AB4: 4E800421  bctrl
	ctx.lr = 0x82426AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426AB8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82426ABC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82426AC0: 409A0030  bne cr6, 0x82426af0
	if !ctx.cr[6].eq {
	pc = 0x82426AF0; continue 'dispatch;
	}
	// 82426AC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426AC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426ACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426AD0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82426AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426AD8: 4E800421  bctrl
	ctx.lr = 0x82426ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426ADC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426AE0: 40820010  bne 0x82426af0
	if !ctx.cr[0].eq {
	pc = 0x82426AF0; continue 'dispatch;
	}
	// 82426AE4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82426AE8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82426AEC: 48000334  b 0x82426e20
	pc = 0x82426E20; continue 'dispatch;
	// 82426AF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426AF4: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82426AF8: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82426AFC: 615AFFFF  ori r26, r10, 0xffff
	ctx.r[26].u64 = ctx.r[10].u64 | 65535;
	// 82426B00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426B04: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82426B08: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426B0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426B10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82426B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426B18: 4E800421  bctrl
	ctx.lr = 0x82426B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426B1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426B20: 48007601  bl 0x8242e120
	ctx.lr = 0x82426B24;
	sub_8242E120(ctx, base);
	// 82426B24: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426B28: 40820170  bne 0x82426c98
	if !ctx.cr[0].eq {
	pc = 0x82426C98; continue 'dispatch;
	}
	// 82426B2C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426B30: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82426B34: 41980164  blt cr6, 0x82426c98
	if ctx.cr[6].lt {
	pc = 0x82426C98; continue 'dispatch;
	}
	// 82426B38: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426B3C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426B40: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 82426B44: 409A0154  bne cr6, 0x82426c98
	if !ctx.cr[6].eq {
	pc = 0x82426C98; continue 'dispatch;
	}
	// 82426B48: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82426B4C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426B50: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82426B54: 480015D5  bl 0x82428128
	ctx.lr = 0x82426B58;
	sub_82428128(ctx, base);
	// 82426B58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426B5C: 40820078  bne 0x82426bd4
	if !ctx.cr[0].eq {
	pc = 0x82426BD4; continue 'dispatch;
	}
	// 82426B60: A8810050  lha r4, 0x50(r1)
	ctx.r[4].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 82426B64: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426B68: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426B6C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426B70: 40990020  ble cr6, 0x82426b90
	if !ctx.cr[6].gt {
	pc = 0x82426B90; continue 'dispatch;
	}
	// 82426B74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426B78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426B7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426B80: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426B88: 4E800421  bctrl
	ctx.lr = 0x82426B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426B8C: 48000294  b 0x82426e20
	pc = 0x82426E20; continue 'dispatch;
	// 82426B90: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82426B94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82426B98: 48002371  bl 0x82428f08
	ctx.lr = 0x82426B9C;
	sub_82428F08(ctx, base);
	// 82426B9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426BA0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426BA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426BA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426BAC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82426BB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426BB4: 4E800421  bctrl
	ctx.lr = 0x82426BB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426BB8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426BBC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426BC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426BC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426BD0: 4E800421  bctrl
	ctx.lr = 0x82426BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426BD4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82426BD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426BDC: 419A0244  beq cr6, 0x82426e20
	if ctx.cr[6].eq {
	pc = 0x82426E20; continue 'dispatch;
	}
	// 82426BE0: 48000088  b 0x82426c68
	pc = 0x82426C68; continue 'dispatch;
	// 82426BE4: 837F0018  lwz r27, 0x18(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426BE8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82426BEC: 2C1B0000  cmpwi r27, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426BF0: 40810024  ble 0x82426c14
	if !ctx.cr[0].gt {
	pc = 0x82426C14; continue 'dispatch;
	}
	// 82426BF4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426BF8: 7D4BE0AE  lbzx r10, r11, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82426BFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82426C00: 409A0014  bne cr6, 0x82426c14
	if !ctx.cr[6].eq {
	pc = 0x82426C14; continue 'dispatch;
	}
	// 82426C04: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426C08: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82426C0C: 7F1C5000  cmpw cr6, r28, r10
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82426C10: 4198FFE8  blt cr6, 0x82426bf8
	if ctx.cr[6].lt {
	pc = 0x82426BF8; continue 'dispatch;
	}
	// 82426C14: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82426C18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426C1C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82426C20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82426C24: 480022E5  bl 0x82428f08
	ctx.lr = 0x82426C28;
	sub_82428F08(ctx, base);
	// 82426C28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426C2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426C30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426C34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426C38: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82426C3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426C40: 4E800421  bctrl
	ctx.lr = 0x82426C44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426C44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426C48: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426C4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426C50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426C54: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426C5C: 4E800421  bctrl
	ctx.lr = 0x82426C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426C60: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82426C64: 419801BC  blt cr6, 0x82426e20
	if ctx.cr[6].lt {
	pc = 0x82426E20; continue 'dispatch;
	}
	// 82426C68: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426C6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426C70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426C74: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82426C78: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82426C7C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426C84: 4E800421  bctrl
	ctx.lr = 0x82426C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426C88: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426C8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82426C90: 409AFF54  bne cr6, 0x82426be4
	if !ctx.cr[6].eq {
	pc = 0x82426BE4; continue 'dispatch;
	}
	// 82426C94: 4800018C  b 0x82426e20
	pc = 0x82426E20; continue 'dispatch;
	// 82426C98: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426C9C: 4800753D  bl 0x8242e1d8
	ctx.lr = 0x82426CA0;
	sub_8242E1D8(ctx, base);
	// 82426CA0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82426CA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82426CA8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426CAC: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82426CB0: 4198005C  blt cr6, 0x82426d0c
	if ctx.cr[6].lt {
	pc = 0x82426D0C; continue 'dispatch;
	}
	// 82426CB4: 4800746D  bl 0x8242e120
	ctx.lr = 0x82426CB8;
	sub_8242E120(ctx, base);
	// 82426CB8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82426CBC: 409A006C  bne cr6, 0x82426d28
	if !ctx.cr[6].eq {
	pc = 0x82426D28; continue 'dispatch;
	}
	// 82426CC0: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82426CC4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82426CC8: 409A0060  bne cr6, 0x82426d28
	if !ctx.cr[6].eq {
	pc = 0x82426D28; continue 'dispatch;
	}
	// 82426CCC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426CD0: 48007501  bl 0x8242e1d0
	ctx.lr = 0x82426CD4;
	sub_8242E1D0(ctx, base);
	// 82426CD4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82426CD8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82426CDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426CE0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82426CE4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426CE8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82426CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426CF0: 4E800421  bctrl
	ctx.lr = 0x82426CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426CF4: 7C6B0E70  srawi r11, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 82426CF8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82426CFC: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82426D00: 40980038  bge cr6, 0x82426d38
	if !ctx.cr[6].lt {
	pc = 0x82426D38; continue 'dispatch;
	}
	// 82426D04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426D08: 4BFFFE6C  b 0x82426b74
	pc = 0x82426B74; continue 'dispatch;
	// 82426D0C: 48007415  bl 0x8242e120
	ctx.lr = 0x82426D10;
	sub_8242E120(ctx, base);
	// 82426D10: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 82426D14: 409AFFB8  bne cr6, 0x82426ccc
	if !ctx.cr[6].eq {
	pc = 0x82426CCC; continue 'dispatch;
	}
	// 82426D18: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82426D1C: 396B0240  addi r11, r11, 0x240
	ctx.r[11].s64 = ctx.r[11].s64 + 576;
	// 82426D20: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82426D24: 4198FFA8  blt cr6, 0x82426ccc
	if ctx.cr[6].lt {
	pc = 0x82426CCC; continue 'dispatch;
	}
	// 82426D28: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82426D2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426D30: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82426D34: 4BFFFE40  b 0x82426b74
	pc = 0x82426B74; continue 'dispatch;
	// 82426D38: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82426D3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82426D40: 419A009C  beq cr6, 0x82426ddc
	if ctx.cr[6].eq {
	pc = 0x82426DDC; continue 'dispatch;
	}
	// 82426D44: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426D48: 480073D9  bl 0x8242e120
	ctx.lr = 0x82426D4C;
	sub_8242E120(ctx, base);
	// 82426D4C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82426D50: 409A008C  bne cr6, 0x82426ddc
	if !ctx.cr[6].eq {
	pc = 0x82426DDC; continue 'dispatch;
	}
	// 82426D54: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426D58: 48007411  bl 0x8242e168
	ctx.lr = 0x82426D5C;
	sub_8242E168(ctx, base);
	// 82426D5C: 2F030010  cmpwi cr6, r3, 0x10
	ctx.cr[6].compare_i32(ctx.r[3].s32, 16, &mut ctx.xer);
	// 82426D60: 409A0068  bne cr6, 0x82426dc8
	if !ctx.cr[6].eq {
	pc = 0x82426DC8; continue 'dispatch;
	}
	// 82426D64: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426D68: 480073C1  bl 0x8242e128
	ctx.lr = 0x82426D6C;
	sub_8242E128(ctx, base);
	// 82426D6C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426D70: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82426D74: 7D4A1BD6  divw r10, r10, r3
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 82426D78: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82426D7C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82426D80: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82426D84: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82426D88: 40990054  ble cr6, 0x82426ddc
	if !ctx.cr[6].gt {
	pc = 0x82426DDC; continue 'dispatch;
	}
	// 82426D8C: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 82426D90: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82426D94: 7D6B19D6  mullw r11, r11, r3
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82426D98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426D9C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82426DA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82426DA4: 48002165  bl 0x82428f08
	ctx.lr = 0x82426DA8;
	sub_82428F08(ctx, base);
	// 82426DA8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426DAC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426DB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426DB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426DB8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426DC0: 4E800421  bctrl
	ctx.lr = 0x82426DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426DC4: 48000018  b 0x82426ddc
	pc = 0x82426DDC; continue 'dispatch;
	// 82426DC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82426DCC: 388B2074  addi r4, r11, 0x2074
	ctx.r[4].s64 = ctx.r[11].s64 + 8308;
	// 82426DD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82426DD4: 386B2050  addi r3, r11, 0x2050
	ctx.r[3].s64 = ctx.r[11].s64 + 8272;
	// 82426DD8: 4BFFA549  bl 0x82421320
	ctx.lr = 0x82426DDC;
	sub_82421320(ctx, base);
	// 82426DDC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426DE0: 48007341  bl 0x8242e120
	ctx.lr = 0x82426DE4;
	sub_8242E120(ctx, base);
	// 82426DE4: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 82426DE8: 409A0020  bne cr6, 0x82426e08
	if !ctx.cr[6].eq {
	pc = 0x82426E08; continue 'dispatch;
	}
	// 82426DEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426DF0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82426DF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426DF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426DFC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426E04: 4E800421  bctrl
	ctx.lr = 0x82426E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426E08: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426E0C: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82426E10: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426E14: 4800761D  bl 0x8242e430
	ctx.lr = 0x82426E18;
	sub_8242E430(ctx, base);
	// 82426E18: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82426E1C: 48007675  bl 0x8242e490
	ctx.lr = 0x82426E20;
	sub_8242E490(ctx, base);
	// 82426E20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82426E24: 4810E2D8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82426E28 size=432
    let mut pc: u32 = 0x82426E28;
    'dispatch: loop {
        match pc {
            0x82426E28 => {
    //   block [0x82426E28..0x82426FD8)
	// 82426E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426E2C: 4810E27D  bl 0x825350a8
	ctx.lr = 0x82426E30;
	sub_82535080(ctx, base);
	// 82426E30: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426E34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82426E38: 835F0004  lwz r26, 4(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426E3C: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82426E40: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82426E44: 48007395  bl 0x8242e1d8
	ctx.lr = 0x82426E48;
	sub_8242E1D8(ctx, base);
	// 82426E48: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82426E4C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82426E50: 480076D9  bl 0x8242e528
	ctx.lr = 0x82426E54;
	sub_8242E528(ctx, base);
	// 82426E54: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82426E58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82426E5C: 480076D5  bl 0x8242e530
	ctx.lr = 0x82426E60;
	sub_8242E530(ctx, base);
	// 82426E60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82426E64: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82426E68: 480072B9  bl 0x8242e120
	ctx.lr = 0x82426E6C;
	sub_8242E120(ctx, base);
	// 82426E6C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82426E70: 409A0010  bne cr6, 0x82426e80
	if !ctx.cr[6].eq {
	pc = 0x82426E80; continue 'dispatch;
	}
	// 82426E74: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82426E78: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82426E7C: 419A0018  beq cr6, 0x82426e94
	if ctx.cr[6].eq {
	pc = 0x82426E94; continue 'dispatch;
	}
	// 82426E80: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82426E84: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 82426E88: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82426E8C: 41980008  blt cr6, 0x82426e94
	if ctx.cr[6].lt {
	pc = 0x82426E94; continue 'dispatch;
	}
	// 82426E90: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82426E94: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82426E98: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426E9C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82426EA0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82426EA4: 48002065  bl 0x82428f08
	ctx.lr = 0x82426EA8;
	sub_82428F08(ctx, base);
	// 82426EA8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426EAC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426EB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426EB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82426EB8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82426EBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426EC0: 4E800421  bctrl
	ctx.lr = 0x82426EC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426EC4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426EC8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426ECC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426ED0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82426ED4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426ED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426EDC: 4E800421  bctrl
	ctx.lr = 0x82426EE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426EE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426EE4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82426EE8: 48007241  bl 0x8242e128
	ctx.lr = 0x82426EEC;
	sub_8242E128(ctx, base);
	// 82426EEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426EF0: 4081009C  ble 0x82426f8c
	if !ctx.cr[0].gt {
	pc = 0x82426F8C; continue 'dispatch;
	}
	// 82426EF4: 57D8083C  slwi r24, r30, 1
	ctx.r[24].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82426EF8: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82426EFC: 3B9F001C  addi r28, r31, 0x1c
	ctx.r[28].s64 = ctx.r[31].s64 + 28;
	// 82426F00: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82426F04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426F08: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82426F0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82426F10: 48001FF9  bl 0x82428f08
	ctx.lr = 0x82426F14;
	sub_82428F08(ctx, base);
	// 82426F14: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82426F18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82426F1C: 4182001C  beq 0x82426f38
	if ctx.cr[0].eq {
	pc = 0x82426F38; continue 'dispatch;
	}
	// 82426F20: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82426F24: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82426F28: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82426F2C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82426F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426F34: 4E800421  bctrl
	ctx.lr = 0x82426F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426F38: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426F3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82426F40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82426F44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426F48: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82426F4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426F50: 4E800421  bctrl
	ctx.lr = 0x82426F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426F54: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426F58: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82426F5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82426F60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82426F64: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82426F68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82426F6C: 4E800421  bctrl
	ctx.lr = 0x82426F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82426F70: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426F74: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82426F78: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82426F7C: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 82426F80: 480071A9  bl 0x8242e128
	ctx.lr = 0x82426F84;
	sub_8242E128(ctx, base);
	// 82426F84: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82426F88: 4198FF78  blt cr6, 0x82426f00
	if ctx.cr[6].lt {
	pc = 0x82426F00; continue 'dispatch;
	}
	// 82426F8C: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82426F90: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82426F94: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82426F98: 7D0AF214  add r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82426F9C: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82426FA0: 7CEBCA14  add r7, r11, r25
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82426FA4: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82426FA8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82426FAC: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82426FB0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82426FB4: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82426FB8: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82426FBC: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82426FC0: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 82426FC4: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82426FC8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82426FCC: 48007515  bl 0x8242e4e0
	ctx.lr = 0x82426FD0;
	sub_8242E4E0(ctx, base);
	// 82426FD0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82426FD4: 4810E124  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82426FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82426FD8 size=216
    let mut pc: u32 = 0x82426FD8;
    'dispatch: loop {
        match pc {
            0x82426FD8 => {
    //   block [0x82426FD8..0x824270B0)
	// 82426FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82426FDC: 4810E0DD  bl 0x825350b8
	ctx.lr = 0x82426FE0;
	sub_82535080(ctx, base);
	// 82426FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82426FE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82426FE8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82426FEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82426FF0: 48007439  bl 0x8242e428
	ctx.lr = 0x82426FF4;
	sub_8242E428(ctx, base);
	// 82426FF4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82426FF8: 4082000C  bne 0x82427004
	if !ctx.cr[0].eq {
	pc = 0x82427004; continue 'dispatch;
	}
	// 82426FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427000: 4BFFFA71  bl 0x82426a70
	ctx.lr = 0x82427004;
	sub_82426A70(ctx, base);
	// 82427004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427008: 48007DA9  bl 0x8242edb0
	ctx.lr = 0x8242700C;
	sub_8242EDB0(ctx, base);
	// 8242700C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427010: 48007419  bl 0x8242e428
	ctx.lr = 0x82427014;
	sub_8242E428(ctx, base);
	// 82427014: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82427018: 409A000C  bne cr6, 0x82427024
	if !ctx.cr[6].eq {
	pc = 0x82427024; continue 'dispatch;
	}
	// 8242701C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427020: 4BFFFE09  bl 0x82426e28
	ctx.lr = 0x82427024;
	sub_82426E28(ctx, base);
	// 82427024: A97E0098  lha r11, 0x98(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 82427028: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8242702C: 419A0024  beq cr6, 0x82427050
	if ctx.cr[6].eq {
	pc = 0x82427050; continue 'dispatch;
	}
	// 82427030: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82427034: 419A001C  beq cr6, 0x82427050
	if ctx.cr[6].eq {
	pc = 0x82427050; continue 'dispatch;
	}
	// 82427038: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 8242703C: 419A0014  beq cr6, 0x82427050
	if ctx.cr[6].eq {
	pc = 0x82427050; continue 'dispatch;
	}
	// 82427040: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82427044: 419A000C  beq cr6, 0x82427050
	if ctx.cr[6].eq {
	pc = 0x82427050; continue 'dispatch;
	}
	// 82427048: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 8242704C: 409A005C  bne cr6, 0x824270a8
	if !ctx.cr[6].eq {
	pc = 0x824270A8; continue 'dispatch;
	}
	// 82427050: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82427054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427058: 48007181  bl 0x8242e1d8
	ctx.lr = 0x8242705C;
	sub_8242E1D8(ctx, base);
	// 8242705C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82427060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427064: 480074C5  bl 0x8242e528
	ctx.lr = 0x82427068;
	sub_8242E528(ctx, base);
	// 82427068: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8242706C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427070: 480074C1  bl 0x8242e530
	ctx.lr = 0x82427074;
	sub_8242E530(ctx, base);
	// 82427074: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82427078: 7D4BE850  subf r10, r11, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8242707C: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82427080: 41980008  blt cr6, 0x82427088
	if ctx.cr[6].lt {
	pc = 0x82427088; continue 'dispatch;
	}
	// 82427084: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82427088: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8242708C: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82427090: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82427094: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82427098: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8242709C: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 824270A0: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 824270A4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824270A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824270AC: 4810E05C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824270B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824270B0 size=140
    let mut pc: u32 = 0x824270B0;
    'dispatch: loop {
        match pc {
            0x824270B0 => {
    //   block [0x824270B0..0x8242713C)
	// 824270B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824270B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824270B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824270BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824270C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824270C4: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 824270C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824270CC: 40990014  ble cr6, 0x824270e0
	if !ctx.cr[6].gt {
	pc = 0x824270E0; continue 'dispatch;
	}
	// 824270D0: 48007E81  bl 0x8242ef50
	ctx.lr = 0x824270D4;
	sub_8242EF50(ctx, base);
	// 824270D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824270D8: 4BFFF449  bl 0x82426520
	ctx.lr = 0x824270DC;
	sub_82426520(ctx, base);
	// 824270DC: 48007E75  bl 0x8242ef50
	ctx.lr = 0x824270E0;
	sub_8242EF50(ctx, base);
	// 824270E0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824270E4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824270E8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824270EC: 409A0010  bne cr6, 0x824270fc
	if !ctx.cr[6].eq {
	pc = 0x824270FC; continue 'dispatch;
	}
	// 824270F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824270F4: 4BFFFEE5  bl 0x82426fd8
	ctx.lr = 0x824270F8;
	sub_82426FD8(ctx, base);
	// 824270F8: 48000014  b 0x8242710c
	pc = 0x8242710C; continue 'dispatch;
	// 824270FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82427100: 409A000C  bne cr6, 0x8242710c
	if !ctx.cr[6].eq {
	pc = 0x8242710C; continue 'dispatch;
	}
	// 82427104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427108: 4BFFF0B9  bl 0x824261c0
	ctx.lr = 0x8242710C;
	sub_824261C0(ctx, base);
	// 8242710C: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82427110: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82427114: 40990014  ble cr6, 0x82427128
	if !ctx.cr[6].gt {
	pc = 0x82427128; continue 'dispatch;
	}
	// 82427118: 48007E39  bl 0x8242ef50
	ctx.lr = 0x8242711C;
	sub_8242EF50(ctx, base);
	// 8242711C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427120: 4BFFF531  bl 0x82426650
	ctx.lr = 0x82427124;
	sub_82426650(ctx, base);
	// 82427124: 48007E2D  bl 0x8242ef50
	ctx.lr = 0x82427128;
	sub_8242EF50(ctx, base);
	// 82427128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242712C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427140 size=140
    let mut pc: u32 = 0x82427140;
    'dispatch: loop {
        match pc {
            0x82427140 => {
    //   block [0x82427140..0x824271CC)
	// 82427140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82427144: 4810DF79  bl 0x825350bc
	ctx.lr = 0x82427148;
	sub_82535080(ctx, base);
	// 82427148: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242714C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82427150: 3BCB97C4  addi r30, r11, -0x683c
	ctx.r[30].s64 = ctx.r[11].s64 + -26684;
	// 82427154: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82427158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242715C: 419A0018  beq cr6, 0x82427174
	if ctx.cr[6].eq {
	pc = 0x82427174; continue 'dispatch;
	}
	// 82427160: 397EFFF4  addi r11, r30, -0xc
	ctx.r[11].s64 = ctx.r[30].s64 + -12;
	// 82427164: 807EFFF8  lwz r3, -8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427168: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242716C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427170: 4E800421  bctrl
	ctx.lr = 0x82427174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82427174: 3D608312  lis r11, -0x7cee
	ctx.r[11].s64 = -2095972352;
	// 82427178: 3BAB0BC0  addi r29, r11, 0xbc0
	ctx.r[29].s64 = ctx.r[11].s64 + 3008;
	// 8242717C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82427180: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427184: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82427188: 409A000C  bne cr6, 0x82427194
	if !ctx.cr[6].eq {
	pc = 0x82427194; continue 'dispatch;
	}
	// 8242718C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427190: 4BFFFF21  bl 0x824270b0
	ctx.lr = 0x82427194;
	sub_824270B0(ctx, base);
	// 82427194: 3BFF00B4  addi r31, r31, 0xb4
	ctx.r[31].s64 = ctx.r[31].s64 + 180;
	// 82427198: 397D1680  addi r11, r29, 0x1680
	ctx.r[11].s64 = ctx.r[29].s64 + 5760;
	// 8242719C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824271A0: 4198FFE0  blt cr6, 0x82427180
	if ctx.cr[6].lt {
	pc = 0x82427180; continue 'dispatch;
	}
	// 824271A4: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824271A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824271AC: 419A0018  beq cr6, 0x824271c4
	if ctx.cr[6].eq {
	pc = 0x824271C4; continue 'dispatch;
	}
	// 824271B0: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 824271B4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824271B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824271BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824271C0: 4E800421  bctrl
	ctx.lr = 0x824271C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824271C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824271C8: 4810DF44  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824271D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824271D0 size=72
    let mut pc: u32 = 0x824271D0;
    'dispatch: loop {
        match pc {
            0x824271D0 => {
    //   block [0x824271D0..0x82427218)
	// 824271D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824271D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824271D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824271DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824271E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824271E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824271E8: 419A001C  beq cr6, 0x82427204
	if ctx.cr[6].eq {
	pc = 0x82427204; continue 'dispatch;
	}
	// 824271EC: 48007D65  bl 0x8242ef50
	ctx.lr = 0x824271F0;
	sub_8242EF50(ctx, base);
	// 824271F0: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 824271F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824271F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824271FC: 4810DFD5  bl 0x825351d0
	ctx.lr = 0x82427200;
	sub_825351D0(ctx, base);
	// 82427200: 48007D51  bl 0x8242ef50
	ctx.lr = 0x82427204;
	sub_8242EF50(ctx, base);
	// 82427204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82427208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242720C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427218 size=420
    let mut pc: u32 = 0x82427218;
    'dispatch: loop {
        match pc {
            0x82427218 => {
    //   block [0x82427218..0x824273BC)
	// 82427218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242721C: 4810DE91  bl 0x825350ac
	ctx.lr = 0x82427220;
	sub_82535080(ctx, base);
	// 82427220: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82427228: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8242722C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82427230: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82427234: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82427238: 40810024  ble 0x8242725c
	if !ctx.cr[0].gt {
	pc = 0x8242725C; continue 'dispatch;
	}
	// 8242723C: 397E0014  addi r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 + 20;
	// 82427240: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82427244: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82427248: 893E0002  lbz r9, 2(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242724C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82427250: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82427254: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82427258: 4198FFE8  blt cr6, 0x82427240
	if ctx.cr[6].lt {
	pc = 0x82427240; continue 'dispatch;
	}
	// 8242725C: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82427260: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82427264: 939E002C  stw r28, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82427268: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242726C: 4081009C  ble 0x82427308
	if !ctx.cr[0].gt {
	pc = 0x82427308; continue 'dispatch;
	}
	// 82427270: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82427274: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242727C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427280: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82427284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427288: 4E800421  bctrl
	ctx.lr = 0x8242728C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242728C: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427290: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 82427294: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82427298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242729C: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 824272A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824272A4: 4E800421  bctrl
	ctx.lr = 0x824272A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824272A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824272AC: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 824272B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824272B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824272B8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 824272BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824272C0: 4E800421  bctrl
	ctx.lr = 0x824272C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824272C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824272C8: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824272CC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824272D0: 4810DF01  bl 0x825351d0
	ctx.lr = 0x824272D4;
	sub_825351D0(ctx, base);
	// 824272D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824272D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824272DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824272E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824272E4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824272E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824272EC: 4E800421  bctrl
	ctx.lr = 0x824272F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824272F0: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 824272F4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824272F8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824272FC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82427300: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82427304: 4198FF70  blt cr6, 0x82427274
	if ctx.cr[6].lt {
	pc = 0x82427274; continue 'dispatch;
	}
	// 82427308: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 8242730C: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82427310: 4081009C  ble 0x824273ac
	if !ctx.cr[0].gt {
	pc = 0x824273AC; continue 'dispatch;
	}
	// 82427314: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82427318: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242731C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427324: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82427328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242732C: 4E800421  bctrl
	ctx.lr = 0x82427330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82427330: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427334: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 82427338: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242733C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427340: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82427344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427348: 4E800421  bctrl
	ctx.lr = 0x8242734C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242734C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82427350: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82427354: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82427358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242735C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82427360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427364: 4E800421  bctrl
	ctx.lr = 0x82427368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82427368: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242736C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82427370: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82427374: 4810DE5D  bl 0x825351d0
	ctx.lr = 0x82427378;
	sub_825351D0(ctx, base);
	// 82427378: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242737C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82427380: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82427384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427388: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8242738C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427390: 4E800421  bctrl
	ctx.lr = 0x82427394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82427394: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82427398: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8242739C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824273A0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824273A4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824273A8: 4198FF70  blt cr6, 0x82427318
	if ctx.cr[6].lt {
	pc = 0x82427318; continue 'dispatch;
	}
	// 824273AC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 824273B0: 997E0001  stb r11, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824273B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824273B8: 4810DD44  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824273C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824273C0 size=8
    let mut pc: u32 = 0x824273C0;
    'dispatch: loop {
        match pc {
            0x824273C0 => {
    //   block [0x824273C0..0x824273C8)
	// 824273C0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 824273C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824273C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824273C8 size=280
    let mut pc: u32 = 0x824273C8;
    'dispatch: loop {
        match pc {
            0x824273C8 => {
    //   block [0x824273C8..0x824274E0)
	// 824273C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824273CC: 4810DCED  bl 0x825350b8
	ctx.lr = 0x824273D0;
	sub_82535080(ctx, base);
	// 824273D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824273D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824273D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824273DC: 409A0018  bne cr6, 0x824273f4
	if !ctx.cr[6].eq {
	pc = 0x824273F4; continue 'dispatch;
	}
	// 824273E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824273E4: 386B20D4  addi r3, r11, 0x20d4
	ctx.r[3].s64 = ctx.r[11].s64 + 8404;
	// 824273E8: 480039B9  bl 0x8242ada0
	ctx.lr = 0x824273EC;
	sub_8242ADA0(ctx, base);
	// 824273EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824273F0: 480000E8  b 0x824274d8
	pc = 0x824274D8; continue 'dispatch;
	// 824273F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824273F8: 48007B59  bl 0x8242ef50
	ctx.lr = 0x824273FC;
	sub_8242EF50(ctx, base);
	// 824273FC: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 82427400: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82427404: 394B7CA0  addi r10, r11, 0x7ca0
	ctx.r[10].s64 = ctx.r[11].s64 + 31904;
	// 82427408: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8242740C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82427410: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82427414: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427418: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8242741C: 419A0020  beq cr6, 0x8242743c
	if ctx.cr[6].eq {
	pc = 0x8242743C; continue 'dispatch;
	}
	// 82427420: 3D0A0001  addis r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 65536;
	// 82427424: 396B0238  addi r11, r11, 0x238
	ctx.r[11].s64 = ctx.r[11].s64 + 568;
	// 82427428: 39088E00  addi r8, r8, -0x7200
	ctx.r[8].s64 = ctx.r[8].s64 + -29184;
	// 8242742C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82427430: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82427434: 4198FFE0  blt cr6, 0x82427414
	if ctx.cr[6].lt {
	pc = 0x82427414; continue 'dispatch;
	}
	// 82427438: 48000010  b 0x82427448
	pc = 0x82427448; continue 'dispatch;
	// 8242743C: 1D690238  mulli r11, r9, 0x238
	ctx.r[11].s64 = ctx.r[9].s64 * 568;
	// 82427440: 7FEB5215  add. r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82427444: 40820014  bne 0x82427458
	if !ctx.cr[0].eq {
	pc = 0x82427458; continue 'dispatch;
	}
	// 82427448: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242744C: 386B20A4  addi r3, r11, 0x20a4
	ctx.r[3].s64 = ctx.r[11].s64 + 8356;
	// 82427450: 48003951  bl 0x8242ada0
	ctx.lr = 0x82427454;
	sub_8242ADA0(ctx, base);
	// 82427454: 48000078  b 0x824274cc
	pc = 0x824274CC; continue 'dispatch;
	// 82427458: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8242745C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82427460: 9BBF0001  stb r29, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 82427464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427468: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242746C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82427470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427474: 4E800421  bctrl
	ctx.lr = 0x82427478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82427478: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242747C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82427480: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82427484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82427488: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8242748C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427490: 4E800421  bctrl
	ctx.lr = 0x82427494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82427494: 7D7C1A14  add r11, r28, r3
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[3].u64;
	// 82427498: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8242749C: 55671838  slwi r7, r11, 3
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824274A0: 393F0050  addi r9, r31, 0x50
	ctx.r[9].s64 = ctx.r[31].s64 + 80;
	// 824274A4: 7D0743D6  divw r8, r7, r8
	ctx.r[8].s32 = ctx.r[7].s32 / ctx.r[8].s32;
	// 824274A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824274AC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824274B0: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 824274B4: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824274B8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824274BC: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 824274C0: 4082FFF4  bne 0x824274b4
	if !ctx.cr[0].eq {
	pc = 0x824274B4; continue 'dispatch;
	}
	// 824274C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824274C8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824274CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824274D0: 48007A81  bl 0x8242ef50
	ctx.lr = 0x824274D4;
	sub_8242EF50(ctx, base);
	// 824274D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824274D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824274DC: 4810DC2C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824274E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824274E0 size=328
    let mut pc: u32 = 0x824274E0;
    'dispatch: loop {
        match pc {
            0x824274E0 => {
    //   block [0x824274E0..0x82427628)
	// 824274E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824274E4: 4810DBD9  bl 0x825350bc
	ctx.lr = 0x824274E8;
	sub_82535080(ctx, base);
	// 824274E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824274EC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824274F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824274F4: 409A0018  bne cr6, 0x8242750c
	if !ctx.cr[6].eq {
	pc = 0x8242750C; continue 'dispatch;
	}
	// 824274F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824274FC: 386B2130  addi r3, r11, 0x2130
	ctx.r[3].s64 = ctx.r[11].s64 + 8496;
	// 82427500: 480038A1  bl 0x8242ada0
	ctx.lr = 0x82427504;
	sub_8242ADA0(ctx, base);
	// 82427504: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427508: 48000118  b 0x82427620
	pc = 0x82427620; continue 'dispatch;
	// 8242750C: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82427510: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82427514: 4098FFF0  bge cr6, 0x82427504
	if !ctx.cr[6].lt {
	pc = 0x82427504; continue 'dispatch;
	}
	// 82427518: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8242751C: 409A0014  bne cr6, 0x82427530
	if !ctx.cr[6].eq {
	pc = 0x82427530; continue 'dispatch;
	}
	// 82427520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82427524: 386B2104  addi r3, r11, 0x2104
	ctx.r[3].s64 = ctx.r[11].s64 + 8452;
	// 82427528: 48003879  bl 0x8242ada0
	ctx.lr = 0x8242752C;
	sub_8242ADA0(ctx, base);
	// 8242752C: 4BFFFFD8  b 0x82427504
	pc = 0x82427504; continue 'dispatch;
	// 82427530: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82427534: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82427538: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8242753C: 6128FFFF  ori r8, r9, 0xffff
	ctx.r[8].u64 = ctx.r[9].u64 | 65535;
	// 82427540: 392B000F  addi r9, r11, 0xf
	ctx.r[9].s64 = ctx.r[11].s64 + 15;
	// 82427544: 7D232670  srawi r3, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 82427548: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 8242754C: 54632036  slwi r3, r3, 4
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82427550: 7D234850  subf r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	// 82427554: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82427558: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8242755C: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82427560: 81290038  lwz r9, 0x38(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 82427564: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82427568: 419A0008  beq cr6, 0x82427570
	if ctx.cr[6].eq {
	pc = 0x82427570; continue 'dispatch;
	}
	// 8242756C: 38690001  addi r3, r9, 1
	ctx.r[3].s64 = ctx.r[9].s64 + 1;
	// 82427570: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82427574: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82427578: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8242757C: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82427580: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82427584: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82427588: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8242758C: 8BE90000  lbz r31, 0(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427590: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82427594: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82427598: 409AFFF4  bne cr6, 0x8242758c
	if !ctx.cr[6].eq {
	pc = 0x8242758C; continue 'dispatch;
	}
	// 8242759C: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 824275A0: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 824275A4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824275A8: 5528003F  rotlwi. r8, r9, 0
	ctx.r[8].u64 = ((ctx.r[9].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824275AC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824275B0: 41820020  beq 0x824275d0
	if ctx.cr[0].eq {
	pc = 0x824275D0; continue 'dispatch;
	}
	// 824275B4: 7FE920AE  lbzx r31, r9, r4
	ctx.r[31].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824275B8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824275BC: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824275C0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824275C4: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 824275C8: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 824275CC: 4198FFE8  blt cr6, 0x824275b4
	if ctx.cr[6].lt {
	pc = 0x824275B4; continue 'dispatch;
	}
	// 824275D0: 90EB0014  stw r7, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824275D4: 90CB0010  stw r6, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 824275D8: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 824275DC: 93AB0018  stw r29, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824275E0: 93AB001C  stw r29, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 824275E4: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 824275E8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 824275EC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824275F0: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 824275F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824275F8: 7D272670  srawi r7, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 824275FC: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82427600: 7D070194  addze r8, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82427604: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82427608: 916A0024  stw r11, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8242760C: 7D684850  subf r11, r8, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82427610: 916A001C  stw r11, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82427614: 409A000C  bne cr6, 0x82427620
	if !ctx.cr[6].eq {
	pc = 0x82427620; continue 'dispatch;
	}
	// 82427618: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8242761C: 996A0001  stb r11, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82427620: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82427624: 4810DAE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427628 size=16
    let mut pc: u32 = 0x82427628;
    'dispatch: loop {
        match pc {
            0x82427628 => {
    //   block [0x82427628..0x82427638)
	// 82427628: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8242762C: 816A97CC  lwz r11, -0x6834(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26676 as u32) ) } as u64;
	// 82427630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82427634: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427638 size=28
    let mut pc: u32 = 0x82427638;
    'dispatch: loop {
        match pc {
            0x82427638 => {
    //   block [0x82427638..0x82427654)
	// 82427638: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8242763C: 808B97D4  lwz r4, -0x682c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26668 as u32) ) } as u64;
	// 82427640: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82427644: 806B97D0  lwz r3, -0x6830(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26672 as u32) ) } as u64;
	// 82427648: 816A97CC  lwz r11, -0x6834(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26676 as u32) ) } as u64;
	// 8242764C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82427650: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427654 size=4
    let mut pc: u32 = 0x82427654;
    'dispatch: loop {
        match pc {
            0x82427654 => {
    //   block [0x82427654..0x82427658)
	// 82427654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427658 size=64
    let mut pc: u32 = 0x82427658;
    'dispatch: loop {
        match pc {
            0x82427658 => {
    //   block [0x82427658..0x82427698)
	// 82427658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242765C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242766C: 4BFFBFCD  bl 0x82423638
	ctx.lr = 0x82427670;
	sub_82423638(ctx, base);
	// 82427670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427674: 4BFFFD55  bl 0x824273c8
	ctx.lr = 0x82427678;
	sub_824273C8(ctx, base);
	// 82427678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242767C: 480078CD  bl 0x8242ef48
	ctx.lr = 0x82427680;
	sub_8242EF48(ctx, base);
	// 82427680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82427688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242768C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427698 size=64
    let mut pc: u32 = 0x82427698;
    'dispatch: loop {
        match pc {
            0x82427698 => {
    //   block [0x82427698..0x824276D8)
	// 82427698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824276A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824276A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824276A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824276AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824276B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824276B4: 4BFFBF85  bl 0x82423638
	ctx.lr = 0x824276B8;
	sub_82423638(ctx, base);
	// 824276B8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 824276BC: 4800788D  bl 0x8242ef48
	ctx.lr = 0x824276C0;
	sub_8242EF48(ctx, base);
	// 824276C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824276C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824276C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824276CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824276D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824276D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824276D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824276D8 size=80
    let mut pc: u32 = 0x824276D8;
    'dispatch: loop {
        match pc {
            0x824276D8 => {
    //   block [0x824276D8..0x82427728)
	// 824276D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824276DC: 4810D9D9  bl 0x825350b4
	ctx.lr = 0x824276E0;
	sub_82535080(ctx, base);
	// 824276E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824276E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824276E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824276EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824276F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824276F4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 824276F8: 4BFFBF41  bl 0x82423638
	ctx.lr = 0x824276FC;
	sub_82423638(ctx, base);
	// 824276FC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82427700: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82427704: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82427708: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8242770C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427710: 4BFFFDD1  bl 0x824274e0
	ctx.lr = 0x82427714;
	sub_824274E0(ctx, base);
	// 82427714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82427718: 48007831  bl 0x8242ef48
	ctx.lr = 0x8242771C;
	sub_8242EF48(ctx, base);
	// 8242771C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82427724: 4810D9E0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427728 size=96
    let mut pc: u32 = 0x82427728;
    'dispatch: loop {
        match pc {
            0x82427728 => {
    //   block [0x82427728..0x82427788)
	// 82427728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242773C: 4BFFBEFD  bl 0x82423638
	ctx.lr = 0x82427740;
	sub_82423638(ctx, base);
	// 82427740: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82427744: 409A0014  bne cr6, 0x82427758
	if !ctx.cr[6].eq {
	pc = 0x82427758; continue 'dispatch;
	}
	// 82427748: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242774C: 386B215C  addi r3, r11, 0x215c
	ctx.r[3].s64 = ctx.r[11].s64 + 8540;
	// 82427750: 48003651  bl 0x8242ada0
	ctx.lr = 0x82427754;
	sub_8242ADA0(ctx, base);
	// 82427754: 4800001C  b 0x82427770
	pc = 0x82427770; continue 'dispatch;
	// 82427758: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8242775C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82427760: 409A0010  bne cr6, 0x82427770
	if !ctx.cr[6].eq {
	pc = 0x82427770; continue 'dispatch;
	}
	// 82427764: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82427768: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8242776C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82427770: 480077D9  bl 0x8242ef48
	ctx.lr = 0x82427774;
	sub_8242EF48(ctx, base);
	// 82427774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82427778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242777C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427788 size=104
    let mut pc: u32 = 0x82427788;
    'dispatch: loop {
        match pc {
            0x82427788 => {
    //   block [0x82427788..0x824277F0)
	// 82427788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242778C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82427794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242779C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824277A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824277A4: 4BFFBE95  bl 0x82423638
	ctx.lr = 0x824277A8;
	sub_82423638(ctx, base);
	// 824277A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824277AC: 409A0014  bne cr6, 0x824277c0
	if !ctx.cr[6].eq {
	pc = 0x824277C0; continue 'dispatch;
	}
	// 824277B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824277B4: 386B2188  addi r3, r11, 0x2188
	ctx.r[3].s64 = ctx.r[11].s64 + 8584;
	// 824277B8: 480035E9  bl 0x8242ada0
	ctx.lr = 0x824277BC;
	sub_8242ADA0(ctx, base);
	// 824277BC: 48000018  b 0x824277d4
	pc = 0x824277D4; continue 'dispatch;
	// 824277C0: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 824277C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824277C8: 419A0008  beq cr6, 0x824277d0
	if ctx.cr[6].eq {
	pc = 0x824277D0; continue 'dispatch;
	}
	// 824277CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824277D0: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 824277D4: 48007775  bl 0x8242ef48
	ctx.lr = 0x824277D8;
	sub_8242EF48(ctx, base);
	// 824277D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824277DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824277E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824277E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824277E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824277EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824277F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824277F0 size=104
    let mut pc: u32 = 0x824277F0;
    'dispatch: loop {
        match pc {
            0x824277F0 => {
    //   block [0x824277F0..0x82427858)
	// 824277F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824277F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824277F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824277FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427804: 4BFFBE35  bl 0x82423638
	ctx.lr = 0x82427808;
	sub_82423638(ctx, base);
	// 82427808: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242780C: 3BCB7CA0  addi r30, r11, 0x7ca0
	ctx.r[30].s64 = ctx.r[11].s64 + 31904;
	// 82427810: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82427814: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427818: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8242781C: 409A000C  bne cr6, 0x82427828
	if !ctx.cr[6].eq {
	pc = 0x82427828; continue 'dispatch;
	}
	// 82427820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427824: 4800799D  bl 0x8242f1c0
	ctx.lr = 0x82427828;
	sub_8242F1C0(ctx, base);
	// 82427828: 3D7E0001  addis r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 65536;
	// 8242782C: 3BFF0238  addi r31, r31, 0x238
	ctx.r[31].s64 = ctx.r[31].s64 + 568;
	// 82427830: 396B8E00  addi r11, r11, -0x7200
	ctx.r[11].s64 = ctx.r[11].s64 + -29184;
	// 82427834: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82427838: 4198FFDC  blt cr6, 0x82427814
	if ctx.cr[6].lt {
	pc = 0x82427814; continue 'dispatch;
	}
	// 8242783C: 4800770D  bl 0x8242ef48
	ctx.lr = 0x82427840;
	sub_8242EF48(ctx, base);
	// 82427840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82427844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242784C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82427850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427858 size=88
    let mut pc: u32 = 0x82427858;
    'dispatch: loop {
        match pc {
            0x82427858 => {
    //   block [0x82427858..0x824278B0)
	// 82427858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242786C: 4BFFBDCD  bl 0x82423638
	ctx.lr = 0x82427870;
	sub_82423638(ctx, base);
	// 82427870: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82427874: 409A0018  bne cr6, 0x8242788c
	if !ctx.cr[6].eq {
	pc = 0x8242788C; continue 'dispatch;
	}
	// 82427878: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242787C: 386B21B4  addi r3, r11, 0x21b4
	ctx.r[3].s64 = ctx.r[11].s64 + 8628;
	// 82427880: 48003521  bl 0x8242ada0
	ctx.lr = 0x82427884;
	sub_8242ADA0(ctx, base);
	// 82427884: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82427888: 4800000C  b 0x82427894
	pc = 0x82427894; continue 'dispatch;
	// 8242788C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82427890: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 82427894: 480076B5  bl 0x8242ef48
	ctx.lr = 0x82427898;
	sub_8242EF48(ctx, base);
	// 82427898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242789C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824278A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824278A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824278A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824278AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824278B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824278B0 size=84
    let mut pc: u32 = 0x824278B0;
    'dispatch: loop {
        match pc {
            0x824278B0 => {
    //   block [0x824278B0..0x82427904)
	// 824278B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824278B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824278B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824278BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824278C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824278C4: 4BFFBD75  bl 0x82423638
	ctx.lr = 0x824278C8;
	sub_82423638(ctx, base);
	// 824278C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824278CC: 409A0018  bne cr6, 0x824278e4
	if !ctx.cr[6].eq {
	pc = 0x824278E4; continue 'dispatch;
	}
	// 824278D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824278D4: 386B21E0  addi r3, r11, 0x21e0
	ctx.r[3].s64 = ctx.r[11].s64 + 8672;
	// 824278D8: 480034C9  bl 0x8242ada0
	ctx.lr = 0x824278DC;
	sub_8242ADA0(ctx, base);
	// 824278DC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 824278E0: 48000008  b 0x824278e8
	pc = 0x824278E8; continue 'dispatch;
	// 824278E4: 83FF0024  lwz r31, 0x24(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824278E8: 48007661  bl 0x8242ef48
	ctx.lr = 0x824278EC;
	sub_8242EF48(ctx, base);
	// 824278EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824278F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824278F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824278F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824278FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427908 size=128
    let mut pc: u32 = 0x82427908;
    'dispatch: loop {
        match pc {
            0x82427908 => {
    //   block [0x82427908..0x82427988)
	// 82427908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82427914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242791C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82427920: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82427924: 4BFFBD15  bl 0x82423638
	ctx.lr = 0x82427928;
	sub_82423638(ctx, base);
	// 82427928: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242792C: 409A0014  bne cr6, 0x82427940
	if !ctx.cr[6].eq {
	pc = 0x82427940; continue 'dispatch;
	}
	// 82427930: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82427934: 386B2238  addi r3, r11, 0x2238
	ctx.r[3].s64 = ctx.r[11].s64 + 8760;
	// 82427938: 48003469  bl 0x8242ada0
	ctx.lr = 0x8242793C;
	sub_8242ADA0(ctx, base);
	// 8242793C: 48000030  b 0x8242796c
	pc = 0x8242796C; continue 'dispatch;
	// 82427940: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82427944: 41980018  blt cr6, 0x8242795c
	if ctx.cr[6].lt {
	pc = 0x8242795C; continue 'dispatch;
	}
	// 82427948: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8242794C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82427950: 4199000C  bgt cr6, 0x8242795c
	if ctx.cr[6].gt {
	pc = 0x8242795C; continue 'dispatch;
	}
	// 82427954: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82427958: 48000014  b 0x8242796c
	pc = 0x8242796C; continue 'dispatch;
	// 8242795C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82427960: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82427964: 386B220C  addi r3, r11, 0x220c
	ctx.r[3].s64 = ctx.r[11].s64 + 8716;
	// 82427968: 48003439  bl 0x8242ada0
	ctx.lr = 0x8242796C;
	sub_8242ADA0(ctx, base);
	// 8242796C: 480075DD  bl 0x8242ef48
	ctx.lr = 0x82427970;
	sub_8242EF48(ctx, base);
	// 82427970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82427974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242797C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82427980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427988 size=88
    let mut pc: u32 = 0x82427988;
    'dispatch: loop {
        match pc {
            0x82427988 => {
    //   block [0x82427988..0x824279E0)
	// 82427988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242798C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82427994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242799C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824279A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824279A4: 4BFFBC95  bl 0x82423638
	ctx.lr = 0x824279A8;
	sub_82423638(ctx, base);
	// 824279A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824279AC: 409A0014  bne cr6, 0x824279c0
	if !ctx.cr[6].eq {
	pc = 0x824279C0; continue 'dispatch;
	}
	// 824279B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824279B4: 386B2264  addi r3, r11, 0x2264
	ctx.r[3].s64 = ctx.r[11].s64 + 8804;
	// 824279B8: 480033E9  bl 0x8242ada0
	ctx.lr = 0x824279BC;
	sub_8242ADA0(ctx, base);
	// 824279BC: 48000008  b 0x824279c4
	pc = 0x824279C4; continue 'dispatch;
	// 824279C0: 9BDF0003  stb r30, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 824279C4: 48007585  bl 0x8242ef48
	ctx.lr = 0x824279C8;
	sub_8242EF48(ctx, base);
	// 824279C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824279CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824279D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824279D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824279D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824279DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824279E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824279E0 size=96
    let mut pc: u32 = 0x824279E0;
    'dispatch: loop {
        match pc {
            0x824279E0 => {
    //   block [0x824279E0..0x82427A40)
	// 824279E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824279E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824279E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824279EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824279F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824279F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824279F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824279FC: 4BFFBC3D  bl 0x82423638
	ctx.lr = 0x82427A00;
	sub_82423638(ctx, base);
	// 82427A00: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 82427A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82427A08: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 82427A0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82427A10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82427A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427A18: 4BFFFAC9  bl 0x824274e0
	ctx.lr = 0x82427A1C;
	sub_824274E0(ctx, base);
	// 82427A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82427A20: 48007529  bl 0x8242ef48
	ctx.lr = 0x82427A24;
	sub_8242EF48(ctx, base);
	// 82427A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82427A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427A34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82427A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427A40 size=148
    let mut pc: u32 = 0x82427A40;
    'dispatch: loop {
        match pc {
            0x82427A40 => {
    //   block [0x82427A40..0x82427AD4)
	// 82427A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82427A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82427A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82427A58: 4BFFBBE1  bl 0x82423638
	ctx.lr = 0x82427A5C;
	sub_82423638(ctx, base);
	// 82427A5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82427A60: 409A0014  bne cr6, 0x82427a74
	if !ctx.cr[6].eq {
	pc = 0x82427A74; continue 'dispatch;
	}
	// 82427A64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82427A68: 386B2290  addi r3, r11, 0x2290
	ctx.r[3].s64 = ctx.r[11].s64 + 8848;
	// 82427A6C: 48003335  bl 0x8242ada0
	ctx.lr = 0x82427A70;
	sub_8242ADA0(ctx, base);
	// 82427A70: 48000048  b 0x82427ab8
	pc = 0x82427AB8; continue 'dispatch;
	// 82427A74: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82427A78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82427A7C: 419A003C  beq cr6, 0x82427ab8
	if ctx.cr[6].eq {
	pc = 0x82427AB8; continue 'dispatch;
	}
	// 82427A80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82427A84: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82427A88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82427A8C: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 82427A90: 41820018  beq 0x82427aa8
	if ctx.cr[0].eq {
	pc = 0x82427AA8; continue 'dispatch;
	}
	// 82427A94: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82427A98: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82427A9C: 409A000C  bne cr6, 0x82427aa8
	if !ctx.cr[6].eq {
	pc = 0x82427AA8; continue 'dispatch;
	}
	// 82427AA0: 4BFFAAA1  bl 0x82422540
	ctx.lr = 0x82427AA4;
	sub_82422540(ctx, base);
	// 82427AA4: 9BDF0002  stb r30, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82427AA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427AAC: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82427AB0: 4BFFFC79  bl 0x82427728
	ctx.lr = 0x82427AB4;
	sub_82427728(ctx, base);
	// 82427AB4: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82427AB8: 48007491  bl 0x8242ef48
	ctx.lr = 0x82427ABC;
	sub_8242EF48(ctx, base);
	// 82427ABC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82427AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427AC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82427ACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427AD8 size=88
    let mut pc: u32 = 0x82427AD8;
    'dispatch: loop {
        match pc {
            0x82427AD8 => {
    //   block [0x82427AD8..0x82427B30)
	// 82427AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82427ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427AE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427AE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427AE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82427AEC: 4BFFBB4D  bl 0x82423638
	ctx.lr = 0x82427AF0;
	sub_82423638(ctx, base);
	// 82427AF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82427AF4: 419A0024  beq cr6, 0x82427b18
	if ctx.cr[6].eq {
	pc = 0x82427B18; continue 'dispatch;
	}
	// 82427AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427AFC: 4BFFFF45  bl 0x82427a40
	ctx.lr = 0x82427B00;
	sub_82427A40(ctx, base);
	// 82427B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82427B04: 38A00238  li r5, 0x238
	ctx.r[5].s64 = 568;
	// 82427B08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82427B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427B10: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82427B14: 4810D6BD  bl 0x825351d0
	ctx.lr = 0x82427B18;
	sub_825351D0(ctx, base);
	// 82427B18: 48007431  bl 0x8242ef48
	ctx.lr = 0x82427B1C;
	sub_8242EF48(ctx, base);
	// 82427B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82427B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427B28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82427B30 size=116
    let mut pc: u32 = 0x82427B30;
    'dispatch: loop {
        match pc {
            0x82427B30 => {
    //   block [0x82427B30..0x82427BA4)
	// 82427B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82427B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82427B44: 4BFFBAF5  bl 0x82423638
	ctx.lr = 0x82427B48;
	sub_82423638(ctx, base);
	// 82427B48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82427B4C: 409A0014  bne cr6, 0x82427b60
	if !ctx.cr[6].eq {
	pc = 0x82427B60; continue 'dispatch;
	}
	// 82427B50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82427B54: 386B22BC  addi r3, r11, 0x22bc
	ctx.r[3].s64 = ctx.r[11].s64 + 8892;
	// 82427B58: 48003249  bl 0x8242ada0
	ctx.lr = 0x82427B5C;
	sub_8242ADA0(ctx, base);
	// 82427B5C: 48000030  b 0x82427b8c
	pc = 0x82427B8C; continue 'dispatch;
	// 82427B60: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82427B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82427B68: 419A000C  beq cr6, 0x82427b74
	if ctx.cr[6].eq {
	pc = 0x82427B74; continue 'dispatch;
	}
	// 82427B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82427B70: 4BFFFED1  bl 0x82427a40
	ctx.lr = 0x82427B74;
	sub_82427A40(ctx, base);
	// 82427B74: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82427B78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82427B7C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82427B80: 41990008  bgt cr6, 0x82427b88
	if ctx.cr[6].gt {
	pc = 0x82427B88; continue 'dispatch;
	}
	// 82427B84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82427B88: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82427B8C: 480073BD  bl 0x8242ef48
	ctx.lr = 0x82427B90;
	sub_8242EF48(ctx, base);
	// 82427B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82427B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427B9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82427BA8 size=236
    let mut pc: u32 = 0x82427BA8;
    'dispatch: loop {
        match pc {
            0x82427BA8 => {
    //   block [0x82427BA8..0x82427C94)
	// 82427BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82427BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82427BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82427BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82427BB8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82427BBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82427BC0: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 82427BC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82427BC8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82427BCC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82427BD0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82427BD4: C80B2020  lfd f0, 0x2020(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8224 as u32) ) };
	// 82427BD8: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 82427BDC: EFE0002C  fsqrts f31, f0
	ctx.f[31].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82427BE0: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82427BE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82427BE8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82427BEC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82427BF0: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82427BF4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82427BF8: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82427BFC: C00B2204  lfs f0, 0x2204(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82427C00: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82427C04: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82427C08: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82427C0C: 4810E71D  bl 0x82536328
	ctx.lr = 0x82427C10;
	sub_82536328(ctx, base);
	// 82427C10: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82427C14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82427C18: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82427C1C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82427C20: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82427C24: EDBF6828  fsubs f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[13].f64) as f32) as f64);
	// 82427C28: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82427C2C: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82427C30: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82427C34: ED80602C  fsqrts f12, f12
	ctx.f[12].f64 = ((ctx.f[12].f64).sqrt() as f32) as f64;
	// 82427C38: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82427C3C: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82427C40: C1AB26B8  lfs f13, 0x26b8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82427C44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82427C48: ED800032  fmuls f12, f0, f0
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82427C4C: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82427C50: C00B243C  lfs f0, 0x243c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82427C54: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82427C58: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82427C5C: D9A10058  stfd f13, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[13].u64 ) };
	// 82427C60: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82427C64: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82427C68: A161005E  lhz r11, 0x5e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 82427C6C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427C70: A1610056  lhz r11, 0x56(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82427C74: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82427C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82427C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82427C84: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82427C88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82427C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82427C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427C98 size=52
    let mut pc: u32 = 0x82427C98;
    'dispatch: loop {
        match pc {
            0x82427C98 => {
    //   block [0x82427C98..0x82427CCC)
	// 82427C98: 3544FFFF  addic. r10, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82427C9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82427CA0: 4081001C  ble 0x82427cbc
	if !ctx.cr[0].gt {
	pc = 0x82427CBC; continue 'dispatch;
	}
	// 82427CA4: 7D2B1A2E  lhzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82427CA8: 2B098000  cmplwi cr6, r9, 0x8000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32768 as u32, &mut ctx.xer);
	// 82427CAC: 419A0020  beq cr6, 0x82427ccc
	if ctx.cr[6].eq {
		sub_82427CCC(ctx, base);
		return;
	}
	// 82427CB0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82427CB4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82427CB8: 4198FFEC  blt cr6, 0x82427ca4
	if ctx.cr[6].lt {
	pc = 0x82427CA4; continue 'dispatch;
	}
	// 82427CBC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82427CC4: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427CCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427CCC size=24
    let mut pc: u32 = 0x82427CCC;
    'dispatch: loop {
        match pc {
            0x82427CCC => {
    //   block [0x82427CCC..0x82427CE4)
	// 82427CCC: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82427CD0: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82427CD4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82427CD8: 4098FFE4  bge cr6, 0x82427cbc
	if !ctx.cr[6].lt {
		sub_82427C98(ctx, base);
		return;
	}
	// 82427CDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427CE0: 4BFFFFE4  b 0x82427cc4
	sub_82427C98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427CE8 size=16
    let mut pc: u32 = 0x82427CE8;
    'dispatch: loop {
        match pc {
            0x82427CE8 => {
    //   block [0x82427CE8..0x82427CF8)
	// 82427CE8: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 82427CEC: 4098000C  bge cr6, 0x82427cf8
	if !ctx.cr[6].lt {
		sub_82427CF8(ctx, base);
		return;
	}
	// 82427CF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427CF8 size=36
    let mut pc: u32 = 0x82427CF8;
    'dispatch: loop {
        match pc {
            0x82427CF8 => {
    //   block [0x82427CF8..0x82427D1C)
	// 82427CF8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427CFC: 88830001  lbz r4, 1(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82427D00: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82427D04: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82427D08: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82427D0C: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 82427D10: 419A000C  beq cr6, 0x82427d1c
	if ctx.cr[6].eq {
		sub_82427D1C(ctx, base);
		return;
	}
	// 82427D14: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82427D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427D1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427D1C size=168
    let mut pc: u32 = 0x82427D1C;
    'dispatch: loop {
        match pc {
            0x82427D1C => {
    //   block [0x82427D1C..0x82427DC4)
	// 82427D1C: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82427D20: 88830003  lbz r4, 3(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82427D24: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82427D28: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82427D2C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82427D30: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427D34: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82427D38: 99660000  stb r11, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82427D3C: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 82427D40: 99680000  stb r11, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82427D44: 89630006  lbz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82427D48: 99670000  stb r11, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82427D4C: 89630007  lbz r11, 7(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 82427D50: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82427D54: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82427D58: 89230009  lbz r9, 9(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(9 as u32) ) } as u64;
	// 82427D5C: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82427D60: 88C3000A  lbz r6, 0xa(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82427D64: 88A3000B  lbz r5, 0xb(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11 as u32) ) } as u64;
	// 82427D68: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82427D6C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82427D70: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 82427D74: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82427D78: 7D6B2B78  or r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 82427D7C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82427D80: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82427D84: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82427D88: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82427D8C: 8923000E  lbz r9, 0xe(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82427D90: 88C3000F  lbz r6, 0xf(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 82427D94: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82427D98: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82427D9C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82427DA0: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82427DA4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82427DA8: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 82427DAC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82427DB0: 89670000  lbz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427DB4: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82427DB8: 4082000C  bne 0x82427dc4
	if !ctx.cr[0].eq {
		sub_82427DC4(ctx, base);
		return;
	}
	// 82427DBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82427DC0: 48000018  b 0x82427dd8
	sub_82427DC4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427DC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427DC4 size=36
    let mut pc: u32 = 0x82427DC4;
    'dispatch: loop {
        match pc {
            0x82427DC4 => {
    //   block [0x82427DC4..0x82427DE8)
	// 82427DC4: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427DC8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82427DCC: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82427DD0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82427DD4: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 82427DD8: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82427DDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427DE0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82427DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427DE8 size=16
    let mut pc: u32 = 0x82427DE8;
    'dispatch: loop {
        match pc {
            0x82427DE8 => {
    //   block [0x82427DE8..0x82427DF8)
	// 82427DE8: 2F040012  cmpwi cr6, r4, 0x12
	ctx.cr[6].compare_i32(ctx.r[4].s32, 18, &mut ctx.xer);
	// 82427DEC: 4098000C  bge cr6, 0x82427df8
	if !ctx.cr[6].lt {
		sub_82427DF8(ctx, base);
		return;
	}
	// 82427DF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427DF8 size=20
    let mut pc: u32 = 0x82427DF8;
    'dispatch: loop {
        match pc {
            0x82427DF8 => {
    //   block [0x82427DF8..0x82427E0C)
	// 82427DF8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427DFC: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 82427E00: 419A000C  beq cr6, 0x82427e0c
	if ctx.cr[6].eq {
		sub_82427E0C(ctx, base);
		return;
	}
	// 82427E04: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82427E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427E0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427E0C size=28
    let mut pc: u32 = 0x82427E0C;
    'dispatch: loop {
        match pc {
            0x82427E0C => {
    //   block [0x82427E0C..0x82427E28)
	// 82427E0C: A9630002  lha r11, 2(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82427E10: 2F0B000E  cmpwi cr6, r11, 0xe
	ctx.cr[6].compare_i32(ctx.r[11].s32, 14, &mut ctx.xer);
	// 82427E14: 4198FFDC  blt cr6, 0x82427df0
	if ctx.cr[6].lt {
		sub_82427DE8(ctx, base);
		return;
	}
	// 82427E18: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82427E1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427E20: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82427E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427E28 size=20
    let mut pc: u32 = 0x82427E28;
    'dispatch: loop {
        match pc {
            0x82427E28 => {
    //   block [0x82427E28..0x82427E3C)
	// 82427E28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82427E2C: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 82427E30: 4098000C  bge cr6, 0x82427e3c
	if !ctx.cr[6].lt {
		sub_82427E3C(ctx, base);
		return;
	}
	// 82427E34: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82427E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427E3C size=20
    let mut pc: u32 = 0x82427E3C;
    'dispatch: loop {
        match pc {
            0x82427E3C => {
    //   block [0x82427E3C..0x82427E50)
	// 82427E3C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82427E40: 2B0A8000  cmplwi cr6, r10, 0x8000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32768 as u32, &mut ctx.xer);
	// 82427E44: 419A000C  beq cr6, 0x82427e50
	if ctx.cr[6].eq {
		sub_82427E50(ctx, base);
		return;
	}
	// 82427E48: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82427E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82427E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82427E50 size=36
    let mut pc: u32 = 0x82427E50;
    'dispatch: loop {
        match pc {
            0x82427E50 => {
    //   block [0x82427E50..0x82427E74)
	// 82427E50: A94B0002  lha r10, 2(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82427E54: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82427E58: 4198FFDC  blt cr6, 0x82427e34
	if ctx.cr[6].lt {
		sub_82427E28(ctx, base);
		return;
	}
	// 82427E5C: 894B0012  lbz r10, 0x12(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82427E60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82427E64: 99450000  stb r10, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82427E68: 896B0013  lbz r11, 0x13(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 82427E6C: 99660000  stb r11, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82427E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


