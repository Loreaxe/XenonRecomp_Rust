pub fn sub_82903F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82903F48 size=108
    let mut pc: u32 = 0x82903F48;
    'dispatch: loop {
        match pc {
            0x82903F48 => {
    //   block [0x82903F48..0x82903FB4)
	// 82903F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82903F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82903F50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82903F54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82903F58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82903F5C: 485589CD  bl 0x82e5c928
	ctx.lr = 0x82903F60;
	sub_82E5C928(ctx, base);
	// 82903F60: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82903F64: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82903F68: 396B4C74  addi r11, r11, 0x4c74
	ctx.r[11].s64 = ctx.r[11].s64 + 19572;
	// 82903F6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82903F70: 4BFC7BB1  bl 0x828cbb20
	ctx.lr = 0x82903F74;
	sub_828CBB20(ctx, base);
	// 82903F74: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82903F78: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82903F7C: 392B4CAC  addi r9, r11, 0x4cac
	ctx.r[9].s64 = ctx.r[11].s64 + 19628;
	// 82903F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82903F84: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82903F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82903F8C: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82903F90: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82903F94: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82903F98: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82903F9C: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82903FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82903FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82903FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82903FAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82903FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82903FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82903FB8 size=112
    let mut pc: u32 = 0x82903FB8;
    'dispatch: loop {
        match pc {
            0x82903FB8 => {
    //   block [0x82903FB8..0x82904028)
	// 82903FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82903FBC: 488A41B1  bl 0x831a816c
	ctx.lr = 0x82903FC0;
	sub_831A8130(ctx, base);
	// 82903FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82903FC4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82903FC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82903FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82903FD0: 388B4CE0  addi r4, r11, 0x4ce0
	ctx.r[4].s64 = ctx.r[11].s64 + 19680;
	// 82903FD4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82903FD8: 3860007C  li r3, 0x7c
	ctx.r[3].s64 = 124;
	// 82903FDC: 484EE40D  bl 0x82df23e8
	ctx.lr = 0x82903FE0;
	sub_82DF23E8(ctx, base);
	// 82903FE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82903FE4: 41820010  beq 0x82903ff4
	if ctx.cr[0].eq {
	pc = 0x82903FF4; continue 'dispatch;
	}
	// 82903FE8: 4BFFFF61  bl 0x82903f48
	ctx.lr = 0x82903FEC;
	sub_82903F48(ctx, base);
	// 82903FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82903FF0: 48000008  b 0x82903ff8
	pc = 0x82903FF8; continue 'dispatch;
	// 82903FF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82903FF8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82903FFC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82904000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82904004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904008: 4BFFF811  bl 0x82903818
	ctx.lr = 0x8290400C;
	sub_82903818(ctx, base);
	// 8290400C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82904010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82904014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904018: 4B9BBFE9  bl 0x822c0000
	ctx.lr = 0x8290401C;
	sub_822C0000(ctx, base);
	// 8290401C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82904020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904024: 488A4198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82904028 size=16
    let mut pc: u32 = 0x82904028;
    'dispatch: loop {
        match pc {
            0x82904028 => {
    //   block [0x82904028..0x82904038)
	// 82904028: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8290402C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82904030: 386BAFDC  addi r3, r11, -0x5024
	ctx.r[3].s64 = ctx.r[11].s64 + -20516;
	// 82904034: 48002F14  b 0x82906f48
	sub_82906F48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904038 size=92
    let mut pc: u32 = 0x82904038;
    'dispatch: loop {
        match pc {
            0x82904038 => {
    //   block [0x82904038..0x82904094)
	// 82904038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290403C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290404C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82904050: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82904054: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82904058: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290405C: 388B4D54  addi r4, r11, 0x4d54
	ctx.r[4].s64 = ctx.r[11].s64 + 19796;
	// 82904060: 484EF9A9  bl 0x82df3a08
	ctx.lr = 0x82904064;
	sub_82DF3A08(ctx, base);
	// 82904064: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82904068: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290406C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82904070: 48009DE1  bl 0x8290de50
	ctx.lr = 0x82904074;
	sub_8290DE50(ctx, base);
	// 82904074: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904078: 484EF3B1  bl 0x82df3428
	ctx.lr = 0x8290407C;
	sub_82DF3428(ctx, base);
	// 8290407C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904088: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290408C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82904098 size=12
    let mut pc: u32 = 0x82904098;
    'dispatch: loop {
        match pc {
            0x82904098 => {
    //   block [0x82904098..0x829040A4)
	// 82904098: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8290409C: 386BB094  addi r3, r11, -0x4f6c
	ctx.r[3].s64 = ctx.r[11].s64 + -20332;
	// 829040A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829040A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829040A8 size=16
    let mut pc: u32 = 0x829040A8;
    'dispatch: loop {
        match pc {
            0x829040A8 => {
    //   block [0x829040A8..0x829040B8)
	// 829040A8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829040AC: 396BAFDC  addi r11, r11, -0x5024
	ctx.r[11].s64 = ctx.r[11].s64 + -20516;
	// 829040B0: 386B00A8  addi r3, r11, 0xa8
	ctx.r[3].s64 = ctx.r[11].s64 + 168;
	// 829040B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829040B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829040B8 size=12
    let mut pc: u32 = 0x829040B8;
    'dispatch: loop {
        match pc {
            0x829040B8 => {
    //   block [0x829040B8..0x829040C4)
	// 829040B8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829040BC: 386BAFDC  addi r3, r11, -0x5024
	ctx.r[3].s64 = ctx.r[11].s64 + -20516;
	// 829040C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829040C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829040C8 size=16
    let mut pc: u32 = 0x829040C8;
    'dispatch: loop {
        match pc {
            0x829040C8 => {
    //   block [0x829040C8..0x829040D8)
	// 829040C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829040CC: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 829040D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829040D4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829040D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829040D8 size=160
    let mut pc: u32 = 0x829040D8;
    'dispatch: loop {
        match pc {
            0x829040D8 => {
    //   block [0x829040D8..0x82904178)
	// 829040D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829040DC: 488A4091  bl 0x831a816c
	ctx.lr = 0x829040E0;
	sub_831A8130(ctx, base);
	// 829040E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829040E4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829040E8: 3BC40018  addi r30, r4, 0x18
	ctx.r[30].s64 = ctx.r[4].s64 + 24;
	// 829040EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 829040F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829040F4: 83EB0BA0  lwz r31, 0xba0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2976 as u32) ) } as u64;
	// 829040F8: 484EF0B9  bl 0x82df31b0
	ctx.lr = 0x829040FC;
	sub_82DF31B0(ctx, base);
	// 829040FC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904100: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904104: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82904108: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8290410C: 41820014  beq 0x82904120
	if ctx.cr[0].eq {
	pc = 0x82904120; continue 'dispatch;
	}
	// 82904110: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82904114: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82904118: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290411C: 419AFFE0  beq cr6, 0x829040fc
	if ctx.cr[6].eq {
	pc = 0x829040FC; continue 'dispatch;
	}
	// 82904120: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82904124: 4182004C  beq 0x82904170
	if ctx.cr[0].eq {
	pc = 0x82904170; continue 'dispatch;
	}
	// 82904128: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290412C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904130: 83EB0B80  lwz r31, 0xb80(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2944 as u32) ) } as u64;
	// 82904134: 484EF07D  bl 0x82df31b0
	ctx.lr = 0x82904138;
	sub_82DF31B0(ctx, base);
	// 82904138: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290413C: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904140: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82904144: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82904148: 41820014  beq 0x8290415c
	if ctx.cr[0].eq {
	pc = 0x8290415C; continue 'dispatch;
	}
	// 8290414C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82904150: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82904154: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82904158: 419AFFE0  beq cr6, 0x82904138
	if ctx.cr[6].eq {
	pc = 0x82904138; continue 'dispatch;
	}
	// 8290415C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82904160: 41820010  beq 0x82904170
	if ctx.cr[0].eq {
	pc = 0x82904170; continue 'dispatch;
	}
	// 82904164: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82904168: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 8290416C: 4BEE8B7D  bl 0x827ecce8
	ctx.lr = 0x82904170;
	sub_827ECCE8(ctx, base);
	// 82904170: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904174: 488A4048  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904178 size=116
    let mut pc: u32 = 0x82904178;
    'dispatch: loop {
        match pc {
            0x82904178 => {
    //   block [0x82904178..0x829041EC)
	// 82904178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290417C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290418C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82904190: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82904194: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82904198: 409A000C  bne cr6, 0x829041a4
	if !ctx.cr[6].eq {
	pc = 0x829041A4; continue 'dispatch;
	}
	// 8290419C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 829041A0: 48000030  b 0x829041d0
	pc = 0x829041D0; continue 'dispatch;
	// 829041A4: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 829041A8: 419A0024  beq cr6, 0x829041cc
	if ctx.cr[6].eq {
	pc = 0x829041CC; continue 'dispatch;
	}
	// 829041AC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829041B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829041B4: 388BCE88  addi r4, r11, -0x3178
	ctx.r[4].s64 = ctx.r[11].s64 + -12664;
	// 829041B8: 488A3F41  bl 0x831a80f8
	ctx.lr = 0x829041BC;
	sub_831A80F8(ctx, base);
	// 829041BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829041C0: 4182000C  beq 0x829041cc
	if ctx.cr[0].eq {
	pc = 0x829041CC; continue 'dispatch;
	}
	// 829041C4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 829041C8: 4800000C  b 0x829041d4
	pc = 0x829041D4; continue 'dispatch;
	// 829041CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829041D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829041D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829041D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829041DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829041E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829041E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829041E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829041F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829041F0 size=136
    let mut pc: u32 = 0x829041F0;
    'dispatch: loop {
        match pc {
            0x829041F0 => {
    //   block [0x829041F0..0x82904278)
	// 829041F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829041F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829041F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829041FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904204: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82904208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290420C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82904210: 409A0020  bne cr6, 0x82904230
	if !ctx.cr[6].eq {
	pc = 0x82904230; continue 'dispatch;
	}
	// 82904214: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82904218: 419A0048  beq cr6, 0x82904260
	if ctx.cr[6].eq {
	pc = 0x82904260; continue 'dispatch;
	}
	// 8290421C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82904220: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82904224: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82904228: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8290422C: 48000034  b 0x82904260
	pc = 0x82904260; continue 'dispatch;
	// 82904230: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82904234: 419A002C  beq cr6, 0x82904260
	if ctx.cr[6].eq {
	pc = 0x82904260; continue 'dispatch;
	}
	// 82904238: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290423C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904240: 388BCED0  addi r4, r11, -0x3130
	ctx.r[4].s64 = ctx.r[11].s64 + -12592;
	// 82904244: 488A3EB5  bl 0x831a80f8
	ctx.lr = 0x82904248;
	sub_831A80F8(ctx, base);
	// 82904248: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290424C: 4182000C  beq 0x82904258
	if ctx.cr[0].eq {
	pc = 0x82904258; continue 'dispatch;
	}
	// 82904250: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82904254: 4800000C  b 0x82904260
	pc = 0x82904260; continue 'dispatch;
	// 82904258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290425C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904260: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290426C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82904270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904278 size=136
    let mut pc: u32 = 0x82904278;
    'dispatch: loop {
        match pc {
            0x82904278 => {
    //   block [0x82904278..0x82904300)
	// 82904278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290427C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290428C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82904290: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82904294: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82904298: 409A0020  bne cr6, 0x829042b8
	if !ctx.cr[6].eq {
	pc = 0x829042B8; continue 'dispatch;
	}
	// 8290429C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 829042A0: 419A0048  beq cr6, 0x829042e8
	if ctx.cr[6].eq {
	pc = 0x829042E8; continue 'dispatch;
	}
	// 829042A4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 829042A8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 829042AC: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 829042B0: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 829042B4: 48000034  b 0x829042e8
	pc = 0x829042E8; continue 'dispatch;
	// 829042B8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 829042BC: 419A002C  beq cr6, 0x829042e8
	if ctx.cr[6].eq {
	pc = 0x829042E8; continue 'dispatch;
	}
	// 829042C0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829042C4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829042C8: 388BCF90  addi r4, r11, -0x3070
	ctx.r[4].s64 = ctx.r[11].s64 + -12400;
	// 829042CC: 488A3E2D  bl 0x831a80f8
	ctx.lr = 0x829042D0;
	sub_831A80F8(ctx, base);
	// 829042D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829042D4: 4182000C  beq 0x829042e0
	if ctx.cr[0].eq {
	pc = 0x829042E0; continue 'dispatch;
	}
	// 829042D8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 829042DC: 4800000C  b 0x829042e8
	pc = 0x829042E8; continue 'dispatch;
	// 829042E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829042E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829042E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829042EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829042F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829042F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829042F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829042FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904300 size=168
    let mut pc: u32 = 0x82904300;
    'dispatch: loop {
        match pc {
            0x82904300 => {
    //   block [0x82904300..0x829043A8)
	// 82904300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290430C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82904318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290431C: 4BEE7E45  bl 0x827ec160
	ctx.lr = 0x82904320;
	sub_827EC160(ctx, base);
	// 82904320: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904324: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82904328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290432C: 4E800421  bctrl
	ctx.lr = 0x82904330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904330: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82904334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904338: 481BDC59  bl 0x82ac1f90
	ctx.lr = 0x8290433C;
	sub_82AC1F90(ctx, base);
	// 8290433C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904340: 4BEE7E21  bl 0x827ec160
	ctx.lr = 0x82904344;
	sub_827EC160(ctx, base);
	// 82904344: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904348: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8290434C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82904350: 4E800421  bctrl
	ctx.lr = 0x82904354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904354: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82904358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290435C: 481A717D  bl 0x82aab4d8
	ctx.lr = 0x82904360;
	sub_82AAB4D8(ctx, base);
	// 82904360: 807F0398  lwz r3, 0x398(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 82904364: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82904368: 419A0008  beq cr6, 0x82904370
	if ctx.cr[6].eq {
	pc = 0x82904370; continue 'dispatch;
	}
	// 8290436C: 4807878D  bl 0x8297caf8
	ctx.lr = 0x82904370;
	sub_8297CAF8(ctx, base);
	// 82904370: 807F0390  lwz r3, 0x390(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(912 as u32) ) } as u64;
	// 82904374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82904378: 419A000C  beq cr6, 0x82904384
	if ctx.cr[6].eq {
	pc = 0x82904384; continue 'dispatch;
	}
	// 8290437C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82904380: 4801E5F9  bl 0x82922978
	ctx.lr = 0x82904384;
	sub_82922978(ctx, base);
	// 82904384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82904388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290438C: 4805AD95  bl 0x8295f120
	ctx.lr = 0x82904390;
	sub_8295F120(ctx, base);
	// 82904390: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290439C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829043A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829043A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829043A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829043A8 size=72
    let mut pc: u32 = 0x829043A8;
    'dispatch: loop {
        match pc {
            0x829043A8 => {
    //   block [0x829043A8..0x829043F0)
	// 829043A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829043AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829043B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829043B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829043B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829043BC: 389F03AC  addi r4, r31, 0x3ac
	ctx.r[4].s64 = ctx.r[31].s64 + 940;
	// 829043C0: 817F03AC  lwz r11, 0x3ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(940 as u32) ) } as u64;
	// 829043C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829043C8: 419A000C  beq cr6, 0x829043d4
	if ctx.cr[6].eq {
	pc = 0x829043D4; continue 'dispatch;
	}
	// 829043CC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 829043D0: 4BC0C2D9  bl 0x825106a8
	ctx.lr = 0x829043D4;
	sub_825106A8(ctx, base);
	// 829043D4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 829043D8: 4BC0E7A1  bl 0x82512b78
	ctx.lr = 0x829043DC;
	sub_82512B78(ctx, base);
	// 829043DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829043E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829043E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829043E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829043EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829043F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829043F0 size=196
    let mut pc: u32 = 0x829043F0;
    'dispatch: loop {
        match pc {
            0x829043F0 => {
    //   block [0x829043F0..0x829044B4)
	// 829043F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829043F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829043F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829043FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82904408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290440C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82904410: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82904414: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904418: 4B9BC521  bl 0x822c0938
	ctx.lr = 0x8290441C;
	sub_822C0938(ctx, base);
	// 8290441C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82904420: 41820028  beq 0x82904448
	if ctx.cr[0].eq {
	pc = 0x82904448; continue 'dispatch;
	}
	// 82904424: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82904428: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290442C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82904430: 392B4D90  addi r9, r11, 0x4d90
	ctx.r[9].s64 = ctx.r[11].s64 + 19856;
	// 82904434: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82904438: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290443C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82904440: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82904444: 48000008  b 0x8290444c
	pc = 0x8290444C; continue 'dispatch;
	// 82904448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290444C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904450: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82904454: 409A0044  bne cr6, 0x82904498
	if !ctx.cr[6].eq {
	pc = 0x82904498; continue 'dispatch;
	}
	// 82904458: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290445C: 419A001C  beq cr6, 0x82904478
	if ctx.cr[6].eq {
	pc = 0x82904478; continue 'dispatch;
	}
	// 82904460: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904464: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82904468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290446C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82904474: 4E800421  bctrl
	ctx.lr = 0x82904478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904478: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290447C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82904480: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904484: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82904488: 816BCDEC  lwz r11, -0x3214(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12820 as u32) ) } as u64;
	// 8290448C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82904490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82904494: 4B9BBB6D  bl 0x822c0000
	ctx.lr = 0x82904498;
	sub_822C0000(ctx, base);
	// 82904498: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290449C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829044A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829044A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829044A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829044AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829044B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829044B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829044B8 size=196
    let mut pc: u32 = 0x829044B8;
    'dispatch: loop {
        match pc {
            0x829044B8 => {
    //   block [0x829044B8..0x8290457C)
	// 829044B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829044BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829044C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829044C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829044C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829044CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829044D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829044D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 829044D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829044DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829044E0: 4B9BC459  bl 0x822c0938
	ctx.lr = 0x829044E4;
	sub_822C0938(ctx, base);
	// 829044E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829044E8: 41820028  beq 0x82904510
	if ctx.cr[0].eq {
	pc = 0x82904510; continue 'dispatch;
	}
	// 829044EC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829044F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 829044F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 829044F8: 392B4DA4  addi r9, r11, 0x4da4
	ctx.r[9].s64 = ctx.r[11].s64 + 19876;
	// 829044FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82904500: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82904504: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82904508: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290450C: 48000008  b 0x82904514
	pc = 0x82904514; continue 'dispatch;
	// 82904510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82904514: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904518: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290451C: 409A0044  bne cr6, 0x82904560
	if !ctx.cr[6].eq {
	pc = 0x82904560; continue 'dispatch;
	}
	// 82904520: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82904524: 419A001C  beq cr6, 0x82904540
	if ctx.cr[6].eq {
	pc = 0x82904540; continue 'dispatch;
	}
	// 82904528: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290452C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82904530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904534: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904538: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290453C: 4E800421  bctrl
	ctx.lr = 0x82904540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904540: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82904544: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82904548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290454C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82904550: 816BCDEC  lwz r11, -0x3214(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12820 as u32) ) } as u64;
	// 82904554: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82904558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290455C: 4B9BBAA5  bl 0x822c0000
	ctx.lr = 0x82904560;
	sub_822C0000(ctx, base);
	// 82904560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290456C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82904574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904580 size=72
    let mut pc: u32 = 0x82904580;
    'dispatch: loop {
        match pc {
            0x82904580 => {
    //   block [0x82904580..0x829045C8)
	// 82904580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290458C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82904590: 419A001C  beq cr6, 0x829045ac
	if ctx.cr[6].eq {
	pc = 0x829045AC; continue 'dispatch;
	}
	// 82904594: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82904598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8290459C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 829045A0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829045A4: 4BFFFBD5  bl 0x82904178
	ctx.lr = 0x829045A8;
	sub_82904178(ctx, base);
	// 829045A8: 48000010  b 0x829045b8
	pc = 0x829045B8; continue 'dispatch;
	// 829045AC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829045B0: 396BCE88  addi r11, r11, -0x3178
	ctx.r[11].s64 = ctx.r[11].s64 + -12664;
	// 829045B4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829045B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829045BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829045C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829045C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829045C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829045C8 size=212
    let mut pc: u32 = 0x829045C8;
    'dispatch: loop {
        match pc {
            0x829045C8 => {
    //   block [0x829045C8..0x8290469C)
	// 829045C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829045CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829045D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829045D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829045D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829045DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829045E0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829045E4: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 829045E8: 396B4F94  addi r11, r11, 0x4f94
	ctx.r[11].s64 = ctx.r[11].s64 + 20372;
	// 829045EC: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 829045F0: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 829045F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829045F8: 394A4DEC  addi r10, r10, 0x4dec
	ctx.r[10].s64 = ctx.r[10].s64 + 19948;
	// 829045FC: 39294DD4  addi r9, r9, 0x4dd4
	ctx.r[9].s64 = ctx.r[9].s64 + 19924;
	// 82904600: 39684DB8  addi r11, r8, 0x4db8
	ctx.r[11].s64 = ctx.r[8].s64 + 19896;
	// 82904604: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82904608: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 8290460C: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82904610: 917F0218  stw r11, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[11].u32 ) };
	// 82904614: 807F03B0  lwz r3, 0x3b0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(944 as u32) ) } as u64;
	// 82904618: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290461C: 419A0008  beq cr6, 0x82904624
	if ctx.cr[6].eq {
	pc = 0x82904624; continue 'dispatch;
	}
	// 82904620: 4B9BC271  bl 0x822c0890
	ctx.lr = 0x82904624;
	sub_822C0890(ctx, base);
	// 82904624: 807F03A8  lwz r3, 0x3a8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(936 as u32) ) } as u64;
	// 82904628: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290462C: 419A0018  beq cr6, 0x82904644
	if ctx.cr[6].eq {
	pc = 0x82904644; continue 'dispatch;
	}
	// 82904630: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904634: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82904638: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290463C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82904640: 4E800421  bctrl
	ctx.lr = 0x82904644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904644: 807F03A4  lwz r3, 0x3a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(932 as u32) ) } as u64;
	// 82904648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290464C: 419A0008  beq cr6, 0x82904654
	if ctx.cr[6].eq {
	pc = 0x82904654; continue 'dispatch;
	}
	// 82904650: 4B9BC241  bl 0x822c0890
	ctx.lr = 0x82904654;
	sub_822C0890(ctx, base);
	// 82904654: 807F039C  lwz r3, 0x39c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(924 as u32) ) } as u64;
	// 82904658: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290465C: 419A0008  beq cr6, 0x82904664
	if ctx.cr[6].eq {
	pc = 0x82904664; continue 'dispatch;
	}
	// 82904660: 4B9BC231  bl 0x822c0890
	ctx.lr = 0x82904664;
	sub_822C0890(ctx, base);
	// 82904664: 807F0394  lwz r3, 0x394(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 82904668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290466C: 419A0008  beq cr6, 0x82904674
	if ctx.cr[6].eq {
	pc = 0x82904674; continue 'dispatch;
	}
	// 82904670: 4B9BC221  bl 0x822c0890
	ctx.lr = 0x82904674;
	sub_822C0890(ctx, base);
	// 82904674: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904678: 4805AC21  bl 0x8295f298
	ctx.lr = 0x8290467C;
	sub_8295F298(ctx, base);
	// 8290467C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904680: 4BEA2FD9  bl 0x827a7658
	ctx.lr = 0x82904684;
	sub_827A7658(ctx, base);
	// 82904684: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290468C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904690: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82904694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829046A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829046A0 size=8
    let mut pc: u32 = 0x829046A0;
    'dispatch: loop {
        match pc {
            0x829046A0 => {
    //   block [0x829046A0..0x829046A8)
	// 829046A0: 3863FFC8  addi r3, r3, -0x38
	ctx.r[3].s64 = ctx.r[3].s64 + -56;
	// 829046A4: 480003CC  b 0x82904a70
	sub_82904A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829046A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829046A8 size=8
    let mut pc: u32 = 0x829046A8;
    'dispatch: loop {
        match pc {
            0x829046A8 => {
    //   block [0x829046A8..0x829046B0)
	// 829046A8: 3863FDE8  addi r3, r3, -0x218
	ctx.r[3].s64 = ctx.r[3].s64 + -536;
	// 829046AC: 480003C4  b 0x82904a70
	sub_82904A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829046B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829046B0 size=8
    let mut pc: u32 = 0x829046B0;
    'dispatch: loop {
        match pc {
            0x829046B0 => {
    //   block [0x829046B0..0x829046B8)
	// 829046B0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 829046B4: 480003BC  b 0x82904a70
	sub_82904A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829046B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829046B8 size=244
    let mut pc: u32 = 0x829046B8;
    'dispatch: loop {
        match pc {
            0x829046B8 => {
    //   block [0x829046B8..0x829047AC)
	// 829046B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829046BC: 488A3AAD  bl 0x831a8168
	ctx.lr = 0x829046C0;
	sub_831A8130(ctx, base);
	// 829046C0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829046C4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829046C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829046CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829046D0: 808BD080  lwz r4, -0x2f80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12160 as u32) ) } as u64;
	// 829046D4: 484EF335  bl 0x82df3a08
	ctx.lr = 0x829046D8;
	sub_82DF3A08(ctx, base);
	// 829046D8: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 829046DC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 829046E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829046E4: 4BEE7AED  bl 0x827ec1d0
	ctx.lr = 0x829046E8;
	sub_827EC1D0(ctx, base);
	// 829046E8: 4BEE5BC1  bl 0x827ea2a8
	ctx.lr = 0x829046EC;
	sub_827EA2A8(ctx, base);
	// 829046EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 829046F0: 484EEC19  bl 0x82df3308
	ctx.lr = 0x829046F4;
	sub_82DF3308(ctx, base);
	// 829046F4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 829046F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829046FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82904700: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82904704: 697D0001  xori r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u64 ^ 1;
	// 82904708: 484EED21  bl 0x82df3428
	ctx.lr = 0x8290470C;
	sub_82DF3428(ctx, base);
	// 8290470C: 817F03A8  lwz r11, 0x3a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(936 as u32) ) } as u64;
	// 82904710: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82904714: 419A0090  beq cr6, 0x829047a4
	if ctx.cr[6].eq {
	pc = 0x829047A4; continue 'dispatch;
	}
	// 82904718: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8290471C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82904720: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 82904724: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82904728: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 8290472C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82904730: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82904734: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82904738: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290473C: 3B8100A0  addi r28, r1, 0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + 160;
	// 82904740: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82904744: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82904748: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8290474C: 13885C07  vcmpneb. (lvlx128) v28, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82904750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829047B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829047B0 size=148
    let mut pc: u32 = 0x829047B0;
    'dispatch: loop {
        match pc {
            0x829047B0 => {
    //   block [0x829047B0..0x82904844)
	// 829047B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829047B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829047B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829047BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829047C0: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 829047C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829047C8: 4805AA11  bl 0x8295f1d8
	ctx.lr = 0x829047CC;
	sub_8295F1D8(ctx, base);
	// 829047CC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829047D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829047D4: 808BD08C  lwz r4, -0x2f74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12148 as u32) ) } as u64;
	// 829047D8: 484EF231  bl 0x82df3a08
	ctx.lr = 0x829047DC;
	sub_82DF3A08(ctx, base);
	// 829047DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829047E0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 829047E4: 4BEF168D  bl 0x827f5e70
	ctx.lr = 0x829047E8;
	sub_827F5E70(ctx, base);
	// 829047E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829047EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829047F0: 4805ABE9  bl 0x8295f3d8
	ctx.lr = 0x829047F4;
	sub_8295F3D8(ctx, base);
	// 829047F4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 829047F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829047FC: 419A0008  beq cr6, 0x82904804
	if ctx.cr[6].eq {
	pc = 0x82904804; continue 'dispatch;
	}
	// 82904800: 4B9BC091  bl 0x822c0890
	ctx.lr = 0x82904804;
	sub_822C0890(ctx, base);
	// 82904804: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904808: 484EEC21  bl 0x82df3428
	ctx.lr = 0x8290480C;
	sub_82DF3428(ctx, base);
	// 8290480C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82904810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904814: 808B0B2C  lwz r4, 0xb2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2860 as u32) ) } as u64;
	// 82904818: 484EF1F1  bl 0x82df3a08
	ctx.lr = 0x8290481C;
	sub_82DF3A08(ctx, base);
	// 8290481C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82904820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904824: 4805ABBD  bl 0x8295f3e0
	ctx.lr = 0x82904828;
	sub_8295F3E0(ctx, base);
	// 82904828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290482C: 484EEBFD  bl 0x82df3428
	ctx.lr = 0x82904830;
	sub_82DF3428(ctx, base);
	// 82904830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290483C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82904848 size=156
    let mut pc: u32 = 0x82904848;
    'dispatch: loop {
        match pc {
            0x82904848 => {
    //   block [0x82904848..0x829048E4)
	// 82904848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290484C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290485C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82904860: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904864: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82904868: 481A7481  bl 0x82aabce8
	ctx.lr = 0x8290486C;
	sub_82AABCE8(ctx, base);
	// 8290486C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904870: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904874: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82904878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290487C: 4E800421  bctrl
	ctx.lr = 0x82904880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904880: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82904884: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82904888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290488C: 419A000C  beq cr6, 0x82904898
	if ctx.cr[6].eq {
	pc = 0x82904898; continue 'dispatch;
	}
	// 82904890: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82904894: 4B9BBFFD  bl 0x822c0890
	ctx.lr = 0x82904898;
	sub_822C0890(ctx, base);
	// 82904898: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8290489C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829048A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829048A4: C02B7BC4  lfs f1, 0x7bc4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31684 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829048A8: 4805A771  bl 0x8295f018
	ctx.lr = 0x829048AC;
	sub_8295F018(ctx, base);
	// 829048AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829048B0: 4800BA99  bl 0x82910348
	ctx.lr = 0x829048B4;
	sub_82910348(ctx, base);
	// 829048B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829048B8: 4800BAD9  bl 0x82910390
	ctx.lr = 0x829048BC;
	sub_82910390(ctx, base);
	// 829048BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829048C0: 48004469  bl 0x82908d28
	ctx.lr = 0x829048C4;
	sub_82908D28(ctx, base);
	// 829048C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829048C8: 4BC0BD09  bl 0x825105d0
	ctx.lr = 0x829048CC;
	sub_825105D0(ctx, base);
	// 829048CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829048D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829048D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829048D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829048DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829048E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829048E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829048E8 size=84
    let mut pc: u32 = 0x829048E8;
    'dispatch: loop {
        match pc {
            0x829048E8 => {
    //   block [0x829048E8..0x8290493C)
	// 829048E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829048EC: 488A3881  bl 0x831a816c
	ctx.lr = 0x829048F0;
	sub_831A8130(ctx, base);
	// 829048F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829048F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829048F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 829048FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82904900: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82904904: 409A0008  bne cr6, 0x8290490c
	if !ctx.cr[6].eq {
	pc = 0x8290490C; continue 'dispatch;
	}
	// 82904908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290490C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904910: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82904914: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82904918: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290491C: 4BFFFAD5  bl 0x829043f0
	ctx.lr = 0x82904920;
	sub_829043F0(ctx, base);
	// 82904920: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82904924: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82904928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290492C: 4B9BB6D5  bl 0x822c0000
	ctx.lr = 0x82904930;
	sub_822C0000(ctx, base);
	// 82904930: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82904934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904938: 488A3884  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904940 size=72
    let mut pc: u32 = 0x82904940;
    'dispatch: loop {
        match pc {
            0x82904940 => {
    //   block [0x82904940..0x82904988)
	// 82904940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290494C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82904950: 419A001C  beq cr6, 0x8290496c
	if ctx.cr[6].eq {
	pc = 0x8290496C; continue 'dispatch;
	}
	// 82904954: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82904958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8290495C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82904960: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82904964: 4BFFF88D  bl 0x829041f0
	ctx.lr = 0x82904968;
	sub_829041F0(ctx, base);
	// 82904968: 48000010  b 0x82904978
	pc = 0x82904978; continue 'dispatch;
	// 8290496C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82904970: 396BCED0  addi r11, r11, -0x3130
	ctx.r[11].s64 = ctx.r[11].s64 + -12592;
	// 82904974: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904988 size=72
    let mut pc: u32 = 0x82904988;
    'dispatch: loop {
        match pc {
            0x82904988 => {
    //   block [0x82904988..0x829049D0)
	// 82904988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904994: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82904998: 419A001C  beq cr6, 0x829049b4
	if ctx.cr[6].eq {
	pc = 0x829049B4; continue 'dispatch;
	}
	// 8290499C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 829049A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 829049A4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 829049A8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829049AC: 4BFFF8CD  bl 0x82904278
	ctx.lr = 0x829049B0;
	sub_82904278(ctx, base);
	// 829049B0: 48000010  b 0x829049c0
	pc = 0x829049C0; continue 'dispatch;
	// 829049B4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829049B8: 396BCF90  addi r11, r11, -0x3070
	ctx.r[11].s64 = ctx.r[11].s64 + -12400;
	// 829049BC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829049C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829049C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829049C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829049CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829049D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829049D0 size=156
    let mut pc: u32 = 0x829049D0;
    'dispatch: loop {
        match pc {
            0x829049D0 => {
    //   block [0x829049D0..0x82904A6C)
	// 829049D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829049D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829049D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829049DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829049E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829049E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 829049E8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 829049EC: 396BA214  addi r11, r11, -0x5dec
	ctx.r[11].s64 = ctx.r[11].s64 + -24044;
	// 829049F0: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 829049F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829049F8: 4805A921  bl 0x8295f318
	ctx.lr = 0x829049FC;
	sub_8295F318(ctx, base);
	// 829049FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82904A00: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82904A04: 396B4F94  addi r11, r11, 0x4f94
	ctx.r[11].s64 = ctx.r[11].s64 + 20372;
	// 82904A08: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82904A0C: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 82904A10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82904A14: 394A4DEC  addi r10, r10, 0x4dec
	ctx.r[10].s64 = ctx.r[10].s64 + 19948;
	// 82904A18: 39294DD4  addi r9, r9, 0x4dd4
	ctx.r[9].s64 = ctx.r[9].s64 + 19924;
	// 82904A1C: 39084DB8  addi r8, r8, 0x4db8
	ctx.r[8].s64 = ctx.r[8].s64 + 19896;
	// 82904A20: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82904A24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82904A28: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82904A2C: 911F0218  stw r8, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[8].u32 ) };
	// 82904A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904A34: 917F0390  stw r11, 0x390(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(912 as u32), ctx.r[11].u32 ) };
	// 82904A38: 917F0394  stw r11, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[11].u32 ) };
	// 82904A3C: 917F0398  stw r11, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[11].u32 ) };
	// 82904A40: 917F039C  stw r11, 0x39c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(924 as u32), ctx.r[11].u32 ) };
	// 82904A44: 917F03A0  stw r11, 0x3a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(928 as u32), ctx.r[11].u32 ) };
	// 82904A48: 917F03A4  stw r11, 0x3a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(932 as u32), ctx.r[11].u32 ) };
	// 82904A4C: 917F03A8  stw r11, 0x3a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(936 as u32), ctx.r[11].u32 ) };
	// 82904A50: 917F03AC  stw r11, 0x3ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(940 as u32), ctx.r[11].u32 ) };
	// 82904A54: 917F03B0  stw r11, 0x3b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(944 as u32), ctx.r[11].u32 ) };
	// 82904A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82904A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904A64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904A70 size=76
    let mut pc: u32 = 0x82904A70;
    'dispatch: loop {
        match pc {
            0x82904A70 => {
    //   block [0x82904A70..0x82904ABC)
	// 82904A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904A78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904A7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904A80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82904A88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82904A8C: 4BFFFB3D  bl 0x829045c8
	ctx.lr = 0x82904A90;
	sub_829045C8(ctx, base);
	// 82904A90: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82904A94: 4182000C  beq 0x82904aa0
	if ctx.cr[0].eq {
	pc = 0x82904AA0; continue 'dispatch;
	}
	// 82904A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904A9C: 484ED93D  bl 0x82df23d8
	ctx.lr = 0x82904AA0;
	sub_82DF23D8(ctx, base);
	// 82904AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904AA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904AB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82904AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904AC0 size=148
    let mut pc: u32 = 0x82904AC0;
    'dispatch: loop {
        match pc {
            0x82904AC0 => {
    //   block [0x82904AC0..0x82904B54)
	// 82904AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904AC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904ACC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904AD0: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82904AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904AD8: 4805A701  bl 0x8295f1d8
	ctx.lr = 0x82904ADC;
	sub_8295F1D8(ctx, base);
	// 82904ADC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82904AE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904AE4: 808BD080  lwz r4, -0x2f80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12160 as u32) ) } as u64;
	// 82904AE8: 484EEF21  bl 0x82df3a08
	ctx.lr = 0x82904AEC;
	sub_82DF3A08(ctx, base);
	// 82904AEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82904AF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82904AF4: 4BEF137D  bl 0x827f5e70
	ctx.lr = 0x82904AF8;
	sub_827F5E70(ctx, base);
	// 82904AF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82904AFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904B00: 4805A8D9  bl 0x8295f3d8
	ctx.lr = 0x82904B04;
	sub_8295F3D8(ctx, base);
	// 82904B04: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82904B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82904B0C: 419A0008  beq cr6, 0x82904b14
	if ctx.cr[6].eq {
	pc = 0x82904B14; continue 'dispatch;
	}
	// 82904B10: 4B9BBD81  bl 0x822c0890
	ctx.lr = 0x82904B14;
	sub_822C0890(ctx, base);
	// 82904B14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904B18: 484EE911  bl 0x82df3428
	ctx.lr = 0x82904B1C;
	sub_82DF3428(ctx, base);
	// 82904B1C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82904B20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904B24: 808BD0B0  lwz r4, -0x2f50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12112 as u32) ) } as u64;
	// 82904B28: 484EEEE1  bl 0x82df3a08
	ctx.lr = 0x82904B2C;
	sub_82DF3A08(ctx, base);
	// 82904B2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82904B30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904B34: 4805A8AD  bl 0x8295f3e0
	ctx.lr = 0x82904B38;
	sub_8295F3E0(ctx, base);
	// 82904B38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904B3C: 484EE8ED  bl 0x82df3428
	ctx.lr = 0x82904B40;
	sub_82DF3428(ctx, base);
	// 82904B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82904B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82904B58 size=164
    let mut pc: u32 = 0x82904B58;
    'dispatch: loop {
        match pc {
            0x82904B58 => {
    //   block [0x82904B58..0x82904BFC)
	// 82904B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904B60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904B64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904B6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82904B70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82904B74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82904B78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904B7C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82904B80: 484EEE89  bl 0x82df3a08
	ctx.lr = 0x82904B84;
	sub_82DF3A08(ctx, base);
	// 82904B84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904B88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904B8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82904B90: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82904B94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82904B98: 4E800421  bctrl
	ctx.lr = 0x82904B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904B9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82904BA0: 484EE889  bl 0x82df3428
	ctx.lr = 0x82904BA4;
	sub_82DF3428(ctx, base);
	// 82904BA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82904BA8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82904BAC: 480121ED  bl 0x82916d98
	ctx.lr = 0x82904BB0;
	sub_82916D98(ctx, base);
	// 82904BB0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82904BB4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82904BB8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82904BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904BC0: 80CB00F8  lwz r6, 0xf8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 82904BC4: 808A85F8  lwz r4, -0x7a08(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31240 as u32) ) } as u64;
	// 82904BC8: 80AB00F4  lwz r5, 0xf4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 82904BCC: 4BF10A15  bl 0x828155e0
	ctx.lr = 0x82904BD0;
	sub_828155E0(ctx, base);
	// 82904BD0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82904BD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82904BD8: 419A0008  beq cr6, 0x82904be0
	if ctx.cr[6].eq {
	pc = 0x82904BE0; continue 'dispatch;
	}
	// 82904BDC: 4B9BBCB5  bl 0x822c0890
	ctx.lr = 0x82904BE0;
	sub_822C0890(ctx, base);
	// 82904BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82904BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82904BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82904BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82904BF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82904BF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82904BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82904C00 size=656
    let mut pc: u32 = 0x82904C00;
    'dispatch: loop {
        match pc {
            0x82904C00 => {
    //   block [0x82904C00..0x82904E90)
	// 82904C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904C04: 488A3561  bl 0x831a8164
	ctx.lr = 0x82904C08;
	sub_831A8130(ctx, base);
	// 82904C08: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904C0C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82904C10: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82904C14: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82904C18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82904C1C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82904C20: C0CB89AC  lfs f6, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82904C24: C08A08A4  lfs f4, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82904C28: FCA03090  fmr f5, f6
	ctx.f[5].f64 = ctx.f[6].f64;
	// 82904C2C: C0292784  lfs f1, 0x2784(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(10116 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82904C30: FC602090  fmr f3, f4
	ctx.f[3].f64 = ctx.f[4].f64;
	// 82904C34: FC403090  fmr f2, f6
	ctx.f[2].f64 = ctx.f[6].f64;
	// 82904C38: 4BEF1329  bl 0x827f5f60
	ctx.lr = 0x82904C3C;
	sub_827F5F60(ctx, base);
	// 82904C3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82904C40: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82904C44: 388B5D2C  addi r4, r11, 0x5d2c
	ctx.r[4].s64 = ctx.r[11].s64 + 23852;
	// 82904C48: 484EEC31  bl 0x82df3878
	ctx.lr = 0x82904C4C;
	sub_82DF3878(ctx, base);
	// 82904C4C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82904C50: 386100CC  addi r3, r1, 0xcc
	ctx.r[3].s64 = ctx.r[1].s64 + 204;
	// 82904C54: 388B4590  addi r4, r11, 0x4590
	ctx.r[4].s64 = ctx.r[11].s64 + 17808;
	// 82904C58: 484EEC21  bl 0x82df3878
	ctx.lr = 0x82904C5C;
	sub_82DF3878(ctx, base);
	// 82904C5C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82904C60: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82904C64: 388B4578  addi r4, r11, 0x4578
	ctx.r[4].s64 = ctx.r[11].s64 + 17784;
	// 82904C68: 484EEC11  bl 0x82df3878
	ctx.lr = 0x82904C6C;
	sub_82DF3878(ctx, base);
	// 82904C6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82904C70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82904C74: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82904C78: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82904C7C: 38894560  addi r4, r9, 0x4560
	ctx.r[4].s64 = ctx.r[9].s64 + 17760;
	// 82904C80: C00BC65C  lfs f0, -0x39a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14756 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82904C84: C1AA9450  lfs f13, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82904C88: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82904C8C: D1A10084  stfs f13, 0x84(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82904C90: 484EEBE9  bl 0x82df3878
	ctx.lr = 0x82904C94;
	sub_82DF3878(ctx, base);
	// 82904C94: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82904C98: 807F0390  lwz r3, 0x390(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(912 as u32) ) } as u64;
	// 82904C9C: 816B67DC  lwz r11, 0x67dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26588 as u32) ) } as u64;
	// 82904CA0: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82904CA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904CA8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82904CAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82904CB0: 4E800421  bctrl
	ctx.lr = 0x82904CB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904CB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82904CB8: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82904CBC: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82904CC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904CC4: C00B0030  lfs f0, 0x30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82904CC8: C1AB0034  lfs f13, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82904CCC: C18B0038  lfs f12, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82904CD0: C16B003C  lfs f11, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82904CD4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82904CD8: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82904CDC: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82904CE0: D161006C  stfs f11, 0x6c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82904CE4: 4810E9A5  bl 0x82a13688
	ctx.lr = 0x82904CE8;
	sub_82A13688(ctx, base);
	// 82904CE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82904CEC: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82904CF0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82904CF4: 4BA02A85  bl 0x82307778
	ctx.lr = 0x82904CF8;
	sub_82307778(ctx, base);
	// 82904CF8: 39610100  addi r11, r1, 0x100
	ctx.r[11].s64 = ctx.r[1].s64 + 256;
	// 82904CFC: 39410160  addi r10, r1, 0x160
	ctx.r[10].s64 = ctx.r[1].s64 + 352;
	// 82904D00: C00100E0  lfs f0, 0xe0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82904D04: 39210100  addi r9, r1, 0x100
	ctx.r[9].s64 = ctx.r[1].s64 + 256;
	// 82904D08: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82904D0C: 38E10170  addi r7, r1, 0x170
	ctx.r[7].s64 = ctx.r[1].s64 + 368;
	// 82904D10: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82904D14: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82904D18: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82904E90 size=332
    let mut pc: u32 = 0x82904E90;
    'dispatch: loop {
        match pc {
            0x82904E90 => {
    //   block [0x82904E90..0x82904FDC)
	// 82904E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904E94: 488A32D5  bl 0x831a8168
	ctx.lr = 0x82904E98;
	sub_831A8130(ctx, base);
	// 82904E98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904E9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82904EA0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82904EA4: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 82904EA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82904EAC: 38CB5DCC  addi r6, r11, 0x5dcc
	ctx.r[6].s64 = ctx.r[11].s64 + 24012;
	// 82904EB0: 38AAC8DC  addi r5, r10, -0x3724
	ctx.r[5].s64 = ctx.r[10].s64 + -14116;
	// 82904EB4: 807D00F4  lwz r3, 0xf4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(244 as u32) ) } as u64;
	// 82904EB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82904EBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82904EC0: 488A5089  bl 0x831a9f48
	ctx.lr = 0x82904EC4;
	sub_831A9F48(ctx, base);
	// 82904EC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82904EC8: 41820014  beq 0x82904edc
	if ctx.cr[0].eq {
	pc = 0x82904EDC; continue 'dispatch;
	}
	// 82904ECC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82904ED0: 48012161  bl 0x82917030
	ctx.lr = 0x82904ED4;
	sub_82917030(ctx, base);
	// 82904ED4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82904ED8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82904EDC: 839D0394  lwz r28, 0x394(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(916 as u32) ) } as u64;
	// 82904EE0: 83DD0390  lwz r30, 0x390(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(912 as u32) ) } as u64;
	// 82904EE4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82904EE8: 419A0024  beq cr6, 0x82904f0c
	if ctx.cr[6].eq {
	pc = 0x82904F0C; continue 'dispatch;
	}
	// 82904EEC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82904EF0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82904EF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82904EF8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82904EFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82904F00: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82904F04: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82904F08: 4082FFE8  bne 0x82904ef0
	if !ctx.cr[0].eq {
	pc = 0x82904EF0; continue 'dispatch;
	}
	// 82904F0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82904F10: 419A0070  beq cr6, 0x82904f80
	if ctx.cr[6].eq {
	pc = 0x82904F80; continue 'dispatch;
	}
	// 82904F14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82904F18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904F1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82904F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82904F24: 4E800421  bctrl
	ctx.lr = 0x82904F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82904F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82904F2C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82904F30: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82904F34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82904F38: C00B0030  lfs f0, 0x30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82904F3C: C1AB0034  lfs f13, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82904F40: C18B0038  lfs f12, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82904F44: C16B003C  lfs f11, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82904F48: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82904F4C: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82904F50: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82904F54: D161005C  stfs f11, 0x5c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82904F58: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82904FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82904FE0 size=212
    let mut pc: u32 = 0x82904FE0;
    'dispatch: loop {
        match pc {
            0x82904FE0 => {
    //   block [0x82904FE0..0x829050B4)
	// 82904FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82904FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82904FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82904FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82904FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82904FF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82904FF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82904FFC: 83CB039C  lwz r30, 0x39c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(924 as u32) ) } as u64;
	// 82905000: 806B0398  lwz r3, 0x398(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(920 as u32) ) } as u64;
	// 82905004: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82905008: 419A0024  beq cr6, 0x8290502c
	if ctx.cr[6].eq {
	pc = 0x8290502C; continue 'dispatch;
	}
	// 8290500C: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 82905010: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82905014: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82905018: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8290501C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82905020: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82905024: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82905028: 4082FFE8  bne 0x82905010
	if !ctx.cr[0].eq {
	pc = 0x82905010; continue 'dispatch;
	}
	// 8290502C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905030: 419A0048  beq cr6, 0x82905078
	if ctx.cr[6].eq {
	pc = 0x82905078; continue 'dispatch;
	}
	// 82905034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82905038: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290503C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82905040: 4E800421  bctrl
	ctx.lr = 0x82905044;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82905044: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82905048: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8290504C: C0030030  lfs f0, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905050: C1A30034  lfs f13, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82905054: C1830038  lfs f12, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82905058: C163003C  lfs f11, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8290505C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82905060: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82905064: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82905068: D161005C  stfs f11, 0x5c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8290506C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829050B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829050B8 size=112
    let mut pc: u32 = 0x829050B8;
    'dispatch: loop {
        match pc {
            0x829050B8 => {
    //   block [0x829050B8..0x82905128)
	// 829050B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829050BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829050C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829050C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829050C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829050CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829050D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829050D4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 829050D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829050DC: 4BFFF3DD  bl 0x829044b8
	ctx.lr = 0x829050E0;
	sub_829044B8(ctx, base);
	// 829050E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 829050E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829050E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829050EC: 4B9BAF15  bl 0x822c0000
	ctx.lr = 0x829050F0;
	sub_822C0000(ctx, base);
	// 829050F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829050F4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 829050F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829050FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82905100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905104: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82905108: 419A0008  beq cr6, 0x82905110
	if ctx.cr[6].eq {
	pc = 0x82905110; continue 'dispatch;
	}
	// 8290510C: 4B9BB785  bl 0x822c0890
	ctx.lr = 0x82905110;
	sub_822C0890(ctx, base);
	// 82905110: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82905114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82905118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290511C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82905120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82905124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82905128 size=100
    let mut pc: u32 = 0x82905128;
    'dispatch: loop {
        match pc {
            0x82905128 => {
    //   block [0x82905128..0x8290518C)
	// 82905128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290512C: 488A3041  bl 0x831a816c
	ctx.lr = 0x82905130;
	sub_831A8130(ctx, base);
	// 82905130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82905134: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905138: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290513C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82905140: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82905144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82905148: 388B4FE0  addi r4, r11, 0x4fe0
	ctx.r[4].s64 = ctx.r[11].s64 + 20448;
	// 8290514C: 38A00093  li r5, 0x93
	ctx.r[5].s64 = 147;
	// 82905150: 386003C0  li r3, 0x3c0
	ctx.r[3].s64 = 960;
	// 82905154: 484ED295  bl 0x82df23e8
	ctx.lr = 0x82905158;
	sub_82DF23E8(ctx, base);
	// 82905158: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290515C: 41820018  beq 0x82905174
	if ctx.cr[0].eq {
	pc = 0x82905174; continue 'dispatch;
	}
	// 82905160: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82905164: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905168: 4BFFF869  bl 0x829049d0
	ctx.lr = 0x8290516C;
	sub_829049D0(ctx, base);
	// 8290516C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905170: 48000008  b 0x82905178
	pc = 0x82905178; continue 'dispatch;
	// 82905174: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82905178: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290517C: 4BFFF76D  bl 0x829048e8
	ctx.lr = 0x82905180;
	sub_829048E8(ctx, base);
	// 82905180: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82905188: 488A3034  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82905190 size=420
    let mut pc: u32 = 0x82905190;
    'dispatch: loop {
        match pc {
            0x82905190 => {
    //   block [0x82905190..0x82905334)
	// 82905190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82905194: 488A2FCD  bl 0x831a8160
	ctx.lr = 0x82905198;
	sub_831A8130(ctx, base);
	// 82905198: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290519C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 829051A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829051A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829051A8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 829051AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 829051B0: 485C7839  bl 0x82ecc9e8
	ctx.lr = 0x829051B4;
	sub_82ECC9E8(ctx, base);
	// 829051B4: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 829051B8: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 829051BC: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 829051C0: 99610110  stb r11, 0x110(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[11].u8 ) };
	// 829051C4: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 829051C8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 829051CC: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 829051D0: 4859B561  bl 0x82ea0730
	ctx.lr = 0x829051D4;
	sub_82EA0730(ctx, base);
	// 829051D4: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 829051D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 829051DC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 829051E0: C02AC664  lfs f1, -0x399c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829051E4: 4861500D  bl 0x82f1a1f0
	ctx.lr = 0x829051E8;
	sub_82F1A1F0(ctx, base);
	// 829051E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829051EC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 829051F0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 829051F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829051F8: 4B9E1A59  bl 0x822e6c50
	ctx.lr = 0x829051FC;
	sub_822E6C50(ctx, base);
	// 829051FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82905200: 419A000C  beq cr6, 0x8290520c
	if ctx.cr[6].eq {
	pc = 0x8290520C; continue 'dispatch;
	}
	// 82905204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905208: 4B9E3061  bl 0x822e8268
	ctx.lr = 0x8290520C;
	sub_822E8268(ctx, base);
	// 8290520C: 83410054  lwz r26, 0x54(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82905210: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82905214: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 82905218: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8290521C: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82905220: 4859B511  bl 0x82ea0730
	ctx.lr = 0x82905224;
	sub_82EA0730(ctx, base);
	// 82905224: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82905228: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8290522C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82905230: 485C66B1  bl 0x82ecb8e0
	ctx.lr = 0x82905234;
	sub_82ECB8E0(ctx, base);
	// 82905234: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82905238: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290523C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82905240: 388B4FE0  addi r4, r11, 0x4fe0
	ctx.r[4].s64 = ctx.r[11].s64 + 20448;
	// 82905244: 38A00184  li r5, 0x184
	ctx.r[5].s64 = 388;
	// 82905248: 38600110  li r3, 0x110
	ctx.r[3].s64 = 272;
	// 8290524C: 484ED19D  bl 0x82df23e8
	ctx.lr = 0x82905250;
	sub_82DF23E8(ctx, base);
	// 82905250: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82905254: 41820024  beq 0x82905278
	if ctx.cr[0].eq {
	pc = 0x82905278; continue 'dispatch;
	}
	// 82905258: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8290525C: 389C0038  addi r4, r28, 0x38
	ctx.r[4].s64 = ctx.r[28].s64 + 56;
	// 82905260: 409A0008  bne cr6, 0x82905268
	if !ctx.cr[6].eq {
	pc = 0x82905268; continue 'dispatch;
	}
	// 82905264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82905268: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290526C: 4B9E8C45  bl 0x822edeb0
	ctx.lr = 0x82905270;
	sub_822EDEB0(ctx, base);
	// 82905270: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905274: 48000008  b 0x8290527c
	pc = 0x8290527C; continue 'dispatch;
	// 82905278: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290527C: 3BFC03AC  addi r31, r28, 0x3ac
	ctx.r[31].s64 = ctx.r[28].s64 + 940;
	// 82905280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905284: 4BA5807D  bl 0x8235d300
	ctx.lr = 0x82905288;
	sub_8235D300(ctx, base);
	// 82905288: 807C03AC  lwz r3, 0x3ac(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(940 as u32) ) } as u64;
	// 8290528C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905290: 419A007C  beq cr6, 0x8290530c
	if ctx.cr[6].eq {
	pc = 0x8290530C; continue 'dispatch;
	}
	// 82905294: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82905298: 808B6810  lwz r4, 0x6810(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26640 as u32) ) } as u64;
	// 8290529C: 4BB8A105  bl 0x8248f3a0
	ctx.lr = 0x829052A0;
	sub_8248F3A0(ctx, base);
	// 829052A0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 829052A4: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 829052A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829052AC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 829052B0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 829052B4: 419A0024  beq cr6, 0x829052d8
	if ctx.cr[6].eq {
	pc = 0x829052D8; continue 'dispatch;
	}
	// 829052B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 829052BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 829052C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829052C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 829052C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 829052CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 829052D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829052D4: 4082FFE8  bne 0x829052bc
	if !ctx.cr[0].eq {
	pc = 0x829052BC; continue 'dispatch;
	}
	// 829052D8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 829052DC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829052E0: 4B9E8C81  bl 0x822edf60
	ctx.lr = 0x829052E4;
	sub_822EDF60(ctx, base);
	// 829052E4: 3BDC0010  addi r30, r28, 0x10
	ctx.r[30].s64 = ctx.r[28].s64 + 16;
	// 829052E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829052EC: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829052F0: 4810E391  bl 0x82a13680
	ctx.lr = 0x829052F4;
	sub_82A13680(ctx, base);
	// 829052F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829052F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829052FC: 4B9E8425  bl 0x822ed720
	ctx.lr = 0x82905300;
	sub_822ED720(ctx, base);
	// 82905300: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905308: 4BC0BA39  bl 0x82510d40
	ctx.lr = 0x8290530C;
	sub_82510D40(ctx, base);
	// 8290530C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905314: 419A0008  beq cr6, 0x8290531c
	if ctx.cr[6].eq {
	pc = 0x8290531C; continue 'dispatch;
	}
	// 82905318: 4B9E2F51  bl 0x822e8268
	ctx.lr = 0x8290531C;
	sub_822E8268(ctx, base);
	// 8290531C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82905320: 419A000C  beq cr6, 0x8290532c
	if ctx.cr[6].eq {
	pc = 0x8290532C; continue 'dispatch;
	}
	// 82905324: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82905328: 4B9E2F41  bl 0x822e8268
	ctx.lr = 0x8290532C;
	sub_822E8268(ctx, base);
	// 8290532C: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82905330: 488A2E80  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82905338 size=208
    let mut pc: u32 = 0x82905338;
    'dispatch: loop {
        match pc {
            0x82905338 => {
    //   block [0x82905338..0x82905408)
	// 82905338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290533C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82905340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82905344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82905348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290534C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82905350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82905354: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82905358: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290535C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82905360: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82905364: 4BF63155  bl 0x828684b8
	ctx.lr = 0x82905368;
	sub_828684B8(ctx, base);
	// 82905368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290536C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82905370: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82905374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82905378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290537C: 419A0024  beq cr6, 0x829053a0
	if ctx.cr[6].eq {
	pc = 0x829053A0; continue 'dispatch;
	}
	// 82905380: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82905384: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82905388: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290538C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82905390: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82905394: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82905398: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290539C: 4082FFE8  bne 0x82905384
	if !ctx.cr[0].eq {
	pc = 0x82905384; continue 'dispatch;
	}
	// 829053A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829053A4: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 829053A8: 48702D81  bl 0x83008128
	ctx.lr = 0x829053AC;
	sub_83008128(ctx, base);
	// 829053AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829053B0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 829053B4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 829053B8: 388A4FE0  addi r4, r10, 0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + 20448;
	// 829053BC: 38A002B4  li r5, 0x2b4
	ctx.r[5].s64 = 692;
	// 829053C0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 829053C4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829053C8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 829053CC: 48553675  bl 0x82e58a40
	ctx.lr = 0x829053D0;
	sub_82E58A40(ctx, base);
	// 829053D0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 829053D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829053D8: 419A0008  beq cr6, 0x829053e0
	if ctx.cr[6].eq {
	pc = 0x829053E0; continue 'dispatch;
	}
	// 829053DC: 4B9BB4B5  bl 0x822c0890
	ctx.lr = 0x829053E0;
	sub_822C0890(ctx, base);
	// 829053E0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 829053E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829053E8: 419A0008  beq cr6, 0x829053f0
	if ctx.cr[6].eq {
	pc = 0x829053F0; continue 'dispatch;
	}
	// 829053EC: 4B9BB4A5  bl 0x822c0890
	ctx.lr = 0x829053F0;
	sub_822C0890(ctx, base);
	// 829053F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 829053F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829053F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829053FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82905400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82905404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82905408 size=504
    let mut pc: u32 = 0x82905408;
    'dispatch: loop {
        match pc {
            0x82905408 => {
    //   block [0x82905408..0x82905600)
	// 82905408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290540C: 488A2D59  bl 0x831a8164
	ctx.lr = 0x82905410;
	sub_831A8130(ctx, base);
	// 82905410: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82905414: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82905418: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290541C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82905420: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82905424: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82905428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290542C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82905430: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82905434: 484EE5D5  bl 0x82df3a08
	ctx.lr = 0x82905438;
	sub_82DF3A08(ctx, base);
	// 82905438: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 8290543C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905440: 837F03A0  lwz r27, 0x3a0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(928 as u32) ) } as u64;
	// 82905444: 4BEE79BD  bl 0x827ece00
	ctx.lr = 0x82905448;
	sub_827ECE00(ctx, base);
	// 82905448: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290544C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82905450: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82905454: 4801CC3D  bl 0x82922090
	ctx.lr = 0x82905458;
	sub_82922090(ctx, base);
	// 82905458: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290545C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82905460: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82905464: 41820040  beq 0x829054a4
	if ctx.cr[0].eq {
	pc = 0x829054A4; continue 'dispatch;
	}
	// 82905468: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8290546C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82905470: 814A0BD4  lwz r10, 0xbd4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3028 as u32) ) } as u64;
	// 82905474: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82905478: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290547C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82905480: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82905484: 41820014  beq 0x82905498
	if ctx.cr[0].eq {
	pc = 0x82905498; continue 'dispatch;
	}
	// 82905488: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8290548C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82905490: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82905494: 419AFFE0  beq cr6, 0x82905474
	if ctx.cr[6].eq {
	pc = 0x82905474; continue 'dispatch;
	}
	// 82905498: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290549C: 41820028  beq 0x829054c4
	if ctx.cr[0].eq {
	pc = 0x829054C4; continue 'dispatch;
	}
	// 829054A0: 4800001C  b 0x829054bc
	pc = 0x829054BC; continue 'dispatch;
	// 829054A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829054A8: 480039B1  bl 0x82908e58
	ctx.lr = 0x829054AC;
	sub_82908E58(ctx, base);
	// 829054AC: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 829054B0: 40990014  ble cr6, 0x829054c4
	if !ctx.cr[6].gt {
	pc = 0x829054C4; continue 'dispatch;
	}
	// 829054B4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829054B8: 808B0B34  lwz r4, 0xb34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2868 as u32) ) } as u64;
	// 829054BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829054C0: 484EE3B9  bl 0x82df3878
	ctx.lr = 0x829054C4;
	sub_82DF3878(ctx, base);
	// 829054C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829054C8: 484EE6E1  bl 0x82df3ba8
	ctx.lr = 0x829054CC;
	sub_82DF3BA8(ctx, base);
	// 829054CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829054D0: 4082011C  bne 0x829055ec
	if !ctx.cr[0].eq {
	pc = 0x829055EC; continue 'dispatch;
	}
	// 829054D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829054D8: 4810E1A9  bl 0x82a13680
	ctx.lr = 0x829054DC;
	sub_82A13680(ctx, base);
	// 829054DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829054E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829054E4: 4BCE214D  bl 0x825e7630
	ctx.lr = 0x829054E8;
	sub_825E7630(ctx, base);
	// 829054E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829054EC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 829054F0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 829054F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829054F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 829054FC: 419A0024  beq cr6, 0x82905520
	if ctx.cr[6].eq {
	pc = 0x82905520; continue 'dispatch;
	}
	// 82905500: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82905504: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82905508: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290550C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82905510: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82905514: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82905518: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290551C: 4082FFE8  bne 0x82905504
	if !ctx.cr[0].eq {
	pc = 0x82905504; continue 'dispatch;
	}
	// 82905520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905524: 3B610058  addi r27, r1, 0x58
	ctx.r[27].s64 = ctx.r[1].s64 + 88;
	// 82905528: 48702C01  bl 0x83008128
	ctx.lr = 0x8290552C;
	sub_83008128(ctx, base);
	// 8290552C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905530: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82905534: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82905538: 388B4FE0  addi r4, r11, 0x4fe0
	ctx.r[4].s64 = ctx.r[11].s64 + 20448;
	// 8290553C: 38A002D3  li r5, 0x2d3
	ctx.r[5].s64 = 723;
	// 82905540: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82905544: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82905548: 485534F9  bl 0x82e58a40
	ctx.lr = 0x8290554C;
	sub_82E58A40(ctx, base);
	// 8290554C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82905550: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905554: 419A0008  beq cr6, 0x8290555c
	if ctx.cr[6].eq {
	pc = 0x8290555C; continue 'dispatch;
	}
	// 82905558: 4B9BB339  bl 0x822c0890
	ctx.lr = 0x8290555C;
	sub_822C0890(ctx, base);
	// 8290555C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82905560: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905564: 419A0008  beq cr6, 0x8290556c
	if ctx.cr[6].eq {
	pc = 0x8290556C; continue 'dispatch;
	}
	// 82905568: 4B9BB329  bl 0x822c0890
	ctx.lr = 0x8290556C;
	sub_822C0890(ctx, base);
	// 8290556C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82905570: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82905574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905578: 4800B201  bl 0x82910778
	ctx.lr = 0x8290557C;
	sub_82910778(ctx, base);
	// 8290557C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905580: 480038D9  bl 0x82908e58
	ctx.lr = 0x82905584;
	sub_82908E58(ctx, base);
	// 82905584: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82905588: 4099000C  ble cr6, 0x82905594
	if !ctx.cr[6].gt {
	pc = 0x82905594; continue 'dispatch;
	}
	// 8290558C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82905590: 4800001C  b 0x829055ac
	pc = 0x829055AC; continue 'dispatch;
	// 82905594: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82905598: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290559C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 829055A0: 808B0B70  lwz r4, 0xb70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2928 as u32) ) } as u64;
	// 829055A4: 484EE465  bl 0x82df3a08
	ctx.lr = 0x829055A8;
	sub_82DF3A08(ctx, base);
	// 829055A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829055AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829055B0: 484EE621  bl 0x82df3bd0
	ctx.lr = 0x829055B4;
	sub_82DF3BD0(ctx, base);
	// 829055B4: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829055B8: 4182000C  beq 0x829055c4
	if ctx.cr[0].eq {
	pc = 0x829055C4; continue 'dispatch;
	}
	// 829055BC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829055C0: 484EDE69  bl 0x82df3428
	ctx.lr = 0x829055C4;
	sub_82DF3428(ctx, base);
	// 829055C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829055C8: 484EDBE9  bl 0x82df31b0
	ctx.lr = 0x829055CC;
	sub_82DF31B0(ctx, base);
	// 829055CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829055D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829055D4: 484EE435  bl 0x82df3a08
	ctx.lr = 0x829055D8;
	sub_82DF3A08(ctx, base);
	// 829055D8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 829055DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829055E0: 4BEE7709  bl 0x827ecce8
	ctx.lr = 0x829055E4;
	sub_827ECCE8(ctx, base);
	// 829055E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829055E8: 484EDE41  bl 0x82df3428
	ctx.lr = 0x829055EC;
	sub_82DF3428(ctx, base);
	// 829055EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829055F0: 484EDE39  bl 0x82df3428
	ctx.lr = 0x829055F4;
	sub_82DF3428(ctx, base);
	// 829055F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 829055F8: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 829055FC: 488A2BB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82905600 size=332
    let mut pc: u32 = 0x82905600;
    'dispatch: loop {
        match pc {
            0x82905600 => {
    //   block [0x82905600..0x8290574C)
	// 82905600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82905604: 488A2B65  bl 0x831a8168
	ctx.lr = 0x82905608;
	sub_831A8130(ctx, base);
	// 82905608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290560C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82905610: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82905614: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82905618: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8290561C: 41820038  beq 0x82905654
	if ctx.cr[0].eq {
	pc = 0x82905654; continue 'dispatch;
	}
	// 82905620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905624: 488A4365  bl 0x831a9988
	ctx.lr = 0x82905628;
	sub_831A9988(ctx, base);
	// 82905628: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8290562C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905630: 386B3984  addi r3, r11, 0x3984
	ctx.r[3].s64 = ctx.r[11].s64 + 14724;
	// 82905634: 488A2AC5  bl 0x831a80f8
	ctx.lr = 0x82905638;
	sub_831A80F8(ctx, base);
	// 82905638: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290563C: 41820018  beq 0x82905654
	if ctx.cr[0].eq {
	pc = 0x82905654; continue 'dispatch;
	}
	// 82905640: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905644: 387EFFC8  addi r3, r30, -0x38
	ctx.r[3].s64 = ctx.r[30].s64 + -56;
	// 82905648: 4BFFFDC1  bl 0x82905408
	ctx.lr = 0x8290564C;
	sub_82905408(ctx, base);
	// 8290564C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82905650: 480000F4  b 0x82905744
	pc = 0x82905744; continue 'dispatch;
	// 82905654: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82905658: 419A00DC  beq cr6, 0x82905734
	if ctx.cr[6].eq {
	pc = 0x82905734; continue 'dispatch;
	}
	// 8290565C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905660: 488A4329  bl 0x831a9988
	ctx.lr = 0x82905664;
	sub_831A9988(ctx, base);
	// 82905664: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82905668: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290566C: 386B5CEC  addi r3, r11, 0x5cec
	ctx.r[3].s64 = ctx.r[11].s64 + 23788;
	// 82905670: 488A2A89  bl 0x831a80f8
	ctx.lr = 0x82905674;
	sub_831A80F8(ctx, base);
	// 82905674: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905678: 41820014  beq 0x8290568c
	if ctx.cr[0].eq {
	pc = 0x8290568C; continue 'dispatch;
	}
	// 8290567C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905680: 387EFFC8  addi r3, r30, -0x38
	ctx.r[3].s64 = ctx.r[30].s64 + -56;
	// 82905684: 4BFFFCB5  bl 0x82905338
	ctx.lr = 0x82905688;
	sub_82905338(ctx, base);
	// 82905688: 4BFFFFC4  b 0x8290564c
	pc = 0x8290564C; continue 'dispatch;
	// 8290568C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82905690: 419A00A4  beq cr6, 0x82905734
	if ctx.cr[6].eq {
	pc = 0x82905734; continue 'dispatch;
	}
	// 82905694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905698: 488A42F1  bl 0x831a9988
	ctx.lr = 0x8290569C;
	sub_831A9988(ctx, base);
	// 8290569C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 829056A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829056A4: 386B5CC4  addi r3, r11, 0x5cc4
	ctx.r[3].s64 = ctx.r[11].s64 + 23748;
	// 829056A8: 488A2A51  bl 0x831a80f8
	ctx.lr = 0x829056AC;
	sub_831A80F8(ctx, base);
	// 829056AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829056B0: 41820014  beq 0x829056c4
	if ctx.cr[0].eq {
	pc = 0x829056C4; continue 'dispatch;
	}
	// 829056B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829056B8: 387EFFC8  addi r3, r30, -0x38
	ctx.r[3].s64 = ctx.r[30].s64 + -56;
	// 829056BC: 4BFFEA1D  bl 0x829040d8
	ctx.lr = 0x829056C0;
	sub_829040D8(ctx, base);
	// 829056C0: 4BFFFF8C  b 0x8290564c
	pc = 0x8290564C; continue 'dispatch;
	// 829056C4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 829056C8: 419A006C  beq cr6, 0x82905734
	if ctx.cr[6].eq {
	pc = 0x82905734; continue 'dispatch;
	}
	// 829056CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829056D0: 488A42B9  bl 0x831a9988
	ctx.lr = 0x829056D4;
	sub_831A9988(ctx, base);
	// 829056D4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 829056D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829056DC: 386B3A44  addi r3, r11, 0x3a44
	ctx.r[3].s64 = ctx.r[11].s64 + 14916;
	// 829056E0: 488A2A19  bl 0x831a80f8
	ctx.lr = 0x829056E4;
	sub_831A80F8(ctx, base);
	// 829056E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829056E8: 41820014  beq 0x829056fc
	if ctx.cr[0].eq {
	pc = 0x829056FC; continue 'dispatch;
	}
	// 829056EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829056F0: 387EFFC8  addi r3, r30, -0x38
	ctx.r[3].s64 = ctx.r[30].s64 + -56;
	// 829056F4: 4BFFF79D  bl 0x82904e90
	ctx.lr = 0x829056F8;
	sub_82904E90(ctx, base);
	// 829056F8: 4BFFFF54  b 0x8290564c
	pc = 0x8290564C; continue 'dispatch;
	// 829056FC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82905700: 419A0034  beq cr6, 0x82905734
	if ctx.cr[6].eq {
	pc = 0x82905734; continue 'dispatch;
	}
	// 82905704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905708: 488A4281  bl 0x831a9988
	ctx.lr = 0x8290570C;
	sub_831A9988(ctx, base);
	// 8290570C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82905710: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905714: 386B2380  addi r3, r11, 0x2380
	ctx.r[3].s64 = ctx.r[11].s64 + 9088;
	// 82905718: 488A29E1  bl 0x831a80f8
	ctx.lr = 0x8290571C;
	sub_831A80F8(ctx, base);
	// 8290571C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905720: 41820014  beq 0x82905734
	if ctx.cr[0].eq {
	pc = 0x82905734; continue 'dispatch;
	}
	// 82905724: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905728: 387EFFC8  addi r3, r30, -0x38
	ctx.r[3].s64 = ctx.r[30].s64 + -56;
	// 8290572C: 4BFFF8B5  bl 0x82904fe0
	ctx.lr = 0x82905730;
	sub_82904FE0(ctx, base);
	// 82905730: 4BFFFF1C  b 0x8290564c
	pc = 0x8290564C; continue 'dispatch;
	// 82905734: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82905738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290573C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905740: 48059819  bl 0x8295ef58
	ctx.lr = 0x82905744;
	sub_8295EF58(ctx, base);
	// 82905744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82905748: 488A2A70  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82905750 size=128
    let mut pc: u32 = 0x82905750;
    'dispatch: loop {
        match pc {
            0x82905750 => {
    //   block [0x82905750..0x829057D0)
	// 82905750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82905754: 488A2A19  bl 0x831a816c
	ctx.lr = 0x82905758;
	sub_831A8130(ctx, base);
	// 82905758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290575C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82905760: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82905764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82905768: 3BEBAFB4  addi r31, r11, -0x504c
	ctx.r[31].s64 = ctx.r[11].s64 + -20556;
	// 8290576C: 816AAFBC  lwz r11, -0x5044(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20548 as u32) ) } as u64;
	// 82905770: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82905774: 40820024  bne 0x82905798
	if !ctx.cr[0].eq {
	pc = 0x82905798; continue 'dispatch;
	}
	// 82905778: 3D208236  lis r9, -0x7dca
	ctx.r[9].s64 = -2110390272;
	// 8290577C: 3D008290  lis r8, -0x7d70
	ctx.r[8].s64 = -2104492032;
	// 82905780: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82905784: 3929A778  addi r9, r9, -0x5888
	ctx.r[9].s64 = ctx.r[9].s64 + -22664;
	// 82905788: 39084580  addi r8, r8, 0x4580
	ctx.r[8].s64 = ctx.r[8].s64 + 17792;
	// 8290578C: 916AAFBC  stw r11, -0x5044(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20548 as u32), ctx.r[11].u32 ) };
	// 82905790: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82905794: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82905798: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8290579C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 829057A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829057A4: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 829057A8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 829057AC: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829057B0: 4BFC2129  bl 0x828c78d8
	ctx.lr = 0x829057B4;
	sub_828C78D8(ctx, base);
	// 829057B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829057B8: 4182000C  beq 0x829057c4
	if ctx.cr[0].eq {
	pc = 0x829057C4; continue 'dispatch;
	}
	// 829057BC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 829057C0: 48000008  b 0x829057c8
	pc = 0x829057C8; continue 'dispatch;
	// 829057C4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 829057C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 829057CC: 488A29F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829057D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829057D0 size=492
    let mut pc: u32 = 0x829057D0;
    'dispatch: loop {
        match pc {
            0x829057D0 => {
    //   block [0x829057D0..0x829059BC)
	// 829057D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829057D4: 488A2991  bl 0x831a8164
	ctx.lr = 0x829057D8;
	sub_831A8130(ctx, base);
	// 829057D8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 829057DC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829057E0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829057E4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 829057E8: 3BCB4FE0  addi r30, r11, 0x4fe0
	ctx.r[30].s64 = ctx.r[11].s64 + 20448;
	// 829057EC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 829057F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 829057F4: 38A0008A  li r5, 0x8a
	ctx.r[5].s64 = 138;
	// 829057F8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 829057FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82905800: 386003C0  li r3, 0x3c0
	ctx.r[3].s64 = 960;
	// 82905804: 484ECBE5  bl 0x82df23e8
	ctx.lr = 0x82905808;
	sub_82DF23E8(ctx, base);
	// 82905808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290580C: 41820020  beq 0x8290582c
	if ctx.cr[0].eq {
	pc = 0x8290582C; continue 'dispatch;
	}
	// 82905810: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82905814: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82905818: 38ABBA80  addi r5, r11, -0x4580
	ctx.r[5].s64 = ctx.r[11].s64 + -17792;
	// 8290581C: 388A6910  addi r4, r10, 0x6910
	ctx.r[4].s64 = ctx.r[10].s64 + 26896;
	// 82905820: 4BFFF1B1  bl 0x829049d0
	ctx.lr = 0x82905824;
	sub_829049D0(ctx, base);
	// 82905824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82905828: 48000008  b 0x82905830
	pc = 0x82905830; continue 'dispatch;
	// 8290582C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82905830: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82905834: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905838: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290583C: 4BFFEBB5  bl 0x829043f0
	ctx.lr = 0x82905840;
	sub_829043F0(ctx, base);
	// 82905840: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82905844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905848: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290584C: 4B9BA7B5  bl 0x822c0000
	ctx.lr = 0x82905850;
	sub_822C0000(ctx, base);
	// 82905850: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82905854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82905858: 38A0008B  li r5, 0x8b
	ctx.r[5].s64 = 139;
	// 8290585C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82905860: 4B9BAB79  bl 0x822c03d8
	ctx.lr = 0x82905864;
	sub_822C03D8(ctx, base);
	// 82905864: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82905868: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8290586C: 418200E0  beq 0x8290594c
	if ctx.cr[0].eq {
	pc = 0x8290594C; continue 'dispatch;
	}
	// 82905870: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82905874: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905878: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8290587C: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82905880: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82905884: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82905888: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290588C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82905890: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905894: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82905898: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8290589C: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 829058A0: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 829058A4: 419A0024  beq cr6, 0x829058c8
	if ctx.cr[6].eq {
	pc = 0x829058C8; continue 'dispatch;
	}
	// 829058A8: 395D0004  addi r10, r29, 4
	ctx.r[10].s64 = ctx.r[29].s64 + 4;
	// 829058AC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 829058B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829058B4: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 829058B8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 829058BC: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 829058C0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829058C4: 4082FFE8  bne 0x829058ac
	if !ctx.cr[0].eq {
	pc = 0x829058AC; continue 'dispatch;
	}
	// 829058C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829058CC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 829058D0: 409A0008  bne cr6, 0x829058d8
	if !ctx.cr[6].eq {
	pc = 0x829058D8; continue 'dispatch;
	}
	// 829058D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829058D8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 829058DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 829058E0: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 829058E4: 419A0024  beq cr6, 0x82905908
	if ctx.cr[6].eq {
	pc = 0x82905908; continue 'dispatch;
	}
	// 829058E8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 829058EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 829058F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829058F4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 829058F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 829058FC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82905900: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82905904: 4082FFE8  bne 0x829058ec
	if !ctx.cr[0].eq {
	pc = 0x829058EC; continue 'dispatch;
	}
	// 82905908: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8290590C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82905910: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82905914: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82905918: 48577329  bl 0x82e7cc40
	ctx.lr = 0x8290591C;
	sub_82E7CC40(ctx, base);
	// 8290591C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82905920: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82905924: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82905928: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8290592C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82905930: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82905934: 4BE935FD  bl 0x82798f30
	ctx.lr = 0x82905938;
	sub_82798F30(ctx, base);
	// 82905938: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290593C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905940: 4BE95D29  bl 0x8279b668
	ctx.lr = 0x82905944;
	sub_8279B668(ctx, base);
	// 82905944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82905948: 48000008  b 0x82905950
	pc = 0x82905950; continue 'dispatch;
	// 8290594C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82905950: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82905954: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82905958: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290595C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905960: 4BC922F1  bl 0x82597c50
	ctx.lr = 0x82905964;
	sub_82597C50(ctx, base);
	// 82905964: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82905968: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290596C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905970: 4B9BA691  bl 0x822c0000
	ctx.lr = 0x82905974;
	sub_822C0000(ctx, base);
	// 82905974: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905978: 41820024  beq 0x8290599c
	if ctx.cr[0].eq {
	pc = 0x8290599C; continue 'dispatch;
	}
	// 8290597C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82905980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905984: 419A0008  beq cr6, 0x8290598c
	if ctx.cr[6].eq {
	pc = 0x8290598C; continue 'dispatch;
	}
	// 82905988: 4B9BAF09  bl 0x822c0890
	ctx.lr = 0x8290598C;
	sub_822C0890(ctx, base);
	// 8290598C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82905990: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82905994: 419A0008  beq cr6, 0x8290599c
	if ctx.cr[6].eq {
	pc = 0x8290599C; continue 'dispatch;
	}
	// 82905998: 4B9BAEF9  bl 0x822c0890
	ctx.lr = 0x8290599C;
	sub_822C0890(ctx, base);
	// 8290599C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 829059A0: 419A000C  beq cr6, 0x829059ac
	if ctx.cr[6].eq {
	pc = 0x829059AC; continue 'dispatch;
	}
	// 829059A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829059A8: 4B9BAEE9  bl 0x822c0890
	ctx.lr = 0x829059AC;
	sub_822C0890(ctx, base);
	// 829059AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 829059B0: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 829059B4: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 829059B8: 488A27FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829059C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829059C0 size=128
    let mut pc: u32 = 0x829059C0;
    'dispatch: loop {
        match pc {
            0x829059C0 => {
    //   block [0x829059C0..0x82905A40)
	// 829059C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829059C4: 488A27A9  bl 0x831a816c
	ctx.lr = 0x829059C8;
	sub_831A8130(ctx, base);
	// 829059C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829059CC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 829059D0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829059D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829059D8: 3BEBAFC0  addi r31, r11, -0x5040
	ctx.r[31].s64 = ctx.r[11].s64 + -20544;
	// 829059DC: 816AAFC8  lwz r11, -0x5038(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20536 as u32) ) } as u64;
	// 829059E0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 829059E4: 40820024  bne 0x82905a08
	if !ctx.cr[0].eq {
	pc = 0x82905A08; continue 'dispatch;
	}
	// 829059E8: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 829059EC: 3D008290  lis r8, -0x7d70
	ctx.r[8].s64 = -2104492032;
	// 829059F0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 829059F4: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 829059F8: 39084940  addi r8, r8, 0x4940
	ctx.r[8].s64 = ctx.r[8].s64 + 18752;
	// 829059FC: 916AAFC8  stw r11, -0x5038(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20536 as u32), ctx.r[11].u32 ) };
	// 82905A00: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82905A04: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82905A08: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82905A0C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82905A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905A14: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82905A18: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82905A1C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905A20: 4BD4EBA1  bl 0x826545c0
	ctx.lr = 0x82905A24;
	sub_826545C0(ctx, base);
	// 82905A24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905A28: 4182000C  beq 0x82905a34
	if ctx.cr[0].eq {
	pc = 0x82905A34; continue 'dispatch;
	}
	// 82905A2C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82905A30: 48000008  b 0x82905a38
	pc = 0x82905A38; continue 'dispatch;
	// 82905A34: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82905A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82905A3C: 488A2780  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82905A40 size=128
    let mut pc: u32 = 0x82905A40;
    'dispatch: loop {
        match pc {
            0x82905A40 => {
    //   block [0x82905A40..0x82905AC0)
	// 82905A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82905A44: 488A2729  bl 0x831a816c
	ctx.lr = 0x82905A48;
	sub_831A8130(ctx, base);
	// 82905A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82905A4C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82905A50: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82905A54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82905A58: 3BEBAFCC  addi r31, r11, -0x5034
	ctx.r[31].s64 = ctx.r[11].s64 + -20532;
	// 82905A5C: 816AAFD4  lwz r11, -0x502c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20524 as u32) ) } as u64;
	// 82905A60: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82905A64: 40820024  bne 0x82905a88
	if !ctx.cr[0].eq {
	pc = 0x82905A88; continue 'dispatch;
	}
	// 82905A68: 3D208271  lis r9, -0x7d8f
	ctx.r[9].s64 = -2106523648;
	// 82905A6C: 3D008290  lis r8, -0x7d70
	ctx.r[8].s64 = -2104492032;
	// 82905A70: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82905A74: 39298E48  addi r9, r9, -0x71b8
	ctx.r[9].s64 = ctx.r[9].s64 + -29112;
	// 82905A78: 39084988  addi r8, r8, 0x4988
	ctx.r[8].s64 = ctx.r[8].s64 + 18824;
	// 82905A7C: 916AAFD4  stw r11, -0x502c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20524 as u32), ctx.r[11].u32 ) };
	// 82905A80: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82905A84: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82905A88: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82905A8C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82905A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82905A94: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82905A98: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82905A9C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905AA0: 4BD4EB21  bl 0x826545c0
	ctx.lr = 0x82905AA4;
	sub_826545C0(ctx, base);
	// 82905AA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905AA8: 4182000C  beq 0x82905ab4
	if ctx.cr[0].eq {
	pc = 0x82905AB4; continue 'dispatch;
	}
	// 82905AAC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82905AB0: 48000008  b 0x82905ab8
	pc = 0x82905AB8; continue 'dispatch;
	// 82905AB4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82905AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82905ABC: 488A2700  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82905AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82905AC0 size=4668
    let mut pc: u32 = 0x82905AC0;
    'dispatch: loop {
        match pc {
            0x82905AC0 => {
    //   block [0x82905AC0..0x82906CFC)
	// 82905AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82905AC4: 488A266D  bl 0x831a8130
	ctx.lr = 0x82905AC8;
	sub_831A8130(ctx, base);
	// 82905AC8: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82905ACC: 488A2FAD  bl 0x831a8a78
	ctx.lr = 0x82905AD0;
	sub_831A8A40(ctx, base);
	// 82905AD0: 9421FAF0  stwu r1, -0x510(r1)
	ea = ctx.r[1].u32.wrapping_add(-1296 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82905AD4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82905AD8: 90A10534  stw r5, 0x534(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1332 as u32), ctx.r[5].u32 ) };
	// 82905ADC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82905AE0: 9301053C  stw r24, 0x53c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1340 as u32), ctx.r[24].u32 ) };
	// 82905AE4: 4805936D  bl 0x8295ee50
	ctx.lr = 0x82905AE8;
	sub_8295EE50(ctx, base);
	// 82905AE8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 82905AEC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82905AF0: 39EB7670  addi r15, r11, 0x7670
	ctx.r[15].s64 = ctx.r[11].s64 + 30320;
	// 82905AF4: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82905AF8: 484EDF11  bl 0x82df3a08
	ctx.lr = 0x82905AFC;
	sub_82DF3A08(ctx, base);
	// 82905AFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905B00: 38A10078  addi r5, r1, 0x78
	ctx.r[5].s64 = ctx.r[1].s64 + 120;
	// 82905B04: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82905B08: 4800AAF9  bl 0x82910600
	ctx.lr = 0x82905B0C;
	sub_82910600(ctx, base);
	// 82905B0C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82905B10: 484ED919  bl 0x82df3428
	ctx.lr = 0x82905B14;
	sub_82DF3428(ctx, base);
	// 82905B14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905B18: 4800ADA9  bl 0x829108c0
	ctx.lr = 0x82905B1C;
	sub_829108C0(ctx, base);
	// 82905B1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905B20: 41820010  beq 0x82905b30
	if ctx.cr[0].eq {
	pc = 0x82905B30; continue 'dispatch;
	}
	// 82905B24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905B28: 4BC0AAA9  bl 0x825105d0
	ctx.lr = 0x82905B2C;
	sub_825105D0(ctx, base);
	// 82905B2C: 480011C0  b 0x82906cec
	pc = 0x82906CEC; continue 'dispatch;
	// 82905B30: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82905B38: 3A2B4FE0  addi r17, r11, 0x4fe0
	ctx.r[17].s64 = ctx.r[11].s64 + 20448;
	// 82905B3C: 38A000B9  li r5, 0xb9
	ctx.r[5].s64 = 185;
	// 82905B40: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82905B44: 9221007C  stw r17, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[17].u32 ) };
	// 82905B48: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 82905B4C: 4B9BA88D  bl 0x822c03d8
	ctx.lr = 0x82905B50;
	sub_822C03D8(ctx, base);
	// 82905B50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82905B54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82905B58: 41820020  beq 0x82905b78
	if ctx.cr[0].eq {
	pc = 0x82905B78; continue 'dispatch;
	}
	// 82905B5C: 357DFFF0  addic. r11, r29, -0x10
	ctx.xer.ca = (ctx.r[29].u32 > (!(-16 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82905B60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82905B64: 40820008  bne 0x82905b6c
	if !ctx.cr[0].eq {
	pc = 0x82905B6C; continue 'dispatch;
	}
	// 82905B68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905B6C: 48001195  bl 0x82906d00
	ctx.lr = 0x82905B70;
	sub_82906D00(ctx, base);
	// 82905B70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905B74: 48000008  b 0x82905b7c
	pc = 0x82905B7C; continue 'dispatch;
	// 82905B78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82905B7C: 39DD00E4  addi r14, r29, 0xe4
	ctx.r[14].s64 = ctx.r[29].s64 + 228;
	// 82905B80: 7DC37378  mr r3, r14
	ctx.r[3].u64 = ctx.r[14].u64;
	// 82905B84: 4BFFF535  bl 0x829050b8
	ctx.lr = 0x82905B88;
	sub_829050B8(ctx, base);
	// 82905B88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82905B8C: 3F808200  lis r28, -0x7e00
	ctx.r[28].s64 = -2113929216;
	// 82905B90: C3AB9450  lfs f29, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82905B94: 83DD00E4  lwz r30, 0xe4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(228 as u32) ) } as u64;
	// 82905B98: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82905B9C: 419A0074  beq cr6, 0x82905c10
	if ctx.cr[6].eq {
	pc = 0x82905C10; continue 'dispatch;
	}
	// 82905BA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905BA4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82905BA8: 4810DAD9  bl 0x82a13680
	ctx.lr = 0x82905BAC;
	sub_82A13680(ctx, base);
	// 82905BAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905BB0: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82905BB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82905BBC: 4E800421  bctrl
	ctx.lr = 0x82905BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82905BC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82905BC4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82905BC8: 4810DAC1  bl 0x82a13688
	ctx.lr = 0x82905BCC;
	sub_82A13688(ctx, base);
	// 82905BCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82905BD0: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82905BD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82905BDC: 4E800421  bctrl
	ctx.lr = 0x82905BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82905BE0: C07C08A8  lfs f3, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82905BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82905BE8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82905BEC: 386103D0  addi r3, r1, 0x3d0
	ctx.r[3].s64 = ctx.r[1].s64 + 976;
	// 82905BF0: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 82905BF4: 4801B7BD  bl 0x829213b0
	ctx.lr = 0x82905BF8;
	sub_829213B0(ctx, base);
	// 82905BF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82905BFC: 388103D0  addi r4, r1, 0x3d0
	ctx.r[4].s64 = ctx.r[1].s64 + 976;
	// 82905C00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82905C04: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82905C08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82905C0C: 4E800421  bctrl
	ctx.lr = 0x82905C10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82905C10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82905C14: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82905C18: 4BEE69B1  bl 0x827ec5c8
	ctx.lr = 0x82905C1C;
	sub_827EC5C8(ctx, base);
	// 82905C1C: 81610104  lwz r11, 0x104(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82905C20: 82030000  lwz r16, 0(r3)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82905C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82905C28: 419A000C  beq cr6, 0x82905c34
	if ctx.cr[6].eq {
	pc = 0x82905C34; continue 'dispatch;
	}
	// 82905C2C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82905C30: 4B9BAC61  bl 0x822c0890
	ctx.lr = 0x82905C34;
	sub_822C0890(ctx, base);
	// 82905C34: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82905C38: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82905C3C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82905C40: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82905C44: 3B2BAFDC  addi r25, r11, -0x5024
	ctx.r[25].s64 = ctx.r[11].s64 + -20516;
	// 82905C48: 396A4D6C  addi r11, r10, 0x4d6c
	ctx.r[11].s64 = ctx.r[10].s64 + 19820;
	// 82905C4C: C3897BC8  lfs f28, 0x7bc8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(31688 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82905C50: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 82905C54: C3C808A4  lfs f30, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82905C58: 93210078  stw r25, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[25].u32 ) };
	// 82905C5C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82905C60: 419A0868  beq cr6, 0x829064c8
	if ctx.cr[6].eq {
	pc = 0x829064C8; continue 'dispatch;
	}
	// 82905C64: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905C68: D3A10138  stfs f29, 0x138(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 82905C6C: 3F40832D  lis r26, -0x7cd3
	ctx.r[26].s64 = -2094202880;
	// 82905C70: D3C10140  stfs f30, 0x140(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 82905C74: 396B50B4  addi r11, r11, 0x50b4
	ctx.r[11].s64 = ctx.r[11].s64 + 20660;
	// 82905C78: 93E1013C  stw r31, 0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), ctx.r[31].u32 ) };
	// 82905C7C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82905C80: 9BE1014C  stb r31, 0x14c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[31].u8 ) };
	// 82905C84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82905C88: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 82905C8C: 817AD078  lwz r11, -0x2f88(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-12168 as u32) ) } as u64;
	// 82905C90: C3EA9534  lfs f31, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82905C94: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905C98: D3E10144  stfs f31, 0x144(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), tmp.u32 ) };
	// 82905C9C: 91410134  stw r10, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[10].u32 ) };
	// 82905CA0: D3E10148  stfs f31, 0x148(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), tmp.u32 ) };
	// 82905CA4: 91610130  stw r11, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 82905CA8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82905CAC: 484F8C1D  bl 0x82dfe8c8
	ctx.lr = 0x82905CB0;
	sub_82DFE8C8(ctx, base);
	// 82905CB0: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 82905CB4: 484F8C15  bl 0x82dfe8c8
	ctx.lr = 0x82905CB8;
	sub_82DFE8C8(ctx, base);
	// 82905CB8: 3F20832D  lis r25, -0x7cd3
	ctx.r[25].s64 = -2094202880;
	// 82905CBC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905CC0: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905CC4: D0010168  stfs f0, 0x168(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 82905CC8: 93E1016C  stw r31, 0x16c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), ctx.r[31].u32 ) };
	// 82905CCC: 3A4B50A8  addi r18, r11, 0x50a8
	ctx.r[18].s64 = ctx.r[11].s64 + 20648;
	// 82905CD0: D3C10170  stfs f30, 0x170(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 82905CD4: D3E10174  stfs f31, 0x174(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 82905CD8: 9BE1017C  stb r31, 0x17c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), ctx.r[31].u8 ) };
	// 82905CDC: D3E10178  stfs f31, 0x178(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), tmp.u32 ) };
	// 82905CE0: 92410164  stw r18, 0x164(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(356 as u32), ctx.r[18].u32 ) };
	// 82905CE4: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82905CE8: 8179F3F8  lwz r11, -0xc08(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 82905CEC: 91610160  stw r11, 0x160(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 82905CF0: 484F8BD9  bl 0x82dfe8c8
	ctx.lr = 0x82905CF4;
	sub_82DFE8C8(ctx, base);
	// 82905CF4: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 82905CF8: 484F8BD1  bl 0x82dfe8c8
	ctx.lr = 0x82905CFC;
	sub_82DFE8C8(ctx, base);
	// 82905CFC: 3F00832D  lis r24, -0x7cd3
	ctx.r[24].s64 = -2094202880;
	// 82905D00: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905D04: D3810198  stfs f28, 0x198(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(408 as u32), tmp.u32 ) };
	// 82905D08: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82905D0C: D3C101A0  stfs f30, 0x1a0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), tmp.u32 ) };
	// 82905D10: 3B6B509C  addi r27, r11, 0x509c
	ctx.r[27].s64 = ctx.r[11].s64 + 20636;
	// 82905D14: D3E101A4  stfs f31, 0x1a4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), tmp.u32 ) };
	// 82905D18: D3E101A8  stfs f31, 0x1a8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(424 as u32), tmp.u32 ) };
	// 82905D1C: 93C1019C  stw r30, 0x19c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), ctx.r[30].u32 ) };
	// 82905D20: 93610194  stw r27, 0x194(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(404 as u32), ctx.r[27].u32 ) };
	// 82905D24: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 82905D28: 9BE101AC  stb r31, 0x1ac(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), ctx.r[31].u8 ) };
	// 82905D2C: 8178D07C  lwz r11, -0x2f84(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-12164 as u32) ) } as u64;
	// 82905D30: 91610190  stw r11, 0x190(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 82905D34: 484F8B95  bl 0x82dfe8c8
	ctx.lr = 0x82905D38;
	sub_82DFE8C8(ctx, base);
	// 82905D38: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 82905D3C: 484F8B8D  bl 0x82dfe8c8
	ctx.lr = 0x82905D40;
	sub_82DFE8C8(ctx, base);
	// 82905D40: 3EE0832D  lis r23, -0x7cd3
	ctx.r[23].s64 = -2094202880;
	// 82905D44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82905D48: D3C101D0  stfs f30, 0x1d0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(464 as u32), tmp.u32 ) };
	// 82905D4C: D3E101D4  stfs f31, 0x1d4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(468 as u32), tmp.u32 ) };
	// 82905D50: 936101C4  stw r27, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[27].u32 ) };
	// 82905D54: D3E101D8  stfs f31, 0x1d8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(472 as u32), tmp.u32 ) };
	// 82905D58: 93C101CC  stw r30, 0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 82905D5C: 9BE101DC  stb r31, 0x1dc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(476 as u32), ctx.r[31].u8 ) };
	// 82905D60: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 82905D64: 8177D074  lwz r11, -0x2f8c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-12172 as u32) ) } as u64;
	// 82905D68: C00A2980  lfs f0, 0x2980(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905D6C: D00101C8  stfs f0, 0x1c8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), tmp.u32 ) };
	// 82905D70: 916101C0  stw r11, 0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 82905D74: 484F8B55  bl 0x82dfe8c8
	ctx.lr = 0x82905D78;
	sub_82DFE8C8(ctx, base);
	// 82905D78: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 82905D7C: 484F8B4D  bl 0x82dfe8c8
	ctx.lr = 0x82905D80;
	sub_82DFE8C8(ctx, base);
	// 82905D80: 3EC0832D  lis r22, -0x7cd3
	ctx.r[22].s64 = -2094202880;
	// 82905D84: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905D88: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905D8C: D00101F8  stfs f0, 0x1f8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(504 as u32), tmp.u32 ) };
	// 82905D90: 93C101FC  stw r30, 0x1fc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(508 as u32), ctx.r[30].u32 ) };
	// 82905D94: 394B508C  addi r10, r11, 0x508c
	ctx.r[10].s64 = ctx.r[11].s64 + 20620;
	// 82905D98: D3C10200  stfs f30, 0x200(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), tmp.u32 ) };
	// 82905D9C: D3E10204  stfs f31, 0x204(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(516 as u32), tmp.u32 ) };
	// 82905DA0: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 82905DA4: 8176D088  lwz r11, -0x2f78(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12152 as u32) ) } as u64;
	// 82905DA8: D3E10208  stfs f31, 0x208(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(520 as u32), tmp.u32 ) };
	// 82905DAC: 914101F4  stw r10, 0x1f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), ctx.r[10].u32 ) };
	// 82905DB0: 9BE1020C  stb r31, 0x20c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(524 as u32), ctx.r[31].u8 ) };
	// 82905DB4: 916101F0  stw r11, 0x1f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 82905DB8: 484F8B11  bl 0x82dfe8c8
	ctx.lr = 0x82905DBC;
	sub_82DFE8C8(ctx, base);
	// 82905DBC: 38610218  addi r3, r1, 0x218
	ctx.r[3].s64 = ctx.r[1].s64 + 536;
	// 82905DC0: 484F8B09  bl 0x82dfe8c8
	ctx.lr = 0x82905DC4;
	sub_82DFE8C8(ctx, base);
	// 82905DC4: 3EA0832D  lis r21, -0x7cd3
	ctx.r[21].s64 = -2094202880;
	// 82905DC8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905DCC: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905DD0: D0010228  stfs f0, 0x228(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(552 as u32), tmp.u32 ) };
	// 82905DD4: 93C1022C  stw r30, 0x22c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(556 as u32), ctx.r[30].u32 ) };
	// 82905DD8: 394B5080  addi r10, r11, 0x5080
	ctx.r[10].s64 = ctx.r[11].s64 + 20608;
	// 82905DDC: D3C10230  stfs f30, 0x230(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(560 as u32), tmp.u32 ) };
	// 82905DE0: D3E10234  stfs f31, 0x234(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(564 as u32), tmp.u32 ) };
	// 82905DE4: 9BE1023C  stb r31, 0x23c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(572 as u32), ctx.r[31].u8 ) };
	// 82905DE8: D3E10238  stfs f31, 0x238(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(568 as u32), tmp.u32 ) };
	// 82905DEC: 91410224  stw r10, 0x224(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(548 as u32), ctx.r[10].u32 ) };
	// 82905DF0: 38610240  addi r3, r1, 0x240
	ctx.r[3].s64 = ctx.r[1].s64 + 576;
	// 82905DF4: 8175D084  lwz r11, -0x2f7c(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12156 as u32) ) } as u64;
	// 82905DF8: 91610220  stw r11, 0x220(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(544 as u32), ctx.r[11].u32 ) };
	// 82905DFC: 484F8ACD  bl 0x82dfe8c8
	ctx.lr = 0x82905E00;
	sub_82DFE8C8(ctx, base);
	// 82905E00: 38610248  addi r3, r1, 0x248
	ctx.r[3].s64 = ctx.r[1].s64 + 584;
	// 82905E04: 484F8AC5  bl 0x82dfe8c8
	ctx.lr = 0x82905E08;
	sub_82DFE8C8(ctx, base);
	// 82905E08: 3F60832D  lis r27, -0x7cd3
	ctx.r[27].s64 = -2094202880;
	// 82905E0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905E10: D3C10260  stfs f30, 0x260(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(608 as u32), tmp.u32 ) };
	// 82905E14: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82905E18: D3E10264  stfs f31, 0x264(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(612 as u32), tmp.u32 ) };
	// 82905E1C: 392B5074  addi r9, r11, 0x5074
	ctx.r[9].s64 = ctx.r[11].s64 + 20596;
	// 82905E20: D3E10268  stfs f31, 0x268(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(616 as u32), tmp.u32 ) };
	// 82905E24: 93C1025C  stw r30, 0x25c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(604 as u32), ctx.r[30].u32 ) };
	// 82905E28: 38610270  addi r3, r1, 0x270
	ctx.r[3].s64 = ctx.r[1].s64 + 624;
	// 82905E2C: 9BE1026C  stb r31, 0x26c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(620 as u32), ctx.r[31].u8 ) };
	// 82905E30: 91210254  stw r9, 0x254(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(596 as u32), ctx.r[9].u32 ) };
	// 82905E34: C00AACFC  lfs f0, -0x5304(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21252 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905E38: D0010258  stfs f0, 0x258(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(600 as u32), tmp.u32 ) };
	// 82905E3C: 817BD080  lwz r11, -0x2f80(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-12160 as u32) ) } as u64;
	// 82905E40: 91610250  stw r11, 0x250(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(592 as u32), ctx.r[11].u32 ) };
	// 82905E44: 484F8A85  bl 0x82dfe8c8
	ctx.lr = 0x82905E48;
	sub_82DFE8C8(ctx, base);
	// 82905E48: 38610278  addi r3, r1, 0x278
	ctx.r[3].s64 = ctx.r[1].s64 + 632;
	// 82905E4C: 484F8A7D  bl 0x82dfe8c8
	ctx.lr = 0x82905E50;
	sub_82DFE8C8(ctx, base);
	// 82905E50: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82905E54: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82905E58: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905E5C: D0010288  stfs f0, 0x288(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 82905E60: 93C1028C  stw r30, 0x28c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(652 as u32), ctx.r[30].u32 ) };
	// 82905E64: 394A5068  addi r10, r10, 0x5068
	ctx.r[10].s64 = ctx.r[10].s64 + 20584;
	// 82905E68: D3C10290  stfs f30, 0x290(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(656 as u32), tmp.u32 ) };
	// 82905E6C: D3E10294  stfs f31, 0x294(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 82905E70: 9BE1029C  stb r31, 0x29c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(668 as u32), ctx.r[31].u8 ) };
	// 82905E74: D3E10298  stfs f31, 0x298(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(664 as u32), tmp.u32 ) };
	// 82905E78: 91410284  stw r10, 0x284(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 82905E7C: 386102A0  addi r3, r1, 0x2a0
	ctx.r[3].s64 = ctx.r[1].s64 + 672;
	// 82905E80: 816BD08C  lwz r11, -0x2f74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12148 as u32) ) } as u64;
	// 82905E84: 91610280  stw r11, 0x280(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(640 as u32), ctx.r[11].u32 ) };
	// 82905E88: 484F8A41  bl 0x82dfe8c8
	ctx.lr = 0x82905E8C;
	sub_82DFE8C8(ctx, base);
	// 82905E8C: 386102A8  addi r3, r1, 0x2a8
	ctx.r[3].s64 = ctx.r[1].s64 + 680;
	// 82905E90: 484F8A39  bl 0x82dfe8c8
	ctx.lr = 0x82905E94;
	sub_82DFE8C8(ctx, base);
	// 82905E94: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82905E98: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82905E9C: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905EA0: D00102B8  stfs f0, 0x2b8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(696 as u32), tmp.u32 ) };
	// 82905EA4: 93C102BC  stw r30, 0x2bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(700 as u32), ctx.r[30].u32 ) };
	// 82905EA8: 394A505C  addi r10, r10, 0x505c
	ctx.r[10].s64 = ctx.r[10].s64 + 20572;
	// 82905EAC: D3C102C0  stfs f30, 0x2c0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(704 as u32), tmp.u32 ) };
	// 82905EB0: D3E102C4  stfs f31, 0x2c4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(708 as u32), tmp.u32 ) };
	// 82905EB4: 9BE102CC  stb r31, 0x2cc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(716 as u32), ctx.r[31].u8 ) };
	// 82905EB8: D3E102C8  stfs f31, 0x2c8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(712 as u32), tmp.u32 ) };
	// 82905EBC: 914102B4  stw r10, 0x2b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 82905EC0: 386102D0  addi r3, r1, 0x2d0
	ctx.r[3].s64 = ctx.r[1].s64 + 720;
	// 82905EC4: 816BD090  lwz r11, -0x2f70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12144 as u32) ) } as u64;
	// 82905EC8: 916102B0  stw r11, 0x2b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(688 as u32), ctx.r[11].u32 ) };
	// 82905ECC: 484F89FD  bl 0x82dfe8c8
	ctx.lr = 0x82905ED0;
	sub_82DFE8C8(ctx, base);
	// 82905ED0: 386102D8  addi r3, r1, 0x2d8
	ctx.r[3].s64 = ctx.r[1].s64 + 728;
	// 82905ED4: 484F89F5  bl 0x82dfe8c8
	ctx.lr = 0x82905ED8;
	sub_82DFE8C8(ctx, base);
	// 82905ED8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905EDC: D3A102E8  stfs f29, 0x2e8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(744 as u32), tmp.u32 ) };
	// 82905EE0: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82905EE4: D3C102F0  stfs f30, 0x2f0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(752 as u32), tmp.u32 ) };
	// 82905EE8: 392B5050  addi r9, r11, 0x5050
	ctx.r[9].s64 = ctx.r[11].s64 + 20560;
	// 82905EEC: 816AF470  lwz r11, -0xb90(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2960 as u32) ) } as u64;
	// 82905EF0: D3E102F4  stfs f31, 0x2f4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(756 as u32), tmp.u32 ) };
	// 82905EF4: D3E102F8  stfs f31, 0x2f8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(760 as u32), tmp.u32 ) };
	// 82905EF8: 93E102EC  stw r31, 0x2ec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(748 as u32), ctx.r[31].u32 ) };
	// 82905EFC: 912102E4  stw r9, 0x2e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(740 as u32), ctx.r[9].u32 ) };
	// 82905F00: 38610300  addi r3, r1, 0x300
	ctx.r[3].s64 = ctx.r[1].s64 + 768;
	// 82905F04: 9BE102FC  stb r31, 0x2fc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(764 as u32), ctx.r[31].u8 ) };
	// 82905F08: 916102E0  stw r11, 0x2e0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(736 as u32), ctx.r[11].u32 ) };
	// 82905F0C: 484F89BD  bl 0x82dfe8c8
	ctx.lr = 0x82905F10;
	sub_82DFE8C8(ctx, base);
	// 82905F10: 38610308  addi r3, r1, 0x308
	ctx.r[3].s64 = ctx.r[1].s64 + 776;
	// 82905F14: 484F89B5  bl 0x82dfe8c8
	ctx.lr = 0x82905F18;
	sub_82DFE8C8(ctx, base);
	// 82905F18: 3E80832D  lis r20, -0x7cd3
	ctx.r[20].s64 = -2094202880;
	// 82905F1C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82905F20: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905F24: D0010318  stfs f0, 0x318(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(792 as u32), tmp.u32 ) };
	// 82905F28: 93E1031C  stw r31, 0x31c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(796 as u32), ctx.r[31].u32 ) };
	// 82905F2C: 394B5044  addi r10, r11, 0x5044
	ctx.r[10].s64 = ctx.r[11].s64 + 20548;
	// 82905F30: D3C10320  stfs f30, 0x320(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(800 as u32), tmp.u32 ) };
	// 82905F34: D3E10324  stfs f31, 0x324(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(804 as u32), tmp.u32 ) };
	// 82905F38: 9BE1032C  stb r31, 0x32c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(812 as u32), ctx.r[31].u8 ) };
	// 82905F3C: D3E10328  stfs f31, 0x328(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(808 as u32), tmp.u32 ) };
	// 82905F40: 91410314  stw r10, 0x314(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(788 as u32), ctx.r[10].u32 ) };
	// 82905F44: 38610330  addi r3, r1, 0x330
	ctx.r[3].s64 = ctx.r[1].s64 + 816;
	// 82905F48: 8174F468  lwz r11, -0xb98(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-2968 as u32) ) } as u64;
	// 82905F4C: 91610310  stw r11, 0x310(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(784 as u32), ctx.r[11].u32 ) };
	// 82905F50: 484F8979  bl 0x82dfe8c8
	ctx.lr = 0x82905F54;
	sub_82DFE8C8(ctx, base);
	// 82905F54: 38610338  addi r3, r1, 0x338
	ctx.r[3].s64 = ctx.r[1].s64 + 824;
	// 82905F58: 484F8971  bl 0x82dfe8c8
	ctx.lr = 0x82905F5C;
	sub_82DFE8C8(ctx, base);
	// 82905F5C: 3E60832D  lis r19, -0x7cd3
	ctx.r[19].s64 = -2094202880;
	// 82905F60: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905F64: 93C1034C  stw r30, 0x34c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(844 as u32), ctx.r[30].u32 ) };
	// 82905F68: D0010348  stfs f0, 0x348(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(840 as u32), tmp.u32 ) };
	// 82905F6C: 92410344  stw r18, 0x344(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(836 as u32), ctx.r[18].u32 ) };
	// 82905F70: D3C10350  stfs f30, 0x350(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(848 as u32), tmp.u32 ) };
	// 82905F74: 9BE1035C  stb r31, 0x35c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(860 as u32), ctx.r[31].u8 ) };
	// 82905F78: D3E10354  stfs f31, 0x354(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(852 as u32), tmp.u32 ) };
	// 82905F7C: 38610360  addi r3, r1, 0x360
	ctx.r[3].s64 = ctx.r[1].s64 + 864;
	// 82905F80: 8173F47C  lwz r11, -0xb84(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2948 as u32) ) } as u64;
	// 82905F84: D3E10358  stfs f31, 0x358(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(856 as u32), tmp.u32 ) };
	// 82905F88: 91610340  stw r11, 0x340(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(832 as u32), ctx.r[11].u32 ) };
	// 82905F8C: 484F893D  bl 0x82dfe8c8
	ctx.lr = 0x82905F90;
	sub_82DFE8C8(ctx, base);
	// 82905F90: 38610368  addi r3, r1, 0x368
	ctx.r[3].s64 = ctx.r[1].s64 + 872;
	// 82905F94: 484F8935  bl 0x82dfe8c8
	ctx.lr = 0x82905F98;
	sub_82DFE8C8(ctx, base);
	// 82905F98: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82905F9C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905FA0: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905FA4: D0010378  stfs f0, 0x378(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(888 as u32), tmp.u32 ) };
	// 82905FA8: 93C1037C  stw r30, 0x37c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(892 as u32), ctx.r[30].u32 ) };
	// 82905FAC: D3C10380  stfs f30, 0x380(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(896 as u32), tmp.u32 ) };
	// 82905FB0: 9BE1038C  stb r31, 0x38c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(908 as u32), ctx.r[31].u8 ) };
	// 82905FB4: D3E10384  stfs f31, 0x384(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(900 as u32), tmp.u32 ) };
	// 82905FB8: 38610390  addi r3, r1, 0x390
	ctx.r[3].s64 = ctx.r[1].s64 + 912;
	// 82905FBC: D3E10388  stfs f31, 0x388(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(904 as u32), tmp.u32 ) };
	// 82905FC0: 91410374  stw r10, 0x374(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(884 as u32), ctx.r[10].u32 ) };
	// 82905FC4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82905FC8: 816BF460  lwz r11, -0xba0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2976 as u32) ) } as u64;
	// 82905FCC: 91610370  stw r11, 0x370(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(880 as u32), ctx.r[11].u32 ) };
	// 82905FD0: 484F88F9  bl 0x82dfe8c8
	ctx.lr = 0x82905FD4;
	sub_82DFE8C8(ctx, base);
	// 82905FD4: 38610398  addi r3, r1, 0x398
	ctx.r[3].s64 = ctx.r[1].s64 + 920;
	// 82905FD8: 484F88F1  bl 0x82dfe8c8
	ctx.lr = 0x82905FDC;
	sub_82DFE8C8(ctx, base);
	// 82905FDC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82905FE0: 3E40832D  lis r18, -0x7cd3
	ctx.r[18].s64 = -2094202880;
	// 82905FE4: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82905FE8: D00103A8  stfs f0, 0x3a8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(936 as u32), tmp.u32 ) };
	// 82905FEC: 93C103AC  stw r30, 0x3ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(940 as u32), ctx.r[30].u32 ) };
	// 82905FF0: D3C103B0  stfs f30, 0x3b0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(944 as u32), tmp.u32 ) };
	// 82905FF4: 9BE103BC  stb r31, 0x3bc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(956 as u32), ctx.r[31].u8 ) };
	// 82905FF8: D3E103B4  stfs f31, 0x3b4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(948 as u32), tmp.u32 ) };
	// 82905FFC: 386103C0  addi r3, r1, 0x3c0
	ctx.r[3].s64 = ctx.r[1].s64 + 960;
	// 82906000: 916103A4  stw r11, 0x3a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(932 as u32), ctx.r[11].u32 ) };
	// 82906004: D3E103B8  stfs f31, 0x3b8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(952 as u32), tmp.u32 ) };
	// 82906008: 8172F480  lwz r11, -0xb80(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-2944 as u32) ) } as u64;
	// 8290600C: 916103A0  stw r11, 0x3a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(928 as u32), ctx.r[11].u32 ) };
	// 82906010: 484F88B9  bl 0x82dfe8c8
	ctx.lr = 0x82906014;
	sub_82DFE8C8(ctx, base);
	// 82906014: 386103C8  addi r3, r1, 0x3c8
	ctx.r[3].s64 = ctx.r[1].s64 + 968;
	// 82906018: 484F88B1  bl 0x82dfe8c8
	ctx.lr = 0x8290601C;
	sub_82DFE8C8(ctx, base);
	// 8290601C: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82906020: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82906024: 38A000EB  li r5, 0xeb
	ctx.r[5].s64 = 235;
	// 82906028: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8290602C: 4B9BA3AD  bl 0x822c03d8
	ctx.lr = 0x82906030;
	sub_822C03D8(ctx, base);
	// 82906030: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82906034: 4182001C  beq 0x82906050
	if ctx.cr[0].eq {
	pc = 0x82906050; continue 'dispatch;
	}
	// 82906038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290603C: 4BEE4C75  bl 0x827eacb0
	ctx.lr = 0x82906040;
	sub_827EACB0(ctx, base);
	// 82906040: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82906044: 396B09D8  addi r11, r11, 0x9d8
	ctx.r[11].s64 = ctx.r[11].s64 + 2520;
	// 82906048: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290604C: 48000008  b 0x82906054
	pc = 0x82906054; continue 'dispatch;
	// 82906050: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82906054: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82906058: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290605C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82906060: 4BF60B81  bl 0x82866be0
	ctx.lr = 0x82906064;
	sub_82866BE0(ctx, base);
	// 82906064: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82906068: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290606C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82906070: 4B9B9F91  bl 0x822c0000
	ctx.lr = 0x82906074;
	sub_822C0000(ctx, base);
	// 82906074: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82906078: 3BDD01CC  addi r30, r29, 0x1cc
	ctx.r[30].s64 = ctx.r[29].s64 + 460;
	// 8290607C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82906080: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82906084: 917D01CC  stw r11, 0x1cc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 82906088: 4B9BE3D9  bl 0x822c4460
	ctx.lr = 0x8290608C;
	sub_822C4460(ctx, base);
	// 8290608C: 817D01CC  lwz r11, 0x1cc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(460 as u32) ) } as u64;
	// 82906090: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82906094: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82906098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290609C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 829060A0: 69710001  xori r17, r11, 1
	ctx.r[17].u64 = ctx.r[11].u64 ^ 1;
	// 829060A4: 419A0008  beq cr6, 0x829060ac
	if ctx.cr[6].eq {
	pc = 0x829060AC; continue 'dispatch;
	}
	// 829060A8: 4B9BA7E9  bl 0x822c0890
	ctx.lr = 0x829060AC;
	sub_822C0890(ctx, base);
	// 829060AC: 562B063F  clrlwi. r11, r17, 0x18
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829060B0: 41820068  beq 0x82906118
	if ctx.cr[0].eq {
	pc = 0x82906118; continue 'dispatch;
	}
	// 829060B4: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 829060B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829060BC: 484ED94D  bl 0x82df3a08
	ctx.lr = 0x829060C0;
	sub_82DF3A08(ctx, base);
	// 829060C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 829060C4: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 829060C8: 3A210060  addi r17, r1, 0x60
	ctx.r[17].s64 = ctx.r[1].s64 + 96;
	// 829060CC: 81FE0000  lwz r15, 0(r30)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 829060D0: 4BC093F9  bl 0x8250f4c8
	ctx.lr = 0x829060D4;
	sub_8250F4C8(ctx, base);
	// 829060D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829060D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829060DC: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 829060E0: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 829060E4: C02B4D88  lfs f1, 0x4d88(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19848 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829060E8: 4BEE4881  bl 0x827ea968
	ctx.lr = 0x829060EC;
	sub_827EA968(ctx, base);
	// 829060EC: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 829060F0: 484EBBA1  bl 0x82df1c90
	ctx.lr = 0x829060F4;
	sub_82DF1C90(ctx, base);
	// 829060F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829060F8: 484ED331  bl 0x82df3428
	ctx.lr = 0x829060FC;
	sub_82DF3428(ctx, base);
	// 829060FC: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 82906100: 38810130  addi r4, r1, 0x130
	ctx.r[4].s64 = ctx.r[1].s64 + 304;
	// 82906104: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906108: 4BEE45B9  bl 0x827ea6c0
	ctx.lr = 0x8290610C;
	sub_827EA6C0(ctx, base);
	// 8290610C: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82906110: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906114: 4BEE45B5  bl 0x827ea6c8
	ctx.lr = 0x82906118;
	sub_827EA6C8(ctx, base);
	// 82906118: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290611C: 4BEE60B5  bl 0x827ec1d0
	ctx.lr = 0x82906120;
	sub_827EC1D0(ctx, base);
	// 82906120: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82906124: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82906128: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290612C: 41820284  beq 0x829063b0
	if ctx.cr[0].eq {
	pc = 0x829063B0; continue 'dispatch;
	}
	// 82906130: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906134: 809AD078  lwz r4, -0x2f88(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-12168 as u32) ) } as u64;
	// 82906138: 484ED8D1  bl 0x82df3a08
	ctx.lr = 0x8290613C;
	sub_82DF3A08(ctx, base);
	// 8290613C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82906140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906144: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82906148: 4BEE41F1  bl 0x827ea338
	ctx.lr = 0x8290614C;
	sub_827EA338(ctx, base);
	// 8290614C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906150: 484ED2D9  bl 0x82df3428
	ctx.lr = 0x82906154;
	sub_82DF3428(ctx, base);
	// 82906154: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906158: 8096D088  lwz r4, -0x2f78(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12152 as u32) ) } as u64;
	// 8290615C: 484ED8AD  bl 0x82df3a08
	ctx.lr = 0x82906160;
	sub_82DF3A08(ctx, base);
	// 82906160: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82906164: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906168: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290616C: 4BEE41CD  bl 0x827ea338
	ctx.lr = 0x82906170;
	sub_827EA338(ctx, base);
	// 82906170: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906174: 484ED2B5  bl 0x82df3428
	ctx.lr = 0x82906178;
	sub_82DF3428(ctx, base);
	// 82906178: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290617C: 8095D084  lwz r4, -0x2f7c(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12156 as u32) ) } as u64;
	// 82906180: 484ED889  bl 0x82df3a08
	ctx.lr = 0x82906184;
	sub_82DF3A08(ctx, base);
	// 82906184: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82906188: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290618C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82906190: 4BEE41A9  bl 0x827ea338
	ctx.lr = 0x82906194;
	sub_827EA338(ctx, base);
	// 82906194: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906198: 484ED291  bl 0x82df3428
	ctx.lr = 0x8290619C;
	sub_82DF3428(ctx, base);
	// 8290619C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829061A0: 809BD080  lwz r4, -0x2f80(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-12160 as u32) ) } as u64;
	// 829061A4: 484ED865  bl 0x82df3a08
	ctx.lr = 0x829061A8;
	sub_82DF3A08(ctx, base);
	// 829061A8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 829061AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829061B0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 829061B4: 4BEE4185  bl 0x827ea338
	ctx.lr = 0x829061B8;
	sub_827EA338(ctx, base);
	// 829061B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829061BC: 484ED26D  bl 0x82df3428
	ctx.lr = 0x829061C0;
	sub_82DF3428(ctx, base);
	// 829061C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829061C4: 8092F480  lwz r4, -0xb80(r18)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-2944 as u32) ) } as u64;
	// 829061C8: 484ED841  bl 0x82df3a08
	ctx.lr = 0x829061CC;
	sub_82DF3A08(ctx, base);
	// 829061CC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 829061D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829061D4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 829061D8: 4BEE4161  bl 0x827ea338
	ctx.lr = 0x829061DC;
	sub_827EA338(ctx, base);
	// 829061DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829061E0: 484ED249  bl 0x82df3428
	ctx.lr = 0x829061E4;
	sub_82DF3428(ctx, base);
	// 829061E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829061E8: 8094F468  lwz r4, -0xb98(r20)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-2968 as u32) ) } as u64;
	// 829061EC: 484ED81D  bl 0x82df3a08
	ctx.lr = 0x829061F0;
	sub_82DF3A08(ctx, base);
	// 829061F0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 829061F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829061F8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 829061FC: 4BEE413D  bl 0x827ea338
	ctx.lr = 0x82906200;
	sub_827EA338(ctx, base);
	// 82906200: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906204: 484ED225  bl 0x82df3428
	ctx.lr = 0x82906208;
	sub_82DF3428(ctx, base);
	// 82906208: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290620C: 8093F47C  lwz r4, -0xb84(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2948 as u32) ) } as u64;
	// 82906210: 484ED7F9  bl 0x82df3a08
	ctx.lr = 0x82906214;
	sub_82DF3A08(ctx, base);
	// 82906214: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82906218: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290621C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82906220: 4BEE4119  bl 0x827ea338
	ctx.lr = 0x82906224;
	sub_827EA338(ctx, base);
	// 82906224: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906228: 484ED201  bl 0x82df3428
	ctx.lr = 0x8290622C;
	sub_82DF3428(ctx, base);
	// 8290622C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906230: 8099F3F8  lwz r4, -0xc08(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 82906234: 484ED7D5  bl 0x82df3a08
	ctx.lr = 0x82906238;
	sub_82DF3A08(ctx, base);
	// 82906238: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8290623C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906240: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82906244: 4BEE40F5  bl 0x827ea338
	ctx.lr = 0x82906248;
	sub_827EA338(ctx, base);
	// 82906248: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290624C: 484ED1DD  bl 0x82df3428
	ctx.lr = 0x82906250;
	sub_82DF3428(ctx, base);
	// 82906250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906254: 8099F3F8  lwz r4, -0xc08(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 82906258: 484ED7B1  bl 0x82df3a08
	ctx.lr = 0x8290625C;
	sub_82DF3A08(ctx, base);
	// 8290625C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906260: 8097D074  lwz r4, -0x2f8c(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-12172 as u32) ) } as u64;
	// 82906264: 484ED7A5  bl 0x82df3a08
	ctx.lr = 0x82906268;
	sub_82DF3A08(ctx, base);
	// 82906268: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290626C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82906270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906274: 4BEE40AD  bl 0x827ea320
	ctx.lr = 0x82906278;
	sub_827EA320(ctx, base);
	// 82906278: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290627C: 484ED1AD  bl 0x82df3428
	ctx.lr = 0x82906280;
	sub_82DF3428(ctx, base);
	// 82906280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906284: 484ED1A5  bl 0x82df3428
	ctx.lr = 0x82906288;
	sub_82DF3428(ctx, base);
	// 82906288: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290628C: 809AD078  lwz r4, -0x2f88(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-12168 as u32) ) } as u64;
	// 82906290: 484ED779  bl 0x82df3a08
	ctx.lr = 0x82906294;
	sub_82DF3A08(ctx, base);
	// 82906294: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906298: 8098D07C  lwz r4, -0x2f84(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-12164 as u32) ) } as u64;
	// 8290629C: 484ED76D  bl 0x82df3a08
	ctx.lr = 0x829062A0;
	sub_82DF3A08(ctx, base);
	// 829062A0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 829062A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829062A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829062AC: 4BEE4075  bl 0x827ea320
	ctx.lr = 0x829062B0;
	sub_827EA320(ctx, base);
	// 829062B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829062B4: 484ED175  bl 0x82df3428
	ctx.lr = 0x829062B8;
	sub_82DF3428(ctx, base);
	// 829062B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829062BC: 484ED16D  bl 0x82df3428
	ctx.lr = 0x829062C0;
	sub_82DF3428(ctx, base);
	// 829062C0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 829062C4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 829062C8: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 829062CC: 396B46B8  addi r11, r11, 0x46b8
	ctx.r[11].s64 = ctx.r[11].s64 + 18104;
	// 829062D0: 388AC15C  addi r4, r10, -0x3ea4
	ctx.r[4].s64 = ctx.r[10].s64 + -16036;
	// 829062D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829062D8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 829062DC: 484ED72D  bl 0x82df3a08
	ctx.lr = 0x829062E0;
	sub_82DF3A08(ctx, base);
	// 829062E0: 3B5DFFF0  addi r26, r29, -0x10
	ctx.r[26].s64 = ctx.r[29].s64 + -16;
	// 829062E4: 386103D0  addi r3, r1, 0x3d0
	ctx.r[3].s64 = ctx.r[1].s64 + 976;
	// 829062E8: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 829062EC: 93410068  stw r26, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 829062F0: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 829062F4: 93E103D0  stw r31, 0x3d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(976 as u32), ctx.r[31].u32 ) };
	// 829062F8: 4BFFF6C9  bl 0x829059c0
	ctx.lr = 0x829062FC;
	sub_829059C0(ctx, base);
	// 829062FC: 38A103D0  addi r5, r1, 0x3d0
	ctx.r[5].s64 = ctx.r[1].s64 + 976;
	// 82906300: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82906304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906308: 4BEE4501  bl 0x827ea808
	ctx.lr = 0x8290630C;
	sub_827EA808(ctx, base);
	// 8290630C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906310: 484ED119  bl 0x82df3428
	ctx.lr = 0x82906314;
	sub_82DF3428(ctx, base);
	// 82906314: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82906318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290631C: 809BD080  lwz r4, -0x2f80(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-12160 as u32) ) } as u64;
	// 82906320: 396B4C00  addi r11, r11, 0x4c00
	ctx.r[11].s64 = ctx.r[11].s64 + 19456;
	// 82906324: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82906328: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290632C: 484ED6DD  bl 0x82df3a08
	ctx.lr = 0x82906330;
	sub_82DF3A08(ctx, base);
	// 82906330: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906334: 934103D8  stw r26, 0x3d8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(984 as u32), ctx.r[26].u32 ) };
	// 82906338: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8290633C: E8A103D8  ld r5, 0x3d8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(984 as u32) ) };
	// 82906340: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906344: 4BFFF6FD  bl 0x82905a40
	ctx.lr = 0x82906348;
	sub_82905A40(ctx, base);
	// 82906348: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290634C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82906350: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82906354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906358: C02BE848  lfs f1, -0x17b8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6072 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290635C: 4BEE4455  bl 0x827ea7b0
	ctx.lr = 0x82906360;
	sub_827EA7B0(ctx, base);
	// 82906360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906364: 484ED0C5  bl 0x82df3428
	ctx.lr = 0x82906368;
	sub_82DF3428(ctx, base);
	// 82906368: 357DFFF0  addic. r11, r29, -0x10
	ctx.xer.ca = (ctx.r[29].u32 > (!(-16 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290636C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82906370: 40820008  bne 0x82906378
	if !ctx.cr[0].eq {
	pc = 0x82906378; continue 'dispatch;
	}
	// 82906374: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82906378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290637C: 809BD080  lwz r4, -0x2f80(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-12160 as u32) ) } as u64;
	// 82906380: 484ED689  bl 0x82df3a08
	ctx.lr = 0x82906384;
	sub_82DF3A08(ctx, base);
	// 82906384: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906388: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 8290638C: 4BEE5E45  bl 0x827ec1d0
	ctx.lr = 0x82906390;
	sub_827EC1D0(ctx, base);
	// 82906390: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82906394: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906398: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8290639C: 386B0080  addi r3, r11, 0x80
	ctx.r[3].s64 = ctx.r[11].s64 + 128;
	// 829063A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 829063A4: 48051695  bl 0x82957a38
	ctx.lr = 0x829063A8;
	sub_82957A38(ctx, base);
	// 829063A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829063AC: 484ED07D  bl 0x82df3428
	ctx.lr = 0x829063B0;
	sub_82DF3428(ctx, base);
	// 829063B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829063B4: 4BEE5E1D  bl 0x827ec1d0
	ctx.lr = 0x829063B8;
	sub_827EC1D0(ctx, base);
	// 829063B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829063BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829063C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829063C4: 4E800421  bctrl
	ctx.lr = 0x829063C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829063C8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 829063CC: 418200F0  beq 0x829064bc
	if ctx.cr[0].eq {
	pc = 0x829064BC; continue 'dispatch;
	}
	// 829063D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 829063D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829063D8: 388B1EDC  addi r4, r11, 0x1edc
	ctx.r[4].s64 = ctx.r[11].s64 + 7900;
	// 829063DC: 484ED62D  bl 0x82df3a08
	ctx.lr = 0x829063E0;
	sub_82DF3A08(ctx, base);
	// 829063E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829063E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829063E8: 482A7879  bl 0x82badc60
	ctx.lr = 0x829063EC;
	sub_82BADC60(ctx, base);
	// 829063EC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 829063F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829063F4: 484ED035  bl 0x82df3428
	ctx.lr = 0x829063F8;
	sub_82DF3428(ctx, base);
	// 829063F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829063FC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82906400: 484ED609  bl 0x82df3a08
	ctx.lr = 0x82906404;
	sub_82DF3A08(ctx, base);
	// 82906404: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82906408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290640C: 482A7855  bl 0x82badc60
	ctx.lr = 0x82906410;
	sub_82BADC60(ctx, base);
	// 82906410: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82906414: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906418: 484ED011  bl 0x82df3428
	ctx.lr = 0x8290641C;
	sub_82DF3428(ctx, base);
	// 8290641C: C01C08A8  lfs f0, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82906420: D3C100E0  stfs f30, 0xe0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 82906424: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82906428: D00100E4  stfs f0, 0xe4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8290642C: 390100E0  addi r8, r1, 0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + 224;
	// 82906430: D3C100E8  stfs f30, 0xe8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 82906434: 38E100F0  addi r7, r1, 0xf0
	ctx.r[7].s64 = ctx.r[1].s64 + 240;
	// 82906438: D3C100EC  stfs f30, 0xec(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8290643C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82906440: D00100F0  stfs f0, 0xf0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 82906444: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82906448: D3C100F4  stfs f30, 0xf4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8290644C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906450: D3C100F8  stfs f30, 0xf8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 82906454: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 82906458: C02BD8B0  lfs f1, -0x2750(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10064 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290645C: 48076D95  bl 0x8297d1f0
	ctx.lr = 0x82906460;
	sub_8297D1F0(ctx, base);
	// 82906460: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82906464: 38A0010D  li r5, 0x10d
	ctx.r[5].s64 = 269;
	// 82906468: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8290646C: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82906470: 4B9B9F69  bl 0x822c03d8
	ctx.lr = 0x82906474;
	sub_822C03D8(ctx, base);
	// 82906474: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906478: 41820018  beq 0x82906490
	if ctx.cr[0].eq {
	pc = 0x82906490; continue 'dispatch;
	}
	// 8290647C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82906480: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82906484: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82906488: 48076DB1  bl 0x8297d238
	ctx.lr = 0x8290648C;
	sub_8297D238(ctx, base);
	// 8290648C: 48000008  b 0x82906494
	pc = 0x82906494; continue 'dispatch;
	// 82906490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82906494: 817D0398  lwz r11, 0x398(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(920 as u32) ) } as u64;
	// 82906498: 907D0398  stw r3, 0x398(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(920 as u32), ctx.r[3].u32 ) };
	// 8290649C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829064A0: 419A001C  beq cr6, 0x829064bc
	if ctx.cr[6].eq {
	pc = 0x829064BC; continue 'dispatch;
	}
	// 829064A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829064A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 829064AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 829064B0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 829064B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829064B8: 4E800421  bctrl
	ctx.lr = 0x829064BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829064BC: 8301053C  lwz r24, 0x53c(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1340 as u32) ) } as u64;
	// 829064C0: 83210078  lwz r25, 0x78(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 829064C4: 8221007C  lwz r17, 0x7c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 829064C8: 808E0000  lwz r4, 0(r14)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 829064CC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 829064D0: 419A039C  beq cr6, 0x8290686c
	if ctx.cr[6].eq {
	pc = 0x8290686C; continue 'dispatch;
	}
	// 829064D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829064D8: 4BEE5BD1  bl 0x827ec0a8
	ctx.lr = 0x829064DC;
	sub_827EC0A8(ctx, base);
	// 829064DC: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 829064E0: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 829064E4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829064E8: 388BED10  addi r4, r11, -0x12f0
	ctx.r[4].s64 = ctx.r[11].s64 + -4848;
	// 829064EC: 4BF62BBD  bl 0x828690a8
	ctx.lr = 0x829064F0;
	sub_828690A8(ctx, base);
	// 829064F0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829064F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829064F8: 3BDD0164  addi r30, r29, 0x164
	ctx.r[30].s64 = ctx.r[29].s64 + 356;
	// 829064FC: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 82906500: 808B0BEC  lwz r4, 0xbec(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3052 as u32) ) } as u64;
	// 82906504: 484ED505  bl 0x82df3a08
	ctx.lr = 0x82906508;
	sub_82DF3A08(ctx, base);
	// 82906508: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290650C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906510: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82906514: 48555C45  bl 0x82e5c158
	ctx.lr = 0x82906518;
	sub_82E5C158(ctx, base);
	// 82906518: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 8290651C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82906520: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906524: 388BE608  addi r4, r11, -0x19f8
	ctx.r[4].s64 = ctx.r[11].s64 + -6648;
	// 82906528: 4BF62B81  bl 0x828690a8
	ctx.lr = 0x8290652C;
	sub_828690A8(ctx, base);
	// 8290652C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82906530: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906534: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 82906538: 808B0BD8  lwz r4, 0xbd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3032 as u32) ) } as u64;
	// 8290653C: 484ED4CD  bl 0x82df3a08
	ctx.lr = 0x82906540;
	sub_82DF3A08(ctx, base);
	// 82906540: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906548: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8290654C: 48555C0D  bl 0x82e5c158
	ctx.lr = 0x82906550;
	sub_82E5C158(ctx, base);
	// 82906550: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906554: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82906558: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290655C: 388BE608  addi r4, r11, -0x19f8
	ctx.r[4].s64 = ctx.r[11].s64 + -6648;
	// 82906560: 4BF62B49  bl 0x828690a8
	ctx.lr = 0x82906564;
	sub_828690A8(ctx, base);
	// 82906564: 3F60832D  lis r27, -0x7cd3
	ctx.r[27].s64 = -2094202880;
	// 82906568: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290656C: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82906570: 809B0BF0  lwz r4, 0xbf0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(3056 as u32) ) } as u64;
	// 82906574: 484ED495  bl 0x82df3a08
	ctx.lr = 0x82906578;
	sub_82DF3A08(ctx, base);
	// 82906578: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290657C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906580: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82906584: 48555BD5  bl 0x82e5c158
	ctx.lr = 0x82906588;
	sub_82E5C158(ctx, base);
	// 82906588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290658C: 809B0BF0  lwz r4, 0xbf0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(3056 as u32) ) } as u64;
	// 82906590: 484ED479  bl 0x82df3a08
	ctx.lr = 0x82906594;
	sub_82DF3A08(ctx, base);
	// 82906594: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82906598: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290659C: 4BEE5F6D  bl 0x827ec508
	ctx.lr = 0x829065A0;
	sub_827EC508(ctx, base);
	// 829065A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829065A4: 484ECE85  bl 0x82df3428
	ctx.lr = 0x829065A8;
	sub_82DF3428(ctx, base);
	// 829065A8: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 829065AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 829065B0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829065B4: 388B8A60  addi r4, r11, -0x75a0
	ctx.r[4].s64 = ctx.r[11].s64 + -30112;
	// 829065B8: 4BF62AF1  bl 0x828690a8
	ctx.lr = 0x829065BC;
	sub_828690A8(ctx, base);
	// 829065BC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829065C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829065C4: 3BDD00FC  addi r30, r29, 0xfc
	ctx.r[30].s64 = ctx.r[29].s64 + 252;
	// 829065C8: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 829065CC: 808B0B2C  lwz r4, 0xb2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2860 as u32) ) } as u64;
	// 829065D0: 484ED439  bl 0x82df3a08
	ctx.lr = 0x829065D4;
	sub_82DF3A08(ctx, base);
	// 829065D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829065D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829065DC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 829065E0: 48555B79  bl 0x82e5c158
	ctx.lr = 0x829065E4;
	sub_82E5C158(ctx, base);
	// 829065E4: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 829065E8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 829065EC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829065F0: 388B8A60  addi r4, r11, -0x75a0
	ctx.r[4].s64 = ctx.r[11].s64 + -30112;
	// 829065F4: 4BF62AB5  bl 0x828690a8
	ctx.lr = 0x829065F8;
	sub_828690A8(ctx, base);
	// 829065F8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829065FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906600: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 82906604: 808B0B28  lwz r4, 0xb28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2856 as u32) ) } as u64;
	// 82906608: 484ED401  bl 0x82df3a08
	ctx.lr = 0x8290660C;
	sub_82DF3A08(ctx, base);
	// 8290660C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906614: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82906618: 48555B41  bl 0x82e5c158
	ctx.lr = 0x8290661C;
	sub_82E5C158(ctx, base);
	// 8290661C: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906620: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 82906624: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906628: 388B8C48  addi r4, r11, -0x73b8
	ctx.r[4].s64 = ctx.r[11].s64 + -29624;
	// 8290662C: 4BFFF125  bl 0x82905750
	ctx.lr = 0x82906630;
	sub_82905750(ctx, base);
	// 82906630: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82906634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906638: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 8290663C: 808B0B58  lwz r4, 0xb58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2904 as u32) ) } as u64;
	// 82906640: 484ED3C9  bl 0x82df3a08
	ctx.lr = 0x82906644;
	sub_82DF3A08(ctx, base);
	// 82906644: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290664C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82906650: 48555B09  bl 0x82e5c158
	ctx.lr = 0x82906654;
	sub_82E5C158(ctx, base);
	// 82906654: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906658: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8290665C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906660: 388B9EF8  addi r4, r11, -0x6108
	ctx.r[4].s64 = ctx.r[11].s64 + -24840;
	// 82906664: 4BF62A45  bl 0x828690a8
	ctx.lr = 0x82906668;
	sub_828690A8(ctx, base);
	// 82906668: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290666C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906670: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 82906674: 808B0B64  lwz r4, 0xb64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2916 as u32) ) } as u64;
	// 82906678: 484ED391  bl 0x82df3a08
	ctx.lr = 0x8290667C;
	sub_82DF3A08(ctx, base);
	// 8290667C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906680: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906684: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82906688: 48555AD1  bl 0x82e5c158
	ctx.lr = 0x8290668C;
	sub_82E5C158(ctx, base);
	// 8290668C: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906690: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82906694: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906698: 388B4700  addi r4, r11, 0x4700
	ctx.r[4].s64 = ctx.r[11].s64 + 18176;
	// 8290669C: 4BF62A0D  bl 0x828690a8
	ctx.lr = 0x829066A0;
	sub_828690A8(ctx, base);
	// 829066A0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829066A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829066A8: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 829066AC: 808B0B68  lwz r4, 0xb68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2920 as u32) ) } as u64;
	// 829066B0: 484ED359  bl 0x82df3a08
	ctx.lr = 0x829066B4;
	sub_82DF3A08(ctx, base);
	// 829066B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829066B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829066BC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 829066C0: 48555A99  bl 0x82e5c158
	ctx.lr = 0x829066C4;
	sub_82E5C158(ctx, base);
	// 829066C4: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 829066C8: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 829066CC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829066D0: 388B8CB8  addi r4, r11, -0x7348
	ctx.r[4].s64 = ctx.r[11].s64 + -29512;
	// 829066D4: 4BFFF07D  bl 0x82905750
	ctx.lr = 0x829066D8;
	sub_82905750(ctx, base);
	// 829066D8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829066DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829066E0: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 829066E4: 808B0AF8  lwz r4, 0xaf8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2808 as u32) ) } as u64;
	// 829066E8: 484ED321  bl 0x82df3a08
	ctx.lr = 0x829066EC;
	sub_82DF3A08(ctx, base);
	// 829066EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829066F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829066F4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 829066F8: 48555A61  bl 0x82e5c158
	ctx.lr = 0x829066FC;
	sub_82E5C158(ctx, base);
	// 829066FC: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906700: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 82906704: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906708: 388B8CB8  addi r4, r11, -0x7348
	ctx.r[4].s64 = ctx.r[11].s64 + -29512;
	// 8290670C: 4BFFF045  bl 0x82905750
	ctx.lr = 0x82906710;
	sub_82905750(ctx, base);
	// 82906710: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82906714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906718: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 8290671C: 808B0B24  lwz r4, 0xb24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) } as u64;
	// 82906720: 484ED2E9  bl 0x82df3a08
	ctx.lr = 0x82906724;
	sub_82DF3A08(ctx, base);
	// 82906724: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906728: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290672C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82906730: 48555A29  bl 0x82e5c158
	ctx.lr = 0x82906734;
	sub_82E5C158(ctx, base);
	// 82906734: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906738: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 8290673C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906740: 388B8B68  addi r4, r11, -0x7498
	ctx.r[4].s64 = ctx.r[11].s64 + -29848;
	// 82906744: 4BFFF00D  bl 0x82905750
	ctx.lr = 0x82906748;
	sub_82905750(ctx, base);
	// 82906748: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290674C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906750: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 82906754: 808B0B34  lwz r4, 0xb34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2868 as u32) ) } as u64;
	// 82906758: 484ED2B1  bl 0x82df3a08
	ctx.lr = 0x8290675C;
	sub_82DF3A08(ctx, base);
	// 8290675C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906764: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82906768: 485559F1  bl 0x82e5c158
	ctx.lr = 0x8290676C;
	sub_82E5C158(ctx, base);
	// 8290676C: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906770: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 82906774: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906778: 388B8BD8  addi r4, r11, -0x7428
	ctx.r[4].s64 = ctx.r[11].s64 + -29736;
	// 8290677C: 4BFFEFD5  bl 0x82905750
	ctx.lr = 0x82906780;
	sub_82905750(ctx, base);
	// 82906780: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82906784: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906788: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 8290678C: 808B0B70  lwz r4, 0xb70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2928 as u32) ) } as u64;
	// 82906790: 484ED279  bl 0x82df3a08
	ctx.lr = 0x82906794;
	sub_82DF3A08(ctx, base);
	// 82906794: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290679C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 829067A0: 485559B9  bl 0x82e5c158
	ctx.lr = 0x829067A4;
	sub_82E5C158(ctx, base);
	// 829067A4: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 829067A8: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 829067AC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829067B0: 388B8A18  addi r4, r11, -0x75e8
	ctx.r[4].s64 = ctx.r[11].s64 + -30184;
	// 829067B4: 4BFFEF9D  bl 0x82905750
	ctx.lr = 0x829067B8;
	sub_82905750(ctx, base);
	// 829067B8: 3F60832D  lis r27, -0x7cd3
	ctx.r[27].s64 = -2094202880;
	// 829067BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829067C0: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 829067C4: 809BD0A8  lwz r4, -0x2f58(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-12120 as u32) ) } as u64;
	// 829067C8: 484ED241  bl 0x82df3a08
	ctx.lr = 0x829067CC;
	sub_82DF3A08(ctx, base);
	// 829067CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829067D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829067D4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 829067D8: 48555981  bl 0x82e5c158
	ctx.lr = 0x829067DC;
	sub_82E5C158(ctx, base);
	// 829067DC: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 829067E0: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 829067E4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829067E8: 388B8A88  addi r4, r11, -0x7578
	ctx.r[4].s64 = ctx.r[11].s64 + -30072;
	// 829067EC: 4BFFEF65  bl 0x82905750
	ctx.lr = 0x829067F0;
	sub_82905750(ctx, base);
	// 829067F0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829067F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829067F8: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 829067FC: 808BD0AC  lwz r4, -0x2f54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12116 as u32) ) } as u64;
	// 82906800: 484ED209  bl 0x82df3a08
	ctx.lr = 0x82906804;
	sub_82DF3A08(ctx, base);
	// 82906804: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290680C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82906810: 48555949  bl 0x82e5c158
	ctx.lr = 0x82906814;
	sub_82E5C158(ctx, base);
	// 82906814: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82906818: 3D608291  lis r11, -0x7d6f
	ctx.r[11].s64 = -2104426496;
	// 8290681C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82906820: 388B8AF8  addi r4, r11, -0x7508
	ctx.r[4].s64 = ctx.r[11].s64 + -29960;
	// 82906824: 4BFFEF2D  bl 0x82905750
	ctx.lr = 0x82906828;
	sub_82905750(ctx, base);
	// 82906828: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290682C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906830: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82906834: 808BD0B0  lwz r4, -0x2f50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12112 as u32) ) } as u64;
	// 82906838: 484ED1D1  bl 0x82df3a08
	ctx.lr = 0x8290683C;
	sub_82DF3A08(ctx, base);
	// 8290683C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906840: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906844: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82906848: 48555911  bl 0x82e5c158
	ctx.lr = 0x8290684C;
	sub_82E5C158(ctx, base);
	// 8290684C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906850: 809BD0A8  lwz r4, -0x2f58(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-12120 as u32) ) } as u64;
	// 82906854: 484ED1B5  bl 0x82df3a08
	ctx.lr = 0x82906858;
	sub_82DF3A08(ctx, base);
	// 82906858: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290685C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906860: 4BEE6489  bl 0x827ecce8
	ctx.lr = 0x82906864;
	sub_827ECCE8(ctx, base);
	// 82906864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82906868: 484ECBC1  bl 0x82df3428
	ctx.lr = 0x8290686C;
	sub_82DF3428(ctx, base);
	// 8290686C: 83C10070  lwz r30, 0x70(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82906870: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82906874: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82906878: 484ED191  bl 0x82df3a08
	ctx.lr = 0x8290687C;
	sub_82DF3A08(ctx, base);
	// 8290687C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82906880: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 82906884: 4BEE5D45  bl 0x827ec5c8
	ctx.lr = 0x82906888;
	sub_827EC5C8(ctx, base);
	// 82906888: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290688C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82906890: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82906894: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906898: 4850E799  bl 0x82e15030
	ctx.lr = 0x8290689C;
	sub_82E15030(ctx, base);
	// 8290689C: 8061010C  lwz r3, 0x10c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(268 as u32) ) } as u64;
	// 829068A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829068A4: 419A0008  beq cr6, 0x829068ac
	if ctx.cr[6].eq {
	pc = 0x829068AC; continue 'dispatch;
	}
	// 829068A8: 4B9B9FE9  bl 0x822c0890
	ctx.lr = 0x829068AC;
	sub_822C0890(ctx, base);
	// 829068AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 829068B0: 484ECB79  bl 0x82df3428
	ctx.lr = 0x829068B4;
	sub_82DF3428(ctx, base);
	// 829068B4: 816100D0  lwz r11, 0xd0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 829068B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829068BC: 419A0080  beq cr6, 0x8290693c
	if ctx.cr[6].eq {
	pc = 0x8290693C; continue 'dispatch;
	}
	// 829068C0: 386103D0  addi r3, r1, 0x3d0
	ctx.r[3].s64 = ctx.r[1].s64 + 976;
	// 829068C4: FC60E090  fmr f3, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[28].f64;
	// 829068C8: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 829068CC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 829068D0: 48575749  bl 0x82e7c018
	ctx.lr = 0x829068D4;
	sub_82E7C018(ctx, base);
	// 829068D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 829068D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 829068DC: 38610410  addi r3, r1, 0x410
	ctx.r[3].s64 = ctx.r[1].s64 + 1040;
	// 829068E0: C02B95A0  lfs f1, -0x6a60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27232 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829068E4: 48576105  bl 0x82e7c9e8
	ctx.lr = 0x829068E8;
	sub_82E7C9E8(ctx, base);
	// 829068E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829068EC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829068F0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 829068F4: 4B9BE00D  bl 0x822c4900
	ctx.lr = 0x829068F8;
	sub_822C4900(ctx, base);
	// 829068F8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 829068FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82906900: 38A00134  li r5, 0x134
	ctx.r[5].s64 = 308;
	// 82906904: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82906908: 484EBAE1  bl 0x82df23e8
	ctx.lr = 0x8290690C;
	sub_82DF23E8(ctx, base);
	// 8290690C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906910: 41820014  beq 0x82906924
	if ctx.cr[0].eq {
	pc = 0x82906924; continue 'dispatch;
	}
	// 82906914: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82906918: 4850C7D9  bl 0x82e130f0
	ctx.lr = 0x8290691C;
	sub_82E130F0(ctx, base);
	// 8290691C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906920: 48000008  b 0x82906928
	pc = 0x82906928; continue 'dispatch;
	// 82906924: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82906928: 387D0380  addi r3, r29, 0x380
	ctx.r[3].s64 = ctx.r[29].s64 + 896;
	// 8290692C: 4B9DB425  bl 0x822e1d50
	ctx.lr = 0x82906930;
	sub_822E1D50(ctx, base);
	// 82906930: 806100D0  lwz r3, 0xd0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82906934: 809D0380  lwz r4, 0x380(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(896 as u32) ) } as u64;
	// 82906938: 4850C4F9  bl 0x82e12e30
	ctx.lr = 0x8290693C;
	sub_82E12E30(ctx, base);
	// 8290693C: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82906940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906944: 419A0008  beq cr6, 0x8290694c
	if ctx.cr[6].eq {
	pc = 0x8290694C; continue 'dispatch;
	}
	// 82906948: 4B9B9F49  bl 0x822c0890
	ctx.lr = 0x8290694C;
	sub_822C0890(ctx, base);
	// 8290694C: 807D0380  lwz r3, 0x380(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(896 as u32) ) } as u64;
	// 82906950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906954: 419A0008  beq cr6, 0x8290695c
	if ctx.cr[6].eq {
	pc = 0x8290695C; continue 'dispatch;
	}
	// 82906958: 4850BDB9  bl 0x82e12710
	ctx.lr = 0x8290695C;
	sub_82E12710(ctx, base);
	// 8290695C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82906960: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82906964: 484ED0A5  bl 0x82df3a08
	ctx.lr = 0x82906968;
	sub_82DF3A08(ctx, base);
	// 82906968: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290696C: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 82906970: 4BEE5C59  bl 0x827ec5c8
	ctx.lr = 0x82906974;
	sub_827EC5C8(ctx, base);
	// 82906974: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82906978: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8290697C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82906980: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906984: 4850E6AD  bl 0x82e15030
	ctx.lr = 0x82906988;
	sub_82E15030(ctx, base);
	// 82906988: 8061011C  lwz r3, 0x11c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 8290698C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906990: 419A0008  beq cr6, 0x82906998
	if ctx.cr[6].eq {
	pc = 0x82906998; continue 'dispatch;
	}
	// 82906994: 4B9B9EFD  bl 0x822c0890
	ctx.lr = 0x82906998;
	sub_822C0890(ctx, base);
	// 82906998: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290699C: 484ECA8D  bl 0x82df3428
	ctx.lr = 0x829069A0;
	sub_82DF3428(ctx, base);
	// 829069A0: 816100D8  lwz r11, 0xd8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 829069A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829069A8: 419A0060  beq cr6, 0x82906a08
	if ctx.cr[6].eq {
	pc = 0x82906A08; continue 'dispatch;
	}
	// 829069AC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 829069B0: FC60F090  fmr f3, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[30].f64;
	// 829069B4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 829069B8: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 829069BC: C02B6150  lfs f1, 0x6150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829069C0: 48575659  bl 0x82e7c018
	ctx.lr = 0x829069C4;
	sub_82E7C018(ctx, base);
	// 829069C4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 829069C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 829069CC: 38A00140  li r5, 0x140
	ctx.r[5].s64 = 320;
	// 829069D0: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 829069D4: 484EBA15  bl 0x82df23e8
	ctx.lr = 0x829069D8;
	sub_82DF23E8(ctx, base);
	// 829069D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829069DC: 41820014  beq 0x829069f0
	if ctx.cr[0].eq {
	pc = 0x829069F0; continue 'dispatch;
	}
	// 829069E0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 829069E4: 4850C70D  bl 0x82e130f0
	ctx.lr = 0x829069E8;
	sub_82E130F0(ctx, base);
	// 829069E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829069EC: 48000008  b 0x829069f4
	pc = 0x829069F4; continue 'dispatch;
	// 829069F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829069F4: 387D0388  addi r3, r29, 0x388
	ctx.r[3].s64 = ctx.r[29].s64 + 904;
	// 829069F8: 4B9DB359  bl 0x822e1d50
	ctx.lr = 0x829069FC;
	sub_822E1D50(ctx, base);
	// 829069FC: 806100D8  lwz r3, 0xd8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 82906A00: 809D0388  lwz r4, 0x388(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(904 as u32) ) } as u64;
	// 82906A04: 4850C42D  bl 0x82e12e30
	ctx.lr = 0x82906A08;
	sub_82E12E30(ctx, base);
	// 82906A08: 806100DC  lwz r3, 0xdc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 82906A0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906A10: 419A0008  beq cr6, 0x82906a18
	if ctx.cr[6].eq {
	pc = 0x82906A18; continue 'dispatch;
	}
	// 82906A14: 4B9B9E7D  bl 0x822c0890
	ctx.lr = 0x82906A18;
	sub_822C0890(ctx, base);
	// 82906A18: 837D038C  lwz r27, 0x38c(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(908 as u32) ) } as u64;
	// 82906A1C: 3BDD0388  addi r30, r29, 0x388
	ctx.r[30].s64 = ctx.r[29].s64 + 904;
	// 82906A20: 817D0388  lwz r11, 0x388(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(904 as u32) ) } as u64;
	// 82906A24: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82906A28: 93610074  stw r27, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 82906A2C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82906A30: 419A0024  beq cr6, 0x82906a54
	if ctx.cr[6].eq {
	pc = 0x82906A54; continue 'dispatch;
	}
	// 82906A34: 395B0004  addi r10, r27, 4
	ctx.r[10].s64 = ctx.r[27].s64 + 4;
	// 82906A38: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82906A3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906A40: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82906A44: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82906A48: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82906A4C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906A50: 4082FFE8  bne 0x82906a38
	if !ctx.cr[0].eq {
	pc = 0x82906A38; continue 'dispatch;
	}
	// 82906A54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82906A58: 419A0010  beq cr6, 0x82906a68
	if ctx.cr[6].eq {
	pc = 0x82906A68; continue 'dispatch;
	}
	// 82906A5C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82906A60: 387DFFF0  addi r3, r29, -0x10
	ctx.r[3].s64 = ctx.r[29].s64 + -16;
	// 82906A64: 4BFFE72D  bl 0x82905190
	ctx.lr = 0x82906A68;
	sub_82905190(ctx, base);
	// 82906A68: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82906A6C: 419A000C  beq cr6, 0x82906a78
	if ctx.cr[6].eq {
	pc = 0x82906A78; continue 'dispatch;
	}
	// 82906A70: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82906A74: 4B9B9E1D  bl 0x822c0890
	ctx.lr = 0x82906A78;
	sub_822C0890(ctx, base);
	// 82906A78: 835E0004  lwz r26, 4(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82906A7C: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906A80: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82906A84: 419A0024  beq cr6, 0x82906aa8
	if ctx.cr[6].eq {
	pc = 0x82906AA8; continue 'dispatch;
	}
	// 82906A88: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82906A8C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82906A90: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906A94: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82906A98: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82906A9C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82906AA0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906AA4: 4082FFE8  bne 0x82906a8c
	if !ctx.cr[0].eq {
	pc = 0x82906A8C; continue 'dispatch;
	}
	// 82906AA8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82906AAC: 419A00DC  beq cr6, 0x82906b88
	if ctx.cr[6].eq {
	pc = 0x82906B88; continue 'dispatch;
	}
	// 82906AB0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906AB4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82906AB8: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82906ABC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82906AC0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82906AC4: 48599C6D  bl 0x82ea0730
	ctx.lr = 0x82906AC8;
	sub_82EA0730(ctx, base);
	// 82906AC8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82906ACC: C03C08A8  lfs f1, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82906AD0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82906AD4: 4861371D  bl 0x82f1a1f0
	ctx.lr = 0x82906AD8;
	sub_82F1A1F0(ctx, base);
	// 82906AD8: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82906ADC: 936100C0  stw r27, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[27].u32 ) };
	// 82906AE0: 4182000C  beq 0x82906aec
	if ctx.cr[0].eq {
	pc = 0x82906AEC; continue 'dispatch;
	}
	// 82906AE4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82906AE8: 4B9E1761  bl 0x822e8248
	ctx.lr = 0x82906AEC;
	sub_822E8248(ctx, base);
	// 82906AEC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 82906AF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82906AF4: 388B0988  addi r4, r11, 0x988
	ctx.r[4].s64 = ctx.r[11].s64 + 2440;
	// 82906AF8: 484F2A39  bl 0x82df9530
	ctx.lr = 0x82906AFC;
	sub_82DF9530(ctx, base);
	// 82906AFC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82906B00: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82906B04: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82906B08: 419A0024  beq cr6, 0x82906b2c
	if ctx.cr[6].eq {
	pc = 0x82906B2C; continue 'dispatch;
	}
	// 82906B0C: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82906B10: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82906B14: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906B18: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82906B1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82906B20: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82906B24: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906B28: 4082FFE8  bne 0x82906b10
	if !ctx.cr[0].eq {
	pc = 0x82906B10; continue 'dispatch;
	}
	// 82906B2C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82906B30: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82906B34: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82906B38: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82906B3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906B40: 80CB6820  lwz r6, 0x6820(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26656 as u32) ) } as u64;
	// 82906B44: 4BC0C5B5  bl 0x825130f8
	ctx.lr = 0x82906B48;
	sub_825130F8(ctx, base);
	// 82906B48: 816100C0  lwz r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 82906B4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82906B50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82906B54: 419A000C  beq cr6, 0x82906b60
	if ctx.cr[6].eq {
	pc = 0x82906B60; continue 'dispatch;
	}
	// 82906B58: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82906B5C: 4B9E170D  bl 0x822e8268
	ctx.lr = 0x82906B60;
	sub_822E8268(ctx, base);
	// 82906B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82906B64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906B68: 48002349  bl 0x82908eb0
	ctx.lr = 0x82906B6C;
	sub_82908EB0(ctx, base);
	// 82906B6C: 38802012  li r4, 0x2012
	ctx.r[4].s64 = 8210;
	// 82906B70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82906B74: 4BB8863D  bl 0x8248f1b0
	ctx.lr = 0x82906B78;
	sub_8248F1B0(ctx, base);
	// 82906B78: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82906B7C: 419A000C  beq cr6, 0x82906b88
	if ctx.cr[6].eq {
	pc = 0x82906B88; continue 'dispatch;
	}
	// 82906B80: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82906B84: 4B9E16E5  bl 0x822e8268
	ctx.lr = 0x82906B88;
	sub_822E8268(ctx, base);
	// 82906B88: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82906B8C: 419A000C  beq cr6, 0x82906b98
	if ctx.cr[6].eq {
	pc = 0x82906B98; continue 'dispatch;
	}
	// 82906B90: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82906B94: 4B9B9CFD  bl 0x822c0890
	ctx.lr = 0x82906B98;
	sub_822C0890(ctx, base);
	// 82906B98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906B9C: C0390000  lfs f1, 0(r25)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82906BA0: 480078B1  bl 0x8290e450
	ctx.lr = 0x82906BA4;
	sub_8290E450(ctx, base);
	// 82906BA4: C0BC08A8  lfs f5, 0x8a8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2216 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82906BA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906BAC: C09900A4  lfs f4, 0xa4(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82906BB0: C07900A0  lfs f3, 0xa0(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(160 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82906BB4: FC202890  fmr f1, f5
	ctx.f[1].f64 = ctx.f[5].f64;
	// 82906BB8: C059009C  lfs f2, 0x9c(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(156 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82906BBC: 480096ED  bl 0x829102a8
	ctx.lr = 0x82906BC0;
	sub_829102A8(ctx, base);
	// 82906BC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906BC4: 80810534  lwz r4, 0x534(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1332 as u32) ) } as u64;
	// 82906BC8: 48007439  bl 0x8290e000
	ctx.lr = 0x82906BCC;
	sub_8290E000(ctx, base);
	// 82906BCC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82906BD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82906BD4: 38A00163  li r5, 0x163
	ctx.r[5].s64 = 355;
	// 82906BD8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82906BDC: 4B9B97FD  bl 0x822c03d8
	ctx.lr = 0x82906BE0;
	sub_822C03D8(ctx, base);
	// 82906BE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906BE4: 41820020  beq 0x82906c04
	if ctx.cr[0].eq {
	pc = 0x82906C04; continue 'dispatch;
	}
	// 82906BE8: 357DFFF0  addic. r11, r29, -0x10
	ctx.xer.ca = (ctx.r[29].u32 > (!(-16 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82906BEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82906BF0: 40820008  bne 0x82906bf8
	if !ctx.cr[0].eq {
	pc = 0x82906BF8; continue 'dispatch;
	}
	// 82906BF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82906BF8: 4803CFA1  bl 0x82943b98
	ctx.lr = 0x82906BFC;
	sub_82943B98(ctx, base);
	// 82906BFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82906C00: 48000008  b 0x82906c08
	pc = 0x82906C08; continue 'dispatch;
	// 82906C04: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82906C08: 93C100C8  stw r30, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 82906C0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82906C10: 386100CC  addi r3, r1, 0xcc
	ctx.r[3].s64 = ctx.r[1].s64 + 204;
	// 82906C14: 4BF5FD75  bl 0x82866988
	ctx.lr = 0x82906C18;
	sub_82866988(ctx, base);
	// 82906C18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82906C1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82906C20: 386100CC  addi r3, r1, 0xcc
	ctx.r[3].s64 = ctx.r[1].s64 + 204;
	// 82906C24: 4B9B93DD  bl 0x822c0000
	ctx.lr = 0x82906C28;
	sub_822C0000(ctx, base);
	// 82906C28: 388100C8  addi r4, r1, 0xc8
	ctx.r[4].s64 = ctx.r[1].s64 + 200;
	// 82906C2C: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 82906C30: 4BF617A1  bl 0x828683d0
	ctx.lr = 0x82906C34;
	sub_828683D0(ctx, base);
	// 82906C34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906C38: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82906C3C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82906C40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82906C44: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82906C48: 419A0024  beq cr6, 0x82906c6c
	if ctx.cr[6].eq {
	pc = 0x82906C6C; continue 'dispatch;
	}
	// 82906C4C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82906C50: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82906C54: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906C58: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82906C5C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82906C60: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82906C64: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906C68: 4082FFE8  bne 0x82906c50
	if !ctx.cr[0].eq {
	pc = 0x82906C50; continue 'dispatch;
	}
	// 82906C6C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82906C70: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82906C74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906C78: 48004871  bl 0x8290b4e8
	ctx.lr = 0x82906C7C;
	sub_8290B4E8(ctx, base);
	// 82906C7C: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82906C80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906C84: 419A0008  beq cr6, 0x82906c8c
	if ctx.cr[6].eq {
	pc = 0x82906C8C; continue 'dispatch;
	}
	// 82906C88: 4B9B9C09  bl 0x822c0890
	ctx.lr = 0x82906C8C;
	sub_822C0890(ctx, base);
	// 82906C8C: 806100CC  lwz r3, 0xcc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 82906C90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906C94: 419A0008  beq cr6, 0x82906c9c
	if ctx.cr[6].eq {
	pc = 0x82906C9C; continue 'dispatch;
	}
	// 82906C98: 4B9B9BF9  bl 0x822c0890
	ctx.lr = 0x82906C9C;
	sub_822C0890(ctx, base);
	// 82906C9C: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82906CA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82906CA4: 38A00169  li r5, 0x169
	ctx.r[5].s64 = 361;
	// 82906CA8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82906CAC: 4B9B972D  bl 0x822c03d8
	ctx.lr = 0x82906CB0;
	sub_822C03D8(ctx, base);
	// 82906CB0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82906CB4: 4182002C  beq 0x82906ce0
	if ctx.cr[0].eq {
	pc = 0x82906CE0; continue 'dispatch;
	}
	// 82906CB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82906CBC: 4BEE54A5  bl 0x827ec160
	ctx.lr = 0x82906CC0;
	sub_827EC160(ctx, base);
	// 82906CC0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82906CC4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82906CC8: 38AB4D54  addi r5, r11, 0x4d54
	ctx.r[5].s64 = ctx.r[11].s64 + 19796;
	// 82906CCC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82906CD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906CD4: 4801BAED  bl 0x829227c0
	ctx.lr = 0x82906CD8;
	sub_829227C0(ctx, base);
	// 82906CD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82906CDC: 48000008  b 0x82906ce4
	pc = 0x82906CE4; continue 'dispatch;
	// 82906CE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82906CE4: 387D0390  addi r3, r29, 0x390
	ctx.r[3].s64 = ctx.r[29].s64 + 912;
	// 82906CE8: 4BF622D1  bl 0x82868fb8
	ctx.lr = 0x82906CEC;
	sub_82868FB8(ctx, base);
	// 82906CEC: 38210510  addi r1, r1, 0x510
	ctx.r[1].s64 = ctx.r[1].s64 + 1296;
	// 82906CF0: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82906CF4: 488A1DD1  bl 0x831a8ac4
	ctx.lr = 0x82906CF8;
	sub_831A8A8C(ctx, base);
	// 82906CF8: 488A1488  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82906D00 size=60
    let mut pc: u32 = 0x82906D00;
    'dispatch: loop {
        match pc {
            0x82906D00 => {
    //   block [0x82906D00..0x82906D3C)
	// 82906D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82906D0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906D10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82906D14: 480101C5  bl 0x82916ed8
	ctx.lr = 0x82906D18;
	sub_82916ED8(ctx, base);
	// 82906D18: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82906D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82906D20: 396B50CC  addi r11, r11, 0x50cc
	ctx.r[11].s64 = ctx.r[11].s64 + 20684;
	// 82906D24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82906D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82906D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82906D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82906D40 size=60
    let mut pc: u32 = 0x82906D40;
    'dispatch: loop {
        match pc {
            0x82906D40 => {
    //   block [0x82906D40..0x82906D7C)
	// 82906D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906D4C: 4BEE6ACD  bl 0x827ed818
	ctx.lr = 0x82906D50;
	sub_827ED818(ctx, base);
	// 82906D50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906D54: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82906D58: 40820008  bne 0x82906d60
	if !ctx.cr[0].eq {
	pc = 0x82906D60; continue 'dispatch;
	}
	// 82906D5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82906D60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906D64: 419A0008  beq cr6, 0x82906d6c
	if ctx.cr[6].eq {
	pc = 0x82906D6C; continue 'dispatch;
	}
	// 82906D68: 4BFFDD59  bl 0x82904ac0
	ctx.lr = 0x82906D6C;
	sub_82904AC0(ctx, base);
	// 82906D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82906D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82906D80 size=60
    let mut pc: u32 = 0x82906D80;
    'dispatch: loop {
        match pc {
            0x82906D80 => {
    //   block [0x82906D80..0x82906DBC)
	// 82906D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906D8C: 4BEE6A8D  bl 0x827ed818
	ctx.lr = 0x82906D90;
	sub_827ED818(ctx, base);
	// 82906D90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906D94: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82906D98: 40820008  bne 0x82906da0
	if !ctx.cr[0].eq {
	pc = 0x82906DA0; continue 'dispatch;
	}
	// 82906D9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82906DA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906DA4: 419A0008  beq cr6, 0x82906dac
	if ctx.cr[6].eq {
	pc = 0x82906DAC; continue 'dispatch;
	}
	// 82906DA8: 4BFFDA09  bl 0x829047b0
	ctx.lr = 0x82906DAC;
	sub_829047B0(ctx, base);
	// 82906DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82906DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82906DC0 size=60
    let mut pc: u32 = 0x82906DC0;
    'dispatch: loop {
        match pc {
            0x82906DC0 => {
    //   block [0x82906DC0..0x82906DFC)
	// 82906DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906DCC: 4BEE6A4D  bl 0x827ed818
	ctx.lr = 0x82906DD0;
	sub_827ED818(ctx, base);
	// 82906DD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906DD4: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82906DD8: 40820008  bne 0x82906de0
	if !ctx.cr[0].eq {
	pc = 0x82906DE0; continue 'dispatch;
	}
	// 82906DDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82906DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906DE4: 419A0008  beq cr6, 0x82906dec
	if ctx.cr[6].eq {
	pc = 0x82906DEC; continue 'dispatch;
	}
	// 82906DE8: 4BFFD5C1  bl 0x829043a8
	ctx.lr = 0x82906DEC;
	sub_829043A8(ctx, base);
	// 82906DEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82906DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82906E00 size=72
    let mut pc: u32 = 0x82906E00;
    'dispatch: loop {
        match pc {
            0x82906E00 => {
    //   block [0x82906E00..0x82906E48)
	// 82906E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906E0C: 4BEE6A0D  bl 0x827ed818
	ctx.lr = 0x82906E10;
	sub_827ED818(ctx, base);
	// 82906E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82906E14: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82906E18: 40820008  bne 0x82906e20
	if !ctx.cr[0].eq {
	pc = 0x82906E20; continue 'dispatch;
	}
	// 82906E1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82906E20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82906E24: 419A000C  beq cr6, 0x82906e30
	if ctx.cr[6].eq {
	pc = 0x82906E30; continue 'dispatch;
	}
	// 82906E28: 4BFFD291  bl 0x829040b8
	ctx.lr = 0x82906E2C;
	sub_829040B8(ctx, base);
	// 82906E2C: 4800000C  b 0x82906e38
	pc = 0x82906E38; continue 'dispatch;
	// 82906E30: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82906E34: 386BB0A0  addi r3, r11, -0x4f60
	ctx.r[3].s64 = ctx.r[11].s64 + -20320;
	// 82906E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82906E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82906E48 size=88
    let mut pc: u32 = 0x82906E48;
    'dispatch: loop {
        match pc {
            0x82906E48 => {
    //   block [0x82906E48..0x82906EA0)
	// 82906E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82906E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82906E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82906E60: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82906E64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82906E68: 396B50CC  addi r11, r11, 0x50cc
	ctx.r[11].s64 = ctx.r[11].s64 + 20684;
	// 82906E6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82906E70: 4800FCF9  bl 0x82916b68
	ctx.lr = 0x82906E74;
	sub_82916B68(ctx, base);
	// 82906E74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82906E78: 4182000C  beq 0x82906e84
	if ctx.cr[0].eq {
	pc = 0x82906E84; continue 'dispatch;
	}
	// 82906E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82906E80: 4B9B93E9  bl 0x822c0268
	ctx.lr = 0x82906E84;
	sub_822C0268(ctx, base);
	// 82906E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82906E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82906E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906E94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82906E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82906E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82906EA0 size=168
    let mut pc: u32 = 0x82906EA0;
    'dispatch: loop {
        match pc {
            0x82906EA0 => {
    //   block [0x82906EA0..0x82906F48)
	// 82906EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82906EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82906EAC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82906EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82906EB8: 48053059  bl 0x82959f10
	ctx.lr = 0x82906EBC;
	sub_82959F10(ctx, base);
	// 82906EBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82906EC0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82906EC4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82906EC8: C00B959C  lfs f0, -0x6a64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82906ECC: C3EA89AC  lfs f31, -0x7654(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82906ED0: D01F0078  stfs f0, 0x78(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82906ED4: D3FF007C  stfs f31, 0x7c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82906ED8: 4804F761  bl 0x82956638
	ctx.lr = 0x82906EDC;
	sub_82956638(ctx, base);
	// 82906EDC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82906EE0: D3FF009C  stfs f31, 0x9c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82906EE4: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 82906EE8: C00B6154  lfs f0, 0x6154(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24916 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82906EEC: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82906EF0: D01F00A4  stfs f0, 0xa4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82906EF4: 48050D05  bl 0x82957bf8
	ctx.lr = 0x82906EF8;
	sub_82957BF8(ctx, base);
	// 82906EF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82906EFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82906F00: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82906F04: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82906F08: 997F0048  stb r11, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 82906F0C: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82906F10: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82906F14: C00AE0B4  lfs f0, -0x1f4c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8012 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82906F18: 997F0058  stb r11, 0x58(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82906F1C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82906F20: 913F00A8  stw r9, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[9].u32 ) };
	// 82906F24: 911F00AC  stw r8, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[8].u32 ) };
	// 82906F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82906F2C: 90FF00B0  stw r7, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[7].u32 ) };
	// 82906F30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82906F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82906F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82906F3C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82906F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82906F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82906F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82906F48 size=792
    let mut pc: u32 = 0x82906F48;
    'dispatch: loop {
        match pc {
            0x82906F48 => {
    //   block [0x82906F48..0x82907260)
	// 82906F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82906F4C: 488A1221  bl 0x831a816c
	ctx.lr = 0x82906F50;
	sub_831A8130(ctx, base);
	// 82906F50: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82906F54: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82906F58: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82906F5C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82906F60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82906F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82906F68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82906F6C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906F70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82906F74: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82906F78: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82906F7C: 419A0024  beq cr6, 0x82906fa0
	if ctx.cr[6].eq {
	pc = 0x82906FA0; continue 'dispatch;
	}
	// 82906F80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82906F84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82906F88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906F8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82906F90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82906F94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82906F98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82906F9C: 4082FFE8  bne 0x82906f84
	if !ctx.cr[0].eq {
	pc = 0x82906F84; continue 'dispatch;
	}
	// 82906FA0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82906FA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82906FA8: 48053B71  bl 0x8295ab18
	ctx.lr = 0x82906FAC;
	sub_8295AB18(ctx, base);
	// 82906FAC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82906FB0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906FB4: 388B3D90  addi r4, r11, 0x3d90
	ctx.r[4].s64 = ctx.r[11].s64 + 15760;
	// 82906FB8: 484ECA51  bl 0x82df3a08
	ctx.lr = 0x82906FBC;
	sub_82DF3A08(ctx, base);
	// 82906FBC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82906FC0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82906FC4: 388B3D88  addi r4, r11, 0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + 15752;
	// 82906FC8: 484ECA41  bl 0x82df3a08
	ctx.lr = 0x82906FCC;
	sub_82DF3A08(ctx, base);
	// 82906FCC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82906FD0: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82906FD4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82906FD8: 4BCC3529  bl 0x825ca500
	ctx.lr = 0x82906FDC;
	sub_825CA500(ctx, base);
	// 82906FDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82906FE0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82906FE4: 484EC445  bl 0x82df3428
	ctx.lr = 0x82906FE8;
	sub_82DF3428(ctx, base);
	// 82906FE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82906FEC: 484EC43D  bl 0x82df3428
	ctx.lr = 0x82906FF0;
	sub_82DF3428(ctx, base);
	// 82906FF0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82906FF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82906FF8: 388B51E4  addi r4, r11, 0x51e4
	ctx.r[4].s64 = ctx.r[11].s64 + 20964;
	// 82906FFC: 484ECA0D  bl 0x82df3a08
	ctx.lr = 0x82907000;
	sub_82DF3A08(ctx, base);
	// 82907000: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907004: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907008: 388B51D8  addi r4, r11, 0x51d8
	ctx.r[4].s64 = ctx.r[11].s64 + 20952;
	// 8290700C: 484EC9FD  bl 0x82df3a08
	ctx.lr = 0x82907010;
	sub_82DF3A08(ctx, base);
	// 82907010: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82907014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82907018: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8290701C: 38BE0078  addi r5, r30, 0x78
	ctx.r[5].s64 = ctx.r[30].s64 + 120;
	// 82907020: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82907024: C3EB08A8  lfs f31, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82907028: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290702C: C3CA6218  lfs f30, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82907030: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82907034: C3A908A4  lfs f29, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82907038: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8290703C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82907040: 4BC9C269  bl 0x825a32a8
	ctx.lr = 0x82907044;
	sub_825A32A8(ctx, base);
	// 82907044: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82907048: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290704C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907050: 4BC9A721  bl 0x825a1770
	ctx.lr = 0x82907054;
	sub_825A1770(ctx, base);
	// 82907054: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82907058: 484EC3D1  bl 0x82df3428
	ctx.lr = 0x8290705C;
	sub_82DF3428(ctx, base);
	// 8290705C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82907060: 4B9C1C59  bl 0x822c8cb8
	ctx.lr = 0x82907064;
	sub_822C8CB8(ctx, base);
	// 82907064: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907068: 484EC3C1  bl 0x82df3428
	ctx.lr = 0x8290706C;
	sub_82DF3428(ctx, base);
	// 8290706C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82907070: 484EC3B9  bl 0x82df3428
	ctx.lr = 0x82907074;
	sub_82DF3428(ctx, base);
	// 82907074: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907078: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290707C: 388B4800  addi r4, r11, 0x4800
	ctx.r[4].s64 = ctx.r[11].s64 + 18432;
	// 82907080: 484EC989  bl 0x82df3a08
	ctx.lr = 0x82907084;
	sub_82DF3A08(ctx, base);
	// 82907084: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907088: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290708C: 388B47F4  addi r4, r11, 0x47f4
	ctx.r[4].s64 = ctx.r[11].s64 + 18420;
	// 82907090: 484EC979  bl 0x82df3a08
	ctx.lr = 0x82907094;
	sub_82DF3A08(ctx, base);
	// 82907094: 38BE007C  addi r5, r30, 0x7c
	ctx.r[5].s64 = ctx.r[30].s64 + 124;
	// 82907098: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290709C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 829070A0: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 829070A4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 829070A8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 829070AC: 4BC9C1FD  bl 0x825a32a8
	ctx.lr = 0x829070B0;
	sub_825A32A8(ctx, base);
	// 829070B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 829070B4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 829070B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829070BC: 4BC9A6B5  bl 0x825a1770
	ctx.lr = 0x829070C0;
	sub_825A1770(ctx, base);
	// 829070C0: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 829070C4: 484EC365  bl 0x82df3428
	ctx.lr = 0x829070C8;
	sub_82DF3428(ctx, base);
	// 829070C8: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 829070CC: 4B9C1BED  bl 0x822c8cb8
	ctx.lr = 0x829070D0;
	sub_822C8CB8(ctx, base);
	// 829070D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 829070D4: 484EC355  bl 0x82df3428
	ctx.lr = 0x829070D8;
	sub_82DF3428(ctx, base);
	// 829070D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829070DC: 484EC34D  bl 0x82df3428
	ctx.lr = 0x829070E0;
	sub_82DF3428(ctx, base);
	// 829070E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829070E4: 4BCC2BED  bl 0x825c9cd0
	ctx.lr = 0x829070E8;
	sub_825C9CD0(ctx, base);
	// 829070E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 829070EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829070F0: 388B6980  addi r4, r11, 0x6980
	ctx.r[4].s64 = ctx.r[11].s64 + 27008;
	// 829070F4: 484EC915  bl 0x82df3a08
	ctx.lr = 0x829070F8;
	sub_82DF3A08(ctx, base);
	// 829070F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829070FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82907100: 388BA7FC  addi r4, r11, -0x5804
	ctx.r[4].s64 = ctx.r[11].s64 + -22532;
	// 82907104: 484EC905  bl 0x82df3a08
	ctx.lr = 0x82907108;
	sub_82DF3A08(ctx, base);
	// 82907108: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290710C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82907110: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82907114: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82907118: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290711C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82907120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907124: 4E800421  bctrl
	ctx.lr = 0x82907128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907128: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290712C: 484EC2FD  bl 0x82df3428
	ctx.lr = 0x82907130;
	sub_82DF3428(ctx, base);
	// 82907130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907134: 484EC2F5  bl 0x82df3428
	ctx.lr = 0x82907138;
	sub_82DF3428(ctx, base);
	// 82907138: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290713C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907140: 388B47EC  addi r4, r11, 0x47ec
	ctx.r[4].s64 = ctx.r[11].s64 + 18412;
	// 82907144: 484EC8C5  bl 0x82df3a08
	ctx.lr = 0x82907148;
	sub_82DF3A08(ctx, base);
	// 82907148: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290714C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82907150: 388B47E0  addi r4, r11, 0x47e0
	ctx.r[4].s64 = ctx.r[11].s64 + 18400;
	// 82907154: 484EC8B5  bl 0x82df3a08
	ctx.lr = 0x82907158;
	sub_82DF3A08(ctx, base);
	// 82907158: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290715C: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82907160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907164: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82907168: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8290716C: 419A0024  beq cr6, 0x82907190
	if ctx.cr[6].eq {
	pc = 0x82907190; continue 'dispatch;
	}
	// 82907170: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82907174: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82907178: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290717C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82907180: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82907184: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82907188: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290718C: 4082FFE8  bne 0x82907174
	if !ctx.cr[0].eq {
	pc = 0x82907174; continue 'dispatch;
	}
	// 82907190: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82907194: 387E0080  addi r3, r30, 0x80
	ctx.r[3].s64 = ctx.r[30].s64 + 128;
	// 82907198: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8290719C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 829071A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 829071A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829071A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829071AC: 4E800421  bctrl
	ctx.lr = 0x829071B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829071B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 829071B4: 484EC275  bl 0x82df3428
	ctx.lr = 0x829071B8;
	sub_82DF3428(ctx, base);
	// 829071B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829071BC: 484EC26D  bl 0x82df3428
	ctx.lr = 0x829071C0;
	sub_82DF3428(ctx, base);
	// 829071C0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 829071C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829071C8: 419A0008  beq cr6, 0x829071d0
	if ctx.cr[6].eq {
	pc = 0x829071D0; continue 'dispatch;
	}
	// 829071CC: 4B9B96C5  bl 0x822c0890
	ctx.lr = 0x829071D0;
	sub_822C0890(ctx, base);
	// 829071D0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 829071D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829071D8: 388B7A1C  addi r4, r11, 0x7a1c
	ctx.r[4].s64 = ctx.r[11].s64 + 31260;
	// 829071DC: 484EC82D  bl 0x82df3a08
	ctx.lr = 0x829071E0;
	sub_82DF3A08(ctx, base);
	// 829071E0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 829071E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 829071E8: 388B1A04  addi r4, r11, 0x1a04
	ctx.r[4].s64 = ctx.r[11].s64 + 6660;
	// 829071EC: 484EC81D  bl 0x82df3a08
	ctx.lr = 0x829071F0;
	sub_82DF3A08(ctx, base);
	// 829071F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829071F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829071F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829071FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82907200: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82907204: 419A0024  beq cr6, 0x82907228
	if ctx.cr[6].eq {
	pc = 0x82907228; continue 'dispatch;
	}
	// 82907208: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290720C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82907210: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82907214: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82907218: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290721C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82907220: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82907224: 4082FFE8  bne 0x8290720c
	if !ctx.cr[0].eq {
	pc = 0x8290720C; continue 'dispatch;
	}
	// 82907228: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8290722C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82907230: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82907234: 387E00A8  addi r3, r30, 0xa8
	ctx.r[3].s64 = ctx.r[30].s64 + 168;
	// 82907238: 480509D9  bl 0x82957c10
	ctx.lr = 0x8290723C;
	sub_82957C10(ctx, base);
	// 8290723C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82907240: 484EC1E9  bl 0x82df3428
	ctx.lr = 0x82907244;
	sub_82DF3428(ctx, base);
	// 82907244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907248: 484EC1E1  bl 0x82df3428
	ctx.lr = 0x8290724C;
	sub_82DF3428(ctx, base);
	// 8290724C: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82907250: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82907254: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82907258: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8290725C: 488A0F60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82907260 size=544
    let mut pc: u32 = 0x82907260;
    'dispatch: loop {
        match pc {
            0x82907260 => {
    //   block [0x82907260..0x82907480)
	// 82907260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907264: 488A0F09  bl 0x831a816c
	ctx.lr = 0x82907268;
	sub_831A8130(ctx, base);
	// 82907268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290726C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82907270: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82907274: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907278: 4BFFFB89  bl 0x82906e00
	ctx.lr = 0x8290727C;
	sub_82906E00(ctx, base);
	// 8290727C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82907280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907284: 4BFBB64D  bl 0x828c28d0
	ctx.lr = 0x82907288;
	sub_828C28D0(ctx, base);
	// 82907288: C01F0078  lfs f0, 0x78(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290728C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82907290: 409900B0  ble cr6, 0x82907340
	if !ctx.cr[6].gt {
	pc = 0x82907340; continue 'dispatch;
	}
	// 82907294: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82907298: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8290729C: 3BEBB168  addi r31, r11, -0x4e98
	ctx.r[31].s64 = ctx.r[11].s64 + -20120;
	// 829072A0: 816AB178  lwz r11, -0x4e88(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20104 as u32) ) } as u64;
	// 829072A4: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 829072A8: 4082002C  bne 0x829072d4
	if !ctx.cr[0].eq {
	pc = 0x829072D4; continue 'dispatch;
	}
	// 829072AC: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 829072B0: 3D00832D  lis r8, -0x7cd3
	ctx.r[8].s64 = -2094202880;
	// 829072B4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 829072B8: 916AB178  stw r11, -0x4e88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20104 as u32), ctx.r[11].u32 ) };
	// 829072BC: 8169F3F8  lwz r11, -0xc08(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 829072C0: 8148D074  lwz r10, -0x2f8c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12172 as u32) ) } as u64;
	// 829072C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829072C8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 829072CC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 829072D0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 829072D4: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 829072D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 829072DC: 419A0180  beq cr6, 0x8290745c
	if ctx.cr[6].eq {
	pc = 0x8290745C; continue 'dispatch;
	}
	// 829072E0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829072E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829072E8: 808B0BF0  lwz r4, 0xbf0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3056 as u32) ) } as u64;
	// 829072EC: 484EC71D  bl 0x82df3a08
	ctx.lr = 0x829072F0;
	sub_82DF3A08(ctx, base);
	// 829072F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829072F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829072F8: 4BEE8479  bl 0x827ef770
	ctx.lr = 0x829072FC;
	sub_827EF770(ctx, base);
	// 829072FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907300: 484EC129  bl 0x82df3428
	ctx.lr = 0x82907304;
	sub_82DF3428(ctx, base);
	// 82907304: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907308: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290730C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82907310: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82907314: 484EC6F5  bl 0x82df3a08
	ctx.lr = 0x82907318;
	sub_82DF3A08(ctx, base);
	// 82907318: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290731C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907324: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290732C: 4E800421  bctrl
	ctx.lr = 0x82907330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907334: 484EC0F5  bl 0x82df3428
	ctx.lr = 0x82907338;
	sub_82DF3428(ctx, base);
	// 82907338: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290733C: 4800011C  b 0x82907458
	pc = 0x82907458; continue 'dispatch;
	// 82907340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907344: 4BFBB58D  bl 0x828c28d0
	ctx.lr = 0x82907348;
	sub_828C28D0(ctx, base);
	// 82907348: C01F007C  lfs f0, 0x7c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290734C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82907350: 409900B0  ble cr6, 0x82907400
	if !ctx.cr[6].gt {
	pc = 0x82907400; continue 'dispatch;
	}
	// 82907354: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82907358: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8290735C: 3BEBB158  addi r31, r11, -0x4ea8
	ctx.r[31].s64 = ctx.r[11].s64 + -20136;
	// 82907360: 816AB178  lwz r11, -0x4e88(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20104 as u32) ) } as u64;
	// 82907364: 556907BD  rlwinm. r9, r11, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82907368: 4082002C  bne 0x82907394
	if !ctx.cr[0].eq {
	pc = 0x82907394; continue 'dispatch;
	}
	// 8290736C: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82907370: 3D00832D  lis r8, -0x7cd3
	ctx.r[8].s64 = -2094202880;
	// 82907374: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82907378: 916AB178  stw r11, -0x4e88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20104 as u32), ctx.r[11].u32 ) };
	// 8290737C: 8169D078  lwz r11, -0x2f88(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12168 as u32) ) } as u64;
	// 82907380: 8148D07C  lwz r10, -0x2f84(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12164 as u32) ) } as u64;
	// 82907384: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907388: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290738C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82907390: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82907394: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907398: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8290739C: 419A00C0  beq cr6, 0x8290745c
	if ctx.cr[6].eq {
	pc = 0x8290745C; continue 'dispatch;
	}
	// 829073A0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829073A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829073A8: 808B0BEC  lwz r4, 0xbec(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3052 as u32) ) } as u64;
	// 829073AC: 484EC65D  bl 0x82df3a08
	ctx.lr = 0x829073B0;
	sub_82DF3A08(ctx, base);
	// 829073B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829073B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829073B8: 4BEE83B9  bl 0x827ef770
	ctx.lr = 0x829073BC;
	sub_827EF770(ctx, base);
	// 829073BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829073C0: 484EC069  bl 0x82df3428
	ctx.lr = 0x829073C4;
	sub_82DF3428(ctx, base);
	// 829073C4: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 829073C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829073CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 829073D0: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 829073D4: 484EC635  bl 0x82df3a08
	ctx.lr = 0x829073D8;
	sub_82DF3A08(ctx, base);
	// 829073D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 829073DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829073E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829073E4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 829073E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829073EC: 4E800421  bctrl
	ctx.lr = 0x829073F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829073F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829073F4: 484EC035  bl 0x82df3428
	ctx.lr = 0x829073F8;
	sub_82DF3428(ctx, base);
	// 829073F8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 829073FC: 4800005C  b 0x82907458
	pc = 0x82907458; continue 'dispatch;
	// 82907400: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907404: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82907408: 419A0054  beq cr6, 0x8290745c
	if ctx.cr[6].eq {
	pc = 0x8290745C; continue 'dispatch;
	}
	// 8290740C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907414: 808B0BEC  lwz r4, 0xbec(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3052 as u32) ) } as u64;
	// 82907418: 484EC5F1  bl 0x82df3a08
	ctx.lr = 0x8290741C;
	sub_82DF3A08(ctx, base);
	// 8290741C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907424: 4BEE834D  bl 0x827ef770
	ctx.lr = 0x82907428;
	sub_827EF770(ctx, base);
	// 82907428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290742C: 484EBFFD  bl 0x82df3428
	ctx.lr = 0x82907430;
	sub_82DF3428(ctx, base);
	// 82907430: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907434: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907438: 808BD0AC  lwz r4, -0x2f54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12116 as u32) ) } as u64;
	// 8290743C: 484EC5CD  bl 0x82df3a08
	ctx.lr = 0x82907440;
	sub_82DF3A08(ctx, base);
	// 82907440: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907448: 4BEE82F9  bl 0x827ef740
	ctx.lr = 0x8290744C;
	sub_827EF740(ctx, base);
	// 8290744C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907450: 484EBFD9  bl 0x82df3428
	ctx.lr = 0x82907454;
	sub_82DF3428(ctx, base);
	// 82907454: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82907458: 917D008C  stw r11, 0x8c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8290745C: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907460: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82907464: 419A000C  beq cr6, 0x82907470
	if ctx.cr[6].eq {
	pc = 0x82907470; continue 'dispatch;
	}
	// 82907468: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8290746C: 409A000C  bne cr6, 0x82907478
	if !ctx.cr[6].eq {
	pc = 0x82907478; continue 'dispatch;
	}
	// 82907470: 387D006C  addi r3, r29, 0x6c
	ctx.r[3].s64 = ctx.r[29].s64 + 108;
	// 82907474: 484FF0AD  bl 0x82e06520
	ctx.lr = 0x82907478;
	sub_82E06520(ctx, base);
	// 82907478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290747C: 488A0D40  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907480 size=88
    let mut pc: u32 = 0x82907480;
    'dispatch: loop {
        match pc {
            0x82907480 => {
    //   block [0x82907480..0x829074D8)
	// 82907480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290748C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907490: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907498: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290749C: 808BD078  lwz r4, -0x2f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12168 as u32) ) } as u64;
	// 829074A0: 484EC569  bl 0x82df3a08
	ctx.lr = 0x829074A4;
	sub_82DF3A08(ctx, base);
	// 829074A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829074A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829074AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829074B0: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 829074B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829074B8: 4E800421  bctrl
	ctx.lr = 0x829074BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829074BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829074C0: 484EBF69  bl 0x82df3428
	ctx.lr = 0x829074C4;
	sub_82DF3428(ctx, base);
	// 829074C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829074C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829074CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829074D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829074D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829074D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829074D8 size=84
    let mut pc: u32 = 0x829074D8;
    'dispatch: loop {
        match pc {
            0x829074D8 => {
    //   block [0x829074D8..0x8290752C)
	// 829074D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829074DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829074E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829074E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829074E8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829074EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829074F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829074F4: 808B0BF0  lwz r4, 0xbf0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3056 as u32) ) } as u64;
	// 829074F8: 484EC511  bl 0x82df3a08
	ctx.lr = 0x829074FC;
	sub_82DF3A08(ctx, base);
	// 829074FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907500: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907504: 4BEE826D  bl 0x827ef770
	ctx.lr = 0x82907508;
	sub_827EF770(ctx, base);
	// 82907508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290750C: 484EBF1D  bl 0x82df3428
	ctx.lr = 0x82907510;
	sub_82DF3428(ctx, base);
	// 82907510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907514: 4BFFF82D  bl 0x82906d40
	ctx.lr = 0x82907518;
	sub_82906D40(ctx, base);
	// 82907518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290751C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907530 size=124
    let mut pc: u32 = 0x82907530;
    'dispatch: loop {
        match pc {
            0x82907530 => {
    //   block [0x82907530..0x829075AC)
	// 82907530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290753C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907540: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907544: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907548: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290754C: 808BD078  lwz r4, -0x2f88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12168 as u32) ) } as u64;
	// 82907550: 484EC4B9  bl 0x82df3a08
	ctx.lr = 0x82907554;
	sub_82DF3A08(ctx, base);
	// 82907554: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290755C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907560: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907568: 4E800421  bctrl
	ctx.lr = 0x8290756C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290756C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907570: 484EBEB9  bl 0x82df3428
	ctx.lr = 0x82907574;
	sub_82DF3428(ctx, base);
	// 82907574: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907578: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290757C: 808B0BEC  lwz r4, 0xbec(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3052 as u32) ) } as u64;
	// 82907580: 484EC489  bl 0x82df3a08
	ctx.lr = 0x82907584;
	sub_82DF3A08(ctx, base);
	// 82907584: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290758C: 4BEE81E5  bl 0x827ef770
	ctx.lr = 0x82907590;
	sub_827EF770(ctx, base);
	// 82907590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907594: 484EBE95  bl 0x82df3428
	ctx.lr = 0x82907598;
	sub_82DF3428(ctx, base);
	// 82907598: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290759C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829075A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829075A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829075A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829075B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829075B0 size=76
    let mut pc: u32 = 0x829075B0;
    'dispatch: loop {
        match pc {
            0x829075B0 => {
    //   block [0x829075B0..0x829075FC)
	// 829075B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829075B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829075B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829075BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829075C0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829075C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829075C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829075CC: 808BD0A8  lwz r4, -0x2f58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12120 as u32) ) } as u64;
	// 829075D0: 484EC439  bl 0x82df3a08
	ctx.lr = 0x829075D4;
	sub_82DF3A08(ctx, base);
	// 829075D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829075D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829075DC: 4BEE8165  bl 0x827ef740
	ctx.lr = 0x829075E0;
	sub_827EF740(ctx, base);
	// 829075E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829075E4: 484EBE45  bl 0x82df3428
	ctx.lr = 0x829075E8;
	sub_82DF3428(ctx, base);
	// 829075E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829075EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829075F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829075F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829075F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907600 size=88
    let mut pc: u32 = 0x82907600;
    'dispatch: loop {
        match pc {
            0x82907600 => {
    //   block [0x82907600..0x82907658)
	// 82907600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907608: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290760C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907610: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907614: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907618: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290761C: 808BD084  lwz r4, -0x2f7c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12156 as u32) ) } as u64;
	// 82907620: 484EC3E9  bl 0x82df3a08
	ctx.lr = 0x82907624;
	sub_82DF3A08(ctx, base);
	// 82907624: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907628: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290762C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907630: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82907634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907638: 4E800421  bctrl
	ctx.lr = 0x8290763C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290763C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907640: 484EBDE9  bl 0x82df3428
	ctx.lr = 0x82907644;
	sub_82DF3428(ctx, base);
	// 82907644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290764C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907658 size=108
    let mut pc: u32 = 0x82907658;
    'dispatch: loop {
        match pc {
            0x82907658 => {
    //   block [0x82907658..0x829076C4)
	// 82907658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290765C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907664: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907668: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290766C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907674: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82907678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290767C: 4E800421  bctrl
	ctx.lr = 0x82907680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907680: 4BEE2C91  bl 0x827ea310
	ctx.lr = 0x82907684;
	sub_827EA310(ctx, base);
	// 82907684: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82907688: 41820028  beq 0x829076b0
	if ctx.cr[0].eq {
	pc = 0x829076B0; continue 'dispatch;
	}
	// 8290768C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907690: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907694: 808BD0A8  lwz r4, -0x2f58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12120 as u32) ) } as u64;
	// 82907698: 484EC371  bl 0x82df3a08
	ctx.lr = 0x8290769C;
	sub_82DF3A08(ctx, base);
	// 8290769C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829076A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829076A4: 4BEE809D  bl 0x827ef740
	ctx.lr = 0x829076A8;
	sub_827EF740(ctx, base);
	// 829076A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829076AC: 484EBD7D  bl 0x82df3428
	ctx.lr = 0x829076B0;
	sub_82DF3428(ctx, base);
	// 829076B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829076B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829076B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829076BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829076C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829076C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829076C8 size=160
    let mut pc: u32 = 0x829076C8;
    'dispatch: loop {
        match pc {
            0x829076C8 => {
    //   block [0x829076C8..0x82907768)
	// 829076C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829076CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829076D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829076D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829076D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829076DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829076E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829076E4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 829076E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829076EC: 4E800421  bctrl
	ctx.lr = 0x829076F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829076F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829076F4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829076F8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 829076FC: 40990024  ble cr6, 0x82907720
	if !ctx.cr[6].gt {
	pc = 0x82907720; continue 'dispatch;
	}
	// 82907700: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907704: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907708: 808B0B34  lwz r4, 0xb34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2868 as u32) ) } as u64;
	// 8290770C: 484EC2FD  bl 0x82df3a08
	ctx.lr = 0x82907710;
	sub_82DF3A08(ctx, base);
	// 82907710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907718: 4BEE8029  bl 0x827ef740
	ctx.lr = 0x8290771C;
	sub_827EF740(ctx, base);
	// 8290771C: 48000030  b 0x8290774c
	pc = 0x8290774C; continue 'dispatch;
	// 82907720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907724: 4BFFF69D  bl 0x82906dc0
	ctx.lr = 0x82907728;
	sub_82906DC0(ctx, base);
	// 82907728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290772C: 4BFFF655  bl 0x82906d80
	ctx.lr = 0x82907730;
	sub_82906D80(ctx, base);
	// 82907730: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907734: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907738: 808B0BF0  lwz r4, 0xbf0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3056 as u32) ) } as u64;
	// 8290773C: 484EC2CD  bl 0x82df3a08
	ctx.lr = 0x82907740;
	sub_82DF3A08(ctx, base);
	// 82907740: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907748: 4BEE8029  bl 0x827ef770
	ctx.lr = 0x8290774C;
	sub_827EF770(ctx, base);
	// 8290774C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907750: 484EBCD9  bl 0x82df3428
	ctx.lr = 0x82907754;
	sub_82DF3428(ctx, base);
	// 82907754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290775C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82907768 size=16
    let mut pc: u32 = 0x82907768;
    'dispatch: loop {
        match pc {
            0x82907768 => {
    //   block [0x82907768..0x82907778)
	// 82907768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290776C: 894B0051  lbz r10, 0x51(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(81 as u32) ) } as u64;
	// 82907770: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82907774: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82907778 size=24
    let mut pc: u32 = 0x82907778;
    'dispatch: loop {
        match pc {
            0x82907778 => {
    //   block [0x82907778..0x82907790)
	// 82907778: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8290777C: 892A0051  lbz r9, 0x51(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(81 as u32) ) } as u64;
	// 82907780: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82907784: 409A0040  bne cr6, 0x829077c4
	if !ctx.cr[6].eq {
		sub_829077AC(ctx, base);
		return;
	}
	// 82907788: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290778C: 4800000C  b 0x82907798
	sub_82907790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82907790 size=28
    let mut pc: u32 = 0x82907790;
    'dispatch: loop {
        match pc {
            0x82907790 => {
    //   block [0x82907790..0x829077AC)
	// 82907790: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82907794: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907798: 892B0051  lbz r9, 0x51(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(81 as u32) ) } as u64;
	// 8290779C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 829077A0: 419AFFF0  beq cr6, 0x82907790
	if ctx.cr[6].eq {
	pc = 0x82907790; continue 'dispatch;
	}
	// 829077A4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 829077A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829077AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829077AC size=48
    let mut pc: u32 = 0x829077AC;
    'dispatch: loop {
        match pc {
            0x829077AC => {
    //   block [0x829077AC..0x829077DC)
	// 829077AC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829077B0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 829077B4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 829077B8: 409A001C  bne cr6, 0x829077d4
	if !ctx.cr[6].eq {
	pc = 0x829077D4; continue 'dispatch;
	}
	// 829077BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829077C0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 829077C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 829077C8: 894B0051  lbz r10, 0x51(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(81 as u32) ) } as u64;
	// 829077CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 829077D0: 419AFFDC  beq cr6, 0x829077ac
	if ctx.cr[6].eq {
	pc = 0x829077AC; continue 'dispatch;
	}
	// 829077D4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829077D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829077E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829077E0 size=136
    let mut pc: u32 = 0x829077E0;
    'dispatch: loop {
        match pc {
            0x829077E0 => {
    //   block [0x829077E0..0x82907868)
	// 829077E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829077E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829077E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829077EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829077F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829077F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829077F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829077FC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82907800: 409A0020  bne cr6, 0x82907820
	if !ctx.cr[6].eq {
	pc = 0x82907820; continue 'dispatch;
	}
	// 82907804: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907808: 419A0048  beq cr6, 0x82907850
	if ctx.cr[6].eq {
	pc = 0x82907850; continue 'dispatch;
	}
	// 8290780C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82907810: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82907814: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82907818: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8290781C: 48000034  b 0x82907850
	pc = 0x82907850; continue 'dispatch;
	// 82907820: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82907824: 419A002C  beq cr6, 0x82907850
	if ctx.cr[6].eq {
	pc = 0x82907850; continue 'dispatch;
	}
	// 82907828: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290782C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907830: 388BD478  addi r4, r11, -0x2b88
	ctx.r[4].s64 = ctx.r[11].s64 + -11144;
	// 82907834: 488A08C5  bl 0x831a80f8
	ctx.lr = 0x82907838;
	sub_831A80F8(ctx, base);
	// 82907838: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290783C: 4182000C  beq 0x82907848
	if ctx.cr[0].eq {
	pc = 0x82907848; continue 'dispatch;
	}
	// 82907840: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82907844: 4800000C  b 0x82907850
	pc = 0x82907850; continue 'dispatch;
	// 82907848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290784C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290785C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907860: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907868 size=136
    let mut pc: u32 = 0x82907868;
    'dispatch: loop {
        match pc {
            0x82907868 => {
    //   block [0x82907868..0x829078F0)
	// 82907868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290786C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290787C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907880: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907884: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82907888: 409A0020  bne cr6, 0x829078a8
	if !ctx.cr[6].eq {
	pc = 0x829078A8; continue 'dispatch;
	}
	// 8290788C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907890: 419A0048  beq cr6, 0x829078d8
	if ctx.cr[6].eq {
	pc = 0x829078D8; continue 'dispatch;
	}
	// 82907894: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82907898: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8290789C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 829078A0: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 829078A4: 48000034  b 0x829078d8
	pc = 0x829078D8; continue 'dispatch;
	// 829078A8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 829078AC: 419A002C  beq cr6, 0x829078d8
	if ctx.cr[6].eq {
	pc = 0x829078D8; continue 'dispatch;
	}
	// 829078B0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829078B4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829078B8: 388BD550  addi r4, r11, -0x2ab0
	ctx.r[4].s64 = ctx.r[11].s64 + -10928;
	// 829078BC: 488A083D  bl 0x831a80f8
	ctx.lr = 0x829078C0;
	sub_831A80F8(ctx, base);
	// 829078C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829078C4: 4182000C  beq 0x829078d0
	if ctx.cr[0].eq {
	pc = 0x829078D0; continue 'dispatch;
	}
	// 829078C8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 829078CC: 4800000C  b 0x829078d8
	pc = 0x829078D8; continue 'dispatch;
	// 829078D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829078D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829078D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829078DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829078E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829078E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829078E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829078EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829078F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829078F0 size=136
    let mut pc: u32 = 0x829078F0;
    'dispatch: loop {
        match pc {
            0x829078F0 => {
    //   block [0x829078F0..0x82907978)
	// 829078F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829078F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829078F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829078FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907904: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907908: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290790C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82907910: 409A0020  bne cr6, 0x82907930
	if !ctx.cr[6].eq {
	pc = 0x82907930; continue 'dispatch;
	}
	// 82907914: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907918: 419A0048  beq cr6, 0x82907960
	if ctx.cr[6].eq {
	pc = 0x82907960; continue 'dispatch;
	}
	// 8290791C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82907920: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82907924: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82907928: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8290792C: 48000034  b 0x82907960
	pc = 0x82907960; continue 'dispatch;
	// 82907930: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82907934: 419A002C  beq cr6, 0x82907960
	if ctx.cr[6].eq {
	pc = 0x82907960; continue 'dispatch;
	}
	// 82907938: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290793C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907940: 388BD630  addi r4, r11, -0x29d0
	ctx.r[4].s64 = ctx.r[11].s64 + -10704;
	// 82907944: 488A07B5  bl 0x831a80f8
	ctx.lr = 0x82907948;
	sub_831A80F8(ctx, base);
	// 82907948: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290794C: 4182000C  beq 0x82907958
	if ctx.cr[0].eq {
	pc = 0x82907958; continue 'dispatch;
	}
	// 82907950: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82907954: 4800000C  b 0x82907960
	pc = 0x82907960; continue 'dispatch;
	// 82907958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290795C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907960: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290796C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82907978 size=80
    let mut pc: u32 = 0x82907978;
    'dispatch: loop {
        match pc {
            0x82907978 => {
    //   block [0x82907978..0x829079C8)
	// 82907978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290798C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82907990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82907994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907998: 4BFFF469  bl 0x82906e00
	ctx.lr = 0x8290799C;
	sub_82906E00(ctx, base);
	// 8290799C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829079A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829079A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829079A8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829079AC: 4BFFF8B5  bl 0x82907260
	ctx.lr = 0x829079B0;
	sub_82907260(ctx, base);
	// 829079B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829079B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829079B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829079BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829079C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829079C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829079C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829079C8 size=60
    let mut pc: u32 = 0x829079C8;
    'dispatch: loop {
        match pc {
            0x829079C8 => {
    //   block [0x829079C8..0x82907A04)
	// 829079C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829079CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829079D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829079D4: 4884CC2D  bl 0x83154600
	ctx.lr = 0x829079D8;
	sub_83154600(ctx, base);
	// 829079D8: 4800ED91  bl 0x82916768
	ctx.lr = 0x829079DC;
	sub_82916768(ctx, base);
	// 829079DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829079E0: 41820014  beq 0x829079f4
	if ctx.cr[0].eq {
	pc = 0x829079F4; continue 'dispatch;
	}
	// 829079E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829079E8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 829079EC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829079F0: 48007389  bl 0x8290ed78
	ctx.lr = 0x829079F4;
	sub_8290ED78(ctx, base);
	// 829079F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829079F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829079FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82907A08 size=204
    let mut pc: u32 = 0x82907A08;
    'dispatch: loop {
        match pc {
            0x82907A08 => {
    //   block [0x82907A08..0x82907AD4)
	// 82907A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907A1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907A20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907A28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907A2C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82907A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907A34: 4E800421  bctrl
	ctx.lr = 0x82907A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907A38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82907A3C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82907A40: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82907A44: 41990010  bgt cr6, 0x82907a54
	if ctx.cr[6].gt {
	pc = 0x82907A54; continue 'dispatch;
	}
	// 82907A48: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907A4C: 808B0B2C  lwz r4, 0xb2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2860 as u32) ) } as u64;
	// 82907A50: 48000024  b 0x82907a74
	pc = 0x82907A74; continue 'dispatch;
	// 82907A54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907A58: 48551D09  bl 0x82e59760
	ctx.lr = 0x82907A5C;
	sub_82E59760(ctx, base);
	// 82907A5C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907A60: C00B5228  lfs f0, 0x5228(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21032 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82907A64: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82907A68: 40990028  ble cr6, 0x82907a90
	if !ctx.cr[6].gt {
	pc = 0x82907A90; continue 'dispatch;
	}
	// 82907A6C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907A70: 808B0AF8  lwz r4, 0xaf8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2808 as u32) ) } as u64;
	// 82907A74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907A78: 484EBF91  bl 0x82df3a08
	ctx.lr = 0x82907A7C;
	sub_82DF3A08(ctx, base);
	// 82907A7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82907A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907A84: 4BEE7CBD  bl 0x827ef740
	ctx.lr = 0x82907A88;
	sub_827EF740(ctx, base);
	// 82907A88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907A8C: 484EB99D  bl 0x82df3428
	ctx.lr = 0x82907A90;
	sub_82DF3428(ctx, base);
	// 82907A90: 83DE006C  lwz r30, 0x6c(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82907A94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82907A98: 419A0024  beq cr6, 0x82907abc
	if ctx.cr[6].eq {
	pc = 0x82907ABC; continue 'dispatch;
	}
	// 82907A9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907AA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907AAC: 4E800421  bctrl
	ctx.lr = 0x82907AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907AB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82907AB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907AB8: 4823F0F1  bl 0x82b46ba8
	ctx.lr = 0x82907ABC;
	sub_82B46BA8(ctx, base);
	// 82907ABC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907AC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907ACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907AD8 size=196
    let mut pc: u32 = 0x82907AD8;
    'dispatch: loop {
        match pc {
            0x82907AD8 => {
    //   block [0x82907AD8..0x82907B9C)
	// 82907AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907AEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907AF4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907AF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907AFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907B00: 4B9B8E39  bl 0x822c0938
	ctx.lr = 0x82907B04;
	sub_822C0938(ctx, base);
	// 82907B04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907B08: 41820028  beq 0x82907b30
	if ctx.cr[0].eq {
	pc = 0x82907B30; continue 'dispatch;
	}
	// 82907B0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907B10: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907B14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907B18: 392B5230  addi r9, r11, 0x5230
	ctx.r[9].s64 = ctx.r[11].s64 + 21040;
	// 82907B1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907B20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907B24: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907B28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907B2C: 48000008  b 0x82907b34
	pc = 0x82907B34; continue 'dispatch;
	// 82907B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907B34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907B38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907B3C: 409A0044  bne cr6, 0x82907b80
	if !ctx.cr[6].eq {
	pc = 0x82907B80; continue 'dispatch;
	}
	// 82907B40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907B44: 419A001C  beq cr6, 0x82907b60
	if ctx.cr[6].eq {
	pc = 0x82907B60; continue 'dispatch;
	}
	// 82907B48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907B4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82907B50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907B54: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907B58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907B5C: 4E800421  bctrl
	ctx.lr = 0x82907B60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907B60: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907B64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82907B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907B6C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82907B70: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82907B74: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82907B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82907B7C: 4B9B8485  bl 0x822c0000
	ctx.lr = 0x82907B80;
	sub_822C0000(ctx, base);
	// 82907B80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907BA0 size=196
    let mut pc: u32 = 0x82907BA0;
    'dispatch: loop {
        match pc {
            0x82907BA0 => {
    //   block [0x82907BA0..0x82907C64)
	// 82907BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907BBC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907BC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907BC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907BC8: 4B9B8D71  bl 0x822c0938
	ctx.lr = 0x82907BCC;
	sub_822C0938(ctx, base);
	// 82907BCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907BD0: 41820028  beq 0x82907bf8
	if ctx.cr[0].eq {
	pc = 0x82907BF8; continue 'dispatch;
	}
	// 82907BD4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907BD8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907BE0: 392B5244  addi r9, r11, 0x5244
	ctx.r[9].s64 = ctx.r[11].s64 + 21060;
	// 82907BE4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907BE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907BEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907BF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907BF4: 48000008  b 0x82907bfc
	pc = 0x82907BFC; continue 'dispatch;
	// 82907BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907BFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907C04: 409A0044  bne cr6, 0x82907c48
	if !ctx.cr[6].eq {
	pc = 0x82907C48; continue 'dispatch;
	}
	// 82907C08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907C0C: 419A001C  beq cr6, 0x82907c28
	if ctx.cr[6].eq {
	pc = 0x82907C28; continue 'dispatch;
	}
	// 82907C10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907C14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82907C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907C1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907C24: 4E800421  bctrl
	ctx.lr = 0x82907C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907C28: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907C2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82907C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907C34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82907C38: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82907C3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82907C40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82907C44: 4B9B83BD  bl 0x822c0000
	ctx.lr = 0x82907C48;
	sub_822C0000(ctx, base);
	// 82907C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907C68 size=196
    let mut pc: u32 = 0x82907C68;
    'dispatch: loop {
        match pc {
            0x82907C68 => {
    //   block [0x82907C68..0x82907D2C)
	// 82907C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907C70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907C74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907C78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907C7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907C84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907C88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907C8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907C90: 4B9B8CA9  bl 0x822c0938
	ctx.lr = 0x82907C94;
	sub_822C0938(ctx, base);
	// 82907C94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907C98: 41820028  beq 0x82907cc0
	if ctx.cr[0].eq {
	pc = 0x82907CC0; continue 'dispatch;
	}
	// 82907C9C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907CA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907CA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907CA8: 392B5258  addi r9, r11, 0x5258
	ctx.r[9].s64 = ctx.r[11].s64 + 21080;
	// 82907CAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907CB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907CB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907CB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907CBC: 48000008  b 0x82907cc4
	pc = 0x82907CC4; continue 'dispatch;
	// 82907CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907CC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907CCC: 409A0044  bne cr6, 0x82907d10
	if !ctx.cr[6].eq {
	pc = 0x82907D10; continue 'dispatch;
	}
	// 82907CD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907CD4: 419A001C  beq cr6, 0x82907cf0
	if ctx.cr[6].eq {
	pc = 0x82907CF0; continue 'dispatch;
	}
	// 82907CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907CDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82907CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907CE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907CE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907CEC: 4E800421  bctrl
	ctx.lr = 0x82907CF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907CF0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907CF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82907CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907CFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82907D00: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82907D04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82907D08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82907D0C: 4B9B82F5  bl 0x822c0000
	ctx.lr = 0x82907D10;
	sub_822C0000(ctx, base);
	// 82907D10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907D14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907D20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907D30 size=196
    let mut pc: u32 = 0x82907D30;
    'dispatch: loop {
        match pc {
            0x82907D30 => {
    //   block [0x82907D30..0x82907DF4)
	// 82907D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907D44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907D4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907D50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907D54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907D58: 4B9B8BE1  bl 0x822c0938
	ctx.lr = 0x82907D5C;
	sub_822C0938(ctx, base);
	// 82907D5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907D60: 41820028  beq 0x82907d88
	if ctx.cr[0].eq {
	pc = 0x82907D88; continue 'dispatch;
	}
	// 82907D64: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907D68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907D6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907D70: 392B526C  addi r9, r11, 0x526c
	ctx.r[9].s64 = ctx.r[11].s64 + 21100;
	// 82907D74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907D78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907D7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907D80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907D84: 48000008  b 0x82907d8c
	pc = 0x82907D8C; continue 'dispatch;
	// 82907D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907D8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907D94: 409A0044  bne cr6, 0x82907dd8
	if !ctx.cr[6].eq {
	pc = 0x82907DD8; continue 'dispatch;
	}
	// 82907D98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907D9C: 419A001C  beq cr6, 0x82907db8
	if ctx.cr[6].eq {
	pc = 0x82907DB8; continue 'dispatch;
	}
	// 82907DA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907DA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82907DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907DAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907DB4: 4E800421  bctrl
	ctx.lr = 0x82907DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907DB8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907DBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82907DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907DC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82907DC8: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82907DCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82907DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82907DD4: 4B9B822D  bl 0x822c0000
	ctx.lr = 0x82907DD8;
	sub_822C0000(ctx, base);
	// 82907DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907DE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907DEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907DF8 size=196
    let mut pc: u32 = 0x82907DF8;
    'dispatch: loop {
        match pc {
            0x82907DF8 => {
    //   block [0x82907DF8..0x82907EBC)
	// 82907DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907E0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907E14: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907E18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907E1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907E20: 4B9B8B19  bl 0x822c0938
	ctx.lr = 0x82907E24;
	sub_822C0938(ctx, base);
	// 82907E24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907E28: 41820028  beq 0x82907e50
	if ctx.cr[0].eq {
	pc = 0x82907E50; continue 'dispatch;
	}
	// 82907E2C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907E30: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907E34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907E38: 392B5280  addi r9, r11, 0x5280
	ctx.r[9].s64 = ctx.r[11].s64 + 21120;
	// 82907E3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907E40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907E44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907E48: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907E4C: 48000008  b 0x82907e54
	pc = 0x82907E54; continue 'dispatch;
	// 82907E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907E54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907E5C: 409A0044  bne cr6, 0x82907ea0
	if !ctx.cr[6].eq {
	pc = 0x82907EA0; continue 'dispatch;
	}
	// 82907E60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907E64: 419A001C  beq cr6, 0x82907e80
	if ctx.cr[6].eq {
	pc = 0x82907E80; continue 'dispatch;
	}
	// 82907E68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907E6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82907E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907E74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907E78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907E7C: 4E800421  bctrl
	ctx.lr = 0x82907E80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907E80: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907E84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82907E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907E8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82907E90: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82907E94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82907E98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82907E9C: 4B9B8165  bl 0x822c0000
	ctx.lr = 0x82907EA0;
	sub_822C0000(ctx, base);
	// 82907EA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907EA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907EB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907EB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907EC0 size=196
    let mut pc: u32 = 0x82907EC0;
    'dispatch: loop {
        match pc {
            0x82907EC0 => {
    //   block [0x82907EC0..0x82907F84)
	// 82907EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907ED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907EDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907EE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907EE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907EE8: 4B9B8A51  bl 0x822c0938
	ctx.lr = 0x82907EEC;
	sub_822C0938(ctx, base);
	// 82907EEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907EF0: 41820028  beq 0x82907f18
	if ctx.cr[0].eq {
	pc = 0x82907F18; continue 'dispatch;
	}
	// 82907EF4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907EF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907EFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907F00: 392B5294  addi r9, r11, 0x5294
	ctx.r[9].s64 = ctx.r[11].s64 + 21140;
	// 82907F04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907F08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907F0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907F10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907F14: 48000008  b 0x82907f1c
	pc = 0x82907F1C; continue 'dispatch;
	// 82907F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907F1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907F20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907F24: 409A0044  bne cr6, 0x82907f68
	if !ctx.cr[6].eq {
	pc = 0x82907F68; continue 'dispatch;
	}
	// 82907F28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907F2C: 419A001C  beq cr6, 0x82907f48
	if ctx.cr[6].eq {
	pc = 0x82907F48; continue 'dispatch;
	}
	// 82907F30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907F34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82907F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82907F3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82907F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82907F44: 4E800421  bctrl
	ctx.lr = 0x82907F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82907F48: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82907F4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82907F50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82907F54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82907F58: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82907F5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82907F60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82907F64: 4B9B809D  bl 0x822c0000
	ctx.lr = 0x82907F68;
	sub_822C0000(ctx, base);
	// 82907F68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82907F6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82907F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82907F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82907F78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82907F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82907F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82907F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82907F88 size=196
    let mut pc: u32 = 0x82907F88;
    'dispatch: loop {
        match pc {
            0x82907F88 => {
    //   block [0x82907F88..0x8290804C)
	// 82907F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82907F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82907F90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82907F94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82907F98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82907F9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82907FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907FA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82907FA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82907FAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907FB0: 4B9B8989  bl 0x822c0938
	ctx.lr = 0x82907FB4;
	sub_822C0938(ctx, base);
	// 82907FB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82907FB8: 41820028  beq 0x82907fe0
	if ctx.cr[0].eq {
	pc = 0x82907FE0; continue 'dispatch;
	}
	// 82907FBC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82907FC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82907FC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82907FC8: 392B52A8  addi r9, r11, 0x52a8
	ctx.r[9].s64 = ctx.r[11].s64 + 21160;
	// 82907FCC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82907FD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82907FD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82907FD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82907FDC: 48000008  b 0x82907fe4
	pc = 0x82907FE4; continue 'dispatch;
	// 82907FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82907FE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82907FE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82907FEC: 409A0044  bne cr6, 0x82908030
	if !ctx.cr[6].eq {
	pc = 0x82908030; continue 'dispatch;
	}
	// 82907FF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82907FF4: 419A001C  beq cr6, 0x82908010
	if ctx.cr[6].eq {
	pc = 0x82908010; continue 'dispatch;
	}
	// 82907FF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82907FFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82908000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908004: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82908008: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290800C: 4E800421  bctrl
	ctx.lr = 0x82908010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82908010: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908014: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82908018: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290801C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82908020: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 82908024: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82908028: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290802C: 4B9B7FD5  bl 0x822c0000
	ctx.lr = 0x82908030;
	sub_822C0000(ctx, base);
	// 82908030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290803C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82908044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908050 size=196
    let mut pc: u32 = 0x82908050;
    'dispatch: loop {
        match pc {
            0x82908050 => {
    //   block [0x82908050..0x82908114)
	// 82908050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290805C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908064: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82908068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290806C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82908070: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82908074: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82908078: 4B9B88C1  bl 0x822c0938
	ctx.lr = 0x8290807C;
	sub_822C0938(ctx, base);
	// 8290807C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908080: 41820028  beq 0x829080a8
	if ctx.cr[0].eq {
	pc = 0x829080A8; continue 'dispatch;
	}
	// 82908084: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908088: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290808C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82908090: 392B52BC  addi r9, r11, 0x52bc
	ctx.r[9].s64 = ctx.r[11].s64 + 21180;
	// 82908094: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82908098: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290809C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 829080A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 829080A4: 48000008  b 0x829080ac
	pc = 0x829080AC; continue 'dispatch;
	// 829080A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829080AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829080B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829080B4: 409A0044  bne cr6, 0x829080f8
	if !ctx.cr[6].eq {
	pc = 0x829080F8; continue 'dispatch;
	}
	// 829080B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 829080BC: 419A001C  beq cr6, 0x829080d8
	if ctx.cr[6].eq {
	pc = 0x829080D8; continue 'dispatch;
	}
	// 829080C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829080C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 829080C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829080CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829080D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829080D4: 4E800421  bctrl
	ctx.lr = 0x829080D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829080D8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829080DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 829080E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829080E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 829080E8: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 829080EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 829080F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 829080F4: 4B9B7F0D  bl 0x822c0000
	ctx.lr = 0x829080F8;
	sub_822C0000(ctx, base);
	// 829080F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829080FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908108: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290810C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908118 size=72
    let mut pc: u32 = 0x82908118;
    'dispatch: loop {
        match pc {
            0x82908118 => {
    //   block [0x82908118..0x82908160)
	// 82908118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290811C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908124: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82908128: 419A001C  beq cr6, 0x82908144
	if ctx.cr[6].eq {
	pc = 0x82908144; continue 'dispatch;
	}
	// 8290812C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82908130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82908134: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82908138: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290813C: 4BFFF6A5  bl 0x829077e0
	ctx.lr = 0x82908140;
	sub_829077E0(ctx, base);
	// 82908140: 48000010  b 0x82908150
	pc = 0x82908150; continue 'dispatch;
	// 82908144: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908148: 396BD478  addi r11, r11, -0x2b88
	ctx.r[11].s64 = ctx.r[11].s64 + -11144;
	// 8290814C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82908150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82908154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290815C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908160 size=72
    let mut pc: u32 = 0x82908160;
    'dispatch: loop {
        match pc {
            0x82908160 => {
    //   block [0x82908160..0x829081A8)
	// 82908160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290816C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82908170: 419A001C  beq cr6, 0x8290818c
	if ctx.cr[6].eq {
	pc = 0x8290818C; continue 'dispatch;
	}
	// 82908174: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82908178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8290817C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82908180: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82908184: 4BFFF6E5  bl 0x82907868
	ctx.lr = 0x82908188;
	sub_82907868(ctx, base);
	// 82908188: 48000010  b 0x82908198
	pc = 0x82908198; continue 'dispatch;
	// 8290818C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908190: 396BD550  addi r11, r11, -0x2ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -10928;
	// 82908194: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82908198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290819C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829081A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829081A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829081A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829081A8 size=72
    let mut pc: u32 = 0x829081A8;
    'dispatch: loop {
        match pc {
            0x829081A8 => {
    //   block [0x829081A8..0x829081F0)
	// 829081A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829081AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829081B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829081B4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 829081B8: 419A001C  beq cr6, 0x829081d4
	if ctx.cr[6].eq {
	pc = 0x829081D4; continue 'dispatch;
	}
	// 829081BC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 829081C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 829081C4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 829081C8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829081CC: 4BFFF725  bl 0x829078f0
	ctx.lr = 0x829081D0;
	sub_829078F0(ctx, base);
	// 829081D0: 48000010  b 0x829081e0
	pc = 0x829081E0; continue 'dispatch;
	// 829081D4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829081D8: 396BD630  addi r11, r11, -0x29d0
	ctx.r[11].s64 = ctx.r[11].s64 + -10704;
	// 829081DC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829081E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829081E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829081E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829081EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829081F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829081F0 size=104
    let mut pc: u32 = 0x829081F0;
    'dispatch: loop {
        match pc {
            0x829081F0 => {
    //   block [0x829081F0..0x82908258)
	// 829081F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829081F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829081F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829081FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908200: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908208: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290820C: 808BD0A8  lwz r4, -0x2f58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12120 as u32) ) } as u64;
	// 82908210: 484EB7F9  bl 0x82df3a08
	ctx.lr = 0x82908214;
	sub_82DF3A08(ctx, base);
	// 82908214: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82908218: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8290821C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908220: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82908224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908228: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8290822C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82908230: 48556861  bl 0x82e5ea90
	ctx.lr = 0x82908234;
	sub_82E5EA90(ctx, base);
	// 82908234: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82908238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290823C: 419A0008  beq cr6, 0x82908244
	if ctx.cr[6].eq {
	pc = 0x82908244; continue 'dispatch;
	}
	// 82908240: 4B9B8651  bl 0x822c0890
	ctx.lr = 0x82908244;
	sub_822C0890(ctx, base);
	// 82908244: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290824C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82908258 size=264
    let mut pc: u32 = 0x82908258;
    'dispatch: loop {
        match pc {
            0x82908258 => {
    //   block [0x82908258..0x82908360)
	// 82908258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290825C: 4889FF11  bl 0x831a816c
	ctx.lr = 0x82908260;
	sub_831A8130(ctx, base);
	// 82908260: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908264: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82908268: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290826C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908270: 4800E4F9  bl 0x82916768
	ctx.lr = 0x82908274;
	sub_82916768(ctx, base);
	// 82908274: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82908278: 4182008C  beq 0x82908304
	if ctx.cr[0].eq {
	pc = 0x82908304; continue 'dispatch;
	}
	// 8290827C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82908280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908284: 388B1EDC  addi r4, r11, 0x1edc
	ctx.r[4].s64 = ctx.r[11].s64 + 7900;
	// 82908288: 484EB781  bl 0x82df3a08
	ctx.lr = 0x8290828C;
	sub_82DF3A08(ctx, base);
	// 8290828C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82908290: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82908294: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82908298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290829C: C02BA2EC  lfs f1, -0x5d14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23828 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829082A0: 480043E1  bl 0x8290c680
	ctx.lr = 0x829082A4;
	sub_8290C680(ctx, base);
	// 829082A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829082A8: 484EB181  bl 0x82df3428
	ctx.lr = 0x829082AC;
	sub_82DF3428(ctx, base);
	// 829082AC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829082B0: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 829082B4: C02B5228  lfs f1, 0x5228(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21032 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829082B8: 4800F159  bl 0x82917410
	ctx.lr = 0x829082BC;
	sub_82917410(ctx, base);
	// 829082BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829082C0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 829082C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829082C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 829082CC: 816B018C  lwz r11, 0x18c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 829082D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829082D4: 4E800421  bctrl
	ctx.lr = 0x829082D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829082D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 829082DC: 395E006C  addi r10, r30, 0x6c
	ctx.r[10].s64 = ctx.r[30].s64 + 108;
	// 829082E0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 829082E4: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 829082E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829082EC: 917E006C  stw r11, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 829082F0: 4B9BC171  bl 0x822c4460
	ctx.lr = 0x829082F4;
	sub_822C4460(ctx, base);
	// 829082F4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 829082F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829082FC: 419A0008  beq cr6, 0x82908304
	if ctx.cr[6].eq {
	pc = 0x82908304; continue 'dispatch;
	}
	// 82908300: 4B9B8591  bl 0x822c0890
	ctx.lr = 0x82908304;
	sub_822C0890(ctx, base);
	// 82908304: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82908308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290830C: 38AB2B00  addi r5, r11, 0x2b00
	ctx.r[5].s64 = ctx.r[11].s64 + 11008;
	// 82908310: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82908314: 4BEE79B5  bl 0x827efcc8
	ctx.lr = 0x82908318;
	sub_827EFCC8(ctx, base);
	// 82908318: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290831C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82908320: 419A0008  beq cr6, 0x82908328
	if ctx.cr[6].eq {
	pc = 0x82908328; continue 'dispatch;
	}
	// 82908324: 4B9B856D  bl 0x822c0890
	ctx.lr = 0x82908328;
	sub_822C0890(ctx, base);
	// 82908328: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290832C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908330: 808BF470  lwz r4, -0xb90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2960 as u32) ) } as u64;
	// 82908334: 484EB6D5  bl 0x82df3a08
	ctx.lr = 0x82908338;
	sub_82DF3A08(ctx, base);
	// 82908338: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290833C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82908340: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908344: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82908348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290834C: 4E800421  bctrl
	ctx.lr = 0x82908350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82908350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908354: 484EB0D5  bl 0x82df3428
	ctx.lr = 0x82908358;
	sub_82DF3428(ctx, base);
	// 82908358: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8290835C: 4889FE60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908360 size=128
    let mut pc: u32 = 0x82908360;
    'dispatch: loop {
        match pc {
            0x82908360 => {
    //   block [0x82908360..0x829083E0)
	// 82908360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908364: 4889FE09  bl 0x831a816c
	ctx.lr = 0x82908368;
	sub_831A8130(ctx, base);
	// 82908368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290836C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82908370: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82908374: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82908378: 3BEBB17C  addi r31, r11, -0x4e84
	ctx.r[31].s64 = ctx.r[11].s64 + -20100;
	// 8290837C: 816AB184  lwz r11, -0x4e7c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20092 as u32) ) } as u64;
	// 82908380: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82908384: 40820024  bne 0x829083a8
	if !ctx.cr[0].eq {
	pc = 0x829083A8; continue 'dispatch;
	}
	// 82908388: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8290838C: 3D008291  lis r8, -0x7d6f
	ctx.r[8].s64 = -2104426496;
	// 82908390: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82908394: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 82908398: 39088118  addi r8, r8, -0x7ee8
	ctx.r[8].s64 = ctx.r[8].s64 + -32488;
	// 8290839C: 916AB184  stw r11, -0x4e7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20092 as u32), ctx.r[11].u32 ) };
	// 829083A0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 829083A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 829083A8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 829083AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 829083B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829083B4: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 829083B8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 829083BC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829083C0: 4BD4C201  bl 0x826545c0
	ctx.lr = 0x829083C4;
	sub_826545C0(ctx, base);
	// 829083C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829083C8: 4182000C  beq 0x829083d4
	if ctx.cr[0].eq {
	pc = 0x829083D4; continue 'dispatch;
	}
	// 829083CC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 829083D0: 48000008  b 0x829083d8
	pc = 0x829083D8; continue 'dispatch;
	// 829083D4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 829083D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 829083DC: 4889FDE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829083E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829083E0 size=128
    let mut pc: u32 = 0x829083E0;
    'dispatch: loop {
        match pc {
            0x829083E0 => {
    //   block [0x829083E0..0x82908460)
	// 829083E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829083E4: 4889FD89  bl 0x831a816c
	ctx.lr = 0x829083E8;
	sub_831A8130(ctx, base);
	// 829083E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829083EC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 829083F0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829083F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829083F8: 3BEBB188  addi r31, r11, -0x4e78
	ctx.r[31].s64 = ctx.r[11].s64 + -20088;
	// 829083FC: 816AB190  lwz r11, -0x4e70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20080 as u32) ) } as u64;
	// 82908400: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82908404: 40820024  bne 0x82908428
	if !ctx.cr[0].eq {
	pc = 0x82908428; continue 'dispatch;
	}
	// 82908408: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8290840C: 3D008291  lis r8, -0x7d6f
	ctx.r[8].s64 = -2104426496;
	// 82908410: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82908414: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 82908418: 39088160  addi r8, r8, -0x7ea0
	ctx.r[8].s64 = ctx.r[8].s64 + -32416;
	// 8290841C: 916AB190  stw r11, -0x4e70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20080 as u32), ctx.r[11].u32 ) };
	// 82908420: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82908424: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82908428: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8290842C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82908430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908434: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82908438: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8290843C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82908440: 4BD4C181  bl 0x826545c0
	ctx.lr = 0x82908444;
	sub_826545C0(ctx, base);
	// 82908444: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82908448: 4182000C  beq 0x82908454
	if ctx.cr[0].eq {
	pc = 0x82908454; continue 'dispatch;
	}
	// 8290844C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908450: 48000008  b 0x82908458
	pc = 0x82908458; continue 'dispatch;
	// 82908454: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82908458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290845C: 4889FD60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908460 size=128
    let mut pc: u32 = 0x82908460;
    'dispatch: loop {
        match pc {
            0x82908460 => {
    //   block [0x82908460..0x829084E0)
	// 82908460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908464: 4889FD09  bl 0x831a816c
	ctx.lr = 0x82908468;
	sub_831A8130(ctx, base);
	// 82908468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290846C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82908470: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82908474: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82908478: 3BEBB194  addi r31, r11, -0x4e6c
	ctx.r[31].s64 = ctx.r[11].s64 + -20076;
	// 8290847C: 816AB19C  lwz r11, -0x4e64(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20068 as u32) ) } as u64;
	// 82908480: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82908484: 40820024  bne 0x829084a8
	if !ctx.cr[0].eq {
	pc = 0x829084A8; continue 'dispatch;
	}
	// 82908488: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8290848C: 3D008291  lis r8, -0x7d6f
	ctx.r[8].s64 = -2104426496;
	// 82908490: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82908494: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 82908498: 390881A8  addi r8, r8, -0x7e58
	ctx.r[8].s64 = ctx.r[8].s64 + -32344;
	// 8290849C: 916AB19C  stw r11, -0x4e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20068 as u32), ctx.r[11].u32 ) };
	// 829084A0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 829084A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 829084A8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 829084AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 829084B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829084B4: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 829084B8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 829084BC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829084C0: 4BD4C101  bl 0x826545c0
	ctx.lr = 0x829084C4;
	sub_826545C0(ctx, base);
	// 829084C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829084C8: 4182000C  beq 0x829084d4
	if ctx.cr[0].eq {
	pc = 0x829084D4; continue 'dispatch;
	}
	// 829084CC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 829084D0: 48000008  b 0x829084d8
	pc = 0x829084D8; continue 'dispatch;
	// 829084D4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 829084D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 829084DC: 4889FCE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829084E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829084E0 size=212
    let mut pc: u32 = 0x829084E0;
    'dispatch: loop {
        match pc {
            0x829084E0 => {
    //   block [0x829084E0..0x829085B4)
	// 829084E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829084E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829084E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829084EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829084F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829084F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 829084F8: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 829084FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82908500: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82908504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908508: 388B5D9C  addi r4, r11, 0x5d9c
	ctx.r[4].s64 = ctx.r[11].s64 + 23964;
	// 8290850C: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 82908510: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82908514: 4B9B7EC5  bl 0x822c03d8
	ctx.lr = 0x82908518;
	sub_822C03D8(ctx, base);
	// 82908518: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290851C: 41820040  beq 0x8290855c
	if ctx.cr[0].eq {
	pc = 0x8290855C; continue 'dispatch;
	}
	// 82908520: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82908524: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82908528: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290852C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908530: 394A52D0  addi r10, r10, 0x52d0
	ctx.r[10].s64 = ctx.r[10].s64 + 21200;
	// 82908534: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82908538: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290853C: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82908540: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82908544: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82908548: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8290854C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82908550: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82908554: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82908558: 48000008  b 0x82908560
	pc = 0x82908560; continue 'dispatch;
	// 8290855C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908560: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82908564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908568: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290856C: 4BFFFAE5  bl 0x82908050
	ctx.lr = 0x82908570;
	sub_82908050(ctx, base);
	// 82908570: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908574: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908578: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290857C: 4B9B7A85  bl 0x822c0000
	ctx.lr = 0x82908580;
	sub_822C0000(ctx, base);
	// 82908580: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82908584: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908588: 482AAB81  bl 0x82bb3108
	ctx.lr = 0x8290858C;
	sub_82BB3108(ctx, base);
	// 8290858C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82908590: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82908594: 419A0008  beq cr6, 0x8290859c
	if ctx.cr[6].eq {
	pc = 0x8290859C; continue 'dispatch;
	}
	// 82908598: 4B9B82F9  bl 0x822c0890
	ctx.lr = 0x8290859C;
	sub_822C0890(ctx, base);
	// 8290859C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829085A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829085A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829085A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829085AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829085B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829085B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829085B8 size=80
    let mut pc: u32 = 0x829085B8;
    'dispatch: loop {
        match pc {
            0x829085B8 => {
    //   block [0x829085B8..0x82908608)
	// 829085B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829085BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829085C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829085C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829085C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829085CC: 4855435D  bl 0x82e5c928
	ctx.lr = 0x829085D0;
	sub_82E5C928(ctx, base);
	// 829085D0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829085D4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 829085D8: 396B52DC  addi r11, r11, 0x52dc
	ctx.r[11].s64 = ctx.r[11].s64 + 21212;
	// 829085DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829085E0: 4BFC3541  bl 0x828cbb20
	ctx.lr = 0x829085E4;
	sub_828CBB20(ctx, base);
	// 829085E4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829085E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829085EC: 396B5314  addi r11, r11, 0x5314
	ctx.r[11].s64 = ctx.r[11].s64 + 21268;
	// 829085F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829085F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829085F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829085FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82908608 size=248
    let mut pc: u32 = 0x82908608;
    'dispatch: loop {
        match pc {
            0x82908608 => {
    //   block [0x82908608..0x82908700)
	// 82908608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290860C: 4889FB61  bl 0x831a816c
	ctx.lr = 0x82908610;
	sub_831A8130(ctx, base);
	// 82908610: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908618: 4BFFFFA1  bl 0x829085b8
	ctx.lr = 0x8290861C;
	sub_829085B8(ctx, base);
	// 8290861C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908620: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82908624: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82908628: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8290862C: 396B5354  addi r11, r11, 0x5354
	ctx.r[11].s64 = ctx.r[11].s64 + 21332;
	// 82908630: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 82908634: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82908638: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290863C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82908640: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82908644: 396A7260  addi r11, r10, 0x7260
	ctx.r[11].s64 = ctx.r[10].s64 + 29280;
	// 82908648: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 8290864C: 3BBF006C  addi r29, r31, 0x6c
	ctx.r[29].s64 = ctx.r[31].s64 + 108;
	// 82908650: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82908654: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82908658: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8290865C: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82908660: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82908664: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82908668: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8290866C: 93C10070  stw r30, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82908670: 4BFFFCF1  bl 0x82908360
	ctx.lr = 0x82908674;
	sub_82908360(ctx, base);
	// 82908674: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82908678: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290867C: C04B7BC8  lfs f2, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82908680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908684: C02A08A8  lfs f1, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82908688: 4BEED899  bl 0x827f5f20
	ctx.lr = 0x8290868C;
	sub_827F5F20(ctx, base);
	// 8290868C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908690: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82908694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908698: 388B5348  addi r4, r11, 0x5348
	ctx.r[4].s64 = ctx.r[11].s64 + 21320;
	// 8290869C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 829086A0: 4BFBDB51  bl 0x828c61f0
	ctx.lr = 0x829086A4;
	sub_828C61F0(ctx, base);
	// 829086A4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 829086A8: 4B9C0611  bl 0x822c8cb8
	ctx.lr = 0x829086AC;
	sub_822C8CB8(ctx, base);
	// 829086AC: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 829086B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829086B4: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 829086B8: 396B79C8  addi r11, r11, 0x79c8
	ctx.r[11].s64 = ctx.r[11].s64 + 31176;
	// 829086BC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 829086C0: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 829086C4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 829086C8: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 829086CC: 4BFFFE15  bl 0x829084e0
	ctx.lr = 0x829086D0;
	sub_829084E0(ctx, base);
	// 829086D0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829086D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829086D8: 808BD0A8  lwz r4, -0x2f58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12120 as u32) ) } as u64;
	// 829086DC: 484EB32D  bl 0x82df3a08
	ctx.lr = 0x829086E0;
	sub_82DF3A08(ctx, base);
	// 829086E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829086E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829086E8: 48551001  bl 0x82e596e8
	ctx.lr = 0x829086EC;
	sub_82E596E8(ctx, base);
	// 829086EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829086F0: 484EAD39  bl 0x82df3428
	ctx.lr = 0x829086F4;
	sub_82DF3428(ctx, base);
	// 829086F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829086F8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 829086FC: 4889FAC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82908700 size=192
    let mut pc: u32 = 0x82908700;
    'dispatch: loop {
        match pc {
            0x82908700 => {
    //   block [0x82908700..0x829087C0)
	// 82908700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290870C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908714: 4BFFFEA5  bl 0x829085b8
	ctx.lr = 0x82908718;
	sub_829085B8(ctx, base);
	// 82908718: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290871C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82908720: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 82908724: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82908728: 392B538C  addi r9, r11, 0x538c
	ctx.r[9].s64 = ctx.r[11].s64 + 21388;
	// 8290872C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82908730: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82908734: 394A74D8  addi r10, r10, 0x74d8
	ctx.r[10].s64 = ctx.r[10].s64 + 29912;
	// 82908738: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290873C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82908740: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82908744: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82908748: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290874C: 4BFFFC95  bl 0x829083e0
	ctx.lr = 0x82908750;
	sub_829083E0(ctx, base);
	// 82908750: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82908754: C04B7BC8  lfs f2, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82908758: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290875C: FC201090  fmr f1, f2
	ctx.f[1].f64 = ctx.f[2].f64;
	// 82908760: 4BEED7C1  bl 0x827f5f20
	ctx.lr = 0x82908764;
	sub_827F5F20(ctx, base);
	// 82908764: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82908768: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290876C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908770: 388BB1A8  addi r4, r11, -0x4e58
	ctx.r[4].s64 = ctx.r[11].s64 + -20056;
	// 82908774: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82908778: 4BFBDA79  bl 0x828c61f0
	ctx.lr = 0x8290877C;
	sub_828C61F0(ctx, base);
	// 8290877C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82908780: 4B9C0539  bl 0x822c8cb8
	ctx.lr = 0x82908784;
	sub_822C8CB8(ctx, base);
	// 82908784: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290878C: 808BD0AC  lwz r4, -0x2f54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12116 as u32) ) } as u64;
	// 82908790: 484EB279  bl 0x82df3a08
	ctx.lr = 0x82908794;
	sub_82DF3A08(ctx, base);
	// 82908794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908798: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290879C: 48550F4D  bl 0x82e596e8
	ctx.lr = 0x829087A0;
	sub_82E596E8(ctx, base);
	// 829087A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829087A4: 484EAC85  bl 0x82df3428
	ctx.lr = 0x829087A8;
	sub_82DF3428(ctx, base);
	// 829087A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829087AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 829087B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829087B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829087B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829087BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829087C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829087C0 size=196
    let mut pc: u32 = 0x829087C0;
    'dispatch: loop {
        match pc {
            0x829087C0 => {
    //   block [0x829087C0..0x82908884)
	// 829087C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829087C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829087C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829087CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829087D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829087D4: 4BFFFDE5  bl 0x829085b8
	ctx.lr = 0x829087D8;
	sub_829085B8(ctx, base);
	// 829087D8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829087DC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 829087E0: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 829087E4: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 829087E8: 392B53CC  addi r9, r11, 0x53cc
	ctx.r[9].s64 = ctx.r[11].s64 + 21452;
	// 829087EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829087F0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 829087F4: 394A75B0  addi r10, r10, 0x75b0
	ctx.r[10].s64 = ctx.r[10].s64 + 30128;
	// 829087F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829087FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82908800: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82908804: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82908808: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290880C: 4BFFFC55  bl 0x82908460
	ctx.lr = 0x82908810;
	sub_82908460(ctx, base);
	// 82908810: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82908814: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82908818: C04B7BC8  lfs f2, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8290881C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908820: C02AA1C4  lfs f1, -0x5e3c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82908824: 4BEED6FD  bl 0x827f5f20
	ctx.lr = 0x82908828;
	sub_827F5F20(ctx, base);
	// 82908828: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290882C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82908830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908834: 388B53C0  addi r4, r11, 0x53c0
	ctx.r[4].s64 = ctx.r[11].s64 + 21440;
	// 82908838: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8290883C: 4BFBD9B5  bl 0x828c61f0
	ctx.lr = 0x82908840;
	sub_828C61F0(ctx, base);
	// 82908840: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82908844: 4B9C0475  bl 0x822c8cb8
	ctx.lr = 0x82908848;
	sub_822C8CB8(ctx, base);
	// 82908848: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290884C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908850: 808BD0B0  lwz r4, -0x2f50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12112 as u32) ) } as u64;
	// 82908854: 484EB1B5  bl 0x82df3a08
	ctx.lr = 0x82908858;
	sub_82DF3A08(ctx, base);
	// 82908858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290885C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82908860: 48550E89  bl 0x82e596e8
	ctx.lr = 0x82908864;
	sub_82E596E8(ctx, base);
	// 82908864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908868: 484EABC1  bl 0x82df3428
	ctx.lr = 0x8290886C;
	sub_82DF3428(ctx, base);
	// 8290886C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908870: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82908874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290887C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908888 size=96
    let mut pc: u32 = 0x82908888;
    'dispatch: loop {
        match pc {
            0x82908888 => {
    //   block [0x82908888..0x829088E8)
	// 82908888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908894: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290889C: 4BFFFD1D  bl 0x829085b8
	ctx.lr = 0x829088A0;
	sub_829085B8(ctx, base);
	// 829088A0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829088A4: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 829088A8: 396B5404  addi r11, r11, 0x5404
	ctx.r[11].s64 = ctx.r[11].s64 + 21508;
	// 829088AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829088B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829088B4: 808A0B34  lwz r4, 0xb34(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2868 as u32) ) } as u64;
	// 829088B8: 484EB151  bl 0x82df3a08
	ctx.lr = 0x829088BC;
	sub_82DF3A08(ctx, base);
	// 829088BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829088C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829088C4: 48550E25  bl 0x82e596e8
	ctx.lr = 0x829088C8;
	sub_82E596E8(ctx, base);
	// 829088C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829088CC: 484EAB5D  bl 0x82df3428
	ctx.lr = 0x829088D0;
	sub_82DF3428(ctx, base);
	// 829088D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829088D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829088D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829088DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829088E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829088E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829088E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829088E8 size=96
    let mut pc: u32 = 0x829088E8;
    'dispatch: loop {
        match pc {
            0x829088E8 => {
    //   block [0x829088E8..0x82908948)
	// 829088E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829088EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829088F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829088F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829088F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829088FC: 4BFFFCBD  bl 0x829085b8
	ctx.lr = 0x82908900;
	sub_829085B8(ctx, base);
	// 82908900: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908904: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82908908: 396B543C  addi r11, r11, 0x543c
	ctx.r[11].s64 = ctx.r[11].s64 + 21564;
	// 8290890C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908910: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82908914: 808A0B70  lwz r4, 0xb70(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2928 as u32) ) } as u64;
	// 82908918: 484EB0F1  bl 0x82df3a08
	ctx.lr = 0x8290891C;
	sub_82DF3A08(ctx, base);
	// 8290891C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908920: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82908924: 48550DC5  bl 0x82e596e8
	ctx.lr = 0x82908928;
	sub_82E596E8(ctx, base);
	// 82908928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290892C: 484EAAFD  bl 0x82df3428
	ctx.lr = 0x82908930;
	sub_82DF3428(ctx, base);
	// 82908930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290893C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908948 size=108
    let mut pc: u32 = 0x82908948;
    'dispatch: loop {
        match pc {
            0x82908948 => {
    //   block [0x82908948..0x829089B4)
	// 82908948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290894C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908954: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290895C: 4BFFFC5D  bl 0x829085b8
	ctx.lr = 0x82908960;
	sub_829085B8(ctx, base);
	// 82908960: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82908964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82908968: 394A5474  addi r10, r10, 0x5474
	ctx.r[10].s64 = ctx.r[10].s64 + 21620;
	// 8290896C: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82908970: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82908974: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908978: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8290897C: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82908980: 80890B58  lwz r4, 0xb58(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2904 as u32) ) } as u64;
	// 82908984: 484EB085  bl 0x82df3a08
	ctx.lr = 0x82908988;
	sub_82DF3A08(ctx, base);
	// 82908988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290898C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82908990: 48550D59  bl 0x82e596e8
	ctx.lr = 0x82908994;
	sub_82E596E8(ctx, base);
	// 82908994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908998: 484EAA91  bl 0x82df3428
	ctx.lr = 0x8290899C;
	sub_82DF3428(ctx, base);
	// 8290899C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829089A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829089A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829089A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829089AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829089B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829089B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829089B8 size=96
    let mut pc: u32 = 0x829089B8;
    'dispatch: loop {
        match pc {
            0x829089B8 => {
    //   block [0x829089B8..0x82908A18)
	// 829089B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829089BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829089C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829089C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829089C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829089CC: 4BFFFBED  bl 0x829085b8
	ctx.lr = 0x829089D0;
	sub_829085B8(ctx, base);
	// 829089D0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829089D4: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 829089D8: 396B54AC  addi r11, r11, 0x54ac
	ctx.r[11].s64 = ctx.r[11].s64 + 21676;
	// 829089DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829089E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829089E4: 808A0AF8  lwz r4, 0xaf8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2808 as u32) ) } as u64;
	// 829089E8: 484EB021  bl 0x82df3a08
	ctx.lr = 0x829089EC;
	sub_82DF3A08(ctx, base);
	// 829089EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829089F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829089F4: 48550CF5  bl 0x82e596e8
	ctx.lr = 0x829089F8;
	sub_82E596E8(ctx, base);
	// 829089F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829089FC: 484EAA2D  bl 0x82df3428
	ctx.lr = 0x82908A00;
	sub_82DF3428(ctx, base);
	// 82908A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908A04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908A10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908A18 size=112
    let mut pc: u32 = 0x82908A18;
    'dispatch: loop {
        match pc {
            0x82908A18 => {
    //   block [0x82908A18..0x82908A88)
	// 82908A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908A1C: 4889F751  bl 0x831a816c
	ctx.lr = 0x82908A20;
	sub_831A8130(ctx, base);
	// 82908A20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908A24: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908A28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908A30: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908A34: 38A00056  li r5, 0x56
	ctx.r[5].s64 = 86;
	// 82908A38: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82908A3C: 484E99AD  bl 0x82df23e8
	ctx.lr = 0x82908A40;
	sub_82DF23E8(ctx, base);
	// 82908A40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908A44: 41820010  beq 0x82908a54
	if ctx.cr[0].eq {
	pc = 0x82908A54; continue 'dispatch;
	}
	// 82908A48: 4BFFFBC1  bl 0x82908608
	ctx.lr = 0x82908A4C;
	sub_82908608(ctx, base);
	// 82908A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908A50: 48000008  b 0x82908a58
	pc = 0x82908A58; continue 'dispatch;
	// 82908A54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908A58: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908A5C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908A60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908A68: 4BFFF139  bl 0x82907ba0
	ctx.lr = 0x82908A6C;
	sub_82907BA0(ctx, base);
	// 82908A6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908A70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908A78: 4B9B7589  bl 0x822c0000
	ctx.lr = 0x82908A7C;
	sub_822C0000(ctx, base);
	// 82908A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908A80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908A84: 4889F738  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908A88 size=112
    let mut pc: u32 = 0x82908A88;
    'dispatch: loop {
        match pc {
            0x82908A88 => {
    //   block [0x82908A88..0x82908AF8)
	// 82908A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908A8C: 4889F6E1  bl 0x831a816c
	ctx.lr = 0x82908A90;
	sub_831A8130(ctx, base);
	// 82908A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908A94: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908A98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908AA0: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908AA4: 38A000C1  li r5, 0xc1
	ctx.r[5].s64 = 193;
	// 82908AA8: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 82908AAC: 484E993D  bl 0x82df23e8
	ctx.lr = 0x82908AB0;
	sub_82DF23E8(ctx, base);
	// 82908AB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908AB4: 41820010  beq 0x82908ac4
	if ctx.cr[0].eq {
	pc = 0x82908AC4; continue 'dispatch;
	}
	// 82908AB8: 4BFFFC49  bl 0x82908700
	ctx.lr = 0x82908ABC;
	sub_82908700(ctx, base);
	// 82908ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908AC0: 48000008  b 0x82908ac8
	pc = 0x82908AC8; continue 'dispatch;
	// 82908AC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908AC8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908ACC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908AD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908AD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908AD8: 4BFFF191  bl 0x82907c68
	ctx.lr = 0x82908ADC;
	sub_82907C68(ctx, base);
	// 82908ADC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908AE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908AE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908AE8: 4B9B7519  bl 0x822c0000
	ctx.lr = 0x82908AEC;
	sub_822C0000(ctx, base);
	// 82908AEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908AF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908AF4: 4889F6C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908AF8 size=112
    let mut pc: u32 = 0x82908AF8;
    'dispatch: loop {
        match pc {
            0x82908AF8 => {
    //   block [0x82908AF8..0x82908B68)
	// 82908AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908AFC: 4889F671  bl 0x831a816c
	ctx.lr = 0x82908B00;
	sub_831A8130(ctx, base);
	// 82908B00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908B04: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908B08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908B10: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908B14: 38A000F4  li r5, 0xf4
	ctx.r[5].s64 = 244;
	// 82908B18: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 82908B1C: 484E98CD  bl 0x82df23e8
	ctx.lr = 0x82908B20;
	sub_82DF23E8(ctx, base);
	// 82908B20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908B24: 41820010  beq 0x82908b34
	if ctx.cr[0].eq {
	pc = 0x82908B34; continue 'dispatch;
	}
	// 82908B28: 4BFFFC99  bl 0x829087c0
	ctx.lr = 0x82908B2C;
	sub_829087C0(ctx, base);
	// 82908B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908B30: 48000008  b 0x82908b38
	pc = 0x82908B38; continue 'dispatch;
	// 82908B34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908B38: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908B3C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908B40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908B44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908B48: 4BFFF1E9  bl 0x82907d30
	ctx.lr = 0x82908B4C;
	sub_82907D30(ctx, base);
	// 82908B4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908B50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908B54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908B58: 4B9B74A9  bl 0x822c0000
	ctx.lr = 0x82908B5C;
	sub_822C0000(ctx, base);
	// 82908B5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908B60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908B64: 4889F658  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908B68 size=112
    let mut pc: u32 = 0x82908B68;
    'dispatch: loop {
        match pc {
            0x82908B68 => {
    //   block [0x82908B68..0x82908BD8)
	// 82908B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908B6C: 4889F601  bl 0x831a816c
	ctx.lr = 0x82908B70;
	sub_831A8130(ctx, base);
	// 82908B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908B74: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908B78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908B80: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908B84: 38A00127  li r5, 0x127
	ctx.r[5].s64 = 295;
	// 82908B88: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 82908B8C: 484E985D  bl 0x82df23e8
	ctx.lr = 0x82908B90;
	sub_82DF23E8(ctx, base);
	// 82908B90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908B94: 41820010  beq 0x82908ba4
	if ctx.cr[0].eq {
	pc = 0x82908BA4; continue 'dispatch;
	}
	// 82908B98: 4BFFFCF1  bl 0x82908888
	ctx.lr = 0x82908B9C;
	sub_82908888(ctx, base);
	// 82908B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908BA0: 48000008  b 0x82908ba8
	pc = 0x82908BA8; continue 'dispatch;
	// 82908BA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908BA8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908BAC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908BB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908BB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908BB8: 4BFFF241  bl 0x82907df8
	ctx.lr = 0x82908BBC;
	sub_82907DF8(ctx, base);
	// 82908BBC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908BC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908BC8: 4B9B7439  bl 0x822c0000
	ctx.lr = 0x82908BCC;
	sub_822C0000(ctx, base);
	// 82908BCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908BD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908BD4: 4889F5E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908BD8 size=112
    let mut pc: u32 = 0x82908BD8;
    'dispatch: loop {
        match pc {
            0x82908BD8 => {
    //   block [0x82908BD8..0x82908C48)
	// 82908BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908BDC: 4889F591  bl 0x831a816c
	ctx.lr = 0x82908BE0;
	sub_831A8130(ctx, base);
	// 82908BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908BE4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908BE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908BF0: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908BF4: 38A00154  li r5, 0x154
	ctx.r[5].s64 = 340;
	// 82908BF8: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 82908BFC: 484E97ED  bl 0x82df23e8
	ctx.lr = 0x82908C00;
	sub_82DF23E8(ctx, base);
	// 82908C00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908C04: 41820010  beq 0x82908c14
	if ctx.cr[0].eq {
	pc = 0x82908C14; continue 'dispatch;
	}
	// 82908C08: 4BFFFCE1  bl 0x829088e8
	ctx.lr = 0x82908C0C;
	sub_829088E8(ctx, base);
	// 82908C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908C10: 48000008  b 0x82908c18
	pc = 0x82908C18; continue 'dispatch;
	// 82908C14: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908C18: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908C1C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908C20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908C24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908C28: 4BFFF299  bl 0x82907ec0
	ctx.lr = 0x82908C2C;
	sub_82907EC0(ctx, base);
	// 82908C2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908C30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908C34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908C38: 4B9B73C9  bl 0x822c0000
	ctx.lr = 0x82908C3C;
	sub_822C0000(ctx, base);
	// 82908C3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908C40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908C44: 4889F578  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908C48 size=112
    let mut pc: u32 = 0x82908C48;
    'dispatch: loop {
        match pc {
            0x82908C48 => {
    //   block [0x82908C48..0x82908CB8)
	// 82908C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908C4C: 4889F521  bl 0x831a816c
	ctx.lr = 0x82908C50;
	sub_831A8130(ctx, base);
	// 82908C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908C54: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908C58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908C60: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908C64: 38A0018D  li r5, 0x18d
	ctx.r[5].s64 = 397;
	// 82908C68: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 82908C6C: 484E977D  bl 0x82df23e8
	ctx.lr = 0x82908C70;
	sub_82DF23E8(ctx, base);
	// 82908C70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908C74: 41820010  beq 0x82908c84
	if ctx.cr[0].eq {
	pc = 0x82908C84; continue 'dispatch;
	}
	// 82908C78: 4BFFFCD1  bl 0x82908948
	ctx.lr = 0x82908C7C;
	sub_82908948(ctx, base);
	// 82908C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908C80: 48000008  b 0x82908c88
	pc = 0x82908C88; continue 'dispatch;
	// 82908C84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908C88: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908C8C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908C90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908C98: 4BFFF2F1  bl 0x82907f88
	ctx.lr = 0x82908C9C;
	sub_82907F88(ctx, base);
	// 82908C9C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908CA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908CA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908CA8: 4B9B7359  bl 0x822c0000
	ctx.lr = 0x82908CAC;
	sub_822C0000(ctx, base);
	// 82908CAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908CB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908CB4: 4889F508  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908CB8 size=112
    let mut pc: u32 = 0x82908CB8;
    'dispatch: loop {
        match pc {
            0x82908CB8 => {
    //   block [0x82908CB8..0x82908D28)
	// 82908CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908CBC: 4889F4B1  bl 0x831a816c
	ctx.lr = 0x82908CC0;
	sub_831A8130(ctx, base);
	// 82908CC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908CC4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82908CC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82908CD0: 388B54E0  addi r4, r11, 0x54e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21728;
	// 82908CD4: 38A0003B  li r5, 0x3b
	ctx.r[5].s64 = 59;
	// 82908CD8: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 82908CDC: 484E970D  bl 0x82df23e8
	ctx.lr = 0x82908CE0;
	sub_82DF23E8(ctx, base);
	// 82908CE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82908CE4: 41820010  beq 0x82908cf4
	if ctx.cr[0].eq {
	pc = 0x82908CF4; continue 'dispatch;
	}
	// 82908CE8: 4BFFFCD1  bl 0x829089b8
	ctx.lr = 0x82908CEC;
	sub_829089B8(ctx, base);
	// 82908CEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908CF0: 48000008  b 0x82908cf8
	pc = 0x82908CF8; continue 'dispatch;
	// 82908CF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82908CF8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82908CFC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82908D00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908D08: 4BFFEDD1  bl 0x82907ad8
	ctx.lr = 0x82908D0C;
	sub_82907AD8(ctx, base);
	// 82908D0C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82908D10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82908D18: 4B9B72E9  bl 0x822c0000
	ctx.lr = 0x82908D1C;
	sub_822C0000(ctx, base);
	// 82908D1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82908D20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908D24: 4889F498  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908D28 size=76
    let mut pc: u32 = 0x82908D28;
    'dispatch: loop {
        match pc {
            0x82908D28 => {
    //   block [0x82908D28..0x82908D74)
	// 82908D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908D38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908D3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82908D40: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82908D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82908D48: 4E800421  bctrl
	ctx.lr = 0x82908D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82908D4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82908D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908D54: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82908D58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82908D5C: 4E800421  bctrl
	ctx.lr = 0x82908D60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82908D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82908D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908D6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908D78 size=112
    let mut pc: u32 = 0x82908D78;
    'dispatch: loop {
        match pc {
            0x82908D78 => {
    //   block [0x82908D78..0x82908DE8)
	// 82908D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908D80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82908D84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908D88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908D8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908D90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82908D94: 4854D635  bl 0x82e563c8
	ctx.lr = 0x82908D98;
	sub_82E563C8(ctx, base);
	// 82908D98: 57DE063E  clrlwi r30, r30, 0x18
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82908D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908DA0: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82908DA4: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82908DA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82908DAC: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82908DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82908DB4: 4E800421  bctrl
	ctx.lr = 0x82908DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82908DB8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82908DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908DC0: 419A000C  beq cr6, 0x82908dcc
	if ctx.cr[6].eq {
	pc = 0x82908DCC; continue 'dispatch;
	}
	// 82908DC4: 4BC08EC5  bl 0x82511c88
	ctx.lr = 0x82908DC8;
	sub_82511C88(ctx, base);
	// 82908DC8: 48000008  b 0x82908dd0
	pc = 0x82908DD0; continue 'dispatch;
	// 82908DCC: 4BC08EDD  bl 0x82511ca8
	ctx.lr = 0x82908DD0;
	sub_82511CA8(ctx, base);
	// 82908DD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82908DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908DDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82908DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908DE8 size=8
    let mut pc: u32 = 0x82908DE8;
    'dispatch: loop {
        match pc {
            0x82908DE8 => {
    //   block [0x82908DE8..0x82908DF0)
	// 82908DE8: 80630244  lwz r3, 0x244(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(580 as u32) ) } as u64;
	// 82908DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908DF0 size=20
    let mut pc: u32 = 0x82908DF0;
    'dispatch: loop {
        match pc {
            0x82908DF0 => {
    //   block [0x82908DF0..0x82908E04)
	// 82908DF0: 81630244  lwz r11, 0x244(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(580 as u32) ) } as u64;
	// 82908DF4: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82908DF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82908DFC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82908E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908E08 size=60
    let mut pc: u32 = 0x82908E08;
    'dispatch: loop {
        match pc {
            0x82908E08 => {
    //   block [0x82908E08..0x82908E44)
	// 82908E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908E10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908E14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908E18: 80630244  lwz r3, 0x244(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(580 as u32) ) } as u64;
	// 82908E1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82908E20: 48019B79  bl 0x82922998
	ctx.lr = 0x82908E24;
	sub_82922998(ctx, base);
	// 82908E24: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82908E28: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82908E2C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82908E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82908E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908E48 size=8
    let mut pc: u32 = 0x82908E48;
    'dispatch: loop {
        match pc {
            0x82908E48 => {
    //   block [0x82908E48..0x82908E50)
	// 82908E48: 806302D0  lwz r3, 0x2d0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(720 as u32) ) } as u64;
	// 82908E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908E50 size=8
    let mut pc: u32 = 0x82908E50;
    'dispatch: loop {
        match pc {
            0x82908E50 => {
    //   block [0x82908E50..0x82908E58)
	// 82908E50: 806302E0  lwz r3, 0x2e0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(736 as u32) ) } as u64;
	// 82908E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908E58 size=8
    let mut pc: u32 = 0x82908E58;
    'dispatch: loop {
        match pc {
            0x82908E58 => {
    //   block [0x82908E58..0x82908E60)
	// 82908E58: C023032C  lfs f1, 0x32c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(812 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82908E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908E60 size=8
    let mut pc: u32 = 0x82908E60;
    'dispatch: loop {
        match pc {
            0x82908E60 => {
    //   block [0x82908E60..0x82908E68)
	// 82908E60: C0230330  lfs f1, 0x330(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(816 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82908E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908E68 size=24
    let mut pc: u32 = 0x82908E68;
    'dispatch: loop {
        match pc {
            0x82908E68 => {
    //   block [0x82908E68..0x82908E80)
	// 82908E68: C003032C  lfs f0, 0x32c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82908E6C: EDA1002A  fadds f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82908E70: C0030330  lfs f0, 0x330(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(816 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82908E74: D1A3032C  stfs f13, 0x32c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 82908E78: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82908E7C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908E80 size=8
    let mut pc: u32 = 0x82908E80;
    'dispatch: loop {
        match pc {
            0x82908E80 => {
    //   block [0x82908E80..0x82908E88)
	// 82908E80: D003032C  stfs f0, 0x32c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 82908E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908E88 size=28
    let mut pc: u32 = 0x82908E88;
    'dispatch: loop {
        match pc {
            0x82908E88 => {
    //   block [0x82908E88..0x82908EA4)
	// 82908E88: C003032C  lfs f0, 0x32c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82908E8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82908E90: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82908E94: D1A3032C  stfs f13, 0x32c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 82908E98: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82908E9C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82908EA0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908EA4 size=8
    let mut pc: u32 = 0x82908EA4;
    'dispatch: loop {
        match pc {
            0x82908EA4 => {
    //   block [0x82908EA4..0x82908EAC)
	// 82908EA4: D003032C  stfs f0, 0x32c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 82908EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908EB0 size=4
    let mut pc: u32 = 0x82908EB0;
    'dispatch: loop {
        match pc {
            0x82908EB0 => {
    //   block [0x82908EB0..0x82908EB4)
	// 82908EB0: 4BC09210  b 0x825120c0
	sub_825120C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908EB8 size=8
    let mut pc: u32 = 0x82908EB8;
    'dispatch: loop {
        match pc {
            0x82908EB8 => {
    //   block [0x82908EB8..0x82908EC0)
	// 82908EB8: 98830354  stb r4, 0x354(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(852 as u32), ctx.r[4].u8 ) };
	// 82908EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908EC0 size=8
    let mut pc: u32 = 0x82908EC0;
    'dispatch: loop {
        match pc {
            0x82908EC0 => {
    //   block [0x82908EC0..0x82908EC8)
	// 82908EC0: 88630354  lbz r3, 0x354(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(852 as u32) ) } as u64;
	// 82908EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82908EC8 size=8
    let mut pc: u32 = 0x82908EC8;
    'dispatch: loop {
        match pc {
            0x82908EC8 => {
    //   block [0x82908EC8..0x82908ED0)
	// 82908EC8: 38630270  addi r3, r3, 0x270
	ctx.r[3].s64 = ctx.r[3].s64 + 624;
	// 82908ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82908ED0 size=8
    let mut pc: u32 = 0x82908ED0;
    'dispatch: loop {
        match pc {
            0x82908ED0 => {
    //   block [0x82908ED0..0x82908ED8)
	// 82908ED0: C0230270  lfs f1, 0x270(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(624 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82908ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908ED8 size=52
    let mut pc: u32 = 0x82908ED8;
    'dispatch: loop {
        match pc {
            0x82908ED8 => {
    //   block [0x82908ED8..0x82908F0C)
	// 82908ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908EE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82908EEC: 4BEE32DD  bl 0x827ec1c8
	ctx.lr = 0x82908EF0;
	sub_827EC1C8(ctx, base);
	// 82908EF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908EF4: 4BB864AD  bl 0x8248f3a0
	ctx.lr = 0x82908EF8;
	sub_8248F3A0(ctx, base);
	// 82908EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82908EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82908F10 size=52
    let mut pc: u32 = 0x82908F10;
    'dispatch: loop {
        match pc {
            0x82908F10 => {
    //   block [0x82908F10..0x82908F44)
	// 82908F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82908F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82908F1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908F20: 83E30248  lwz r31, 0x248(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(584 as u32) ) } as u64;
	// 82908F24: 4BEE32A5  bl 0x827ec1c8
	ctx.lr = 0x82908F28;
	sub_827EC1C8(ctx, base);
	// 82908F28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82908F2C: 4BB86475  bl 0x8248f3a0
	ctx.lr = 0x82908F30;
	sub_8248F3A0(ctx, base);
	// 82908F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82908F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82908F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82908F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82908F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82908F48 size=176
    let mut pc: u32 = 0x82908F48;
    'dispatch: loop {
        match pc {
            0x82908F48 => {
    //   block [0x82908F48..0x82908FF8)
	// 82908F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908F4C: 4889F21D  bl 0x831a8168
	ctx.lr = 0x82908F50;
	sub_831A8130(ctx, base);
	// 82908F50: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82908F54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82908F58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82908F5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82908F60: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82908F64: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82908F68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82908F6C: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82908F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82908F74: 4E800421  bctrl
	ctx.lr = 0x82908F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82908F78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82908F7C: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82908F80: 48019A19  bl 0x82922998
	ctx.lr = 0x82908F84;
	sub_82922998(ctx, base);
	// 82908F84: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82908F88: 40820010  bne 0x82908f98
	if !ctx.cr[0].eq {
	pc = 0x82908F98; continue 'dispatch;
	}
	// 82908F8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82908F90: C3EB9450  lfs f31, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82908F94: 4800000C  b 0x82908fa0
	pc = 0x82908FA0; continue 'dispatch;
	// 82908F98: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82908F9C: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82908FA0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82908FA4: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82908FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908FAC: 41820010  beq 0x82908fbc
	if ctx.cr[0].eq {
	pc = 0x82908FBC; continue 'dispatch;
	}
	// 82908FB0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908FB4: 396BD9C8  addi r11, r11, -0x2638
	ctx.r[11].s64 = ctx.r[11].s64 + -9784;
	// 82908FB8: 4800000C  b 0x82908fc4
	pc = 0x82908FC4; continue 'dispatch;
	// 82908FBC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82908FC0: 396BD9C0  addi r11, r11, -0x2640
	ctx.r[11].s64 = ctx.r[11].s64 + -9792;
	// 82908FC4: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82908FC8: 484EAA41  bl 0x82df3a08
	ctx.lr = 0x82908FCC;
	sub_82DF3A08(ctx, base);
	// 82908FCC: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82908FD0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82908FD4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82908FD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82908FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82908FE0: 4BEE3221  bl 0x827ec200
	ctx.lr = 0x82908FE4;
	sub_827EC200(ctx, base);
	// 82908FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82908FE8: 484EA441  bl 0x82df3428
	ctx.lr = 0x82908FEC;
	sub_82DF3428(ctx, base);
	// 82908FEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82908FF0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82908FF4: 4889F1C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82908FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82908FF8 size=104
    let mut pc: u32 = 0x82908FF8;
    'dispatch: loop {
        match pc {
            0x82908FF8 => {
    //   block [0x82908FF8..0x82909060)
	// 82908FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82908FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82909004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290900C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82909010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909018: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290901C: 808BD9D0  lwz r4, -0x2630(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9776 as u32) ) } as u64;
	// 82909020: 484EA9E9  bl 0x82df3a08
	ctx.lr = 0x82909024;
	sub_82DF3A08(ctx, base);
	// 82909024: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909028: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290902C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82909030: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82909034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909038: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290903C: 4BEE31C5  bl 0x827ec200
	ctx.lr = 0x82909040;
	sub_827EC200(ctx, base);
	// 82909040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909044: 484EA3E5  bl 0x82df3428
	ctx.lr = 0x82909048;
	sub_82DF3428(ctx, base);
	// 82909048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290904C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909054: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290905C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909060 size=16
    let mut pc: u32 = 0x82909060;
    'dispatch: loop {
        match pc {
            0x82909060 => {
    //   block [0x82909060..0x82909070)
	// 82909060: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82909064: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82909068: 409A0008  bne cr6, 0x82909070
	if !ctx.cr[6].eq {
		sub_82909070(ctx, base);
		return;
	}
	// 8290906C: 4BC07564  b 0x825105d0
	sub_825105D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909070 size=16
    let mut pc: u32 = 0x82909070;
    'dispatch: loop {
        match pc {
            0x82909070 => {
    //   block [0x82909070..0x82909080)
	// 82909070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909074: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82909078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290907C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909080 size=12
    let mut pc: u32 = 0x82909080;
    'dispatch: loop {
        match pc {
            0x82909080 => {
    //   block [0x82909080..0x8290908C)
	// 82909080: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82909084: 99630338  stb r11, 0x338(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(824 as u32), ctx.r[11].u8 ) };
	// 82909088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909090 size=296
    let mut pc: u32 = 0x82909090;
    'dispatch: loop {
        match pc {
            0x82909090 => {
    //   block [0x82909090..0x829091B8)
	// 82909090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290909C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829090A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829090A4: 4BEE3D5D  bl 0x827ece00
	ctx.lr = 0x829090A8;
	sub_827ECE00(ctx, base);
	// 829090A8: 484EA109  bl 0x82df31b0
	ctx.lr = 0x829090AC;
	sub_82DF31B0(ctx, base);
	// 829090AC: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 829090B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 829090B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 829090B8: 991F0019  stb r8, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[8].u8 ) };
	// 829090BC: 814A0B7C  lwz r10, 0xb7c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2940 as u32) ) } as u64;
	// 829090C0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829090C4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 829090C8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 829090CC: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 829090D0: 41820014  beq 0x829090e4
	if ctx.cr[0].eq {
	pc = 0x829090E4; continue 'dispatch;
	}
	// 829090D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 829090D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 829090DC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 829090E0: 419AFFE0  beq cr6, 0x829090c0
	if ctx.cr[6].eq {
	pc = 0x829090C0; continue 'dispatch;
	}
	// 829090E4: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 829090E8: 418200B8  beq 0x829091a0
	if ctx.cr[0].eq {
	pc = 0x829091A0; continue 'dispatch;
	}
	// 829090EC: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 829090F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 829090F4: 814A0B78  lwz r10, 0xb78(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2936 as u32) ) } as u64;
	// 829090F8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829090FC: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909100: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909104: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82909108: 41820014  beq 0x8290911c
	if ctx.cr[0].eq {
	pc = 0x8290911C; continue 'dispatch;
	}
	// 8290910C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82909110: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82909114: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82909118: 419AFFE0  beq cr6, 0x829090f8
	if ctx.cr[6].eq {
	pc = 0x829090F8; continue 'dispatch;
	}
	// 8290911C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909120: 41820080  beq 0x829091a0
	if ctx.cr[0].eq {
	pc = 0x829091A0; continue 'dispatch;
	}
	// 82909124: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82909128: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290912C: 814A0B90  lwz r10, 0xb90(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2960 as u32) ) } as u64;
	// 82909130: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909134: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909138: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290913C: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82909140: 41820014  beq 0x82909154
	if ctx.cr[0].eq {
	pc = 0x82909154; continue 'dispatch;
	}
	// 82909144: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82909148: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290914C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82909150: 419AFFE0  beq cr6, 0x82909130
	if ctx.cr[6].eq {
	pc = 0x82909130; continue 'dispatch;
	}
	// 82909154: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909158: 41820048  beq 0x829091a0
	if ctx.cr[0].eq {
	pc = 0x829091A0; continue 'dispatch;
	}
	// 8290915C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82909160: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82909164: 814A0B94  lwz r10, 0xb94(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2964 as u32) ) } as u64;
	// 82909168: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290916C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909170: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909174: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82909178: 41820014  beq 0x8290918c
	if ctx.cr[0].eq {
	pc = 0x8290918C; continue 'dispatch;
	}
	// 8290917C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82909180: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82909184: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82909188: 419AFFE0  beq cr6, 0x82909168
	if ctx.cr[6].eq {
	pc = 0x82909168; continue 'dispatch;
	}
	// 8290918C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909190: 41820010  beq 0x829091a0
	if ctx.cr[0].eq {
	pc = 0x829091A0; continue 'dispatch;
	}
	// 82909194: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82909198: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 8290919C: 48000008  b 0x829091a4
	pc = 0x829091A4; continue 'dispatch;
	// 829091A0: 991F0018  stb r8, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u8 ) };
	// 829091A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829091A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829091AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829091B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829091B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829091B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829091B8 size=64
    let mut pc: u32 = 0x829091B8;
    'dispatch: loop {
        match pc {
            0x829091B8 => {
    //   block [0x829091B8..0x829091F8)
	// 829091B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829091BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829091C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829091C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829091C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829091CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829091D0: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 829091D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829091D8: 4E800421  bctrl
	ctx.lr = 0x829091DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829091DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829091E0: 4BC073F1  bl 0x825105d0
	ctx.lr = 0x829091E4;
	sub_825105D0(ctx, base);
	// 829091E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829091E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829091EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829091F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829091F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829091F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x829091F8 size=12
    let mut pc: u32 = 0x829091F8;
    'dispatch: loop {
        match pc {
            0x829091F8 => {
    //   block [0x829091F8..0x82909204)
	// 829091F8: C0030270  lfs f0, 0x270(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829091FC: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82909200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909208 size=8
    let mut pc: u32 = 0x82909208;
    'dispatch: loop {
        match pc {
            0x82909208 => {
    //   block [0x82909208..0x82909210)
	// 82909208: 908302E0  stw r4, 0x2e0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(736 as u32), ctx.r[4].u32 ) };
	// 8290920C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909210 size=80
    let mut pc: u32 = 0x82909210;
    'dispatch: loop {
        match pc {
            0x82909210 => {
    //   block [0x82909210..0x82909260)
	// 82909210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290921C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909220: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82909224: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82909228: 3BEBB1A0  addi r31, r11, -0x4e60
	ctx.r[31].s64 = ctx.r[11].s64 + -20064;
	// 8290922C: 816AB1AC  lwz r11, -0x4e54(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20052 as u32) ) } as u64;
	// 82909230: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82909234: 40820014  bne 0x82909248
	if !ctx.cr[0].eq {
	pc = 0x82909248; continue 'dispatch;
	}
	// 82909238: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8290923C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909240: 916AB1AC  stw r11, -0x4e54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20052 as u32), ctx.r[11].u32 ) };
	// 82909244: 480521B5  bl 0x8295b3f8
	ctx.lr = 0x82909248;
	sub_8295B3F8(ctx, base);
	// 82909248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290924C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82909250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290925C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909260 size=88
    let mut pc: u32 = 0x82909260;
    'dispatch: loop {
        match pc {
            0x82909260 => {
    //   block [0x82909260..0x829092B8)
	// 82909260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290926C: 80630244  lwz r3, 0x244(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(580 as u32) ) } as u64;
	// 82909270: 48019729  bl 0x82922998
	ctx.lr = 0x82909274;
	sub_82922998(ctx, base);
	// 82909274: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82909278: 41980028  blt cr6, 0x829092a0
	if ctx.cr[6].lt {
	pc = 0x829092A0; continue 'dispatch;
	}
	// 8290927C: 419A0018  beq cr6, 0x82909294
	if ctx.cr[6].eq {
	pc = 0x82909294; continue 'dispatch;
	}
	// 82909280: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82909284: 409A001C  bne cr6, 0x829092a0
	if !ctx.cr[6].eq {
	pc = 0x829092A0; continue 'dispatch;
	}
	// 82909288: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8290928C: C02BD7BC  lfs f1, -0x2844(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82909290: 48000018  b 0x829092a8
	pc = 0x829092A8; continue 'dispatch;
	// 82909294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82909298: C02BA1C4  lfs f1, -0x5e3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290929C: 4800000C  b 0x829092a8
	pc = 0x829092A8; continue 'dispatch;
	// 829092A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829092A4: C02B08A8  lfs f1, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829092A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829092AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829092B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829092B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829092B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x829092B8 size=8
    let mut pc: u32 = 0x829092B8;
    'dispatch: loop {
        match pc {
            0x829092B8 => {
    //   block [0x829092B8..0x829092C0)
	// 829092B8: D0230334  stfs f1, 0x334(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(820 as u32), tmp.u32 ) };
	// 829092BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829092C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829092C0 size=16
    let mut pc: u32 = 0x829092C0;
    'dispatch: loop {
        match pc {
            0x829092C0 => {
    //   block [0x829092C0..0x829092D0)
	// 829092C0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829092C4: 816C0104  lwz r11, 0x104(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(260 as u32) ) } as u64;
	// 829092C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829092CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829092D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829092D0 size=16
    let mut pc: u32 = 0x829092D0;
    'dispatch: loop {
        match pc {
            0x829092D0 => {
    //   block [0x829092D0..0x829092E0)
	// 829092D0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829092D4: 816C0114  lwz r11, 0x114(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(276 as u32) ) } as u64;
	// 829092D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829092DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829092E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829092E0 size=16
    let mut pc: u32 = 0x829092E0;
    'dispatch: loop {
        match pc {
            0x829092E0 => {
    //   block [0x829092E0..0x829092F0)
	// 829092E0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829092E4: 816C0124  lwz r11, 0x124(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(292 as u32) ) } as u64;
	// 829092E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829092EC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829092F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829092F0 size=16
    let mut pc: u32 = 0x829092F0;
    'dispatch: loop {
        match pc {
            0x829092F0 => {
    //   block [0x829092F0..0x82909300)
	// 829092F0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829092F4: 816C0134  lwz r11, 0x134(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(308 as u32) ) } as u64;
	// 829092F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829092FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909300 size=16
    let mut pc: u32 = 0x82909300;
    'dispatch: loop {
        match pc {
            0x82909300 => {
    //   block [0x82909300..0x82909310)
	// 82909300: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909304: 816C0108  lwz r11, 0x108(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(264 as u32) ) } as u64;
	// 82909308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290930C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909310 size=16
    let mut pc: u32 = 0x82909310;
    'dispatch: loop {
        match pc {
            0x82909310 => {
    //   block [0x82909310..0x82909320)
	// 82909310: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909314: 816C0118  lwz r11, 0x118(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(280 as u32) ) } as u64;
	// 82909318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290931C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909320 size=16
    let mut pc: u32 = 0x82909320;
    'dispatch: loop {
        match pc {
            0x82909320 => {
    //   block [0x82909320..0x82909330)
	// 82909320: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909324: 816C0128  lwz r11, 0x128(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(296 as u32) ) } as u64;
	// 82909328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290932C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909330 size=16
    let mut pc: u32 = 0x82909330;
    'dispatch: loop {
        match pc {
            0x82909330 => {
    //   block [0x82909330..0x82909340)
	// 82909330: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909334: 816C0138  lwz r11, 0x138(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(312 as u32) ) } as u64;
	// 82909338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290933C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909340 size=16
    let mut pc: u32 = 0x82909340;
    'dispatch: loop {
        match pc {
            0x82909340 => {
    //   block [0x82909340..0x82909350)
	// 82909340: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909344: 816C010C  lwz r11, 0x10c(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(268 as u32) ) } as u64;
	// 82909348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290934C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909350 size=16
    let mut pc: u32 = 0x82909350;
    'dispatch: loop {
        match pc {
            0x82909350 => {
    //   block [0x82909350..0x82909360)
	// 82909350: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909354: 816C011C  lwz r11, 0x11c(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(284 as u32) ) } as u64;
	// 82909358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290935C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909360 size=16
    let mut pc: u32 = 0x82909360;
    'dispatch: loop {
        match pc {
            0x82909360 => {
    //   block [0x82909360..0x82909370)
	// 82909360: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909364: 816C012C  lwz r11, 0x12c(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(300 as u32) ) } as u64;
	// 82909368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290936C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909370 size=16
    let mut pc: u32 = 0x82909370;
    'dispatch: loop {
        match pc {
            0x82909370 => {
    //   block [0x82909370..0x82909380)
	// 82909370: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909374: 816C013C  lwz r11, 0x13c(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(316 as u32) ) } as u64;
	// 82909378: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290937C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909380 size=16
    let mut pc: u32 = 0x82909380;
    'dispatch: loop {
        match pc {
            0x82909380 => {
    //   block [0x82909380..0x82909390)
	// 82909380: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909384: 816C0110  lwz r11, 0x110(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(272 as u32) ) } as u64;
	// 82909388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290938C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909390 size=16
    let mut pc: u32 = 0x82909390;
    'dispatch: loop {
        match pc {
            0x82909390 => {
    //   block [0x82909390..0x829093A0)
	// 82909390: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909394: 816C0120  lwz r11, 0x120(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(288 as u32) ) } as u64;
	// 82909398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290939C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829093A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829093A0 size=16
    let mut pc: u32 = 0x829093A0;
    'dispatch: loop {
        match pc {
            0x829093A0 => {
    //   block [0x829093A0..0x829093B0)
	// 829093A0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829093A4: 816C0130  lwz r11, 0x130(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(304 as u32) ) } as u64;
	// 829093A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829093AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829093B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829093B0 size=80
    let mut pc: u32 = 0x829093B0;
    'dispatch: loop {
        match pc {
            0x829093B0 => {
    //   block [0x829093B0..0x82909400)
	// 829093B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829093B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829093B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829093BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829093C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829093C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829093C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829093CC: 4BEE2D9D  bl 0x827ec168
	ctx.lr = 0x829093D0;
	sub_827EC168(ctx, base);
	// 829093D0: 807F0258  lwz r3, 0x258(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 829093D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829093D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829093DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 829093E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829093E4: 4E800421  bctrl
	ctx.lr = 0x829093E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829093E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829093EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829093F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829093F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829093F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829093FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909400 size=16
    let mut pc: u32 = 0x82909400;
    'dispatch: loop {
        match pc {
            0x82909400 => {
    //   block [0x82909400..0x82909410)
	// 82909400: 89630348  lbz r11, 0x348(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(840 as u32) ) } as u64;
	// 82909404: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82909408: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8290940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909410 size=116
    let mut pc: u32 = 0x82909410;
    'dispatch: loop {
        match pc {
            0x82909410 => {
    //   block [0x82909410..0x82909484)
	// 82909410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290941C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290942C: 4BC06715  bl 0x8250fb40
	ctx.lr = 0x82909430;
	sub_8250FB40(ctx, base);
	// 82909430: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82909434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909438: 4182001C  beq 0x82909454
	if ctx.cr[0].eq {
	pc = 0x82909454; continue 'dispatch;
	}
	// 8290943C: 4BEE2E45  bl 0x827ec280
	ctx.lr = 0x82909440;
	sub_827EC280(ctx, base);
	// 82909440: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 82909444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909448: 419A0024  beq cr6, 0x8290946c
	if ctx.cr[6].eq {
	pc = 0x8290946C; continue 'dispatch;
	}
	// 8290944C: 48017FA5  bl 0x829213f0
	ctx.lr = 0x82909450;
	sub_829213F0(ctx, base);
	// 82909450: 4800001C  b 0x8290946c
	pc = 0x8290946C; continue 'dispatch;
	// 82909454: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82909458: 4BEE2E19  bl 0x827ec270
	ctx.lr = 0x8290945C;
	sub_827EC270(ctx, base);
	// 8290945C: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 82909460: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909464: 419A0008  beq cr6, 0x8290946c
	if ctx.cr[6].eq {
	pc = 0x8290946C; continue 'dispatch;
	}
	// 82909468: 48017F71  bl 0x829213d8
	ctx.lr = 0x8290946C;
	sub_829213D8(ctx, base);
	// 8290946C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909478: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290947C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909488 size=92
    let mut pc: u32 = 0x82909488;
    'dispatch: loop {
        match pc {
            0x82909488 => {
    //   block [0x82909488..0x829094E4)
	// 82909488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290948C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909494: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909498: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290949C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829094A0: 4BC06029  bl 0x8250f4c8
	ctx.lr = 0x829094A4;
	sub_8250F4C8(ctx, base);
	// 829094A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829094A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829094AC: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 829094B0: 409A0008  bne cr6, 0x829094b8
	if !ctx.cr[6].eq {
	pc = 0x829094B8; continue 'dispatch;
	}
	// 829094B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 829094B8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 829094BC: 4BBFF1C5  bl 0x82508680
	ctx.lr = 0x829094C0;
	sub_82508680(ctx, base);
	// 829094C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829094C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829094C8: 484E87C9  bl 0x82df1c90
	ctx.lr = 0x829094CC;
	sub_82DF1C90(ctx, base);
	// 829094CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829094D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829094D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829094D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829094DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829094E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829094E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829094E8 size=124
    let mut pc: u32 = 0x829094E8;
    'dispatch: loop {
        match pc {
            0x829094E8 => {
    //   block [0x829094E8..0x82909564)
	// 829094E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829094EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829094F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829094F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829094F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829094FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909504: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82909508: 4854D559  bl 0x82e56a60
	ctx.lr = 0x8290950C;
	sub_82E56A60(ctx, base);
	// 8290950C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82909510: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 82909514: 809F02D0  lwz r4, 0x2d0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as u64;
	// 82909518: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290951C: 3BEBDFEC  addi r31, r11, -0x2014
	ctx.r[31].s64 = ctx.r[11].s64 + -8212;
	// 82909520: 3BCA2258  addi r30, r10, 0x2258
	ctx.r[30].s64 = ctx.r[10].s64 + 8792;
	// 82909524: 48557E3D  bl 0x82e61360
	ctx.lr = 0x82909528;
	sub_82E61360(ctx, base);
	// 82909528: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290952C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82909530: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82909534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82909538: 488A0A11  bl 0x831a9f48
	ctx.lr = 0x8290953C;
	sub_831A9F48(ctx, base);
	// 8290953C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909544: 484E874D  bl 0x82df1c90
	ctx.lr = 0x82909548;
	sub_82DF1C90(ctx, base);
	// 82909548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290954C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909558: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290955C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82909568 size=24
    let mut pc: u32 = 0x82909568;
    'dispatch: loop {
        match pc {
            0x82909568 => {
    //   block [0x82909568..0x82909580)
	// 82909568: C0030330  lfs f0, 0x330(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(816 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290956C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82909570: 40980008  bge cr6, 0x82909578
	if !ctx.cr[6].lt {
	pc = 0x82909578; continue 'dispatch;
	}
	// 82909574: FC000890  fmr f0, f1
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82909578: D003032C  stfs f0, 0x32c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 8290957C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909580 size=8
    let mut pc: u32 = 0x82909580;
    'dispatch: loop {
        match pc {
            0x82909580 => {
    //   block [0x82909580..0x82909588)
	// 82909580: 80630250  lwz r3, 0x250(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(592 as u32) ) } as u64;
	// 82909584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909588 size=8
    let mut pc: u32 = 0x82909588;
    'dispatch: loop {
        match pc {
            0x82909588 => {
    //   block [0x82909588..0x82909590)
	// 82909588: 80630258  lwz r3, 0x258(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(600 as u32) ) } as u64;
	// 8290958C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909590 size=72
    let mut pc: u32 = 0x82909590;
    'dispatch: loop {
        match pc {
            0x82909590 => {
    //   block [0x82909590..0x829095D8)
	// 82909590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909598: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8290959C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829095A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829095A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 829095A8: 4BBE14E1  bl 0x824eaa88
	ctx.lr = 0x829095AC;
	sub_824EAA88(ctx, base);
	// 829095AC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829095B0: 4BBE02A1  bl 0x824e9850
	ctx.lr = 0x829095B4;
	sub_824E9850(ctx, base);
	// 829095B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829095B8: EFE107F2  fmuls f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 829095BC: 484E86D5  bl 0x82df1c90
	ctx.lr = 0x829095C0;
	sub_82DF1C90(ctx, base);
	// 829095C0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 829095C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829095C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829095CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829095D0: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829095D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829095D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829095D8 size=128
    let mut pc: u32 = 0x829095D8;
    'dispatch: loop {
        match pc {
            0x829095D8 => {
    //   block [0x829095D8..0x82909658)
	// 829095D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829095DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829095E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829095E4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 829095E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829095EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829095F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829095F4: 4BBE1495  bl 0x824eaa88
	ctx.lr = 0x829095F8;
	sub_824EAA88(ctx, base);
	// 829095F8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829095FC: 4BBE0255  bl 0x824e9850
	ctx.lr = 0x82909600;
	sub_824E9850(ctx, base);
	// 82909600: 7BEB0020  clrldi r11, r31, 0x20
	ctx.r[11].u64 = ctx.r[31].u64 & 0x00000000FFFFFFFFu64;
	// 82909604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909608: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8290960C: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82909610: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82909614: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82909618: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8290961C: 484E8675  bl 0x82df1c90
	ctx.lr = 0x82909620;
	sub_82DF1C90(ctx, base);
	// 82909620: FC00FE5E  fctidz f0, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82909624: 395FFFFF  addi r10, r31, -1
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	// 82909628: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8290962C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82909630: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82909634: 40980008  bge cr6, 0x8290963c
	if !ctx.cr[6].lt {
	pc = 0x8290963C; continue 'dispatch;
	}
	// 82909638: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8290963C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82909640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82909644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290964C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82909658 size=16
    let mut pc: u32 = 0x82909658;
    'dispatch: loop {
        match pc {
            0x82909658 => {
    //   block [0x82909658..0x82909668)
	// 82909658: 39600280  li r11, 0x280
	ctx.r[11].s64 = 640;
	// 8290965C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909668 size=240
    let mut pc: u32 = 0x82909668;
    'dispatch: loop {
        match pc {
            0x82909668 => {
    //   block [0x82909668..0x82909758)
	// 82909668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290966C: 4889EAF9  bl 0x831a8164
	ctx.lr = 0x82909670;
	sub_831A8130(ctx, base);
	// 82909670: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909678: 897F01E4  lbz r11, 0x1e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(484 as u32) ) } as u64;
	// 8290967C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82909680: 408200D0  bne 0x82909750
	if !ctx.cr[0].eq {
	pc = 0x82909750; continue 'dispatch;
	}
	// 82909684: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82909688: 83CB666C  lwz r30, 0x666c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26220 as u32) ) } as u64;
	// 8290968C: 4BEE2B3D  bl 0x827ec1c8
	ctx.lr = 0x82909690;
	sub_827EC1C8(ctx, base);
	// 82909690: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82909694: 4BB85D0D  bl 0x8248f3a0
	ctx.lr = 0x82909698;
	sub_8248F3A0(ctx, base);
	// 82909698: 83DF01DC  lwz r30, 0x1dc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290969C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 829096A0: 419A00A8  beq cr6, 0x82909748
	if ctx.cr[6].eq {
	pc = 0x82909748; continue 'dispatch;
	}
	// 829096A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829096A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829096AC: 4BC05E6D  bl 0x8250f518
	ctx.lr = 0x829096B0;
	sub_8250F518(ctx, base);
	// 829096B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829096B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829096B8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 829096BC: 409A0008  bne cr6, 0x829096c4
	if !ctx.cr[6].eq {
	pc = 0x829096C4; continue 'dispatch;
	}
	// 829096C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 829096C4: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 829096C8: 4BC1E979  bl 0x82528040
	ctx.lr = 0x829096CC;
	sub_82528040(ctx, base);
	// 829096CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829096D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 829096D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829096D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 829096DC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 829096E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 829096E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829096E8: 4E800421  bctrl
	ctx.lr = 0x829096EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829096EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829096F0: 484E85A1  bl 0x82df1c90
	ctx.lr = 0x829096F4;
	sub_82DF1C90(ctx, base);
	// 829096F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829096F8: 4BEE2AD9  bl 0x827ec1d0
	ctx.lr = 0x829096FC;
	sub_827EC1D0(ctx, base);
	// 829096FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82909700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909704: 83BF01DC  lwz r29, 0x1dc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909708: 48109F81  bl 0x82a13688
	ctx.lr = 0x8290970C;
	sub_82A13688(ctx, base);
	// 8290970C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82909710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909714: 48109F6D  bl 0x82a13680
	ctx.lr = 0x82909718;
	sub_82A13680(ctx, base);
	// 82909718: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290971C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82909720: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82909724: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290972C: 4E800421  bctrl
	ctx.lr = 0x82909730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909730: 481DA161  bl 0x82ae3890
	ctx.lr = 0x82909734;
	sub_82AE3890(ctx, base);
	// 82909734: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82909738: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290973C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82909740: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82909744: 4B9EAB15  bl 0x822f4258
	ctx.lr = 0x82909748;
	sub_822F4258(ctx, base);
	// 82909748: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290974C: 997F01E4  stb r11, 0x1e4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[11].u8 ) };
	// 82909750: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82909754: 4889EA60  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909758 size=100
    let mut pc: u32 = 0x82909758;
    'dispatch: loop {
        match pc {
            0x82909758 => {
    //   block [0x82909758..0x829097BC)
	// 82909758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290976C: 897F01F0  lbz r11, 0x1f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 82909770: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82909774: 41820034  beq 0x829097a8
	if ctx.cr[0].eq {
	pc = 0x829097A8; continue 'dispatch;
	}
	// 82909778: 4BEE2A51  bl 0x827ec1c8
	ctx.lr = 0x8290977C;
	sub_827EC1C8(ctx, base);
	// 8290977C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82909780: 4182000C  beq 0x8290978c
	if ctx.cr[0].eq {
	pc = 0x8290978C; continue 'dispatch;
	}
	// 82909784: 809F0248  lwz r4, 0x248(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 82909788: 4BB85C19  bl 0x8248f3a0
	ctx.lr = 0x8290978C;
	sub_8248F3A0(ctx, base);
	// 8290978C: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82909790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909794: 419A000C  beq cr6, 0x829097a0
	if ctx.cr[6].eq {
	pc = 0x829097A0; continue 'dispatch;
	}
	// 82909798: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290979C: 4BB85BAD  bl 0x8248f348
	ctx.lr = 0x829097A0;
	sub_8248F348(ctx, base);
	// 829097A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829097A4: 997F01F0  stb r11, 0x1f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[11].u8 ) };
	// 829097A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829097AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829097B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829097B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829097B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829097C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829097C0 size=84
    let mut pc: u32 = 0x829097C0;
    'dispatch: loop {
        match pc {
            0x829097C0 => {
    //   block [0x829097C0..0x82909814)
	// 829097C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829097C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829097C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829097CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829097D0: 808401DC  lwz r4, 0x1dc(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(476 as u32) ) } as u64;
	// 829097D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829097D8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 829097DC: 419A0014  beq cr6, 0x829097f0
	if ctx.cr[6].eq {
	pc = 0x829097F0; continue 'dispatch;
	}
	// 829097E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 829097E4: 4B9EA485  bl 0x822f3c68
	ctx.lr = 0x829097E8;
	sub_822F3C68(ctx, base);
	// 829097E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829097EC: 48000014  b 0x82909800
	pc = 0x82909800; continue 'dispatch;
	// 829097F0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829097F4: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 829097F8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909818 size=108
    let mut pc: u32 = 0x82909818;
    'dispatch: loop {
        match pc {
            0x82909818 => {
    //   block [0x82909818..0x82909884)
	// 82909818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909828: 808401DC  lwz r4, 0x1dc(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290982C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909830: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82909834: 419A0014  beq cr6, 0x82909848
	if ctx.cr[6].eq {
	pc = 0x82909848; continue 'dispatch;
	}
	// 82909838: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290983C: 4B9EA4AD  bl 0x822f3ce8
	ctx.lr = 0x82909840;
	sub_822F3CE8(ctx, base);
	// 82909840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909844: 4800002C  b 0x82909870
	pc = 0x82909870; continue 'dispatch;
	// 82909848: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8290984C: 394B6900  addi r10, r11, 0x6900
	ctx.r[10].s64 = ctx.r[11].s64 + 26880;
	// 82909850: C00B6900  lfs f0, 0x6900(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909854: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82909858: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290985C: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82909860: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909864: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82909868: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290986C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82909870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82909874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290987C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909888 size=112
    let mut pc: u32 = 0x82909888;
    'dispatch: loop {
        match pc {
            0x82909888 => {
    //   block [0x82909888..0x829098F8)
	// 82909888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909898: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8290989C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829098A0: 808B01DC  lwz r4, 0x1dc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(476 as u32) ) } as u64;
	// 829098A4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 829098A8: 419A0010  beq cr6, 0x829098b8
	if ctx.cr[6].eq {
	pc = 0x829098B8; continue 'dispatch;
	}
	// 829098AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 829098B0: 4B9EA3F9  bl 0x822f3ca8
	ctx.lr = 0x829098B4;
	sub_822F3CA8(ctx, base);
	// 829098B4: 4800002C  b 0x829098e0
	pc = 0x829098E0; continue 'dispatch;
	// 829098B8: 808B01E8  lwz r4, 0x1e8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(488 as u32) ) } as u64;
	// 829098BC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 829098C0: 419A0010  beq cr6, 0x829098d0
	if ctx.cr[6].eq {
	pc = 0x829098D0; continue 'dispatch;
	}
	// 829098C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829098C8: 4B9E36D1  bl 0x822ecf98
	ctx.lr = 0x829098CC;
	sub_822ECF98(ctx, base);
	// 829098CC: 48000014  b 0x829098e0
	pc = 0x829098E0; continue 'dispatch;
	// 829098D0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829098D4: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 829098D8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829098F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829098F8 size=96
    let mut pc: u32 = 0x829098F8;
    'dispatch: loop {
        match pc {
            0x829098F8 => {
    //   block [0x829098F8..0x82909958)
	// 829098F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829098FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82909904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290990C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82909914: 807F01DC  lwz r3, 0x1dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909918: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290991C: 419A0010  beq cr6, 0x8290992c
	if ctx.cr[6].eq {
	pc = 0x8290992C; continue 'dispatch;
	}
	// 82909920: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82909924: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82909928: 4B9EA4E1  bl 0x822f3e08
	ctx.lr = 0x8290992C;
	sub_822F3E08(ctx, base);
	// 8290992C: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82909930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909934: 419A000C  beq cr6, 0x82909940
	if ctx.cr[6].eq {
	pc = 0x82909940; continue 'dispatch;
	}
	// 82909938: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290993C: 4B9E3A0D  bl 0x822ed348
	ctx.lr = 0x82909940;
	sub_822ED348(ctx, base);
	// 82909940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290994C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909958 size=16
    let mut pc: u32 = 0x82909958;
    'dispatch: loop {
        match pc {
            0x82909958 => {
    //   block [0x82909958..0x82909968)
	// 82909958: 806301DC  lwz r3, 0x1dc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290995C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82909960: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909964: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909968 size=8
    let mut pc: u32 = 0x82909968;
    'dispatch: loop {
        match pc {
            0x82909968 => {
    //   block [0x82909968..0x82909970)
	// 82909968: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290996C: 4B9EA7AC  b 0x822f4118
	sub_822F4118(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909970 size=4
    let mut pc: u32 = 0x82909970;
    'dispatch: loop {
        match pc {
            0x82909970 => {
    //   block [0x82909970..0x82909974)
	// 82909970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909978 size=16
    let mut pc: u32 = 0x82909978;
    'dispatch: loop {
        match pc {
            0x82909978 => {
    //   block [0x82909978..0x82909988)
	// 82909978: 806301DC  lwz r3, 0x1dc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290997C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82909980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909984: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909988 size=8
    let mut pc: u32 = 0x82909988;
    'dispatch: loop {
        match pc {
            0x82909988 => {
    //   block [0x82909988..0x82909990)
	// 82909988: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290998C: 4B9EA83C  b 0x822f41c8
	sub_822F41C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909990 size=4
    let mut pc: u32 = 0x82909990;
    'dispatch: loop {
        match pc {
            0x82909990 => {
    //   block [0x82909990..0x82909994)
	// 82909990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909998 size=88
    let mut pc: u32 = 0x82909998;
    'dispatch: loop {
        match pc {
            0x82909998 => {
    //   block [0x82909998..0x829099F0)
	// 82909998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829099A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829099A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829099A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829099AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829099B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829099B4: 807F01DC  lwz r3, 0x1dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 829099B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829099BC: 419A0008  beq cr6, 0x829099c4
	if ctx.cr[6].eq {
	pc = 0x829099C4; continue 'dispatch;
	}
	// 829099C0: 4B9EA3B1  bl 0x822f3d70
	ctx.lr = 0x829099C4;
	sub_822F3D70(ctx, base);
	// 829099C4: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 829099C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829099CC: 419A000C  beq cr6, 0x829099d8
	if ctx.cr[6].eq {
	pc = 0x829099D8; continue 'dispatch;
	}
	// 829099D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829099D4: 4B9E377D  bl 0x822ed150
	ctx.lr = 0x829099D8;
	sub_822ED150(ctx, base);
	// 829099D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829099DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829099E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829099E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829099E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829099EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829099F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829099F0 size=88
    let mut pc: u32 = 0x829099F0;
    'dispatch: loop {
        match pc {
            0x829099F0 => {
    //   block [0x829099F0..0x82909A48)
	// 829099F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829099F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829099F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829099FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909A08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82909A0C: 807F01DC  lwz r3, 0x1dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909A10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909A14: 419A0008  beq cr6, 0x82909a1c
	if ctx.cr[6].eq {
	pc = 0x82909A1C; continue 'dispatch;
	}
	// 82909A18: 4B9EA429  bl 0x822f3e40
	ctx.lr = 0x82909A1C;
	sub_822F3E40(ctx, base);
	// 82909A1C: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82909A20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909A24: 419A000C  beq cr6, 0x82909a30
	if ctx.cr[6].eq {
	pc = 0x82909A30; continue 'dispatch;
	}
	// 82909A28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82909A2C: 4B9E35B5  bl 0x822ecfe0
	ctx.lr = 0x82909A30;
	sub_822ECFE0(ctx, base);
	// 82909A30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909A3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909A40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909A48 size=72
    let mut pc: u32 = 0x82909A48;
    'dispatch: loop {
        match pc {
            0x82909A48 => {
    //   block [0x82909A48..0x82909A90)
	// 82909A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909A4C: 4889E721  bl 0x831a816c
	ctx.lr = 0x82909A50;
	sub_831A8130(ctx, base);
	// 82909A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909A54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82909A58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82909A5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82909A60: 807D01DC  lwz r3, 0x1dc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909A64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909A68: 419A0008  beq cr6, 0x82909a70
	if ctx.cr[6].eq {
	pc = 0x82909A70; continue 'dispatch;
	}
	// 82909A6C: 4B9EA455  bl 0x822f3ec0
	ctx.lr = 0x82909A70;
	sub_822F3EC0(ctx, base);
	// 82909A70: 807D01E8  lwz r3, 0x1e8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(488 as u32) ) } as u64;
	// 82909A74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909A78: 419A0010  beq cr6, 0x82909a88
	if ctx.cr[6].eq {
	pc = 0x82909A88; continue 'dispatch;
	}
	// 82909A7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82909A80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82909A84: 4B9E35AD  bl 0x822ed030
	ctx.lr = 0x82909A88;
	sub_822ED030(ctx, base);
	// 82909A88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909A8C: 4889E730  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909A90 size=88
    let mut pc: u32 = 0x82909A90;
    'dispatch: loop {
        match pc {
            0x82909A90 => {
    //   block [0x82909A90..0x82909AE8)
	// 82909A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82909A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909AA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82909AAC: 807F01DC  lwz r3, 0x1dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909AB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909AB4: 419A0008  beq cr6, 0x82909abc
	if ctx.cr[6].eq {
	pc = 0x82909ABC; continue 'dispatch;
	}
	// 82909AB8: 4B9B6549  bl 0x822c0000
	ctx.lr = 0x82909ABC;
	sub_822C0000(ctx, base);
	// 82909ABC: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82909AC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909AC4: 419A000C  beq cr6, 0x82909ad0
	if ctx.cr[6].eq {
	pc = 0x82909AD0; continue 'dispatch;
	}
	// 82909AC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82909ACC: 4B9E35D5  bl 0x822ed0a0
	ctx.lr = 0x82909AD0;
	sub_822ED0A0(ctx, base);
	// 82909AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909ADC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909AE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909AE8 size=88
    let mut pc: u32 = 0x82909AE8;
    'dispatch: loop {
        match pc {
            0x82909AE8 => {
    //   block [0x82909AE8..0x82909B40)
	// 82909AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909AF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909AF4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82909AF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909B00: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82909B04: 807F01DC  lwz r3, 0x1dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909B0C: 419A0008  beq cr6, 0x82909b14
	if ctx.cr[6].eq {
	pc = 0x82909B14; continue 'dispatch;
	}
	// 82909B10: 4B9EA4E9  bl 0x822f3ff8
	ctx.lr = 0x82909B14;
	sub_822F3FF8(ctx, base);
	// 82909B14: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82909B18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909B1C: 419A000C  beq cr6, 0x82909b28
	if ctx.cr[6].eq {
	pc = 0x82909B28; continue 'dispatch;
	}
	// 82909B20: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82909B24: 4B9E4035  bl 0x822edb58
	ctx.lr = 0x82909B28;
	sub_822EDB58(ctx, base);
	// 82909B28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909B34: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909B40 size=12
    let mut pc: u32 = 0x82909B40;
    'dispatch: loop {
        match pc {
            0x82909B40 => {
    //   block [0x82909B40..0x82909B4C)
	// 82909B40: 806301DC  lwz r3, 0x1dc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(476 as u32) ) } as u64;
	// 82909B44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82909B48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909B4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909B4C size=8
    let mut pc: u32 = 0x82909B4C;
    'dispatch: loop {
        match pc {
            0x82909B4C => {
    //   block [0x82909B4C..0x82909B54)
	// 82909B4C: 4B9E9B84  b 0x822f36d0
	sub_822F36D0(ctx, base);
	return;
	// 82909B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909B58 size=120
    let mut pc: u32 = 0x82909B58;
    'dispatch: loop {
        match pc {
            0x82909B58 => {
    //   block [0x82909B58..0x82909BD0)
	// 82909B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909B5C: 4889E60D  bl 0x831a8168
	ctx.lr = 0x82909B60;
	sub_831A8130(ctx, base);
	// 82909B60: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909B64: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82909B68: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82909B6C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82909B70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82909B74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82909B78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82909B7C: 13FD58C7  vcmpequd (lvx128) v31, v29, v11
	tmp.u32 = ctx.r[29].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82909B80: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909BD0 size=128
    let mut pc: u32 = 0x82909BD0;
    'dispatch: loop {
        match pc {
            0x82909BD0 => {
    //   block [0x82909BD0..0x82909C50)
	// 82909BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909BD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82909BDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909BE0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909BE4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82909BE8: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82909BEC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82909BF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82909BF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82909BF8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82909BFC: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909C50 size=452
    let mut pc: u32 = 0x82909C50;
    'dispatch: loop {
        match pc {
            0x82909C50 => {
    //   block [0x82909C50..0x82909E14)
	// 82909C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909C54: 4889E511  bl 0x831a8164
	ctx.lr = 0x82909C58;
	sub_831A8130(ctx, base);
	// 82909C58: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82909C5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909C60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909C64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909C68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82909C6C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82909C70: C1BF033C  lfs f13, 0x33c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82909C74: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909C78: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82909C7C: 4099000C  ble cr6, 0x82909c88
	if !ctx.cr[6].gt {
	pc = 0x82909C88; continue 'dispatch;
	}
	// 82909C80: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82909C84: 41820184  beq 0x82909e08
	if ctx.cr[0].eq {
	pc = 0x82909E08; continue 'dispatch;
	}
	// 82909C88: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82909C8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82909C90: 40990008  ble cr6, 0x82909c98
	if !ctx.cr[6].gt {
	pc = 0x82909C98; continue 'dispatch;
	}
	// 82909C94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82909C98: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82909C9C: 3F808338  lis r28, -0x7cc8
	ctx.r[28].s64 = -2093481984;
	// 82909CA0: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82909CA4: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82909CA8: C1AAEE94  lfs f13, -0x116c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4460 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82909CAC: D1BF033C  stfs f13, 0x33c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), tmp.u32 ) };
	// 82909CB0: 815CB1F8  lwz r10, -0x4e08(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-19976 as u32) ) } as u64;
	// 82909CB4: 8169EE98  lwz r11, -0x1168(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4456 as u32) ) } as u64;
	// 82909CB8: C1BF032C  lfs f13, 0x32c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82909CBC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82909CC0: 40980074  bge cr6, 0x82909d34
	if !ctx.cr[6].lt {
	pc = 0x82909D34; continue 'dispatch;
	}
	// 82909CC4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82909CC8: 41990064  bgt cr6, 0x82909d2c
	if ctx.cr[6].gt {
	pc = 0x82909D2C; continue 'dispatch;
	}
	// 82909CCC: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 82909CD0: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909CD4: 4800D35D  bl 0x82917030
	ctx.lr = 0x82909CD8;
	sub_82917030(ctx, base);
	// 82909CD8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82909CDC: 40820050  bne 0x82909d2c
	if !ctx.cr[0].eq {
	pc = 0x82909D2C; continue 'dispatch;
	}
	// 82909CE0: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82909CE4: 48018CB5  bl 0x82922998
	ctx.lr = 0x82909CE8;
	sub_82922998(ctx, base);
	// 82909CE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909CEC: 40820010  bne 0x82909cfc
	if !ctx.cr[0].eq {
	pc = 0x82909CFC; continue 'dispatch;
	}
	// 82909CF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82909CF4: C3EB9450  lfs f31, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82909CF8: 4800000C  b 0x82909d04
	pc = 0x82909D04; continue 'dispatch;
	// 82909CFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909D00: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82909D04: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82909D08: 48018CE9  bl 0x829229f0
	ctx.lr = 0x82909D0C;
	sub_829229F0(ctx, base);
	// 82909D0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909D10: 40820010  bne 0x82909d20
	if !ctx.cr[0].eq {
	pc = 0x82909D20; continue 'dispatch;
	}
	// 82909D14: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82909D18: 388B5730  addi r4, r11, 0x5730
	ctx.r[4].s64 = ctx.r[11].s64 + 22320;
	// 82909D1C: 4800007C  b 0x82909d98
	pc = 0x82909D98; continue 'dispatch;
	// 82909D20: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82909D24: 388B5718  addi r4, r11, 0x5718
	ctx.r[4].s64 = ctx.r[11].s64 + 22296;
	// 82909D28: 48000070  b 0x82909d98
	pc = 0x82909D98; continue 'dispatch;
	// 82909D2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82909D30: 480000A8  b 0x82909dd8
	pc = 0x82909DD8; continue 'dispatch;
	// 82909D34: FF0D0000  fcmpu cr6, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82909D38: 4199009C  bgt cr6, 0x82909dd4
	if ctx.cr[6].gt {
	pc = 0x82909DD4; continue 'dispatch;
	}
	// 82909D3C: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 82909D40: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909D44: 4800D2ED  bl 0x82917030
	ctx.lr = 0x82909D48;
	sub_82917030(ctx, base);
	// 82909D48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82909D4C: 40820088  bne 0x82909dd4
	if !ctx.cr[0].eq {
	pc = 0x82909DD4; continue 'dispatch;
	}
	// 82909D50: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82909D54: 48018C45  bl 0x82922998
	ctx.lr = 0x82909D58;
	sub_82922998(ctx, base);
	// 82909D58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909D5C: 40820010  bne 0x82909d6c
	if !ctx.cr[0].eq {
	pc = 0x82909D6C; continue 'dispatch;
	}
	// 82909D60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82909D64: C3EB9450  lfs f31, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82909D68: 4800000C  b 0x82909d74
	pc = 0x82909D74; continue 'dispatch;
	// 82909D6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909D70: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82909D74: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82909D78: 48018C79  bl 0x829229f0
	ctx.lr = 0x82909D7C;
	sub_829229F0(ctx, base);
	// 82909D7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82909D80: 40820010  bne 0x82909d90
	if !ctx.cr[0].eq {
	pc = 0x82909D90; continue 'dispatch;
	}
	// 82909D84: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82909D88: 388B56FC  addi r4, r11, 0x56fc
	ctx.r[4].s64 = ctx.r[11].s64 + 22268;
	// 82909D8C: 4800000C  b 0x82909d98
	pc = 0x82909D98; continue 'dispatch;
	// 82909D90: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82909D94: 388B56DC  addi r4, r11, 0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + 22236;
	// 82909D98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909D9C: 484E9C6D  bl 0x82df3a08
	ctx.lr = 0x82909DA0;
	sub_82DF3A08(ctx, base);
	// 82909DA0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82909DA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82909DA8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82909DAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82909DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909DB4: 4BEE244D  bl 0x827ec200
	ctx.lr = 0x82909DB8;
	sub_827EC200(ctx, base);
	// 82909DB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82909DBC: 484E966D  bl 0x82df3428
	ctx.lr = 0x82909DC0;
	sub_82DF3428(ctx, base);
	// 82909DC0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82909DC4: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 82909DC8: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909DCC: 4800D255  bl 0x82917020
	ctx.lr = 0x82909DD0;
	sub_82917020(ctx, base);
	// 82909DD0: 48000024  b 0x82909df4
	pc = 0x82909DF4; continue 'dispatch;
	// 82909DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82909DD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909DE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82909DE4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82909DE8: 816B00E8  lwz r11, 0xe8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(232 as u32) ) } as u64;
	// 82909DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82909DF0: 4E800421  bctrl
	ctx.lr = 0x82909DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909DF4: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82909DF8: 41820010  beq 0x82909e08
	if ctx.cr[0].eq {
	pc = 0x82909E08; continue 'dispatch;
	}
	// 82909DFC: 817CB1F8  lwz r11, -0x4e08(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-19976 as u32) ) } as u64;
	// 82909E00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82909E04: 917CB1F8  stw r11, -0x4e08(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-19976 as u32), ctx.r[11].u32 ) };
	// 82909E08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82909E0C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82909E10: 4889E3A4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909E18 size=148
    let mut pc: u32 = 0x82909E18;
    'dispatch: loop {
        match pc {
            0x82909E18 => {
    //   block [0x82909E18..0x82909EAC)
	// 82909E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909E20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82909E24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909E30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82909E34: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909E38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909E3C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82909E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82909E44: 4E800421  bctrl
	ctx.lr = 0x82909E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909E48: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909E4C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82909E50: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82909E54: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909E58: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909E5C: D01E0018  stfs f0, 0x18(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82909E60: 8169001C  lwz r11, 0x1c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82909E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82909E68: 4E800421  bctrl
	ctx.lr = 0x82909E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909E6C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909E70: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909E74: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82909E78: D01E001C  stfs f0, 0x1c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82909E7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909E80: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82909E84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82909E88: 4E800421  bctrl
	ctx.lr = 0x82909E8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909E8C: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909E90: D01E0020  stfs f0, 0x20(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82909E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82909EA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82909EA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82909EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909EB0 size=8
    let mut pc: u32 = 0x82909EB0;
    'dispatch: loop {
        match pc {
            0x82909EB0 => {
    //   block [0x82909EB0..0x82909EB8)
	// 82909EB0: 80630300  lwz r3, 0x300(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(768 as u32) ) } as u64;
	// 82909EB4: 4BEBFFDC  b 0x827c9e90
	sub_827C9E90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909EB8 size=8
    let mut pc: u32 = 0x82909EB8;
    'dispatch: loop {
        match pc {
            0x82909EB8 => {
    //   block [0x82909EB8..0x82909EC0)
	// 82909EB8: 80630300  lwz r3, 0x300(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(768 as u32) ) } as u64;
	// 82909EBC: 4BEBFB6C  b 0x827c9a28
	sub_827C9A28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82909EC0 size=8
    let mut pc: u32 = 0x82909EC0;
    'dispatch: loop {
        match pc {
            0x82909EC0 => {
    //   block [0x82909EC0..0x82909EC8)
	// 82909EC0: 80630300  lwz r3, 0x300(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(768 as u32) ) } as u64;
	// 82909EC4: 4BEBF79C  b 0x827c9660
	sub_827C9660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82909EC8 size=228
    let mut pc: u32 = 0x82909EC8;
    'dispatch: loop {
        match pc {
            0x82909EC8 => {
    //   block [0x82909EC8..0x82909FAC)
	// 82909EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909ECC: 4889E2A1  bl 0x831a816c
	ctx.lr = 0x82909ED0;
	sub_831A8130(ctx, base);
	// 82909ED0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82909ED4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909ED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909EDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82909EE0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82909EE4: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909EE8: 4800D149  bl 0x82917030
	ctx.lr = 0x82909EEC;
	sub_82917030(ctx, base);
	// 82909EEC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82909EF0: 41820010  beq 0x82909f00
	if ctx.cr[0].eq {
	pc = 0x82909F00; continue 'dispatch;
	}
	// 82909EF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909EF8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909EFC: 48000070  b 0x82909f6c
	pc = 0x82909F6C; continue 'dispatch;
	// 82909F00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909F08: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82909F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82909F10: 4E800421  bctrl
	ctx.lr = 0x82909F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909F14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82909F18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82909F1C: 4804E79D  bl 0x829586b8
	ctx.lr = 0x82909F20;
	sub_829586B8(ctx, base);
	// 82909F20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82909F24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82909F28: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82909F2C: 4804E7D5  bl 0x82958700
	ctx.lr = 0x82909F30;
	sub_82958700(ctx, base);
	// 82909F30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909F34: C18B08A4  lfs f12, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82909F38: FF1F6000  fcmpu cr6, f31, f12
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[12].f64);
	// 82909F3C: 4199000C  bgt cr6, 0x82909f48
	if ctx.cr[6].gt {
	pc = 0x82909F48; continue 'dispatch;
	}
	// 82909F40: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 82909F44: 40990010  ble cr6, 0x82909f54
	if !ctx.cr[6].gt {
	pc = 0x82909F54; continue 'dispatch;
	}
	// 82909F48: C01F032C  lfs f0, 0x32c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909F4C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82909F50: 4098000C  bge cr6, 0x82909f5c
	if !ctx.cr[6].lt {
	pc = 0x82909F5C; continue 'dispatch;
	}
	// 82909F54: D19E0018  stfs f12, 0x18(r30)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82909F58: 48000048  b 0x82909fa0
	pc = 0x82909FA0; continue 'dispatch;
	// 82909F5C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82909F60: 40990014  ble cr6, 0x82909f74
	if !ctx.cr[6].gt {
	pc = 0x82909F74; continue 'dispatch;
	}
	// 82909F64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82909F68: C00B9534  lfs f0, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82909F6C: D01E0018  stfs f0, 0x18(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82909F70: 48000030  b 0x82909fa0
	pc = 0x82909FA0; continue 'dispatch;
	// 82909F74: C1BF0330  lfs f13, 0x330(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(816 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82909F78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82909F7C: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82909F80: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82909F84: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82909F88: 41990014  bgt cr6, 0x82909f9c
	if ctx.cr[6].gt {
	pc = 0x82909F9C; continue 'dispatch;
	}
	// 82909F8C: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82909F90: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82909F94: 40980008  bge cr6, 0x82909f9c
	if !ctx.cr[6].lt {
	pc = 0x82909F9C; continue 'dispatch;
	}
	// 82909F98: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	// 82909F9C: D1BE0018  stfs f13, 0x18(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82909FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82909FA4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82909FA8: 4889E214  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82909FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82909FB0 size=92
    let mut pc: u32 = 0x82909FB0;
    'dispatch: loop {
        match pc {
            0x82909FB0 => {
    //   block [0x82909FB0..0x8290A00C)
	// 82909FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82909FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82909FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82909FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82909FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82909FC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82909FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82909FCC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82909FD0: 88BE0018  lbz r5, 0x18(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82909FD4: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82909FD8: 4800D049  bl 0x82917020
	ctx.lr = 0x82909FDC;
	sub_82917020(ctx, base);
	// 82909FDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82909FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82909FE4: 889E0018  lbz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82909FE8: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82909FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82909FF0: 4E800421  bctrl
	ctx.lr = 0x82909FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82909FF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82909FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82909FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A010 size=80
    let mut pc: u32 = 0x8290A010;
    'dispatch: loop {
        match pc {
            0x8290A010 => {
    //   block [0x8290A010..0x8290A060)
	// 8290A010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A01C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A024: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290A028: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8290A02C: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A030: 4800CFF1  bl 0x82917020
	ctx.lr = 0x8290A034;
	sub_82917020(ctx, base);
	// 8290A034: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A038: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290A03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A040: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290A044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A048: 4E800421  bctrl
	ctx.lr = 0x8290A04C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A04C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290A050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A060 size=80
    let mut pc: u32 = 0x8290A060;
    'dispatch: loop {
        match pc {
            0x8290A060 => {
    //   block [0x8290A060..0x8290A0B0)
	// 8290A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A06C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290A078: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8290A07C: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A080: 4800CFA1  bl 0x82917020
	ctx.lr = 0x8290A084;
	sub_82917020(ctx, base);
	// 8290A084: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A088: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290A08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A090: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290A094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A098: 4E800421  bctrl
	ctx.lr = 0x8290A09C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A09C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290A0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A0A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A0B0 size=96
    let mut pc: u32 = 0x8290A0B0;
    'dispatch: loop {
        match pc {
            0x8290A0B0 => {
    //   block [0x8290A0B0..0x8290A110)
	// 8290A0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A0B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A0BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A0C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A0C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A0C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290A0CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8290A0D0: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A0D4: 4800CF5D  bl 0x82917030
	ctx.lr = 0x8290A0D8;
	sub_82917030(ctx, base);
	// 8290A0D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290A0DC: 4082001C  bne 0x8290a0f8
	if !ctx.cr[0].eq {
	pc = 0x8290A0F8; continue 'dispatch;
	}
	// 8290A0E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A0E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A0E8: 889E0018  lbz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290A0EC: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290A0F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A0F4: 4E800421  bctrl
	ctx.lr = 0x8290A0F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A0F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290A110 size=268
    let mut pc: u32 = 0x8290A110;
    'dispatch: loop {
        match pc {
            0x8290A110 => {
    //   block [0x8290A110..0x8290A21C)
	// 8290A110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A114: 4889E059  bl 0x831a816c
	ctx.lr = 0x8290A118;
	sub_831A8130(ctx, base);
	// 8290A118: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A11C: 83A300E4  lwz r29, 0xe4(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A120: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290A124: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8290A128: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A12C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A130: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8290A134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A138: 4E800421  bctrl
	ctx.lr = 0x8290A13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A13C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290A140: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290A144: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 8290A148: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8290A14C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290A150: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290A154: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8290A158: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290A15C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290A160: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8290A164: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8290A168: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8290A16C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8290A170: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8290A174: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8290A178: D1A10078  stfs f13, 0x78(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8290A17C: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8290A180: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A184: 48572FF5  bl 0x82e7d178
	ctx.lr = 0x8290A188;
	sub_82E7D178(ctx, base);
	// 8290A188: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290A18C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8290A190: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A198: 4E800421  bctrl
	ctx.lr = 0x8290A19C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A19C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8290A1A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290A1A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290A1AC: C02B029C  lfs f1, 0x29c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(668 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290A1B0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A1B4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8290A1B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A1BC: 4E800421  bctrl
	ctx.lr = 0x8290A1C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A1C0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290A1C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A1C8: 808B0BC4  lwz r4, 0xbc4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3012 as u32) ) } as u64;
	// 8290A1CC: 484E983D  bl 0x82df3a08
	ctx.lr = 0x8290A1D0;
	sub_82DF3A08(ctx, base);
	// 8290A1D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A1D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290A1D8: 4BEE5569  bl 0x827ef740
	ctx.lr = 0x8290A1DC;
	sub_827EF740(ctx, base);
	// 8290A1DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A1E0: 484E9249  bl 0x82df3428
	ctx.lr = 0x8290A1E4;
	sub_82DF3428(ctx, base);
	// 8290A1E4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290A1E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A1EC: 808BF3F8  lwz r4, -0xc08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3080 as u32) ) } as u64;
	// 8290A1F0: 484E9819  bl 0x82df3a08
	ctx.lr = 0x8290A1F4;
	sub_82DF3A08(ctx, base);
	// 8290A1F4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A1F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290A1FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A200: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8290A204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A208: 4E800421  bctrl
	ctx.lr = 0x8290A20C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A20C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A210: 484E9219  bl 0x82df3428
	ctx.lr = 0x8290A214;
	sub_82DF3428(ctx, base);
	// 8290A214: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8290A218: 4889DFA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A220 size=16
    let mut pc: u32 = 0x8290A220;
    'dispatch: loop {
        match pc {
            0x8290A220 => {
    //   block [0x8290A220..0x8290A230)
	// 8290A220: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A224: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290A228: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 8290A22C: 4800CDF4  b 0x82917020
	sub_82917020(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A230 size=16
    let mut pc: u32 = 0x8290A230;
    'dispatch: loop {
        match pc {
            0x8290A230 => {
    //   block [0x8290A230..0x8290A240)
	// 8290A230: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290A238: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 8290A23C: 4800CDE4  b 0x82917020
	sub_82917020(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A240 size=124
    let mut pc: u32 = 0x8290A240;
    'dispatch: loop {
        match pc {
            0x8290A240 => {
    //   block [0x8290A240..0x8290A2BC)
	// 8290A240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A24C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A258: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290A25C: 817F031C  lwz r11, 0x31c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8290A260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290A264: 419A0040  beq cr6, 0x8290a2a4
	if ctx.cr[6].eq {
	pc = 0x8290A2A4; continue 'dispatch;
	}
	// 8290A268: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290A26C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A270: 388B5748  addi r4, r11, 0x5748
	ctx.r[4].s64 = ctx.r[11].s64 + 22344;
	// 8290A274: 484EF2BD  bl 0x82df9530
	ctx.lr = 0x8290A278;
	sub_82DF9530(ctx, base);
	// 8290A278: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290A27C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A280: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8290A284: 409A0020  bne cr6, 0x8290a2a4
	if !ctx.cr[6].eq {
	pc = 0x8290A2A4; continue 'dispatch;
	}
	// 8290A288: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A28C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A290: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8290A294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A298: 4E800421  bctrl
	ctx.lr = 0x8290A29C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A2A0: 4BC06331  bl 0x825105d0
	ctx.lr = 0x8290A2A4;
	sub_825105D0(ctx, base);
	// 8290A2A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290A2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A2B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A2B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A2C0 size=8
    let mut pc: u32 = 0x8290A2C0;
    'dispatch: loop {
        match pc {
            0x8290A2C0 => {
    //   block [0x8290A2C0..0x8290A2C8)
	// 8290A2C0: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A2C4: 4800C85C  b 0x82916b20
	sub_82916B20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A2C8 size=8
    let mut pc: u32 = 0x8290A2C8;
    'dispatch: loop {
        match pc {
            0x8290A2C8 => {
    //   block [0x8290A2C8..0x8290A2D0)
	// 8290A2C8: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A2CC: 4800C86C  b 0x82916b38
	sub_82916B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290A2D0 size=76
    let mut pc: u32 = 0x8290A2D0;
    'dispatch: loop {
        match pc {
            0x8290A2D0 => {
    //   block [0x8290A2D0..0x8290A31C)
	// 8290A2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A2DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A2E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290A2E4: 808300E4  lwz r4, 0xe4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290A2E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A2EC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A2F0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290A2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A2F8: 4E800421  bctrl
	ctx.lr = 0x8290A2FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A2FC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290A300: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A320 size=8
    let mut pc: u32 = 0x8290A320;
    'dispatch: loop {
        match pc {
            0x8290A320 => {
    //   block [0x8290A320..0x8290A328)
	// 8290A320: 80630300  lwz r3, 0x300(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(768 as u32) ) } as u64;
	// 8290A324: 4BEBF3A4  b 0x827c96c8
	sub_827C96C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A328 size=8
    let mut pc: u32 = 0x8290A328;
    'dispatch: loop {
        match pc {
            0x8290A328 => {
    //   block [0x8290A328..0x8290A330)
	// 8290A328: 80630300  lwz r3, 0x300(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(768 as u32) ) } as u64;
	// 8290A32C: 4BEBF404  b 0x827c9730
	sub_827C9730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A330 size=124
    let mut pc: u32 = 0x8290A330;
    'dispatch: loop {
        match pc {
            0x8290A330 => {
    //   block [0x8290A330..0x8290A3AC)
	// 8290A330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A334: 4889DE39  bl 0x831a816c
	ctx.lr = 0x8290A338;
	sub_831A8130(ctx, base);
	// 8290A338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A33C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A340: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290A344: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290A348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A34C: 83BF0308  lwz r29, 0x308(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 8290A350: 4BC05179  bl 0x8250f4c8
	ctx.lr = 0x8290A354;
	sub_8250F4C8(ctx, base);
	// 8290A354: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290A358: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A35C: 480170AD  bl 0x82921408
	ctx.lr = 0x8290A360;
	sub_82921408(ctx, base);
	// 8290A360: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 8290A364: 419A002C  beq cr6, 0x8290a390
	if ctx.cr[6].eq {
	pc = 0x8290A390; continue 'dispatch;
	}
	// 8290A368: 2B1E0002  cmplwi cr6, r30, 2
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2 as u32, &mut ctx.xer);
	// 8290A36C: 419A0018  beq cr6, 0x8290a384
	if ctx.cr[6].eq {
	pc = 0x8290A384; continue 'dispatch;
	}
	// 8290A370: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 8290A374: 409A0030  bne cr6, 0x8290a3a4
	if !ctx.cr[6].eq {
	pc = 0x8290A3A4; continue 'dispatch;
	}
	// 8290A378: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A37C: 816B0170  lwz r11, 0x170(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(368 as u32) ) } as u64;
	// 8290A380: 48000018  b 0x8290a398
	pc = 0x8290A398; continue 'dispatch;
	// 8290A384: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A388: 816B016C  lwz r11, 0x16c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 8290A38C: 4800000C  b 0x8290a398
	pc = 0x8290A398; continue 'dispatch;
	// 8290A390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A394: 816B0168  lwz r11, 0x168(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(360 as u32) ) } as u64;
	// 8290A398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A39C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A3A0: 4E800421  bctrl
	ctx.lr = 0x8290A3A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A3A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290A3A8: 4889DE14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A3B0 size=124
    let mut pc: u32 = 0x8290A3B0;
    'dispatch: loop {
        match pc {
            0x8290A3B0 => {
    //   block [0x8290A3B0..0x8290A42C)
	// 8290A3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A3B4: 4889DDB9  bl 0x831a816c
	ctx.lr = 0x8290A3B8;
	sub_831A8130(ctx, base);
	// 8290A3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A3BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A3C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290A3C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290A3C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A3CC: 83BF0308  lwz r29, 0x308(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 8290A3D0: 4BC050F9  bl 0x8250f4c8
	ctx.lr = 0x8290A3D4;
	sub_8250F4C8(ctx, base);
	// 8290A3D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290A3D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290A3DC: 4801702D  bl 0x82921408
	ctx.lr = 0x8290A3E0;
	sub_82921408(ctx, base);
	// 8290A3E0: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 8290A3E4: 419A002C  beq cr6, 0x8290a410
	if ctx.cr[6].eq {
	pc = 0x8290A410; continue 'dispatch;
	}
	// 8290A3E8: 2B1E0002  cmplwi cr6, r30, 2
	ctx.cr[6].compare_u32(ctx.r[30].u32, 2 as u32, &mut ctx.xer);
	// 8290A3EC: 419A0018  beq cr6, 0x8290a404
	if ctx.cr[6].eq {
	pc = 0x8290A404; continue 'dispatch;
	}
	// 8290A3F0: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 8290A3F4: 409A0030  bne cr6, 0x8290a424
	if !ctx.cr[6].eq {
	pc = 0x8290A424; continue 'dispatch;
	}
	// 8290A3F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A3FC: 816B017C  lwz r11, 0x17c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(380 as u32) ) } as u64;
	// 8290A400: 48000018  b 0x8290a418
	pc = 0x8290A418; continue 'dispatch;
	// 8290A404: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A408: 816B0178  lwz r11, 0x178(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(376 as u32) ) } as u64;
	// 8290A40C: 4800000C  b 0x8290a418
	pc = 0x8290A418; continue 'dispatch;
	// 8290A410: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A414: 816B0174  lwz r11, 0x174(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(372 as u32) ) } as u64;
	// 8290A418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A41C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A420: 4E800421  bctrl
	ctx.lr = 0x8290A424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290A428: 4889DD94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290A430 size=8
    let mut pc: u32 = 0x8290A430;
    'dispatch: loop {
        match pc {
            0x8290A430 => {
    //   block [0x8290A430..0x8290A438)
	// 8290A430: 80630310  lwz r3, 0x310(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(784 as u32) ) } as u64;
	// 8290A434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290A438 size=60
    let mut pc: u32 = 0x8290A438;
    'dispatch: loop {
        match pc {
            0x8290A438 => {
    //   block [0x8290A438..0x8290A474)
	// 8290A438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A44C: 3864FDF8  addi r3, r4, -0x208
	ctx.r[3].s64 = ctx.r[4].s64 + -520;
	// 8290A450: 48109231  bl 0x82a13680
	ctx.lr = 0x8290A454;
	sub_82A13680(ctx, base);
	// 8290A454: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A478 size=52
    let mut pc: u32 = 0x8290A478;
    'dispatch: loop {
        match pc {
            0x8290A478 => {
    //   block [0x8290A478..0x8290A4AC)
	// 8290A478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A484: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290A488: 8063FEDC  lwz r3, -0x124(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-292 as u32) ) } as u64;
	// 8290A48C: 4800CBA5  bl 0x82917030
	ctx.lr = 0x8290A490;
	sub_82917030(ctx, base);
	// 8290A490: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8290A494: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8290A498: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8290A49C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290A4A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A4A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A4B0 size=52
    let mut pc: u32 = 0x8290A4B0;
    'dispatch: loop {
        match pc {
            0x8290A4B0 => {
    //   block [0x8290A4B0..0x8290A4E4)
	// 8290A4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A4BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290A4C0: 8063FEDC  lwz r3, -0x124(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-292 as u32) ) } as u64;
	// 8290A4C4: 4800CB6D  bl 0x82917030
	ctx.lr = 0x8290A4C8;
	sub_82917030(ctx, base);
	// 8290A4C8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8290A4CC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8290A4D0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8290A4D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290A4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A4E8 size=104
    let mut pc: u32 = 0x8290A4E8;
    'dispatch: loop {
        match pc {
            0x8290A4E8 => {
    //   block [0x8290A4E8..0x8290A550)
	// 8290A4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A4F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A4F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A4F8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290A4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A504: 808B0B40  lwz r4, 0xb40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2880 as u32) ) } as u64;
	// 8290A508: 484E9501  bl 0x82df3a08
	ctx.lr = 0x8290A50C;
	sub_82DF3A08(ctx, base);
	// 8290A50C: 807FFEDC  lwz r3, -0x124(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-292 as u32) ) } as u64;
	// 8290A510: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8290A514: 4BEE5235  bl 0x827ef748
	ctx.lr = 0x8290A518;
	sub_827EF748(ctx, base);
	// 8290A518: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290A51C: 484E8DED  bl 0x82df3308
	ctx.lr = 0x8290A520;
	sub_82DF3308(ctx, base);
	// 8290A520: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A524: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A528: 484E8F01  bl 0x82df3428
	ctx.lr = 0x8290A52C;
	sub_82DF3428(ctx, base);
	// 8290A52C: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 8290A530: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8290A534: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8290A538: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8290A53C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290A540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A550 size=232
    let mut pc: u32 = 0x8290A550;
    'dispatch: loop {
        match pc {
            0x8290A550 => {
    //   block [0x8290A550..0x8290A638)
	// 8290A550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A55C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A568: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A56C: 809F0244  lwz r4, 0x244(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290A570: 48018441  bl 0x829229b0
	ctx.lr = 0x8290A574;
	sub_829229B0(ctx, base);
	// 8290A574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290A578: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290A57C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 8290A580: 484E9489  bl 0x82df3a08
	ctx.lr = 0x8290A584;
	sub_82DF3A08(ctx, base);
	// 8290A584: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8290A588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A58C: 484E8D15  bl 0x82df32a0
	ctx.lr = 0x8290A590;
	sub_82DF32A0(ctx, base);
	// 8290A590: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290A594: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290A598: 484E8E91  bl 0x82df3428
	ctx.lr = 0x8290A59C;
	sub_82DF3428(ctx, base);
	// 8290A59C: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290A5A0: 41820078  beq 0x8290a618
	if ctx.cr[0].eq {
	pc = 0x8290A618; continue 'dispatch;
	}
	// 8290A5A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290A5A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290A5AC: 4BC04D85  bl 0x8250f330
	ctx.lr = 0x8290A5B0;
	sub_8250F330(ctx, base);
	// 8290A5B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290A5B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290A5B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A5BC: 4BBE0B9D  bl 0x824eb158
	ctx.lr = 0x8290A5C0;
	sub_824EB158(ctx, base);
	// 8290A5C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290A5C4: 484E76CD  bl 0x82df1c90
	ctx.lr = 0x8290A5C8;
	sub_82DF1C90(ctx, base);
	// 8290A5C8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290A5CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290A5D0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8290A5D4: 409A0008  bne cr6, 0x8290a5dc
	if !ctx.cr[6].eq {
	pc = 0x8290A5DC; continue 'dispatch;
	}
	// 8290A5D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290A5DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290A5E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290A5E4: 4BBF6E95  bl 0x82501478
	ctx.lr = 0x8290A5E8;
	sub_82501478(ctx, base);
	// 8290A5E8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8290A5EC: 41820024  beq 0x8290a610
	if ctx.cr[0].eq {
	pc = 0x8290A610; continue 'dispatch;
	}
	// 8290A5F0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290A5F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290A5F8: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8290A5FC: 409A0008  bne cr6, 0x8290a604
	if !ctx.cr[6].eq {
	pc = 0x8290A604; continue 'dispatch;
	}
	// 8290A600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A604: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290A608: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290A60C: 4BBF5575  bl 0x824ffb80
	ctx.lr = 0x8290A610;
	sub_824FFB80(ctx, base);
	// 8290A610: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290A614: 484E767D  bl 0x82df1c90
	ctx.lr = 0x8290A618;
	sub_82DF1C90(ctx, base);
	// 8290A618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A61C: 484E8E0D  bl 0x82df3428
	ctx.lr = 0x8290A620;
	sub_82DF3428(ctx, base);
	// 8290A620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290A624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A62C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A630: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290A638 size=272
    let mut pc: u32 = 0x8290A638;
    'dispatch: loop {
        match pc {
            0x8290A638 => {
    //   block [0x8290A638..0x8290A748)
	// 8290A638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A640: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A644: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290A64C: 48109035  bl 0x82a13680
	ctx.lr = 0x8290A650;
	sub_82A13680(ctx, base);
	// 8290A650: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290A654: C1BF0334  lfs f13, 0x334(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(820 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290A658: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8290A65C: C00B9D1C  lfs f0, -0x62e4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25316 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290A660: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8290A664: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8290A668: 4098000C  bge cr6, 0x8290a674
	if !ctx.cr[6].lt {
	pc = 0x8290A674; continue 'dispatch;
	}
	// 8290A66C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8290A670: 480000C4  b 0x8290a734
	pc = 0x8290A734; continue 'dispatch;
	// 8290A674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A678: 48109009  bl 0x82a13680
	ctx.lr = 0x8290A67C;
	sub_82A13680(ctx, base);
	// 8290A67C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8290A680: C1BF0334  lfs f13, 0x334(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(820 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290A684: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8290A688: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290A68C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8290A690: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8290A694: 4098009C  bge cr6, 0x8290a730
	if !ctx.cr[6].lt {
	pc = 0x8290A730; continue 'dispatch;
	}
	// 8290A698: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290A69C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290A6A0: 4BC04E79  bl 0x8250f518
	ctx.lr = 0x8290A6A4;
	sub_8250F518(ctx, base);
	// 8290A6A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290A6A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A6AC: 4BB86A45  bl 0x824910f0
	ctx.lr = 0x8290A6B0;
	sub_824910F0(ctx, base);
	// 8290A6B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290A6B4: 484E75DD  bl 0x82df1c90
	ctx.lr = 0x8290A6B8;
	sub_82DF1C90(ctx, base);
	// 8290A6B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A6BC: 48108FC5  bl 0x82a13680
	ctx.lr = 0x8290A6C0;
	sub_82A13680(ctx, base);
	// 8290A6C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290A6C4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8290A6C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290A6CC: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8290A6D0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8290A6D4: 3CE08204  lis r7, -0x7dfc
	ctx.r[7].s64 = -2113667072;
	// 8290A6D8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290A6DC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A748 size=196
    let mut pc: u32 = 0x8290A748;
    'dispatch: loop {
        match pc {
            0x8290A748 => {
    //   block [0x8290A748..0x8290A80C)
	// 8290A748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A750: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A754: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A75C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290A760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A764: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290A768: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290A76C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A770: 4B9B61C9  bl 0x822c0938
	ctx.lr = 0x8290A774;
	sub_822C0938(ctx, base);
	// 8290A774: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290A778: 41820028  beq 0x8290a7a0
	if ctx.cr[0].eq {
	pc = 0x8290A7A0; continue 'dispatch;
	}
	// 8290A77C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290A780: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290A784: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290A788: 392B5578  addi r9, r11, 0x5578
	ctx.r[9].s64 = ctx.r[11].s64 + 21880;
	// 8290A78C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290A790: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290A794: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290A798: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290A79C: 48000008  b 0x8290a7a4
	pc = 0x8290A7A4; continue 'dispatch;
	// 8290A7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A7A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A7A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290A7AC: 409A0044  bne cr6, 0x8290a7f0
	if !ctx.cr[6].eq {
	pc = 0x8290A7F0; continue 'dispatch;
	}
	// 8290A7B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290A7B4: 419A001C  beq cr6, 0x8290a7d0
	if ctx.cr[6].eq {
	pc = 0x8290A7D0; continue 'dispatch;
	}
	// 8290A7B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A7BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290A7C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A7C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A7C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A7CC: 4E800421  bctrl
	ctx.lr = 0x8290A7D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A7D0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290A7D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290A7D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A7DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290A7E0: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290A7E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290A7E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290A7EC: 4B9B5815  bl 0x822c0000
	ctx.lr = 0x8290A7F0;
	sub_822C0000(ctx, base);
	// 8290A7F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290A7F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290A7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A800: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A810 size=196
    let mut pc: u32 = 0x8290A810;
    'dispatch: loop {
        match pc {
            0x8290A810 => {
    //   block [0x8290A810..0x8290A8D4)
	// 8290A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A81C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290A828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A82C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290A830: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290A834: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A838: 4B9B6101  bl 0x822c0938
	ctx.lr = 0x8290A83C;
	sub_822C0938(ctx, base);
	// 8290A83C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290A840: 41820028  beq 0x8290a868
	if ctx.cr[0].eq {
	pc = 0x8290A868; continue 'dispatch;
	}
	// 8290A844: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290A848: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290A84C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290A850: 392B558C  addi r9, r11, 0x558c
	ctx.r[9].s64 = ctx.r[11].s64 + 21900;
	// 8290A854: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290A858: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290A85C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290A860: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290A864: 48000008  b 0x8290a86c
	pc = 0x8290A86C; continue 'dispatch;
	// 8290A868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A86C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A870: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290A874: 409A0044  bne cr6, 0x8290a8b8
	if !ctx.cr[6].eq {
	pc = 0x8290A8B8; continue 'dispatch;
	}
	// 8290A878: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290A87C: 419A001C  beq cr6, 0x8290a898
	if ctx.cr[6].eq {
	pc = 0x8290A898; continue 'dispatch;
	}
	// 8290A880: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A884: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290A888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A88C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A890: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A894: 4E800421  bctrl
	ctx.lr = 0x8290A898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A898: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290A89C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290A8A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A8A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290A8A8: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290A8AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290A8B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290A8B4: 4B9B574D  bl 0x822c0000
	ctx.lr = 0x8290A8B8;
	sub_822C0000(ctx, base);
	// 8290A8B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290A8BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290A8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A8C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A8CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A8D8 size=196
    let mut pc: u32 = 0x8290A8D8;
    'dispatch: loop {
        match pc {
            0x8290A8D8 => {
    //   block [0x8290A8D8..0x8290A99C)
	// 8290A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A8EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290A8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A8F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290A8F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290A8FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A900: 4B9B6039  bl 0x822c0938
	ctx.lr = 0x8290A904;
	sub_822C0938(ctx, base);
	// 8290A904: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290A908: 41820028  beq 0x8290a930
	if ctx.cr[0].eq {
	pc = 0x8290A930; continue 'dispatch;
	}
	// 8290A90C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290A910: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290A914: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290A918: 392B55A0  addi r9, r11, 0x55a0
	ctx.r[9].s64 = ctx.r[11].s64 + 21920;
	// 8290A91C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290A920: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290A924: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290A928: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290A92C: 48000008  b 0x8290a934
	pc = 0x8290A934; continue 'dispatch;
	// 8290A930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A934: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290A93C: 409A0044  bne cr6, 0x8290a980
	if !ctx.cr[6].eq {
	pc = 0x8290A980; continue 'dispatch;
	}
	// 8290A940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290A944: 419A001C  beq cr6, 0x8290a960
	if ctx.cr[6].eq {
	pc = 0x8290A960; continue 'dispatch;
	}
	// 8290A948: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A94C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290A950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290A954: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290A958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290A95C: 4E800421  bctrl
	ctx.lr = 0x8290A960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290A960: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290A964: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290A968: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290A96C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290A970: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290A974: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290A978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290A97C: 4B9B5685  bl 0x822c0000
	ctx.lr = 0x8290A980;
	sub_822C0000(ctx, base);
	// 8290A980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290A984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290A988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290A98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290A990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290A994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290A998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290A9A0 size=196
    let mut pc: u32 = 0x8290A9A0;
    'dispatch: loop {
        match pc {
            0x8290A9A0 => {
    //   block [0x8290A9A0..0x8290AA64)
	// 8290A9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290A9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290A9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290A9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290A9B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290A9B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290A9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A9BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290A9C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290A9C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290A9C8: 4B9B5F71  bl 0x822c0938
	ctx.lr = 0x8290A9CC;
	sub_822C0938(ctx, base);
	// 8290A9CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290A9D0: 41820028  beq 0x8290a9f8
	if ctx.cr[0].eq {
	pc = 0x8290A9F8; continue 'dispatch;
	}
	// 8290A9D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290A9D8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290A9DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290A9E0: 392B55B4  addi r9, r11, 0x55b4
	ctx.r[9].s64 = ctx.r[11].s64 + 21940;
	// 8290A9E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290A9E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290A9EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290A9F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290A9F4: 48000008  b 0x8290a9fc
	pc = 0x8290A9FC; continue 'dispatch;
	// 8290A9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290A9FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AA00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AA04: 409A0044  bne cr6, 0x8290aa48
	if !ctx.cr[6].eq {
	pc = 0x8290AA48; continue 'dispatch;
	}
	// 8290AA08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AA0C: 419A001C  beq cr6, 0x8290aa28
	if ctx.cr[6].eq {
	pc = 0x8290AA28; continue 'dispatch;
	}
	// 8290AA10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AA14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AA1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AA20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AA24: 4E800421  bctrl
	ctx.lr = 0x8290AA28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AA28: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AA2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AA30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AA34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AA38: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AA3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AA40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AA44: 4B9B55BD  bl 0x822c0000
	ctx.lr = 0x8290AA48;
	sub_822C0000(ctx, base);
	// 8290AA48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290AA4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290AA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290AA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290AA58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290AA5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290AA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290AA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290AA68 size=196
    let mut pc: u32 = 0x8290AA68;
    'dispatch: loop {
        match pc {
            0x8290AA68 => {
    //   block [0x8290AA68..0x8290AB2C)
	// 8290AA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290AA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AA70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AA74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AA7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290AA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AA84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290AA88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290AA8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AA90: 4B9B5EA9  bl 0x822c0938
	ctx.lr = 0x8290AA94;
	sub_822C0938(ctx, base);
	// 8290AA94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290AA98: 41820028  beq 0x8290aac0
	if ctx.cr[0].eq {
	pc = 0x8290AAC0; continue 'dispatch;
	}
	// 8290AA9C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290AAA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290AAA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290AAA8: 392B55C8  addi r9, r11, 0x55c8
	ctx.r[9].s64 = ctx.r[11].s64 + 21960;
	// 8290AAAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290AAB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290AAB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290AAB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290AABC: 48000008  b 0x8290aac4
	pc = 0x8290AAC4; continue 'dispatch;
	// 8290AAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AAC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AAC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AACC: 409A0044  bne cr6, 0x8290ab10
	if !ctx.cr[6].eq {
	pc = 0x8290AB10; continue 'dispatch;
	}
	// 8290AAD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AAD4: 419A001C  beq cr6, 0x8290aaf0
	if ctx.cr[6].eq {
	pc = 0x8290AAF0; continue 'dispatch;
	}
	// 8290AAD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AADC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AAE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AAE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AAEC: 4E800421  bctrl
	ctx.lr = 0x8290AAF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AAF0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AAF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AAF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AAFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AB00: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AB04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AB08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AB0C: 4B9B54F5  bl 0x822c0000
	ctx.lr = 0x8290AB10;
	sub_822C0000(ctx, base);
	// 8290AB10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290AB14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290AB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290AB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290AB20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290AB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290AB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290AB30 size=196
    let mut pc: u32 = 0x8290AB30;
    'dispatch: loop {
        match pc {
            0x8290AB30 => {
    //   block [0x8290AB30..0x8290ABF4)
	// 8290AB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290AB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AB38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AB3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AB40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AB44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290AB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AB4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290AB50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290AB54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AB58: 4B9B5DE1  bl 0x822c0938
	ctx.lr = 0x8290AB5C;
	sub_822C0938(ctx, base);
	// 8290AB5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290AB60: 41820028  beq 0x8290ab88
	if ctx.cr[0].eq {
	pc = 0x8290AB88; continue 'dispatch;
	}
	// 8290AB64: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290AB68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290AB6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290AB70: 392B55F0  addi r9, r11, 0x55f0
	ctx.r[9].s64 = ctx.r[11].s64 + 22000;
	// 8290AB74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290AB78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290AB7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290AB80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290AB84: 48000008  b 0x8290ab8c
	pc = 0x8290AB8C; continue 'dispatch;
	// 8290AB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AB8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AB90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AB94: 409A0044  bne cr6, 0x8290abd8
	if !ctx.cr[6].eq {
	pc = 0x8290ABD8; continue 'dispatch;
	}
	// 8290AB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AB9C: 419A001C  beq cr6, 0x8290abb8
	if ctx.cr[6].eq {
	pc = 0x8290ABB8; continue 'dispatch;
	}
	// 8290ABA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290ABA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290ABA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290ABAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290ABB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290ABB4: 4E800421  bctrl
	ctx.lr = 0x8290ABB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290ABB8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290ABBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290ABC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290ABC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290ABC8: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290ABCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290ABD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290ABD4: 4B9B542D  bl 0x822c0000
	ctx.lr = 0x8290ABD8;
	sub_822C0000(ctx, base);
	// 8290ABD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290ABDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290ABE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290ABE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290ABE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290ABEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290ABF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290ABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290ABF8 size=196
    let mut pc: u32 = 0x8290ABF8;
    'dispatch: loop {
        match pc {
            0x8290ABF8 => {
    //   block [0x8290ABF8..0x8290ACBC)
	// 8290ABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290ABFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AC00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AC04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AC0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290AC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AC14: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290AC18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290AC1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AC20: 4B9B5D19  bl 0x822c0938
	ctx.lr = 0x8290AC24;
	sub_822C0938(ctx, base);
	// 8290AC24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290AC28: 41820028  beq 0x8290ac50
	if ctx.cr[0].eq {
	pc = 0x8290AC50; continue 'dispatch;
	}
	// 8290AC2C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290AC30: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290AC34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290AC38: 392B5604  addi r9, r11, 0x5604
	ctx.r[9].s64 = ctx.r[11].s64 + 22020;
	// 8290AC3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290AC40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290AC44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290AC48: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290AC4C: 48000008  b 0x8290ac54
	pc = 0x8290AC54; continue 'dispatch;
	// 8290AC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AC54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AC5C: 409A0044  bne cr6, 0x8290aca0
	if !ctx.cr[6].eq {
	pc = 0x8290ACA0; continue 'dispatch;
	}
	// 8290AC60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AC64: 419A001C  beq cr6, 0x8290ac80
	if ctx.cr[6].eq {
	pc = 0x8290AC80; continue 'dispatch;
	}
	// 8290AC68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AC6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AC74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AC78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AC7C: 4E800421  bctrl
	ctx.lr = 0x8290AC80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AC80: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AC84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AC88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AC8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AC90: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AC94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AC9C: 4B9B5365  bl 0x822c0000
	ctx.lr = 0x8290ACA0;
	sub_822C0000(ctx, base);
	// 8290ACA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290ACA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290ACA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290ACAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290ACB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290ACB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290ACB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290ACC0 size=196
    let mut pc: u32 = 0x8290ACC0;
    'dispatch: loop {
        match pc {
            0x8290ACC0 => {
    //   block [0x8290ACC0..0x8290AD84)
	// 8290ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290ACC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290ACCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290ACD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290ACD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290ACD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290ACDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290ACE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290ACE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290ACE8: 4B9B5C51  bl 0x822c0938
	ctx.lr = 0x8290ACEC;
	sub_822C0938(ctx, base);
	// 8290ACEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290ACF0: 41820028  beq 0x8290ad18
	if ctx.cr[0].eq {
	pc = 0x8290AD18; continue 'dispatch;
	}
	// 8290ACF4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290ACF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290ACFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290AD00: 392B5618  addi r9, r11, 0x5618
	ctx.r[9].s64 = ctx.r[11].s64 + 22040;
	// 8290AD04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290AD08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290AD0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290AD10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290AD14: 48000008  b 0x8290ad1c
	pc = 0x8290AD1C; continue 'dispatch;
	// 8290AD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AD1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AD20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AD24: 409A0044  bne cr6, 0x8290ad68
	if !ctx.cr[6].eq {
	pc = 0x8290AD68; continue 'dispatch;
	}
	// 8290AD28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AD2C: 419A001C  beq cr6, 0x8290ad48
	if ctx.cr[6].eq {
	pc = 0x8290AD48; continue 'dispatch;
	}
	// 8290AD30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AD34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AD3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AD40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AD44: 4E800421  bctrl
	ctx.lr = 0x8290AD48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AD48: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AD4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AD50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AD54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AD58: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AD5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AD60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AD64: 4B9B529D  bl 0x822c0000
	ctx.lr = 0x8290AD68;
	sub_822C0000(ctx, base);
	// 8290AD68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290AD6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290AD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290AD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290AD78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290AD7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290AD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290AD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290AD88 size=196
    let mut pc: u32 = 0x8290AD88;
    'dispatch: loop {
        match pc {
            0x8290AD88 => {
    //   block [0x8290AD88..0x8290AE4C)
	// 8290AD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290AD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AD90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AD94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AD98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AD9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290ADA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290ADA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290ADA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290ADAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290ADB0: 4B9B5B89  bl 0x822c0938
	ctx.lr = 0x8290ADB4;
	sub_822C0938(ctx, base);
	// 8290ADB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290ADB8: 41820028  beq 0x8290ade0
	if ctx.cr[0].eq {
	pc = 0x8290ADE0; continue 'dispatch;
	}
	// 8290ADBC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290ADC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290ADC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290ADC8: 392B562C  addi r9, r11, 0x562c
	ctx.r[9].s64 = ctx.r[11].s64 + 22060;
	// 8290ADCC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290ADD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290ADD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290ADD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290ADDC: 48000008  b 0x8290ade4
	pc = 0x8290ADE4; continue 'dispatch;
	// 8290ADE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290ADE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290ADE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290ADEC: 409A0044  bne cr6, 0x8290ae30
	if !ctx.cr[6].eq {
	pc = 0x8290AE30; continue 'dispatch;
	}
	// 8290ADF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290ADF4: 419A001C  beq cr6, 0x8290ae10
	if ctx.cr[6].eq {
	pc = 0x8290AE10; continue 'dispatch;
	}
	// 8290ADF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290ADFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AE00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AE04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AE08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AE0C: 4E800421  bctrl
	ctx.lr = 0x8290AE10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AE10: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AE14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AE18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AE1C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AE20: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AE24: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AE28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AE2C: 4B9B51D5  bl 0x822c0000
	ctx.lr = 0x8290AE30;
	sub_822C0000(ctx, base);
	// 8290AE30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290AE34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290AE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290AE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290AE40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290AE44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290AE48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290AE50 size=196
    let mut pc: u32 = 0x8290AE50;
    'dispatch: loop {
        match pc {
            0x8290AE50 => {
    //   block [0x8290AE50..0x8290AF14)
	// 8290AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AE58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AE5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AE60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AE64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290AE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AE6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290AE70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290AE74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AE78: 4B9B5AC1  bl 0x822c0938
	ctx.lr = 0x8290AE7C;
	sub_822C0938(ctx, base);
	// 8290AE7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290AE80: 41820028  beq 0x8290aea8
	if ctx.cr[0].eq {
	pc = 0x8290AEA8; continue 'dispatch;
	}
	// 8290AE84: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290AE88: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290AE8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290AE90: 392B5640  addi r9, r11, 0x5640
	ctx.r[9].s64 = ctx.r[11].s64 + 22080;
	// 8290AE94: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290AE98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290AE9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290AEA0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290AEA4: 48000008  b 0x8290aeac
	pc = 0x8290AEAC; continue 'dispatch;
	// 8290AEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AEAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AEB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AEB4: 409A0044  bne cr6, 0x8290aef8
	if !ctx.cr[6].eq {
	pc = 0x8290AEF8; continue 'dispatch;
	}
	// 8290AEB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AEBC: 419A001C  beq cr6, 0x8290aed8
	if ctx.cr[6].eq {
	pc = 0x8290AED8; continue 'dispatch;
	}
	// 8290AEC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AEC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AECC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AED4: 4E800421  bctrl
	ctx.lr = 0x8290AED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AED8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AEDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AEE4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AEE8: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AEEC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AEF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AEF4: 4B9B510D  bl 0x822c0000
	ctx.lr = 0x8290AEF8;
	sub_822C0000(ctx, base);
	// 8290AEF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290AEFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290AF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290AF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290AF08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290AF0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290AF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290AF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290AF18 size=196
    let mut pc: u32 = 0x8290AF18;
    'dispatch: loop {
        match pc {
            0x8290AF18 => {
    //   block [0x8290AF18..0x8290AFDC)
	// 8290AF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290AF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AF20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AF24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AF28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AF2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290AF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AF34: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290AF38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290AF3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AF40: 4B9B59F9  bl 0x822c0938
	ctx.lr = 0x8290AF44;
	sub_822C0938(ctx, base);
	// 8290AF44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290AF48: 41820028  beq 0x8290af70
	if ctx.cr[0].eq {
	pc = 0x8290AF70; continue 'dispatch;
	}
	// 8290AF4C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290AF50: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290AF54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290AF58: 392B5654  addi r9, r11, 0x5654
	ctx.r[9].s64 = ctx.r[11].s64 + 22100;
	// 8290AF5C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290AF60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290AF64: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290AF68: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290AF6C: 48000008  b 0x8290af74
	pc = 0x8290AF74; continue 'dispatch;
	// 8290AF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AF74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290AF78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290AF7C: 409A0044  bne cr6, 0x8290afc0
	if !ctx.cr[6].eq {
	pc = 0x8290AFC0; continue 'dispatch;
	}
	// 8290AF80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290AF84: 419A001C  beq cr6, 0x8290afa0
	if ctx.cr[6].eq {
	pc = 0x8290AFA0; continue 'dispatch;
	}
	// 8290AF88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AF8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290AF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290AF94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290AF98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290AF9C: 4E800421  bctrl
	ctx.lr = 0x8290AFA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290AFA0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290AFA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290AFA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290AFAC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290AFB0: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290AFB4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290AFB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290AFBC: 4B9B5045  bl 0x822c0000
	ctx.lr = 0x8290AFC0;
	sub_822C0000(ctx, base);
	// 8290AFC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290AFC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290AFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290AFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290AFD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290AFD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290AFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290AFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290AFE0 size=196
    let mut pc: u32 = 0x8290AFE0;
    'dispatch: loop {
        match pc {
            0x8290AFE0 => {
    //   block [0x8290AFE0..0x8290B0A4)
	// 8290AFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290AFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290AFE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290AFEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290AFF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290AFF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290AFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290AFFC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290B000: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B004: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B008: 4B9B5931  bl 0x822c0938
	ctx.lr = 0x8290B00C;
	sub_822C0938(ctx, base);
	// 8290B00C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B010: 41820028  beq 0x8290b038
	if ctx.cr[0].eq {
	pc = 0x8290B038; continue 'dispatch;
	}
	// 8290B014: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B018: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290B01C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290B020: 392B5668  addi r9, r11, 0x5668
	ctx.r[9].s64 = ctx.r[11].s64 + 22120;
	// 8290B024: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290B028: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290B02C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290B030: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290B034: 48000008  b 0x8290b03c
	pc = 0x8290B03C; continue 'dispatch;
	// 8290B038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B03C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B040: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B044: 409A0044  bne cr6, 0x8290b088
	if !ctx.cr[6].eq {
	pc = 0x8290B088; continue 'dispatch;
	}
	// 8290B048: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B04C: 419A001C  beq cr6, 0x8290b068
	if ctx.cr[6].eq {
	pc = 0x8290B068; continue 'dispatch;
	}
	// 8290B050: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B054: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B05C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B064: 4E800421  bctrl
	ctx.lr = 0x8290B068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B068: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290B06C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290B070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B074: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290B078: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290B07C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290B080: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290B084: 4B9B4F7D  bl 0x822c0000
	ctx.lr = 0x8290B088;
	sub_822C0000(ctx, base);
	// 8290B088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290B08C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290B090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B098: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B09C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B0A8 size=196
    let mut pc: u32 = 0x8290B0A8;
    'dispatch: loop {
        match pc {
            0x8290B0A8 => {
    //   block [0x8290B0A8..0x8290B16C)
	// 8290B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B0B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290B0B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B0BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B0C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290B0C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B0CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B0D0: 4B9B5869  bl 0x822c0938
	ctx.lr = 0x8290B0D4;
	sub_822C0938(ctx, base);
	// 8290B0D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B0D8: 41820028  beq 0x8290b100
	if ctx.cr[0].eq {
	pc = 0x8290B100; continue 'dispatch;
	}
	// 8290B0DC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B0E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290B0E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290B0E8: 392B567C  addi r9, r11, 0x567c
	ctx.r[9].s64 = ctx.r[11].s64 + 22140;
	// 8290B0EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290B0F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290B0F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290B0F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290B0FC: 48000008  b 0x8290b104
	pc = 0x8290B104; continue 'dispatch;
	// 8290B100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B104: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B10C: 409A0044  bne cr6, 0x8290b150
	if !ctx.cr[6].eq {
	pc = 0x8290B150; continue 'dispatch;
	}
	// 8290B110: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B114: 419A001C  beq cr6, 0x8290b130
	if ctx.cr[6].eq {
	pc = 0x8290B130; continue 'dispatch;
	}
	// 8290B118: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B11C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B124: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B12C: 4E800421  bctrl
	ctx.lr = 0x8290B130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B130: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290B134: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290B138: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B13C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290B140: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290B144: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290B148: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290B14C: 4B9B4EB5  bl 0x822c0000
	ctx.lr = 0x8290B150;
	sub_822C0000(ctx, base);
	// 8290B150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290B154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290B158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B160: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B170 size=196
    let mut pc: u32 = 0x8290B170;
    'dispatch: loop {
        match pc {
            0x8290B170 => {
    //   block [0x8290B170..0x8290B234)
	// 8290B170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290B17C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B184: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B18C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290B190: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B194: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B198: 4B9B57A1  bl 0x822c0938
	ctx.lr = 0x8290B19C;
	sub_822C0938(ctx, base);
	// 8290B19C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B1A0: 41820028  beq 0x8290b1c8
	if ctx.cr[0].eq {
	pc = 0x8290B1C8; continue 'dispatch;
	}
	// 8290B1A4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B1A8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290B1AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290B1B0: 392B5690  addi r9, r11, 0x5690
	ctx.r[9].s64 = ctx.r[11].s64 + 22160;
	// 8290B1B4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290B1B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290B1BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290B1C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290B1C4: 48000008  b 0x8290b1cc
	pc = 0x8290B1CC; continue 'dispatch;
	// 8290B1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B1CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B1D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B1D4: 409A0044  bne cr6, 0x8290b218
	if !ctx.cr[6].eq {
	pc = 0x8290B218; continue 'dispatch;
	}
	// 8290B1D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B1DC: 419A001C  beq cr6, 0x8290b1f8
	if ctx.cr[6].eq {
	pc = 0x8290B1F8; continue 'dispatch;
	}
	// 8290B1E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B1E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B1E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B1EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B1F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B1F4: 4E800421  bctrl
	ctx.lr = 0x8290B1F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B1F8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290B1FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290B200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B204: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290B208: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290B20C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290B210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290B214: 4B9B4DED  bl 0x822c0000
	ctx.lr = 0x8290B218;
	sub_822C0000(ctx, base);
	// 8290B218: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290B21C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290B220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B228: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B22C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B238 size=196
    let mut pc: u32 = 0x8290B238;
    'dispatch: loop {
        match pc {
            0x8290B238 => {
    //   block [0x8290B238..0x8290B2FC)
	// 8290B238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290B244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B248: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B24C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B254: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290B258: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B25C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B260: 4B9B56D9  bl 0x822c0938
	ctx.lr = 0x8290B264;
	sub_822C0938(ctx, base);
	// 8290B264: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B268: 41820028  beq 0x8290b290
	if ctx.cr[0].eq {
	pc = 0x8290B290; continue 'dispatch;
	}
	// 8290B26C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B270: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290B274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290B278: 392B56A4  addi r9, r11, 0x56a4
	ctx.r[9].s64 = ctx.r[11].s64 + 22180;
	// 8290B27C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290B280: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290B284: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290B288: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290B28C: 48000008  b 0x8290b294
	pc = 0x8290B294; continue 'dispatch;
	// 8290B290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B294: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B29C: 409A0044  bne cr6, 0x8290b2e0
	if !ctx.cr[6].eq {
	pc = 0x8290B2E0; continue 'dispatch;
	}
	// 8290B2A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B2A4: 419A001C  beq cr6, 0x8290b2c0
	if ctx.cr[6].eq {
	pc = 0x8290B2C0; continue 'dispatch;
	}
	// 8290B2A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B2AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B2B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B2B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B2BC: 4E800421  bctrl
	ctx.lr = 0x8290B2C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B2C0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290B2C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290B2C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B2CC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290B2D0: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290B2D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290B2D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290B2DC: 4B9B4D25  bl 0x822c0000
	ctx.lr = 0x8290B2E0;
	sub_822C0000(ctx, base);
	// 8290B2E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290B2E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290B2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B2F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B2F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B300 size=196
    let mut pc: u32 = 0x8290B300;
    'dispatch: loop {
        match pc {
            0x8290B300 => {
    //   block [0x8290B300..0x8290B3C4)
	// 8290B300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290B30C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B314: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B31C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290B320: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B324: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B328: 4B9B5611  bl 0x822c0938
	ctx.lr = 0x8290B32C;
	sub_822C0938(ctx, base);
	// 8290B32C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B330: 41820028  beq 0x8290b358
	if ctx.cr[0].eq {
	pc = 0x8290B358; continue 'dispatch;
	}
	// 8290B334: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B338: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290B33C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290B340: 392B56B8  addi r9, r11, 0x56b8
	ctx.r[9].s64 = ctx.r[11].s64 + 22200;
	// 8290B344: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290B348: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290B34C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290B350: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290B354: 48000008  b 0x8290b35c
	pc = 0x8290B35C; continue 'dispatch;
	// 8290B358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B35C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B360: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B364: 409A0044  bne cr6, 0x8290b3a8
	if !ctx.cr[6].eq {
	pc = 0x8290B3A8; continue 'dispatch;
	}
	// 8290B368: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B36C: 419A001C  beq cr6, 0x8290b388
	if ctx.cr[6].eq {
	pc = 0x8290B388; continue 'dispatch;
	}
	// 8290B370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B374: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B37C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B384: 4E800421  bctrl
	ctx.lr = 0x8290B388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B388: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290B38C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290B390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B394: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290B398: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290B39C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290B3A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290B3A4: 4B9B4C5D  bl 0x822c0000
	ctx.lr = 0x8290B3A8;
	sub_822C0000(ctx, base);
	// 8290B3A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290B3AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290B3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B3B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B3BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B3C8 size=196
    let mut pc: u32 = 0x8290B3C8;
    'dispatch: loop {
        match pc {
            0x8290B3C8 => {
    //   block [0x8290B3C8..0x8290B48C)
	// 8290B3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290B3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B3DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B3E4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8290B3E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B3EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B3F0: 4B9B5549  bl 0x822c0938
	ctx.lr = 0x8290B3F4;
	sub_822C0938(ctx, base);
	// 8290B3F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B3F8: 41820028  beq 0x8290b420
	if ctx.cr[0].eq {
	pc = 0x8290B420; continue 'dispatch;
	}
	// 8290B3FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B400: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8290B404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8290B408: 392B56CC  addi r9, r11, 0x56cc
	ctx.r[9].s64 = ctx.r[11].s64 + 22220;
	// 8290B40C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290B410: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290B414: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8290B418: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8290B41C: 48000008  b 0x8290b424
	pc = 0x8290B424; continue 'dispatch;
	// 8290B420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290B424: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290B428: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B42C: 409A0044  bne cr6, 0x8290b470
	if !ctx.cr[6].eq {
	pc = 0x8290B470; continue 'dispatch;
	}
	// 8290B430: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B434: 419A001C  beq cr6, 0x8290b450
	if ctx.cr[6].eq {
	pc = 0x8290B450; continue 'dispatch;
	}
	// 8290B438: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B43C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B444: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B44C: 4E800421  bctrl
	ctx.lr = 0x8290B450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B450: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290B454: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290B458: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B45C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8290B460: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 8290B464: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8290B468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290B46C: 4B9B4B95  bl 0x822c0000
	ctx.lr = 0x8290B470;
	sub_822C0000(ctx, base);
	// 8290B470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290B474: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290B478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B480: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B490 size=84
    let mut pc: u32 = 0x8290B490;
    'dispatch: loop {
        match pc {
            0x8290B490 => {
    //   block [0x8290B490..0x8290B4E4)
	// 8290B490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B49C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B4A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B4A4: 80C302D0  lwz r6, 0x2d0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(720 as u32) ) } as u64;
	// 8290B4A8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B4AC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8290B4B0: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290B4B4: 38A00444  li r5, 0x444
	ctx.r[5].s64 = 1092;
	// 8290B4B8: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 8290B4BC: 4854D585  bl 0x82e58a40
	ctx.lr = 0x8290B4C0;
	sub_82E58A40(ctx, base);
	// 8290B4C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290B4C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290B4C8: 419A0008  beq cr6, 0x8290b4d0
	if ctx.cr[6].eq {
	pc = 0x8290B4D0; continue 'dispatch;
	}
	// 8290B4CC: 4B9B53C5  bl 0x822c0890
	ctx.lr = 0x8290B4D0;
	sub_822C0890(ctx, base);
	// 8290B4D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290B4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B4DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B4E8 size=84
    let mut pc: u32 = 0x8290B4E8;
    'dispatch: loop {
        match pc {
            0x8290B4E8 => {
    //   block [0x8290B4E8..0x8290B53C)
	// 8290B4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B4F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B4F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B4F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B4FC: 80C302D4  lwz r6, 0x2d4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(724 as u32) ) } as u64;
	// 8290B500: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B504: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8290B508: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290B50C: 38A0044A  li r5, 0x44a
	ctx.r[5].s64 = 1098;
	// 8290B510: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 8290B514: 4854D52D  bl 0x82e58a40
	ctx.lr = 0x8290B518;
	sub_82E58A40(ctx, base);
	// 8290B518: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290B51C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290B520: 419A0008  beq cr6, 0x8290b528
	if ctx.cr[6].eq {
	pc = 0x8290B528; continue 'dispatch;
	}
	// 8290B524: 4B9B536D  bl 0x822c0890
	ctx.lr = 0x8290B528;
	sub_822C0890(ctx, base);
	// 8290B528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290B52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B540 size=96
    let mut pc: u32 = 0x8290B540;
    'dispatch: loop {
        match pc {
            0x8290B540 => {
    //   block [0x8290B540..0x8290B5A0)
	// 8290B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B54C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B550: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290B554: 80C302D4  lwz r6, 0x2d4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(724 as u32) ) } as u64;
	// 8290B558: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B55C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8290B560: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290B564: 38A00450  li r5, 0x450
	ctx.r[5].s64 = 1104;
	// 8290B568: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 8290B56C: 4854BA7D  bl 0x82e56fe8
	ctx.lr = 0x8290B570;
	sub_82E56FE8(ctx, base);
	// 8290B570: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290B574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290B578: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B57C: 419A000C  beq cr6, 0x8290b588
	if ctx.cr[6].eq {
	pc = 0x8290B588; continue 'dispatch;
	}
	// 8290B580: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290B584: 4B9B530D  bl 0x822c0890
	ctx.lr = 0x8290B588;
	sub_822C0890(ctx, base);
	// 8290B588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B58C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290B590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290B5A0 size=92
    let mut pc: u32 = 0x8290B5A0;
    'dispatch: loop {
        match pc {
            0x8290B5A0 => {
    //   block [0x8290B5A0..0x8290B5FC)
	// 8290B5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B5A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B5AC: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B600 size=300
    let mut pc: u32 = 0x8290B600;
    'dispatch: loop {
        match pc {
            0x8290B600 => {
    //   block [0x8290B600..0x8290B72C)
	// 8290B600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B604: 4889CB65  bl 0x831a8168
	ctx.lr = 0x8290B608;
	sub_831A8130(ctx, base);
	// 8290B608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B60C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B610: 897E01E4  lbz r11, 0x1e4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(484 as u32) ) } as u64;
	// 8290B614: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B618: 4182010C  beq 0x8290b724
	if ctx.cr[0].eq {
	pc = 0x8290B724; continue 'dispatch;
	}
	// 8290B61C: 83FE0248  lwz r31, 0x248(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(584 as u32) ) } as u64;
	// 8290B620: 4BEE0BA9  bl 0x827ec1c8
	ctx.lr = 0x8290B624;
	sub_827EC1C8(ctx, base);
	// 8290B624: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290B628: 4BB83D79  bl 0x8248f3a0
	ctx.lr = 0x8290B62C;
	sub_8248F3A0(ctx, base);
	// 8290B62C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8290B630: 83FE01DC  lwz r31, 0x1dc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290B634: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290B638: 419A00E8  beq cr6, 0x8290b720
	if ctx.cr[6].eq {
	pc = 0x8290B720; continue 'dispatch;
	}
	// 8290B63C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290B640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B644: 4BC03ED5  bl 0x8250f518
	ctx.lr = 0x8290B648;
	sub_8250F518(ctx, base);
	// 8290B648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B64C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290B650: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8290B654: 409A0008  bne cr6, 0x8290b65c
	if !ctx.cr[6].eq {
	pc = 0x8290B65C; continue 'dispatch;
	}
	// 8290B658: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8290B65C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B660: 4BC1C9E1  bl 0x82528040
	ctx.lr = 0x8290B664;
	sub_82528040(ctx, base);
	// 8290B664: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290B668: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290B66C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B670: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B674: 4E800421  bctrl
	ctx.lr = 0x8290B678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B67C: 484E6615  bl 0x82df1c90
	ctx.lr = 0x8290B680;
	sub_82DF1C90(ctx, base);
	// 8290B680: 817E01DC  lwz r11, 0x1dc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290B684: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8290B688: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290B68C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290B690: 40990030  ble cr6, 0x8290b6c0
	if !ctx.cr[6].gt {
	pc = 0x8290B6C0; continue 'dispatch;
	}
	// 8290B694: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8290B698: 807E01DC  lwz r3, 0x1dc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290B69C: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290B6A0: 7C8BEAAE  lhax r4, r11, r29
	ctx.r[4].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as i16) as i64;
	// 8290B6A4: 4B9E88BD  bl 0x822f3f60
	ctx.lr = 0x8290B6A8;
	sub_822F3F60(ctx, base);
	// 8290B6A8: 817E01DC  lwz r11, 0x1dc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290B6AC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8290B6B0: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 8290B6B4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290B6B8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8290B6BC: 4198FFDC  blt cr6, 0x8290b698
	if ctx.cr[6].lt {
	pc = 0x8290B698; continue 'dispatch;
	}
	// 8290B6C0: 817E01DC  lwz r11, 0x1dc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290B6C4: 3BEB0058  addi r31, r11, 0x58
	ctx.r[31].s64 = ctx.r[11].s64 + 88;
	// 8290B6C8: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8290B6CC: 556B00BF  clrlwi. r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290B6D0: 4080001C  bge 0x8290b6ec
	if !ctx.cr[0].lt {
	pc = 0x8290B6EC; continue 'dispatch;
	}
	// 8290B6D4: 5564083D  rlwinm. r4, r11, 1, 0, 0x1e
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8290B6D8: 41810008  bgt 0x8290b6e0
	if ctx.cr[0].gt {
	pc = 0x8290B6E0; continue 'dispatch;
	}
	// 8290B6DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8290B6E0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8290B6E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B6E8: 4859B111  bl 0x82ea67f8
	ctx.lr = 0x8290B6EC;
	sub_82EA67F8(ctx, base);
	// 8290B6EC: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8290B6F0: 817E01DC  lwz r11, 0x1dc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(476 as u32) ) } as u64;
	// 8290B6F4: 3BEB0064  addi r31, r11, 0x64
	ctx.r[31].s64 = ctx.r[11].s64 + 100;
	// 8290B6F8: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290B6FC: 556B00BF  clrlwi. r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290B700: 4080001C  bge 0x8290b71c
	if !ctx.cr[0].lt {
	pc = 0x8290B71C; continue 'dispatch;
	}
	// 8290B704: 5564083D  rlwinm. r4, r11, 1, 0, 0x1e
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8290B708: 41810008  bgt 0x8290b710
	if ctx.cr[0].gt {
	pc = 0x8290B710; continue 'dispatch;
	}
	// 8290B70C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8290B710: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 8290B714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B718: 4859B0E1  bl 0x82ea67f8
	ctx.lr = 0x8290B71C;
	sub_82EA67F8(ctx, base);
	// 8290B71C: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8290B720: 9B9E01E4  stb r28, 0x1e4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(484 as u32), ctx.r[28].u8 ) };
	// 8290B724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290B728: 4889CA90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B730 size=156
    let mut pc: u32 = 0x8290B730;
    'dispatch: loop {
        match pc {
            0x8290B730 => {
    //   block [0x8290B730..0x8290B7CC)
	// 8290B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B73C: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290B744: 897F01F0  lbz r11, 0x1f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 8290B748: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B74C: 4082006C  bne 0x8290b7b8
	if !ctx.cr[0].eq {
	pc = 0x8290B7B8; continue 'dispatch;
	}
	// 8290B750: 4BEE0A79  bl 0x827ec1c8
	ctx.lr = 0x8290B754;
	sub_827EC1C8(ctx, base);
	// 8290B754: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290B758: 41820010  beq 0x8290b768
	if ctx.cr[0].eq {
	pc = 0x8290B768; continue 'dispatch;
	}
	// 8290B75C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8290B760: 808B666C  lwz r4, 0x666c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26220 as u32) ) } as u64;
	// 8290B764: 4BB83C3D  bl 0x8248f3a0
	ctx.lr = 0x8290B768;
	sub_8248F3A0(ctx, base);
	// 8290B768: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290B76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B770: 4BEE09E9  bl 0x827ec158
	ctx.lr = 0x8290B774;
	sub_827EC158(ctx, base);
	// 8290B774: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8290B778: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 8290B77C: 4B9E2285  bl 0x822eda00
	ctx.lr = 0x8290B780;
	sub_822EDA00(ctx, base);
	// 8290B780: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 8290B784: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290B788: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8290B78C: 4B9B9175  bl 0x822c4900
	ctx.lr = 0x8290B790;
	sub_822C4900(ctx, base);
	// 8290B790: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290B794: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 8290B798: 4B9E2189  bl 0x822ed920
	ctx.lr = 0x8290B79C;
	sub_822ED920(ctx, base);
	// 8290B79C: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 8290B7A0: 4B9E1951  bl 0x822ed0f0
	ctx.lr = 0x8290B7A4;
	sub_822ED0F0(ctx, base);
	// 8290B7A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8290B7A8: 807F01E8  lwz r3, 0x1e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 8290B7AC: 4BB83B9D  bl 0x8248f348
	ctx.lr = 0x8290B7B0;
	sub_8248F348(ctx, base);
	// 8290B7B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290B7B4: 997F01F0  stb r11, 0x1f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[11].u8 ) };
	// 8290B7B8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8290B7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290B7D0 size=144
    let mut pc: u32 = 0x8290B7D0;
    'dispatch: loop {
        match pc {
            0x8290B7D0 => {
    //   block [0x8290B7D0..0x8290B860)
	// 8290B7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290B7D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290B7DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290B7E0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B7E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290B7E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290B7EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B7F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B7F4: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8290B7F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B7FC: 4E800421  bctrl
	ctx.lr = 0x8290B800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B800: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B804: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290B808: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290B80C: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 8290B810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B814: 4E800421  bctrl
	ctx.lr = 0x8290B818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B818: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8290B81C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290B820: 48570679  bl 0x82e7be98
	ctx.lr = 0x8290B824;
	sub_82E7BE98(ctx, base);
	// 8290B824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290B828: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290B82C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8290B830: 485707A9  bl 0x82e7bfd8
	ctx.lr = 0x8290B834;
	sub_82E7BFD8(ctx, base);
	// 8290B834: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290B838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B83C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290B840: 4B9B90C1  bl 0x822c4900
	ctx.lr = 0x8290B844;
	sub_822C4900(ctx, base);
	// 8290B844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B848: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8290B84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290B850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290B854: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290B858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290B85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290B860 size=12
    let mut pc: u32 = 0x8290B860;
    'dispatch: loop {
        match pc {
            0x8290B860 => {
    //   block [0x8290B860..0x8290B86C)
	// 8290B860: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290B864: 41820008  beq 0x8290b86c
	if ctx.cr[0].eq {
		sub_8290B86C(ctx, base);
		return;
	}
	// 8290B868: 4BFFDE00  b 0x82909668
	sub_82909668(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B86C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290B86C size=4
    let mut pc: u32 = 0x8290B86C;
    'dispatch: loop {
        match pc {
            0x8290B86C => {
    //   block [0x8290B86C..0x8290B870)
	// 8290B86C: 4BFFFD94  b 0x8290b600
	sub_8290B600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290B870 size=428
    let mut pc: u32 = 0x8290B870;
    'dispatch: loop {
        match pc {
            0x8290B870 => {
    //   block [0x8290B870..0x8290BA1C)
	// 8290B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290B874: 4889C8F9  bl 0x831a816c
	ctx.lr = 0x8290B878;
	sub_831A8130(ctx, base);
	// 8290B878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290B87C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290B880: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290B884: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290B888: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8290B88C: C1BF032C  lfs f13, 0x32c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290B890: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290B894: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8290B898: 41990060  bgt cr6, 0x8290b8f8
	if ctx.cr[6].gt {
	pc = 0x8290B8F8; continue 'dispatch;
	}
	// 8290B89C: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 8290B8A0: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290B8A4: 4800B78D  bl 0x82917030
	ctx.lr = 0x8290B8A8;
	sub_82917030(ctx, base);
	// 8290B8A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290B8AC: 4082004C  bne 0x8290b8f8
	if !ctx.cr[0].eq {
	pc = 0x8290B8F8; continue 'dispatch;
	}
	// 8290B8B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B8B4: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290B8B8: 38DD0040  addi r6, r29, 0x40
	ctx.r[6].s64 = ctx.r[29].s64 + 64;
	// 8290B8BC: 38AA5804  addi r5, r10, 0x5804
	ctx.r[5].s64 = ctx.r[10].s64 + 22532;
	// 8290B8C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290B8C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290B8C8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8290B8CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B8D0: 4E800421  bctrl
	ctx.lr = 0x8290B8D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B8D4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290B8D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290B8DC: 419A0008  beq cr6, 0x8290b8e4
	if ctx.cr[6].eq {
	pc = 0x8290B8E4; continue 'dispatch;
	}
	// 8290B8E0: 4B9B4FB1  bl 0x822c0890
	ctx.lr = 0x8290B8E4;
	sub_822C0890(ctx, base);
	// 8290B8E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290B8E8: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290B8EC: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 8290B8F0: 4800B731  bl 0x82917020
	ctx.lr = 0x8290B8F4;
	sub_82917020(ctx, base);
	// 8290B8F4: 48000120  b 0x8290ba14
	pc = 0x8290BA14; continue 'dispatch;
	// 8290B8F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8290B8FC: 419A0028  beq cr6, 0x8290b924
	if ctx.cr[6].eq {
	pc = 0x8290B924; continue 'dispatch;
	}
	// 8290B900: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B904: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290B908: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290B90C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290B910: 816B0100  lwz r11, 0x100(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 8290B914: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290B918: 4E800421  bctrl
	ctx.lr = 0x8290B91C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290B91C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290B920: 408200F4  bne 0x8290ba14
	if !ctx.cr[0].eq {
	pc = 0x8290BA14; continue 'dispatch;
	}
	// 8290B924: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8290B928: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290B92C: 814A85F8  lwz r10, -0x7a08(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31240 as u32) ) } as u64;
	// 8290B930: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8290B934: 419A0078  beq cr6, 0x8290b9ac
	if ctx.cr[6].eq {
	pc = 0x8290B9AC; continue 'dispatch;
	}
	// 8290B938: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8290B93C: 814A85F4  lwz r10, -0x7a0c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31244 as u32) ) } as u64;
	// 8290B940: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8290B944: 419A0068  beq cr6, 0x8290b9ac
	if ctx.cr[6].eq {
	pc = 0x8290B9AC; continue 'dispatch;
	}
	// 8290B948: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8290B94C: 814A85CC  lwz r10, -0x7a34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31284 as u32) ) } as u64;
	// 8290B950: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8290B954: 419A0058  beq cr6, 0x8290b9ac
	if ctx.cr[6].eq {
	pc = 0x8290B9AC; continue 'dispatch;
	}
	// 8290B958: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8290B95C: 814A85C8  lwz r10, -0x7a38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31288 as u32) ) } as u64;
	// 8290B960: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8290B964: 419A0048  beq cr6, 0x8290b9ac
	if ctx.cr[6].eq {
	pc = 0x8290B9AC; continue 'dispatch;
	}
	// 8290B968: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8290B96C: 814A85C4  lwz r10, -0x7a3c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31292 as u32) ) } as u64;
	// 8290B970: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8290B974: 419A0038  beq cr6, 0x8290b9ac
	if ctx.cr[6].eq {
	pc = 0x8290B9AC; continue 'dispatch;
	}
	// 8290B978: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290B97C: 48017075  bl 0x829229f0
	ctx.lr = 0x8290B980;
	sub_829229F0(ctx, base);
	// 8290B980: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290B984: 40820010  bne 0x8290b994
	if !ctx.cr[0].eq {
	pc = 0x8290B994; continue 'dispatch;
	}
	// 8290B988: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B98C: 38AB57EC  addi r5, r11, 0x57ec
	ctx.r[5].s64 = ctx.r[11].s64 + 22508;
	// 8290B990: 48000058  b 0x8290b9e8
	pc = 0x8290B9E8; continue 'dispatch;
	// 8290B994: 817F0244  lwz r11, 0x244(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290B998: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 8290B99C: 419A0044  beq cr6, 0x8290b9e0
	if ctx.cr[6].eq {
	pc = 0x8290B9E0; continue 'dispatch;
	}
	// 8290B9A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8290B9A4: 38AB3ABC  addi r5, r11, 0x3abc
	ctx.r[5].s64 = ctx.r[11].s64 + 15036;
	// 8290B9A8: 48000040  b 0x8290b9e8
	pc = 0x8290B9E8; continue 'dispatch;
	// 8290B9AC: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290B9B0: 48017041  bl 0x829229f0
	ctx.lr = 0x8290B9B4;
	sub_829229F0(ctx, base);
	// 8290B9B4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290B9B8: 40820010  bne 0x8290b9c8
	if !ctx.cr[0].eq {
	pc = 0x8290B9C8; continue 'dispatch;
	}
	// 8290B9BC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B9C0: 38AB57D4  addi r5, r11, 0x57d4
	ctx.r[5].s64 = ctx.r[11].s64 + 22484;
	// 8290B9C4: 48000024  b 0x8290b9e8
	pc = 0x8290B9E8; continue 'dispatch;
	// 8290B9C8: 817F0244  lwz r11, 0x244(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290B9CC: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 8290B9D0: 409A0010  bne cr6, 0x8290b9e0
	if !ctx.cr[6].eq {
	pc = 0x8290B9E0; continue 'dispatch;
	}
	// 8290B9D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B9D8: 38AB57C0  addi r5, r11, 0x57c0
	ctx.r[5].s64 = ctx.r[11].s64 + 22464;
	// 8290B9DC: 4800000C  b 0x8290b9e8
	pc = 0x8290B9E8; continue 'dispatch;
	// 8290B9E0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290B9E4: 38AB57A8  addi r5, r11, 0x57a8
	ctx.r[5].s64 = ctx.r[11].s64 + 22440;
	// 8290B9E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290B9EC: 38DD0040  addi r6, r29, 0x40
	ctx.r[6].s64 = ctx.r[29].s64 + 64;
	// 8290B9F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290B9F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290B9F8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8290B9FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BA00: 4E800421  bctrl
	ctx.lr = 0x8290BA04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BA04: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290BA08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BA0C: 419A0008  beq cr6, 0x8290ba14
	if ctx.cr[6].eq {
	pc = 0x8290BA14; continue 'dispatch;
	}
	// 8290BA10: 4B9B4E81  bl 0x822c0890
	ctx.lr = 0x8290BA14;
	sub_822C0890(ctx, base);
	// 8290BA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290BA18: 4889C7A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290BA20 size=204
    let mut pc: u32 = 0x8290BA20;
    'dispatch: loop {
        match pc {
            0x8290BA20 => {
    //   block [0x8290BA20..0x8290BAEC)
	// 8290BA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290BA24: 4889C749  bl 0x831a816c
	ctx.lr = 0x8290BA28;
	sub_831A8130(ctx, base);
	// 8290BA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290BA2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290BA30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290BA34: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290BA38: C1BE032C  lfs f13, 0x32c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(812 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290BA3C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290BA40: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8290BA44: 41990034  bgt cr6, 0x8290ba78
	if ctx.cr[6].gt {
	pc = 0x8290BA78; continue 'dispatch;
	}
	// 8290BA48: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 8290BA4C: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290BA50: 4800B5E1  bl 0x82917030
	ctx.lr = 0x8290BA54;
	sub_82917030(ctx, base);
	// 8290BA54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290BA58: 40820020  bne 0x8290ba78
	if !ctx.cr[0].eq {
	pc = 0x8290BA78; continue 'dispatch;
	}
	// 8290BA5C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290BA60: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290BA64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290BA68: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 8290BA6C: 3BEB5804  addi r31, r11, 0x5804
	ctx.r[31].s64 = ctx.r[11].s64 + 22532;
	// 8290BA70: 4800B5B1  bl 0x82917020
	ctx.lr = 0x8290BA74;
	sub_82917020(ctx, base);
	// 8290BA74: 48000040  b 0x8290bab4
	pc = 0x8290BAB4; continue 'dispatch;
	// 8290BA78: 807E0244  lwz r3, 0x244(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290BA7C: 48016F75  bl 0x829229f0
	ctx.lr = 0x8290BA80;
	sub_829229F0(ctx, base);
	// 8290BA80: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290BA84: 40820010  bne 0x8290ba94
	if !ctx.cr[0].eq {
	pc = 0x8290BA94; continue 'dispatch;
	}
	// 8290BA88: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290BA8C: 3BEB57EC  addi r31, r11, 0x57ec
	ctx.r[31].s64 = ctx.r[11].s64 + 22508;
	// 8290BA90: 48000024  b 0x8290bab4
	pc = 0x8290BAB4; continue 'dispatch;
	// 8290BA94: 817E0244  lwz r11, 0x244(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290BA98: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 8290BA9C: 409A0010  bne cr6, 0x8290baac
	if !ctx.cr[6].eq {
	pc = 0x8290BAAC; continue 'dispatch;
	}
	// 8290BAA0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290BAA4: 3BEB57A8  addi r31, r11, 0x57a8
	ctx.r[31].s64 = ctx.r[11].s64 + 22440;
	// 8290BAA8: 4800000C  b 0x8290bab4
	pc = 0x8290BAB4; continue 'dispatch;
	// 8290BAAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8290BAB0: 3BEB3ABC  addi r31, r11, 0x3abc
	ctx.r[31].s64 = ctx.r[11].s64 + 15036;
	// 8290BAB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BAB8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8290BABC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290BAC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290BAC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290BAC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8290BACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BAD0: 4E800421  bctrl
	ctx.lr = 0x8290BAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BAD4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290BAD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BADC: 419A0008  beq cr6, 0x8290bae4
	if ctx.cr[6].eq {
	pc = 0x8290BAE4; continue 'dispatch;
	}
	// 8290BAE0: 4B9B4DB1  bl 0x822c0890
	ctx.lr = 0x8290BAE4;
	sub_822C0890(ctx, base);
	// 8290BAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290BAE8: 4889C6D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290BAF0 size=80
    let mut pc: u32 = 0x8290BAF0;
    'dispatch: loop {
        match pc {
            0x8290BAF0 => {
    //   block [0x8290BAF0..0x8290BB40)
	// 8290BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290BAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290BAFC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290BB00: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BB04: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8290BB08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BB0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290BB10: 80ABE014  lwz r5, -0x1fec(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8172 as u32) ) } as u64;
	// 8290BB14: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 8290BB18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BB1C: 4E800421  bctrl
	ctx.lr = 0x8290BB20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BB20: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290BB24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BB28: 419A0008  beq cr6, 0x8290bb30
	if ctx.cr[6].eq {
	pc = 0x8290BB30; continue 'dispatch;
	}
	// 8290BB2C: 4B9B4D65  bl 0x822c0890
	ctx.lr = 0x8290BB30;
	sub_822C0890(ctx, base);
	// 8290BB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290BB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290BB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290BB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290BB40 size=460
    let mut pc: u32 = 0x8290BB40;
    'dispatch: loop {
        match pc {
            0x8290BB40 => {
    //   block [0x8290BB40..0x8290BD0C)
	// 8290BB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290BB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290BB48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290BB4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290BB50: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290BB54: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290BB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290BB5C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8290BB60: 409A0160  bne cr6, 0x8290bcc0
	if !ctx.cr[6].eq {
	pc = 0x8290BCC0; continue 'dispatch;
	}
	// 8290BB64: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8290BB68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8290BB6C: 409A0154  bne cr6, 0x8290bcc0
	if !ctx.cr[6].eq {
	pc = 0x8290BCC0; continue 'dispatch;
	}
	// 8290BB70: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BB74: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8290BB78: 4198013C  blt cr6, 0x8290bcb4
	if ctx.cr[6].lt {
	pc = 0x8290BCB4; continue 'dispatch;
	}
	// 8290BB7C: 419A0120  beq cr6, 0x8290bc9c
	if ctx.cr[6].eq {
	pc = 0x8290BC9C; continue 'dispatch;
	}
	// 8290BB80: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8290BB84: 40980170  bge cr6, 0x8290bcf4
	if !ctx.cr[6].lt {
	pc = 0x8290BCF4; continue 'dispatch;
	}
	// 8290BB88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BB8C: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290BB90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BB94: 4E800421  bctrl
	ctx.lr = 0x8290BB98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BB98: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290BB9C: 41820158  beq 0x8290bcf4
	if ctx.cr[0].eq {
	pc = 0x8290BCF4; continue 'dispatch;
	}
	// 8290BBA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290BBA4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BBA8: 484E7E61  bl 0x82df3a08
	ctx.lr = 0x8290BBAC;
	sub_82DF3A08(ctx, base);
	// 8290BBAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290BBB0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290BBB4: 4BEE0A15  bl 0x827ec5c8
	ctx.lr = 0x8290BBB8;
	sub_827EC5C8(ctx, base);
	// 8290BBB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290BBBC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8290BBC0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290BBC4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BBC8: 48509469  bl 0x82e15030
	ctx.lr = 0x8290BBCC;
	sub_82E15030(ctx, base);
	// 8290BBCC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290BBD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BBD4: 419A0008  beq cr6, 0x8290bbdc
	if ctx.cr[6].eq {
	pc = 0x8290BBDC; continue 'dispatch;
	}
	// 8290BBD8: 4B9B4CB9  bl 0x822c0890
	ctx.lr = 0x8290BBDC;
	sub_822C0890(ctx, base);
	// 8290BBDC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290BBE0: 484E7849  bl 0x82df3428
	ctx.lr = 0x8290BBE4;
	sub_82DF3428(ctx, base);
	// 8290BBE4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290BBE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BBEC: 419A009C  beq cr6, 0x8290bc88
	if ctx.cr[6].eq {
	pc = 0x8290BC88; continue 'dispatch;
	}
	// 8290BBF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BBF4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8290BBF8: 3BCA6910  addi r30, r10, 0x6910
	ctx.r[30].s64 = ctx.r[10].s64 + 26896;
	// 8290BBFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290BC00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BC04: 4E800421  bctrl
	ctx.lr = 0x8290BC08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BC08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BC0C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290BC10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290BC14: 485700B5  bl 0x82e7bcc8
	ctx.lr = 0x8290BC18;
	sub_82E7BCC8(ctx, base);
	// 8290BC18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BC1C: 48107A65  bl 0x82a13680
	ctx.lr = 0x8290BC20;
	sub_82A13680(ctx, base);
	// 8290BC20: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290BC24: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8290BC28: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290BC2C: 48016DC5  bl 0x829229f0
	ctx.lr = 0x8290BC30;
	sub_829229F0(ctx, base);
	// 8290BC30: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290BC34: 40820010  bne 0x8290bc44
	if !ctx.cr[0].eq {
	pc = 0x8290BC44; continue 'dispatch;
	}
	// 8290BC38: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290BC3C: 388B5838  addi r4, r11, 0x5838
	ctx.r[4].s64 = ctx.r[11].s64 + 22584;
	// 8290BC40: 4800000C  b 0x8290bc4c
	pc = 0x8290BC4C; continue 'dispatch;
	// 8290BC44: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290BC48: 388B5824  addi r4, r11, 0x5824
	ctx.r[4].s64 = ctx.r[11].s64 + 22564;
	// 8290BC4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290BC50: 484E7DB9  bl 0x82df3a08
	ctx.lr = 0x8290BC54;
	sub_82DF3A08(ctx, base);
	// 8290BC54: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8290BC58: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8290BC5C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290BC60: 48570379  bl 0x82e7bfd8
	ctx.lr = 0x8290BC64;
	sub_82E7BFD8(ctx, base);
	// 8290BC64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290BC68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BC6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BC70: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290BC74: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290BC78: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290BC7C: 4BEE0585  bl 0x827ec200
	ctx.lr = 0x8290BC80;
	sub_827EC200(ctx, base);
	// 8290BC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290BC84: 484E77A5  bl 0x82df3428
	ctx.lr = 0x8290BC88;
	sub_82DF3428(ctx, base);
	// 8290BC88: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290BC8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BC90: 419A0064  beq cr6, 0x8290bcf4
	if ctx.cr[6].eq {
	pc = 0x8290BCF4; continue 'dispatch;
	}
	// 8290BC94: 4B9B4BFD  bl 0x822c0890
	ctx.lr = 0x8290BC98;
	sub_822C0890(ctx, base);
	// 8290BC98: 4800005C  b 0x8290bcf4
	pc = 0x8290BCF4; continue 'dispatch;
	// 8290BC9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BCA0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BCA4: 481079DD  bl 0x82a13680
	ctx.lr = 0x8290BCA8;
	sub_82A13680(ctx, base);
	// 8290BCA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BCAC: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8290BCB0: 48000024  b 0x8290bcd4
	pc = 0x8290BCD4; continue 'dispatch;
	// 8290BCB4: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8290BCB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290BCBC: 41820038  beq 0x8290bcf4
	if ctx.cr[0].eq {
	pc = 0x8290BCF4; continue 'dispatch;
	}
	// 8290BCC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BCC4: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BCC8: 481079B9  bl 0x82a13680
	ctx.lr = 0x8290BCCC;
	sub_82A13680(ctx, base);
	// 8290BCCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BCD0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290BCD4: 48570305  bl 0x82e7bfd8
	ctx.lr = 0x8290BCD8;
	sub_82E7BFD8(ctx, base);
	// 8290BCD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BCDC: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290BCE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BCE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290BCE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290BCEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BCF0: 4E800421  bctrl
	ctx.lr = 0x8290BCF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BCF4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8290BCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290BCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290BD00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290BD04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290BD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290BD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290BD10 size=148
    let mut pc: u32 = 0x8290BD10;
    'dispatch: loop {
        match pc {
            0x8290BD10 => {
    //   block [0x8290BD10..0x8290BDA4)
	// 8290BD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290BD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290BD18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290BD1C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8290BD20: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8290BD24: 814A0B6C  lwz r10, 0xb6c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2924 as u32) ) } as u64;
	// 8290BD28: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BD2C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BD30: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290BD34: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8290BD38: 41820014  beq 0x8290bd4c
	if ctx.cr[0].eq {
	pc = 0x8290BD4C; continue 'dispatch;
	}
	// 8290BD3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8290BD40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290BD44: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8290BD48: 419AFFE0  beq cr6, 0x8290bd28
	if ctx.cr[6].eq {
	pc = 0x8290BD28; continue 'dispatch;
	}
	// 8290BD4C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290BD50: 40820040  bne 0x8290bd90
	if !ctx.cr[0].eq {
	pc = 0x8290BD90; continue 'dispatch;
	}
	// 8290BD54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BD58: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290BD5C: 38C40040  addi r6, r4, 0x40
	ctx.r[6].s64 = ctx.r[4].s64 + 64;
	// 8290BD60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BD64: 38AA4924  addi r5, r10, 0x4924
	ctx.r[5].s64 = ctx.r[10].s64 + 18724;
	// 8290BD68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290BD6C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8290BD70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BD74: 4E800421  bctrl
	ctx.lr = 0x8290BD78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BD78: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290BD7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BD80: 419A0008  beq cr6, 0x8290bd88
	if ctx.cr[6].eq {
	pc = 0x8290BD88; continue 'dispatch;
	}
	// 8290BD84: 4B9B4B0D  bl 0x822c0890
	ctx.lr = 0x8290BD88;
	sub_822C0890(ctx, base);
	// 8290BD88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8290BD8C: 48000008  b 0x8290bd94
	pc = 0x8290BD94; continue 'dispatch;
	// 8290BD90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290BD94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8290BD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290BD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290BDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290BDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290BDA8 size=424
    let mut pc: u32 = 0x8290BDA8;
    'dispatch: loop {
        match pc {
            0x8290BDA8 => {
    //   block [0x8290BDA8..0x8290BF50)
	// 8290BDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290BDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290BDB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290BDB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290BDB8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290BDBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290BDC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290BDC4: C01F032C  lfs f0, 0x32c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290BDC8: D01E0018  stfs f0, 0x18(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8290BDCC: C01F0330  lfs f0, 0x330(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(816 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290BDD0: D01E001C  stfs f0, 0x1c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8290BDD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BDD8: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290BDDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BDE0: 4E800421  bctrl
	ctx.lr = 0x8290BDE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BDE4: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290BDE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290BDEC: 4182009C  beq 0x8290be88
	if ctx.cr[0].eq {
	pc = 0x8290BE88; continue 'dispatch;
	}
	// 8290BDF0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290BDF4: 484E7C15  bl 0x82df3a08
	ctx.lr = 0x8290BDF8;
	sub_82DF3A08(ctx, base);
	// 8290BDF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BDFC: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8290BE00: 4BEE03D9  bl 0x827ec1d8
	ctx.lr = 0x8290BE04;
	sub_827EC1D8(ctx, base);
	// 8290BE04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290BE08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290BE0C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290BE10: 48509221  bl 0x82e15030
	ctx.lr = 0x8290BE14;
	sub_82E15030(ctx, base);
	// 8290BE14: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BE18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BE1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290BE20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BE24: 4E800421  bctrl
	ctx.lr = 0x8290BE28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BE28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290BE2C: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
	// 8290BE30: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290BE34: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8290BE38: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8290BE3C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8290BE40: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 8290BE44: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290BE48: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 8290BE4C: 13DF5C07  vcmpneb. (lvlx128) v30, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290BE50: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 8290BE54: 13AA5C07  vcmpneb. (lvlx128) v29, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290BE58: 13895C07  vcmpneb. (lvlx128) v28, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290BE5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290BF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290BF50 size=780
    let mut pc: u32 = 0x8290BF50;
    'dispatch: loop {
        match pc {
            0x8290BF50 => {
    //   block [0x8290BF50..0x8290C25C)
	// 8290BF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290BF54: 4889C219  bl 0x831a816c
	ctx.lr = 0x8290BF58;
	sub_831A8130(ctx, base);
	// 8290BF58: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8290BF5C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290BF60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290BF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290BF68: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8290BF6C: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290BF70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290BF74: 41820258  beq 0x8290c1cc
	if ctx.cr[0].eq {
	pc = 0x8290C1CC; continue 'dispatch;
	}
	// 8290BF78: 83BF00E4  lwz r29, 0xe4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290BF7C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290BF80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290BF84: 4800B09D  bl 0x82917020
	ctx.lr = 0x8290BF88;
	sub_82917020(ctx, base);
	// 8290BF88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290BF8C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290BF90: 4800AE09  bl 0x82916d98
	ctx.lr = 0x8290BF94;
	sub_82916D98(ctx, base);
	// 8290BF94: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290BF98: C01E001C  lfs f0, 0x1c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290BF9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290BFA0: 38AB5888  addi r5, r11, 0x5888
	ctx.r[5].s64 = ctx.r[11].s64 + 22664;
	// 8290BFA4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290BFA8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290BFAC: D00B00FC  stfs f0, 0xfc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8290BFB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BFB4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8290BFB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BFBC: 4E800421  bctrl
	ctx.lr = 0x8290BFC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BFC0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290BFC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290BFC8: 419A0008  beq cr6, 0x8290bfd0
	if ctx.cr[6].eq {
	pc = 0x8290BFD0; continue 'dispatch;
	}
	// 8290BFCC: 4B9B48C5  bl 0x822c0890
	ctx.lr = 0x8290BFD0;
	sub_822C0890(ctx, base);
	// 8290BFD0: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290BFD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BFD8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8290BFDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BFE0: 4E800421  bctrl
	ctx.lr = 0x8290BFE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BFE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290BFE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290BFEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290BFF0: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290BFF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290BFF8: 4E800421  bctrl
	ctx.lr = 0x8290BFFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290BFFC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290C000: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8290C004: 388B5870  addi r4, r11, 0x5870
	ctx.r[4].s64 = ctx.r[11].s64 + 22640;
	// 8290C008: 41820088  beq 0x8290c090
	if ctx.cr[0].eq {
	pc = 0x8290C090; continue 'dispatch;
	}
	// 8290C00C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C010: 484E79F9  bl 0x82df3a08
	ctx.lr = 0x8290C014;
	sub_82DF3A08(ctx, base);
	// 8290C014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C018: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290C01C: 484E79ED  bl 0x82df3a08
	ctx.lr = 0x8290C020;
	sub_82DF3A08(ctx, base);
	// 8290C020: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C024: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290C028: 4BEE05A1  bl 0x827ec5c8
	ctx.lr = 0x8290C02C;
	sub_827EC5C8(ctx, base);
	// 8290C02C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C030: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290C034: C3FE0000  lfs f31, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290C038: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290C03C: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 8290C040: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C044: 48508FED  bl 0x82e15030
	ctx.lr = 0x8290C048;
	sub_82E15030(ctx, base);
	// 8290C048: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C04C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290C050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C054: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C058: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290C05C: 4BEE019D  bl 0x827ec1f8
	ctx.lr = 0x8290C060;
	sub_827EC1F8(ctx, base);
	// 8290C060: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290C064: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C068: 419A0008  beq cr6, 0x8290c070
	if ctx.cr[6].eq {
	pc = 0x8290C070; continue 'dispatch;
	}
	// 8290C06C: 4B9B4825  bl 0x822c0890
	ctx.lr = 0x8290C070;
	sub_822C0890(ctx, base);
	// 8290C070: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8290C074: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C078: 419A0008  beq cr6, 0x8290c080
	if ctx.cr[6].eq {
	pc = 0x8290C080; continue 'dispatch;
	}
	// 8290C07C: 4B9B4815  bl 0x822c0890
	ctx.lr = 0x8290C080;
	sub_822C0890(ctx, base);
	// 8290C080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C084: 484E73A5  bl 0x82df3428
	ctx.lr = 0x8290C088;
	sub_82DF3428(ctx, base);
	// 8290C088: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C08C: 48000088  b 0x8290c114
	pc = 0x8290C114; continue 'dispatch;
	// 8290C090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C094: 484E7975  bl 0x82df3a08
	ctx.lr = 0x8290C098;
	sub_82DF3A08(ctx, base);
	// 8290C098: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290C09C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C0A0: 388BC604  addi r4, r11, -0x39fc
	ctx.r[4].s64 = ctx.r[11].s64 + -14844;
	// 8290C0A4: 484E7965  bl 0x82df3a08
	ctx.lr = 0x8290C0A8;
	sub_82DF3A08(ctx, base);
	// 8290C0A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C0AC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8290C0B0: 4BEE0519  bl 0x827ec5c8
	ctx.lr = 0x8290C0B4;
	sub_827EC5C8(ctx, base);
	// 8290C0B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C0B8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8290C0BC: C3FE0000  lfs f31, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290C0C0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290C0C4: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290C0C8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C0CC: 48508F65  bl 0x82e15030
	ctx.lr = 0x8290C0D0;
	sub_82E15030(ctx, base);
	// 8290C0D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C0D4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290C0D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C0DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C0E0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290C0E4: 4BEE0115  bl 0x827ec1f8
	ctx.lr = 0x8290C0E8;
	sub_827EC1F8(ctx, base);
	// 8290C0E8: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8290C0EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C0F0: 419A0008  beq cr6, 0x8290c0f8
	if ctx.cr[6].eq {
	pc = 0x8290C0F8; continue 'dispatch;
	}
	// 8290C0F4: 4B9B479D  bl 0x822c0890
	ctx.lr = 0x8290C0F8;
	sub_822C0890(ctx, base);
	// 8290C0F8: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8290C0FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C100: 419A0008  beq cr6, 0x8290c108
	if ctx.cr[6].eq {
	pc = 0x8290C108; continue 'dispatch;
	}
	// 8290C104: 4B9B478D  bl 0x822c0890
	ctx.lr = 0x8290C108;
	sub_822C0890(ctx, base);
	// 8290C108: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C10C: 484E731D  bl 0x82df3428
	ctx.lr = 0x8290C110;
	sub_82DF3428(ctx, base);
	// 8290C110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C114: 484E7315  bl 0x82df3428
	ctx.lr = 0x8290C118;
	sub_82DF3428(ctx, base);
	// 8290C118: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290C11C: 419A00A8  beq cr6, 0x8290c1c4
	if ctx.cr[6].eq {
	pc = 0x8290C1C4; continue 'dispatch;
	}
	// 8290C120: 817F0328  lwz r11, 0x328(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(808 as u32) ) } as u64;
	// 8290C124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290C128: 409A009C  bne cr6, 0x8290c1c4
	if !ctx.cr[6].eq {
	pc = 0x8290C1C4; continue 'dispatch;
	}
	// 8290C12C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290C130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C134: 388B5858  addi r4, r11, 0x5858
	ctx.r[4].s64 = ctx.r[11].s64 + 22616;
	// 8290C138: 484E78D1  bl 0x82df3a08
	ctx.lr = 0x8290C13C;
	sub_82DF3A08(ctx, base);
	// 8290C13C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C140: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C144: 484E78C5  bl 0x82df3a08
	ctx.lr = 0x8290C148;
	sub_82DF3A08(ctx, base);
	// 8290C148: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C14C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8290C150: 4BEE0479  bl 0x827ec5c8
	ctx.lr = 0x8290C154;
	sub_827EC5C8(ctx, base);
	// 8290C154: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C158: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8290C15C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8290C160: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290C164: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C168: 48508EC9  bl 0x82e15030
	ctx.lr = 0x8290C16C;
	sub_82E15030(ctx, base);
	// 8290C16C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290C170: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C178: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C17C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290C180: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8290C184: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290C188: 4BEE0081  bl 0x827ec208
	ctx.lr = 0x8290C18C;
	sub_827EC208(ctx, base);
	// 8290C18C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8290C190: 907F0328  stw r3, 0x328(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[3].u32 ) };
	// 8290C194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290C198: 419A000C  beq cr6, 0x8290c1a4
	if ctx.cr[6].eq {
	pc = 0x8290C1A4; continue 'dispatch;
	}
	// 8290C19C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290C1A0: 4B9B46F1  bl 0x822c0890
	ctx.lr = 0x8290C1A4;
	sub_822C0890(ctx, base);
	// 8290C1A4: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8290C1A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C1AC: 419A0008  beq cr6, 0x8290c1b4
	if ctx.cr[6].eq {
	pc = 0x8290C1B4; continue 'dispatch;
	}
	// 8290C1B0: 4B9B46E1  bl 0x822c0890
	ctx.lr = 0x8290C1B4;
	sub_822C0890(ctx, base);
	// 8290C1B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C1B8: 484E7271  bl 0x82df3428
	ctx.lr = 0x8290C1BC;
	sub_82DF3428(ctx, base);
	// 8290C1BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C1C0: 484E7269  bl 0x82df3428
	ctx.lr = 0x8290C1C4;
	sub_82DF3428(ctx, base);
	// 8290C1C4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290C1C8: 4800007C  b 0x8290c244
	pc = 0x8290C244; continue 'dispatch;
	// 8290C1CC: 83DF00E4  lwz r30, 0xe4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290C1D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290C1D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C1D8: 4800AE49  bl 0x82917020
	ctx.lr = 0x8290C1DC;
	sub_82917020(ctx, base);
	// 8290C1DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290C1E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290C1E4: 4800ABB5  bl 0x82916d98
	ctx.lr = 0x8290C1E8;
	sub_82916D98(ctx, base);
	// 8290C1E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290C1EC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290C1F0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8290C1F4: D00B00FC  stfs f0, 0xfc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8290C1F8: 817F0328  lwz r11, 0x328(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(808 as u32) ) } as u64;
	// 8290C1FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290C200: 419A0040  beq cr6, 0x8290c240
	if ctx.cr[6].eq {
	pc = 0x8290C240; continue 'dispatch;
	}
	// 8290C204: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C208: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8290C20C: 4BC032BD  bl 0x8250f4c8
	ctx.lr = 0x8290C210;
	sub_8250F4C8(ctx, base);
	// 8290C210: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290C218: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8290C21C: 409A0008  bne cr6, 0x8290c224
	if !ctx.cr[6].eq {
	pc = 0x8290C224; continue 'dispatch;
	}
	// 8290C220: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290C224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C228: 80BF0328  lwz r5, 0x328(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(808 as u32) ) } as u64;
	// 8290C22C: 4BEE0065  bl 0x827ec290
	ctx.lr = 0x8290C230;
	sub_827EC290(ctx, base);
	// 8290C230: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8290C234: 484E5A5D  bl 0x82df1c90
	ctx.lr = 0x8290C238;
	sub_82DF1C90(ctx, base);
	// 8290C238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290C23C: 917F0328  stw r11, 0x328(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[11].u32 ) };
	// 8290C240: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290C244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C248: 419A0008  beq cr6, 0x8290c250
	if ctx.cr[6].eq {
	pc = 0x8290C250; continue 'dispatch;
	}
	// 8290C24C: 4B9B4645  bl 0x822c0890
	ctx.lr = 0x8290C250;
	sub_822C0890(ctx, base);
	// 8290C250: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8290C254: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8290C258: 4889BF64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290C260 size=464
    let mut pc: u32 = 0x8290C260;
    'dispatch: loop {
        match pc {
            0x8290C260 => {
    //   block [0x8290C260..0x8290C430)
	// 8290C260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C264: 4889BF09  bl 0x831a816c
	ctx.lr = 0x8290C268;
	sub_831A8130(ctx, base);
	// 8290C268: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8290C26C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290C274: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290C278: C01F032C  lfs f0, 0x32c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290C27C: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290C280: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8290C284: 409901A0  ble cr6, 0x8290c424
	if !ctx.cr[6].gt {
	pc = 0x8290C424; continue 'dispatch;
	}
	// 8290C288: C1BF0330  lfs f13, 0x330(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(816 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290C28C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8290C290: 40980194  bge cr6, 0x8290c424
	if !ctx.cr[6].lt {
	pc = 0x8290C424; continue 'dispatch;
	}
	// 8290C294: C1840018  lfs f12, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8290C298: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 8290C29C: D01F032C  stfs f0, 0x32c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 8290C2A0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8290C2A4: 40990008  ble cr6, 0x8290c2ac
	if !ctx.cr[6].gt {
	pc = 0x8290C2AC; continue 'dispatch;
	}
	// 8290C2A8: D1BF032C  stfs f13, 0x32c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 8290C2AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C2B0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290C2B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C2B8: 38AA0E60  addi r5, r10, 0xe60
	ctx.r[5].s64 = ctx.r[10].s64 + 3680;
	// 8290C2BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290C2C0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8290C2C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C2C8: 4E800421  bctrl
	ctx.lr = 0x8290C2CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C2CC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290C2D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C2D4: 419A0008  beq cr6, 0x8290c2dc
	if ctx.cr[6].eq {
	pc = 0x8290C2DC; continue 'dispatch;
	}
	// 8290C2D8: 4B9B45B9  bl 0x822c0890
	ctx.lr = 0x8290C2DC;
	sub_822C0890(ctx, base);
	// 8290C2DC: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290C2E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C2E4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8290C2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C2EC: 4E800421  bctrl
	ctx.lr = 0x8290C2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C2F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C2F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290C2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C2FC: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290C300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C304: 4E800421  bctrl
	ctx.lr = 0x8290C308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C308: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290C30C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8290C310: 388B589C  addi r4, r11, 0x589c
	ctx.r[4].s64 = ctx.r[11].s64 + 22684;
	// 8290C314: 41820088  beq 0x8290c39c
	if ctx.cr[0].eq {
	pc = 0x8290C39C; continue 'dispatch;
	}
	// 8290C318: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C31C: 484E76ED  bl 0x82df3a08
	ctx.lr = 0x8290C320;
	sub_82DF3A08(ctx, base);
	// 8290C320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C324: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290C328: 484E76E1  bl 0x82df3a08
	ctx.lr = 0x8290C32C;
	sub_82DF3A08(ctx, base);
	// 8290C32C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C330: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290C334: 4BEE0295  bl 0x827ec5c8
	ctx.lr = 0x8290C338;
	sub_827EC5C8(ctx, base);
	// 8290C338: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C33C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290C340: C3FE0000  lfs f31, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290C344: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290C348: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 8290C34C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C350: 48508CE1  bl 0x82e15030
	ctx.lr = 0x8290C354;
	sub_82E15030(ctx, base);
	// 8290C354: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C358: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290C35C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C360: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C364: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290C368: 4BEDFE91  bl 0x827ec1f8
	ctx.lr = 0x8290C36C;
	sub_827EC1F8(ctx, base);
	// 8290C36C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290C370: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C374: 419A0008  beq cr6, 0x8290c37c
	if ctx.cr[6].eq {
	pc = 0x8290C37C; continue 'dispatch;
	}
	// 8290C378: 4B9B4519  bl 0x822c0890
	ctx.lr = 0x8290C37C;
	sub_822C0890(ctx, base);
	// 8290C37C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290C380: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C384: 419A0008  beq cr6, 0x8290c38c
	if ctx.cr[6].eq {
	pc = 0x8290C38C; continue 'dispatch;
	}
	// 8290C388: 4B9B4509  bl 0x822c0890
	ctx.lr = 0x8290C38C;
	sub_822C0890(ctx, base);
	// 8290C38C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C390: 484E7099  bl 0x82df3428
	ctx.lr = 0x8290C394;
	sub_82DF3428(ctx, base);
	// 8290C394: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C398: 48000088  b 0x8290c420
	pc = 0x8290C420; continue 'dispatch;
	// 8290C39C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C3A0: 484E7669  bl 0x82df3a08
	ctx.lr = 0x8290C3A4;
	sub_82DF3A08(ctx, base);
	// 8290C3A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290C3A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C3AC: 388BC604  addi r4, r11, -0x39fc
	ctx.r[4].s64 = ctx.r[11].s64 + -14844;
	// 8290C3B0: 484E7659  bl 0x82df3a08
	ctx.lr = 0x8290C3B4;
	sub_82DF3A08(ctx, base);
	// 8290C3B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C3B8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290C3BC: 4BEE020D  bl 0x827ec5c8
	ctx.lr = 0x8290C3C0;
	sub_827EC5C8(ctx, base);
	// 8290C3C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C3C4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8290C3C8: C3FE0000  lfs f31, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290C3CC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290C3D0: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290C3D4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C3D8: 48508C59  bl 0x82e15030
	ctx.lr = 0x8290C3DC;
	sub_82E15030(ctx, base);
	// 8290C3DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C3E0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290C3E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C3E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C3EC: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290C3F0: 4BEDFE09  bl 0x827ec1f8
	ctx.lr = 0x8290C3F4;
	sub_827EC1F8(ctx, base);
	// 8290C3F4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290C3F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C3FC: 419A0008  beq cr6, 0x8290c404
	if ctx.cr[6].eq {
	pc = 0x8290C404; continue 'dispatch;
	}
	// 8290C400: 4B9B4491  bl 0x822c0890
	ctx.lr = 0x8290C404;
	sub_822C0890(ctx, base);
	// 8290C404: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8290C408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C40C: 419A0008  beq cr6, 0x8290c414
	if ctx.cr[6].eq {
	pc = 0x8290C414; continue 'dispatch;
	}
	// 8290C410: 4B9B4481  bl 0x822c0890
	ctx.lr = 0x8290C414;
	sub_822C0890(ctx, base);
	// 8290C414: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290C418: 484E7011  bl 0x82df3428
	ctx.lr = 0x8290C41C;
	sub_82DF3428(ctx, base);
	// 8290C41C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C420: 484E7009  bl 0x82df3428
	ctx.lr = 0x8290C424;
	sub_82DF3428(ctx, base);
	// 8290C424: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8290C428: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8290C42C: 4889BD90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290C430 size=292
    let mut pc: u32 = 0x8290C430;
    'dispatch: loop {
        match pc {
            0x8290C430 => {
    //   block [0x8290C430..0x8290C554)
	// 8290C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C434: 4889BD39  bl 0x831a816c
	ctx.lr = 0x8290C438;
	sub_831A8130(ctx, base);
	// 8290C438: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C43C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290C440: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290C444: 4BEDFD95  bl 0x827ec1d8
	ctx.lr = 0x8290C448;
	sub_827EC1D8(ctx, base);
	// 8290C448: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8290C44C: 41820100  beq 0x8290c54c
	if ctx.cr[0].eq {
	pc = 0x8290C54C; continue 'dispatch;
	}
	// 8290C450: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8290C458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C45C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8290C460: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290C464: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C468: 4E800421  bctrl
	ctx.lr = 0x8290C46C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C46C: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290C470: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C474: 4182003C  beq 0x8290c4b0
	if ctx.cr[0].eq {
	pc = 0x8290C4B0; continue 'dispatch;
	}
	// 8290C478: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8290C47C: 484E758D  bl 0x82df3a08
	ctx.lr = 0x8290C480;
	sub_82DF3A08(ctx, base);
	// 8290C480: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290C484: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C488: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290C48C: 48508BA5  bl 0x82e15030
	ctx.lr = 0x8290C490;
	sub_82E15030(ctx, base);
	// 8290C490: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C494: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8290C498: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8290C49C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C4A0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290C4A4: 4B9B7FBD  bl 0x822c4460
	ctx.lr = 0x8290C4A8;
	sub_822C4460(ctx, base);
	// 8290C4A8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290C4AC: 4800003C  b 0x8290c4e8
	pc = 0x8290C4E8; continue 'dispatch;
	// 8290C4B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290C4B4: 388B7868  addi r4, r11, 0x7868
	ctx.r[4].s64 = ctx.r[11].s64 + 30824;
	// 8290C4B8: 484E7551  bl 0x82df3a08
	ctx.lr = 0x8290C4BC;
	sub_82DF3A08(ctx, base);
	// 8290C4BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290C4C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C4C4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290C4C8: 48508B69  bl 0x82e15030
	ctx.lr = 0x8290C4CC;
	sub_82E15030(ctx, base);
	// 8290C4CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C4D0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8290C4D4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8290C4D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C4DC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290C4E0: 4B9B7F81  bl 0x822c4460
	ctx.lr = 0x8290C4E4;
	sub_822C4460(ctx, base);
	// 8290C4E4: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290C4E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C4EC: 419A0008  beq cr6, 0x8290c4f4
	if ctx.cr[6].eq {
	pc = 0x8290C4F4; continue 'dispatch;
	}
	// 8290C4F0: 4B9B43A1  bl 0x822c0890
	ctx.lr = 0x8290C4F4;
	sub_822C0890(ctx, base);
	// 8290C4F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C4F8: 484E6F31  bl 0x82df3428
	ctx.lr = 0x8290C4FC;
	sub_82DF3428(ctx, base);
	// 8290C4FC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290C500: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C504: 419A0038  beq cr6, 0x8290c53c
	if ctx.cr[6].eq {
	pc = 0x8290C53C; continue 'dispatch;
	}
	// 8290C508: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C50C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8290C510: 3BEA6910  addi r31, r10, 0x6910
	ctx.r[31].s64 = ctx.r[10].s64 + 26896;
	// 8290C514: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290C518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C51C: 4E800421  bctrl
	ctx.lr = 0x8290C520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C520: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C524: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290C528: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290C52C: 4856F79D  bl 0x82e7bcc8
	ctx.lr = 0x8290C530;
	sub_82E7BCC8(ctx, base);
	// 8290C530: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8290C534: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290C558 size=292
    let mut pc: u32 = 0x8290C558;
    'dispatch: loop {
        match pc {
            0x8290C558 => {
    //   block [0x8290C558..0x8290C67C)
	// 8290C558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C55C: 4889BC11  bl 0x831a816c
	ctx.lr = 0x8290C560;
	sub_831A8130(ctx, base);
	// 8290C560: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C564: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290C568: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290C56C: 4BEDFC6D  bl 0x827ec1d8
	ctx.lr = 0x8290C570;
	sub_827EC1D8(ctx, base);
	// 8290C570: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8290C574: 41820100  beq 0x8290c674
	if ctx.cr[0].eq {
	pc = 0x8290C674; continue 'dispatch;
	}
	// 8290C578: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C57C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8290C580: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C584: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8290C588: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290C58C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C590: 4E800421  bctrl
	ctx.lr = 0x8290C594;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C594: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290C598: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C59C: 4182003C  beq 0x8290c5d8
	if ctx.cr[0].eq {
	pc = 0x8290C5D8; continue 'dispatch;
	}
	// 8290C5A0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290C5A4: 484E7465  bl 0x82df3a08
	ctx.lr = 0x8290C5A8;
	sub_82DF3A08(ctx, base);
	// 8290C5A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290C5AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C5B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290C5B4: 48508A7D  bl 0x82e15030
	ctx.lr = 0x8290C5B8;
	sub_82E15030(ctx, base);
	// 8290C5B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C5BC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8290C5C0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8290C5C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C5C8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290C5CC: 4B9B7E95  bl 0x822c4460
	ctx.lr = 0x8290C5D0;
	sub_822C4460(ctx, base);
	// 8290C5D0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290C5D4: 4800003C  b 0x8290c610
	pc = 0x8290C610; continue 'dispatch;
	// 8290C5D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290C5DC: 388BC604  addi r4, r11, -0x39fc
	ctx.r[4].s64 = ctx.r[11].s64 + -14844;
	// 8290C5E0: 484E7429  bl 0x82df3a08
	ctx.lr = 0x8290C5E4;
	sub_82DF3A08(ctx, base);
	// 8290C5E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290C5E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C5EC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290C5F0: 48508A41  bl 0x82e15030
	ctx.lr = 0x8290C5F4;
	sub_82E15030(ctx, base);
	// 8290C5F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C5F8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8290C5FC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8290C600: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C604: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290C608: 4B9B7E59  bl 0x822c4460
	ctx.lr = 0x8290C60C;
	sub_822C4460(ctx, base);
	// 8290C60C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290C610: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C614: 419A0008  beq cr6, 0x8290c61c
	if ctx.cr[6].eq {
	pc = 0x8290C61C; continue 'dispatch;
	}
	// 8290C618: 4B9B4279  bl 0x822c0890
	ctx.lr = 0x8290C61C;
	sub_822C0890(ctx, base);
	// 8290C61C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C620: 484E6E09  bl 0x82df3428
	ctx.lr = 0x8290C624;
	sub_82DF3428(ctx, base);
	// 8290C624: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290C628: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C62C: 419A0038  beq cr6, 0x8290c664
	if ctx.cr[6].eq {
	pc = 0x8290C664; continue 'dispatch;
	}
	// 8290C630: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C634: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8290C638: 3BEA6910  addi r31, r10, 0x6910
	ctx.r[31].s64 = ctx.r[10].s64 + 26896;
	// 8290C63C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290C640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C644: 4E800421  bctrl
	ctx.lr = 0x8290C648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C648: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C64C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290C650: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290C654: 4856F675  bl 0x82e7bcc8
	ctx.lr = 0x8290C658;
	sub_82E7BCC8(ctx, base);
	// 8290C658: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8290C65C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290C680 size=160
    let mut pc: u32 = 0x8290C680;
    'dispatch: loop {
        match pc {
            0x8290C680 => {
    //   block [0x8290C680..0x8290C720)
	// 8290C680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C684: 4889BADD  bl 0x831a8160
	ctx.lr = 0x8290C688;
	sub_831A8130(ctx, base);
	// 8290C688: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8290C68C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290C694: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8290C698: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290C69C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C6A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C6A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8290C6A8: 839F0244  lwz r28, 0x244(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290C6AC: 4BEDFF1D  bl 0x827ec5c8
	ctx.lr = 0x8290C6B0;
	sub_827EC5C8(ctx, base);
	// 8290C6B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C6B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8290C6B8: 839F0308  lwz r28, 0x308(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 8290C6BC: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C6C0: 48016331  bl 0x829229f0
	ctx.lr = 0x8290C6C4;
	sub_829229F0(ctx, base);
	// 8290C6C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8290C6C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C6CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8290C6D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290C6D4: 4850895D  bl 0x82e15030
	ctx.lr = 0x8290C6D8;
	sub_82E15030(ctx, base);
	// 8290C6D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290C6DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C6E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290C6E4: 4BC02DE5  bl 0x8250f4c8
	ctx.lr = 0x8290C6E8;
	sub_8250F4C8(ctx, base);
	// 8290C6E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C6EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8290C6F0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290C6F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C6F8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8290C6FC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8290C700: 48014E41  bl 0x82921540
	ctx.lr = 0x8290C704;
	sub_82921540(ctx, base);
	// 8290C704: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290C708: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C70C: 419A0008  beq cr6, 0x8290c714
	if ctx.cr[6].eq {
	pc = 0x8290C714; continue 'dispatch;
	}
	// 8290C710: 4B9B4181  bl 0x822c0890
	ctx.lr = 0x8290C714;
	sub_822C0890(ctx, base);
	// 8290C714: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8290C718: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8290C71C: 4889BA94  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290C720 size=136
    let mut pc: u32 = 0x8290C720;
    'dispatch: loop {
        match pc {
            0x8290C720 => {
    //   block [0x8290C720..0x8290C7A8)
	// 8290C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C724: 4889BA45  bl 0x831a8168
	ctx.lr = 0x8290C728;
	sub_831A8130(ctx, base);
	// 8290C728: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8290C72C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290C734: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8290C738: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290C73C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C740: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C744: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8290C748: 4BEDFE81  bl 0x827ec5c8
	ctx.lr = 0x8290C74C;
	sub_827EC5C8(ctx, base);
	// 8290C74C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290C750: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290C754: 83DF0308  lwz r30, 0x308(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 8290C758: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290C75C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C760: 485088D1  bl 0x82e15030
	ctx.lr = 0x8290C764;
	sub_82E15030(ctx, base);
	// 8290C764: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8290C768: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C76C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290C770: 4BC02D59  bl 0x8250f4c8
	ctx.lr = 0x8290C774;
	sub_8250F4C8(ctx, base);
	// 8290C774: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290C778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C77C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290C780: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8290C784: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8290C788: 48014CF1  bl 0x82921478
	ctx.lr = 0x8290C78C;
	sub_82921478(ctx, base);
	// 8290C78C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290C790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290C794: 419A0008  beq cr6, 0x8290c79c
	if ctx.cr[6].eq {
	pc = 0x8290C79C; continue 'dispatch;
	}
	// 8290C798: 4B9B40F9  bl 0x822c0890
	ctx.lr = 0x8290C79C;
	sub_822C0890(ctx, base);
	// 8290C79C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290C7A0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8290C7A4: 4889BA14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290C7A8 size=108
    let mut pc: u32 = 0x8290C7A8;
    'dispatch: loop {
        match pc {
            0x8290C7A8 => {
    //   block [0x8290C7A8..0x8290C814)
	// 8290C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C7AC: 4889B9BD  bl 0x831a8168
	ctx.lr = 0x8290C7B0;
	sub_831A8130(ctx, base);
	// 8290C7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C7B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290C7B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C7BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290C7C0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8290C7C4: 4BC02D05  bl 0x8250f4c8
	ctx.lr = 0x8290C7C8;
	sub_8250F4C8(ctx, base);
	// 8290C7C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C7CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290C7D0: 3BABFFFC  addi r29, r11, -4
	ctx.r[29].s64 = ctx.r[11].s64 + -4;
	// 8290C7D4: 409A0008  bne cr6, 0x8290c7dc
	if !ctx.cr[6].eq {
	pc = 0x8290C7DC; continue 'dispatch;
	}
	// 8290C7D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8290C7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C7E0: 83FF0310  lwz r31, 0x310(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(784 as u32) ) } as u64;
	// 8290C7E4: 48106E9D  bl 0x82a13680
	ctx.lr = 0x8290C7E8;
	sub_82A13680(ctx, base);
	// 8290C7E8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8290C7EC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8290C7F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8290C7F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290C7F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C7FC: 48239325  bl 0x82b45b20
	ctx.lr = 0x8290C800;
	sub_82B45B20(ctx, base);
	// 8290C800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C804: 484E548D  bl 0x82df1c90
	ctx.lr = 0x8290C808;
	sub_82DF1C90(ctx, base);
	// 8290C808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290C810: 4889B9A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290C818 size=96
    let mut pc: u32 = 0x8290C818;
    'dispatch: loop {
        match pc {
            0x8290C818 => {
    //   block [0x8290C818..0x8290C878)
	// 8290C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C81C: 4889B94D  bl 0x831a8168
	ctx.lr = 0x8290C820;
	sub_831A8130(ctx, base);
	// 8290C820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290C828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C82C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290C830: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8290C834: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8290C838: 4BC02C91  bl 0x8250f4c8
	ctx.lr = 0x8290C83C;
	sub_8250F4C8(ctx, base);
	// 8290C83C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290C844: 38ABFFFC  addi r5, r11, -4
	ctx.r[5].s64 = ctx.r[11].s64 + -4;
	// 8290C848: 409A0008  bne cr6, 0x8290c850
	if !ctx.cr[6].eq {
	pc = 0x8290C850; continue 'dispatch;
	}
	// 8290C84C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290C850: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8290C854: 809E0310  lwz r4, 0x310(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(784 as u32) ) } as u64;
	// 8290C858: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8290C85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C860: 482392C1  bl 0x82b45b20
	ctx.lr = 0x8290C864;
	sub_82B45B20(ctx, base);
	// 8290C864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290C868: 484E5429  bl 0x82df1c90
	ctx.lr = 0x8290C86C;
	sub_82DF1C90(ctx, base);
	// 8290C86C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290C870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290C874: 4889B944  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


