// src/recompiler/instructions/handle/compare.rs
use super::*;

/// Condition register trap/hint (treated as no-ops for now).
pub(crate) fn handle_cctpl(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_cctpm(_: &mut LowerCtx) -> bool { true }

// ----- 64-bit signed/unsigned -----

pub(crate) fn handle_cmpd(ctx: &mut LowerCtx) -> bool {
    // CR field comes from a CRx register operand (not an immediate)
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let b   = ctx.op_reg(2);

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_i64({ra}.s64, {rb}.s64, &mut {xer});"));
    true
}

pub(crate) fn handle_cmpdi(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as i64;

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_i64({ra}.s64, {imm}, &mut {xer});"));
    true
}

pub(crate) fn handle_cmpld(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let b   = ctx.op_reg(2);

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_u64({ra}.u64, {rb}.u64, &mut {xer});"));
    true
}

pub(crate) fn handle_cmpldi(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as u64;

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_u64({ra}.u64, {imm}, &mut {xer});"));
    true
}

// ----- 32-bit signed/unsigned -----

pub(crate) fn handle_cmplw(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let b   = ctx.op_reg(2);

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_u32({ra}.u32, {rb}.u32, &mut {xer});"));
    true
}

pub(crate) fn handle_cmplwi(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as u64;

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_u32({ra}.u32, {imm} as u32, &mut {xer});"));
    true
}

pub(crate) fn handle_cmpw(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let b   = ctx.op_reg(2);

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_i32({ra}.s32, {rb}.s32, &mut {xer});"));
    true
}

pub(crate) fn handle_cmpwi(ctx: &mut LowerCtx) -> bool {
    let crf = ctx.branch_cr_index();
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as i32;

    let cr  = ctx.cr(crf).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_i32({ra}.s32, {imm}, &mut {xer});"));
    true
}
