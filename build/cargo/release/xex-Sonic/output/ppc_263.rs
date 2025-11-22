pub fn sub_830DA880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DA880 size=12
    let mut pc: u32 = 0x830DA880;
    'dispatch: loop {
        match pc {
            0x830DA880 => {
    //   block [0x830DA880..0x830DA88C)
	// 830DA880: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DA884: 386B7DE0  addi r3, r11, 0x7de0
	ctx.r[3].s64 = ctx.r[11].s64 + 32224;
	// 830DA888: 4BFFEE40  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DA890 size=176
    let mut pc: u32 = 0x830DA890;
    'dispatch: loop {
        match pc {
            0x830DA890 => {
    //   block [0x830DA890..0x830DA940)
	// 830DA890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DA894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DA898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DA89C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DA8A0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DA8A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DA8A8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DA8AC: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DA8B0: 394A7E00  addi r10, r10, 0x7e00
	ctx.r[10].s64 = ctx.r[10].s64 + 32256;
	// 830DA8B4: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DA8B8: 39297BA0  addi r9, r9, 0x7ba0
	ctx.r[9].s64 = ctx.r[9].s64 + 31648;
	// 830DA8BC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DA8C0: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DA8C4: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DA8C8: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DA8CC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DA8D0: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DA8D4: 4B6F4905  bl 0x827cf1d8
	ctx.lr = 0x830DA8D8;
	sub_827CF1D8(ctx, base);
	// 830DA8D8: 3D60830D  lis r11, -0x7cf3
	ctx.r[11].s64 = -2096300032;
	// 830DA8DC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DA8E0: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DA8E4: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DA8E8: 396B3078  addi r11, r11, 0x3078
	ctx.r[11].s64 = ctx.r[11].s64 + 12408;
	// 830DA8EC: 394A30F0  addi r10, r10, 0x30f0
	ctx.r[10].s64 = ctx.r[10].s64 + 12528;
	// 830DA8F0: 3929EEA8  addi r9, r9, -0x1158
	ctx.r[9].s64 = ctx.r[9].s64 + -4440;
	// 830DA8F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DA8F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DA8FC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DA900: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DA904: 4BFFDA3D  bl 0x830d8340
	ctx.lr = 0x830DA908;
	sub_830D8340(ctx, base);
	// 830DA908: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DA90C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DA910: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DA914: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DA918: 388BD8EC  addi r4, r11, -0x2714
	ctx.r[4].s64 = ctx.r[11].s64 + -10004;
	// 830DA91C: 4BFFEB2D  bl 0x830d9448
	ctx.lr = 0x830DA920;
	sub_830D9448(ctx, base);
	// 830DA920: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DA924: 41800008  blt 0x830da92c
	if ctx.cr[0].lt {
	pc = 0x830DA92C; continue 'dispatch;
	}
	// 830DA928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DA92C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DA930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DA934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DA938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DA93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DA940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DA940 size=12
    let mut pc: u32 = 0x830DA940;
    'dispatch: loop {
        match pc {
            0x830DA940 => {
    //   block [0x830DA940..0x830DA94C)
	// 830DA940: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DA944: 386B7E00  addi r3, r11, 0x7e00
	ctx.r[3].s64 = ctx.r[11].s64 + 32256;
	// 830DA948: 4BFFED80  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DA950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DA950 size=176
    let mut pc: u32 = 0x830DA950;
    'dispatch: loop {
        match pc {
            0x830DA950 => {
    //   block [0x830DA950..0x830DAA00)
	// 830DA950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DA954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DA958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DA95C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DA960: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DA964: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DA968: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DA96C: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DA970: 394A7E14  addi r10, r10, 0x7e14
	ctx.r[10].s64 = ctx.r[10].s64 + 32276;
	// 830DA974: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DA978: 39297BA0  addi r9, r9, 0x7ba0
	ctx.r[9].s64 = ctx.r[9].s64 + 31648;
	// 830DA97C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DA980: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DA984: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DA988: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DA98C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DA990: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DA994: 4B6F4845  bl 0x827cf1d8
	ctx.lr = 0x830DA998;
	sub_827CF1D8(ctx, base);
	// 830DA998: 3D60830D  lis r11, -0x7cf3
	ctx.r[11].s64 = -2096300032;
	// 830DA99C: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DA9A0: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DA9A4: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DA9A8: 396B63C0  addi r11, r11, 0x63c0
	ctx.r[11].s64 = ctx.r[11].s64 + 25536;
	// 830DA9AC: 394A6438  addi r10, r10, 0x6438
	ctx.r[10].s64 = ctx.r[10].s64 + 25656;
	// 830DA9B0: 3929EEB0  addi r9, r9, -0x1150
	ctx.r[9].s64 = ctx.r[9].s64 + -4432;
	// 830DA9B4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DA9B8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DA9BC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DA9C0: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DA9C4: 4BFFDC3D  bl 0x830d8600
	ctx.lr = 0x830DA9C8;
	sub_830D8600(ctx, base);
	// 830DA9C8: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DA9CC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DA9D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DA9D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DA9D8: 388BC0C8  addi r4, r11, -0x3f38
	ctx.r[4].s64 = ctx.r[11].s64 + -16184;
	// 830DA9DC: 4BFFEA6D  bl 0x830d9448
	ctx.lr = 0x830DA9E0;
	sub_830D9448(ctx, base);
	// 830DA9E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DA9E4: 41800008  blt 0x830da9ec
	if ctx.cr[0].lt {
	pc = 0x830DA9EC; continue 'dispatch;
	}
	// 830DA9E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DA9EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DA9F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DA9F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DA9F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DA9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAA00 size=12
    let mut pc: u32 = 0x830DAA00;
    'dispatch: loop {
        match pc {
            0x830DAA00 => {
    //   block [0x830DAA00..0x830DAA0C)
	// 830DAA00: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAA04: 386B7E14  addi r3, r11, 0x7e14
	ctx.r[3].s64 = ctx.r[11].s64 + 32276;
	// 830DAA08: 4BFFECC0  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAA10 size=160
    let mut pc: u32 = 0x830DAA10;
    'dispatch: loop {
        match pc {
            0x830DAA10 => {
    //   block [0x830DAA10..0x830DAAB0)
	// 830DAA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAA1C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 830DAA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DAA24: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DAA28: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DAA2C: 39297E24  addi r9, r9, 0x7e24
	ctx.r[9].s64 = ctx.r[9].s64 + 32292;
	// 830DAA30: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 830DAA34: 3CA0830D  lis r5, -0x7cf3
	ctx.r[5].s64 = -2096300032;
	// 830DAA38: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 830DAA3C: 3D008218  lis r8, -0x7de8
	ctx.r[8].s64 = -2112356352;
	// 830DAA40: F96A0010  std r11, 0x10(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 830DAA44: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DAA48: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 830DAA4C: 39087BA0  addi r8, r8, 0x7ba0
	ctx.r[8].s64 = ctx.r[8].s64 + 31648;
	// 830DAA50: F96A0020  std r11, 0x20(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 830DAA54: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830DAA58: 38E76160  addi r7, r7, 0x6160
	ctx.r[7].s64 = ctx.r[7].s64 + 24928;
	// 830DAA5C: 3946EE30  addi r10, r6, -0x11d0
	ctx.r[10].s64 = ctx.r[6].s64 + -4560;
	// 830DAA60: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 830DAA64: 3925EEB8  addi r9, r5, -0x1148
	ctx.r[9].s64 = ctx.r[5].s64 + -4424;
	// 830DAA68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830DAA6C: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 830DAA70: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830DAA74: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DAA78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAA7C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DAA80: 3888C0CC  addi r4, r8, -0x3f34
	ctx.r[4].s64 = ctx.r[8].s64 + -16180;
	// 830DAA84: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830DAA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830DAA8C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830DAA90: 4BFFE9B9  bl 0x830d9448
	ctx.lr = 0x830DAA94;
	sub_830D9448(ctx, base);
	// 830DAA94: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAA98: 41800008  blt 0x830daaa0
	if ctx.cr[0].lt {
	pc = 0x830DAAA0; continue 'dispatch;
	}
	// 830DAA9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAAA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DAAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DAAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DAAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAAB0 size=12
    let mut pc: u32 = 0x830DAAB0;
    'dispatch: loop {
        match pc {
            0x830DAAB0 => {
    //   block [0x830DAAB0..0x830DAABC)
	// 830DAAB0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAAB4: 386B7E24  addi r3, r11, 0x7e24
	ctx.r[3].s64 = ctx.r[11].s64 + 32292;
	// 830DAAB8: 4BFFEC10  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAAC0 size=172
    let mut pc: u32 = 0x830DAAC0;
    'dispatch: loop {
        match pc {
            0x830DAAC0 => {
    //   block [0x830DAAC0..0x830DAB6C)
	// 830DAAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAAC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DAACC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAAD0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DAAD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DAAD8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DAADC: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DAAE0: 394A7E38  addi r10, r10, 0x7e38
	ctx.r[10].s64 = ctx.r[10].s64 + 32312;
	// 830DAAE4: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DAAE8: 39297BFC  addi r9, r9, 0x7bfc
	ctx.r[9].s64 = ctx.r[9].s64 + 31740;
	// 830DAAEC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DAAF0: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DAAF4: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DAAF8: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DAAFC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DAB00: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DAB04: 4800B95D  bl 0x830e6460
	ctx.lr = 0x830DAB08;
	sub_830E6460(ctx, base);
	// 830DAB08: 3D60830D  lis r11, -0x7cf3
	ctx.r[11].s64 = -2096300032;
	// 830DAB0C: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DAB10: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DAB14: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 830DAB18: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DAB1C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 830DAB20: 396B6478  addi r11, r11, 0x6478
	ctx.r[11].s64 = ctx.r[11].s64 + 25720;
	// 830DAB24: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DAB28: 394AEE30  addi r10, r10, -0x11d0
	ctx.r[10].s64 = ctx.r[10].s64 + -4560;
	// 830DAB2C: 3929EEC0  addi r9, r9, -0x1140
	ctx.r[9].s64 = ctx.r[9].s64 + -4416;
	// 830DAB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DAB34: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DAB38: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DAB3C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DAB40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAB44: 388BC0D0  addi r4, r11, -0x3f30
	ctx.r[4].s64 = ctx.r[11].s64 + -16176;
	// 830DAB48: 4BFFE901  bl 0x830d9448
	ctx.lr = 0x830DAB4C;
	sub_830D9448(ctx, base);
	// 830DAB4C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAB50: 41800008  blt 0x830dab58
	if ctx.cr[0].lt {
	pc = 0x830DAB58; continue 'dispatch;
	}
	// 830DAB54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAB58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DAB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DAB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DAB64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DAB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAB70 size=12
    let mut pc: u32 = 0x830DAB70;
    'dispatch: loop {
        match pc {
            0x830DAB70 => {
    //   block [0x830DAB70..0x830DAB7C)
	// 830DAB70: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAB74: 386B7E38  addi r3, r11, 0x7e38
	ctx.r[3].s64 = ctx.r[11].s64 + 32312;
	// 830DAB78: 4BFFEB50  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAB80 size=172
    let mut pc: u32 = 0x830DAB80;
    'dispatch: loop {
        match pc {
            0x830DAB80 => {
    //   block [0x830DAB80..0x830DAC2C)
	// 830DAB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAB88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DAB8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAB90: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DAB94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DAB98: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DAB9C: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DABA0: 394A7E54  addi r10, r10, 0x7e54
	ctx.r[10].s64 = ctx.r[10].s64 + 32340;
	// 830DABA4: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DABA8: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DABAC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DABB0: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DABB4: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DABB8: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DABBC: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DABC0: 39297B64  addi r9, r9, 0x7b64
	ctx.r[9].s64 = ctx.r[9].s64 + 31588;
	// 830DABC4: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DABC8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DABCC: 390864D8  addi r8, r8, 0x64d8
	ctx.r[8].s64 = ctx.r[8].s64 + 25816;
	// 830DABD0: 39676568  addi r11, r7, 0x6568
	ctx.r[11].s64 = ctx.r[7].s64 + 25960;
	// 830DABD4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DABD8: 3946EEC8  addi r10, r6, -0x1138
	ctx.r[10].s64 = ctx.r[6].s64 + -4408;
	// 830DABDC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DABE0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DABE4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DABE8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DABEC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DABF0: 4BFF5E91  bl 0x830d0a80
	ctx.lr = 0x830DABF4;
	sub_830D0A80(ctx, base);
	// 830DABF4: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DABF8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DABFC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DAC00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAC04: 388BC0D4  addi r4, r11, -0x3f2c
	ctx.r[4].s64 = ctx.r[11].s64 + -16172;
	// 830DAC08: 4BFFE841  bl 0x830d9448
	ctx.lr = 0x830DAC0C;
	sub_830D9448(ctx, base);
	// 830DAC0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAC10: 41800008  blt 0x830dac18
	if ctx.cr[0].lt {
	pc = 0x830DAC18; continue 'dispatch;
	}
	// 830DAC14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAC18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DAC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DAC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DAC24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DAC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAC30 size=12
    let mut pc: u32 = 0x830DAC30;
    'dispatch: loop {
        match pc {
            0x830DAC30 => {
    //   block [0x830DAC30..0x830DAC3C)
	// 830DAC30: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAC34: 386B7E54  addi r3, r11, 0x7e54
	ctx.r[3].s64 = ctx.r[11].s64 + 32340;
	// 830DAC38: 4BFFEA90  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAC40 size=172
    let mut pc: u32 = 0x830DAC40;
    'dispatch: loop {
        match pc {
            0x830DAC40 => {
    //   block [0x830DAC40..0x830DACEC)
	// 830DAC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAC48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DAC4C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAC50: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DAC54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DAC58: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DAC5C: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DAC60: 394A7E78  addi r10, r10, 0x7e78
	ctx.r[10].s64 = ctx.r[10].s64 + 32376;
	// 830DAC64: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DAC68: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DAC6C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DAC70: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DAC74: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DAC78: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DAC7C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DAC80: 39297B64  addi r9, r9, 0x7b64
	ctx.r[9].s64 = ctx.r[9].s64 + 31588;
	// 830DAC84: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DAC88: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DAC8C: 390865B8  addi r8, r8, 0x65b8
	ctx.r[8].s64 = ctx.r[8].s64 + 26040;
	// 830DAC90: 39676568  addi r11, r7, 0x6568
	ctx.r[11].s64 = ctx.r[7].s64 + 25960;
	// 830DAC94: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DAC98: 3946EED0  addi r10, r6, -0x1130
	ctx.r[10].s64 = ctx.r[6].s64 + -4400;
	// 830DAC9C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DACA0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DACA4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DACA8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DACAC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DACB0: 4BFFDBC9  bl 0x830d8878
	ctx.lr = 0x830DACB4;
	sub_830D8878(ctx, base);
	// 830DACB4: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DACB8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DACBC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DACC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DACC4: 388BC0D8  addi r4, r11, -0x3f28
	ctx.r[4].s64 = ctx.r[11].s64 + -16168;
	// 830DACC8: 4BFFE781  bl 0x830d9448
	ctx.lr = 0x830DACCC;
	sub_830D9448(ctx, base);
	// 830DACCC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DACD0: 41800008  blt 0x830dacd8
	if ctx.cr[0].lt {
	pc = 0x830DACD8; continue 'dispatch;
	}
	// 830DACD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DACD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DACE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DACE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DACF0 size=12
    let mut pc: u32 = 0x830DACF0;
    'dispatch: loop {
        match pc {
            0x830DACF0 => {
    //   block [0x830DACF0..0x830DACFC)
	// 830DACF0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DACF4: 386B7E78  addi r3, r11, 0x7e78
	ctx.r[3].s64 = ctx.r[11].s64 + 32376;
	// 830DACF8: 4BFFE9D0  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAD00 size=176
    let mut pc: u32 = 0x830DAD00;
    'dispatch: loop {
        match pc {
            0x830DAD00 => {
    //   block [0x830DAD00..0x830DADB0)
	// 830DAD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAD08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DAD0C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAD10: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DAD14: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DAD18: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DAD1C: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DAD20: 394A7E8C  addi r10, r10, 0x7e8c
	ctx.r[10].s64 = ctx.r[10].s64 + 32396;
	// 830DAD24: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DAD28: 39297BFC  addi r9, r9, 0x7bfc
	ctx.r[9].s64 = ctx.r[9].s64 + 31740;
	// 830DAD2C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DAD30: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DAD34: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DAD38: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DAD3C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DAD40: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DAD44: 4B6F4495  bl 0x827cf1d8
	ctx.lr = 0x830DAD48;
	sub_827CF1D8(ctx, base);
	// 830DAD48: 3D60830D  lis r11, -0x7cf3
	ctx.r[11].s64 = -2096300032;
	// 830DAD4C: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DAD50: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DAD54: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DAD58: 396B3130  addi r11, r11, 0x3130
	ctx.r[11].s64 = ctx.r[11].s64 + 12592;
	// 830DAD5C: 394A2E40  addi r10, r10, 0x2e40
	ctx.r[10].s64 = ctx.r[10].s64 + 11840;
	// 830DAD60: 3929EED8  addi r9, r9, -0x1128
	ctx.r[9].s64 = ctx.r[9].s64 + -4392;
	// 830DAD64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DAD68: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DAD6C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DAD70: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DAD74: 4BFFDC65  bl 0x830d89d8
	ctx.lr = 0x830DAD78;
	sub_830D89D8(ctx, base);
	// 830DAD78: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DAD7C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DAD80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DAD84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAD88: 388BC0DC  addi r4, r11, -0x3f24
	ctx.r[4].s64 = ctx.r[11].s64 + -16164;
	// 830DAD8C: 4BFFE6BD  bl 0x830d9448
	ctx.lr = 0x830DAD90;
	sub_830D9448(ctx, base);
	// 830DAD90: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAD94: 41800008  blt 0x830dad9c
	if ctx.cr[0].lt {
	pc = 0x830DAD9C; continue 'dispatch;
	}
	// 830DAD98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAD9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DADA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DADA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DADA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DADB0 size=12
    let mut pc: u32 = 0x830DADB0;
    'dispatch: loop {
        match pc {
            0x830DADB0 => {
    //   block [0x830DADB0..0x830DADBC)
	// 830DADB0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DADB4: 386B7E8C  addi r3, r11, 0x7e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 32396;
	// 830DADB8: 4BFFE910  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DADC0 size=172
    let mut pc: u32 = 0x830DADC0;
    'dispatch: loop {
        match pc {
            0x830DADC0 => {
    //   block [0x830DADC0..0x830DAE6C)
	// 830DADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DADC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DADCC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DADD0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DADD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DADD8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DADDC: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DADE0: 394A7EA4  addi r10, r10, 0x7ea4
	ctx.r[10].s64 = ctx.r[10].s64 + 32420;
	// 830DADE4: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DADE8: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DADEC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DADF0: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DADF4: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DADF8: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DADFC: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DAE00: 39297B64  addi r9, r9, 0x7b64
	ctx.r[9].s64 = ctx.r[9].s64 + 31588;
	// 830DAE04: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DAE08: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DAE0C: 39086668  addi r8, r8, 0x6668
	ctx.r[8].s64 = ctx.r[8].s64 + 26216;
	// 830DAE10: 396731B8  addi r11, r7, 0x31b8
	ctx.r[11].s64 = ctx.r[7].s64 + 12728;
	// 830DAE14: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DAE18: 3946EEE0  addi r10, r6, -0x1120
	ctx.r[10].s64 = ctx.r[6].s64 + -4384;
	// 830DAE1C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DAE20: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DAE24: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DAE28: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DAE2C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DAE30: 4BFFDE11  bl 0x830d8c40
	ctx.lr = 0x830DAE34;
	sub_830D8C40(ctx, base);
	// 830DAE34: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DAE38: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DAE3C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DAE40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAE44: 388BC0E0  addi r4, r11, -0x3f20
	ctx.r[4].s64 = ctx.r[11].s64 + -16160;
	// 830DAE48: 4BFFE601  bl 0x830d9448
	ctx.lr = 0x830DAE4C;
	sub_830D9448(ctx, base);
	// 830DAE4C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAE50: 41800008  blt 0x830dae58
	if ctx.cr[0].lt {
	pc = 0x830DAE58; continue 'dispatch;
	}
	// 830DAE54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAE58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DAE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DAE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DAE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DAE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAE70 size=12
    let mut pc: u32 = 0x830DAE70;
    'dispatch: loop {
        match pc {
            0x830DAE70 => {
    //   block [0x830DAE70..0x830DAE7C)
	// 830DAE70: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAE74: 386B7EA4  addi r3, r11, 0x7ea4
	ctx.r[3].s64 = ctx.r[11].s64 + 32420;
	// 830DAE78: 4BFFE850  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAE80 size=12
    let mut pc: u32 = 0x830DAE80;
    'dispatch: loop {
        match pc {
            0x830DAE80 => {
    //   block [0x830DAE80..0x830DAE8C)
	// 830DAE80: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAE84: 386B7EBC  addi r3, r11, 0x7ebc
	ctx.r[3].s64 = ctx.r[11].s64 + 32444;
	// 830DAE88: 4BFFE840  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAE90 size=172
    let mut pc: u32 = 0x830DAE90;
    'dispatch: loop {
        match pc {
            0x830DAE90 => {
    //   block [0x830DAE90..0x830DAF3C)
	// 830DAE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAE98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DAE9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAEA0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DAEA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DAEA8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DAEAC: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DAEB0: 394A7ED0  addi r10, r10, 0x7ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 32464;
	// 830DAEB4: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DAEB8: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DAEBC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DAEC0: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DAEC4: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DAEC8: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DAECC: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DAED0: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DAED4: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DAED8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DAEDC: 39083230  addi r8, r8, 0x3230
	ctx.r[8].s64 = ctx.r[8].s64 + 12848;
	// 830DAEE0: 396732A8  addi r11, r7, 0x32a8
	ctx.r[11].s64 = ctx.r[7].s64 + 12968;
	// 830DAEE4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DAEE8: 3946EEF0  addi r10, r6, -0x1110
	ctx.r[10].s64 = ctx.r[6].s64 + -4368;
	// 830DAEEC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DAEF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DAEF4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DAEF8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DAEFC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DAF00: 4BFFA391  bl 0x830d5290
	ctx.lr = 0x830DAF04;
	sub_830D5290(ctx, base);
	// 830DAF04: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DAF08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DAF0C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DAF10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAF14: 388BC0E8  addi r4, r11, -0x3f18
	ctx.r[4].s64 = ctx.r[11].s64 + -16152;
	// 830DAF18: 4BFFE531  bl 0x830d9448
	ctx.lr = 0x830DAF1C;
	sub_830D9448(ctx, base);
	// 830DAF1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAF20: 41800008  blt 0x830daf28
	if ctx.cr[0].lt {
	pc = 0x830DAF28; continue 'dispatch;
	}
	// 830DAF24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAF28: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DAF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DAF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DAF34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DAF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DAF40 size=12
    let mut pc: u32 = 0x830DAF40;
    'dispatch: loop {
        match pc {
            0x830DAF40 => {
    //   block [0x830DAF40..0x830DAF4C)
	// 830DAF40: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DAF44: 386B7ED0  addi r3, r11, 0x7ed0
	ctx.r[3].s64 = ctx.r[11].s64 + 32464;
	// 830DAF48: 4BFFE780  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DAF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DAF50 size=172
    let mut pc: u32 = 0x830DAF50;
    'dispatch: loop {
        match pc {
            0x830DAF50 => {
    //   block [0x830DAF50..0x830DAFFC)
	// 830DAF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DAF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DAF58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DAF5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DAF60: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DAF64: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DAF68: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DAF6C: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DAF70: 394A7EF4  addi r10, r10, 0x7ef4
	ctx.r[10].s64 = ctx.r[10].s64 + 32500;
	// 830DAF74: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DAF78: 3CC0827D  lis r6, -0x7d83
	ctx.r[6].s64 = -2105737216;
	// 830DAF7C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DAF80: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DAF84: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DAF88: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DAF8C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DAF90: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DAF94: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DAF98: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DAF9C: 39086700  addi r8, r8, 0x6700
	ctx.r[8].s64 = ctx.r[8].s64 + 26368;
	// 830DAFA0: 396768E0  addi r11, r7, 0x68e0
	ctx.r[11].s64 = ctx.r[7].s64 + 26848;
	// 830DAFA4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DAFA8: 3946F1D8  addi r10, r6, -0xe28
	ctx.r[10].s64 = ctx.r[6].s64 + -3624;
	// 830DAFAC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DAFB0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DAFB4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DAFB8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DAFBC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DAFC0: 4BFF6709  bl 0x830d16c8
	ctx.lr = 0x830DAFC4;
	sub_830D16C8(ctx, base);
	// 830DAFC4: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DAFC8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DAFCC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DAFD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DAFD4: 388BC0EC  addi r4, r11, -0x3f14
	ctx.r[4].s64 = ctx.r[11].s64 + -16148;
	// 830DAFD8: 4BFFE471  bl 0x830d9448
	ctx.lr = 0x830DAFDC;
	sub_830D9448(ctx, base);
	// 830DAFDC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DAFE0: 41800008  blt 0x830dafe8
	if ctx.cr[0].lt {
	pc = 0x830DAFE8; continue 'dispatch;
	}
	// 830DAFE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DAFE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DAFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DAFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DAFF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DAFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB000 size=12
    let mut pc: u32 = 0x830DB000;
    'dispatch: loop {
        match pc {
            0x830DB000 => {
    //   block [0x830DB000..0x830DB00C)
	// 830DB000: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB004: 386B7EF4  addi r3, r11, 0x7ef4
	ctx.r[3].s64 = ctx.r[11].s64 + 32500;
	// 830DB008: 4BFFE6C0  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB010 size=172
    let mut pc: u32 = 0x830DB010;
    'dispatch: loop {
        match pc {
            0x830DB010 => {
    //   block [0x830DB010..0x830DB0BC)
	// 830DB010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB01C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB020: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DB024: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DB028: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DB02C: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DB030: 394A7F1C  addi r10, r10, 0x7f1c
	ctx.r[10].s64 = ctx.r[10].s64 + 32540;
	// 830DB034: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DB038: 3CC0827D  lis r6, -0x7d83
	ctx.r[6].s64 = -2105737216;
	// 830DB03C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DB040: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DB044: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DB048: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DB04C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DB050: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DB054: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DB058: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DB05C: 39086778  addi r8, r8, 0x6778
	ctx.r[8].s64 = ctx.r[8].s64 + 26488;
	// 830DB060: 396768E0  addi r11, r7, 0x68e0
	ctx.r[11].s64 = ctx.r[7].s64 + 26848;
	// 830DB064: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DB068: 3946F1D8  addi r10, r6, -0xe28
	ctx.r[10].s64 = ctx.r[6].s64 + -3624;
	// 830DB06C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DB070: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DB074: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DB078: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DB07C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DB080: 4BFFA681  bl 0x830d5700
	ctx.lr = 0x830DB084;
	sub_830D5700(ctx, base);
	// 830DB084: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DB088: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DB08C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DB090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DB094: 388BC0F0  addi r4, r11, -0x3f10
	ctx.r[4].s64 = ctx.r[11].s64 + -16144;
	// 830DB098: 4BFFE3B1  bl 0x830d9448
	ctx.lr = 0x830DB09C;
	sub_830D9448(ctx, base);
	// 830DB09C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DB0A0: 41800008  blt 0x830db0a8
	if ctx.cr[0].lt {
	pc = 0x830DB0A8; continue 'dispatch;
	}
	// 830DB0A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB0A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DB0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB0B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB0C0 size=12
    let mut pc: u32 = 0x830DB0C0;
    'dispatch: loop {
        match pc {
            0x830DB0C0 => {
    //   block [0x830DB0C0..0x830DB0CC)
	// 830DB0C0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB0C4: 386B7F1C  addi r3, r11, 0x7f1c
	ctx.r[3].s64 = ctx.r[11].s64 + 32540;
	// 830DB0C8: 4BFFE600  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB0D0 size=172
    let mut pc: u32 = 0x830DB0D0;
    'dispatch: loop {
        match pc {
            0x830DB0D0 => {
    //   block [0x830DB0D0..0x830DB17C)
	// 830DB0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB0D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB0DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB0E0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DB0E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DB0E8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DB0EC: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DB0F0: 394A7F40  addi r10, r10, 0x7f40
	ctx.r[10].s64 = ctx.r[10].s64 + 32576;
	// 830DB0F4: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DB0F8: 3CC0827D  lis r6, -0x7d83
	ctx.r[6].s64 = -2105737216;
	// 830DB0FC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DB100: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DB104: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DB108: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DB10C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DB110: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DB114: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DB118: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DB11C: 390867F0  addi r8, r8, 0x67f0
	ctx.r[8].s64 = ctx.r[8].s64 + 26608;
	// 830DB120: 396768E0  addi r11, r7, 0x68e0
	ctx.r[11].s64 = ctx.r[7].s64 + 26848;
	// 830DB124: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DB128: 3946F1D8  addi r10, r6, -0xe28
	ctx.r[10].s64 = ctx.r[6].s64 + -3624;
	// 830DB12C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DB130: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DB134: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DB138: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DB13C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DB140: 4BFFA6E9  bl 0x830d5828
	ctx.lr = 0x830DB144;
	sub_830D5828(ctx, base);
	// 830DB144: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DB148: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DB14C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DB150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DB154: 388BC0F4  addi r4, r11, -0x3f0c
	ctx.r[4].s64 = ctx.r[11].s64 + -16140;
	// 830DB158: 4BFFE2F1  bl 0x830d9448
	ctx.lr = 0x830DB15C;
	sub_830D9448(ctx, base);
	// 830DB15C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DB160: 41800008  blt 0x830db168
	if ctx.cr[0].lt {
	pc = 0x830DB168; continue 'dispatch;
	}
	// 830DB164: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB168: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DB16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB180 size=12
    let mut pc: u32 = 0x830DB180;
    'dispatch: loop {
        match pc {
            0x830DB180 => {
    //   block [0x830DB180..0x830DB18C)
	// 830DB180: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB184: 386B7F40  addi r3, r11, 0x7f40
	ctx.r[3].s64 = ctx.r[11].s64 + 32576;
	// 830DB188: 4BFFE540  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB190 size=160
    let mut pc: u32 = 0x830DB190;
    'dispatch: loop {
        match pc {
            0x830DB190 => {
    //   block [0x830DB190..0x830DB230)
	// 830DB190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB19C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 830DB1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DB1A4: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DB1A8: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DB1AC: 39297F68  addi r9, r9, 0x7f68
	ctx.r[9].s64 = ctx.r[9].s64 + 32616;
	// 830DB1B0: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 830DB1B4: 3CA0827D  lis r5, -0x7d83
	ctx.r[5].s64 = -2105737216;
	// 830DB1B8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 830DB1BC: 3D008218  lis r8, -0x7de8
	ctx.r[8].s64 = -2112356352;
	// 830DB1C0: F96A0010  std r11, 0x10(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 830DB1C4: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DB1C8: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 830DB1CC: 39087EBC  addi r8, r8, 0x7ebc
	ctx.r[8].s64 = ctx.r[8].s64 + 32444;
	// 830DB1D0: F96A0020  std r11, 0x20(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 830DB1D4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830DB1D8: 38E76868  addi r7, r7, 0x6868
	ctx.r[7].s64 = ctx.r[7].s64 + 26728;
	// 830DB1DC: 394668E0  addi r10, r6, 0x68e0
	ctx.r[10].s64 = ctx.r[6].s64 + 26848;
	// 830DB1E0: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 830DB1E4: 3925F1D8  addi r9, r5, -0xe28
	ctx.r[9].s64 = ctx.r[5].s64 + -3624;
	// 830DB1E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830DB1EC: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 830DB1F0: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830DB1F4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DB1F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DB1FC: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DB200: 3888C0F8  addi r4, r8, -0x3f08
	ctx.r[4].s64 = ctx.r[8].s64 + -16136;
	// 830DB204: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830DB208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830DB20C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830DB210: 4BFFE239  bl 0x830d9448
	ctx.lr = 0x830DB214;
	sub_830D9448(ctx, base);
	// 830DB214: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DB218: 41800008  blt 0x830db220
	if ctx.cr[0].lt {
	pc = 0x830DB220; continue 'dispatch;
	}
	// 830DB21C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DB224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB230 size=12
    let mut pc: u32 = 0x830DB230;
    'dispatch: loop {
        match pc {
            0x830DB230 => {
    //   block [0x830DB230..0x830DB23C)
	// 830DB230: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB234: 386B7F68  addi r3, r11, 0x7f68
	ctx.r[3].s64 = ctx.r[11].s64 + 32616;
	// 830DB238: 4BFFE490  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB240 size=12
    let mut pc: u32 = 0x830DB240;
    'dispatch: loop {
        match pc {
            0x830DB240 => {
    //   block [0x830DB240..0x830DB24C)
	// 830DB240: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB244: 386B7F84  addi r3, r11, 0x7f84
	ctx.r[3].s64 = ctx.r[11].s64 + 32644;
	// 830DB248: 4BFFE480  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB250 size=172
    let mut pc: u32 = 0x830DB250;
    'dispatch: loop {
        match pc {
            0x830DB250 => {
    //   block [0x830DB250..0x830DB2FC)
	// 830DB250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB25C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB260: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DB264: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DB268: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DB26C: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DB270: 394A7FA4  addi r10, r10, 0x7fa4
	ctx.r[10].s64 = ctx.r[10].s64 + 32676;
	// 830DB274: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DB278: 3CC0827D  lis r6, -0x7d83
	ctx.r[6].s64 = -2105737216;
	// 830DB27C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DB280: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DB284: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DB288: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DB28C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DB290: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DB294: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DB298: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DB29C: 39086928  addi r8, r8, 0x6928
	ctx.r[8].s64 = ctx.r[8].s64 + 26920;
	// 830DB2A0: 396768E0  addi r11, r7, 0x68e0
	ctx.r[11].s64 = ctx.r[7].s64 + 26848;
	// 830DB2A4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DB2A8: 3946F1D8  addi r10, r6, -0xe28
	ctx.r[10].s64 = ctx.r[6].s64 + -3624;
	// 830DB2AC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DB2B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DB2B4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DB2B8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DB2BC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DB2C0: 4BFF67C1  bl 0x830d1a80
	ctx.lr = 0x830DB2C4;
	sub_830D1A80(ctx, base);
	// 830DB2C4: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DB2C8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DB2CC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DB2D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DB2D4: 388BC100  addi r4, r11, -0x3f00
	ctx.r[4].s64 = ctx.r[11].s64 + -16128;
	// 830DB2D8: 4BFFE171  bl 0x830d9448
	ctx.lr = 0x830DB2DC;
	sub_830D9448(ctx, base);
	// 830DB2DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DB2E0: 41800008  blt 0x830db2e8
	if ctx.cr[0].lt {
	pc = 0x830DB2E8; continue 'dispatch;
	}
	// 830DB2E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB2E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DB2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB2F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB300 size=12
    let mut pc: u32 = 0x830DB300;
    'dispatch: loop {
        match pc {
            0x830DB300 => {
    //   block [0x830DB300..0x830DB30C)
	// 830DB300: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB304: 386B7FA4  addi r3, r11, 0x7fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 32676;
	// 830DB308: 4BFFE3C0  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB310 size=172
    let mut pc: u32 = 0x830DB310;
    'dispatch: loop {
        match pc {
            0x830DB310 => {
    //   block [0x830DB310..0x830DB3BC)
	// 830DB310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB31C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB320: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DB324: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DB328: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DB32C: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DB330: 394A7FC4  addi r10, r10, 0x7fc4
	ctx.r[10].s64 = ctx.r[10].s64 + 32708;
	// 830DB334: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DB338: 3CC0827D  lis r6, -0x7d83
	ctx.r[6].s64 = -2105737216;
	// 830DB33C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DB340: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DB344: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DB348: 3D00830D  lis r8, -0x7cf3
	ctx.r[8].s64 = -2096300032;
	// 830DB34C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DB350: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DB354: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DB358: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DB35C: 390869A0  addi r8, r8, 0x69a0
	ctx.r[8].s64 = ctx.r[8].s64 + 27040;
	// 830DB360: 396768E0  addi r11, r7, 0x68e0
	ctx.r[11].s64 = ctx.r[7].s64 + 26848;
	// 830DB364: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DB368: 3946F1D8  addi r10, r6, -0xe28
	ctx.r[10].s64 = ctx.r[6].s64 + -3624;
	// 830DB36C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DB370: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DB374: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DB378: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DB37C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DB380: 4BFF68C9  bl 0x830d1c48
	ctx.lr = 0x830DB384;
	sub_830D1C48(ctx, base);
	// 830DB384: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DB388: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DB38C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DB390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DB394: 388BC104  addi r4, r11, -0x3efc
	ctx.r[4].s64 = ctx.r[11].s64 + -16124;
	// 830DB398: 4BFFE0B1  bl 0x830d9448
	ctx.lr = 0x830DB39C;
	sub_830D9448(ctx, base);
	// 830DB39C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DB3A0: 41800008  blt 0x830db3a8
	if ctx.cr[0].lt {
	pc = 0x830DB3A8; continue 'dispatch;
	}
	// 830DB3A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB3A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DB3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB3B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DB3C0 size=12
    let mut pc: u32 = 0x830DB3C0;
    'dispatch: loop {
        match pc {
            0x830DB3C0 => {
    //   block [0x830DB3C0..0x830DB3CC)
	// 830DB3C0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830DB3C4: 386B7FC4  addi r3, r11, 0x7fc4
	ctx.r[3].s64 = ctx.r[11].s64 + 32708;
	// 830DB3C8: 4BFFE300  b 0x830d96c8
	sub_830D96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB3D0 size=140
    let mut pc: u32 = 0x830DB3D0;
    'dispatch: loop {
        match pc {
            0x830DB3D0 => {
    //   block [0x830DB3D0..0x830DB45C)
	// 830DB3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB3D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DB3DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB3E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB3E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DB3E8: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 830DB3EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DB3F0: 480020C1  bl 0x830dd4b0
	ctx.lr = 0x830DB3F4;
	sub_830DD4B0(ctx, base);
	// 830DB3F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DB3F8: 41820028  beq 0x830db420
	if ctx.cr[0].eq {
	pc = 0x830DB420; continue 'dispatch;
	}
	// 830DB3FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DB400: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830DB404: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DB408: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DB40C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830DB410: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830DB414: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830DB418: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830DB41C: 48000008  b 0x830db424
	pc = 0x830DB424; continue 'dispatch;
	// 830DB420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DB424: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830DB428: 409A0010  bne cr6, 0x830db438
	if !ctx.cr[6].eq {
	pc = 0x830DB438; continue 'dispatch;
	}
	// 830DB42C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DB430: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DB434: 48000010  b 0x830db444
	pc = 0x830DB444; continue 'dispatch;
	// 830DB438: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DB43C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB440: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830DB444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DB448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DB454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB460 size=64
    let mut pc: u32 = 0x830DB460;
    'dispatch: loop {
        match pc {
            0x830DB460 => {
    //   block [0x830DB460..0x830DB4A0)
	// 830DB460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB46C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DB474: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DB478: 419A0010  beq cr6, 0x830db488
	if ctx.cr[6].eq {
	pc = 0x830DB488; continue 'dispatch;
	}
	// 830DB47C: 4BFF92F5  bl 0x830d4770
	ctx.lr = 0x830DB480;
	sub_830D4770(ctx, base);
	// 830DB480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DB484: 48002055  bl 0x830dd4d8
	ctx.lr = 0x830DB488;
	sub_830DD4D8(ctx, base);
	// 830DB488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB48C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DB490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB4A0 size=116
    let mut pc: u32 = 0x830DB4A0;
    'dispatch: loop {
        match pc {
            0x830DB4A0 => {
    //   block [0x830DB4A0..0x830DB514)
	// 830DB4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB4A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DB4AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB4B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DB4B8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830DB4BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DB4C0: 48001FF1  bl 0x830dd4b0
	ctx.lr = 0x830DB4C4;
	sub_830DD4B0(ctx, base);
	// 830DB4C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DB4C8: 41820010  beq 0x830db4d8
	if ctx.cr[0].eq {
	pc = 0x830DB4D8; continue 'dispatch;
	}
	// 830DB4CC: 4BFFC775  bl 0x830d7c40
	ctx.lr = 0x830DB4D0;
	sub_830D7C40(ctx, base);
	// 830DB4D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830DB4D4: 48000008  b 0x830db4dc
	pc = 0x830DB4DC; continue 'dispatch;
	// 830DB4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DB4DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DB4E0: 409A0010  bne cr6, 0x830db4f0
	if !ctx.cr[6].eq {
	pc = 0x830DB4F0; continue 'dispatch;
	}
	// 830DB4E4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DB4E8: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DB4EC: 48000010  b 0x830db4fc
	pc = 0x830DB4FC; continue 'dispatch;
	// 830DB4F0: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DB4F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB4F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DB4FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DB500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB508: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DB50C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB518 size=100
    let mut pc: u32 = 0x830DB518;
    'dispatch: loop {
        match pc {
            0x830DB518 => {
    //   block [0x830DB518..0x830DB57C)
	// 830DB518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB51C: 480CCC51  bl 0x831a816c
	ctx.lr = 0x830DB520;
	sub_831A8130(ctx, base);
	// 830DB520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB524: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DB528: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830DB52C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830DB530: 48001F81  bl 0x830dd4b0
	ctx.lr = 0x830DB534;
	sub_830DD4B0(ctx, base);
	// 830DB534: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DB538: 41820018  beq 0x830db550
	if ctx.cr[0].eq {
	pc = 0x830DB550; continue 'dispatch;
	}
	// 830DB53C: 4BFFA8BD  bl 0x830d5df8
	ctx.lr = 0x830DB540;
	sub_830D5DF8(ctx, base);
	// 830DB540: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DB544: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830DB548: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830DB54C: 48000008  b 0x830db554
	pc = 0x830DB554; continue 'dispatch;
	// 830DB550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DB554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DB558: 409A0010  bne cr6, 0x830db568
	if !ctx.cr[6].eq {
	pc = 0x830DB568; continue 'dispatch;
	}
	// 830DB55C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DB560: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DB564: 48000010  b 0x830db574
	pc = 0x830DB574; continue 'dispatch;
	// 830DB568: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830DB56C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB570: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DB574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DB578: 480CCC44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB580 size=116
    let mut pc: u32 = 0x830DB580;
    'dispatch: loop {
        match pc {
            0x830DB580 => {
    //   block [0x830DB580..0x830DB5F4)
	// 830DB580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DB588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DB58C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DB590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DB598: 386000B8  li r3, 0xb8
	ctx.r[3].s64 = 184;
	// 830DB59C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DB5A0: 48001F11  bl 0x830dd4b0
	ctx.lr = 0x830DB5A4;
	sub_830DD4B0(ctx, base);
	// 830DB5A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DB5A8: 41820010  beq 0x830db5b8
	if ctx.cr[0].eq {
	pc = 0x830DB5B8; continue 'dispatch;
	}
	// 830DB5AC: 4BFFA335  bl 0x830d58e0
	ctx.lr = 0x830DB5B0;
	sub_830D58E0(ctx, base);
	// 830DB5B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830DB5B4: 48000008  b 0x830db5bc
	pc = 0x830DB5BC; continue 'dispatch;
	// 830DB5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DB5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DB5C0: 409A0010  bne cr6, 0x830db5d0
	if !ctx.cr[6].eq {
	pc = 0x830DB5D0; continue 'dispatch;
	}
	// 830DB5C4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DB5C8: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DB5CC: 48000010  b 0x830db5dc
	pc = 0x830DB5DC; continue 'dispatch;
	// 830DB5D0: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DB5D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DB5D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DB5DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DB5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DB5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DB5E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DB5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DB5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB5F8 size=116
    let mut pc: u32 = 0x830DB5F8;
    'dispatch: loop {
        match pc {
            0x830DB5F8 => {
    //   block [0x830DB5F8..0x830DB66C)
	// 830DB5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB5FC: 480CCB6D  bl 0x831a8168
	ctx.lr = 0x830DB600;
	sub_831A8130(ctx, base);
	// 830DB600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DB608: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830DB60C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DB610: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DB614: 419A0048  beq cr6, 0x830db65c
	if ctx.cr[6].eq {
	pc = 0x830DB65C; continue 'dispatch;
	}
	// 830DB618: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DB61C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830DB620: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DB624: 4099002C  ble cr6, 0x830db650
	if !ctx.cr[6].gt {
	pc = 0x830DB650; continue 'dispatch;
	}
	// 830DB628: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830DB62C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DB630: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DB634: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 830DB638: 4BFF19B9  bl 0x830ccff0
	ctx.lr = 0x830DB63C;
	sub_830CCFF0(ctx, base);
	// 830DB63C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DB640: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830DB644: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 830DB648: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DB64C: 4198FFE0  blt cr6, 0x830db62c
	if ctx.cr[6].lt {
	pc = 0x830DB62C; continue 'dispatch;
	}
	// 830DB650: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DB654: 48001E85  bl 0x830dd4d8
	ctx.lr = 0x830DB658;
	sub_830DD4D8(ctx, base);
	// 830DB658: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830DB65C: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830DB660: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830DB664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DB668: 480CCB50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DB670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DB670 size=1620
    let mut pc: u32 = 0x830DB670;
    'dispatch: loop {
        match pc {
            0x830DB670 => {
    //   block [0x830DB670..0x830DBCC4)
	// 830DB670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DB674: 480CCAF9  bl 0x831a816c
	ctx.lr = 0x830DB678;
	sub_831A8130(ctx, base);
	// 830DB678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DB67C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830DB680: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DB684: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DB688: 3BEBD448  addi r31, r11, -0x2bb8
	ctx.r[31].s64 = ctx.r[11].s64 + -11192;
	// 830DB68C: 8168D778  lwz r11, -0x2888(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10376 as u32) ) } as u64;
	// 830DB690: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DB694: 4082061C  bne 0x830dbcb0
	if !ctx.cr[0].eq {
	pc = 0x830DBCB0; continue 'dispatch;
	}
	// 830DB698: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 830DB69C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830DB6A0: 392A8910  addi r9, r10, -0x76f0
	ctx.r[9].s64 = ctx.r[10].s64 + -30448;
	// 830DB6A4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830DB6A8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB6AC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830DB6B0: 9168D778  stw r11, -0x2888(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-10376 as u32), ctx.r[11].u32 ) };
	// 830DB6B4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DB6B8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830DB6BC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830DB6C0: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DB6C4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830DB6C8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DB6CC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830DB6D0: 4BFFA729  bl 0x830d5df8
	ctx.lr = 0x830DB6D4;
	sub_830D5DF8(ctx, base);
	// 830DB6D4: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830DB6D8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830DB6DC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DB6E0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830DB6E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DB6E8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830DB6EC: 392B8A58  addi r9, r11, -0x75a8
	ctx.r[9].s64 = ctx.r[11].s64 + -30120;
	// 830DB6F0: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830DB6F4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB6F8: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830DB6FC: 913F0040  stw r9, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 830DB700: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 830DB704: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830DB708: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830DB70C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830DB710: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830DB714: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 830DB718: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 830DB71C: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 830DB720: 480151A1  bl 0x830f08c0
	ctx.lr = 0x830DB724;
	sub_830F08C0(ctx, base);
	// 830DB724: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB728: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB72C: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 830DB730: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 830DB734: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB738: 39296D08  addi r9, r9, 0x6d08
	ctx.r[9].s64 = ctx.r[9].s64 + 27912;
	// 830DB73C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DB740: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 830DB744: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830DB748: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830DB74C: 392B8A48  addi r9, r11, -0x75b8
	ctx.r[9].s64 = ctx.r[11].s64 + -30136;
	// 830DB750: FBDF0058  std r30, 0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 830DB754: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 830DB758: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 830DB75C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB760: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 830DB764: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830DB768: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830DB76C: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830DB770: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 830DB774: 913F0068  stw r9, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 830DB778: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 830DB77C: 480151AD  bl 0x830f0928
	ctx.lr = 0x830DB780;
	sub_830F0928(ctx, base);
	// 830DB780: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB784: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 830DB788: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 830DB78C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB790: 39296DB0  addi r9, r9, 0x6db0
	ctx.r[9].s64 = ctx.r[9].s64 + 28080;
	// 830DB794: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB798: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830DB79C: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 830DB7A0: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 830DB7A4: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 830DB7A8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830DB7AC: FBDF0088  std r30, 0x88(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u64 ) };
	// 830DB7B0: 39288A34  addi r9, r8, -0x75cc
	ctx.r[9].s64 = ctx.r[8].s64 + -30156;
	// 830DB7B4: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 830DB7B8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 830DB7BC: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 830DB7C0: 913F00A0  stw r9, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 830DB7C4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DB7C8: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 830DB7CC: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 830DB7D0: 913F009C  stw r9, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[9].u32 ) };
	// 830DB7D4: 915F00A4  stw r10, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 830DB7D8: 48007591  bl 0x830e2d68
	ctx.lr = 0x830DB7DC;
	sub_830E2D68(ctx, base);
	// 830DB7DC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB7E0: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB7E4: 907F00A8  stw r3, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[3].u32 ) };
	// 830DB7E8: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 830DB7EC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB7F0: 3929D538  addi r9, r9, -0x2ac8
	ctx.r[9].s64 = ctx.r[9].s64 + -10952;
	// 830DB7F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DB7F8: 915F00B0  stw r10, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 830DB7FC: 913F00B4  stw r9, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[9].u32 ) };
	// 830DB800: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830DB804: 392B824C  addi r9, r11, -0x7db4
	ctx.r[9].s64 = ctx.r[11].s64 + -32180;
	// 830DB808: FBDF00B8  std r30, 0xb8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u64 ) };
	// 830DB80C: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 830DB810: 913F00D0  stw r9, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[9].u32 ) };
	// 830DB814: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB818: 3920003C  li r9, 0x3c
	ctx.r[9].s64 = 60;
	// 830DB81C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 830DB820: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 830DB824: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 830DB828: 397F00B8  addi r11, r31, 0xb8
	ctx.r[11].s64 = ctx.r[31].s64 + 184;
	// 830DB82C: 913F00C8  stw r9, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 830DB830: 915F00D4  stw r10, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 830DB834: 480075AD  bl 0x830e2de0
	ctx.lr = 0x830DB838;
	sub_830E2DE0(ctx, base);
	// 830DB838: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB83C: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB840: 907F00D8  stw r3, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[3].u32 ) };
	// 830DB844: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 830DB848: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB84C: 3929D548  addi r9, r9, -0x2ab8
	ctx.r[9].s64 = ctx.r[9].s64 + -10936;
	// 830DB850: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DB854: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 830DB858: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 830DB85C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DB860: 392B8238  addi r9, r11, -0x7dc8
	ctx.r[9].s64 = ctx.r[11].s64 + -32200;
	// 830DB864: FBDF00E8  std r30, 0xe8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u64 ) };
	// 830DB868: 915F00F4  stw r10, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 830DB86C: 913F0100  stw r9, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[9].u32 ) };
	// 830DB870: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB874: 39200048  li r9, 0x48
	ctx.r[9].s64 = 72;
	// 830DB878: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 830DB87C: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830DB880: 917F00FC  stw r11, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 830DB884: 397F00E8  addi r11, r31, 0xe8
	ctx.r[11].s64 = ctx.r[31].s64 + 232;
	// 830DB888: 913F00F8  stw r9, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[9].u32 ) };
	// 830DB88C: 915F0104  stw r10, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 830DB890: 480075C9  bl 0x830e2e58
	ctx.lr = 0x830DB894;
	sub_830E2E58(ctx, base);
	// 830DB894: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB898: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB89C: 907F0108  stw r3, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[3].u32 ) };
	// 830DB8A0: 3929D5A8  addi r9, r9, -0x2a58
	ctx.r[9].s64 = ctx.r[9].s64 + -10840;
	// 830DB8A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB8A8: 915F0110  stw r10, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 830DB8AC: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 830DB8B0: 913F0114  stw r9, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[9].u32 ) };
	// 830DB8B4: 917F010C  stw r11, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[11].u32 ) };
	// 830DB8B8: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 830DB8BC: 39288A24  addi r9, r8, -0x75dc
	ctx.r[9].s64 = ctx.r[8].s64 + -30172;
	// 830DB8C0: FBDF0118  std r30, 0x118(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u64 ) };
	// 830DB8C4: 917F0120  stw r11, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 830DB8C8: 913F0130  stw r9, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[9].u32 ) };
	// 830DB8CC: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 830DB8D0: 915F0124  stw r10, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[10].u32 ) };
	// 830DB8D4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DB8D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830DB8DC: 917F0128  stw r11, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 830DB8E0: 913F012C  stw r9, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[9].u32 ) };
	// 830DB8E4: 397F0118  addi r11, r31, 0x118
	ctx.r[11].s64 = ctx.r[31].s64 + 280;
	// 830DB8E8: 915F0134  stw r10, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[10].u32 ) };
	// 830DB8EC: 48007415  bl 0x830e2d00
	ctx.lr = 0x830DB8F0;
	sub_830E2D00(ctx, base);
	// 830DB8F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DB8F4: 907F0138  stw r3, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[3].u32 ) };
	// 830DB8F8: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 830DB8FC: 93DF013C  stw r30, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[30].u32 ) };
	// 830DB900: 392B8A14  addi r9, r11, -0x75ec
	ctx.r[9].s64 = ctx.r[11].s64 + -30188;
	// 830DB904: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 830DB908: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 830DB90C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB910: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 830DB914: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 830DB918: 913F0160  stw r9, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[9].u32 ) };
	// 830DB91C: 392000E8  li r9, 0xe8
	ctx.r[9].s64 = 232;
	// 830DB920: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 830DB924: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830DB928: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 830DB92C: 917F015C  stw r11, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[11].u32 ) };
	// 830DB930: 397F013C  addi r11, r31, 0x13c
	ctx.r[11].s64 = ctx.r[31].s64 + 316;
	// 830DB934: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 830DB938: 915F0164  stw r10, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 830DB93C: 4800759D  bl 0x830e2ed8
	ctx.lr = 0x830DB940;
	sub_830E2ED8(ctx, base);
	// 830DB940: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB944: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB948: 907F0168  stw r3, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[3].u32 ) };
	// 830DB94C: 3929D648  addi r9, r9, -0x29b8
	ctx.r[9].s64 = ctx.r[9].s64 + -10680;
	// 830DB950: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB954: 915F0170  stw r10, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 830DB958: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 830DB95C: 913F0174  stw r9, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[9].u32 ) };
	// 830DB960: 917F016C  stw r11, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 830DB964: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 830DB968: 39288A08  addi r9, r8, -0x75f8
	ctx.r[9].s64 = ctx.r[8].s64 + -30200;
	// 830DB96C: FBDF0178  std r30, 0x178(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[30].u64 ) };
	// 830DB970: 917F0180  stw r11, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 830DB974: 913F0190  stw r9, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[9].u32 ) };
	// 830DB978: 39600058  li r11, 0x58
	ctx.r[11].s64 = 88;
	// 830DB97C: 915F0184  stw r10, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[10].u32 ) };
	// 830DB980: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DB984: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 830DB988: 917F0188  stw r11, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 830DB98C: 913F018C  stw r9, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[9].u32 ) };
	// 830DB990: 397F0178  addi r11, r31, 0x178
	ctx.r[11].s64 = ctx.r[31].s64 + 376;
	// 830DB994: 915F0194  stw r10, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[10].u32 ) };
	// 830DB998: 480073D1  bl 0x830e2d68
	ctx.lr = 0x830DB99C;
	sub_830E2D68(ctx, base);
	// 830DB99C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB9A0: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DB9A4: 907F0198  stw r3, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[3].u32 ) };
	// 830DB9A8: 917F019C  stw r11, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[11].u32 ) };
	// 830DB9AC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DB9B0: 3929D5B8  addi r9, r9, -0x2a48
	ctx.r[9].s64 = ctx.r[9].s64 + -10824;
	// 830DB9B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DB9B8: 915F01A0  stw r10, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 830DB9BC: 913F01A4  stw r9, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[9].u32 ) };
	// 830DB9C0: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 830DB9C4: 392B89FC  addi r9, r11, -0x7604
	ctx.r[9].s64 = ctx.r[11].s64 + -30212;
	// 830DB9C8: FBDF01A8  std r30, 0x1a8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[30].u64 ) };
	// 830DB9CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DB9D0: 913F01C0  stw r9, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[9].u32 ) };
	// 830DB9D4: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 830DB9D8: 917F01B0  stw r11, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 830DB9DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830DB9E0: 915F01B4  stw r10, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[10].u32 ) };
	// 830DB9E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DB9E8: 917F01BC  stw r11, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[11].u32 ) };
	// 830DB9EC: 397F01A8  addi r11, r31, 0x1a8
	ctx.r[11].s64 = ctx.r[31].s64 + 424;
	// 830DB9F0: 913F01B8  stw r9, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[9].u32 ) };
	// 830DB9F4: 915F01C4  stw r10, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[10].u32 ) };
	// 830DB9F8: 4BFFA4E1  bl 0x830d5ed8
	ctx.lr = 0x830DB9FC;
	sub_830D5ED8(ctx, base);
	// 830DB9FC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBA00: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DBA04: 907F01C8  stw r3, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[3].u32 ) };
	// 830DBA08: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 830DBA0C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBA10: 39296F50  addi r9, r9, 0x6f50
	ctx.r[9].s64 = ctx.r[9].s64 + 28496;
	// 830DBA14: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBA18: 915F01D0  stw r10, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 830DBA1C: 913F01D4  stw r9, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[9].u32 ) };
	// 830DBA20: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 830DBA24: 392B89E8  addi r9, r11, -0x7618
	ctx.r[9].s64 = ctx.r[11].s64 + -30232;
	// 830DBA28: FBDF01D8  std r30, 0x1d8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[30].u64 ) };
	// 830DBA2C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBA30: 913F01F0  stw r9, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[9].u32 ) };
	// 830DBA34: 392000EC  li r9, 0xec
	ctx.r[9].s64 = 236;
	// 830DBA38: 917F01E0  stw r11, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[11].u32 ) };
	// 830DBA3C: 915F01E4  stw r10, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[10].u32 ) };
	// 830DBA40: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830DBA44: 917F01EC  stw r11, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[11].u32 ) };
	// 830DBA48: 397F01D8  addi r11, r31, 0x1d8
	ctx.r[11].s64 = ctx.r[31].s64 + 472;
	// 830DBA4C: 913F01E8  stw r9, 0x1e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), ctx.r[9].u32 ) };
	// 830DBA50: 915F01F4  stw r10, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[10].u32 ) };
	// 830DBA54: 48007485  bl 0x830e2ed8
	ctx.lr = 0x830DBA58;
	sub_830E2ED8(ctx, base);
	// 830DBA58: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBA5C: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DBA60: 907F01F8  stw r3, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[3].u32 ) };
	// 830DBA64: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 830DBA68: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBA6C: 3929F410  addi r9, r9, -0xbf0
	ctx.r[9].s64 = ctx.r[9].s64 + -3056;
	// 830DBA70: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBA74: 915F0200  stw r10, 0x200(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 830DBA78: 913F0204  stw r9, 0x204(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(516 as u32), ctx.r[9].u32 ) };
	// 830DBA7C: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 830DBA80: 392B89B4  addi r9, r11, -0x764c
	ctx.r[9].s64 = ctx.r[11].s64 + -30284;
	// 830DBA84: FBDF0208  std r30, 0x208(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[30].u64 ) };
	// 830DBA88: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBA8C: 913F0220  stw r9, 0x220(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(544 as u32), ctx.r[9].u32 ) };
	// 830DBA90: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 830DBA94: 917F0210  stw r11, 0x210(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(528 as u32), ctx.r[11].u32 ) };
	// 830DBA98: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 830DBA9C: 915F0214  stw r10, 0x214(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(532 as u32), ctx.r[10].u32 ) };
	// 830DBAA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBAA4: 917F021C  stw r11, 0x21c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(540 as u32), ctx.r[11].u32 ) };
	// 830DBAA8: 397F0208  addi r11, r31, 0x208
	ctx.r[11].s64 = ctx.r[31].s64 + 520;
	// 830DBAAC: 913F0218  stw r9, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[9].u32 ) };
	// 830DBAB0: 915F0224  stw r10, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[10].u32 ) };
	// 830DBAB4: 4BFFA3B5  bl 0x830d5e68
	ctx.lr = 0x830DBAB8;
	sub_830D5E68(ctx, base);
	// 830DBAB8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBABC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DBAC0: 907F0228  stw r3, 0x228(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[3].u32 ) };
	// 830DBAC4: 917F022C  stw r11, 0x22c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), ctx.r[11].u32 ) };
	// 830DBAC8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBACC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBAD0: 913F0234  stw r9, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[9].u32 ) };
	// 830DBAD4: 915F0230  stw r10, 0x230(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 830DBAD8: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 830DBADC: 392B899C  addi r9, r11, -0x7664
	ctx.r[9].s64 = ctx.r[11].s64 + -30308;
	// 830DBAE0: FBDF0238  std r30, 0x238(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(568 as u32), ctx.r[30].u64 ) };
	// 830DBAE4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830DBAE8: 913F0250  stw r9, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[9].u32 ) };
	// 830DBAEC: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 830DBAF0: 917F0240  stw r11, 0x240(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(576 as u32), ctx.r[11].u32 ) };
	// 830DBAF4: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 830DBAF8: 915F0244  stw r10, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[10].u32 ) };
	// 830DBAFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBB00: 917F024C  stw r11, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[11].u32 ) };
	// 830DBB04: 397F0238  addi r11, r31, 0x238
	ctx.r[11].s64 = ctx.r[31].s64 + 568;
	// 830DBB08: 913F0248  stw r9, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[9].u32 ) };
	// 830DBB0C: 915F0254  stw r10, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[10].u32 ) };
	// 830DBB10: 4BFFA359  bl 0x830d5e68
	ctx.lr = 0x830DBB14;
	sub_830D5E68(ctx, base);
	// 830DBB14: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBB18: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DBB1C: 907F0258  stw r3, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[3].u32 ) };
	// 830DBB20: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 830DBB24: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBB28: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBB2C: 913F0264  stw r9, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[9].u32 ) };
	// 830DBB30: 915F0260  stw r10, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[10].u32 ) };
	// 830DBB34: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 830DBB38: 392B897C  addi r9, r11, -0x7684
	ctx.r[9].s64 = ctx.r[11].s64 + -30340;
	// 830DBB3C: FBDF0268  std r30, 0x268(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u64 ) };
	// 830DBB40: 915F0274  stw r10, 0x274(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), ctx.r[10].u32 ) };
	// 830DBB44: 913F0280  stw r9, 0x280(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), ctx.r[9].u32 ) };
	// 830DBB48: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBB4C: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 830DBB50: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830DBB54: 917F0270  stw r11, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 830DBB58: 917F027C  stw r11, 0x27c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(636 as u32), ctx.r[11].u32 ) };
	// 830DBB5C: 397F0268  addi r11, r31, 0x268
	ctx.r[11].s64 = ctx.r[31].s64 + 616;
	// 830DBB60: 913F0278  stw r9, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[9].u32 ) };
	// 830DBB64: 915F0284  stw r10, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 830DBB68: 48007371  bl 0x830e2ed8
	ctx.lr = 0x830DBB6C;
	sub_830E2ED8(ctx, base);
	// 830DBB6C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBB70: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DBB74: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DBB78: 917F028C  stw r11, 0x28c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), ctx.r[11].u32 ) };
	// 830DBB7C: 3929F458  addi r9, r9, -0xba8
	ctx.r[9].s64 = ctx.r[9].s64 + -2984;
	// 830DBB80: 907F0288  stw r3, 0x288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), ctx.r[3].u32 ) };
	// 830DBB84: 394AF4A0  addi r10, r10, -0xb60
	ctx.r[10].s64 = ctx.r[10].s64 + -2912;
	// 830DBB88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBB8C: 913F0294  stw r9, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[9].u32 ) };
	// 830DBB90: 915F0290  stw r10, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[10].u32 ) };
	// 830DBB94: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 830DBB98: 392B8960  addi r9, r11, -0x76a0
	ctx.r[9].s64 = ctx.r[11].s64 + -30368;
	// 830DBB9C: FBDF0298  std r30, 0x298(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[30].u64 ) };
	// 830DBBA0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBBA4: 913F02B0  stw r9, 0x2b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[9].u32 ) };
	// 830DBBA8: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 830DBBAC: 917F02A0  stw r11, 0x2a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(672 as u32), ctx.r[11].u32 ) };
	// 830DBBB0: 39601000  li r11, 0x1000
	ctx.r[11].s64 = 4096;
	// 830DBBB4: 915F02A4  stw r10, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[10].u32 ) };
	// 830DBBB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBBBC: 917F02AC  stw r11, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[11].u32 ) };
	// 830DBBC0: 397F0298  addi r11, r31, 0x298
	ctx.r[11].s64 = ctx.r[31].s64 + 664;
	// 830DBBC4: 913F02A8  stw r9, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[9].u32 ) };
	// 830DBBC8: 915F02B4  stw r10, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 830DBBCC: 4BFFA29D  bl 0x830d5e68
	ctx.lr = 0x830DBBD0;
	sub_830D5E68(ctx, base);
	// 830DBBD0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DBBD4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBBD8: 907F02B8  stw r3, 0x2b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(696 as u32), ctx.r[3].u32 ) };
	// 830DBBDC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBBE0: 913F02C4  stw r9, 0x2c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(708 as u32), ctx.r[9].u32 ) };
	// 830DBBE4: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 830DBBE8: 915F02C0  stw r10, 0x2c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(704 as u32), ctx.r[10].u32 ) };
	// 830DBBEC: 917F02BC  stw r11, 0x2bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(700 as u32), ctx.r[11].u32 ) };
	// 830DBBF0: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 830DBBF4: 39288944  addi r9, r8, -0x76bc
	ctx.r[9].s64 = ctx.r[8].s64 + -30396;
	// 830DBBF8: FBDF02C8  std r30, 0x2c8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(712 as u32), ctx.r[30].u64 ) };
	// 830DBBFC: 917F02D0  stw r11, 0x2d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(720 as u32), ctx.r[11].u32 ) };
	// 830DBC00: 913F02E0  stw r9, 0x2e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), ctx.r[9].u32 ) };
	// 830DBC04: 396000F0  li r11, 0xf0
	ctx.r[11].s64 = 240;
	// 830DBC08: 915F02D4  stw r10, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[10].u32 ) };
	// 830DBC0C: 39204000  li r9, 0x4000
	ctx.r[9].s64 = 16384;
	// 830DBC10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBC14: 917F02D8  stw r11, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[11].u32 ) };
	// 830DBC18: 913F02DC  stw r9, 0x2dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(732 as u32), ctx.r[9].u32 ) };
	// 830DBC1C: 397F02C8  addi r11, r31, 0x2c8
	ctx.r[11].s64 = ctx.r[31].s64 + 712;
	// 830DBC20: 915F02E4  stw r10, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[10].u32 ) };
	// 830DBC24: 4BFFA245  bl 0x830d5e68
	ctx.lr = 0x830DBC28;
	sub_830D5E68(ctx, base);
	// 830DBC28: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBC2C: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DBC30: 907F02E8  stw r3, 0x2e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(744 as u32), ctx.r[3].u32 ) };
	// 830DBC34: 917F02EC  stw r11, 0x2ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(748 as u32), ctx.r[11].u32 ) };
	// 830DBC38: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBC3C: 3929D5C8  addi r9, r9, -0x2a38
	ctx.r[9].s64 = ctx.r[9].s64 + -10808;
	// 830DBC40: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBC44: 915F02F0  stw r10, 0x2f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[10].u32 ) };
	// 830DBC48: 913F02F4  stw r9, 0x2f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(756 as u32), ctx.r[9].u32 ) };
	// 830DBC4C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 830DBC50: 392B8918  addi r9, r11, -0x76e8
	ctx.r[9].s64 = ctx.r[11].s64 + -30440;
	// 830DBC54: FBDF02F8  std r30, 0x2f8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[30].u64 ) };
	// 830DBC58: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBC5C: 913F0310  stw r9, 0x310(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(784 as u32), ctx.r[9].u32 ) };
	// 830DBC60: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 830DBC64: 917F0300  stw r11, 0x300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[11].u32 ) };
	// 830DBC68: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 830DBC6C: 915F0304  stw r10, 0x304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(772 as u32), ctx.r[10].u32 ) };
	// 830DBC70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBC74: 917F030C  stw r11, 0x30c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(780 as u32), ctx.r[11].u32 ) };
	// 830DBC78: 397F02F8  addi r11, r31, 0x2f8
	ctx.r[11].s64 = ctx.r[31].s64 + 760;
	// 830DBC7C: 913F0308  stw r9, 0x308(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[9].u32 ) };
	// 830DBC80: 915F0314  stw r10, 0x314(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(788 as u32), ctx.r[10].u32 ) };
	// 830DBC84: 4BFFA1E5  bl 0x830d5e68
	ctx.lr = 0x830DBC88;
	sub_830D5E68(ctx, base);
	// 830DBC88: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DBC8C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBC90: 907F0318  stw r3, 0x318(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(792 as u32), ctx.r[3].u32 ) };
	// 830DBC94: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBC98: 39296E58  addi r9, r9, 0x6e58
	ctx.r[9].s64 = ctx.r[9].s64 + 28248;
	// 830DBC9C: 917F031C  stw r11, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[11].u32 ) };
	// 830DBCA0: 915F0320  stw r10, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[10].u32 ) };
	// 830DBCA4: 397F0328  addi r11, r31, 0x328
	ctx.r[11].s64 = ctx.r[31].s64 + 808;
	// 830DBCA8: 913F0324  stw r9, 0x324(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(804 as u32), ctx.r[9].u32 ) };
	// 830DBCAC: FBDF0328  std r30, 0x328(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[30].u64 ) };
	// 830DBCB0: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 830DBCB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DBCB8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DBCBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DBCC0: 480CC4FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DBCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DBCC8 size=1168
    let mut pc: u32 = 0x830DBCC8;
    'dispatch: loop {
        match pc {
            0x830DBCC8 => {
    //   block [0x830DBCC8..0x830DC158)
	// 830DBCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DBCCC: 480CC4A1  bl 0x831a816c
	ctx.lr = 0x830DBCD0;
	sub_831A8130(ctx, base);
	// 830DBCD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DBCD4: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830DBCD8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830DBCDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DBCE0: 3BEB5480  addi r31, r11, 0x5480
	ctx.r[31].s64 = ctx.r[11].s64 + 21632;
	// 830DBCE4: 8168D77C  lwz r11, -0x2884(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10372 as u32) ) } as u64;
	// 830DBCE8: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DBCEC: 40820458  bne 0x830dc144
	if !ctx.cr[0].eq {
	pc = 0x830DC144; continue 'dispatch;
	}
	// 830DBCF0: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 830DBCF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830DBCF8: 392A8B20  addi r9, r10, -0x74e0
	ctx.r[9].s64 = ctx.r[10].s64 + -29920;
	// 830DBCFC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830DBD00: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBD04: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830DBD08: 9168D77C  stw r11, -0x2884(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-10372 as u32), ctx.r[11].u32 ) };
	// 830DBD0C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DBD10: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830DBD14: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830DBD18: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBD1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830DBD20: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DBD24: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830DBD28: 4BFFA0D1  bl 0x830d5df8
	ctx.lr = 0x830DBD2C;
	sub_830D5DF8(ctx, base);
	// 830DBD2C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830DBD30: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830DBD34: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBD38: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830DBD3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBD40: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830DBD44: 392B8B10  addi r9, r11, -0x74f0
	ctx.r[9].s64 = ctx.r[11].s64 + -29936;
	// 830DBD48: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830DBD4C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBD50: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830DBD54: 913F0040  stw r9, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 830DBD58: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 830DBD5C: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830DBD60: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBD64: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830DBD68: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830DBD6C: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 830DBD70: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 830DBD74: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 830DBD78: 4BFFA081  bl 0x830d5df8
	ctx.lr = 0x830DBD7C;
	sub_830D5DF8(ctx, base);
	// 830DBD7C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBD80: 3D20830F  lis r9, -0x7cf1
	ctx.r[9].s64 = -2096168960;
	// 830DBD84: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 830DBD88: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 830DBD8C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBD90: 3929A828  addi r9, r9, -0x57d8
	ctx.r[9].s64 = ctx.r[9].s64 + -22488;
	// 830DBD94: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBD98: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 830DBD9C: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830DBDA0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830DBDA4: 392B8B00  addi r9, r11, -0x7500
	ctx.r[9].s64 = ctx.r[11].s64 + -29952;
	// 830DBDA8: FBDF0058  std r30, 0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 830DBDAC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBDB0: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 830DBDB4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 830DBDB8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830DBDBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830DBDC0: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 830DBDC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBDC8: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830DBDCC: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 830DBDD0: 913F0068  stw r9, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 830DBDD4: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 830DBDD8: 4BFFA101  bl 0x830d5ed8
	ctx.lr = 0x830DBDDC;
	sub_830D5ED8(ctx, base);
	// 830DBDDC: 3D20830F  lis r9, -0x7cf1
	ctx.r[9].s64 = -2096168960;
	// 830DBDE0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBDE4: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 830DBDE8: 3929A918  addi r9, r9, -0x56e8
	ctx.r[9].s64 = ctx.r[9].s64 + -22248;
	// 830DBDEC: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 830DBDF0: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 830DBDF4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBDF8: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 830DBDFC: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 830DBE00: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830DBE04: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830DBE08: 39288AE0  addi r9, r8, -0x7520
	ctx.r[9].s64 = ctx.r[8].s64 + -29984;
	// 830DBE0C: FBDF0088  std r30, 0x88(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u64 ) };
	// 830DBE10: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 830DBE14: 913F00A0  stw r9, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 830DBE18: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 830DBE1C: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 830DBE20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 830DBE24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DBE28: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 830DBE2C: 913F009C  stw r9, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[9].u32 ) };
	// 830DBE30: 915F00A4  stw r10, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 830DBE34: 4BFFA035  bl 0x830d5e68
	ctx.lr = 0x830DBE38;
	sub_830D5E68(ctx, base);
	// 830DBE38: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830DBE3C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DBE40: 907F00A8  stw r3, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[3].u32 ) };
	// 830DBE44: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBE48: 915F00B0  stw r10, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 830DBE4C: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 830DBE50: 913F00B4  stw r9, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[9].u32 ) };
	// 830DBE54: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 830DBE58: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830DBE5C: 39288AD0  addi r9, r8, -0x7530
	ctx.r[9].s64 = ctx.r[8].s64 + -30000;
	// 830DBE60: FBDF00B8  std r30, 0xb8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u64 ) };
	// 830DBE64: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 830DBE68: 913F00D0  stw r9, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[9].u32 ) };
	// 830DBE6C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830DBE70: 915F00C4  stw r10, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 830DBE74: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 830DBE78: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBE7C: 913F00CC  stw r9, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[9].u32 ) };
	// 830DBE80: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 830DBE84: 397F00B8  addi r11, r31, 0xb8
	ctx.r[11].s64 = ctx.r[31].s64 + 184;
	// 830DBE88: 915F00D4  stw r10, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 830DBE8C: 4BFF9F6D  bl 0x830d5df8
	ctx.lr = 0x830DBE90;
	sub_830D5DF8(ctx, base);
	// 830DBE90: 907F00D8  stw r3, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[3].u32 ) };
	// 830DBE94: 93DF00DC  stw r30, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 830DBE98: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBE9C: 93DF00E0  stw r30, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[30].u32 ) };
	// 830DBEA0: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBEA4: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 830DBEA8: 392B8ABC  addi r9, r11, -0x7544
	ctx.r[9].s64 = ctx.r[11].s64 + -30020;
	// 830DBEAC: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 830DBEB0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBEB4: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 830DBEB8: 913F0100  stw r9, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[9].u32 ) };
	// 830DBEBC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 830DBEC0: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830DBEC4: 917F00FC  stw r11, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 830DBEC8: 397F00DC  addi r11, r31, 0xdc
	ctx.r[11].s64 = ctx.r[31].s64 + 220;
	// 830DBECC: 915F00F4  stw r10, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 830DBED0: 913F00F8  stw r9, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[9].u32 ) };
	// 830DBED4: 915F0104  stw r10, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 830DBED8: 4BFF9F21  bl 0x830d5df8
	ctx.lr = 0x830DBEDC;
	sub_830D5DF8(ctx, base);
	// 830DBEDC: 907F0108  stw r3, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[3].u32 ) };
	// 830DBEE0: 397F010C  addi r11, r31, 0x10c
	ctx.r[11].s64 = ctx.r[31].s64 + 268;
	// 830DBEE4: 93DF010C  stw r30, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[30].u32 ) };
	// 830DBEE8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBEEC: 93DF0110  stw r30, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[30].u32 ) };
	// 830DBEF0: 93DF0114  stw r30, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 830DBEF4: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 830DBEF8: 93DF0118  stw r30, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u32 ) };
	// 830DBEFC: 392B8AB0  addi r9, r11, -0x7550
	ctx.r[9].s64 = ctx.r[11].s64 + -30032;
	// 830DBF00: 93DF011C  stw r30, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[30].u32 ) };
	// 830DBF04: 913F0130  stw r9, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[9].u32 ) };
	// 830DBF08: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBF0C: 915F0124  stw r10, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[10].u32 ) };
	// 830DBF10: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 830DBF14: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBF18: 917F0120  stw r11, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 830DBF1C: 913F0128  stw r9, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[9].u32 ) };
	// 830DBF20: 917F012C  stw r11, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 830DBF24: 915F0134  stw r10, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[10].u32 ) };
	// 830DBF28: 4BFF9ED1  bl 0x830d5df8
	ctx.lr = 0x830DBF2C;
	sub_830D5DF8(ctx, base);
	// 830DBF2C: 907F0138  stw r3, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[3].u32 ) };
	// 830DBF30: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBF34: 93DF013C  stw r30, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[30].u32 ) };
	// 830DBF38: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 830DBF3C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 830DBF40: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 830DBF44: 392B8AA0  addi r9, r11, -0x7560
	ctx.r[9].s64 = ctx.r[11].s64 + -30048;
	// 830DBF48: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 830DBF4C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBF50: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 830DBF54: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 830DBF58: 913F0160  stw r9, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[9].u32 ) };
	// 830DBF5C: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 830DBF60: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 830DBF64: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBF68: 917F015C  stw r11, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[11].u32 ) };
	// 830DBF6C: 913F0158  stw r9, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 830DBF70: 397F013C  addi r11, r31, 0x13c
	ctx.r[11].s64 = ctx.r[31].s64 + 316;
	// 830DBF74: 915F0164  stw r10, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 830DBF78: 4BFF9E81  bl 0x830d5df8
	ctx.lr = 0x830DBF7C;
	sub_830D5DF8(ctx, base);
	// 830DBF7C: 907F0168  stw r3, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[3].u32 ) };
	// 830DBF80: 93DF016C  stw r30, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[30].u32 ) };
	// 830DBF84: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBF88: 93DF0170  stw r30, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[30].u32 ) };
	// 830DBF8C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 830DBF90: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 830DBF94: 392B8A84  addi r9, r11, -0x757c
	ctx.r[9].s64 = ctx.r[11].s64 + -30076;
	// 830DBF98: 93DF0178  stw r30, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[30].u32 ) };
	// 830DBF9C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBFA0: 93DF017C  stw r30, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[30].u32 ) };
	// 830DBFA4: 913F0190  stw r9, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[9].u32 ) };
	// 830DBFA8: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 830DBFAC: 915F0184  stw r10, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[10].u32 ) };
	// 830DBFB0: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DBFB4: 917F0180  stw r11, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 830DBFB8: 917F018C  stw r11, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[11].u32 ) };
	// 830DBFBC: 397F016C  addi r11, r31, 0x16c
	ctx.r[11].s64 = ctx.r[31].s64 + 364;
	// 830DBFC0: 913F0188  stw r9, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[9].u32 ) };
	// 830DBFC4: 915F0194  stw r10, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[10].u32 ) };
	// 830DBFC8: 4BFF9E31  bl 0x830d5df8
	ctx.lr = 0x830DBFCC;
	sub_830D5DF8(ctx, base);
	// 830DBFCC: 907F0198  stw r3, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[3].u32 ) };
	// 830DBFD0: 93DF019C  stw r30, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[30].u32 ) };
	// 830DBFD4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DBFD8: 93DF01A0  stw r30, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 830DBFDC: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 830DBFE0: 93DF01A4  stw r30, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[30].u32 ) };
	// 830DBFE4: 392B8A64  addi r9, r11, -0x759c
	ctx.r[9].s64 = ctx.r[11].s64 + -30108;
	// 830DBFE8: 93DF01A8  stw r30, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[30].u32 ) };
	// 830DBFEC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DBFF0: 93DF01AC  stw r30, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[30].u32 ) };
	// 830DBFF4: 913F01C0  stw r9, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[9].u32 ) };
	// 830DBFF8: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 830DBFFC: 915F01B4  stw r10, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[10].u32 ) };
	// 830DC000: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DC004: 917F01B0  stw r11, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 830DC008: 917F01BC  stw r11, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[11].u32 ) };
	// 830DC00C: 397F019C  addi r11, r31, 0x19c
	ctx.r[11].s64 = ctx.r[31].s64 + 412;
	// 830DC010: 913F01B8  stw r9, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[9].u32 ) };
	// 830DC014: 915F01C4  stw r10, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[10].u32 ) };
	// 830DC018: 4BFF9DE1  bl 0x830d5df8
	ctx.lr = 0x830DC01C;
	sub_830D5DF8(ctx, base);
	// 830DC01C: 907F01C8  stw r3, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[3].u32 ) };
	// 830DC020: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DC024: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 830DC028: 93DF01D0  stw r30, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[30].u32 ) };
	// 830DC02C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 830DC030: 93DF01D4  stw r30, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[30].u32 ) };
	// 830DC034: 392B8430  addi r9, r11, -0x7bd0
	ctx.r[9].s64 = ctx.r[11].s64 + -31696;
	// 830DC038: 93DF01D8  stw r30, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[30].u32 ) };
	// 830DC03C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830DC040: 93DF01DC  stw r30, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[30].u32 ) };
	// 830DC044: 917F01E0  stw r11, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[11].u32 ) };
	// 830DC048: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DC04C: 913F01F0  stw r9, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[9].u32 ) };
	// 830DC050: 3920002C  li r9, 0x2c
	ctx.r[9].s64 = 44;
	// 830DC054: 915F01E4  stw r10, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[10].u32 ) };
	// 830DC058: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DC05C: 917F01EC  stw r11, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[11].u32 ) };
	// 830DC060: 397F01CC  addi r11, r31, 0x1cc
	ctx.r[11].s64 = ctx.r[31].s64 + 460;
	// 830DC064: 913F01E8  stw r9, 0x1e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), ctx.r[9].u32 ) };
	// 830DC068: 915F01F4  stw r10, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[10].u32 ) };
	// 830DC06C: 4BFF9D8D  bl 0x830d5df8
	ctx.lr = 0x830DC070;
	sub_830D5DF8(ctx, base);
	// 830DC070: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DC074: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DC078: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DC07C: 394A7358  addi r10, r10, 0x7358
	ctx.r[10].s64 = ctx.r[10].s64 + 29528;
	// 830DC080: 39297330  addi r9, r9, 0x7330
	ctx.r[9].s64 = ctx.r[9].s64 + 29488;
	// 830DC084: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 830DC088: 915F0200  stw r10, 0x200(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 830DC08C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 830DC090: 913F0204  stw r9, 0x204(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(516 as u32), ctx.r[9].u32 ) };
	// 830DC094: 907F01F8  stw r3, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[3].u32 ) };
	// 830DC098: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 830DC09C: FBDF0208  std r30, 0x208(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[30].u64 ) };
	// 830DC0A0: 3CE08219  lis r7, -0x7de7
	ctx.r[7].s64 = -2112290816;
	// 830DC0A4: 917F0210  stw r11, 0x210(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(528 as u32), ctx.r[11].u32 ) };
	// 830DC0A8: 915F0214  stw r10, 0x214(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(532 as u32), ctx.r[10].u32 ) };
	// 830DC0AC: 397F0208  addi r11, r31, 0x208
	ctx.r[11].s64 = ctx.r[31].s64 + 520;
	// 830DC0B0: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 830DC0B4: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830DC0B8: 394783E8  addi r10, r7, -0x7c18
	ctx.r[10].s64 = ctx.r[7].s64 + -31768;
	// 830DC0BC: 913F0218  stw r9, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[9].u32 ) };
	// 830DC0C0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830DC0C4: 911F021C  stw r8, 0x21c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(540 as u32), ctx.r[8].u32 ) };
	// 830DC0C8: 915F0220  stw r10, 0x220(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(544 as u32), ctx.r[10].u32 ) };
	// 830DC0CC: 917F0224  stw r11, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[11].u32 ) };
	// 830DC0D0: 4BFF8481  bl 0x830d4550
	ctx.lr = 0x830DC0D4;
	sub_830D4550(ctx, base);
	// 830DC0D4: 907F0228  stw r3, 0x228(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[3].u32 ) };
	// 830DC0D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DC0DC: 93DF022C  stw r30, 0x22c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), ctx.r[30].u32 ) };
	// 830DC0E0: 93DF0230  stw r30, 0x230(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), ctx.r[30].u32 ) };
	// 830DC0E4: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 830DC0E8: 93DF0234  stw r30, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[30].u32 ) };
	// 830DC0EC: 392B880C  addi r9, r11, -0x77f4
	ctx.r[9].s64 = ctx.r[11].s64 + -30708;
	// 830DC0F0: 93DF0238  stw r30, 0x238(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(568 as u32), ctx.r[30].u32 ) };
	// 830DC0F4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 830DC0F8: 93DF023C  stw r30, 0x23c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(572 as u32), ctx.r[30].u32 ) };
	// 830DC0FC: 917F0240  stw r11, 0x240(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(576 as u32), ctx.r[11].u32 ) };
	// 830DC100: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830DC104: 913F0250  stw r9, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[9].u32 ) };
	// 830DC108: 39200034  li r9, 0x34
	ctx.r[9].s64 = 52;
	// 830DC10C: 915F0244  stw r10, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[10].u32 ) };
	// 830DC110: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830DC114: 917F024C  stw r11, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[11].u32 ) };
	// 830DC118: 397F022C  addi r11, r31, 0x22c
	ctx.r[11].s64 = ctx.r[31].s64 + 556;
	// 830DC11C: 913F0248  stw r9, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[9].u32 ) };
	// 830DC120: 915F0254  stw r10, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[10].u32 ) };
	// 830DC124: 4BFF9CD5  bl 0x830d5df8
	ctx.lr = 0x830DC128;
	sub_830D5DF8(ctx, base);
	// 830DC128: 907F0258  stw r3, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[3].u32 ) };
	// 830DC12C: 93DF025C  stw r30, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[30].u32 ) };
	// 830DC130: 397F025C  addi r11, r31, 0x25c
	ctx.r[11].s64 = ctx.r[31].s64 + 604;
	// 830DC134: 93DF0260  stw r30, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u32 ) };
	// 830DC138: 93DF0264  stw r30, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[30].u32 ) };
	// 830DC13C: 93DF0268  stw r30, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u32 ) };
	// 830DC140: 93DF026C  stw r30, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[30].u32 ) };
	// 830DC144: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 830DC148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC14C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DC150: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DC154: 480CC068  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC158 size=144
    let mut pc: u32 = 0x830DC158;
    'dispatch: loop {
        match pc {
            0x830DC158 => {
    //   block [0x830DC158..0x830DC1E8)
	// 830DC158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DC160: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DC164: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DC16C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DC170: 48001369  bl 0x830dd4d8
	ctx.lr = 0x830DC174;
	sub_830DD4D8(ctx, base);
	// 830DC174: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830DC178: 48001361  bl 0x830dd4d8
	ctx.lr = 0x830DC17C;
	sub_830DD4D8(ctx, base);
	// 830DC17C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830DC180: 48001359  bl 0x830dd4d8
	ctx.lr = 0x830DC184;
	sub_830DD4D8(ctx, base);
	// 830DC184: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830DC188: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DC18C: 419A0020  beq cr6, 0x830dc1ac
	if ctx.cr[6].eq {
	pc = 0x830DC1AC; continue 'dispatch;
	}
	// 830DC190: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DC194: 480035C5  bl 0x830df758
	ctx.lr = 0x830DC198;
	sub_830DF758(ctx, base);
	// 830DC198: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC19C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DC1A0: 409A000C  bne cr6, 0x830dc1ac
	if !ctx.cr[6].eq {
	pc = 0x830DC1AC; continue 'dispatch;
	}
	// 830DC1A4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830DC1A8: 4BFFD681  bl 0x830d9828
	ctx.lr = 0x830DC1AC;
	sub_830D9828(ctx, base);
	// 830DC1AC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830DC1B0: 48001329  bl 0x830dd4d8
	ctx.lr = 0x830DC1B4;
	sub_830DD4D8(ctx, base);
	// 830DC1B4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830DC1B8: 48001321  bl 0x830dd4d8
	ctx.lr = 0x830DC1BC;
	sub_830DD4D8(ctx, base);
	// 830DC1BC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830DC1C0: 48001319  bl 0x830dd4d8
	ctx.lr = 0x830DC1C4;
	sub_830DD4D8(ctx, base);
	// 830DC1C4: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830DC1C8: 48001311  bl 0x830dd4d8
	ctx.lr = 0x830DC1CC;
	sub_830DD4D8(ctx, base);
	// 830DC1CC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830DC1D0: 4800A741  bl 0x830e6910
	ctx.lr = 0x830DC1D4;
	sub_830E6910(ctx, base);
	// 830DC1D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DC1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DC1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DC1E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DC1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC1E8 size=432
    let mut pc: u32 = 0x830DC1E8;
    'dispatch: loop {
        match pc {
            0x830DC1E8 => {
    //   block [0x830DC1E8..0x830DC398)
	// 830DC1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC1EC: 480CBF79  bl 0x831a8164
	ctx.lr = 0x830DC1F0;
	sub_831A8130(ctx, base);
	// 830DC1F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC1F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830DC1F8: 908100AC  stw r4, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 830DC1FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830DC200: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830DC204: 419A0018  beq cr6, 0x830dc21c
	if ctx.cr[6].eq {
	pc = 0x830DC21C; continue 'dispatch;
	}
	// 830DC208: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830DC20C: 4B1E3DF5  bl 0x822c0000
	ctx.lr = 0x830DC210;
	sub_822C0000(ctx, base);
	// 830DC210: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830DC214: 4802811D  bl 0x83104330
	ctx.lr = 0x830DC218;
	sub_83104330(ctx, base);
	// 830DC218: 48000178  b 0x830dc390
	pc = 0x830DC390; continue 'dispatch;
	// 830DC21C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DC220: 4BFFA7F9  bl 0x830d6a18
	ctx.lr = 0x830DC224;
	sub_830D6A18(ctx, base);
	// 830DC224: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830DC228: 40820010  bne 0x830dc238
	if !ctx.cr[0].eq {
	pc = 0x830DC238; continue 'dispatch;
	}
	// 830DC22C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DC230: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DC234: 4800015C  b 0x830dc390
	pc = 0x830DC390; continue 'dispatch;
	// 830DC238: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DC23C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DC240: 83ABC188  lwz r29, -0x3e78(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15992 as u32) ) } as u64;
	// 830DC244: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DC248: 4182000C  beq 0x830dc254
	if ctx.cr[0].eq {
	pc = 0x830DC254; continue 'dispatch;
	}
	// 830DC24C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DC250: 419A00D8  beq cr6, 0x830dc328
	if ctx.cr[6].eq {
	pc = 0x830DC328; continue 'dispatch;
	}
	// 830DC254: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DC258: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DC25C: 480258C5  bl 0x83101b20
	ctx.lr = 0x830DC260;
	sub_83101B20(ctx, base);
	// 830DC260: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC264: 418000C4  blt 0x830dc328
	if ctx.cr[0].lt {
	pc = 0x830DC328; continue 'dispatch;
	}
	// 830DC268: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830DC26C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DC270: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830DC274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830DC278: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DC27C: 48007DA5  bl 0x830e4020
	ctx.lr = 0x830DC280;
	sub_830E4020(ctx, base);
	// 830DC280: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC284: 41800064  blt 0x830dc2e8
	if ctx.cr[0].lt {
	pc = 0x830DC2E8; continue 'dispatch;
	}
	// 830DC288: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830DC28C: 48001225  bl 0x830dd4b0
	ctx.lr = 0x830DC290;
	sub_830DD4B0(ctx, base);
	// 830DC290: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DC294: 41820010  beq 0x830dc2a4
	if ctx.cr[0].eq {
	pc = 0x830DC2A4; continue 'dispatch;
	}
	// 830DC298: 480274D1  bl 0x83103768
	ctx.lr = 0x830DC29C;
	sub_83103768(ctx, base);
	// 830DC29C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DC2A0: 48000008  b 0x830dc2a8
	pc = 0x830DC2A8; continue 'dispatch;
	// 830DC2A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830DC2A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DC2AC: 409A0010  bne cr6, 0x830dc2bc
	if !ctx.cr[6].eq {
	pc = 0x830DC2BC; continue 'dispatch;
	}
	// 830DC2B0: 3FE08007  lis r31, -0x7ff9
	ctx.r[31].s64 = -2147024896;
	// 830DC2B4: 63FF000E  ori r31, r31, 0xe
	ctx.r[31].u64 = ctx.r[31].u64 | 14;
	// 830DC2B8: 48000028  b 0x830dc2e0
	pc = 0x830DC2E0; continue 'dispatch;
	// 830DC2BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830DC2C0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC2C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC2C8: 48028079  bl 0x83104340
	ctx.lr = 0x830DC2CC;
	sub_83104340(ctx, base);
	// 830DC2CC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC2D0: 41800010  blt 0x830dc2e0
	if ctx.cr[0].lt {
	pc = 0x830DC2E0; continue 'dispatch;
	}
	// 830DC2D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC2D8: 4B1E3D29  bl 0x822c0000
	ctx.lr = 0x830DC2DC;
	sub_822C0000(ctx, base);
	// 830DC2DC: 906100AC  stw r3, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[3].u32 ) };
	// 830DC2E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC2E4: 4BFFD545  bl 0x830d9828
	ctx.lr = 0x830DC2E8;
	sub_830D9828(ctx, base);
	// 830DC2E8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DC2EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DC2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DC2F4: 419A0010  beq cr6, 0x830dc304
	if ctx.cr[6].eq {
	pc = 0x830DC304; continue 'dispatch;
	}
	// 830DC2F8: 48026D11  bl 0x83103008
	ctx.lr = 0x830DC2FC;
	sub_83103008(ctx, base);
	// 830DC2FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC300: 480011D9  bl 0x830dd4d8
	ctx.lr = 0x830DC304;
	sub_830DD4D8(ctx, base);
	// 830DC304: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC308: 41980020  blt cr6, 0x830dc328
	if ctx.cr[6].lt {
	pc = 0x830DC328; continue 'dispatch;
	}
	// 830DC30C: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 830DC310: 4B1E3CF1  bl 0x822c0000
	ctx.lr = 0x830DC314;
	sub_822C0000(ctx, base);
	// 830DC314: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830DC318: 48028019  bl 0x83104330
	ctx.lr = 0x830DC31C;
	sub_83104330(ctx, base);
	// 830DC31C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DC320: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 830DC324: 480040DD  bl 0x830e0400
	ctx.lr = 0x830DC328;
	sub_830E0400(ctx, base);
	// 830DC328: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830DC32C: 419A0060  beq cr6, 0x830dc38c
	if ctx.cr[6].eq {
	pc = 0x830DC38C; continue 'dispatch;
	}
	// 830DC330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC334: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DC338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830DC33C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DC340: 4BFF5ED9  bl 0x830d2218
	ctx.lr = 0x830DC344;
	sub_830D2218(ctx, base);
	// 830DC344: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC348: 41800044  blt 0x830dc38c
	if ctx.cr[0].lt {
	pc = 0x830DC38C; continue 'dispatch;
	}
	// 830DC34C: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DC350: 388100AC  addi r4, r1, 0xac
	ctx.r[4].s64 = ctx.r[1].s64 + 172;
	// 830DC354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC358: 48005829  bl 0x830e1b80
	ctx.lr = 0x830DC35C;
	sub_830E1B80(ctx, base);
	// 830DC35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DC360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC364: 48001175  bl 0x830dd4d8
	ctx.lr = 0x830DC368;
	sub_830DD4D8(ctx, base);
	// 830DC368: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC36C: 41980020  blt cr6, 0x830dc38c
	if ctx.cr[6].lt {
	pc = 0x830DC38C; continue 'dispatch;
	}
	// 830DC370: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 830DC374: 4B1E3C8D  bl 0x822c0000
	ctx.lr = 0x830DC378;
	sub_822C0000(ctx, base);
	// 830DC378: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830DC37C: 48027FB5  bl 0x83104330
	ctx.lr = 0x830DC380;
	sub_83104330(ctx, base);
	// 830DC380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DC384: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 830DC388: 48004079  bl 0x830e0400
	ctx.lr = 0x830DC38C;
	sub_830E0400(ctx, base);
	// 830DC38C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC390: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DC394: 480CBE20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC398 size=952
    let mut pc: u32 = 0x830DC398;
    'dispatch: loop {
        match pc {
            0x830DC398 => {
    //   block [0x830DC398..0x830DC750)
	// 830DC398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC39C: 480CBD9D  bl 0x831a8138
	ctx.lr = 0x830DC3A0;
	sub_831A8130(ctx, base);
	// 830DC3A0: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC3A4: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 830DC3A8: 817DC184  lwz r11, -0x3e7c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-15996 as u32) ) } as u64;
	// 830DC3AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DC3B0: 409A000C  bne cr6, 0x830dc3bc
	if !ctx.cr[6].eq {
	pc = 0x830DC3BC; continue 'dispatch;
	}
	// 830DC3B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DC3B8: 48000390  b 0x830dc748
	pc = 0x830DC748; continue 'dispatch;
	// 830DC3BC: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830DC3C0: 807EC188  lwz r3, -0x3e78(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-15992 as u32) ) } as u64;
	// 830DC3C4: 48001115  bl 0x830dd4d8
	ctx.lr = 0x830DC3C8;
	sub_830DD4D8(ctx, base);
	// 830DC3C8: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 830DC3CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC3D0: 917EC188  stw r11, -0x3e78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-15992 as u32), ctx.r[11].u32 ) };
	// 830DC3D4: 807F56F8  lwz r3, 0x56f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22264 as u32) ) } as u64;
	// 830DC3D8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830DC3DC: 419A0010  beq cr6, 0x830dc3ec
	if ctx.cr[6].eq {
	pc = 0x830DC3EC; continue 'dispatch;
	}
	// 830DC3E0: 481668FD  bl 0x83242cdc
	ctx.lr = 0x830DC3E4;
	// extern call 0x83242CDC → crate::xboxkrnl::KeTlsFree
	crate::xboxkrnl::KeTlsFree(ctx, base);
	// 830DC3E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830DC3E8: 917F56F8  stw r11, 0x56f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22264 as u32), ctx.r[11].u32 ) };
	// 830DC3EC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DC3F0: 3BEBC1B0  addi r31, r11, -0x3e50
	ctx.r[31].s64 = ctx.r[11].s64 + -15952;
	// 830DC3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC3F8: 48026FE1  bl 0x831033d8
	ctx.lr = 0x830DC3FC;
	sub_831033D8(ctx, base);
	// 830DC3FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC400: 48026F91  bl 0x83103390
	ctx.lr = 0x830DC404;
	sub_83103390(ctx, base);
	// 830DC404: 3D60830E  lis r11, -0x7cf2
	ctx.r[11].s64 = -2096234496;
	// 830DC408: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DC40C: 396BB3C0  addi r11, r11, -0x4c40
	ctx.r[11].s64 = ctx.r[11].s64 + -19520;
	// 830DC410: 3D40830E  lis r10, -0x7cf2
	ctx.r[10].s64 = -2096234496;
	// 830DC414: 3D20830E  lis r9, -0x7cf2
	ctx.r[9].s64 = -2096234496;
	// 830DC418: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830DC41C: 394AB300  addi r10, r10, -0x4d00
	ctx.r[10].s64 = ctx.r[10].s64 + -19712;
	// 830DC420: 3929B240  addi r9, r9, -0x4dc0
	ctx.r[9].s64 = ctx.r[9].s64 + -19904;
	// 830DC424: 3968B230  addi r11, r8, -0x4dd0
	ctx.r[11].s64 = ctx.r[8].s64 + -19920;
	// 830DC428: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 830DC42C: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DC430: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 830DC434: 3CC0830E  lis r6, -0x7cf2
	ctx.r[6].s64 = -2096234496;
	// 830DC438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830DC43C: 3CA0830E  lis r5, -0x7cf2
	ctx.r[5].s64 = -2096234496;
	// 830DC440: 3947B180  addi r10, r7, -0x4e80
	ctx.r[10].s64 = ctx.r[7].s64 + -20096;
	// 830DC444: 3926B0C0  addi r9, r6, -0x4f40
	ctx.r[9].s64 = ctx.r[6].s64 + -20288;
	// 830DC448: 3965B000  addi r11, r5, -0x5000
	ctx.r[11].s64 = ctx.r[5].s64 + -20480;
	// 830DC44C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 830DC450: 3C80830E  lis r4, -0x7cf2
	ctx.r[4].s64 = -2096234496;
	// 830DC454: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 830DC458: 3C60830E  lis r3, -0x7cf2
	ctx.r[3].s64 = -2096234496;
	// 830DC45C: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 830DC460: 3FE0830E  lis r31, -0x7cf2
	ctx.r[31].s64 = -2096234496;
	// 830DC464: 3944AF40  addi r10, r4, -0x50c0
	ctx.r[10].s64 = ctx.r[4].s64 + -20672;
	// 830DC468: 3923AE80  addi r9, r3, -0x5180
	ctx.r[9].s64 = ctx.r[3].s64 + -20864;
	// 830DC46C: 397FA710  addi r11, r31, -0x58f0
	ctx.r[11].s64 = ctx.r[31].s64 + -22768;
	// 830DC470: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830DC474: 3FC0830E  lis r30, -0x7cf2
	ctx.r[30].s64 = -2096234496;
	// 830DC478: 91210080  stw r9, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 830DC47C: 3F80830E  lis r28, -0x7cf2
	ctx.r[28].s64 = -2096234496;
	// 830DC480: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830DC484: 3F60830E  lis r27, -0x7cf2
	ctx.r[27].s64 = -2096234496;
	// 830DC488: 395EA7C0  addi r10, r30, -0x5840
	ctx.r[10].s64 = ctx.r[30].s64 + -22592;
	// 830DC48C: 393CAE70  addi r9, r28, -0x5190
	ctx.r[9].s64 = ctx.r[28].s64 + -20880;
	// 830DC490: 397BADB0  addi r11, r27, -0x5250
	ctx.r[11].s64 = ctx.r[27].s64 + -21072;
	// 830DC494: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 830DC498: 3F40830E  lis r26, -0x7cf2
	ctx.r[26].s64 = -2096234496;
	// 830DC49C: 9121008C  stw r9, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 830DC4A0: 3F20830E  lis r25, -0x7cf2
	ctx.r[25].s64 = -2096234496;
	// 830DC4A4: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 830DC4A8: 3F00830E  lis r24, -0x7cf2
	ctx.r[24].s64 = -2096234496;
	// 830DC4AC: 395A9C70  addi r10, r26, -0x6390
	ctx.r[10].s64 = ctx.r[26].s64 + -25488;
	// 830DC4B0: 3939AB70  addi r9, r25, -0x5490
	ctx.r[9].s64 = ctx.r[25].s64 + -21648;
	// 830DC4B4: 3978ACF0  addi r11, r24, -0x5310
	ctx.r[11].s64 = ctx.r[24].s64 + -21264;
	// 830DC4B8: 91410094  stw r10, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 830DC4BC: 3EE0830E  lis r23, -0x7cf2
	ctx.r[23].s64 = -2096234496;
	// 830DC4C0: 91210098  stw r9, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[9].u32 ) };
	// 830DC4C4: 3EC0830E  lis r22, -0x7cf2
	ctx.r[22].s64 = -2096234496;
	// 830DC4C8: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 830DC4CC: 3EA0830E  lis r21, -0x7cf2
	ctx.r[21].s64 = -2096234496;
	// 830DC4D0: 3957AC30  addi r10, r23, -0x53d0
	ctx.r[10].s64 = ctx.r[23].s64 + -21456;
	// 830DC4D4: 3936AA00  addi r9, r22, -0x5600
	ctx.r[9].s64 = ctx.r[22].s64 + -22016;
	// 830DC4D8: 3975AAB0  addi r11, r21, -0x5550
	ctx.r[11].s64 = ctx.r[21].s64 + -21840;
	// 830DC4DC: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 830DC4E0: 912100A4  stw r9, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[9].u32 ) };
	// 830DC4E4: 3D40830E  lis r10, -0x7cf2
	ctx.r[10].s64 = -2096234496;
	// 830DC4E8: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 830DC4EC: 3D20830E  lis r9, -0x7cf2
	ctx.r[9].s64 = -2096234496;
	// 830DC4F0: 3D60830E  lis r11, -0x7cf2
	ctx.r[11].s64 = -2096234496;
	// 830DC4F4: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DC4F8: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DC4FC: 3CC0830E  lis r6, -0x7cf2
	ctx.r[6].s64 = -2096234496;
	// 830DC500: 3CA0830E  lis r5, -0x7cf2
	ctx.r[5].s64 = -2096234496;
	// 830DC504: 3C80830E  lis r4, -0x7cf2
	ctx.r[4].s64 = -2096234496;
	// 830DC508: 3C60830E  lis r3, -0x7cf2
	ctx.r[3].s64 = -2096234496;
	// 830DC50C: 3FE0830E  lis r31, -0x7cf2
	ctx.r[31].s64 = -2096234496;
	// 830DC510: 3FC0830E  lis r30, -0x7cf2
	ctx.r[30].s64 = -2096234496;
	// 830DC514: 3F80830E  lis r28, -0x7cf2
	ctx.r[28].s64 = -2096234496;
	// 830DC518: 3F60830E  lis r27, -0x7cf2
	ctx.r[27].s64 = -2096234496;
	// 830DC51C: 3F40830E  lis r26, -0x7cf2
	ctx.r[26].s64 = -2096234496;
	// 830DC520: 3F20830E  lis r25, -0x7cf2
	ctx.r[25].s64 = -2096234496;
	// 830DC524: 3F00830E  lis r24, -0x7cf2
	ctx.r[24].s64 = -2096234496;
	// 830DC528: 3EE0830E  lis r23, -0x7cf2
	ctx.r[23].s64 = -2096234496;
	// 830DC52C: 3EC0830E  lis r22, -0x7cf2
	ctx.r[22].s64 = -2096234496;
	// 830DC530: 394AA940  addi r10, r10, -0x56c0
	ctx.r[10].s64 = ctx.r[10].s64 + -22208;
	// 830DC534: 3929A880  addi r9, r9, -0x5780
	ctx.r[9].s64 = ctx.r[9].s64 + -22400;
	// 830DC538: 396BA650  addi r11, r11, -0x59b0
	ctx.r[11].s64 = ctx.r[11].s64 + -22960;
	// 830DC53C: 914100AC  stw r10, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 830DC540: 912100B0  stw r9, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 830DC544: 3948A640  addi r10, r8, -0x59c0
	ctx.r[10].s64 = ctx.r[8].s64 + -22976;
	// 830DC548: 916100B4  stw r11, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 830DC54C: 3927A4C0  addi r9, r7, -0x5b40
	ctx.r[9].s64 = ctx.r[7].s64 + -23360;
	// 830DC550: 3966A580  addi r11, r6, -0x5a80
	ctx.r[11].s64 = ctx.r[6].s64 + -23168;
	// 830DC554: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 830DC558: 912100BC  stw r9, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[9].u32 ) };
	// 830DC55C: 3945A400  addi r10, r5, -0x5c00
	ctx.r[10].s64 = ctx.r[5].s64 + -23552;
	// 830DC560: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 830DC564: 3924A3F0  addi r9, r4, -0x5c10
	ctx.r[9].s64 = ctx.r[4].s64 + -23568;
	// 830DC568: 3963A3E0  addi r11, r3, -0x5c20
	ctx.r[11].s64 = ctx.r[3].s64 + -23584;
	// 830DC56C: 914100C4  stw r10, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 830DC570: 912100C8  stw r9, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 830DC574: 395FA320  addi r10, r31, -0x5ce0
	ctx.r[10].s64 = ctx.r[31].s64 + -23776;
	// 830DC578: 916100CC  stw r11, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 830DC57C: 393E9D40  addi r9, r30, -0x62c0
	ctx.r[9].s64 = ctx.r[30].s64 + -25280;
	// 830DC580: 397CA270  addi r11, r28, -0x5d90
	ctx.r[11].s64 = ctx.r[28].s64 + -23952;
	// 830DC584: 914100D0  stw r10, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[10].u32 ) };
	// 830DC588: 912100D4  stw r9, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[9].u32 ) };
	// 830DC58C: 395B9BB0  addi r10, r27, -0x6450
	ctx.r[10].s64 = ctx.r[27].s64 + -25680;
	// 830DC590: 916100D8  stw r11, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 830DC594: 393AA1B0  addi r9, r26, -0x5e50
	ctx.r[9].s64 = ctx.r[26].s64 + -24144;
	// 830DC598: 3979A100  addi r11, r25, -0x5f00
	ctx.r[11].s64 = ctx.r[25].s64 + -24320;
	// 830DC59C: 914100DC  stw r10, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[10].u32 ) };
	// 830DC5A0: 912100E0  stw r9, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[9].u32 ) };
	// 830DC5A4: 3958A040  addi r10, r24, -0x5fc0
	ctx.r[10].s64 = ctx.r[24].s64 + -24512;
	// 830DC5A8: 916100E4  stw r11, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 830DC5AC: 39379ED0  addi r9, r23, -0x6130
	ctx.r[9].s64 = ctx.r[23].s64 + -24880;
	// 830DC5B0: 39769F80  addi r11, r22, -0x6080
	ctx.r[11].s64 = ctx.r[22].s64 + -24704;
	// 830DC5B4: 914100E8  stw r10, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 830DC5B8: 3EA0830E  lis r21, -0x7cf2
	ctx.r[21].s64 = -2096234496;
	// 830DC5BC: 912100EC  stw r9, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[9].u32 ) };
	// 830DC5C0: 3E80830E  lis r20, -0x7cf2
	ctx.r[20].s64 = -2096234496;
	// 830DC5C4: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830DC5C8: 3E60830E  lis r19, -0x7cf2
	ctx.r[19].s64 = -2096234496;
	// 830DC5CC: 39559E10  addi r10, r21, -0x61f0
	ctx.r[10].s64 = ctx.r[21].s64 + -25072;
	// 830DC5D0: 39349D50  addi r9, r20, -0x62b0
	ctx.r[9].s64 = ctx.r[20].s64 + -25264;
	// 830DC5D4: 39739C90  addi r11, r19, -0x6370
	ctx.r[11].s64 = ctx.r[19].s64 + -25456;
	// 830DC5D8: 914100F4  stw r10, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 830DC5DC: 3E40830E  lis r18, -0x7cf2
	ctx.r[18].s64 = -2096234496;
	// 830DC5E0: 912100F8  stw r9, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[9].u32 ) };
	// 830DC5E4: 3E20830E  lis r17, -0x7cf2
	ctx.r[17].s64 = -2096234496;
	// 830DC5E8: 916100FC  stw r11, 0xfc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 830DC5EC: 3E00830E  lis r16, -0x7cf2
	ctx.r[16].s64 = -2096234496;
	// 830DC5F0: 39529C80  addi r10, r18, -0x6380
	ctx.r[10].s64 = ctx.r[18].s64 + -25472;
	// 830DC5F4: 39319AF0  addi r9, r17, -0x6510
	ctx.r[9].s64 = ctx.r[17].s64 + -25872;
	// 830DC5F8: 39709A28  addi r11, r16, -0x65d8
	ctx.r[11].s64 = ctx.r[16].s64 + -26072;
	// 830DC5FC: 91410100  stw r10, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[10].u32 ) };
	// 830DC600: 91210104  stw r9, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[9].u32 ) };
	// 830DC604: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 830DC608: 91610108  stw r11, 0x108(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 830DC60C: 3BE0002B  li r31, 0x2b
	ctx.r[31].s64 = 43;
	// 830DC610: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DC614: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DC618: 4E800421  bctrl
	ctx.lr = 0x830DC61C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DC61C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DC620: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830DC624: 4082FFEC  bne 0x830dc610
	if !ctx.cr[0].eq {
	pc = 0x830DC610; continue 'dispatch;
	}
	// 830DC628: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DC62C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DC630: 3BEBC3E0  addi r31, r11, -0x3c20
	ctx.r[31].s64 = ctx.r[11].s64 + -15392;
	// 830DC634: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830DC638: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DC63C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830DC640: 409A0024  bne cr6, 0x830dc664
	if !ctx.cr[6].eq {
	pc = 0x830DC664; continue 'dispatch;
	}
	// 830DC644: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830DC648: 393F006C  addi r9, r31, 0x6c
	ctx.r[9].s64 = ctx.r[31].s64 + 108;
	// 830DC64C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830DC650: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830DC654: 4198FFE4  blt cr6, 0x830dc638
	if ctx.cr[6].lt {
	pc = 0x830DC638; continue 'dispatch;
	}
	// 830DC658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC65C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DC660: 48000044  b 0x830dc6a4
	pc = 0x830DC6A4; continue 'dispatch;
	// 830DC664: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DC668: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 830DC66C: 4BFFFFF0  b 0x830dc65c
	pc = 0x830DC65C; continue 'dispatch;
	// 830DC670: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DC674: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DC678: 48000E61  bl 0x830dd4d8
	ctx.lr = 0x830DC67C;
	sub_830DD4D8(ctx, base);
	// 830DC67C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DC680: 48000E59  bl 0x830dd4d8
	ctx.lr = 0x830DC684;
	sub_830DD4D8(ctx, base);
	// 830DC684: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830DC688: 48000E51  bl 0x830dd4d8
	ctx.lr = 0x830DC68C;
	sub_830DD4D8(ctx, base);
	// 830DC68C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC690: 48000E49  bl 0x830dd4d8
	ctx.lr = 0x830DC694;
	sub_830DD4D8(ctx, base);
	// 830DC694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC698: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DC69C: 4BFF268D  bl 0x830ced28
	ctx.lr = 0x830DC6A0;
	sub_830CED28(ctx, base);
	// 830DC6A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC6A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DC6A8: 409AFFC8  bne cr6, 0x830dc670
	if !ctx.cr[6].eq {
	pc = 0x830DC670; continue 'dispatch;
	}
	// 830DC6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC6B0: 4BFF98F9  bl 0x830d5fa8
	ctx.lr = 0x830DC6B4;
	sub_830D5FA8(ctx, base);
	// 830DC6B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DC6B8: 48008611  bl 0x830e4cc8
	ctx.lr = 0x830DC6BC;
	sub_830E4CC8(ctx, base);
	// 830DC6BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DC6C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DC6C4: 4800843D  bl 0x830e4b00
	ctx.lr = 0x830DC6C8;
	sub_830E4B00(ctx, base);
	// 830DC6C8: 48007DD9  bl 0x830e44a0
	ctx.lr = 0x830DC6CC;
	sub_830E44A0(ctx, base);
	// 830DC6CC: 48023755  bl 0x830ffe20
	ctx.lr = 0x830DC6D0;
	sub_830FFE20(ctx, base);
	// 830DC6D0: 48022B31  bl 0x830ff200
	ctx.lr = 0x830DC6D4;
	sub_830FF200(ctx, base);
	// 830DC6D4: 48001FED  bl 0x830de6c0
	ctx.lr = 0x830DC6D8;
	sub_830DE6C0(ctx, base);
	// 830DC6D8: 48002469  bl 0x830deb40
	ctx.lr = 0x830DC6DC;
	sub_830DEB40(ctx, base);
	// 830DC6DC: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830DC6E0: 3BDFC190  addi r30, r31, -0x3e70
	ctx.r[30].s64 = ctx.r[31].s64 + -15984;
	// 830DC6E4: 807FC190  lwz r3, -0x3e70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-15984 as u32) ) } as u64;
	// 830DC6E8: 48000DF1  bl 0x830dd4d8
	ctx.lr = 0x830DC6EC;
	sub_830DD4D8(ctx, base);
	// 830DC6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DC6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830DC6F8: 917FC190  stw r11, -0x3e70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-15984 as u32), ctx.r[11].u32 ) };
	// 830DC6FC: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830DC700: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830DC704: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830DC708: 3BDFC19C  addi r30, r31, -0x3e64
	ctx.r[30].s64 = ctx.r[31].s64 + -15972;
	// 830DC70C: 807FC19C  lwz r3, -0x3e64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-15972 as u32) ) } as u64;
	// 830DC710: 48000DC9  bl 0x830dd4d8
	ctx.lr = 0x830DC714;
	sub_830DD4D8(ctx, base);
	// 830DC714: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830DC718: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 830DC71C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DC724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830DC728: 917FC19C  stw r11, -0x3e64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-15972 as u32), ctx.r[11].u32 ) };
	// 830DC72C: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830DC730: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DC734: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830DC738: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DC73C: 9148C10C  stw r10, -0x3ef4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-16116 as u32), ctx.r[10].u32 ) };
	// 830DC740: 9127C18C  stw r9, -0x3e74(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-15988 as u32), ctx.r[9].u32 ) };
	// 830DC744: 917DC184  stw r11, -0x3e7c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-15996 as u32), ctx.r[11].u32 ) };
	// 830DC748: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 830DC74C: 480CBA3C  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DC750 size=8
    let mut pc: u32 = 0x830DC750;
    'dispatch: loop {
        match pc {
            0x830DC750 => {
    //   block [0x830DC750..0x830DC758)
	// 830DC750: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830DC754: 4BFFFA94  b 0x830dc1e8
	sub_830DC1E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC758 size=260
    let mut pc: u32 = 0x830DC758;
    'dispatch: loop {
        match pc {
            0x830DC758 => {
    //   block [0x830DC758..0x830DC85C)
	// 830DC758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC75C: 480CBA0D  bl 0x831a8168
	ctx.lr = 0x830DC760;
	sub_831A8130(ctx, base);
	// 830DC760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DC768: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830DC76C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830DC770: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830DC774: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DC778: 419A00D4  beq cr6, 0x830dc84c
	if ctx.cr[6].eq {
	pc = 0x830DC84C; continue 'dispatch;
	}
	// 830DC77C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830DC780: 419A00CC  beq cr6, 0x830dc84c
	if ctx.cr[6].eq {
	pc = 0x830DC84C; continue 'dispatch;
	}
	// 830DC784: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC788: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DC78C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DC790: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DC794: 4802538D  bl 0x83101b20
	ctx.lr = 0x830DC798;
	sub_83101B20(ctx, base);
	// 830DC798: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DC79C: 418000B8  blt 0x830dc854
	if ctx.cr[0].lt {
	pc = 0x830DC854; continue 'dispatch;
	}
	// 830DC7A0: 7FCBF0F8  nor r11, r30, r30
	ctx.r[11].u64 = !(ctx.r[30].u64 | ctx.r[30].u64);
	// 830DC7A4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DC7A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830DC7AC: 556607FE  clrlwi r6, r11, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830DC7B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830DC7B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DC7B8: 48007869  bl 0x830e4020
	ctx.lr = 0x830DC7BC;
	sub_830E4020(ctx, base);
	// 830DC7BC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DC7C0: 41800068  blt 0x830dc828
	if ctx.cr[0].lt {
	pc = 0x830DC828; continue 'dispatch;
	}
	// 830DC7C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DC7C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC7CC: 4BFFA2A5  bl 0x830d6a70
	ctx.lr = 0x830DC7D0;
	sub_830D6A70(ctx, base);
	// 830DC7D0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DC7D4: 41800040  blt 0x830dc814
	if ctx.cr[0].lt {
	pc = 0x830DC814; continue 'dispatch;
	}
	// 830DC7D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DC7DC: 816BC188  lwz r11, -0x3e78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15992 as u32) ) } as u64;
	// 830DC7E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DC7E4: 419A0014  beq cr6, 0x830dc7f8
	if ctx.cr[6].eq {
	pc = 0x830DC7F8; continue 'dispatch;
	}
	// 830DC7E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830DC7EC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC7F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DC7F4: 4BFFF9F5  bl 0x830dc1e8
	ctx.lr = 0x830DC7F8;
	sub_830DC1E8(ctx, base);
	// 830DC7F8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830DC7FC: 419A0024  beq cr6, 0x830dc820
	if ctx.cr[6].eq {
	pc = 0x830DC820; continue 'dispatch;
	}
	// 830DC800: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DC804: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC808: 48005559  bl 0x830e1d60
	ctx.lr = 0x830DC80C;
	sub_830E1D60(ctx, base);
	// 830DC80C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DC810: 40800010  bge 0x830dc820
	if !ctx.cr[0].lt {
	pc = 0x830DC820; continue 'dispatch;
	}
	// 830DC814: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC818: 4BFFD011  bl 0x830d9828
	ctx.lr = 0x830DC81C;
	sub_830D9828(ctx, base);
	// 830DC81C: 4800000C  b 0x830dc828
	pc = 0x830DC828; continue 'dispatch;
	// 830DC820: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DC824: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DC828: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DC82C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DC830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DC834: 419A0010  beq cr6, 0x830dc844
	if ctx.cr[6].eq {
	pc = 0x830DC844; continue 'dispatch;
	}
	// 830DC838: 480267D1  bl 0x83103008
	ctx.lr = 0x830DC83C;
	sub_83103008(ctx, base);
	// 830DC83C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DC840: 48000C99  bl 0x830dd4d8
	ctx.lr = 0x830DC844;
	sub_830DD4D8(ctx, base);
	// 830DC844: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DC848: 4800000C  b 0x830dc854
	pc = 0x830DC854; continue 'dispatch;
	// 830DC84C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DC850: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DC854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DC858: 480CB960  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DC860 size=8
    let mut pc: u32 = 0x830DC860;
    'dispatch: loop {
        match pc {
            0x830DC860 => {
    //   block [0x830DC860..0x830DC868)
	// 830DC860: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830DC864: 4BFFFEF4  b 0x830dc758
	sub_830DC758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC868 size=160
    let mut pc: u32 = 0x830DC868;
    'dispatch: loop {
        match pc {
            0x830DC868 => {
    //   block [0x830DC868..0x830DC908)
	// 830DC868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DC870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DC874: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC878: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DC87C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DC880: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DC884: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DC888: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DC88C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DC890: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DC894: 48008865  bl 0x830e50f8
	ctx.lr = 0x830DC898;
	sub_830E50F8(ctx, base);
	// 830DC898: 3D60830D  lis r11, -0x7cf3
	ctx.r[11].s64 = -2096300032;
	// 830DC89C: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DC8A0: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830DC8A4: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DC8A8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 830DC8AC: 396B2988  addi r11, r11, 0x2988
	ctx.r[11].s64 = ctx.r[11].s64 + 10632;
	// 830DC8B0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DC8B4: 394A2A00  addi r10, r10, 0x2a00
	ctx.r[10].s64 = ctx.r[10].s64 + 10752;
	// 830DC8B8: 3929EDA8  addi r9, r9, -0x1258
	ctx.r[9].s64 = ctx.r[9].s64 + -4696;
	// 830DC8BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DC8C0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DC8C4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DC8C8: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DC8CC: 4BFFEDA5  bl 0x830db670
	ctx.lr = 0x830DC8D0;
	sub_830DB670(ctx, base);
	// 830DC8D0: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DC8D4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DC8D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DC8DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DC8E0: 388BC06C  addi r4, r11, -0x3f94
	ctx.r[4].s64 = ctx.r[11].s64 + -16276;
	// 830DC8E4: 4BFFCB65  bl 0x830d9448
	ctx.lr = 0x830DC8E8;
	sub_830D9448(ctx, base);
	// 830DC8E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DC8EC: 41800008  blt 0x830dc8f4
	if ctx.cr[0].lt {
	pc = 0x830DC8F4; continue 'dispatch;
	}
	// 830DC8F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DC8F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DC8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DC8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DC900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DC904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC908 size=176
    let mut pc: u32 = 0x830DC908;
    'dispatch: loop {
        match pc {
            0x830DC908 => {
    //   block [0x830DC908..0x830DC9B8)
	// 830DC908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DC910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DC914: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC918: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DC91C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DC920: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DC924: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DC928: 394A7BA0  addi r10, r10, 0x7ba0
	ctx.r[10].s64 = ctx.r[10].s64 + 31648;
	// 830DC92C: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DC930: 39297B64  addi r9, r9, 0x7b64
	ctx.r[9].s64 = ctx.r[9].s64 + 31588;
	// 830DC934: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DC938: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DC93C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DC940: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DC944: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DC948: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DC94C: 4B6F288D  bl 0x827cf1d8
	ctx.lr = 0x830DC950;
	sub_827CF1D8(ctx, base);
	// 830DC950: 3D60830D  lis r11, -0x7cf3
	ctx.r[11].s64 = -2096300032;
	// 830DC954: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DC958: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DC95C: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DC960: 396B2AC0  addi r11, r11, 0x2ac0
	ctx.r[11].s64 = ctx.r[11].s64 + 10944;
	// 830DC964: 394A2B88  addi r10, r10, 0x2b88
	ctx.r[10].s64 = ctx.r[10].s64 + 11144;
	// 830DC968: 3929EDF0  addi r9, r9, -0x1210
	ctx.r[9].s64 = ctx.r[9].s64 + -4624;
	// 830DC96C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DC970: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DC974: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DC978: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DC97C: 4BFFF34D  bl 0x830dbcc8
	ctx.lr = 0x830DC980;
	sub_830DBCC8(ctx, base);
	// 830DC980: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DC984: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DC988: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DC98C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DC990: 388BC078  addi r4, r11, -0x3f88
	ctx.r[4].s64 = ctx.r[11].s64 + -16264;
	// 830DC994: 4BFFCAB5  bl 0x830d9448
	ctx.lr = 0x830DC998;
	sub_830D9448(ctx, base);
	// 830DC998: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DC99C: 41800008  blt 0x830dc9a4
	if ctx.cr[0].lt {
	pc = 0x830DC9A4; continue 'dispatch;
	}
	// 830DC9A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DC9A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DC9A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DC9AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DC9B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DC9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DC9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DC9B8 size=160
    let mut pc: u32 = 0x830DC9B8;
    'dispatch: loop {
        match pc {
            0x830DC9B8 => {
    //   block [0x830DC9B8..0x830DCA58)
	// 830DC9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DC9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DC9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DC9C4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 830DC9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DC9CC: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DC9D0: 3CC0830E  lis r6, -0x7cf2
	ctx.r[6].s64 = -2096234496;
	// 830DC9D4: 39297D10  addi r9, r9, 0x7d10
	ctx.r[9].s64 = ctx.r[9].s64 + 32016;
	// 830DC9D8: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 830DC9DC: 3CA0830D  lis r5, -0x7cf3
	ctx.r[5].s64 = -2096300032;
	// 830DC9E0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 830DC9E4: 3D008218  lis r8, -0x7de8
	ctx.r[8].s64 = -2112356352;
	// 830DC9E8: F96A0010  std r11, 0x10(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 830DC9EC: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DC9F0: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 830DC9F4: 39087B64  addi r8, r8, 0x7b64
	ctx.r[8].s64 = ctx.r[8].s64 + 31588;
	// 830DC9F8: F96A0020  std r11, 0x20(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 830DC9FC: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830DCA00: 38E7B3D0  addi r7, r7, -0x4c30
	ctx.r[7].s64 = ctx.r[7].s64 + -19504;
	// 830DCA04: 3946B460  addi r10, r6, -0x4ba0
	ctx.r[10].s64 = ctx.r[6].s64 + -19360;
	// 830DCA08: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 830DCA0C: 3925EE60  addi r9, r5, -0x11a0
	ctx.r[9].s64 = ctx.r[5].s64 + -4512;
	// 830DCA10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830DCA14: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 830DCA18: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830DCA1C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DCA20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DCA24: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DCA28: 3888C0A8  addi r4, r8, -0x3f58
	ctx.r[4].s64 = ctx.r[8].s64 + -16216;
	// 830DCA2C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830DCA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830DCA34: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830DCA38: 4BFFCA11  bl 0x830d9448
	ctx.lr = 0x830DCA3C;
	sub_830D9448(ctx, base);
	// 830DCA3C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DCA40: 41800008  blt 0x830dca48
	if ctx.cr[0].lt {
	pc = 0x830DCA48; continue 'dispatch;
	}
	// 830DCA44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCA48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DCA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCA58 size=176
    let mut pc: u32 = 0x830DCA58;
    'dispatch: loop {
        match pc {
            0x830DCA58 => {
    //   block [0x830DCA58..0x830DCB08)
	// 830DCA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCA64: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCA68: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DCA6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DCA70: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DCA74: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DCA78: 394A7D2C  addi r10, r10, 0x7d2c
	ctx.r[10].s64 = ctx.r[10].s64 + 32044;
	// 830DCA7C: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DCA80: 39297C70  addi r9, r9, 0x7c70
	ctx.r[9].s64 = ctx.r[9].s64 + 31856;
	// 830DCA84: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DCA88: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DCA8C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DCA90: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DCA94: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DCA98: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DCA9C: 4B6F273D  bl 0x827cf1d8
	ctx.lr = 0x830DCAA0;
	sub_827CF1D8(ctx, base);
	// 830DCAA0: 3D60830E  lis r11, -0x7cf2
	ctx.r[11].s64 = -2096234496;
	// 830DCAA4: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DCAA8: 3D40830D  lis r10, -0x7cf3
	ctx.r[10].s64 = -2096300032;
	// 830DCAAC: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DCAB0: 396BB4A0  addi r11, r11, -0x4b60
	ctx.r[11].s64 = ctx.r[11].s64 + -19296;
	// 830DCAB4: 394A2E40  addi r10, r10, 0x2e40
	ctx.r[10].s64 = ctx.r[10].s64 + 11840;
	// 830DCAB8: 3929EE68  addi r9, r9, -0x1198
	ctx.r[9].s64 = ctx.r[9].s64 + -4504;
	// 830DCABC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DCAC0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DCAC4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DCAC8: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DCACC: 4BFFB1F5  bl 0x830d7cc0
	ctx.lr = 0x830DCAD0;
	sub_830D7CC0(ctx, base);
	// 830DCAD0: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DCAD4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DCAD8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DCADC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DCAE0: 388BC0AC  addi r4, r11, -0x3f54
	ctx.r[4].s64 = ctx.r[11].s64 + -16212;
	// 830DCAE4: 4BFFC965  bl 0x830d9448
	ctx.lr = 0x830DCAE8;
	sub_830D9448(ctx, base);
	// 830DCAE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DCAEC: 41800008  blt 0x830dcaf4
	if ctx.cr[0].lt {
	pc = 0x830DCAF4; continue 'dispatch;
	}
	// 830DCAF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCAF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DCAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCB08 size=172
    let mut pc: u32 = 0x830DCB08;
    'dispatch: loop {
        match pc {
            0x830DCB08 => {
    //   block [0x830DCB08..0x830DCBB4)
	// 830DCB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCB10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCB14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCB18: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DCB1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DCB20: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DCB24: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DCB28: 394A7EBC  addi r10, r10, 0x7ebc
	ctx.r[10].s64 = ctx.r[10].s64 + 32444;
	// 830DCB2C: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DCB30: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DCB34: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DCB38: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DCB3C: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DCB40: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DCB44: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DCB48: 39297B64  addi r9, r9, 0x7b64
	ctx.r[9].s64 = ctx.r[9].s64 + 31588;
	// 830DCB4C: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DCB50: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DCB54: 3908B518  addi r8, r8, -0x4ae8
	ctx.r[8].s64 = ctx.r[8].s64 + -19176;
	// 830DCB58: 396731E8  addi r11, r7, 0x31e8
	ctx.r[11].s64 = ctx.r[7].s64 + 12776;
	// 830DCB5C: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DCB60: 3946EEE8  addi r10, r6, -0x1118
	ctx.r[10].s64 = ctx.r[6].s64 + -4376;
	// 830DCB64: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DCB68: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DCB6C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DCB70: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DCB74: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DCB78: 4BFFC349  bl 0x830d8ec0
	ctx.lr = 0x830DCB7C;
	sub_830D8EC0(ctx, base);
	// 830DCB7C: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DCB80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DCB84: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DCB88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DCB8C: 388BC0E4  addi r4, r11, -0x3f1c
	ctx.r[4].s64 = ctx.r[11].s64 + -16156;
	// 830DCB90: 4BFFC8B9  bl 0x830d9448
	ctx.lr = 0x830DCB94;
	sub_830D9448(ctx, base);
	// 830DCB94: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DCB98: 41800008  blt 0x830dcba0
	if ctx.cr[0].lt {
	pc = 0x830DCBA0; continue 'dispatch;
	}
	// 830DCB9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCBA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DCBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCBAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCBB8 size=172
    let mut pc: u32 = 0x830DCBB8;
    'dispatch: loop {
        match pc {
            0x830DCBB8 => {
    //   block [0x830DCBB8..0x830DCC64)
	// 830DCBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCBC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCBC4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCBC8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DCBCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DCBD0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DCBD4: 3CE0830D  lis r7, -0x7cf3
	ctx.r[7].s64 = -2096300032;
	// 830DCBD8: 394A7F84  addi r10, r10, 0x7f84
	ctx.r[10].s64 = ctx.r[10].s64 + 32644;
	// 830DCBDC: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DCBE0: 3CC0827D  lis r6, -0x7d83
	ctx.r[6].s64 = -2105737216;
	// 830DCBE4: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DCBE8: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DCBEC: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DCBF0: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DCBF4: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DCBF8: 39297EBC  addi r9, r9, 0x7ebc
	ctx.r[9].s64 = ctx.r[9].s64 + 32444;
	// 830DCBFC: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DCC00: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DCC04: 3908B580  addi r8, r8, -0x4a80
	ctx.r[8].s64 = ctx.r[8].s64 + -19072;
	// 830DCC08: 396768E0  addi r11, r7, 0x68e0
	ctx.r[11].s64 = ctx.r[7].s64 + 26848;
	// 830DCC0C: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DCC10: 3946F1D8  addi r10, r6, -0xe28
	ctx.r[10].s64 = ctx.r[6].s64 + -3624;
	// 830DCC14: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DCC18: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DCC1C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DCC20: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DCC24: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DCC28: 4BFF4CF9  bl 0x830d1920
	ctx.lr = 0x830DCC2C;
	sub_830D1920(ctx, base);
	// 830DCC2C: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DCC30: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DCC34: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DCC38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DCC3C: 388BC0FC  addi r4, r11, -0x3f04
	ctx.r[4].s64 = ctx.r[11].s64 + -16132;
	// 830DCC40: 4BFFC809  bl 0x830d9448
	ctx.lr = 0x830DCC44;
	sub_830D9448(ctx, base);
	// 830DCC44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DCC48: 41800008  blt 0x830dcc50
	if ctx.cr[0].lt {
	pc = 0x830DCC50; continue 'dispatch;
	}
	// 830DCC4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCC50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DCC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCC5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCC68 size=120
    let mut pc: u32 = 0x830DCC68;
    'dispatch: loop {
        match pc {
            0x830DCC68 => {
    //   block [0x830DCC68..0x830DCCE0)
	// 830DCC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCC70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DCC74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCC78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCC7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DCC80: 386000D4  li r3, 0xd4
	ctx.r[3].s64 = 212;
	// 830DCC84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DCC88: 48000829  bl 0x830dd4b0
	ctx.lr = 0x830DCC8C;
	sub_830DD4B0(ctx, base);
	// 830DCC8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DCC90: 41820010  beq 0x830dcca0
	if ctx.cr[0].eq {
	pc = 0x830DCCA0; continue 'dispatch;
	}
	// 830DCC94: 4BFFA5BD  bl 0x830d7250
	ctx.lr = 0x830DCC98;
	sub_830D7250(ctx, base);
	// 830DCC98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830DCC9C: 48000008  b 0x830dcca4
	pc = 0x830DCCA4; continue 'dispatch;
	// 830DCCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DCCA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DCCA8: 409A0010  bne cr6, 0x830dccb8
	if !ctx.cr[6].eq {
	pc = 0x830DCCB8; continue 'dispatch;
	}
	// 830DCCAC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DCCB0: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DCCB4: 48000014  b 0x830dccc8
	pc = 0x830DCCC8; continue 'dispatch;
	// 830DCCB8: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DCCBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCCC0: 93EB0018  stw r31, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 830DCCC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DCCC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DCCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCCD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DCCD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCCE0 size=64
    let mut pc: u32 = 0x830DCCE0;
    'dispatch: loop {
        match pc {
            0x830DCCE0 => {
    //   block [0x830DCCE0..0x830DCD20)
	// 830DCCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCCEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCCF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DCCF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DCCF8: 419A0010  beq cr6, 0x830dcd08
	if ctx.cr[6].eq {
	pc = 0x830DCD08; continue 'dispatch;
	}
	// 830DCCFC: 4BFFA5E5  bl 0x830d72e0
	ctx.lr = 0x830DCD00;
	sub_830D72E0(ctx, base);
	// 830DCD00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DCD04: 480007D5  bl 0x830dd4d8
	ctx.lr = 0x830DCD08;
	sub_830DD4D8(ctx, base);
	// 830DCD08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCD0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DCD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCD20 size=172
    let mut pc: u32 = 0x830DCD20;
    'dispatch: loop {
        match pc {
            0x830DCD20 => {
    //   block [0x830DCD20..0x830DCDCC)
	// 830DCD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCD28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCD2C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCD30: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DCD34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DCD38: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DCD3C: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DCD40: 394A7BD4  addi r10, r10, 0x7bd4
	ctx.r[10].s64 = ctx.r[10].s64 + 31700;
	// 830DCD44: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DCD48: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DCD4C: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DCD50: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DCD54: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DCD58: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DCD5C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DCD60: 39297B64  addi r9, r9, 0x7b64
	ctx.r[9].s64 = ctx.r[9].s64 + 31588;
	// 830DCD64: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DCD68: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DCD6C: 3908CC68  addi r8, r8, -0x3398
	ctx.r[8].s64 = ctx.r[8].s64 + -13208;
	// 830DCD70: 3967CCE0  addi r11, r7, -0x3320
	ctx.r[11].s64 = ctx.r[7].s64 + -13088;
	// 830DCD74: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DCD78: 3946EDE8  addi r10, r6, -0x1218
	ctx.r[10].s64 = ctx.r[6].s64 + -4632;
	// 830DCD7C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DCD80: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DCD84: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DCD88: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DCD8C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DCD90: 4BFF7649  bl 0x830d43d8
	ctx.lr = 0x830DCD94;
	sub_830D43D8(ctx, base);
	// 830DCD94: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DCD98: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DCD9C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DCDA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DCDA4: 388BC074  addi r4, r11, -0x3f8c
	ctx.r[4].s64 = ctx.r[11].s64 + -16268;
	// 830DCDA8: 4BFFC6A1  bl 0x830d9448
	ctx.lr = 0x830DCDAC;
	sub_830D9448(ctx, base);
	// 830DCDAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DCDB0: 41800008  blt 0x830dcdb8
	if ctx.cr[0].lt {
	pc = 0x830DCDB8; continue 'dispatch;
	}
	// 830DCDB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCDB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DCDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCDC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCDD0 size=116
    let mut pc: u32 = 0x830DCDD0;
    'dispatch: loop {
        match pc {
            0x830DCDD0 => {
    //   block [0x830DCDD0..0x830DCE44)
	// 830DCDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCDD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DCDDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCDE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCDE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DCDE8: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830DCDEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DCDF0: 480006C1  bl 0x830dd4b0
	ctx.lr = 0x830DCDF4;
	sub_830DD4B0(ctx, base);
	// 830DCDF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DCDF8: 41820010  beq 0x830dce08
	if ctx.cr[0].eq {
	pc = 0x830DCE08; continue 'dispatch;
	}
	// 830DCDFC: 4BFF77BD  bl 0x830d45b8
	ctx.lr = 0x830DCE00;
	sub_830D45B8(ctx, base);
	// 830DCE00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830DCE04: 48000008  b 0x830dce0c
	pc = 0x830DCE0C; continue 'dispatch;
	// 830DCE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DCE0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DCE10: 409A0010  bne cr6, 0x830dce20
	if !ctx.cr[6].eq {
	pc = 0x830DCE20; continue 'dispatch;
	}
	// 830DCE14: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DCE18: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DCE1C: 48000010  b 0x830dce2c
	pc = 0x830DCE2C; continue 'dispatch;
	// 830DCE20: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DCE24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCE28: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DCE2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DCE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCE38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DCE3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCE48 size=64
    let mut pc: u32 = 0x830DCE48;
    'dispatch: loop {
        match pc {
            0x830DCE48 => {
    //   block [0x830DCE48..0x830DCE88)
	// 830DCE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCE50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCE54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCE58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DCE5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DCE60: 419A0010  beq cr6, 0x830dce70
	if ctx.cr[6].eq {
	pc = 0x830DCE70; continue 'dispatch;
	}
	// 830DCE64: 4BFFF2F5  bl 0x830dc158
	ctx.lr = 0x830DCE68;
	sub_830DC158(ctx, base);
	// 830DCE68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DCE6C: 4800066D  bl 0x830dd4d8
	ctx.lr = 0x830DCE70;
	sub_830DD4D8(ctx, base);
	// 830DCE70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCE74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DCE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCE80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCE88 size=160
    let mut pc: u32 = 0x830DCE88;
    'dispatch: loop {
        match pc {
            0x830DCE88 => {
    //   block [0x830DCE88..0x830DCF28)
	// 830DCE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCE8C: 480CB2E1  bl 0x831a816c
	ctx.lr = 0x830DCE90;
	sub_831A8130(ctx, base);
	// 830DCE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCE94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DCE98: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DCE9C: 4800063D  bl 0x830dd4d8
	ctx.lr = 0x830DCEA0;
	sub_830DD4D8(ctx, base);
	// 830DCEA0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DCEA4: 48000635  bl 0x830dd4d8
	ctx.lr = 0x830DCEA8;
	sub_830DD4D8(ctx, base);
	// 830DCEA8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830DCEAC: 4800062D  bl 0x830dd4d8
	ctx.lr = 0x830DCEB0;
	sub_830DD4D8(ctx, base);
	// 830DCEB0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830DCEB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830DCEB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DCEBC: 40990044  ble cr6, 0x830dcf00
	if !ctx.cr[6].gt {
	pc = 0x830DCF00; continue 'dispatch;
	}
	// 830DCEC0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830DCEC4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830DCEC8: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830DCECC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830DCED0: 419A001C  beq cr6, 0x830dceec
	if ctx.cr[6].eq {
	pc = 0x830DCEEC; continue 'dispatch;
	}
	// 830DCED4: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DCED8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DCEDC: 480005FD  bl 0x830dd4d8
	ctx.lr = 0x830DCEE0;
	sub_830DD4D8(ctx, base);
	// 830DCEE0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830DCEE4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830DCEE8: 480005F1  bl 0x830dd4d8
	ctx.lr = 0x830DCEEC;
	sub_830DD4D8(ctx, base);
	// 830DCEEC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830DCEF0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830DCEF4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830DCEF8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DCEFC: 4198FFC8  blt cr6, 0x830dcec4
	if ctx.cr[6].lt {
	pc = 0x830DCEC4; continue 'dispatch;
	}
	// 830DCF00: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 830DCF04: 48009A0D  bl 0x830e6910
	ctx.lr = 0x830DCF08;
	sub_830E6910(ctx, base);
	// 830DCF08: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 830DCF0C: 48009A05  bl 0x830e6910
	ctx.lr = 0x830DCF10;
	sub_830E6910(ctx, base);
	// 830DCF10: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 830DCF14: 4BFFE6E5  bl 0x830db5f8
	ctx.lr = 0x830DCF18;
	sub_830DB5F8(ctx, base);
	// 830DCF18: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 830DCF1C: 4BFFE6DD  bl 0x830db5f8
	ctx.lr = 0x830DCF20;
	sub_830DB5F8(ctx, base);
	// 830DCF20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DCF24: 480CB298  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCF28 size=176
    let mut pc: u32 = 0x830DCF28;
    'dispatch: loop {
        match pc {
            0x830DCF28 => {
    //   block [0x830DCF28..0x830DCFD8)
	// 830DCF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCF34: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCF38: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DCF3C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DCF40: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DCF44: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DCF48: 394A7BFC  addi r10, r10, 0x7bfc
	ctx.r[10].s64 = ctx.r[10].s64 + 31740;
	// 830DCF4C: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DCF50: 39297BA0  addi r9, r9, 0x7ba0
	ctx.r[9].s64 = ctx.r[9].s64 + 31648;
	// 830DCF54: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DCF58: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DCF5C: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DCF60: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DCF64: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DCF68: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DCF6C: 4B6F226D  bl 0x827cf1d8
	ctx.lr = 0x830DCF70;
	sub_827CF1D8(ctx, base);
	// 830DCF70: 3D60830E  lis r11, -0x7cf2
	ctx.r[11].s64 = -2096234496;
	// 830DCF74: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830DCF78: 3D40830E  lis r10, -0x7cf2
	ctx.r[10].s64 = -2096234496;
	// 830DCF7C: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830DCF80: 396BCDD0  addi r11, r11, -0x3230
	ctx.r[11].s64 = ctx.r[11].s64 + -12848;
	// 830DCF84: 394ACE48  addi r10, r10, -0x31b8
	ctx.r[10].s64 = ctx.r[10].s64 + -12728;
	// 830DCF88: 3929EE00  addi r9, r9, -0x1200
	ctx.r[9].s64 = ctx.r[9].s64 + -4608;
	// 830DCF8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830DCF90: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DCF94: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 830DCF98: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 830DCF9C: 4BFFA3FD  bl 0x830d7398
	ctx.lr = 0x830DCFA0;
	sub_830D7398(ctx, base);
	// 830DCFA0: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DCFA4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DCFA8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DCFAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DCFB0: 388BC080  addi r4, r11, -0x3f80
	ctx.r[4].s64 = ctx.r[11].s64 + -16256;
	// 830DCFB4: 4BFFC495  bl 0x830d9448
	ctx.lr = 0x830DCFB8;
	sub_830D9448(ctx, base);
	// 830DCFB8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DCFBC: 41800008  blt 0x830dcfc4
	if ctx.cr[0].lt {
	pc = 0x830DCFC4; continue 'dispatch;
	}
	// 830DCFC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DCFC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DCFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DCFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DCFD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DCFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DCFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DCFD8 size=176
    let mut pc: u32 = 0x830DCFD8;
    'dispatch: loop {
        match pc {
            0x830DCFD8 => {
    //   block [0x830DCFD8..0x830DD088)
	// 830DCFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DCFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DCFE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DCFE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DCFE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DCFEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DCFF0: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 830DCFF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DCFF8: 480004B9  bl 0x830dd4b0
	ctx.lr = 0x830DCFFC;
	sub_830DD4B0(ctx, base);
	// 830DCFFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DD000: 4182004C  beq 0x830dd04c
	if ctx.cr[0].eq {
	pc = 0x830DD04C; continue 'dispatch;
	}
	// 830DD004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DD008: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830DD00C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830DD010: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830DD014: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830DD018: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830DD01C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830DD020: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830DD024: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830DD028: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830DD02C: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830DD030: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830DD034: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830DD038: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830DD03C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DD040: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DD044: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830DD048: 48000008  b 0x830dd050
	pc = 0x830DD050; continue 'dispatch;
	// 830DD04C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DD050: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830DD054: 409A0010  bne cr6, 0x830dd064
	if !ctx.cr[6].eq {
	pc = 0x830DD064; continue 'dispatch;
	}
	// 830DD058: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD05C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DD060: 48000010  b 0x830dd070
	pc = 0x830DD070; continue 'dispatch;
	// 830DD064: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DD068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD06C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830DD070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DD074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DD078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DD07C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DD080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DD084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD088 size=64
    let mut pc: u32 = 0x830DD088;
    'dispatch: loop {
        match pc {
            0x830DD088 => {
    //   block [0x830DD088..0x830DD0C8)
	// 830DD088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DD090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DD094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DD09C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD0A0: 419A0010  beq cr6, 0x830dd0b0
	if ctx.cr[6].eq {
	pc = 0x830DD0B0; continue 'dispatch;
	}
	// 830DD0A4: 4BFFFDE5  bl 0x830dce88
	ctx.lr = 0x830DD0A8;
	sub_830DCE88(ctx, base);
	// 830DD0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD0AC: 4800042D  bl 0x830dd4d8
	ctx.lr = 0x830DD0B0;
	sub_830DD4D8(ctx, base);
	// 830DD0B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DD0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DD0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DD0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DD0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD0C8 size=172
    let mut pc: u32 = 0x830DD0C8;
    'dispatch: loop {
        match pc {
            0x830DD0C8 => {
    //   block [0x830DD0C8..0x830DD174)
	// 830DD0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DD0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DD0D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD0D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DD0DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DD0E0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830DD0E4: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DD0E8: 394A7D8C  addi r10, r10, 0x7d8c
	ctx.r[10].s64 = ctx.r[10].s64 + 32140;
	// 830DD0EC: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 830DD0F0: 3CC0830D  lis r6, -0x7cf3
	ctx.r[6].s64 = -2096300032;
	// 830DD0F4: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 830DD0F8: 3D208218  lis r9, -0x7de8
	ctx.r[9].s64 = -2112356352;
	// 830DD0FC: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 830DD100: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DD104: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 830DD108: 39297D7C  addi r9, r9, 0x7d7c
	ctx.r[9].s64 = ctx.r[9].s64 + 32124;
	// 830DD10C: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 830DD110: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DD114: 3908CFD8  addi r8, r8, -0x3028
	ctx.r[8].s64 = ctx.r[8].s64 + -12328;
	// 830DD118: 3967D088  addi r11, r7, -0x2f78
	ctx.r[11].s64 = ctx.r[7].s64 + -12152;
	// 830DD11C: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DD120: 3946EE88  addi r10, r6, -0x1178
	ctx.r[10].s64 = ctx.r[6].s64 + -4472;
	// 830DD124: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 830DD128: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830DD12C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 830DD130: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DD134: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DD138: 4BFFAF59  bl 0x830d8090
	ctx.lr = 0x830DD13C;
	sub_830D8090(ctx, base);
	// 830DD13C: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830DD140: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DD144: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 830DD148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DD14C: 388BC0BC  addi r4, r11, -0x3f44
	ctx.r[4].s64 = ctx.r[11].s64 + -16196;
	// 830DD150: 4BFFC2F9  bl 0x830d9448
	ctx.lr = 0x830DD154;
	sub_830D9448(ctx, base);
	// 830DD154: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DD158: 41800008  blt 0x830dd160
	if ctx.cr[0].lt {
	pc = 0x830DD160; continue 'dispatch;
	}
	// 830DD15C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD160: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DD164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DD168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DD16C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DD170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD178 size=824
    let mut pc: u32 = 0x830DD178;
    'dispatch: loop {
        match pc {
            0x830DD178 => {
    //   block [0x830DD178..0x830DD4B0)
	// 830DD178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD17C: 480CAFC5  bl 0x831a8140
	ctx.lr = 0x830DD180;
	sub_831A8130(ctx, base);
	// 830DD180: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DD188: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD18C: 419A001C  beq cr6, 0x830dd1a8
	if ctx.cr[6].eq {
	pc = 0x830DD1A8; continue 'dispatch;
	}
	// 830DD190: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD194: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 830DD198: 40990010  ble cr6, 0x830dd1a8
	if !ctx.cr[6].gt {
	pc = 0x830DD1A8; continue 'dispatch;
	}
	// 830DD19C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830DD1A0: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830DD1A4: 48000304  b 0x830dd4a8
	pc = 0x830DD4A8; continue 'dispatch;
	// 830DD1A8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830DD1AC: 817EC184  lwz r11, -0x3e7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-15996 as u32) ) } as u64;
	// 830DD1B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DD1B4: 419A000C  beq cr6, 0x830dd1c0
	if ctx.cr[6].eq {
	pc = 0x830DD1C0; continue 'dispatch;
	}
	// 830DD1B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DD1BC: 480002EC  b 0x830dd4a8
	pc = 0x830DD4A8; continue 'dispatch;
	// 830DD1C0: 48165B0D  bl 0x83242ccc
	ctx.lr = 0x830DD1C4;
	// extern call 0x83242CCC → crate::xboxkrnl::KeTlsAlloc
	crate::xboxkrnl::KeTlsAlloc(ctx, base);
	// 830DD1C4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830DD1C8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830DD1CC: 906B56F8  stw r3, 0x56f8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22264 as u32), ctx.r[3].u32 ) };
	// 830DD1D0: 409A0010  bne cr6, 0x830dd1e0
	if !ctx.cr[6].eq {
	pc = 0x830DD1E0; continue 'dispatch;
	}
	// 830DD1D4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD1D8: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DD1DC: 480002CC  b 0x830dd4a8
	pc = 0x830DD4A8; continue 'dispatch;
	// 830DD1E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD1E4: 419A0030  beq cr6, 0x830dd214
	if ctx.cr[6].eq {
	pc = 0x830DD214; continue 'dispatch;
	}
	// 830DD1E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD1EC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DD1F0: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 830DD1F4: 4198000C  blt cr6, 0x830dd200
	if ctx.cr[6].lt {
	pc = 0x830DD200; continue 'dispatch;
	}
	// 830DD1F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DD1FC: 48000008  b 0x830dd204
	pc = 0x830DD204; continue 'dispatch;
	// 830DD200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DD204: 916AC18C  stw r11, -0x3e74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15988 as u32), ctx.r[11].u32 ) };
	// 830DD208: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DD20C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DD210: 916AC180  stw r11, -0x3e80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16000 as u32), ctx.r[11].u32 ) };
	// 830DD214: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830DD218: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DD21C: 917EC184  stw r11, -0x3e7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-15996 as u32), ctx.r[11].u32 ) };
	// 830DD220: 386AC1B0  addi r3, r10, -0x3e50
	ctx.r[3].s64 = ctx.r[10].s64 + -15952;
	// 830DD224: 480264AD  bl 0x831036d0
	ctx.lr = 0x830DD228;
	sub_831036D0(ctx, base);
	// 830DD228: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DD22C: 41800274  blt 0x830dd4a0
	if ctx.cr[0].lt {
	pc = 0x830DD4A0; continue 'dispatch;
	}
	// 830DD230: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DD234: 386BC190  addi r3, r11, -0x3e70
	ctx.r[3].s64 = ctx.r[11].s64 + -15984;
	// 830DD238: 4BFF0E81  bl 0x830ce0b8
	ctx.lr = 0x830DD23C;
	sub_830CE0B8(ctx, base);
	// 830DD23C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DD240: 41800260  blt 0x830dd4a0
	if ctx.cr[0].lt {
	pc = 0x830DD4A0; continue 'dispatch;
	}
	// 830DD244: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DD248: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 830DD24C: 386BC19C  addi r3, r11, -0x3e64
	ctx.r[3].s64 = ctx.r[11].s64 + -15972;
	// 830DD250: 38800300  li r4, 0x300
	ctx.r[4].s64 = 768;
	// 830DD254: 4BFF2105  bl 0x830cf358
	ctx.lr = 0x830DD258;
	sub_830CF358(ctx, base);
	// 830DD258: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DD25C: 41800244  blt 0x830dd4a0
	if ctx.cr[0].lt {
	pc = 0x830DD4A0; continue 'dispatch;
	}
	// 830DD260: 3D60830E  lis r11, -0x7cf2
	ctx.r[11].s64 = -2096234496;
	// 830DD264: 3D40830E  lis r10, -0x7cf2
	ctx.r[10].s64 = -2096234496;
	// 830DD268: 396BC868  addi r11, r11, -0x3798
	ctx.r[11].s64 = ctx.r[11].s64 + -14232;
	// 830DD26C: 394A9A50  addi r10, r10, -0x65b0
	ctx.r[10].s64 = ctx.r[10].s64 + -26032;
	// 830DD270: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DD274: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DD278: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DD27C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830DD280: 3D20830E  lis r9, -0x7cf2
	ctx.r[9].s64 = -2096234496;
	// 830DD284: 3968C908  addi r11, r8, -0x36f8
	ctx.r[11].s64 = ctx.r[8].s64 + -14072;
	// 830DD288: 3929CD20  addi r9, r9, -0x32e0
	ctx.r[9].s64 = ctx.r[9].s64 + -13024;
	// 830DD28C: 3947CF28  addi r10, r7, -0x30d8
	ctx.r[10].s64 = ctx.r[7].s64 + -12504;
	// 830DD290: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830DD294: 3CC0830E  lis r6, -0x7cf2
	ctx.r[6].s64 = -2096234496;
	// 830DD298: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 830DD29C: 3CA0830E  lis r5, -0x7cf2
	ctx.r[5].s64 = -2096234496;
	// 830DD2A0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 830DD2A4: 3C80830E  lis r4, -0x7cf2
	ctx.r[4].s64 = -2096234496;
	// 830DD2A8: 39269D60  addi r9, r6, -0x62a0
	ctx.r[9].s64 = ctx.r[6].s64 + -25248;
	// 830DD2AC: 39659EE0  addi r11, r5, -0x6120
	ctx.r[11].s64 = ctx.r[5].s64 + -24864;
	// 830DD2B0: 39449E20  addi r10, r4, -0x61e0
	ctx.r[10].s64 = ctx.r[4].s64 + -25056;
	// 830DD2B4: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 830DD2B8: 3C60830E  lis r3, -0x7cf2
	ctx.r[3].s64 = -2096234496;
	// 830DD2BC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830DD2C0: 3FE0830E  lis r31, -0x7cf2
	ctx.r[31].s64 = -2096234496;
	// 830DD2C4: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 830DD2C8: 3FC0830E  lis r30, -0x7cf2
	ctx.r[30].s64 = -2096234496;
	// 830DD2CC: 39239F90  addi r9, r3, -0x6070
	ctx.r[9].s64 = ctx.r[3].s64 + -24688;
	// 830DD2D0: 397FA050  addi r11, r31, -0x5fb0
	ctx.r[11].s64 = ctx.r[31].s64 + -24496;
	// 830DD2D4: 395EA110  addi r10, r30, -0x5ef0
	ctx.r[10].s64 = ctx.r[30].s64 + -24304;
	// 830DD2D8: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 830DD2DC: 3FA0830E  lis r29, -0x7cf2
	ctx.r[29].s64 = -2096234496;
	// 830DD2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830DD2E4: 3F80830E  lis r28, -0x7cf2
	ctx.r[28].s64 = -2096234496;
	// 830DD2E8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 830DD2EC: 3F60830E  lis r27, -0x7cf2
	ctx.r[27].s64 = -2096234496;
	// 830DD2F0: 393D9B00  addi r9, r29, -0x6500
	ctx.r[9].s64 = ctx.r[29].s64 + -25856;
	// 830DD2F4: 397CA1C0  addi r11, r28, -0x5e40
	ctx.r[11].s64 = ctx.r[28].s64 + -24128;
	// 830DD2F8: 395B9CA0  addi r10, r27, -0x6360
	ctx.r[10].s64 = ctx.r[27].s64 + -25440;
	// 830DD2FC: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 830DD300: 3F40830E  lis r26, -0x7cf2
	ctx.r[26].s64 = -2096234496;
	// 830DD304: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830DD308: 3F20830E  lis r25, -0x7cf2
	ctx.r[25].s64 = -2096234496;
	// 830DD30C: 91410084  stw r10, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830DD310: 3F00830E  lis r24, -0x7cf2
	ctx.r[24].s64 = -2096234496;
	// 830DD314: 393AA280  addi r9, r26, -0x5d80
	ctx.r[9].s64 = ctx.r[26].s64 + -23936;
	// 830DD318: 3979A330  addi r11, r25, -0x5cd0
	ctx.r[11].s64 = ctx.r[25].s64 + -23760;
	// 830DD31C: 3958C9B8  addi r10, r24, -0x3648
	ctx.r[10].s64 = ctx.r[24].s64 + -13896;
	// 830DD320: 91210088  stw r9, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 830DD324: 3EE0830E  lis r23, -0x7cf2
	ctx.r[23].s64 = -2096234496;
	// 830DD328: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 830DD32C: 3EC0830E  lis r22, -0x7cf2
	ctx.r[22].s64 = -2096234496;
	// 830DD330: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 830DD334: 3EA0830E  lis r21, -0x7cf2
	ctx.r[21].s64 = -2096234496;
	// 830DD338: 3937CA58  addi r9, r23, -0x35a8
	ctx.r[9].s64 = ctx.r[23].s64 + -13736;
	// 830DD33C: 3976A410  addi r11, r22, -0x5bf0
	ctx.r[11].s64 = ctx.r[22].s64 + -23536;
	// 830DD340: 3955A4D0  addi r10, r21, -0x5b30
	ctx.r[10].s64 = ctx.r[21].s64 + -23344;
	// 830DD344: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 830DD348: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 830DD34C: 3D20830E  lis r9, -0x7cf2
	ctx.r[9].s64 = -2096234496;
	// 830DD350: 9141009C  stw r10, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 830DD354: 3D60830E  lis r11, -0x7cf2
	ctx.r[11].s64 = -2096234496;
	// 830DD358: 3D40830E  lis r10, -0x7cf2
	ctx.r[10].s64 = -2096234496;
	// 830DD35C: 3D00830E  lis r8, -0x7cf2
	ctx.r[8].s64 = -2096234496;
	// 830DD360: 3CE0830E  lis r7, -0x7cf2
	ctx.r[7].s64 = -2096234496;
	// 830DD364: 3CC0830E  lis r6, -0x7cf2
	ctx.r[6].s64 = -2096234496;
	// 830DD368: 3CA0830E  lis r5, -0x7cf2
	ctx.r[5].s64 = -2096234496;
	// 830DD36C: 3C80830E  lis r4, -0x7cf2
	ctx.r[4].s64 = -2096234496;
	// 830DD370: 3C60830E  lis r3, -0x7cf2
	ctx.r[3].s64 = -2096234496;
	// 830DD374: 3FE0830E  lis r31, -0x7cf2
	ctx.r[31].s64 = -2096234496;
	// 830DD378: 3FC0830E  lis r30, -0x7cf2
	ctx.r[30].s64 = -2096234496;
	// 830DD37C: 3FA0830E  lis r29, -0x7cf2
	ctx.r[29].s64 = -2096234496;
	// 830DD380: 3F80830E  lis r28, -0x7cf2
	ctx.r[28].s64 = -2096234496;
	// 830DD384: 3F60830E  lis r27, -0x7cf2
	ctx.r[27].s64 = -2096234496;
	// 830DD388: 3F40830E  lis r26, -0x7cf2
	ctx.r[26].s64 = -2096234496;
	// 830DD38C: 3F20830E  lis r25, -0x7cf2
	ctx.r[25].s64 = -2096234496;
	// 830DD390: 3F00830E  lis r24, -0x7cf2
	ctx.r[24].s64 = -2096234496;
	// 830DD394: 3EE0830E  lis r23, -0x7cf2
	ctx.r[23].s64 = -2096234496;
	// 830DD398: 3EC0830E  lis r22, -0x7cf2
	ctx.r[22].s64 = -2096234496;
	// 830DD39C: 3EA0830E  lis r21, -0x7cf2
	ctx.r[21].s64 = -2096234496;
	// 830DD3A0: 3E80830E  lis r20, -0x7cf2
	ctx.r[20].s64 = -2096234496;
	// 830DD3A4: 3929A590  addi r9, r9, -0x5a70
	ctx.r[9].s64 = ctx.r[9].s64 + -23152;
	// 830DD3A8: 396BD0C8  addi r11, r11, -0x2f38
	ctx.r[11].s64 = ctx.r[11].s64 + -12088;
	// 830DD3AC: 912100A0  stw r9, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 830DD3B0: 394AA7D0  addi r10, r10, -0x5830
	ctx.r[10].s64 = ctx.r[10].s64 + -22576;
	// 830DD3B4: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 830DD3B8: 3928A890  addi r9, r8, -0x5770
	ctx.r[9].s64 = ctx.r[8].s64 + -22384;
	// 830DD3BC: 3967AA10  addi r11, r7, -0x55f0
	ctx.r[11].s64 = ctx.r[7].s64 + -22000;
	// 830DD3C0: 914100A8  stw r10, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 830DD3C4: 912100AC  stw r9, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[9].u32 ) };
	// 830DD3C8: 3946A950  addi r10, r6, -0x56b0
	ctx.r[10].s64 = ctx.r[6].s64 + -22192;
	// 830DD3CC: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 830DD3D0: 3925AB80  addi r9, r5, -0x5480
	ctx.r[9].s64 = ctx.r[5].s64 + -21632;
	// 830DD3D4: 3964AC40  addi r11, r4, -0x53c0
	ctx.r[11].s64 = ctx.r[4].s64 + -21440;
	// 830DD3D8: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 830DD3DC: 912100B8  stw r9, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[9].u32 ) };
	// 830DD3E0: 3943AAC0  addi r10, r3, -0x5540
	ctx.r[10].s64 = ctx.r[3].s64 + -21824;
	// 830DD3E4: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 830DD3E8: 397EAD00  addi r11, r30, -0x5300
	ctx.r[11].s64 = ctx.r[30].s64 + -21248;
	// 830DD3EC: 393F9BC0  addi r9, r31, -0x6440
	ctx.r[9].s64 = ctx.r[31].s64 + -25664;
	// 830DD3F0: 914100C0  stw r10, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 830DD3F4: 916100C8  stw r11, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 830DD3F8: 395DADC0  addi r10, r29, -0x5240
	ctx.r[10].s64 = ctx.r[29].s64 + -21056;
	// 830DD3FC: 912100C4  stw r9, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[9].u32 ) };
	// 830DD400: 393CA660  addi r9, r28, -0x59a0
	ctx.r[9].s64 = ctx.r[28].s64 + -22944;
	// 830DD404: 397BA720  addi r11, r27, -0x58e0
	ctx.r[11].s64 = ctx.r[27].s64 + -22752;
	// 830DD408: 914100CC  stw r10, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 830DD40C: 912100D0  stw r9, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[9].u32 ) };
	// 830DD410: 395ACB08  addi r10, r26, -0x34f8
	ctx.r[10].s64 = ctx.r[26].s64 + -13560;
	// 830DD414: 916100D4  stw r11, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 830DD418: 3939AE90  addi r9, r25, -0x5170
	ctx.r[9].s64 = ctx.r[25].s64 + -20848;
	// 830DD41C: 3978AF50  addi r11, r24, -0x50b0
	ctx.r[11].s64 = ctx.r[24].s64 + -20656;
	// 830DD420: 914100D8  stw r10, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[10].u32 ) };
	// 830DD424: 912100DC  stw r9, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[9].u32 ) };
	// 830DD428: 3957B010  addi r10, r23, -0x4ff0
	ctx.r[10].s64 = ctx.r[23].s64 + -20464;
	// 830DD42C: 916100E0  stw r11, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 830DD430: 3936B0D0  addi r9, r22, -0x4f30
	ctx.r[9].s64 = ctx.r[22].s64 + -20272;
	// 830DD434: 3975B190  addi r11, r21, -0x4e70
	ctx.r[11].s64 = ctx.r[21].s64 + -20080;
	// 830DD438: 914100E4  stw r10, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 830DD43C: 3E60830E  lis r19, -0x7cf2
	ctx.r[19].s64 = -2096234496;
	// 830DD440: 912100E8  stw r9, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[9].u32 ) };
	// 830DD444: 3E40830E  lis r18, -0x7cf2
	ctx.r[18].s64 = -2096234496;
	// 830DD448: 916100EC  stw r11, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 830DD44C: 3954CBB8  addi r10, r20, -0x3448
	ctx.r[10].s64 = ctx.r[20].s64 + -13384;
	// 830DD450: 3933B250  addi r9, r19, -0x4db0
	ctx.r[9].s64 = ctx.r[19].s64 + -19888;
	// 830DD454: 3972B310  addi r11, r18, -0x4cf0
	ctx.r[11].s64 = ctx.r[18].s64 + -19696;
	// 830DD458: 914100F0  stw r10, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[10].u32 ) };
	// 830DD45C: 912100F4  stw r9, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[9].u32 ) };
	// 830DD460: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830DD464: 916100F8  stw r11, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 830DD468: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 830DD46C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD474: 4E800421  bctrl
	ctx.lr = 0x830DD478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DD478: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DD47C: 41800024  blt 0x830dd4a0
	if ctx.cr[0].lt {
	pc = 0x830DD4A0; continue 'dispatch;
	}
	// 830DD480: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830DD484: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830DD488: 2B1D002B  cmplwi cr6, r29, 0x2b
	ctx.cr[6].compare_u32(ctx.r[29].u32, 43 as u32, &mut ctx.xer);
	// 830DD48C: 4198FFE0  blt cr6, 0x830dd46c
	if ctx.cr[6].lt {
	pc = 0x830DD46C; continue 'dispatch;
	}
	// 830DD490: 48022981  bl 0x830ffe10
	ctx.lr = 0x830DD494;
	sub_830FFE10(ctx, base);
	// 830DD494: 48001495  bl 0x830de928
	ctx.lr = 0x830DD498;
	sub_830DE928(ctx, base);
	// 830DD498: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DD49C: 40800008  bge 0x830dd4a4
	if !ctx.cr[0].lt {
	pc = 0x830DD4A4; continue 'dispatch;
	}
	// 830DD4A0: 4BFFEEF9  bl 0x830dc398
	ctx.lr = 0x830DD4A4;
	sub_830DC398(ctx, base);
	// 830DD4A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD4A8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 830DD4AC: 480CACE4  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4B0 size=8
    let mut pc: u32 = 0x830DD4B0;
    'dispatch: loop {
        match pc {
            0x830DD4B0 => {
    //   block [0x830DD4B0..0x830DD4B8)
	// 830DD4B0: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD4B4: 480E6AA4  b 0x831c3f58
	sub_831C3F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4B8 size=16
    let mut pc: u32 = 0x830DD4B8;
    'dispatch: loop {
        match pc {
            0x830DD4B8 => {
    //   block [0x830DD4B8..0x830DD4C8)
	// 830DD4B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830DD4BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD4C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DD4C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4C8 size=12
    let mut pc: u32 = 0x830DD4C8;
    'dispatch: loop {
        match pc {
            0x830DD4C8 => {
    //   block [0x830DD4C8..0x830DD4D4)
	// 830DD4C8: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD4CC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830DD4D0: 480E6B40  b 0x831c4010
	sub_831C4010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4D4 size=4
    let mut pc: u32 = 0x830DD4D4;
    'dispatch: loop {
        match pc {
            0x830DD4D4 => {
    //   block [0x830DD4D4..0x830DD4D8)
	// 830DD4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4D8 size=8
    let mut pc: u32 = 0x830DD4D8;
    'dispatch: loop {
        match pc {
            0x830DD4D8 => {
    //   block [0x830DD4D8..0x830DD4E0)
	// 830DD4D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD4DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4E0 size=8
    let mut pc: u32 = 0x830DD4E0;
    'dispatch: loop {
        match pc {
            0x830DD4E0 => {
    //   block [0x830DD4E0..0x830DD4E8)
	// 830DD4E0: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD4E4: 480E6B0C  b 0x831c3ff0
	sub_831C3FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD4E8 size=4
    let mut pc: u32 = 0x830DD4E8;
    'dispatch: loop {
        match pc {
            0x830DD4E8 => {
    //   block [0x830DD4E8..0x830DD4EC)
	// 830DD4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD4F0 size=144
    let mut pc: u32 = 0x830DD4F0;
    'dispatch: loop {
        match pc {
            0x830DD4F0 => {
    //   block [0x830DD4F0..0x830DD580)
	// 830DD4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD4F4: 480CAC79  bl 0x831a816c
	ctx.lr = 0x830DD4F8;
	sub_831A8130(ctx, base);
	// 830DD4F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DD500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DD504: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD508: 419A001C  beq cr6, 0x830dd524
	if ctx.cr[6].eq {
	pc = 0x830DD524; continue 'dispatch;
	}
	// 830DD50C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DD510: 409A0014  bne cr6, 0x830dd524
	if !ctx.cr[6].eq {
	pc = 0x830DD524; continue 'dispatch;
	}
	// 830DD514: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD518: 480E6AD9  bl 0x831c3ff0
	ctx.lr = 0x830DD51C;
	sub_831C3FF0(ctx, base);
	// 830DD51C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD520: 48000058  b 0x830dd578
	pc = 0x830DD578; continue 'dispatch;
	// 830DD524: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DD52C: 480E6A2D  bl 0x831c3f58
	ctx.lr = 0x830DD530;
	sub_831C3F58(ctx, base);
	// 830DD530: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830DD534: 4182FFE8  beq 0x830dd51c
	if ctx.cr[0].eq {
	pc = 0x830DD51C; continue 'dispatch;
	}
	// 830DD538: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD53C: 419A0038  beq cr6, 0x830dd574
	if ctx.cr[6].eq {
	pc = 0x830DD574; continue 'dispatch;
	}
	// 830DD540: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD548: 480E6AC9  bl 0x831c4010
	ctx.lr = 0x830DD54C;
	sub_831C4010(ctx, base);
	// 830DD54C: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830DD550: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830DD554: 41980008  blt cr6, 0x830dd55c
	if ctx.cr[6].lt {
	pc = 0x830DD55C; continue 'dispatch;
	}
	// 830DD558: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830DD55C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DD560: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DD564: 480CAFAD  bl 0x831a8510
	ctx.lr = 0x830DD568;
	sub_831A8510(ctx, base);
	// 830DD568: 3C80248E  lis r4, 0x248e
	ctx.r[4].s64 = 613285888;
	// 830DD56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD570: 480E6A81  bl 0x831c3ff0
	ctx.lr = 0x830DD574;
	sub_831C3FF0(ctx, base);
	// 830DD574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DD578: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DD57C: 480CAC40  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD580 size=168
    let mut pc: u32 = 0x830DD580;
    'dispatch: loop {
        match pc {
            0x830DD580 => {
    //   block [0x830DD580..0x830DD628)
	// 830DD580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD584: 480CABD9  bl 0x831a815c
	ctx.lr = 0x830DD588;
	sub_831A8130(ctx, base);
	// 830DD588: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD58C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DD590: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830DD594: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830DD598: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830DD59C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DD5A0: 409A0010  bne cr6, 0x830dd5b0
	if !ctx.cr[6].eq {
	pc = 0x830DD5B0; continue 'dispatch;
	}
	// 830DD5A4: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DD5A8: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 830DD5AC: 48000074  b 0x830dd620
	pc = 0x830DD620; continue 'dispatch;
	// 830DD5B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DD5B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DD5B8: 3BAB8B58  addi r29, r11, -0x74a8
	ctx.r[29].s64 = ctx.r[11].s64 + -29864;
	// 830DD5BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830DD5C0: 480D4AF1  bl 0x831b20b0
	ctx.lr = 0x830DD5C4;
	sub_831B20B0(ctx, base);
	// 830DD5C4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DD5C8: 4182FFDC  beq 0x830dd5a4
	if ctx.cr[0].eq {
	pc = 0x830DD5A4; continue 'dispatch;
	}
	// 830DD5CC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830DD5D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DD5D4: B39F0000  sth r28, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 830DD5D8: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830DD5DC: 480CBAED  bl 0x831a90c8
	ctx.lr = 0x830DD5E0;
	sub_831A90C8(ctx, base);
	// 830DD5E0: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DD5E4: 38800023  li r4, 0x23
	ctx.r[4].s64 = 35;
	// 830DD5E8: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830DD5EC: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DD5F0: 480D4889  bl 0x831b1e78
	ctx.lr = 0x830DD5F4;
	sub_831B1E78(ctx, base);
	// 830DD5F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DD5F8: 41820020  beq 0x830dd618
	if ctx.cr[0].eq {
	pc = 0x830DD618; continue 'dispatch;
	}
	// 830DD5FC: A1430002  lhz r10, 2(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 830DD600: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830DD604: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DD608: 41820008  beq 0x830dd610
	if ctx.cr[0].eq {
	pc = 0x830DD610; continue 'dispatch;
	}
	// 830DD60C: B3830000  sth r28, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 830DD610: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DD614: 48000008  b 0x830dd61c
	pc = 0x830DD61C; continue 'dispatch;
	// 830DD618: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830DD61C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD620: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DD624: 480CAB88  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD628 size=252
    let mut pc: u32 = 0x830DD628;
    'dispatch: loop {
        match pc {
            0x830DD628 => {
    //   block [0x830DD628..0x830DD724)
	// 830DD628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD62C: 480CAB35  bl 0x831a8160
	ctx.lr = 0x830DD630;
	sub_831A8130(ctx, base);
	// 830DD630: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD634: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830DD638: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DD63C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DD640: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830DD644: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830DD648: 480CBA81  bl 0x831a90c8
	ctx.lr = 0x830DD64C;
	sub_831A90C8(ctx, base);
	// 830DD64C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DD650: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DD654: 480CBA75  bl 0x831a90c8
	ctx.lr = 0x830DD658;
	sub_831A90C8(ctx, base);
	// 830DD658: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DD65C: 7FFF1A14  add r31, r31, r3
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 830DD660: 3B4B8B58  addi r26, r11, -0x74a8
	ctx.r[26].s64 = ctx.r[11].s64 + -29864;
	// 830DD664: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830DD668: 480CBA61  bl 0x831a90c8
	ctx.lr = 0x830DD66C;
	sub_831A90C8(ctx, base);
	// 830DD66C: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830DD670: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830DD674: 419A0014  beq cr6, 0x830dd688
	if ctx.cr[6].eq {
	pc = 0x830DD688; continue 'dispatch;
	}
	// 830DD678: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DD67C: 480CBA4D  bl 0x831a90c8
	ctx.lr = 0x830DD680;
	sub_831A90C8(ctx, base);
	// 830DD680: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830DD684: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 830DD688: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830DD68C: 57E3083C  slwi r3, r31, 1
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830DD690: 4BFFFE21  bl 0x830dd4b0
	ctx.lr = 0x830DD694;
	sub_830DD4B0(ctx, base);
	// 830DD694: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DD698: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DD69C: 40820010  bne 0x830dd6ac
	if !ctx.cr[0].eq {
	pc = 0x830DD6AC; continue 'dispatch;
	}
	// 830DD6A0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD6A4: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DD6A8: 48000074  b 0x830dd71c
	pc = 0x830DD71C; continue 'dispatch;
	// 830DD6AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830DD6B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DD6B4: 480D4805  bl 0x831b1eb8
	ctx.lr = 0x830DD6B8;
	sub_831B1EB8(ctx, base);
	// 830DD6B8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830DD6BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DD6C0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD6C4: 480D4A65  bl 0x831b2128
	ctx.lr = 0x830DD6C8;
	sub_831B2128(ctx, base);
	// 830DD6C8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830DD6CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DD6D0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD6D4: 480D4A55  bl 0x831b2128
	ctx.lr = 0x830DD6D8;
	sub_831B2128(ctx, base);
	// 830DD6D8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830DD6DC: 419A003C  beq cr6, 0x830dd718
	if ctx.cr[6].eq {
	pc = 0x830DD718; continue 'dispatch;
	}
	// 830DD6E0: 38800023  li r4, 0x23
	ctx.r[4].s64 = 35;
	// 830DD6E4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD6E8: 480D4791  bl 0x831b1e78
	ctx.lr = 0x830DD6EC;
	sub_831B1E78(ctx, base);
	// 830DD6EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DD6F0: 40820018  bne 0x830dd708
	if !ctx.cr[0].eq {
	pc = 0x830DD708; continue 'dispatch;
	}
	// 830DD6F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DD6F8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD6FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DD700: 38AB8B60  addi r5, r11, -0x74a0
	ctx.r[5].s64 = ctx.r[11].s64 + -29856;
	// 830DD704: 480D4A25  bl 0x831b2128
	ctx.lr = 0x830DD708;
	sub_831B2128(ctx, base);
	// 830DD708: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830DD70C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD710: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DD714: 480D4A15  bl 0x831b2128
	ctx.lr = 0x830DD718;
	sub_831B2128(ctx, base);
	// 830DD718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD71C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830DD720: 480CAA90  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD728 size=156
    let mut pc: u32 = 0x830DD728;
    'dispatch: loop {
        match pc {
            0x830DD728 => {
    //   block [0x830DD728..0x830DD7C4)
	// 830DD728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD72C: 480CAA41  bl 0x831a816c
	ctx.lr = 0x830DD730;
	sub_831A8130(ctx, base);
	// 830DD730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD734: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DD738: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DD73C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DD740: 419A0074  beq cr6, 0x830dd7b4
	if ctx.cr[6].eq {
	pc = 0x830DD7B4; continue 'dispatch;
	}
	// 830DD744: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD748: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DD74C: 41820068  beq 0x830dd7b4
	if ctx.cr[0].eq {
	pc = 0x830DD7B4; continue 'dispatch;
	}
	// 830DD750: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD754: 419A0060  beq cr6, 0x830dd7b4
	if ctx.cr[6].eq {
	pc = 0x830DD7B4; continue 'dispatch;
	}
	// 830DD758: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DD75C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830DD760: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DD764: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830DD768: 806BC18C  lwz r3, -0x3e74(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15988 as u32) ) } as u64;
	// 830DD76C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD770: 419A0038  beq cr6, 0x830dd7a8
	if ctx.cr[6].eq {
	pc = 0x830DD7A8; continue 'dispatch;
	}
	// 830DD774: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD778: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830DD77C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD780: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD784: 4E800421  bctrl
	ctx.lr = 0x830DD788;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DD788: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DD78C: 40800030  bge 0x830dd7bc
	if !ctx.cr[0].lt {
	pc = 0x830DD7BC; continue 'dispatch;
	}
	// 830DD790: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 830DD794: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830DD798: 616B4001  ori r11, r11, 0x4001
	ctx.r[11].u64 = ctx.r[11].u64 | 16385;
	// 830DD79C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DD7A0: 409A001C  bne cr6, 0x830dd7bc
	if !ctx.cr[6].eq {
	pc = 0x830DD7BC; continue 'dispatch;
	}
	// 830DD7A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DD7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD7AC: 4BFF14DD  bl 0x830cec88
	ctx.lr = 0x830DD7B0;
	sub_830CEC88(ctx, base);
	// 830DD7B0: 4800000C  b 0x830dd7bc
	pc = 0x830DD7BC; continue 'dispatch;
	// 830DD7B4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD7B8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DD7BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DD7C0: 480CA9FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD7C8 size=92
    let mut pc: u32 = 0x830DD7C8;
    'dispatch: loop {
        match pc {
            0x830DD7C8 => {
    //   block [0x830DD7C8..0x830DD824)
	// 830DD7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD7CC: 480CA9A1  bl 0x831a816c
	ctx.lr = 0x830DD7D0;
	sub_831A8130(ctx, base);
	// 830DD7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD7D4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830DD7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DD7DC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830DD7E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DD7E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830DD7E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830DD7EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DD7F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DD7F4: 4BFFFE35  bl 0x830dd628
	ctx.lr = 0x830DD7F8;
	sub_830DD628(ctx, base);
	// 830DD7F8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DD7FC: 4180001C  blt 0x830dd818
	if ctx.cr[0].lt {
	pc = 0x830DD818; continue 'dispatch;
	}
	// 830DD800: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DD804: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DD808: 48027E89  bl 0x83105690
	ctx.lr = 0x830DD80C;
	sub_83105690(ctx, base);
	// 830DD80C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DD810: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DD814: 4BFFFCC5  bl 0x830dd4d8
	ctx.lr = 0x830DD818;
	sub_830DD4D8(ctx, base);
	// 830DD818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DD81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DD820: 480CA99C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD828 size=32
    let mut pc: u32 = 0x830DD828;
    'dispatch: loop {
        match pc {
            0x830DD828 => {
    //   block [0x830DD828..0x830DD848)
	// 830DD828: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD82C: 419A001C  beq cr6, 0x830dd848
	if ctx.cr[6].eq {
		sub_830DD848(ctx, base);
		return;
	}
	// 830DD830: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830DD834: 419A0014  beq cr6, 0x830dd848
	if ctx.cr[6].eq {
		sub_830DD848(ctx, base);
		return;
	}
	// 830DD838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD83C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830DD840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD844: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD848 size=12
    let mut pc: u32 = 0x830DD848;
    'dispatch: loop {
        match pc {
            0x830DD848 => {
    //   block [0x830DD848..0x830DD854)
	// 830DD848: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD84C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DD850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD858 size=20
    let mut pc: u32 = 0x830DD858;
    'dispatch: loop {
        match pc {
            0x830DD858 => {
    //   block [0x830DD858..0x830DD86C)
	// 830DD858: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD85C: 409A0010  bne cr6, 0x830dd86c
	if !ctx.cr[6].eq {
		sub_830DD86C(ctx, base);
		return;
	}
	// 830DD860: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD864: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DD868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD86C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD86C size=16
    let mut pc: u32 = 0x830DD86C;
    'dispatch: loop {
        match pc {
            0x830DD86C => {
    //   block [0x830DD86C..0x830DD87C)
	// 830DD86C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD870: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830DD874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD878: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD880 size=120
    let mut pc: u32 = 0x830DD880;
    'dispatch: loop {
        match pc {
            0x830DD880 => {
    //   block [0x830DD880..0x830DD8F8)
	// 830DD880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DD888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DD88C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD890: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830DD894: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD898: 419A0044  beq cr6, 0x830dd8dc
	if ctx.cr[6].eq {
	pc = 0x830DD8DC; continue 'dispatch;
	}
	// 830DD89C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD8A0: 419A003C  beq cr6, 0x830dd8dc
	if ctx.cr[6].eq {
	pc = 0x830DD8DC; continue 'dispatch;
	}
	// 830DD8A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD8A8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830DD8AC: 409A001C  bne cr6, 0x830dd8c8
	if !ctx.cr[6].eq {
	pc = 0x830DD8C8; continue 'dispatch;
	}
	// 830DD8B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830DD8B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD8B8: 4E800421  bctrl
	ctx.lr = 0x830DD8BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DD8BC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DD8C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DD8C4: 48000020  b 0x830dd8e4
	pc = 0x830DD8E4; continue 'dispatch;
	// 830DD8C8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830DD8CC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830DD8D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD8D4: 4E800421  bctrl
	ctx.lr = 0x830DD8D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DD8D8: 4800000C  b 0x830dd8e4
	pc = 0x830DD8E4; continue 'dispatch;
	// 830DD8DC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD8E0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DD8E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DD8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DD8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DD8F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DD8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD8F8 size=12
    let mut pc: u32 = 0x830DD8F8;
    'dispatch: loop {
        match pc {
            0x830DD8F8 => {
    //   block [0x830DD8F8..0x830DD904)
	// 830DD8F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD8FC: 409A0008  bne cr6, 0x830dd904
	if !ctx.cr[6].eq {
		sub_830DD904(ctx, base);
		return;
	}
	// 830DD900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD904 size=16
    let mut pc: u32 = 0x830DD904;
    'dispatch: loop {
        match pc {
            0x830DD904 => {
    //   block [0x830DD904..0x830DD914)
	// 830DD904: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD908: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830DD90C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD910: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD918 size=20
    let mut pc: u32 = 0x830DD918;
    'dispatch: loop {
        match pc {
            0x830DD918 => {
    //   block [0x830DD918..0x830DD92C)
	// 830DD918: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DD91C: 409A0010  bne cr6, 0x830dd92c
	if !ctx.cr[6].eq {
		sub_830DD92C(ctx, base);
		return;
	}
	// 830DD920: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DD924: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DD928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD92C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DD92C size=16
    let mut pc: u32 = 0x830DD92C;
    'dispatch: loop {
        match pc {
            0x830DD92C => {
    //   block [0x830DD92C..0x830DD93C)
	// 830DD92C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD930: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830DD934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD938: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD940 size=100
    let mut pc: u32 = 0x830DD940;
    'dispatch: loop {
        match pc {
            0x830DD940 => {
    //   block [0x830DD940..0x830DD9A4)
	// 830DD940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DD948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DD94C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DD950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DD958: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD95C: 419A0030  beq cr6, 0x830dd98c
	if ctx.cr[6].eq {
	pc = 0x830DD98C; continue 'dispatch;
	}
	// 830DD960: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DD964: 3BCBD784  addi r30, r11, -0x287c
	ctx.r[30].s64 = ctx.r[11].s64 + -10364;
	// 830DD968: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DD96C: 48165001  bl 0x8324296c
	ctx.lr = 0x830DD970;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830DD970: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD978: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DD97C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD980: 4E800421  bctrl
	ctx.lr = 0x830DD984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DD984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DD988: 48164FD5  bl 0x8324295c
	ctx.lr = 0x830DD98C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830DD98C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DD990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DD994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DD998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DD99C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DD9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DD9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DD9A8 size=200
    let mut pc: u32 = 0x830DD9A8;
    'dispatch: loop {
        match pc {
            0x830DD9A8 => {
    //   block [0x830DD9A8..0x830DDA70)
	// 830DD9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DD9AC: 480CA7B9  bl 0x831a8164
	ctx.lr = 0x830DD9B0;
	sub_831A8130(ctx, base);
	// 830DD9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DD9B4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830DD9B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830DD9BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DD9C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DD9C4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DD9C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DD9CC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DD9D0: 409A0010  bne cr6, 0x830dd9e0
	if !ctx.cr[6].eq {
	pc = 0x830DD9E0; continue 'dispatch;
	}
	// 830DD9D4: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 830DD9D8: 63BD0057  ori r29, r29, 0x57
	ctx.r[29].u64 = ctx.r[29].u64 | 87;
	// 830DD9DC: 48000064  b 0x830dda40
	pc = 0x830DDA40; continue 'dispatch;
	// 830DD9E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DD9E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830DD9E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830DD9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DD9F0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830DD9F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DD9F8: 4E800421  bctrl
	ctx.lr = 0x830DD9FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DD9FC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830DDA00: 41800040  blt 0x830dda40
	if ctx.cr[0].lt {
	pc = 0x830DDA40; continue 'dispatch;
	}
	// 830DDA04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDA0C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830DDA10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DDA14: 4E800421  bctrl
	ctx.lr = 0x830DDA18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DDA18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DDA1C: 4BFFFA95  bl 0x830dd4b0
	ctx.lr = 0x830DDA20;
	sub_830DD4B0(ctx, base);
	// 830DDA20: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DDA24: 40820028  bne 0x830dda4c
	if !ctx.cr[0].eq {
	pc = 0x830DDA4C; continue 'dispatch;
	}
	// 830DDA28: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 830DDA2C: 63BD000E  ori r29, r29, 0xe
	ctx.r[29].u64 = ctx.r[29].u64 | 14;
	// 830DDA30: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DDA34: 419A000C  beq cr6, 0x830dda40
	if ctx.cr[6].eq {
	pc = 0x830DDA40; continue 'dispatch;
	}
	// 830DDA38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DDA3C: 4BFFFA9D  bl 0x830dd4d8
	ctx.lr = 0x830DDA40;
	sub_830DD4D8(ctx, base);
	// 830DDA40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDA44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DDA48: 480CA76C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830DDA4C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830DDA50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830DDA54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DDA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDA5C: 4BFFFE25  bl 0x830dd880
	ctx.lr = 0x830DDA60;
	sub_830DD880(ctx, base);
	// 830DDA60: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830DDA64: 4180FFCC  blt 0x830dda30
	if ctx.cr[0].lt {
	pc = 0x830DDA30; continue 'dispatch;
	}
	// 830DDA68: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830DDA6C: 4BFFFFD4  b 0x830dda40
	pc = 0x830DDA40; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDA70 size=60
    let mut pc: u32 = 0x830DDA70;
    'dispatch: loop {
        match pc {
            0x830DDA70 => {
    //   block [0x830DDA70..0x830DDAAC)
	// 830DDA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DDA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDA7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DDA80: 419A001C  beq cr6, 0x830dda9c
	if ctx.cr[6].eq {
	pc = 0x830DDA9C; continue 'dispatch;
	}
	// 830DDA84: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DDA88: 388B8B58  addi r4, r11, -0x74a8
	ctx.r[4].s64 = ctx.r[11].s64 + -29864;
	// 830DDA8C: 480D4625  bl 0x831b20b0
	ctx.lr = 0x830DDA90;
	sub_831B20B0(ctx, base);
	// 830DDA90: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830DDA94: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830DDA98: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830DDA9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DDAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DDAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DDAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDAB0 size=108
    let mut pc: u32 = 0x830DDAB0;
    'dispatch: loop {
        match pc {
            0x830DDAB0 => {
    //   block [0x830DDAB0..0x830DDB1C)
	// 830DDAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DDAB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DDABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDAC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DDAC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DDAC8: 419A003C  beq cr6, 0x830ddb04
	if ctx.cr[6].eq {
	pc = 0x830DDB04; continue 'dispatch;
	}
	// 830DDACC: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDAD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DDAD4: 41820030  beq 0x830ddb04
	if ctx.cr[0].eq {
	pc = 0x830DDB04; continue 'dispatch;
	}
	// 830DDAD8: 480CB5F1  bl 0x831a90c8
	ctx.lr = 0x830DDADC;
	sub_831A90C8(ctx, base);
	// 830DDADC: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DDAE0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830DDAE4: A16BFFFE  lhz r11, -2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 830DDAE8: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 830DDAEC: 419A0018  beq cr6, 0x830ddb04
	if ctx.cr[6].eq {
	pc = 0x830DDB04; continue 'dispatch;
	}
	// 830DDAF0: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 830DDAF4: 419A0010  beq cr6, 0x830ddb04
	if ctx.cr[6].eq {
	pc = 0x830DDB04; continue 'dispatch;
	}
	// 830DDAF8: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 830DDAFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DDB00: 409A0008  bne cr6, 0x830ddb08
	if !ctx.cr[6].eq {
	pc = 0x830DDB08; continue 'dispatch;
	}
	// 830DDB04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DDB08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DDB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DDB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DDB14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DDB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDB20 size=576
    let mut pc: u32 = 0x830DDB20;
    'dispatch: loop {
        match pc {
            0x830DDB20 => {
    //   block [0x830DDB20..0x830DDD60)
	// 830DDB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDB24: 480CA631  bl 0x831a8154
	ctx.lr = 0x830DDB28;
	sub_831A8130(ctx, base);
	// 830DDB28: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDB2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DDB30: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 830DDB34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830DDB38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DDB3C: 419A0214  beq cr6, 0x830ddd50
	if ctx.cr[6].eq {
	pc = 0x830DDD50; continue 'dispatch;
	}
	// 830DDB40: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830DDB44: 419A020C  beq cr6, 0x830ddd50
	if ctx.cr[6].eq {
	pc = 0x830DDD50; continue 'dispatch;
	}
	// 830DDB48: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830DDB4C: 419A0204  beq cr6, 0x830ddd50
	if ctx.cr[6].eq {
	pc = 0x830DDD50; continue 'dispatch;
	}
	// 830DDB50: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830DDB54: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DDB58: 93570000  stw r26, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 830DDB5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDB60: 388B8B58  addi r4, r11, -0x74a8
	ctx.r[4].s64 = ctx.r[11].s64 + -29864;
	// 830DDB64: 480D454D  bl 0x831b20b0
	ctx.lr = 0x830DDB68;
	sub_831B20B0(ctx, base);
	// 830DDB68: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830DDB6C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830DDB70: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830DDB74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DDB78: 409A01CC  bne cr6, 0x830ddd44
	if !ctx.cr[6].eq {
	pc = 0x830DDD44; continue 'dispatch;
	}
	// 830DDB7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDB80: 4BFFFF31  bl 0x830ddab0
	ctx.lr = 0x830DDB84;
	sub_830DDAB0(ctx, base);
	// 830DDB84: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DDB88: 418201BC  beq 0x830ddd44
	if ctx.cr[0].eq {
	pc = 0x830DDD44; continue 'dispatch;
	}
	// 830DDB8C: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 830DDB90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DDB94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DDB98: 4BFF07D1  bl 0x830ce368
	ctx.lr = 0x830DDB9C;
	sub_830CE368(ctx, base);
	// 830DDB9C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DDBA0: 418001B8  blt 0x830ddd58
	if ctx.cr[0].lt {
	pc = 0x830DDD58; continue 'dispatch;
	}
	// 830DDBA4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 830DDBA8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DDBAC: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 830DDBB0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DDBB4: 4BFFF9CD  bl 0x830dd580
	ctx.lr = 0x830DDBB8;
	sub_830DD580(ctx, base);
	// 830DDBB8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DDBBC: 41800178  blt 0x830ddd34
	if ctx.cr[0].lt {
	pc = 0x830DDD34; continue 'dispatch;
	}
	// 830DDBC0: 83210058  lwz r25, 0x58(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830DDBC4: 8301005C  lwz r24, 0x5c(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830DDBC8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830DDBCC: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 830DDBD0: 409A0008  bne cr6, 0x830ddbd8
	if !ctx.cr[6].eq {
	pc = 0x830DDBD8; continue 'dispatch;
	}
	// 830DDBD4: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 830DDBD8: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDBDC: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 830DDBE0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830DDBE4: 48000038  b 0x830ddc1c
	pc = 0x830DDC1C; continue 'dispatch;
	// 830DDBE8: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 830DDBEC: 409A0038  bne cr6, 0x830ddc24
	if !ctx.cr[6].eq {
	pc = 0x830DDC24; continue 'dispatch;
	}
	// 830DDBF0: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830DDBF4: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 830DDBF8: 409A002C  bne cr6, 0x830ddc24
	if !ctx.cr[6].eq {
	pc = 0x830DDC24; continue 'dispatch;
	}
	// 830DDBFC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DDC00: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 830DDC04: 419A000C  beq cr6, 0x830ddc10
	if ctx.cr[6].eq {
	pc = 0x830DDC10; continue 'dispatch;
	}
	// 830DDC08: 2B0A005C  cmplwi cr6, r10, 0x5c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 92 as u32, &mut ctx.xer);
	// 830DDC0C: 409A0018  bne cr6, 0x830ddc24
	if !ctx.cr[6].eq {
	pc = 0x830DDC24; continue 'dispatch;
	}
	// 830DDC10: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 830DDC14: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830DDC18: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDC1C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DDC20: 4082FFC8  bne 0x830ddbe8
	if !ctx.cr[0].eq {
	pc = 0x830DDBE8; continue 'dispatch;
	}
	// 830DDC24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DDC28: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 830DDC2C: 480CB49D  bl 0x831a90c8
	ctx.lr = 0x830DDC30;
	sub_831A90C8(ctx, base);
	// 830DDC30: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DDC34: 355F0001  addic. r10, r31, 1
	ctx.xer.ca = (ctx.r[31].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DDC38: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830DDC3C: 4081004C  ble 0x830ddc88
	if !ctx.cr[0].gt {
	pc = 0x830DDC88; continue 'dispatch;
	}
	// 830DDC40: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830DDC44: 41980030  blt cr6, 0x830ddc74
	if ctx.cr[6].lt {
	pc = 0x830DDC74; continue 'dispatch;
	}
	// 830DDC48: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDC4C: 2B09002F  cmplwi cr6, r9, 0x2f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 47 as u32, &mut ctx.xer);
	// 830DDC50: 419A0018  beq cr6, 0x830ddc68
	if ctx.cr[6].eq {
	pc = 0x830DDC68; continue 'dispatch;
	}
	// 830DDC54: 2B09005C  cmplwi cr6, r9, 0x5c
	ctx.cr[6].compare_u32(ctx.r[9].u32, 92 as u32, &mut ctx.xer);
	// 830DDC58: 419A0010  beq cr6, 0x830ddc68
	if ctx.cr[6].eq {
	pc = 0x830DDC68; continue 'dispatch;
	}
	// 830DDC5C: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 830DDC60: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830DDC64: 4098FFE4  bge cr6, 0x830ddc48
	if !ctx.cr[6].lt {
	pc = 0x830DDC48; continue 'dispatch;
	}
	// 830DDC68: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DDC6C: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 830DDC70: 4181FFD0  bgt 0x830ddc40
	if ctx.cr[0].gt {
	pc = 0x830DDC40; continue 'dispatch;
	}
	// 830DDC74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DDC78: 40990010  ble cr6, 0x830ddc88
	if !ctx.cr[6].gt {
	pc = 0x830DDC88; continue 'dispatch;
	}
	// 830DDC7C: 3FC08030  lis r30, -0x7fd0
	ctx.r[30].s64 = -2144337920;
	// 830DDC80: 63DE0012  ori r30, r30, 0x12
	ctx.r[30].u64 = ctx.r[30].u64 | 18;
	// 830DDC84: 480000B0  b 0x830ddd34
	pc = 0x830DDD34; continue 'dispatch;
	// 830DDC88: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830DDC8C: 41980010  blt cr6, 0x830ddc9c
	if ctx.cr[6].lt {
	pc = 0x830DDC9C; continue 'dispatch;
	}
	// 830DDC90: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDC94: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DDC98: 41820008  beq 0x830ddca0
	if ctx.cr[0].eq {
	pc = 0x830DDCA0; continue 'dispatch;
	}
	// 830DDC9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830DDCA0: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 830DDCA4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DDCA8: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830DDCAC: 480CB41D  bl 0x831a90c8
	ctx.lr = 0x830DDCB0;
	sub_831A90C8(ctx, base);
	// 830DDCB0: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 830DDCB4: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 830DDCB8: 57A3083C  slwi r3, r29, 1
	ctx.r[3].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830DDCBC: 4BFF0585  bl 0x830ce240
	ctx.lr = 0x830DDCC0;
	sub_830CE240(ctx, base);
	// 830DDCC0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DDCC4: 40820010  bne 0x830ddcd4
	if !ctx.cr[0].eq {
	pc = 0x830DDCD4; continue 'dispatch;
	}
	// 830DDCC8: 3FC08007  lis r30, -0x7ff9
	ctx.r[30].s64 = -2147024896;
	// 830DDCCC: 63DE000E  ori r30, r30, 0xe
	ctx.r[30].u64 = ctx.r[30].u64 | 14;
	// 830DDCD0: 48000064  b 0x830ddd34
	pc = 0x830DDD34; continue 'dispatch;
	// 830DDCD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830DDCD8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830DDCDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830DDCE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDCE4: 480D4555  bl 0x831b2238
	ctx.lr = 0x830DDCE8;
	sub_831B2238(ctx, base);
	// 830DDCE8: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DDCEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830DDCF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830DDCF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDCF8: 7F4BFB2E  sthx r26, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[26].u16) };
	// 830DDCFC: 480D442D  bl 0x831b2128
	ctx.lr = 0x830DDD00;
	sub_831B2128(ctx, base);
	// 830DDD00: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DDD04: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830DDD08: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830DDD0C: 419A0010  beq cr6, 0x830ddd1c
	if ctx.cr[6].eq {
	pc = 0x830DDD1C; continue 'dispatch;
	}
	// 830DDD10: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830DDD14: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830DDD18: 4800000C  b 0x830ddd24
	pc = 0x830DDD24; continue 'dispatch;
	// 830DDD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830DDD20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830DDD24: 4BFFF905  bl 0x830dd628
	ctx.lr = 0x830DDD28;
	sub_830DD628(ctx, base);
	// 830DDD28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DDD2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDD30: 4BFF05A9  bl 0x830ce2d8
	ctx.lr = 0x830DDD34;
	sub_830CE2D8(ctx, base);
	// 830DDD34: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DDD38: 4BFF05A1  bl 0x830ce2d8
	ctx.lr = 0x830DDD3C;
	sub_830CE2D8(ctx, base);
	// 830DDD3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DDD40: 48000018  b 0x830ddd58
	pc = 0x830DDD58; continue 'dispatch;
	// 830DDD44: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DDD48: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 830DDD4C: 4800000C  b 0x830ddd58
	pc = 0x830DDD58; continue 'dispatch;
	// 830DDD50: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DDD54: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DDD58: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830DDD5C: 480CA448  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDD60 size=104
    let mut pc: u32 = 0x830DDD60;
    'dispatch: loop {
        match pc {
            0x830DDD60 => {
    //   block [0x830DDD60..0x830DDDC8)
	// 830DDD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDD64: 480CA405  bl 0x831a8168
	ctx.lr = 0x830DDD68;
	sub_831A8130(ctx, base);
	// 830DDD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDD6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DDD70: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830DDD74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830DDD78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DDD7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DDD80: 40990034  ble cr6, 0x830dddb4
	if !ctx.cr[6].gt {
	pc = 0x830DDDB4; continue 'dispatch;
	}
	// 830DDD84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830DDD88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDD8C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DDD90: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830DDD94: 480D409D  bl 0x831b1e30
	ctx.lr = 0x830DDD98;
	sub_831B1E30(ctx, base);
	// 830DDD98: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DDD9C: 41820024  beq 0x830dddc0
	if ctx.cr[0].eq {
	pc = 0x830DDDC0; continue 'dispatch;
	}
	// 830DDDA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DDDA4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830DDDA8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 830DDDAC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DDDB0: 4198FFD8  blt cr6, 0x830ddd88
	if ctx.cr[6].lt {
	pc = 0x830DDD88; continue 'dispatch;
	}
	// 830DDDB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830DDDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DDDBC: 480CA3FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830DDDC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDDC4: 4BFFFFF4  b 0x830dddb8
	pc = 0x830DDDB8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDDC8 size=148
    let mut pc: u32 = 0x830DDDC8;
    'dispatch: loop {
        match pc {
            0x830DDDC8 => {
    //   block [0x830DDDC8..0x830DDE5C)
	// 830DDDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DDDD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DDDD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DDDD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDDDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DDDE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DDDE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DDDE8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DDDEC: 40990054  ble cr6, 0x830dde40
	if !ctx.cr[6].gt {
	pc = 0x830DDE40; continue 'dispatch;
	}
	// 830DDDF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DDDF4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830DDDF8: 40990008  ble cr6, 0x830dde00
	if !ctx.cr[6].gt {
	pc = 0x830DDE00; continue 'dispatch;
	}
	// 830DDDFC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830DDE00: 57C4083C  slwi r4, r30, 1
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830DDE04: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDE08: 4BFFF6E9  bl 0x830dd4f0
	ctx.lr = 0x830DDE0C;
	sub_830DD4F0(ctx, base);
	// 830DDE0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DDE10: 40820028  bne 0x830dde38
	if !ctx.cr[0].eq {
	pc = 0x830DDE38; continue 'dispatch;
	}
	// 830DDE14: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDE18: 4BFFF6C1  bl 0x830dd4d8
	ctx.lr = 0x830DDE1C;
	sub_830DD4D8(ctx, base);
	// 830DDE1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DDE20: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DDE24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DDE28: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DDE2C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830DDE30: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DDE34: 48000010  b 0x830dde44
	pc = 0x830DDE44; continue 'dispatch;
	// 830DDE38: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DDE3C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830DDE40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DDE44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DDE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DDE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DDE50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DDE54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DDE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDE60 size=160
    let mut pc: u32 = 0x830DDE60;
    'dispatch: loop {
        match pc {
            0x830DDE60 => {
    //   block [0x830DDE60..0x830DDF00)
	// 830DDE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDE64: 480CA301  bl 0x831a8164
	ctx.lr = 0x830DDE68;
	sub_831A8130(ctx, base);
	// 830DDE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDE6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DDE70: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830DDE74: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830DDE78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDE7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830DDE80: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DDE84: 48164FE9  bl 0x83242e6c
	ctx.lr = 0x830DDE88;
	// extern call 0x83242E6C → crate::xboxkrnl::_vscwprintf
	crate::xboxkrnl::_vscwprintf(ctx, base);
	// 830DDE88: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DDE8C: 41800068  blt 0x830ddef4
	if ctx.cr[0].lt {
	pc = 0x830DDEF4; continue 'dispatch;
	}
	// 830DDE90: 7F63FA14  add r27, r3, r31
	ctx.r[27].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830DDE94: 389B0001  addi r4, r27, 1
	ctx.r[4].s64 = ctx.r[27].s64 + 1;
	// 830DDE98: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830DDE9C: 40990058  ble cr6, 0x830ddef4
	if !ctx.cr[6].gt {
	pc = 0x830DDEF4; continue 'dispatch;
	}
	// 830DDEA0: 7F041840  cmplw cr6, r4, r3
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DDEA4: 40990050  ble cr6, 0x830ddef4
	if !ctx.cr[6].gt {
	pc = 0x830DDEF4; continue 'dispatch;
	}
	// 830DDEA8: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 830DDEAC: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 830DDEB0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830DDEB4: 40980040  bge cr6, 0x830ddef4
	if !ctx.cr[6].lt {
	pc = 0x830DDEF4; continue 'dispatch;
	}
	// 830DDEB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DDEBC: 4BFFFF0D  bl 0x830dddc8
	ctx.lr = 0x830DDEC0;
	sub_830DDDC8(ctx, base);
	// 830DDEC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DDEC4: 41800030  blt 0x830ddef4
	if ctx.cr[0].lt {
	pc = 0x830DDEF4; continue 'dispatch;
	}
	// 830DDEC8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDECC: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DDED0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830DDED4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830DDED8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830DDEDC: 48164F81  bl 0x83242e5c
	ctx.lr = 0x830DDEE0;
	// extern call 0x83242E5C → crate::xboxkrnl::vswprintf
	crate::xboxkrnl::vswprintf(ctx, base);
	// 830DDEE0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DDEE4: 41800010  blt 0x830ddef4
	if ctx.cr[0].lt {
	pc = 0x830DDEF4; continue 'dispatch;
	}
	// 830DDEE8: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 830DDEEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DDEF0: 48000008  b 0x830ddef8
	pc = 0x830DDEF8; continue 'dispatch;
	// 830DDEF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DDEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DDEFC: 480CA2B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDF00 size=148
    let mut pc: u32 = 0x830DDF00;
    'dispatch: loop {
        match pc {
            0x830DDF00 => {
    //   block [0x830DDF00..0x830DDF94)
	// 830DDF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDF04: 480CA269  bl 0x831a816c
	ctx.lr = 0x830DDF08;
	sub_831A8130(ctx, base);
	// 830DDF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDF0C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DDF10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DDF14: 3BABD784  addi r29, r11, -0x287c
	ctx.r[29].s64 = ctx.r[11].s64 + -10364;
	// 830DDF18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DDF1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDF20: 48164A4D  bl 0x8324296c
	ctx.lr = 0x830DDF24;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830DDF24: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DDF28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DDF2C: 83EBD7A4  lwz r31, -0x285c(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DDF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDF34: 4BFFFE2D  bl 0x830ddd60
	ctx.lr = 0x830DDF38;
	sub_830DDD60(ctx, base);
	// 830DDF38: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830DDF3C: 409A000C  bne cr6, 0x830ddf48
	if !ctx.cr[6].eq {
	pc = 0x830DDF48; continue 'dispatch;
	}
	// 830DDF40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DDF44: 48000010  b 0x830ddf54
	pc = 0x830DDF54; continue 'dispatch;
	// 830DDF48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DDF4C: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830DDF50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830DDF54: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DDF58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DDF5C: 409A0020  bne cr6, 0x830ddf7c
	if !ctx.cr[6].eq {
	pc = 0x830DDF7C; continue 'dispatch;
	}
	// 830DDF60: 3FE08030  lis r31, -0x7fd0
	ctx.r[31].s64 = -2144337920;
	// 830DDF64: 63FF001B  ori r31, r31, 0x1b
	ctx.r[31].u64 = ctx.r[31].u64 | 27;
	// 830DDF68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DDF6C: 481649F1  bl 0x8324295c
	ctx.lr = 0x830DDF70;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830DDF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DDF74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DDF78: 480CA244  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830DDF7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDF80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDF84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DDF88: 4E800421  bctrl
	ctx.lr = 0x830DDF8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DDF8C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DDF90: 4BFFFFD8  b 0x830ddf68
	pc = 0x830DDF68; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DDF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DDF98 size=136
    let mut pc: u32 = 0x830DDF98;
    'dispatch: loop {
        match pc {
            0x830DDF98 => {
    //   block [0x830DDF98..0x830DE020)
	// 830DDF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DDF9C: 480CA1CD  bl 0x831a8168
	ctx.lr = 0x830DDFA0;
	sub_831A8130(ctx, base);
	// 830DDFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DDFA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DDFA8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830DDFAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDFB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DDFB4: 419A0048  beq cr6, 0x830ddffc
	if ctx.cr[6].eq {
	pc = 0x830DDFFC; continue 'dispatch;
	}
	// 830DDFB8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DDFBC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830DDFC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DDFC4: 4099002C  ble cr6, 0x830ddff0
	if !ctx.cr[6].gt {
	pc = 0x830DDFF0; continue 'dispatch;
	}
	// 830DDFC8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830DDFCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDFD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DDFD4: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 830DDFD8: 4BFEF019  bl 0x830ccff0
	ctx.lr = 0x830DDFDC;
	sub_830CCFF0(ctx, base);
	// 830DDFDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DDFE0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830DDFE4: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 830DDFE8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DDFEC: 4198FFE0  blt cr6, 0x830ddfcc
	if ctx.cr[6].lt {
	pc = 0x830DDFCC; continue 'dispatch;
	}
	// 830DDFF0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DDFF4: 4BFFF4E5  bl 0x830dd4d8
	ctx.lr = 0x830DDFF8;
	sub_830DD4D8(ctx, base);
	// 830DDFF8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830DDFFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE000: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DE004: 419A000C  beq cr6, 0x830de010
	if ctx.cr[6].eq {
	pc = 0x830DE010; continue 'dispatch;
	}
	// 830DE008: 4BFFF4D1  bl 0x830dd4d8
	ctx.lr = 0x830DE00C;
	sub_830DD4D8(ctx, base);
	// 830DE00C: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830DE010: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830DE014: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830DE018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DE01C: 480CA19C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE020 size=72
    let mut pc: u32 = 0x830DE020;
    'dispatch: loop {
        match pc {
            0x830DE020 => {
    //   block [0x830DE020..0x830DE068)
	// 830DE020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DE028: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 830DE02C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 830DE030: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 830DE034: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 830DE038: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 830DE03C: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 830DE040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE044: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830DE048: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 830DE04C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830DE050: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE054: 4BFFFE0D  bl 0x830dde60
	ctx.lr = 0x830DE058;
	sub_830DDE60(ctx, base);
	// 830DE058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DE05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DE060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DE064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE068 size=124
    let mut pc: u32 = 0x830DE068;
    'dispatch: loop {
        match pc {
            0x830DE068 => {
    //   block [0x830DE068..0x830DE0E4)
	// 830DE068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE06C: 480CA0FD  bl 0x831a8168
	ctx.lr = 0x830DE070;
	sub_831A8130(ctx, base);
	// 830DE070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE074: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE078: 3B8BD784  addi r28, r11, -0x287c
	ctx.r[28].s64 = ctx.r[11].s64 + -10364;
	// 830DE07C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DE080: 481648ED  bl 0x8324296c
	ctx.lr = 0x830DE084;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830DE084: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 830DE088: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DE08C: 807DD7A4  lwz r3, -0x285c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DE090: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DE094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DE098: 40990038  ble cr6, 0x830de0d0
	if !ctx.cr[6].gt {
	pc = 0x830DE0D0; continue 'dispatch;
	}
	// 830DE09C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830DE0A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE0A4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830DE0A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE0AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE0B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE0B4: 4E800421  bctrl
	ctx.lr = 0x830DE0B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE0B8: 807DD7A4  lwz r3, -0x285c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DE0BC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830DE0C0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830DE0C4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DE0C8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830DE0CC: 4198FFD4  blt cr6, 0x830de0a0
	if ctx.cr[6].lt {
	pc = 0x830DE0A0; continue 'dispatch;
	}
	// 830DE0D0: 4BFFFEC9  bl 0x830ddf98
	ctx.lr = 0x830DE0D4;
	sub_830DDF98(ctx, base);
	// 830DE0D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830DE0D8: 48164885  bl 0x8324295c
	ctx.lr = 0x830DE0DC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830DE0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DE0E0: 480CA0D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE0E8 size=904
    let mut pc: u32 = 0x830DE0E8;
    'dispatch: loop {
        match pc {
            0x830DE0E8 => {
    //   block [0x830DE0E8..0x830DE470)
	// 830DE0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE0EC: 480CA061  bl 0x831a814c
	ctx.lr = 0x830DE0F0;
	sub_831A8130(ctx, base);
	// 830DE0F0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE0F4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE0F8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 830DE0FC: 3AABD784  addi r21, r11, -0x287c
	ctx.r[21].s64 = ctx.r[11].s64 + -10364;
	// 830DE100: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830DE104: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830DE108: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 830DE10C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830DE110: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830DE114: 48164859  bl 0x8324296c
	ctx.lr = 0x830DE118;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830DE118: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830DE11C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830DE120: 419A0008  beq cr6, 0x830de128
	if ctx.cr[6].eq {
	pc = 0x830DE128; continue 'dispatch;
	}
	// 830DE124: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830DE128: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830DE12C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830DE130: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830DE134: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830DE138: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830DE13C: 4BFFF68D  bl 0x830dd7c8
	ctx.lr = 0x830DE140;
	sub_830DD7C8(ctx, base);
	// 830DE140: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE144: 41800248  blt 0x830de38c
	if ctx.cr[0].lt {
	pc = 0x830DE38C; continue 'dispatch;
	}
	// 830DE148: 82C10054  lwz r22, 0x54(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE14C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 830DE150: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 830DE154: 419A0030  beq cr6, 0x830de184
	if ctx.cr[6].eq {
	pc = 0x830DE184; continue 'dispatch;
	}
	// 830DE158: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830DE15C: 409A0020  bne cr6, 0x830de17c
	if !ctx.cr[6].eq {
	pc = 0x830DE17C; continue 'dispatch;
	}
	// 830DE160: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE164: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830DE168: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE16C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE170: 4E800421  bctrl
	ctx.lr = 0x830DE174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE174: 7FB6EB78  mr r22, r29
	ctx.r[22].u64 = ctx.r[29].u64;
	// 830DE178: 92C10054  stw r22, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[22].u32 ) };
	// 830DE17C: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 830DE180: 409A0184  bne cr6, 0x830de304
	if !ctx.cr[6].eq {
	pc = 0x830DE304; continue 'dispatch;
	}
	// 830DE184: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DE188: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830DE18C: 4BFFFD75  bl 0x830ddf00
	ctx.lr = 0x830DE190;
	sub_830DDF00(ctx, base);
	// 830DE190: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE194: 418001F8  blt 0x830de38c
	if ctx.cr[0].lt {
	pc = 0x830DE38C; continue 'dispatch;
	}
	// 830DE198: 82C10054  lwz r22, 0x54(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE19C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830DE1A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830DE1A4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830DE1A8: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE1AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830DE1B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE1B4: 4E800421  bctrl
	ctx.lr = 0x830DE1B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE1B8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE1BC: 418001D4  blt 0x830de390
	if ctx.cr[0].lt {
	pc = 0x830DE390; continue 'dispatch;
	}
	// 830DE1C0: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830DE1C4: 419A0164  beq cr6, 0x830de328
	if ctx.cr[6].eq {
	pc = 0x830DE328; continue 'dispatch;
	}
	// 830DE1C8: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830DE1CC: 4BFFF2E5  bl 0x830dd4b0
	ctx.lr = 0x830DE1D0;
	sub_830DD4B0(ctx, base);
	// 830DE1D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE1D4: 41820034  beq 0x830de208
	if ctx.cr[0].eq {
	pc = 0x830DE208; continue 'dispatch;
	}
	// 830DE1D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DE1DC: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830DE1E0: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830DE1E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DE1E8: 396B8B3C  addi r11, r11, -0x74c4
	ctx.r[11].s64 = ctx.r[11].s64 + -29892;
	// 830DE1EC: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830DE1F0: 9323001C  stw r25, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[25].u32 ) };
	// 830DE1F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DE1F8: 93A30018  stw r29, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830DE1FC: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830DE200: 93230010  stw r25, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 830DE204: 48000008  b 0x830de20c
	pc = 0x830DE20C; continue 'dispatch;
	// 830DE208: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 830DE20C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DE210: 409A0010  bne cr6, 0x830de220
	if !ctx.cr[6].eq {
	pc = 0x830DE220; continue 'dispatch;
	}
	// 830DE214: 3FC08007  lis r30, -0x7ff9
	ctx.r[30].s64 = -2147024896;
	// 830DE218: 63DE000E  ori r30, r30, 0xe
	ctx.r[30].u64 = ctx.r[30].u64 | 14;
	// 830DE21C: 48000174  b 0x830de390
	pc = 0x830DE390; continue 'dispatch;
	// 830DE220: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE228: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE22C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830DE230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE234: 4E800421  bctrl
	ctx.lr = 0x830DE238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE238: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE23C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DE240: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830DE244: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE248: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE24C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE250: 4E800421  bctrl
	ctx.lr = 0x830DE254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE254: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830DE258: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE25C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE264: 40980014  bge cr6, 0x830de278
	if !ctx.cr[6].lt {
	pc = 0x830DE278; continue 'dispatch;
	}
	// 830DE268: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE26C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE270: 4E800421  bctrl
	ctx.lr = 0x830DE274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE274: 480000C0  b 0x830de334
	pc = 0x830DE334; continue 'dispatch;
	// 830DE278: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830DE27C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830DE280: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830DE284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE288: 4E800421  bctrl
	ctx.lr = 0x830DE28C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE28C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE290: 40800010  bge 0x830de2a0
	if !ctx.cr[0].lt {
	pc = 0x830DE2A0; continue 'dispatch;
	}
	// 830DE294: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE29C: 4BFFFFCC  b 0x830de268
	pc = 0x830DE268; continue 'dispatch;
	// 830DE2A0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830DE2A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830DE2A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830DE2AC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830DE2B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830DE2B4: 4BFFF375  bl 0x830dd628
	ctx.lr = 0x830DE2B8;
	sub_830DD628(ctx, base);
	// 830DE2B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE2BC: 41800030  blt 0x830de2ec
	if ctx.cr[0].lt {
	pc = 0x830DE2EC; continue 'dispatch;
	}
	// 830DE2C0: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE2C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DE2CC: 4802625D  bl 0x83104528
	ctx.lr = 0x830DE2D0;
	sub_83104528(ctx, base);
	// 830DE2D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE2D4: 41800010  blt 0x830de2e4
	if ctx.cr[0].lt {
	pc = 0x830DE2E4; continue 'dispatch;
	}
	// 830DE2D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DE2DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE2E0: 48027661  bl 0x83105940
	ctx.lr = 0x830DE2E4;
	sub_83105940(ctx, base);
	// 830DE2E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE2E8: 4BFFF1F1  bl 0x830dd4d8
	ctx.lr = 0x830DE2EC;
	sub_830DD4D8(ctx, base);
	// 830DE2EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE2F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE2F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE2FC: 4E800421  bctrl
	ctx.lr = 0x830DE300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE300: 48000028  b 0x830de328
	pc = 0x830DE328; continue 'dispatch;
	// 830DE304: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE308: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830DE30C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830DE310: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830DE314: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830DE318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE31C: 4E800421  bctrl
	ctx.lr = 0x830DE320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE320: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE324: 4180006C  blt 0x830de390
	if ctx.cr[0].lt {
	pc = 0x830DE390; continue 'dispatch;
	}
	// 830DE328: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE32C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 830DE330: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DE334: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE338: 41980058  blt cr6, 0x830de390
	if ctx.cr[6].lt {
	pc = 0x830DE390; continue 'dispatch;
	}
	// 830DE33C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DE344: 419A00FC  beq cr6, 0x830de440
	if ctx.cr[6].eq {
	pc = 0x830DE440; continue 'dispatch;
	}
	// 830DE348: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830DE34C: 419A00F4  beq cr6, 0x830de440
	if ctx.cr[6].eq {
	pc = 0x830DE440; continue 'dispatch;
	}
	// 830DE350: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830DE354: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DE358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE35C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830DE360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE364: 4E800421  bctrl
	ctx.lr = 0x830DE368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE368: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE36C: 418000D4  blt 0x830de440
	if ctx.cr[0].lt {
	pc = 0x830DE440; continue 'dispatch;
	}
	// 830DE370: 933B0000  stw r25, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 830DE374: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DE37C: 419A00C4  beq cr6, 0x830de440
	if ctx.cr[6].eq {
	pc = 0x830DE440; continue 'dispatch;
	}
	// 830DE380: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE384: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830DE388: 480000B0  b 0x830de438
	pc = 0x830DE438; continue 'dispatch;
	// 830DE38C: 82C10054  lwz r22, 0x54(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE390: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830DE394: 817FC18C  lwz r11, -0x3e74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-15988 as u32) ) } as u64;
	// 830DE398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DE39C: 419A0088  beq cr6, 0x830de424
	if ctx.cr[6].eq {
	pc = 0x830DE424; continue 'dispatch;
	}
	// 830DE3A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830DE3A4: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 830DE3A8: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 830DE3AC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830DE3B0: 38AB9D20  addi r5, r11, -0x62e0
	ctx.r[5].s64 = ctx.r[11].s64 + -25312;
	// 830DE3B4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 830DE3B8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830DE3BC: 419A0008  beq cr6, 0x830de3c4
	if ctx.cr[6].eq {
	pc = 0x830DE3C4; continue 'dispatch;
	}
	// 830DE3C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830DE3C4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 830DE3C8: 419A0008  beq cr6, 0x830de3d0
	if ctx.cr[6].eq {
	pc = 0x830DE3D0; continue 'dispatch;
	}
	// 830DE3CC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830DE3D0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DE3D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830DE3D8: 388B8B88  addi r4, r11, -0x7478
	ctx.r[4].s64 = ctx.r[11].s64 + -29816;
	// 830DE3DC: 4BFFFC45  bl 0x830de020
	ctx.lr = 0x830DE3E0;
	sub_830DE020(ctx, base);
	// 830DE3E0: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830DE3E4: 419A0018  beq cr6, 0x830de3fc
	if ctx.cr[6].eq {
	pc = 0x830DE3FC; continue 'dispatch;
	}
	// 830DE3E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DE3EC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 830DE3F0: 388B8B80  addi r4, r11, -0x7480
	ctx.r[4].s64 = ctx.r[11].s64 + -29824;
	// 830DE3F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830DE3F8: 4BFFFC29  bl 0x830de020
	ctx.lr = 0x830DE3FC;
	sub_830DE020(ctx, base);
	// 830DE3FC: 807FC18C  lwz r3, -0x3e74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-15988 as u32) ) } as u64;
	// 830DE400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DE404: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830DE408: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830DE40C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE410: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830DE414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE418: 4E800421  bctrl
	ctx.lr = 0x830DE41C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE41C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE420: 4BFFF0B9  bl 0x830dd4d8
	ctx.lr = 0x830DE424;
	sub_830DD4D8(ctx, base);
	// 830DE424: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DE42C: 419A0014  beq cr6, 0x830de440
	if ctx.cr[6].eq {
	pc = 0x830DE440; continue 'dispatch;
	}
	// 830DE430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE434: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE43C: 4E800421  bctrl
	ctx.lr = 0x830DE440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE440: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 830DE444: 419A0018  beq cr6, 0x830de45c
	if ctx.cr[6].eq {
	pc = 0x830DE45C; continue 'dispatch;
	}
	// 830DE448: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE44C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830DE450: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE458: 4E800421  bctrl
	ctx.lr = 0x830DE45C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE45C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830DE460: 481644FD  bl 0x8324295c
	ctx.lr = 0x830DE464;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830DE464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE468: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 830DE46C: 480C9D30  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE470 size=408
    let mut pc: u32 = 0x830DE470;
    'dispatch: loop {
        match pc {
            0x830DE470 => {
    //   block [0x830DE470..0x830DE608)
	// 830DE470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE474: 480C9CE5  bl 0x831a8158
	ctx.lr = 0x830DE478;
	sub_831A8130(ctx, base);
	// 830DE478: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE47C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE480: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830DE484: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 830DE488: 896BD7A0  lbz r11, -0x2860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-10336 as u32) ) } as u64;
	// 830DE48C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE490: 40820010  bne 0x830de4a0
	if !ctx.cr[0].eq {
	pc = 0x830DE4A0; continue 'dispatch;
	}
	// 830DE494: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830DE498: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830DE49C: 48000164  b 0x830de600
	pc = 0x830DE600; continue 'dispatch;
	// 830DE4A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DE4A4: 419A0154  beq cr6, 0x830de5f8
	if ctx.cr[6].eq {
	pc = 0x830DE5F8; continue 'dispatch;
	}
	// 830DE4A8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830DE4AC: 419A014C  beq cr6, 0x830de5f8
	if ctx.cr[6].eq {
	pc = 0x830DE5F8; continue 'dispatch;
	}
	// 830DE4B0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 830DE4B4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830DE4B8: 931D0000  stw r24, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 830DE4BC: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 830DE4C0: 4BFFF269  bl 0x830dd728
	ctx.lr = 0x830DE4C4;
	sub_830DD728(ctx, base);
	// 830DE4C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE4C8: 41800138  blt 0x830de600
	if ctx.cr[0].lt {
	pc = 0x830DE600; continue 'dispatch;
	}
	// 830DE4CC: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 830DE4D0: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 830DE4D4: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 830DE4D8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE4DC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830DE4E0: 4BFFF0A1  bl 0x830dd580
	ctx.lr = 0x830DE4E4;
	sub_830DD580(ctx, base);
	// 830DE4E4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DE4E8: 418000F8  blt 0x830de5e0
	if ctx.cr[0].lt {
	pc = 0x830DE5E0; continue 'dispatch;
	}
	// 830DE4EC: 4BFF081D  bl 0x830ced08
	ctx.lr = 0x830DE4F0;
	sub_830CED08(ctx, base);
	// 830DE4F0: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830DE4F4: 8341005C  lwz r26, 0x5c(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830DE4F8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830DE4FC: 83C10060  lwz r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 830DE500: 418200C4  beq 0x830de5c4
	if ctx.cr[0].eq {
	pc = 0x830DE5C4; continue 'dispatch;
	}
	// 830DE504: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830DE508: 419A0048  beq cr6, 0x830de550
	if ctx.cr[6].eq {
	pc = 0x830DE550; continue 'dispatch;
	}
	// 830DE50C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830DE510: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830DE514: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DE518: 48027CF1  bl 0x83106208
	ctx.lr = 0x830DE51C;
	sub_83106208(ctx, base);
	// 830DE51C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DE520: 418000C0  blt 0x830de5e0
	if ctx.cr[0].lt {
	pc = 0x830DE5E0; continue 'dispatch;
	}
	// 830DE524: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 830DE528: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE52C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830DE530: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830DE534: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE538: 4BFFFBB1  bl 0x830de0e8
	ctx.lr = 0x830DE53C;
	sub_830DE0E8(ctx, base);
	// 830DE53C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DE540: 408000A0  bge 0x830de5e0
	if !ctx.cr[0].lt {
	pc = 0x830DE5E0; continue 'dispatch;
	}
	// 830DE544: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE548: 4BFFEF91  bl 0x830dd4d8
	ctx.lr = 0x830DE54C;
	sub_830DD4D8(ctx, base);
	// 830DE54C: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 830DE550: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DE554: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE558: 388B8BA8  addi r4, r11, -0x7458
	ctx.r[4].s64 = ctx.r[11].s64 + -29784;
	// 830DE55C: 480D38D5  bl 0x831b1e30
	ctx.lr = 0x830DE560;
	sub_831B1E30(ctx, base);
	// 830DE560: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE564: 41820060  beq 0x830de5c4
	if ctx.cr[0].eq {
	pc = 0x830DE5C4; continue 'dispatch;
	}
	// 830DE568: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DE56C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE570: 388B8B98  addi r4, r11, -0x7468
	ctx.r[4].s64 = ctx.r[11].s64 + -29800;
	// 830DE574: 480D38BD  bl 0x831b1e30
	ctx.lr = 0x830DE578;
	sub_831B1E30(ctx, base);
	// 830DE578: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE57C: 41820048  beq 0x830de5c4
	if ctx.cr[0].eq {
	pc = 0x830DE5C4; continue 'dispatch;
	}
	// 830DE580: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830DE584: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830DE588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830DE58C: 48027C7D  bl 0x83106208
	ctx.lr = 0x830DE590;
	sub_83106208(ctx, base);
	// 830DE590: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DE594: 4180004C  blt 0x830de5e0
	if ctx.cr[0].lt {
	pc = 0x830DE5E0; continue 'dispatch;
	}
	// 830DE598: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 830DE59C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE5A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830DE5A4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830DE5A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE5AC: 4BFFFB3D  bl 0x830de0e8
	ctx.lr = 0x830DE5B0;
	sub_830DE0E8(ctx, base);
	// 830DE5B0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830DE5B4: 4080002C  bge 0x830de5e0
	if !ctx.cr[0].lt {
	pc = 0x830DE5E0; continue 'dispatch;
	}
	// 830DE5B8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE5BC: 4BFFEF1D  bl 0x830dd4d8
	ctx.lr = 0x830DE5C0;
	sub_830DD4D8(ctx, base);
	// 830DE5C0: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 830DE5C4: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 830DE5C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830DE5CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830DE5D0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830DE5D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE5D8: 4BFFFB11  bl 0x830de0e8
	ctx.lr = 0x830DE5DC;
	sub_830DE0E8(ctx, base);
	// 830DE5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830DE5E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE5E4: 4BFFEEF5  bl 0x830dd4d8
	ctx.lr = 0x830DE5E8;
	sub_830DD4D8(ctx, base);
	// 830DE5E8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE5EC: 4BFFEEED  bl 0x830dd4d8
	ctx.lr = 0x830DE5F0;
	sub_830DD4D8(ctx, base);
	// 830DE5F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE5F4: 4800000C  b 0x830de600
	pc = 0x830DE600; continue 'dispatch;
	// 830DE5F8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DE5FC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DE600: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830DE604: 480C9BA4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE608 size=180
    let mut pc: u32 = 0x830DE608;
    'dispatch: loop {
        match pc {
            0x830DE608 => {
    //   block [0x830DE608..0x830DE6BC)
	// 830DE608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE60C: 480C9B61  bl 0x831a816c
	ctx.lr = 0x830DE610;
	sub_831A8130(ctx, base);
	// 830DE610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE614: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE618: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DE61C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830DE620: 896BD7A0  lbz r11, -0x2860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-10336 as u32) ) } as u64;
	// 830DE624: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE628: 40820010  bne 0x830de638
	if !ctx.cr[0].eq {
	pc = 0x830DE638; continue 'dispatch;
	}
	// 830DE62C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830DE630: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830DE634: 48000080  b 0x830de6b4
	pc = 0x830DE6B4; continue 'dispatch;
	// 830DE638: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DE63C: 419A0070  beq cr6, 0x830de6ac
	if ctx.cr[6].eq {
	pc = 0x830DE6AC; continue 'dispatch;
	}
	// 830DE640: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DE644: 419A0068  beq cr6, 0x830de6ac
	if ctx.cr[6].eq {
	pc = 0x830DE6AC; continue 'dispatch;
	}
	// 830DE648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DE64C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DE650: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DE654: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DE658: 4BFFF0D1  bl 0x830dd728
	ctx.lr = 0x830DE65C;
	sub_830DD728(ctx, base);
	// 830DE65C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE660: 41800054  blt 0x830de6b4
	if ctx.cr[0].lt {
	pc = 0x830DE6B4; continue 'dispatch;
	}
	// 830DE664: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 830DE668: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE66C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 830DE670: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 830DE674: 4BFFEF0D  bl 0x830dd580
	ctx.lr = 0x830DE678;
	sub_830DD580(ctx, base);
	// 830DE678: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DE67C: 41800020  blt 0x830de69c
	if ctx.cr[0].lt {
	pc = 0x830DE69C; continue 'dispatch;
	}
	// 830DE680: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830DE684: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830DE688: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830DE68C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830DE690: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830DE694: 4BFFFA55  bl 0x830de0e8
	ctx.lr = 0x830DE698;
	sub_830DE0E8(ctx, base);
	// 830DE698: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DE69C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DE6A0: 4BFFEE39  bl 0x830dd4d8
	ctx.lr = 0x830DE6A4;
	sub_830DD4D8(ctx, base);
	// 830DE6A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE6A8: 4800000C  b 0x830de6b4
	pc = 0x830DE6B4; continue 'dispatch;
	// 830DE6AC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DE6B0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DE6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DE6B8: 480C9B04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE6C0 size=120
    let mut pc: u32 = 0x830DE6C0;
    'dispatch: loop {
        match pc {
            0x830DE6C0 => {
    //   block [0x830DE6C0..0x830DE738)
	// 830DE6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DE6C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DE6CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DE6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE6D4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE6D8: 3BEBD7A4  addi r31, r11, -0x285c
	ctx.r[31].s64 = ctx.r[11].s64 + -10332;
	// 830DE6DC: 897FFFFC  lbz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830DE6E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE6E4: 4182003C  beq 0x830de720
	if ctx.cr[0].eq {
	pc = 0x830DE720; continue 'dispatch;
	}
	// 830DE6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DE6EC: 997FFFFC  stb r11, -4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u8 ) };
	// 830DE6F0: 48026F39  bl 0x83105628
	ctx.lr = 0x830DE6F4;
	sub_83105628(ctx, base);
	// 830DE6F4: 4BFEE9FD  bl 0x830cd0f0
	ctx.lr = 0x830DE6F8;
	sub_830CD0F0(ctx, base);
	// 830DE6F8: 4BFFF971  bl 0x830de068
	ctx.lr = 0x830DE6FC;
	sub_830DE068(ctx, base);
	// 830DE6FC: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE700: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DE704: 419A0014  beq cr6, 0x830de718
	if ctx.cr[6].eq {
	pc = 0x830DE718; continue 'dispatch;
	}
	// 830DE708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE70C: 4BFFF88D  bl 0x830ddf98
	ctx.lr = 0x830DE710;
	sub_830DDF98(ctx, base);
	// 830DE710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE714: 4BFFEDC5  bl 0x830dd4d8
	ctx.lr = 0x830DE718;
	sub_830DD4D8(ctx, base);
	// 830DE718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DE71C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DE720: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DE724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DE728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DE72C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DE730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DE734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE738 size=492
    let mut pc: u32 = 0x830DE738;
    'dispatch: loop {
        match pc {
            0x830DE738 => {
    //   block [0x830DE738..0x830DE924)
	// 830DE738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE73C: 480C9A21  bl 0x831a815c
	ctx.lr = 0x830DE740;
	sub_831A8130(ctx, base);
	// 830DE740: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE744: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830DE748: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 830DE74C: 936100B4  stw r27, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 830DE750: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830DE754: 409A0010  bne cr6, 0x830de764
	if !ctx.cr[6].eq {
	pc = 0x830DE764; continue 'dispatch;
	}
	// 830DE758: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DE75C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DE760: 480001BC  b 0x830de91c
	pc = 0x830DE91C; continue 'dispatch;
	// 830DE764: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE768: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DE76C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830DE770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE774: 4E800421  bctrl
	ctx.lr = 0x830DE778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE778: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE77C: 41820198  beq 0x830de914
	if ctx.cr[0].eq {
	pc = 0x830DE914; continue 'dispatch;
	}
	// 830DE780: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE784: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DE788: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DE78C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE790: 4E800421  bctrl
	ctx.lr = 0x830DE794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE794: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830DE798: 4182017C  beq 0x830de914
	if ctx.cr[0].eq {
	pc = 0x830DE914; continue 'dispatch;
	}
	// 830DE79C: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE7A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE7A4: 41820170  beq 0x830de914
	if ctx.cr[0].eq {
	pc = 0x830DE914; continue 'dispatch;
	}
	// 830DE7A8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE7AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DE7B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DE7B4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830DE7B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE7BC: 4E800421  bctrl
	ctx.lr = 0x830DE7C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE7C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE7C4: 41800158  blt 0x830de91c
	if ctx.cr[0].lt {
	pc = 0x830DE91C; continue 'dispatch;
	}
	// 830DE7C8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE7CC: 3B4BD784  addi r26, r11, -0x287c
	ctx.r[26].s64 = ctx.r[11].s64 + -10364;
	// 830DE7D0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830DE7D4: 48164199  bl 0x8324296c
	ctx.lr = 0x830DE7D8;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830DE7D8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830DE7DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830DE7E0: 83FED7A4  lwz r31, -0x285c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DE7E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE7E8: 4BFFF579  bl 0x830ddd60
	ctx.lr = 0x830DE7EC;
	sub_830DDD60(ctx, base);
	// 830DE7EC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830DE7F0: 409A000C  bne cr6, 0x830de7fc
	if !ctx.cr[6].eq {
	pc = 0x830DE7FC; continue 'dispatch;
	}
	// 830DE7F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DE7F8: 48000010  b 0x830de808
	pc = 0x830DE808; continue 'dispatch;
	// 830DE7FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE800: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830DE804: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830DE808: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830DE80C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DE810: 419A005C  beq cr6, 0x830de86c
	if ctx.cr[6].eq {
	pc = 0x830DE86C; continue 'dispatch;
	}
	// 830DE814: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE81C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DE820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE824: 4E800421  bctrl
	ctx.lr = 0x830DE828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE828: 83BED7A4  lwz r29, -0x285c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DE82C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830DE830: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DE834: 4BFFF52D  bl 0x830ddd60
	ctx.lr = 0x830DE838;
	sub_830DDD60(ctx, base);
	// 830DE838: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830DE83C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 830DE840: 419A0014  beq cr6, 0x830de854
	if ctx.cr[6].eq {
	pc = 0x830DE854; continue 'dispatch;
	}
	// 830DE844: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DE848: 48005A49  bl 0x830e4290
	ctx.lr = 0x830DE84C;
	sub_830E4290(ctx, base);
	// 830DE84C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE850: 4082001C  bne 0x830de86c
	if !ctx.cr[0].eq {
	pc = 0x830DE86C; continue 'dispatch;
	}
	// 830DE854: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 830DE858: 63FF4005  ori r31, r31, 0x4005
	ctx.r[31].u64 = ctx.r[31].u64 | 16389;
	// 830DE85C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830DE860: 481640FD  bl 0x8324295c
	ctx.lr = 0x830DE864;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830DE864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE868: 480000B4  b 0x830de91c
	pc = 0x830DE91C; continue 'dispatch;
	// 830DE86C: 38A100B4  addi r5, r1, 0xb4
	ctx.r[5].s64 = ctx.r[1].s64 + 180;
	// 830DE870: 807ED7A4  lwz r3, -0x285c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DE874: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830DE878: 4BFEE9D1  bl 0x830cd248
	ctx.lr = 0x830DE87C;
	sub_830CD248(ctx, base);
	// 830DE87C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE880: 40820044  bne 0x830de8c4
	if !ctx.cr[0].eq {
	pc = 0x830DE8C4; continue 'dispatch;
	}
	// 830DE884: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DE888: 419A0030  beq cr6, 0x830de8b8
	if ctx.cr[6].eq {
	pc = 0x830DE8B8; continue 'dispatch;
	}
	// 830DE88C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE890: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 830DE894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE898: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DE89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE8A0: 4E800421  bctrl
	ctx.lr = 0x830DE8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE8A4: 817ED7A4  lwz r11, -0x285c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-10332 as u32) ) } as u64;
	// 830DE8A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830DE8AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830DE8B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830DE8B4: 4BFEE995  bl 0x830cd248
	ctx.lr = 0x830DE8B8;
	sub_830CD248(ctx, base);
	// 830DE8B8: 3FE08007  lis r31, -0x7ff9
	ctx.r[31].s64 = -2147024896;
	// 830DE8BC: 63FF000E  ori r31, r31, 0xe
	ctx.r[31].u64 = ctx.r[31].u64 | 14;
	// 830DE8C0: 4BFFFF9C  b 0x830de85c
	pc = 0x830DE85C; continue 'dispatch;
	// 830DE8C4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE8C8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830DE8CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE8D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE8D4: 4E800421  bctrl
	ctx.lr = 0x830DE8D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE8D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830DE8DC: 48164081  bl 0x8324295c
	ctx.lr = 0x830DE8E0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830DE8E0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830DE8E4: 419A000C  beq cr6, 0x830de8f0
	if ctx.cr[6].eq {
	pc = 0x830DE8F0; continue 'dispatch;
	}
	// 830DE8E8: 93F90000  stw r31, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830DE8EC: 48000020  b 0x830de90c
	pc = 0x830DE90C; continue 'dispatch;
	// 830DE8F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DE8F4: 419A0018  beq cr6, 0x830de90c
	if ctx.cr[6].eq {
	pc = 0x830DE90C; continue 'dispatch;
	}
	// 830DE8F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE8FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DE900: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE908: 4E800421  bctrl
	ctx.lr = 0x830DE90C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DE90C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DE910: 4800000C  b 0x830de91c
	pc = 0x830DE91C; continue 'dispatch;
	// 830DE914: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830DE918: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830DE91C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830DE920: 480C988C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DE928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DE928 size=336
    let mut pc: u32 = 0x830DE928;
    'dispatch: loop {
        match pc {
            0x830DE928 => {
    //   block [0x830DE928..0x830DEA78)
	// 830DE928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DE92C: 480C983D  bl 0x831a8168
	ctx.lr = 0x830DE930;
	sub_831A8130(ctx, base);
	// 830DE930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DE934: 48026B05  bl 0x83105438
	ctx.lr = 0x830DE938;
	sub_83105438(ctx, base);
	// 830DE938: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DE93C: 41800134  blt 0x830dea70
	if ctx.cr[0].lt {
	pc = 0x830DEA70; continue 'dispatch;
	}
	// 830DE940: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830DE944: 4BFFEB6D  bl 0x830dd4b0
	ctx.lr = 0x830DE948;
	sub_830DD4B0(ctx, base);
	// 830DE948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE94C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830DE950: 41820024  beq 0x830de974
	if ctx.cr[0].eq {
	pc = 0x830DE974; continue 'dispatch;
	}
	// 830DE954: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DE958: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830DE95C: 93830004  stw r28, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830DE960: 3BEBD7A0  addi r31, r11, -0x2860
	ctx.r[31].s64 = ctx.r[11].s64 + -10336;
	// 830DE964: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830DE968: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830DE96C: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830DE970: 48000010  b 0x830de980
	pc = 0x830DE980; continue 'dispatch;
	// 830DE974: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DE978: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830DE97C: 3BEAD7A0  addi r31, r10, -0x2860
	ctx.r[31].s64 = ctx.r[10].s64 + -10336;
	// 830DE980: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DE984: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DE988: 409A0010  bne cr6, 0x830de998
	if !ctx.cr[6].eq {
	pc = 0x830DE998; continue 'dispatch;
	}
	// 830DE98C: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 830DE990: 63BD000E  ori r29, r29, 0xe
	ctx.r[29].u64 = ctx.r[29].u64 | 14;
	// 830DE994: 480000C0  b 0x830dea54
	pc = 0x830DEA54; continue 'dispatch;
	// 830DE998: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830DE99C: 4BFFEB15  bl 0x830dd4b0
	ctx.lr = 0x830DE9A0;
	sub_830DD4B0(ctx, base);
	// 830DE9A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DE9A4: 41820020  beq 0x830de9c4
	if ctx.cr[0].eq {
	pc = 0x830DE9C4; continue 'dispatch;
	}
	// 830DE9A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830DE9AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830DE9B0: 396B8B64  addi r11, r11, -0x749c
	ctx.r[11].s64 = ctx.r[11].s64 + -29852;
	// 830DE9B4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830DE9B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DE9BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DE9C0: 48000008  b 0x830de9c8
	pc = 0x830DE9C8; continue 'dispatch;
	// 830DE9C4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830DE9C8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DE9CC: 409A0010  bne cr6, 0x830de9dc
	if !ctx.cr[6].eq {
	pc = 0x830DE9DC; continue 'dispatch;
	}
	// 830DE9D0: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 830DE9D4: 63BD000E  ori r29, r29, 0xe
	ctx.r[29].u64 = ctx.r[29].u64 | 14;
	// 830DE9D8: 48000048  b 0x830dea20
	pc = 0x830DEA20; continue 'dispatch;
	// 830DE9DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DE9E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE9E4: 4BFFFD55  bl 0x830de738
	ctx.lr = 0x830DE9E8;
	sub_830DE738(ctx, base);
	// 830DE9E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DE9EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DE9F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DE9F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DE9F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830DE9FC: 4E800421  bctrl
	ctx.lr = 0x830DEA00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830DEA00: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830DEA04: 4198001C  blt cr6, 0x830dea20
	if ctx.cr[6].lt {
	pc = 0x830DEA20; continue 'dispatch;
	}
	// 830DEA08: 48027C09  bl 0x83106610
	ctx.lr = 0x830DEA0C;
	sub_83106610(ctx, base);
	// 830DEA0C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830DEA10: 41800010  blt 0x830dea20
	if ctx.cr[0].lt {
	pc = 0x830DEA20; continue 'dispatch;
	}
	// 830DEA14: 480281F5  bl 0x83106c08
	ctx.lr = 0x830DEA18;
	sub_83106C08(ctx, base);
	// 830DEA18: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830DEA1C: 40800048  bge 0x830dea64
	if !ctx.cr[0].lt {
	pc = 0x830DEA64; continue 'dispatch;
	}
	// 830DEA20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DEA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DEA28: 419A002C  beq cr6, 0x830dea54
	if ctx.cr[6].eq {
	pc = 0x830DEA54; continue 'dispatch;
	}
	// 830DEA2C: 4BFFF63D  bl 0x830de068
	ctx.lr = 0x830DEA30;
	sub_830DE068(ctx, base);
	// 830DEA30: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DEA34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DEA38: 419A0014  beq cr6, 0x830dea4c
	if ctx.cr[6].eq {
	pc = 0x830DEA4C; continue 'dispatch;
	}
	// 830DEA3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DEA40: 4BFFF559  bl 0x830ddf98
	ctx.lr = 0x830DEA44;
	sub_830DDF98(ctx, base);
	// 830DEA44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DEA48: 4BFFEA91  bl 0x830dd4d8
	ctx.lr = 0x830DEA4C;
	sub_830DD4D8(ctx, base);
	// 830DEA4C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830DEA50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DEA54: 4BFEE69D  bl 0x830cd0f0
	ctx.lr = 0x830DEA58;
	sub_830CD0F0(ctx, base);
	// 830DEA58: 48026BD1  bl 0x83105628
	ctx.lr = 0x830DEA5C;
	sub_83105628(ctx, base);
	// 830DEA5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DEA60: 48000010  b 0x830dea70
	pc = 0x830DEA70; continue 'dispatch;
	// 830DEA64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830DEA68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DEA6C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830DEA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DEA74: 480C9744  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEA78 size=108
    let mut pc: u32 = 0x830DEA78;
    'dispatch: loop {
        match pc {
            0x830DEA78 => {
    //   block [0x830DEA78..0x830DEAE4)
	// 830DEA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEA80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DEA84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DEA88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEA8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DEA90: 816300F0  lwz r11, 0xf0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 830DEA94: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DEA98: 419A000C  beq cr6, 0x830deaa4
	if ctx.cr[6].eq {
	pc = 0x830DEAA4; continue 'dispatch;
	}
	// 830DEA9C: 556B07B8  rlwinm r11, r11, 0, 0x1e, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830DEAA0: 48000008  b 0x830deaa8
	pc = 0x830DEAA8; continue 'dispatch;
	// 830DEAA4: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 830DEAA8: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830DEAAC: 916300F0  stw r11, 0xf0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830DEAB0: 48000014  b 0x830deac4
	pc = 0x830DEAC4; continue 'dispatch;
	// 830DEAB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DEAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DEABC: 4BFFFFBD  bl 0x830dea78
	ctx.lr = 0x830DEAC0;
	sub_830DEA78(ctx, base);
	// 830DEAC0: 83FF0020  lwz r31, 0x20(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830DEAC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DEAC8: 409AFFEC  bne cr6, 0x830deab4
	if !ctx.cr[6].eq {
	pc = 0x830DEAB4; continue 'dispatch;
	}
	// 830DEACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DEAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DEAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DEAD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DEADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DEAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEAE8 size=88
    let mut pc: u32 = 0x830DEAE8;
    'dispatch: loop {
        match pc {
            0x830DEAE8 => {
    //   block [0x830DEAE8..0x830DEB40)
	// 830DEAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEAF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830DEAF8: 419A0034  beq cr6, 0x830deb2c
	if ctx.cr[6].eq {
	pc = 0x830DEB2C; continue 'dispatch;
	}
	// 830DEAFC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830DEB00: 419A002C  beq cr6, 0x830deb2c
	if ctx.cr[6].eq {
	pc = 0x830DEB2C; continue 'dispatch;
	}
	// 830DEB04: 4BFF006D  bl 0x830ceb70
	ctx.lr = 0x830DEB08;
	sub_830CEB70(ctx, base);
	// 830DEB08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DEB0C: 41820020  beq 0x830deb2c
	if ctx.cr[0].eq {
	pc = 0x830DEB2C; continue 'dispatch;
	}
	// 830DEB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DEB14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DEB18: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DEB1C: 4BFEF955  bl 0x830ce470
	ctx.lr = 0x830DEB20;
	sub_830CE470(ctx, base);
	// 830DEB20: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DEB24: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DEB28: 40800008  bge 0x830deb30
	if !ctx.cr[0].lt {
	pc = 0x830DEB30; continue 'dispatch;
	}
	// 830DEB2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DEB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DEB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DEB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DEB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEB40 size=16
    let mut pc: u32 = 0x830DEB40;
    'dispatch: loop {
        match pc {
            0x830DEB40 => {
    //   block [0x830DEB40..0x830DEB50)
	// 830DEB40: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 830DEB44: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830DEB48: 916A56F4  stw r11, 0x56f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22260 as u32), ctx.r[11].u32 ) };
	// 830DEB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEB50 size=68
    let mut pc: u32 = 0x830DEB50;
    'dispatch: loop {
        match pc {
            0x830DEB50 => {
    //   block [0x830DEB50..0x830DEB94)
	// 830DEB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEB58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEB5C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEB60: 808BC080  lwz r4, -0x3f80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16256 as u32) ) } as u64;
	// 830DEB64: 4BFF000D  bl 0x830ceb70
	ctx.lr = 0x830DEB68;
	sub_830CEB70(ctx, base);
	// 830DEB68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DEB6C: 41820018  beq 0x830deb84
	if ctx.cr[0].eq {
	pc = 0x830DEB84; continue 'dispatch;
	}
	// 830DEB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DEB74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DEB78: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DEB7C: 4BFEF8F5  bl 0x830ce470
	ctx.lr = 0x830DEB80;
	sub_830CE470(ctx, base);
	// 830DEB80: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DEB84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DEB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DEB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DEB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEB98 size=48
    let mut pc: u32 = 0x830DEB98;
    'dispatch: loop {
        match pc {
            0x830DEB98 => {
    //   block [0x830DEB98..0x830DEBC8)
	// 830DEB98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830DEB9C: 816B56F0  lwz r11, 0x56f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22256 as u32) ) } as u64;
	// 830DEBA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DEBA4: 409A0024  bne cr6, 0x830debc8
	if !ctx.cr[6].eq {
		sub_830DEBC8(ctx, base);
		return;
	}
	// 830DEBA8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830DEBAC: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 830DEBB0: 419A0018  beq cr6, 0x830debc8
	if ctx.cr[6].eq {
		sub_830DEBC8(ctx, base);
		return;
	}
	// 830DEBB4: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 830DEBB8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830DEBBC: 3969D804  addi r11, r9, -0x27fc
	ctx.r[11].s64 = ctx.r[9].s64 + -10236;
	// 830DEBC0: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830DEBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEBC8 size=12
    let mut pc: u32 = 0x830DEBC8;
    'dispatch: loop {
        match pc {
            0x830DEBC8 => {
    //   block [0x830DEBC8..0x830DEBD4)
	// 830DEBC8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEBCC: 386BD800  addi r3, r11, -0x2800
	ctx.r[3].s64 = ctx.r[11].s64 + -10240;
	// 830DEBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEBD8 size=148
    let mut pc: u32 = 0x830DEBD8;
    'dispatch: loop {
        match pc {
            0x830DEBD8 => {
    //   block [0x830DEBD8..0x830DEC6C)
	// 830DEBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEBE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DEBE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DEBE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEBEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DEBF0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830DEBF4: 4BFF001D  bl 0x830cec10
	ctx.lr = 0x830DEBF8;
	sub_830CEC10(ctx, base);
	// 830DEBF8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830DEBFC: 816B56F0  lwz r11, 0x56f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22256 as u32) ) } as u64;
	// 830DEC00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DEC04: 409A0024  bne cr6, 0x830dec28
	if !ctx.cr[6].eq {
	pc = 0x830DEC28; continue 'dispatch;
	}
	// 830DEC08: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 830DEC0C: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 830DEC10: 419A0018  beq cr6, 0x830dec28
	if ctx.cr[6].eq {
	pc = 0x830DEC28; continue 'dispatch;
	}
	// 830DEC14: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 830DEC18: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830DEC1C: 3969D804  addi r11, r9, -0x27fc
	ctx.r[11].s64 = ctx.r[9].s64 + -10236;
	// 830DEC20: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830DEC24: 4800000C  b 0x830dec30
	pc = 0x830DEC30; continue 'dispatch;
	// 830DEC28: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEC2C: 396BD800  addi r11, r11, -0x2800
	ctx.r[11].s64 = ctx.r[11].s64 + -10240;
	// 830DEC30: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DEC34: 419A000C  beq cr6, 0x830dec40
	if ctx.cr[6].eq {
	pc = 0x830DEC40; continue 'dispatch;
	}
	// 830DEC38: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DEC3C: 48000018  b 0x830dec54
	pc = 0x830DEC54; continue 'dispatch;
	// 830DEC40: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DEC44: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DEC48: 409A000C  bne cr6, 0x830dec54
	if !ctx.cr[6].eq {
	pc = 0x830DEC54; continue 'dispatch;
	}
	// 830DEC4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DEC50: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830DEC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DEC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DEC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DEC60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DEC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DEC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEC70 size=112
    let mut pc: u32 = 0x830DEC70;
    'dispatch: loop {
        match pc {
            0x830DEC70 => {
    //   block [0x830DEC70..0x830DECE0)
	// 830DEC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEC7C: 4BFEFF95  bl 0x830cec10
	ctx.lr = 0x830DEC80;
	sub_830CEC10(ctx, base);
	// 830DEC80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEC84: 814BD800  lwz r10, -0x2800(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10240 as u32) ) } as u64;
	// 830DEC88: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DEC8C: 409A000C  bne cr6, 0x830dec98
	if !ctx.cr[6].eq {
	pc = 0x830DEC98; continue 'dispatch;
	}
	// 830DEC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830DEC94: 914BD800  stw r10, -0x2800(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-10240 as u32), ctx.r[10].u32 ) };
	// 830DEC98: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830DECA0: 394BD804  addi r10, r11, -0x27fc
	ctx.r[10].s64 = ctx.r[11].s64 + -10236;
	// 830DECA4: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830DECA8: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830DECAC: 7F081840  cmplw cr6, r8, r3
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DECB0: 409A000C  bne cr6, 0x830decbc
	if !ctx.cr[6].eq {
	pc = 0x830DECBC; continue 'dispatch;
	}
	// 830DECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830DECB8: 7D0B512E  stwx r8, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 830DECBC: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 830DECC0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830DECC4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 830DECC8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830DECCC: 4198FFD8  blt cr6, 0x830deca4
	if ctx.cr[6].lt {
	pc = 0x830DECA4; continue 'dispatch;
	}
	// 830DECD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DECD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DECD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DECDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DECE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DECE0 size=44
    let mut pc: u32 = 0x830DECE0;
    'dispatch: loop {
        match pc {
            0x830DECE0 => {
    //   block [0x830DECE0..0x830DED0C)
	// 830DECE0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DECE4: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DECE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DECEC: 55290739  rlwinm. r9, r9, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830DECF0: 814AC180  lwz r10, -0x3e80(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16000 as u32) ) } as u64;
	// 830DECF4: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 830DECF8: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830DECFC: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 830DED00: 4182000C  beq 0x830ded0c
	if ctx.cr[0].eq {
		sub_830DED0C(ctx, base);
		return;
	}
	// 830DED04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DED08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DED0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DED0C size=152
    let mut pc: u32 = 0x830DED0C;
    'dispatch: loop {
        match pc {
            0x830DED0C => {
    //   block [0x830DED0C..0x830DEDA4)
	// 830DED0C: 2B0B5823  cmplwi cr6, r11, 0x5823
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22563 as u32, &mut ctx.xer);
	// 830DED10: 419A00EC  beq cr6, 0x830dedfc
	if ctx.cr[6].eq {
		sub_830DEDFC(ctx, base);
		return;
	}
	// 830DED14: 2B0B5812  cmplwi cr6, r11, 0x5812
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22546 as u32, &mut ctx.xer);
	// 830DED18: 419A00E4  beq cr6, 0x830dedfc
	if ctx.cr[6].eq {
		sub_830DEDFC(ctx, base);
		return;
	}
	// 830DED1C: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 830DED20: 409A000C  bne cr6, 0x830ded2c
	if !ctx.cr[6].eq {
	pc = 0x830DED2C; continue 'dispatch;
	}
	// 830DED24: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DED28: 409A00D4  bne cr6, 0x830dedfc
	if !ctx.cr[6].eq {
		sub_830DEDFC(ctx, base);
		return;
	}
	// 830DED2C: 2B0B5822  cmplwi cr6, r11, 0x5822
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22562 as u32, &mut ctx.xer);
	// 830DED30: 419A00C4  beq cr6, 0x830dedf4
	if ctx.cr[6].eq {
		sub_830DEDF4(ctx, base);
		return;
	}
	// 830DED34: 2B0B5813  cmplwi cr6, r11, 0x5813
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22547 as u32, &mut ctx.xer);
	// 830DED38: 419A00BC  beq cr6, 0x830dedf4
	if ctx.cr[6].eq {
		sub_830DEDF4(ctx, base);
		return;
	}
	// 830DED3C: 2B0B0027  cmplwi cr6, r11, 0x27
	ctx.cr[6].compare_u32(ctx.r[11].u32, 39 as u32, &mut ctx.xer);
	// 830DED40: 409A000C  bne cr6, 0x830ded4c
	if !ctx.cr[6].eq {
	pc = 0x830DED4C; continue 'dispatch;
	}
	// 830DED44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DED48: 409A00AC  bne cr6, 0x830dedf4
	if !ctx.cr[6].eq {
		sub_830DEDF4(ctx, base);
		return;
	}
	// 830DED4C: 2B0B5820  cmplwi cr6, r11, 0x5820
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22560 as u32, &mut ctx.xer);
	// 830DED50: 419A009C  beq cr6, 0x830dedec
	if ctx.cr[6].eq {
		sub_830DEDEC(ctx, base);
		return;
	}
	// 830DED54: 2B0B5810  cmplwi cr6, r11, 0x5810
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22544 as u32, &mut ctx.xer);
	// 830DED58: 419A0094  beq cr6, 0x830dedec
	if ctx.cr[6].eq {
		sub_830DEDEC(ctx, base);
		return;
	}
	// 830DED5C: 2B0B0026  cmplwi cr6, r11, 0x26
	ctx.cr[6].compare_u32(ctx.r[11].u32, 38 as u32, &mut ctx.xer);
	// 830DED60: 409A000C  bne cr6, 0x830ded6c
	if !ctx.cr[6].eq {
	pc = 0x830DED6C; continue 'dispatch;
	}
	// 830DED64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DED68: 409A0084  bne cr6, 0x830dedec
	if !ctx.cr[6].eq {
		sub_830DEDEC(ctx, base);
		return;
	}
	// 830DED6C: 2B0B5821  cmplwi cr6, r11, 0x5821
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22561 as u32, &mut ctx.xer);
	// 830DED70: 419A0074  beq cr6, 0x830dede4
	if ctx.cr[6].eq {
		sub_830DEDE4(ctx, base);
		return;
	}
	// 830DED74: 2B0B5811  cmplwi cr6, r11, 0x5811
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22545 as u32, &mut ctx.xer);
	// 830DED78: 419A006C  beq cr6, 0x830dede4
	if ctx.cr[6].eq {
		sub_830DEDE4(ctx, base);
		return;
	}
	// 830DED7C: 2B0B0028  cmplwi cr6, r11, 0x28
	ctx.cr[6].compare_u32(ctx.r[11].u32, 40 as u32, &mut ctx.xer);
	// 830DED80: 409A000C  bne cr6, 0x830ded8c
	if !ctx.cr[6].eq {
	pc = 0x830DED8C; continue 'dispatch;
	}
	// 830DED84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DED88: 409A005C  bne cr6, 0x830dede4
	if !ctx.cr[6].eq {
		sub_830DEDE4(ctx, base);
		return;
	}
	// 830DED8C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 830DED90: 409A0014  bne cr6, 0x830deda4
	if !ctx.cr[6].eq {
		sub_830DEDA4(ctx, base);
		return;
	}
	// 830DED94: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DED98: 419AFF6C  beq cr6, 0x830ded04
	if ctx.cr[6].eq {
		sub_830DECE0(ctx, base);
		return;
	}
	// 830DED9C: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 830DEDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEDA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEDA4 size=56
    let mut pc: u32 = 0x830DEDA4;
    'dispatch: loop {
        match pc {
            0x830DEDA4 => {
    //   block [0x830DEDA4..0x830DEDDC)
	// 830DEDA4: 2B0B5806  cmplwi cr6, r11, 0x5806
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22534 as u32, &mut ctx.xer);
	// 830DEDA8: 419A0034  beq cr6, 0x830deddc
	if ctx.cr[6].eq {
		sub_830DEDDC(ctx, base);
		return;
	}
	// 830DEDAC: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 830DEDB0: 409A000C  bne cr6, 0x830dedbc
	if !ctx.cr[6].eq {
	pc = 0x830DEDBC; continue 'dispatch;
	}
	// 830DEDB4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DEDB8: 409A0024  bne cr6, 0x830deddc
	if !ctx.cr[6].eq {
		sub_830DEDDC(ctx, base);
		return;
	}
	// 830DEDBC: 2B0B5807  cmplwi cr6, r11, 0x5807
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22535 as u32, &mut ctx.xer);
	// 830DEDC0: 419A0014  beq cr6, 0x830dedd4
	if ctx.cr[6].eq {
	pc = 0x830DEDD4; continue 'dispatch;
	}
	// 830DEDC4: 2B0B0022  cmplwi cr6, r11, 0x22
	ctx.cr[6].compare_u32(ctx.r[11].u32, 34 as u32, &mut ctx.xer);
	// 830DEDC8: 409AFF3C  bne cr6, 0x830ded04
	if !ctx.cr[6].eq {
		sub_830DECE0(ctx, base);
		return;
	}
	// 830DEDCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830DEDD0: 419AFF34  beq cr6, 0x830ded04
	if ctx.cr[6].eq {
		sub_830DECE0(ctx, base);
		return;
	}
	// 830DEDD4: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 830DEDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEDDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEDDC size=8
    let mut pc: u32 = 0x830DEDDC;
    'dispatch: loop {
        match pc {
            0x830DEDDC => {
    //   block [0x830DEDDC..0x830DEDE4)
	// 830DEDDC: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 830DEDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEDE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEDE4 size=8
    let mut pc: u32 = 0x830DEDE4;
    'dispatch: loop {
        match pc {
            0x830DEDE4 => {
    //   block [0x830DEDE4..0x830DEDEC)
	// 830DEDE4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830DEDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEDEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEDEC size=8
    let mut pc: u32 = 0x830DEDEC;
    'dispatch: loop {
        match pc {
            0x830DEDEC => {
    //   block [0x830DEDEC..0x830DEDF4)
	// 830DEDEC: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 830DEDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEDF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEDF4 size=8
    let mut pc: u32 = 0x830DEDF4;
    'dispatch: loop {
        match pc {
            0x830DEDF4 => {
    //   block [0x830DEDF4..0x830DEDFC)
	// 830DEDF4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830DEDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEDFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEDFC size=8
    let mut pc: u32 = 0x830DEDFC;
    'dispatch: loop {
        match pc {
            0x830DEDFC => {
    //   block [0x830DEDFC..0x830DEE04)
	// 830DEDFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DEE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE08 size=20
    let mut pc: u32 = 0x830DEE08;
    'dispatch: loop {
        match pc {
            0x830DEE08 => {
    //   block [0x830DEE08..0x830DEE1C)
	// 830DEE08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEE0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DEE10: 816BD814  lwz r11, -0x27ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10220 as u32) ) } as u64;
	// 830DEE14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DEE18: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE1C size=8
    let mut pc: u32 = 0x830DEE1C;
    'dispatch: loop {
        match pc {
            0x830DEE1C => {
    //   block [0x830DEE1C..0x830DEE24)
	// 830DEE1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DEE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE28 size=12
    let mut pc: u32 = 0x830DEE28;
    'dispatch: loop {
        match pc {
            0x830DEE28 => {
    //   block [0x830DEE28..0x830DEE34)
	// 830DEE28: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEE2C: 806BD848  lwz r3, -0x27b8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10168 as u32) ) } as u64;
	// 830DEE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE38 size=28
    let mut pc: u32 = 0x830DEE38;
    'dispatch: loop {
        match pc {
            0x830DEE38 => {
    //   block [0x830DEE38..0x830DEE54)
	// 830DEE38: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830DEE3C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830DEE40: 41980014  blt cr6, 0x830dee54
	if ctx.cr[6].lt {
		sub_830DEE54(ctx, base);
		return;
	}
	// 830DEE44: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 830DEE48: 419A0014  beq cr6, 0x830dee5c
	if ctx.cr[6].eq {
		sub_830DEE54(ctx, base);
		return;
	}
	// 830DEE4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DEE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE54 size=20
    let mut pc: u32 = 0x830DEE54;
    'dispatch: loop {
        match pc {
            0x830DEE54 => {
    //   block [0x830DEE54..0x830DEE68)
	// 830DEE54: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 830DEE58: 409A0010  bne cr6, 0x830dee68
	if !ctx.cr[6].eq {
		sub_830DEE68(ctx, base);
		return;
	}
	// 830DEE5C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEE60: 396BD848  addi r11, r11, -0x27b8
	ctx.r[11].s64 = ctx.r[11].s64 + -10168;
	// 830DEE64: 48000014  b 0x830dee78
	sub_830DEE68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE68 size=24
    let mut pc: u32 = 0x830DEE68;
    'dispatch: loop {
        match pc {
            0x830DEE68 => {
    //   block [0x830DEE68..0x830DEE80)
	// 830DEE68: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 830DEE6C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830DEE70: 3969D84C  addi r11, r9, -0x27b4
	ctx.r[11].s64 = ctx.r[9].s64 + -10164;
	// 830DEE74: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830DEE78: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DEE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DEE80 size=8
    let mut pc: u32 = 0x830DEE80;
    'dispatch: loop {
        match pc {
            0x830DEE80 => {
    //   block [0x830DEE80..0x830DEE88)
	// 830DEE80: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 830DEE84: 480036CC  b 0x830e2550
	sub_830E2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEE88 size=120
    let mut pc: u32 = 0x830DEE88;
    'dispatch: loop {
        match pc {
            0x830DEE88 => {
    //   block [0x830DEE88..0x830DEF00)
	// 830DEE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEE8C: 480C92E1  bl 0x831a816c
	ctx.lr = 0x830DEE90;
	sub_831A8130(ctx, base);
	// 830DEE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEE94: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEE98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DEE9C: 83EBD848  lwz r31, -0x27b8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10168 as u32) ) } as u64;
	// 830DEEA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DEEA4: 419A0018  beq cr6, 0x830deebc
	if ctx.cr[6].eq {
	pc = 0x830DEEBC; continue 'dispatch;
	}
	// 830DEEA8: 4BFEFD69  bl 0x830cec10
	ctx.lr = 0x830DEEAC;
	sub_830CEC10(ctx, base);
	// 830DEEAC: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830DEEB0: 409A000C  bne cr6, 0x830deebc
	if !ctx.cr[6].eq {
	pc = 0x830DEEBC; continue 'dispatch;
	}
	// 830DEEB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DEEB8: 48000040  b 0x830deef8
	pc = 0x830DEEF8; continue 'dispatch;
	// 830DEEBC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DEEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DEEC4: 4BFFFF75  bl 0x830dee38
	ctx.lr = 0x830DEEC8;
	sub_830DEE38(ctx, base);
	// 830DEEC8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DEECC: 41820014  beq 0x830deee0
	if ctx.cr[0].eq {
	pc = 0x830DEEE0; continue 'dispatch;
	}
	// 830DEED0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DEED4: 4BFEFD3D  bl 0x830cec10
	ctx.lr = 0x830DEED8;
	sub_830CEC10(ctx, base);
	// 830DEED8: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DEEDC: 419AFFD8  beq cr6, 0x830deeb4
	if ctx.cr[6].eq {
	pc = 0x830DEEB4; continue 'dispatch;
	}
	// 830DEEE0: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 830DEEE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830DEEE8: 557F063E  clrlwi r31, r11, 0x18
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830DEEEC: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 830DEEF0: 4198FFD0  blt cr6, 0x830deec0
	if ctx.cr[6].lt {
	pc = 0x830DEEC0; continue 'dispatch;
	}
	// 830DEEF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DEEF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DEEFC: 480C92C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEF00 size=128
    let mut pc: u32 = 0x830DEF00;
    'dispatch: loop {
        match pc {
            0x830DEF00 => {
    //   block [0x830DEF00..0x830DEF80)
	// 830DEF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEF04: 480C9269  bl 0x831a816c
	ctx.lr = 0x830DEF08;
	sub_831A8130(ctx, base);
	// 830DEF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEF0C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEF10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DEF14: 83EBD848  lwz r31, -0x27b8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10168 as u32) ) } as u64;
	// 830DEF18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DEF1C: 419A0018  beq cr6, 0x830def34
	if ctx.cr[6].eq {
	pc = 0x830DEF34; continue 'dispatch;
	}
	// 830DEF20: 4BFEFCF1  bl 0x830cec10
	ctx.lr = 0x830DEF24;
	sub_830CEC10(ctx, base);
	// 830DEF24: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DEF28: 409A000C  bne cr6, 0x830def34
	if !ctx.cr[6].eq {
	pc = 0x830DEF34; continue 'dispatch;
	}
	// 830DEF2C: 386000FF  li r3, 0xff
	ctx.r[3].s64 = 255;
	// 830DEF30: 48000040  b 0x830def70
	pc = 0x830DEF70; continue 'dispatch;
	// 830DEF34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830DEF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DEF3C: 4BFFFEFD  bl 0x830dee38
	ctx.lr = 0x830DEF40;
	sub_830DEE38(ctx, base);
	// 830DEF40: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DEF44: 41820014  beq 0x830def58
	if ctx.cr[0].eq {
	pc = 0x830DEF58; continue 'dispatch;
	}
	// 830DEF48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830DEF4C: 4BFEFCC5  bl 0x830cec10
	ctx.lr = 0x830DEF50;
	sub_830CEC10(ctx, base);
	// 830DEF50: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830DEF54: 419A0024  beq cr6, 0x830def78
	if ctx.cr[6].eq {
	pc = 0x830DEF78; continue 'dispatch;
	}
	// 830DEF58: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 830DEF5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830DEF60: 557F063E  clrlwi r31, r11, 0x18
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830DEF64: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 830DEF68: 4198FFD0  blt cr6, 0x830def38
	if ctx.cr[6].lt {
	pc = 0x830DEF38; continue 'dispatch;
	}
	// 830DEF6C: 386000FE  li r3, 0xfe
	ctx.r[3].s64 = 254;
	// 830DEF70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DEF74: 480C9248  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830DEF78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830DEF7C: 4BFFFFF4  b 0x830def70
	pc = 0x830DEF70; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEF80 size=68
    let mut pc: u32 = 0x830DEF80;
    'dispatch: loop {
        match pc {
            0x830DEF80 => {
    //   block [0x830DEF80..0x830DEFC4)
	// 830DEF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEF88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEF8C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEF90: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DEF94: 4BFFFB55  bl 0x830deae8
	ctx.lr = 0x830DEF98;
	sub_830DEAE8(ctx, base);
	// 830DEF98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DEF9C: 40820010  bne 0x830defac
	if !ctx.cr[0].eq {
	pc = 0x830DEFAC; continue 'dispatch;
	}
	// 830DEFA0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DEFA4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DEFA8: 4800000C  b 0x830defb4
	pc = 0x830DEFB4; continue 'dispatch;
	// 830DEFAC: 480069E5  bl 0x830e5990
	ctx.lr = 0x830DEFB0;
	sub_830E5990(ctx, base);
	// 830DEFB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DEFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DEFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DEFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DEFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DEFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DEFC8 size=64
    let mut pc: u32 = 0x830DEFC8;
    'dispatch: loop {
        match pc {
            0x830DEFC8 => {
    //   block [0x830DEFC8..0x830DF008)
	// 830DEFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DEFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DEFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DEFD4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DEFD8: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DEFDC: 4BFFFB0D  bl 0x830deae8
	ctx.lr = 0x830DEFE0;
	sub_830DEAE8(ctx, base);
	// 830DEFE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DEFE4: 40820010  bne 0x830deff4
	if !ctx.cr[0].eq {
	pc = 0x830DEFF4; continue 'dispatch;
	}
	// 830DEFE8: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DEFEC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DEFF0: 48000008  b 0x830deff8
	pc = 0x830DEFF8; continue 'dispatch;
	// 830DEFF4: 4800B695  bl 0x830ea688
	ctx.lr = 0x830DEFF8;
	sub_830EA688(ctx, base);
	// 830DEFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DEFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF008 size=96
    let mut pc: u32 = 0x830DF008;
    'dispatch: loop {
        match pc {
            0x830DF008 => {
    //   block [0x830DF008..0x830DF068)
	// 830DF008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF014: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF018: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DF01C: 4BFFFACD  bl 0x830deae8
	ctx.lr = 0x830DF020;
	sub_830DEAE8(ctx, base);
	// 830DF020: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF024: 40820010  bne 0x830df034
	if !ctx.cr[0].eq {
	pc = 0x830DF034; continue 'dispatch;
	}
	// 830DF028: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF02C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF030: 48000028  b 0x830df058
	pc = 0x830DF058; continue 'dispatch;
	// 830DF034: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 830DF038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF03C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DF040: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DF044: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF048: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DF04C: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DF050: 48006999  bl 0x830e59e8
	ctx.lr = 0x830DF054;
	sub_830E59E8(ctx, base);
	// 830DF054: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DF05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF068 size=116
    let mut pc: u32 = 0x830DF068;
    'dispatch: loop {
        match pc {
            0x830DF068 => {
    //   block [0x830DF068..0x830DF0DC)
	// 830DF068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF074: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF078: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF07C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF080: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DF084: 4BFFFA65  bl 0x830deae8
	ctx.lr = 0x830DF088;
	sub_830DEAE8(ctx, base);
	// 830DF088: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF08C: 40820010  bne 0x830df09c
	if !ctx.cr[0].eq {
	pc = 0x830DF09C; continue 'dispatch;
	}
	// 830DF090: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF094: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF098: 48000030  b 0x830df0c8
	pc = 0x830DF0C8; continue 'dispatch;
	// 830DF09C: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 830DF0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF0A4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 830DF0A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DF0AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF0B0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830DF0B4: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830DF0B8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 830DF0BC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 830DF0C0: 48006929  bl 0x830e59e8
	ctx.lr = 0x830DF0C4;
	sub_830E59E8(ctx, base);
	// 830DF0C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF0C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DF0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF0E0 size=60
    let mut pc: u32 = 0x830DF0E0;
    'dispatch: loop {
        match pc {
            0x830DF0E0 => {
    //   block [0x830DF0E0..0x830DF11C)
	// 830DF0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF0EC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF0F0: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DF0F4: 4BFFF9F5  bl 0x830deae8
	ctx.lr = 0x830DF0F8;
	sub_830DEAE8(ctx, base);
	// 830DF0F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF0FC: 4082000C  bne 0x830df108
	if !ctx.cr[0].eq {
	pc = 0x830DF108; continue 'dispatch;
	}
	// 830DF100: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830DF104: 48000008  b 0x830df10c
	pc = 0x830DF10C; continue 'dispatch;
	// 830DF108: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830DF10C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF120 size=80
    let mut pc: u32 = 0x830DF120;
    'dispatch: loop {
        match pc {
            0x830DF120 => {
    //   block [0x830DF120..0x830DF170)
	// 830DF120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF12C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF130: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF134: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF138: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DF13C: 4BFFF9AD  bl 0x830deae8
	ctx.lr = 0x830DF140;
	sub_830DEAE8(ctx, base);
	// 830DF140: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF144: 40820010  bne 0x830df154
	if !ctx.cr[0].eq {
	pc = 0x830DF154; continue 'dispatch;
	}
	// 830DF148: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF14C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF150: 4800000C  b 0x830df15c
	pc = 0x830DF15C; continue 'dispatch;
	// 830DF154: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 830DF158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF15C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF170 size=56
    let mut pc: u32 = 0x830DF170;
    'dispatch: loop {
        match pc {
            0x830DF170 => {
    //   block [0x830DF170..0x830DF1A8)
	// 830DF170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF17C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF180: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830DF184: 4BFFF965  bl 0x830deae8
	ctx.lr = 0x830DF188;
	sub_830DEAE8(ctx, base);
	// 830DF188: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF18C: 4182000C  beq 0x830df198
	if ctx.cr[0].eq {
	pc = 0x830DF198; continue 'dispatch;
	}
	// 830DF190: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830DF194: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830DF198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830DF1A8 size=100
    let mut pc: u32 = 0x830DF1A8;
    'dispatch: loop {
        match pc {
            0x830DF1A8 => {
    //   block [0x830DF1A8..0x830DF20C)
	// 830DF1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF1B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF1B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF1B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF1BC: 808BC09C  lwz r4, -0x3f64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16228 as u32) ) } as u64;
	// 830DF1C0: 4BFFF929  bl 0x830deae8
	ctx.lr = 0x830DF1C4;
	sub_830DEAE8(ctx, base);
	// 830DF1C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF1C8: 41820030  beq 0x830df1f8
	if ctx.cr[0].eq {
	pc = 0x830DF1F8; continue 'dispatch;
	}
	// 830DF1CC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830DF1D0: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 830DF1D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DF1D8: 409A001C  bne cr6, 0x830df1f4
	if !ctx.cr[6].eq {
	pc = 0x830DF1F4; continue 'dispatch;
	}
	// 830DF1DC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 830DF1E0: 80A30018  lwz r5, 0x18(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830DF1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830DF1E8: C023000C  lfs f1, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830DF1EC: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830DF1F0: 4802B549  bl 0x8310a738
	ctx.lr = 0x830DF1F4;
	sub_8310A738(ctx, base);
	// 830DF1F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DF1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF210 size=88
    let mut pc: u32 = 0x830DF210;
    'dispatch: loop {
        match pc {
            0x830DF210 => {
    //   block [0x830DF210..0x830DF268)
	// 830DF210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF21C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF220: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF224: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF228: 808BC09C  lwz r4, -0x3f64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16228 as u32) ) } as u64;
	// 830DF22C: 4BFFF8BD  bl 0x830deae8
	ctx.lr = 0x830DF230;
	sub_830DEAE8(ctx, base);
	// 830DF230: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF234: 40820010  bne 0x830df244
	if !ctx.cr[0].eq {
	pc = 0x830DF244; continue 'dispatch;
	}
	// 830DF238: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF23C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF240: 48000014  b 0x830df254
	pc = 0x830DF254; continue 'dispatch;
	// 830DF244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DF248: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830DF24C: 4BFEE4E5  bl 0x830cd730
	ctx.lr = 0x830DF250;
	sub_830CD730(ctx, base);
	// 830DF250: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF268 size=52
    let mut pc: u32 = 0x830DF268;
    'dispatch: loop {
        match pc {
            0x830DF268 => {
    //   block [0x830DF268..0x830DF29C)
	// 830DF268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF274: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF278: 808BC09C  lwz r4, -0x3f64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16228 as u32) ) } as u64;
	// 830DF27C: 4BFFF86D  bl 0x830deae8
	ctx.lr = 0x830DF280;
	sub_830DEAE8(ctx, base);
	// 830DF280: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF284: 41820008  beq 0x830df28c
	if ctx.cr[0].eq {
	pc = 0x830DF28C; continue 'dispatch;
	}
	// 830DF288: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830DF28C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF2A0 size=52
    let mut pc: u32 = 0x830DF2A0;
    'dispatch: loop {
        match pc {
            0x830DF2A0 => {
    //   block [0x830DF2A0..0x830DF2D4)
	// 830DF2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF2A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF2AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF2B0: 808BC09C  lwz r4, -0x3f64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16228 as u32) ) } as u64;
	// 830DF2B4: 4BFFF835  bl 0x830deae8
	ctx.lr = 0x830DF2B8;
	sub_830DEAE8(ctx, base);
	// 830DF2B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF2BC: 41820008  beq 0x830df2c4
	if ctx.cr[0].eq {
	pc = 0x830DF2C4; continue 'dispatch;
	}
	// 830DF2C0: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830DF2C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF2D8 size=104
    let mut pc: u32 = 0x830DF2D8;
    'dispatch: loop {
        match pc {
            0x830DF2D8 => {
    //   block [0x830DF2D8..0x830DF340)
	// 830DF2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF2E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF2E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF2E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF2EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DF2F0: 409A0010  bne cr6, 0x830df300
	if !ctx.cr[6].eq {
	pc = 0x830DF300; continue 'dispatch;
	}
	// 830DF2F4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DF2F8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DF2FC: 48000030  b 0x830df32c
	pc = 0x830DF32C; continue 'dispatch;
	// 830DF300: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF304: 808BC0AC  lwz r4, -0x3f54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16212 as u32) ) } as u64;
	// 830DF308: 4BFFF7E1  bl 0x830deae8
	ctx.lr = 0x830DF30C;
	sub_830DEAE8(ctx, base);
	// 830DF30C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF310: 40820010  bne 0x830df320
	if !ctx.cr[0].eq {
	pc = 0x830DF320; continue 'dispatch;
	}
	// 830DF314: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF318: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF31C: 48000010  b 0x830df32c
	pc = 0x830DF32C; continue 'dispatch;
	// 830DF320: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DF324: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF328: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF32C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF340 size=52
    let mut pc: u32 = 0x830DF340;
    'dispatch: loop {
        match pc {
            0x830DF340 => {
    //   block [0x830DF340..0x830DF374)
	// 830DF340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF34C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF350: 808BC088  lwz r4, -0x3f78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16248 as u32) ) } as u64;
	// 830DF354: 4BFEF81D  bl 0x830ceb70
	ctx.lr = 0x830DF358;
	sub_830CEB70(ctx, base);
	// 830DF358: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830DF35C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830DF360: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830DF364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF378 size=52
    let mut pc: u32 = 0x830DF378;
    'dispatch: loop {
        match pc {
            0x830DF378 => {
    //   block [0x830DF378..0x830DF3AC)
	// 830DF378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF384: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF388: 808BC08C  lwz r4, -0x3f74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16244 as u32) ) } as u64;
	// 830DF38C: 4BFEF7E5  bl 0x830ceb70
	ctx.lr = 0x830DF390;
	sub_830CEB70(ctx, base);
	// 830DF390: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830DF394: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830DF398: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830DF39C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF3B0 size=52
    let mut pc: u32 = 0x830DF3B0;
    'dispatch: loop {
        match pc {
            0x830DF3B0 => {
    //   block [0x830DF3B0..0x830DF3E4)
	// 830DF3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF3BC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF3C0: 808BC088  lwz r4, -0x3f78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16248 as u32) ) } as u64;
	// 830DF3C4: 4BFFF725  bl 0x830deae8
	ctx.lr = 0x830DF3C8;
	sub_830DEAE8(ctx, base);
	// 830DF3C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF3CC: 41820008  beq 0x830df3d4
	if ctx.cr[0].eq {
	pc = 0x830DF3D4; continue 'dispatch;
	}
	// 830DF3D0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DF3D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF3E8 size=56
    let mut pc: u32 = 0x830DF3E8;
    'dispatch: loop {
        match pc {
            0x830DF3E8 => {
    //   block [0x830DF3E8..0x830DF420)
	// 830DF3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF3F4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF3F8: 808BC088  lwz r4, -0x3f78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16248 as u32) ) } as u64;
	// 830DF3FC: 4BFFF6ED  bl 0x830deae8
	ctx.lr = 0x830DF400;
	sub_830DEAE8(ctx, base);
	// 830DF400: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF404: 4182000C  beq 0x830df410
	if ctx.cr[0].eq {
	pc = 0x830DF410; continue 'dispatch;
	}
	// 830DF408: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DF40C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830DF410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF420 size=144
    let mut pc: u32 = 0x830DF420;
    'dispatch: loop {
        match pc {
            0x830DF420 => {
    //   block [0x830DF420..0x830DF4B0)
	// 830DF420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DF42C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF438: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830DF43C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DF440: 419A0050  beq cr6, 0x830df490
	if ctx.cr[6].eq {
	pc = 0x830DF490; continue 'dispatch;
	}
	// 830DF444: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DF448: 419A0048  beq cr6, 0x830df490
	if ctx.cr[6].eq {
	pc = 0x830DF490; continue 'dispatch;
	}
	// 830DF44C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF450: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DF454: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF458: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF45C: 808AC088  lwz r4, -0x3f78(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16248 as u32) ) } as u64;
	// 830DF460: 4BFFF689  bl 0x830deae8
	ctx.lr = 0x830DF464;
	sub_830DEAE8(ctx, base);
	// 830DF464: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830DF468: 40820010  bne 0x830df478
	if !ctx.cr[0].eq {
	pc = 0x830DF478; continue 'dispatch;
	}
	// 830DF46C: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF470: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF474: 48000024  b 0x830df498
	pc = 0x830DF498; continue 'dispatch;
	// 830DF478: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830DF47C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF480: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830DF484: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830DF488: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF48C: 4800000C  b 0x830df498
	pc = 0x830DF498; continue 'dispatch;
	// 830DF490: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DF494: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DF498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DF49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF4A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DF4A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DF4B0 size=12
    let mut pc: u32 = 0x830DF4B0;
    'dispatch: loop {
        match pc {
            0x830DF4B0 => {
    //   block [0x830DF4B0..0x830DF4BC)
	// 830DF4B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF4B4: 806BD7FC  lwz r3, -0x2804(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10244 as u32) ) } as u64;
	// 830DF4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF4C0 size=104
    let mut pc: u32 = 0x830DF4C0;
    'dispatch: loop {
        match pc {
            0x830DF4C0 => {
    //   block [0x830DF4C0..0x830DF528)
	// 830DF4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF4C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF4CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF4D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF4D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DF4D8: 409A0010  bne cr6, 0x830df4e8
	if !ctx.cr[6].eq {
	pc = 0x830DF4E8; continue 'dispatch;
	}
	// 830DF4DC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DF4E0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DF4E4: 48000030  b 0x830df514
	pc = 0x830DF514; continue 'dispatch;
	// 830DF4E8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF4EC: 808BC0B0  lwz r4, -0x3f50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16208 as u32) ) } as u64;
	// 830DF4F0: 4BFFF5F9  bl 0x830deae8
	ctx.lr = 0x830DF4F4;
	sub_830DEAE8(ctx, base);
	// 830DF4F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF4F8: 40820010  bne 0x830df508
	if !ctx.cr[0].eq {
	pc = 0x830DF508; continue 'dispatch;
	}
	// 830DF4FC: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF500: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF504: 48000010  b 0x830df514
	pc = 0x830DF514; continue 'dispatch;
	// 830DF508: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DF50C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF510: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF528 size=104
    let mut pc: u32 = 0x830DF528;
    'dispatch: loop {
        match pc {
            0x830DF528 => {
    //   block [0x830DF528..0x830DF590)
	// 830DF528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF538: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF53C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DF540: 409A0010  bne cr6, 0x830df550
	if !ctx.cr[6].eq {
	pc = 0x830DF550; continue 'dispatch;
	}
	// 830DF544: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DF548: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DF54C: 48000030  b 0x830df57c
	pc = 0x830DF57C; continue 'dispatch;
	// 830DF550: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF554: 808BC0B4  lwz r4, -0x3f4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16204 as u32) ) } as u64;
	// 830DF558: 4BFFF591  bl 0x830deae8
	ctx.lr = 0x830DF55C;
	sub_830DEAE8(ctx, base);
	// 830DF55C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF560: 40820010  bne 0x830df570
	if !ctx.cr[0].eq {
	pc = 0x830DF570; continue 'dispatch;
	}
	// 830DF564: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF568: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF56C: 48000010  b 0x830df57c
	pc = 0x830DF57C; continue 'dispatch;
	// 830DF570: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DF574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF578: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF57C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF590 size=84
    let mut pc: u32 = 0x830DF590;
    'dispatch: loop {
        match pc {
            0x830DF590 => {
    //   block [0x830DF590..0x830DF5E4)
	// 830DF590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DF59C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF5A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF5A4: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 830DF5A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830DF5AC: 807F56F8  lwz r3, 0x56f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22264 as u32) ) } as u64;
	// 830DF5B0: 481636FD  bl 0x83242cac
	ctx.lr = 0x830DF5B4;
	// extern call 0x83242CAC → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 830DF5B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830DF5B8: 38830001  addi r4, r3, 1
	ctx.r[4].s64 = ctx.r[3].s64 + 1;
	// 830DF5BC: 409A0008  bne cr6, 0x830df5c4
	if !ctx.cr[6].eq {
	pc = 0x830DF5C4; continue 'dispatch;
	}
	// 830DF5C0: 3883FFFF  addi r4, r3, -1
	ctx.r[4].s64 = ctx.r[3].s64 + -1;
	// 830DF5C4: 807F56F8  lwz r3, 0x56f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22264 as u32) ) } as u64;
	// 830DF5C8: 481636F5  bl 0x83242cbc
	ctx.lr = 0x830DF5CC;
	// extern call 0x83242CBC → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 830DF5CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DF5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF5D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DF5DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830DF5E8 size=52
    let mut pc: u32 = 0x830DF5E8;
    'dispatch: loop {
        match pc {
            0x830DF5E8 => {
    //   block [0x830DF5E8..0x830DF61C)
	// 830DF5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF5F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF5F4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830DF5F8: 806B56F8  lwz r3, 0x56f8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22264 as u32) ) } as u64;
	// 830DF5FC: 481636B1  bl 0x83242cac
	ctx.lr = 0x830DF600;
	// extern call 0x83242CAC → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 830DF600: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 830DF604: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 830DF608: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830DF60C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF620 size=84
    let mut pc: u32 = 0x830DF620;
    'dispatch: loop {
        match pc {
            0x830DF620 => {
    //   block [0x830DF620..0x830DF674)
	// 830DF620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF62C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF630: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF634: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF638: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF63C: 4BFFF4AD  bl 0x830deae8
	ctx.lr = 0x830DF640;
	sub_830DEAE8(ctx, base);
	// 830DF640: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF644: 40820010  bne 0x830df654
	if !ctx.cr[0].eq {
	pc = 0x830DF654; continue 'dispatch;
	}
	// 830DF648: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF64C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF650: 48000010  b 0x830df660
	pc = 0x830DF660; continue 'dispatch;
	// 830DF654: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DF658: 4BFFF421  bl 0x830dea78
	ctx.lr = 0x830DF65C;
	sub_830DEA78(ctx, base);
	// 830DF65C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF66C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF678 size=68
    let mut pc: u32 = 0x830DF678;
    'dispatch: loop {
        match pc {
            0x830DF678 => {
    //   block [0x830DF678..0x830DF6BC)
	// 830DF678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF684: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF688: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF68C: 4BFFF45D  bl 0x830deae8
	ctx.lr = 0x830DF690;
	sub_830DEAE8(ctx, base);
	// 830DF690: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF694: 41820014  beq 0x830df6a8
	if ctx.cr[0].eq {
	pc = 0x830DF6A8; continue 'dispatch;
	}
	// 830DF698: 816300F0  lwz r11, 0xf0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 830DF69C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 830DF6A0: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 830DF6A4: 48000008  b 0x830df6ac
	pc = 0x830DF6AC; continue 'dispatch;
	// 830DF6A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF6AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF6C0 size=80
    let mut pc: u32 = 0x830DF6C0;
    'dispatch: loop {
        match pc {
            0x830DF6C0 => {
    //   block [0x830DF6C0..0x830DF710)
	// 830DF6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF6C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF6CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF6D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF6D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF6D8: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF6DC: 4BFFF40D  bl 0x830deae8
	ctx.lr = 0x830DF6E0;
	sub_830DEAE8(ctx, base);
	// 830DF6E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF6E4: 40820010  bne 0x830df6f4
	if !ctx.cr[0].eq {
	pc = 0x830DF6F4; continue 'dispatch;
	}
	// 830DF6E8: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF6EC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF6F0: 4800000C  b 0x830df6fc
	pc = 0x830DF6FC; continue 'dispatch;
	// 830DF6F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DF6F8: 48016BF1  bl 0x830f62e8
	ctx.lr = 0x830DF6FC;
	sub_830F62E8(ctx, base);
	// 830DF6FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF710 size=68
    let mut pc: u32 = 0x830DF710;
    'dispatch: loop {
        match pc {
            0x830DF710 => {
    //   block [0x830DF710..0x830DF754)
	// 830DF710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF71C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF720: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF724: 4BFFF3C5  bl 0x830deae8
	ctx.lr = 0x830DF728;
	sub_830DEAE8(ctx, base);
	// 830DF728: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF72C: 40820010  bne 0x830df73c
	if !ctx.cr[0].eq {
	pc = 0x830DF73C; continue 'dispatch;
	}
	// 830DF730: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF734: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF738: 4800000C  b 0x830df744
	pc = 0x830DF744; continue 'dispatch;
	// 830DF73C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830DF740: 48016991  bl 0x830f60d0
	ctx.lr = 0x830DF744;
	sub_830F60D0(ctx, base);
	// 830DF744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF758 size=116
    let mut pc: u32 = 0x830DF758;
    'dispatch: loop {
        match pc {
            0x830DF758 => {
    //   block [0x830DF758..0x830DF7CC)
	// 830DF758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF768: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF76C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF770: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DF774: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF778: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF77C: 4BFFF36D  bl 0x830deae8
	ctx.lr = 0x830DF780;
	sub_830DEAE8(ctx, base);
	// 830DF780: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF784: 40820010  bne 0x830df794
	if !ctx.cr[0].eq {
	pc = 0x830DF794; continue 'dispatch;
	}
	// 830DF788: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF78C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF790: 48000028  b 0x830df7b8
	pc = 0x830DF7B8; continue 'dispatch;
	// 830DF794: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830DF798: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DF79C: 419A0010  beq cr6, 0x830df7ac
	if ctx.cr[6].eq {
	pc = 0x830DF7AC; continue 'dispatch;
	}
	// 830DF7A0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DF7A4: 4BFEF46D  bl 0x830cec10
	ctx.lr = 0x830DF7A8;
	sub_830CEC10(ctx, base);
	// 830DF7A8: 48000008  b 0x830df7b0
	pc = 0x830DF7B0; continue 'dispatch;
	// 830DF7AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF7B0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DF7B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF7D0 size=116
    let mut pc: u32 = 0x830DF7D0;
    'dispatch: loop {
        match pc {
            0x830DF7D0 => {
    //   block [0x830DF7D0..0x830DF844)
	// 830DF7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF7E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF7E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF7E8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DF7EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF7F0: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF7F4: 4BFFF2F5  bl 0x830deae8
	ctx.lr = 0x830DF7F8;
	sub_830DEAE8(ctx, base);
	// 830DF7F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF7FC: 40820010  bne 0x830df80c
	if !ctx.cr[0].eq {
	pc = 0x830DF80C; continue 'dispatch;
	}
	// 830DF800: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF804: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF808: 48000028  b 0x830df830
	pc = 0x830DF830; continue 'dispatch;
	// 830DF80C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830DF810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DF814: 419A0010  beq cr6, 0x830df824
	if ctx.cr[6].eq {
	pc = 0x830DF824; continue 'dispatch;
	}
	// 830DF818: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DF81C: 4BFEF3F5  bl 0x830cec10
	ctx.lr = 0x830DF820;
	sub_830CEC10(ctx, base);
	// 830DF820: 48000008  b 0x830df828
	pc = 0x830DF828; continue 'dispatch;
	// 830DF824: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF828: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DF82C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF83C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF848 size=116
    let mut pc: u32 = 0x830DF848;
    'dispatch: loop {
        match pc {
            0x830DF848 => {
    //   block [0x830DF848..0x830DF8BC)
	// 830DF848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF858: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF85C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF860: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DF864: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF868: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF86C: 4BFFF27D  bl 0x830deae8
	ctx.lr = 0x830DF870;
	sub_830DEAE8(ctx, base);
	// 830DF870: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF874: 40820010  bne 0x830df884
	if !ctx.cr[0].eq {
	pc = 0x830DF884; continue 'dispatch;
	}
	// 830DF878: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF87C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF880: 48000028  b 0x830df8a8
	pc = 0x830DF8A8; continue 'dispatch;
	// 830DF884: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830DF888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DF88C: 419A0010  beq cr6, 0x830df89c
	if ctx.cr[6].eq {
	pc = 0x830DF89C; continue 'dispatch;
	}
	// 830DF890: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DF894: 4BFEF37D  bl 0x830cec10
	ctx.lr = 0x830DF898;
	sub_830CEC10(ctx, base);
	// 830DF898: 48000008  b 0x830df8a0
	pc = 0x830DF8A0; continue 'dispatch;
	// 830DF89C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF8A0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DF8A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF8B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF8C0 size=116
    let mut pc: u32 = 0x830DF8C0;
    'dispatch: loop {
        match pc {
            0x830DF8C0 => {
    //   block [0x830DF8C0..0x830DF934)
	// 830DF8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF8C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF8CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF8D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF8D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF8D8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DF8DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF8E0: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF8E4: 4BFFF205  bl 0x830deae8
	ctx.lr = 0x830DF8E8;
	sub_830DEAE8(ctx, base);
	// 830DF8E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF8EC: 40820010  bne 0x830df8fc
	if !ctx.cr[0].eq {
	pc = 0x830DF8FC; continue 'dispatch;
	}
	// 830DF8F0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF8F4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF8F8: 48000028  b 0x830df920
	pc = 0x830DF920; continue 'dispatch;
	// 830DF8FC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830DF900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DF904: 419A0010  beq cr6, 0x830df914
	if ctx.cr[6].eq {
	pc = 0x830DF914; continue 'dispatch;
	}
	// 830DF908: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DF90C: 4BFEF305  bl 0x830cec10
	ctx.lr = 0x830DF910;
	sub_830CEC10(ctx, base);
	// 830DF910: 48000008  b 0x830df918
	pc = 0x830DF918; continue 'dispatch;
	// 830DF914: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF918: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DF91C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF92C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF938 size=116
    let mut pc: u32 = 0x830DF938;
    'dispatch: loop {
        match pc {
            0x830DF938 => {
    //   block [0x830DF938..0x830DF9AC)
	// 830DF938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DF940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DF944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF948: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF94C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DF950: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DF954: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DF958: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF95C: 4BFFF18D  bl 0x830deae8
	ctx.lr = 0x830DF960;
	sub_830DEAE8(ctx, base);
	// 830DF960: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF964: 40820010  bne 0x830df974
	if !ctx.cr[0].eq {
	pc = 0x830DF974; continue 'dispatch;
	}
	// 830DF968: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF96C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF970: 48000028  b 0x830df998
	pc = 0x830DF998; continue 'dispatch;
	// 830DF974: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 830DF978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DF97C: 419A0010  beq cr6, 0x830df98c
	if ctx.cr[6].eq {
	pc = 0x830DF98C; continue 'dispatch;
	}
	// 830DF980: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830DF984: 4BFEF28D  bl 0x830cec10
	ctx.lr = 0x830DF988;
	sub_830CEC10(ctx, base);
	// 830DF988: 48000008  b 0x830df990
	pc = 0x830DF990; continue 'dispatch;
	// 830DF98C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF990: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830DF994: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DF998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DF99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DF9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DF9A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DF9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DF9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DF9B0 size=80
    let mut pc: u32 = 0x830DF9B0;
    'dispatch: loop {
        match pc {
            0x830DF9B0 => {
    //   block [0x830DF9B0..0x830DFA00)
	// 830DF9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DF9B4: 480C87B9  bl 0x831a816c
	ctx.lr = 0x830DF9B8;
	sub_831A8130(ctx, base);
	// 830DF9B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DF9BC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DF9C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DF9C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830DF9C8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830DF9CC: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DF9D0: 4BFFF119  bl 0x830deae8
	ctx.lr = 0x830DF9D4;
	sub_830DEAE8(ctx, base);
	// 830DF9D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DF9D8: 40820010  bne 0x830df9e8
	if !ctx.cr[0].eq {
	pc = 0x830DF9E8; continue 'dispatch;
	}
	// 830DF9DC: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DF9E0: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DF9E4: 48000014  b 0x830df9f8
	pc = 0x830DF9F8; continue 'dispatch;
	// 830DF9E8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830DF9EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830DF9F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DF9F4: 48016A7D  bl 0x830f6470
	ctx.lr = 0x830DF9F8;
	sub_830F6470(ctx, base);
	// 830DF9F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DF9FC: 480C87C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFA00 size=80
    let mut pc: u32 = 0x830DFA00;
    'dispatch: loop {
        match pc {
            0x830DFA00 => {
    //   block [0x830DFA00..0x830DFA50)
	// 830DFA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFA0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFA10: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFA14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFA18: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFA1C: 4BFFF0CD  bl 0x830deae8
	ctx.lr = 0x830DFA20;
	sub_830DEAE8(ctx, base);
	// 830DFA20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFA24: 40820010  bne 0x830dfa34
	if !ctx.cr[0].eq {
	pc = 0x830DFA34; continue 'dispatch;
	}
	// 830DFA28: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFA2C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFA30: 4800000C  b 0x830dfa3c
	pc = 0x830DFA3C; continue 'dispatch;
	// 830DFA34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFA38: 48009489  bl 0x830e8ec0
	ctx.lr = 0x830DFA3C;
	sub_830E8EC0(ctx, base);
	// 830DFA3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFA48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFA50 size=80
    let mut pc: u32 = 0x830DFA50;
    'dispatch: loop {
        match pc {
            0x830DFA50 => {
    //   block [0x830DFA50..0x830DFAA0)
	// 830DFA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFA58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFA5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFA60: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFA64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFA68: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFA6C: 4BFFF07D  bl 0x830deae8
	ctx.lr = 0x830DFA70;
	sub_830DEAE8(ctx, base);
	// 830DFA70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFA74: 40820010  bne 0x830dfa84
	if !ctx.cr[0].eq {
	pc = 0x830DFA84; continue 'dispatch;
	}
	// 830DFA78: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFA7C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFA80: 4800000C  b 0x830dfa8c
	pc = 0x830DFA8C; continue 'dispatch;
	// 830DFA84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFA88: 480094F9  bl 0x830e8f80
	ctx.lr = 0x830DFA8C;
	sub_830E8F80(ctx, base);
	// 830DFA8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFA98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFAA0 size=80
    let mut pc: u32 = 0x830DFAA0;
    'dispatch: loop {
        match pc {
            0x830DFAA0 => {
    //   block [0x830DFAA0..0x830DFAF0)
	// 830DFAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFAA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFAAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFAB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFAB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFAB8: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFABC: 4BFFF02D  bl 0x830deae8
	ctx.lr = 0x830DFAC0;
	sub_830DEAE8(ctx, base);
	// 830DFAC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFAC4: 40820010  bne 0x830dfad4
	if !ctx.cr[0].eq {
	pc = 0x830DFAD4; continue 'dispatch;
	}
	// 830DFAC8: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFACC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFAD0: 4800000C  b 0x830dfadc
	pc = 0x830DFADC; continue 'dispatch;
	// 830DFAD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFAD8: 48016771  bl 0x830f6248
	ctx.lr = 0x830DFADC;
	sub_830F6248(ctx, base);
	// 830DFADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFAE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFAF0 size=80
    let mut pc: u32 = 0x830DFAF0;
    'dispatch: loop {
        match pc {
            0x830DFAF0 => {
    //   block [0x830DFAF0..0x830DFB40)
	// 830DFAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFB00: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFB04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFB08: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFB0C: 4BFFEFDD  bl 0x830deae8
	ctx.lr = 0x830DFB10;
	sub_830DEAE8(ctx, base);
	// 830DFB10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFB14: 40820010  bne 0x830dfb24
	if !ctx.cr[0].eq {
	pc = 0x830DFB24; continue 'dispatch;
	}
	// 830DFB18: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFB1C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFB20: 4800000C  b 0x830dfb2c
	pc = 0x830DFB2C; continue 'dispatch;
	// 830DFB24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFB28: 48009481  bl 0x830e8fa8
	ctx.lr = 0x830DFB2C;
	sub_830E8FA8(ctx, base);
	// 830DFB2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFB38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFB40 size=80
    let mut pc: u32 = 0x830DFB40;
    'dispatch: loop {
        match pc {
            0x830DFB40 => {
    //   block [0x830DFB40..0x830DFB90)
	// 830DFB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFB48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFB4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFB50: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFB54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFB58: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFB5C: 4BFFEF8D  bl 0x830deae8
	ctx.lr = 0x830DFB60;
	sub_830DEAE8(ctx, base);
	// 830DFB60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFB64: 40820010  bne 0x830dfb74
	if !ctx.cr[0].eq {
	pc = 0x830DFB74; continue 'dispatch;
	}
	// 830DFB68: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFB6C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFB70: 4800000C  b 0x830dfb7c
	pc = 0x830DFB7C; continue 'dispatch;
	// 830DFB74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFB78: 48005759  bl 0x830e52d0
	ctx.lr = 0x830DFB7C;
	sub_830E52D0(ctx, base);
	// 830DFB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFB90 size=80
    let mut pc: u32 = 0x830DFB90;
    'dispatch: loop {
        match pc {
            0x830DFB90 => {
    //   block [0x830DFB90..0x830DFBE0)
	// 830DFB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFB98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFB9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFBA0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFBA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFBA8: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFBAC: 4BFFEF3D  bl 0x830deae8
	ctx.lr = 0x830DFBB0;
	sub_830DEAE8(ctx, base);
	// 830DFBB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFBB4: 40820010  bne 0x830dfbc4
	if !ctx.cr[0].eq {
	pc = 0x830DFBC4; continue 'dispatch;
	}
	// 830DFBB8: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFBBC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFBC0: 4800000C  b 0x830dfbcc
	pc = 0x830DFBCC; continue 'dispatch;
	// 830DFBC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFBC8: 48009409  bl 0x830e8fd0
	ctx.lr = 0x830DFBCC;
	sub_830E8FD0(ctx, base);
	// 830DFBCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFBD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFBE0 size=80
    let mut pc: u32 = 0x830DFBE0;
    'dispatch: loop {
        match pc {
            0x830DFBE0 => {
    //   block [0x830DFBE0..0x830DFC30)
	// 830DFBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFBE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFBEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFBF0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFBF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFBF8: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFBFC: 4BFFEEED  bl 0x830deae8
	ctx.lr = 0x830DFC00;
	sub_830DEAE8(ctx, base);
	// 830DFC00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFC04: 40820010  bne 0x830dfc14
	if !ctx.cr[0].eq {
	pc = 0x830DFC14; continue 'dispatch;
	}
	// 830DFC08: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFC0C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFC10: 4800000C  b 0x830dfc1c
	pc = 0x830DFC1C; continue 'dispatch;
	// 830DFC14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFC18: 48009419  bl 0x830e9030
	ctx.lr = 0x830DFC1C;
	sub_830E9030(ctx, base);
	// 830DFC1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFC28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFC30 size=80
    let mut pc: u32 = 0x830DFC30;
    'dispatch: loop {
        match pc {
            0x830DFC30 => {
    //   block [0x830DFC30..0x830DFC80)
	// 830DFC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFC38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFC3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFC40: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFC44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFC48: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFC4C: 4BFFEE9D  bl 0x830deae8
	ctx.lr = 0x830DFC50;
	sub_830DEAE8(ctx, base);
	// 830DFC50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFC54: 40820010  bne 0x830dfc64
	if !ctx.cr[0].eq {
	pc = 0x830DFC64; continue 'dispatch;
	}
	// 830DFC58: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFC5C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFC60: 4800000C  b 0x830dfc6c
	pc = 0x830DFC6C; continue 'dispatch;
	// 830DFC64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFC68: 480093A1  bl 0x830e9008
	ctx.lr = 0x830DFC6C;
	sub_830E9008(ctx, base);
	// 830DFC6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFC78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFC80 size=92
    let mut pc: u32 = 0x830DFC80;
    'dispatch: loop {
        match pc {
            0x830DFC80 => {
    //   block [0x830DFC80..0x830DFCDC)
	// 830DFC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFC88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFC8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFC90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFC94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DFC98: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DFC9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFCA0: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFCA4: 4BFFEE45  bl 0x830deae8
	ctx.lr = 0x830DFCA8;
	sub_830DEAE8(ctx, base);
	// 830DFCA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFCAC: 40820010  bne 0x830dfcbc
	if !ctx.cr[0].eq {
	pc = 0x830DFCBC; continue 'dispatch;
	}
	// 830DFCB0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFCB4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFCB8: 48000010  b 0x830dfcc8
	pc = 0x830DFCC8; continue 'dispatch;
	// 830DFCBC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830DFCC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DFCC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFCD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFCE0 size=132
    let mut pc: u32 = 0x830DFCE0;
    'dispatch: loop {
        match pc {
            0x830DFCE0 => {
    //   block [0x830DFCE0..0x830DFD64)
	// 830DFCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFCE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DFCEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFCF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFCF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DFCF8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830DFCFC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DFD00: 419A0044  beq cr6, 0x830dfd44
	if ctx.cr[6].eq {
	pc = 0x830DFD44; continue 'dispatch;
	}
	// 830DFD04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DFD08: 419A003C  beq cr6, 0x830dfd44
	if ctx.cr[6].eq {
	pc = 0x830DFD44; continue 'dispatch;
	}
	// 830DFD0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DFD10: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DFD14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFD18: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFD1C: 4BFFEDCD  bl 0x830deae8
	ctx.lr = 0x830DFD20;
	sub_830DEAE8(ctx, base);
	// 830DFD20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFD24: 40820010  bne 0x830dfd34
	if !ctx.cr[0].eq {
	pc = 0x830DFD34; continue 'dispatch;
	}
	// 830DFD28: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFD2C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFD30: 4800001C  b 0x830dfd4c
	pc = 0x830DFD4C; continue 'dispatch;
	// 830DFD34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830DFD38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DFD3C: 48005835  bl 0x830e5570
	ctx.lr = 0x830DFD40;
	sub_830E5570(ctx, base);
	// 830DFD40: 4800000C  b 0x830dfd4c
	pc = 0x830DFD4C; continue 'dispatch;
	// 830DFD44: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DFD48: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DFD4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DFD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFD58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DFD5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFD68 size=132
    let mut pc: u32 = 0x830DFD68;
    'dispatch: loop {
        match pc {
            0x830DFD68 => {
    //   block [0x830DFD68..0x830DFDEC)
	// 830DFD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFD70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830DFD74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFD78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFD7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DFD80: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830DFD84: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830DFD88: 419A0044  beq cr6, 0x830dfdcc
	if ctx.cr[6].eq {
	pc = 0x830DFDCC; continue 'dispatch;
	}
	// 830DFD8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DFD90: 419A003C  beq cr6, 0x830dfdcc
	if ctx.cr[6].eq {
	pc = 0x830DFDCC; continue 'dispatch;
	}
	// 830DFD94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DFD98: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DFD9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFDA0: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFDA4: 4BFFED45  bl 0x830deae8
	ctx.lr = 0x830DFDA8;
	sub_830DEAE8(ctx, base);
	// 830DFDA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFDAC: 40820010  bne 0x830dfdbc
	if !ctx.cr[0].eq {
	pc = 0x830DFDBC; continue 'dispatch;
	}
	// 830DFDB0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFDB4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFDB8: 4800001C  b 0x830dfdd4
	pc = 0x830DFDD4; continue 'dispatch;
	// 830DFDBC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830DFDC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830DFDC4: 4800573D  bl 0x830e5500
	ctx.lr = 0x830DFDC8;
	sub_830E5500(ctx, base);
	// 830DFDC8: 4800000C  b 0x830dfdd4
	pc = 0x830DFDD4; continue 'dispatch;
	// 830DFDCC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DFDD0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DFDD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830DFDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFDE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830DFDE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFDF0 size=124
    let mut pc: u32 = 0x830DFDF0;
    'dispatch: loop {
        match pc {
            0x830DFDF0 => {
    //   block [0x830DFDF0..0x830DFE6C)
	// 830DFDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFDF4: 480C8379  bl 0x831a816c
	ctx.lr = 0x830DFDF8;
	sub_831A8130(ctx, base);
	// 830DFDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFDFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830DFE00: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830DFE04: 4BFEEE0D  bl 0x830cec10
	ctx.lr = 0x830DFE08;
	sub_830CEC10(ctx, base);
	// 830DFE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DFE0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830DFE10: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830DFE14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DFE18: 419A0008  beq cr6, 0x830dfe20
	if ctx.cr[6].eq {
	pc = 0x830DFE20; continue 'dispatch;
	}
	// 830DFE1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFE20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830DFE24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830DFE28: 4BFFF931  bl 0x830df758
	ctx.lr = 0x830DFE2C;
	sub_830DF758(ctx, base);
	// 830DFE2C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830DFE30: 4180001C  blt 0x830dfe4c
	if ctx.cr[0].lt {
	pc = 0x830DFE4C; continue 'dispatch;
	}
	// 830DFE34: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830DFE38: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830DFE3C: 419A001C  beq cr6, 0x830dfe58
	if ctx.cr[6].eq {
	pc = 0x830DFE58; continue 'dispatch;
	}
	// 830DFE40: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830DFE44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830DFE48: 409AFFD8  bne cr6, 0x830dfe20
	if !ctx.cr[6].eq {
	pc = 0x830DFE20; continue 'dispatch;
	}
	// 830DFE4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DFE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830DFE54: 480C8368  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830DFE58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DFE5C: 419A0008  beq cr6, 0x830dfe64
	if ctx.cr[6].eq {
	pc = 0x830DFE64; continue 'dispatch;
	}
	// 830DFE60: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830DFE64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830DFE68: 4BFFFFE8  b 0x830dfe50
	pc = 0x830DFE50; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830DFE70 size=8
    let mut pc: u32 = 0x830DFE70;
    'dispatch: loop {
        match pc {
            0x830DFE70 => {
    //   block [0x830DFE70..0x830DFE78)
	// 830DFE70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830DFE74: 4BFFFF7C  b 0x830dfdf0
	sub_830DFDF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFE78 size=112
    let mut pc: u32 = 0x830DFE78;
    'dispatch: loop {
        match pc {
            0x830DFE78 => {
    //   block [0x830DFE78..0x830DFEE8)
	// 830DFE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFE80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFE84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFE88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFE8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DFE90: 409A0010  bne cr6, 0x830dfea0
	if !ctx.cr[6].eq {
	pc = 0x830DFEA0; continue 'dispatch;
	}
	// 830DFE94: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DFE98: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DFE9C: 48000038  b 0x830dfed4
	pc = 0x830DFED4; continue 'dispatch;
	// 830DFEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830DFEA4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830DFEA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFEAC: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFEB0: 4BFFEC39  bl 0x830deae8
	ctx.lr = 0x830DFEB4;
	sub_830DEAE8(ctx, base);
	// 830DFEB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFEB8: 40820010  bne 0x830dfec8
	if !ctx.cr[0].eq {
	pc = 0x830DFEC8; continue 'dispatch;
	}
	// 830DFEBC: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFEC0: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFEC4: 48000010  b 0x830dfed4
	pc = 0x830DFED4; continue 'dispatch;
	// 830DFEC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830DFECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DFED0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFED4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFEE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFEE8 size=104
    let mut pc: u32 = 0x830DFEE8;
    'dispatch: loop {
        match pc {
            0x830DFEE8 => {
    //   block [0x830DFEE8..0x830DFF50)
	// 830DFEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFEF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFEF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFEF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFEFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830DFF00: 409A0010  bne cr6, 0x830dff10
	if !ctx.cr[6].eq {
	pc = 0x830DFF10; continue 'dispatch;
	}
	// 830DFF04: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830DFF08: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830DFF0C: 48000030  b 0x830dff3c
	pc = 0x830DFF3C; continue 'dispatch;
	// 830DFF10: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFF14: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFF18: 4BFFEBD1  bl 0x830deae8
	ctx.lr = 0x830DFF1C;
	sub_830DEAE8(ctx, base);
	// 830DFF1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFF20: 40820010  bne 0x830dff30
	if !ctx.cr[0].eq {
	pc = 0x830DFF30; continue 'dispatch;
	}
	// 830DFF24: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFF28: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFF2C: 48000010  b 0x830dff3c
	pc = 0x830DFF3C; continue 'dispatch;
	// 830DFF30: 816300E8  lwz r11, 0xe8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(232 as u32) ) } as u64;
	// 830DFF34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DFF38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830DFF3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFF48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFF50 size=80
    let mut pc: u32 = 0x830DFF50;
    'dispatch: loop {
        match pc {
            0x830DFF50 => {
    //   block [0x830DFF50..0x830DFFA0)
	// 830DFF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFF58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFF5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFF60: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFF64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFF68: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFF6C: 4BFFEB7D  bl 0x830deae8
	ctx.lr = 0x830DFF70;
	sub_830DEAE8(ctx, base);
	// 830DFF70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFF74: 40820010  bne 0x830dff84
	if !ctx.cr[0].eq {
	pc = 0x830DFF84; continue 'dispatch;
	}
	// 830DFF78: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFF7C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFF80: 4800000C  b 0x830dff8c
	pc = 0x830DFF8C; continue 'dispatch;
	// 830DFF84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFF88: 48005431  bl 0x830e53b8
	ctx.lr = 0x830DFF8C;
	sub_830E53B8(ctx, base);
	// 830DFF8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFF98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFFA0 size=84
    let mut pc: u32 = 0x830DFFA0;
    'dispatch: loop {
        match pc {
            0x830DFFA0 => {
    //   block [0x830DFFA0..0x830DFFF4)
	// 830DFFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830DFFA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830DFFAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830DFFB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830DFFB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830DFFB8: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830DFFBC: 4BFFEB2D  bl 0x830deae8
	ctx.lr = 0x830DFFC0;
	sub_830DEAE8(ctx, base);
	// 830DFFC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830DFFC4: 40820010  bne 0x830dffd4
	if !ctx.cr[0].eq {
	pc = 0x830DFFD4; continue 'dispatch;
	}
	// 830DFFC8: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830DFFCC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830DFFD0: 48000010  b 0x830dffe0
	pc = 0x830DFFE0; continue 'dispatch;
	// 830DFFD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830DFFD8: 48005911  bl 0x830e58e8
	ctx.lr = 0x830DFFDC;
	sub_830E58E8(ctx, base);
	// 830DFFDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830DFFE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830DFFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830DFFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830DFFEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830DFFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830DFFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830DFFF8 size=104
    let mut pc: u32 = 0x830DFFF8;
    'dispatch: loop {
        match pc {
            0x830DFFF8 => {
    //   block [0x830DFFF8..0x830E0060)
	// 830DFFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830DFFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0008: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E000C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E0010: 409A0010  bne cr6, 0x830e0020
	if !ctx.cr[6].eq {
	pc = 0x830E0020; continue 'dispatch;
	}
	// 830E0014: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E0018: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E001C: 48000030  b 0x830e004c
	pc = 0x830E004C; continue 'dispatch;
	// 830E0020: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E0024: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E0028: 4BFFEAC1  bl 0x830deae8
	ctx.lr = 0x830E002C;
	sub_830DEAE8(ctx, base);
	// 830E002C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E0030: 40820010  bne 0x830e0040
	if !ctx.cr[0].eq {
	pc = 0x830E0040; continue 'dispatch;
	}
	// 830E0034: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0038: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E003C: 48000010  b 0x830e004c
	pc = 0x830E004C; continue 'dispatch;
	// 830E0040: 480058B9  bl 0x830e58f8
	ctx.lr = 0x830E0044;
	sub_830E58F8(ctx, base);
	// 830E0044: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830E0048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E004C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E005C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0060 size=80
    let mut pc: u32 = 0x830E0060;
    'dispatch: loop {
        match pc {
            0x830E0060 => {
    //   block [0x830E0060..0x830E00B0)
	// 830E0060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E006C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0070: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E0074: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0078: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E007C: 4BFFEA6D  bl 0x830deae8
	ctx.lr = 0x830E0080;
	sub_830DEAE8(ctx, base);
	// 830E0080: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E0084: 40820010  bne 0x830e0094
	if !ctx.cr[0].eq {
	pc = 0x830E0094; continue 'dispatch;
	}
	// 830E0088: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E008C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E0090: 4800000C  b 0x830e009c
	pc = 0x830E009C; continue 'dispatch;
	// 830E0094: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0098: 48009201  bl 0x830e9298
	ctx.lr = 0x830E009C;
	sub_830E9298(ctx, base);
	// 830E009C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E00A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E00A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E00A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E00AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E00B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E00B0 size=88
    let mut pc: u32 = 0x830E00B0;
    'dispatch: loop {
        match pc {
            0x830E00B0 => {
    //   block [0x830E00B0..0x830E0108)
	// 830E00B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E00B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E00B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E00BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E00C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E00C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E00C8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830E00CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E00D0: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E00D4: 4BFFEA15  bl 0x830deae8
	ctx.lr = 0x830E00D8;
	sub_830DEAE8(ctx, base);
	// 830E00D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E00DC: 40820010  bne 0x830e00ec
	if !ctx.cr[0].eq {
	pc = 0x830E00EC; continue 'dispatch;
	}
	// 830E00E0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E00E4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E00E8: 4800000C  b 0x830e00f4
	pc = 0x830E00F4; continue 'dispatch;
	// 830E00EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E00F0: 48005369  bl 0x830e5458
	ctx.lr = 0x830E00F4;
	sub_830E5458(ctx, base);
	// 830E00F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E00F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E00FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0108 size=80
    let mut pc: u32 = 0x830E0108;
    'dispatch: loop {
        match pc {
            0x830E0108 => {
    //   block [0x830E0108..0x830E0158)
	// 830E0108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0118: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E011C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0120: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E0124: 4BFFE9C5  bl 0x830deae8
	ctx.lr = 0x830E0128;
	sub_830DEAE8(ctx, base);
	// 830E0128: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E012C: 40820010  bne 0x830e013c
	if !ctx.cr[0].eq {
	pc = 0x830E013C; continue 'dispatch;
	}
	// 830E0130: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0134: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E0138: 4800000C  b 0x830e0144
	pc = 0x830E0144; continue 'dispatch;
	// 830E013C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0140: 480091D1  bl 0x830e9310
	ctx.lr = 0x830E0144;
	sub_830E9310(ctx, base);
	// 830E0144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E014C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0158 size=96
    let mut pc: u32 = 0x830E0158;
    'dispatch: loop {
        match pc {
            0x830E0158 => {
    //   block [0x830E0158..0x830E01B8)
	// 830E0158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E015C: 480C8009  bl 0x831a8164
	ctx.lr = 0x830E0160;
	sub_831A8130(ctx, base);
	// 830E0160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0164: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E0168: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E016C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E0170: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E0174: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 830E0178: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E017C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 830E0180: 4BFFE969  bl 0x830deae8
	ctx.lr = 0x830E0184;
	sub_830DEAE8(ctx, base);
	// 830E0184: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E0188: 40820010  bne 0x830e0198
	if !ctx.cr[0].eq {
	pc = 0x830E0198; continue 'dispatch;
	}
	// 830E018C: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0190: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E0194: 4800001C  b 0x830e01b0
	pc = 0x830E01B0; continue 'dispatch;
	// 830E0198: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 830E019C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830E01A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830E01A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830E01A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E01AC: 4800921D  bl 0x830e93c8
	ctx.lr = 0x830E01B0;
	sub_830E93C8(ctx, base);
	// 830E01B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830E01B4: 480C8000  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E01B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E01B8 size=124
    let mut pc: u32 = 0x830E01B8;
    'dispatch: loop {
        match pc {
            0x830E01B8 => {
    //   block [0x830E01B8..0x830E0234)
	// 830E01B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E01BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E01C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E01C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E01C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E01CC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830E01D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E01D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E01D8: 409A0010  bne cr6, 0x830e01e8
	if !ctx.cr[6].eq {
	pc = 0x830E01E8; continue 'dispatch;
	}
	// 830E01DC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E01E0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E01E4: 48000038  b 0x830e021c
	pc = 0x830E021C; continue 'dispatch;
	// 830E01E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830E01EC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830E01F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E01F4: 808AC06C  lwz r4, -0x3f94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E01F8: 4BFFE8F1  bl 0x830deae8
	ctx.lr = 0x830E01FC;
	sub_830DEAE8(ctx, base);
	// 830E01FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E0200: 40820010  bne 0x830e0210
	if !ctx.cr[0].eq {
	pc = 0x830E0210; continue 'dispatch;
	}
	// 830E0204: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0208: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E020C: 48000010  b 0x830e021c
	pc = 0x830E021C; continue 'dispatch;
	// 830E0210: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830E0214: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830E0218: 48005251  bl 0x830e5468
	ctx.lr = 0x830E021C;
	sub_830E5468(ctx, base);
	// 830E021C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0228: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E022C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0238 size=104
    let mut pc: u32 = 0x830E0238;
    'dispatch: loop {
        match pc {
            0x830E0238 => {
    //   block [0x830E0238..0x830E02A0)
	// 830E0238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E023C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0248: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E024C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0250: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E0254: 4BFFE895  bl 0x830deae8
	ctx.lr = 0x830E0258;
	sub_830DEAE8(ctx, base);
	// 830E0258: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E025C: 40820010  bne 0x830e026c
	if !ctx.cr[0].eq {
	pc = 0x830E026C; continue 'dispatch;
	}
	// 830E0260: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0264: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E0268: 48000024  b 0x830e028c
	pc = 0x830E028C; continue 'dispatch;
	// 830E026C: 816300F0  lwz r11, 0xf0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 830E0270: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E0274: 419A000C  beq cr6, 0x830e0280
	if ctx.cr[6].eq {
	pc = 0x830E0280; continue 'dispatch;
	}
	// 830E0278: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 830E027C: 48000008  b 0x830e0284
	pc = 0x830E0284; continue 'dispatch;
	// 830E0280: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830E0284: 916300F0  stw r11, 0xf0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830E0288: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E028C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E029C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E02A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E02A0 size=84
    let mut pc: u32 = 0x830E02A0;
    'dispatch: loop {
        match pc {
            0x830E02A0 => {
    //   block [0x830E02A0..0x830E02F4)
	// 830E02A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E02A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E02A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E02AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E02B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E02B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830E02B8: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E02BC: 4BFFE82D  bl 0x830deae8
	ctx.lr = 0x830E02C0;
	sub_830DEAE8(ctx, base);
	// 830E02C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E02C4: 41820018  beq 0x830e02dc
	if ctx.cr[0].eq {
	pc = 0x830E02DC; continue 'dispatch;
	}
	// 830E02C8: 816300F0  lwz r11, 0xf0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 830E02CC: 556B04A4  rlwinm r11, r11, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830E02D0: 2B0B2000  cmplwi cr6, r11, 0x2000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8192 as u32, &mut ctx.xer);
	// 830E02D4: 409A0008  bne cr6, 0x830e02dc
	if !ctx.cr[6].eq {
	pc = 0x830E02DC; continue 'dispatch;
	}
	// 830E02D8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 830E02DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E02E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E02E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E02E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E02EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E02F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E02F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E02F8 size=100
    let mut pc: u32 = 0x830E02F8;
    'dispatch: loop {
        match pc {
            0x830E02F8 => {
    //   block [0x830E02F8..0x830E035C)
	// 830E02F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E02FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E030C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E0310: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830E0314: 4BFFE83D  bl 0x830deb50
	ctx.lr = 0x830E0318;
	sub_830DEB50(ctx, base);
	// 830E0318: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E031C: 40820010  bne 0x830e032c
	if !ctx.cr[0].eq {
	pc = 0x830E032C; continue 'dispatch;
	}
	// 830E0320: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0324: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830E0328: 4800001C  b 0x830e0344
	pc = 0x830E0344; continue 'dispatch;
	// 830E032C: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 830E0330: 409A000C  bne cr6, 0x830e033c
	if !ctx.cr[6].eq {
	pc = 0x830E033C; continue 'dispatch;
	}
	// 830E0334: 93E3004C  stw r31, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[31].u32 ) };
	// 830E0338: 48000008  b 0x830e0340
	pc = 0x830E0340; continue 'dispatch;
	// 830E033C: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 830E0340: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E0344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E034C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0350: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0360 size=156
    let mut pc: u32 = 0x830E0360;
    'dispatch: loop {
        match pc {
            0x830E0360 => {
    //   block [0x830E0360..0x830E03FC)
	// 830E0360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0364: 480C7E09  bl 0x831a816c
	ctx.lr = 0x830E0368;
	sub_831A8130(ctx, base);
	// 830E0368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E036C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830E0370: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830E0374: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830E0378: 419A0074  beq cr6, 0x830e03ec
	if ctx.cr[6].eq {
	pc = 0x830E03EC; continue 'dispatch;
	}
	// 830E037C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830E0380: 419A006C  beq cr6, 0x830e03ec
	if ctx.cr[6].eq {
	pc = 0x830E03EC; continue 'dispatch;
	}
	// 830E0384: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830E0388: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830E038C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830E0390: 4BFFD121  bl 0x830dd4b0
	ctx.lr = 0x830E0394;
	sub_830DD4B0(ctx, base);
	// 830E0394: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E0398: 4182000C  beq 0x830e03a4
	if ctx.cr[0].eq {
	pc = 0x830E03A4; continue 'dispatch;
	}
	// 830E039C: 480233CD  bl 0x83103768
	ctx.lr = 0x830E03A0;
	sub_83103768(ctx, base);
	// 830E03A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E03A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E03A8: 409A0010  bne cr6, 0x830e03b8
	if !ctx.cr[6].eq {
	pc = 0x830E03B8; continue 'dispatch;
	}
	// 830E03AC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E03B0: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830E03B4: 48000040  b 0x830e03f4
	pc = 0x830E03F4; continue 'dispatch;
	// 830E03B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830E03BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E03C0: 48023981  bl 0x83103d40
	ctx.lr = 0x830E03C4;
	sub_83103D40(ctx, base);
	// 830E03C4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830E03C8: 40800018  bge 0x830e03e0
	if !ctx.cr[0].lt {
	pc = 0x830E03E0; continue 'dispatch;
	}
	// 830E03CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E03D0: 48023969  bl 0x83103d38
	ctx.lr = 0x830E03D4;
	sub_83103D38(ctx, base);
	// 830E03D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E03D8: 4BFFD101  bl 0x830dd4d8
	ctx.lr = 0x830E03DC;
	sub_830DD4D8(ctx, base);
	// 830E03DC: 48000008  b 0x830e03e4
	pc = 0x830E03E4; continue 'dispatch;
	// 830E03E0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830E03E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E03E8: 4800000C  b 0x830e03f4
	pc = 0x830E03F4; continue 'dispatch;
	// 830E03EC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E03F0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E03F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E03F8: 480C7DC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0400 size=80
    let mut pc: u32 = 0x830E0400;
    'dispatch: loop {
        match pc {
            0x830E0400 => {
    //   block [0x830E0400..0x830E0450)
	// 830E0400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E040C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0410: 4B1DFBF1  bl 0x822c0000
	ctx.lr = 0x830E0414;
	sub_822C0000(ctx, base);
	// 830E0414: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E0418: 40820010  bne 0x830e0428
	if !ctx.cr[0].eq {
	pc = 0x830E0428; continue 'dispatch;
	}
	// 830E041C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E0420: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E0424: 48000018  b 0x830e043c
	pc = 0x830E043C; continue 'dispatch;
	// 830E0428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E042C: 4802390D  bl 0x83103d38
	ctx.lr = 0x830E0430;
	sub_83103D38(ctx, base);
	// 830E0430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0434: 4BFFD0A5  bl 0x830dd4d8
	ctx.lr = 0x830E0438;
	sub_830DD4D8(ctx, base);
	// 830E0438: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E043C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E044C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0450 size=96
    let mut pc: u32 = 0x830E0450;
    'dispatch: loop {
        match pc {
            0x830E0450 => {
    //   block [0x830E0450..0x830E04B0)
	// 830E0450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E045C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0464: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E0468: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E046C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E0470: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E0474: 4BFFE675  bl 0x830deae8
	ctx.lr = 0x830E0478;
	sub_830DEAE8(ctx, base);
	// 830E0478: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E047C: 40820010  bne 0x830e048c
	if !ctx.cr[0].eq {
	pc = 0x830E048C; continue 'dispatch;
	}
	// 830E0480: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E0484: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E0488: 48000010  b 0x830e0498
	pc = 0x830E0498; continue 'dispatch;
	// 830E048C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830E0490: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0494: 4801031D  bl 0x830f07b0
	ctx.lr = 0x830E0498;
	sub_830F07B0(ctx, base);
	// 830E0498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E04A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E04A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E04A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E04AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E04B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E04B0 size=12
    let mut pc: u32 = 0x830E04B0;
    'dispatch: loop {
        match pc {
            0x830E04B0 => {
    //   block [0x830E04B0..0x830E04BC)
	// 830E04B0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830E04B4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830E04B8: 48004D80  b 0x830e5238
	sub_830E5238(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E04C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E04C0 size=72
    let mut pc: u32 = 0x830E04C0;
    'dispatch: loop {
        match pc {
            0x830E04C0 => {
    //   block [0x830E04C0..0x830E0508)
	// 830E04C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E04C4: 480C7CA5  bl 0x831a8168
	ctx.lr = 0x830E04C8;
	sub_831A8130(ctx, base);
	// 830E04C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E04CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E04D0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 830E04D4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E04D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E04DC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E04E0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 830E04E4: 48021B0D  bl 0x83101ff0
	ctx.lr = 0x830E04E8;
	sub_83101FF0(ctx, base);
	// 830E04E8: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 830E04EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E04F0: 480235A9  bl 0x83103a98
	ctx.lr = 0x830E04F4;
	sub_83103A98(ctx, base);
	// 830E04F4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E04F8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830E04FC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830E0500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830E0504: 480C7CB4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0508 size=56
    let mut pc: u32 = 0x830E0508;
    'dispatch: loop {
        match pc {
            0x830E0508 => {
    //   block [0x830E0508..0x830E0540)
	// 830E0508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0514: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830E0518: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 830E051C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0520: 48021AD1  bl 0x83101ff0
	ctx.lr = 0x830E0524;
	sub_83101FF0(ctx, base);
	// 830E0524: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 830E0528: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 830E052C: 4802356D  bl 0x83103a98
	ctx.lr = 0x830E0530;
	sub_83103A98(ctx, base);
	// 830E0530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E053C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0540 size=64
    let mut pc: u32 = 0x830E0540;
    'dispatch: loop {
        match pc {
            0x830E0540 => {
    //   block [0x830E0540..0x830E0580)
	// 830E0540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0544: 480C7C29  bl 0x831a816c
	ctx.lr = 0x830E0548;
	sub_831A8130(ctx, base);
	// 830E0548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E054C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0550: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 830E0554: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0558: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E055C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E0560: 48021A91  bl 0x83101ff0
	ctx.lr = 0x830E0564;
	sub_83101FF0(ctx, base);
	// 830E0564: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 830E0568: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E056C: 4802352D  bl 0x83103a98
	ctx.lr = 0x830E0570;
	sub_83103A98(ctx, base);
	// 830E0570: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0574: 9BBF0004  stb r29, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 830E0578: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E057C: 480C7C40  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0580 size=64
    let mut pc: u32 = 0x830E0580;
    'dispatch: loop {
        match pc {
            0x830E0580 => {
    //   block [0x830E0580..0x830E05C0)
	// 830E0580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0584: 480C7BE9  bl 0x831a816c
	ctx.lr = 0x830E0588;
	sub_831A8130(ctx, base);
	// 830E0588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E058C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0590: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 830E0594: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0598: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E059C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E05A0: 48021A51  bl 0x83101ff0
	ctx.lr = 0x830E05A4;
	sub_83101FF0(ctx, base);
	// 830E05A4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 830E05A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E05AC: 480234ED  bl 0x83103a98
	ctx.lr = 0x830E05B0;
	sub_83103A98(ctx, base);
	// 830E05B0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E05B4: 9BBF0004  stb r29, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 830E05B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E05BC: 480C7C00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E05C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E05C0 size=56
    let mut pc: u32 = 0x830E05C0;
    'dispatch: loop {
        match pc {
            0x830E05C0 => {
    //   block [0x830E05C0..0x830E05F8)
	// 830E05C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E05C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E05C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E05CC: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830E05D0: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 830E05D4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E05D8: 48021A19  bl 0x83101ff0
	ctx.lr = 0x830E05DC;
	sub_83101FF0(ctx, base);
	// 830E05DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E05E0: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 830E05E4: 480234B5  bl 0x83103a98
	ctx.lr = 0x830E05E8;
	sub_83103A98(ctx, base);
	// 830E05E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E05EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E05F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E05F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E05F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E05F8 size=80
    let mut pc: u32 = 0x830E05F8;
    'dispatch: loop {
        match pc {
            0x830E05F8 => {
    //   block [0x830E05F8..0x830E0648)
	// 830E05F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E05FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E060C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0610: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 830E0614: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0618: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E061C: 480219D5  bl 0x83101ff0
	ctx.lr = 0x830E0620;
	sub_83101FF0(ctx, base);
	// 830E0620: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E0624: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0628: 48023471  bl 0x83103a98
	ctx.lr = 0x830E062C;
	sub_83103A98(ctx, base);
	// 830E062C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E063C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0648 size=80
    let mut pc: u32 = 0x830E0648;
    'dispatch: loop {
        match pc {
            0x830E0648 => {
    //   block [0x830E0648..0x830E0698)
	// 830E0648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E065C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0660: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 830E0664: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0668: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E066C: 48021985  bl 0x83101ff0
	ctx.lr = 0x830E0670;
	sub_83101FF0(ctx, base);
	// 830E0670: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E0674: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0678: 48023421  bl 0x83103a98
	ctx.lr = 0x830E067C;
	sub_83103A98(ctx, base);
	// 830E067C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E068C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0698 size=56
    let mut pc: u32 = 0x830E0698;
    'dispatch: loop {
        match pc {
            0x830E0698 => {
    //   block [0x830E0698..0x830E06D0)
	// 830E0698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E06A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E06A4: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830E06A8: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 830E06AC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E06B0: 48021941  bl 0x83101ff0
	ctx.lr = 0x830E06B4;
	sub_83101FF0(ctx, base);
	// 830E06B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E06B8: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 830E06BC: 480233DD  bl 0x83103a98
	ctx.lr = 0x830E06C0;
	sub_83103A98(ctx, base);
	// 830E06C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E06C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E06C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E06CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E06D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E06D0 size=56
    let mut pc: u32 = 0x830E06D0;
    'dispatch: loop {
        match pc {
            0x830E06D0 => {
    //   block [0x830E06D0..0x830E0708)
	// 830E06D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E06D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E06D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E06DC: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830E06E0: 38800806  li r4, 0x806
	ctx.r[4].s64 = 2054;
	// 830E06E4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E06E8: 48021909  bl 0x83101ff0
	ctx.lr = 0x830E06EC;
	sub_83101FF0(ctx, base);
	// 830E06EC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E06F0: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 830E06F4: 480233A5  bl 0x83103a98
	ctx.lr = 0x830E06F8;
	sub_83103A98(ctx, base);
	// 830E06F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E06FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0708 size=80
    let mut pc: u32 = 0x830E0708;
    'dispatch: loop {
        match pc {
            0x830E0708 => {
    //   block [0x830E0708..0x830E0758)
	// 830E0708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E071C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0720: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 830E0724: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0728: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E072C: 480218C5  bl 0x83101ff0
	ctx.lr = 0x830E0730;
	sub_83101FF0(ctx, base);
	// 830E0730: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E0734: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0738: 48023361  bl 0x83103a98
	ctx.lr = 0x830E073C;
	sub_83103A98(ctx, base);
	// 830E073C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E074C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0758 size=72
    let mut pc: u32 = 0x830E0758;
    'dispatch: loop {
        match pc {
            0x830E0758 => {
    //   block [0x830E0758..0x830E07A0)
	// 830E0758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0768: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E076C: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 830E0770: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0774: 4802187D  bl 0x83101ff0
	ctx.lr = 0x830E0778;
	sub_83101FF0(ctx, base);
	// 830E0778: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E077C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0780: 48023319  bl 0x83103a98
	ctx.lr = 0x830E0784;
	sub_83103A98(ctx, base);
	// 830E0784: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E0788: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E078C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0798: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E079C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E07A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E07A0 size=72
    let mut pc: u32 = 0x830E07A0;
    'dispatch: loop {
        match pc {
            0x830E07A0 => {
    //   block [0x830E07A0..0x830E07E8)
	// 830E07A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E07A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E07A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E07AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E07B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E07B4: 38800023  li r4, 0x23
	ctx.r[4].s64 = 35;
	// 830E07B8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E07BC: 48021835  bl 0x83101ff0
	ctx.lr = 0x830E07C0;
	sub_83101FF0(ctx, base);
	// 830E07C0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E07C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E07C8: 480232D1  bl 0x83103a98
	ctx.lr = 0x830E07CC;
	sub_83103A98(ctx, base);
	// 830E07CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E07D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E07D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E07D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E07DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E07E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E07E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E07E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E07E8 size=80
    let mut pc: u32 = 0x830E07E8;
    'dispatch: loop {
        match pc {
            0x830E07E8 => {
    //   block [0x830E07E8..0x830E0838)
	// 830E07E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E07EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E07F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E07F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E07F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E07FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0800: 3880002B  li r4, 0x2b
	ctx.r[4].s64 = 43;
	// 830E0804: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0808: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E080C: 480217E5  bl 0x83101ff0
	ctx.lr = 0x830E0810;
	sub_83101FF0(ctx, base);
	// 830E0810: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E0814: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0818: 48023281  bl 0x83103a98
	ctx.lr = 0x830E081C;
	sub_83103A98(ctx, base);
	// 830E081C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0820: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E082C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0838 size=80
    let mut pc: u32 = 0x830E0838;
    'dispatch: loop {
        match pc {
            0x830E0838 => {
    //   block [0x830E0838..0x830E0888)
	// 830E0838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E084C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0850: 3880001A  li r4, 0x1a
	ctx.r[4].s64 = 26;
	// 830E0854: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0858: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E085C: 48021795  bl 0x83101ff0
	ctx.lr = 0x830E0860;
	sub_83101FF0(ctx, base);
	// 830E0860: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E0864: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0868: 48023231  bl 0x83103a98
	ctx.lr = 0x830E086C;
	sub_83103A98(ctx, base);
	// 830E086C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E087C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0888 size=80
    let mut pc: u32 = 0x830E0888;
    'dispatch: loop {
        match pc {
            0x830E0888 => {
    //   block [0x830E0888..0x830E08D8)
	// 830E0888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E089C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E08A0: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 830E08A4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E08A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E08AC: 48021745  bl 0x83101ff0
	ctx.lr = 0x830E08B0;
	sub_83101FF0(ctx, base);
	// 830E08B0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 830E08B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E08B8: 480231E1  bl 0x83103a98
	ctx.lr = 0x830E08BC;
	sub_83103A98(ctx, base);
	// 830E08BC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E08C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E08C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E08C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E08CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E08D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E08D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E08D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E08D8 size=56
    let mut pc: u32 = 0x830E08D8;
    'dispatch: loop {
        match pc {
            0x830E08D8 => {
    //   block [0x830E08D8..0x830E0910)
	// 830E08D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E08DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E08E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E08E4: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830E08E8: 388007D2  li r4, 0x7d2
	ctx.r[4].s64 = 2002;
	// 830E08EC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E08F0: 48021701  bl 0x83101ff0
	ctx.lr = 0x830E08F4;
	sub_83101FF0(ctx, base);
	// 830E08F4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830E08F8: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 830E08FC: 4802319D  bl 0x83103a98
	ctx.lr = 0x830E0900;
	sub_83103A98(ctx, base);
	// 830E0900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830E0904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E090C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0910 size=64
    let mut pc: u32 = 0x830E0910;
    'dispatch: loop {
        match pc {
            0x830E0910 => {
    //   block [0x830E0910..0x830E0950)
	// 830E0910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0914: 480C7859  bl 0x831a816c
	ctx.lr = 0x830E0918;
	sub_831A8130(ctx, base);
	// 830E0918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E091C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0920: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 830E0924: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0928: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E092C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E0930: 480216C1  bl 0x83101ff0
	ctx.lr = 0x830E0934;
	sub_83101FF0(ctx, base);
	// 830E0934: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 830E0938: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E093C: 4802315D  bl 0x83103a98
	ctx.lr = 0x830E0940;
	sub_83103A98(ctx, base);
	// 830E0940: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0944: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830E0948: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E094C: 480C7870  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0950 size=64
    let mut pc: u32 = 0x830E0950;
    'dispatch: loop {
        match pc {
            0x830E0950 => {
    //   block [0x830E0950..0x830E0990)
	// 830E0950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0954: 480C7819  bl 0x831a816c
	ctx.lr = 0x830E0958;
	sub_831A8130(ctx, base);
	// 830E0958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E095C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0960: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 830E0964: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0968: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E096C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E0970: 48021681  bl 0x83101ff0
	ctx.lr = 0x830E0974;
	sub_83101FF0(ctx, base);
	// 830E0974: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 830E0978: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E097C: 4802311D  bl 0x83103a98
	ctx.lr = 0x830E0980;
	sub_83103A98(ctx, base);
	// 830E0980: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E0984: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830E0988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E098C: 480C7830  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0990 size=64
    let mut pc: u32 = 0x830E0990;
    'dispatch: loop {
        match pc {
            0x830E0990 => {
    //   block [0x830E0990..0x830E09D0)
	// 830E0990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0994: 480C77D9  bl 0x831a816c
	ctx.lr = 0x830E0998;
	sub_831A8130(ctx, base);
	// 830E0998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E099C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E09A0: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 830E09A4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E09A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830E09AC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830E09B0: 48021641  bl 0x83101ff0
	ctx.lr = 0x830E09B4;
	sub_83101FF0(ctx, base);
	// 830E09B4: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 830E09B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E09BC: 480230DD  bl 0x83103a98
	ctx.lr = 0x830E09C0;
	sub_83103A98(ctx, base);
	// 830E09C0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830E09C4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830E09C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E09CC: 480C77F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E09D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E09D0 size=88
    let mut pc: u32 = 0x830E09D0;
    'dispatch: loop {
        match pc {
            0x830E09D0 => {
    //   block [0x830E09D0..0x830E0A28)
	// 830E09D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E09D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E09D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E09DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E09E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E09E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E09E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E09EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E09F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E09F4: 4BFFFDAD  bl 0x830e07a0
	ctx.lr = 0x830E09F8;
	sub_830E07A0(ctx, base);
	// 830E09F8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830E09FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0A00: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 830E0A04: 4BFEE20D  bl 0x830cec10
	ctx.lr = 0x830E0A08;
	sub_830CEC10(ctx, base);
	// 830E0A08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0A0C: 4BFF51BD  bl 0x830d5bc8
	ctx.lr = 0x830E0A10;
	sub_830D5BC8(ctx, base);
	// 830E0A10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E0A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0A28 size=88
    let mut pc: u32 = 0x830E0A28;
    'dispatch: loop {
        match pc {
            0x830E0A28 => {
    //   block [0x830E0A28..0x830E0A80)
	// 830E0A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0A30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E0A34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0A38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0A40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E0A44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0A48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0A4C: 4BFEE875  bl 0x830cf2c0
	ctx.lr = 0x830E0A50;
	sub_830CF2C0(ctx, base);
	// 830E0A50: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830E0A54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0A58: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 830E0A5C: 4BFEE1B5  bl 0x830cec10
	ctx.lr = 0x830E0A60;
	sub_830CEC10(ctx, base);
	// 830E0A60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0A64: 4BFF5165  bl 0x830d5bc8
	ctx.lr = 0x830E0A68;
	sub_830D5BC8(ctx, base);
	// 830E0A68: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E0A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0A74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E0A78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0A80 size=68
    let mut pc: u32 = 0x830E0A80;
    'dispatch: loop {
        match pc {
            0x830E0A80 => {
    //   block [0x830E0A80..0x830E0AC4)
	// 830E0A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0A88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0A8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0A90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0A94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0A98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0A9C: 4BFFFBFD  bl 0x830e0698
	ctx.lr = 0x830E0AA0;
	sub_830E0698(ctx, base);
	// 830E0AA0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E0AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0AA8: 4BFF5121  bl 0x830d5bc8
	ctx.lr = 0x830E0AAC;
	sub_830D5BC8(ctx, base);
	// 830E0AAC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E0AB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E0AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0ABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0AC8 size=156
    let mut pc: u32 = 0x830E0AC8;
    'dispatch: loop {
        match pc {
            0x830E0AC8 => {
    //   block [0x830E0AC8..0x830E0B64)
	// 830E0AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0AD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0AD4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0AD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0ADC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830E0AE0: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 830E0AE4: 409A0068  bne cr6, 0x830e0b4c
	if !ctx.cr[6].eq {
	pc = 0x830E0B4C; continue 'dispatch;
	}
	// 830E0AE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0AEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0AF0: 4BFFFBE1  bl 0x830e06d0
	ctx.lr = 0x830E0AF4;
	sub_830E06D0(ctx, base);
	// 830E0AF4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E0AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0AFC: 4BFF50CD  bl 0x830d5bc8
	ctx.lr = 0x830E0B00;
	sub_830D5BC8(ctx, base);
	// 830E0B00: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E0B04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E0B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0B10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0B14: 4E800020  blr
	return;
	// 830E0B18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0B1C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0B20: 4BFFFBB1  bl 0x830e06d0
	ctx.lr = 0x830E0B24;
	sub_830E06D0(ctx, base);
	// 830E0B24: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E0B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0B2C: 4BFF509D  bl 0x830d5bc8
	ctx.lr = 0x830E0B30;
	sub_830D5BC8(ctx, base);
	// 830E0B30: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E0B34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E0B38: 409A0024  bne cr6, 0x830e0b5c
	if !ctx.cr[6].eq {
	pc = 0x830E0B5C; continue 'dispatch;
	}
	// 830E0B3C: 388100A4  addi r4, r1, 0xa4
	ctx.r[4].s64 = ctx.r[1].s64 + 164;
	// 830E0B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0B44: 4BFFEC15  bl 0x830df758
	ctx.lr = 0x830E0B48;
	sub_830DF758(ctx, base);
	// 830E0B48: 83E100A4  lwz r31, 0xa4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 830E0B4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E0B50: 409AFFC8  bne cr6, 0x830e0b18
	if !ctx.cr[6].eq {
	pc = 0x830E0B18; continue 'dispatch;
	}
	// 830E0B54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E0B58: 4BFFFFAC  b 0x830e0b04
	pc = 0x830E0B04; continue 'dispatch;
	// 830E0B5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830E0B60: 4BFFFFA4  b 0x830e0b04
	pc = 0x830E0B04; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0B68 size=56
    let mut pc: u32 = 0x830E0B68;
    'dispatch: loop {
        match pc {
            0x830E0B68 => {
    //   block [0x830E0B68..0x830E0BA0)
	// 830E0B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0B74: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830E0B78: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 830E0B7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830E0B80: 48021471  bl 0x83101ff0
	ctx.lr = 0x830E0B84;
	sub_83101FF0(ctx, base);
	// 830E0B84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0B88: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 830E0B8C: 4BFF8D8D  bl 0x830d9918
	ctx.lr = 0x830E0B90;
	sub_830D9918(ctx, base);
	// 830E0B90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E0B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0BA0 size=68
    let mut pc: u32 = 0x830E0BA0;
    'dispatch: loop {
        match pc {
            0x830E0BA0 => {
    //   block [0x830E0BA0..0x830E0BE4)
	// 830E0BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0BA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0BAC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0BB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0BB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0BB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0BBC: 4BFFFD1D  bl 0x830e08d8
	ctx.lr = 0x830E0BC0;
	sub_830E08D8(ctx, base);
	// 830E0BC0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E0BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0BC8: 4BFF5001  bl 0x830d5bc8
	ctx.lr = 0x830E0BCC;
	sub_830D5BC8(ctx, base);
	// 830E0BCC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E0BD0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E0BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0BE8 size=72
    let mut pc: u32 = 0x830E0BE8;
    'dispatch: loop {
        match pc {
            0x830E0BE8 => {
    //   block [0x830E0BE8..0x830E0C30)
	// 830E0BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E0BF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E0BF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0BFC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830E0C00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0C04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0C08: 4BFFFC81  bl 0x830e0888
	ctx.lr = 0x830E0C0C;
	sub_830E0888(ctx, base);
	// 830E0C0C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E0C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0C14: 4BFF4FB5  bl 0x830d5bc8
	ctx.lr = 0x830E0C18;
	sub_830D5BC8(ctx, base);
	// 830E0C18: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E0C1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E0C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E0C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E0C28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E0C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0C30 size=556
    let mut pc: u32 = 0x830E0C30;
    'dispatch: loop {
        match pc {
            0x830E0C30 => {
    //   block [0x830E0C30..0x830E0E5C)
	// 830E0C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0C34: 480C7535  bl 0x831a8168
	ctx.lr = 0x830E0C38;
	sub_831A8130(ctx, base);
	// 830E0C38: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0C3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E0C40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0C44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830E0C48: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0C4C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0C50: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E0C54: 418200B4  beq 0x830e0d08
	if ctx.cr[0].eq {
	pc = 0x830E0D08; continue 'dispatch;
	}
	// 830E0C58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0C5C: 4802185D  bl 0x831024b8
	ctx.lr = 0x830E0C60;
	sub_831024B8(ctx, base);
	// 830E0C60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0C64: A0BE0002  lhz r5, 2(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 830E0C68: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 830E0C6C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0C74: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830E0C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830E0C7C: 4E800421  bctrl
	ctx.lr = 0x830E0C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830E0C80: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0C84: 40800018  bge 0x830e0c9c
	if !ctx.cr[0].lt {
	pc = 0x830E0C9C; continue 'dispatch;
	}
	// 830E0C88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0C8C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0C90: 48021DF9  bl 0x83102a88
	ctx.lr = 0x830E0C94;
	sub_83102A88(ctx, base);
	// 830E0C94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0C98: 480001BC  b 0x830e0e54
	pc = 0x830E0E54; continue 'dispatch;
	// 830E0C9C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 830E0CA0: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 830E0CA4: 419A0010  beq cr6, 0x830e0cb4
	if ctx.cr[6].eq {
	pc = 0x830E0CB4; continue 'dispatch;
	}
	// 830E0CA8: 3FE08030  lis r31, -0x7fd0
	ctx.r[31].s64 = -2144337920;
	// 830E0CAC: 63FF0001  ori r31, r31, 1
	ctx.r[31].u64 = ctx.r[31].u64 | 1;
	// 830E0CB0: 4BFFFFDC  b 0x830e0c8c
	pc = 0x830E0C8C; continue 'dispatch;
	// 830E0CB4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830E0CB8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830E0CBC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0CC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830E0CC4: 4099003C  ble cr6, 0x830e0d00
	if !ctx.cr[6].gt {
	pc = 0x830E0D00; continue 'dispatch;
	}
	// 830E0CC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0CCC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830E0CD0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 830E0CD4: 7D4AF9D6  mullw r10, r10, r31
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 830E0CD8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830E0CDC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 830E0CE0: 4BFFFF51  bl 0x830e0c30
	ctx.lr = 0x830E0CE4;
	sub_830E0C30(ctx, base);
	// 830E0CE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0CE8: 4180FFA0  blt 0x830e0c88
	if ctx.cr[0].lt {
	pc = 0x830E0C88; continue 'dispatch;
	}
	// 830E0CEC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830E0CF0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830E0CF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0CF8: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830E0CFC: 4198FFCC  blt cr6, 0x830e0cc8
	if ctx.cr[6].lt {
	pc = 0x830E0CC8; continue 'dispatch;
	}
	// 830E0D00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E0D04: 48000148  b 0x830e0e4c
	pc = 0x830E0E4C; continue 'dispatch;
	// 830E0D08: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830E0D0C: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 830E0D10: 409A0080  bne cr6, 0x830e0d90
	if !ctx.cr[6].eq {
	pc = 0x830E0D90; continue 'dispatch;
	}
	// 830E0D14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0D18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830E0D1C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0D24: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830E0D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830E0D2C: 4E800421  bctrl
	ctx.lr = 0x830E0D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830E0D30: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0D34: 41800120  blt 0x830e0e54
	if ctx.cr[0].lt {
	pc = 0x830E0E54; continue 'dispatch;
	}
	// 830E0D38: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0D3C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E0D40: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E0D44: 41820028  beq 0x830e0d6c
	if ctx.cr[0].eq {
	pc = 0x830E0D6C; continue 'dispatch;
	}
	// 830E0D48: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 830E0D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E0D50: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830E0D54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830E0D58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830E0D5C: 4E800421  bctrl
	ctx.lr = 0x830E0D60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830E0D60: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0D64: 408000EC  bge 0x830e0e50
	if !ctx.cr[0].lt {
	pc = 0x830E0E50; continue 'dispatch;
	}
	// 830E0D68: 480000EC  b 0x830e0e54
	pc = 0x830E0E54; continue 'dispatch;
	// 830E0D6C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830E0D70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830E0D74: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 830E0D78: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 830E0D7C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0D80: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 830E0D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830E0D88: 4E800421  bctrl
	ctx.lr = 0x830E0D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830E0D8C: 4BFFFFD4  b 0x830e0d60
	pc = 0x830E0D60; continue 'dispatch;
	// 830E0D90: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830E0D94: 48021725  bl 0x831024b8
	ctx.lr = 0x830E0D98;
	sub_831024B8(ctx, base);
	// 830E0D98: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 830E0D9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E0DA0: 48025FE9  bl 0x83106d88
	ctx.lr = 0x830E0DA4;
	sub_83106D88(ctx, base);
	// 830E0DA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E0DA8: A0BE0002  lhz r5, 2(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 830E0DAC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 830E0DB0: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0DB8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830E0DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830E0DC0: 4E800421  bctrl
	ctx.lr = 0x830E0DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830E0DC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E0DC8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E0DCC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830E0DD0: 4080000C  bge 0x830e0ddc
	if !ctx.cr[0].lt {
	pc = 0x830E0DDC; continue 'dispatch;
	}
	// 830E0DD4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830E0DD8: 4BFFFEB8  b 0x830e0c90
	pc = 0x830E0C90; continue 'dispatch;
	// 830E0DDC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830E0DE0: 419A0068  beq cr6, 0x830e0e48
	if ctx.cr[6].eq {
	pc = 0x830E0E48; continue 'dispatch;
	}
	// 830E0DE4: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 830E0DE8: 817F56F4  lwz r11, 0x56f4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22260 as u32) ) } as u64;
	// 830E0DEC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 830E0DF0: 409A0040  bne cr6, 0x830e0e30
	if !ctx.cr[6].eq {
	pc = 0x830E0E30; continue 'dispatch;
	}
	// 830E0DF4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E0DF8: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 830E0DFC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830E0E00: 388A8B20  addi r4, r10, -0x74e0
	ctx.r[4].s64 = ctx.r[10].s64 + -29920;
	// 830E0E04: 806BC078  lwz r3, -0x3f88(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830E0E08: 4BFEDA61  bl 0x830ce868
	ctx.lr = 0x830E0E0C;
	sub_830CE868(ctx, base);
	// 830E0E0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0E10: 41800014  blt 0x830e0e24
	if ctx.cr[0].lt {
	pc = 0x830E0E24; continue 'dispatch;
	}
	// 830E0E14: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E0E18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0E1C: 917F56F4  stw r11, 0x56f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22260 as u32), ctx.r[11].u32 ) };
	// 830E0E20: 48000008  b 0x830e0e28
	pc = 0x830E0E28; continue 'dispatch;
	// 830E0E24: 817F56F4  lwz r11, 0x56f4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22260 as u32) ) } as u64;
	// 830E0E28: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 830E0E2C: 419A001C  beq cr6, 0x830e0e48
	if ctx.cr[6].eq {
	pc = 0x830E0E48; continue 'dispatch;
	}
	// 830E0E30: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E0E34: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830E0E38: 409A0010  bne cr6, 0x830e0e48
	if !ctx.cr[6].eq {
	pc = 0x830E0E48; continue 'dispatch;
	}
	// 830E0E3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830E0E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E0E44: 48025ECD  bl 0x83106d10
	ctx.lr = 0x830E0E48;
	sub_83106D10(ctx, base);
	// 830E0E48: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830E0E4C: 48021C3D  bl 0x83102a88
	ctx.lr = 0x830E0E50;
	sub_83102A88(ctx, base);
	// 830E0E50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E0E54: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 830E0E58: 480C7360  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830E0E60 size=276
    let mut pc: u32 = 0x830E0E60;
    'dispatch: loop {
        match pc {
            0x830E0E60 => {
    //   block [0x830E0E60..0x830E0F74)
	// 830E0E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0E64: 480C7309  bl 0x831a816c
	ctx.lr = 0x830E0E68;
	sub_831A8130(ctx, base);
	// 830E0E68: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 830E0E6C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 830E0E70: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0E74: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 830E0E78: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 830E0E7C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 830E0E80: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 830E0E84: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 830E0E88: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 830E0E8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830E0E90: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 830E0E94: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 830E0E98: C3CB9528  lfs f30, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 830E0E9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830E0EA0: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 830E0EA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 830E0EA8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 830E0EAC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830E0EB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E0EB4: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830E0EB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830E0EBC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 830E0EC0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 830E0EC4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830E0EC8: C3EA08A8  lfs f31, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 830E0ECC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 830E0ED0: C0091514  lfs f0, 0x1514(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5396 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830E0ED4: EDAD07B2  fmuls f13, f13, f30
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64) as f32) as f64);
	// 830E0ED8: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 830E0EDC: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 830E0EE0: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830E0EE4: EDACFFBC  fnmsubs f13, f12, f30, f31
	ctx.f[13].f64 = -(((ctx.f[12].f64 * ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 830E0EE8: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830E0EEC: 48037455  bl 0x83118340
	ctx.lr = 0x830E0EF0;
	sub_83118340(ctx, base);
	// 830E0EF0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 830E0EF4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830E0EF8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830E0EFC: 48036D2D  bl 0x83117c28
	ctx.lr = 0x830E0F00;
	sub_83117C28(ctx, base);
	// 830E0F00: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 830E0F04: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 830E0F08: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 830E0F0C: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 830E0F10: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 830E0F14: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830E0F18: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 830E0F1C: C00A00DC  lfs f0, 0xdc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830E0F20: EDADFFB8  fmsubs f13, f13, f30, f31
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 830E0F24: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830E0F28: 48037419  bl 0x83118340
	ctx.lr = 0x830E0F2C;
	sub_83118340(ctx, base);
	// 830E0F2C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 830E0F30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830E0F38: 48036CF1  bl 0x83117c28
	ctx.lr = 0x830E0F3C;
	sub_83117C28(ctx, base);
	// 830E0F3C: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830E0F40: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830E0F44: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 830E0F48: C1810058  lfs f12, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830E0F4C: EDADF82A  fadds f13, f13, f31
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 830E0F50: C161005C  lfs f11, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 830E0F54: D19E0000  stfs f12, 0(r30)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830E0F58: D17E0004  stfs f11, 4(r30)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830E0F5C: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830E0F60: D1BD0004  stfs f13, 4(r29)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830E0F64: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 830E0F68: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 830E0F6C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 830E0F70: 480C724C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E0F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E0F78 size=196
    let mut pc: u32 = 0x830E0F78;
    'dispatch: loop {
        match pc {
            0x830E0F78 => {
    //   block [0x830E0F78..0x830E103C)
	// 830E0F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E0F7C: 480C71F1  bl 0x831a816c
	ctx.lr = 0x830E0F80;
	sub_831A8130(ctx, base);
	// 830E0F80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E0F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E0F88: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830E0F8C: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 830E0F90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830E0F94: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830E0F98: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830E0F9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830E0FA0: 4BFFF521  bl 0x830e04c0
	ctx.lr = 0x830E0FA4;
	sub_830E04C0(ctx, base);
	// 830E0FA4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 830E0FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0FAC: 4BFF4C1D  bl 0x830d5bc8
	ctx.lr = 0x830E0FB0;
	sub_830D5BC8(ctx, base);
	// 830E0FB0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0FB4: 40800014  bge 0x830e0fc8
	if !ctx.cr[0].lt {
	pc = 0x830E0FC8; continue 'dispatch;
	}
	// 830E0FB8: 3D608030  lis r11, -0x7fd0
	ctx.r[11].s64 = -2144337920;
	// 830E0FBC: 616B0026  ori r11, r11, 0x26
	ctx.r[11].u64 = ctx.r[11].u64 | 38;
	// 830E0FC0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830E0FC4: 409A0070  bne cr6, 0x830e1034
	if !ctx.cr[6].eq {
	pc = 0x830E1034; continue 'dispatch;
	}
	// 830E0FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E0FCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E0FD4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830E0FD8: 4BFFE7F9  bl 0x830df7d0
	ctx.lr = 0x830E0FDC;
	sub_830DF7D0(ctx, base);
	// 830E0FDC: 48000030  b 0x830e100c
	pc = 0x830E100C; continue 'dispatch;
	// 830E0FE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E0FE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E0FE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E0FEC: 4BFFFF8D  bl 0x830e0f78
	ctx.lr = 0x830E0FF0;
	sub_830E0F78(ctx, base);
	// 830E0FF0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E0FF4: 41800040  blt 0x830e1034
	if ctx.cr[0].lt {
	pc = 0x830E1034; continue 'dispatch;
	}
	// 830E0FF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E0FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1000: 4BFFE8C1  bl 0x830df8c0
	ctx.lr = 0x830E1004;
	sub_830DF8C0(ctx, base);
	// 830E1004: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1008: 4180002C  blt 0x830e1034
	if ctx.cr[0].lt {
	pc = 0x830E1034; continue 'dispatch;
	}
	// 830E100C: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1010: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830E1014: 409AFFCC  bne cr6, 0x830e0fe0
	if !ctx.cr[6].eq {
	pc = 0x830E0FE0; continue 'dispatch;
	}
	// 830E1018: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830E101C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830E1020: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830E1024: 4BFFF6E5  bl 0x830e0708
	ctx.lr = 0x830E1028;
	sub_830E0708(ctx, base);
	// 830E1028: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 830E102C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1030: 4BFF4B99  bl 0x830d5bc8
	ctx.lr = 0x830E1034;
	sub_830D5BC8(ctx, base);
	// 830E1034: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830E1038: 480C7184  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830E1040 size=296
    let mut pc: u32 = 0x830E1040;
    'dispatch: loop {
        match pc {
            0x830E1040 => {
    //   block [0x830E1040..0x830E1168)
	// 830E1040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1044: 480C7109  bl 0x831a814c
	ctx.lr = 0x830E1048;
	sub_831A8130(ctx, base);
	// 830E1048: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E104C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830E1050: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 830E1054: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830E1058: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830E105C: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 830E1060: C00B9584  lfs f0, -0x6a7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830E1064: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830E1068: C1AAD72C  lfs f13, -0x28d4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10452 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830E106C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 830E1070: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830E1074: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 830E1078: D1BA0000  stfs f13, 0(r26)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830E107C: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 830E1080: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830E1084: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830E1088: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830E108C: 3B0A8A48  addi r24, r10, -0x75b8
	ctx.r[24].s64 = ctx.r[10].s64 + -30136;
	// 830E1090: 3AE98A58  addi r23, r9, -0x75a8
	ctx.r[23].s64 = ctx.r[9].s64 + -30120;
	// 830E1094: 3B8B8A08  addi r28, r11, -0x75f8
	ctx.r[28].s64 = ctx.r[11].s64 + -30200;
	// 830E1098: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E109C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830E10A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830E10A4: 419A0008  beq cr6, 0x830e10ac
	if ctx.cr[6].eq {
	pc = 0x830E10AC; continue 'dispatch;
	}
	// 830E10A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E10AC: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830E10B0: 40980098  bge cr6, 0x830e1148
	if !ctx.cr[6].lt {
	pc = 0x830E1148; continue 'dispatch;
	}
	// 830E10B4: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 830E10B8: 40980090  bge cr6, 0x830e1148
	if !ctx.cr[6].lt {
	pc = 0x830E1148; continue 'dispatch;
	}
	// 830E10BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E10C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830E10C4: 7D4AE9D6  mullw r10, r10, r29
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[29].s32 as i64);
	// 830E10C8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830E10CC: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 830E10D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830E10D4: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830E10D8: 480D0D59  bl 0x831b1e30
	ctx.lr = 0x830E10DC;
	sub_831B1E30(ctx, base);
	// 830E10DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E10E0: 40820018  bne 0x830e10f8
	if !ctx.cr[0].eq {
	pc = 0x830E10F8; continue 'dispatch;
	}
	// 830E10E4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830E10E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E10EC: 48025D85  bl 0x83106e70
	ctx.lr = 0x830E10F0;
	sub_83106E70(ctx, base);
	// 830E10F0: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 830E10F4: 48000048  b 0x830e113c
	pc = 0x830E113C; continue 'dispatch;
	// 830E10F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E10FC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830E1100: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830E1104: 480D0D2D  bl 0x831b1e30
	ctx.lr = 0x830E1108;
	sub_831B1E30(ctx, base);
	// 830E1108: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E110C: 4082000C  bne 0x830e1118
	if !ctx.cr[0].eq {
	pc = 0x830E1118; continue 'dispatch;
	}
	// 830E1110: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830E1114: 48000020  b 0x830e1134
	pc = 0x830E1134; continue 'dispatch;
	// 830E1118: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E111C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830E1120: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830E1124: 480D0D0D  bl 0x831b1e30
	ctx.lr = 0x830E1128;
	sub_831B1E30(ctx, base);
	// 830E1128: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E112C: 40820014  bne 0x830e1140
	if !ctx.cr[0].eq {
	pc = 0x830E1140; continue 'dispatch;
	}
	// 830E1130: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830E1134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1138: 48025C29  bl 0x83106d60
	ctx.lr = 0x830E113C;
	sub_83106D60(ctx, base);
	// 830E113C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830E1140: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830E1144: 4BFFFF54  b 0x830e1098
	pc = 0x830E1098; continue 'dispatch;
	// 830E1148: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 830E114C: 419A000C  beq cr6, 0x830e1158
	if ctx.cr[6].eq {
	pc = 0x830E1158; continue 'dispatch;
	}
	// 830E1150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1154: 4800000C  b 0x830e1160
	pc = 0x830E1160; continue 'dispatch;
	// 830E1158: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E115C: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 830E1160: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830E1164: 480C7038  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1168 size=132
    let mut pc: u32 = 0x830E1168;
    'dispatch: loop {
        match pc {
            0x830E1168 => {
    //   block [0x830E1168..0x830E11EC)
	// 830E1168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E116C: 480C6FFD  bl 0x831a8168
	ctx.lr = 0x830E1170;
	sub_831A8130(ctx, base);
	// 830E1170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1174: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830E1178: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830E117C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1180: 41820048  beq 0x830e11c8
	if ctx.cr[0].eq {
	pc = 0x830E11C8; continue 'dispatch;
	}
	// 830E1184: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830E1188: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 830E118C: 1D6A0044  mulli r11, r10, 0x44
	ctx.r[11].s64 = ctx.r[10].s64 * 68;
	// 830E1190: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1194: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830E1198: 41800018  blt 0x830e11b0
	if ctx.cr[0].lt {
	pc = 0x830E11B0; continue 'dispatch;
	}
	// 830E119C: 3BDEFFBC  addi r30, r30, -0x44
	ctx.r[30].s64 = ctx.r[30].s64 + -68;
	// 830E11A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E11A4: 4801E6D5  bl 0x830ff878
	ctx.lr = 0x830E11A8;
	sub_830FF878(ctx, base);
	// 830E11A8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E11AC: 4080FFF0  bge 0x830e119c
	if !ctx.cr[0].lt {
	pc = 0x830E119C; continue 'dispatch;
	}
	// 830E11B0: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E11B4: 4182000C  beq 0x830e11c0
	if ctx.cr[0].eq {
	pc = 0x830E11C0; continue 'dispatch;
	}
	// 830E11B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E11BC: 4BFFC31D  bl 0x830dd4d8
	ctx.lr = 0x830E11C0;
	sub_830DD4D8(ctx, base);
	// 830E11C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E11C4: 48000020  b 0x830e11e4
	pc = 0x830E11E4; continue 'dispatch;
	// 830E11C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E11CC: 4801E6AD  bl 0x830ff878
	ctx.lr = 0x830E11D0;
	sub_830FF878(ctx, base);
	// 830E11D0: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E11D4: 4182000C  beq 0x830e11e0
	if ctx.cr[0].eq {
	pc = 0x830E11E0; continue 'dispatch;
	}
	// 830E11D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E11DC: 4BFFC2FD  bl 0x830dd4d8
	ctx.lr = 0x830E11E0;
	sub_830DD4D8(ctx, base);
	// 830E11E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E11E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830E11E8: 480C6FD0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E11F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E11F0 size=404
    let mut pc: u32 = 0x830E11F0;
    'dispatch: loop {
        match pc {
            0x830E11F0 => {
    //   block [0x830E11F0..0x830E1384)
	// 830E11F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E11F4: 480C6F65  bl 0x831a8158
	ctx.lr = 0x830E11F8;
	sub_831A8130(ctx, base);
	// 830E11F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E11FC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E1200: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830E1204: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830E1208: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830E120C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830E1210: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830E1214: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830E1218: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 830E121C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830E1220: 4BFFD8C9  bl 0x830deae8
	ctx.lr = 0x830E1224;
	sub_830DEAE8(ctx, base);
	// 830E1224: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830E1228: 4082000C  bne 0x830e1234
	if !ctx.cr[0].eq {
	pc = 0x830E1234; continue 'dispatch;
	}
	// 830E122C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1230: 4800014C  b 0x830e137c
	pc = 0x830E137C; continue 'dispatch;
	// 830E1234: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830E1238: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830E123C: 41820028  beq 0x830e1264
	if ctx.cr[0].eq {
	pc = 0x830E1264; continue 'dispatch;
	}
	// 830E1240: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1244: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830E1248: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830E124C: 4BFFEA35  bl 0x830dfc80
	ctx.lr = 0x830E1250;
	sub_830DFC80(ctx, base);
	// 830E1250: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1254: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830E1258: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830E125C: 4BFFEA25  bl 0x830dfc80
	ctx.lr = 0x830E1260;
	sub_830DFC80(ctx, base);
	// 830E1260: 4BFFFFCC  b 0x830e122c
	pc = 0x830E122C; continue 'dispatch;
	// 830E1264: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 830E1268: 2B1C0001  cmplwi cr6, r28, 1
	ctx.cr[6].compare_u32(ctx.r[28].u32, 1 as u32, &mut ctx.xer);
	// 830E126C: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830E1270: 41980048  blt cr6, 0x830e12b8
	if ctx.cr[6].lt {
	pc = 0x830E12B8; continue 'dispatch;
	}
	// 830E1274: 419A003C  beq cr6, 0x830e12b0
	if ctx.cr[6].eq {
	pc = 0x830E12B0; continue 'dispatch;
	}
	// 830E1278: 2B1C0003  cmplwi cr6, r28, 3
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3 as u32, &mut ctx.xer);
	// 830E127C: 4198002C  blt cr6, 0x830e12a8
	if ctx.cr[6].lt {
	pc = 0x830E12A8; continue 'dispatch;
	}
	// 830E1280: 419A0020  beq cr6, 0x830e12a0
	if ctx.cr[6].eq {
	pc = 0x830E12A0; continue 'dispatch;
	}
	// 830E1284: 2B1C0005  cmplwi cr6, r28, 5
	ctx.cr[6].compare_u32(ctx.r[28].u32, 5 as u32, &mut ctx.xer);
	// 830E1288: 41980010  blt cr6, 0x830e1298
	if ctx.cr[6].lt {
	pc = 0x830E1298; continue 'dispatch;
	}
	// 830E128C: 409A00E0  bne cr6, 0x830e136c
	if !ctx.cr[6].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E1290: 83DD0028  lwz r30, 0x28(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 830E1294: 48000028  b 0x830e12bc
	pc = 0x830E12BC; continue 'dispatch;
	// 830E1298: 83DD0024  lwz r30, 0x24(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 830E129C: 48000020  b 0x830e12bc
	pc = 0x830E12BC; continue 'dispatch;
	// 830E12A0: 83DD0018  lwz r30, 0x18(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830E12A4: 48000018  b 0x830e12bc
	pc = 0x830E12BC; continue 'dispatch;
	// 830E12A8: 83DD0014  lwz r30, 0x14(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830E12AC: 48000010  b 0x830e12bc
	pc = 0x830E12BC; continue 'dispatch;
	// 830E12B0: 83DD0020  lwz r30, 0x20(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830E12B4: 48000008  b 0x830e12bc
	pc = 0x830E12BC; continue 'dispatch;
	// 830E12B8: 83DD001C  lwz r30, 0x1c(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 830E12BC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830E12C0: 419A00AC  beq cr6, 0x830e136c
	if ctx.cr[6].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E12C4: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E12C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E12CC: 418200A0  beq 0x830e136c
	if ctx.cr[0].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E12D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E12D4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830E12D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830E12DC: 4BFFE47D  bl 0x830df758
	ctx.lr = 0x830E12E0;
	sub_830DF758(ctx, base);
	// 830E12E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E12E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E12E8: 419A0084  beq cr6, 0x830e136c
	if ctx.cr[6].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E12EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830E12F0: 48024E41  bl 0x83106130
	ctx.lr = 0x830E12F4;
	sub_83106130(ctx, base);
	// 830E12F4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E12F8: 41820074  beq 0x830e136c
	if ctx.cr[0].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E12FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830E1300: 4BFED911  bl 0x830cec10
	ctx.lr = 0x830E1304;
	sub_830CEC10(ctx, base);
	// 830E1304: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830E1308: 419A0064  beq cr6, 0x830e136c
	if ctx.cr[6].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E130C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 830E1310: 419A0014  beq cr6, 0x830e1324
	if ctx.cr[6].eq {
	pc = 0x830E1324; continue 'dispatch;
	}
	// 830E1314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1318: 4BFFF769  bl 0x830e0a80
	ctx.lr = 0x830E131C;
	sub_830E0A80(ctx, base);
	// 830E131C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1320: 41820030  beq 0x830e1350
	if ctx.cr[0].eq {
	pc = 0x830E1350; continue 'dispatch;
	}
	// 830E1324: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 830E1328: 419A0014  beq cr6, 0x830e133c
	if ctx.cr[6].eq {
	pc = 0x830E133C; continue 'dispatch;
	}
	// 830E132C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1330: 4BFFF871  bl 0x830e0ba0
	ctx.lr = 0x830E1334;
	sub_830E0BA0(ctx, base);
	// 830E1334: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1338: 41820018  beq 0x830e1350
	if ctx.cr[0].eq {
	pc = 0x830E1350; continue 'dispatch;
	}
	// 830E133C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830E1340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1344: 4BFFF785  bl 0x830e0ac8
	ctx.lr = 0x830E1348;
	sub_830E0AC8(ctx, base);
	// 830E1348: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E134C: 41820020  beq 0x830e136c
	if ctx.cr[0].eq {
	pc = 0x830E136C; continue 'dispatch;
	}
	// 830E1350: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 830E1354: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 830E1358: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830E135C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E1360: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830E1364: 4BFFFE8D  bl 0x830e11f0
	ctx.lr = 0x830E1368;
	sub_830E11F0(ctx, base);
	// 830E1368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E136C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830E1370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1374: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830E1378: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830E137C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830E1380: 480C6E28  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1388 size=128
    let mut pc: u32 = 0x830E1388;
    'dispatch: loop {
        match pc {
            0x830E1388 => {
    //   block [0x830E1388..0x830E1408)
	// 830E1388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E138C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E1390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E1394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E1398: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E139C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E13A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E13A4: 409A0010  bne cr6, 0x830e13b4
	if !ctx.cr[6].eq {
	pc = 0x830E13B4; continue 'dispatch;
	}
	// 830E13A8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E13AC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E13B0: 48000040  b 0x830e13f0
	pc = 0x830E13F0; continue 'dispatch;
	// 830E13B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E13B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E13BC: 4BFED855  bl 0x830cec10
	ctx.lr = 0x830E13C0;
	sub_830CEC10(ctx, base);
	// 830E13C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830E13C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E13C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E13CC: 4BFFF1F5  bl 0x830e05c0
	ctx.lr = 0x830E13D0;
	sub_830E05C0(ctx, base);
	// 830E13D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E13D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E13D8: 4BFF47F1  bl 0x830d5bc8
	ctx.lr = 0x830E13DC;
	sub_830D5BC8(ctx, base);
	// 830E13DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E13E0: 41800010  blt 0x830e13f0
	if ctx.cr[0].lt {
	pc = 0x830E13F0; continue 'dispatch;
	}
	// 830E13E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E13E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E13EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E13F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E13F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E13F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E13FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E1400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E1404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1408 size=168
    let mut pc: u32 = 0x830E1408;
    'dispatch: loop {
        match pc {
            0x830E1408 => {
    //   block [0x830E1408..0x830E14B0)
	// 830E1408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E140C: 480C6D61  bl 0x831a816c
	ctx.lr = 0x830E1410;
	sub_831A8130(ctx, base);
	// 830E1410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1414: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E1418: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830E141C: 83EBD848  lwz r31, -0x27b8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10168 as u32) ) } as u64;
	// 830E1420: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E1424: 419A0030  beq cr6, 0x830e1454
	if ctx.cr[6].eq {
	pc = 0x830E1454; continue 'dispatch;
	}
	// 830E1428: 4BFED7E9  bl 0x830cec10
	ctx.lr = 0x830E142C;
	sub_830CEC10(ctx, base);
	// 830E142C: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830E1430: 419A001C  beq cr6, 0x830e144c
	if ctx.cr[6].eq {
	pc = 0x830E144C; continue 'dispatch;
	}
	// 830E1434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E1438: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E143C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1440: 4BFFE9B1  bl 0x830dfdf0
	ctx.lr = 0x830E1444;
	sub_830DFDF0(ctx, base);
	// 830E1444: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1448: 4182000C  beq 0x830e1454
	if ctx.cr[0].eq {
	pc = 0x830E1454; continue 'dispatch;
	}
	// 830E144C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830E1450: 48000058  b 0x830e14a8
	pc = 0x830E14A8; continue 'dispatch;
	// 830E1454: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830E1458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E145C: 4BFFD9DD  bl 0x830dee38
	ctx.lr = 0x830E1460;
	sub_830DEE38(ctx, base);
	// 830E1460: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1464: 4182002C  beq 0x830e1490
	if ctx.cr[0].eq {
	pc = 0x830E1490; continue 'dispatch;
	}
	// 830E1468: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E146C: 4BFED7A5  bl 0x830cec10
	ctx.lr = 0x830E1470;
	sub_830CEC10(ctx, base);
	// 830E1470: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830E1474: 419AFFD8  beq cr6, 0x830e144c
	if ctx.cr[6].eq {
	pc = 0x830E144C; continue 'dispatch;
	}
	// 830E1478: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E147C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E1480: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1484: 4BFFE96D  bl 0x830dfdf0
	ctx.lr = 0x830E1488;
	sub_830DFDF0(ctx, base);
	// 830E1488: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E148C: 4082FFC0  bne 0x830e144c
	if !ctx.cr[0].eq {
	pc = 0x830E144C; continue 'dispatch;
	}
	// 830E1490: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 830E1494: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830E1498: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830E149C: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 830E14A0: 4198FFB8  blt cr6, 0x830e1458
	if ctx.cr[6].lt {
	pc = 0x830E1458; continue 'dispatch;
	}
	// 830E14A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E14A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E14AC: 480C6D10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E14B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E14B0 size=168
    let mut pc: u32 = 0x830E14B0;
    'dispatch: loop {
        match pc {
            0x830E14B0 => {
    //   block [0x830E14B0..0x830E1558)
	// 830E14B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E14B4: 480C6CB9  bl 0x831a816c
	ctx.lr = 0x830E14B8;
	sub_831A8130(ctx, base);
	// 830E14B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E14BC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E14C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830E14C4: 83EBD848  lwz r31, -0x27b8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10168 as u32) ) } as u64;
	// 830E14C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E14CC: 419A0030  beq cr6, 0x830e14fc
	if ctx.cr[6].eq {
	pc = 0x830E14FC; continue 'dispatch;
	}
	// 830E14D0: 4BFED741  bl 0x830cec10
	ctx.lr = 0x830E14D4;
	sub_830CEC10(ctx, base);
	// 830E14D4: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830E14D8: 419A001C  beq cr6, 0x830e14f4
	if ctx.cr[6].eq {
	pc = 0x830E14F4; continue 'dispatch;
	}
	// 830E14DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E14E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E14E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E14E8: 4BFFE909  bl 0x830dfdf0
	ctx.lr = 0x830E14EC;
	sub_830DFDF0(ctx, base);
	// 830E14EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E14F0: 4182000C  beq 0x830e14fc
	if ctx.cr[0].eq {
	pc = 0x830E14FC; continue 'dispatch;
	}
	// 830E14F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E14F8: 48000058  b 0x830e1550
	pc = 0x830E1550; continue 'dispatch;
	// 830E14FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830E1500: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1504: 4BFFD935  bl 0x830dee38
	ctx.lr = 0x830E1508;
	sub_830DEE38(ctx, base);
	// 830E1508: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E150C: 4182002C  beq 0x830e1538
	if ctx.cr[0].eq {
	pc = 0x830E1538; continue 'dispatch;
	}
	// 830E1510: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1514: 4BFED6FD  bl 0x830cec10
	ctx.lr = 0x830E1518;
	sub_830CEC10(ctx, base);
	// 830E1518: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830E151C: 419AFFD8  beq cr6, 0x830e14f4
	if ctx.cr[6].eq {
	pc = 0x830E14F4; continue 'dispatch;
	}
	// 830E1520: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E1524: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E1528: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E152C: 4BFFE8C5  bl 0x830dfdf0
	ctx.lr = 0x830E1530;
	sub_830DFDF0(ctx, base);
	// 830E1530: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1534: 4082FFC0  bne 0x830e14f4
	if !ctx.cr[0].eq {
	pc = 0x830E14F4; continue 'dispatch;
	}
	// 830E1538: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 830E153C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830E1540: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830E1544: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 830E1548: 4198FFB8  blt cr6, 0x830e1500
	if ctx.cr[6].lt {
	pc = 0x830E1500; continue 'dispatch;
	}
	// 830E154C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1550: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E1554: 480C6C68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1558 size=176
    let mut pc: u32 = 0x830E1558;
    'dispatch: loop {
        match pc {
            0x830E1558 => {
    //   block [0x830E1558..0x830E1608)
	// 830E1558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E1560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E1564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E1568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E156C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E1570: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830E1574: 409A0010  bne cr6, 0x830e1584
	if !ctx.cr[6].eq {
	pc = 0x830E1584; continue 'dispatch;
	}
	// 830E1578: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E157C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E1580: 48000070  b 0x830e15f0
	pc = 0x830E15F0; continue 'dispatch;
	// 830E1584: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830E1588: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E158C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830E1590: 808BC078  lwz r4, -0x3f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16264 as u32) ) } as u64;
	// 830E1594: 4BFED5DD  bl 0x830ceb70
	ctx.lr = 0x830E1598;
	sub_830CEB70(ctx, base);
	// 830E1598: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E159C: 40820010  bne 0x830e15ac
	if !ctx.cr[0].eq {
	pc = 0x830E15AC; continue 'dispatch;
	}
	// 830E15A0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E15A4: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830E15A8: 48000048  b 0x830e15f0
	pc = 0x830E15F0; continue 'dispatch;
	// 830E15AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E15B0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830E15B4: 4BFFE21D  bl 0x830df7d0
	ctx.lr = 0x830E15B8;
	sub_830DF7D0(ctx, base);
	// 830E15B8: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E15BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E15C0: 409A0010  bne cr6, 0x830e15d0
	if !ctx.cr[6].eq {
	pc = 0x830E15D0; continue 'dispatch;
	}
	// 830E15C4: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E15C8: 60630017  ori r3, r3, 0x17
	ctx.r[3].u64 = ctx.r[3].u64 | 23;
	// 830E15CC: 48000024  b 0x830e15f0
	pc = 0x830E15F0; continue 'dispatch;
	// 830E15D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E15D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E15D8: 808BC07C  lwz r4, -0x3f84(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16260 as u32) ) } as u64;
	// 830E15DC: 4BFED595  bl 0x830ceb70
	ctx.lr = 0x830E15E0;
	sub_830CEB70(ctx, base);
	// 830E15E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E15E4: 4182FFE0  beq 0x830e15c4
	if ctx.cr[0].eq {
	pc = 0x830E15C4; continue 'dispatch;
	}
	// 830E15E8: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830E15EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E15F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E15F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E15F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E15FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E1600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E1604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1608 size=216
    let mut pc: u32 = 0x830E1608;
    'dispatch: loop {
        match pc {
            0x830E1608 => {
    //   block [0x830E1608..0x830E16E0)
	// 830E1608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E160C: 480C6B59  bl 0x831a8164
	ctx.lr = 0x830E1610;
	sub_831A8130(ctx, base);
	// 830E1610: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1614: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E1618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E161C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1620: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830E1624: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830E1628: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830E162C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830E1630: 4BFFFF29  bl 0x830e1558
	ctx.lr = 0x830E1634;
	sub_830E1558(ctx, base);
	// 830E1634: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1638: 418000A0  blt 0x830e16d8
	if ctx.cr[0].lt {
	pc = 0x830E16D8; continue 'dispatch;
	}
	// 830E163C: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1640: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 830E1644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830E1648: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E164C: 4BFFEB6D  bl 0x830e01b8
	ctx.lr = 0x830E1650;
	sub_830E01B8(ctx, base);
	// 830E1650: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 830E1654: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830E1658: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E165C: 4BFFEB5D  bl 0x830e01b8
	ctx.lr = 0x830E1660;
	sub_830E01B8(ctx, base);
	// 830E1660: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830E1664: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830E1668: 41980068  blt cr6, 0x830e16d0
	if ctx.cr[6].lt {
	pc = 0x830E16D0; continue 'dispatch;
	}
	// 830E166C: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830E1670: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830E1674: 4198005C  blt cr6, 0x830e16d0
	if ctx.cr[6].lt {
	pc = 0x830E16D0; continue 'dispatch;
	}
	// 830E1678: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830E167C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830E1680: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830E1684: 419A0038  beq cr6, 0x830e16bc
	if ctx.cr[6].eq {
	pc = 0x830E16BC; continue 'dispatch;
	}
	// 830E1688: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830E168C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830E1690: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1694: 4BFFEB25  bl 0x830e01b8
	ctx.lr = 0x830E1698;
	sub_830E01B8(ctx, base);
	// 830E1698: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E169C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 830E16A0: 41980030  blt cr6, 0x830e16d0
	if ctx.cr[6].lt {
	pc = 0x830E16D0; continue 'dispatch;
	}
	// 830E16A4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830E16A8: 7F052000  cmpw cr6, r5, r4
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[4].s32, &mut ctx.xer);
	// 830E16AC: 41980024  blt cr6, 0x830e16d0
	if ctx.cr[6].lt {
	pc = 0x830E16D0; continue 'dispatch;
	}
	// 830E16B0: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830E16B4: 7F053000  cmpw cr6, r5, r6
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830E16B8: 41990018  bgt cr6, 0x830e16d0
	if ctx.cr[6].gt {
	pc = 0x830E16D0; continue 'dispatch;
	}
	// 830E16BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830E16C0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830E16C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E16C8: 4BFFEA91  bl 0x830e0158
	ctx.lr = 0x830E16CC;
	sub_830E0158(ctx, base);
	// 830E16CC: 4800000C  b 0x830e16d8
	pc = 0x830E16D8; continue 'dispatch;
	// 830E16D0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E16D4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E16D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E16DC: 480C6AD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E16E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E16E0 size=116
    let mut pc: u32 = 0x830E16E0;
    'dispatch: loop {
        match pc {
            0x830E16E0 => {
    //   block [0x830E16E0..0x830E1754)
	// 830E16E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E16E4: 480C6A89  bl 0x831a816c
	ctx.lr = 0x830E16E8;
	sub_831A8130(ctx, base);
	// 830E16E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E16EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830E16F0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830E16F4: 41980048  blt cr6, 0x830e173c
	if ctx.cr[6].lt {
	pc = 0x830E173C; continue 'dispatch;
	}
	// 830E16F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830E16FC: 3BCB9140  addi r30, r11, -0x6ec0
	ctx.r[30].s64 = ctx.r[11].s64 + -28352;
	// 830E1700: 549F2036  slwi r31, r4, 4
	ctx.r[31].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 830E1704: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 830E1708: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 830E170C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 830E1710: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E1714: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1718: 7CDF582E  lwzx r6, r31, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830E171C: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830E1720: 4BFFFEE9  bl 0x830e1608
	ctx.lr = 0x830E1724;
	sub_830E1608(ctx, base);
	// 830E1724: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1728: 40800020  bge 0x830e1748
	if !ctx.cr[0].lt {
	pc = 0x830E1748; continue 'dispatch;
	}
	// 830E172C: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 830E1730: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830E1734: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830E1738: 4098FFC8  bge cr6, 0x830e1700
	if !ctx.cr[6].lt {
	pc = 0x830E1700; continue 'dispatch;
	}
	// 830E173C: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E1740: 60630026  ori r3, r3, 0x26
	ctx.r[3].u64 = ctx.r[3].u64 | 38;
	// 830E1744: 48000008  b 0x830e174c
	pc = 0x830E174C; continue 'dispatch;
	// 830E1748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E174C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E1750: 480C6A6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1758 size=260
    let mut pc: u32 = 0x830E1758;
    'dispatch: loop {
        match pc {
            0x830E1758 => {
    //   block [0x830E1758..0x830E185C)
	// 830E1758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E175C: 480C6A05  bl 0x831a8160
	ctx.lr = 0x830E1760;
	sub_831A8130(ctx, base);
	// 830E1760: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1764: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830E1768: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830E176C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830E1770: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830E1774: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830E1778: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830E177C: 419A00D0  beq cr6, 0x830e184c
	if ctx.cr[6].eq {
	pc = 0x830E184C; continue 'dispatch;
	}
	// 830E1780: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830E1784: 419A00C8  beq cr6, 0x830e184c
	if ctx.cr[6].eq {
	pc = 0x830E184C; continue 'dispatch;
	}
	// 830E1788: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830E178C: 419A00C0  beq cr6, 0x830e184c
	if ctx.cr[6].eq {
	pc = 0x830E184C; continue 'dispatch;
	}
	// 830E1790: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 830E1794: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830E1798: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830E179C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830E17A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830E17A4: 4BFFFE65  bl 0x830e1608
	ctx.lr = 0x830E17A8;
	sub_830E1608(ctx, base);
	// 830E17A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E17AC: 40800098  bge 0x830e1844
	if !ctx.cr[0].lt {
	pc = 0x830E1844; continue 'dispatch;
	}
	// 830E17B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E17B4: 419A0020  beq cr6, 0x830e17d4
	if ctx.cr[6].eq {
	pc = 0x830E17D4; continue 'dispatch;
	}
	// 830E17B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830E17BC: 419A0018  beq cr6, 0x830e17d4
	if ctx.cr[6].eq {
	pc = 0x830E17D4; continue 'dispatch;
	}
	// 830E17C0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 830E17C4: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 830E17C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830E17CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830E17D0: 4BFFFFB0  b 0x830e1780
	pc = 0x830E1780; continue 'dispatch;
	// 830E17D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830E17D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830E17DC: 3BCB9140  addi r30, r11, -0x6ec0
	ctx.r[30].s64 = ctx.r[11].s64 + -28352;
	// 830E17E0: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 830E17E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830E17E8: 809FFFFC  lwz r4, -4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830E17EC: 480D0645  bl 0x831b1e30
	ctx.lr = 0x830E17F0;
	sub_831B1E30(ctx, base);
	// 830E17F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E17F4: 40820018  bne 0x830e180c
	if !ctx.cr[0].eq {
	pc = 0x830E180C; continue 'dispatch;
	}
	// 830E17F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830E17FC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E1800: 480D0631  bl 0x831b1e30
	ctx.lr = 0x830E1804;
	sub_831B1E30(ctx, base);
	// 830E1804: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1808: 41820024  beq 0x830e182c
	if ctx.cr[0].eq {
	pc = 0x830E182C; continue 'dispatch;
	}
	// 830E180C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 830E1810: 397E0188  addi r11, r30, 0x188
	ctx.r[11].s64 = ctx.r[30].s64 + 392;
	// 830E1814: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830E1818: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830E181C: 4198FFC8  blt cr6, 0x830e17e4
	if ctx.cr[6].lt {
	pc = 0x830E17E4; continue 'dispatch;
	}
	// 830E1820: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E1824: 60630026  ori r3, r3, 0x26
	ctx.r[3].u64 = ctx.r[3].u64 | 38;
	// 830E1828: 4800002C  b 0x830e1854
	pc = 0x830E1854; continue 'dispatch;
	// 830E182C: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830E1830: 395E000C  addi r10, r30, 0xc
	ctx.r[10].s64 = ctx.r[30].s64 + 12;
	// 830E1834: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830E1838: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830E183C: 4BFFFEA5  bl 0x830e16e0
	ctx.lr = 0x830E1840;
	sub_830E16E0(ctx, base);
	// 830E1840: 48000014  b 0x830e1854
	pc = 0x830E1854; continue 'dispatch;
	// 830E1844: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1848: 4800000C  b 0x830e1854
	pc = 0x830E1854; continue 'dispatch;
	// 830E184C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E1850: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E1854: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E1858: 480C6958  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1860 size=16
    let mut pc: u32 = 0x830E1860;
    'dispatch: loop {
        match pc {
            0x830E1860 => {
    //   block [0x830E1860..0x830E1870)
	// 830E1860: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 830E1864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1868: 409A0008  bne cr6, 0x830e1870
	if !ctx.cr[6].eq {
		sub_830E1870(ctx, base);
		return;
	}
	// 830E186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1870 size=16
    let mut pc: u32 = 0x830E1870;
    'dispatch: loop {
        match pc {
            0x830E1870 => {
    //   block [0x830E1870..0x830E1880)
	// 830E1870: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830E1874: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830E1878: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830E187C: 4BFFF974  b 0x830e11f0
	sub_830E11F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1880 size=156
    let mut pc: u32 = 0x830E1880;
    'dispatch: loop {
        match pc {
            0x830E1880 => {
    //   block [0x830E1880..0x830E191C)
	// 830E1880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1884: 480C68E5  bl 0x831a8168
	ctx.lr = 0x830E1888;
	sub_831A8130(ctx, base);
	// 830E1888: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E188C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E1890: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E1894: 808BC080  lwz r4, -0x3f80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16256 as u32) ) } as u64;
	// 830E1898: 4BFFD251  bl 0x830deae8
	ctx.lr = 0x830E189C;
	sub_830DEAE8(ctx, base);
	// 830E189C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830E18A0: 41820074  beq 0x830e1914
	if ctx.cr[0].eq {
	pc = 0x830E1914; continue 'dispatch;
	}
	// 830E18A4: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 830E18A8: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830E18AC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830E18B0: 409A0064  bne cr6, 0x830e1914
	if !ctx.cr[6].eq {
	pc = 0x830E1914; continue 'dispatch;
	}
	// 830E18B4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830E18B8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830E18BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E18C0: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 830E18C4: 48022225  bl 0x83103ae8
	ctx.lr = 0x830E18C8;
	sub_83103AE8(ctx, base);
	// 830E18C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E18CC: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 830E18D0: 4BFED341  bl 0x830cec10
	ctx.lr = 0x830E18D4;
	sub_830CEC10(ctx, base);
	// 830E18D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830E18D8: 4BFF42F1  bl 0x830d5bc8
	ctx.lr = 0x830E18DC;
	sub_830D5BC8(ctx, base);
	// 830E18DC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E18E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830E18E4: 409A001C  bne cr6, 0x830e1900
	if !ctx.cr[6].eq {
	pc = 0x830E1900; continue 'dispatch;
	}
	// 830E18E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E18EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E18F0: 4BFFDE69  bl 0x830df758
	ctx.lr = 0x830E18F4;
	sub_830DF758(ctx, base);
	// 830E18F4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E18F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E18FC: 409AFFBC  bne cr6, 0x830e18b8
	if !ctx.cr[6].eq {
	pc = 0x830E18B8; continue 'dispatch;
	}
	// 830E1900: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1904: 4BFFF265  bl 0x830e0b68
	ctx.lr = 0x830E1908;
	sub_830E0B68(ctx, base);
	// 830E1908: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 830E190C: 556B07B8  rlwinm r11, r11, 0, 0x1e, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830E1910: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830E1914: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830E1918: 480C68A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1920 size=228
    let mut pc: u32 = 0x830E1920;
    'dispatch: loop {
        match pc {
            0x830E1920 => {
    //   block [0x830E1920..0x830E1A04)
	// 830E1920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1924: 480C6845  bl 0x831a8168
	ctx.lr = 0x830E1928;
	sub_831A8130(ctx, base);
	// 830E1928: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E192C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830E1930: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830E1934: 809FC080  lwz r4, -0x3f80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16256 as u32) ) } as u64;
	// 830E1938: 4BFFD1B1  bl 0x830deae8
	ctx.lr = 0x830E193C;
	sub_830DEAE8(ctx, base);
	// 830E193C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830E1940: 40820010  bne 0x830e1950
	if !ctx.cr[0].eq {
	pc = 0x830E1950; continue 'dispatch;
	}
	// 830E1944: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830E1948: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830E194C: 480000B0  b 0x830e19fc
	pc = 0x830E19FC; continue 'dispatch;
	// 830E1950: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 830E1954: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1958: 419A0034  beq cr6, 0x830e198c
	if ctx.cr[6].eq {
	pc = 0x830E198C; continue 'dispatch;
	}
	// 830E195C: 809FC080  lwz r4, -0x3f80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16256 as u32) ) } as u64;
	// 830E1960: 4BFFD189  bl 0x830deae8
	ctx.lr = 0x830E1964;
	sub_830DEAE8(ctx, base);
	// 830E1964: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E1968: 4182006C  beq 0x830e19d4
	if ctx.cr[0].eq {
	pc = 0x830E19D4; continue 'dispatch;
	}
	// 830E196C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 830E1970: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1974: 41820060  beq 0x830e19d4
	if ctx.cr[0].eq {
	pc = 0x830E19D4; continue 'dispatch;
	}
	// 830E1978: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 830E197C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830E1980: 48020671  bl 0x83101ff0
	ctx.lr = 0x830E1984;
	sub_83101FF0(ctx, base);
	// 830E1984: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 830E1988: 48000044  b 0x830e19cc
	pc = 0x830E19CC; continue 'dispatch;
	// 830E198C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830E1990: 48006971  bl 0x830e8300
	ctx.lr = 0x830E1994;
	sub_830E8300(ctx, base);
	// 830E1994: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830E1998: 4182003C  beq 0x830e19d4
	if ctx.cr[0].eq {
	pc = 0x830E19D4; continue 'dispatch;
	}
	// 830E199C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E19A0: 809FC080  lwz r4, -0x3f80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16256 as u32) ) } as u64;
	// 830E19A4: 4BFFD145  bl 0x830deae8
	ctx.lr = 0x830E19A8;
	sub_830DEAE8(ctx, base);
	// 830E19A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E19AC: 41820028  beq 0x830e19d4
	if ctx.cr[0].eq {
	pc = 0x830E19D4; continue 'dispatch;
	}
	// 830E19B0: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 830E19B4: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E19B8: 4182001C  beq 0x830e19d4
	if ctx.cr[0].eq {
	pc = 0x830E19D4; continue 'dispatch;
	}
	// 830E19BC: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 830E19C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830E19C4: 4802062D  bl 0x83101ff0
	ctx.lr = 0x830E19C8;
	sub_83101FF0(ctx, base);
	// 830E19C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E19CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E19D0: 4BFF41F9  bl 0x830d5bc8
	ctx.lr = 0x830E19D4;
	sub_830D5BC8(ctx, base);
	// 830E19D4: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 830E19D8: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E19DC: 4182001C  beq 0x830e19f8
	if ctx.cr[0].eq {
	pc = 0x830E19F8; continue 'dispatch;
	}
	// 830E19E0: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 830E19E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830E19E8: 48020609  bl 0x83101ff0
	ctx.lr = 0x830E19EC;
	sub_83101FF0(ctx, base);
	// 830E19EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E19F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830E19F4: 4BFF41D5  bl 0x830d5bc8
	ctx.lr = 0x830E19F8;
	sub_830D5BC8(ctx, base);
	// 830E19F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E19FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E1A00: 480C67B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1A08 size=104
    let mut pc: u32 = 0x830E1A08;
    'dispatch: loop {
        match pc {
            0x830E1A08 => {
    //   block [0x830E1A08..0x830E1A70)
	// 830E1A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E1A10: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 830E1A14: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 830E1A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1A1C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830E1A20: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830E1A24: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830E1A28: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830E1A2C: 4BFFD0BD  bl 0x830deae8
	ctx.lr = 0x830E1A30;
	sub_830DEAE8(ctx, base);
	// 830E1A30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E1A34: 40820010  bne 0x830e1a44
	if !ctx.cr[0].eq {
	pc = 0x830E1A44; continue 'dispatch;
	}
	// 830E1A38: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830E1A3C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 830E1A40: 48000018  b 0x830e1a58
	pc = 0x830E1A58; continue 'dispatch;
	// 830E1A44: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 830E1A48: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 830E1A4C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 830E1A50: 4BFF5129  bl 0x830d6b78
	ctx.lr = 0x830E1A54;
	sub_830D6B78(ctx, base);
	// 830E1A54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1A58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E1A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E1A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E1A64: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E1A68: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E1A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1A70 size=136
    let mut pc: u32 = 0x830E1A70;
    'dispatch: loop {
        match pc {
            0x830E1A70 => {
    //   block [0x830E1A70..0x830E1AF8)
	// 830E1A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E1A78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E1A7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E1A80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1A84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830E1A88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830E1A8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830E1A90: 419A004C  beq cr6, 0x830e1adc
	if ctx.cr[6].eq {
	pc = 0x830E1ADC; continue 'dispatch;
	}
	// 830E1A94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830E1A98: 419A0020  beq cr6, 0x830e1ab8
	if ctx.cr[6].eq {
	pc = 0x830E1AB8; continue 'dispatch;
	}
	// 830E1A9C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830E1AA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1AA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830E1AA8: 4BFFEB51  bl 0x830e05f8
	ctx.lr = 0x830E1AAC;
	sub_830E05F8(ctx, base);
	// 830E1AAC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E1AB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1AB4: 4BFF7E65  bl 0x830d9918
	ctx.lr = 0x830E1AB8;
	sub_830D9918(ctx, base);
	// 830E1AB8: 57EB07FE  clrlwi r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	// 830E1ABC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830E1AC0: 409A000C  bne cr6, 0x830e1acc
	if !ctx.cr[6].eq {
	pc = 0x830E1ACC; continue 'dispatch;
	}
	// 830E1AC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1AC8: 4802EBD1  bl 0x83110698
	ctx.lr = 0x830E1ACC;
	sub_83110698(ctx, base);
	// 830E1ACC: 57EB077A  rlwinm r11, r31, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 830E1AD0: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830E1AD4: 409A0008  bne cr6, 0x830e1adc
	if !ctx.cr[6].eq {
	pc = 0x830E1ADC; continue 'dispatch;
	}
	// 830E1AD8: 48028B29  bl 0x8310a600
	ctx.lr = 0x830E1ADC;
	sub_8310A600(ctx, base);
	// 830E1ADC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830E1AE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830E1AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E1AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E1AEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E1AF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E1AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1AF8 size=132
    let mut pc: u32 = 0x830E1AF8;
    'dispatch: loop {
        match pc {
            0x830E1AF8 => {
    //   block [0x830E1AF8..0x830E1B7C)
	// 830E1AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1AFC: 480C666D  bl 0x831a8168
	ctx.lr = 0x830E1B00;
	sub_831A8130(ctx, base);
	// 830E1B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1B04: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830E1B08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830E1B0C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1B10: 41820048  beq 0x830e1b58
	if ctx.cr[0].eq {
	pc = 0x830E1B58; continue 'dispatch;
	}
	// 830E1B14: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830E1B18: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 830E1B1C: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830E1B20: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1B24: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830E1B28: 41800018  blt 0x830e1b40
	if ctx.cr[0].lt {
	pc = 0x830E1B40; continue 'dispatch;
	}
	// 830E1B2C: 3BDEFFF0  addi r30, r30, -0x10
	ctx.r[30].s64 = ctx.r[30].s64 + -16;
	// 830E1B30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1B34: 48025195  bl 0x83106cc8
	ctx.lr = 0x830E1B38;
	sub_83106CC8(ctx, base);
	// 830E1B38: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1B3C: 4080FFF0  bge 0x830e1b2c
	if !ctx.cr[0].lt {
	pc = 0x830E1B2C; continue 'dispatch;
	}
	// 830E1B40: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1B44: 4182000C  beq 0x830e1b50
	if ctx.cr[0].eq {
	pc = 0x830E1B50; continue 'dispatch;
	}
	// 830E1B48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1B4C: 4BFFB98D  bl 0x830dd4d8
	ctx.lr = 0x830E1B50;
	sub_830DD4D8(ctx, base);
	// 830E1B50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1B54: 48000020  b 0x830e1b74
	pc = 0x830E1B74; continue 'dispatch;
	// 830E1B58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1B5C: 4802516D  bl 0x83106cc8
	ctx.lr = 0x830E1B60;
	sub_83106CC8(ctx, base);
	// 830E1B60: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1B64: 4182000C  beq 0x830e1b70
	if ctx.cr[0].eq {
	pc = 0x830E1B70; continue 'dispatch;
	}
	// 830E1B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1B6C: 4BFFB96D  bl 0x830dd4d8
	ctx.lr = 0x830E1B70;
	sub_830DD4D8(ctx, base);
	// 830E1B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830E1B78: 480C6640  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1B80 size=480
    let mut pc: u32 = 0x830E1B80;
    'dispatch: loop {
        match pc {
            0x830E1B80 => {
    //   block [0x830E1B80..0x830E1D60)
	// 830E1B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1B84: 480C65E1  bl 0x831a8164
	ctx.lr = 0x830E1B88;
	sub_831A8130(ctx, base);
	// 830E1B88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1B8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830E1B90: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830E1B94: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 830E1B98: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 830E1B9C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830E1BA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1BA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830E1BA8: 4BFFC8C9  bl 0x830de470
	ctx.lr = 0x830E1BAC;
	sub_830DE470(ctx, base);
	// 830E1BAC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1BB0: 418001A4  blt 0x830e1d54
	if ctx.cr[0].lt {
	pc = 0x830E1D54; continue 'dispatch;
	}
	// 830E1BB4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830E1BB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1BBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1BC0: 419A004C  beq cr6, 0x830e1c0c
	if ctx.cr[6].eq {
	pc = 0x830E1C0C; continue 'dispatch;
	}
	// 830E1BC4: 4BFFBD35  bl 0x830dd8f8
	ctx.lr = 0x830E1BC8;
	sub_830DD8F8(ctx, base);
	// 830E1BC8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830E1BCC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830E1BD0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1BD4: 4BFFBC55  bl 0x830dd828
	ctx.lr = 0x830E1BD8;
	sub_830DD828(ctx, base);
	// 830E1BD8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1BDC: 418000E4  blt 0x830e1cc0
	if ctx.cr[0].lt {
	pc = 0x830E1CC0; continue 'dispatch;
	}
	// 830E1BE0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830E1BE4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E1BE8: 4BFFE779  bl 0x830e0360
	ctx.lr = 0x830E1BEC;
	sub_830E0360(ctx, base);
	// 830E1BEC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830E1BF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E1BF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1BF8: 419A0024  beq cr6, 0x830e1c1c
	if ctx.cr[6].eq {
	pc = 0x830E1C1C; continue 'dispatch;
	}
	// 830E1BFC: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E1C00: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1C04: 4BFFBC55  bl 0x830dd858
	ctx.lr = 0x830E1C08;
	sub_830DD858(ctx, base);
	// 830E1C08: 48000024  b 0x830e1c2c
	pc = 0x830E1C2C; continue 'dispatch;
	// 830E1C0C: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 830E1C10: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830E1C14: 4BFFBD95  bl 0x830dd9a8
	ctx.lr = 0x830E1C18;
	sub_830DD9A8(ctx, base);
	// 830E1C18: 4BFFFFC0  b 0x830e1bd8
	pc = 0x830E1BD8; continue 'dispatch;
	// 830E1C1C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E1C20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1C24: 419A000C  beq cr6, 0x830e1c30
	if ctx.cr[6].eq {
	pc = 0x830E1C30; continue 'dispatch;
	}
	// 830E1C28: 4BFFB8B1  bl 0x830dd4d8
	ctx.lr = 0x830E1C2C;
	sub_830DD4D8(ctx, base);
	// 830E1C2C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830E1C30: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1C34: 4BFFBD0D  bl 0x830dd940
	ctx.lr = 0x830E1C38;
	sub_830DD940(ctx, base);
	// 830E1C38: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1C3C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830E1C40: 41980114  blt cr6, 0x830e1d54
	if ctx.cr[6].lt {
	pc = 0x830E1D54; continue 'dispatch;
	}
	// 830E1C44: 4BFED0C5  bl 0x830ced08
	ctx.lr = 0x830E1C48;
	sub_830CED08(ctx, base);
	// 830E1C48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830E1C4C: 41820108  beq 0x830e1d54
	if ctx.cr[0].eq {
	pc = 0x830E1D54; continue 'dispatch;
	}
	// 830E1C50: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E1C54: 4B1DE3AD  bl 0x822c0000
	ctx.lr = 0x830E1C58;
	sub_822C0000(ctx, base);
	// 830E1C58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830E1C5C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830E1C60: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830E1C64: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830E1C68: 409A00EC  bne cr6, 0x830e1d54
	if !ctx.cr[6].eq {
	pc = 0x830E1D54; continue 'dispatch;
	}
	// 830E1C6C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 830E1C70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1C74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830E1C78: 4BFFC991  bl 0x830de608
	ctx.lr = 0x830E1C7C;
	sub_830DE608(ctx, base);
	// 830E1C7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1C80: 418000D4  blt 0x830e1d54
	if ctx.cr[0].lt {
	pc = 0x830E1D54; continue 'dispatch;
	}
	// 830E1C84: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830E1C88: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1C8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1C90: 419A001C  beq cr6, 0x830e1cac
	if ctx.cr[6].eq {
	pc = 0x830E1CAC; continue 'dispatch;
	}
	// 830E1C94: 4BFFBC65  bl 0x830dd8f8
	ctx.lr = 0x830E1C98;
	sub_830DD8F8(ctx, base);
	// 830E1C98: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830E1C9C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830E1CA0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1CA4: 4BFFBB85  bl 0x830dd828
	ctx.lr = 0x830E1CA8;
	sub_830DD828(ctx, base);
	// 830E1CA8: 48000010  b 0x830e1cb8
	pc = 0x830E1CB8; continue 'dispatch;
	// 830E1CAC: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 830E1CB0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830E1CB4: 4BFFBCF5  bl 0x830dd9a8
	ctx.lr = 0x830E1CB8;
	sub_830DD9A8(ctx, base);
	// 830E1CB8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830E1CBC: 40800010  bge 0x830e1ccc
	if !ctx.cr[0].lt {
	pc = 0x830E1CCC; continue 'dispatch;
	}
	// 830E1CC0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1CC4: 4BFFBC7D  bl 0x830dd940
	ctx.lr = 0x830E1CC8;
	sub_830DD940(ctx, base);
	// 830E1CC8: 4800008C  b 0x830e1d54
	pc = 0x830E1D54; continue 'dispatch;
	// 830E1CCC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830E1CD0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E1CD4: 4BFFE68D  bl 0x830e0360
	ctx.lr = 0x830E1CD8;
	sub_830E0360(ctx, base);
	// 830E1CD8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830E1CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E1CE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830E1CE4: 419A0014  beq cr6, 0x830e1cf8
	if ctx.cr[6].eq {
	pc = 0x830E1CF8; continue 'dispatch;
	}
	// 830E1CE8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E1CEC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1CF0: 4BFFBB69  bl 0x830dd858
	ctx.lr = 0x830E1CF4;
	sub_830DD858(ctx, base);
	// 830E1CF4: 48000014  b 0x830e1d08
	pc = 0x830E1D08; continue 'dispatch;
	// 830E1CF8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830E1CFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1D00: 419A000C  beq cr6, 0x830e1d0c
	if ctx.cr[6].eq {
	pc = 0x830E1D0C; continue 'dispatch;
	}
	// 830E1D04: 4BFFB7D5  bl 0x830dd4d8
	ctx.lr = 0x830E1D08;
	sub_830DD4D8(ctx, base);
	// 830E1D08: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830E1D0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1D10: 4BFFBC31  bl 0x830dd940
	ctx.lr = 0x830E1D14;
	sub_830DD940(ctx, base);
	// 830E1D14: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1D18: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830E1D1C: 41980030  blt cr6, 0x830e1d4c
	if ctx.cr[6].lt {
	pc = 0x830E1D4C; continue 'dispatch;
	}
	// 830E1D20: 83C10060  lwz r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 830E1D24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1D28: 4B1DE2D9  bl 0x822c0000
	ctx.lr = 0x830E1D2C;
	sub_822C0000(ctx, base);
	// 830E1D2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830E1D30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830E1D34: 48021CCD  bl 0x83103a00
	ctx.lr = 0x830E1D38;
	sub_83103A00(ctx, base);
	// 830E1D38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E1D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830E1D40: 4BFFE6C1  bl 0x830e0400
	ctx.lr = 0x830E1D44;
	sub_830E0400(ctx, base);
	// 830E1D44: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830E1D48: 4098000C  bge cr6, 0x830e1d54
	if !ctx.cr[6].lt {
	pc = 0x830E1D54; continue 'dispatch;
	}
	// 830E1D4C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830E1D50: 4BFFE6B1  bl 0x830e0400
	ctx.lr = 0x830E1D54;
	sub_830E0400(ctx, base);
	// 830E1D54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1D58: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830E1D5C: 480C6458  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830E1D60 size=84
    let mut pc: u32 = 0x830E1D60;
    'dispatch: loop {
        match pc {
            0x830E1D60 => {
    //   block [0x830E1D60..0x830E1DB4)
	// 830E1D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830E1D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830E1D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830E1D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830E1D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830E1D74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830E1D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E1D7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830E1D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830E1D84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830E1D88: 4BFFD9D1  bl 0x830df758
	ctx.lr = 0x830E1D8C;
	sub_830DF758(ctx, base);
	// 830E1D8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830E1D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830E1D94: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830E1D98: 4BFFF1E1  bl 0x830e0f78
	ctx.lr = 0x830E1D9C;
	sub_830E0F78(ctx, base);
	// 830E1D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830E1DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830E1DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830E1DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830E1DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830E1DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1DB8 size=24
    let mut pc: u32 = 0x830E1DB8;
    'dispatch: loop {
        match pc {
            0x830E1DB8 => {
    //   block [0x830E1DB8..0x830E1DD0)
	// 830E1DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830E1DBC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830E1DC0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E1DC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1DC8: 409A0008  bne cr6, 0x830e1dd0
	if !ctx.cr[6].eq {
		sub_830E1DD0(ctx, base);
		return;
	}
	// 830E1DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1DD0 size=4
    let mut pc: u32 = 0x830E1DD0;
    'dispatch: loop {
        match pc {
            0x830E1DD0 => {
    //   block [0x830E1DD0..0x830E1DD4)
	// 830E1DD0: 4BFEC898  b 0x830ce668
	sub_830CE668(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1DD8 size=24
    let mut pc: u32 = 0x830E1DD8;
    'dispatch: loop {
        match pc {
            0x830E1DD8 => {
    //   block [0x830E1DD8..0x830E1DF0)
	// 830E1DD8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E1DDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1DE0: 409A0010  bne cr6, 0x830e1df0
	if !ctx.cr[6].eq {
		sub_830E1DF0(ctx, base);
		return;
	}
	// 830E1DE4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830E1DE8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830E1DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1DF0 size=4
    let mut pc: u32 = 0x830E1DF0;
    'dispatch: loop {
        match pc {
            0x830E1DF0 => {
    //   block [0x830E1DF0..0x830E1DF4)
	// 830E1DF0: 4BFEC900  b 0x830ce6f0
	sub_830CE6F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1DF8 size=24
    let mut pc: u32 = 0x830E1DF8;
    'dispatch: loop {
        match pc {
            0x830E1DF8 => {
    //   block [0x830E1DF8..0x830E1E10)
	// 830E1DF8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E1DFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1E00: 409A0010  bne cr6, 0x830e1e10
	if !ctx.cr[6].eq {
		sub_830E1E10(ctx, base);
		return;
	}
	// 830E1E04: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830E1E08: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830E1E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E10 size=4
    let mut pc: u32 = 0x830E1E10;
    'dispatch: loop {
        match pc {
            0x830E1E10 => {
    //   block [0x830E1E10..0x830E1E14)
	// 830E1E10: 4BFECB10  b 0x830ce920
	sub_830CE920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E18 size=24
    let mut pc: u32 = 0x830E1E18;
    'dispatch: loop {
        match pc {
            0x830E1E18 => {
    //   block [0x830E1E18..0x830E1E30)
	// 830E1E18: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E1E1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1E20: 409A0010  bne cr6, 0x830e1e30
	if !ctx.cr[6].eq {
		sub_830E1E30(ctx, base);
		return;
	}
	// 830E1E24: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830E1E28: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830E1E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E30 size=4
    let mut pc: u32 = 0x830E1E30;
    'dispatch: loop {
        match pc {
            0x830E1E30 => {
    //   block [0x830E1E30..0x830E1E34)
	// 830E1E30: 4BFECBC0  b 0x830ce9f0
	sub_830CE9F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E38 size=24
    let mut pc: u32 = 0x830E1E38;
    'dispatch: loop {
        match pc {
            0x830E1E38 => {
    //   block [0x830E1E38..0x830E1E50)
	// 830E1E38: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E1E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1E40: 409A0010  bne cr6, 0x830e1e50
	if !ctx.cr[6].eq {
		sub_830E1E50(ctx, base);
		return;
	}
	// 830E1E44: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830E1E48: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830E1E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E50 size=4
    let mut pc: u32 = 0x830E1E50;
    'dispatch: loop {
        match pc {
            0x830E1E50 => {
    //   block [0x830E1E50..0x830E1E54)
	// 830E1E50: 4BFF0498  b 0x830d22e8
	sub_830D22E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E58 size=24
    let mut pc: u32 = 0x830E1E58;
    'dispatch: loop {
        match pc {
            0x830E1E58 => {
    //   block [0x830E1E58..0x830E1E70)
	// 830E1E58: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830E1E5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830E1E60: 409A0010  bne cr6, 0x830e1e70
	if !ctx.cr[6].eq {
		sub_830E1E70(ctx, base);
		return;
	}
	// 830E1E64: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 830E1E68: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830E1E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830E1E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830E1E70 size=4
    let mut pc: u32 = 0x830E1E70;
    'dispatch: loop {
        match pc {
            0x830E1E70 => {
    //   block [0x830E1E70..0x830E1E74)
	// 830E1E70: 4BFF0588  b 0x830d23f8
	sub_830D23F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


