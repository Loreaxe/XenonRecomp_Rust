// src/recompiler/instructions/extend.rs
use super::*;

/// Sign-extend byte to 64 bits: rD = (rS.s8 as i64)
pub(crate) fn handle_extsb(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d);
    let rs = ctx.r(s);
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.s64 = {rs}.s8 as i64; }}",
        rd = rd,
        rs = rs,
    ));

    let is_dot = ctx.insn.mnemonic().map_or(false, |m| m.ends_with('.'));
    if is_dot {
        let cr0 = ctx.cr(0);
        let xer = ctx.xer();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}",
            cr0 = cr0,
            rd  = rd,
            xer = xer,
        ));
    }
    true
}


/// Sign-extend halfword to 64 bits: rD = (rS.s16 as i64)
pub(crate) fn handle_extsh(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d);
    let rs = ctx.r(s);
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.s64 = {rs}.s16 as i64; }}",
        rd = rd,
        rs = rs,
    ));

    let is_dot = ctx.insn.mnemonic().map_or(false, |m| m.ends_with('.'));
    if is_dot {
        let cr0 = ctx.cr(0);
        let xer = ctx.xer();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}",
            cr0 = cr0,
            rd  = rd,
            xer = xer,
        ));
    }
    true
}

/// Sign-extend word to 64 bits: rD = (rS.s32 as i64)
pub(crate) fn handle_extsw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d);
    let rs = ctx.r(s);
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.s64 = {rs}.s32 as i64; }}",
        rd = rd,
        rs = rs,
    ));

    let is_dot = ctx.insn.mnemonic().map_or(false, |m| m.ends_with('.'));
    if is_dot {
        let cr0 = ctx.cr(0);
        let xer = ctx.xer();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}",
            cr0 = cr0,
            rd  = rd,
            xer = xer,
        ));
    }
    true
}
