pub fn sub_825E6048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6048 size=128
    let mut pc: u32 = 0x825E6048;
    'dispatch: loop {
        match pc {
            0x825E6048 => {
    //   block [0x825E6048..0x825E60C8)
	// 825E6048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6058: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E605C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6060: 3BEBC730  addi r31, r11, -0x38d0
	ctx.r[31].s64 = ctx.r[11].s64 + -14544;
	// 825E6064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6068: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E606C: 4BF4F165  bl 0x825351d0
	ctx.lr = 0x825E6070;
	sub_825351D0(ctx, base);
	// 825E6070: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6074: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6078: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E607C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6080: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6084: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6088: 616B0065  ori r11, r11, 0x65
	ctx.r[11].u64 = ctx.r[11].u64 | 101;
	// 825E608C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6094: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6098: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E609C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E60A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E60A4: 616B0011  ori r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u64 | 17;
	// 825E60A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E60AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E60B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E60B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E60B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E60BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E60C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E60C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E60C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E60C8 size=128
    let mut pc: u32 = 0x825E60C8;
    'dispatch: loop {
        match pc {
            0x825E60C8 => {
    //   block [0x825E60C8..0x825E6148)
	// 825E60C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E60CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E60D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E60D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E60D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E60DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E60E0: 3BEBBFF8  addi r31, r11, -0x4008
	ctx.r[31].s64 = ctx.r[11].s64 + -16392;
	// 825E60E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E60E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E60EC: 4BF4F0E5  bl 0x825351d0
	ctx.lr = 0x825E60F0;
	sub_825351D0(ctx, base);
	// 825E60F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E60F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E60F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E60FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6100: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6104: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6108: 616B0066  ori r11, r11, 0x66
	ctx.r[11].u64 = ctx.r[11].u64 | 102;
	// 825E610C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6114: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6118: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E611C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6120: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6124: 616B0012  ori r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u64 | 18;
	// 825E6128: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E612C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6130: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E613C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6148 size=128
    let mut pc: u32 = 0x825E6148;
    'dispatch: loop {
        match pc {
            0x825E6148 => {
    //   block [0x825E6148..0x825E61C8)
	// 825E6148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E614C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6158: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E615C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6160: 3BEBC880  addi r31, r11, -0x3780
	ctx.r[31].s64 = ctx.r[11].s64 + -14208;
	// 825E6164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6168: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E616C: 4BF4F065  bl 0x825351d0
	ctx.lr = 0x825E6170;
	sub_825351D0(ctx, base);
	// 825E6170: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6174: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6178: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E617C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6180: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6184: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6188: 616B0067  ori r11, r11, 0x67
	ctx.r[11].u64 = ctx.r[11].u64 | 103;
	// 825E618C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6194: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6198: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E619C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E61A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E61A4: 616B0013  ori r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 | 19;
	// 825E61A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E61AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E61B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E61B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E61B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E61BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E61C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E61C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E61C8 size=128
    let mut pc: u32 = 0x825E61C8;
    'dispatch: loop {
        match pc {
            0x825E61C8 => {
    //   block [0x825E61C8..0x825E6248)
	// 825E61C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E61CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E61D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E61D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E61D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E61DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E61E0: 3BEBCC70  addi r31, r11, -0x3390
	ctx.r[31].s64 = ctx.r[11].s64 + -13200;
	// 825E61E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E61E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E61EC: 4BF4EFE5  bl 0x825351d0
	ctx.lr = 0x825E61F0;
	sub_825351D0(ctx, base);
	// 825E61F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E61F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E61F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E61FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6200: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6204: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6208: 616B0068  ori r11, r11, 0x68
	ctx.r[11].u64 = ctx.r[11].u64 | 104;
	// 825E620C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6214: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6218: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E621C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6220: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6224: 616B0014  ori r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u64 | 20;
	// 825E6228: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E622C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6230: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E623C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6248 size=128
    let mut pc: u32 = 0x825E6248;
    'dispatch: loop {
        match pc {
            0x825E6248 => {
    //   block [0x825E6248..0x825E62C8)
	// 825E6248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E624C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6258: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E625C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6260: 3BEBCCA8  addi r31, r11, -0x3358
	ctx.r[31].s64 = ctx.r[11].s64 + -13144;
	// 825E6264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6268: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E626C: 4BF4EF65  bl 0x825351d0
	ctx.lr = 0x825E6270;
	sub_825351D0(ctx, base);
	// 825E6270: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6278: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E627C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6280: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6284: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6288: 616B0069  ori r11, r11, 0x69
	ctx.r[11].u64 = ctx.r[11].u64 | 105;
	// 825E628C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6294: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6298: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E629C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E62A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E62A4: 616B0015  ori r11, r11, 0x15
	ctx.r[11].u64 = ctx.r[11].u64 | 21;
	// 825E62A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E62AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E62B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E62B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E62B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E62BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E62C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E62C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E62C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E62C8 size=128
    let mut pc: u32 = 0x825E62C8;
    'dispatch: loop {
        match pc {
            0x825E62C8 => {
    //   block [0x825E62C8..0x825E6348)
	// 825E62C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E62CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E62D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E62D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E62D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E62DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E62E0: 3BEBC4C8  addi r31, r11, -0x3b38
	ctx.r[31].s64 = ctx.r[11].s64 + -15160;
	// 825E62E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E62E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E62EC: 4BF4EEE5  bl 0x825351d0
	ctx.lr = 0x825E62F0;
	sub_825351D0(ctx, base);
	// 825E62F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E62F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E62F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E62FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6300: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6304: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6308: 616B006A  ori r11, r11, 0x6a
	ctx.r[11].u64 = ctx.r[11].u64 | 106;
	// 825E630C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6314: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6318: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E631C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6320: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6324: 616B0016  ori r11, r11, 0x16
	ctx.r[11].u64 = ctx.r[11].u64 | 22;
	// 825E6328: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E632C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6330: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E633C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6348 size=128
    let mut pc: u32 = 0x825E6348;
    'dispatch: loop {
        match pc {
            0x825E6348 => {
    //   block [0x825E6348..0x825E63C8)
	// 825E6348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6354: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6358: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E635C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6360: 3BEBC7A0  addi r31, r11, -0x3860
	ctx.r[31].s64 = ctx.r[11].s64 + -14432;
	// 825E6364: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6368: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E636C: 4BF4EE65  bl 0x825351d0
	ctx.lr = 0x825E6370;
	sub_825351D0(ctx, base);
	// 825E6370: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6378: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E637C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6380: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6384: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6388: 616B006B  ori r11, r11, 0x6b
	ctx.r[11].u64 = ctx.r[11].u64 | 107;
	// 825E638C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6394: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6398: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E639C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E63A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E63A4: 616B0017  ori r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 | 23;
	// 825E63A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E63AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E63B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E63B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E63B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E63BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E63C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E63C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E63C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E63C8 size=128
    let mut pc: u32 = 0x825E63C8;
    'dispatch: loop {
        match pc {
            0x825E63C8 => {
    //   block [0x825E63C8..0x825E6448)
	// 825E63C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E63CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E63D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E63D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E63D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E63DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E63E0: 3BEBCB90  addi r31, r11, -0x3470
	ctx.r[31].s64 = ctx.r[11].s64 + -13424;
	// 825E63E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E63E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E63EC: 4BF4EDE5  bl 0x825351d0
	ctx.lr = 0x825E63F0;
	sub_825351D0(ctx, base);
	// 825E63F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E63F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E63F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E63FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6400: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6404: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6408: 616B006D  ori r11, r11, 0x6d
	ctx.r[11].u64 = ctx.r[11].u64 | 109;
	// 825E640C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6414: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6418: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E641C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6420: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6424: 616B0019  ori r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u64 | 25;
	// 825E6428: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E642C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6430: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E643C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6448 size=128
    let mut pc: u32 = 0x825E6448;
    'dispatch: loop {
        match pc {
            0x825E6448 => {
    //   block [0x825E6448..0x825E64C8)
	// 825E6448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E644C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6458: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E645C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6460: 3BEBCD18  addi r31, r11, -0x32e8
	ctx.r[31].s64 = ctx.r[11].s64 + -13032;
	// 825E6464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6468: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E646C: 4BF4ED65  bl 0x825351d0
	ctx.lr = 0x825E6470;
	sub_825351D0(ctx, base);
	// 825E6470: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6478: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E647C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6480: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6484: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6488: 616B006E  ori r11, r11, 0x6e
	ctx.r[11].u64 = ctx.r[11].u64 | 110;
	// 825E648C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6494: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6498: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E649C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E64A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E64A4: 616B001A  ori r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u64 | 26;
	// 825E64A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E64AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E64B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E64B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E64B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E64BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E64C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E64C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E64C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E64C8 size=128
    let mut pc: u32 = 0x825E64C8;
    'dispatch: loop {
        match pc {
            0x825E64C8 => {
    //   block [0x825E64C8..0x825E6548)
	// 825E64C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E64CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E64D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E64D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E64D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E64DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E64E0: 3BEBC7D8  addi r31, r11, -0x3828
	ctx.r[31].s64 = ctx.r[11].s64 + -14376;
	// 825E64E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E64E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E64EC: 4BF4ECE5  bl 0x825351d0
	ctx.lr = 0x825E64F0;
	sub_825351D0(ctx, base);
	// 825E64F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E64F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E64F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E64FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6500: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6504: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6508: 616B006F  ori r11, r11, 0x6f
	ctx.r[11].u64 = ctx.r[11].u64 | 111;
	// 825E650C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6514: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6518: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E651C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6520: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6524: 616B001B  ori r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u64 | 27;
	// 825E6528: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E652C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6530: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E653C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6548 size=128
    let mut pc: u32 = 0x825E6548;
    'dispatch: loop {
        match pc {
            0x825E6548 => {
    //   block [0x825E6548..0x825E65C8)
	// 825E6548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E654C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6558: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E655C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6560: 3BEBC0A0  addi r31, r11, -0x3f60
	ctx.r[31].s64 = ctx.r[11].s64 + -16224;
	// 825E6564: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6568: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E656C: 4BF4EC65  bl 0x825351d0
	ctx.lr = 0x825E6570;
	sub_825351D0(ctx, base);
	// 825E6570: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6578: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E657C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6580: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6584: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6588: 616B0070  ori r11, r11, 0x70
	ctx.r[11].u64 = ctx.r[11].u64 | 112;
	// 825E658C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6594: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6598: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E659C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E65A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E65A4: 616B001C  ori r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u64 | 28;
	// 825E65A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E65AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E65B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E65B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E65B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E65BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E65C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E65C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E65C8 size=128
    let mut pc: u32 = 0x825E65C8;
    'dispatch: loop {
        match pc {
            0x825E65C8 => {
    //   block [0x825E65C8..0x825E6648)
	// 825E65C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E65CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E65D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E65D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E65D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E65DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E65E0: 3BEBC420  addi r31, r11, -0x3be0
	ctx.r[31].s64 = ctx.r[11].s64 + -15328;
	// 825E65E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E65E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E65EC: 4BF4EBE5  bl 0x825351d0
	ctx.lr = 0x825E65F0;
	sub_825351D0(ctx, base);
	// 825E65F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E65F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E65F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E65FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6600: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6604: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6608: 616B0072  ori r11, r11, 0x72
	ctx.r[11].u64 = ctx.r[11].u64 | 114;
	// 825E660C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6614: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6618: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E661C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6620: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6624: 616B001E  ori r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u64 | 30;
	// 825E6628: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E662C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6630: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E663C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6648 size=128
    let mut pc: u32 = 0x825E6648;
    'dispatch: loop {
        match pc {
            0x825E6648 => {
    //   block [0x825E6648..0x825E66C8)
	// 825E6648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E664C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6658: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E665C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6660: 3BEBC110  addi r31, r11, -0x3ef0
	ctx.r[31].s64 = ctx.r[11].s64 + -16112;
	// 825E6664: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6668: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E666C: 4BF4EB65  bl 0x825351d0
	ctx.lr = 0x825E6670;
	sub_825351D0(ctx, base);
	// 825E6670: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6678: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E667C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6680: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6684: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6688: 616B0073  ori r11, r11, 0x73
	ctx.r[11].u64 = ctx.r[11].u64 | 115;
	// 825E668C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6694: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6698: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E669C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E66A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E66A4: 616B001F  ori r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u64 | 31;
	// 825E66A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E66AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E66B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E66B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E66B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E66BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E66C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E66C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E66C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E66C8 size=128
    let mut pc: u32 = 0x825E66C8;
    'dispatch: loop {
        match pc {
            0x825E66C8 => {
    //   block [0x825E66C8..0x825E6748)
	// 825E66C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E66CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E66D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E66D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E66D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E66DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E66E0: 3BEBC6F8  addi r31, r11, -0x3908
	ctx.r[31].s64 = ctx.r[11].s64 + -14600;
	// 825E66E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E66E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E66EC: 4BF4EAE5  bl 0x825351d0
	ctx.lr = 0x825E66F0;
	sub_825351D0(ctx, base);
	// 825E66F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E66F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E66F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E66FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6700: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6704: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6708: 616B0074  ori r11, r11, 0x74
	ctx.r[11].u64 = ctx.r[11].u64 | 116;
	// 825E670C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6714: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6718: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E671C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6720: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6724: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 825E6728: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E672C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6730: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6748 size=128
    let mut pc: u32 = 0x825E6748;
    'dispatch: loop {
        match pc {
            0x825E6748 => {
    //   block [0x825E6748..0x825E67C8)
	// 825E6748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E674C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6758: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E675C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6760: 3BEBC340  addi r31, r11, -0x3cc0
	ctx.r[31].s64 = ctx.r[11].s64 + -15552;
	// 825E6764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6768: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E676C: 4BF4EA65  bl 0x825351d0
	ctx.lr = 0x825E6770;
	sub_825351D0(ctx, base);
	// 825E6770: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6774: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6778: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E677C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6780: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6784: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6788: 616B0075  ori r11, r11, 0x75
	ctx.r[11].u64 = ctx.r[11].u64 | 117;
	// 825E678C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6794: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6798: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E679C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E67A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E67A4: 616B0021  ori r11, r11, 0x21
	ctx.r[11].u64 = ctx.r[11].u64 | 33;
	// 825E67A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E67AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E67B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E67B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E67B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E67BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E67C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E67C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E67C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E67C8 size=128
    let mut pc: u32 = 0x825E67C8;
    'dispatch: loop {
        match pc {
            0x825E67C8 => {
    //   block [0x825E67C8..0x825E6848)
	// 825E67C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E67CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E67D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E67D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E67D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E67DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E67E0: 3BEBC960  addi r31, r11, -0x36a0
	ctx.r[31].s64 = ctx.r[11].s64 + -13984;
	// 825E67E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E67E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E67EC: 4BF4E9E5  bl 0x825351d0
	ctx.lr = 0x825E67F0;
	sub_825351D0(ctx, base);
	// 825E67F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E67F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E67F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E67FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6800: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6804: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6808: 616B0079  ori r11, r11, 0x79
	ctx.r[11].u64 = ctx.r[11].u64 | 121;
	// 825E680C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6814: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6818: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E681C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6820: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6824: 616B0026  ori r11, r11, 0x26
	ctx.r[11].u64 = ctx.r[11].u64 | 38;
	// 825E6828: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E682C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6830: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6848 size=128
    let mut pc: u32 = 0x825E6848;
    'dispatch: loop {
        match pc {
            0x825E6848 => {
    //   block [0x825E6848..0x825E68C8)
	// 825E6848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6858: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E685C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6860: 3BEBC570  addi r31, r11, -0x3a90
	ctx.r[31].s64 = ctx.r[11].s64 + -14992;
	// 825E6864: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6868: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E686C: 4BF4E965  bl 0x825351d0
	ctx.lr = 0x825E6870;
	sub_825351D0(ctx, base);
	// 825E6870: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6874: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6878: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E687C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6880: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6884: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6888: 616B007A  ori r11, r11, 0x7a
	ctx.r[11].u64 = ctx.r[11].u64 | 122;
	// 825E688C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6894: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6898: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E689C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E68A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E68A4: 616B0027  ori r11, r11, 0x27
	ctx.r[11].u64 = ctx.r[11].u64 | 39;
	// 825E68A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E68AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E68B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E68B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E68B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E68BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E68C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E68C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E68C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E68C8 size=128
    let mut pc: u32 = 0x825E68C8;
    'dispatch: loop {
        match pc {
            0x825E68C8 => {
    //   block [0x825E68C8..0x825E6948)
	// 825E68C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E68CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E68D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E68D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E68D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E68DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E68E0: 3BEBC2D0  addi r31, r11, -0x3d30
	ctx.r[31].s64 = ctx.r[11].s64 + -15664;
	// 825E68E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E68E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E68EC: 4BF4E8E5  bl 0x825351d0
	ctx.lr = 0x825E68F0;
	sub_825351D0(ctx, base);
	// 825E68F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E68F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E68F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E68FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6900: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6904: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6908: 616B007C  ori r11, r11, 0x7c
	ctx.r[11].u64 = ctx.r[11].u64 | 124;
	// 825E690C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6914: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6918: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E691C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6920: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6924: 616B0029  ori r11, r11, 0x29
	ctx.r[11].u64 = ctx.r[11].u64 | 41;
	// 825E6928: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E692C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6930: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E693C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6948 size=128
    let mut pc: u32 = 0x825E6948;
    'dispatch: loop {
        match pc {
            0x825E6948 => {
    //   block [0x825E6948..0x825E69C8)
	// 825E6948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E694C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6958: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E695C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6960: 3BEBCC38  addi r31, r11, -0x33c8
	ctx.r[31].s64 = ctx.r[11].s64 + -13256;
	// 825E6964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6968: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E696C: 4BF4E865  bl 0x825351d0
	ctx.lr = 0x825E6970;
	sub_825351D0(ctx, base);
	// 825E6970: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6974: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6978: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E697C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6980: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6984: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6988: 616B007D  ori r11, r11, 0x7d
	ctx.r[11].u64 = ctx.r[11].u64 | 125;
	// 825E698C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6994: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6998: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E699C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E69A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E69A4: 616B002A  ori r11, r11, 0x2a
	ctx.r[11].u64 = ctx.r[11].u64 | 42;
	// 825E69A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E69AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E69B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E69B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E69B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E69BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E69C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E69C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E69C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E69C8 size=128
    let mut pc: u32 = 0x825E69C8;
    'dispatch: loop {
        match pc {
            0x825E69C8 => {
    //   block [0x825E69C8..0x825E6A48)
	// 825E69C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E69CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E69D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E69D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E69D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E69DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E69E0: 3BEBC228  addi r31, r11, -0x3dd8
	ctx.r[31].s64 = ctx.r[11].s64 + -15832;
	// 825E69E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E69E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E69EC: 4BF4E7E5  bl 0x825351d0
	ctx.lr = 0x825E69F0;
	sub_825351D0(ctx, base);
	// 825E69F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E69F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E69F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E69FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6A00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6A04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6A08: 616B007E  ori r11, r11, 0x7e
	ctx.r[11].u64 = ctx.r[11].u64 | 126;
	// 825E6A0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6A14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6A18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6A1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6A20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6A24: 616B002B  ori r11, r11, 0x2b
	ctx.r[11].u64 = ctx.r[11].u64 | 43;
	// 825E6A28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6A30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6A34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6A40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6A48 size=128
    let mut pc: u32 = 0x825E6A48;
    'dispatch: loop {
        match pc {
            0x825E6A48 => {
    //   block [0x825E6A48..0x825E6AC8)
	// 825E6A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6A58: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6A5C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6A60: 3BEBC180  addi r31, r11, -0x3e80
	ctx.r[31].s64 = ctx.r[11].s64 + -16000;
	// 825E6A64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6A68: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6A6C: 4BF4E765  bl 0x825351d0
	ctx.lr = 0x825E6A70;
	sub_825351D0(ctx, base);
	// 825E6A70: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6A74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6A78: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6A7C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6A80: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6A84: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6A88: 616B005E  ori r11, r11, 0x5e
	ctx.r[11].u64 = ctx.r[11].u64 | 94;
	// 825E6A8C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6A94: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6A98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6A9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6AA0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6AA4: 616B000A  ori r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u64 | 10;
	// 825E6AA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6AAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6AB0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6AB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6AC8 size=128
    let mut pc: u32 = 0x825E6AC8;
    'dispatch: loop {
        match pc {
            0x825E6AC8 => {
    //   block [0x825E6AC8..0x825E6B48)
	// 825E6AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6AD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6AD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6AD8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6ADC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6AE0: 3BEBC688  addi r31, r11, -0x3978
	ctx.r[31].s64 = ctx.r[11].s64 + -14712;
	// 825E6AE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6AE8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6AEC: 4BF4E6E5  bl 0x825351d0
	ctx.lr = 0x825E6AF0;
	sub_825351D0(ctx, base);
	// 825E6AF0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6AF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6AF8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6AFC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6B00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6B04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6B08: 616B005F  ori r11, r11, 0x5f
	ctx.r[11].u64 = ctx.r[11].u64 | 95;
	// 825E6B0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6B14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6B18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6B1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6B20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6B24: 616B000B  ori r11, r11, 0xb
	ctx.r[11].u64 = ctx.r[11].u64 | 11;
	// 825E6B28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6B2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6B30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6B48 size=128
    let mut pc: u32 = 0x825E6B48;
    'dispatch: loop {
        match pc {
            0x825E6B48 => {
    //   block [0x825E6B48..0x825E6BC8)
	// 825E6B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6B58: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6B5C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6B60: 3BEBC538  addi r31, r11, -0x3ac8
	ctx.r[31].s64 = ctx.r[11].s64 + -15048;
	// 825E6B64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6B68: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6B6C: 4BF4E665  bl 0x825351d0
	ctx.lr = 0x825E6B70;
	sub_825351D0(ctx, base);
	// 825E6B70: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6B74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6B78: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6B7C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6B80: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6B84: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6B88: 616B0060  ori r11, r11, 0x60
	ctx.r[11].u64 = ctx.r[11].u64 | 96;
	// 825E6B8C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6B94: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6B98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6B9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6BA0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6BA4: 616B000C  ori r11, r11, 0xc
	ctx.r[11].u64 = ctx.r[11].u64 | 12;
	// 825E6BA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6BAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6BB0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6BB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6BC8 size=128
    let mut pc: u32 = 0x825E6BC8;
    'dispatch: loop {
        match pc {
            0x825E6BC8 => {
    //   block [0x825E6BC8..0x825E6C48)
	// 825E6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6BD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6BD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6BD8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6BDC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6BE0: 3BEBC1B8  addi r31, r11, -0x3e48
	ctx.r[31].s64 = ctx.r[11].s64 + -15944;
	// 825E6BE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6BE8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6BEC: 4BF4E5E5  bl 0x825351d0
	ctx.lr = 0x825E6BF0;
	sub_825351D0(ctx, base);
	// 825E6BF0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6BF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6BF8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6BFC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6C00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6C04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6C08: 616B0061  ori r11, r11, 0x61
	ctx.r[11].u64 = ctx.r[11].u64 | 97;
	// 825E6C0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6C14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6C18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6C1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6C20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6C24: 616B000D  ori r11, r11, 0xd
	ctx.r[11].u64 = ctx.r[11].u64 | 13;
	// 825E6C28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6C2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6C30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6C40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6C48 size=128
    let mut pc: u32 = 0x825E6C48;
    'dispatch: loop {
        match pc {
            0x825E6C48 => {
    //   block [0x825E6C48..0x825E6CC8)
	// 825E6C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6C50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6C54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6C58: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6C5C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6C60: 3BEBC618  addi r31, r11, -0x39e8
	ctx.r[31].s64 = ctx.r[11].s64 + -14824;
	// 825E6C64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6C68: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6C6C: 4BF4E565  bl 0x825351d0
	ctx.lr = 0x825E6C70;
	sub_825351D0(ctx, base);
	// 825E6C70: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6C74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6C78: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6C7C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6C80: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6C84: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6C88: 616B005E  ori r11, r11, 0x5e
	ctx.r[11].u64 = ctx.r[11].u64 | 94;
	// 825E6C8C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6C94: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6C98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6C9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6CA0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6CA4: 616B000A  ori r11, r11, 0xa
	ctx.r[11].u64 = ctx.r[11].u64 | 10;
	// 825E6CA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6CAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6CB0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6CB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6CC8 size=128
    let mut pc: u32 = 0x825E6CC8;
    'dispatch: loop {
        match pc {
            0x825E6CC8 => {
    //   block [0x825E6CC8..0x825E6D48)
	// 825E6CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6CD8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6CDC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6CE0: 3BEBC378  addi r31, r11, -0x3c88
	ctx.r[31].s64 = ctx.r[11].s64 + -15496;
	// 825E6CE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6CE8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6CEC: 4BF4E4E5  bl 0x825351d0
	ctx.lr = 0x825E6CF0;
	sub_825351D0(ctx, base);
	// 825E6CF0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6CF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6CF8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6CFC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6D00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6D04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6D08: 616B005F  ori r11, r11, 0x5f
	ctx.r[11].u64 = ctx.r[11].u64 | 95;
	// 825E6D0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6D14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6D1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6D20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6D24: 616B000B  ori r11, r11, 0xb
	ctx.r[11].u64 = ctx.r[11].u64 | 11;
	// 825E6D28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6D2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6D30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6D34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6D40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6D48 size=128
    let mut pc: u32 = 0x825E6D48;
    'dispatch: loop {
        match pc {
            0x825E6D48 => {
    //   block [0x825E6D48..0x825E6DC8)
	// 825E6D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6D58: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6D5C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6D60: 3BEBC8B8  addi r31, r11, -0x3748
	ctx.r[31].s64 = ctx.r[11].s64 + -14152;
	// 825E6D64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6D68: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6D6C: 4BF4E465  bl 0x825351d0
	ctx.lr = 0x825E6D70;
	sub_825351D0(ctx, base);
	// 825E6D70: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6D74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6D78: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6D7C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6D80: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6D84: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6D88: 616B0060  ori r11, r11, 0x60
	ctx.r[11].u64 = ctx.r[11].u64 | 96;
	// 825E6D8C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6D94: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6D98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6D9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6DA0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6DA4: 616B000C  ori r11, r11, 0xc
	ctx.r[11].u64 = ctx.r[11].u64 | 12;
	// 825E6DA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6DAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6DB0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6DB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6DC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6DC8 size=128
    let mut pc: u32 = 0x825E6DC8;
    'dispatch: loop {
        match pc {
            0x825E6DC8 => {
    //   block [0x825E6DC8..0x825E6E48)
	// 825E6DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6DD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6DD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6DD8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6DDC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6DE0: 3BEBC030  addi r31, r11, -0x3fd0
	ctx.r[31].s64 = ctx.r[11].s64 + -16336;
	// 825E6DE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6DE8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6DEC: 4BF4E3E5  bl 0x825351d0
	ctx.lr = 0x825E6DF0;
	sub_825351D0(ctx, base);
	// 825E6DF0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6DF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6DF8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6DFC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6E00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6E04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6E08: 616B0061  ori r11, r11, 0x61
	ctx.r[11].u64 = ctx.r[11].u64 | 97;
	// 825E6E0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6E14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6E18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6E1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6E20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6E24: 616B000D  ori r11, r11, 0xd
	ctx.r[11].u64 = ctx.r[11].u64 | 13;
	// 825E6E28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6E30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6E34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6E48 size=128
    let mut pc: u32 = 0x825E6E48;
    'dispatch: loop {
        match pc {
            0x825E6E48 => {
    //   block [0x825E6E48..0x825E6EC8)
	// 825E6E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6E58: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6E5C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6E60: 3BEBC928  addi r31, r11, -0x36d8
	ctx.r[31].s64 = ctx.r[11].s64 + -14040;
	// 825E6E64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6E68: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6E6C: 4BF4E365  bl 0x825351d0
	ctx.lr = 0x825E6E70;
	sub_825351D0(ctx, base);
	// 825E6E70: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6E74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6E78: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6E7C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6E80: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6E84: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6E88: 616B0103  ori r11, r11, 0x103
	ctx.r[11].u64 = ctx.r[11].u64 | 259;
	// 825E6E8C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6E94: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6E98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6E9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6EA0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6EA4: 616B00FF  ori r11, r11, 0xff
	ctx.r[11].u64 = ctx.r[11].u64 | 255;
	// 825E6EA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6EAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6EB0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6EC8 size=128
    let mut pc: u32 = 0x825E6EC8;
    'dispatch: loop {
        match pc {
            0x825E6EC8 => {
    //   block [0x825E6EC8..0x825E6F48)
	// 825E6EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6ED0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6ED8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6EDC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6EE0: 3BEBC8F0  addi r31, r11, -0x3710
	ctx.r[31].s64 = ctx.r[11].s64 + -14096;
	// 825E6EE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6EE8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6EEC: 4BF4E2E5  bl 0x825351d0
	ctx.lr = 0x825E6EF0;
	sub_825351D0(ctx, base);
	// 825E6EF0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6EF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6EF8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6EFC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6F00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6F04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6F08: 616B0104  ori r11, r11, 0x104
	ctx.r[11].u64 = ctx.r[11].u64 | 260;
	// 825E6F0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6F14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6F18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6F1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6F20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6F24: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 825E6F28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6F2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6F30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6F48 size=128
    let mut pc: u32 = 0x825E6F48;
    'dispatch: loop {
        match pc {
            0x825E6F48 => {
    //   block [0x825E6F48..0x825E6FC8)
	// 825E6F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6F50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6F54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6F58: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6F5C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6F60: 3BEBC458  addi r31, r11, -0x3ba8
	ctx.r[31].s64 = ctx.r[11].s64 + -15272;
	// 825E6F64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6F68: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6F6C: 4BF4E265  bl 0x825351d0
	ctx.lr = 0x825E6F70;
	sub_825351D0(ctx, base);
	// 825E6F70: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6F74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6F78: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6F7C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E6F80: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E6F84: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6F88: 616B0105  ori r11, r11, 0x105
	ctx.r[11].u64 = ctx.r[11].u64 | 261;
	// 825E6F8C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E6F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6F94: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E6F98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E6F9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E6FA0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E6FA4: 616B0101  ori r11, r11, 0x101
	ctx.r[11].u64 = ctx.r[11].u64 | 257;
	// 825E6FA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E6FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E6FB0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E6FB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E6FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E6FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E6FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E6FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E6FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E6FC8 size=128
    let mut pc: u32 = 0x825E6FC8;
    'dispatch: loop {
        match pc {
            0x825E6FC8 => {
    //   block [0x825E6FC8..0x825E7048)
	// 825E6FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E6FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E6FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E6FD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E6FD8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E6FDC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E6FE0: 3BEBC768  addi r31, r11, -0x3898
	ctx.r[31].s64 = ctx.r[11].s64 + -14488;
	// 825E6FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E6FE8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E6FEC: 4BF4E1E5  bl 0x825351d0
	ctx.lr = 0x825E6FF0;
	sub_825351D0(ctx, base);
	// 825E6FF0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E6FF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E6FF8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E6FFC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7000: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7004: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7008: 616B0106  ori r11, r11, 0x106
	ctx.r[11].u64 = ctx.r[11].u64 | 262;
	// 825E700C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7014: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7018: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E701C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7020: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7024: 616B0102  ori r11, r11, 0x102
	ctx.r[11].u64 = ctx.r[11].u64 | 258;
	// 825E7028: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E702C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7030: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E703C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7048 size=128
    let mut pc: u32 = 0x825E7048;
    'dispatch: loop {
        match pc {
            0x825E7048 => {
    //   block [0x825E7048..0x825E70C8)
	// 825E7048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E704C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7058: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E705C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7060: 3BEBC0D8  addi r31, r11, -0x3f28
	ctx.r[31].s64 = ctx.r[11].s64 + -16168;
	// 825E7064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7068: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E706C: 4BF4E165  bl 0x825351d0
	ctx.lr = 0x825E7070;
	sub_825351D0(ctx, base);
	// 825E7070: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7074: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7078: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E707C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7080: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7084: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7088: 616B0065  ori r11, r11, 0x65
	ctx.r[11].u64 = ctx.r[11].u64 | 101;
	// 825E708C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7094: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7098: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E709C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E70A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E70A4: 616B0011  ori r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u64 | 17;
	// 825E70A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E70AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E70B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E70B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E70B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E70BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E70C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E70C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E70C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E70C8 size=128
    let mut pc: u32 = 0x825E70C8;
    'dispatch: loop {
        match pc {
            0x825E70C8 => {
    //   block [0x825E70C8..0x825E7148)
	// 825E70C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E70CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E70D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E70D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E70D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E70DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E70E0: 3BEBC5A8  addi r31, r11, -0x3a58
	ctx.r[31].s64 = ctx.r[11].s64 + -14936;
	// 825E70E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E70E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E70EC: 4BF4E0E5  bl 0x825351d0
	ctx.lr = 0x825E70F0;
	sub_825351D0(ctx, base);
	// 825E70F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E70F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E70F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E70FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7100: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7104: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7108: 616B0066  ori r11, r11, 0x66
	ctx.r[11].u64 = ctx.r[11].u64 | 102;
	// 825E710C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7114: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7118: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E711C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7120: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7124: 616B0012  ori r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u64 | 18;
	// 825E7128: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E712C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7130: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E713C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7148 size=128
    let mut pc: u32 = 0x825E7148;
    'dispatch: loop {
        match pc {
            0x825E7148 => {
    //   block [0x825E7148..0x825E71C8)
	// 825E7148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E714C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7158: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E715C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7160: 3BEBC3E8  addi r31, r11, -0x3c18
	ctx.r[31].s64 = ctx.r[11].s64 + -15384;
	// 825E7164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7168: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E716C: 4BF4E065  bl 0x825351d0
	ctx.lr = 0x825E7170;
	sub_825351D0(ctx, base);
	// 825E7170: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7174: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7178: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E717C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7180: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7184: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7188: 616B0067  ori r11, r11, 0x67
	ctx.r[11].u64 = ctx.r[11].u64 | 103;
	// 825E718C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7194: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7198: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E719C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E71A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E71A4: 616B0013  ori r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u64 | 19;
	// 825E71A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E71AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E71B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E71B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E71B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E71BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E71C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E71C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E71C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E71C8 size=128
    let mut pc: u32 = 0x825E71C8;
    'dispatch: loop {
        match pc {
            0x825E71C8 => {
    //   block [0x825E71C8..0x825E7248)
	// 825E71C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E71CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E71D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E71D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E71D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E71DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E71E0: 3BEBCA40  addi r31, r11, -0x35c0
	ctx.r[31].s64 = ctx.r[11].s64 + -13760;
	// 825E71E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E71E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E71EC: 4BF4DFE5  bl 0x825351d0
	ctx.lr = 0x825E71F0;
	sub_825351D0(ctx, base);
	// 825E71F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E71F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E71F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E71FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7200: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7204: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7208: 616B0068  ori r11, r11, 0x68
	ctx.r[11].u64 = ctx.r[11].u64 | 104;
	// 825E720C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7214: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7218: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E721C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7220: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7224: 616B0014  ori r11, r11, 0x14
	ctx.r[11].u64 = ctx.r[11].u64 | 20;
	// 825E7228: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E722C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7230: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E723C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7248 size=128
    let mut pc: u32 = 0x825E7248;
    'dispatch: loop {
        match pc {
            0x825E7248 => {
    //   block [0x825E7248..0x825E72C8)
	// 825E7248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E724C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7258: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E725C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7260: 3BEBC5E0  addi r31, r11, -0x3a20
	ctx.r[31].s64 = ctx.r[11].s64 + -14880;
	// 825E7264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7268: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E726C: 4BF4DF65  bl 0x825351d0
	ctx.lr = 0x825E7270;
	sub_825351D0(ctx, base);
	// 825E7270: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7278: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E727C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7280: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7284: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7288: 616B0069  ori r11, r11, 0x69
	ctx.r[11].u64 = ctx.r[11].u64 | 105;
	// 825E728C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7294: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7298: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E729C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E72A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E72A4: 616B0015  ori r11, r11, 0x15
	ctx.r[11].u64 = ctx.r[11].u64 | 21;
	// 825E72A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E72AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E72B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E72B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E72B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E72BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E72C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E72C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E72C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E72C8 size=128
    let mut pc: u32 = 0x825E72C8;
    'dispatch: loop {
        match pc {
            0x825E72C8 => {
    //   block [0x825E72C8..0x825E7348)
	// 825E72C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E72CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E72D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E72D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E72D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E72DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E72E0: 3BEBC260  addi r31, r11, -0x3da0
	ctx.r[31].s64 = ctx.r[11].s64 + -15776;
	// 825E72E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E72E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E72EC: 4BF4DEE5  bl 0x825351d0
	ctx.lr = 0x825E72F0;
	sub_825351D0(ctx, base);
	// 825E72F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E72F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E72F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E72FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7300: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7304: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7308: 616B006A  ori r11, r11, 0x6a
	ctx.r[11].u64 = ctx.r[11].u64 | 106;
	// 825E730C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7314: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7318: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E731C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7320: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7324: 616B0016  ori r11, r11, 0x16
	ctx.r[11].u64 = ctx.r[11].u64 | 22;
	// 825E7328: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E732C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7330: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E733C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7348 size=128
    let mut pc: u32 = 0x825E7348;
    'dispatch: loop {
        match pc {
            0x825E7348 => {
    //   block [0x825E7348..0x825E73C8)
	// 825E7348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7354: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7358: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E735C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7360: 3BEBC650  addi r31, r11, -0x39b0
	ctx.r[31].s64 = ctx.r[11].s64 + -14768;
	// 825E7364: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7368: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E736C: 4BF4DE65  bl 0x825351d0
	ctx.lr = 0x825E7370;
	sub_825351D0(ctx, base);
	// 825E7370: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7378: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E737C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7380: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7384: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7388: 616B006B  ori r11, r11, 0x6b
	ctx.r[11].u64 = ctx.r[11].u64 | 107;
	// 825E738C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7394: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7398: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E739C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E73A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E73A4: 616B0017  ori r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u64 | 23;
	// 825E73A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E73AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E73B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E73B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E73B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E73BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E73C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E73C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E73C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E73C8 size=128
    let mut pc: u32 = 0x825E73C8;
    'dispatch: loop {
        match pc {
            0x825E73C8 => {
    //   block [0x825E73C8..0x825E7448)
	// 825E73C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E73CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E73D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E73D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E73D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E73DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E73E0: 3BEBC068  addi r31, r11, -0x3f98
	ctx.r[31].s64 = ctx.r[11].s64 + -16280;
	// 825E73E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E73E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E73EC: 4BF4DDE5  bl 0x825351d0
	ctx.lr = 0x825E73F0;
	sub_825351D0(ctx, base);
	// 825E73F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E73F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E73F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E73FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7400: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7404: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7408: 616B006D  ori r11, r11, 0x6d
	ctx.r[11].u64 = ctx.r[11].u64 | 109;
	// 825E740C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7414: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7418: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E741C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7420: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7424: 616B0019  ori r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u64 | 25;
	// 825E7428: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E742C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7430: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7448 size=128
    let mut pc: u32 = 0x825E7448;
    'dispatch: loop {
        match pc {
            0x825E7448 => {
    //   block [0x825E7448..0x825E74C8)
	// 825E7448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7458: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E745C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7460: 3BEBC500  addi r31, r11, -0x3b00
	ctx.r[31].s64 = ctx.r[11].s64 + -15104;
	// 825E7464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7468: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E746C: 4BF4DD65  bl 0x825351d0
	ctx.lr = 0x825E7470;
	sub_825351D0(ctx, base);
	// 825E7470: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7478: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E747C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7480: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7484: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7488: 616B006E  ori r11, r11, 0x6e
	ctx.r[11].u64 = ctx.r[11].u64 | 110;
	// 825E748C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7494: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7498: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E749C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E74A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E74A4: 616B001A  ori r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u64 | 26;
	// 825E74A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E74AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E74B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E74B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E74B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E74BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E74C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E74C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E74C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E74C8 size=128
    let mut pc: u32 = 0x825E74C8;
    'dispatch: loop {
        match pc {
            0x825E74C8 => {
    //   block [0x825E74C8..0x825E7548)
	// 825E74C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E74CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E74D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E74D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E74D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E74DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E74E0: 3BEBC308  addi r31, r11, -0x3cf8
	ctx.r[31].s64 = ctx.r[11].s64 + -15608;
	// 825E74E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E74E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E74EC: 4BF4DCE5  bl 0x825351d0
	ctx.lr = 0x825E74F0;
	sub_825351D0(ctx, base);
	// 825E74F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E74F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E74F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E74FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7500: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7504: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7508: 616B006F  ori r11, r11, 0x6f
	ctx.r[11].u64 = ctx.r[11].u64 | 111;
	// 825E750C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7514: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7518: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E751C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7520: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7524: 616B001B  ori r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u64 | 27;
	// 825E7528: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E752C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7530: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7548 size=128
    let mut pc: u32 = 0x825E7548;
    'dispatch: loop {
        match pc {
            0x825E7548 => {
    //   block [0x825E7548..0x825E75C8)
	// 825E7548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7558: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E755C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7560: 3BEBC998  addi r31, r11, -0x3668
	ctx.r[31].s64 = ctx.r[11].s64 + -13928;
	// 825E7564: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7568: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E756C: 4BF4DC65  bl 0x825351d0
	ctx.lr = 0x825E7570;
	sub_825351D0(ctx, base);
	// 825E7570: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7578: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E757C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7580: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7584: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7588: 616B0070  ori r11, r11, 0x70
	ctx.r[11].u64 = ctx.r[11].u64 | 112;
	// 825E758C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7594: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7598: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E759C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E75A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E75A4: 616B001C  ori r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u64 | 28;
	// 825E75A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E75AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E75B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E75B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E75B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E75BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E75C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E75C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E75C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E75C8 size=128
    let mut pc: u32 = 0x825E75C8;
    'dispatch: loop {
        match pc {
            0x825E75C8 => {
    //   block [0x825E75C8..0x825E7648)
	// 825E75C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E75CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E75D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E75D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E75D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E75DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E75E0: 3BEBC490  addi r31, r11, -0x3b70
	ctx.r[31].s64 = ctx.r[11].s64 + -15216;
	// 825E75E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E75E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E75EC: 4BF4DBE5  bl 0x825351d0
	ctx.lr = 0x825E75F0;
	sub_825351D0(ctx, base);
	// 825E75F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E75F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E75F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E75FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7600: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7604: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7608: 616B0072  ori r11, r11, 0x72
	ctx.r[11].u64 = ctx.r[11].u64 | 114;
	// 825E760C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7614: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7618: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E761C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7620: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7624: 616B001E  ori r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u64 | 30;
	// 825E7628: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E762C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7630: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E763C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7648 size=128
    let mut pc: u32 = 0x825E7648;
    'dispatch: loop {
        match pc {
            0x825E7648 => {
    //   block [0x825E7648..0x825E76C8)
	// 825E7648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E764C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7658: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E765C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7660: 3BEBC848  addi r31, r11, -0x37b8
	ctx.r[31].s64 = ctx.r[11].s64 + -14264;
	// 825E7664: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7668: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E766C: 4BF4DB65  bl 0x825351d0
	ctx.lr = 0x825E7670;
	sub_825351D0(ctx, base);
	// 825E7670: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7678: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E767C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7680: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7684: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7688: 616B0073  ori r11, r11, 0x73
	ctx.r[11].u64 = ctx.r[11].u64 | 115;
	// 825E768C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7694: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7698: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E769C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E76A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E76A4: 616B001F  ori r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u64 | 31;
	// 825E76A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E76AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E76B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E76B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E76B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E76BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E76C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E76C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E76C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E76C8 size=128
    let mut pc: u32 = 0x825E76C8;
    'dispatch: loop {
        match pc {
            0x825E76C8 => {
    //   block [0x825E76C8..0x825E7748)
	// 825E76C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E76CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E76D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E76D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E76D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E76DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E76E0: 3BEBBFC0  addi r31, r11, -0x4040
	ctx.r[31].s64 = ctx.r[11].s64 + -16448;
	// 825E76E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E76E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E76EC: 4BF4DAE5  bl 0x825351d0
	ctx.lr = 0x825E76F0;
	sub_825351D0(ctx, base);
	// 825E76F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E76F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E76F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E76FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7700: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7704: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7708: 616B0074  ori r11, r11, 0x74
	ctx.r[11].u64 = ctx.r[11].u64 | 116;
	// 825E770C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7714: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7718: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E771C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7720: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7724: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 825E7728: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E772C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7730: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E773C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7748 size=128
    let mut pc: u32 = 0x825E7748;
    'dispatch: loop {
        match pc {
            0x825E7748 => {
    //   block [0x825E7748..0x825E77C8)
	// 825E7748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7758: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E775C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7760: 3BEBC1F0  addi r31, r11, -0x3e10
	ctx.r[31].s64 = ctx.r[11].s64 + -15888;
	// 825E7764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7768: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E776C: 4BF4DA65  bl 0x825351d0
	ctx.lr = 0x825E7770;
	sub_825351D0(ctx, base);
	// 825E7770: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7774: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7778: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E777C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7780: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7784: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7788: 616B0075  ori r11, r11, 0x75
	ctx.r[11].u64 = ctx.r[11].u64 | 117;
	// 825E778C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7794: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7798: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E779C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E77A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E77A4: 616B0021  ori r11, r11, 0x21
	ctx.r[11].u64 = ctx.r[11].u64 | 33;
	// 825E77A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E77AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E77B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E77B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E77B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E77BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E77C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E77C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E77C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E77C8 size=128
    let mut pc: u32 = 0x825E77C8;
    'dispatch: loop {
        match pc {
            0x825E77C8 => {
    //   block [0x825E77C8..0x825E7848)
	// 825E77C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E77CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E77D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E77D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E77D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E77DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E77E0: 3BEBCD50  addi r31, r11, -0x32b0
	ctx.r[31].s64 = ctx.r[11].s64 + -12976;
	// 825E77E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E77E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E77EC: 4BF4D9E5  bl 0x825351d0
	ctx.lr = 0x825E77F0;
	sub_825351D0(ctx, base);
	// 825E77F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E77F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E77F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E77FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7800: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7804: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7808: 616B0079  ori r11, r11, 0x79
	ctx.r[11].u64 = ctx.r[11].u64 | 121;
	// 825E780C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7814: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7818: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E781C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7820: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7824: 616B0026  ori r11, r11, 0x26
	ctx.r[11].u64 = ctx.r[11].u64 | 38;
	// 825E7828: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E782C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7830: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E783C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7848 size=128
    let mut pc: u32 = 0x825E7848;
    'dispatch: loop {
        match pc {
            0x825E7848 => {
    //   block [0x825E7848..0x825E78C8)
	// 825E7848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E784C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7858: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E785C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7860: 3BEBCA08  addi r31, r11, -0x35f8
	ctx.r[31].s64 = ctx.r[11].s64 + -13816;
	// 825E7864: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7868: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E786C: 4BF4D965  bl 0x825351d0
	ctx.lr = 0x825E7870;
	sub_825351D0(ctx, base);
	// 825E7870: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7874: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7878: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E787C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7880: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7884: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7888: 616B007A  ori r11, r11, 0x7a
	ctx.r[11].u64 = ctx.r[11].u64 | 122;
	// 825E788C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7894: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7898: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E789C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E78A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E78A4: 616B0027  ori r11, r11, 0x27
	ctx.r[11].u64 = ctx.r[11].u64 | 39;
	// 825E78A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E78AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E78B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E78B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E78B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E78BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E78C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E78C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E78C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E78C8 size=128
    let mut pc: u32 = 0x825E78C8;
    'dispatch: loop {
        match pc {
            0x825E78C8 => {
    //   block [0x825E78C8..0x825E7948)
	// 825E78C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E78CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E78D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E78D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E78D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E78DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E78E0: 3BEBCBC8  addi r31, r11, -0x3438
	ctx.r[31].s64 = ctx.r[11].s64 + -13368;
	// 825E78E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E78E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E78EC: 4BF4D8E5  bl 0x825351d0
	ctx.lr = 0x825E78F0;
	sub_825351D0(ctx, base);
	// 825E78F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E78F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E78F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E78FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7900: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7904: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7908: 616B007C  ori r11, r11, 0x7c
	ctx.r[11].u64 = ctx.r[11].u64 | 124;
	// 825E790C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7914: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7918: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E791C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7920: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7924: 616B0029  ori r11, r11, 0x29
	ctx.r[11].u64 = ctx.r[11].u64 | 41;
	// 825E7928: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E792C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7930: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E793C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7948 size=128
    let mut pc: u32 = 0x825E7948;
    'dispatch: loop {
        match pc {
            0x825E7948 => {
    //   block [0x825E7948..0x825E79C8)
	// 825E7948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E794C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7958: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E795C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E7960: 3BEBCD88  addi r31, r11, -0x3278
	ctx.r[31].s64 = ctx.r[11].s64 + -12920;
	// 825E7964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7968: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E796C: 4BF4D865  bl 0x825351d0
	ctx.lr = 0x825E7970;
	sub_825351D0(ctx, base);
	// 825E7970: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E7974: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E7978: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E797C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7980: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7984: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7988: 616B007D  ori r11, r11, 0x7d
	ctx.r[11].u64 = ctx.r[11].u64 | 125;
	// 825E798C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7994: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7998: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E799C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E79A0: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E79A4: 616B002A  ori r11, r11, 0x2a
	ctx.r[11].u64 = ctx.r[11].u64 | 42;
	// 825E79A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E79AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E79B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E79B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E79B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E79BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E79C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E79C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E79C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E79C8 size=128
    let mut pc: u32 = 0x825E79C8;
    'dispatch: loop {
        match pc {
            0x825E79C8 => {
    //   block [0x825E79C8..0x825E7A48)
	// 825E79C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E79CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E79D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E79D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E79D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E79DC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 825E79E0: 3BEBCAE8  addi r31, r11, -0x3518
	ctx.r[31].s64 = ctx.r[11].s64 + -13592;
	// 825E79E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E79E8: 387F000D  addi r3, r31, 0xd
	ctx.r[3].s64 = ctx.r[31].s64 + 13;
	// 825E79EC: 4BF4D7E5  bl 0x825351d0
	ctx.lr = 0x825E79F0;
	sub_825351D0(ctx, base);
	// 825E79F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 825E79F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E79F8: 396B1160  addi r11, r11, 0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + 4448;
	// 825E79FC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825E7A00: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825E7A04: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7A08: 616B007E  ori r11, r11, 0x7e
	ctx.r[11].u64 = ctx.r[11].u64 | 126;
	// 825E7A0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825E7A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7A14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825E7A18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825E7A1C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 825E7A20: 3D600C00  lis r11, 0xc00
	ctx.r[11].s64 = 201326592;
	// 825E7A24: 616B002B  ori r11, r11, 0x2b
	ctx.r[11].u64 = ctx.r[11].u64 | 43;
	// 825E7A28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825E7A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E7A30: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825E7A34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E7A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7A40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7A48 size=40
    let mut pc: u32 = 0x825E7A48;
    'dispatch: loop {
        match pc {
            0x825E7A48 => {
    //   block [0x825E7A48..0x825E7A60)
	// 825E7A48: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 825E7A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E7A50: 396B74C0  addi r11, r11, 0x74c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29888;
	// 825E7A54: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 825E7A58: 396B00A0  addi r11, r11, 0xa0
	ctx.r[11].s64 = ctx.r[11].s64 + 160;
	// 825E7A5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x825E7A60; continue 'dispatch;
            }
            0x825E7A60 => {
    //   block [0x825E7A60..0x825E7A70)
	// 825E7A60: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825E7A64: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825E7A68: 4200FFF8  bdnz 0x825e7a60
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825E7A60; continue 'dispatch;
	}
	// 825E7A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7A70 size=76
    let mut pc: u32 = 0x825E7A70;
    'dispatch: loop {
        match pc {
            0x825E7A70 => {
    //   block [0x825E7A70..0x825E7A90)
	// 825E7A70: 3D608285  lis r11, -0x7d7b
	ctx.r[11].s64 = -2105212928;
	// 825E7A74: 3D4082D0  lis r10, -0x7d30
	ctx.r[10].s64 = -2100297728;
	// 825E7A78: 396B5FA0  addi r11, r11, 0x5fa0
	ctx.r[11].s64 = ctx.r[11].s64 + 24480;
	// 825E7A7C: 392ABE80  addi r9, r10, -0x4180
	ctx.r[9].s64 = ctx.r[10].s64 + -16768;
	// 825E7A80: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 825E7A84: 39690040  addi r11, r9, 0x40
	ctx.r[11].s64 = ctx.r[9].s64 + 64;
	// 825E7A88: 39200015  li r9, 0x15
	ctx.r[9].s64 = 21;
	// 825E7A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	pc = 0x825E7A90; continue 'dispatch;
            }
            0x825E7A90 => {
    //   block [0x825E7A90..0x825E7ABC)
	// 825E7A90: 80EAFFFC  lwz r7, -4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 825E7A94: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 825E7A98: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825E7A9C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 825E7AA0: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 825E7AA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825E7AA8: 90EBFFF8  stw r7, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 825E7AAC: 90CBFFFC  stw r6, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[6].u32 ) };
	// 825E7AB0: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825E7AB4: 409AFFDC  bne cr6, 0x825e7a90
	if !ctx.cr[6].eq {
	pc = 0x825E7A90; continue 'dispatch;
	}
	// 825E7AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7AC0 size=12
    let mut pc: u32 = 0x825E7AC0;
    'dispatch: loop {
        match pc {
            0x825E7AC0 => {
    //   block [0x825E7AC0..0x825E7ACC)
	// 825E7AC0: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7AC4: 386BC000  addi r3, r11, -0x4000
	ctx.r[3].s64 = ctx.r[11].s64 + -16384;
	// 825E7AC8: 4BD8B628  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7AD0 size=12
    let mut pc: u32 = 0x825E7AD0;
    'dispatch: loop {
        match pc {
            0x825E7AD0 => {
    //   block [0x825E7AD0..0x825E7ADC)
	// 825E7AD0: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7AD4: 386BC440  addi r3, r11, -0x3bc0
	ctx.r[3].s64 = ctx.r[11].s64 + -15296;
	// 825E7AD8: 4BD8B618  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7AE0 size=12
    let mut pc: u32 = 0x825E7AE0;
    'dispatch: loop {
        match pc {
            0x825E7AE0 => {
    //   block [0x825E7AE0..0x825E7AEC)
	// 825E7AE0: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7AE4: 386BC840  addi r3, r11, -0x37c0
	ctx.r[3].s64 = ctx.r[11].s64 + -14272;
	// 825E7AE8: 4BD8B608  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7AF0 size=12
    let mut pc: u32 = 0x825E7AF0;
    'dispatch: loop {
        match pc {
            0x825E7AF0 => {
    //   block [0x825E7AF0..0x825E7AFC)
	// 825E7AF0: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7AF4: 386BD080  addi r3, r11, -0x2f80
	ctx.r[3].s64 = ctx.r[11].s64 + -12160;
	// 825E7AF8: 4BD8B5F8  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7B00 size=12
    let mut pc: u32 = 0x825E7B00;
    'dispatch: loop {
        match pc {
            0x825E7B00 => {
    //   block [0x825E7B00..0x825E7B0C)
	// 825E7B00: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B04: 386BCC80  addi r3, r11, -0x3380
	ctx.r[3].s64 = ctx.r[11].s64 + -13184;
	// 825E7B08: 4BD8B5E8  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7B10 size=12
    let mut pc: u32 = 0x825E7B10;
    'dispatch: loop {
        match pc {
            0x825E7B10 => {
    //   block [0x825E7B10..0x825E7B1C)
	// 825E7B10: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B14: 386BD480  addi r3, r11, -0x2b80
	ctx.r[3].s64 = ctx.r[11].s64 + -11136;
	// 825E7B18: 4BD8B5D8  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7B20 size=12
    let mut pc: u32 = 0x825E7B20;
    'dispatch: loop {
        match pc {
            0x825E7B20 => {
    //   block [0x825E7B20..0x825E7B2C)
	// 825E7B20: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B24: 386BD8C0  addi r3, r11, -0x2740
	ctx.r[3].s64 = ctx.r[11].s64 + -10048;
	// 825E7B28: 4BD8B5C8  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7B30 size=12
    let mut pc: u32 = 0x825E7B30;
    'dispatch: loop {
        match pc {
            0x825E7B30 => {
    //   block [0x825E7B30..0x825E7B3C)
	// 825E7B30: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B34: 386BDD00  addi r3, r11, -0x2300
	ctx.r[3].s64 = ctx.r[11].s64 + -8960;
	// 825E7B38: 4BD8B5B8  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7B40 size=40
    let mut pc: u32 = 0x825E7B40;
    'dispatch: loop {
        match pc {
            0x825E7B40 => {
    //   block [0x825E7B40..0x825E7B68)
	// 825E7B40: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E7B48: 396BDCC8  addi r11, r11, -0x2338
	ctx.r[11].s64 = ctx.r[11].s64 + -9016;
	// 825E7B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E7B50: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825E7B54: 3D40DEAD  lis r10, -0x2153
	ctx.r[10].s64 = -559087616;
	// 825E7B58: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825E7B5C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825E7B60: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825E7B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7B68 size=12
    let mut pc: u32 = 0x825E7B68;
    'dispatch: loop {
        match pc {
            0x825E7B68 => {
    //   block [0x825E7B68..0x825E7B74)
	// 825E7B68: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B6C: 386BE140  addi r3, r11, -0x1ec0
	ctx.r[3].s64 = ctx.r[11].s64 + -7872;
	// 825E7B70: 4BD8B580  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7B78 size=316
    let mut pc: u32 = 0x825E7B78;
    'dispatch: loop {
        match pc {
            0x825E7B78 => {
    //   block [0x825E7B78..0x825E7CB4)
	// 825E7B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E7B7C: 4BF4D541  bl 0x825350bc
	ctx.lr = 0x825E7B80;
	sub_82535080(ctx, base);
	// 825E7B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7B84: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7B88: 3BEBE558  addi r31, r11, -0x1aa8
	ctx.r[31].s64 = ctx.r[11].s64 + -6824;
	// 825E7B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825E7B90: 4BD14C71  bl 0x822fc800
	ctx.lr = 0x825E7B94;
	sub_822FC800(ctx, base);
	// 825E7B94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825E7B98: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 825E7B9C: 396BCB6C  addi r11, r11, -0x3494
	ctx.r[11].s64 = ctx.r[11].s64 + -13460;
	// 825E7BA0: 394AFEE8  addi r10, r10, -0x118
	ctx.r[10].s64 = ctx.r[10].s64 + -280;
	// 825E7BA4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825E7BA8: 387F0A90  addi r3, r31, 0xa90
	ctx.r[3].s64 = ctx.r[31].s64 + 2704;
	// 825E7BAC: 3BA9E1B0  addi r29, r9, -0x1e50
	ctx.r[29].s64 = ctx.r[9].s64 + -7760;
	// 825E7BB0: 917F0A78  stw r11, 0xa78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2680 as u32), ctx.r[11].u32 ) };
	// 825E7BB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825E7BB8: 915F0A80  stw r10, 0xa80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2688 as u32), ctx.r[10].u32 ) };
	// 825E7BBC: 917F0A84  stw r11, 0xa84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2692 as u32), ctx.r[11].u32 ) };
	// 825E7BC0: 917F0A88  stw r11, 0xa88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2696 as u32), ctx.r[11].u32 ) };
	// 825E7BC4: 93BF0A8C  stw r29, 0xa8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2700 as u32), ctx.r[29].u32 ) };
	// 825E7BC8: 481256C5  bl 0x8270d28c
	ctx.lr = 0x825E7BCC;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 825E7BCC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825E7BD0: 38A00110  li r5, 0x110
	ctx.r[5].s64 = 272;
	// 825E7BD4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 825E7BD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E7BDC: 387F0B2C  addi r3, r31, 0xb2c
	ctx.r[3].s64 = ctx.r[31].s64 + 2860;
	// 825E7BE0: 917F0AE0  stw r11, 0xae0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2784 as u32), ctx.r[11].u32 ) };
	// 825E7BE4: 917F0AE4  stw r11, 0xae4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2788 as u32), ctx.r[11].u32 ) };
	// 825E7BE8: 917F0AEC  stw r11, 0xaec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2796 as u32), ctx.r[11].u32 ) };
	// 825E7BEC: 917F0AF0  stw r11, 0xaf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2800 as u32), ctx.r[11].u32 ) };
	// 825E7BF0: 917F0AF8  stw r11, 0xaf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2808 as u32), ctx.r[11].u32 ) };
	// 825E7BF4: 917F0AFC  stw r11, 0xafc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2812 as u32), ctx.r[11].u32 ) };
	// 825E7BF8: 917F0B04  stw r11, 0xb04(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2820 as u32), ctx.r[11].u32 ) };
	// 825E7BFC: 917F0B08  stw r11, 0xb08(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2824 as u32), ctx.r[11].u32 ) };
	// 825E7C00: 917F0C44  stw r11, 0xc44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3140 as u32), ctx.r[11].u32 ) };
	// 825E7C04: 917F0C48  stw r11, 0xc48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3144 as u32), ctx.r[11].u32 ) };
	// 825E7C08: 917F0C50  stw r11, 0xc50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3152 as u32), ctx.r[11].u32 ) };
	// 825E7C0C: 917F0C54  stw r11, 0xc54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3156 as u32), ctx.r[11].u32 ) };
	// 825E7C10: 917F0C64  stw r11, 0xc64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3172 as u32), ctx.r[11].u32 ) };
	// 825E7C14: 917F0C68  stw r11, 0xc68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3176 as u32), ctx.r[11].u32 ) };
	// 825E7C18: 4BF4D5B9  bl 0x825351d0
	ctx.lr = 0x825E7C1C;
	sub_825351D0(ctx, base);
	// 825E7C1C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 825E7C20: 387F0E64  addi r3, r31, 0xe64
	ctx.r[3].s64 = ctx.r[31].s64 + 3684;
	// 825E7C24: 917F0C5C  stw r11, 0xc5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3164 as u32), ctx.r[11].u32 ) };
	// 825E7C28: 917F0C60  stw r11, 0xc60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3168 as u32), ctx.r[11].u32 ) };
	// 825E7C2C: 917F0C70  stw r11, 0xc70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3184 as u32), ctx.r[11].u32 ) };
	// 825E7C30: 917F0C74  stw r11, 0xc74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3188 as u32), ctx.r[11].u32 ) };
	// 825E7C34: 917F0C3C  stw r11, 0xc3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3132 as u32), ctx.r[11].u32 ) };
	// 825E7C38: 917F0C40  stw r11, 0xc40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3136 as u32), ctx.r[11].u32 ) };
	// 825E7C3C: 917F0C78  stw r11, 0xc78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3192 as u32), ctx.r[11].u32 ) };
	// 825E7C40: 917F0C80  stw r11, 0xc80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3200 as u32), ctx.r[11].u32 ) };
	// 825E7C44: 93DF0C94  stw r30, 0xc94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3220 as u32), ctx.r[30].u32 ) };
	// 825E7C48: 93DF0C98  stw r30, 0xc98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3224 as u32), ctx.r[30].u32 ) };
	// 825E7C4C: 93DF0C9C  stw r30, 0xc9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3228 as u32), ctx.r[30].u32 ) };
	// 825E7C50: 93DF0CA0  stw r30, 0xca0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3232 as u32), ctx.r[30].u32 ) };
	// 825E7C54: 93DF0C84  stw r30, 0xc84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3204 as u32), ctx.r[30].u32 ) };
	// 825E7C58: 93DF0C88  stw r30, 0xc88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3208 as u32), ctx.r[30].u32 ) };
	// 825E7C5C: 93DF0C8C  stw r30, 0xc8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3212 as u32), ctx.r[30].u32 ) };
	// 825E7C60: 93DF0C90  stw r30, 0xc90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3216 as u32), ctx.r[30].u32 ) };
	// 825E7C64: 93BF0E60  stw r29, 0xe60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3680 as u32), ctx.r[29].u32 ) };
	// 825E7C68: 48125625  bl 0x8270d28c
	ctx.lr = 0x825E7C6C;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 825E7C6C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 825E7C70: 387F0E84  addi r3, r31, 0xe84
	ctx.r[3].s64 = ctx.r[31].s64 + 3716;
	// 825E7C74: 917F0E58  stw r11, 0xe58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3672 as u32), ctx.r[11].u32 ) };
	// 825E7C78: 917F0E5C  stw r11, 0xe5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3676 as u32), ctx.r[11].u32 ) };
	// 825E7C7C: 93BF0E80  stw r29, 0xe80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3712 as u32), ctx.r[29].u32 ) };
	// 825E7C80: 4812560D  bl 0x8270d28c
	ctx.lr = 0x825E7C84;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 825E7C84: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 825E7C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825E7C8C: 917F0EC0  stw r11, 0xec0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3776 as u32), ctx.r[11].u32 ) };
	// 825E7C90: 917F0EC4  stw r11, 0xec4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3780 as u32), ctx.r[11].u32 ) };
	// 825E7C94: F97F0EF0  std r11, 0xef0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(3824 as u32), ctx.r[11].u64 ) };
	// 825E7C98: 93DF0EEC  stw r30, 0xeec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3820 as u32), ctx.r[30].u32 ) };
	// 825E7C9C: 4BD14E4D  bl 0x822fcae8
	ctx.lr = 0x825E7CA0;
	sub_822FCAE8(ctx, base);
	// 825E7CA0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7CA4: 386BC3C0  addi r3, r11, -0x3c40
	ctx.r[3].s64 = ctx.r[11].s64 + -15424;
	// 825E7CA8: 4BF4AE91  bl 0x82532b38
	ctx.lr = 0x825E7CAC;
	sub_82532B38(ctx, base);
	// 825E7CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825E7CB0: 4BF4D45C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7CB8 size=12
    let mut pc: u32 = 0x825E7CB8;
    'dispatch: loop {
        match pc {
            0x825E7CB8 => {
    //   block [0x825E7CB8..0x825E7CC4)
	// 825E7CB8: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7CBC: 386BF500  addi r3, r11, -0xb00
	ctx.r[3].s64 = ctx.r[11].s64 + -2816;
	// 825E7CC0: 4BD8B430  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7CC8 size=12
    let mut pc: u32 = 0x825E7CC8;
    'dispatch: loop {
        match pc {
            0x825E7CC8 => {
    //   block [0x825E7CC8..0x825E7CD4)
	// 825E7CC8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7CCC: 386BC3D0  addi r3, r11, -0x3c30
	ctx.r[3].s64 = ctx.r[11].s64 + -15408;
	// 825E7CD0: 4BF4AE68  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7CD8 size=12
    let mut pc: u32 = 0x825E7CD8;
    'dispatch: loop {
        match pc {
            0x825E7CD8 => {
    //   block [0x825E7CD8..0x825E7CE4)
	// 825E7CD8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7CDC: 386BC420  addi r3, r11, -0x3be0
	ctx.r[3].s64 = ctx.r[11].s64 + -15328;
	// 825E7CE0: 4BF4AE58  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7CE8 size=12
    let mut pc: u32 = 0x825E7CE8;
    'dispatch: loop {
        match pc {
            0x825E7CE8 => {
    //   block [0x825E7CE8..0x825E7CF4)
	// 825E7CE8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7CEC: 386BC440  addi r3, r11, -0x3bc0
	ctx.r[3].s64 = ctx.r[11].s64 + -15296;
	// 825E7CF0: 4BF4AE48  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7CF8 size=80
    let mut pc: u32 = 0x825E7CF8;
    'dispatch: loop {
        match pc {
            0x825E7CF8 => {
    //   block [0x825E7CF8..0x825E7D18)
	// 825E7CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E7CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825E7D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E7D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7D0C: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7D10: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 825E7D14: 3BCBF908  addi r30, r11, -0x6f8
	ctx.r[30].s64 = ctx.r[11].s64 + -1784;
	pc = 0x825E7D18; continue 'dispatch;
            }
            0x825E7D18 => {
    //   block [0x825E7D18..0x825E7D48)
	// 825E7D18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825E7D1C: 4BB3C4E5  bl 0x82124200
	ctx.lr = 0x825E7D20;
	sub_82124200(ctx, base);
	// 825E7D20: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 825E7D24: 3BDE1018  addi r30, r30, 0x1018
	ctx.r[30].s64 = ctx.r[30].s64 + 4120;
	// 825E7D28: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825E7D2C: 4098FFEC  bge cr6, 0x825e7d18
	if !ctx.cr[6].lt {
	pc = 0x825E7D18; continue 'dispatch;
	}
	// 825E7D30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825E7D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E7D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7D3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825E7D40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E7D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7D48 size=36
    let mut pc: u32 = 0x825E7D48;
    'dispatch: loop {
        match pc {
            0x825E7D48 => {
    //   block [0x825E7D48..0x825E7D5C)
	// 825E7D48: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E7D50: 396B4998  addi r11, r11, 0x4998
	ctx.r[11].s64 = ctx.r[11].s64 + 18840;
	// 825E7D54: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 825E7D58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x825E7D5C; continue 'dispatch;
            }
            0x825E7D5C => {
    //   block [0x825E7D5C..0x825E7D6C)
	// 825E7D5C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825E7D60: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825E7D64: 4200FFF8  bdnz 0x825e7d5c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825E7D5C; continue 'dispatch;
	}
	// 825E7D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7D70 size=12
    let mut pc: u32 = 0x825E7D70;
    'dispatch: loop {
        match pc {
            0x825E7D70 => {
    //   block [0x825E7D70..0x825E7D7C)
	// 825E7D70: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7D74: 386BC460  addi r3, r11, -0x3ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -15264;
	// 825E7D78: 4BF4ADC0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7D80 size=36
    let mut pc: u32 = 0x825E7D80;
    'dispatch: loop {
        match pc {
            0x825E7D80 => {
    //   block [0x825E7D80..0x825E7D94)
	// 825E7D80: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E7D88: 396B49D8  addi r11, r11, 0x49d8
	ctx.r[11].s64 = ctx.r[11].s64 + 18904;
	// 825E7D8C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 825E7D90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x825E7D94; continue 'dispatch;
            }
            0x825E7D94 => {
    //   block [0x825E7D94..0x825E7DA4)
	// 825E7D94: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825E7D98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825E7D9C: 4200FFF8  bdnz 0x825e7d94
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825E7D94; continue 'dispatch;
	}
	// 825E7DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7DA8 size=52
    let mut pc: u32 = 0x825E7DA8;
    'dispatch: loop {
        match pc {
            0x825E7DA8 => {
    //   block [0x825E7DA8..0x825E7DDC)
	// 825E7DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E7DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7DB4: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7DB8: 386B4AE8  addi r3, r11, 0x4ae8
	ctx.r[3].s64 = ctx.r[11].s64 + 19176;
	// 825E7DBC: 4BD50085  bl 0x82337e40
	ctx.lr = 0x825E7DC0;
	sub_82337E40(ctx, base);
	// 825E7DC0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7DC4: 386BC480  addi r3, r11, -0x3b80
	ctx.r[3].s64 = ctx.r[11].s64 + -15232;
	// 825E7DC8: 4BF4AD71  bl 0x82532b38
	ctx.lr = 0x825E7DCC;
	sub_82532B38(ctx, base);
	// 825E7DCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E7DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7DE0 size=76
    let mut pc: u32 = 0x825E7DE0;
    'dispatch: loop {
        match pc {
            0x825E7DE0 => {
    //   block [0x825E7DE0..0x825E7E00)
	// 825E7DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E7DE4: 4BF4D2D9  bl 0x825350bc
	ctx.lr = 0x825E7DE8;
	sub_82535080(ctx, base);
	// 825E7DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7DEC: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 825E7DF0: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 825E7DF4: 3BCB61D8  addi r30, r11, 0x61d8
	ctx.r[30].s64 = ctx.r[11].s64 + 25048;
	// 825E7DF8: 3D60000F  lis r11, 0xf
	ctx.r[11].s64 = 983040;
	// 825E7DFC: 617D9498  ori r29, r11, 0x9498
	ctx.r[29].u64 = ctx.r[11].u64 | 38040;
	pc = 0x825E7E00; continue 'dispatch;
            }
            0x825E7E00 => {
    //   block [0x825E7E00..0x825E7E2C)
	// 825E7E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825E7E04: 4BD51DC5  bl 0x82339bc8
	ctx.lr = 0x825E7E08;
	sub_82339BC8(ctx, base);
	// 825E7E08: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 825E7E0C: 7FDEEA14  add r30, r30, r29
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 825E7E10: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825E7E14: 4098FFEC  bge cr6, 0x825e7e00
	if !ctx.cr[6].lt {
	pc = 0x825E7E00; continue 'dispatch;
	}
	// 825E7E18: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7E1C: 386BC498  addi r3, r11, -0x3b68
	ctx.r[3].s64 = ctx.r[11].s64 + -15208;
	// 825E7E20: 4BF4AD19  bl 0x82532b38
	ctx.lr = 0x825E7E24;
	sub_82532B38(ctx, base);
	// 825E7E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825E7E28: 4BF4D2E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7E30 size=12
    let mut pc: u32 = 0x825E7E30;
    'dispatch: loop {
        match pc {
            0x825E7E30 => {
    //   block [0x825E7E30..0x825E7E3C)
	// 825E7E30: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7E34: 386BC4A0  addi r3, r11, -0x3b60
	ctx.r[3].s64 = ctx.r[11].s64 + -15200;
	// 825E7E38: 4BF4AD00  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7E40 size=12
    let mut pc: u32 = 0x825E7E40;
    'dispatch: loop {
        match pc {
            0x825E7E40 => {
    //   block [0x825E7E40..0x825E7E4C)
	// 825E7E40: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7E44: 386BC4C0  addi r3, r11, -0x3b40
	ctx.r[3].s64 = ctx.r[11].s64 + -15168;
	// 825E7E48: 4BF4ACF0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E7E50 size=12
    let mut pc: u32 = 0x825E7E50;
    'dispatch: loop {
        match pc {
            0x825E7E50 => {
    //   block [0x825E7E50..0x825E7E5C)
	// 825E7E50: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7E54: 386BC4E0  addi r3, r11, -0x3b20
	ctx.r[3].s64 = ctx.r[11].s64 + -15136;
	// 825E7E58: 4BF4ACE0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E7E60 size=52
    let mut pc: u32 = 0x825E7E60;
    'dispatch: loop {
        match pc {
            0x825E7E60 => {
    //   block [0x825E7E60..0x825E7E94)
	// 825E7E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E7E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E7E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E7E6C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E7E70: 386BB460  addi r3, r11, -0x4ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -19360;
	// 825E7E74: 4BD575D5  bl 0x8233f448
	ctx.lr = 0x825E7E78;
	sub_8233F448(ctx, base);
	// 825E7E78: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E7E7C: 386BC500  addi r3, r11, -0x3b00
	ctx.r[3].s64 = ctx.r[11].s64 + -15104;
	// 825E7E80: 4BF4ACB9  bl 0x82532b38
	ctx.lr = 0x825E7E84;
	sub_82532B38(ctx, base);
	// 825E7E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E7E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E7E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E7E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E7E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E7E98 size=816
    let mut pc: u32 = 0x825E7E98;
    'dispatch: loop {
        match pc {
            0x825E7E98 => {
    //   block [0x825E7E98..0x825E81C8)
	// 825E7E98: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E7E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E7EA0: 396B8480  addi r11, r11, -0x7b80
	ctx.r[11].s64 = ctx.r[11].s64 + -31616;
	// 825E7EA4: 3D000300  lis r8, 0x300
	ctx.r[8].s64 = 50331648;
	// 825E7EA8: 3D208287  lis r9, -0x7d79
	ctx.r[9].s64 = -2105081856;
	// 825E7EAC: 610800CF  ori r8, r8, 0xcf
	ctx.r[8].u64 = ctx.r[8].u64 | 207;
	// 825E7EB0: 39297880  addi r9, r9, 0x7880
	ctx.r[9].s64 = ctx.r[9].s64 + 30848;
	// 825E7EB4: F94B00B8  std r10, 0xb8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u64 ) };
	// 825E7EB8: 38E00021  li r7, 0x21
	ctx.r[7].s64 = 33;
	// 825E7EBC: F94B00C0  std r10, 0xc0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[10].u64 ) };
	// 825E7EC0: F94B00C8  std r10, 0xc8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u64 ) };
	// 825E7EC4: 910B00D0  stw r8, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[8].u32 ) };
	// 825E7EC8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825E7ECC: C188AC48  lfs f12, -0x53b8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-21432 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825E7ED0: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825E7ED4: D18B00DC  stfs f12, 0xdc(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 825E7ED8: 914B00D4  stw r10, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 825E7EDC: C168210C  lfs f11, 0x210c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8460 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825E7EE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E7EE4: D16B00E0  stfs f11, 0xe0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 825E7EE8: 910B00D8  stw r8, 0xd8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(216 as u32), ctx.r[8].u32 ) };
	// 825E7EEC: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825E7EF0: C1A81FF8  lfs f13, 0x1ff8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E7EF4: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 825E7EF8: D1AB00E4  stfs f13, 0xe4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 825E7EFC: D1AB00E8  stfs f13, 0xe8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 825E7F00: 914B00EC  stw r10, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 825E7F04: C008BA38  lfs f0, -0x45c8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E7F08: 39090180  addi r8, r9, 0x180
	ctx.r[8].s64 = ctx.r[9].s64 + 384;
	// 825E7F0C: D00B00F0  stfs f0, 0xf0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 825E7F10: 910B0140  stw r8, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[8].u32 ) };
	// 825E7F14: D00B00F4  stfs f0, 0xf4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 825E7F18: 914B0144  stw r10, 0x144(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 825E7F1C: D00B00F8  stfs f0, 0xf8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 825E7F20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E7F24: 914B0148  stw r10, 0x148(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), ctx.r[10].u32 ) };
	// 825E7F28: D00B00FC  stfs f0, 0xfc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 825E7F2C: 914B014C  stw r10, 0x14c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), ctx.r[10].u32 ) };
	// 825E7F30: D00B0100  stfs f0, 0x100(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 825E7F34: 910B0150  stw r8, 0x150(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), ctx.r[8].u32 ) };
	// 825E7F38: D00B0104  stfs f0, 0x104(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 825E7F3C: 910B0154  stw r8, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[8].u32 ) };
	// 825E7F40: 3D002808  lis r8, 0x2808
	ctx.r[8].s64 = 671612928;
	// 825E7F44: D00B0108  stfs f0, 0x108(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 825E7F48: 61080002  ori r8, r8, 2
	ctx.r[8].u64 = ctx.r[8].u64 | 2;
	// 825E7F4C: 78E8000E  rldimi r8, r7, 0x20, 0
	ctx.r[8].u64 = ((ctx.r[7].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[8].u64 & 0x00000000FFFFFFFF);
	// 825E7F50: F90B0158  std r8, 0x158(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[8].u64 ) };
	// 825E7F54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E7F58: D00B010C  stfs f0, 0x10c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 825E7F5C: B10B0160  sth r8, 0x160(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(352 as u32), ctx.r[8].u16 ) };
	// 825E7F60: D00B0110  stfs f0, 0x110(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 825E7F64: B10B0162  sth r8, 0x162(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(354 as u32), ctx.r[8].u16 ) };
	// 825E7F68: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825E7F6C: D00B0114  stfs f0, 0x114(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 825E7F70: B10B0164  sth r8, 0x164(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(356 as u32), ctx.r[8].u16 ) };
	// 825E7F74: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825E7F78: D00B0118  stfs f0, 0x118(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 825E7F7C: B10B0166  sth r8, 0x166(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(358 as u32), ctx.r[8].u16 ) };
	// 825E7F80: D00B011C  stfs f0, 0x11c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 825E7F84: 914B0168  stw r10, 0x168(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(360 as u32), ctx.r[10].u32 ) };
	// 825E7F88: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 825E7F8C: D00B0120  stfs f0, 0x120(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 825E7F90: 914B016C  stw r10, 0x16c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(364 as u32), ctx.r[10].u32 ) };
	// 825E7F94: D00B0124  stfs f0, 0x124(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 825E7F98: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 825E7F9C: D00B0128  stfs f0, 0x128(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 825E7FA0: 914B0174  stw r10, 0x174(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(372 as u32), ctx.r[10].u32 ) };
	// 825E7FA4: C148D6D0  lfs f10, -0x2930(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10544 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825E7FA8: D14B012C  stfs f10, 0x12c(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 825E7FAC: 914B0178  stw r10, 0x178(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(376 as u32), ctx.r[10].u32 ) };
	// 825E7FB0: D00B0130  stfs f0, 0x130(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 825E7FB4: 914B017C  stw r10, 0x17c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(380 as u32), ctx.r[10].u32 ) };
	// 825E7FB8: D00B0134  stfs f0, 0x134(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 825E7FBC: 914B0180  stw r10, 0x180(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(384 as u32), ctx.r[10].u32 ) };
	// 825E7FC0: D1AB0138  stfs f13, 0x138(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 825E7FC4: D00B013C  stfs f0, 0x13c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 825E7FC8: D1AB0184  stfs f13, 0x184(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 825E7FCC: F94B0188  std r10, 0x188(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u64 ) };
	// 825E7FD0: F94B0190  std r10, 0x190(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(400 as u32), ctx.r[10].u64 ) };
	// 825E7FD4: F94B0198  std r10, 0x198(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(408 as u32), ctx.r[10].u64 ) };
	// 825E7FD8: 3D000300  lis r8, 0x300
	ctx.r[8].s64 = 50331648;
	// 825E7FDC: 61080002  ori r8, r8, 2
	ctx.r[8].u64 = ctx.r[8].u64 | 2;
	// 825E7FE0: 910B01A0  stw r8, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[8].u32 ) };
	// 825E7FE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E7FE8: D18B01AC  stfs f12, 0x1ac(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 825E7FEC: 914B01A4  stw r10, 0x1a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(420 as u32), ctx.r[10].u32 ) };
	// 825E7FF0: D16B01B0  stfs f11, 0x1b0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(432 as u32), tmp.u32 ) };
	// 825E7FF4: 910B01A8  stw r8, 0x1a8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(424 as u32), ctx.r[8].u32 ) };
	// 825E7FF8: D1AB01B4  stfs f13, 0x1b4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(436 as u32), tmp.u32 ) };
	// 825E7FFC: D1AB01B8  stfs f13, 0x1b8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 825E8000: 914B01BC  stw r10, 0x1bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(444 as u32), ctx.r[10].u32 ) };
	// 825E8004: D00B01C0  stfs f0, 0x1c0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 825E8008: 39090300  addi r8, r9, 0x300
	ctx.r[8].s64 = ctx.r[9].s64 + 768;
	// 825E800C: D00B01C4  stfs f0, 0x1c4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(452 as u32), tmp.u32 ) };
	// 825E8010: 39290480  addi r9, r9, 0x480
	ctx.r[9].s64 = ctx.r[9].s64 + 1152;
	// 825E8014: D00B01C8  stfs f0, 0x1c8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(456 as u32), tmp.u32 ) };
	// 825E8018: D00B01CC  stfs f0, 0x1cc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 825E801C: D00B01D0  stfs f0, 0x1d0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), tmp.u32 ) };
	// 825E8020: D00B01D4  stfs f0, 0x1d4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(468 as u32), tmp.u32 ) };
	// 825E8024: D00B01D8  stfs f0, 0x1d8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(472 as u32), tmp.u32 ) };
	// 825E8028: D00B01DC  stfs f0, 0x1dc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(476 as u32), tmp.u32 ) };
	// 825E802C: D00B01E0  stfs f0, 0x1e0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(480 as u32), tmp.u32 ) };
	// 825E8030: D00B01E4  stfs f0, 0x1e4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(484 as u32), tmp.u32 ) };
	// 825E8034: D00B01E8  stfs f0, 0x1e8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), tmp.u32 ) };
	// 825E8038: D00B01EC  stfs f0, 0x1ec(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(492 as u32), tmp.u32 ) };
	// 825E803C: D00B01F0  stfs f0, 0x1f0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(496 as u32), tmp.u32 ) };
	// 825E8040: D00B01F4  stfs f0, 0x1f4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(500 as u32), tmp.u32 ) };
	// 825E8044: 910B0210  stw r8, 0x210(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(528 as u32), ctx.r[8].u32 ) };
	// 825E8048: D00B01F8  stfs f0, 0x1f8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(504 as u32), tmp.u32 ) };
	// 825E804C: 914B0214  stw r10, 0x214(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(532 as u32), ctx.r[10].u32 ) };
	// 825E8050: D14B01FC  stfs f10, 0x1fc(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(508 as u32), tmp.u32 ) };
	// 825E8054: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E8058: 914B0218  stw r10, 0x218(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(536 as u32), ctx.r[10].u32 ) };
	// 825E805C: D00B0200  stfs f0, 0x200(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), tmp.u32 ) };
	// 825E8060: 914B021C  stw r10, 0x21c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(540 as u32), ctx.r[10].u32 ) };
	// 825E8064: D00B0204  stfs f0, 0x204(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(516 as u32), tmp.u32 ) };
	// 825E8068: 910B0220  stw r8, 0x220(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(544 as u32), ctx.r[8].u32 ) };
	// 825E806C: D1AB0208  stfs f13, 0x208(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(520 as u32), tmp.u32 ) };
	// 825E8070: 910B0224  stw r8, 0x224(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(548 as u32), ctx.r[8].u32 ) };
	// 825E8074: 3D002808  lis r8, 0x2808
	ctx.r[8].s64 = 671612928;
	// 825E8078: D00B020C  stfs f0, 0x20c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(524 as u32), tmp.u32 ) };
	// 825E807C: 61080002  ori r8, r8, 2
	ctx.r[8].u64 = ctx.r[8].u64 | 2;
	// 825E8080: 78E8000E  rldimi r8, r7, 0x20, 0
	ctx.r[8].u64 = ((ctx.r[7].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[8].u64 & 0x00000000FFFFFFFF);
	// 825E8084: F90B0228  std r8, 0x228(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(552 as u32), ctx.r[8].u64 ) };
	// 825E8088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E808C: B10B0230  sth r8, 0x230(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[8].u16 ) };
	// 825E8090: D1AB0254  stfs f13, 0x254(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(596 as u32), tmp.u32 ) };
	// 825E8094: B10B0232  sth r8, 0x232(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(562 as u32), ctx.r[8].u16 ) };
	// 825E8098: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825E809C: B10B0234  sth r8, 0x234(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(564 as u32), ctx.r[8].u16 ) };
	// 825E80A0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825E80A4: B10B0236  sth r8, 0x236(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(566 as u32), ctx.r[8].u16 ) };
	// 825E80A8: 3D000300  lis r8, 0x300
	ctx.r[8].s64 = 50331648;
	// 825E80AC: 914B0238  stw r10, 0x238(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(568 as u32), ctx.r[10].u32 ) };
	// 825E80B0: 914B023C  stw r10, 0x23c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(572 as u32), ctx.r[10].u32 ) };
	// 825E80B4: 61080003  ori r8, r8, 3
	ctx.r[8].u64 = ctx.r[8].u64 | 3;
	// 825E80B8: 914B0240  stw r10, 0x240(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(576 as u32), ctx.r[10].u32 ) };
	// 825E80BC: 914B0244  stw r10, 0x244(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(580 as u32), ctx.r[10].u32 ) };
	// 825E80C0: 914B0248  stw r10, 0x248(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(584 as u32), ctx.r[10].u32 ) };
	// 825E80C4: 914B024C  stw r10, 0x24c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(588 as u32), ctx.r[10].u32 ) };
	// 825E80C8: 914B0250  stw r10, 0x250(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(592 as u32), ctx.r[10].u32 ) };
	// 825E80CC: F94B0258  std r10, 0x258(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(600 as u32), ctx.r[10].u64 ) };
	// 825E80D0: F94B0260  std r10, 0x260(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(608 as u32), ctx.r[10].u64 ) };
	// 825E80D4: F94B0268  std r10, 0x268(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(616 as u32), ctx.r[10].u64 ) };
	// 825E80D8: 910B0270  stw r8, 0x270(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(624 as u32), ctx.r[8].u32 ) };
	// 825E80DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E80E0: D18B027C  stfs f12, 0x27c(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(636 as u32), tmp.u32 ) };
	// 825E80E4: 914B0274  stw r10, 0x274(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(628 as u32), ctx.r[10].u32 ) };
	// 825E80E8: D16B0280  stfs f11, 0x280(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(640 as u32), tmp.u32 ) };
	// 825E80EC: 910B0278  stw r8, 0x278(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(632 as u32), ctx.r[8].u32 ) };
	// 825E80F0: D1AB0284  stfs f13, 0x284(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(644 as u32), tmp.u32 ) };
	// 825E80F4: D1AB0288  stfs f13, 0x288(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 825E80F8: 914B028C  stw r10, 0x28c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(652 as u32), ctx.r[10].u32 ) };
	// 825E80FC: D00B0290  stfs f0, 0x290(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(656 as u32), tmp.u32 ) };
	// 825E8100: 912B02E0  stw r9, 0x2e0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(736 as u32), ctx.r[9].u32 ) };
	// 825E8104: D00B0294  stfs f0, 0x294(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 825E8108: 914B02E4  stw r10, 0x2e4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(740 as u32), ctx.r[10].u32 ) };
	// 825E810C: D00B0298  stfs f0, 0x298(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(664 as u32), tmp.u32 ) };
	// 825E8110: 914B02E8  stw r10, 0x2e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(744 as u32), ctx.r[10].u32 ) };
	// 825E8114: D00B029C  stfs f0, 0x29c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 825E8118: 914B02EC  stw r10, 0x2ec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(748 as u32), ctx.r[10].u32 ) };
	// 825E811C: D00B02A0  stfs f0, 0x2a0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(672 as u32), tmp.u32 ) };
	// 825E8120: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825E8124: D00B02A4  stfs f0, 0x2a4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(676 as u32), tmp.u32 ) };
	// 825E8128: 39000021  li r8, 0x21
	ctx.r[8].s64 = 33;
	// 825E812C: D00B02A8  stfs f0, 0x2a8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(680 as u32), tmp.u32 ) };
	// 825E8130: D00B02AC  stfs f0, 0x2ac(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(684 as u32), tmp.u32 ) };
	// 825E8134: D00B02B0  stfs f0, 0x2b0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(688 as u32), tmp.u32 ) };
	// 825E8138: D00B02B4  stfs f0, 0x2b4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(692 as u32), tmp.u32 ) };
	// 825E813C: D00B02B8  stfs f0, 0x2b8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(696 as u32), tmp.u32 ) };
	// 825E8140: D00B02BC  stfs f0, 0x2bc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(700 as u32), tmp.u32 ) };
	// 825E8144: D00B02C0  stfs f0, 0x2c0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(704 as u32), tmp.u32 ) };
	// 825E8148: D00B02C4  stfs f0, 0x2c4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(708 as u32), tmp.u32 ) };
	// 825E814C: D00B02C8  stfs f0, 0x2c8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(712 as u32), tmp.u32 ) };
	// 825E8150: D14B02CC  stfs f10, 0x2cc(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(716 as u32), tmp.u32 ) };
	// 825E8154: D00B02D0  stfs f0, 0x2d0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(720 as u32), tmp.u32 ) };
	// 825E8158: D00B02D4  stfs f0, 0x2d4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(724 as u32), tmp.u32 ) };
	// 825E815C: D1AB02D8  stfs f13, 0x2d8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(728 as u32), tmp.u32 ) };
	// 825E8160: D00B02DC  stfs f0, 0x2dc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(732 as u32), tmp.u32 ) };
	// 825E8164: D1AB0324  stfs f13, 0x324(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(804 as u32), tmp.u32 ) };
	// 825E8168: 912B02F0  stw r9, 0x2f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(752 as u32), ctx.r[9].u32 ) };
	// 825E816C: 912B02F4  stw r9, 0x2f4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(756 as u32), ctx.r[9].u32 ) };
	// 825E8170: 3D202808  lis r9, 0x2808
	ctx.r[9].s64 = 671612928;
	// 825E8174: 61290002  ori r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u64 | 2;
	// 825E8178: 7909000E  rldimi r9, r8, 0x20, 0
	ctx.r[9].u64 = ((ctx.r[8].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[9].u64 & 0x00000000FFFFFFFF);
	// 825E817C: F92B02F8  std r9, 0x2f8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(760 as u32), ctx.r[9].u64 ) };
	// 825E8180: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825E8184: B12B0300  sth r9, 0x300(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(768 as u32), ctx.r[9].u16 ) };
	// 825E8188: B12B0302  sth r9, 0x302(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(770 as u32), ctx.r[9].u16 ) };
	// 825E818C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825E8190: B12B0304  sth r9, 0x304(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(772 as u32), ctx.r[9].u16 ) };
	// 825E8194: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825E8198: B12B0306  sth r9, 0x306(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(774 as u32), ctx.r[9].u16 ) };
	// 825E819C: 914B0308  stw r10, 0x308(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(776 as u32), ctx.r[10].u32 ) };
	// 825E81A0: 914B030C  stw r10, 0x30c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(780 as u32), ctx.r[10].u32 ) };
	// 825E81A4: 914B0310  stw r10, 0x310(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(784 as u32), ctx.r[10].u32 ) };
	// 825E81A8: 914B0314  stw r10, 0x314(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(788 as u32), ctx.r[10].u32 ) };
	// 825E81AC: 914B0318  stw r10, 0x318(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(792 as u32), ctx.r[10].u32 ) };
	// 825E81B0: 914B031C  stw r10, 0x31c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(796 as u32), ctx.r[10].u32 ) };
	// 825E81B4: 914B0320  stw r10, 0x320(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(800 as u32), ctx.r[10].u32 ) };
	// 825E81B8: F94B0328  std r10, 0x328(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(808 as u32), ctx.r[10].u64 ) };
	// 825E81BC: F94B0330  std r10, 0x330(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(816 as u32), ctx.r[10].u64 ) };
	// 825E81C0: F94B0338  std r10, 0x338(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u64 ) };
	// 825E81C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E81C8 size=780
    let mut pc: u32 = 0x825E81C8;
    'dispatch: loop {
        match pc {
            0x825E81C8 => {
    //   block [0x825E81C8..0x825E84D4)
	// 825E81C8: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 825E81CC: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E81D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E81D4: 396B87C0  addi r11, r11, -0x7840
	ctx.r[11].s64 = ctx.r[11].s64 + -30784;
	// 825E81D8: 3D000300  lis r8, 0x300
	ctx.r[8].s64 = 50331648;
	// 825E81DC: C1872320  lfs f12, 0x2320(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8992 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825E81E0: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 825E81E4: 610800CF  ori r8, r8, 0xcf
	ctx.r[8].u64 = ctx.r[8].u64 | 207;
	// 825E81E8: 3D208287  lis r9, -0x7d79
	ctx.r[9].s64 = -2105081856;
	// 825E81EC: F94B00B8  std r10, 0xb8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u64 ) };
	// 825E81F0: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 825E81F4: F94B00C0  std r10, 0xc0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[10].u64 ) };
	// 825E81F8: 39297E80  addi r9, r9, 0x7e80
	ctx.r[9].s64 = ctx.r[9].s64 + 32384;
	// 825E81FC: C1672ADC  lfs f11, 0x2adc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(10972 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825E8200: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 825E8204: F94B00C8  std r10, 0xc8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u64 ) };
	// 825E8208: 910B00D0  stw r8, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[8].u32 ) };
	// 825E820C: D18B00DC  stfs f12, 0xdc(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 825E8210: 914B00D4  stw r10, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 825E8214: D16B00E0  stfs f11, 0xe0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 825E8218: 914B00D8  stw r10, 0xd8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(216 as u32), ctx.r[10].u32 ) };
	// 825E821C: C1473280  lfs f10, 0x3280(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12928 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825E8220: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 825E8224: D14B00E4  stfs f10, 0xe4(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 825E8228: C1A71FF8  lfs f13, 0x1ff8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E822C: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 825E8230: D1AB00E8  stfs f13, 0xe8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 825E8234: 914B00EC  stw r10, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 825E8238: C007BA38  lfs f0, -0x45c8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E823C: 38E90180  addi r7, r9, 0x180
	ctx.r[7].s64 = ctx.r[9].s64 + 384;
	// 825E8240: D00B00F0  stfs f0, 0xf0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 825E8244: 90EB0140  stw r7, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[7].u32 ) };
	// 825E8248: D00B00F4  stfs f0, 0xf4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 825E824C: 914B0144  stw r10, 0x144(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 825E8250: D00B00F8  stfs f0, 0xf8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 825E8254: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825E8258: 914B0148  stw r10, 0x148(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), ctx.r[10].u32 ) };
	// 825E825C: D00B00FC  stfs f0, 0xfc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 825E8260: 914B014C  stw r10, 0x14c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), ctx.r[10].u32 ) };
	// 825E8264: D00B0100  stfs f0, 0x100(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 825E8268: 90EB0150  stw r7, 0x150(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), ctx.r[7].u32 ) };
	// 825E826C: D00B0104  stfs f0, 0x104(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 825E8270: 90EB0154  stw r7, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[7].u32 ) };
	// 825E8274: 3CE02908  lis r7, 0x2908
	ctx.r[7].s64 = 688390144;
	// 825E8278: D00B0108  stfs f0, 0x108(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 825E827C: 60E70002  ori r7, r7, 2
	ctx.r[7].u64 = ctx.r[7].u64 | 2;
	// 825E8280: 78C7000E  rldimi r7, r6, 0x20, 0
	ctx.r[7].u64 = ((ctx.r[6].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[7].u64 & 0x00000000FFFFFFFF);
	// 825E8284: F8EB0158  std r7, 0x158(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[7].u64 ) };
	// 825E8288: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825E828C: D00B010C  stfs f0, 0x10c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 825E8290: B0EB0160  sth r7, 0x160(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(352 as u32), ctx.r[7].u16 ) };
	// 825E8294: D00B0110  stfs f0, 0x110(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 825E8298: B0EB0162  sth r7, 0x162(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(354 as u32), ctx.r[7].u16 ) };
	// 825E829C: D00B0114  stfs f0, 0x114(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 825E82A0: B14B0164  sth r10, 0x164(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(356 as u32), ctx.r[10].u16 ) };
	// 825E82A4: D00B0118  stfs f0, 0x118(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 825E82A8: B14B0166  sth r10, 0x166(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(358 as u32), ctx.r[10].u16 ) };
	// 825E82AC: D00B011C  stfs f0, 0x11c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 825E82B0: 914B0168  stw r10, 0x168(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(360 as u32), ctx.r[10].u32 ) };
	// 825E82B4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 825E82B8: D00B0120  stfs f0, 0x120(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 825E82BC: 914B016C  stw r10, 0x16c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(364 as u32), ctx.r[10].u32 ) };
	// 825E82C0: D00B0124  stfs f0, 0x124(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 825E82C4: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 825E82C8: D00B0128  stfs f0, 0x128(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 825E82CC: 914B0174  stw r10, 0x174(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(372 as u32), ctx.r[10].u32 ) };
	// 825E82D0: C127D6D0  lfs f9, -0x2930(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-10544 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825E82D4: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 825E82D8: D12B012C  stfs f9, 0x12c(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 825E82DC: 914B0178  stw r10, 0x178(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(376 as u32), ctx.r[10].u32 ) };
	// 825E82E0: D00B0130  stfs f0, 0x130(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 825E82E4: 914B017C  stw r10, 0x17c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(380 as u32), ctx.r[10].u32 ) };
	// 825E82E8: D00B0134  stfs f0, 0x134(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 825E82EC: 914B0180  stw r10, 0x180(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(384 as u32), ctx.r[10].u32 ) };
	// 825E82F0: D1AB0138  stfs f13, 0x138(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 825E82F4: C1072140  lfs f8, 0x2140(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8512 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 825E82F8: D00B013C  stfs f0, 0x13c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 825E82FC: D10B0184  stfs f8, 0x184(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 825E8300: F94B0188  std r10, 0x188(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u64 ) };
	// 825E8304: F94B0190  std r10, 0x190(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(400 as u32), ctx.r[10].u64 ) };
	// 825E8308: F94B0198  std r10, 0x198(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(408 as u32), ctx.r[10].u64 ) };
	// 825E830C: 38E90300  addi r7, r9, 0x300
	ctx.r[7].s64 = ctx.r[9].s64 + 768;
	// 825E8310: 910B01A0  stw r8, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[8].u32 ) };
	// 825E8314: D18B01AC  stfs f12, 0x1ac(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 825E8318: 914B01A4  stw r10, 0x1a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(420 as u32), ctx.r[10].u32 ) };
	// 825E831C: D16B01B0  stfs f11, 0x1b0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(432 as u32), tmp.u32 ) };
	// 825E8320: 914B01A8  stw r10, 0x1a8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(424 as u32), ctx.r[10].u32 ) };
	// 825E8324: D14B01B4  stfs f10, 0x1b4(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(436 as u32), tmp.u32 ) };
	// 825E8328: 39290480  addi r9, r9, 0x480
	ctx.r[9].s64 = ctx.r[9].s64 + 1152;
	// 825E832C: D1AB01B8  stfs f13, 0x1b8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 825E8330: 914B01BC  stw r10, 0x1bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(444 as u32), ctx.r[10].u32 ) };
	// 825E8334: D00B01C0  stfs f0, 0x1c0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 825E8338: D00B01C4  stfs f0, 0x1c4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(452 as u32), tmp.u32 ) };
	// 825E833C: D00B01C8  stfs f0, 0x1c8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(456 as u32), tmp.u32 ) };
	// 825E8340: D00B01CC  stfs f0, 0x1cc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 825E8344: D00B01D0  stfs f0, 0x1d0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), tmp.u32 ) };
	// 825E8348: D00B01D4  stfs f0, 0x1d4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(468 as u32), tmp.u32 ) };
	// 825E834C: D00B01D8  stfs f0, 0x1d8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(472 as u32), tmp.u32 ) };
	// 825E8350: D00B01DC  stfs f0, 0x1dc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(476 as u32), tmp.u32 ) };
	// 825E8354: D00B01E0  stfs f0, 0x1e0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(480 as u32), tmp.u32 ) };
	// 825E8358: D00B01E4  stfs f0, 0x1e4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(484 as u32), tmp.u32 ) };
	// 825E835C: D00B01E8  stfs f0, 0x1e8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), tmp.u32 ) };
	// 825E8360: D00B01EC  stfs f0, 0x1ec(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(492 as u32), tmp.u32 ) };
	// 825E8364: D00B01F0  stfs f0, 0x1f0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(496 as u32), tmp.u32 ) };
	// 825E8368: D00B01F4  stfs f0, 0x1f4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(500 as u32), tmp.u32 ) };
	// 825E836C: 90EB0210  stw r7, 0x210(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(528 as u32), ctx.r[7].u32 ) };
	// 825E8370: D00B01F8  stfs f0, 0x1f8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(504 as u32), tmp.u32 ) };
	// 825E8374: 914B0214  stw r10, 0x214(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(532 as u32), ctx.r[10].u32 ) };
	// 825E8378: D12B01FC  stfs f9, 0x1fc(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(508 as u32), tmp.u32 ) };
	// 825E837C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825E8380: 914B0218  stw r10, 0x218(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(536 as u32), ctx.r[10].u32 ) };
	// 825E8384: D00B0200  stfs f0, 0x200(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), tmp.u32 ) };
	// 825E8388: 914B021C  stw r10, 0x21c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(540 as u32), ctx.r[10].u32 ) };
	// 825E838C: D00B0204  stfs f0, 0x204(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(516 as u32), tmp.u32 ) };
	// 825E8390: 90EB0220  stw r7, 0x220(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(544 as u32), ctx.r[7].u32 ) };
	// 825E8394: D1AB0208  stfs f13, 0x208(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(520 as u32), tmp.u32 ) };
	// 825E8398: 90EB0224  stw r7, 0x224(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(548 as u32), ctx.r[7].u32 ) };
	// 825E839C: 3CE02908  lis r7, 0x2908
	ctx.r[7].s64 = 688390144;
	// 825E83A0: D00B020C  stfs f0, 0x20c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(524 as u32), tmp.u32 ) };
	// 825E83A4: 60E70002  ori r7, r7, 2
	ctx.r[7].u64 = ctx.r[7].u64 | 2;
	// 825E83A8: 78C7000E  rldimi r7, r6, 0x20, 0
	ctx.r[7].u64 = ((ctx.r[6].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[7].u64 & 0x00000000FFFFFFFF);
	// 825E83AC: F8EB0228  std r7, 0x228(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(552 as u32), ctx.r[7].u64 ) };
	// 825E83B0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825E83B4: B0EB0230  sth r7, 0x230(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[7].u16 ) };
	// 825E83B8: D10B0254  stfs f8, 0x254(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(596 as u32), tmp.u32 ) };
	// 825E83BC: B0EB0232  sth r7, 0x232(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(562 as u32), ctx.r[7].u16 ) };
	// 825E83C0: B14B0234  sth r10, 0x234(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(564 as u32), ctx.r[10].u16 ) };
	// 825E83C4: B14B0236  sth r10, 0x236(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(566 as u32), ctx.r[10].u16 ) };
	// 825E83C8: 914B0238  stw r10, 0x238(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(568 as u32), ctx.r[10].u32 ) };
	// 825E83CC: 914B023C  stw r10, 0x23c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(572 as u32), ctx.r[10].u32 ) };
	// 825E83D0: 914B0240  stw r10, 0x240(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(576 as u32), ctx.r[10].u32 ) };
	// 825E83D4: 914B0244  stw r10, 0x244(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(580 as u32), ctx.r[10].u32 ) };
	// 825E83D8: 914B0248  stw r10, 0x248(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(584 as u32), ctx.r[10].u32 ) };
	// 825E83DC: 914B024C  stw r10, 0x24c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(588 as u32), ctx.r[10].u32 ) };
	// 825E83E0: 914B0250  stw r10, 0x250(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(592 as u32), ctx.r[10].u32 ) };
	// 825E83E4: F94B0258  std r10, 0x258(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(600 as u32), ctx.r[10].u64 ) };
	// 825E83E8: F94B0260  std r10, 0x260(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(608 as u32), ctx.r[10].u64 ) };
	// 825E83EC: F94B0268  std r10, 0x268(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(616 as u32), ctx.r[10].u64 ) };
	// 825E83F0: 910B0270  stw r8, 0x270(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(624 as u32), ctx.r[8].u32 ) };
	// 825E83F4: D18B027C  stfs f12, 0x27c(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(636 as u32), tmp.u32 ) };
	// 825E83F8: 914B0274  stw r10, 0x274(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(628 as u32), ctx.r[10].u32 ) };
	// 825E83FC: D16B0280  stfs f11, 0x280(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(640 as u32), tmp.u32 ) };
	// 825E8400: 914B0278  stw r10, 0x278(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(632 as u32), ctx.r[10].u32 ) };
	// 825E8404: D14B0284  stfs f10, 0x284(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(644 as u32), tmp.u32 ) };
	// 825E8408: D1AB0288  stfs f13, 0x288(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 825E840C: 914B028C  stw r10, 0x28c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(652 as u32), ctx.r[10].u32 ) };
	// 825E8410: D00B0290  stfs f0, 0x290(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(656 as u32), tmp.u32 ) };
	// 825E8414: 912B02E0  stw r9, 0x2e0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(736 as u32), ctx.r[9].u32 ) };
	// 825E8418: D00B0294  stfs f0, 0x294(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 825E841C: 914B02E4  stw r10, 0x2e4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(740 as u32), ctx.r[10].u32 ) };
	// 825E8420: D00B0298  stfs f0, 0x298(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(664 as u32), tmp.u32 ) };
	// 825E8424: 914B02E8  stw r10, 0x2e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(744 as u32), ctx.r[10].u32 ) };
	// 825E8428: D00B029C  stfs f0, 0x29c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 825E842C: 914B02EC  stw r10, 0x2ec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(748 as u32), ctx.r[10].u32 ) };
	// 825E8430: D00B02A0  stfs f0, 0x2a0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(672 as u32), tmp.u32 ) };
	// 825E8434: D00B02A4  stfs f0, 0x2a4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(676 as u32), tmp.u32 ) };
	// 825E8438: D00B02A8  stfs f0, 0x2a8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(680 as u32), tmp.u32 ) };
	// 825E843C: D00B02AC  stfs f0, 0x2ac(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(684 as u32), tmp.u32 ) };
	// 825E8440: D00B02B0  stfs f0, 0x2b0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(688 as u32), tmp.u32 ) };
	// 825E8444: D00B02B4  stfs f0, 0x2b4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(692 as u32), tmp.u32 ) };
	// 825E8448: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825E844C: D00B02B8  stfs f0, 0x2b8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(696 as u32), tmp.u32 ) };
	// 825E8450: 39000120  li r8, 0x120
	ctx.r[8].s64 = 288;
	// 825E8454: D00B02BC  stfs f0, 0x2bc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(700 as u32), tmp.u32 ) };
	// 825E8458: D00B02C0  stfs f0, 0x2c0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(704 as u32), tmp.u32 ) };
	// 825E845C: D00B02C4  stfs f0, 0x2c4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(708 as u32), tmp.u32 ) };
	// 825E8460: D00B02C8  stfs f0, 0x2c8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(712 as u32), tmp.u32 ) };
	// 825E8464: D12B02CC  stfs f9, 0x2cc(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(716 as u32), tmp.u32 ) };
	// 825E8468: D00B02D0  stfs f0, 0x2d0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(720 as u32), tmp.u32 ) };
	// 825E846C: D00B02D4  stfs f0, 0x2d4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(724 as u32), tmp.u32 ) };
	// 825E8470: D1AB02D8  stfs f13, 0x2d8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(728 as u32), tmp.u32 ) };
	// 825E8474: D00B02DC  stfs f0, 0x2dc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(732 as u32), tmp.u32 ) };
	// 825E8478: D10B0324  stfs f8, 0x324(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(804 as u32), tmp.u32 ) };
	// 825E847C: 912B02F0  stw r9, 0x2f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(752 as u32), ctx.r[9].u32 ) };
	// 825E8480: 912B02F4  stw r9, 0x2f4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(756 as u32), ctx.r[9].u32 ) };
	// 825E8484: 3D202908  lis r9, 0x2908
	ctx.r[9].s64 = 688390144;
	// 825E8488: 61290002  ori r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u64 | 2;
	// 825E848C: 7909000E  rldimi r9, r8, 0x20, 0
	ctx.r[9].u64 = ((ctx.r[8].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[9].u64 & 0x00000000FFFFFFFF);
	// 825E8490: F92B02F8  std r9, 0x2f8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(760 as u32), ctx.r[9].u64 ) };
	// 825E8494: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825E8498: B12B0300  sth r9, 0x300(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(768 as u32), ctx.r[9].u16 ) };
	// 825E849C: B12B0302  sth r9, 0x302(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(770 as u32), ctx.r[9].u16 ) };
	// 825E84A0: B14B0304  sth r10, 0x304(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(772 as u32), ctx.r[10].u16 ) };
	// 825E84A4: B14B0306  sth r10, 0x306(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(774 as u32), ctx.r[10].u16 ) };
	// 825E84A8: 914B0308  stw r10, 0x308(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(776 as u32), ctx.r[10].u32 ) };
	// 825E84AC: 914B030C  stw r10, 0x30c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(780 as u32), ctx.r[10].u32 ) };
	// 825E84B0: 914B0310  stw r10, 0x310(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(784 as u32), ctx.r[10].u32 ) };
	// 825E84B4: 914B0314  stw r10, 0x314(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(788 as u32), ctx.r[10].u32 ) };
	// 825E84B8: 914B0318  stw r10, 0x318(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(792 as u32), ctx.r[10].u32 ) };
	// 825E84BC: 914B031C  stw r10, 0x31c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(796 as u32), ctx.r[10].u32 ) };
	// 825E84C0: 914B0320  stw r10, 0x320(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(800 as u32), ctx.r[10].u32 ) };
	// 825E84C4: F94B0328  std r10, 0x328(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(808 as u32), ctx.r[10].u64 ) };
	// 825E84C8: F94B0330  std r10, 0x330(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(816 as u32), ctx.r[10].u64 ) };
	// 825E84CC: F94B0338  std r10, 0x338(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u64 ) };
	// 825E84D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E84D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E84D8 size=92
    let mut pc: u32 = 0x825E84D8;
    'dispatch: loop {
        match pc {
            0x825E84D8 => {
    //   block [0x825E84D8..0x825E8500)
	// 825E84D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E84DC: 4BF4CBD9  bl 0x825350b4
	ctx.lr = 0x825E84E0;
	sub_82535080(ctx, base);
	// 825E84E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E84E4: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E84E8: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 825E84EC: 396BF4F8  addi r11, r11, -0xb08
	ctx.r[11].s64 = ctx.r[11].s64 + -2824;
	// 825E84F0: 3B60041D  li r27, 0x41d
	ctx.r[27].s64 = 1053;
	// 825E84F4: 3BEB000B  addi r31, r11, 0xb
	ctx.r[31].s64 = ctx.r[11].s64 + 11;
	// 825E84F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825E84FC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	pc = 0x825E8500; continue 'dispatch;
            }
            0x825E8500 => {
    //   block [0x825E8500..0x825E8534)
	// 825E8500: 38A00048  li r5, 0x48
	ctx.r[5].s64 = 72;
	// 825E8504: B37FFFFD  sth r27, -3(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(-3 as u32), ctx.r[27].u16 ) };
	// 825E8508: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E850C: 9B9FFFFF  stb r28, -1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-1 as u32), ctx.r[28].u8 ) };
	// 825E8510: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 825E8514: 9BBF0000  stb r29, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825E8518: 4BF4CCB9  bl 0x825351d0
	ctx.lr = 0x825E851C;
	sub_825351D0(ctx, base);
	// 825E851C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 825E8520: 3BFF0054  addi r31, r31, 0x54
	ctx.r[31].s64 = ctx.r[31].s64 + 84;
	// 825E8524: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825E8528: 4098FFD8  bge cr6, 0x825e8500
	if !ctx.cr[6].lt {
	pc = 0x825E8500; continue 'dispatch;
	}
	// 825E852C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E8530: 4BF4CBD4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8538 size=12
    let mut pc: u32 = 0x825E8538;
    'dispatch: loop {
        match pc {
            0x825E8538 => {
    //   block [0x825E8538..0x825E8544)
	// 825E8538: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E853C: 386BC508  addi r3, r11, -0x3af8
	ctx.r[3].s64 = ctx.r[11].s64 + -15096;
	// 825E8540: 4BF4A5F8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8548 size=12
    let mut pc: u32 = 0x825E8548;
    'dispatch: loop {
        match pc {
            0x825E8548 => {
    //   block [0x825E8548..0x825E8554)
	// 825E8548: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E854C: 386BC528  addi r3, r11, -0x3ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -15064;
	// 825E8550: 4BF4A5E8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8558 size=12
    let mut pc: u32 = 0x825E8558;
    'dispatch: loop {
        match pc {
            0x825E8558 => {
    //   block [0x825E8558..0x825E8564)
	// 825E8558: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E855C: 386BC548  addi r3, r11, -0x3ab8
	ctx.r[3].s64 = ctx.r[11].s64 + -15032;
	// 825E8560: 4BF4A5D8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825E8568 size=76
    let mut pc: u32 = 0x825E8568;
    'dispatch: loop {
        match pc {
            0x825E8568 => {
    //   block [0x825E8568..0x825E85B4)
	// 825E8568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E856C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E8570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E8574: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E8578: E86BC278  ld r3, -0x3d88(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-15752 as u32) ) };
	// 825E857C: 4BF4DAD5  bl 0x82536050
	ctx.lr = 0x825E8580;
	sub_82536050(ctx, base);
	// 825E8580: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825E8584: 3D4082B5  lis r10, -0x7d4b
	ctx.r[10].s64 = -2102067200;
	// 825E8588: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E858C: 394A0C40  addi r10, r10, 0xc40
	ctx.r[10].s64 = ctx.r[10].s64 + 3136;
	// 825E8590: 396BF810  addi r11, r11, -0x7f0
	ctx.r[11].s64 = ctx.r[11].s64 + -2032;
	// 825E8594: C00A06B0  lfs f0, 0x6b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E8598: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 825E859C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825E85A0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825E85A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E85A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E85AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E85B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E85B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825E85B8 size=76
    let mut pc: u32 = 0x825E85B8;
    'dispatch: loop {
        match pc {
            0x825E85B8 => {
    //   block [0x825E85B8..0x825E8604)
	// 825E85B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E85BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E85C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E85C4: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E85C8: E86BC278  ld r3, -0x3d88(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-15752 as u32) ) };
	// 825E85CC: 4BF4DA85  bl 0x82536050
	ctx.lr = 0x825E85D0;
	sub_82536050(ctx, base);
	// 825E85D0: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825E85D4: 3D4082B5  lis r10, -0x7d4b
	ctx.r[10].s64 = -2102067200;
	// 825E85D8: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E85DC: 394A0C40  addi r10, r10, 0xc40
	ctx.r[10].s64 = ctx.r[10].s64 + 3136;
	// 825E85E0: 396BF844  addi r11, r11, -0x7bc
	ctx.r[11].s64 = ctx.r[11].s64 + -1980;
	// 825E85E4: C00A06B0  lfs f0, 0x6b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E85E8: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 825E85EC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825E85F0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825E85F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E85F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E85FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E8600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825E8608 size=76
    let mut pc: u32 = 0x825E8608;
    'dispatch: loop {
        match pc {
            0x825E8608 => {
    //   block [0x825E8608..0x825E8654)
	// 825E8608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E860C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E8610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E8614: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E8618: E86BC278  ld r3, -0x3d88(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-15752 as u32) ) };
	// 825E861C: 4BF4DA35  bl 0x82536050
	ctx.lr = 0x825E8620;
	sub_82536050(ctx, base);
	// 825E8620: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825E8624: 3D4082B5  lis r10, -0x7d4b
	ctx.r[10].s64 = -2102067200;
	// 825E8628: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E862C: 394A0C40  addi r10, r10, 0xc40
	ctx.r[10].s64 = ctx.r[10].s64 + 3136;
	// 825E8630: 396BF870  addi r11, r11, -0x790
	ctx.r[11].s64 = ctx.r[11].s64 + -1936;
	// 825E8634: C00A06B0  lfs f0, 0x6b0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E8638: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 825E863C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825E8640: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 825E8644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E8648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E864C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E8650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8658 size=44
    let mut pc: u32 = 0x825E8658;
    'dispatch: loop {
        match pc {
            0x825E8658 => {
    //   block [0x825E8658..0x825E8684)
	// 825E8658: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 825E865C: 3D40830F  lis r10, -0x7cf1
	ctx.r[10].s64 = -2096168960;
	// 825E8660: 396BBFD8  addi r11, r11, -0x4028
	ctx.r[11].s64 = ctx.r[11].s64 + -16424;
	// 825E8664: 394AF848  addi r10, r10, -0x7b8
	ctx.r[10].s64 = ctx.r[10].s64 + -1976;
	// 825E8668: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825E866C: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825E8670: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825E8674: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825E8678: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825E867C: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825E8680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E8688 size=308
    let mut pc: u32 = 0x825E8688;
    'dispatch: loop {
        match pc {
            0x825E8688 => {
    //   block [0x825E8688..0x825E87BC)
	// 825E8688: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E868C: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8690: 394BC440  addi r10, r11, -0x3bc0
	ctx.r[10].s64 = ctx.r[11].s64 + -15296;
	// 825E8694: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 825E8698: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E869C: 7C0A5C2E  lfsx f0, r10, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E86A0: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E86A4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E86A8: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E86AC: 396BF880  addi r11, r11, -0x780
	ctx.r[11].s64 = ctx.r[11].s64 + -1920;
	// 825E86B0: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E86B4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825E86B8: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825E86BC: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E86C0: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E86C4: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825E86C8: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E86CC: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E86D0: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E86D4: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825E86D8: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E86DC: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E86E0: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E86E4: D1AB0010  stfs f13, 0x10(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825E86E8: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E86EC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E86F0: D1AB0014  stfs f13, 0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825E86F4: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E86F8: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E86FC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8700: D1AB0018  stfs f13, 0x18(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825E8704: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8708: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E870C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8710: D1AB001C  stfs f13, 0x1c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825E8714: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8718: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E871C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8720: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825E8724: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8728: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E872C: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E8730: D1AB0024  stfs f13, 0x24(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825E8734: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8738: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E873C: D1AB0028  stfs f13, 0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825E8740: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E8744: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8748: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E874C: D1AB002C  stfs f13, 0x2c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825E8750: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8754: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8758: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E875C: D1AB0030  stfs f13, 0x30(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825E8760: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8764: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8768: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E876C: D1AB0034  stfs f13, 0x34(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825E8770: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8774: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8778: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E877C: D1AB0038  stfs f13, 0x38(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825E8780: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8784: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8788: D1AB003C  stfs f13, 0x3c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825E878C: C1AA3330  lfs f13, 0x3330(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8790: D1AB0040  stfs f13, 0x40(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825E8794: C1AA6664  lfs f13, 0x6664(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8798: D1AB0044  stfs f13, 0x44(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825E879C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E87A0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E87A4: D1AB0048  stfs f13, 0x48(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825E87A8: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E87AC: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E87B0: D1AB004C  stfs f13, 0x4c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825E87B4: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825E87B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E87C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E87C0 size=308
    let mut pc: u32 = 0x825E87C0;
    'dispatch: loop {
        match pc {
            0x825E87C0 => {
    //   block [0x825E87C0..0x825E88F4)
	// 825E87C0: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E87C4: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E87C8: 394BC440  addi r10, r11, -0x3bc0
	ctx.r[10].s64 = ctx.r[11].s64 + -15296;
	// 825E87CC: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E87D0: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E87D4: 396BF8D8  addi r11, r11, -0x728
	ctx.r[11].s64 = ctx.r[11].s64 + -1832;
	// 825E87D8: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E87DC: C1AA3330  lfs f13, 0x3330(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E87E0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825E87E4: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825E87E8: C1AA6664  lfs f13, 0x6664(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E87EC: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825E87F0: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E87F4: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E87F8: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825E87FC: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8800: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8804: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8808: D1AB0010  stfs f13, 0x10(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825E880C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8810: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8814: D1AB0014  stfs f13, 0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825E8818: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E881C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8820: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8824: D1AB0018  stfs f13, 0x18(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825E8828: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E882C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8830: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8834: D1AB001C  stfs f13, 0x1c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825E8838: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E883C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8840: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8844: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825E8848: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E884C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8850: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8854: D1AB0024  stfs f13, 0x24(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825E8858: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E885C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8860: D1AB0028  stfs f13, 0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825E8864: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E8868: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E886C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8870: D1AB002C  stfs f13, 0x2c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825E8874: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8878: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E887C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8880: D1AB0030  stfs f13, 0x30(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825E8884: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8888: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E888C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8890: D1AB0034  stfs f13, 0x34(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825E8894: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8898: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E889C: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E88A0: D1AB0038  stfs f13, 0x38(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825E88A4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E88A8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E88AC: D1AB003C  stfs f13, 0x3c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825E88B0: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E88B4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E88B8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E88BC: D1AB0040  stfs f13, 0x40(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825E88C0: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E88C4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E88C8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E88CC: D1AB0044  stfs f13, 0x44(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825E88D0: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E88D4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E88D8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E88DC: D1AB0048  stfs f13, 0x48(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825E88E0: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E88E4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E88E8: D1AB004C  stfs f13, 0x4c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825E88EC: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825E88F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E88F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E88F8 size=308
    let mut pc: u32 = 0x825E88F8;
    'dispatch: loop {
        match pc {
            0x825E88F8 => {
    //   block [0x825E88F8..0x825E8A2C)
	// 825E88F8: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E88FC: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8900: 394BC440  addi r10, r11, -0x3bc0
	ctx.r[10].s64 = ctx.r[11].s64 + -15296;
	// 825E8904: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 825E8908: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E890C: 7C0A5C2E  lfsx f0, r10, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E8910: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E8914: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8918: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E891C: 396BF988  addi r11, r11, -0x678
	ctx.r[11].s64 = ctx.r[11].s64 + -1656;
	// 825E8920: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8924: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825E8928: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825E892C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8930: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8934: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825E8938: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E893C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8940: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8944: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825E8948: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E894C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8950: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8954: D1AB0010  stfs f13, 0x10(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825E8958: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E895C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8960: D1AB0014  stfs f13, 0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825E8964: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E8968: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E896C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8970: D1AB0018  stfs f13, 0x18(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825E8974: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8978: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E897C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8980: D1AB001C  stfs f13, 0x1c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825E8984: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8988: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E898C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8990: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825E8994: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8998: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E899C: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E89A0: D1AB0024  stfs f13, 0x24(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825E89A4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E89A8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E89AC: D1AB0028  stfs f13, 0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825E89B0: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E89B4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E89B8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E89BC: D1AB002C  stfs f13, 0x2c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825E89C0: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E89C4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E89C8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E89CC: D1AB0030  stfs f13, 0x30(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825E89D0: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E89D4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E89D8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E89DC: D1AB0034  stfs f13, 0x34(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825E89E0: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E89E4: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E89E8: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E89EC: D1AB0038  stfs f13, 0x38(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825E89F0: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E89F4: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E89F8: D1AB003C  stfs f13, 0x3c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825E89FC: C1AA3330  lfs f13, 0x3330(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A00: D1AB0040  stfs f13, 0x40(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825E8A04: C1AA6664  lfs f13, 0x6664(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A08: D1AB0044  stfs f13, 0x44(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825E8A0C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A10: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E8A14: D1AB0048  stfs f13, 0x48(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825E8A18: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8A1C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A20: D1AB004C  stfs f13, 0x4c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825E8A24: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825E8A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E8A30 size=308
    let mut pc: u32 = 0x825E8A30;
    'dispatch: loop {
        match pc {
            0x825E8A30 => {
    //   block [0x825E8A30..0x825E8B64)
	// 825E8A30: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E8A34: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E8A38: 394BC440  addi r10, r11, -0x3bc0
	ctx.r[10].s64 = ctx.r[11].s64 + -15296;
	// 825E8A3C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E8A40: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8A44: 396BF930  addi r11, r11, -0x6d0
	ctx.r[11].s64 = ctx.r[11].s64 + -1744;
	// 825E8A48: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825E8A4C: C1AA3330  lfs f13, 0x3330(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A50: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825E8A54: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825E8A58: C1AA6664  lfs f13, 0x6664(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A5C: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825E8A60: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A64: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825E8A68: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825E8A6C: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8A70: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A74: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8A78: D1AB0010  stfs f13, 0x10(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825E8A7C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A80: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8A84: D1AB0014  stfs f13, 0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825E8A88: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E8A8C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8A90: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8A94: D1AB0018  stfs f13, 0x18(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 825E8A98: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8A9C: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8AA0: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8AA4: D1AB001C  stfs f13, 0x1c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 825E8AA8: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8AAC: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8AB0: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 825E8AB4: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825E8AB8: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8ABC: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8AC0: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8AC4: D1AB0024  stfs f13, 0x24(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825E8AC8: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8ACC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8AD0: D1AB0028  stfs f13, 0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825E8AD4: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E8AD8: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8ADC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8AE0: D1AB002C  stfs f13, 0x2c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 825E8AE4: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8AE8: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8AEC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8AF0: D1AB0030  stfs f13, 0x30(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825E8AF4: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8AF8: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8AFC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 825E8B00: D1AB0034  stfs f13, 0x34(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825E8B04: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8B08: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8B0C: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E8B10: D1AB0038  stfs f13, 0x38(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825E8B14: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8B18: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E8B1C: D1AB003C  stfs f13, 0x3c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 825E8B20: 61293330  ori r9, r9, 0x3330
	ctx.r[9].u64 = ctx.r[9].u64 | 13104;
	// 825E8B24: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8B28: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E8B2C: D1AB0040  stfs f13, 0x40(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825E8B30: 61296664  ori r9, r9, 0x6664
	ctx.r[9].u64 = ctx.r[9].u64 | 26212;
	// 825E8B34: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8B38: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E8B3C: D1AB0044  stfs f13, 0x44(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825E8B40: 61299998  ori r9, r9, 0x9998
	ctx.r[9].u64 = ctx.r[9].u64 | 39320;
	// 825E8B44: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8B48: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 825E8B4C: D1AB0048  stfs f13, 0x48(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825E8B50: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 825E8B54: 7DAA4C2E  lfsx f13, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825E8B58: D1AB004C  stfs f13, 0x4c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825E8B5C: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825E8B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825E8B68 size=64
    let mut pc: u32 = 0x825E8B68;
    'dispatch: loop {
        match pc {
            0x825E8B68 => {
    //   block [0x825E8B68..0x825E8B84)
	// 825E8B68: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E8B6C: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 825E8B70: 392B8BB0  addi r9, r11, -0x7450
	ctx.r[9].s64 = ctx.r[11].s64 + -29776;
	// 825E8B74: 3900003C  li r8, 0x3c
	ctx.r[8].s64 = 60;
	// 825E8B78: 39690004  addi r11, r9, 4
	ctx.r[11].s64 = ctx.r[9].s64 + 4;
	// 825E8B7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E8B80: C007BA38  lfs f0, -0x45c8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x825E8B84; continue 'dispatch;
            }
            0x825E8B84 => {
    //   block [0x825E8B84..0x825E8BA8)
	// 825E8B84: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825E8B88: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825E8B8C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825E8B90: 38E900A4  addi r7, r9, 0xa4
	ctx.r[7].s64 = ctx.r[9].s64 + 164;
	// 825E8B94: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825E8B98: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 825E8B9C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 825E8BA0: 4198FFE4  blt cr6, 0x825e8b84
	if ctx.cr[6].lt {
	pc = 0x825E8B84; continue 'dispatch;
	}
	// 825E8BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8BA8 size=92
    let mut pc: u32 = 0x825E8BA8;
    'dispatch: loop {
        match pc {
            0x825E8BA8 => {
    //   block [0x825E8BA8..0x825E8BC0)
	// 825E8BA8: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E8BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E8BB0: 396BFAC0  addi r11, r11, -0x540
	ctx.r[11].s64 = ctx.r[11].s64 + -1344;
	// 825E8BB4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825E8BB8: 394B0268  addi r10, r11, 0x268
	ctx.r[10].s64 = ctx.r[11].s64 + 616;
	// 825E8BBC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x825E8BC0; continue 'dispatch;
            }
            0x825E8BC0 => {
    //   block [0x825E8BC0..0x825E8C04)
	// 825E8BC0: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 825E8BC4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825E8BC8: 4200FFF8  bdnz 0x825e8bc0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825E8BC0; continue 'dispatch;
	}
	// 825E8BCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E8BD0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825E8BD4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825E8BD8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825E8BDC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825E8BE0: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825E8BE4: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 825E8BE8: 994B0016  stb r10, 0x16(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(22 as u32), ctx.r[10].u8 ) };
	// 825E8BEC: 994B0015  stb r10, 0x15(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 825E8BF0: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 825E8BF4: 994B0017  stb r10, 0x17(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(23 as u32), ctx.r[10].u8 ) };
	// 825E8BF8: B14B001C  sth r10, 0x1c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u16 ) };
	// 825E8BFC: B14B001E  sth r10, 0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 825E8C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8C08 size=32
    let mut pc: u32 = 0x825E8C08;
    'dispatch: loop {
        match pc {
            0x825E8C08 => {
    //   block [0x825E8C08..0x825E8C28)
	// 825E8C08: 3D40830F  lis r10, -0x7cf1
	ctx.r[10].s64 = -2096168960;
	// 825E8C0C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825E8C10: 394A1E00  addi r10, r10, 0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + 7680;
	// 825E8C14: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825E8C18: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825E8C1C: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825E8C20: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825E8C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8C28 size=12
    let mut pc: u32 = 0x825E8C28;
    'dispatch: loop {
        match pc {
            0x825E8C28 => {
    //   block [0x825E8C28..0x825E8C34)
	// 825E8C28: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E8C2C: 386B1E40  addi r3, r11, 0x1e40
	ctx.r[3].s64 = ctx.r[11].s64 + 7744;
	// 825E8C30: 4BD8A4C0  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8C38 size=12
    let mut pc: u32 = 0x825E8C38;
    'dispatch: loop {
        match pc {
            0x825E8C38 => {
    //   block [0x825E8C38..0x825E8C44)
	// 825E8C38: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E8C3C: 386B2640  addi r3, r11, 0x2640
	ctx.r[3].s64 = ctx.r[11].s64 + 9792;
	// 825E8C40: 4BD8A4B0  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8C48 size=12
    let mut pc: u32 = 0x825E8C48;
    'dispatch: loop {
        match pc {
            0x825E8C48 => {
    //   block [0x825E8C48..0x825E8C54)
	// 825E8C48: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E8C4C: 386B2240  addi r3, r11, 0x2240
	ctx.r[3].s64 = ctx.r[11].s64 + 8768;
	// 825E8C50: 4BD8A4A0  b 0x823730f0
	sub_823730F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E8C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E8C58 size=964
    let mut pc: u32 = 0x825E8C58;
    'dispatch: loop {
        match pc {
            0x825E8C58 => {
    //   block [0x825E8C58..0x825E901C)
	// 825E8C58: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825E8C5C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E8C60: 3C801200  lis r4, 0x1200
	ctx.r[4].s64 = 301989888;
	// 825E8C64: 396BCDE0  addi r11, r11, -0x3220
	ctx.r[11].s64 = ctx.r[11].s64 + -12832;
	// 825E8C68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E8C6C: 60840001  ori r4, r4, 1
	ctx.r[4].u64 = ctx.r[4].u64 | 1;
	// 825E8C70: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E8C74: 3CC08285  lis r6, -0x7d7b
	ctx.r[6].s64 = -2105212928;
	// 825E8C78: 390AF9F0  addi r8, r10, -0x610
	ctx.r[8].s64 = ctx.r[10].s64 + -1552;
	// 825E8C7C: 912B0034  stw r9, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 825E8C80: 908B0038  stw r4, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 825E8C84: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 825E8C88: B12B003C  sth r9, 0x3c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[9].u16 ) };
	// 825E8C8C: 3D4082B5  lis r10, -0x7d4b
	ctx.r[10].s64 = -2102067200;
	// 825E8C90: 38C682E8  addi r6, r6, -0x7d18
	ctx.r[6].s64 = ctx.r[6].s64 + -32024;
	// 825E8C94: 38EA4020  addi r7, r10, 0x4020
	ctx.r[7].s64 = ctx.r[10].s64 + 16416;
	// 825E8C98: 3D408285  lis r10, -0x7d7b
	ctx.r[10].s64 = -2105212928;
	// 825E8C9C: B08B003E  sth r4, 0x3e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(62 as u32), ctx.r[4].u16 ) };
	// 825E8CA0: 3CA082B5  lis r5, -0x7d4b
	ctx.r[5].s64 = -2102067200;
	// 825E8CA4: B08B0040  sth r4, 0x40(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[4].u16 ) };
	// 825E8CA8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825E8CAC: B12B0042  sth r9, 0x42(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(66 as u32), ctx.r[9].u16 ) };
	// 825E8CB0: 394A82D0  addi r10, r10, -0x7d30
	ctx.r[10].s64 = ctx.r[10].s64 + -32048;
	// 825E8CB4: B12B0044  sth r9, 0x44(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u16 ) };
	// 825E8CB8: 38A533E0  addi r5, r5, 0x33e0
	ctx.r[5].s64 = ctx.r[5].s64 + 13280;
	// 825E8CBC: 386AFD00  addi r3, r10, -0x300
	ctx.r[3].s64 = ctx.r[10].s64 + -768;
	// 825E8CC0: 3BEAFB00  addi r31, r10, -0x500
	ctx.r[31].s64 = ctx.r[10].s64 + -1280;
	// 825E8CC4: B08B0046  sth r4, 0x46(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(70 as u32), ctx.r[4].u16 ) };
	// 825E8CC8: B08B0048  sth r4, 0x48(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[4].u16 ) };
	// 825E8CCC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 825E8CD0: B08B004A  sth r4, 0x4a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(74 as u32), ctx.r[4].u16 ) };
	// 825E8CD4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 825E8CD8: B08B004C  sth r4, 0x4c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[4].u16 ) };
	// 825E8CDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825E8CE0: B08B004E  sth r4, 0x4e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(78 as u32), ctx.r[4].u16 ) };
	// 825E8CE4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 825E8CE8: B08B0050  sth r4, 0x50(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[4].u16 ) };
	// 825E8CEC: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 825E8CF0: B08B0052  sth r4, 0x52(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(82 as u32), ctx.r[4].u16 ) };
	// 825E8CF4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 825E8CF8: B08B0054  sth r4, 0x54(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[4].u16 ) };
	// 825E8CFC: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 825E8D00: B08B0056  sth r4, 0x56(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(86 as u32), ctx.r[4].u16 ) };
	// 825E8D04: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 825E8D08: B08B0058  sth r4, 0x58(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[4].u16 ) };
	// 825E8D0C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 825E8D10: B08B005A  sth r4, 0x5a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(90 as u32), ctx.r[4].u16 ) };
	// 825E8D14: 910B005C  stw r8, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E8D18: 90EB0060  stw r7, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 825E8D1C: 3CE01200  lis r7, 0x1200
	ctx.r[7].s64 = 301989888;
	// 825E8D20: 906B0064  stw r3, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 825E8D24: 90CB0068  stw r6, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 825E8D28: 912B006C  stw r9, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825E8D2C: 90EB0070  stw r7, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 825E8D30: 38E00400  li r7, 0x400
	ctx.r[7].s64 = 1024;
	// 825E8D34: B12B0074  sth r9, 0x74(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[9].u16 ) };
	// 825E8D38: B0EB0076  sth r7, 0x76(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(118 as u32), ctx.r[7].u16 ) };
	// 825E8D3C: 38E00100  li r7, 0x100
	ctx.r[7].s64 = 256;
	// 825E8D40: B0EB0078  sth r7, 0x78(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u16 ) };
	// 825E8D44: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 825E8D48: B12B007A  sth r9, 0x7a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(122 as u32), ctx.r[9].u16 ) };
	// 825E8D4C: B12B007C  sth r9, 0x7c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[9].u16 ) };
	// 825E8D50: B0EB007E  sth r7, 0x7e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(126 as u32), ctx.r[7].u16 ) };
	// 825E8D54: B0EB0080  sth r7, 0x80(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[7].u16 ) };
	// 825E8D58: B0EB0082  sth r7, 0x82(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(130 as u32), ctx.r[7].u16 ) };
	// 825E8D5C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825E8D60: B0EB0084  sth r7, 0x84(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[7].u16 ) };
	// 825E8D64: B0EB0086  sth r7, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[7].u16 ) };
	// 825E8D68: 38E0001E  li r7, 0x1e
	ctx.r[7].s64 = 30;
	// 825E8D6C: B0EB0088  sth r7, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[7].u16 ) };
	// 825E8D70: B0EB008A  sth r7, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[7].u16 ) };
	// 825E8D74: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 825E8D78: B0EB008C  sth r7, 0x8c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[7].u16 ) };
	// 825E8D7C: 38E0001B  li r7, 0x1b
	ctx.r[7].s64 = 27;
	// 825E8D80: B0EB008E  sth r7, 0x8e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(142 as u32), ctx.r[7].u16 ) };
	// 825E8D84: 38E00100  li r7, 0x100
	ctx.r[7].s64 = 256;
	// 825E8D88: B12B0090  sth r9, 0x90(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[9].u16 ) };
	// 825E8D8C: B0EB0092  sth r7, 0x92(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(146 as u32), ctx.r[7].u16 ) };
	// 825E8D90: 910B0094  stw r8, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[8].u32 ) };
	// 825E8D94: 90AB0098  stw r5, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[5].u32 ) };
	// 825E8D98: 3D001200  lis r8, 0x1200
	ctx.r[8].s64 = 301989888;
	// 825E8D9C: 93EB009C  stw r31, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 825E8DA0: 912B00A0  stw r9, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 825E8DA4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 825E8DA8: 61080002  ori r8, r8, 2
	ctx.r[8].u64 = ctx.r[8].u64 | 2;
	// 825E8DAC: 912B00A4  stw r9, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[9].u32 ) };
	// 825E8DB0: 3CE082B5  lis r7, -0x7d4b
	ctx.r[7].s64 = -2102067200;
	// 825E8DB4: 38AAFF00  addi r5, r10, -0x100
	ctx.r[5].s64 = ctx.r[10].s64 + -256;
	// 825E8DB8: 38E727E0  addi r7, r7, 0x27e0
	ctx.r[7].s64 = ctx.r[7].s64 + 10208;
	// 825E8DBC: 3C801200  lis r4, 0x1200
	ctx.r[4].s64 = 301989888;
	// 825E8DC0: 910B00A8  stw r8, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[8].u32 ) };
	// 825E8DC4: 390000C0  li r8, 0xc0
	ctx.r[8].s64 = 192;
	// 825E8DC8: B12B00AC  sth r9, 0xac(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[9].u16 ) };
	// 825E8DCC: 60840003  ori r4, r4, 3
	ctx.r[4].u64 = ctx.r[4].u64 | 3;
	// 825E8DD0: 386AFF40  addi r3, r10, -0xc0
	ctx.r[3].s64 = ctx.r[10].s64 + -192;
	// 825E8DD4: 3BEAFF58  addi r31, r10, -0xa8
	ctx.r[31].s64 = ctx.r[10].s64 + -168;
	// 825E8DD8: B10B00AE  sth r8, 0xae(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(174 as u32), ctx.r[8].u16 ) };
	// 825E8DDC: B10B00B0  sth r8, 0xb0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[8].u16 ) };
	// 825E8DE0: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 825E8DE4: B12B00B2  sth r9, 0xb2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(178 as u32), ctx.r[9].u16 ) };
	// 825E8DE8: B12B00B4  sth r9, 0xb4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u16 ) };
	// 825E8DEC: B10B00B6  sth r8, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[8].u16 ) };
	// 825E8DF0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 825E8DF4: B10B00B8  sth r8, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[8].u16 ) };
	// 825E8DF8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825E8DFC: B10B00BA  sth r8, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[8].u16 ) };
	// 825E8E00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E8E04: B10B00BC  sth r8, 0xbc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[8].u16 ) };
	// 825E8E08: B10B00BE  sth r8, 0xbe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(190 as u32), ctx.r[8].u16 ) };
	// 825E8E0C: 3900003F  li r8, 0x3f
	ctx.r[8].s64 = 63;
	// 825E8E10: B10B00C0  sth r8, 0xc0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[8].u16 ) };
	// 825E8E14: 3900001F  li r8, 0x1f
	ctx.r[8].s64 = 31;
	// 825E8E18: B10B00C2  sth r8, 0xc2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(194 as u32), ctx.r[8].u16 ) };
	// 825E8E1C: 3900003F  li r8, 0x3f
	ctx.r[8].s64 = 63;
	// 825E8E20: B10B00C4  sth r8, 0xc4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(196 as u32), ctx.r[8].u16 ) };
	// 825E8E24: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825E8E28: B0CB00C6  sth r6, 0xc6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(198 as u32), ctx.r[6].u16 ) };
	// 825E8E2C: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 825E8E30: 3908F9DC  addi r8, r8, -0x624
	ctx.r[8].s64 = ctx.r[8].s64 + -1572;
	// 825E8E34: B12B00C8  sth r9, 0xc8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[9].u16 ) };
	// 825E8E38: B0CB00CA  sth r6, 0xca(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(202 as u32), ctx.r[6].u16 ) };
	// 825E8E3C: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 825E8E40: 910B00CC  stw r8, 0xcc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), ctx.r[8].u32 ) };
	// 825E8E44: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825E8E48: 90EB00D0  stw r7, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[7].u32 ) };
	// 825E8E4C: 3CE082B5  lis r7, -0x7d4b
	ctx.r[7].s64 = -2102067200;
	// 825E8E50: 90AB00D4  stw r5, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[5].u32 ) };
	// 825E8E54: 3CA082B5  lis r5, -0x7d4b
	ctx.r[5].s64 = -2102067200;
	// 825E8E58: 912B00D8  stw r9, 0xd8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(216 as u32), ctx.r[9].u32 ) };
	// 825E8E5C: 3908F9D0  addi r8, r8, -0x630
	ctx.r[8].s64 = ctx.r[8].s64 + -1584;
	// 825E8E60: 912B00DC  stw r9, 0xdc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), ctx.r[9].u32 ) };
	// 825E8E64: 908B00E0  stw r4, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[4].u32 ) };
	// 825E8E68: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 825E8E6C: B12B00E4  sth r9, 0xe4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[9].u16 ) };
	// 825E8E70: 38E72BE0  addi r7, r7, 0x2be0
	ctx.r[7].s64 = ctx.r[7].s64 + 11232;
	// 825E8E74: 38C6F978  addi r6, r6, -0x688
	ctx.r[6].s64 = ctx.r[6].s64 + -1672;
	// 825E8E78: 38A52FE0  addi r5, r5, 0x2fe0
	ctx.r[5].s64 = ctx.r[5].s64 + 12256;
	// 825E8E7C: B08B00E6  sth r4, 0xe6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(230 as u32), ctx.r[4].u16 ) };
	// 825E8E80: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 825E8E84: B08B00E8  sth r4, 0xe8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[4].u16 ) };
	// 825E8E88: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825E8E8C: B12B00EA  sth r9, 0xea(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(234 as u32), ctx.r[9].u16 ) };
	// 825E8E90: B12B00EC  sth r9, 0xec(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[9].u16 ) };
	// 825E8E94: B08B00EE  sth r4, 0xee(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(238 as u32), ctx.r[4].u16 ) };
	// 825E8E98: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 825E8E9C: B08B00F0  sth r4, 0xf0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[4].u16 ) };
	// 825E8EA0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825E8EA4: B08B00F2  sth r4, 0xf2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(242 as u32), ctx.r[4].u16 ) };
	// 825E8EA8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825E8EAC: B12B00F4  sth r9, 0xf4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), ctx.r[9].u16 ) };
	// 825E8EB0: B12B00F6  sth r9, 0xf6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(246 as u32), ctx.r[9].u16 ) };
	// 825E8EB4: B08B00F8  sth r4, 0xf8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[4].u16 ) };
	// 825E8EB8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 825E8EBC: B08B00FA  sth r4, 0xfa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(250 as u32), ctx.r[4].u16 ) };
	// 825E8EC0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825E8EC4: B08B00FC  sth r4, 0xfc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(252 as u32), ctx.r[4].u16 ) };
	// 825E8EC8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 825E8ECC: B08B00FE  sth r4, 0xfe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(254 as u32), ctx.r[4].u16 ) };
	// 825E8ED0: B12B0100  sth r9, 0x100(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(256 as u32), ctx.r[9].u16 ) };
	// 825E8ED4: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825E8ED8: B08B0102  sth r4, 0x102(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(258 as u32), ctx.r[4].u16 ) };
	// 825E8EDC: 910B0104  stw r8, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[8].u32 ) };
	// 825E8EE0: 3D001200  lis r8, 0x1200
	ctx.r[8].s64 = 301989888;
	// 825E8EE4: 90EB0108  stw r7, 0x108(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[7].u32 ) };
	// 825E8EE8: 3CE082B5  lis r7, -0x7d4b
	ctx.r[7].s64 = -2102067200;
	// 825E8EEC: 61080004  ori r8, r8, 4
	ctx.r[8].u64 = ctx.r[8].u64 | 4;
	// 825E8EF0: 906B010C  stw r3, 0x10c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(268 as u32), ctx.r[3].u32 ) };
	// 825E8EF4: 912B0110  stw r9, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[9].u32 ) };
	// 825E8EF8: 38E73C20  addi r7, r7, 0x3c20
	ctx.r[7].s64 = ctx.r[7].s64 + 15392;
	// 825E8EFC: 912B0114  stw r9, 0x114(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(276 as u32), ctx.r[9].u32 ) };
	// 825E8F00: 910B0118  stw r8, 0x118(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(280 as u32), ctx.r[8].u32 ) };
	// 825E8F04: 390001C0  li r8, 0x1c0
	ctx.r[8].s64 = 448;
	// 825E8F08: B12B011C  sth r9, 0x11c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(284 as u32), ctx.r[9].u16 ) };
	// 825E8F0C: B10B011E  sth r8, 0x11e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(286 as u32), ctx.r[8].u16 ) };
	// 825E8F10: 39000180  li r8, 0x180
	ctx.r[8].s64 = 384;
	// 825E8F14: B10B0120  sth r8, 0x120(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(288 as u32), ctx.r[8].u16 ) };
	// 825E8F18: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 825E8F1C: B12B0122  sth r9, 0x122(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(290 as u32), ctx.r[9].u16 ) };
	// 825E8F20: B12B0124  sth r9, 0x124(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(292 as u32), ctx.r[9].u16 ) };
	// 825E8F24: B10B0126  sth r8, 0x126(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(294 as u32), ctx.r[8].u16 ) };
	// 825E8F28: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 825E8F2C: B10B0128  sth r8, 0x128(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[8].u16 ) };
	// 825E8F30: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825E8F34: B10B012A  sth r8, 0x12a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(298 as u32), ctx.r[8].u16 ) };
	// 825E8F38: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 825E8F3C: B12B012C  sth r9, 0x12c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(300 as u32), ctx.r[9].u16 ) };
	// 825E8F40: B12B012E  sth r9, 0x12e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(302 as u32), ctx.r[9].u16 ) };
	// 825E8F44: B10B0130  sth r8, 0x130(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(304 as u32), ctx.r[8].u16 ) };
	// 825E8F48: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 825E8F4C: B10B0132  sth r8, 0x132(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(306 as u32), ctx.r[8].u16 ) };
	// 825E8F50: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 825E8F54: B10B0134  sth r8, 0x134(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(308 as u32), ctx.r[8].u16 ) };
	// 825E8F58: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 825E8F5C: B10B0136  sth r8, 0x136(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(310 as u32), ctx.r[8].u16 ) };
	// 825E8F60: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 825E8F64: B12B0138  sth r9, 0x138(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(312 as u32), ctx.r[9].u16 ) };
	// 825E8F68: B10B013A  sth r8, 0x13a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(314 as u32), ctx.r[8].u16 ) };
	// 825E8F6C: 3D001200  lis r8, 0x1200
	ctx.r[8].s64 = 301989888;
	// 825E8F70: 90CB013C  stw r6, 0x13c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(316 as u32), ctx.r[6].u32 ) };
	// 825E8F74: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825E8F78: 61080005  ori r8, r8, 5
	ctx.r[8].u64 = ctx.r[8].u64 | 5;
	// 825E8F7C: 90AB0140  stw r5, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[5].u32 ) };
	// 825E8F80: 93EB0144  stw r31, 0x144(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(324 as u32), ctx.r[31].u32 ) };
	// 825E8F84: 912B0148  stw r9, 0x148(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), ctx.r[9].u32 ) };
	// 825E8F88: 912B014C  stw r9, 0x14c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), ctx.r[9].u32 ) };
	// 825E8F8C: 910B0150  stw r8, 0x150(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), ctx.r[8].u32 ) };
	// 825E8F90: 390000F0  li r8, 0xf0
	ctx.r[8].s64 = 240;
	// 825E8F94: B12B0154  sth r9, 0x154(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[9].u16 ) };
	// 825E8F98: B10B0156  sth r8, 0x156(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(342 as u32), ctx.r[8].u16 ) };
	// 825E8F9C: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 825E8FA0: B10B0158  sth r8, 0x158(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[8].u16 ) };
	// 825E8FA4: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 825E8FA8: B12B015A  sth r9, 0x15a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(346 as u32), ctx.r[9].u16 ) };
	// 825E8FAC: B12B015C  sth r9, 0x15c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(348 as u32), ctx.r[9].u16 ) };
	// 825E8FB0: 3908F96C  addi r8, r8, -0x694
	ctx.r[8].s64 = ctx.r[8].s64 + -1684;
	// 825E8FB4: B0CB015E  sth r6, 0x15e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(350 as u32), ctx.r[6].u16 ) };
	// 825E8FB8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825E8FBC: B0CB0160  sth r6, 0x160(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(352 as u32), ctx.r[6].u16 ) };
	// 825E8FC0: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 825E8FC4: B0CB0162  sth r6, 0x162(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(354 as u32), ctx.r[6].u16 ) };
	// 825E8FC8: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825E8FCC: B12B0164  sth r9, 0x164(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(356 as u32), ctx.r[9].u16 ) };
	// 825E8FD0: B12B0166  sth r9, 0x166(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(358 as u32), ctx.r[9].u16 ) };
	// 825E8FD4: B0CB0168  sth r6, 0x168(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(360 as u32), ctx.r[6].u16 ) };
	// 825E8FD8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825E8FDC: B0CB016A  sth r6, 0x16a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(362 as u32), ctx.r[6].u16 ) };
	// 825E8FE0: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825E8FE4: B0CB016C  sth r6, 0x16c(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(364 as u32), ctx.r[6].u16 ) };
	// 825E8FE8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825E8FEC: B0CB016E  sth r6, 0x16e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(366 as u32), ctx.r[6].u16 ) };
	// 825E8FF0: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 825E8FF4: B12B0170  sth r9, 0x170(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[9].u16 ) };
	// 825E8FF8: B0CB0172  sth r6, 0x172(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(370 as u32), ctx.r[6].u16 ) };
	// 825E8FFC: 910B0174  stw r8, 0x174(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(372 as u32), ctx.r[8].u32 ) };
	// 825E9000: 90EB0178  stw r7, 0x178(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(376 as u32), ctx.r[7].u32 ) };
	// 825E9004: 914B017C  stw r10, 0x17c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(380 as u32), ctx.r[10].u32 ) };
	// 825E9008: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 825E900C: 914B0180  stw r10, 0x180(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(384 as u32), ctx.r[10].u32 ) };
	// 825E9010: 912B0184  stw r9, 0x184(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(388 as u32), ctx.r[9].u32 ) };
	// 825E9014: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825E9018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9020 size=24
    let mut pc: u32 = 0x825E9020;
    'dispatch: loop {
        match pc {
            0x825E9020 => {
    //   block [0x825E9020..0x825E9038)
	// 825E9020: 3D40830F  lis r10, -0x7cf1
	ctx.r[10].s64 = -2096168960;
	// 825E9024: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825E9028: 394A2A40  addi r10, r10, 0x2a40
	ctx.r[10].s64 = ctx.r[10].s64 + 10816;
	// 825E902C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825E9030: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825E9034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9038 size=40
    let mut pc: u32 = 0x825E9038;
    'dispatch: loop {
        match pc {
            0x825E9038 => {
    //   block [0x825E9038..0x825E9048)
	// 825E9038: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E903C: 394B2A48  addi r10, r11, 0x2a48
	ctx.r[10].s64 = ctx.r[11].s64 + 10824;
	// 825E9040: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 825E9044: 394A001C  addi r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + 28;
	pc = 0x825E9048; continue 'dispatch;
            }
            0x825E9048 => {
    //   block [0x825E9048..0x825E9060)
	// 825E9048: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E904C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825E9050: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825E9054: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 825E9058: 4080FFF0  bge 0x825e9048
	if !ctx.cr[0].lt {
	pc = 0x825E9048; continue 'dispatch;
	}
	// 825E905C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9060 size=12
    let mut pc: u32 = 0x825E9060;
    'dispatch: loop {
        match pc {
            0x825E9060 => {
    //   block [0x825E9060..0x825E906C)
	// 825E9060: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9064: 386BC568  addi r3, r11, -0x3a98
	ctx.r[3].s64 = ctx.r[11].s64 + -15000;
	// 825E9068: 4BF49AD0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9070 size=44
    let mut pc: u32 = 0x825E9070;
    'dispatch: loop {
        match pc {
            0x825E9070 => {
    //   block [0x825E9070..0x825E9088)
	// 825E9070: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E9074: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 825E9078: 394B3100  addi r10, r11, 0x3100
	ctx.r[10].s64 = ctx.r[11].s64 + 12544;
	// 825E907C: 3960002F  li r11, 0x2f
	ctx.r[11].s64 = 47;
	// 825E9080: 394A0088  addi r10, r10, 0x88
	ctx.r[10].s64 = ctx.r[10].s64 + 136;
	// 825E9084: 3929FC24  addi r9, r9, -0x3dc
	ctx.r[9].s64 = ctx.r[9].s64 + -988;
	pc = 0x825E9088; continue 'dispatch;
            }
            0x825E9088 => {
    //   block [0x825E9088..0x825E909C)
	// 825E9088: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825E908C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825E9090: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 825E9094: 4080FFF4  bge 0x825e9088
	if !ctx.cr[0].lt {
	pc = 0x825E9088; continue 'dispatch;
	}
	// 825E9098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E90A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E90A0 size=84
    let mut pc: u32 = 0x825E90A0;
    'dispatch: loop {
        match pc {
            0x825E90A0 => {
    //   block [0x825E90A0..0x825E90C4)
	// 825E90A0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 825E90A4: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 825E90A8: 390A0D0C  addi r8, r10, 0xd0c
	ctx.r[8].s64 = ctx.r[10].s64 + 3340;
	// 825E90AC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E90B0: 38CB4A00  addi r6, r11, 0x4a00
	ctx.r[6].s64 = ctx.r[11].s64 + 18944;
	// 825E90B4: 38EAFC28  addi r7, r10, -0x3d8
	ctx.r[7].s64 = ctx.r[10].s64 + -984;
	// 825E90B8: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 825E90BC: 396602D0  addi r11, r6, 0x2d0
	ctx.r[11].s64 = ctx.r[6].s64 + 720;
	// 825E90C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x825E90C4; continue 'dispatch;
            }
            0x825E90C4 => {
    //   block [0x825E90C4..0x825E90F4)
	// 825E90C4: 3CA0DEAD  lis r5, -0x2153
	ctx.r[5].s64 = -559087616;
	// 825E90C8: 914BFE58  stw r10, -0x1a8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-424 as u32), ctx.r[10].u32 ) };
	// 825E90CC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825E90D0: 90ABFE5C  stw r5, -0x1a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-420 as u32), ctx.r[5].u32 ) };
	// 825E90D4: 90EBFF20  stw r7, -0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-224 as u32), ctx.r[7].u32 ) };
	// 825E90D8: F94BFFD0  std r10, -0x30(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(-48 as u32), ctx.r[10].u64 ) };
	// 825E90DC: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 825E90E0: 910BFFC0  stw r8, -0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-64 as u32), ctx.r[8].u32 ) };
	// 825E90E4: 396B03C0  addi r11, r11, 0x3c0
	ctx.r[11].s64 = ctx.r[11].s64 + 960;
	// 825E90E8: 4080FFDC  bge 0x825e90c4
	if !ctx.cr[0].lt {
	pc = 0x825E90C4; continue 'dispatch;
	}
	// 825E90EC: 91460058  stw r10, 0x58(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825E90F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E90F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E90F8 size=60
    let mut pc: u32 = 0x825E90F8;
    'dispatch: loop {
        match pc {
            0x825E90F8 => {
    //   block [0x825E90F8..0x825E910C)
	// 825E90F8: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E90FC: 392001FE  li r9, 0x1fe
	ctx.r[9].s64 = 510;
	// 825E9100: 396B86B8  addi r11, r11, -0x7948
	ctx.r[11].s64 = ctx.r[11].s64 + -31048;
	// 825E9104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9108: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	pc = 0x825E910C; continue 'dispatch;
            }
            0x825E910C => {
    //   block [0x825E910C..0x825E9134)
	// 825E910C: 994BFFFE  stb r10, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u8 ) };
	// 825E9110: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825E9114: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 825E9118: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 825E911C: 914B0002  stw r10, 2(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u32 ) };
	// 825E9120: 914B0006  stw r10, 6(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u32 ) };
	// 825E9124: 914B0012  stw r10, 0x12(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[10].u32 ) };
	// 825E9128: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 825E912C: 4080FFE0  bge 0x825e910c
	if !ctx.cr[0].lt {
	pc = 0x825E910C; continue 'dispatch;
	}
	// 825E9130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9138 size=12
    let mut pc: u32 = 0x825E9138;
    'dispatch: loop {
        match pc {
            0x825E9138 => {
    //   block [0x825E9138..0x825E9144)
	// 825E9138: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E913C: 386BC570  addi r3, r11, -0x3a90
	ctx.r[3].s64 = ctx.r[11].s64 + -14992;
	// 825E9140: 4BF499F8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9148 size=72
    let mut pc: u32 = 0x825E9148;
    'dispatch: loop {
        match pc {
            0x825E9148 => {
    //   block [0x825E9148..0x825E915C)
	// 825E9148: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E914C: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825E9150: 396BC630  addi r11, r11, -0x39d0
	ctx.r[11].s64 = ctx.r[11].s64 + -14800;
	// 825E9154: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9158: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	pc = 0x825E915C; continue 'dispatch;
            }
            0x825E915C => {
    //   block [0x825E915C..0x825E9190)
	// 825E915C: 914BFFF8  stw r10, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 825E9160: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825E9164: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 825E9168: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825E916C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825E9170: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825E9174: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825E9178: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825E917C: 396B0078  addi r11, r11, 0x78
	ctx.r[11].s64 = ctx.r[11].s64 + 120;
	// 825E9180: 4080FFDC  bge 0x825e915c
	if !ctx.cr[0].lt {
	pc = 0x825E915C; continue 'dispatch;
	}
	// 825E9184: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9188: 386BC590  addi r3, r11, -0x3a70
	ctx.r[3].s64 = ctx.r[11].s64 + -14960;
	// 825E918C: 4BF499AC  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825E9190 size=128
    let mut pc: u32 = 0x825E9190;
    'dispatch: loop {
        match pc {
            0x825E9190 => {
    //   block [0x825E9190..0x825E91C4)
	// 825E9190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9194: 4BF4BF29  bl 0x825350bc
	ctx.lr = 0x825E9198;
	sub_82535080(ctx, base);
	// 825E9198: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 825E919C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825E91A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E91A4: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 825E91A8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E91AC: 3BABC440  addi r29, r11, -0x3bc0
	ctx.r[29].s64 = ctx.r[11].s64 + -15296;
	// 825E91B0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825E91B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825E91B8: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 825E91BC: C3CA2154  lfs f30, 0x2154(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8532 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825E91C0: C3EB2204  lfs f31, 0x2204(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x825E91C4; continue 'dispatch;
            }
            0x825E91C4 => {
    //   block [0x825E91C4..0x825E9210)
	// 825E91C4: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 825E91C8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825E91CC: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825E91D0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825E91D4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825E91D8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 825E91DC: EC2007B2  fmuls f1, f0, f30
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 825E91E0: 4BF4D079  bl 0x82536258
	ctx.lr = 0x825E91E4;
	sub_82536258(ctx, base);
	// 825E91E4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 825E91E8: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825E91EC: 3D7D0004  addis r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 262144;
	// 825E91F0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825E91F4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825E91F8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825E91FC: 4198FFC8  blt cr6, 0x825e91c4
	if ctx.cr[6].lt {
	pc = 0x825E91C4; continue 'dispatch;
	}
	// 825E9200: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825E9204: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 825E9208: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825E920C: 4BF4BF00  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9210 size=72
    let mut pc: u32 = 0x825E9210;
    'dispatch: loop {
        match pc {
            0x825E9210 => {
    //   block [0x825E9210..0x825E9258)
	// 825E9210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E921C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9220: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E9224: 3BEBEA00  addi r31, r11, -0x1600
	ctx.r[31].s64 = ctx.r[11].s64 + -5632;
	// 825E9228: 387F0454  addi r3, r31, 0x454
	ctx.r[3].s64 = ctx.r[31].s64 + 1108;
	// 825E922C: 4BDB14CD  bl 0x8239a6f8
	ctx.lr = 0x825E9230;
	sub_8239A6F8(ctx, base);
	// 825E9230: 387F0548  addi r3, r31, 0x548
	ctx.r[3].s64 = ctx.r[31].s64 + 1352;
	// 825E9234: 4BDB14C5  bl 0x8239a6f8
	ctx.lr = 0x825E9238;
	sub_8239A6F8(ctx, base);
	// 825E9238: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E923C: 386BC598  addi r3, r11, -0x3a68
	ctx.r[3].s64 = ctx.r[11].s64 + -14952;
	// 825E9240: 4BF498F9  bl 0x82532b38
	ctx.lr = 0x825E9244;
	sub_82532B38(ctx, base);
	// 825E9244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E924C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E9254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9258 size=104
    let mut pc: u32 = 0x825E9258;
    'dispatch: loop {
        match pc {
            0x825E9258 => {
    //   block [0x825E9258..0x825E92C0)
	// 825E9258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E925C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E9264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9268: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E926C: 3BEBCA80  addi r31, r11, -0x3580
	ctx.r[31].s64 = ctx.r[11].s64 + -13696;
	// 825E9270: 387F1BFC  addi r3, r31, 0x1bfc
	ctx.r[3].s64 = ctx.r[31].s64 + 7164;
	// 825E9274: 4BDB155D  bl 0x8239a7d0
	ctx.lr = 0x825E9278;
	sub_8239A7D0(ctx, base);
	// 825E9278: 387F1C60  addi r3, r31, 0x1c60
	ctx.r[3].s64 = ctx.r[31].s64 + 7264;
	// 825E927C: 4BDB1555  bl 0x8239a7d0
	ctx.lr = 0x825E9280;
	sub_8239A7D0(ctx, base);
	// 825E9280: 387F1CC4  addi r3, r31, 0x1cc4
	ctx.r[3].s64 = ctx.r[31].s64 + 7364;
	// 825E9284: 4BDB1475  bl 0x8239a6f8
	ctx.lr = 0x825E9288;
	sub_8239A6F8(ctx, base);
	// 825E9288: 387F1DB8  addi r3, r31, 0x1db8
	ctx.r[3].s64 = ctx.r[31].s64 + 7608;
	// 825E928C: 4BDB146D  bl 0x8239a6f8
	ctx.lr = 0x825E9290;
	sub_8239A6F8(ctx, base);
	// 825E9290: 387F1EAC  addi r3, r31, 0x1eac
	ctx.r[3].s64 = ctx.r[31].s64 + 7852;
	// 825E9294: 4BDB153D  bl 0x8239a7d0
	ctx.lr = 0x825E9298;
	sub_8239A7D0(ctx, base);
	// 825E9298: 387F1F10  addi r3, r31, 0x1f10
	ctx.r[3].s64 = ctx.r[31].s64 + 7952;
	// 825E929C: 4BDB1535  bl 0x8239a7d0
	ctx.lr = 0x825E92A0;
	sub_8239A7D0(ctx, base);
	// 825E92A0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E92A4: 386BC5F8  addi r3, r11, -0x3a08
	ctx.r[3].s64 = ctx.r[11].s64 + -14856;
	// 825E92A8: 4BF49891  bl 0x82532b38
	ctx.lr = 0x825E92AC;
	sub_82532B38(ctx, base);
	// 825E92AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E92B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E92B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E92B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E92BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E92C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E92C0 size=52
    let mut pc: u32 = 0x825E92C0;
    'dispatch: loop {
        match pc {
            0x825E92C0 => {
    //   block [0x825E92C0..0x825E92F4)
	// 825E92C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E92C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E92C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E92CC: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E92D0: 386BF080  addi r3, r11, -0xf80
	ctx.r[3].s64 = ctx.r[11].s64 + -3968;
	// 825E92D4: 4BDB51E5  bl 0x8239e4b8
	ctx.lr = 0x825E92D8;
	sub_8239E4B8(ctx, base);
	// 825E92D8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E92DC: 386BC618  addi r3, r11, -0x39e8
	ctx.r[3].s64 = ctx.r[11].s64 + -14824;
	// 825E92E0: 4BF49859  bl 0x82532b38
	ctx.lr = 0x825E92E4;
	sub_82532B38(ctx, base);
	// 825E92E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E92E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E92EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E92F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E92F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E92F8 size=52
    let mut pc: u32 = 0x825E92F8;
    'dispatch: loop {
        match pc {
            0x825E92F8 => {
    //   block [0x825E92F8..0x825E932C)
	// 825E92F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E92FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9304: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E9308: 386B0A00  addi r3, r11, 0xa00
	ctx.r[3].s64 = ctx.r[11].s64 + 2560;
	// 825E930C: 4BDBFF05  bl 0x823a9210
	ctx.lr = 0x825E9310;
	sub_823A9210(ctx, base);
	// 825E9310: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9314: 386BC680  addi r3, r11, -0x3980
	ctx.r[3].s64 = ctx.r[11].s64 + -14720;
	// 825E9318: 4BF49821  bl 0x82532b38
	ctx.lr = 0x825E931C;
	sub_82532B38(ctx, base);
	// 825E931C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9330 size=12
    let mut pc: u32 = 0x825E9330;
    'dispatch: loop {
        match pc {
            0x825E9330 => {
    //   block [0x825E9330..0x825E933C)
	// 825E9330: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9334: 386BC6B0  addi r3, r11, -0x3950
	ctx.r[3].s64 = ctx.r[11].s64 + -14672;
	// 825E9338: 4BF49800  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9340 size=12
    let mut pc: u32 = 0x825E9340;
    'dispatch: loop {
        match pc {
            0x825E9340 => {
    //   block [0x825E9340..0x825E934C)
	// 825E9340: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9344: 386BC730  addi r3, r11, -0x38d0
	ctx.r[3].s64 = ctx.r[11].s64 + -14544;
	// 825E9348: 4BF497F0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9350 size=12
    let mut pc: u32 = 0x825E9350;
    'dispatch: loop {
        match pc {
            0x825E9350 => {
    //   block [0x825E9350..0x825E935C)
	// 825E9350: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9354: 386BC750  addi r3, r11, -0x38b0
	ctx.r[3].s64 = ctx.r[11].s64 + -14512;
	// 825E9358: 4BF497E0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9360 size=12
    let mut pc: u32 = 0x825E9360;
    'dispatch: loop {
        match pc {
            0x825E9360 => {
    //   block [0x825E9360..0x825E936C)
	// 825E9360: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9364: 386BC770  addi r3, r11, -0x3890
	ctx.r[3].s64 = ctx.r[11].s64 + -14480;
	// 825E9368: 4BF497D0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9370 size=12
    let mut pc: u32 = 0x825E9370;
    'dispatch: loop {
        match pc {
            0x825E9370 => {
    //   block [0x825E9370..0x825E937C)
	// 825E9370: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9374: 386BC788  addi r3, r11, -0x3878
	ctx.r[3].s64 = ctx.r[11].s64 + -14456;
	// 825E9378: 4BF497C0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9380 size=52
    let mut pc: u32 = 0x825E9380;
    'dispatch: loop {
        match pc {
            0x825E9380 => {
    //   block [0x825E9380..0x825E93B4)
	// 825E9380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E938C: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E9390: 386B1B00  addi r3, r11, 0x1b00
	ctx.r[3].s64 = ctx.r[11].s64 + 6912;
	// 825E9394: 4BDB5D8D  bl 0x8239f120
	ctx.lr = 0x825E9398;
	sub_8239F120(ctx, base);
	// 825E9398: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E939C: 386BC7A8  addi r3, r11, -0x3858
	ctx.r[3].s64 = ctx.r[11].s64 + -14424;
	// 825E93A0: 4BF49799  bl 0x82532b38
	ctx.lr = 0x825E93A4;
	sub_82532B38(ctx, base);
	// 825E93A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E93A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E93AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E93B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E93B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E93B8 size=36
    let mut pc: u32 = 0x825E93B8;
    'dispatch: loop {
        match pc {
            0x825E93B8 => {
    //   block [0x825E93B8..0x825E93DC)
	// 825E93B8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E93BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E93C0: 396B9000  addi r11, r11, -0x7000
	ctx.r[11].s64 = ctx.r[11].s64 + -28672;
	// 825E93C4: 3D208271  lis r9, -0x7d8f
	ctx.r[9].s64 = -2106523648;
	// 825E93C8: 3869C7C0  addi r3, r9, -0x3840
	ctx.r[3].s64 = ctx.r[9].s64 + -14400;
	// 825E93CC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825E93D0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825E93D4: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825E93D8: 4BF49760  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E93E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E93E0 size=136
    let mut pc: u32 = 0x825E93E0;
    'dispatch: loop {
        match pc {
            0x825E93E0 => {
    //   block [0x825E93E0..0x825E9404)
	// 825E93E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E93E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E93E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E93EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E93F0: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 825E93F4: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825E93F8: 3BEB1E80  addi r31, r11, 0x1e80
	ctx.r[31].s64 = ctx.r[11].s64 + 7808;
	// 825E93FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9400: 397F0040  addi r11, r31, 0x40
	ctx.r[11].s64 = ctx.r[31].s64 + 64;
	pc = 0x825E9404; continue 'dispatch;
            }
            0x825E9404 => {
    //   block [0x825E9404..0x825E9468)
	// 825E9404: 914BFFF8  stw r10, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 825E9408: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825E940C: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 825E9410: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825E9414: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825E9418: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825E941C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825E9420: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825E9424: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825E9428: 4080FFDC  bge 0x825e9404
	if !ctx.cr[0].lt {
	pc = 0x825E9404; continue 'dispatch;
	}
	// 825E942C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825E9430: 387F40A8  addi r3, r31, 0x40a8
	ctx.r[3].s64 = ctx.r[31].s64 + 16552;
	// 825E9434: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 825E9438: 915F00A4  stw r10, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 825E943C: 4BDB63D5  bl 0x8239f810
	ctx.lr = 0x825E9440;
	sub_8239F810(ctx, base);
	// 825E9440: 387F58DC  addi r3, r31, 0x58dc
	ctx.r[3].s64 = ctx.r[31].s64 + 22748;
	// 825E9444: 4BDB63CD  bl 0x8239f810
	ctx.lr = 0x825E9448;
	sub_8239F810(ctx, base);
	// 825E9448: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E944C: 386BC820  addi r3, r11, -0x37e0
	ctx.r[3].s64 = ctx.r[11].s64 + -14304;
	// 825E9450: 4BF496E9  bl 0x82532b38
	ctx.lr = 0x825E9454;
	sub_82532B38(ctx, base);
	// 825E9454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E945C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E9464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9468 size=12
    let mut pc: u32 = 0x825E9468;
    'dispatch: loop {
        match pc {
            0x825E9468 => {
    //   block [0x825E9468..0x825E9474)
	// 825E9468: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E946C: 386BC830  addi r3, r11, -0x37d0
	ctx.r[3].s64 = ctx.r[11].s64 + -14288;
	// 825E9470: 4BF496C8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9478 size=12
    let mut pc: u32 = 0x825E9478;
    'dispatch: loop {
        match pc {
            0x825E9478 => {
    //   block [0x825E9478..0x825E9484)
	// 825E9478: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E947C: 386BC838  addi r3, r11, -0x37c8
	ctx.r[3].s64 = ctx.r[11].s64 + -14280;
	// 825E9480: 4BF496B8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9488 size=12
    let mut pc: u32 = 0x825E9488;
    'dispatch: loop {
        match pc {
            0x825E9488 => {
    //   block [0x825E9488..0x825E9494)
	// 825E9488: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E948C: 386BC890  addi r3, r11, -0x3770
	ctx.r[3].s64 = ctx.r[11].s64 + -14192;
	// 825E9490: 4BF496A8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9498 size=52
    let mut pc: u32 = 0x825E9498;
    'dispatch: loop {
        match pc {
            0x825E9498 => {
    //   block [0x825E9498..0x825E94CC)
	// 825E9498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E94A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E94A4: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E94A8: 386BF500  addi r3, r11, -0xb00
	ctx.r[3].s64 = ctx.r[11].s64 + -2816;
	// 825E94AC: 4BDB6FCD  bl 0x823a0478
	ctx.lr = 0x825E94B0;
	sub_823A0478(ctx, base);
	// 825E94B0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E94B4: 386BC8C8  addi r3, r11, -0x3738
	ctx.r[3].s64 = ctx.r[11].s64 + -14136;
	// 825E94B8: 4BF49681  bl 0x82532b38
	ctx.lr = 0x825E94BC;
	sub_82532B38(ctx, base);
	// 825E94BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E94C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E94C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E94C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E94D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E94D0 size=116
    let mut pc: u32 = 0x825E94D0;
    'dispatch: loop {
        match pc {
            0x825E94D0 => {
    //   block [0x825E94D0..0x825E94F4)
	// 825E94D0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E94D4: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E94D8: 390A053C  addi r8, r10, 0x53c
	ctx.r[8].s64 = ctx.r[10].s64 + 1340;
	// 825E94DC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E94E0: 396BD280  addi r11, r11, -0x2d80
	ctx.r[11].s64 = ctx.r[11].s64 + -11648;
	// 825E94E4: 38EA053C  addi r7, r10, 0x53c
	ctx.r[7].s64 = ctx.r[10].s64 + 1340;
	// 825E94E8: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 825E94EC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825E94F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x825E94F4; continue 'dispatch;
            }
            0x825E94F4 => {
    //   block [0x825E94F4..0x825E94FC)
	// 825E94F4: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 825E94F8: 910BFFF8  stw r8, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	pc = 0x825E94FC; continue 'dispatch;
            }
            0x825E94FC => {
    //   block [0x825E94FC..0x825E9544)
	// 825E94FC: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 825E9500: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825E9504: 7CC02028  lwarx r6, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 825E9508: 7D40212D  stwcx. r10, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825E950C: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825E9510: 4082FFEC  bne 0x825e94fc
	if !ctx.cr[0].eq {
	pc = 0x825E94FC; continue 'dispatch;
	}
	// 825E9514: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825E9518: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825E951C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825E9520: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825E9524: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825E9528: 90EBFFF8  stw r7, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 825E952C: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 825E9530: 396B0218  addi r11, r11, 0x218
	ctx.r[11].s64 = ctx.r[11].s64 + 536;
	// 825E9534: 4080FFC0  bge 0x825e94f4
	if !ctx.cr[0].lt {
	pc = 0x825E94F4; continue 'dispatch;
	}
	// 825E9538: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E953C: 386BC8E8  addi r3, r11, -0x3718
	ctx.r[3].s64 = ctx.r[11].s64 + -14104;
	// 825E9540: 4BF495F8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9548 size=12
    let mut pc: u32 = 0x825E9548;
    'dispatch: loop {
        match pc {
            0x825E9548 => {
    //   block [0x825E9548..0x825E9554)
	// 825E9548: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E954C: 386BC918  addi r3, r11, -0x36e8
	ctx.r[3].s64 = ctx.r[11].s64 + -14056;
	// 825E9550: 4BF495E8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9558 size=12
    let mut pc: u32 = 0x825E9558;
    'dispatch: loop {
        match pc {
            0x825E9558 => {
    //   block [0x825E9558..0x825E9564)
	// 825E9558: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E955C: 386BC978  addi r3, r11, -0x3688
	ctx.r[3].s64 = ctx.r[11].s64 + -13960;
	// 825E9560: 4BF495D8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9568 size=12
    let mut pc: u32 = 0x825E9568;
    'dispatch: loop {
        match pc {
            0x825E9568 => {
    //   block [0x825E9568..0x825E9574)
	// 825E9568: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E956C: 386BC998  addi r3, r11, -0x3668
	ctx.r[3].s64 = ctx.r[11].s64 + -13928;
	// 825E9570: 4BF495C8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9578 size=12
    let mut pc: u32 = 0x825E9578;
    'dispatch: loop {
        match pc {
            0x825E9578 => {
    //   block [0x825E9578..0x825E9584)
	// 825E9578: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E957C: 386BC9A0  addi r3, r11, -0x3660
	ctx.r[3].s64 = ctx.r[11].s64 + -13920;
	// 825E9580: 4BF495B8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9588 size=12
    let mut pc: u32 = 0x825E9588;
    'dispatch: loop {
        match pc {
            0x825E9588 => {
    //   block [0x825E9588..0x825E9594)
	// 825E9588: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E958C: 386BC9C0  addi r3, r11, -0x3640
	ctx.r[3].s64 = ctx.r[11].s64 + -13888;
	// 825E9590: 4BF495A8  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9598 size=12
    let mut pc: u32 = 0x825E9598;
    'dispatch: loop {
        match pc {
            0x825E9598 => {
    //   block [0x825E9598..0x825E95A4)
	// 825E9598: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E959C: 386BC9E0  addi r3, r11, -0x3620
	ctx.r[3].s64 = ctx.r[11].s64 + -13856;
	// 825E95A0: 4BF49598  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E95A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E95A8 size=12
    let mut pc: u32 = 0x825E95A8;
    'dispatch: loop {
        match pc {
            0x825E95A8 => {
    //   block [0x825E95A8..0x825E95B4)
	// 825E95A8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E95AC: 386BCA00  addi r3, r11, -0x3600
	ctx.r[3].s64 = ctx.r[11].s64 + -13824;
	// 825E95B0: 4BF49588  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E95B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E95B8 size=12
    let mut pc: u32 = 0x825E95B8;
    'dispatch: loop {
        match pc {
            0x825E95B8 => {
    //   block [0x825E95B8..0x825E95C4)
	// 825E95B8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E95BC: 386BCA20  addi r3, r11, -0x35e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13792;
	// 825E95C0: 4BF49578  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E95C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E95C8 size=12
    let mut pc: u32 = 0x825E95C8;
    'dispatch: loop {
        match pc {
            0x825E95C8 => {
    //   block [0x825E95C8..0x825E95D4)
	// 825E95C8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E95CC: 386BCA40  addi r3, r11, -0x35c0
	ctx.r[3].s64 = ctx.r[11].s64 + -13760;
	// 825E95D0: 4BF49568  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E95D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E95D8 size=12
    let mut pc: u32 = 0x825E95D8;
    'dispatch: loop {
        match pc {
            0x825E95D8 => {
    //   block [0x825E95D8..0x825E95E4)
	// 825E95D8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E95DC: 386BCA48  addi r3, r11, -0x35b8
	ctx.r[3].s64 = ctx.r[11].s64 + -13752;
	// 825E95E0: 4BF49558  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E95E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E95E8 size=52
    let mut pc: u32 = 0x825E95E8;
    'dispatch: loop {
        match pc {
            0x825E95E8 => {
    //   block [0x825E95E8..0x825E961C)
	// 825E95E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E95EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E95F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E95F4: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E95F8: 386B0700  addi r3, r11, 0x700
	ctx.r[3].s64 = ctx.r[11].s64 + 1792;
	// 825E95FC: 4BDBA7AD  bl 0x823a3da8
	ctx.lr = 0x825E9600;
	sub_823A3DA8(ctx, base);
	// 825E9600: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9604: 386BCA58  addi r3, r11, -0x35a8
	ctx.r[3].s64 = ctx.r[11].s64 + -13736;
	// 825E9608: 4BF49531  bl 0x82532b38
	ctx.lr = 0x825E960C;
	sub_82532B38(ctx, base);
	// 825E960C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9620 size=12
    let mut pc: u32 = 0x825E9620;
    'dispatch: loop {
        match pc {
            0x825E9620 => {
    //   block [0x825E9620..0x825E962C)
	// 825E9620: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9624: 386BCAB8  addi r3, r11, -0x3548
	ctx.r[3].s64 = ctx.r[11].s64 + -13640;
	// 825E9628: 4BF49510  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9630 size=12
    let mut pc: u32 = 0x825E9630;
    'dispatch: loop {
        match pc {
            0x825E9630 => {
    //   block [0x825E9630..0x825E963C)
	// 825E9630: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9634: 386BCAC0  addi r3, r11, -0x3540
	ctx.r[3].s64 = ctx.r[11].s64 + -13632;
	// 825E9638: 4BF49500  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9640 size=12
    let mut pc: u32 = 0x825E9640;
    'dispatch: loop {
        match pc {
            0x825E9640 => {
    //   block [0x825E9640..0x825E964C)
	// 825E9640: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9644: 386BCAC8  addi r3, r11, -0x3538
	ctx.r[3].s64 = ctx.r[11].s64 + -13624;
	// 825E9648: 4BF494F0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9650 size=12
    let mut pc: u32 = 0x825E9650;
    'dispatch: loop {
        match pc {
            0x825E9650 => {
    //   block [0x825E9650..0x825E965C)
	// 825E9650: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9654: 386BCAE0  addi r3, r11, -0x3520
	ctx.r[3].s64 = ctx.r[11].s64 + -13600;
	// 825E9658: 4BF494E0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9660 size=12
    let mut pc: u32 = 0x825E9660;
    'dispatch: loop {
        match pc {
            0x825E9660 => {
    //   block [0x825E9660..0x825E966C)
	// 825E9660: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9664: 386BCAE8  addi r3, r11, -0x3518
	ctx.r[3].s64 = ctx.r[11].s64 + -13592;
	// 825E9668: 4BF494D0  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9670 size=52
    let mut pc: u32 = 0x825E9670;
    'dispatch: loop {
        match pc {
            0x825E9670 => {
    //   block [0x825E9670..0x825E96A4)
	// 825E9670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E967C: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E9680: 386B1100  addi r3, r11, 0x1100
	ctx.r[3].s64 = ctx.r[11].s64 + 4352;
	// 825E9684: 4BDBE8AD  bl 0x823a7f30
	ctx.lr = 0x825E9688;
	sub_823A7F30(ctx, base);
	// 825E9688: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E968C: 386BCAF0  addi r3, r11, -0x3510
	ctx.r[3].s64 = ctx.r[11].s64 + -13584;
	// 825E9690: 4BF494A9  bl 0x82532b38
	ctx.lr = 0x825E9694;
	sub_82532B38(ctx, base);
	// 825E9694: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E969C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E96A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E96A8 size=12
    let mut pc: u32 = 0x825E96A8;
    'dispatch: loop {
        match pc {
            0x825E96A8 => {
    //   block [0x825E96A8..0x825E96B4)
	// 825E96A8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E96AC: 386BCB00  addi r3, r11, -0x3500
	ctx.r[3].s64 = ctx.r[11].s64 + -13568;
	// 825E96B0: 4BF49488  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E96B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E96B8 size=140
    let mut pc: u32 = 0x825E96B8;
    'dispatch: loop {
        match pc {
            0x825E96B8 => {
    //   block [0x825E96B8..0x825E96D4)
	// 825E96B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E96BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E96C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E96C4: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E96C8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 825E96CC: 38CB9B00  addi r6, r11, -0x6500
	ctx.r[6].s64 = ctx.r[11].s64 + -25856;
	// 825E96D0: 3866003C  addi r3, r6, 0x3c
	ctx.r[3].s64 = ctx.r[6].s64 + 60;
	pc = 0x825E96D4; continue 'dispatch;
            }
            0x825E96D4 => {
    //   block [0x825E96D4..0x825E970C)
	// 825E96D4: 4BDC6945  bl 0x823b0018
	ctx.lr = 0x825E96D8;
	sub_823B0018(ctx, base);
	// 825E96D8: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825E96DC: 3863018C  addi r3, r3, 0x18c
	ctx.r[3].s64 = ctx.r[3].s64 + 396;
	// 825E96E0: 4080FFF4  bge 0x825e96d4
	if !ctx.cr[0].lt {
	pc = 0x825E96D4; continue 'dispatch;
	}
	// 825E96E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825E96E8: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 825E96EC: 396B1784  addi r11, r11, 0x1784
	ctx.r[11].s64 = ctx.r[11].s64 + 6020;
	// 825E96F0: 9166066C  stw r11, 0x66c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(1644 as u32), ctx.r[11].u32 ) };
	// 825E96F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E96F8: 916606B4  stw r11, 0x6b4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(1716 as u32), ctx.r[11].u32 ) };
	// 825E96FC: 916606B8  stw r11, 0x6b8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(1720 as u32), ctx.r[11].u32 ) };
	// 825E9700: 916606BC  stw r11, 0x6bc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(1724 as u32), ctx.r[11].u32 ) };
	// 825E9704: 916606C0  stw r11, 0x6c0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(1728 as u32), ctx.r[11].u32 ) };
	// 825E9708: 39660028  addi r11, r6, 0x28
	ctx.r[11].s64 = ctx.r[6].s64 + 40;
	pc = 0x825E970C; continue 'dispatch;
            }
            0x825E970C => {
    //   block [0x825E970C..0x825E9744)
	// 825E970C: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 825E9710: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825E9714: 912B069C  stw r9, 0x69c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1692 as u32), ctx.r[9].u32 ) };
	// 825E9718: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E971C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825E9720: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825E9724: 4082FFE8  bne 0x825e970c
	if !ctx.cr[0].eq {
	pc = 0x825E970C; continue 'dispatch;
	}
	// 825E9728: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E972C: 386BCB08  addi r3, r11, -0x34f8
	ctx.r[3].s64 = ctx.r[11].s64 + -13560;
	// 825E9730: 4BF49409  bl 0x82532b38
	ctx.lr = 0x825E9734;
	sub_82532B38(ctx, base);
	// 825E9734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E973C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9748 size=108
    let mut pc: u32 = 0x825E9748;
    'dispatch: loop {
        match pc {
            0x825E9748 => {
    //   block [0x825E9748..0x825E97B4)
	// 825E9748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E974C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E9754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9758: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E975C: 38A00D80  li r5, 0xd80
	ctx.r[5].s64 = 3456;
	// 825E9760: 3BEB2200  addi r31, r11, 0x2200
	ctx.r[31].s64 = ctx.r[11].s64 + 8704;
	// 825E9764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E9768: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825E976C: 4BF4BA65  bl 0x825351d0
	ctx.lr = 0x825E9770;
	sub_825351D0(ctx, base);
	// 825E9770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9774: 38A0006C  li r5, 0x6c
	ctx.r[5].s64 = 108;
	// 825E9778: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E977C: 387F0DB4  addi r3, r31, 0xdb4
	ctx.r[3].s64 = ctx.r[31].s64 + 3508;
	// 825E9780: 917F0DB0  stw r11, 0xdb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3504 as u32), ctx.r[11].u32 ) };
	// 825E9784: 4BF4BA4D  bl 0x825351d0
	ctx.lr = 0x825E9788;
	sub_825351D0(ctx, base);
	// 825E9788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E978C: 3D408271  lis r10, -0x7d8f
	ctx.r[10].s64 = -2106523648;
	// 825E9790: 386ACB48  addi r3, r10, -0x34b8
	ctx.r[3].s64 = ctx.r[10].s64 + -13496;
	// 825E9794: 917F0E20  stw r11, 0xe20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3616 as u32), ctx.r[11].u32 ) };
	// 825E9798: 917F0E24  stw r11, 0xe24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3620 as u32), ctx.r[11].u32 ) };
	// 825E979C: 4BF4939D  bl 0x82532b38
	ctx.lr = 0x825E97A0;
	sub_82532B38(ctx, base);
	// 825E97A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E97A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E97A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E97AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E97B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E97B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E97B8 size=112
    let mut pc: u32 = 0x825E97B8;
    'dispatch: loop {
        match pc {
            0x825E97B8 => {
    //   block [0x825E97B8..0x825E9828)
	// 825E97B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E97BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E97C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825E97C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E97C8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 825E97CC: 38A00D80  li r5, 0xd80
	ctx.r[5].s64 = 3456;
	// 825E97D0: 3BEB1380  addi r31, r11, 0x1380
	ctx.r[31].s64 = ctx.r[11].s64 + 4992;
	// 825E97D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E97D8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825E97DC: 4BF4B9F5  bl 0x825351d0
	ctx.lr = 0x825E97E0;
	sub_825351D0(ctx, base);
	// 825E97E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E97E4: 38A0006C  li r5, 0x6c
	ctx.r[5].s64 = 108;
	// 825E97E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E97EC: 387F0DC0  addi r3, r31, 0xdc0
	ctx.r[3].s64 = ctx.r[31].s64 + 3520;
	// 825E97F0: 917F0DBC  stw r11, 0xdbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3516 as u32), ctx.r[11].u32 ) };
	// 825E97F4: 4BF4B9DD  bl 0x825351d0
	ctx.lr = 0x825E97F8;
	sub_825351D0(ctx, base);
	// 825E97F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E97FC: 3D408271  lis r10, -0x7d8f
	ctx.r[10].s64 = -2106523648;
	// 825E9800: 386ACB68  addi r3, r10, -0x3498
	ctx.r[3].s64 = ctx.r[10].s64 + -13464;
	// 825E9804: 917F0E2C  stw r11, 0xe2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3628 as u32), ctx.r[11].u32 ) };
	// 825E9808: 917F0E30  stw r11, 0xe30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3632 as u32), ctx.r[11].u32 ) };
	// 825E980C: 917F0E40  stw r11, 0xe40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3648 as u32), ctx.r[11].u32 ) };
	// 825E9810: 4BF49329  bl 0x82532b38
	ctx.lr = 0x825E9814;
	sub_82532B38(ctx, base);
	// 825E9814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E981C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825E9824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9828 size=12
    let mut pc: u32 = 0x825E9828;
    'dispatch: loop {
        match pc {
            0x825E9828 => {
    //   block [0x825E9828..0x825E9834)
	// 825E9828: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E982C: 386BCBF0  addi r3, r11, -0x3410
	ctx.r[3].s64 = ctx.r[11].s64 + -13328;
	// 825E9830: 4BF49308  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9838 size=52
    let mut pc: u32 = 0x825E9838;
    'dispatch: loop {
        match pc {
            0x825E9838 => {
    //   block [0x825E9838..0x825E986C)
	// 825E9838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9844: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E9848: 386BDC90  addi r3, r11, -0x2370
	ctx.r[3].s64 = ctx.r[11].s64 + -9072;
	// 825E984C: 4BDE1FAD  bl 0x823cb7f8
	ctx.lr = 0x825E9850;
	sub_823CB7F8(ctx, base);
	// 825E9850: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9854: 386BCC08  addi r3, r11, -0x33f8
	ctx.r[3].s64 = ctx.r[11].s64 + -13304;
	// 825E9858: 4BF492E1  bl 0x82532b38
	ctx.lr = 0x825E985C;
	sub_82532B38(ctx, base);
	// 825E985C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E9860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9870 size=20
    let mut pc: u32 = 0x825E9870;
    'dispatch: loop {
        match pc {
            0x825E9870 => {
    //   block [0x825E9870..0x825E9884)
	// 825E9870: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E9874: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 825E9878: 386B37C0  addi r3, r11, 0x37c0
	ctx.r[3].s64 = ctx.r[11].s64 + 14272;
	// 825E987C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825E9880: 4BF4B950  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9888 size=52
    let mut pc: u32 = 0x825E9888;
    'dispatch: loop {
        match pc {
            0x825E9888 => {
    //   block [0x825E9888..0x825E98BC)
	// 825E9888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9894: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E9898: 386B3838  addi r3, r11, 0x3838
	ctx.r[3].s64 = ctx.r[11].s64 + 14392;
	// 825E989C: 4BE23BA5  bl 0x8240d440
	ctx.lr = 0x825E98A0;
	sub_8240D440(ctx, base);
	// 825E98A0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E98A4: 386BCC48  addi r3, r11, -0x33b8
	ctx.r[3].s64 = ctx.r[11].s64 + -13240;
	// 825E98A8: 4BF49291  bl 0x82532b38
	ctx.lr = 0x825E98AC;
	sub_82532B38(ctx, base);
	// 825E98AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825E98B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E98B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E98B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E98C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E98C0 size=12
    let mut pc: u32 = 0x825E98C0;
    'dispatch: loop {
        match pc {
            0x825E98C0 => {
    //   block [0x825E98C0..0x825E98CC)
	// 825E98C0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E98C4: 386BCC50  addi r3, r11, -0x33b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13232;
	// 825E98C8: 4BF49270  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825E98D0 size=176
    let mut pc: u32 = 0x825E98D0;
    'dispatch: loop {
        match pc {
            0x825E98D0 => {
    //   block [0x825E98D0..0x825E990C)
	// 825E98D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E98D4: 4BF4B7E5  bl 0x825350b8
	ctx.lr = 0x825E98D8;
	sub_82535080(ctx, base);
	// 825E98D8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 825E98DC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825E98E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E98E4: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E98E8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E98EC: 396B5870  addi r11, r11, 0x5870
	ctx.r[11].s64 = ctx.r[11].s64 + 22640;
	// 825E98F0: 3B800007  li r28, 7
	ctx.r[28].s64 = 7;
	// 825E98F4: 3BEB00F4  addi r31, r11, 0xf4
	ctx.r[31].s64 = ctx.r[11].s64 + 244;
	// 825E98F8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825E98FC: C3EA1FF8  lfs f31, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825E9900: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825E9904: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 825E9908: C3CB2268  lfs f30, 0x2268(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8808 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	pc = 0x825E990C; continue 'dispatch;
            }
            0x825E990C => {
    //   block [0x825E990C..0x825E9980)
	// 825E990C: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 825E9910: 93DFFF98  stw r30, -0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-104 as u32), ctx.r[30].u32 ) };
	// 825E9914: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825E9918: 9BDF0004  stb r30, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u8 ) };
	// 825E991C: 9BDF0005  stb r30, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[30].u8 ) };
	// 825E9920: 9BDF0006  stb r30, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u8 ) };
	// 825E9924: 9BDF0007  stb r30, 7(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7 as u32), ctx.r[30].u8 ) };
	// 825E9928: 93BF0040  stw r29, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 825E992C: 4BE1DE6D  bl 0x82407798
	ctx.lr = 0x825E9930;
	sub_82407798(ctx, base);
	// 825E9930: D3DF0118  stfs f30, 0x118(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 825E9934: 9BDF011C  stb r30, 0x11c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[30].u8 ) };
	// 825E9938: D3FF1128  stfs f31, 0x1128(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4392 as u32), tmp.u32 ) };
	// 825E993C: 9BDF011D  stb r30, 0x11d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(285 as u32), ctx.r[30].u8 ) };
	// 825E9940: D3FF112C  stfs f31, 0x112c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4396 as u32), tmp.u32 ) };
	// 825E9944: 93BF0120  stw r29, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[29].u32 ) };
	// 825E9948: 93DF0124  stw r30, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[30].u32 ) };
	// 825E994C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825E9950: 93BF1130  stw r29, 0x1130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4400 as u32), ctx.r[29].u32 ) };
	// 825E9954: 93DF1134  stw r30, 0x1134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4404 as u32), ctx.r[30].u32 ) };
	// 825E9958: 93DF1138  stw r30, 0x1138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4408 as u32), ctx.r[30].u32 ) };
	// 825E995C: 3BFF132C  addi r31, r31, 0x132c
	ctx.r[31].s64 = ctx.r[31].s64 + 4908;
	// 825E9960: 4080FFAC  bge 0x825e990c
	if !ctx.cr[0].lt {
	pc = 0x825E990C; continue 'dispatch;
	}
	// 825E9964: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9968: 386BCC58  addi r3, r11, -0x33a8
	ctx.r[3].s64 = ctx.r[11].s64 + -13224;
	// 825E996C: 4BF491CD  bl 0x82532b38
	ctx.lr = 0x825E9970;
	sub_82532B38(ctx, base);
	// 825E9970: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825E9974: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 825E9978: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 825E997C: 4BF4B78C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9980 size=108
    let mut pc: u32 = 0x825E9980;
    'dispatch: loop {
        match pc {
            0x825E9980 => {
    //   block [0x825E9980..0x825E99EC)
	// 825E9980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E998C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9990: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9994: 38EB6E00  addi r7, r11, 0x6e00
	ctx.r[7].s64 = ctx.r[11].s64 + 28160;
	// 825E9998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E999C: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 825E99A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E99A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E99A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E99AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E99B0: 386A8D64  addi r3, r10, -0x729c
	ctx.r[3].s64 = ctx.r[10].s64 + -29340;
	// 825E99B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E99B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E99BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E99C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E99C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E99C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E99CC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825E99D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E99D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E99D8: 4BE7D449  bl 0x82466e20
	ctx.lr = 0x825E99DC;
	sub_82466E20(ctx, base);
	// 825E99DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E99E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E99E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E99E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E99F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E99F0 size=108
    let mut pc: u32 = 0x825E99F0;
    'dispatch: loop {
        match pc {
            0x825E99F0 => {
    //   block [0x825E99F0..0x825E9A5C)
	// 825E99F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E99F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E99F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E99FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9A00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9A04: 38EB6E30  addi r7, r11, 0x6e30
	ctx.r[7].s64 = ctx.r[11].s64 + 28208;
	// 825E9A08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E9A0C: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 825E9A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9A14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9A18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9A20: 386A8D94  addi r3, r10, -0x726c
	ctx.r[3].s64 = ctx.r[10].s64 + -29292;
	// 825E9A24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9A3C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825E9A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9A44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9A48: 4BE7D3D9  bl 0x82466e20
	ctx.lr = 0x825E9A4C;
	sub_82466E20(ctx, base);
	// 825E9A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9A60 size=40
    let mut pc: u32 = 0x825E9A60;
    'dispatch: loop {
        match pc {
            0x825E9A60 => {
    //   block [0x825E9A60..0x825E9A88)
	// 825E9A60: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9A64: 814B4558  lwz r10, 0x4558(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17752 as u32) ) } as u64;
	// 825E9A68: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9A6C: 396B4578  addi r11, r11, 0x4578
	ctx.r[11].s64 = ctx.r[11].s64 + 17784;
	// 825E9A70: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825E9A74: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825E9A78: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9A7C: 814A455C  lwz r10, 0x455c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(17756 as u32) ) } as u64;
	// 825E9A80: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 825E9A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9A88 size=112
    let mut pc: u32 = 0x825E9A88;
    'dispatch: loop {
        match pc {
            0x825E9A88 => {
    //   block [0x825E9A88..0x825E9AF8)
	// 825E9A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9A94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9A98: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9A9C: 392A72B8  addi r9, r10, 0x72b8
	ctx.r[9].s64 = ctx.r[10].s64 + 29368;
	// 825E9AA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9AA4: 390B4578  addi r8, r11, 0x4578
	ctx.r[8].s64 = ctx.r[11].s64 + 17784;
	// 825E9AA8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825E9AAC: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 825E9AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9AB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9ABC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 825E9AC0: 386A8DC4  addi r3, r10, -0x723c
	ctx.r[3].s64 = ctx.r[10].s64 + -29244;
	// 825E9AC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9AC8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825E9ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9AE4: 4BE7D33D  bl 0x82466e20
	ctx.lr = 0x825E9AE8;
	sub_82466E20(ctx, base);
	// 825E9AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9AF8 size=112
    let mut pc: u32 = 0x825E9AF8;
    'dispatch: loop {
        match pc {
            0x825E9AF8 => {
    //   block [0x825E9AF8..0x825E9B68)
	// 825E9AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9B04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9B08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9B0C: 38AA9130  addi r5, r10, -0x6ed0
	ctx.r[5].s64 = ctx.r[10].s64 + -28368;
	// 825E9B10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9B14: 390B7340  addi r8, r11, 0x7340
	ctx.r[8].s64 = ctx.r[11].s64 + 29504;
	// 825E9B18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825E9B1C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 825E9B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9B24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9B30: 386A8DF4  addi r3, r10, -0x720c
	ctx.r[3].s64 = ctx.r[10].s64 + -29196;
	// 825E9B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825E9B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9B4C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825E9B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9B54: 4BE7D2CD  bl 0x82466e20
	ctx.lr = 0x825E9B58;
	sub_82466E20(ctx, base);
	// 825E9B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9B68 size=24
    let mut pc: u32 = 0x825E9B68;
    'dispatch: loop {
        match pc {
            0x825E9B68 => {
    //   block [0x825E9B68..0x825E9B80)
	// 825E9B68: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9B6C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9B70: 394A46A0  addi r10, r10, 0x46a0
	ctx.r[10].s64 = ctx.r[10].s64 + 18080;
	// 825E9B74: 816B468C  lwz r11, 0x468c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18060 as u32) ) } as u64;
	// 825E9B78: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825E9B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9B80 size=112
    let mut pc: u32 = 0x825E9B80;
    'dispatch: loop {
        match pc {
            0x825E9B80 => {
    //   block [0x825E9B80..0x825E9BF0)
	// 825E9B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9B8C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9B90: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9B94: 392A7440  addi r9, r10, 0x7440
	ctx.r[9].s64 = ctx.r[10].s64 + 29760;
	// 825E9B98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9B9C: 390B46A0  addi r8, r11, 0x46a0
	ctx.r[8].s64 = ctx.r[11].s64 + 18080;
	// 825E9BA0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825E9BA4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 825E9BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9BB4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825E9BB8: 386A8E24  addi r3, r10, -0x71dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29148;
	// 825E9BBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9BC0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825E9BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9BDC: 4BE7D245  bl 0x82466e20
	ctx.lr = 0x825E9BE0;
	sub_82466E20(ctx, base);
	// 825E9BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9BF0 size=108
    let mut pc: u32 = 0x825E9BF0;
    'dispatch: loop {
        match pc {
            0x825E9BF0 => {
    //   block [0x825E9BF0..0x825E9C5C)
	// 825E9BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9C00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9C04: 38EB7714  addi r7, r11, 0x7714
	ctx.r[7].s64 = ctx.r[11].s64 + 30484;
	// 825E9C08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E9C0C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 825E9C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9C14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9C20: 386A8E54  addi r3, r10, -0x71ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29100;
	// 825E9C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9C3C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9C48: 4BE7D1D9  bl 0x82466e20
	ctx.lr = 0x825E9C4C;
	sub_82466E20(ctx, base);
	// 825E9C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9C60 size=108
    let mut pc: u32 = 0x825E9C60;
    'dispatch: loop {
        match pc {
            0x825E9C60 => {
    //   block [0x825E9C60..0x825E9CCC)
	// 825E9C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9C6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9C70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9C74: 38EB7744  addi r7, r11, 0x7744
	ctx.r[7].s64 = ctx.r[11].s64 + 30532;
	// 825E9C78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E9C7C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 825E9C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9C90: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 825E9C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9CAC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825E9CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9CB8: 4BE7D169  bl 0x82466e20
	ctx.lr = 0x825E9CBC;
	sub_82466E20(ctx, base);
	// 825E9CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9CD0 size=24
    let mut pc: u32 = 0x825E9CD0;
    'dispatch: loop {
        match pc {
            0x825E9CD0 => {
    //   block [0x825E9CD0..0x825E9CE8)
	// 825E9CD0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9CD4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9CD8: 394A4800  addi r10, r10, 0x4800
	ctx.r[10].s64 = ctx.r[10].s64 + 18432;
	// 825E9CDC: 816B47C8  lwz r11, 0x47c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18376 as u32) ) } as u64;
	// 825E9CE0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825E9CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9CE8 size=112
    let mut pc: u32 = 0x825E9CE8;
    'dispatch: loop {
        match pc {
            0x825E9CE8 => {
    //   block [0x825E9CE8..0x825E9D58)
	// 825E9CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9CF4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9CF8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9CFC: 392A7774  addi r9, r10, 0x7774
	ctx.r[9].s64 = ctx.r[10].s64 + 30580;
	// 825E9D00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9D04: 390B4800  addi r8, r11, 0x4800
	ctx.r[8].s64 = ctx.r[11].s64 + 18432;
	// 825E9D08: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825E9D0C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 825E9D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9D14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9D1C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 825E9D20: 386A8EB4  addi r3, r10, -0x714c
	ctx.r[3].s64 = ctx.r[10].s64 + -29004;
	// 825E9D24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9D28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E9D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9D44: 4BE7D0DD  bl 0x82466e20
	ctx.lr = 0x825E9D48;
	sub_82466E20(ctx, base);
	// 825E9D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9D58 size=108
    let mut pc: u32 = 0x825E9D58;
    'dispatch: loop {
        match pc {
            0x825E9D58 => {
    //   block [0x825E9D58..0x825E9DC4)
	// 825E9D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9D68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9D6C: 38EB7858  addi r7, r11, 0x7858
	ctx.r[7].s64 = ctx.r[11].s64 + 30808;
	// 825E9D70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E9D74: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 825E9D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9D7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9D88: 386A8EE4  addi r3, r10, -0x711c
	ctx.r[3].s64 = ctx.r[10].s64 + -28956;
	// 825E9D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9DA4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825E9DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9DB0: 4BE7D071  bl 0x82466e20
	ctx.lr = 0x825E9DB4;
	sub_82466E20(ctx, base);
	// 825E9DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9DC8 size=24
    let mut pc: u32 = 0x825E9DC8;
    'dispatch: loop {
        match pc {
            0x825E9DC8 => {
    //   block [0x825E9DC8..0x825E9DE0)
	// 825E9DC8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9DCC: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9DD0: 394A4900  addi r10, r10, 0x4900
	ctx.r[10].s64 = ctx.r[10].s64 + 18688;
	// 825E9DD4: 816B48D8  lwz r11, 0x48d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18648 as u32) ) } as u64;
	// 825E9DD8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825E9DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9DE0 size=112
    let mut pc: u32 = 0x825E9DE0;
    'dispatch: loop {
        match pc {
            0x825E9DE0 => {
    //   block [0x825E9DE0..0x825E9E50)
	// 825E9DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9DEC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9DF0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9DF4: 392A7844  addi r9, r10, 0x7844
	ctx.r[9].s64 = ctx.r[10].s64 + 30788;
	// 825E9DF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9DFC: 390B4900  addi r8, r11, 0x4900
	ctx.r[8].s64 = ctx.r[11].s64 + 18688;
	// 825E9E00: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825E9E04: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 825E9E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9E0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9E14: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 825E9E18: 386A8F14  addi r3, r10, -0x70ec
	ctx.r[3].s64 = ctx.r[10].s64 + -28908;
	// 825E9E1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9E20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E9E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9E3C: 4BE7CFE5  bl 0x82466e20
	ctx.lr = 0x825E9E40;
	sub_82466E20(ctx, base);
	// 825E9E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9E50 size=108
    let mut pc: u32 = 0x825E9E50;
    'dispatch: loop {
        match pc {
            0x825E9E50 => {
    //   block [0x825E9E50..0x825E9EBC)
	// 825E9E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9E5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9E60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9E64: 38EB78A8  addi r7, r11, 0x78a8
	ctx.r[7].s64 = ctx.r[11].s64 + 30888;
	// 825E9E68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E9E6C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 825E9E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9E74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9E78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9E80: 386A8F44  addi r3, r10, -0x70bc
	ctx.r[3].s64 = ctx.r[10].s64 + -28860;
	// 825E9E84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9E9C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9EA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9EA8: 4BE7CF79  bl 0x82466e20
	ctx.lr = 0x825E9EAC;
	sub_82466E20(ctx, base);
	// 825E9EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9EC0 size=108
    let mut pc: u32 = 0x825E9EC0;
    'dispatch: loop {
        match pc {
            0x825E9EC0 => {
    //   block [0x825E9EC0..0x825E9F2C)
	// 825E9EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9ECC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9ED0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9ED4: 38EB78E4  addi r7, r11, 0x78e4
	ctx.r[7].s64 = ctx.r[11].s64 + 30948;
	// 825E9ED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E9EDC: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 825E9EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9EE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9EF0: 386A8F74  addi r3, r10, -0x708c
	ctx.r[3].s64 = ctx.r[10].s64 + -28812;
	// 825E9EF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9F0C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825E9F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9F18: 4BE7CF09  bl 0x82466e20
	ctx.lr = 0x825E9F1C;
	sub_82466E20(ctx, base);
	// 825E9F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9F30 size=108
    let mut pc: u32 = 0x825E9F30;
    'dispatch: loop {
        match pc {
            0x825E9F30 => {
    //   block [0x825E9F30..0x825E9F9C)
	// 825E9F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9F40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9F44: 38EB7940  addi r7, r11, 0x7940
	ctx.r[7].s64 = ctx.r[11].s64 + 31040;
	// 825E9F48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825E9F4C: 388A7A00  addi r4, r10, 0x7a00
	ctx.r[4].s64 = ctx.r[10].s64 + 31232;
	// 825E9F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9F54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9F58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9F60: 386A8FA4  addi r3, r10, -0x705c
	ctx.r[3].s64 = ctx.r[10].s64 + -28764;
	// 825E9F64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9F7C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9F88: 4BE7CE99  bl 0x82466e20
	ctx.lr = 0x825E9F8C;
	sub_82466E20(ctx, base);
	// 825E9F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9FA0 size=108
    let mut pc: u32 = 0x825E9FA0;
    'dispatch: loop {
        match pc {
            0x825E9FA0 => {
    //   block [0x825E9FA0..0x825EA00C)
	// 825E9FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9FAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9FB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9FB4: 38EB79A0  addi r7, r11, 0x79a0
	ctx.r[7].s64 = ctx.r[11].s64 + 31136;
	// 825E9FB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825E9FBC: 388A7A18  addi r4, r10, 0x7a18
	ctx.r[4].s64 = ctx.r[10].s64 + 31256;
	// 825E9FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9FC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9FD0: 386A8FD4  addi r3, r10, -0x702c
	ctx.r[3].s64 = ctx.r[10].s64 + -28716;
	// 825E9FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9FEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9FF8: 4BE7CE29  bl 0x82466e20
	ctx.lr = 0x825E9FFC;
	sub_82466E20(ctx, base);
	// 825E9FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA010 size=108
    let mut pc: u32 = 0x825EA010;
    'dispatch: loop {
        match pc {
            0x825EA010 => {
    //   block [0x825EA010..0x825EA07C)
	// 825EA010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA01C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA020: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA024: 38EB7AF0  addi r7, r11, 0x7af0
	ctx.r[7].s64 = ctx.r[11].s64 + 31472;
	// 825EA028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA02C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 825EA030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA034: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA040: 386A9008  addi r3, r10, -0x6ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -28664;
	// 825EA044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA05C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825EA060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA068: 4BE7CDB9  bl 0x82466e20
	ctx.lr = 0x825EA06C;
	sub_82466E20(ctx, base);
	// 825EA06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA080 size=108
    let mut pc: u32 = 0x825EA080;
    'dispatch: loop {
        match pc {
            0x825EA080 => {
    //   block [0x825EA080..0x825EA0EC)
	// 825EA080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA08C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA090: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA094: 38EB7B20  addi r7, r11, 0x7b20
	ctx.r[7].s64 = ctx.r[11].s64 + 31520;
	// 825EA098: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EA09C: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 825EA0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA0A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA0A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA0B0: 386A9038  addi r3, r10, -0x6fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -28616;
	// 825EA0B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA0CC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825EA0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA0D8: 4BE7CD49  bl 0x82466e20
	ctx.lr = 0x825EA0DC;
	sub_82466E20(ctx, base);
	// 825EA0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA0F0 size=28
    let mut pc: u32 = 0x825EA0F0;
    'dispatch: loop {
        match pc {
            0x825EA0F0 => {
    //   block [0x825EA0F0..0x825EA10C)
	// 825EA0F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA0F4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA0F8: 396B4AF0  addi r11, r11, 0x4af0
	ctx.r[11].s64 = ctx.r[11].s64 + 19184;
	// 825EA0FC: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA100: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA104: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA110 size=108
    let mut pc: u32 = 0x825EA110;
    'dispatch: loop {
        match pc {
            0x825EA110 => {
    //   block [0x825EA110..0x825EA17C)
	// 825EA110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA11C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA120: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA124: 38EB7E50  addi r7, r11, 0x7e50
	ctx.r[7].s64 = ctx.r[11].s64 + 32336;
	// 825EA128: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825EA12C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 825EA130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA140: 386A9070  addi r3, r10, -0x6f90
	ctx.r[3].s64 = ctx.r[10].s64 + -28560;
	// 825EA144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA15C: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 825EA160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA168: 4BE7CCB9  bl 0x82466e20
	ctx.lr = 0x825EA16C;
	sub_82466E20(ctx, base);
	// 825EA16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA180 size=108
    let mut pc: u32 = 0x825EA180;
    'dispatch: loop {
        match pc {
            0x825EA180 => {
    //   block [0x825EA180..0x825EA1EC)
	// 825EA180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA18C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA190: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA194: 38EB7F58  addi r7, r11, 0x7f58
	ctx.r[7].s64 = ctx.r[11].s64 + 32600;
	// 825EA198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA19C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 825EA1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA1A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA1B0: 386A90A0  addi r3, r10, -0x6f60
	ctx.r[3].s64 = ctx.r[10].s64 + -28512;
	// 825EA1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA1CC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825EA1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA1D8: 4BE7CC49  bl 0x82466e20
	ctx.lr = 0x825EA1DC;
	sub_82466E20(ctx, base);
	// 825EA1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA1F0 size=108
    let mut pc: u32 = 0x825EA1F0;
    'dispatch: loop {
        match pc {
            0x825EA1F0 => {
    //   block [0x825EA1F0..0x825EA25C)
	// 825EA1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA204: 38EB7FE8  addi r7, r11, 0x7fe8
	ctx.r[7].s64 = ctx.r[11].s64 + 32744;
	// 825EA208: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EA20C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 825EA210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA214: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA220: 386A90D0  addi r3, r10, -0x6f30
	ctx.r[3].s64 = ctx.r[10].s64 + -28464;
	// 825EA224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA23C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EA240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA248: 4BE7CBD9  bl 0x82466e20
	ctx.lr = 0x825EA24C;
	sub_82466E20(ctx, base);
	// 825EA24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA260 size=112
    let mut pc: u32 = 0x825EA260;
    'dispatch: loop {
        match pc {
            0x825EA260 => {
    //   block [0x825EA260..0x825EA2D0)
	// 825EA260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA26C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA270: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA274: 392A8128  addi r9, r10, -0x7ed8
	ctx.r[9].s64 = ctx.r[10].s64 + -32472;
	// 825EA278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA27C: 390B8150  addi r8, r11, -0x7eb0
	ctx.r[8].s64 = ctx.r[11].s64 + -32432;
	// 825EA280: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EA284: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 825EA288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA28C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EA294: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA298: 386A9100  addi r3, r10, -0x6f00
	ctx.r[3].s64 = ctx.r[10].s64 + -28416;
	// 825EA29C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EA2A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825EA2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA2BC: 4BE7CB65  bl 0x82466e20
	ctx.lr = 0x825EA2C0;
	sub_82466E20(ctx, base);
	// 825EA2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA2D0 size=96
    let mut pc: u32 = 0x825EA2D0;
    'dispatch: loop {
        match pc {
            0x825EA2D0 => {
    //   block [0x825EA2D0..0x825EA330)
	// 825EA2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA2DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA2E4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 825EA2E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA2F0: 386A9130  addi r3, r10, -0x6ed0
	ctx.r[3].s64 = ctx.r[10].s64 + -28368;
	// 825EA2F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA2FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EA300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA30C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 825EA310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EA314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EA31C: 4BE7CB05  bl 0x82466e20
	ctx.lr = 0x825EA320;
	sub_82466E20(ctx, base);
	// 825EA320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA330 size=112
    let mut pc: u32 = 0x825EA330;
    'dispatch: loop {
        match pc {
            0x825EA330 => {
    //   block [0x825EA330..0x825EA3A0)
	// 825EA330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA33C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA340: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA344: 392A8258  addi r9, r10, -0x7da8
	ctx.r[9].s64 = ctx.r[10].s64 + -32168;
	// 825EA348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA34C: 390B8270  addi r8, r11, -0x7d90
	ctx.r[8].s64 = ctx.r[11].s64 + -32144;
	// 825EA350: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EA354: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 825EA358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA35C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EA364: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA368: 386A9160  addi r3, r10, -0x6ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -28320;
	// 825EA36C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EA370: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EA374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA38C: 4BE7CA95  bl 0x82466e20
	ctx.lr = 0x825EA390;
	sub_82466E20(ctx, base);
	// 825EA390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA3A0 size=76
    let mut pc: u32 = 0x825EA3A0;
    'dispatch: loop {
        match pc {
            0x825EA3A0 => {
    //   block [0x825EA3A0..0x825EA3EC)
	// 825EA3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA3A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825EA3AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA3B0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EA3B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825EA3B8: 3BEB9198  addi r31, r11, -0x6e68
	ctx.r[31].s64 = ctx.r[11].s64 + -28264;
	// 825EA3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825EA3C0: 4812395D  bl 0x8270dd1c
	ctx.lr = 0x825EA3C4;
	// extern call 0x8270DD1C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 825EA3C4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825EA3C8: 3D408271  lis r10, -0x7d8f
	ctx.r[10].s64 = -2106523648;
	// 825EA3CC: 386ACCB0  addi r3, r10, -0x3350
	ctx.r[3].s64 = ctx.r[10].s64 + -13136;
	// 825EA3D0: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825EA3D4: 4BF48765  bl 0x82532b38
	ctx.lr = 0x825EA3D8;
	sub_82532B38(ctx, base);
	// 825EA3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825EA3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA3E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825EA3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA3F0 size=28
    let mut pc: u32 = 0x825EA3F0;
    'dispatch: loop {
        match pc {
            0x825EA3F0 => {
    //   block [0x825EA3F0..0x825EA40C)
	// 825EA3F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA3F4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA3F8: 396B4F74  addi r11, r11, 0x4f74
	ctx.r[11].s64 = ctx.r[11].s64 + 20340;
	// 825EA3FC: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA400: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA404: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA410 size=28
    let mut pc: u32 = 0x825EA410;
    'dispatch: loop {
        match pc {
            0x825EA410 => {
    //   block [0x825EA410..0x825EA42C)
	// 825EA410: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA414: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA418: 396B4F88  addi r11, r11, 0x4f88
	ctx.r[11].s64 = ctx.r[11].s64 + 20360;
	// 825EA41C: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA420: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA424: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA430 size=108
    let mut pc: u32 = 0x825EA430;
    'dispatch: loop {
        match pc {
            0x825EA430 => {
    //   block [0x825EA430..0x825EA49C)
	// 825EA430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA43C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA444: 38EB8C90  addi r7, r11, -0x7370
	ctx.r[7].s64 = ctx.r[11].s64 + -29552;
	// 825EA448: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EA44C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 825EA450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA454: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA460: 386A9214  addi r3, r10, -0x6dec
	ctx.r[3].s64 = ctx.r[10].s64 + -28140;
	// 825EA464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA47C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EA480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA488: 4BE7C999  bl 0x82466e20
	ctx.lr = 0x825EA48C;
	sub_82466E20(ctx, base);
	// 825EA48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA4A0 size=108
    let mut pc: u32 = 0x825EA4A0;
    'dispatch: loop {
        match pc {
            0x825EA4A0 => {
    //   block [0x825EA4A0..0x825EA50C)
	// 825EA4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA4AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA4B4: 38EB8CD8  addi r7, r11, -0x7328
	ctx.r[7].s64 = ctx.r[11].s64 + -29480;
	// 825EA4B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EA4BC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 825EA4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA4D0: 386A9244  addi r3, r10, -0x6dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28092;
	// 825EA4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA4EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA4F8: 4BE7C929  bl 0x82466e20
	ctx.lr = 0x825EA4FC;
	sub_82466E20(ctx, base);
	// 825EA4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA510 size=108
    let mut pc: u32 = 0x825EA510;
    'dispatch: loop {
        match pc {
            0x825EA510 => {
    //   block [0x825EA510..0x825EA57C)
	// 825EA510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA51C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA524: 38EB8DC0  addi r7, r11, -0x7240
	ctx.r[7].s64 = ctx.r[11].s64 + -29248;
	// 825EA528: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825EA52C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 825EA530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA540: 386A9274  addi r3, r10, -0x6d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28044;
	// 825EA544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA55C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825EA560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA568: 4BE7C8B9  bl 0x82466e20
	ctx.lr = 0x825EA56C;
	sub_82466E20(ctx, base);
	// 825EA56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA580 size=108
    let mut pc: u32 = 0x825EA580;
    'dispatch: loop {
        match pc {
            0x825EA580 => {
    //   block [0x825EA580..0x825EA5EC)
	// 825EA580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA58C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA594: 38EB8F68  addi r7, r11, -0x7098
	ctx.r[7].s64 = ctx.r[11].s64 + -28824;
	// 825EA598: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825EA59C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 825EA5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA5A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA5B0: 386A92A4  addi r3, r10, -0x6d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27996;
	// 825EA5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA5CC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 825EA5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA5D8: 4BE7C849  bl 0x82466e20
	ctx.lr = 0x825EA5DC;
	sub_82466E20(ctx, base);
	// 825EA5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA5F0 size=72
    let mut pc: u32 = 0x825EA5F0;
    'dispatch: loop {
        match pc {
            0x825EA5F0 => {
    //   block [0x825EA5F0..0x825EA638)
	// 825EA5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA5FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA600: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 825EA604: 38CB9084  addi r6, r11, -0x6f7c
	ctx.r[6].s64 = ctx.r[11].s64 + -28540;
	// 825EA608: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825EA60C: 388BB518  addi r4, r11, -0x4ae8
	ctx.r[4].s64 = ctx.r[11].s64 + -19176;
	// 825EA610: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EA614: 386B92D8  addi r3, r11, -0x6d28
	ctx.r[3].s64 = ctx.r[11].s64 + -27944;
	// 825EA618: 4BE914A1  bl 0x8247bab8
	ctx.lr = 0x825EA61C;
	sub_8247BAB8(ctx, base);
	// 825EA61C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825EA620: 386BCCD8  addi r3, r11, -0x3328
	ctx.r[3].s64 = ctx.r[11].s64 + -13096;
	// 825EA624: 4BF48515  bl 0x82532b38
	ctx.lr = 0x825EA628;
	sub_82532B38(ctx, base);
	// 825EA628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825EA62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA638 size=28
    let mut pc: u32 = 0x825EA638;
    'dispatch: loop {
        match pc {
            0x825EA638 => {
    //   block [0x825EA638..0x825EA654)
	// 825EA638: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA63C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA640: 396B5308  addi r11, r11, 0x5308
	ctx.r[11].s64 = ctx.r[11].s64 + 21256;
	// 825EA644: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA648: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA64C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA658 size=28
    let mut pc: u32 = 0x825EA658;
    'dispatch: loop {
        match pc {
            0x825EA658 => {
    //   block [0x825EA658..0x825EA674)
	// 825EA658: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA65C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA660: 396B5410  addi r11, r11, 0x5410
	ctx.r[11].s64 = ctx.r[11].s64 + 21520;
	// 825EA664: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA668: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA66C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA678 size=36
    let mut pc: u32 = 0x825EA678;
    'dispatch: loop {
        match pc {
            0x825EA678 => {
    //   block [0x825EA678..0x825EA69C)
	// 825EA678: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA67C: 816B9A2C  lwz r11, -0x65d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26068 as u32) ) } as u64;
	// 825EA680: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825EA684: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825EA688: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA68C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 825EA690: 396B5470  addi r11, r11, 0x5470
	ctx.r[11].s64 = ctx.r[11].s64 + 21616;
	// 825EA694: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 825EA698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA6A0 size=24
    let mut pc: u32 = 0x825EA6A0;
    'dispatch: loop {
        match pc {
            0x825EA6A0 => {
    //   block [0x825EA6A0..0x825EA6B8)
	// 825EA6A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA6A4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EA6A8: 394A54FC  addi r10, r10, 0x54fc
	ctx.r[10].s64 = ctx.r[10].s64 + 21756;
	// 825EA6AC: 816B56AC  lwz r11, 0x56ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22188 as u32) ) } as u64;
	// 825EA6B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EA6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA6B8 size=108
    let mut pc: u32 = 0x825EA6B8;
    'dispatch: loop {
        match pc {
            0x825EA6B8 => {
    //   block [0x825EA6B8..0x825EA724)
	// 825EA6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA6C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA6CC: 38EB54FC  addi r7, r11, 0x54fc
	ctx.r[7].s64 = ctx.r[11].s64 + 21756;
	// 825EA6D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA6D4: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 825EA6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA6E8: 386A92F4  addi r3, r10, -0x6d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27916;
	// 825EA6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA704: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EA708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA710: 4BE7C711  bl 0x82466e20
	ctx.lr = 0x825EA714;
	sub_82466E20(ctx, base);
	// 825EA714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA728 size=108
    let mut pc: u32 = 0x825EA728;
    'dispatch: loop {
        match pc {
            0x825EA728 => {
    //   block [0x825EA728..0x825EA794)
	// 825EA728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA734: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA73C: 38EB9C0C  addi r7, r11, -0x63f4
	ctx.r[7].s64 = ctx.r[11].s64 + -25588;
	// 825EA740: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EA744: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 825EA748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA74C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA758: 386A9324  addi r3, r10, -0x6cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -27868;
	// 825EA75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA774: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA780: 4BE7C6A1  bl 0x82466e20
	ctx.lr = 0x825EA784;
	sub_82466E20(ctx, base);
	// 825EA784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA798 size=24
    let mut pc: u32 = 0x825EA798;
    'dispatch: loop {
        match pc {
            0x825EA798 => {
    //   block [0x825EA798..0x825EA7B0)
	// 825EA798: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA79C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EA7A0: 394A554C  addi r10, r10, 0x554c
	ctx.r[10].s64 = ctx.r[10].s64 + 21836;
	// 825EA7A4: 816B56AC  lwz r11, 0x56ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22188 as u32) ) } as u64;
	// 825EA7A8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EA7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA7B0 size=108
    let mut pc: u32 = 0x825EA7B0;
    'dispatch: loop {
        match pc {
            0x825EA7B0 => {
    //   block [0x825EA7B0..0x825EA81C)
	// 825EA7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA7BC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA7C4: 38EB554C  addi r7, r11, 0x554c
	ctx.r[7].s64 = ctx.r[11].s64 + 21836;
	// 825EA7C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA7CC: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 825EA7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA7D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA7E0: 386A9354  addi r3, r10, -0x6cac
	ctx.r[3].s64 = ctx.r[10].s64 + -27820;
	// 825EA7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA7FC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EA800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA808: 4BE7C619  bl 0x82466e20
	ctx.lr = 0x825EA80C;
	sub_82466E20(ctx, base);
	// 825EA80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA820 size=112
    let mut pc: u32 = 0x825EA820;
    'dispatch: loop {
        match pc {
            0x825EA820 => {
    //   block [0x825EA820..0x825EA890)
	// 825EA820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA830: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA834: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EA838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA83C: 390B9C7C  addi r8, r11, -0x6384
	ctx.r[8].s64 = ctx.r[11].s64 + -25476;
	// 825EA840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EA844: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 825EA848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA84C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EA854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA858: 386A9384  addi r3, r10, -0x6c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -27772;
	// 825EA85C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EA860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA874: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EA878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA87C: 4BE7C5A5  bl 0x82466e20
	ctx.lr = 0x825EA880;
	sub_82466E20(ctx, base);
	// 825EA880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA890 size=108
    let mut pc: u32 = 0x825EA890;
    'dispatch: loop {
        match pc {
            0x825EA890 => {
    //   block [0x825EA890..0x825EA8FC)
	// 825EA890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA89C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA8A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA8A4: 38EB9CAC  addi r7, r11, -0x6354
	ctx.r[7].s64 = ctx.r[11].s64 + -25428;
	// 825EA8A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA8AC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 825EA8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA8B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA8B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA8C0: 386A93B4  addi r3, r10, -0x6c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27724;
	// 825EA8C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA8DC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EA8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA8E8: 4BE7C539  bl 0x82466e20
	ctx.lr = 0x825EA8EC;
	sub_82466E20(ctx, base);
	// 825EA8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA900 size=108
    let mut pc: u32 = 0x825EA900;
    'dispatch: loop {
        match pc {
            0x825EA900 => {
    //   block [0x825EA900..0x825EA96C)
	// 825EA900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA90C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA914: 38EB9D08  addi r7, r11, -0x62f8
	ctx.r[7].s64 = ctx.r[11].s64 + -25336;
	// 825EA918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA91C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 825EA920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA930: 386A93E4  addi r3, r10, -0x6c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -27676;
	// 825EA934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA94C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EA950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA958: 4BE7C4C9  bl 0x82466e20
	ctx.lr = 0x825EA95C;
	sub_82466E20(ctx, base);
	// 825EA95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA970 size=108
    let mut pc: u32 = 0x825EA970;
    'dispatch: loop {
        match pc {
            0x825EA970 => {
    //   block [0x825EA970..0x825EA9DC)
	// 825EA970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA97C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA984: 38EB9E40  addi r7, r11, -0x61c0
	ctx.r[7].s64 = ctx.r[11].s64 + -25024;
	// 825EA988: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825EA98C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 825EA990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA9A0: 386A9414  addi r3, r10, -0x6bec
	ctx.r[3].s64 = ctx.r[10].s64 + -27628;
	// 825EA9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA9BC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EA9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA9C8: 4BE7C459  bl 0x82466e20
	ctx.lr = 0x825EA9CC;
	sub_82466E20(ctx, base);
	// 825EA9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA9E0 size=108
    let mut pc: u32 = 0x825EA9E0;
    'dispatch: loop {
        match pc {
            0x825EA9E0 => {
    //   block [0x825EA9E0..0x825EAA4C)
	// 825EA9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA9EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA9F4: 38EB9F28  addi r7, r11, -0x60d8
	ctx.r[7].s64 = ctx.r[11].s64 + -24792;
	// 825EA9F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825EA9FC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 825EAA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAA04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAA10: 386A9444  addi r3, r10, -0x6bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -27580;
	// 825EAA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAA2C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EAA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAA38: 4BE7C3E9  bl 0x82466e20
	ctx.lr = 0x825EAA3C;
	sub_82466E20(ctx, base);
	// 825EAA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAA50 size=108
    let mut pc: u32 = 0x825EAA50;
    'dispatch: loop {
        match pc {
            0x825EAA50 => {
    //   block [0x825EAA50..0x825EAABC)
	// 825EAA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAA5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAA64: 38EBA02C  addi r7, r11, -0x5fd4
	ctx.r[7].s64 = ctx.r[11].s64 + -24532;
	// 825EAA68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAA6C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 825EAA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAA80: 386A9474  addi r3, r10, -0x6b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -27532;
	// 825EAA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAA9C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAAA8: 4BE7C379  bl 0x82466e20
	ctx.lr = 0x825EAAAC;
	sub_82466E20(ctx, base);
	// 825EAAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAAC0 size=108
    let mut pc: u32 = 0x825EAAC0;
    'dispatch: loop {
        match pc {
            0x825EAAC0 => {
    //   block [0x825EAAC0..0x825EAB2C)
	// 825EAAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAACC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAAD4: 38EBA068  addi r7, r11, -0x5f98
	ctx.r[7].s64 = ctx.r[11].s64 + -24472;
	// 825EAAD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAADC: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 825EAAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAAE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAAE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAAF0: 386A94A4  addi r3, r10, -0x6b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27484;
	// 825EAAF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAB0C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAB14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAB18: 4BE7C309  bl 0x82466e20
	ctx.lr = 0x825EAB1C;
	sub_82466E20(ctx, base);
	// 825EAB1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAB30 size=108
    let mut pc: u32 = 0x825EAB30;
    'dispatch: loop {
        match pc {
            0x825EAB30 => {
    //   block [0x825EAB30..0x825EAB9C)
	// 825EAB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAB3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAB44: 38EBA0A8  addi r7, r11, -0x5f58
	ctx.r[7].s64 = ctx.r[11].s64 + -24408;
	// 825EAB48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAB4C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 825EAB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAB54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAB58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAB60: 386A94D4  addi r3, r10, -0x6b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27436;
	// 825EAB64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAB7C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAB88: 4BE7C299  bl 0x82466e20
	ctx.lr = 0x825EAB8C;
	sub_82466E20(ctx, base);
	// 825EAB8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAB90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAB94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EABA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EABA0 size=108
    let mut pc: u32 = 0x825EABA0;
    'dispatch: loop {
        match pc {
            0x825EABA0 => {
    //   block [0x825EABA0..0x825EAC0C)
	// 825EABA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EABA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EABA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EABAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EABB4: 38EBA0E8  addi r7, r11, -0x5f18
	ctx.r[7].s64 = ctx.r[11].s64 + -24344;
	// 825EABB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EABBC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 825EABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EABC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EABC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EABCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EABD0: 386A9504  addi r3, r10, -0x6afc
	ctx.r[3].s64 = ctx.r[10].s64 + -27388;
	// 825EABD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EABD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EABDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EABE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EABEC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EABF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EABF8: 4BE7C229  bl 0x82466e20
	ctx.lr = 0x825EABFC;
	sub_82466E20(ctx, base);
	// 825EABFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAC10 size=108
    let mut pc: u32 = 0x825EAC10;
    'dispatch: loop {
        match pc {
            0x825EAC10 => {
    //   block [0x825EAC10..0x825EAC7C)
	// 825EAC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAC1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAC24: 38EBA118  addi r7, r11, -0x5ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -24296;
	// 825EAC28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAC2C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 825EAC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAC40: 386A9534  addi r3, r10, -0x6acc
	ctx.r[3].s64 = ctx.r[10].s64 + -27340;
	// 825EAC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAC5C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EAC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAC68: 4BE7C1B9  bl 0x82466e20
	ctx.lr = 0x825EAC6C;
	sub_82466E20(ctx, base);
	// 825EAC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAC80 size=108
    let mut pc: u32 = 0x825EAC80;
    'dispatch: loop {
        match pc {
            0x825EAC80 => {
    //   block [0x825EAC80..0x825EACEC)
	// 825EAC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAC8C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAC94: 38EBA168  addi r7, r11, -0x5e98
	ctx.r[7].s64 = ctx.r[11].s64 + -24216;
	// 825EAC98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825EAC9C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 825EACA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EACA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EACA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EACAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EACB0: 386A9564  addi r3, r10, -0x6a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -27292;
	// 825EACB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EACB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EACBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EACC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EACC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EACC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EACCC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EACD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EACD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EACD8: 4BE7C149  bl 0x82466e20
	ctx.lr = 0x825EACDC;
	sub_82466E20(ctx, base);
	// 825EACDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EACE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EACE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EACE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EACF0 size=108
    let mut pc: u32 = 0x825EACF0;
    'dispatch: loop {
        match pc {
            0x825EACF0 => {
    //   block [0x825EACF0..0x825EAD5C)
	// 825EACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EACF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EACFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAD04: 38EBA254  addi r7, r11, -0x5dac
	ctx.r[7].s64 = ctx.r[11].s64 + -23980;
	// 825EAD08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAD0C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 825EAD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAD14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAD18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAD1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAD20: 386A9594  addi r3, r10, -0x6a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -27244;
	// 825EAD24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAD3C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAD44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAD48: 4BE7C0D9  bl 0x82466e20
	ctx.lr = 0x825EAD4C;
	sub_82466E20(ctx, base);
	// 825EAD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAD60 size=108
    let mut pc: u32 = 0x825EAD60;
    'dispatch: loop {
        match pc {
            0x825EAD60 => {
    //   block [0x825EAD60..0x825EADCC)
	// 825EAD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAD6C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAD74: 38EBA298  addi r7, r11, -0x5d68
	ctx.r[7].s64 = ctx.r[11].s64 + -23912;
	// 825EAD78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAD7C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 825EAD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAD84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAD90: 386A95C4  addi r3, r10, -0x6a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -27196;
	// 825EAD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EADA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EADA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EADA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EADAC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 825EADB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EADB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EADB8: 4BE7C069  bl 0x82466e20
	ctx.lr = 0x825EADBC;
	sub_82466E20(ctx, base);
	// 825EADBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EADC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EADC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EADC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EADD0 size=108
    let mut pc: u32 = 0x825EADD0;
    'dispatch: loop {
        match pc {
            0x825EADD0 => {
    //   block [0x825EADD0..0x825EAE3C)
	// 825EADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EADD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EADD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EADDC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EADE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EADE4: 38EBA2B0  addi r7, r11, -0x5d50
	ctx.r[7].s64 = ctx.r[11].s64 + -23888;
	// 825EADE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EADEC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 825EADF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EADF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EADF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EADFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAE00: 386A95F4  addi r3, r10, -0x6a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27148;
	// 825EAE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAE1C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EAE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAE28: 4BE7BFF9  bl 0x82466e20
	ctx.lr = 0x825EAE2C;
	sub_82466E20(ctx, base);
	// 825EAE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAE40 size=108
    let mut pc: u32 = 0x825EAE40;
    'dispatch: loop {
        match pc {
            0x825EAE40 => {
    //   block [0x825EAE40..0x825EAEAC)
	// 825EAE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAE4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAE54: 38EBA320  addi r7, r11, -0x5ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -23776;
	// 825EAE58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EAE5C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 825EAE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAE64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAE68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAE70: 386A9624  addi r3, r10, -0x69dc
	ctx.r[3].s64 = ctx.r[10].s64 + -27100;
	// 825EAE74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAE8C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EAE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAE94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAE98: 4BE7BF89  bl 0x82466e20
	ctx.lr = 0x825EAE9C;
	sub_82466E20(ctx, base);
	// 825EAE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAEB0 size=112
    let mut pc: u32 = 0x825EAEB0;
    'dispatch: loop {
        match pc {
            0x825EAEB0 => {
    //   block [0x825EAEB0..0x825EAF20)
	// 825EAEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAEBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAEC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAEC4: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EAEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAECC: 390BA374  addi r8, r11, -0x5c8c
	ctx.r[8].s64 = ctx.r[11].s64 + -23692;
	// 825EAED0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EAED4: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 825EAED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAEDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EAEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAEE8: 386A9654  addi r3, r10, -0x69ac
	ctx.r[3].s64 = ctx.r[10].s64 + -27052;
	// 825EAEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EAEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAF04: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 825EAF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAF0C: 4BE7BF15  bl 0x82466e20
	ctx.lr = 0x825EAF10;
	sub_82466E20(ctx, base);
	// 825EAF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAF20 size=108
    let mut pc: u32 = 0x825EAF20;
    'dispatch: loop {
        match pc {
            0x825EAF20 => {
    //   block [0x825EAF20..0x825EAF8C)
	// 825EAF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAF2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAF34: 396BA460  addi r11, r11, -0x5ba0
	ctx.r[11].s64 = ctx.r[11].s64 + -23456;
	// 825EAF38: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 825EAF3C: 390B0138  addi r8, r11, 0x138
	ctx.r[8].s64 = ctx.r[11].s64 + 312;
	// 825EAF40: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 825EAF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAF48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAF4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EAF50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EAF54: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 825EAF58: 386A9684  addi r3, r10, -0x697c
	ctx.r[3].s64 = ctx.r[10].s64 + -27004;
	// 825EAF5C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 825EAF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAF68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825EAF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAF70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825EAF74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAF78: 4BE7BEA9  bl 0x82466e20
	ctx.lr = 0x825EAF7C;
	sub_82466E20(ctx, base);
	// 825EAF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAF80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAF84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAF90 size=108
    let mut pc: u32 = 0x825EAF90;
    'dispatch: loop {
        match pc {
            0x825EAF90 => {
    //   block [0x825EAF90..0x825EAFFC)
	// 825EAF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAF9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAFA4: 38EBA610  addi r7, r11, -0x59f0
	ctx.r[7].s64 = ctx.r[11].s64 + -23024;
	// 825EAFA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EAFAC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825EAFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAFB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAFB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAFC0: 386A96B4  addi r3, r10, -0x694c
	ctx.r[3].s64 = ctx.r[10].s64 + -26956;
	// 825EAFC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAFDC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EAFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAFE8: 4BE7BE39  bl 0x82466e20
	ctx.lr = 0x825EAFEC;
	sub_82466E20(ctx, base);
	// 825EAFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB000 size=108
    let mut pc: u32 = 0x825EB000;
    'dispatch: loop {
        match pc {
            0x825EB000 => {
    //   block [0x825EB000..0x825EB06C)
	// 825EB000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB00C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB014: 38EBA66C  addi r7, r11, -0x5994
	ctx.r[7].s64 = ctx.r[11].s64 + -22932;
	// 825EB018: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB01C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825EB020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB030: 386A96E4  addi r3, r10, -0x691c
	ctx.r[3].s64 = ctx.r[10].s64 + -26908;
	// 825EB034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB04C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB058: 4BE7BDC9  bl 0x82466e20
	ctx.lr = 0x825EB05C;
	sub_82466E20(ctx, base);
	// 825EB05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB070 size=108
    let mut pc: u32 = 0x825EB070;
    'dispatch: loop {
        match pc {
            0x825EB070 => {
    //   block [0x825EB070..0x825EB0DC)
	// 825EB070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB07C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB084: 392BA734  addi r9, r11, -0x58cc
	ctx.r[9].s64 = ctx.r[11].s64 + -22732;
	// 825EB088: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EB08C: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 825EB090: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 825EB094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB098: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB09C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB0A0: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB0A4: 386A9714  addi r3, r10, -0x68ec
	ctx.r[3].s64 = ctx.r[10].s64 + -26860;
	// 825EB0A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB0AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB0B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB0B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB0B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB0BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB0C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB0C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB0C8: 4BE7BD59  bl 0x82466e20
	ctx.lr = 0x825EB0CC;
	sub_82466E20(ctx, base);
	// 825EB0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB0E0 size=108
    let mut pc: u32 = 0x825EB0E0;
    'dispatch: loop {
        match pc {
            0x825EB0E0 => {
    //   block [0x825EB0E0..0x825EB14C)
	// 825EB0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB0EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB0F4: 38EBA858  addi r7, r11, -0x57a8
	ctx.r[7].s64 = ctx.r[11].s64 + -22440;
	// 825EB0F8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825EB0FC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 825EB100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB110: 386A9744  addi r3, r10, -0x68bc
	ctx.r[3].s64 = ctx.r[10].s64 + -26812;
	// 825EB114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB12C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB138: 4BE7BCE9  bl 0x82466e20
	ctx.lr = 0x825EB13C;
	sub_82466E20(ctx, base);
	// 825EB13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB150 size=112
    let mut pc: u32 = 0x825EB150;
    'dispatch: loop {
        match pc {
            0x825EB150 => {
    //   block [0x825EB150..0x825EB1C0)
	// 825EB150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB15C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB160: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB164: 38AA96B4  addi r5, r10, -0x694c
	ctx.r[5].s64 = ctx.r[10].s64 + -26956;
	// 825EB168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB16C: 390BA988  addi r8, r11, -0x5678
	ctx.r[8].s64 = ctx.r[11].s64 + -22136;
	// 825EB170: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EB174: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 825EB178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB17C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB188: 386A9774  addi r3, r10, -0x688c
	ctx.r[3].s64 = ctx.r[10].s64 + -26764;
	// 825EB18C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EB190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB1A4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 825EB1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB1AC: 4BE7BC75  bl 0x82466e20
	ctx.lr = 0x825EB1B0;
	sub_82466E20(ctx, base);
	// 825EB1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB1C0 size=108
    let mut pc: u32 = 0x825EB1C0;
    'dispatch: loop {
        match pc {
            0x825EB1C0 => {
    //   block [0x825EB1C0..0x825EB22C)
	// 825EB1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB1CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB1D4: 38EBA9CC  addi r7, r11, -0x5634
	ctx.r[7].s64 = ctx.r[11].s64 + -22068;
	// 825EB1D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB1DC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 825EB1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB1E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB1E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB1F0: 386A97A4  addi r3, r10, -0x685c
	ctx.r[3].s64 = ctx.r[10].s64 + -26716;
	// 825EB1F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB20C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EB210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB218: 4BE7BC09  bl 0x82466e20
	ctx.lr = 0x825EB21C;
	sub_82466E20(ctx, base);
	// 825EB21C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB230 size=108
    let mut pc: u32 = 0x825EB230;
    'dispatch: loop {
        match pc {
            0x825EB230 => {
    //   block [0x825EB230..0x825EB29C)
	// 825EB230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB23C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB244: 38EBAA14  addi r7, r11, -0x55ec
	ctx.r[7].s64 = ctx.r[11].s64 + -21996;
	// 825EB248: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB24C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 825EB250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB260: 386A97D4  addi r3, r10, -0x682c
	ctx.r[3].s64 = ctx.r[10].s64 + -26668;
	// 825EB264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB27C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EB280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB288: 4BE7BB99  bl 0x82466e20
	ctx.lr = 0x825EB28C;
	sub_82466E20(ctx, base);
	// 825EB28C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB2A0 size=108
    let mut pc: u32 = 0x825EB2A0;
    'dispatch: loop {
        match pc {
            0x825EB2A0 => {
    //   block [0x825EB2A0..0x825EB30C)
	// 825EB2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB2AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB2B4: 38EBAB7C  addi r7, r11, -0x5484
	ctx.r[7].s64 = ctx.r[11].s64 + -21636;
	// 825EB2B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB2BC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 825EB2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB2C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB2C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB2D0: 386A9804  addi r3, r10, -0x67fc
	ctx.r[3].s64 = ctx.r[10].s64 + -26620;
	// 825EB2D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB2EC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB2F8: 4BE7BB29  bl 0x82466e20
	ctx.lr = 0x825EB2FC;
	sub_82466E20(ctx, base);
	// 825EB2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB310 size=112
    let mut pc: u32 = 0x825EB310;
    'dispatch: loop {
        match pc {
            0x825EB310 => {
    //   block [0x825EB310..0x825EB380)
	// 825EB310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB31C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB320: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB324: 392BAB68  addi r9, r11, -0x5498
	ctx.r[9].s64 = ctx.r[11].s64 + -21656;
	// 825EB328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB32C: 39090048  addi r8, r9, 0x48
	ctx.r[8].s64 = ctx.r[9].s64 + 72;
	// 825EB330: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EB334: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EB338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB33C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB340: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 825EB344: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB348: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 825EB34C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB350: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EB354: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB358: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB360: 386B9834  addi r3, r11, -0x67cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26572;
	// 825EB364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB368: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB36C: 4BE7BAB5  bl 0x82466e20
	ctx.lr = 0x825EB370;
	sub_82466E20(ctx, base);
	// 825EB370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB380 size=108
    let mut pc: u32 = 0x825EB380;
    'dispatch: loop {
        match pc {
            0x825EB380 => {
    //   block [0x825EB380..0x825EB3EC)
	// 825EB380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB38C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB394: 38EBACC8  addi r7, r11, -0x5338
	ctx.r[7].s64 = ctx.r[11].s64 + -21304;
	// 825EB398: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EB39C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 825EB3A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB3A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB3A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB3B0: 386A9864  addi r3, r10, -0x679c
	ctx.r[3].s64 = ctx.r[10].s64 + -26524;
	// 825EB3B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB3C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB3CC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 825EB3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB3D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB3D8: 4BE7BA49  bl 0x82466e20
	ctx.lr = 0x825EB3DC;
	sub_82466E20(ctx, base);
	// 825EB3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB3F0 size=108
    let mut pc: u32 = 0x825EB3F0;
    'dispatch: loop {
        match pc {
            0x825EB3F0 => {
    //   block [0x825EB3F0..0x825EB45C)
	// 825EB3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB3FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB404: 38EBAD38  addi r7, r11, -0x52c8
	ctx.r[7].s64 = ctx.r[11].s64 + -21192;
	// 825EB408: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EB40C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 825EB410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB414: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB420: 386A9894  addi r3, r10, -0x676c
	ctx.r[3].s64 = ctx.r[10].s64 + -26476;
	// 825EB424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB43C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825EB440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB448: 4BE7B9D9  bl 0x82466e20
	ctx.lr = 0x825EB44C;
	sub_82466E20(ctx, base);
	// 825EB44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB460 size=108
    let mut pc: u32 = 0x825EB460;
    'dispatch: loop {
        match pc {
            0x825EB460 => {
    //   block [0x825EB460..0x825EB4CC)
	// 825EB460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB46C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB474: 38EBAE28  addi r7, r11, -0x51d8
	ctx.r[7].s64 = ctx.r[11].s64 + -20952;
	// 825EB478: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EB47C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825EB480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB490: 386A98C4  addi r3, r10, -0x673c
	ctx.r[3].s64 = ctx.r[10].s64 + -26428;
	// 825EB494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB4AC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 825EB4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB4B8: 4BE7B969  bl 0x82466e20
	ctx.lr = 0x825EB4BC;
	sub_82466E20(ctx, base);
	// 825EB4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB4D0 size=108
    let mut pc: u32 = 0x825EB4D0;
    'dispatch: loop {
        match pc {
            0x825EB4D0 => {
    //   block [0x825EB4D0..0x825EB53C)
	// 825EB4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB4DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB4E4: 38EBAEE8  addi r7, r11, -0x5118
	ctx.r[7].s64 = ctx.r[11].s64 + -20760;
	// 825EB4E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB4EC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 825EB4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB4F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB4F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB500: 386A98F4  addi r3, r10, -0x670c
	ctx.r[3].s64 = ctx.r[10].s64 + -26380;
	// 825EB504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB51C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 825EB520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB528: 4BE7B8F9  bl 0x82466e20
	ctx.lr = 0x825EB52C;
	sub_82466E20(ctx, base);
	// 825EB52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB540 size=108
    let mut pc: u32 = 0x825EB540;
    'dispatch: loop {
        match pc {
            0x825EB540 => {
    //   block [0x825EB540..0x825EB5AC)
	// 825EB540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB54C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB554: 38EBAF24  addi r7, r11, -0x50dc
	ctx.r[7].s64 = ctx.r[11].s64 + -20700;
	// 825EB558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB55C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 825EB560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB570: 386A9924  addi r3, r10, -0x66dc
	ctx.r[3].s64 = ctx.r[10].s64 + -26332;
	// 825EB574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB58C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB598: 4BE7B889  bl 0x82466e20
	ctx.lr = 0x825EB59C;
	sub_82466E20(ctx, base);
	// 825EB59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB5B0 size=24
    let mut pc: u32 = 0x825EB5B0;
    'dispatch: loop {
        match pc {
            0x825EB5B0 => {
    //   block [0x825EB5B0..0x825EB5C8)
	// 825EB5B0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB5B4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB5B8: 394A5790  addi r10, r10, 0x5790
	ctx.r[10].s64 = ctx.r[10].s64 + 22416;
	// 825EB5BC: 816B5778  lwz r11, 0x5778(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22392 as u32) ) } as u64;
	// 825EB5C0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EB5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB5C8 size=112
    let mut pc: u32 = 0x825EB5C8;
    'dispatch: loop {
        match pc {
            0x825EB5C8 => {
    //   block [0x825EB5C8..0x825EB638)
	// 825EB5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB5D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB5D8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB5DC: 392AAFF0  addi r9, r10, -0x5010
	ctx.r[9].s64 = ctx.r[10].s64 + -20496;
	// 825EB5E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB5E4: 390B5790  addi r8, r11, 0x5790
	ctx.r[8].s64 = ctx.r[11].s64 + 22416;
	// 825EB5E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EB5EC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 825EB5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB5F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB5F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB5FC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EB600: 386A9954  addi r3, r10, -0x66ac
	ctx.r[3].s64 = ctx.r[10].s64 + -26284;
	// 825EB604: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB608: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB61C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB624: 4BE7B7FD  bl 0x82466e20
	ctx.lr = 0x825EB628;
	sub_82466E20(ctx, base);
	// 825EB628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB638 size=24
    let mut pc: u32 = 0x825EB638;
    'dispatch: loop {
        match pc {
            0x825EB638 => {
    //   block [0x825EB638..0x825EB650)
	// 825EB638: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB63C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB640: 394A57E8  addi r10, r10, 0x57e8
	ctx.r[10].s64 = ctx.r[10].s64 + 22504;
	// 825EB644: 816B56AC  lwz r11, 0x56ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22188 as u32) ) } as u64;
	// 825EB648: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EB64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB650 size=108
    let mut pc: u32 = 0x825EB650;
    'dispatch: loop {
        match pc {
            0x825EB650 => {
    //   block [0x825EB650..0x825EB6BC)
	// 825EB650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB65C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB664: 38EB57E8  addi r7, r11, 0x57e8
	ctx.r[7].s64 = ctx.r[11].s64 + 22504;
	// 825EB668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB66C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 825EB670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB680: 386A9984  addi r3, r10, -0x667c
	ctx.r[3].s64 = ctx.r[10].s64 + -26236;
	// 825EB684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB69C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB6A8: 4BE7B779  bl 0x82466e20
	ctx.lr = 0x825EB6AC;
	sub_82466E20(ctx, base);
	// 825EB6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB6C0 size=24
    let mut pc: u32 = 0x825EB6C0;
    'dispatch: loop {
        match pc {
            0x825EB6C0 => {
    //   block [0x825EB6C0..0x825EB6D8)
	// 825EB6C0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB6C4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB6C8: 394A5830  addi r10, r10, 0x5830
	ctx.r[10].s64 = ctx.r[10].s64 + 22576;
	// 825EB6CC: 816B5818  lwz r11, 0x5818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22552 as u32) ) } as u64;
	// 825EB6D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EB6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB6D8 size=112
    let mut pc: u32 = 0x825EB6D8;
    'dispatch: loop {
        match pc {
            0x825EB6D8 => {
    //   block [0x825EB6D8..0x825EB748)
	// 825EB6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB6E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB6E8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB6EC: 392AB084  addi r9, r10, -0x4f7c
	ctx.r[9].s64 = ctx.r[10].s64 + -20348;
	// 825EB6F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB6F4: 390B5830  addi r8, r11, 0x5830
	ctx.r[8].s64 = ctx.r[11].s64 + 22576;
	// 825EB6F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EB6FC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 825EB700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB704: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB70C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 825EB710: 386A99B4  addi r3, r10, -0x664c
	ctx.r[3].s64 = ctx.r[10].s64 + -26188;
	// 825EB714: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB718: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB734: 4BE7B6ED  bl 0x82466e20
	ctx.lr = 0x825EB738;
	sub_82466E20(ctx, base);
	// 825EB738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB748 size=108
    let mut pc: u32 = 0x825EB748;
    'dispatch: loop {
        match pc {
            0x825EB748 => {
    //   block [0x825EB748..0x825EB7B4)
	// 825EB748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB754: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB75C: 38EBB0E8  addi r7, r11, -0x4f18
	ctx.r[7].s64 = ctx.r[11].s64 + -20248;
	// 825EB760: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EB764: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825EB768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB76C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB778: 386A99E4  addi r3, r10, -0x661c
	ctx.r[3].s64 = ctx.r[10].s64 + -26140;
	// 825EB77C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB794: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 825EB798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB7A0: 4BE7B681  bl 0x82466e20
	ctx.lr = 0x825EB7A4;
	sub_82466E20(ctx, base);
	// 825EB7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB7B8 size=108
    let mut pc: u32 = 0x825EB7B8;
    'dispatch: loop {
        match pc {
            0x825EB7B8 => {
    //   block [0x825EB7B8..0x825EB824)
	// 825EB7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB7C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB7C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB7CC: 38EBB1B8  addi r7, r11, -0x4e48
	ctx.r[7].s64 = ctx.r[11].s64 + -20040;
	// 825EB7D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB7D4: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 825EB7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB7DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB7E8: 386A9A14  addi r3, r10, -0x65ec
	ctx.r[3].s64 = ctx.r[10].s64 + -26092;
	// 825EB7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB804: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB810: 4BE7B611  bl 0x82466e20
	ctx.lr = 0x825EB814;
	sub_82466E20(ctx, base);
	// 825EB814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB828 size=112
    let mut pc: u32 = 0x825EB828;
    'dispatch: loop {
        match pc {
            0x825EB828 => {
    //   block [0x825EB828..0x825EB898)
	// 825EB828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB834: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB838: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB83C: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EB840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB844: 390BB1E8  addi r8, r11, -0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + -19992;
	// 825EB848: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EB84C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825EB850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB854: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB860: 386A9A44  addi r3, r10, -0x65bc
	ctx.r[3].s64 = ctx.r[10].s64 + -26044;
	// 825EB864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EB868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB87C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 825EB880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB884: 4BE7B59D  bl 0x82466e20
	ctx.lr = 0x825EB888;
	sub_82466E20(ctx, base);
	// 825EB888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB898 size=108
    let mut pc: u32 = 0x825EB898;
    'dispatch: loop {
        match pc {
            0x825EB898 => {
    //   block [0x825EB898..0x825EB904)
	// 825EB898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB8A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB8AC: 38EBB2C0  addi r7, r11, -0x4d40
	ctx.r[7].s64 = ctx.r[11].s64 + -19776;
	// 825EB8B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB8B4: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825EB8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB8BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB8C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB8C8: 386A9A74  addi r3, r10, -0x658c
	ctx.r[3].s64 = ctx.r[10].s64 + -25996;
	// 825EB8CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB8E4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB8EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB8F0: 4BE7B531  bl 0x82466e20
	ctx.lr = 0x825EB8F4;
	sub_82466E20(ctx, base);
	// 825EB8F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB908 size=108
    let mut pc: u32 = 0x825EB908;
    'dispatch: loop {
        match pc {
            0x825EB908 => {
    //   block [0x825EB908..0x825EB974)
	// 825EB908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB914: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB91C: 38EBB2F0  addi r7, r11, -0x4d10
	ctx.r[7].s64 = ctx.r[11].s64 + -19728;
	// 825EB920: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EB924: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 825EB928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB938: 386A9AA4  addi r3, r10, -0x655c
	ctx.r[3].s64 = ctx.r[10].s64 + -25948;
	// 825EB93C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB954: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825EB958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB95C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB960: 4BE7B4C1  bl 0x82466e20
	ctx.lr = 0x825EB964;
	sub_82466E20(ctx, base);
	// 825EB964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB978 size=24
    let mut pc: u32 = 0x825EB978;
    'dispatch: loop {
        match pc {
            0x825EB978 => {
    //   block [0x825EB978..0x825EB990)
	// 825EB978: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB97C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB980: 394A5910  addi r10, r10, 0x5910
	ctx.r[10].s64 = ctx.r[10].s64 + 22800;
	// 825EB984: 816B58F8  lwz r11, 0x58f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22776 as u32) ) } as u64;
	// 825EB988: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EB98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB990 size=112
    let mut pc: u32 = 0x825EB990;
    'dispatch: loop {
        match pc {
            0x825EB990 => {
    //   block [0x825EB990..0x825EBA00)
	// 825EB990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB99C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB9A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB9A4: 392AB43C  addi r9, r10, -0x4bc4
	ctx.r[9].s64 = ctx.r[10].s64 + -19396;
	// 825EB9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB9AC: 390B5910  addi r8, r11, 0x5910
	ctx.r[8].s64 = ctx.r[11].s64 + 22800;
	// 825EB9B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EB9B4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 825EB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB9BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB9C4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 825EB9C8: 386A9AD4  addi r3, r10, -0x652c
	ctx.r[3].s64 = ctx.r[10].s64 + -25900;
	// 825EB9CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB9D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB9E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB9EC: 4BE7B435  bl 0x82466e20
	ctx.lr = 0x825EB9F0;
	sub_82466E20(ctx, base);
	// 825EB9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBA00 size=108
    let mut pc: u32 = 0x825EBA00;
    'dispatch: loop {
        match pc {
            0x825EBA00 => {
    //   block [0x825EBA00..0x825EBA6C)
	// 825EBA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBA0C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBA14: 38EB5988  addi r7, r11, 0x5988
	ctx.r[7].s64 = ctx.r[11].s64 + 22920;
	// 825EBA18: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EBA1C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 825EBA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBA24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBA28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBA30: 386A9B04  addi r3, r10, -0x64fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25852;
	// 825EBA34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBA58: 4BE7B3C9  bl 0x82466e20
	ctx.lr = 0x825EBA5C;
	sub_82466E20(ctx, base);
	// 825EBA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EBA70 size=24
    let mut pc: u32 = 0x825EBA70;
    'dispatch: loop {
        match pc {
            0x825EBA70 => {
    //   block [0x825EBA70..0x825EBA88)
	// 825EBA70: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBA74: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EBA78: 394ADBC8  addi r10, r10, -0x2438
	ctx.r[10].s64 = ctx.r[10].s64 + -9272;
	// 825EBA7C: 816B5A00  lwz r11, 0x5a00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23040 as u32) ) } as u64;
	// 825EBA80: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825EBA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBA88 size=112
    let mut pc: u32 = 0x825EBA88;
    'dispatch: loop {
        match pc {
            0x825EBA88 => {
    //   block [0x825EBA88..0x825EBAF8)
	// 825EBA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBA94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBA98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EBA9C: 392A0DCC  addi r9, r10, 0xdcc
	ctx.r[9].s64 = ctx.r[10].s64 + 3532;
	// 825EBAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBAA4: 390BDBC8  addi r8, r11, -0x2438
	ctx.r[8].s64 = ctx.r[11].s64 + -9272;
	// 825EBAA8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EBAAC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 825EBAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBAC0: 386A9B34  addi r3, r10, -0x64cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25804;
	// 825EBAC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EBAC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EBACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBAE4: 4BE7B33D  bl 0x82466e20
	ctx.lr = 0x825EBAE8;
	sub_82466E20(ctx, base);
	// 825EBAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBAF8 size=108
    let mut pc: u32 = 0x825EBAF8;
    'dispatch: loop {
        match pc {
            0x825EBAF8 => {
    //   block [0x825EBAF8..0x825EBB64)
	// 825EBAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBB04: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBB0C: 38EB5A04  addi r7, r11, 0x5a04
	ctx.r[7].s64 = ctx.r[11].s64 + 23044;
	// 825EBB10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EBB14: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 825EBB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBB20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBB28: 386A9B64  addi r3, r10, -0x649c
	ctx.r[3].s64 = ctx.r[10].s64 + -25756;
	// 825EBB2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBB4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBB50: 4BE7B2D1  bl 0x82466e20
	ctx.lr = 0x825EBB54;
	sub_82466E20(ctx, base);
	// 825EBB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBB68 size=108
    let mut pc: u32 = 0x825EBB68;
    'dispatch: loop {
        match pc {
            0x825EBB68 => {
    //   block [0x825EBB68..0x825EBBD4)
	// 825EBB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBB74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBB7C: 38EB5A34  addi r7, r11, 0x5a34
	ctx.r[7].s64 = ctx.r[11].s64 + 23092;
	// 825EBB80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EBB84: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 825EBB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBB90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBB98: 386A9B94  addi r3, r10, -0x646c
	ctx.r[3].s64 = ctx.r[10].s64 + -25708;
	// 825EBB9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBBBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBBC0: 4BE7B261  bl 0x82466e20
	ctx.lr = 0x825EBBC4;
	sub_82466E20(ctx, base);
	// 825EBBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EBBD8 size=24
    let mut pc: u32 = 0x825EBBD8;
    'dispatch: loop {
        match pc {
            0x825EBBD8 => {
    //   block [0x825EBBD8..0x825EBBF0)
	// 825EBBD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBBDC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EBBE0: 394ADC10  addi r10, r10, -0x23f0
	ctx.r[10].s64 = ctx.r[10].s64 + -9200;
	// 825EBBE4: 816B5A64  lwz r11, 0x5a64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23140 as u32) ) } as u64;
	// 825EBBE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EBBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBBF0 size=116
    let mut pc: u32 = 0x825EBBF0;
    'dispatch: loop {
        match pc {
            0x825EBBF0 => {
    //   block [0x825EBBF0..0x825EBC64)
	// 825EBBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBBFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EBC00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBC04: 390BDC10  addi r8, r11, -0x23f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9200;
	// 825EBC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBC0C: 392A0E00  addi r9, r10, 0xe00
	ctx.r[9].s64 = ctx.r[10].s64 + 3584;
	// 825EBC10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBC14: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EBC18: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EBC1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBC24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBC34: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EBC38: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 825EBC3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EBC40: 386B9BC4  addi r3, r11, -0x643c
	ctx.r[3].s64 = ctx.r[11].s64 + -25660;
	// 825EBC44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EBC48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBC50: 4BE7B1D1  bl 0x82466e20
	ctx.lr = 0x825EBC54;
	sub_82466E20(ctx, base);
	// 825EBC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBC68 size=108
    let mut pc: u32 = 0x825EBC68;
    'dispatch: loop {
        match pc {
            0x825EBC68 => {
    //   block [0x825EBC68..0x825EBCD4)
	// 825EBC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBC74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBC78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBC7C: 38EB5A68  addi r7, r11, 0x5a68
	ctx.r[7].s64 = ctx.r[11].s64 + 23144;
	// 825EBC80: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EBC84: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 825EBC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBC90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBC98: 386A9BF4  addi r3, r10, -0x640c
	ctx.r[3].s64 = ctx.r[10].s64 + -25612;
	// 825EBC9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBCAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBCB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBCBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBCC0: 4BE7B161  bl 0x82466e20
	ctx.lr = 0x825EBCC4;
	sub_82466E20(ctx, base);
	// 825EBCC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBCC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBCCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBCD8 size=112
    let mut pc: u32 = 0x825EBCD8;
    'dispatch: loop {
        match pc {
            0x825EBCD8 => {
    //   block [0x825EBCD8..0x825EBD48)
	// 825EBCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBCE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBCE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBCE8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBCEC: 38AA9BC4  addi r5, r10, -0x643c
	ctx.r[5].s64 = ctx.r[10].s64 + -25660;
	// 825EBCF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBCF4: 390B5AF8  addi r8, r11, 0x5af8
	ctx.r[8].s64 = ctx.r[11].s64 + 23288;
	// 825EBCF8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825EBCFC: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 825EBD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBD04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBD08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBD10: 386A9C24  addi r3, r10, -0x63dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25564;
	// 825EBD14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EBD18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBD1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBD34: 4BE7B0ED  bl 0x82466e20
	ctx.lr = 0x825EBD38;
	sub_82466E20(ctx, base);
	// 825EBD38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBD48 size=112
    let mut pc: u32 = 0x825EBD48;
    'dispatch: loop {
        match pc {
            0x825EBD48 => {
    //   block [0x825EBD48..0x825EBDB8)
	// 825EBD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBD54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBD58: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBD5C: 38AA9BC4  addi r5, r10, -0x643c
	ctx.r[5].s64 = ctx.r[10].s64 + -25660;
	// 825EBD60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBD64: 390B5C18  addi r8, r11, 0x5c18
	ctx.r[8].s64 = ctx.r[11].s64 + 23576;
	// 825EBD68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EBD6C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 825EBD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBD74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBD78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBD80: 386A9C54  addi r3, r10, -0x63ac
	ctx.r[3].s64 = ctx.r[10].s64 + -25516;
	// 825EBD84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EBD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBD90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBD98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBDA4: 4BE7B07D  bl 0x82466e20
	ctx.lr = 0x825EBDA8;
	sub_82466E20(ctx, base);
	// 825EBDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


