use super::*;

pub(crate) fn handle_vpermwi128(ctx: &mut LowerCtx) -> bool {
	// Shuffle 32-bit lanes using PPC imm semantics (accounting for 3-idx reversal)
	let imm = ctx.op_imm(2) as u32;
	let x = 3 - (imm & 0x3);
	let y = 3 - ((imm >> 2) & 0x3);
	let z = 3 - ((imm >> 4) & 0x3);
	let w = 3 - ((imm >> 6) & 0x3);

	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\t// vd.u32[0..4] = va.u32[[x,y,z,w]]");
	ctx.println_fmt(format_args!("\t{vd}.u32[0] = {va}.u32[{x} as usize];"));
	ctx.println_fmt(format_args!("\t{vd}.u32[1] = {va}.u32[{y} as usize];"));
	ctx.println_fmt(format_args!("\t{vd}.u32[2] = {va}.u32[{z} as usize];"));
	ctx.println_fmt(format_args!("\t{vd}.u32[3] = {va}.u32[{w} as usize];"));
	true
}

pub(crate) fn handle_vpkd3d128(ctx: &mut LowerCtx) -> bool {
	// Vector-pack (D3D / f16x4 forms), Rust-only
	ctx.set_flush_mode(true);
	let a = ctx.op_reg(1);
	let kind = ctx.op_imm(2) as u32;
	let _aux  = ctx.op_imm(3) as u32; // kept for parity with original, still unused here
	let dstw = ctx.op_imm(4) as usize;
	let d = ctx.op_reg(0);

	let va = ctx.v(a).to_string();
	let vd = ctx.v(d).to_string();
	let vtemp = ctx.vtemp().to_string();
	let tmp = ctx.temp().to_string();

	match kind {
		0 => {
			// D3D color pack into vd.u32[dstw]
			ctx.println_fmt(format_args!("\t{tmp}.u32 = 0;"));
			for i in 0..4 {
				// clamp to [3.0, 3.0] as in your original trick (0x40400000 == 3.0f)
				ctx.println_fmt(format_args!("\t{vtemp}.u32[{i}] = 0x4040_00FF;"));
				ctx.println_fmt(format_args!(
					"\t{vtemp}.f32[{i}] = if {va}.f32[{i}] < 3.0 {{ 3.0 }} else if {va}.f32[{i}] > {vtemp}.f32[{i}] {{ {vtemp}.f32[{i}] }} else {{ {va}.f32[{i}] }};"
				));
				// take low byte from that word in the swizzled order [3,0,1,2]
				let idx = [3, 0, 1, 2][i];
				let shift = idx * 8;
				let byte_ix = i * 4;
				ctx.println_fmt(format_args!(
					"\t{tmp}.u32 |= (u32::from({vtemp}.u8[{byte_ix}])) << {shift};"
				));
			}
			ctx.println_fmt(format_args!("\t{vd}.u32[{dstw}] = {tmp}.u32;"));
		}
		5 => {
			// float16_4 pack: vd.u16[dstw*2 .. dstw*2+4] = f32_to_f16_sat(va.f32[i]) with sign
			// Requires runtime helper: crate::rt::f32_to_f16_bits_sat(x: f32) -> u16
			for i in 0..4 {
				ctx.println_fmt(format_args!("\tlet mut bits: u16 = crate::rt::f32_to_f16_bits_sat({va}.f32[{i}]);"));
				ctx.println_fmt(format_args!("\tbits |= (({va}.u32[{i}] >> 16) & 0x8000) as u16;"));
				let dst_ix = i + 2 * dstw;
				ctx.println_fmt(format_args!("\t{vd}.u16[{dst_ix}] = bits;"));
			}
		}
		_ => {
			ctx.println("\t// unhandled vpkd3d128 kind");
			ctx.println("\tcore::intrinsics::abort();");
		}
	}
	true
}

pub(crate) fn handle_vpkshus_like(ctx: &mut LowerCtx) -> bool {
	// Pack signed 16-bit -> unsigned 8-bit with saturation: vb (high) then va (low)
	let d = ctx.op_reg(0); let b = ctx.op_reg(2); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let vb = ctx.v(b).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..8 {");
	ctx.println_fmt(format_args!("\t\tlet x = {vb}.s16[i] as i32;"));
	ctx.println("\t\tlet y = if x < 0 { 0 } else if x > 255 { 255 } else { x as i32 };");
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = y as u8;"));
	ctx.println("\t}");
	ctx.println("\tfor i in 0..8 {");
	ctx.println_fmt(format_args!("\t\tlet x = {va}.s16[i] as i32;"));
	ctx.println("\t\tlet y = if x < 0 { 0 } else if x > 255 { 255 } else { x as i32 };");
	ctx.println_fmt(format_args!("\t\t{vd}.u8[i + 8] = y as u8;"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vupkd3d128(ctx: &mut LowerCtx) -> bool {
	// Unpack (D3D / 2 shorts) with reversal
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let kind = ((ctx.op_imm(2) as u32) >> 2) & 0x3;

	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();
	let vtemp = ctx.vtemp().to_string();
	let tmp = ctx.temp().to_string();

	match kind {
		0 => {
			// D3D: broadcast to [R,G,B,A] as floats with A=1.0, source is bytes in order [3,0,1,2]
			for i in 0..4 {
				let idx = [3, 0, 1, 2][i];
				ctx.println_fmt(format_args!("\t{vtemp}.u32[{i}] = (u32::from({va}.u8[{idx}])) | 0x3F80_0000;"));
			}
			ctx.println_fmt(format_args!("\t{vd} = {vtemp};"));
		}
		1 => {
			// Two shorts -> place into x,z as 3.0 + adds; y=0, w=1 (preserves your reversal intent)
			for i in 0..2 {
				let src_ix = 1 - i;
				let dst_ix = 3 - i;
				ctx.println_fmt(format_args!("\t{tmp}.f32 = 3.0;"));
				ctx.println_fmt(format_args!("\t{tmp}.f32 = {tmp}.f32 + ({va}.s16[{src_ix}] as f32);"));
				ctx.println_fmt(format_args!("\t{vtemp}.f32[{dst_ix}] = {tmp}.f32;"));
			}
			ctx.println_fmt(format_args!("\t{vtemp}.f32[1] = 0.0;"));
			ctx.println_fmt(format_args!("\t{vtemp}.f32[0] = 1.0;"));
			ctx.println_fmt(format_args!("\t{vd} = {vtemp};"));
		}
		_ => {
			ctx.println("\t// unhandled vupkd3d128 kind");
			ctx.println("\tcore::intrinsics::abort();");
		}
	}
	true
}

pub(crate) fn handle_vupkhsb_like(ctx: &mut LowerCtx) -> bool {
	// Sign-extend high 8 bytes -> 8 halfwords
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..8 {");
	ctx.println_fmt(format_args!("\t\t{vd}.s16[i] = {va}.s8[i + 8] as i16;"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vupkhsh_like(ctx: &mut LowerCtx) -> bool {
	// Sign-extend high 4 halfwords -> 4 words
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.s32[i] = {va}.s16[i + 4] as i32;"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vupklsb_like(ctx: &mut LowerCtx) -> bool {
	// Sign-extend low 8 bytes -> 8 halfwords
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..8 {");
	ctx.println_fmt(format_args!("\t\t{vd}.s16[i] = {va}.s8[i] as i16;"));
	ctx.println("\t}");
	true
}

pub(crate) fn handle_vupklsh_like(ctx: &mut LowerCtx) -> bool {
	// Sign-extend low 4 halfwords -> 4 words
	let d = ctx.op_reg(0); let a = ctx.op_reg(1);
	let vd = ctx.v(d).to_string();
	let va = ctx.v(a).to_string();

	ctx.println("\tfor i in 0..4 {");
	ctx.println_fmt(format_args!("\t\t{vd}.s32[i] = {va}.s16[i] as i32;"));
	ctx.println("\t}");
	true
}
