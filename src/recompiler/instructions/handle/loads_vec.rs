// src/recompiler/instuctions/handle/loads_vecs.rs
use super::*;

/// Core LVEX/LVX family: emit using explicit indices (vd, ra, rb).
pub(crate) fn emit_lvewx_at(ctx: &mut LowerCtx, vd: usize, ra: usize, rb: usize) -> bool {
    let t   = ctx.temp().to_string();
    let vvd = ctx.v(vd).to_string();
    let rra = ctx.r(ra).to_string();
    let rrb = ctx.r(rb).to_string();

    // addr = (ra + rb) & !0xF
    ctx.print(format!("\t{t}.u32 = "));
    if ra != 0 {
        ctx.print(format!("{rra}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));
    ctx.println_fmt(format_args!("\t{t}.u32 &= !0xFu32;"));

    // Note: actual vector shuffle/load is deferred to your runtime
    ctx.println_fmt(format_args!(
        "\t// load 16B at {t}.u32 into {vvd} using VectorMaskL[({t}.u32 & 0xF)]"
    ));
    true
}

/// LVEWX/LVX family (Capstone-decoded normal path)
pub(crate) fn handle_lvewx_like(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);
    emit_lvewx_at(ctx, vd, ra, rb)
}


/// Core LVLX family with explicit (vd, ra, rb).
pub(crate) fn emit_lvlx_at(ctx: &mut LowerCtx, vd: usize, ra: usize, rb: usize) -> bool {
    let t   = ctx.temp().to_string();
    let vvd = ctx.v(vd).to_string();
    let rra = ctx.r(ra).to_string();
    let rrb = ctx.r(rb).to_string();

    ctx.print(format!("\t{t}.u32 = "));
    if ra != 0 {
        ctx.print(format!("{rra}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));
    ctx.println_fmt(format_args!(
        "\t// load shuffled into {vvd} using VectorMaskL[({t}.u32 & 0xF)]"
    ));
    true
}

/// LVLX family: left-aligned vector load with mask table
pub(crate) fn handle_lvlx_like(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);
    emit_lvlx_at(ctx, vd, ra, rb)
}


/// Core LVRX family with explicit (vd, ra, rb).
pub(crate) fn emit_lvrx_at(ctx: &mut LowerCtx, vd: usize, ra: usize, rb: usize) -> bool {
    let t   = ctx.temp().to_string();
    let vvd = ctx.v(vd).to_string();
    let rra = ctx.r(ra).to_string();
    let rrb = ctx.r(rb).to_string();

    ctx.print(format!("\t{t}.u32 = "));
    if ra != 0 {
        ctx.print(format!("{rra}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));
    ctx.println_fmt(format_args!(
        "\t// load reversed into {vvd} using VectorMaskR (or zero if ({t}.u32 & 0xF) == 0)"
    ));
    true
}

/// LVRX family: right-aligned (reverse) vector load
pub(crate) fn handle_lvrx_like(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);
    emit_lvrx_at(ctx, vd, ra, rb)
}

/// LVSL: produce left-shift mask vector based on address low nibble
pub(crate) fn handle_lvsl(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);

    let t   = ctx.temp().to_string();
    let vvd = ctx.v(vd).to_string();
    let rra = ctx.r(ra).to_string();
    let rrb = ctx.r(rb).to_string();

    ctx.print(format!("\t{t}.u32 = "));
    if ra != 0 {
        ctx.print(format!("{rra}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));
    ctx.println_fmt(format_args!(
        "\t// {vvd} = VectorShiftTableL[({t}.u32 & 0xF)]"
    ));
    true
}

/// LVSR: produce right-shift mask vector based on address low nibble
pub(crate) fn handle_lvsr(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);

    let t   = ctx.temp().to_string();
    let vvd = ctx.v(vd).to_string();
    let rra = ctx.r(ra).to_string();
    let rrb = ctx.r(rb).to_string();

    ctx.print(format!("\t{t}.u32 = "));
    if ra != 0 {
        ctx.print(format!("{rra}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));
    ctx.println_fmt(format_args!(
        "\t// {vvd} = VectorShiftTableR[({t}.u32 & 0xF)]"
    ));
    true
}
