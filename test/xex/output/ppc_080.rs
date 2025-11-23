pub fn sub_8260BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BAF0 size=112
    let mut pc: u32 = 0x8260BAF0;
    'dispatch: loop {
        match pc {
            0x8260BAF0 => {
    //   block [0x8260BAF0..0x8260BB60)
	// 8260BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BAFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BB04: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BB0C: 390BFD7C  addi r8, r11, -0x284
	ctx.r[8].s64 = ctx.r[11].s64 + -644;
	// 8260BB10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260BB14: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8260BB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BB28: 386A71B4  addi r3, r10, 0x71b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29108;
	// 8260BB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BB4C: 4BE5B2D5  bl 0x82466e20
	ctx.lr = 0x8260BB50;
	sub_82466E20(ctx, base);
	// 8260BB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BB60 size=112
    let mut pc: u32 = 0x8260BB60;
    'dispatch: loop {
        match pc {
            0x8260BB60 => {
    //   block [0x8260BB60..0x8260BBD0)
	// 8260BB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BB6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BB74: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BB7C: 390BFD94  addi r8, r11, -0x26c
	ctx.r[8].s64 = ctx.r[11].s64 + -620;
	// 8260BB80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260BB84: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8260BB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BB98: 386A71E4  addi r3, r10, 0x71e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29156;
	// 8260BB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BBBC: 4BE5B265  bl 0x82466e20
	ctx.lr = 0x8260BBC0;
	sub_82466E20(ctx, base);
	// 8260BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BBD0 size=112
    let mut pc: u32 = 0x8260BBD0;
    'dispatch: loop {
        match pc {
            0x8260BBD0 => {
    //   block [0x8260BBD0..0x8260BC40)
	// 8260BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BBDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BBE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BBE4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BBEC: 390BFDB0  addi r8, r11, -0x250
	ctx.r[8].s64 = ctx.r[11].s64 + -592;
	// 8260BBF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260BBF4: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8260BBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BBFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BC08: 386A7214  addi r3, r10, 0x7214
	ctx.r[3].s64 = ctx.r[10].s64 + 29204;
	// 8260BC0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BC2C: 4BE5B1F5  bl 0x82466e20
	ctx.lr = 0x8260BC30;
	sub_82466E20(ctx, base);
	// 8260BC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BC40 size=112
    let mut pc: u32 = 0x8260BC40;
    'dispatch: loop {
        match pc {
            0x8260BC40 => {
    //   block [0x8260BC40..0x8260BCB0)
	// 8260BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BC4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BC50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BC54: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BC5C: 390BFDF8  addi r8, r11, -0x208
	ctx.r[8].s64 = ctx.r[11].s64 + -520;
	// 8260BC60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260BC64: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8260BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BC6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BC78: 386A7244  addi r3, r10, 0x7244
	ctx.r[3].s64 = ctx.r[10].s64 + 29252;
	// 8260BC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BC9C: 4BE5B185  bl 0x82466e20
	ctx.lr = 0x8260BCA0;
	sub_82466E20(ctx, base);
	// 8260BCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BCB0 size=112
    let mut pc: u32 = 0x8260BCB0;
    'dispatch: loop {
        match pc {
            0x8260BCB0 => {
    //   block [0x8260BCB0..0x8260BD20)
	// 8260BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BCBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BCC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BCC4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BCCC: 390BFE40  addi r8, r11, -0x1c0
	ctx.r[8].s64 = ctx.r[11].s64 + -448;
	// 8260BCD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260BCD4: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8260BCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BCDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BCE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BCE8: 386A7274  addi r3, r10, 0x7274
	ctx.r[3].s64 = ctx.r[10].s64 + 29300;
	// 8260BCEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BD0C: 4BE5B115  bl 0x82466e20
	ctx.lr = 0x8260BD10;
	sub_82466E20(ctx, base);
	// 8260BD10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BD20 size=112
    let mut pc: u32 = 0x8260BD20;
    'dispatch: loop {
        match pc {
            0x8260BD20 => {
    //   block [0x8260BD20..0x8260BD90)
	// 8260BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BD2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BD30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BD34: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BD38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BD3C: 390BFE58  addi r8, r11, -0x1a8
	ctx.r[8].s64 = ctx.r[11].s64 + -424;
	// 8260BD40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260BD44: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8260BD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BD4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BD58: 386A72A4  addi r3, r10, 0x72a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29348;
	// 8260BD5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BD7C: 4BE5B0A5  bl 0x82466e20
	ctx.lr = 0x8260BD80;
	sub_82466E20(ctx, base);
	// 8260BD80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BD90 size=116
    let mut pc: u32 = 0x8260BD90;
    'dispatch: loop {
        match pc {
            0x8260BD90 => {
    //   block [0x8260BD90..0x8260BE04)
	// 8260BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BD9C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260BDA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260BDA4: 390AFE88  addi r8, r10, -0x178
	ctx.r[8].s64 = ctx.r[10].s64 + -376;
	// 8260BDA8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BDAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260BDB0: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BDB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BDB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260BDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BDC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BDC4: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8260BDC8: 396BDD70  addi r11, r11, -0x2290
	ctx.r[11].s64 = ctx.r[11].s64 + -8848;
	// 8260BDCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BDD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BDD4: 386A72D4  addi r3, r10, 0x72d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29396;
	// 8260BDD8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260BDDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BDE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260BDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BDF0: 4BE5B031  bl 0x82466e20
	ctx.lr = 0x8260BDF4;
	sub_82466E20(ctx, base);
	// 8260BDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BE08 size=116
    let mut pc: u32 = 0x8260BE08;
    'dispatch: loop {
        match pc {
            0x8260BE08 => {
    //   block [0x8260BE08..0x8260BE7C)
	// 8260BE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BE14: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260BE18: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8260BE1C: 390AFF00  addi r8, r10, -0x100
	ctx.r[8].s64 = ctx.r[10].s64 + -256;
	// 8260BE20: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BE24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260BE28: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BE2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BE30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260BE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BE3C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8260BE40: 396BDD88  addi r11, r11, -0x2278
	ctx.r[11].s64 = ctx.r[11].s64 + -8824;
	// 8260BE44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BE48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BE4C: 386A7304  addi r3, r10, 0x7304
	ctx.r[3].s64 = ctx.r[10].s64 + 29444;
	// 8260BE50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260BE54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BE58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260BE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BE68: 4BE5AFB9  bl 0x82466e20
	ctx.lr = 0x8260BE6C;
	sub_82466E20(ctx, base);
	// 8260BE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260BE80 size=24
    let mut pc: u32 = 0x8260BE80;
    'dispatch: loop {
        match pc {
            0x8260BE80 => {
    //   block [0x8260BE80..0x8260BE98)
	// 8260BE80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BE84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260BE88: 394A3DB8  addi r10, r10, 0x3db8
	ctx.r[10].s64 = ctx.r[10].s64 + 15800;
	// 8260BE8C: 816BFDAC  lwz r11, -0x254(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-596 as u32) ) } as u64;
	// 8260BE90: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8260BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BE98 size=116
    let mut pc: u32 = 0x8260BE98;
    'dispatch: loop {
        match pc {
            0x8260BE98 => {
    //   block [0x8260BE98..0x8260BF0C)
	// 8260BE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BEA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260BEA8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BEAC: 392BDDB4  addi r9, r11, -0x224c
	ctx.r[9].s64 = ctx.r[11].s64 + -8780;
	// 8260BEB0: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BEB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BEB8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260BEBC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8260BEC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BEC4: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8260BEC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BECC: 396B3DB8  addi r11, r11, 0x3db8
	ctx.r[11].s64 = ctx.r[11].s64 + 15800;
	// 8260BED0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260BED4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BED8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260BEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BEE0: 386A7334  addi r3, r10, 0x7334
	ctx.r[3].s64 = ctx.r[10].s64 + 29492;
	// 8260BEE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260BEE8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260BEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BEF0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260BEF4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260BEF8: 4BE5AF29  bl 0x82466e20
	ctx.lr = 0x8260BEFC;
	sub_82466E20(ctx, base);
	// 8260BEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BF10 size=112
    let mut pc: u32 = 0x8260BF10;
    'dispatch: loop {
        match pc {
            0x8260BF10 => {
    //   block [0x8260BF10..0x8260BF80)
	// 8260BF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BF1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BF20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BF24: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BF2C: 390BFF90  addi r8, r11, -0x70
	ctx.r[8].s64 = ctx.r[11].s64 + -112;
	// 8260BF30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260BF34: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8260BF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BF3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BF48: 386A7364  addi r3, r10, 0x7364
	ctx.r[3].s64 = ctx.r[10].s64 + 29540;
	// 8260BF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BF6C: 4BE5AEB5  bl 0x82466e20
	ctx.lr = 0x8260BF70;
	sub_82466E20(ctx, base);
	// 8260BF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BF80 size=112
    let mut pc: u32 = 0x8260BF80;
    'dispatch: loop {
        match pc {
            0x8260BF80 => {
    //   block [0x8260BF80..0x8260BFF0)
	// 8260BF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BF8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BF90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BF94: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BF9C: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 8260BFA0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8260BFA4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8260BFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BFAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BFB8: 386A7394  addi r3, r10, 0x7394
	ctx.r[3].s64 = ctx.r[10].s64 + 29588;
	// 8260BFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BFDC: 4BE5AE45  bl 0x82466e20
	ctx.lr = 0x8260BFE0;
	sub_82466E20(ctx, base);
	// 8260BFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BFF0 size=112
    let mut pc: u32 = 0x8260BFF0;
    'dispatch: loop {
        match pc {
            0x8260BFF0 => {
    //   block [0x8260BFF0..0x8260C060)
	// 8260BFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BFFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C000: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C004: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C00C: 390B0098  addi r8, r11, 0x98
	ctx.r[8].s64 = ctx.r[11].s64 + 152;
	// 8260C010: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260C014: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8260C018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C028: 386A73C4  addi r3, r10, 0x73c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29636;
	// 8260C02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C04C: 4BE5ADD5  bl 0x82466e20
	ctx.lr = 0x8260C050;
	sub_82466E20(ctx, base);
	// 8260C050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C060 size=112
    let mut pc: u32 = 0x8260C060;
    'dispatch: loop {
        match pc {
            0x8260C060 => {
    //   block [0x8260C060..0x8260C0D0)
	// 8260C060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C06C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C070: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C074: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C07C: 390B0110  addi r8, r11, 0x110
	ctx.r[8].s64 = ctx.r[11].s64 + 272;
	// 8260C080: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260C084: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8260C088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C098: 386A73F4  addi r3, r10, 0x73f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29684;
	// 8260C09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C0BC: 4BE5AD65  bl 0x82466e20
	ctx.lr = 0x8260C0C0;
	sub_82466E20(ctx, base);
	// 8260C0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C0D0 size=112
    let mut pc: u32 = 0x8260C0D0;
    'dispatch: loop {
        match pc {
            0x8260C0D0 => {
    //   block [0x8260C0D0..0x8260C140)
	// 8260C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C0DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C0E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C0E4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C0EC: 390B0158  addi r8, r11, 0x158
	ctx.r[8].s64 = ctx.r[11].s64 + 344;
	// 8260C0F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260C0F4: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8260C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C108: 386A7424  addi r3, r10, 0x7424
	ctx.r[3].s64 = ctx.r[10].s64 + 29732;
	// 8260C10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C12C: 4BE5ACF5  bl 0x82466e20
	ctx.lr = 0x8260C130;
	sub_82466E20(ctx, base);
	// 8260C130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C140 size=112
    let mut pc: u32 = 0x8260C140;
    'dispatch: loop {
        match pc {
            0x8260C140 => {
    //   block [0x8260C140..0x8260C1B0)
	// 8260C140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C14C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C150: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C154: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C15C: 390B01E8  addi r8, r11, 0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + 488;
	// 8260C160: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260C164: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8260C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C178: 386A7454  addi r3, r10, 0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + 29780;
	// 8260C17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C19C: 4BE5AC85  bl 0x82466e20
	ctx.lr = 0x8260C1A0;
	sub_82466E20(ctx, base);
	// 8260C1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C1B0 size=112
    let mut pc: u32 = 0x8260C1B0;
    'dispatch: loop {
        match pc {
            0x8260C1B0 => {
    //   block [0x8260C1B0..0x8260C220)
	// 8260C1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C1BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C1C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C1C4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C1CC: 390B0248  addi r8, r11, 0x248
	ctx.r[8].s64 = ctx.r[11].s64 + 584;
	// 8260C1D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260C1D4: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8260C1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C1DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C1E8: 386A7484  addi r3, r10, 0x7484
	ctx.r[3].s64 = ctx.r[10].s64 + 29828;
	// 8260C1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C20C: 4BE5AC15  bl 0x82466e20
	ctx.lr = 0x8260C210;
	sub_82466E20(ctx, base);
	// 8260C210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C220 size=112
    let mut pc: u32 = 0x8260C220;
    'dispatch: loop {
        match pc {
            0x8260C220 => {
    //   block [0x8260C220..0x8260C290)
	// 8260C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C22C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C230: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C234: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C23C: 390B02A8  addi r8, r11, 0x2a8
	ctx.r[8].s64 = ctx.r[11].s64 + 680;
	// 8260C240: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260C244: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8260C248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C24C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C258: 386A74B4  addi r3, r10, 0x74b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29876;
	// 8260C25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C27C: 4BE5ABA5  bl 0x82466e20
	ctx.lr = 0x8260C280;
	sub_82466E20(ctx, base);
	// 8260C280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C290 size=112
    let mut pc: u32 = 0x8260C290;
    'dispatch: loop {
        match pc {
            0x8260C290 => {
    //   block [0x8260C290..0x8260C300)
	// 8260C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C29C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C2A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C2A4: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C2AC: 390B02D8  addi r8, r11, 0x2d8
	ctx.r[8].s64 = ctx.r[11].s64 + 728;
	// 8260C2B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260C2B4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8260C2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C2BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C2C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C2C8: 386A74E4  addi r3, r10, 0x74e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29924;
	// 8260C2CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C2EC: 4BE5AB35  bl 0x82466e20
	ctx.lr = 0x8260C2F0;
	sub_82466E20(ctx, base);
	// 8260C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C300 size=100
    let mut pc: u32 = 0x8260C300;
    'dispatch: loop {
        match pc {
            0x8260C300 => {
    //   block [0x8260C300..0x8260C364)
	// 8260C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C30C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C314: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C320: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8260C324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C334: 386A7514  addi r3, r10, 0x7514
	ctx.r[3].s64 = ctx.r[10].s64 + 29972;
	// 8260C338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C33C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C350: 4BE5AAD1  bl 0x82466e20
	ctx.lr = 0x8260C354;
	sub_82466E20(ctx, base);
	// 8260C354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C368 size=112
    let mut pc: u32 = 0x8260C368;
    'dispatch: loop {
        match pc {
            0x8260C368 => {
    //   block [0x8260C368..0x8260C3D8)
	// 8260C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C378: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C37C: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C384: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 8260C388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C38C: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8260C390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C394: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C3A0: 386A7544  addi r3, r10, 0x7544
	ctx.r[3].s64 = ctx.r[10].s64 + 30020;
	// 8260C3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C3C4: 4BE5AA5D  bl 0x82466e20
	ctx.lr = 0x8260C3C8;
	sub_82466E20(ctx, base);
	// 8260C3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C3D8 size=112
    let mut pc: u32 = 0x8260C3D8;
    'dispatch: loop {
        match pc {
            0x8260C3D8 => {
    //   block [0x8260C3D8..0x8260C448)
	// 8260C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C3E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C3E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C3EC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260C3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C3F4: 390B0320  addi r8, r11, 0x320
	ctx.r[8].s64 = ctx.r[11].s64 + 800;
	// 8260C3F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260C3FC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8260C400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C404: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C410: 386A7574  addi r3, r10, 0x7574
	ctx.r[3].s64 = ctx.r[10].s64 + 30068;
	// 8260C414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C434: 4BE5A9ED  bl 0x82466e20
	ctx.lr = 0x8260C438;
	sub_82466E20(ctx, base);
	// 8260C438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C448 size=112
    let mut pc: u32 = 0x8260C448;
    'dispatch: loop {
        match pc {
            0x8260C448 => {
    //   block [0x8260C448..0x8260C4B8)
	// 8260C448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C454: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C458: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C45C: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260C460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C464: 390B0380  addi r8, r11, 0x380
	ctx.r[8].s64 = ctx.r[11].s64 + 896;
	// 8260C468: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C46C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8260C470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C474: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C480: 386A75A4  addi r3, r10, 0x75a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30116;
	// 8260C484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C4A4: 4BE5A97D  bl 0x82466e20
	ctx.lr = 0x8260C4A8;
	sub_82466E20(ctx, base);
	// 8260C4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C4B8 size=112
    let mut pc: u32 = 0x8260C4B8;
    'dispatch: loop {
        match pc {
            0x8260C4B8 => {
    //   block [0x8260C4B8..0x8260C528)
	// 8260C4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C4C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C4CC: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260C4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C4D4: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 8260C4D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260C4DC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8260C4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C4F0: 386A75D4  addi r3, r10, 0x75d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30164;
	// 8260C4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C514: 4BE5A90D  bl 0x82466e20
	ctx.lr = 0x8260C518;
	sub_82466E20(ctx, base);
	// 8260C518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C528 size=112
    let mut pc: u32 = 0x8260C528;
    'dispatch: loop {
        match pc {
            0x8260C528 => {
    //   block [0x8260C528..0x8260C598)
	// 8260C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C538: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C53C: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260C540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C544: 390B03C8  addi r8, r11, 0x3c8
	ctx.r[8].s64 = ctx.r[11].s64 + 968;
	// 8260C548: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C54C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8260C550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C554: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C560: 386A7604  addi r3, r10, 0x7604
	ctx.r[3].s64 = ctx.r[10].s64 + 30212;
	// 8260C564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C584: 4BE5A89D  bl 0x82466e20
	ctx.lr = 0x8260C588;
	sub_82466E20(ctx, base);
	// 8260C588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260C598 size=24
    let mut pc: u32 = 0x8260C598;
    'dispatch: loop {
        match pc {
            0x8260C598 => {
    //   block [0x8260C598..0x8260C5B0)
	// 8260C598: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C59C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260C5A0: 394A3E60  addi r10, r10, 0x3e60
	ctx.r[10].s64 = ctx.r[10].s64 + 15968;
	// 8260C5A4: 816B03E0  lwz r11, 0x3e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(992 as u32) ) } as u64;
	// 8260C5A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260C5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C5B0 size=112
    let mut pc: u32 = 0x8260C5B0;
    'dispatch: loop {
        match pc {
            0x8260C5B0 => {
    //   block [0x8260C5B0..0x8260C620)
	// 8260C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C5BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C5C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C5C4: 392ADE10  addi r9, r10, -0x21f0
	ctx.r[9].s64 = ctx.r[10].s64 + -8688;
	// 8260C5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C5CC: 390B3E60  addi r8, r11, 0x3e60
	ctx.r[8].s64 = ctx.r[11].s64 + 15968;
	// 8260C5D0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260C5D4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8260C5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C5DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C5E8: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 8260C5EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C5F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260C5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C60C: 4BE5A815  bl 0x82466e20
	ctx.lr = 0x8260C610;
	sub_82466E20(ctx, base);
	// 8260C610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C620 size=108
    let mut pc: u32 = 0x8260C620;
    'dispatch: loop {
        match pc {
            0x8260C620 => {
    //   block [0x8260C620..0x8260C68C)
	// 8260C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C62C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C634: 38EB03E4  addi r7, r11, 0x3e4
	ctx.r[7].s64 = ctx.r[11].s64 + 996;
	// 8260C638: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260C63C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8260C640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260C64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C650: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 8260C654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260C658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C678: 4BE5A7A9  bl 0x82466e20
	ctx.lr = 0x8260C67C;
	sub_82466E20(ctx, base);
	// 8260C67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C690 size=108
    let mut pc: u32 = 0x8260C690;
    'dispatch: loop {
        match pc {
            0x8260C690 => {
    //   block [0x8260C690..0x8260C6FC)
	// 8260C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C69C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C6A4: 38EB0400  addi r7, r11, 0x400
	ctx.r[7].s64 = ctx.r[11].s64 + 1024;
	// 8260C6A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260C6AC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8260C6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C6B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C6B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260C6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C6C0: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 8260C6C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260C6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C6E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C6E8: 4BE5A739  bl 0x82466e20
	ctx.lr = 0x8260C6EC;
	sub_82466E20(ctx, base);
	// 8260C6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C700 size=116
    let mut pc: u32 = 0x8260C700;
    'dispatch: loop {
        match pc {
            0x8260C700 => {
    //   block [0x8260C700..0x8260C774)
	// 8260C700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C70C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C710: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C714: 390B0448  addi r8, r11, 0x448
	ctx.r[8].s64 = ctx.r[11].s64 + 1096;
	// 8260C718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C71C: 392ADEC8  addi r9, r10, -0x2138
	ctx.r[9].s64 = ctx.r[10].s64 + -8504;
	// 8260C720: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C724: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260C728: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260C72C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C734: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C744: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260C748: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8260C74C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C750: 386B76C4  addi r3, r11, 0x76c4
	ctx.r[3].s64 = ctx.r[11].s64 + 30404;
	// 8260C754: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260C758: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C760: 4BE5A6C1  bl 0x82466e20
	ctx.lr = 0x8260C764;
	sub_82466E20(ctx, base);
	// 8260C764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260C778 size=24
    let mut pc: u32 = 0x8260C778;
    'dispatch: loop {
        match pc {
            0x8260C778 => {
    //   block [0x8260C778..0x8260C790)
	// 8260C778: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C77C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260C780: 394A3EA8  addi r10, r10, 0x3ea8
	ctx.r[10].s64 = ctx.r[10].s64 + 16040;
	// 8260C784: 816B0460  lwz r11, 0x460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1120 as u32) ) } as u64;
	// 8260C788: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260C78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C790 size=116
    let mut pc: u32 = 0x8260C790;
    'dispatch: loop {
        match pc {
            0x8260C790 => {
    //   block [0x8260C790..0x8260C804)
	// 8260C790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C79C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C7A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C7A4: 390B3EA8  addi r8, r11, 0x3ea8
	ctx.r[8].s64 = ctx.r[11].s64 + 16040;
	// 8260C7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C7AC: 392ADF24  addi r9, r10, -0x20dc
	ctx.r[9].s64 = ctx.r[10].s64 + -8412;
	// 8260C7B0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C7B4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8260C7B8: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260C7BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C7C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C7D4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260C7D8: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8260C7DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C7E0: 386B76F4  addi r3, r11, 0x76f4
	ctx.r[3].s64 = ctx.r[11].s64 + 30452;
	// 8260C7E4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8260C7E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C7F0: 4BE5A631  bl 0x82466e20
	ctx.lr = 0x8260C7F4;
	sub_82466E20(ctx, base);
	// 8260C7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C808 size=108
    let mut pc: u32 = 0x8260C808;
    'dispatch: loop {
        match pc {
            0x8260C808 => {
    //   block [0x8260C808..0x8260C874)
	// 8260C808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C814: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C81C: 38EB046C  addi r7, r11, 0x46c
	ctx.r[7].s64 = ctx.r[11].s64 + 1132;
	// 8260C820: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260C824: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8260C828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260C834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C838: 386A7724  addi r3, r10, 0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + 30500;
	// 8260C83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260C840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C860: 4BE5A5C1  bl 0x82466e20
	ctx.lr = 0x8260C864;
	sub_82466E20(ctx, base);
	// 8260C864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C878 size=112
    let mut pc: u32 = 0x8260C878;
    'dispatch: loop {
        match pc {
            0x8260C878 => {
    //   block [0x8260C878..0x8260C8E8)
	// 8260C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C888: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C88C: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260C890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C894: 390B049C  addi r8, r11, 0x49c
	ctx.r[8].s64 = ctx.r[11].s64 + 1180;
	// 8260C898: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C89C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8260C8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C8A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C8B0: 386A7754  addi r3, r10, 0x7754
	ctx.r[3].s64 = ctx.r[10].s64 + 30548;
	// 8260C8B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C8C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C8C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C8D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C8D4: 4BE5A54D  bl 0x82466e20
	ctx.lr = 0x8260C8D8;
	sub_82466E20(ctx, base);
	// 8260C8D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C8E8 size=112
    let mut pc: u32 = 0x8260C8E8;
    'dispatch: loop {
        match pc {
            0x8260C8E8 => {
    //   block [0x8260C8E8..0x8260C958)
	// 8260C8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C8F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C8F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C8FC: 392ADF68  addi r9, r10, -0x2098
	ctx.r[9].s64 = ctx.r[10].s64 + -8344;
	// 8260C900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C904: 390B04B8  addi r8, r11, 0x4b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1208;
	// 8260C908: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260C90C: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8260C910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C914: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C920: 386A7784  addi r3, r10, 0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + 30596;
	// 8260C924: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C928: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260C92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C93C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C944: 4BE5A4DD  bl 0x82466e20
	ctx.lr = 0x8260C948;
	sub_82466E20(ctx, base);
	// 8260C948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C958 size=112
    let mut pc: u32 = 0x8260C958;
    'dispatch: loop {
        match pc {
            0x8260C958 => {
    //   block [0x8260C958..0x8260C9C8)
	// 8260C958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C964: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C968: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C96C: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260C970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C974: 390B0500  addi r8, r11, 0x500
	ctx.r[8].s64 = ctx.r[11].s64 + 1280;
	// 8260C978: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C97C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8260C980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C990: 386A77B4  addi r3, r10, 0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30644;
	// 8260C994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C9B4: 4BE5A46D  bl 0x82466e20
	ctx.lr = 0x8260C9B8;
	sub_82466E20(ctx, base);
	// 8260C9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C9C8 size=112
    let mut pc: u32 = 0x8260C9C8;
    'dispatch: loop {
        match pc {
            0x8260C9C8 => {
    //   block [0x8260C9C8..0x8260CA38)
	// 8260C9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C9D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C9D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C9DC: 392ADF94  addi r9, r10, -0x206c
	ctx.r[9].s64 = ctx.r[10].s64 + -8300;
	// 8260C9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C9E4: 390B0520  addi r8, r11, 0x520
	ctx.r[8].s64 = ctx.r[11].s64 + 1312;
	// 8260C9E8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8260C9EC: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8260C9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C9F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CA00: 386A77E4  addi r3, r10, 0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30692;
	// 8260CA04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CA08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CA24: 4BE5A3FD  bl 0x82466e20
	ctx.lr = 0x8260CA28;
	sub_82466E20(ctx, base);
	// 8260CA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CA38 size=112
    let mut pc: u32 = 0x8260CA38;
    'dispatch: loop {
        match pc {
            0x8260CA38 => {
    //   block [0x8260CA38..0x8260CAA8)
	// 8260CA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CA44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CA48: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CA4C: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CA50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CA54: 390B05B0  addi r8, r11, 0x5b0
	ctx.r[8].s64 = ctx.r[11].s64 + 1456;
	// 8260CA58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CA5C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8260CA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CA64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CA68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CA70: 386A7814  addi r3, r10, 0x7814
	ctx.r[3].s64 = ctx.r[10].s64 + 30740;
	// 8260CA74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CA94: 4BE5A38D  bl 0x82466e20
	ctx.lr = 0x8260CA98;
	sub_82466E20(ctx, base);
	// 8260CA98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CAA8 size=112
    let mut pc: u32 = 0x8260CAA8;
    'dispatch: loop {
        match pc {
            0x8260CAA8 => {
    //   block [0x8260CAA8..0x8260CB18)
	// 8260CAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CAB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CABC: 38AA7874  addi r5, r10, 0x7874
	ctx.r[5].s64 = ctx.r[10].s64 + 30836;
	// 8260CAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CAC4: 390B05C8  addi r8, r11, 0x5c8
	ctx.r[8].s64 = ctx.r[11].s64 + 1480;
	// 8260CAC8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260CACC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8260CAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CAD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CAE0: 386A7844  addi r3, r10, 0x7844
	ctx.r[3].s64 = ctx.r[10].s64 + 30788;
	// 8260CAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CB04: 4BE5A31D  bl 0x82466e20
	ctx.lr = 0x8260CB08;
	sub_82466E20(ctx, base);
	// 8260CB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CB18 size=100
    let mut pc: u32 = 0x8260CB18;
    'dispatch: loop {
        match pc {
            0x8260CB18 => {
    //   block [0x8260CB18..0x8260CB7C)
	// 8260CB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CB24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CB2C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260CB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CB38: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8260CB3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CB4C: 386A7874  addi r3, r10, 0x7874
	ctx.r[3].s64 = ctx.r[10].s64 + 30836;
	// 8260CB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CB58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260CB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CB60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260CB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CB68: 4BE5A2B9  bl 0x82466e20
	ctx.lr = 0x8260CB6C;
	sub_82466E20(ctx, base);
	// 8260CB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260CB80 size=24
    let mut pc: u32 = 0x8260CB80;
    'dispatch: loop {
        match pc {
            0x8260CB80 => {
    //   block [0x8260CB80..0x8260CB98)
	// 8260CB80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CB84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260CB88: 394A3F80  addi r10, r10, 0x3f80
	ctx.r[10].s64 = ctx.r[10].s64 + 16256;
	// 8260CB8C: 816B051C  lwz r11, 0x51c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1308 as u32) ) } as u64;
	// 8260CB90: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8260CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CB98 size=116
    let mut pc: u32 = 0x8260CB98;
    'dispatch: loop {
        match pc {
            0x8260CB98 => {
    //   block [0x8260CB98..0x8260CC0C)
	// 8260CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CBA4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CBA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260CBAC: 390B3F80  addi r8, r11, 0x3f80
	ctx.r[8].s64 = ctx.r[11].s64 + 16256;
	// 8260CBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CBB4: 392ADFD0  addi r9, r10, -0x2030
	ctx.r[9].s64 = ctx.r[10].s64 + -8240;
	// 8260CBB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CBBC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260CBC0: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CBC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CBC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CBCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CBD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CBD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CBDC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260CBE0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8260CBE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CBE8: 386B78A4  addi r3, r11, 0x78a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30884;
	// 8260CBEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CBF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CBF8: 4BE5A229  bl 0x82466e20
	ctx.lr = 0x8260CBFC;
	sub_82466E20(ctx, base);
	// 8260CBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CC10 size=108
    let mut pc: u32 = 0x8260CC10;
    'dispatch: loop {
        match pc {
            0x8260CC10 => {
    //   block [0x8260CC10..0x8260CC7C)
	// 8260CC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CC1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CC24: 38EB0640  addi r7, r11, 0x640
	ctx.r[7].s64 = ctx.r[11].s64 + 1600;
	// 8260CC28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260CC2C: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8260CC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260CC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CC40: 386A78D4  addi r3, r10, 0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30932;
	// 8260CC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260CC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CC68: 4BE5A1B9  bl 0x82466e20
	ctx.lr = 0x8260CC6C;
	sub_82466E20(ctx, base);
	// 8260CC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CC80 size=112
    let mut pc: u32 = 0x8260CC80;
    'dispatch: loop {
        match pc {
            0x8260CC80 => {
    //   block [0x8260CC80..0x8260CCF0)
	// 8260CC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CC90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CC94: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CC98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CC9C: 390B0670  addi r8, r11, 0x670
	ctx.r[8].s64 = ctx.r[11].s64 + 1648;
	// 8260CCA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CCA4: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8260CCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CCAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CCB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CCB8: 386A7904  addi r3, r10, 0x7904
	ctx.r[3].s64 = ctx.r[10].s64 + 30980;
	// 8260CCBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CCC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CCC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CCD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CCD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CCDC: 4BE5A145  bl 0x82466e20
	ctx.lr = 0x8260CCE0;
	sub_82466E20(ctx, base);
	// 8260CCE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CCF0 size=112
    let mut pc: u32 = 0x8260CCF0;
    'dispatch: loop {
        match pc {
            0x8260CCF0 => {
    //   block [0x8260CCF0..0x8260CD60)
	// 8260CCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CCFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260CD00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CD04: 392ADFF4  addi r9, r10, -0x200c
	ctx.r[9].s64 = ctx.r[10].s64 + -8204;
	// 8260CD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CD0C: 390B0690  addi r8, r11, 0x690
	ctx.r[8].s64 = ctx.r[11].s64 + 1680;
	// 8260CD10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260CD14: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8260CD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CD1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CD20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CD28: 386A7934  addi r3, r10, 0x7934
	ctx.r[3].s64 = ctx.r[10].s64 + 31028;
	// 8260CD2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CD30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CD44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CD4C: 4BE5A0D5  bl 0x82466e20
	ctx.lr = 0x8260CD50;
	sub_82466E20(ctx, base);
	// 8260CD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CD60 size=112
    let mut pc: u32 = 0x8260CD60;
    'dispatch: loop {
        match pc {
            0x8260CD60 => {
    //   block [0x8260CD60..0x8260CDD0)
	// 8260CD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CD6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CD70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CD74: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CD78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CD7C: 390B0738  addi r8, r11, 0x738
	ctx.r[8].s64 = ctx.r[11].s64 + 1848;
	// 8260CD80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CD84: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8260CD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CD8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CD90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CD98: 386A7964  addi r3, r10, 0x7964
	ctx.r[3].s64 = ctx.r[10].s64 + 31076;
	// 8260CD9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CDA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CDA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CDBC: 4BE5A065  bl 0x82466e20
	ctx.lr = 0x8260CDC0;
	sub_82466E20(ctx, base);
	// 8260CDC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CDD0 size=108
    let mut pc: u32 = 0x8260CDD0;
    'dispatch: loop {
        match pc {
            0x8260CDD0 => {
    //   block [0x8260CDD0..0x8260CE3C)
	// 8260CDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CDDC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CDE4: 38EB0750  addi r7, r11, 0x750
	ctx.r[7].s64 = ctx.r[11].s64 + 1872;
	// 8260CDE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260CDEC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8260CDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CDF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260CDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CE00: 386A7994  addi r3, r10, 0x7994
	ctx.r[3].s64 = ctx.r[10].s64 + 31124;
	// 8260CE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260CE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CE28: 4BE59FF9  bl 0x82466e20
	ctx.lr = 0x8260CE2C;
	sub_82466E20(ctx, base);
	// 8260CE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CE40 size=112
    let mut pc: u32 = 0x8260CE40;
    'dispatch: loop {
        match pc {
            0x8260CE40 => {
    //   block [0x8260CE40..0x8260CEB0)
	// 8260CE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CE4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CE50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CE54: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CE5C: 390B0780  addi r8, r11, 0x780
	ctx.r[8].s64 = ctx.r[11].s64 + 1920;
	// 8260CE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CE64: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8260CE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CE6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CE78: 386A79C4  addi r3, r10, 0x79c4
	ctx.r[3].s64 = ctx.r[10].s64 + 31172;
	// 8260CE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CE9C: 4BE59F85  bl 0x82466e20
	ctx.lr = 0x8260CEA0;
	sub_82466E20(ctx, base);
	// 8260CEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CEB0 size=112
    let mut pc: u32 = 0x8260CEB0;
    'dispatch: loop {
        match pc {
            0x8260CEB0 => {
    //   block [0x8260CEB0..0x8260CF20)
	// 8260CEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CEBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260CEC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CEC4: 392AE028  addi r9, r10, -0x1fd8
	ctx.r[9].s64 = ctx.r[10].s64 + -8152;
	// 8260CEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CECC: 390B0798  addi r8, r11, 0x798
	ctx.r[8].s64 = ctx.r[11].s64 + 1944;
	// 8260CED0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260CED4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8260CED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CEDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CEE8: 386A79F4  addi r3, r10, 0x79f4
	ctx.r[3].s64 = ctx.r[10].s64 + 31220;
	// 8260CEEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CEF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CF0C: 4BE59F15  bl 0x82466e20
	ctx.lr = 0x8260CF10;
	sub_82466E20(ctx, base);
	// 8260CF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CF20 size=112
    let mut pc: u32 = 0x8260CF20;
    'dispatch: loop {
        match pc {
            0x8260CF20 => {
    //   block [0x8260CF20..0x8260CF90)
	// 8260CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CF2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CF30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CF34: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CF3C: 390B0840  addi r8, r11, 0x840
	ctx.r[8].s64 = ctx.r[11].s64 + 2112;
	// 8260CF40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260CF44: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8260CF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CF4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CF58: 386A7A24  addi r3, r10, 0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + 31268;
	// 8260CF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CF7C: 4BE59EA5  bl 0x82466e20
	ctx.lr = 0x8260CF80;
	sub_82466E20(ctx, base);
	// 8260CF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CF90 size=112
    let mut pc: u32 = 0x8260CF90;
    'dispatch: loop {
        match pc {
            0x8260CF90 => {
    //   block [0x8260CF90..0x8260D000)
	// 8260CF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CF9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CFA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CFA4: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CFAC: 390B0888  addi r8, r11, 0x888
	ctx.r[8].s64 = ctx.r[11].s64 + 2184;
	// 8260CFB0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8260CFB4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8260CFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CFBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CFC8: 386A7A54  addi r3, r10, 0x7a54
	ctx.r[3].s64 = ctx.r[10].s64 + 31316;
	// 8260CFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CFEC: 4BE59E35  bl 0x82466e20
	ctx.lr = 0x8260CFF0;
	sub_82466E20(ctx, base);
	// 8260CFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D000 size=100
    let mut pc: u32 = 0x8260D000;
    'dispatch: loop {
        match pc {
            0x8260D000 => {
    //   block [0x8260D000..0x8260D064)
	// 8260D000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D00C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D014: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D020: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8260D024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D034: 386A7A84  addi r3, r10, 0x7a84
	ctx.r[3].s64 = ctx.r[10].s64 + 31364;
	// 8260D038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D03C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260D044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260D04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D050: 4BE59DD1  bl 0x82466e20
	ctx.lr = 0x8260D054;
	sub_82466E20(ctx, base);
	// 8260D054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D068 size=112
    let mut pc: u32 = 0x8260D068;
    'dispatch: loop {
        match pc {
            0x8260D068 => {
    //   block [0x8260D068..0x8260D0D8)
	// 8260D068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D074: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D078: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D07C: 38AA76F4  addi r5, r10, 0x76f4
	ctx.r[5].s64 = ctx.r[10].s64 + 30452;
	// 8260D080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D084: 390B0948  addi r8, r11, 0x948
	ctx.r[8].s64 = ctx.r[11].s64 + 2376;
	// 8260D088: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D08C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8260D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D0A0: 386A7AB4  addi r3, r10, 0x7ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 31412;
	// 8260D0A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D0C4: 4BE59D5D  bl 0x82466e20
	ctx.lr = 0x8260D0C8;
	sub_82466E20(ctx, base);
	// 8260D0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D0D8 size=112
    let mut pc: u32 = 0x8260D0D8;
    'dispatch: loop {
        match pc {
            0x8260D0D8 => {
    //   block [0x8260D0D8..0x8260D148)
	// 8260D0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D0E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D0E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D0EC: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260D0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D0F4: 390B0978  addi r8, r11, 0x978
	ctx.r[8].s64 = ctx.r[11].s64 + 2424;
	// 8260D0F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260D0FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8260D100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D110: 386A7AE4  addi r3, r10, 0x7ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 31460;
	// 8260D114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D134: 4BE59CED  bl 0x82466e20
	ctx.lr = 0x8260D138;
	sub_82466E20(ctx, base);
	// 8260D138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D148 size=108
    let mut pc: u32 = 0x8260D148;
    'dispatch: loop {
        match pc {
            0x8260D148 => {
    //   block [0x8260D148..0x8260D1B4)
	// 8260D148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D154: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D15C: 38EB0990  addi r7, r11, 0x990
	ctx.r[7].s64 = ctx.r[11].s64 + 2448;
	// 8260D160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260D164: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8260D168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D178: 386A7B14  addi r3, r10, 0x7b14
	ctx.r[3].s64 = ctx.r[10].s64 + 31508;
	// 8260D17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D1A0: 4BE59C81  bl 0x82466e20
	ctx.lr = 0x8260D1A4;
	sub_82466E20(ctx, base);
	// 8260D1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D1B8 size=112
    let mut pc: u32 = 0x8260D1B8;
    'dispatch: loop {
        match pc {
            0x8260D1B8 => {
    //   block [0x8260D1B8..0x8260D228)
	// 8260D1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D1C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D1C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D1CC: 38AA7A84  addi r5, r10, 0x7a84
	ctx.r[5].s64 = ctx.r[10].s64 + 31364;
	// 8260D1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D1D4: 390B09C0  addi r8, r11, 0x9c0
	ctx.r[8].s64 = ctx.r[11].s64 + 2496;
	// 8260D1D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260D1DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8260D1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D1E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D1F0: 386A7B44  addi r3, r10, 0x7b44
	ctx.r[3].s64 = ctx.r[10].s64 + 31556;
	// 8260D1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D214: 4BE59C0D  bl 0x82466e20
	ctx.lr = 0x8260D218;
	sub_82466E20(ctx, base);
	// 8260D218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D228 size=112
    let mut pc: u32 = 0x8260D228;
    'dispatch: loop {
        match pc {
            0x8260D228 => {
    //   block [0x8260D228..0x8260D298)
	// 8260D228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D234: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260D238: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D23C: 392AE054  addi r9, r10, -0x1fac
	ctx.r[9].s64 = ctx.r[10].s64 + -8108;
	// 8260D240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D244: 390B0A58  addi r8, r11, 0xa58
	ctx.r[8].s64 = ctx.r[11].s64 + 2648;
	// 8260D248: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260D24C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8260D250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D260: 386A7B74  addi r3, r10, 0x7b74
	ctx.r[3].s64 = ctx.r[10].s64 + 31604;
	// 8260D264: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D268: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260D26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D27C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D284: 4BE59B9D  bl 0x82466e20
	ctx.lr = 0x8260D288;
	sub_82466E20(ctx, base);
	// 8260D288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D298 size=112
    let mut pc: u32 = 0x8260D298;
    'dispatch: loop {
        match pc {
            0x8260D298 => {
    //   block [0x8260D298..0x8260D308)
	// 8260D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D2A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D2A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D2AC: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D2B4: 390B0AA0  addi r8, r11, 0xaa0
	ctx.r[8].s64 = ctx.r[11].s64 + 2720;
	// 8260D2B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260D2BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8260D2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D2C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D2D0: 386A7BA4  addi r3, r10, 0x7ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 31652;
	// 8260D2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D2F4: 4BE59B2D  bl 0x82466e20
	ctx.lr = 0x8260D2F8;
	sub_82466E20(ctx, base);
	// 8260D2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D308 size=108
    let mut pc: u32 = 0x8260D308;
    'dispatch: loop {
        match pc {
            0x8260D308 => {
    //   block [0x8260D308..0x8260D374)
	// 8260D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D314: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D31C: 38EB0AB8  addi r7, r11, 0xab8
	ctx.r[7].s64 = ctx.r[11].s64 + 2744;
	// 8260D320: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8260D324: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8260D328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D32C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D338: 386A7BD4  addi r3, r10, 0x7bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31700;
	// 8260D33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D360: 4BE59AC1  bl 0x82466e20
	ctx.lr = 0x8260D364;
	sub_82466E20(ctx, base);
	// 8260D364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D378 size=116
    let mut pc: u32 = 0x8260D378;
    'dispatch: loop {
        match pc {
            0x8260D378 => {
    //   block [0x8260D378..0x8260D3EC)
	// 8260D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D384: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260D388: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8260D38C: 390A0B48  addi r8, r10, 0xb48
	ctx.r[8].s64 = ctx.r[10].s64 + 2888;
	// 8260D390: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D394: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260D398: 38AA7A84  addi r5, r10, 0x7a84
	ctx.r[5].s64 = ctx.r[10].s64 + 31364;
	// 8260D39C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D3A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D3A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D3AC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8260D3B0: 396BE068  addi r11, r11, -0x1f98
	ctx.r[11].s64 = ctx.r[11].s64 + -8088;
	// 8260D3B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D3B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D3BC: 386A7C04  addi r3, r10, 0x7c04
	ctx.r[3].s64 = ctx.r[10].s64 + 31748;
	// 8260D3C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260D3C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D3C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260D3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D3D8: 4BE59A49  bl 0x82466e20
	ctx.lr = 0x8260D3DC;
	sub_82466E20(ctx, base);
	// 8260D3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D3F0 size=108
    let mut pc: u32 = 0x8260D3F0;
    'dispatch: loop {
        match pc {
            0x8260D3F0 => {
    //   block [0x8260D3F0..0x8260D45C)
	// 8260D3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D3FC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D404: 38EB0C20  addi r7, r11, 0xc20
	ctx.r[7].s64 = ctx.r[11].s64 + 3104;
	// 8260D408: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260D40C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8260D410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D414: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D420: 386A7C34  addi r3, r10, 0x7c34
	ctx.r[3].s64 = ctx.r[10].s64 + 31796;
	// 8260D424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D448: 4BE599D9  bl 0x82466e20
	ctx.lr = 0x8260D44C;
	sub_82466E20(ctx, base);
	// 8260D44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D460 size=112
    let mut pc: u32 = 0x8260D460;
    'dispatch: loop {
        match pc {
            0x8260D460 => {
    //   block [0x8260D460..0x8260D4D0)
	// 8260D460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D46C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D470: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D474: 38AA7A84  addi r5, r10, 0x7a84
	ctx.r[5].s64 = ctx.r[10].s64 + 31364;
	// 8260D478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D47C: 390B0C68  addi r8, r11, 0xc68
	ctx.r[8].s64 = ctx.r[11].s64 + 3176;
	// 8260D480: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260D484: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8260D488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D48C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D498: 386A7C64  addi r3, r10, 0x7c64
	ctx.r[3].s64 = ctx.r[10].s64 + 31844;
	// 8260D49C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D4BC: 4BE59965  bl 0x82466e20
	ctx.lr = 0x8260D4C0;
	sub_82466E20(ctx, base);
	// 8260D4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D4D0 size=112
    let mut pc: u32 = 0x8260D4D0;
    'dispatch: loop {
        match pc {
            0x8260D4D0 => {
    //   block [0x8260D4D0..0x8260D540)
	// 8260D4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D4DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D4E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D4E4: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D4E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D4EC: 390B0CE0  addi r8, r11, 0xce0
	ctx.r[8].s64 = ctx.r[11].s64 + 3296;
	// 8260D4F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D4F4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8260D4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D4FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D508: 386A7C94  addi r3, r10, 0x7c94
	ctx.r[3].s64 = ctx.r[10].s64 + 31892;
	// 8260D50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D52C: 4BE598F5  bl 0x82466e20
	ctx.lr = 0x8260D530;
	sub_82466E20(ctx, base);
	// 8260D530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D540 size=108
    let mut pc: u32 = 0x8260D540;
    'dispatch: loop {
        match pc {
            0x8260D540 => {
    //   block [0x8260D540..0x8260D5AC)
	// 8260D540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D54C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D554: 38EB0D10  addi r7, r11, 0xd10
	ctx.r[7].s64 = ctx.r[11].s64 + 3344;
	// 8260D558: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260D55C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8260D560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D570: 386A7CC4  addi r3, r10, 0x7cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31940;
	// 8260D574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D598: 4BE59889  bl 0x82466e20
	ctx.lr = 0x8260D59C;
	sub_82466E20(ctx, base);
	// 8260D59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D5B0 size=108
    let mut pc: u32 = 0x8260D5B0;
    'dispatch: loop {
        match pc {
            0x8260D5B0 => {
    //   block [0x8260D5B0..0x8260D61C)
	// 8260D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D5BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D5C4: 38EB0D70  addi r7, r11, 0xd70
	ctx.r[7].s64 = ctx.r[11].s64 + 3440;
	// 8260D5C8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8260D5CC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8260D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D5D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D5D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D5E0: 386A7CF4  addi r3, r10, 0x7cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31988;
	// 8260D5E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D608: 4BE59819  bl 0x82466e20
	ctx.lr = 0x8260D60C;
	sub_82466E20(ctx, base);
	// 8260D60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D620 size=112
    let mut pc: u32 = 0x8260D620;
    'dispatch: loop {
        match pc {
            0x8260D620 => {
    //   block [0x8260D620..0x8260D690)
	// 8260D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D62C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D630: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D634: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D63C: 390B0DE8  addi r8, r11, 0xde8
	ctx.r[8].s64 = ctx.r[11].s64 + 3560;
	// 8260D640: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260D644: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8260D648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D64C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D658: 386A7D24  addi r3, r10, 0x7d24
	ctx.r[3].s64 = ctx.r[10].s64 + 32036;
	// 8260D65C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D67C: 4BE597A5  bl 0x82466e20
	ctx.lr = 0x8260D680;
	sub_82466E20(ctx, base);
	// 8260D680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260D690 size=24
    let mut pc: u32 = 0x8260D690;
    'dispatch: loop {
        match pc {
            0x8260D690 => {
    //   block [0x8260D690..0x8260D6A8)
	// 8260D690: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D694: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260D698: 394A3FF8  addi r10, r10, 0x3ff8
	ctx.r[10].s64 = ctx.r[10].s64 + 16376;
	// 8260D69C: 816B0A54  lwz r11, 0xa54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2644 as u32) ) } as u64;
	// 8260D6A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260D6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D6A8 size=116
    let mut pc: u32 = 0x8260D6A8;
    'dispatch: loop {
        match pc {
            0x8260D6A8 => {
    //   block [0x8260D6A8..0x8260D71C)
	// 8260D6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D6B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260D6BC: 390B3FF8  addi r8, r11, 0x3ff8
	ctx.r[8].s64 = ctx.r[11].s64 + 16376;
	// 8260D6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D6C4: 392AE0CC  addi r9, r10, -0x1f34
	ctx.r[9].s64 = ctx.r[10].s64 + -7988;
	// 8260D6C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D6CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260D6D0: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260D6D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D6DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D6EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260D6F0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8260D6F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D6F8: 386B7D54  addi r3, r11, 0x7d54
	ctx.r[3].s64 = ctx.r[11].s64 + 32084;
	// 8260D6FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260D700: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D708: 4BE59719  bl 0x82466e20
	ctx.lr = 0x8260D70C;
	sub_82466E20(ctx, base);
	// 8260D70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D720 size=112
    let mut pc: u32 = 0x8260D720;
    'dispatch: loop {
        match pc {
            0x8260D720 => {
    //   block [0x8260D720..0x8260D790)
	// 8260D720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D72C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D730: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D734: 38AA7D54  addi r5, r10, 0x7d54
	ctx.r[5].s64 = ctx.r[10].s64 + 32084;
	// 8260D738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D73C: 390B0E30  addi r8, r11, 0xe30
	ctx.r[8].s64 = ctx.r[11].s64 + 3632;
	// 8260D740: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D744: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8260D748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D74C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D758: 386A7D84  addi r3, r10, 0x7d84
	ctx.r[3].s64 = ctx.r[10].s64 + 32132;
	// 8260D75C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D77C: 4BE596A5  bl 0x82466e20
	ctx.lr = 0x8260D780;
	sub_82466E20(ctx, base);
	// 8260D780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260D790 size=24
    let mut pc: u32 = 0x8260D790;
    'dispatch: loop {
        match pc {
            0x8260D790 => {
    //   block [0x8260D790..0x8260D7A8)
	// 8260D790: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D794: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260D798: 394A4010  addi r10, r10, 0x4010
	ctx.r[10].s64 = ctx.r[10].s64 + 16400;
	// 8260D79C: 816B0E60  lwz r11, 0xe60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3680 as u32) ) } as u64;
	// 8260D7A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260D7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D7A8 size=116
    let mut pc: u32 = 0x8260D7A8;
    'dispatch: loop {
        match pc {
            0x8260D7A8 => {
    //   block [0x8260D7A8..0x8260D81C)
	// 8260D7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D7B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D7B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260D7BC: 390B4010  addi r8, r11, 0x4010
	ctx.r[8].s64 = ctx.r[11].s64 + 16400;
	// 8260D7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D7C4: 392AE108  addi r9, r10, -0x1ef8
	ctx.r[9].s64 = ctx.r[10].s64 + -7928;
	// 8260D7C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D7CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260D7D0: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D7D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D7DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D7EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260D7F0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 8260D7F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D7F8: 386B7DB4  addi r3, r11, 0x7db4
	ctx.r[3].s64 = ctx.r[11].s64 + 32180;
	// 8260D7FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260D800: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D808: 4BE59619  bl 0x82466e20
	ctx.lr = 0x8260D80C;
	sub_82466E20(ctx, base);
	// 8260D80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D820 size=112
    let mut pc: u32 = 0x8260D820;
    'dispatch: loop {
        match pc {
            0x8260D820 => {
    //   block [0x8260D820..0x8260D890)
	// 8260D820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D830: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D834: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D83C: 390B0E68  addi r8, r11, 0xe68
	ctx.r[8].s64 = ctx.r[11].s64 + 3688;
	// 8260D840: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260D844: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8260D848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D84C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D858: 386A7DE4  addi r3, r10, 0x7de4
	ctx.r[3].s64 = ctx.r[10].s64 + 32228;
	// 8260D85C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D87C: 4BE595A5  bl 0x82466e20
	ctx.lr = 0x8260D880;
	sub_82466E20(ctx, base);
	// 8260D880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D890 size=112
    let mut pc: u32 = 0x8260D890;
    'dispatch: loop {
        match pc {
            0x8260D890 => {
    //   block [0x8260D890..0x8260D900)
	// 8260D890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D89C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D8A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D8A4: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D8AC: 390B0EC8  addi r8, r11, 0xec8
	ctx.r[8].s64 = ctx.r[11].s64 + 3784;
	// 8260D8B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D8B4: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 8260D8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D8BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D8C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D8C8: 386A7E14  addi r3, r10, 0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + 32276;
	// 8260D8CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D8EC: 4BE59535  bl 0x82466e20
	ctx.lr = 0x8260D8F0;
	sub_82466E20(ctx, base);
	// 8260D8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D900 size=112
    let mut pc: u32 = 0x8260D900;
    'dispatch: loop {
        match pc {
            0x8260D900 => {
    //   block [0x8260D900..0x8260D970)
	// 8260D900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D910: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D914: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D91C: 390B0EF8  addi r8, r11, 0xef8
	ctx.r[8].s64 = ctx.r[11].s64 + 3832;
	// 8260D920: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260D924: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 8260D928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D938: 386A7E44  addi r3, r10, 0x7e44
	ctx.r[3].s64 = ctx.r[10].s64 + 32324;
	// 8260D93C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D94C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D95C: 4BE594C5  bl 0x82466e20
	ctx.lr = 0x8260D960;
	sub_82466E20(ctx, base);
	// 8260D960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D970 size=108
    let mut pc: u32 = 0x8260D970;
    'dispatch: loop {
        match pc {
            0x8260D970 => {
    //   block [0x8260D970..0x8260D9DC)
	// 8260D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D97C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D984: 38EB0F40  addi r7, r11, 0xf40
	ctx.r[7].s64 = ctx.r[11].s64 + 3904;
	// 8260D988: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260D98C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 8260D990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D9A0: 386A7E74  addi r3, r10, 0x7e74
	ctx.r[3].s64 = ctx.r[10].s64 + 32372;
	// 8260D9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D9C8: 4BE59459  bl 0x82466e20
	ctx.lr = 0x8260D9CC;
	sub_82466E20(ctx, base);
	// 8260D9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D9E0 size=112
    let mut pc: u32 = 0x8260D9E0;
    'dispatch: loop {
        match pc {
            0x8260D9E0 => {
    //   block [0x8260D9E0..0x8260DA50)
	// 8260D9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D9F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D9F4: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D9F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D9FC: 390B0F70  addi r8, r11, 0xf70
	ctx.r[8].s64 = ctx.r[11].s64 + 3952;
	// 8260DA00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260DA04: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 8260DA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DA0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DA10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DA18: 386A7EA4  addi r3, r10, 0x7ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 32420;
	// 8260DA1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260DA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DA3C: 4BE593E5  bl 0x82466e20
	ctx.lr = 0x8260DA40;
	sub_82466E20(ctx, base);
	// 8260DA40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DA50 size=108
    let mut pc: u32 = 0x8260DA50;
    'dispatch: loop {
        match pc {
            0x8260DA50 => {
    //   block [0x8260DA50..0x8260DABC)
	// 8260DA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DA5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DA64: 38EB0F88  addi r7, r11, 0xf88
	ctx.r[7].s64 = ctx.r[11].s64 + 3976;
	// 8260DA68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260DA6C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 8260DA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260DA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DA80: 386A7ED4  addi r3, r10, 0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 32468;
	// 8260DA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260DA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260DAA8: 4BE59379  bl 0x82466e20
	ctx.lr = 0x8260DAAC;
	sub_82466E20(ctx, base);
	// 8260DAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260DAC0 size=24
    let mut pc: u32 = 0x8260DAC0;
    'dispatch: loop {
        match pc {
            0x8260DAC0 => {
    //   block [0x8260DAC0..0x8260DAD8)
	// 8260DAC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DAC4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260DAC8: 394A4088  addi r10, r10, 0x4088
	ctx.r[10].s64 = ctx.r[10].s64 + 16520;
	// 8260DACC: 816B0E64  lwz r11, 0xe64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3684 as u32) ) } as u64;
	// 8260DAD0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8260DAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DAD8 size=108
    let mut pc: u32 = 0x8260DAD8;
    'dispatch: loop {
        match pc {
            0x8260DAD8 => {
    //   block [0x8260DAD8..0x8260DB44)
	// 8260DAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DAE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DAE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DAEC: 38EB4088  addi r7, r11, 0x4088
	ctx.r[7].s64 = ctx.r[11].s64 + 16520;
	// 8260DAF0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260DAF4: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 8260DAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DAFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DB00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260DB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DB08: 386A7F04  addi r3, r10, 0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + 32516;
	// 8260DB0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260DB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DB1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DB2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260DB30: 4BE592F1  bl 0x82466e20
	ctx.lr = 0x8260DB34;
	sub_82466E20(ctx, base);
	// 8260DB34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DB38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DB3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DB48 size=116
    let mut pc: u32 = 0x8260DB48;
    'dispatch: loop {
        match pc {
            0x8260DB48 => {
    //   block [0x8260DB48..0x8260DBBC)
	// 8260DB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DB54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260DB58: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DB5C: 392BE13C  addi r9, r11, -0x1ec4
	ctx.r[9].s64 = ctx.r[11].s64 + -7876;
	// 8260DB60: 38AA8384  addi r5, r10, -0x7c7c
	ctx.r[5].s64 = ctx.r[10].s64 + -31868;
	// 8260DB64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DB68: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260DB6C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8260DB70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DB74: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8260DB78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DB7C: 396B0FD0  addi r11, r11, 0xfd0
	ctx.r[11].s64 = ctx.r[11].s64 + 4048;
	// 8260DB80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260DB84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DB88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260DB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DB90: 386A7F34  addi r3, r10, 0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + 32564;
	// 8260DB94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260DB98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260DB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DBA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260DBA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DBA8: 4BE59279  bl 0x82466e20
	ctx.lr = 0x8260DBAC;
	sub_82466E20(ctx, base);
	// 8260DBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DBC0 size=100
    let mut pc: u32 = 0x8260DBC0;
    'dispatch: loop {
        match pc {
            0x8260DBC0 => {
    //   block [0x8260DBC0..0x8260DC24)
	// 8260DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DBD4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260DBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DBE0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8260DBE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DBF4: 386A7F64  addi r3, r10, 0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + 32612;
	// 8260DBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DBFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DC00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DC08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DC10: 4BE59211  bl 0x82466e20
	ctx.lr = 0x8260DC14;
	sub_82466E20(ctx, base);
	// 8260DC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DC28 size=100
    let mut pc: u32 = 0x8260DC28;
    'dispatch: loop {
        match pc {
            0x8260DC28 => {
    //   block [0x8260DC28..0x8260DC8C)
	// 8260DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DC3C: 38AA7FF4  addi r5, r10, 0x7ff4
	ctx.r[5].s64 = ctx.r[10].s64 + 32756;
	// 8260DC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DC48: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8260DC4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DC5C: 386A7F94  addi r3, r10, 0x7f94
	ctx.r[3].s64 = ctx.r[10].s64 + 32660;
	// 8260DC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DC68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DC78: 4BE591A9  bl 0x82466e20
	ctx.lr = 0x8260DC7C;
	sub_82466E20(ctx, base);
	// 8260DC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DC90 size=100
    let mut pc: u32 = 0x8260DC90;
    'dispatch: loop {
        match pc {
            0x8260DC90 => {
    //   block [0x8260DC90..0x8260DCF4)
	// 8260DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DC9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DCA4: 38AA7F34  addi r5, r10, 0x7f34
	ctx.r[5].s64 = ctx.r[10].s64 + 32564;
	// 8260DCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DCB0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8260DCB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DCC4: 386A7FC4  addi r3, r10, 0x7fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32708;
	// 8260DCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DCE0: 4BE59141  bl 0x82466e20
	ctx.lr = 0x8260DCE4;
	sub_82466E20(ctx, base);
	// 8260DCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DCF8 size=104
    let mut pc: u32 = 0x8260DCF8;
    'dispatch: loop {
        match pc {
            0x8260DCF8 => {
    //   block [0x8260DCF8..0x8260DD60)
	// 8260DCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DD04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260DD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DD0C: 392AE1B8  addi r9, r10, -0x1e48
	ctx.r[9].s64 = ctx.r[10].s64 + -7752;
	// 8260DD10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DD18: 38AA7F64  addi r5, r10, 0x7f64
	ctx.r[5].s64 = ctx.r[10].s64 + 32612;
	// 8260DD1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DD2C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8260DD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DD34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DD38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DD40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DD44: 386A7FF4  addi r3, r10, 0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 32756;
	// 8260DD48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260DD4C: 4BE590D5  bl 0x82466e20
	ctx.lr = 0x8260DD50;
	sub_82466E20(ctx, base);
	// 8260DD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DD60 size=108
    let mut pc: u32 = 0x8260DD60;
    'dispatch: loop {
        match pc {
            0x8260DD60 => {
    //   block [0x8260DD60..0x8260DDCC)
	// 8260DD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DD6C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DD74: 38EB116C  addi r7, r11, 0x116c
	ctx.r[7].s64 = ctx.r[11].s64 + 4460;
	// 8260DD78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260DD7C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8260DD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DD84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260DD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DD90: 386A8024  addi r3, r10, -0x7fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -32732;
	// 8260DD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260DD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DDB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260DDB8: 4BE59069  bl 0x82466e20
	ctx.lr = 0x8260DDBC;
	sub_82466E20(ctx, base);
	// 8260DDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DDD0 size=112
    let mut pc: u32 = 0x8260DDD0;
    'dispatch: loop {
        match pc {
            0x8260DDD0 => {
    //   block [0x8260DDD0..0x8260DE40)
	// 8260DDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DDDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DDE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DDE4: 38AA7FF4  addi r5, r10, 0x7ff4
	ctx.r[5].s64 = ctx.r[10].s64 + 32756;
	// 8260DDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DDEC: 390B11A0  addi r8, r11, 0x11a0
	ctx.r[8].s64 = ctx.r[11].s64 + 4512;
	// 8260DDF0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8260DDF4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8260DDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DDFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DE00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DE08: 386A8054  addi r3, r10, -0x7fac
	ctx.r[3].s64 = ctx.r[10].s64 + -32684;
	// 8260DE0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260DE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DE2C: 4BE58FF5  bl 0x82466e20
	ctx.lr = 0x8260DE30;
	sub_82466E20(ctx, base);
	// 8260DE30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260DE40 size=24
    let mut pc: u32 = 0x8260DE40;
    'dispatch: loop {
        match pc {
            0x8260DE40 => {
    //   block [0x8260DE40..0x8260DE58)
	// 8260DE40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DE44: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260DE48: 394A40E8  addi r10, r10, 0x40e8
	ctx.r[10].s64 = ctx.r[10].s64 + 16616;
	// 8260DE4C: 816B119C  lwz r11, 0x119c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4508 as u32) ) } as u64;
	// 8260DE50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260DE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DE58 size=116
    let mut pc: u32 = 0x8260DE58;
    'dispatch: loop {
        match pc {
            0x8260DE58 => {
    //   block [0x8260DE58..0x8260DECC)
	// 8260DE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DE64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DE68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260DE6C: 390B40E8  addi r8, r11, 0x40e8
	ctx.r[8].s64 = ctx.r[11].s64 + 16616;
	// 8260DE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DE74: 392AE220  addi r9, r10, -0x1de0
	ctx.r[9].s64 = ctx.r[10].s64 + -7648;
	// 8260DE78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DE7C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8260DE80: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260DE84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DE8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DE9C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8260DEA0: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8260DEA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260DEA8: 386B8084  addi r3, r11, -0x7f7c
	ctx.r[3].s64 = ctx.r[11].s64 + -32636;
	// 8260DEAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260DEB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DEB8: 4BE58F69  bl 0x82466e20
	ctx.lr = 0x8260DEBC;
	sub_82466E20(ctx, base);
	// 8260DEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DED0 size=100
    let mut pc: u32 = 0x8260DED0;
    'dispatch: loop {
        match pc {
            0x8260DED0 => {
    //   block [0x8260DED0..0x8260DF34)
	// 8260DED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DEDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DEE4: 38AA8084  addi r5, r10, -0x7f7c
	ctx.r[5].s64 = ctx.r[10].s64 + -32636;
	// 8260DEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DEF0: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8260DEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DF04: 386A80B4  addi r3, r10, -0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32588;
	// 8260DF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DF0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DF10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DF14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DF18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DF20: 4BE58F01  bl 0x82466e20
	ctx.lr = 0x8260DF24;
	sub_82466E20(ctx, base);
	// 8260DF24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DF38 size=100
    let mut pc: u32 = 0x8260DF38;
    'dispatch: loop {
        match pc {
            0x8260DF38 => {
    //   block [0x8260DF38..0x8260DF9C)
	// 8260DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DF44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DF4C: 38AA8114  addi r5, r10, -0x7eec
	ctx.r[5].s64 = ctx.r[10].s64 + -32492;
	// 8260DF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DF58: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8260DF5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DF6C: 386A80E4  addi r3, r10, -0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32540;
	// 8260DF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DF74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DF78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DF80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DF88: 4BE58E99  bl 0x82466e20
	ctx.lr = 0x8260DF8C;
	sub_82466E20(ctx, base);
	// 8260DF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DFA0 size=112
    let mut pc: u32 = 0x8260DFA0;
    'dispatch: loop {
        match pc {
            0x8260DFA0 => {
    //   block [0x8260DFA0..0x8260E010)
	// 8260DFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DFAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DFB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DFB4: 38AA8084  addi r5, r10, -0x7f7c
	ctx.r[5].s64 = ctx.r[10].s64 + -32636;
	// 8260DFB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DFBC: 390B1248  addi r8, r11, 0x1248
	ctx.r[8].s64 = ctx.r[11].s64 + 4680;
	// 8260DFC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260DFC4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8260DFC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DFCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DFD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DFD8: 386A8114  addi r3, r10, -0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + -32492;
	// 8260DFDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260DFE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DFFC: 4BE58E25  bl 0x82466e20
	ctx.lr = 0x8260E000;
	sub_82466E20(ctx, base);
	// 8260E000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E010 size=100
    let mut pc: u32 = 0x8260E010;
    'dispatch: loop {
        match pc {
            0x8260E010 => {
    //   block [0x8260E010..0x8260E074)
	// 8260E010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E01C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E024: 38AA8114  addi r5, r10, -0x7eec
	ctx.r[5].s64 = ctx.r[10].s64 + -32492;
	// 8260E028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E030: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8260E034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E044: 386A8144  addi r3, r10, -0x7ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -32444;
	// 8260E048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E04C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E050: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E060: 4BE58DC1  bl 0x82466e20
	ctx.lr = 0x8260E064;
	sub_82466E20(ctx, base);
	// 8260E064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E078 size=100
    let mut pc: u32 = 0x8260E078;
    'dispatch: loop {
        match pc {
            0x8260E078 => {
    //   block [0x8260E078..0x8260E0DC)
	// 8260E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E08C: 38AA8084  addi r5, r10, -0x7f7c
	ctx.r[5].s64 = ctx.r[10].s64 + -32636;
	// 8260E090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E098: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8260E09C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E0AC: 386A8174  addi r3, r10, -0x7e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32396;
	// 8260E0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E0B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E0B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E0C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E0C8: 4BE58D59  bl 0x82466e20
	ctx.lr = 0x8260E0CC;
	sub_82466E20(ctx, base);
	// 8260E0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E0E0 size=100
    let mut pc: u32 = 0x8260E0E0;
    'dispatch: loop {
        match pc {
            0x8260E0E0 => {
    //   block [0x8260E0E0..0x8260E144)
	// 8260E0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E0EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E0F4: 38AA80B4  addi r5, r10, -0x7f4c
	ctx.r[5].s64 = ctx.r[10].s64 + -32588;
	// 8260E0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E100: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8260E104: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E114: 386A81A4  addi r3, r10, -0x7e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32348;
	// 8260E118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E11C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E120: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E128: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E130: 4BE58CF1  bl 0x82466e20
	ctx.lr = 0x8260E134;
	sub_82466E20(ctx, base);
	// 8260E134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E148 size=100
    let mut pc: u32 = 0x8260E148;
    'dispatch: loop {
        match pc {
            0x8260E148 => {
    //   block [0x8260E148..0x8260E1AC)
	// 8260E148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E15C: 38AA8174  addi r5, r10, -0x7e8c
	ctx.r[5].s64 = ctx.r[10].s64 + -32396;
	// 8260E160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E168: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8260E16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E17C: 386A81D4  addi r3, r10, -0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32300;
	// 8260E180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E188: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E190: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E198: 4BE58C89  bl 0x82466e20
	ctx.lr = 0x8260E19C;
	sub_82466E20(ctx, base);
	// 8260E19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E1B0 size=100
    let mut pc: u32 = 0x8260E1B0;
    'dispatch: loop {
        match pc {
            0x8260E1B0 => {
    //   block [0x8260E1B0..0x8260E214)
	// 8260E1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E1BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E1C4: 38AA80B4  addi r5, r10, -0x7f4c
	ctx.r[5].s64 = ctx.r[10].s64 + -32588;
	// 8260E1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E1D0: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8260E1D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E1DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E1E4: 386A8204  addi r3, r10, -0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -32252;
	// 8260E1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E1EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E1F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E1F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E200: 4BE58C21  bl 0x82466e20
	ctx.lr = 0x8260E204;
	sub_82466E20(ctx, base);
	// 8260E204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E218 size=112
    let mut pc: u32 = 0x8260E218;
    'dispatch: loop {
        match pc {
            0x8260E218 => {
    //   block [0x8260E218..0x8260E288)
	// 8260E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E228: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E22C: 38AA8294  addi r5, r10, -0x7d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -32108;
	// 8260E230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E234: 390B1278  addi r8, r11, 0x1278
	ctx.r[8].s64 = ctx.r[11].s64 + 4728;
	// 8260E238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260E23C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8260E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E250: 386A8234  addi r3, r10, -0x7dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -32204;
	// 8260E254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E274: 4BE58BAD  bl 0x82466e20
	ctx.lr = 0x8260E278;
	sub_82466E20(ctx, base);
	// 8260E278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E288 size=112
    let mut pc: u32 = 0x8260E288;
    'dispatch: loop {
        match pc {
            0x8260E288 => {
    //   block [0x8260E288..0x8260E2F8)
	// 8260E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E298: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E29C: 38AA82C4  addi r5, r10, -0x7d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -32060;
	// 8260E2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E2A4: 390B12A8  addi r8, r11, 0x12a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4776;
	// 8260E2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E2AC: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8260E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E2B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E2C0: 386A8264  addi r3, r10, -0x7d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32156;
	// 8260E2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E2E4: 4BE58B3D  bl 0x82466e20
	ctx.lr = 0x8260E2E8;
	sub_82466E20(ctx, base);
	// 8260E2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E2F8 size=112
    let mut pc: u32 = 0x8260E2F8;
    'dispatch: loop {
        match pc {
            0x8260E2F8 => {
    //   block [0x8260E2F8..0x8260E368)
	// 8260E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E308: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E30C: 38AA8384  addi r5, r10, -0x7c7c
	ctx.r[5].s64 = ctx.r[10].s64 + -31868;
	// 8260E310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E314: 390B12C0  addi r8, r11, 0x12c0
	ctx.r[8].s64 = ctx.r[11].s64 + 4800;
	// 8260E318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260E31C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8260E320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E330: 386A8294  addi r3, r10, -0x7d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32108;
	// 8260E334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E354: 4BE58ACD  bl 0x82466e20
	ctx.lr = 0x8260E358;
	sub_82466E20(ctx, base);
	// 8260E358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E368 size=112
    let mut pc: u32 = 0x8260E368;
    'dispatch: loop {
        match pc {
            0x8260E368 => {
    //   block [0x8260E368..0x8260E3D8)
	// 8260E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E378: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E37C: 38AA8294  addi r5, r10, -0x7d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -32108;
	// 8260E380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E384: 390B12F0  addi r8, r11, 0x12f0
	ctx.r[8].s64 = ctx.r[11].s64 + 4848;
	// 8260E388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E38C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8260E390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E3A0: 386A82C4  addi r3, r10, -0x7d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32060;
	// 8260E3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E3C4: 4BE58A5D  bl 0x82466e20
	ctx.lr = 0x8260E3C8;
	sub_82466E20(ctx, base);
	// 8260E3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E3D8 size=112
    let mut pc: u32 = 0x8260E3D8;
    'dispatch: loop {
        match pc {
            0x8260E3D8 => {
    //   block [0x8260E3D8..0x8260E448)
	// 8260E3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E3E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E3E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E3EC: 38AA82C4  addi r5, r10, -0x7d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -32060;
	// 8260E3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E3F4: 390B1308  addi r8, r11, 0x1308
	ctx.r[8].s64 = ctx.r[11].s64 + 4872;
	// 8260E3F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E3FC: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8260E400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E410: 386A82F4  addi r3, r10, -0x7d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32012;
	// 8260E414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E434: 4BE589ED  bl 0x82466e20
	ctx.lr = 0x8260E438;
	sub_82466E20(ctx, base);
	// 8260E438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E448 size=116
    let mut pc: u32 = 0x8260E448;
    'dispatch: loop {
        match pc {
            0x8260E448 => {
    //   block [0x8260E448..0x8260E4BC)
	// 8260E448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E454: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260E458: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260E45C: 390A1320  addi r8, r10, 0x1320
	ctx.r[8].s64 = ctx.r[10].s64 + 4896;
	// 8260E460: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E464: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260E468: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E46C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E470: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260E474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E47C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8260E480: 396BE234  addi r11, r11, -0x1dcc
	ctx.r[11].s64 = ctx.r[11].s64 + -7628;
	// 8260E484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E48C: 386A8324  addi r3, r10, -0x7cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -31964;
	// 8260E490: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260E494: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E498: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260E49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E4A8: 4BE58979  bl 0x82466e20
	ctx.lr = 0x8260E4AC;
	sub_82466E20(ctx, base);
	// 8260E4AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E4B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E4B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260E4C0 size=48
    let mut pc: u32 = 0x8260E4C0;
    'dispatch: loop {
        match pc {
            0x8260E4C0 => {
    //   block [0x8260E4C0..0x8260E4F0)
	// 8260E4C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E4C4: 814B13D4  lwz r10, 0x13d4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5076 as u32) ) } as u64;
	// 8260E4C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E4CC: 396B41A8  addi r11, r11, 0x41a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16808;
	// 8260E4D0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8260E4D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260E4D8: 814A13D0  lwz r10, 0x13d0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5072 as u32) ) } as u64;
	// 8260E4DC: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8260E4E0: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260E4E4: 814A13CC  lwz r10, 0x13cc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5068 as u32) ) } as u64;
	// 8260E4E8: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 8260E4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E4F0 size=116
    let mut pc: u32 = 0x8260E4F0;
    'dispatch: loop {
        match pc {
            0x8260E4F0 => {
    //   block [0x8260E4F0..0x8260E564)
	// 8260E4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E4FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260E500: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E504: 392BE308  addi r9, r11, -0x1cf8
	ctx.r[9].s64 = ctx.r[11].s64 + -7416;
	// 8260E508: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E50C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E510: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8260E514: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 8260E518: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E51C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8260E520: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E524: 396B41A8  addi r11, r11, 0x41a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16808;
	// 8260E528: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260E52C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E530: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260E534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E538: 386A8354  addi r3, r10, -0x7cac
	ctx.r[3].s64 = ctx.r[10].s64 + -31916;
	// 8260E53C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8260E540: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260E544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E548: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260E54C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E550: 4BE588D1  bl 0x82466e20
	ctx.lr = 0x8260E554;
	sub_82466E20(ctx, base);
	// 8260E554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E568 size=116
    let mut pc: u32 = 0x8260E568;
    'dispatch: loop {
        match pc {
            0x8260E568 => {
    //   block [0x8260E568..0x8260E5DC)
	// 8260E568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E574: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E578: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260E57C: 390B13E0  addi r8, r11, 0x13e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5088;
	// 8260E580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E584: 392AE480  addi r9, r10, -0x1b80
	ctx.r[9].s64 = ctx.r[10].s64 + -7040;
	// 8260E588: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E58C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8260E590: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E594: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E59C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E5AC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8260E5B0: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8260E5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260E5B8: 386B8384  addi r3, r11, -0x7c7c
	ctx.r[3].s64 = ctx.r[11].s64 + -31868;
	// 8260E5BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260E5C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E5C8: 4BE58859  bl 0x82466e20
	ctx.lr = 0x8260E5CC;
	sub_82466E20(ctx, base);
	// 8260E5CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E5E0 size=112
    let mut pc: u32 = 0x8260E5E0;
    'dispatch: loop {
        match pc {
            0x8260E5E0 => {
    //   block [0x8260E5E0..0x8260E650)
	// 8260E5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E5EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E5F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E5F4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E5FC: 390B1470  addi r8, r11, 0x1470
	ctx.r[8].s64 = ctx.r[11].s64 + 5232;
	// 8260E600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E604: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8260E608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E60C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E618: 386A83B4  addi r3, r10, -0x7c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31820;
	// 8260E61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E63C: 4BE587E5  bl 0x82466e20
	ctx.lr = 0x8260E640;
	sub_82466E20(ctx, base);
	// 8260E640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E650 size=112
    let mut pc: u32 = 0x8260E650;
    'dispatch: loop {
        match pc {
            0x8260E650 => {
    //   block [0x8260E650..0x8260E6C0)
	// 8260E650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E65C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E660: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E664: 38AA63D4  addi r5, r10, 0x63d4
	ctx.r[5].s64 = ctx.r[10].s64 + 25556;
	// 8260E668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E66C: 390B1488  addi r8, r11, 0x1488
	ctx.r[8].s64 = ctx.r[11].s64 + 5256;
	// 8260E670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E674: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8260E678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E67C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E688: 386A83E4  addi r3, r10, -0x7c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31772;
	// 8260E68C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E6AC: 4BE58775  bl 0x82466e20
	ctx.lr = 0x8260E6B0;
	sub_82466E20(ctx, base);
	// 8260E6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E6C0 size=108
    let mut pc: u32 = 0x8260E6C0;
    'dispatch: loop {
        match pc {
            0x8260E6C0 => {
    //   block [0x8260E6C0..0x8260E72C)
	// 8260E6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E6CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E6D4: 38EB14A0  addi r7, r11, 0x14a0
	ctx.r[7].s64 = ctx.r[11].s64 + 5280;
	// 8260E6D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260E6DC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8260E6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E6E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E6E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260E6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E6F0: 386A8414  addi r3, r10, -0x7bec
	ctx.r[3].s64 = ctx.r[10].s64 + -31724;
	// 8260E6F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260E6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E718: 4BE58709  bl 0x82466e20
	ctx.lr = 0x8260E71C;
	sub_82466E20(ctx, base);
	// 8260E71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E730 size=112
    let mut pc: u32 = 0x8260E730;
    'dispatch: loop {
        match pc {
            0x8260E730 => {
    //   block [0x8260E730..0x8260E7A0)
	// 8260E730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E73C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E740: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E744: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E74C: 390B14B8  addi r8, r11, 0x14b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5304;
	// 8260E750: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260E754: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8260E758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E75C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E768: 386A8444  addi r3, r10, -0x7bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31676;
	// 8260E76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E78C: 4BE58695  bl 0x82466e20
	ctx.lr = 0x8260E790;
	sub_82466E20(ctx, base);
	// 8260E790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E7A0 size=108
    let mut pc: u32 = 0x8260E7A0;
    'dispatch: loop {
        match pc {
            0x8260E7A0 => {
    //   block [0x8260E7A0..0x8260E80C)
	// 8260E7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E7AC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E7B4: 38EB1500  addi r7, r11, 0x1500
	ctx.r[7].s64 = ctx.r[11].s64 + 5376;
	// 8260E7B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260E7BC: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 8260E7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E7C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E7C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260E7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E7D0: 386A8474  addi r3, r10, -0x7b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31628;
	// 8260E7D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260E7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E7F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E7F8: 4BE58629  bl 0x82466e20
	ctx.lr = 0x8260E7FC;
	sub_82466E20(ctx, base);
	// 8260E7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E810 size=108
    let mut pc: u32 = 0x8260E810;
    'dispatch: loop {
        match pc {
            0x8260E810 => {
    //   block [0x8260E810..0x8260E87C)
	// 8260E810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E81C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E824: 38EB1530  addi r7, r11, 0x1530
	ctx.r[7].s64 = ctx.r[11].s64 + 5424;
	// 8260E828: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260E82C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8260E830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260E83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E840: 386A84A4  addi r3, r10, -0x7b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -31580;
	// 8260E844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260E848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E868: 4BE585B9  bl 0x82466e20
	ctx.lr = 0x8260E86C;
	sub_82466E20(ctx, base);
	// 8260E86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E880 size=112
    let mut pc: u32 = 0x8260E880;
    'dispatch: loop {
        match pc {
            0x8260E880 => {
    //   block [0x8260E880..0x8260E8F0)
	// 8260E880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E88C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E890: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E894: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E89C: 390B1548  addi r8, r11, 0x1548
	ctx.r[8].s64 = ctx.r[11].s64 + 5448;
	// 8260E8A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260E8A4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8260E8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E8AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E8B8: 386A84D4  addi r3, r10, -0x7b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31532;
	// 8260E8BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E8DC: 4BE58545  bl 0x82466e20
	ctx.lr = 0x8260E8E0;
	sub_82466E20(ctx, base);
	// 8260E8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E8F0 size=96
    let mut pc: u32 = 0x8260E8F0;
    'dispatch: loop {
        match pc {
            0x8260E8F0 => {
    //   block [0x8260E8F0..0x8260E950)
	// 8260E8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E8FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E904: 388A507C  addi r4, r10, 0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + 20604;
	// 8260E908: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E910: 386A8504  addi r3, r10, -0x7afc
	ctx.r[3].s64 = ctx.r[10].s64 + -31484;
	// 8260E914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E91C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260E920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E930: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E938: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E93C: 4BE584E5  bl 0x82466e20
	ctx.lr = 0x8260E940;
	sub_82466E20(ctx, base);
	// 8260E940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E950 size=112
    let mut pc: u32 = 0x8260E950;
    'dispatch: loop {
        match pc {
            0x8260E950 => {
    //   block [0x8260E950..0x8260E9C0)
	// 8260E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E95C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260E960: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E964: 392AE4D8  addi r9, r10, -0x1b28
	ctx.r[9].s64 = ctx.r[10].s64 + -6952;
	// 8260E968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E96C: 390B1580  addi r8, r11, 0x1580
	ctx.r[8].s64 = ctx.r[11].s64 + 5504;
	// 8260E970: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260E974: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 8260E978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E97C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E988: 386A8534  addi r3, r10, -0x7acc
	ctx.r[3].s64 = ctx.r[10].s64 + -31436;
	// 8260E98C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260E990: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260E994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E9AC: 4BE58475  bl 0x82466e20
	ctx.lr = 0x8260E9B0;
	sub_82466E20(ctx, base);
	// 8260E9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E9C0 size=116
    let mut pc: u32 = 0x8260E9C0;
    'dispatch: loop {
        match pc {
            0x8260E9C0 => {
    //   block [0x8260E9C0..0x8260EA34)
	// 8260E9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E9CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E9D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260E9D4: 390B1628  addi r8, r11, 0x1628
	ctx.r[8].s64 = ctx.r[11].s64 + 5672;
	// 8260E9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E9DC: 392AE4AC  addi r9, r10, -0x1b54
	ctx.r[9].s64 = ctx.r[10].s64 + -6996;
	// 8260E9E0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E9E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260E9E8: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260E9EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E9F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EA04: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8260EA08: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8260EA0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260EA10: 386B8564  addi r3, r11, -0x7a9c
	ctx.r[3].s64 = ctx.r[11].s64 + -31388;
	// 8260EA14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260EA18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EA20: 4BE58401  bl 0x82466e20
	ctx.lr = 0x8260EA24;
	sub_82466E20(ctx, base);
	// 8260EA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EA38 size=112
    let mut pc: u32 = 0x8260EA38;
    'dispatch: loop {
        match pc {
            0x8260EA38 => {
    //   block [0x8260EA38..0x8260EAA8)
	// 8260EA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EA44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260EA48: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EA4C: 392AE504  addi r9, r10, -0x1afc
	ctx.r[9].s64 = ctx.r[10].s64 + -6908;
	// 8260EA50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EA54: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 8260EA58: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8260EA5C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 8260EA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EA64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EA68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EA70: 386A8594  addi r3, r10, -0x7a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -31340;
	// 8260EA74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260EA78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260EA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260EA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EA94: 4BE5838D  bl 0x82466e20
	ctx.lr = 0x8260EA98;
	sub_82466E20(ctx, base);
	// 8260EA98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EAA8 size=112
    let mut pc: u32 = 0x8260EAA8;
    'dispatch: loop {
        match pc {
            0x8260EAA8 => {
    //   block [0x8260EAA8..0x8260EB18)
	// 8260EAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EAB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EABC: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260EAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EAC4: 390B16A0  addi r8, r11, 0x16a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5792;
	// 8260EAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EACC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8260EAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EAD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EAE0: 386A85C4  addi r3, r10, -0x7a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31292;
	// 8260EAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EB04: 4BE5831D  bl 0x82466e20
	ctx.lr = 0x8260EB08;
	sub_82466E20(ctx, base);
	// 8260EB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EB18 size=112
    let mut pc: u32 = 0x8260EB18;
    'dispatch: loop {
        match pc {
            0x8260EB18 => {
    //   block [0x8260EB18..0x8260EB88)
	// 8260EB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EB24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EB28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EB2C: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 8260EB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EB34: 390B16B8  addi r8, r11, 0x16b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5816;
	// 8260EB38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260EB3C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8260EB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EB44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EB50: 386A85F4  addi r3, r10, -0x7a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31244;
	// 8260EB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EB74: 4BE582AD  bl 0x82466e20
	ctx.lr = 0x8260EB78;
	sub_82466E20(ctx, base);
	// 8260EB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EB88 size=112
    let mut pc: u32 = 0x8260EB88;
    'dispatch: loop {
        match pc {
            0x8260EB88 => {
    //   block [0x8260EB88..0x8260EBF8)
	// 8260EB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EB94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EB98: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EB9C: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 8260EBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EBA4: 390B1700  addi r8, r11, 0x1700
	ctx.r[8].s64 = ctx.r[11].s64 + 5888;
	// 8260EBA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260EBAC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8260EBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EBB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EBB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EBC0: 386A8624  addi r3, r10, -0x79dc
	ctx.r[3].s64 = ctx.r[10].s64 + -31196;
	// 8260EBC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EBE4: 4BE5823D  bl 0x82466e20
	ctx.lr = 0x8260EBE8;
	sub_82466E20(ctx, base);
	// 8260EBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EBF8 size=112
    let mut pc: u32 = 0x8260EBF8;
    'dispatch: loop {
        match pc {
            0x8260EBF8 => {
    //   block [0x8260EBF8..0x8260EC68)
	// 8260EBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EC04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EC08: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EC0C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260EC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EC14: 390B1760  addi r8, r11, 0x1760
	ctx.r[8].s64 = ctx.r[11].s64 + 5984;
	// 8260EC18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260EC1C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8260EC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EC24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EC30: 386A8654  addi r3, r10, -0x79ac
	ctx.r[3].s64 = ctx.r[10].s64 + -31148;
	// 8260EC34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EC54: 4BE581CD  bl 0x82466e20
	ctx.lr = 0x8260EC58;
	sub_82466E20(ctx, base);
	// 8260EC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EC68 size=112
    let mut pc: u32 = 0x8260EC68;
    'dispatch: loop {
        match pc {
            0x8260EC68 => {
    //   block [0x8260EC68..0x8260ECD8)
	// 8260EC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EC74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EC78: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EC7C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260EC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EC84: 390B17C0  addi r8, r11, 0x17c0
	ctx.r[8].s64 = ctx.r[11].s64 + 6080;
	// 8260EC88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260EC8C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8260EC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EC94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ECA0: 386A8684  addi r3, r10, -0x797c
	ctx.r[3].s64 = ctx.r[10].s64 + -31100;
	// 8260ECA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260ECA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ECAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ECB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260ECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ECB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260ECBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ECC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ECC4: 4BE5815D  bl 0x82466e20
	ctx.lr = 0x8260ECC8;
	sub_82466E20(ctx, base);
	// 8260ECC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ECD8 size=112
    let mut pc: u32 = 0x8260ECD8;
    'dispatch: loop {
        match pc {
            0x8260ECD8 => {
    //   block [0x8260ECD8..0x8260ED48)
	// 8260ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ECE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ECE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ECE8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ECEC: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 8260ECF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ECF4: 390B1820  addi r8, r11, 0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + 6176;
	// 8260ECF8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8260ECFC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8260ED00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ED04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260ED08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ED0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ED10: 386A86B4  addi r3, r10, -0x794c
	ctx.r[3].s64 = ctx.r[10].s64 + -31052;
	// 8260ED14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260ED18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ED1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ED20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260ED24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ED28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260ED2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ED30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ED34: 4BE580ED  bl 0x82466e20
	ctx.lr = 0x8260ED38;
	sub_82466E20(ctx, base);
	// 8260ED38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ED48 size=112
    let mut pc: u32 = 0x8260ED48;
    'dispatch: loop {
        match pc {
            0x8260ED48 => {
    //   block [0x8260ED48..0x8260EDB8)
	// 8260ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ED50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ED54: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260ED58: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8260ED5C: 38EA18E0  addi r7, r10, 0x18e0
	ctx.r[7].s64 = ctx.r[10].s64 + 6368;
	// 8260ED60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ED64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260ED68: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8260ED6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ED70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260ED74: 396BE518  addi r11, r11, -0x1ae8
	ctx.r[11].s64 = ctx.r[11].s64 + -6888;
	// 8260ED78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260ED7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260ED80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ED84: 386A86E4  addi r3, r10, -0x791c
	ctx.r[3].s64 = ctx.r[10].s64 + -31004;
	// 8260ED88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ED8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260ED90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ED94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260ED98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ED9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EDA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260EDA4: 4BE5807D  bl 0x82466e20
	ctx.lr = 0x8260EDA8;
	sub_82466E20(ctx, base);
	// 8260EDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EDB8 size=112
    let mut pc: u32 = 0x8260EDB8;
    'dispatch: loop {
        match pc {
            0x8260EDB8 => {
    //   block [0x8260EDB8..0x8260EE28)
	// 8260EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EDC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EDC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EDCC: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EDD4: 390B1AA8  addi r8, r11, 0x1aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 6824;
	// 8260EDD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EDDC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8260EDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EDE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EDF0: 386A8714  addi r3, r10, -0x78ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30956;
	// 8260EDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EE04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260EE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EE14: 4BE5800D  bl 0x82466e20
	ctx.lr = 0x8260EE18;
	sub_82466E20(ctx, base);
	// 8260EE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EE28 size=112
    let mut pc: u32 = 0x8260EE28;
    'dispatch: loop {
        match pc {
            0x8260EE28 => {
    //   block [0x8260EE28..0x8260EE98)
	// 8260EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EE34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EE38: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EE3C: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EE44: 390B1AC0  addi r8, r11, 0x1ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 6848;
	// 8260EE48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EE4C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8260EE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EE54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EE60: 386A8744  addi r3, r10, -0x78bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30908;
	// 8260EE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EE74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260EE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EE84: 4BE57F9D  bl 0x82466e20
	ctx.lr = 0x8260EE88;
	sub_82466E20(ctx, base);
	// 8260EE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EE98 size=112
    let mut pc: u32 = 0x8260EE98;
    'dispatch: loop {
        match pc {
            0x8260EE98 => {
    //   block [0x8260EE98..0x8260EF08)
	// 8260EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EEA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EEAC: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EEB4: 390B1AD8  addi r8, r11, 0x1ad8
	ctx.r[8].s64 = ctx.r[11].s64 + 6872;
	// 8260EEB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260EEBC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8260EEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EED0: 386A8774  addi r3, r10, -0x788c
	ctx.r[3].s64 = ctx.r[10].s64 + -30860;
	// 8260EED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EEF4: 4BE57F2D  bl 0x82466e20
	ctx.lr = 0x8260EEF8;
	sub_82466E20(ctx, base);
	// 8260EEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EF08 size=108
    let mut pc: u32 = 0x8260EF08;
    'dispatch: loop {
        match pc {
            0x8260EF08 => {
    //   block [0x8260EF08..0x8260EF74)
	// 8260EF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EF14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EF1C: 38EB1B08  addi r7, r11, 0x1b08
	ctx.r[7].s64 = ctx.r[11].s64 + 6920;
	// 8260EF20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260EF24: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8260EF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EF2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260EF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EF38: 386A87A4  addi r3, r10, -0x785c
	ctx.r[3].s64 = ctx.r[10].s64 + -30812;
	// 8260EF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260EF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260EF60: 4BE57EC1  bl 0x82466e20
	ctx.lr = 0x8260EF64;
	sub_82466E20(ctx, base);
	// 8260EF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EF78 size=112
    let mut pc: u32 = 0x8260EF78;
    'dispatch: loop {
        match pc {
            0x8260EF78 => {
    //   block [0x8260EF78..0x8260EFE8)
	// 8260EF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EF84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EF88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EF8C: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EF94: 390B1B38  addi r8, r11, 0x1b38
	ctx.r[8].s64 = ctx.r[11].s64 + 6968;
	// 8260EF98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EF9C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8260EFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EFA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EFB0: 386A87D4  addi r3, r10, -0x782c
	ctx.r[3].s64 = ctx.r[10].s64 + -30764;
	// 8260EFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EFC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260EFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EFD4: 4BE57E4D  bl 0x82466e20
	ctx.lr = 0x8260EFD8;
	sub_82466E20(ctx, base);
	// 8260EFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EFE8 size=108
    let mut pc: u32 = 0x8260EFE8;
    'dispatch: loop {
        match pc {
            0x8260EFE8 => {
    //   block [0x8260EFE8..0x8260F054)
	// 8260EFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EFF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EFF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EFFC: 38EB1B50  addi r7, r11, 0x1b50
	ctx.r[7].s64 = ctx.r[11].s64 + 6992;
	// 8260F000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F004: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8260F008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F00C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F018: 386A8804  addi r3, r10, -0x77fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30716;
	// 8260F01C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F03C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F040: 4BE57DE1  bl 0x82466e20
	ctx.lr = 0x8260F044;
	sub_82466E20(ctx, base);
	// 8260F044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F058 size=108
    let mut pc: u32 = 0x8260F058;
    'dispatch: loop {
        match pc {
            0x8260F058 => {
    //   block [0x8260F058..0x8260F0C4)
	// 8260F058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F064: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F06C: 38EB1B80  addi r7, r11, 0x1b80
	ctx.r[7].s64 = ctx.r[11].s64 + 7040;
	// 8260F070: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260F074: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8260F078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F07C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F088: 386A8834  addi r3, r10, -0x77cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30668;
	// 8260F08C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F0B0: 4BE57D71  bl 0x82466e20
	ctx.lr = 0x8260F0B4;
	sub_82466E20(ctx, base);
	// 8260F0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F0C8 size=112
    let mut pc: u32 = 0x8260F0C8;
    'dispatch: loop {
        match pc {
            0x8260F0C8 => {
    //   block [0x8260F0C8..0x8260F138)
	// 8260F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F0D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F0D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F0DC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F0E4: 390B1BC8  addi r8, r11, 0x1bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 7112;
	// 8260F0E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260F0EC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8260F0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F100: 386A8864  addi r3, r10, -0x779c
	ctx.r[3].s64 = ctx.r[10].s64 + -30620;
	// 8260F104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F124: 4BE57CFD  bl 0x82466e20
	ctx.lr = 0x8260F128;
	sub_82466E20(ctx, base);
	// 8260F128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F138 size=112
    let mut pc: u32 = 0x8260F138;
    'dispatch: loop {
        match pc {
            0x8260F138 => {
    //   block [0x8260F138..0x8260F1A8)
	// 8260F138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F144: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F148: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F14C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260F150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F154: 390B1C10  addi r8, r11, 0x1c10
	ctx.r[8].s64 = ctx.r[11].s64 + 7184;
	// 8260F158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260F15C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8260F160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F170: 386A8894  addi r3, r10, -0x776c
	ctx.r[3].s64 = ctx.r[10].s64 + -30572;
	// 8260F174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F194: 4BE57C8D  bl 0x82466e20
	ctx.lr = 0x8260F198;
	sub_82466E20(ctx, base);
	// 8260F198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F1A8 size=108
    let mut pc: u32 = 0x8260F1A8;
    'dispatch: loop {
        match pc {
            0x8260F1A8 => {
    //   block [0x8260F1A8..0x8260F214)
	// 8260F1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F1B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F1BC: 38EB1CA0  addi r7, r11, 0x1ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 7328;
	// 8260F1C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260F1C4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8260F1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F1CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F1D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F1D8: 386A88C4  addi r3, r10, -0x773c
	ctx.r[3].s64 = ctx.r[10].s64 + -30524;
	// 8260F1DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F200: 4BE57C21  bl 0x82466e20
	ctx.lr = 0x8260F204;
	sub_82466E20(ctx, base);
	// 8260F204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F218 size=108
    let mut pc: u32 = 0x8260F218;
    'dispatch: loop {
        match pc {
            0x8260F218 => {
    //   block [0x8260F218..0x8260F284)
	// 8260F218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F224: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F22C: 38EB1CE8  addi r7, r11, 0x1ce8
	ctx.r[7].s64 = ctx.r[11].s64 + 7400;
	// 8260F230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F234: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8260F238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F248: 386A88F4  addi r3, r10, -0x770c
	ctx.r[3].s64 = ctx.r[10].s64 + -30476;
	// 8260F24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F270: 4BE57BB1  bl 0x82466e20
	ctx.lr = 0x8260F274;
	sub_82466E20(ctx, base);
	// 8260F274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F288 size=108
    let mut pc: u32 = 0x8260F288;
    'dispatch: loop {
        match pc {
            0x8260F288 => {
    //   block [0x8260F288..0x8260F2F4)
	// 8260F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F294: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F29C: 38EB1D18  addi r7, r11, 0x1d18
	ctx.r[7].s64 = ctx.r[11].s64 + 7448;
	// 8260F2A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F2A4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8260F2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F2AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F2B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F2B8: 386A8924  addi r3, r10, -0x76dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30428;
	// 8260F2BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F2D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F2DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F2E0: 4BE57B41  bl 0x82466e20
	ctx.lr = 0x8260F2E4;
	sub_82466E20(ctx, base);
	// 8260F2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F2F8 size=112
    let mut pc: u32 = 0x8260F2F8;
    'dispatch: loop {
        match pc {
            0x8260F2F8 => {
    //   block [0x8260F2F8..0x8260F368)
	// 8260F2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F304: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F308: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F30C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F314: 390B1D48  addi r8, r11, 0x1d48
	ctx.r[8].s64 = ctx.r[11].s64 + 7496;
	// 8260F318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260F31C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8260F320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F330: 386A8954  addi r3, r10, -0x76ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30380;
	// 8260F334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F354: 4BE57ACD  bl 0x82466e20
	ctx.lr = 0x8260F358;
	sub_82466E20(ctx, base);
	// 8260F358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F368 size=112
    let mut pc: u32 = 0x8260F368;
    'dispatch: loop {
        match pc {
            0x8260F368 => {
    //   block [0x8260F368..0x8260F3D8)
	// 8260F368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F378: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F37C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F384: 390B1D78  addi r8, r11, 0x1d78
	ctx.r[8].s64 = ctx.r[11].s64 + 7544;
	// 8260F388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260F38C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8260F390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F3A0: 386A8984  addi r3, r10, -0x767c
	ctx.r[3].s64 = ctx.r[10].s64 + -30332;
	// 8260F3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F3C4: 4BE57A5D  bl 0x82466e20
	ctx.lr = 0x8260F3C8;
	sub_82466E20(ctx, base);
	// 8260F3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F3D8 size=112
    let mut pc: u32 = 0x8260F3D8;
    'dispatch: loop {
        match pc {
            0x8260F3D8 => {
    //   block [0x8260F3D8..0x8260F448)
	// 8260F3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F3E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F3E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F3EC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F3F4: 390B1D90  addi r8, r11, 0x1d90
	ctx.r[8].s64 = ctx.r[11].s64 + 7568;
	// 8260F3F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260F3FC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8260F400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F410: 386A89B4  addi r3, r10, -0x764c
	ctx.r[3].s64 = ctx.r[10].s64 + -30284;
	// 8260F414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F434: 4BE579ED  bl 0x82466e20
	ctx.lr = 0x8260F438;
	sub_82466E20(ctx, base);
	// 8260F438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F448 size=108
    let mut pc: u32 = 0x8260F448;
    'dispatch: loop {
        match pc {
            0x8260F448 => {
    //   block [0x8260F448..0x8260F4B4)
	// 8260F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F454: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F45C: 38EB1DA8  addi r7, r11, 0x1da8
	ctx.r[7].s64 = ctx.r[11].s64 + 7592;
	// 8260F460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F464: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8260F468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F46C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F478: 386A89E4  addi r3, r10, -0x761c
	ctx.r[3].s64 = ctx.r[10].s64 + -30236;
	// 8260F47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F4A0: 4BE57981  bl 0x82466e20
	ctx.lr = 0x8260F4A4;
	sub_82466E20(ctx, base);
	// 8260F4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F4B8 size=112
    let mut pc: u32 = 0x8260F4B8;
    'dispatch: loop {
        match pc {
            0x8260F4B8 => {
    //   block [0x8260F4B8..0x8260F528)
	// 8260F4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F4C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F4CC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F4D4: 390B1DD8  addi r8, r11, 0x1dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 7640;
	// 8260F4D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260F4DC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8260F4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F4E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F4F0: 386A8A14  addi r3, r10, -0x75ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30188;
	// 8260F4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F514: 4BE5790D  bl 0x82466e20
	ctx.lr = 0x8260F518;
	sub_82466E20(ctx, base);
	// 8260F518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F528 size=108
    let mut pc: u32 = 0x8260F528;
    'dispatch: loop {
        match pc {
            0x8260F528 => {
    //   block [0x8260F528..0x8260F594)
	// 8260F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F534: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F53C: 38EB1DF0  addi r7, r11, 0x1df0
	ctx.r[7].s64 = ctx.r[11].s64 + 7664;
	// 8260F540: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8260F544: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8260F548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F54C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F558: 386A8A44  addi r3, r10, -0x75bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30140;
	// 8260F55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F580: 4BE578A1  bl 0x82466e20
	ctx.lr = 0x8260F584;
	sub_82466E20(ctx, base);
	// 8260F584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F598 size=112
    let mut pc: u32 = 0x8260F598;
    'dispatch: loop {
        match pc {
            0x8260F598 => {
    //   block [0x8260F598..0x8260F608)
	// 8260F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F5A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F5A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F5AC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F5B4: 390B1EE0  addi r8, r11, 0x1ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 7904;
	// 8260F5B8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8260F5BC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8260F5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F5C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F5D0: 386A8A74  addi r3, r10, -0x758c
	ctx.r[3].s64 = ctx.r[10].s64 + -30092;
	// 8260F5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F5F4: 4BE5782D  bl 0x82466e20
	ctx.lr = 0x8260F5F8;
	sub_82466E20(ctx, base);
	// 8260F5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F608 size=108
    let mut pc: u32 = 0x8260F608;
    'dispatch: loop {
        match pc {
            0x8260F608 => {
    //   block [0x8260F608..0x8260F674)
	// 8260F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F614: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F61C: 38EB2090  addi r7, r11, 0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + 8336;
	// 8260F620: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8260F624: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8260F628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F62C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F638: 386A8AA4  addi r3, r10, -0x755c
	ctx.r[3].s64 = ctx.r[10].s64 + -30044;
	// 8260F63C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F660: 4BE577C1  bl 0x82466e20
	ctx.lr = 0x8260F664;
	sub_82466E20(ctx, base);
	// 8260F664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F678 size=112
    let mut pc: u32 = 0x8260F678;
    'dispatch: loop {
        match pc {
            0x8260F678 => {
    //   block [0x8260F678..0x8260F6E8)
	// 8260F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F688: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F68C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260F690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F694: 390B2228  addi r8, r11, 0x2228
	ctx.r[8].s64 = ctx.r[11].s64 + 8744;
	// 8260F698: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8260F69C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8260F6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F6B0: 386A8AD4  addi r3, r10, -0x752c
	ctx.r[3].s64 = ctx.r[10].s64 + -29996;
	// 8260F6B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F6D4: 4BE5774D  bl 0x82466e20
	ctx.lr = 0x8260F6D8;
	sub_82466E20(ctx, base);
	// 8260F6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F6E8 size=100
    let mut pc: u32 = 0x8260F6E8;
    'dispatch: loop {
        match pc {
            0x8260F6E8 => {
    //   block [0x8260F6E8..0x8260F74C)
	// 8260F6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F6F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F6FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F708: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8260F70C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F71C: 386A8B04  addi r3, r10, -0x74fc
	ctx.r[3].s64 = ctx.r[10].s64 + -29948;
	// 8260F720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F738: 4BE576E9  bl 0x82466e20
	ctx.lr = 0x8260F73C;
	sub_82466E20(ctx, base);
	// 8260F73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F750 size=112
    let mut pc: u32 = 0x8260F750;
    'dispatch: loop {
        match pc {
            0x8260F750 => {
    //   block [0x8260F750..0x8260F7C0)
	// 8260F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F75C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F760: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F764: 38AA8B04  addi r5, r10, -0x74fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29948;
	// 8260F768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F76C: 390B2480  addi r8, r11, 0x2480
	ctx.r[8].s64 = ctx.r[11].s64 + 9344;
	// 8260F770: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260F774: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8260F778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F77C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F788: 386A8B34  addi r3, r10, -0x74cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29900;
	// 8260F78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F7AC: 4BE57675  bl 0x82466e20
	ctx.lr = 0x8260F7B0;
	sub_82466E20(ctx, base);
	// 8260F7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F7C0 size=100
    let mut pc: u32 = 0x8260F7C0;
    'dispatch: loop {
        match pc {
            0x8260F7C0 => {
    //   block [0x8260F7C0..0x8260F824)
	// 8260F7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F7CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F7D4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F7E0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8260F7E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F7F4: 386A8B64  addi r3, r10, -0x749c
	ctx.r[3].s64 = ctx.r[10].s64 + -29852;
	// 8260F7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F7FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F810: 4BE57611  bl 0x82466e20
	ctx.lr = 0x8260F814;
	sub_82466E20(ctx, base);
	// 8260F814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F828 size=108
    let mut pc: u32 = 0x8260F828;
    'dispatch: loop {
        match pc {
            0x8260F828 => {
    //   block [0x8260F828..0x8260F894)
	// 8260F828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F834: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F83C: 38EB24F8  addi r7, r11, 0x24f8
	ctx.r[7].s64 = ctx.r[11].s64 + 9464;
	// 8260F840: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260F844: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8260F848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F84C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F858: 386A8B94  addi r3, r10, -0x746c
	ctx.r[3].s64 = ctx.r[10].s64 + -29804;
	// 8260F85C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F880: 4BE575A1  bl 0x82466e20
	ctx.lr = 0x8260F884;
	sub_82466E20(ctx, base);
	// 8260F884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F898 size=112
    let mut pc: u32 = 0x8260F898;
    'dispatch: loop {
        match pc {
            0x8260F898 => {
    //   block [0x8260F898..0x8260F908)
	// 8260F898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F8A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F8A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F8AC: 38AA8B64  addi r5, r10, -0x749c
	ctx.r[5].s64 = ctx.r[10].s64 + -29852;
	// 8260F8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F8B4: 390B2540  addi r8, r11, 0x2540
	ctx.r[8].s64 = ctx.r[11].s64 + 9536;
	// 8260F8B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260F8BC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8260F8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F8C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F8D0: 386A8BC4  addi r3, r10, -0x743c
	ctx.r[3].s64 = ctx.r[10].s64 + -29756;
	// 8260F8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F8F4: 4BE5752D  bl 0x82466e20
	ctx.lr = 0x8260F8F8;
	sub_82466E20(ctx, base);
	// 8260F8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F908 size=100
    let mut pc: u32 = 0x8260F908;
    'dispatch: loop {
        match pc {
            0x8260F908 => {
    //   block [0x8260F908..0x8260F96C)
	// 8260F908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F914: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F91C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F928: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8260F92C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F93C: 386A8BF4  addi r3, r10, -0x740c
	ctx.r[3].s64 = ctx.r[10].s64 + -29708;
	// 8260F940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F948: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F958: 4BE574C9  bl 0x82466e20
	ctx.lr = 0x8260F95C;
	sub_82466E20(ctx, base);
	// 8260F95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F970 size=100
    let mut pc: u32 = 0x8260F970;
    'dispatch: loop {
        match pc {
            0x8260F970 => {
    //   block [0x8260F970..0x8260F9D4)
	// 8260F970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F97C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F984: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F990: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8260F994: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F9A4: 386A8C24  addi r3, r10, -0x73dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29660;
	// 8260F9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F9B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F9B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F9C0: 4BE57461  bl 0x82466e20
	ctx.lr = 0x8260F9C4;
	sub_82466E20(ctx, base);
	// 8260F9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F9D8 size=112
    let mut pc: u32 = 0x8260F9D8;
    'dispatch: loop {
        match pc {
            0x8260F9D8 => {
    //   block [0x8260F9D8..0x8260FA48)
	// 8260F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F9E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F9E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F9EC: 38AA8BF4  addi r5, r10, -0x740c
	ctx.r[5].s64 = ctx.r[10].s64 + -29708;
	// 8260F9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F9F4: 390B2570  addi r8, r11, 0x2570
	ctx.r[8].s64 = ctx.r[11].s64 + 9584;
	// 8260F9F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260F9FC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8260FA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FA04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FA08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FA10: 386A8C54  addi r3, r10, -0x73ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29612;
	// 8260FA14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FA34: 4BE573ED  bl 0x82466e20
	ctx.lr = 0x8260FA38;
	sub_82466E20(ctx, base);
	// 8260FA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FA48 size=112
    let mut pc: u32 = 0x8260FA48;
    'dispatch: loop {
        match pc {
            0x8260FA48 => {
    //   block [0x8260FA48..0x8260FAB8)
	// 8260FA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FA58: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FA5C: 38AA8C24  addi r5, r10, -0x73dc
	ctx.r[5].s64 = ctx.r[10].s64 + -29660;
	// 8260FA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FA64: 390B25D0  addi r8, r11, 0x25d0
	ctx.r[8].s64 = ctx.r[11].s64 + 9680;
	// 8260FA68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260FA6C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8260FA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FA74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FA80: 386A8C84  addi r3, r10, -0x737c
	ctx.r[3].s64 = ctx.r[10].s64 + -29564;
	// 8260FA84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FA94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FAA4: 4BE5737D  bl 0x82466e20
	ctx.lr = 0x8260FAA8;
	sub_82466E20(ctx, base);
	// 8260FAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FAB8 size=100
    let mut pc: u32 = 0x8260FAB8;
    'dispatch: loop {
        match pc {
            0x8260FAB8 => {
    //   block [0x8260FAB8..0x8260FB1C)
	// 8260FAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FAC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FACC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260FAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FAD8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8260FADC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FAEC: 386A8CB4  addi r3, r10, -0x734c
	ctx.r[3].s64 = ctx.r[10].s64 + -29516;
	// 8260FAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FAF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FAF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260FAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FB00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260FB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FB08: 4BE57319  bl 0x82466e20
	ctx.lr = 0x8260FB0C;
	sub_82466E20(ctx, base);
	// 8260FB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FB20 size=112
    let mut pc: u32 = 0x8260FB20;
    'dispatch: loop {
        match pc {
            0x8260FB20 => {
    //   block [0x8260FB20..0x8260FB90)
	// 8260FB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FB2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FB30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FB34: 38AA8CB4  addi r5, r10, -0x734c
	ctx.r[5].s64 = ctx.r[10].s64 + -29516;
	// 8260FB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FB3C: 390B2630  addi r8, r11, 0x2630
	ctx.r[8].s64 = ctx.r[11].s64 + 9776;
	// 8260FB40: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8260FB44: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8260FB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FB4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FB50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FB58: 386A8CE4  addi r3, r10, -0x731c
	ctx.r[3].s64 = ctx.r[10].s64 + -29468;
	// 8260FB5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FB7C: 4BE572A5  bl 0x82466e20
	ctx.lr = 0x8260FB80;
	sub_82466E20(ctx, base);
	// 8260FB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FB90 size=108
    let mut pc: u32 = 0x8260FB90;
    'dispatch: loop {
        match pc {
            0x8260FB90 => {
    //   block [0x8260FB90..0x8260FBFC)
	// 8260FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FB9C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FBA4: 38EB2720  addi r7, r11, 0x2720
	ctx.r[7].s64 = ctx.r[11].s64 + 10016;
	// 8260FBA8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8260FBAC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8260FBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FBB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FBC0: 386A8D14  addi r3, r10, -0x72ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29420;
	// 8260FBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FBE8: 4BE57239  bl 0x82466e20
	ctx.lr = 0x8260FBEC;
	sub_82466E20(ctx, base);
	// 8260FBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FC00 size=108
    let mut pc: u32 = 0x8260FC00;
    'dispatch: loop {
        match pc {
            0x8260FC00 => {
    //   block [0x8260FC00..0x8260FC6C)
	// 8260FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FC0C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FC14: 38EB2810  addi r7, r11, 0x2810
	ctx.r[7].s64 = ctx.r[11].s64 + 10256;
	// 8260FC18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260FC1C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8260FC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FC24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FC30: 386A8D44  addi r3, r10, -0x72bc
	ctx.r[3].s64 = ctx.r[10].s64 + -29372;
	// 8260FC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FC58: 4BE571C9  bl 0x82466e20
	ctx.lr = 0x8260FC5C;
	sub_82466E20(ctx, base);
	// 8260FC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FC70 size=108
    let mut pc: u32 = 0x8260FC70;
    'dispatch: loop {
        match pc {
            0x8260FC70 => {
    //   block [0x8260FC70..0x8260FCDC)
	// 8260FC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FC7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FC84: 38EB2858  addi r7, r11, 0x2858
	ctx.r[7].s64 = ctx.r[11].s64 + 10328;
	// 8260FC88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8260FC8C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8260FC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FC94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FCA0: 386A8D74  addi r3, r10, -0x728c
	ctx.r[3].s64 = ctx.r[10].s64 + -29324;
	// 8260FCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FCC8: 4BE57159  bl 0x82466e20
	ctx.lr = 0x8260FCCC;
	sub_82466E20(ctx, base);
	// 8260FCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FCE0 size=108
    let mut pc: u32 = 0x8260FCE0;
    'dispatch: loop {
        match pc {
            0x8260FCE0 => {
    //   block [0x8260FCE0..0x8260FD4C)
	// 8260FCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FCEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FCF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FCF4: 38EB2930  addi r7, r11, 0x2930
	ctx.r[7].s64 = ctx.r[11].s64 + 10544;
	// 8260FCF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260FCFC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8260FD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FD04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FD08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FD0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FD10: 386A8DA4  addi r3, r10, -0x725c
	ctx.r[3].s64 = ctx.r[10].s64 + -29276;
	// 8260FD14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FD18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FD38: 4BE570E9  bl 0x82466e20
	ctx.lr = 0x8260FD3C;
	sub_82466E20(ctx, base);
	// 8260FD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FD50 size=100
    let mut pc: u32 = 0x8260FD50;
    'dispatch: loop {
        match pc {
            0x8260FD50 => {
    //   block [0x8260FD50..0x8260FDB4)
	// 8260FD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FD5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FD64: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260FD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FD70: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8260FD74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FD84: 386A8DD4  addi r3, r10, -0x722c
	ctx.r[3].s64 = ctx.r[10].s64 + -29228;
	// 8260FD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260FD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FD98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260FD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FDA0: 4BE57081  bl 0x82466e20
	ctx.lr = 0x8260FDA4;
	sub_82466E20(ctx, base);
	// 8260FDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FDB8 size=112
    let mut pc: u32 = 0x8260FDB8;
    'dispatch: loop {
        match pc {
            0x8260FDB8 => {
    //   block [0x8260FDB8..0x8260FE28)
	// 8260FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FDC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FDC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FDCC: 38AA8DD4  addi r5, r10, -0x722c
	ctx.r[5].s64 = ctx.r[10].s64 + -29228;
	// 8260FDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FDD4: 390B2948  addi r8, r11, 0x2948
	ctx.r[8].s64 = ctx.r[11].s64 + 10568;
	// 8260FDD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260FDDC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8260FDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FDE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FDF0: 386A8E04  addi r3, r10, -0x71fc
	ctx.r[3].s64 = ctx.r[10].s64 + -29180;
	// 8260FDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FE14: 4BE5700D  bl 0x82466e20
	ctx.lr = 0x8260FE18;
	sub_82466E20(ctx, base);
	// 8260FE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FE28 size=108
    let mut pc: u32 = 0x8260FE28;
    'dispatch: loop {
        match pc {
            0x8260FE28 => {
    //   block [0x8260FE28..0x8260FE94)
	// 8260FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FE34: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FE38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FE3C: 38EB2990  addi r7, r11, 0x2990
	ctx.r[7].s64 = ctx.r[11].s64 + 10640;
	// 8260FE40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260FE44: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8260FE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FE4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FE50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FE58: 386A8E34  addi r3, r10, -0x71cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29132;
	// 8260FE5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FE7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FE80: 4BE56FA1  bl 0x82466e20
	ctx.lr = 0x8260FE84;
	sub_82466E20(ctx, base);
	// 8260FE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FE98 size=112
    let mut pc: u32 = 0x8260FE98;
    'dispatch: loop {
        match pc {
            0x8260FE98 => {
    //   block [0x8260FE98..0x8260FF08)
	// 8260FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FEA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FEAC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260FEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FEB4: 390B29D8  addi r8, r11, 0x29d8
	ctx.r[8].s64 = ctx.r[11].s64 + 10712;
	// 8260FEB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260FEBC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8260FEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FED0: 386A8E64  addi r3, r10, -0x719c
	ctx.r[3].s64 = ctx.r[10].s64 + -29084;
	// 8260FED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FEF4: 4BE56F2D  bl 0x82466e20
	ctx.lr = 0x8260FEF8;
	sub_82466E20(ctx, base);
	// 8260FEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FF08 size=108
    let mut pc: u32 = 0x8260FF08;
    'dispatch: loop {
        match pc {
            0x8260FF08 => {
    //   block [0x8260FF08..0x8260FF74)
	// 8260FF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FF14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FF1C: 38EB29F0  addi r7, r11, 0x29f0
	ctx.r[7].s64 = ctx.r[11].s64 + 10736;
	// 8260FF20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260FF24: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8260FF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FF2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FF38: 386A8E94  addi r3, r10, -0x716c
	ctx.r[3].s64 = ctx.r[10].s64 + -29036;
	// 8260FF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FF60: 4BE56EC1  bl 0x82466e20
	ctx.lr = 0x8260FF64;
	sub_82466E20(ctx, base);
	// 8260FF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FF78 size=112
    let mut pc: u32 = 0x8260FF78;
    'dispatch: loop {
        match pc {
            0x8260FF78 => {
    //   block [0x8260FF78..0x8260FFE8)
	// 8260FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FF84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FF88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FF8C: 38AA8E64  addi r5, r10, -0x719c
	ctx.r[5].s64 = ctx.r[10].s64 + -29084;
	// 8260FF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FF94: 390B2A38  addi r8, r11, 0x2a38
	ctx.r[8].s64 = ctx.r[11].s64 + 10808;
	// 8260FF98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260FF9C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8260FFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FFA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FFB0: 386A8EC4  addi r3, r10, -0x713c
	ctx.r[3].s64 = ctx.r[10].s64 + -28988;
	// 8260FFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FFD4: 4BE56E4D  bl 0x82466e20
	ctx.lr = 0x8260FFD8;
	sub_82466E20(ctx, base);
	// 8260FFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FFE8 size=100
    let mut pc: u32 = 0x8260FFE8;
    'dispatch: loop {
        match pc {
            0x8260FFE8 => {
    //   block [0x8260FFE8..0x8261004C)
	// 8260FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FFF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FFFC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610008: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8261000C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261001C: 386A8EF4  addi r3, r10, -0x710c
	ctx.r[3].s64 = ctx.r[10].s64 + -28940;
	// 82610020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261002C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82610034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610038: 4BE56DE9  bl 0x82466e20
	ctx.lr = 0x8261003C;
	sub_82466E20(ctx, base);
	// 8261003C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610050 size=112
    let mut pc: u32 = 0x82610050;
    'dispatch: loop {
        match pc {
            0x82610050 => {
    //   block [0x82610050..0x826100C0)
	// 82610050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261005C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610060: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610064: 38AA8EF4  addi r5, r10, -0x710c
	ctx.r[5].s64 = ctx.r[10].s64 + -28940;
	// 82610068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261006C: 390B2A50  addi r8, r11, 0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + 10832;
	// 82610070: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82610074: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82610078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261007C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610088: 386A8F24  addi r3, r10, -0x70dc
	ctx.r[3].s64 = ctx.r[10].s64 + -28892;
	// 8261008C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261009C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826100A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826100A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826100A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826100AC: 4BE56D75  bl 0x82466e20
	ctx.lr = 0x826100B0;
	sub_82466E20(ctx, base);
	// 826100B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826100B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826100B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826100BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826100C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826100C0 size=108
    let mut pc: u32 = 0x826100C0;
    'dispatch: loop {
        match pc {
            0x826100C0 => {
    //   block [0x826100C0..0x8261012C)
	// 826100C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826100C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826100C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826100CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826100D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826100D4: 38EB2AF8  addi r7, r11, 0x2af8
	ctx.r[7].s64 = ctx.r[11].s64 + 11000;
	// 826100D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826100DC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826100E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826100E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826100E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826100EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826100F0: 386A8F54  addi r3, r10, -0x70ac
	ctx.r[3].s64 = ctx.r[10].s64 + -28844;
	// 826100F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826100F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826100FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261010C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610118: 4BE56D09  bl 0x82466e20
	ctx.lr = 0x8261011C;
	sub_82466E20(ctx, base);
	// 8261011C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610130 size=112
    let mut pc: u32 = 0x82610130;
    'dispatch: loop {
        match pc {
            0x82610130 => {
    //   block [0x82610130..0x826101A0)
	// 82610130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261013C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82610140: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610144: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261014C: 390B2B28  addi r8, r11, 0x2b28
	ctx.r[8].s64 = ctx.r[11].s64 + 11048;
	// 82610150: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82610154: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82610158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261015C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610168: 386A8F84  addi r3, r10, -0x707c
	ctx.r[3].s64 = ctx.r[10].s64 + -28796;
	// 8261016C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261017C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261018C: 4BE56C95  bl 0x82466e20
	ctx.lr = 0x82610190;
	sub_82466E20(ctx, base);
	// 82610190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261019C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826101A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826101A0 size=112
    let mut pc: u32 = 0x826101A0;
    'dispatch: loop {
        match pc {
            0x826101A0 => {
    //   block [0x826101A0..0x82610210)
	// 826101A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826101A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826101A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826101AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826101B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826101B4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826101B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826101BC: 390B2B70  addi r8, r11, 0x2b70
	ctx.r[8].s64 = ctx.r[11].s64 + 11120;
	// 826101C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826101C4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826101C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826101CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826101D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826101D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826101D8: 386A8FB4  addi r3, r10, -0x704c
	ctx.r[3].s64 = ctx.r[10].s64 + -28748;
	// 826101DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826101E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826101E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826101E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826101EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826101F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826101F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826101F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826101FC: 4BE56C25  bl 0x82466e20
	ctx.lr = 0x82610200;
	sub_82466E20(ctx, base);
	// 82610200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261020C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610210 size=100
    let mut pc: u32 = 0x82610210;
    'dispatch: loop {
        match pc {
            0x82610210 => {
    //   block [0x82610210..0x82610274)
	// 82610210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261021C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82610220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610224: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261022C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610230: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82610234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261023C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610244: 386A8FE4  addi r3, r10, -0x701c
	ctx.r[3].s64 = ctx.r[10].s64 + -28700;
	// 82610248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261024C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82610254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261025C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610260: 4BE56BC1  bl 0x82466e20
	ctx.lr = 0x82610264;
	sub_82466E20(ctx, base);
	// 82610264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261026C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610278 size=112
    let mut pc: u32 = 0x82610278;
    'dispatch: loop {
        match pc {
            0x82610278 => {
    //   block [0x82610278..0x826102E8)
	// 82610278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261027C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610288: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261028C: 38AA8FE4  addi r5, r10, -0x701c
	ctx.r[5].s64 = ctx.r[10].s64 + -28700;
	// 82610290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610294: 390B2BB8  addi r8, r11, 0x2bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 11192;
	// 82610298: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261029C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826102A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826102A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826102A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826102AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826102B0: 386A9014  addi r3, r10, -0x6fec
	ctx.r[3].s64 = ctx.r[10].s64 + -28652;
	// 826102B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826102B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826102BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826102C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826102C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826102C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826102CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826102D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826102D4: 4BE56B4D  bl 0x82466e20
	ctx.lr = 0x826102D8;
	sub_82466E20(ctx, base);
	// 826102D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826102DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826102E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826102E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826102E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826102E8 size=112
    let mut pc: u32 = 0x826102E8;
    'dispatch: loop {
        match pc {
            0x826102E8 => {
    //   block [0x826102E8..0x82610358)
	// 826102E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826102EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826102F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826102F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826102F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826102FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610304: 390B2C00  addi r8, r11, 0x2c00
	ctx.r[8].s64 = ctx.r[11].s64 + 11264;
	// 82610308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261030C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82610310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261031C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610320: 386A9044  addi r3, r10, -0x6fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28604;
	// 82610324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261032C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261033C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610344: 4BE56ADD  bl 0x82466e20
	ctx.lr = 0x82610348;
	sub_82466E20(ctx, base);
	// 82610348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610358 size=112
    let mut pc: u32 = 0x82610358;
    'dispatch: loop {
        match pc {
            0x82610358 => {
    //   block [0x82610358..0x826103C8)
	// 82610358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261035C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610364: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82610368: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261036C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610374: 390B2C18  addi r8, r11, 0x2c18
	ctx.r[8].s64 = ctx.r[11].s64 + 11288;
	// 82610378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261037C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82610380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261038C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610390: 386A9074  addi r3, r10, -0x6f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28556;
	// 82610394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261039C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826103A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826103A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826103A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826103AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826103B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826103B4: 4BE56A6D  bl 0x82466e20
	ctx.lr = 0x826103B8;
	sub_82466E20(ctx, base);
	// 826103B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826103BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826103C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826103C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826103C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826103C8 size=112
    let mut pc: u32 = 0x826103C8;
    'dispatch: loop {
        match pc {
            0x826103C8 => {
    //   block [0x826103C8..0x82610438)
	// 826103C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826103CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826103D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826103D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826103D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826103DC: 38AA9044  addi r5, r10, -0x6fbc
	ctx.r[5].s64 = ctx.r[10].s64 + -28604;
	// 826103E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826103E4: 390B2C30  addi r8, r11, 0x2c30
	ctx.r[8].s64 = ctx.r[11].s64 + 11312;
	// 826103E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826103EC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826103F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826103F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826103F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826103FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610400: 386A90A4  addi r3, r10, -0x6f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -28508;
	// 82610404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261040C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261041C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610424: 4BE569FD  bl 0x82466e20
	ctx.lr = 0x82610428;
	sub_82466E20(ctx, base);
	// 82610428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261042C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610438 size=72
    let mut pc: u32 = 0x82610438;
    'dispatch: loop {
        match pc {
            0x82610438 => {
    //   block [0x82610438..0x82610480)
	// 82610438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261043C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610444: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82610448: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8261044C: 38CBCF78  addi r6, r11, -0x3088
	ctx.r[6].s64 = ctx.r[11].s64 + -12424;
	// 82610450: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82610454: 388BE570  addi r4, r11, -0x1a90
	ctx.r[4].s64 = ctx.r[11].s64 + -6800;
	// 82610458: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261045C: 386B90D4  addi r3, r11, -0x6f2c
	ctx.r[3].s64 = ctx.r[11].s64 + -28460;
	// 82610460: 4BE6B629  bl 0x8247ba88
	ctx.lr = 0x82610464;
	sub_8247BA88(ctx, base);
	// 82610464: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82610468: 386BCD20  addi r3, r11, -0x32e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13024;
	// 8261046C: 4BF226CD  bl 0x82532b38
	ctx.lr = 0x82610470;
	sub_82532B38(ctx, base);
	// 82610470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82610474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261047C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610480 size=108
    let mut pc: u32 = 0x82610480;
    'dispatch: loop {
        match pc {
            0x82610480 => {
    //   block [0x82610480..0x826104EC)
	// 82610480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261048C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610494: 38EB4550  addi r7, r11, 0x4550
	ctx.r[7].s64 = ctx.r[11].s64 + 17744;
	// 82610498: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261049C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826104A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826104A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826104A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826104AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826104B0: 386A90EC  addi r3, r10, -0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + -28436;
	// 826104B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826104B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826104BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826104C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826104C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826104C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826104CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826104D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826104D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826104D8: 4BE56949  bl 0x82466e20
	ctx.lr = 0x826104DC;
	sub_82466E20(ctx, base);
	// 826104DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826104E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826104E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826104E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826104F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826104F0 size=24
    let mut pc: u32 = 0x826104F0;
    'dispatch: loop {
        match pc {
            0x826104F0 => {
    //   block [0x826104F0..0x82610508)
	// 826104F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826104F4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826104F8: 394ACB80  addi r10, r10, -0x3480
	ctx.r[10].s64 = ctx.r[10].s64 + -13440;
	// 826104FC: 816B45C8  lwz r11, 0x45c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17864 as u32) ) } as u64;
	// 82610500: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82610504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610508 size=112
    let mut pc: u32 = 0x82610508;
    'dispatch: loop {
        match pc {
            0x82610508 => {
    //   block [0x82610508..0x82610578)
	// 82610508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610514: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610518: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261051C: 392AED9C  addi r9, r10, -0x1264
	ctx.r[9].s64 = ctx.r[10].s64 + -4708;
	// 82610520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610524: 390BCB80  addi r8, r11, -0x3480
	ctx.r[8].s64 = ctx.r[11].s64 + -13440;
	// 82610528: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261052C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82610530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610534: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261053C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610540: 386A911C  addi r3, r10, -0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -28388;
	// 82610544: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610548: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261055C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610564: 4BE568BD  bl 0x82466e20
	ctx.lr = 0x82610568;
	sub_82466E20(ctx, base);
	// 82610568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261056C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610578 size=108
    let mut pc: u32 = 0x82610578;
    'dispatch: loop {
        match pc {
            0x82610578 => {
    //   block [0x82610578..0x826105E4)
	// 82610578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610584: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261058C: 38EB45CC  addi r7, r11, 0x45cc
	ctx.r[7].s64 = ctx.r[11].s64 + 17868;
	// 82610590: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82610594: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82610598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261059C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826105A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826105A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826105A8: 386A914C  addi r3, r10, -0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -28340;
	// 826105AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826105B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826105B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826105B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826105BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826105C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826105C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826105C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826105CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826105D0: 4BE56851  bl 0x82466e20
	ctx.lr = 0x826105D4;
	sub_82466E20(ctx, base);
	// 826105D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826105D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826105DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826105E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826105E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826105E8 size=108
    let mut pc: u32 = 0x826105E8;
    'dispatch: loop {
        match pc {
            0x826105E8 => {
    //   block [0x826105E8..0x82610654)
	// 826105E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826105EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826105F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826105F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826105F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826105FC: 38EB45FC  addi r7, r11, 0x45fc
	ctx.r[7].s64 = ctx.r[11].s64 + 17916;
	// 82610600: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82610604: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82610608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261060C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610618: 386A917C  addi r3, r10, -0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + -28292;
	// 8261061C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261062C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261063C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610640: 4BE567E1  bl 0x82466e20
	ctx.lr = 0x82610644;
	sub_82466E20(ctx, base);
	// 82610644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261064C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82610658 size=24
    let mut pc: u32 = 0x82610658;
    'dispatch: loop {
        match pc {
            0x82610658 => {
    //   block [0x82610658..0x82610670)
	// 82610658: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261065C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82610660: 394ACBC8  addi r10, r10, -0x3438
	ctx.r[10].s64 = ctx.r[10].s64 + -13368;
	// 82610664: 816B462C  lwz r11, 0x462c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17964 as u32) ) } as u64;
	// 82610668: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261066C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610670 size=116
    let mut pc: u32 = 0x82610670;
    'dispatch: loop {
        match pc {
            0x82610670 => {
    //   block [0x82610670..0x826106E4)
	// 82610670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261067C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82610680: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610684: 390BCBC8  addi r8, r11, -0x3438
	ctx.r[8].s64 = ctx.r[11].s64 + -13368;
	// 82610688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261068C: 392AEDD0  addi r9, r10, -0x1230
	ctx.r[9].s64 = ctx.r[10].s64 + -4656;
	// 82610690: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610694: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82610698: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261069C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826106A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826106A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826106A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826106AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826106B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826106B4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826106B8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826106BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826106C0: 386B91AC  addi r3, r11, -0x6e54
	ctx.r[3].s64 = ctx.r[11].s64 + -28244;
	// 826106C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826106C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826106CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826106D0: 4BE56751  bl 0x82466e20
	ctx.lr = 0x826106D4;
	sub_82466E20(ctx, base);
	// 826106D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826106D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826106DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826106E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826106E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826106E8 size=108
    let mut pc: u32 = 0x826106E8;
    'dispatch: loop {
        match pc {
            0x826106E8 => {
    //   block [0x826106E8..0x82610754)
	// 826106E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826106EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826106F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826106F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826106F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826106FC: 38EB4630  addi r7, r11, 0x4630
	ctx.r[7].s64 = ctx.r[11].s64 + 17968;
	// 82610700: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610704: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82610708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261070C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610718: 386A91DC  addi r3, r10, -0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + -28196;
	// 8261071C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261072C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261073C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610740: 4BE566E1  bl 0x82466e20
	ctx.lr = 0x82610744;
	sub_82466E20(ctx, base);
	// 82610744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261074C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610758 size=112
    let mut pc: u32 = 0x82610758;
    'dispatch: loop {
        match pc {
            0x82610758 => {
    //   block [0x82610758..0x826107C8)
	// 82610758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610768: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261076C: 38AA91AC  addi r5, r10, -0x6e54
	ctx.r[5].s64 = ctx.r[10].s64 + -28244;
	// 82610770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610774: 390B46C0  addi r8, r11, 0x46c0
	ctx.r[8].s64 = ctx.r[11].s64 + 18112;
	// 82610778: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8261077C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82610780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610784: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261078C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610790: 386A920C  addi r3, r10, -0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + -28148;
	// 82610794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261079C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826107A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826107A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826107A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826107AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826107B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826107B4: 4BE5666D  bl 0x82466e20
	ctx.lr = 0x826107B8;
	sub_82466E20(ctx, base);
	// 826107B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826107BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826107C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826107C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826107C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826107C8 size=112
    let mut pc: u32 = 0x826107C8;
    'dispatch: loop {
        match pc {
            0x826107C8 => {
    //   block [0x826107C8..0x82610838)
	// 826107C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826107CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826107D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826107D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826107D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826107DC: 38AA91AC  addi r5, r10, -0x6e54
	ctx.r[5].s64 = ctx.r[10].s64 + -28244;
	// 826107E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826107E4: 390B47E0  addi r8, r11, 0x47e0
	ctx.r[8].s64 = ctx.r[11].s64 + 18400;
	// 826107E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826107EC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826107F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826107F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826107F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826107FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610800: 386A923C  addi r3, r10, -0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -28100;
	// 82610804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261080C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261081C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610824: 4BE565FD  bl 0x82466e20
	ctx.lr = 0x82610828;
	sub_82466E20(ctx, base);
	// 82610828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261082C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610838 size=108
    let mut pc: u32 = 0x82610838;
    'dispatch: loop {
        match pc {
            0x82610838 => {
    //   block [0x82610838..0x826108A4)
	// 82610838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610844: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261084C: 38EB47F8  addi r7, r11, 0x47f8
	ctx.r[7].s64 = ctx.r[11].s64 + 18424;
	// 82610850: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610854: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82610858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261085C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610868: 386A926C  addi r3, r10, -0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + -28052;
	// 8261086C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261087C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261088C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610890: 4BE56591  bl 0x82466e20
	ctx.lr = 0x82610894;
	sub_82466E20(ctx, base);
	// 82610894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261089C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826108A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826108A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826108A8 size=112
    let mut pc: u32 = 0x826108A8;
    'dispatch: loop {
        match pc {
            0x826108A8 => {
    //   block [0x826108A8..0x82610918)
	// 826108A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826108AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826108B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826108B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826108B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826108BC: 38AA91AC  addi r5, r10, -0x6e54
	ctx.r[5].s64 = ctx.r[10].s64 + -28244;
	// 826108C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826108C4: 390B4888  addi r8, r11, 0x4888
	ctx.r[8].s64 = ctx.r[11].s64 + 18568;
	// 826108C8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826108CC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826108D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826108D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826108D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826108DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826108E0: 386A929C  addi r3, r10, -0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + -28004;
	// 826108E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826108E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826108EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826108F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826108F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826108F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826108FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610904: 4BE5651D  bl 0x82466e20
	ctx.lr = 0x82610908;
	sub_82466E20(ctx, base);
	// 82610908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261090C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610918 size=108
    let mut pc: u32 = 0x82610918;
    'dispatch: loop {
        match pc {
            0x82610918 => {
    //   block [0x82610918..0x82610984)
	// 82610918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261091C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610924: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261092C: 38EB4978  addi r7, r11, 0x4978
	ctx.r[7].s64 = ctx.r[11].s64 + 18808;
	// 82610930: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82610934: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82610938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261093C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610940: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610948: 386A92CC  addi r3, r10, -0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + -27956;
	// 8261094C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261095C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261096C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610970: 4BE564B1  bl 0x82466e20
	ctx.lr = 0x82610974;
	sub_82466E20(ctx, base);
	// 82610974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261097C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610988 size=108
    let mut pc: u32 = 0x82610988;
    'dispatch: loop {
        match pc {
            0x82610988 => {
    //   block [0x82610988..0x826109F4)
	// 82610988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610994: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261099C: 38EB4990  addi r7, r11, 0x4990
	ctx.r[7].s64 = ctx.r[11].s64 + 18832;
	// 826109A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826109A4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826109A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826109AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826109B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826109B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826109B8: 386A92FC  addi r3, r10, -0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + -27908;
	// 826109BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826109C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826109C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826109C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826109CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826109D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826109D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826109D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826109DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826109E0: 4BE56441  bl 0x82466e20
	ctx.lr = 0x826109E4;
	sub_82466E20(ctx, base);
	// 826109E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826109E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826109EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826109F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826109F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826109F8 size=116
    let mut pc: u32 = 0x826109F8;
    'dispatch: loop {
        match pc {
            0x826109F8 => {
    //   block [0x826109F8..0x82610A6C)
	// 826109F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826109FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610A04: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610A08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610A0C: 390B49F4  addi r8, r11, 0x49f4
	ctx.r[8].s64 = ctx.r[11].s64 + 18932;
	// 82610A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610A14: 392AEDFC  addi r9, r10, -0x1204
	ctx.r[9].s64 = ctx.r[10].s64 + -4612;
	// 82610A18: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610A1C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82610A20: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610A24: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610A2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610A3C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82610A40: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82610A44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610A48: 386B932C  addi r3, r11, -0x6cd4
	ctx.r[3].s64 = ctx.r[11].s64 + -27860;
	// 82610A4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82610A50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610A58: 4BE563C9  bl 0x82466e20
	ctx.lr = 0x82610A5C;
	sub_82466E20(ctx, base);
	// 82610A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610A70 size=108
    let mut pc: u32 = 0x82610A70;
    'dispatch: loop {
        match pc {
            0x82610A70 => {
    //   block [0x82610A70..0x82610ADC)
	// 82610A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610A7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610A84: 38EB4A10  addi r7, r11, 0x4a10
	ctx.r[7].s64 = ctx.r[11].s64 + 18960;
	// 82610A88: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82610A8C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82610A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610A94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610AA0: 386A935C  addi r3, r10, -0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -27812;
	// 82610AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610AC8: 4BE56359  bl 0x82466e20
	ctx.lr = 0x82610ACC;
	sub_82466E20(ctx, base);
	// 82610ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610AE0 size=108
    let mut pc: u32 = 0x82610AE0;
    'dispatch: loop {
        match pc {
            0x82610AE0 => {
    //   block [0x82610AE0..0x82610B4C)
	// 82610AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610AEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610AF4: 38EB4A58  addi r7, r11, 0x4a58
	ctx.r[7].s64 = ctx.r[11].s64 + 19032;
	// 82610AF8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610AFC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 82610B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610B04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610B10: 386A938C  addi r3, r10, -0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + -27764;
	// 82610B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610B38: 4BE562E9  bl 0x82466e20
	ctx.lr = 0x82610B3C;
	sub_82466E20(ctx, base);
	// 82610B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610B50 size=108
    let mut pc: u32 = 0x82610B50;
    'dispatch: loop {
        match pc {
            0x82610B50 => {
    //   block [0x82610B50..0x82610BBC)
	// 82610B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610B5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610B64: 38EB4AE8  addi r7, r11, 0x4ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 19176;
	// 82610B68: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610B6C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82610B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610B74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610B78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610B80: 386A93BC  addi r3, r10, -0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + -27716;
	// 82610B84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610BA8: 4BE56279  bl 0x82466e20
	ctx.lr = 0x82610BAC;
	sub_82466E20(ctx, base);
	// 82610BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610BC0 size=100
    let mut pc: u32 = 0x82610BC0;
    'dispatch: loop {
        match pc {
            0x82610BC0 => {
    //   block [0x82610BC0..0x82610C24)
	// 82610BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610BCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610BD4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610BE0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 82610BE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610BF4: 386A93EC  addi r3, r10, -0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + -27668;
	// 82610BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610BFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610C00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82610C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610C08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82610C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610C10: 4BE56211  bl 0x82466e20
	ctx.lr = 0x82610C14;
	sub_82466E20(ctx, base);
	// 82610C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610C28 size=112
    let mut pc: u32 = 0x82610C28;
    'dispatch: loop {
        match pc {
            0x82610C28 => {
    //   block [0x82610C28..0x82610C98)
	// 82610C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610C34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610C38: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610C3C: 38AA93EC  addi r5, r10, -0x6c14
	ctx.r[5].s64 = ctx.r[10].s64 + -27668;
	// 82610C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610C44: 390B4B78  addi r8, r11, 0x4b78
	ctx.r[8].s64 = ctx.r[11].s64 + 19320;
	// 82610C48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82610C4C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82610C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610C54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610C60: 386A941C  addi r3, r10, -0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + -27620;
	// 82610C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610C84: 4BE5619D  bl 0x82466e20
	ctx.lr = 0x82610C88;
	sub_82466E20(ctx, base);
	// 82610C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610C98 size=108
    let mut pc: u32 = 0x82610C98;
    'dispatch: loop {
        match pc {
            0x82610C98 => {
    //   block [0x82610C98..0x82610D04)
	// 82610C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610CA4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610CAC: 38EB4BD8  addi r7, r11, 0x4bd8
	ctx.r[7].s64 = ctx.r[11].s64 + 19416;
	// 82610CB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82610CB4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82610CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610CBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610CC8: 386A944C  addi r3, r10, -0x6bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -27572;
	// 82610CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610CF0: 4BE56131  bl 0x82466e20
	ctx.lr = 0x82610CF4;
	sub_82466E20(ctx, base);
	// 82610CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610D08 size=108
    let mut pc: u32 = 0x82610D08;
    'dispatch: loop {
        match pc {
            0x82610D08 => {
    //   block [0x82610D08..0x82610D74)
	// 82610D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610D14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610D1C: 38EB4C08  addi r7, r11, 0x4c08
	ctx.r[7].s64 = ctx.r[11].s64 + 19464;
	// 82610D20: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82610D24: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82610D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610D2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610D38: 386A947C  addi r3, r10, -0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + -27524;
	// 82610D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610D60: 4BE560C1  bl 0x82466e20
	ctx.lr = 0x82610D64;
	sub_82466E20(ctx, base);
	// 82610D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610D78 size=108
    let mut pc: u32 = 0x82610D78;
    'dispatch: loop {
        match pc {
            0x82610D78 => {
    //   block [0x82610D78..0x82610DE4)
	// 82610D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610D84: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610D8C: 38EB4C68  addi r7, r11, 0x4c68
	ctx.r[7].s64 = ctx.r[11].s64 + 19560;
	// 82610D90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82610D94: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82610D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610D9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610DA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610DA8: 386A94AC  addi r3, r10, -0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + -27476;
	// 82610DAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610DD0: 4BE56051  bl 0x82466e20
	ctx.lr = 0x82610DD4;
	sub_82466E20(ctx, base);
	// 82610DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82610DE8 size=24
    let mut pc: u32 = 0x82610DE8;
    'dispatch: loop {
        match pc {
            0x82610DE8 => {
    //   block [0x82610DE8..0x82610E00)
	// 82610DE8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610DEC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82610DF0: 394ACC40  addi r10, r10, -0x33c0
	ctx.r[10].s64 = ctx.r[10].s64 + -13248;
	// 82610DF4: 816B4A0C  lwz r11, 0x4a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18956 as u32) ) } as u64;
	// 82610DF8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 82610DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610E00 size=116
    let mut pc: u32 = 0x82610E00;
    'dispatch: loop {
        match pc {
            0x82610E00 => {
    //   block [0x82610E00..0x82610E74)
	// 82610E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610E0C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82610E10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610E14: 390BCC40  addi r8, r11, -0x33c0
	ctx.r[8].s64 = ctx.r[11].s64 + -13248;
	// 82610E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610E1C: 392AEE30  addi r9, r10, -0x11d0
	ctx.r[9].s64 = ctx.r[10].s64 + -4560;
	// 82610E20: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610E24: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82610E28: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610E2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610E34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610E44: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82610E48: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82610E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610E50: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 82610E54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82610E58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610E60: 4BE55FC1  bl 0x82466e20
	ctx.lr = 0x82610E64;
	sub_82466E20(ctx, base);
	// 82610E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610E78 size=112
    let mut pc: u32 = 0x82610E78;
    'dispatch: loop {
        match pc {
            0x82610E78 => {
    //   block [0x82610E78..0x82610EE8)
	// 82610E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610E84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610E88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610E8C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610E94: 390B4CC8  addi r8, r11, 0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 19656;
	// 82610E98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82610E9C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 82610EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610EB0: 386A950C  addi r3, r10, -0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + -27380;
	// 82610EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610ED4: 4BE55F4D  bl 0x82466e20
	ctx.lr = 0x82610ED8;
	sub_82466E20(ctx, base);
	// 82610ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610EE8 size=112
    let mut pc: u32 = 0x82610EE8;
    'dispatch: loop {
        match pc {
            0x82610EE8 => {
    //   block [0x82610EE8..0x82610F58)
	// 82610EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610EF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610EF8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610EFC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610F04: 390B4D10  addi r8, r11, 0x4d10
	ctx.r[8].s64 = ctx.r[11].s64 + 19728;
	// 82610F08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82610F0C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 82610F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610F20: 386A953C  addi r3, r10, -0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -27332;
	// 82610F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610F44: 4BE55EDD  bl 0x82466e20
	ctx.lr = 0x82610F48;
	sub_82466E20(ctx, base);
	// 82610F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82610F58 size=24
    let mut pc: u32 = 0x82610F58;
    'dispatch: loop {
        match pc {
            0x82610F58 => {
    //   block [0x82610F58..0x82610F70)
	// 82610F58: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610F5C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82610F60: 394ACD78  addi r10, r10, -0x3288
	ctx.r[10].s64 = ctx.r[10].s64 + -12936;
	// 82610F64: 816B4D58  lwz r11, 0x4d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19800 as u32) ) } as u64;
	// 82610F68: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82610F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610F70 size=116
    let mut pc: u32 = 0x82610F70;
    'dispatch: loop {
        match pc {
            0x82610F70 => {
    //   block [0x82610F70..0x82610FE4)
	// 82610F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610F7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82610F80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610F84: 390BCD78  addi r8, r11, -0x3288
	ctx.r[8].s64 = ctx.r[11].s64 + -12936;
	// 82610F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610F8C: 392AEE5C  addi r9, r10, -0x11a4
	ctx.r[9].s64 = ctx.r[10].s64 + -4516;
	// 82610F90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610F94: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82610F98: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82610F9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610FA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610FB4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82610FB8: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82610FBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610FC0: 386B956C  addi r3, r11, -0x6a94
	ctx.r[3].s64 = ctx.r[11].s64 + -27284;
	// 82610FC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82610FC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610FD0: 4BE55E51  bl 0x82466e20
	ctx.lr = 0x82610FD4;
	sub_82466E20(ctx, base);
	// 82610FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610FE8 size=108
    let mut pc: u32 = 0x82610FE8;
    'dispatch: loop {
        match pc {
            0x82610FE8 => {
    //   block [0x82610FE8..0x82611054)
	// 82610FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610FF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610FFC: 38EB4D60  addi r7, r11, 0x4d60
	ctx.r[7].s64 = ctx.r[11].s64 + 19808;
	// 82611000: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82611004: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 82611008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261100C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611018: 386A959C  addi r3, r10, -0x6a64
	ctx.r[3].s64 = ctx.r[10].s64 + -27236;
	// 8261101C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261102C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261103C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611040: 4BE55DE1  bl 0x82466e20
	ctx.lr = 0x82611044;
	sub_82466E20(ctx, base);
	// 82611044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261104C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611058 size=108
    let mut pc: u32 = 0x82611058;
    'dispatch: loop {
        match pc {
            0x82611058 => {
    //   block [0x82611058..0x826110C4)
	// 82611058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611064: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261106C: 38EB4DC0  addi r7, r11, 0x4dc0
	ctx.r[7].s64 = ctx.r[11].s64 + 19904;
	// 82611070: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82611074: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 82611078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261107C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611088: 386A95CC  addi r3, r10, -0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + -27188;
	// 8261108C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261109C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826110A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826110A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826110A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826110AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826110B0: 4BE55D71  bl 0x82466e20
	ctx.lr = 0x826110B4;
	sub_82466E20(ctx, base);
	// 826110B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826110B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826110BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826110C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826110C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826110C8 size=112
    let mut pc: u32 = 0x826110C8;
    'dispatch: loop {
        match pc {
            0x826110C8 => {
    //   block [0x826110C8..0x82611138)
	// 826110C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826110CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826110D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826110D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826110D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826110DC: 392BEE90  addi r9, r11, -0x1170
	ctx.r[9].s64 = ctx.r[11].s64 + -4464;
	// 826110E0: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826110E4: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826110E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826110EC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826110F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826110F4: 396B4E68  addi r11, r11, 0x4e68
	ctx.r[11].s64 = ctx.r[11].s64 + 20072;
	// 826110F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826110FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611100: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82611104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611108: 386A95FC  addi r3, r10, -0x6a04
	ctx.r[3].s64 = ctx.r[10].s64 + -27140;
	// 8261110C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82611110: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82611114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611118: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261111C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82611124: 4BE55CFD  bl 0x82466e20
	ctx.lr = 0x82611128;
	sub_82466E20(ctx, base);
	// 82611128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261112C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611138 size=112
    let mut pc: u32 = 0x82611138;
    'dispatch: loop {
        match pc {
            0x82611138 => {
    //   block [0x82611138..0x826111A8)
	// 82611138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261113C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611148: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261114C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82611150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611154: 390B4FB8  addi r8, r11, 0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20408;
	// 82611158: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261115C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82611160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261116C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611170: 386A962C  addi r3, r10, -0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + -27092;
	// 82611174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261117C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611194: 4BE55C8D  bl 0x82466e20
	ctx.lr = 0x82611198;
	sub_82466E20(ctx, base);
	// 82611198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261119C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826111A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826111A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826111A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826111A8 size=112
    let mut pc: u32 = 0x826111A8;
    'dispatch: loop {
        match pc {
            0x826111A8 => {
    //   block [0x826111A8..0x82611218)
	// 826111A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826111AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826111B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826111B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826111B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826111BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826111C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826111C4: 390B5060  addi r8, r11, 0x5060
	ctx.r[8].s64 = ctx.r[11].s64 + 20576;
	// 826111C8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826111CC: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826111D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826111D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826111D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826111DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826111E0: 386A965C  addi r3, r10, -0x69a4
	ctx.r[3].s64 = ctx.r[10].s64 + -27044;
	// 826111E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826111E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826111EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826111F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826111F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826111F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826111FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611204: 4BE55C1D  bl 0x82466e20
	ctx.lr = 0x82611208;
	sub_82466E20(ctx, base);
	// 82611208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261120C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611218 size=112
    let mut pc: u32 = 0x82611218;
    'dispatch: loop {
        match pc {
            0x82611218 => {
    //   block [0x82611218..0x82611288)
	// 82611218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261121C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611228: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261122C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82611230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611234: 390B50F0  addi r8, r11, 0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + 20720;
	// 82611238: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261123C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82611240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261124C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611250: 386A968C  addi r3, r10, -0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + -26996;
	// 82611254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261125C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261126C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611274: 4BE55BAD  bl 0x82466e20
	ctx.lr = 0x82611278;
	sub_82466E20(ctx, base);
	// 82611278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261127C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611288 size=108
    let mut pc: u32 = 0x82611288;
    'dispatch: loop {
        match pc {
            0x82611288 => {
    //   block [0x82611288..0x826112F4)
	// 82611288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611294: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261129C: 38EB5168  addi r7, r11, 0x5168
	ctx.r[7].s64 = ctx.r[11].s64 + 20840;
	// 826112A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826112A4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826112A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826112AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826112B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826112B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826112B8: 386A96BC  addi r3, r10, -0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + -26948;
	// 826112BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826112C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826112C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826112C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826112CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826112D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826112D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826112D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826112DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826112E0: 4BE55B41  bl 0x82466e20
	ctx.lr = 0x826112E4;
	sub_82466E20(ctx, base);
	// 826112E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826112E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826112EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826112F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826112F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826112F8 size=112
    let mut pc: u32 = 0x826112F8;
    'dispatch: loop {
        match pc {
            0x826112F8 => {
    //   block [0x826112F8..0x82611368)
	// 826112F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826112FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611304: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611308: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261130C: 392AEEF0  addi r9, r10, -0x1110
	ctx.r[9].s64 = ctx.r[10].s64 + -4368;
	// 82611310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611314: 390B5214  addi r8, r11, 0x5214
	ctx.r[8].s64 = ctx.r[11].s64 + 21012;
	// 82611318: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8261131C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82611320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261132C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611330: 386A96EC  addi r3, r10, -0x6914
	ctx.r[3].s64 = ctx.r[10].s64 + -26900;
	// 82611334: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82611338: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261133C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261134C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611354: 4BE55ACD  bl 0x82466e20
	ctx.lr = 0x82611358;
	sub_82466E20(ctx, base);
	// 82611358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261135C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611368 size=100
    let mut pc: u32 = 0x82611368;
    'dispatch: loop {
        match pc {
            0x82611368 => {
    //   block [0x82611368..0x826113CC)
	// 82611368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261136C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261137C: 38AA9ECC  addi r5, r10, -0x6134
	ctx.r[5].s64 = ctx.r[10].s64 + -24884;
	// 82611380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611388: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8261138C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261139C: 386A971C  addi r3, r10, -0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26852;
	// 826113A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826113A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826113A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826113AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826113B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826113B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826113B8: 4BE55A69  bl 0x82466e20
	ctx.lr = 0x826113BC;
	sub_82466E20(ctx, base);
	// 826113BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826113C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826113C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826113C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826113D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826113D0 size=108
    let mut pc: u32 = 0x826113D0;
    'dispatch: loop {
        match pc {
            0x826113D0 => {
    //   block [0x826113D0..0x8261143C)
	// 826113D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826113D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826113D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826113DC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826113E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826113E4: 38EB5248  addi r7, r11, 0x5248
	ctx.r[7].s64 = ctx.r[11].s64 + 21064;
	// 826113E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826113EC: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 826113F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826113F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826113F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826113FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611400: 386A974C  addi r3, r10, -0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26804;
	// 82611404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261140C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261141C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611428: 4BE559F9  bl 0x82466e20
	ctx.lr = 0x8261142C;
	sub_82466E20(ctx, base);
	// 8261142C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611440 size=112
    let mut pc: u32 = 0x82611440;
    'dispatch: loop {
        match pc {
            0x82611440 => {
    //   block [0x82611440..0x826114B0)
	// 82611440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261144C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611450: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611454: 392AEF50  addi r9, r10, -0x10b0
	ctx.r[9].s64 = ctx.r[10].s64 + -4272;
	// 82611458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261145C: 390B5278  addi r8, r11, 0x5278
	ctx.r[8].s64 = ctx.r[11].s64 + 21112;
	// 82611460: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82611464: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82611468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261146C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611478: 386A977C  addi r3, r10, -0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + -26756;
	// 8261147C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82611480: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82611484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261148C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261149C: 4BE55985  bl 0x82466e20
	ctx.lr = 0x826114A0;
	sub_82466E20(ctx, base);
	// 826114A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826114A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826114A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826114AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826114B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826114B0 size=112
    let mut pc: u32 = 0x826114B0;
    'dispatch: loop {
        match pc {
            0x826114B0 => {
    //   block [0x826114B0..0x82611520)
	// 826114B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826114B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826114B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826114BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826114C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826114C4: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 826114C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826114CC: 390B52F0  addi r8, r11, 0x52f0
	ctx.r[8].s64 = ctx.r[11].s64 + 21232;
	// 826114D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826114D4: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826114D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826114DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826114E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826114E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826114E8: 386A97AC  addi r3, r10, -0x6854
	ctx.r[3].s64 = ctx.r[10].s64 + -26708;
	// 826114EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826114F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826114F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826114F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826114FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261150C: 4BE55915  bl 0x82466e20
	ctx.lr = 0x82611510;
	sub_82466E20(ctx, base);
	// 82611510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611520 size=116
    let mut pc: u32 = 0x82611520;
    'dispatch: loop {
        match pc {
            0x82611520 => {
    //   block [0x82611520..0x82611594)
	// 82611520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261152C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611530: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82611534: 390A5320  addi r8, r10, 0x5320
	ctx.r[8].s64 = ctx.r[10].s64 + 21280;
	// 82611538: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261153C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611540: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82611544: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611548: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261154C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611554: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82611558: 396BEF64  addi r11, r11, -0x109c
	ctx.r[11].s64 = ctx.r[11].s64 + -4252;
	// 8261155C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611564: 386A97DC  addi r3, r10, -0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + -26660;
	// 82611568: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261156C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611570: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261157C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611580: 4BE558A1  bl 0x82466e20
	ctx.lr = 0x82611584;
	sub_82466E20(ctx, base);
	// 82611584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261158C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611598 size=100
    let mut pc: u32 = 0x82611598;
    'dispatch: loop {
        match pc {
            0x82611598 => {
    //   block [0x82611598..0x826115FC)
	// 82611598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826115A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826115A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826115A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826115AC: 38AA97DC  addi r5, r10, -0x6824
	ctx.r[5].s64 = ctx.r[10].s64 + -26660;
	// 826115B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826115B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826115B8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826115BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826115C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826115C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826115C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826115CC: 386A980C  addi r3, r10, -0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + -26612;
	// 826115D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826115D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826115D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826115DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826115E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826115E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826115E8: 4BE55839  bl 0x82466e20
	ctx.lr = 0x826115EC;
	sub_82466E20(ctx, base);
	// 826115EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826115F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826115F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826115F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82611600 size=24
    let mut pc: u32 = 0x82611600;
    'dispatch: loop {
        match pc {
            0x82611600 => {
    //   block [0x82611600..0x82611618)
	// 82611600: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611604: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82611608: 394ACE80  addi r10, r10, -0x3180
	ctx.r[10].s64 = ctx.r[10].s64 + -12672;
	// 8261160C: 816B53C8  lwz r11, 0x53c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21448 as u32) ) } as u64;
	// 82611610: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82611614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611618 size=116
    let mut pc: u32 = 0x82611618;
    'dispatch: loop {
        match pc {
            0x82611618 => {
    //   block [0x82611618..0x8261168C)
	// 82611618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611624: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611628: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261162C: 392BEFA0  addi r9, r11, -0x1060
	ctx.r[9].s64 = ctx.r[11].s64 + -4192;
	// 82611630: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82611634: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611638: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8261163C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82611640: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82611644: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82611648: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261164C: 396BCE80  addi r11, r11, -0x3180
	ctx.r[11].s64 = ctx.r[11].s64 + -12672;
	// 82611650: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82611654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611658: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8261165C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611660: 386A983C  addi r3, r10, -0x67c4
	ctx.r[3].s64 = ctx.r[10].s64 + -26564;
	// 82611664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82611668: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8261166C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611670: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82611674: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82611678: 4BE557A9  bl 0x82466e20
	ctx.lr = 0x8261167C;
	sub_82466E20(ctx, base);
	// 8261167C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611690 size=116
    let mut pc: u32 = 0x82611690;
    'dispatch: loop {
        match pc {
            0x82611690 => {
    //   block [0x82611690..0x82611704)
	// 82611690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261169C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826116A0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826116A4: 392BEFF4  addi r9, r11, -0x100c
	ctx.r[9].s64 = ctx.r[11].s64 + -4108;
	// 826116A8: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 826116AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826116B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826116B4: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826116B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826116BC: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826116C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826116C4: 396B53D0  addi r11, r11, 0x53d0
	ctx.r[11].s64 = ctx.r[11].s64 + 21456;
	// 826116C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826116CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826116D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826116D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826116D8: 386A986C  addi r3, r10, -0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + -26516;
	// 826116DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826116E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826116E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826116E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826116EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826116F0: 4BE55731  bl 0x82466e20
	ctx.lr = 0x826116F4;
	sub_82466E20(ctx, base);
	// 826116F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826116F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826116FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611708 size=108
    let mut pc: u32 = 0x82611708;
    'dispatch: loop {
        match pc {
            0x82611708 => {
    //   block [0x82611708..0x82611774)
	// 82611708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611714: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261171C: 38EB54A8  addi r7, r11, 0x54a8
	ctx.r[7].s64 = ctx.r[11].s64 + 21672;
	// 82611720: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82611724: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82611728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261172C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611738: 386A989C  addi r3, r10, -0x6764
	ctx.r[3].s64 = ctx.r[10].s64 + -26468;
	// 8261173C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261175C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611760: 4BE556C1  bl 0x82466e20
	ctx.lr = 0x82611764;
	sub_82466E20(ctx, base);
	// 82611764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261176C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82611778 size=24
    let mut pc: u32 = 0x82611778;
    'dispatch: loop {
        match pc {
            0x82611778 => {
    //   block [0x82611778..0x82611790)
	// 82611778: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261177C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82611780: 394ACF28  addi r10, r10, -0x30d8
	ctx.r[10].s64 = ctx.r[10].s64 + -12504;
	// 82611784: 816B5508  lwz r11, 0x5508(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21768 as u32) ) } as u64;
	// 82611788: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611790 size=116
    let mut pc: u32 = 0x82611790;
    'dispatch: loop {
        match pc {
            0x82611790 => {
    //   block [0x82611790..0x82611804)
	// 82611790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261179C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826117A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826117A4: 390BCF28  addi r8, r11, -0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + -12504;
	// 826117A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826117AC: 392AF068  addi r9, r10, -0xf98
	ctx.r[9].s64 = ctx.r[10].s64 + -3992;
	// 826117B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826117B4: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826117B8: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 826117BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826117C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826117C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826117C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826117CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826117D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826117D4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826117D8: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826117DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826117E0: 386B98CC  addi r3, r11, -0x6734
	ctx.r[3].s64 = ctx.r[11].s64 + -26420;
	// 826117E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826117E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826117EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826117F0: 4BE55631  bl 0x82466e20
	ctx.lr = 0x826117F4;
	sub_82466E20(ctx, base);
	// 826117F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826117F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826117FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611808 size=112
    let mut pc: u32 = 0x82611808;
    'dispatch: loop {
        match pc {
            0x82611808 => {
    //   block [0x82611808..0x82611878)
	// 82611808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261180C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611818: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261181C: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82611820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611824: 390B5510  addi r8, r11, 0x5510
	ctx.r[8].s64 = ctx.r[11].s64 + 21776;
	// 82611828: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261182C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82611830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261183C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611840: 386A98FC  addi r3, r10, -0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + -26372;
	// 82611844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261184C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261185C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611864: 4BE555BD  bl 0x82466e20
	ctx.lr = 0x82611868;
	sub_82466E20(ctx, base);
	// 82611868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261186C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82611878 size=24
    let mut pc: u32 = 0x82611878;
    'dispatch: loop {
        match pc {
            0x82611878 => {
    //   block [0x82611878..0x82611890)
	// 82611878: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261187C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82611880: 394AD0C0  addi r10, r10, -0x2f40
	ctx.r[10].s64 = ctx.r[10].s64 + -12096;
	// 82611884: 816B5540  lwz r11, 0x5540(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21824 as u32) ) } as u64;
	// 82611888: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261188C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611890 size=116
    let mut pc: u32 = 0x82611890;
    'dispatch: loop {
        match pc {
            0x82611890 => {
    //   block [0x82611890..0x82611904)
	// 82611890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261189C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826118A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826118A4: 390BD0C0  addi r8, r11, -0x2f40
	ctx.r[8].s64 = ctx.r[11].s64 + -12096;
	// 826118A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826118AC: 392AF0A0  addi r9, r10, -0xf60
	ctx.r[9].s64 = ctx.r[10].s64 + -3936;
	// 826118B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826118B4: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826118B8: 38AA986C  addi r5, r10, -0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + -26516;
	// 826118BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826118C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826118C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826118C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826118CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826118D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826118D4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826118D8: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826118DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826118E0: 386B992C  addi r3, r11, -0x66d4
	ctx.r[3].s64 = ctx.r[11].s64 + -26324;
	// 826118E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826118E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826118EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826118F0: 4BE55531  bl 0x82466e20
	ctx.lr = 0x826118F4;
	sub_82466E20(ctx, base);
	// 826118F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826118F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826118FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611908 size=112
    let mut pc: u32 = 0x82611908;
    'dispatch: loop {
        match pc {
            0x82611908 => {
    //   block [0x82611908..0x82611978)
	// 82611908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611914: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611918: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261191C: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82611920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611924: 390B5544  addi r8, r11, 0x5544
	ctx.r[8].s64 = ctx.r[11].s64 + 21828;
	// 82611928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261192C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82611930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261193C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611940: 386A995C  addi r3, r10, -0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + -26276;
	// 82611944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261194C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261195C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611964: 4BE554BD  bl 0x82466e20
	ctx.lr = 0x82611968;
	sub_82466E20(ctx, base);
	// 82611968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611978 size=100
    let mut pc: u32 = 0x82611978;
    'dispatch: loop {
        match pc {
            0x82611978 => {
    //   block [0x82611978..0x826119DC)
	// 82611978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611984: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261198C: 38AA9ECC  addi r5, r10, -0x6134
	ctx.r[5].s64 = ctx.r[10].s64 + -24884;
	// 82611990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611998: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8261199C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826119A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826119A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826119A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826119AC: 386A998C  addi r3, r10, -0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + -26228;
	// 826119B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826119B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826119B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826119BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826119C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826119C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826119C8: 4BE55459  bl 0x82466e20
	ctx.lr = 0x826119CC;
	sub_82466E20(ctx, base);
	// 826119CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826119D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826119D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826119D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826119E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826119E0 size=108
    let mut pc: u32 = 0x826119E0;
    'dispatch: loop {
        match pc {
            0x826119E0 => {
    //   block [0x826119E0..0x82611A4C)
	// 826119E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826119E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826119E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826119EC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826119F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826119F4: 38EB5560  addi r7, r11, 0x5560
	ctx.r[7].s64 = ctx.r[11].s64 + 21856;
	// 826119F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826119FC: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 82611A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611A04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611A10: 386A99BC  addi r3, r10, -0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + -26180;
	// 82611A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611A38: 4BE553E9  bl 0x82466e20
	ctx.lr = 0x82611A3C;
	sub_82466E20(ctx, base);
	// 82611A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611A50 size=112
    let mut pc: u32 = 0x82611A50;
    'dispatch: loop {
        match pc {
            0x82611A50 => {
    //   block [0x82611A50..0x82611AC0)
	// 82611A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611A5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611A60: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611A64: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611A6C: 390B5638  addi r8, r11, 0x5638
	ctx.r[8].s64 = ctx.r[11].s64 + 22072;
	// 82611A70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82611A74: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82611A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611A88: 386A99EC  addi r3, r10, -0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + -26132;
	// 82611A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611AAC: 4BE55375  bl 0x82466e20
	ctx.lr = 0x82611AB0;
	sub_82466E20(ctx, base);
	// 82611AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611AC0 size=108
    let mut pc: u32 = 0x82611AC0;
    'dispatch: loop {
        match pc {
            0x82611AC0 => {
    //   block [0x82611AC0..0x82611B2C)
	// 82611AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611ACC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611AD4: 38EB5668  addi r7, r11, 0x5668
	ctx.r[7].s64 = ctx.r[11].s64 + 22120;
	// 82611AD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82611ADC: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 82611AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611AE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611AF0: 386A9A1C  addi r3, r10, -0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26084;
	// 82611AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611B18: 4BE55309  bl 0x82466e20
	ctx.lr = 0x82611B1C;
	sub_82466E20(ctx, base);
	// 82611B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611B30 size=112
    let mut pc: u32 = 0x82611B30;
    'dispatch: loop {
        match pc {
            0x82611B30 => {
    //   block [0x82611B30..0x82611BA0)
	// 82611B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611B3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611B40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611B44: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611B4C: 390B5698  addi r8, r11, 0x5698
	ctx.r[8].s64 = ctx.r[11].s64 + 22168;
	// 82611B50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82611B54: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82611B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611B5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611B68: 386A9A4C  addi r3, r10, -0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26036;
	// 82611B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611B8C: 4BE55295  bl 0x82466e20
	ctx.lr = 0x82611B90;
	sub_82466E20(ctx, base);
	// 82611B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611BA0 size=112
    let mut pc: u32 = 0x82611BA0;
    'dispatch: loop {
        match pc {
            0x82611BA0 => {
    //   block [0x82611BA0..0x82611C10)
	// 82611BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611BAC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611BB0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82611BB4: 38EA56B0  addi r7, r10, 0x56b0
	ctx.r[7].s64 = ctx.r[10].s64 + 22192;
	// 82611BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611BBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611BC0: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82611BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611BC8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611BCC: 396BF0B4  addi r11, r11, -0xf4c
	ctx.r[11].s64 = ctx.r[11].s64 + -3916;
	// 82611BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611BD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611BD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611BDC: 386A9A7C  addi r3, r10, -0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + -25988;
	// 82611BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611BE4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82611BE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611BEC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611BF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611BF4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611BF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611BFC: 4BE55225  bl 0x82466e20
	ctx.lr = 0x82611C00;
	sub_82466E20(ctx, base);
	// 82611C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611C10 size=108
    let mut pc: u32 = 0x82611C10;
    'dispatch: loop {
        match pc {
            0x82611C10 => {
    //   block [0x82611C10..0x82611C7C)
	// 82611C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611C1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611C24: 38EB5788  addi r7, r11, 0x5788
	ctx.r[7].s64 = ctx.r[11].s64 + 22408;
	// 82611C28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82611C2C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 82611C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611C34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611C40: 386A9AAC  addi r3, r10, -0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + -25940;
	// 82611C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611C68: 4BE551B9  bl 0x82466e20
	ctx.lr = 0x82611C6C;
	sub_82466E20(ctx, base);
	// 82611C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611C80 size=108
    let mut pc: u32 = 0x82611C80;
    'dispatch: loop {
        match pc {
            0x82611C80 => {
    //   block [0x82611C80..0x82611CEC)
	// 82611C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611C8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611C94: 38EB57A0  addi r7, r11, 0x57a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22432;
	// 82611C98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82611C9C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82611CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611CA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611CB0: 386A9ADC  addi r3, r10, -0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + -25892;
	// 82611CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611CD8: 4BE55149  bl 0x82466e20
	ctx.lr = 0x82611CDC;
	sub_82466E20(ctx, base);
	// 82611CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611CF0 size=108
    let mut pc: u32 = 0x82611CF0;
    'dispatch: loop {
        match pc {
            0x82611CF0 => {
    //   block [0x82611CF0..0x82611D5C)
	// 82611CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611CFC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611D04: 38EB58A8  addi r7, r11, 0x58a8
	ctx.r[7].s64 = ctx.r[11].s64 + 22696;
	// 82611D08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82611D0C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 82611D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611D14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611D20: 386A9B0C  addi r3, r10, -0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25844;
	// 82611D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611D48: 4BE550D9  bl 0x82466e20
	ctx.lr = 0x82611D4C;
	sub_82466E20(ctx, base);
	// 82611D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611D60 size=112
    let mut pc: u32 = 0x82611D60;
    'dispatch: loop {
        match pc {
            0x82611D60 => {
    //   block [0x82611D60..0x82611DD0)
	// 82611D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611D6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611D70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611D74: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611D7C: 390B5908  addi r8, r11, 0x5908
	ctx.r[8].s64 = ctx.r[11].s64 + 22792;
	// 82611D80: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82611D84: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82611D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611D98: 386A9B3C  addi r3, r10, -0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25796;
	// 82611D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611DBC: 4BE55065  bl 0x82466e20
	ctx.lr = 0x82611DC0;
	sub_82466E20(ctx, base);
	// 82611DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611DD0 size=112
    let mut pc: u32 = 0x82611DD0;
    'dispatch: loop {
        match pc {
            0x82611DD0 => {
    //   block [0x82611DD0..0x82611E40)
	// 82611DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611DDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611DE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611DE4: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611DEC: 390B5A28  addi r8, r11, 0x5a28
	ctx.r[8].s64 = ctx.r[11].s64 + 23080;
	// 82611DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82611DF4: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82611DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611DFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611E08: 386A9B6C  addi r3, r10, -0x6494
	ctx.r[3].s64 = ctx.r[10].s64 + -25748;
	// 82611E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611E2C: 4BE54FF5  bl 0x82466e20
	ctx.lr = 0x82611E30;
	sub_82466E20(ctx, base);
	// 82611E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611E40 size=116
    let mut pc: u32 = 0x82611E40;
    'dispatch: loop {
        match pc {
            0x82611E40 => {
    //   block [0x82611E40..0x82611EB4)
	// 82611E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611E4C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611E50: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82611E54: 390A5A40  addi r8, r10, 0x5a40
	ctx.r[8].s64 = ctx.r[10].s64 + 23104;
	// 82611E58: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611E5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611E60: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611E64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611E68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82611E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611E74: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82611E78: 396BF0E4  addi r11, r11, -0xf1c
	ctx.r[11].s64 = ctx.r[11].s64 + -3868;
	// 82611E7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611E84: 386A9B9C  addi r3, r10, -0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + -25700;
	// 82611E88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82611E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611E90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611EA0: 4BE54F81  bl 0x82466e20
	ctx.lr = 0x82611EA4;
	sub_82466E20(ctx, base);
	// 82611EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611EB8 size=108
    let mut pc: u32 = 0x82611EB8;
    'dispatch: loop {
        match pc {
            0x82611EB8 => {
    //   block [0x82611EB8..0x82611F24)
	// 82611EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611EC4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611ECC: 38EB5AA0  addi r7, r11, 0x5aa0
	ctx.r[7].s64 = ctx.r[11].s64 + 23200;
	// 82611ED0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82611ED4: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82611ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611EDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611EE8: 386A9BCC  addi r3, r10, -0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + -25652;
	// 82611EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611F10: 4BE54F11  bl 0x82466e20
	ctx.lr = 0x82611F14;
	sub_82466E20(ctx, base);
	// 82611F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611F28 size=112
    let mut pc: u32 = 0x82611F28;
    'dispatch: loop {
        match pc {
            0x82611F28 => {
    //   block [0x82611F28..0x82611F98)
	// 82611F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611F34: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611F38: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82611F3C: 38EA5B48  addi r7, r10, 0x5b48
	ctx.r[7].s64 = ctx.r[10].s64 + 23368;
	// 82611F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611F44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611F48: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 82611F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611F50: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611F54: 396BF0F8  addi r11, r11, -0xf08
	ctx.r[11].s64 = ctx.r[11].s64 + -3848;
	// 82611F58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611F60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611F64: 386A9BFC  addi r3, r10, -0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + -25604;
	// 82611F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611F6C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82611F70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611F74: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611F78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611F7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611F80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611F84: 4BE54E9D  bl 0x82466e20
	ctx.lr = 0x82611F88;
	sub_82466E20(ctx, base);
	// 82611F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611F98 size=112
    let mut pc: u32 = 0x82611F98;
    'dispatch: loop {
        match pc {
            0x82611F98 => {
    //   block [0x82611F98..0x82612008)
	// 82611F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611FA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611FAC: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611FB4: 390B5BC0  addi r8, r11, 0x5bc0
	ctx.r[8].s64 = ctx.r[11].s64 + 23488;
	// 82611FB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82611FBC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82611FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611FD0: 386A9C2C  addi r3, r10, -0x63d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25556;
	// 82611FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611FF4: 4BE54E2D  bl 0x82466e20
	ctx.lr = 0x82611FF8;
	sub_82466E20(ctx, base);
	// 82611FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612008 size=112
    let mut pc: u32 = 0x82612008;
    'dispatch: loop {
        match pc {
            0x82612008 => {
    //   block [0x82612008..0x82612078)
	// 82612008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261200C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612014: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612018: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261201C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612024: 390B5C08  addi r8, r11, 0x5c08
	ctx.r[8].s64 = ctx.r[11].s64 + 23560;
	// 82612028: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8261202C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82612030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261203C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612040: 386A9C5C  addi r3, r10, -0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + -25508;
	// 82612044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261204C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261205C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612064: 4BE54DBD  bl 0x82466e20
	ctx.lr = 0x82612068;
	sub_82466E20(ctx, base);
	// 82612068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261206C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612078 size=112
    let mut pc: u32 = 0x82612078;
    'dispatch: loop {
        match pc {
            0x82612078 => {
    //   block [0x82612078..0x826120E8)
	// 82612078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261207C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612088: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261208C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612094: 390B5D10  addi r8, r11, 0x5d10
	ctx.r[8].s64 = ctx.r[11].s64 + 23824;
	// 82612098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261209C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826120A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826120A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826120A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826120AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826120B0: 386A9C8C  addi r3, r10, -0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + -25460;
	// 826120B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826120B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826120BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826120C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826120C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826120C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826120CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826120D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826120D4: 4BE54D4D  bl 0x82466e20
	ctx.lr = 0x826120D8;
	sub_82466E20(ctx, base);
	// 826120D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826120DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826120E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826120E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826120E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826120E8 size=112
    let mut pc: u32 = 0x826120E8;
    'dispatch: loop {
        match pc {
            0x826120E8 => {
    //   block [0x826120E8..0x82612158)
	// 826120E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826120EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826120F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826120F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826120F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826120FC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612104: 390B5D28  addi r8, r11, 0x5d28
	ctx.r[8].s64 = ctx.r[11].s64 + 23848;
	// 82612108: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261210C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82612110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261211C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612120: 386A9CBC  addi r3, r10, -0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + -25412;
	// 82612124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261212C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261213C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612144: 4BE54CDD  bl 0x82466e20
	ctx.lr = 0x82612148;
	sub_82466E20(ctx, base);
	// 82612148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261214C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612158 size=108
    let mut pc: u32 = 0x82612158;
    'dispatch: loop {
        match pc {
            0x82612158 => {
    //   block [0x82612158..0x826121C4)
	// 82612158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261215C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612164: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261216C: 38EB5D58  addi r7, r11, 0x5d58
	ctx.r[7].s64 = ctx.r[11].s64 + 23896;
	// 82612170: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82612174: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82612178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261217C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612188: 386A9CEC  addi r3, r10, -0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + -25364;
	// 8261218C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261219C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826121A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826121A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826121A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826121AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826121B0: 4BE54C71  bl 0x82466e20
	ctx.lr = 0x826121B4;
	sub_82466E20(ctx, base);
	// 826121B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826121B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826121BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826121C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826121C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826121C8 size=24
    let mut pc: u32 = 0x826121C8;
    'dispatch: loop {
        match pc {
            0x826121C8 => {
    //   block [0x826121C8..0x826121E0)
	// 826121C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826121CC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826121D0: 394AD1B0  addi r10, r10, -0x2e50
	ctx.r[10].s64 = ctx.r[10].s64 + -11856;
	// 826121D4: 816B555C  lwz r11, 0x555c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21852 as u32) ) } as u64;
	// 826121D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826121DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826121E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826121E0 size=116
    let mut pc: u32 = 0x826121E0;
    'dispatch: loop {
        match pc {
            0x826121E0 => {
    //   block [0x826121E0..0x82612254)
	// 826121E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826121E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826121E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826121EC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826121F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826121F4: 390BD1B0  addi r8, r11, -0x2e50
	ctx.r[8].s64 = ctx.r[11].s64 + -11856;
	// 826121F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826121FC: 392AF130  addi r9, r10, -0xed0
	ctx.r[9].s64 = ctx.r[10].s64 + -3792;
	// 82612200: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612204: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82612208: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 8261220C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261221C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612224: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82612228: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8261222C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82612230: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 82612234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612238: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261223C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612240: 4BE54BE1  bl 0x82466e20
	ctx.lr = 0x82612244;
	sub_82466E20(ctx, base);
	// 82612244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


