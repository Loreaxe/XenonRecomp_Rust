// src/recompiler/instructions/logical.rs
use super::*;

pub(crate) fn handle_and(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 & {rb}.u64;"));
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_andc(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 & !{rb}.u64;"));
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_andi(ctx: &mut LowerCtx) -> bool {
    // ANDI. (records)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = ctx.op_imm(2) as u64;

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 & {imm};"));
    ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    true
}

pub(crate) fn handle_andis(ctx: &mut LowerCtx) -> bool {
    // ANDIS. (records)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = ((ctx.op_imm(2) as u32) << 16) as u64;

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 & {imm};"));
    ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    true
}

pub(crate) fn handle_nand(ctx: &mut LowerCtx) -> bool {
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = !({ra}.u64 & {rb}.u64);"));
    true
}

pub(crate) fn handle_nor(ctx: &mut LowerCtx) -> bool {
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = !({ra}.u64 | {rb}.u64);"));
    true
}

pub(crate) fn handle_or(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 | {rb}.u64;"));
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_orc(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 | ~{rb}.u64;"));
    true
}

pub(crate) fn handle_ori(ctx: &mut LowerCtx) -> bool {
    // ORI (does not record)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = ctx.op_imm(2) as u64;

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 | {imm};"));
    true
}

pub(crate) fn handle_oris(ctx: &mut LowerCtx) -> bool {
    // ORIS (does not record)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = (ctx.op_imm(2) as u64) << 16;

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 | {imm};"));
    true
}

pub(crate) fn handle_xor_scalar(ctx: &mut LowerCtx) -> bool {
    // XOR (GPR)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 ^ {rb}.u64;"));
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_xori_scalar(ctx: &mut LowerCtx, is_hi: bool) -> bool {
    // XORI / XORIS (GPR)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1);
    let mut imm = ctx.op_imm(2) as u64;
    if is_hi { imm <<= 16; }

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 ^ {imm};"));
    true
}

pub(crate) fn handle_not(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = ~{rs}.u64;"));
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}
