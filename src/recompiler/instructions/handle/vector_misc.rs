use super::*;

pub(crate) fn handle_vrefp_like(ctx: &mut LowerCtx) -> bool {
	// VREFP / VREFP128
	ctx.set_flush_mode(true);
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = 1.0f32 / {va}.f32[i];"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vrfi_round(ctx: &mut LowerCtx, mode: &str) -> bool {
	// common helper for VRFIM (neg), VRFIN (nearest), VRFIZ (zero)
	ctx.set_flush_mode(true);
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	match mode {
		"neg" => {
			ctx.println("\tfor i in 0..4 {");
			ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].floor();"));
			ctx.println("\t}");
		}
		"nearest" => {
			ctx.println("\tfor i in 0..4 {");
			ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].round();"));
			ctx.println("\t}");
		}
		_ => {
			// default: toward zero
			ctx.println("\tfor i in 0..4 {");
			ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].trunc();"));
			ctx.println("\t}");
		}
	}
	true
}

pub(crate) fn handle_vrsqrtefp_like(ctx: &mut LowerCtx) -> bool {
	// VRSQRTEFP / VRSQRTEFP128
	ctx.set_flush_mode(true);
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = 1.0f32 / {va}.f32[i].sqrt();"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vexptefp_like(ctx: &mut LowerCtx) -> bool {
	// VEXPTEFP / VEXPTEFP128
	ctx.set_flush_mode(true);
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].exp2();"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vlogefp_like(ctx: &mut LowerCtx) -> bool {
	// VLOGEFP / VLOGEFP128
	ctx.set_flush_mode(true);
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].log2();"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vmr(ctx: &mut LowerCtx) -> bool {
    // vmr vD, vS  — Xenon alias: vd = vs
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let vd = ctx.v(d).to_string();
    let vs = ctx.v(s).to_string();

    ctx.println_fmt(format_args!("\t{vd} = {vs};"));
    true
}