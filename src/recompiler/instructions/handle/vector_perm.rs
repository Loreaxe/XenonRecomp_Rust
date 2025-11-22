use super::*;

pub(crate) fn handle_vmrg_hilo_common(ctx: &mut LowerCtx, kind: &str, hi: bool) -> bool {
	// Interleave high/low halves of A and B into D (Altivec unpackhi/unpacklo analogue).
	let d = ctx.op_reg(0); let a = ctx.op_reg(2); let b = ctx.op_reg(1); // keep your operand swap
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();

	match kind {
		"epi8" => {
			let start = if hi { 8 } else { 0 };
			ctx.println("\tfor i in 0..8 {");
			ctx.println_fmt(format_args!("\t\t{vd}.u8[2*i]     = {va}.u8[{start} + i];"));
			ctx.println_fmt(format_args!("\t\t{vd}.u8[2*i + 1] = {vb}.u8[{start} + i];"));
			ctx.println("\t}");
		}
		"epi16" => {
			let start = if hi { 4 } else { 0 };
			ctx.println("\tfor i in 0..4 {");
			ctx.println_fmt(format_args!("\t\t{vd}.u16[2*i]     = {va}.u16[{start} + i];"));
			ctx.println_fmt(format_args!("\t\t{vd}.u16[2*i + 1] = {vb}.u16[{start} + i];"));
			ctx.println("\t}");
		}
		_ => { // epi32
			let start = if hi { 2 } else { 0 };
			ctx.println("\tfor i in 0..2 {");
			ctx.println_fmt(format_args!("\t\t{vd}.u32[2*i]     = {va}.u32[{start} + i];"));
			ctx.println_fmt(format_args!("\t\t{vd}.u32[2*i + 1] = {vb}.u32[{start} + i];"));
			ctx.println("\t}");
		}
	}
	true
}

pub(crate) fn handle_vor_like(ctx: &mut LowerCtx) -> bool {
	let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();

	if a != b {
		ctx.println("\tfor i in 0..16 {");
		ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[i] | {vb}.u8[i];"));
		ctx.println("\t}");
	} else {
		ctx.println_fmt(format_args!("\t{vd} = {va};"));
	}
	true
}

pub(crate) fn handle_vperm_like(ctx: &mut LowerCtx) -> bool {
	// Altivec vperm: each control byte selects from (A||B) 32-byte concat using low 5 bits.
	let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();
	let vc = ctx.v(c).to_string();

	ctx.println("\tfor i in 0..16 {");
	ctx.println_fmt(format_args!("\t\tlet sel = ({vc}.u8[i] & 0x1F) as usize;"));
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = if sel < 16 {{ {va}.u8[sel] }} else {{ {vb}.u8[sel - 16] }};"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vsel(ctx: &mut LowerCtx) -> bool {
	// (a & mask) | (b & ~mask)
	let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let m = ctx.op_reg(3);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();
	let vm = ctx.v(m).to_string();

	ctx.println("\tfor i in 0..16 {");
	ctx.println_fmt(format_args!("\t\tlet mask = {vm}.u8[i];"));
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = ({va}.u8[i] & mask) | ({vb}.u8[i] & !mask);"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vspltish(ctx: &mut LowerCtx) -> bool {
    // VSPLTISH: splat signed 5-bit immediate into all 16-bit lanes.
    let d = ctx.op_reg(0);
    let imm_raw = ctx.op_imm(1) as i32;

    // Sign-extend 5-bit immediate (SIMM5) to 16 bits.
    // (shift left 27, then arithmetic right 27: keep low 5 bits, sign-extend)
    let val = ((imm_raw << 27) >> 27) as i16;

    let vd = ctx.v(d).to_string();

    ctx.println("\tfor i in 0..8 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u16[i] = {val} as u16;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vspltb(ctx: &mut LowerCtx) -> bool {
	// Broadcast a selected byte (with your reversal)
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let p = 15usize - (ctx.op_imm(2) as usize);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..16 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[{p}];"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vsplth(ctx: &mut LowerCtx) -> bool {
	// Broadcast a selected halfword (with your reversal)
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let p = 7usize - (ctx.op_imm(2) as usize);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..8 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u16[i] = {va}.u16[{p}];"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vspltisb(ctx: &mut LowerCtx) -> bool {
	let d = ctx.op_reg(0);
	let imm = ctx.op_imm(1) as i8; // VSPLTISB is signed 8-bit
	let vd = ctx.v(d).to_string();

	ctx.println("\tfor i in 0..16 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {imm} as u8;"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vspltisw_like(ctx: &mut LowerCtx) -> bool {
	// VSPLTISW / VSPLTISW128 — treat immediate as 32-bit pattern
	let d = ctx.op_reg(0);
	let imm = ctx.op_imm(1) as u32;
	let vd = ctx.v(d).to_string();

	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = {imm};"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vspltw_like(ctx: &mut LowerCtx) -> bool {
	// Broadcast a selected word (with reversal)
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let p = 3usize - (ctx.op_imm(2) as usize);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = {va}.u32[{p}];"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vrlimi128(ctx: &mut LowerCtx) -> bool {
	// Blend lanes of vd with permuted lanes of va based on mask.
	// sel ∈ {0,1,2,3} picks one of four shuffles of va.f32.
	let blend_mask = ctx.op_imm(2) as u32;
	let sel = (ctx.op_imm(3) as usize) & 3;

	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	// Build a permuted temporary
	let tmp = ctx.temp().to_string();
	ctx.println_fmt(format_args!("\t// build permuted temp from {va}.f32 using sel={sel}"));
	ctx.println_fmt(format_args!("\t// sel=0=>[0,1,2,3], sel=1=>[3,0,1,2], sel=2=>[2,3,0,1], sel=3=>[1,2,3,0]"));
	ctx.println_fmt(format_args!("\t{tmp}.f32 = 0.0; // init scratch"));

	// Emit the explicit mapping
	match sel {
		0 => { // identity
			ctx.println_fmt(format_args!("\tlet perm = [{va}.f32[0], {va}.f32[1], {va}.f32[2], {va}.f32[3]];"));
		}
		1 => { // [3,0,1,2]
			ctx.println_fmt(format_args!("\tlet perm = [{va}.f32[3], {va}.f32[0], {va}.f32[1], {va}.f32[2]];"));
		}
		2 => { // [2,3,0,1]
			ctx.println_fmt(format_args!("\tlet perm = [{va}.f32[2], {va}.f32[3], {va}.f32[0], {va}.f32[1]];"));
		}
		_ => { // 3 => [1,2,3,0]
			ctx.println_fmt(format_args!("\tlet perm = [{va}.f32[1], {va}.f32[2], {va}.f32[3], {va}.f32[0]];"));
		}
	}

	// Lane-wise blend using mask bits
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\tlet take = ((({blend_mask}u32) >> i) & 1) != 0;"));
	ctx.println_fmt(format_args!("\t\tif take {{ {vd}.f32[i] = perm[i]; }}"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vand_like(ctx: &mut LowerCtx) -> bool {
	let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();

	ctx.println("\tfor i in 0..16 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[i] & {vb}.u8[i];"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vandc128(ctx: &mut LowerCtx) -> bool {
	// andnot: (~a) & b (operand order matches your C++)
	let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();

	ctx.println("\tfor i in 0..16 {");
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = (!{va}.u8[i]) & {vb}.u8[i];"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vxor_like(ctx: &mut LowerCtx) -> bool {
	let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vb = ctx.v(b).to_string();

	if a != b {
		ctx.println("\tfor i in 0..16 {");
		ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[i] ^ {vb}.u8[i];"));
		ctx.println("\t}");
	} else {
		// a XOR a = 0
		ctx.println("\tfor i in 0..16 {");
		ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = 0;"));
		ctx.println("\t}");
	}
	true
}
