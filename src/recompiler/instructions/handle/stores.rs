// stores.rs (Rust-only emission)
use super::*;

pub(crate) fn handle_stb(ctx: &mut LowerCtx) -> bool {
    // RS is operand 0, MEM (disp(ra)) is operand 1 (D-form)
    let s = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs  = ctx.r(s).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u8( \
                    base as *mut u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32), \
                    {rs}.u8 \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u8( \
                    base as *mut u8, \
                    {imm}u32, \
                    {rs}.u8 \
                ) \
            }};"
        ));
    }

    true
}

pub(crate) fn handle_stbu(ctx: &mut LowerCtx) -> bool {
    // STBU rS, disp(rA)  (update: rA <- EA)
    let s = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs  = ctx.r(s).to_string();
    let ea  = ctx.ea().to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!("\t{ea} = {ra_s}.u32.wrapping_add({imm} as u32);"));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::store_u8(base as *mut u8, {ea}, {rs}.u8) }};"
        ));
        ctx.println_fmt(format_args!("\t{ra_s}.u32 = {ea};"));
    } else {
        // RA field == 0 is reserved in the ISA; we just treat it as absolute EA.
        ctx.println_fmt(format_args!("\t{ea} = {imm}u32;"));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::store_u8(base as *mut u8, {ea}, {rs}.u8) }};"
        ));
    }

    true
}

pub(crate) fn handle_stbx(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u8(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {rs}.u8) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u8(base as *mut u8, {rb}.u32, {rs}.u8) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_std(ctx: &mut LowerCtx) -> bool {
    // STD rS, disp(rA) (D-form, 64-bit)
    let s = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs  = ctx.r(s).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u64( \
                    base as *mut u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32), \
                    {rs}.u64 \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u64( \
                    base as *mut u8, \
                    {imm}u32, \
                    {rs}.u64 \
                ) \
            }};"
        ));
    }

    true
}

pub(crate) fn handle_stdcx(ctx: &mut LowerCtx) -> bool {
	// STDCX. rS, rA, rB  -> CR0.eq = CAS64(base+(ra+rb), expected=reserved, new=rS)
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs   = ctx.r(s).to_string();
	let rra  = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
	let rrb  = ctx.r(rb).to_string();
	let cr0  = ctx.cr(0).to_string();
	let xer  = ctx.xer().to_string();
	let rsvd = ctx.reserved().to_string();

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!("\tlet addr = {ra}.u32.wrapping_add({rb}.u32);", ra = ra_s, rb = rrb));
	} else {
		ctx.println_fmt(format_args!("\tlet addr = {rb}.u32;", rb = rrb));
	}

	ctx.println_fmt(format_args!("\t{cr}.lt = false;", cr = cr0));
	ctx.println_fmt(format_args!("\t{cr}.gt = false;", cr = cr0));
	ctx.println_fmt(format_args!(
		"\tlet ok = unsafe {{ crate::rt::stdcx64(base as *mut u8, addr, {rsvd}.u64 as u64, {rs}.u64) }};"
	));
	ctx.println_fmt(format_args!("\t{cr}.eq = ok;", cr = cr0));
	ctx.println_fmt(format_args!("\t{cr}.so = {xer}.so;", cr = cr0, xer = xer));
	true
}

pub(crate) fn handle_stdu(ctx: &mut LowerCtx) -> bool {
    // STDU rS, disp(rA) (update: rA <- EA)
    let s = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs  = ctx.r(s).to_string();
    let ea  = ctx.ea().to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!("\t{ea} = {ra_s}.u32.wrapping_add({imm} as u32);"));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::store_u64(base as *mut u8, {ea}, {rs}.u64) }};"
        ));
        ctx.println_fmt(format_args!("\t{ra_s}.u32 = {ea};"));
    } else {
        ctx.println_fmt(format_args!("\t{ea} = {imm}u32;"));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::store_u64(base as *mut u8, {ea}, {rs}.u64) }};"
        ));
    }

    true
}

pub(crate) fn handle_stdx(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u64(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {rs}.u64) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u64(base as *mut u8, {rb}.u32, {rs}.u64) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_stfd(ctx: &mut LowerCtx) -> bool {
    // STFD fS, disp(rA)
    ctx.set_flush_mode(false);
    let f = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let ff  = ctx.f(f).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u64( \
                    base as *mut u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32), \
                    {ff}.u64 \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u64( \
                    base as *mut u8, \
                    {imm}u32, \
                    {ff}.u64 \
                ) \
            }};"
        ));
    }

    true
}

pub(crate) fn handle_stfdx(ctx: &mut LowerCtx) -> bool {
	ctx.set_flush_mode(false);
	let f  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let ff  = ctx.f(f).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u64(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {ff}.u64) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u64(base as *mut u8, {rb}.u32, {ff}.u64) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_stfiwx(ctx: &mut LowerCtx) -> bool {
	ctx.set_flush_mode(false);
	let f  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let ff = ctx.f(f).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
	let t   = ctx.temp().to_string();

	ctx.println_fmt(format_args!("\t{t}.f32 = ({ff}.f64 as f32);"));
	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {t}.u32) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {rb}.u32, {t}.u32) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_stfs(ctx: &mut LowerCtx) -> bool {
    // STFS fS, disp(rA)
    ctx.set_flush_mode(false);
    let f = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let ff  = ctx.f(f).to_string();
    let t   = ctx.temp().to_string();
    let imm = disp as i32;

    ctx.println_fmt(format_args!("\t{t}.f32 = ({ff}.f64 as f32);"));

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u32( \
                    base as *mut u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32), \
                    {t}.u32 \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u32( \
                    base as *mut u8, \
                    {imm}u32, \
                    {t}.u32 \
                ) \
            }};"
        ));
    }

    true
}

pub(crate) fn handle_stfsx(ctx: &mut LowerCtx) -> bool {
	ctx.set_flush_mode(false);
	let f  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let ff  = ctx.f(f).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
	let t   = ctx.temp().to_string();

	ctx.println_fmt(format_args!("\t{t}.f32 = ({ff}.f64 as f32);"));
	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {t}.u32) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {rb}.u32, {t}.u32) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_sth(ctx: &mut LowerCtx) -> bool {
    // RS is operand 0
    let s = ctx.op_reg(0);
    // MEM (disp(ra)) is operand 1
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs  = ctx.r(s).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u16( \
                    base as *mut u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32), \
                    {rs}.u16 \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u16( \
                    base as *mut u8, \
                    {imm}u32, \
                    {rs}.u16 \
                ) \
            }};"
        ));
    }

    true
}

pub(crate) fn handle_sthbrx(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
	let val = format!("({rs}.u16.swap_bytes())");

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u16(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {val}) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u16(base as *mut u8, {rb}.u32, {val}) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_sthx(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u16(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {rs}.u16) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u16(base as *mut u8, {rb}.u32, {rs}.u16) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_stw(ctx: &mut LowerCtx) -> bool {
    // RS is operand 0
    let s = ctx.op_reg(0);
    // MEM (disp(ra)) is operand 1
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs = ctx.r(s).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u32( \
                    base as *mut u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32), \
                    {rs}.u32 \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                crate::rt::store_u32( \
                    base as *mut u8, \
                    {imm}u32, \
                    {rs}.u32 \
                ) \
            }};"
        ));
    }

    true
}



pub(crate) fn handle_stwbrx(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
	let val = format!("({rs}.u32.swap_bytes())");

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {val}) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {rb}.u32, {val}) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_stwcx(ctx: &mut LowerCtx) -> bool {
    // STWCX. rS, rA, rB  -> CR0.eq = CAS32(base+(ra+rb), expected=reserved, new=rS)
    let s  = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);

    let rs   = ctx.r(s).to_string();
    let rrb  = ctx.r(rb).to_string();
    let rra  = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let cr0  = ctx.cr(0).to_string();
    let xer  = ctx.xer().to_string();
    let rsvd = ctx.reserved().to_string();

    ctx.println("\t// stwcx.");

    // Effective address: if RA == 0, EA = RB; else EA = RA + RB
    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!("\tlet addr = {ra}.u32.wrapping_add({rb}.u32);", ra = ra_s, rb = rrb));
    } else {
        ctx.println_fmt(format_args!("\tlet addr = {rb}.u32;", rb = rrb));
    }

    ctx.println_fmt(format_args!("\t{cr}.lt = false;", cr = cr0));
    ctx.println_fmt(format_args!("\t{cr}.gt = false;", cr = cr0));
    ctx.println_fmt(format_args!(
        "\tlet ok = unsafe {{ crate::rt::stwcx32(base as *mut u8, addr, {rsvd}.u32, {rs}.u32) }};"
    ));
    ctx.println_fmt(format_args!("\t{cr}.eq = ok;", cr = cr0));
    ctx.println_fmt(format_args!("\t{cr}.so = {xer}.so;", cr = cr0, xer = xer));
    true
}

pub(crate) fn handle_stwu(ctx: &mut LowerCtx) -> bool {
    // STWU rS, disp(rA) (update: rA <- EA)
    let s = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rs  = ctx.r(s).to_string();
    let ea  = ctx.ea().to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!("\t{ea} = {ra_s}.u32.wrapping_add({imm} as u32);"));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ea}, {rs}.u32) }};"
        ));
        ctx.println_fmt(format_args!("\t{ra_s}.u32 = {ea};"));
    } else {
        ctx.println_fmt(format_args!("\t{ea} = {imm}u32;"));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ea}, {rs}.u32) }};"
        ));
    }

    true
}

pub(crate) fn handle_stwux(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rra = ctx.r(ra).to_string();
	let rrb = ctx.r(rb).to_string();
	let ea  = ctx.ea().to_string();

	ctx.println_fmt(format_args!("\t{ea} = {ra}.u32.wrapping_add({rb}.u32);", ra = rra, rb = rrb));
	ctx.println_fmt(format_args!("\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ea}, {rs}.u32) }};"));
	ctx.println_fmt(format_args!("\t{ra}.u32 = {ea};", ra = rra));
	true
}

pub(crate) fn handle_stwx(ctx: &mut LowerCtx) -> bool {
	let s  = ctx.op_reg(0);
	let ra  = ctx.op_reg(1);
	let rb  = ctx.op_reg(2);

	let rs  = ctx.r(s).to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ra}.u32.wrapping_add({rb}.u32), {rs}.u32) }};",
			ra = ra_s, rb = rrb
		));
	} else {
		ctx.println_fmt(format_args!(
			"\tunsafe {{ crate::rt::store_u32(base as *mut u8, {rb}.u32, {rs}.u32) }};",
			rb = rrb
		));
	}
	true
}

pub(crate) fn handle_stvehx(ctx: &mut LowerCtx) -> bool {
	// store a half at (ea & ~1) selecting lane by ((ea & 0xF)>>1) with “from-left” Altivec mapping
	let vd = ctx.op_reg(0);
	let ra = ctx.op_reg(1);
	let rb = ctx.op_reg(2);

	let ea  = ctx.ea().to_string();
	let rrb = ctx.r(rb).to_string();
	let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
	let vvd = ctx.v(vd).to_string();

	// ea = (rA + rB) & !1
	if let Some(ra_s) = rra {
		ctx.println_fmt(format_args!("\t{ea} = ({ra}.u32.wrapping_add({rb}.u32)) & !1u32;", ra = ra_s, rb = rrb));
	} else {
		ctx.println_fmt(format_args!("\t{ea} = ({rb}.u32) & !1u32;", rb = rrb));
	}

	ctx.println_fmt(format_args!("\tlet idx = 7usize - ((({ea} & 0xFu32) >> 1) as usize);"));
	ctx.println_fmt(format_args!(
		"\tunsafe {{ crate::rt::store_u16(base as *mut u8, {ea}, {vvd}.u16[idx]) }};"
	));
	true
}
