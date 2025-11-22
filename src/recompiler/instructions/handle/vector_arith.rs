// vector_arith.rs
use super::*;

pub(crate) fn handle_vaddfp_like(ctx: &mut LowerCtx) -> bool {
    // VADDFP / VADDFP128 : per-lane f32 add
    ctx.set_flush_mode(true);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string(); let va = ctx.v(a).to_string(); let vb = ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i] + {vb}.f32[i];"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vaddshs(ctx: &mut LowerCtx) -> bool {
    // saturated i16 add
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..8 {");
    ctx.println_fmt(format_args!("\t\t{vd}.s16[i] = {va}.s16[i].saturating_add({vb}.s16[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vaddubm(ctx: &mut LowerCtx) -> bool {
    // modulo u8 add
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[i].wrapping_add({vb}.u8[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vaddubs(ctx: &mut LowerCtx) -> bool {
    // saturated u8 add
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[i].saturating_add({vb}.u8[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vadduhm(ctx: &mut LowerCtx) -> bool {
    // modulo u16 add
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..8 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u16[i] = {va}.u16[i].wrapping_add({vb}.u16[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vadduwm(ctx: &mut LowerCtx) -> bool {
    // modulo u32 add
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = {va}.u32[i].wrapping_add({vb}.u32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vadduws(ctx: &mut LowerCtx) -> bool {
    // saturated u32 add
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = {va}.u32[i].saturating_add({vb}.u32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vmaddfp_like(ctx: &mut LowerCtx) -> bool {
    // d = a*b + c (per-lane f32)
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2); let c=ctx.op_reg(3);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string(); let vc=ctx.v(c).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i] * {vb}.f32[i] + {vc}.f32[i];"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vmaxfp_like(ctx: &mut LowerCtx) -> bool {
    // per-lane f32 max (NaN propagates per Rust semantics)
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].max({vb}.f32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vmaxsw(ctx: &mut LowerCtx) -> bool {
    // per-lane i32 max
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.s32[i] = {va}.s32[i].max({vb}.s32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vminfp_like(ctx: &mut LowerCtx) -> bool {
    // per-lane f32 min
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i].min({vb}.f32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vmsum3fp128(ctx: &mut LowerCtx) -> bool {
    // dot over y,z,w (mask 0xEF), splatted to all lanes
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println_fmt(format_args!(
        "\tlet dot = {va}.f32[1] * {vb}.f32[1] \
                 + {va}.f32[2] * {vb}.f32[2] \
                 + {va}.f32[3] * {vb}.f32[3];"
    ));
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = dot;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vmsum4fp128(ctx: &mut LowerCtx) -> bool {
    // full 4-lane dot product, splatted
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println_fmt(format_args!(
        "\tlet dot = {va}.f32[0]*{vb}.f32[0] \
                 + {va}.f32[1]*{vb}.f32[1] \
                 + {va}.f32[2]*{vb}.f32[2] \
                 + {va}.f32[3]*{vb}.f32[3];"
    ));
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = dot;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vmulfp128(ctx: &mut LowerCtx) -> bool {
    // per-lane f32 mul
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i] * {vb}.f32[i];"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vavgsb(ctx: &mut LowerCtx) -> bool {
    // unsigned byte average with rounding (matches earlier SIMDe mapping you had)
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\tlet s = {va}.u8[i] as u16 + {vb}.u8[i] as u16 + 1;"));
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = (s >> 1) as u8;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vavgsh(ctx: &mut LowerCtx) -> bool {
    // unsigned halfword average with rounding (keeps prior behavior)
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..8 {");
    ctx.println_fmt(format_args!("\t\tlet s = {va}.u16[i] as u32 + {vb}.u16[i] as u32 + 1;"));
    ctx.println_fmt(format_args!("\t\t{vd}.u16[i] = (s >> 1) as u16;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vavgub(ctx: &mut LowerCtx) -> bool {
    // unsigned byte average with rounding
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\tlet s = {va}.u8[i] as u16 + {vb}.u8[i] as u16 + 1;"));
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = (s >> 1) as u8;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vctsx_like(ctx: &mut LowerCtx) -> bool {
    // float -> signed i32 with scale 2^sh, truncation + saturation
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1);
    let sh = ctx.op_imm(2) as i32;
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string();
    ctx.println_fmt(format_args!("\tlet scale = f32::from_bits(((127u32 + ({sh} as u32)) << 23));"));
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\tlet x = ({va}.f32[i] * scale).trunc();"));
    ctx.println_fmt(format_args!("\t\tlet y = if x.is_nan() {{ 0.0 }} else {{ x }};"));
    ctx.println_fmt(format_args!("\t\tlet y = y.clamp(i32::MIN as f32, i32::MAX as f32);"));
    ctx.println_fmt(format_args!("\t\t{vd}.s32[i] = y as i32;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vcfsx_like(ctx: &mut LowerCtx) -> bool {
    // signed i32 -> float with scale 2^-sh
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1);
    let sh = ctx.op_imm(2) as i32;
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string();
    ctx.println_fmt(format_args!("\tlet scale = f32::from_bits(((127u32 - ({sh} as u32)) << 23));"));
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = ({va}.s32[i] as f32) * scale;"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vcfux_like(ctx: &mut LowerCtx) -> bool {
    // vcuxwfp128 vd, vb, uimm
    let vd_idx = ctx.op_reg(0);
    let vb_idx = ctx.op_reg(1);
    let uimm   = (ctx.op_imm(2) as i32) & 0x1F;

    let vd = ctx.v(vd_idx).to_string();
    let vb = ctx.v(vb_idx).to_string();

    ctx.println_fmt(format_args!(
        "\t// vcfux/vcuxwfp128: {vd}.f32[i] = ( {vb}.u32[i] as f32 ) * (2.0f32).powi({uimm});"
    ));
    ctx.println_fmt(format_args!(
        "\tfor i in 0..4 {{ {vd}.f32[i] = ({vb}.u32[i] as f32) * (2.0f32).powi({uimm}); }}"
    ));
    true
}

pub(crate) fn handle_vnmsubfp_like(ctx: &mut LowerCtx) -> bool {
    // d = -(a*b - c) == c - a*b (per-lane f32)
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2); let c=ctx.op_reg(3);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string(); let vc=ctx.v(c).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {vc}.f32[i] - ({va}.f32[i] * {vb}.f32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vsubfp_like(ctx: &mut LowerCtx) -> bool {
    // per-lane f32 subtract
    ctx.set_flush_mode(true);
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.f32[i] = {va}.f32[i] - {vb}.f32[i];"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vsubsws_scalar(ctx: &mut LowerCtx) -> bool {
    // saturated i32 subtract
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.s32[i] = {va}.s32[i].saturating_sub({vb}.s32[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vsububs(ctx: &mut LowerCtx) -> bool {
    // saturated u8 subtract
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = {va}.u8[i].saturating_sub({vb}.u8[i]);"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vsubuhm(ctx: &mut LowerCtx) -> bool {
    // modulo u16 subtract
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let vd=ctx.v(d).to_string(); let va=ctx.v(a).to_string(); let vb=ctx.v(b).to_string();
    ctx.println("\tfor i in 0..8 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u16[i] = {va}.u16[i].wrapping_sub({vb}.u16[i]);"));
    ctx.println("\t}");
    true
}
