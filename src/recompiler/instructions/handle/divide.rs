use super::*;

/// Signed 64-bit: rD = rA / rB
pub(crate) fn handle_divd(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.s64 = {ra}.s64 / {rb}.s64;"));
    true
}

/// Unsigned 64-bit: rD = rA / rB; optionally set CR0 if dotted
pub(crate) fn handle_divdu(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 / {rb}.u64;"));

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    if dotted {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

/// Signed 32-bit: rD = rA / rB; optionally set CR0 if dotted
pub(crate) fn handle_divw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.s32 = {ra}.s32 / {rb}.s32;"));

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    if dotted {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

/// Unsigned 32-bit: rD = rA / rB; optionally set CR0 if dotted
pub(crate) fn handle_divwu(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u32 = {ra}.u32 / {rb}.u32;"));

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    if dotted {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}
