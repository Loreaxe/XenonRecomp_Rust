// src/recompiler/instructions/float_ops.rs
use super::*;

/// Clear sign bit
pub(crate) fn handle_fabs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.u64 = {fs}.u64 & !0x8000_0000_0000_0000u64;"));
    true
}

pub(crate) fn handle_fadd(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fa}.f64 + {fb}.f64;"));
    true
}

pub(crate) fn handle_fadds(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = (({fa}.f64 + {fb}.f64) as f32) as f64;"));
    true
}

pub(crate) fn handle_fcfid(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ({fs}.s64 as f64);"));
    true
}

pub(crate) fn handle_fcmpu(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);

    // Figure out which CR field fcmpu targets (cr0..cr7).
    // For:    fcmpu cr6, f0, f31
    // this will return 6.
    let crf = ctx.branch_cr_index();

    // fcmpu crf, fa, fb
    let a = ctx.op_reg(1);  // fa
    let b = ctx.op_reg(2);  // fb

    let cr = ctx.cr(crf).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{cr}.compare_f64({fa}.f64, {fb}.f64);"));
    true
}

pub(crate) fn handle_fctid(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{fd}.s64 = if {fs}.f64 > (i64::MAX as f64) {{ i64::MAX }} else {{ {fs}.f64 as i64 }};"
    ));
    true
}

pub(crate) fn handle_fctidz(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{fd}.s64 = if {fs}.f64 > (i64::MAX as f64) {{ i64::MAX }} else {{ {fs}.f64.trunc() as i64 }};"
    ));
    true
}

pub(crate) fn handle_fctiwz(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{fd}.s64 = if {fs}.f64 > (i32::MAX as f64) {{ i32::MAX as i64 }} else {{ {fs}.f64.trunc() as i32 as i64 }};"
    ));
    true
}

pub(crate) fn handle_fdiv(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fa}.f64 / {fb}.f64;"));
    true
}

pub(crate) fn handle_fdivs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = (({fa}.f64 / {fb}.f64) as f32) as f64;"));
    true
}

pub(crate) fn handle_fmadd(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fa}.f64 * {fb}.f64 + {fc}.f64;"));
    true
}

pub(crate) fn handle_fmadds(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ((({fa}.f64 * {fb}.f64 + {fc}.f64) as f32) as f64);"));
    true
}

pub(crate) fn handle_fmr(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fs}.f64;"));
    true
}

pub(crate) fn handle_fmsub(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fa}.f64 * {fb}.f64 - {fc}.f64;"));
    true
}

pub(crate) fn handle_fmsubs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ((({fa}.f64 * {fb}.f64 - {fc}.f64) as f32) as f64);"));
    true
}

pub(crate) fn handle_fmul(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fa}.f64 * {fb}.f64;"));
    true
}

pub(crate) fn handle_fmuls(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ((({fa}.f64 * {fb}.f64) as f32) as f64);"));
    true
}

pub(crate) fn handle_fnabs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.u64 = {fs}.u64 | 0x8000_0000_0000_0000u64;"));
    true
}

pub(crate) fn handle_fneg(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.u64 = {fs}.u64 ^ 0x8000_0000_0000_0000u64;"));
    true
}

pub(crate) fn handle_fnmadds(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = -({fa}.f64 * {fb}.f64 + {fc}.f64);"));
    true
}

pub(crate) fn handle_fnmsub(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = -({fa}.f64 * {fb}.f64 - {fc}.f64);"));
    true
}

pub(crate) fn handle_fnmsubs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = -((({fa}.f64 * {fb}.f64 - {fc}.f64) as f32) as f64);"));
    true
}

pub(crate) fn handle_fres(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = (1.0f32 / ({fs}.f64 as f32)) as f64;"));
    true
}


pub(crate) fn handle_frsqrte(ctx: &mut LowerCtx) -> bool {
    // frsqrte fD, fB  — double-precision reciprocal sqrt estimate
    ctx.set_flush_mode(true);

    let d = ctx.op_reg(0);
    let b = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fb = ctx.f(b).to_string();

    // Simple model: 1.0 / sqrt(x). Good enough for gameplay; not bit-identical.
    ctx.println_fmt(format_args!(
        "\t{fd}.f64 = 1.0f64 / {fb}.f64.sqrt();"
    ));

    // We ignore Rc=1 CR updates for now; can be added if needed.
    true
}

pub(crate) fn handle_frsp(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ({fs}.f64 as f32) as f64;"));
    true
}

pub(crate) fn handle_fsel(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2); let c = ctx.op_reg(3);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();
    let fc = ctx.f(c).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = if {fa}.f64 >= 0.0 {{ {fb}.f64 }} else {{ {fc}.f64 }};"));
    true
}

pub(crate) fn handle_fsqrt(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ({fs}.f64).sqrt();"));
    true
}

pub(crate) fn handle_fsqrts(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let s = ctx.op_reg(1);

    let fd = ctx.f(d).to_string();
    let fs = ctx.f(s).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = (({fs}.f64).sqrt() as f32) as f64;"));
    true
}

pub(crate) fn handle_fsub(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = {fa}.f64 - {fb}.f64;"));
    true
}

pub(crate) fn handle_fsubs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);

    let fd = ctx.f(d).to_string();
    let fa = ctx.f(a).to_string();
    let fb = ctx.f(b).to_string();

    ctx.println_fmt(format_args!("\t{fd}.f64 = ((({fa}.f64 - {fb}.f64) as f32) as f64);"));
    true
}
