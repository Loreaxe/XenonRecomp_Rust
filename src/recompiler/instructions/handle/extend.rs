// src/recompiler/instructions/extend.rs
use super::*;

/// Sign-extend byte to 64 bits: rD = (rS.s8 as i64)
pub(crate) fn handle_extsb(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    // Cache register strings to avoid multiple &mut borrows of ctx in a single statement
    let rd = ctx.r(d);
    let rs = ctx.r(s);
    ctx.println_fmt(format_args!("\t{rd}.s64 = {rs}.s8 as i64;"));

    // Handle Rc (dot) variant: update CR0 using XER
    let is_dot = ctx.insn.mnemonic().map_or(false, |m| m.ends_with('.'));
    if is_dot {
        let cr0 = ctx.cr(0);
        let xer = ctx.xer();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

/// Sign-extend halfword to 64 bits: rD = (rS.s16 as i64)
pub(crate) fn handle_extsh(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d);
    let rs = ctx.r(s);
    ctx.println_fmt(format_args!("\t{rd}.s64 = {rs}.s16 as i64;"));

    let is_dot = ctx.insn.mnemonic().map_or(false, |m| m.ends_with('.'));
    if is_dot {
        let cr0 = ctx.cr(0);
        let xer = ctx.xer();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

/// Sign-extend word to 64 bits: rD = (rS.s32 as i64)
pub(crate) fn handle_extsw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);

    let rd = ctx.r(d);
    let rs = ctx.r(s);
    ctx.println_fmt(format_args!("\t{rd}.s64 = {rs}.s32 as i64;"));

    let is_dot = ctx.insn.mnemonic().map_or(false, |m| m.ends_with('.'));
    if is_dot {
        let cr0 = ctx.cr(0);
        let xer = ctx.xer();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}
