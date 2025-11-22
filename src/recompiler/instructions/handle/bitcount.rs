use super::*;

/// CNTLZD: count leading zeros of 64-bit operand (returns 0..=64 in a 64-bit GPR)
pub(crate) fn handle_cntlzd(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = if {rs}.u64 == 0 {{ 64 }} else {{ {rs}.u64.leading_zeros() as u64 }};"
    ));
    true
}

/// CNTLZW: count leading zeros of 32-bit operand (returns 0..=32 in a 64-bit GPR)
pub(crate) fn handle_cntlzw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = if {rs}.u32 == 0 {{ 32 }} else {{ {rs}.u32.leading_zeros() as u64 }};"
    ));
    true
}
