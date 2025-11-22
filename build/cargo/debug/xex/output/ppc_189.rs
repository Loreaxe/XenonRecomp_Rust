pub fn sub_832AA440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA440 size=2060
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098790400;
	ctx.r[4].s64 = ctx.r[10].s64 + -30388;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -17392;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + 14632;
	ctx.lr = 0x832AA46C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E910A0);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32632;
	ctx.lr = 0x832AA478;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098790400;
	ctx.r[4].s64 = ctx.r[10].s64 + -30288;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -17112;
	ctx.r[5].s64 = ctx.r[11].s64 + 20728;
	ctx.lr = 0x832AA4B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E94170);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32656;
	ctx.lr = 0x832AA4C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098790400;
	ctx.r[4].s64 = ctx.r[10].s64 + -29964;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16836;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + 22464;
	ctx.lr = 0x832AA504;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E910A0);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32680;
	ctx.lr = 0x832AA510;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29808;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16560;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -30968;
	ctx.lr = 0x832AA554;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E910A0);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32704;
	ctx.lr = 0x832AA560;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29768;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16284;
	ctx.r[5].s64 = ctx.r[11].s64 + -29024;
	ctx.lr = 0x832AA59C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32728;
	ctx.lr = 0x832AA5A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29728;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16008;
	ctx.r[5].s64 = ctx.r[11].s64 + -28920;
	ctx.lr = 0x832AA5E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32752;
	ctx.lr = 0x832AA5F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29692;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	ctx.r[5].s64 = ctx.r[11].s64 + -28696;
	ctx.lr = 0x832AA62C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32760;
	ctx.lr = 0x832AA638;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29656;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	ctx.r[5].s64 = ctx.r[11].s64 + -28568;
	ctx.lr = 0x832AA674;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32736;
	ctx.lr = 0x832AA680;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29620;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15180;
	ctx.r[5].s64 = ctx.r[11].s64 + -28304;
	ctx.lr = 0x832AA6BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32712;
	ctx.lr = 0x832AA6C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29576;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14904;
	ctx.r[5].s64 = ctx.r[11].s64 + -28064;
	ctx.lr = 0x832AA704;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32688;
	ctx.lr = 0x832AA710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29556;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	ctx.r[5].s64 = ctx.r[11].s64 + -27936;
	ctx.lr = 0x832AA74C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32664;
	ctx.lr = 0x832AA758;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29536;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	ctx.r[5].s64 = ctx.r[11].s64 + -27864;
	ctx.lr = 0x832AA794;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32640;
	ctx.lr = 0x832AA7A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14076;
	ctx.r[5].s64 = ctx.r[11].s64 + -27696;
	ctx.lr = 0x832AA7DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32616;
	ctx.lr = 0x832AA7E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29452;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13800;
	ctx.r[5].s64 = ctx.r[11].s64 + -27608;
	ctx.lr = 0x832AA824;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32592;
	ctx.lr = 0x832AA830;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29412;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	ctx.r[5].s64 = ctx.r[11].s64 + -27336;
	ctx.lr = 0x832AA86C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32568;
	ctx.lr = 0x832AA878;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29376;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	ctx.r[5].s64 = ctx.r[11].s64 + -27096;
	ctx.lr = 0x832AA8B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32544;
	ctx.lr = 0x832AA8C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29332;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	ctx.r[5].s64 = ctx.r[11].s64 + -26992;
	ctx.lr = 0x832AA8FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32520;
	ctx.lr = 0x832AA908;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29296;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12696;
	ctx.r[5].s64 = ctx.r[11].s64 + -26800;
	ctx.lr = 0x832AA944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32496;
	ctx.lr = 0x832AA950;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29256;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	ctx.r[5].s64 = ctx.r[11].s64 + -26712;
	ctx.lr = 0x832AA98C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32472;
	ctx.lr = 0x832AA998;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	ctx.lr = 0x832AA9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	ctx.lr = 0x832AA9E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	ctx.lr = 0x832AAA1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	ctx.lr = 0x832AAA28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	ctx.lr = 0x832AAA64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	ctx.lr = 0x832AAA70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	ctx.lr = 0x832AAAAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
}

pub fn sub_832AA508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA508 size=5136
	ctx.r[3].s64 = ctx.r[11].s64 + 32680;
	ctx.lr = 0x832AA510;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29808;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16560;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -30968;
	ctx.lr = 0x832AA554;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E910A0);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32704;
	ctx.lr = 0x832AA560;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29768;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16284;
	ctx.r[5].s64 = ctx.r[11].s64 + -29024;
	ctx.lr = 0x832AA59C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32728;
	ctx.lr = 0x832AA5A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29728;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16008;
	ctx.r[5].s64 = ctx.r[11].s64 + -28920;
	ctx.lr = 0x832AA5E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32752;
	ctx.lr = 0x832AA5F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29692;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	ctx.r[5].s64 = ctx.r[11].s64 + -28696;
	ctx.lr = 0x832AA62C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32760;
	ctx.lr = 0x832AA638;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29656;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	ctx.r[5].s64 = ctx.r[11].s64 + -28568;
	ctx.lr = 0x832AA674;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32736;
	ctx.lr = 0x832AA680;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29620;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15180;
	ctx.r[5].s64 = ctx.r[11].s64 + -28304;
	ctx.lr = 0x832AA6BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32712;
	ctx.lr = 0x832AA6C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29576;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14904;
	ctx.r[5].s64 = ctx.r[11].s64 + -28064;
	ctx.lr = 0x832AA704;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32688;
	ctx.lr = 0x832AA710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29556;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	ctx.r[5].s64 = ctx.r[11].s64 + -27936;
	ctx.lr = 0x832AA74C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32664;
	ctx.lr = 0x832AA758;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29536;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	ctx.r[5].s64 = ctx.r[11].s64 + -27864;
	ctx.lr = 0x832AA794;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32640;
	ctx.lr = 0x832AA7A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14076;
	ctx.r[5].s64 = ctx.r[11].s64 + -27696;
	ctx.lr = 0x832AA7DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32616;
	ctx.lr = 0x832AA7E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29452;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13800;
	ctx.r[5].s64 = ctx.r[11].s64 + -27608;
	ctx.lr = 0x832AA824;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32592;
	ctx.lr = 0x832AA830;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29412;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	ctx.r[5].s64 = ctx.r[11].s64 + -27336;
	ctx.lr = 0x832AA86C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32568;
	ctx.lr = 0x832AA878;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29376;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	ctx.r[5].s64 = ctx.r[11].s64 + -27096;
	ctx.lr = 0x832AA8B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32544;
	ctx.lr = 0x832AA8C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29332;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	ctx.r[5].s64 = ctx.r[11].s64 + -26992;
	ctx.lr = 0x832AA8FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32520;
	ctx.lr = 0x832AA908;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29296;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12696;
	ctx.r[5].s64 = ctx.r[11].s64 + -26800;
	ctx.lr = 0x832AA944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32496;
	ctx.lr = 0x832AA950;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29256;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	ctx.r[5].s64 = ctx.r[11].s64 + -26712;
	ctx.lr = 0x832AA98C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32472;
	ctx.lr = 0x832AA998;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	ctx.lr = 0x832AA9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	ctx.lr = 0x832AA9E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	ctx.lr = 0x832AAA1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	ctx.lr = 0x832AAA28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	ctx.lr = 0x832AAA64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	ctx.lr = 0x832AAA70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	ctx.lr = 0x832AAAAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AA5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA5C0 size=2060
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29728;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -16008;
	ctx.r[5].s64 = ctx.r[11].s64 + -28920;
	ctx.lr = 0x832AA5E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094333952;
	ctx.r[3].s64 = ctx.r[11].s64 + 32752;
	ctx.lr = 0x832AA5F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29692;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	ctx.r[5].s64 = ctx.r[11].s64 + -28696;
	ctx.lr = 0x832AA62C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32760;
	ctx.lr = 0x832AA638;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29656;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	ctx.r[5].s64 = ctx.r[11].s64 + -28568;
	ctx.lr = 0x832AA674;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32736;
	ctx.lr = 0x832AA680;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29620;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -15180;
	ctx.r[5].s64 = ctx.r[11].s64 + -28304;
	ctx.lr = 0x832AA6BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32712;
	ctx.lr = 0x832AA6C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29576;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14904;
	ctx.r[5].s64 = ctx.r[11].s64 + -28064;
	ctx.lr = 0x832AA704;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32688;
	ctx.lr = 0x832AA710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29556;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	ctx.r[5].s64 = ctx.r[11].s64 + -27936;
	ctx.lr = 0x832AA74C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32664;
	ctx.lr = 0x832AA758;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29536;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	ctx.r[5].s64 = ctx.r[11].s64 + -27864;
	ctx.lr = 0x832AA794;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32640;
	ctx.lr = 0x832AA7A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -14076;
	ctx.r[5].s64 = ctx.r[11].s64 + -27696;
	ctx.lr = 0x832AA7DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32616;
	ctx.lr = 0x832AA7E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29452;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13800;
	ctx.r[5].s64 = ctx.r[11].s64 + -27608;
	ctx.lr = 0x832AA824;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32592;
	ctx.lr = 0x832AA830;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29412;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	ctx.r[5].s64 = ctx.r[11].s64 + -27336;
	ctx.lr = 0x832AA86C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32568;
	ctx.lr = 0x832AA878;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29376;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	ctx.r[5].s64 = ctx.r[11].s64 + -27096;
	ctx.lr = 0x832AA8B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32544;
	ctx.lr = 0x832AA8C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29332;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	ctx.r[5].s64 = ctx.r[11].s64 + -26992;
	ctx.lr = 0x832AA8FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32520;
	ctx.lr = 0x832AA908;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29296;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12696;
	ctx.r[5].s64 = ctx.r[11].s64 + -26800;
	ctx.lr = 0x832AA944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32496;
	ctx.lr = 0x832AA950;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29256;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	ctx.r[5].s64 = ctx.r[11].s64 + -26712;
	ctx.lr = 0x832AA98C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32472;
	ctx.lr = 0x832AA998;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	ctx.lr = 0x832AA9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	ctx.lr = 0x832AA9E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	ctx.lr = 0x832AAA1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	ctx.lr = 0x832AAA28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	ctx.lr = 0x832AAA64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	ctx.lr = 0x832AAA70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	ctx.lr = 0x832AAAAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA918 size=1044
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29296;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12696;
	ctx.r[5].s64 = ctx.r[11].s64 + -26800;
	ctx.lr = 0x832AA944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32496;
	ctx.lr = 0x832AA950;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29256;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	ctx.r[5].s64 = ctx.r[11].s64 + -26712;
	ctx.lr = 0x832AA98C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32472;
	ctx.lr = 0x832AA998;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	ctx.lr = 0x832AA9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	ctx.lr = 0x832AA9E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	ctx.lr = 0x832AAA1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	ctx.lr = 0x832AAA28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	ctx.lr = 0x832AAA64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	ctx.lr = 0x832AAA70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	ctx.lr = 0x832AAAAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
}

pub fn sub_832AA9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA9B0 size=5136
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	ctx.lr = 0x832AA9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	ctx.lr = 0x832AA9E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	ctx.lr = 0x832AAA1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	ctx.lr = 0x832AAA28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	ctx.lr = 0x832AAA64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	ctx.lr = 0x832AAA70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	ctx.lr = 0x832AAAAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AAA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAA68 size=2064
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	ctx.lr = 0x832AAA70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	ctx.lr = 0x832AAAAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAAB0 size=2064
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	ctx.lr = 0x832AAAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	ctx.lr = 0x832AAAF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	ctx.lr = 0x832AAB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AAB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAB08 size=5136
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	ctx.lr = 0x832AAB3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	ctx.lr = 0x832AAB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	ctx.lr = 0x832AAB84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	ctx.lr = 0x832AAB90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	ctx.lr = 0x832AABCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	ctx.lr = 0x832AABD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	ctx.lr = 0x832AAC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85708);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	ctx.lr = 0x832AAC20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	ctx.r[10].s64 = -2093744128;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	ctx.lr = 0x832AAC5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86FA8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	ctx.lr = 0x832AAC68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	ctx.lr = 0x832AACAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	ctx.lr = 0x832AACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	ctx.lr = 0x832AACFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	ctx.lr = 0x832AAD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098724864;
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AAD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AAD38 size=1044
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	ctx.lr = 0x832AAD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	ctx.lr = 0x832AAD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB120 size=1040
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -10208;
	ctx.lr = 0x832AB12C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31504;
	ctx.lr = 0x832AB138;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2113667072;
	ctx.r[6].s64 = 0;
	ctx.r[4].s64 = ctx.r[11].s64 + -26260;
	ctx.r[11].s64 = -2093744128;
	ctx.r[5].s64 = 0;
	ctx.r[3].s64 = ctx.r[11].s64 + 22556;
	ctx.lr = 0x832AB170;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D5F8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31240;
	ctx.lr = 0x832AB17C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB188 size=1040
	return;
}

pub fn sub_832AB1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB1F0 size=5136
	ctx.r[10].s64 = -2113667072;
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	ctx.r[4].s64 = ctx.r[10].s64 + -26224;
	ctx.r[11].s64 = -2098659328;
	ctx.r[10].s64 = -2093744128;
	ctx.r[5].s64 = ctx.r[11].s64 + 408;
	ctx.r[3].s64 = ctx.r[10].s64 + 22280;
	ctx.lr = 0x832AB210;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D5F8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31408;
	ctx.lr = 0x832AB21C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB2B8 size=2060
	ctx.lr = 0x832AB2BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB320 size=5136
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098659328;
	ctx.r[4].s64 = ctx.r[10].s64 + -26116;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[31].s64 = ctx.r[10].s64 + 22004;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -2992;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AB35C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E97D10);
	ctx.r[11].s64 = -2113667072;
	ctx.r[10].s64 = -2094268416;
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	ctx.r[3].s64 = ctx.r[10].s64 + -31312;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832AB374;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098659328;
	ctx.r[4].s64 = ctx.r[10].s64 + -26088;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[31].s64 = ctx.r[10].s64 + 23108;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -2920;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AB3C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E97D10);
	ctx.r[11].s64 = -2113667072;
	ctx.r[10].s64 = -2094268416;
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	ctx.r[3].s64 = ctx.r[10].s64 + -31288;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832AB3DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098659328;
	ctx.r[4].s64 = ctx.r[10].s64 + -26072;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[31].s64 = ctx.r[10].s64 + 21728;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -2848;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AB42C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E97D10);
	ctx.r[11].s64 = -2113667072;
	ctx.r[10].s64 = -2094268416;
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	ctx.r[3].s64 = ctx.r[10].s64 + -31264;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832AB444;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2113667072;
	ctx.r[6].s64 = 0;
	ctx.r[4].s64 = ctx.r[11].s64 + -25928;
	ctx.r[11].s64 = -2093744128;
	ctx.r[5].s64 = 0;
	ctx.r[3].s64 = ctx.r[11].s64 + 23660;
	ctx.lr = 0x832AB480;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F60);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31192;
	ctx.lr = 0x832AB48C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB458 size=2060
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2113667072;
	ctx.r[6].s64 = 0;
	ctx.r[4].s64 = ctx.r[11].s64 + -25928;
	ctx.r[11].s64 = -2093744128;
	ctx.r[5].s64 = 0;
	ctx.r[3].s64 = ctx.r[11].s64 + 23660;
	ctx.lr = 0x832AB480;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F60);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31192;
	ctx.lr = 0x832AB48C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB4C0 size=1040
	ctx.r[3].s64 = ctx.r[10].s64 + 23940;
	ctx.lr = 0x832AB4C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EA01E0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31168;
	ctx.lr = 0x832AB4D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB568 size=2060
	ctx.lr = 0x832AB56C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB5E0 size=5136
	ctx.r[10].s64 = -2093744128;
	ctx.r[5].s64 = ctx.r[11].s64 + -27272;
	ctx.r[3].s64 = ctx.r[10].s64 + 25044;
	ctx.lr = 0x832AB5F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB5DE8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -31072;
	ctx.lr = 0x832AB5FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB758 size=1040
	ctx.lr = 0x832AB75C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098593792;
	ctx.r[4].s64 = ctx.r[10].s64 + -24932;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[31].s64 = ctx.r[10].s64 + 27464;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -16744;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AB7AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EA2AB8);
	ctx.r[11].s64 = -2113667072;
	ctx.r[10].s64 = -2094268416;
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	ctx.r[3].s64 = ctx.r[10].s64 + -30928;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832AB7C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098593792;
	ctx.r[4].s64 = ctx.r[10].s64 + -24844;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[31].s64 = ctx.r[10].s64 + 27740;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -15488;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AB814;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EA2AB8);
	ctx.r[11].s64 = -2113667072;
	ctx.r[10].s64 = -2094268416;
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	ctx.r[3].s64 = ctx.r[10].s64 + -30904;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832AB82C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098593792;
	ctx.r[4].s64 = ctx.r[10].s64 + -24724;
	ctx.r[10].s64 = -2093744128;
	ctx.r[8].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + 28016;
	ctx.r[7].s64 = 0;
	ctx.r[6].s64 = 0;
	ctx.r[5].s64 = ctx.r[11].s64 + -9472;
	ctx.lr = 0x832AB874;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D1C0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30880;
	ctx.lr = 0x832AB880;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098593792;
	ctx.r[4].s64 = ctx.r[10].s64 + -24636;
	ctx.r[10].s64 = -2093744128;
	ctx.r[5].s64 = ctx.r[11].s64 + -7176;
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	ctx.lr = 0x832AB8B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EA01E0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30856;
	ctx.lr = 0x832AB8C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AB8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB8C8 size=1040
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABAB8 size=4116
	ctx.r[5].s64 = 0;
	ctx.r[4].s64 = 0;
	ctx.lr = 0x832ABAC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8CBC0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30592;
	ctx.lr = 0x832ABAD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093744128;
	ctx.r[10].s64 = -2113667072;
	ctx.r[6].s64 = ctx.r[11].s64 + 4168;
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	ctx.r[11].s64 = -2098528256;
	ctx.r[10].s64 = -2093678592;
	ctx.r[8].s64 = 1;
	ctx.r[3].s64 = ctx.r[10].s64 + -29596;
	ctx.r[7].s64 = 1;
	ctx.r[5].s64 = ctx.r[11].s64 + 5512;
	ctx.lr = 0x832ABB18;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8D180);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30528;
	ctx.lr = 0x832ABB24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABB28 size=5136
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB80 size=1040
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098528256;
	ctx.r[4].s64 = ctx.r[10].s64 + -22864;
	ctx.r[10].s64 = -2093678592;
	ctx.r[5].s64 = ctx.r[11].s64 + 17008;
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	ctx.lr = 0x832ABBA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82635970);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30432;
	ctx.lr = 0x832ABBB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC98 size=1040
	return;
}

pub fn sub_832ABCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABCE0 size=5136
	return;
}

pub fn sub_832ABD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD38 size=1040
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098593792;
	ctx.r[4].s64 = ctx.r[10].s64 + -22748;
	ctx.r[10].s64 = -2093678592;
	ctx.r[5].s64 = ctx.r[11].s64 + 25512;
	ctx.r[3].s64 = ctx.r[10].s64 + -27444;
	ctx.lr = 0x832ABD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EBCC20);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30288;
	ctx.lr = 0x832ABD64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD80 size=1040
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098528256;
	ctx.r[4].s64 = ctx.r[10].s64 + -22736;
	ctx.r[10].s64 = -2093678592;
	ctx.r[5].s64 = ctx.r[11].s64 + 19952;
	ctx.r[3].s64 = ctx.r[10].s64 + -27980;
	ctx.lr = 0x832ABDA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EBCC20);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -30264;
	ctx.lr = 0x832ABDAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABDF0 size=2060
	ctx.lr = 0x832ABDF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ABE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABE88 size=1040
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832AC0B0 size=2060
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -21160;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -13092;
	ctx.r[5].s64 = ctx.r[11].s64 + -8264;
	ctx.lr = 0x832AC0CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD268);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29736;
	ctx.lr = 0x832AC0D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -21112;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12816;
	ctx.r[5].s64 = ctx.r[11].s64 + -7744;
	ctx.lr = 0x832AC114;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD268);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29712;
	ctx.lr = 0x832AC120;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -21056;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12540;
	ctx.r[5].s64 = ctx.r[11].s64 + -6432;
	ctx.lr = 0x832AC15C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD440);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29688;
	ctx.lr = 0x832AC168;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -12264;
	ctx.r[5].s64 = ctx.r[11].s64 + -5024;
	ctx.lr = 0x832AC1A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD440);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29664;
	ctx.lr = 0x832AC1B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20792;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11988;
	ctx.r[5].s64 = ctx.r[11].s64 + -1752;
	ctx.lr = 0x832AC1EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD618);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29640;
	ctx.lr = 0x832AC1F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11708;
	ctx.r[5].s64 = ctx.r[11].s64 + -520;
	ctx.lr = 0x832AC234;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD618);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29616;
	ctx.lr = 0x832AC240;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2093744128;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[2200617576].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	pc = 0x832AC270; continue 'dispatch;
	ctx.r[11].s64 = -2113929216;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	ctx.r[11].s64 = -2113929216;
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[0].u32.wrapping_add(0 as u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	ctx.r[11].s64 = -2093678592;
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), tmp.u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20644;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11432;
	ctx.r[5].s64 = ctx.r[11].s64 + 2872;
	ctx.lr = 0x832AC2B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD618);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29592;
	ctx.lr = 0x832AC2C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20576;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -11140;
	ctx.r[5].s64 = ctx.r[11].s64 + 3016;
	ctx.lr = 0x832AC2FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD7F0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29568;
	ctx.lr = 0x832AC308;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20540;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10864;
	ctx.r[5].s64 = ctx.r[11].s64 + 3368;
	ctx.lr = 0x832AC344;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD7F0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29544;
	ctx.lr = 0x832AC350;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20520;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	ctx.r[5].s64 = ctx.r[11].s64 + 5696;
	ctx.lr = 0x832AC38C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD9C8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29520;
	ctx.lr = 0x832AC398;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20456;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10312;
	ctx.r[5].s64 = ctx.r[11].s64 + 6496;
	ctx.lr = 0x832AC3D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECD9C8);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29496;
	ctx.lr = 0x832AC3E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20376;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -10036;
	ctx.r[5].s64 = ctx.r[11].s64 + 6912;
	ctx.lr = 0x832AC41C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECDBA0);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29472;
	ctx.lr = 0x832AC428;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20316;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9760;
	ctx.r[5].s64 = ctx.r[11].s64 + 7976;
	ctx.lr = 0x832AC464;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECDD78);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29448;
	ctx.lr = 0x832AC470;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113667072;
	ctx.r[11].s64 = -2098462720;
	ctx.r[4].s64 = ctx.r[10].s64 + -20284;
	ctx.r[10].s64 = -2093678592;
	ctx.r[6].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -9484;
	ctx.r[5].s64 = ctx.r[11].s64 + 9032;
	ctx.lr = 0x832AC4AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECDF50);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29424;
	ctx.lr = 0x832AC4B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093678592;
	ctx.r[10].s64 = -2113667072;
	ctx.r[6].s64 = ctx.r[11].s64 + -8932;
	ctx.r[4].s64 = ctx.r[10].s64 + -20224;
	ctx.r[11].s64 = -2098462720;
	ctx.r[10].s64 = -2093678592;
	ctx.r[5].s64 = ctx.r[11].s64 + 9256;
	ctx.r[3].s64 = ctx.r[10].s64 + -9208;
	ctx.lr = 0x832AC4F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ECE128);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -29400;
	ctx.lr = 0x832AC504;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ACBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACBF0 size=2060
	ctx.lr = 0x832ACBF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ACC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACC58 size=2060
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093613056;
	ctx.r[4].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832ACC74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F00740);
	ctx.r[11].s64 = 1;
	ctx.r[10].s64 = -2094268416;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[11].u8) };
	ctx.r[3].s64 = ctx.r[10].s64 + -28800;
	ctx.lr = 0x832ACC88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832ACEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACEA0 size=2060
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832ACEB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0DE4);
	ctx.r[11].s64 = -2094006272;
	ctx.r[9].s64 = -2094006272;
	ctx.r[8].s64 = -2094006272;
	ctx.r[7].s64 = -2094006272;
	ctx.r[6].s64 = -2094006272;
	ctx.r[5].s64 = -2093613056;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[4].s64 = -2094006272;
	ctx.r[11].s64 = ctx.r[5].s64 + -26856;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[3].s64 = -2094006272;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[31].s64 = -2094006272;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[30].s64 = -2094006272;
	ctx.r[10].s64 = -2094006272;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[9].s64 = -2094006272;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[8].s64 = -2094006272;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[7].s64 = -2094006272;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.r[6].s64 = -2094006272;
	ctx.r[29].s64 = -2094006272;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[24].s64 = -2094006272;
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[23].s64 = -2094006272;
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[22].s64 = -2094006272;
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[21].s64 = -2094006272;
	ctx.r[20].s64 = -2094006272;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = -2094006272;
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = -2094006272;
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].s64 = -2094006272;
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = -2094006272;
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[22].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[19].s64 = 0;
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[23].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[22].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[21].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[19].u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E34);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2094006272;
	ctx.r[11].s64 = ctx.r[11].s64 + -2172;
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	ctx.lr = 0x832AD004;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C1684);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -27816;
	ctx.lr = 0x832AD010;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093613056;
	ctx.r[31].s64 = ctx.r[11].s64 + -26752;
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	ctx.lr = 0x832AD040;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A420);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AD048;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C1684);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[11].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[3].s64 = ctx.r[11].s64 + -27808;
	ctx.lr = 0x832AD06C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[5].s64 = 2;
	ctx.r[11].s64 = -2113601536;
	ctx.r[4].s64 = ctx.r[11].s64 + -19680;
	ctx.r[11].s64 = -2093613056;
	ctx.r[3].s64 = ctx.r[11].s64 + -26688;
	ctx.lr = 0x832AD0A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300AF98);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ACF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACF08 size=2060
	ctx.r[8].s64 = -2094006272;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[7].s64 = -2094006272;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	ctx.r[6].s64 = -2094006272;
	ctx.r[29].s64 = -2094006272;
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[24].s64 = -2094006272;
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[23].s64 = -2094006272;
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[22].s64 = -2094006272;
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[21].s64 = -2094006272;
	ctx.r[20].s64 = -2094006272;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].s64 = -2094006272;
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].s64 = -2094006272;
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].s64 = -2094006272;
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].s64 = -2094006272;
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[22].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[19].s64 = 0;
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[3].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[31].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[30].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[29].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[27].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[26].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[25].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[24].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[23].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[22].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[21].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[19].u32) };
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E34);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2094006272;
	ctx.r[11].s64 = ctx.r[11].s64 + -2172;
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	ctx.lr = 0x832AD004;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C1684);
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -27816;
	ctx.lr = 0x832AD010;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2093613056;
	ctx.r[31].s64 = ctx.r[11].s64 + -26752;
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	ctx.lr = 0x832AD040;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A420);
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832AD048;
	crate::recompiler::externs::call(&mut ctx, base, 0x832C1684);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	ctx.r[11].s64 = -2094268416;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[3].s64 = ctx.r[11].s64 + -27808;
	ctx.lr = 0x832AD06C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[5].s64 = 2;
	ctx.r[11].s64 = -2113601536;
	ctx.r[4].s64 = ctx.r[11].s64 + -19680;
	ctx.r[11].s64 = -2093613056;
	ctx.r[3].s64 = ctx.r[11].s64 + -26688;
	ctx.lr = 0x832AD0A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300AF98);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AD1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AD1D8 size=3084
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[5].s64 = 3;
	ctx.r[11].s64 = -2113601536;
	ctx.r[4].s64 = ctx.r[11].s64 + -17848;
	ctx.r[11].s64 = -2093613056;
	ctx.r[3].s64 = ctx.r[11].s64 + -26640;
	ctx.lr = 0x832AD1F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300AF98);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AD6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AD6A0 size=1040
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[5].s64 = 1;
	ctx.r[11].s64 = -2113601536;
	ctx.r[4].s64 = ctx.r[11].s64 + -15036;
	ctx.r[11].s64 = -2093613056;
	ctx.r[3].s64 = ctx.r[11].s64 + -26340;
	ctx.lr = 0x832AD6BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300AF98);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AD9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AD9A0 size=2060
	return;
}

pub fn sub_832ADA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ADA08 size=2064
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ADB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ADB40 size=2060
	ctx.r[11].s64 = -2093613056;
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[3].u64) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832ADFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ADFD8 size=12
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2113404928;
	ctx.r[10].s64 = -2113798144;
}

pub fn sub_832AE1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AE1D8 size=1040
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[9].s64 = 24;
	ctx.r[4].s64 = ctx.r[4].s64 + -29956;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	ctx.r[9].s64 = 3;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	ctx.r[9].s64 = 13;
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[9].u16) };
	ctx.r[9].s64 = 5;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	ctx.r[9].s64 = 14;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[9].u16) };
	ctx.r[9].s64 = 6;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[6].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	ctx.r[9].s64 = 16;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[9].u16) };
	ctx.r[9].s64 = 6;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[5].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	ctx.r[9].s64 = 18;
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[9].u16) };
	ctx.r[9].s64 = 20;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[4].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[9].u8) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[10].u16) };
	unsafe { crate::rt::store_u16(base as *mut u8, (0 as u32), ctx.r[9].u16) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	return;
}

pub fn sub_832AE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AE738 size=7180
	ctx.r[11].s64 = -2094006272;
	ctx.r[10].s64 = -2094006272;
	ctx.r[10].s64 = ctx.r[10].s64 + 6400;
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2113404928;
	ctx.r[11].s64 = -2094006272;
	ctx.r[9].s64 = ctx.r[10].s64 + -27596;
	ctx.r[10].s64 = -2113404928;
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	ctx.r[7].s64 = 9;
	ctx.r[4].s64 = ctx.r[10].s64 + -27512;
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = -2093613056;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[6].s64 = 36;
	ctx.r[3].s64 = ctx.r[10].s64 + -19964;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[10].s64 = 1;
	ctx.r[8].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[7].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[5].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832AE7B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D5D640);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2113404928;
	ctx.r[10].s64 = -2113404928;
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	ctx.r[8].s64 = 2;
	ctx.r[4].s64 = ctx.r[10].s64 + -26712;
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = -2093613056;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[9].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -19916;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[10].s64 = 0;
	ctx.r[8].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[7].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].s64 = 32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[5].s64 = 0;
	ctx.lr = 0x832AE81C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D5D640);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AE7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AE7E8 size=2060
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[7].u32) };
	ctx.r[9].s64 = 0;
	ctx.r[3].s64 = ctx.r[10].s64 + -19916;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[10].s64 = 0;
	ctx.r[8].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[7].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].s64 = 32;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[5].s64 = 0;
	ctx.lr = 0x832AE81C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D5D640);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AE9C8 size=1044
	ctx.lr = 0x832AE9CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D5D640);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AEAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AEAA0 size=2060
	ctx.r[3].s64 = ctx.r[10].s64 + -19628;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[8].u32) };
	ctx.r[10].s64 = 0;
	ctx.r[8].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[7].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[6].s64 = 48;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[5].s64 = 0;
	ctx.lr = 0x832AEACC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D5D640);
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AEB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AEB18 size=2060
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	return;
}

pub fn sub_832AEB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AEB80 size=2060
	ctx.lr = ctx.r[12].u64;
	return;
	ctx.r[11].s64 = -2094268416;
	ctx.r[3].s64 = ctx.r[11].s64 + -26424;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1920);
	return;
}

pub fn sub_832AEBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AEBE8 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x822F8038);
	return;
}

pub fn sub_832AECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AECF0 size=5136
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832AEDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AEDA8 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B0458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0458 size=2060
	ctx.r[11].s64 = -2092367872;
	ctx.r[3].s64 = ctx.r[11].s64 + 31772;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B04C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B04C0 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1118 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -32756;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1180 size=2060
	return;
}

pub fn sub_832B12D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B12D8 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -32532;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1628 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	return;
}

pub fn sub_832B1680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1680 size=5136
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B16D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B16D8 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1730 size=5136
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639272].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639256].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1760;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 11;
	ctx.r[11].s64 = ctx.r[11].s64 + -32044;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B178C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639376].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639360].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B17C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 19;
	ctx.r[11].s64 = ctx.r[11].s64 + -31976;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B17F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639480].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639464].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1830;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 20;
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 84;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B185C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639584].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639568].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1898;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 20;
	ctx.r[11].s64 = ctx.r[11].s64 + -31792;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 84;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B18C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639688].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639672].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1900;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 3;
	ctx.r[11].s64 = ctx.r[11].s64 + -31708;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B192C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639792].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639776].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31996;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B1994;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639896].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639880].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B19D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31812;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B19FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200640000].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639984].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B17E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B17E8 size=2064
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B17F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639480].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639464].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1830;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 20;
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 84;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B185C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639584].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639568].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1898;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 20;
	ctx.r[11].s64 = ctx.r[11].s64 + -31792;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 84;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B18C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639688].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639672].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1900;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 3;
	ctx.r[11].s64 = ctx.r[11].s64 + -31708;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B192C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639792].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639776].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31996;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B1994;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639896].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639880].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B19D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31812;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B19FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200640000].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639984].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B1960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1960 size=5136
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B1968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31996;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B1994;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200639896].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639880].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B19D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31812;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B19FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200640000].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639984].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B19B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B19B8 size=5136
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639880].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B19D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -31812;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B19FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200640000].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639984].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B1A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1A10 size=5136
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200640000].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200639984].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1A68 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -31676;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1AF0 size=2064
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -31648;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1B68 size=2064
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B1C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1C28 size=2064
	if ctx.cr[0].eq {
			pc = 0x832B1C40; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B1C38;
	crate::recompiler::externs::call(&mut ctx, base, 0x824D7310);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B1C40;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B2010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2010 size=5136
	if !ctx.cr[2200641528].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200641512].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -31216;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2068 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -31116;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B20C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B20C0 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2918 size=1044
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B29A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B29A0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -30412;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2A18 size=6160
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2AB0 size=12
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -30320;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2B70 size=12
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
}

pub fn sub_832B2CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2CE0 size=5136
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B2CEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200644848].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200644832].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -30024;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B2D48; continue 'dispatch;
	}
	ctx.lr = 0x832B2D48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B2D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2D48 size=3088
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B2DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2DC0 size=5140
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2E18 size=5140
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -29928;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2E70 size=5140
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B2E8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200645264].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200645248].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -29808;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B2EE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82734D38);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B2EE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B2EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2EC8 size=5140
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -29808;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B2EE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82734D38);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B2EE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B2F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2F20 size=5140
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2F78 size=5140
	ctx.r[30].s64 = 3;
	ctx.r[11].s64 = ctx.r[11].s64 + 15108;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B2F9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200645536].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200645520].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -29748;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B2FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2FD0 size=5140
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -29748;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3028 size=5136
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3108 size=5140
	return;
}

pub fn sub_832B3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3160 size=7188
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -27984;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B3178;
	crate::recompiler::externs::call(&mut ctx, base, 0x8232A1E8);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B3180;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B31C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B31C0 size=7188
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832B3220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3220 size=7188
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832B3280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3280 size=6160
	crate::recompiler::externs::call(&mut ctx, base, 0x82632C10);
	return;
}

pub fn sub_832B32D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B32D8 size=6160
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -24916;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B32E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82632F48);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B32F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B33B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B33B0 size=4108
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	return;
}

pub fn sub_832B3400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3400 size=4112
	ctx.r[10].s64 = -2092171264;
	ctx.r[11].s64 = 0;
	ctx.r[9].s64 = ctx.r[10].s64 + 7120;
	ctx.r[10].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u8(base as *mut u8, (0 as u32), ctx.r[10].u8) };
	return;
}

pub fn sub_832B3450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3450 size=6156
	return;
}

pub fn sub_832B34A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B34A8 size=7188
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -22992;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3528 size=1036
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -22240;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B3550; continue 'dispatch;
	}
	ctx.lr = 0x832B3550;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B35B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B35B0 size=7188
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B3650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3650 size=1036
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B36E8 size=1044
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -22096;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B3708; continue 'dispatch;
	}
	ctx.lr = 0x832B3708;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B3780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3780 size=5136
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[31].s64 = ctx.r[11].s64 + -22052;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B37A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x822DC7F0);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B37A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B37D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B37D8 size=4112
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -22036;
	crate::recompiler::externs::call(&mut ctx, base, 0x82322558);
	return;
}

pub fn sub_832B3828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3828 size=4108
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200647720].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200647704].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21872;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B38A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B38A0 size=1040
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B38C0; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B38B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x823E78C8);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B38C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B39E0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21780;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3A38 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3A90 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21728;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3AE8 size=5136
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200648424].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200648408].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B3B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3B40 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3B98 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21568;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3BF0 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3C78 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21512;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3D00 size=6156
	return;
}

pub fn sub_832B3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3E00 size=1040
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21224;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3E48 size=4116
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3E98 size=16
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3ED8 size=16
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F68 size=6156
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B3FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3FC0 size=6156
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -21028;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B40E8 size=6160
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4140 size=1040
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -20488;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4270 size=6164
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -20244;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4318 size=6164
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4400 size=7184
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -20072;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B44A0 size=6164
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -20032;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4558 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B45D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B45D0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -19724;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4648 size=5136
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200651288].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B4660;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 15;
	ctx.r[11].s64 = ctx.r[11].s64 + -19520;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B468C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200651408].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200651392].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -19372;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B46C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B46C0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -19372;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4738 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B47B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B47B0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -19180;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4828 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B48A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B48A0 size=5136
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B48A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 6;
	ctx.r[11].s64 = ctx.r[11].s64 + -19000;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B48D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200651992].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200651976].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B4910;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 6;
	ctx.r[11].s64 = ctx.r[11].s64 + -18972;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B493C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200652096].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200652080].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4918 size=5136
	ctx.r[30].s64 = 6;
	ctx.r[11].s64 = ctx.r[11].s64 + -18972;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B493C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200652096].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200652080].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	return;
}

pub fn sub_832B49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B49A0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18928;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4A48 size=7188
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4AA8 size=7188
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18840;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C70 size=4108
	crate::recompiler::externs::call(&mut ctx, base, 0x826FE190);
	return;
}

pub fn sub_832B4D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4D30 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4DD8 size=7188
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18584;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4E38 size=7188
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F18 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FD0 size=4116
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18448;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5020 size=4116
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18428;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5070 size=4116
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18408;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B50C0 size=4116
	if ctx.cr[0].eq {
			pc = 0x832B50D8; continue 'dispatch;
	}
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B50D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A99900);
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = 0x832B50D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B5110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5110 size=4116
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18380;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5160 size=4116
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18360;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B51B0 size=7188
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18340;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5280 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -18224;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B52F8 size=6164
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B53A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53A0 size=6164
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B53A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 4;
	ctx.r[11].s64 = ctx.r[11].s64 + -18128;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B53D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200654808].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200654792].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2093875200;
	ctx.r[11].s64 = -2113929216;
	ctx.r[31].s64 = ctx.r[10].s64 + -23168;
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832B5434;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219B838);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17908;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53F8 size=5136
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200654792].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[10].s64 = -2093875200;
	ctx.r[11].s64 = -2113929216;
	ctx.r[31].s64 = ctx.r[10].s64 + -23168;
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	ctx.r[3].u64 = ctx.r[31].u64;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832B5434;
	crate::recompiler::externs::call(&mut ctx, base, 0x8219B838);
	ctx.r[11].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17908;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5508 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5580 size=5136
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200655224].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200655208].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17680;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B55D8 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17668;
	crate::recompiler::externs::call(&mut ctx, base, 0x828001B8);
	return;
}

pub fn sub_832B5630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5630 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B56A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B56A8 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17592;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B56E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B56E8 size=1040
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17576;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B5730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5730 size=1040
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5778 size=1044
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17540;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B5800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5800 size=1040
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B5868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5868 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17480;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B58A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B58A8 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17464;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B58E8 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17448;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5928 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17432;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE1F8);
	return;
}

pub fn sub_832B5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5968 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17408;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B59C0 size=2064
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B40 size=1040
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B88 size=1040
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -17300;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5BD0 size=1040
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5C18 size=1040
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5C60 size=1040
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -16016;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5CA8 size=1040
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5CF0 size=5140
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -15980;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DE0 size=4112
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -15920;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5E70 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -15868;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5EC8 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B5F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5F20 size=5136
	return;
}

pub fn sub_832B5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5FA8 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -15788;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6040 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B64A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B64A0 size=5136
	ctx.r[12].u64 = ctx.lr;
	ctx.lr = 0x832B64A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E08);
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092302336;
	ctx.r[30].s64 = 62;
	ctx.r[11].s64 = ctx.r[11].s64 + -14584;
	ctx.r[28].s64 = 0;
	ctx.r[31].s64 = ctx.r[11].s64 + 252;
	ctx.r[11].s64 = -2092367872;
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	ctx.r[3].u64 = ctx.r[31].u64;
	ctx.lr = 0x832B64D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821CE168);
	ctx.r[9].u64 = ctx.r[29].u64;
	ctx.r[10].u64 = ctx.msr;
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	ctx.reserved.u32 = unsafe { *(base.add(ctx.r[9].u32) as usize) as *const u32) };
	ctx.r[11].u64 = (ctx.reserved.u32.swap_bytes()) as u64;
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	if !ctx.cr[2200659160].eq {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[28].u32) };
	if !ctx.cr[2200659144].lt {
			// TODO: longjmp model
	unsafe { core::intrinsics::abort(); }
		return;
	}
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB0E58);
	return;
	ctx.r[11].s64 = -2092171264;
	ctx.r[3].s64 = ctx.r[11].s64 + 15416;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B66B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B66B8 size=7184
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6770 size=7188
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	return;
}

pub fn sub_832B67D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B67D0 size=7188
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B6830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6830 size=7188
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -13448;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B68F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68F8 size=6164
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B69A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B69A0 size=6156
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -13168;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B69F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B69F8 size=6160
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6A50 size=4116
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12872;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6AA0 size=7188
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6B00 size=6160
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12660;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6B58 size=6160
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6C38 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6CA0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12472;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6D38 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6D90 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12328;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6DE8 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6E40 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12200;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6EA8 size=6164
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6F10 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -12064;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7128 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B71B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B71B0 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -11644;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7228 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7280 size=4112
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -11592;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7348 size=4112
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B73C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B73C8 size=16
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7428 size=4112
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7498 size=4112
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7508 size=4112
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7578 size=4112
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B75E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B75E8 size=4116
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7638 size=4116
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7688 size=7180
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A98 size=5136
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -10040;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7AF0 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7B58 size=4112
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -9992;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7CE0 size=5140
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7D38 size=5140
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -9788;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7D90 size=5136
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B7E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7E08 size=3084
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -9652;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B8FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8FC8 size=16
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6880;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9100 size=7188
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B9160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9160 size=7188
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B91C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B91C0 size=7188
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9220 size=7188
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6792;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9370 size=2060
	ctx.r[11].s64 = -2092171264;
	ctx.r[31].s64 = ctx.r[11].s64 + 6752;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B9388; continue 'dispatch;
	}
	ctx.lr = 0x832B9388;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B9398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9398 size=2060
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B93C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B93C0 size=2060
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092171264;
	ctx.r[31].s64 = ctx.r[11].s64 + 6768;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B93E0; continue 'dispatch;
	}
	ctx.lr = 0x832B93E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B93E8 size=2060
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B9410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9410 size=2060
	ctx.r[12].u64 = ctx.lr;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[12].u32) };
	unsafe { crate::rt::store_u64(base as *mut u8, (0 as u32), ctx.r[31].u64) };
	ea = ctx.r[0].u32.wrapping_add(0 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[0].u32 = ea;
	ctx.r[11].s64 = -2092171264;
	ctx.r[31].s64 = ctx.r[11].s64 + 6784;
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.cr[0].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	if ctx.cr[0].eq {
			pc = 0x832B9438; continue 'dispatch;
	}
	ctx.lr = 0x832B9438;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B9438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9438 size=2060
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B9460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9460 size=2060
	return;
}

pub fn sub_832B9488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9488 size=2060
	if ctx.cr[0].eq {
			pc = 0x832B9490; continue 'dispatch;
	}
	ctx.lr = 0x832B9490;
	crate::recompiler::externs::call(&mut ctx, base, 0x82223738);
	ctx.r[11].s64 = 0;
	ctx.r[10].s64 = 0;
	ctx.r[9].s64 = 0;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[10].u32) };
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[9].u32) };
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B94B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B94B0 size=2060
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
}

pub fn sub_832B94D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B94D8 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9500 size=2060
	ctx.r[10].s64 = -2092302336;
	ctx.r[11].s64 = -2112946176;
	ctx.r[31].s64 = ctx.r[10].s64 + -6596;
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832B951C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231B398);
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	ctx.lr = 0x832B9524;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231B208);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6580;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9528 size=2060
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6580;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9550 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9578 size=2060
	ctx.r[10].s64 = -2092302336;
	ctx.r[11].s64 = -2112946176;
	ctx.r[31].s64 = ctx.r[10].s64 + -6568;
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832B9594;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231B398);
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	ctx.lr = 0x832B959C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231B208);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6552;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B95A0 size=2060
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6552;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B95C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B95C8 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B95F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B95F0 size=2060
	return;
}

pub fn sub_832B9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9618 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6524;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9640 size=2060
	ctx.r[31].s64 = ctx.r[10].s64 + -6520;
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	unsafe { crate::rt::store_u32(base as *mut u8, (0 as u32), ctx.r[11].u32) };
	ctx.lr = 0x832B9654;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231B398);
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	ctx.lr = 0x832B965C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231B208);
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	ctx.r[12].u64 = unsafe { crate::rt::load_u32(base as *const u8, 0u32) } as u64;
	ctx.lr = ctx.r[12].u64;
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6504;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9668 size=2060
	ctx.r[31].u64 = unsafe { crate::rt::load_u64(base as *const u8, 0u32) };
	return;
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6504;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9690 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6496;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B96B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B96B8 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B96E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B96E0 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6476;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9708 size=2060
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

pub fn sub_832B9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9730 size=2060
	ctx.r[11].s64 = -2092302336;
	ctx.r[3].s64 = ctx.r[11].s64 + -6456;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221C7D8);
	return;
}

