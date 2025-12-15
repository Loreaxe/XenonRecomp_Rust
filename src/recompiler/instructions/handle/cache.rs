// src/recompiler/instructions/handel/cache.rs
use super::*;

/// Optionally: treat the “do nothing” cache hints as hooks for future modeling.
/// For now they’re nops that still return `true`.
pub(crate) fn handle_db16cyc(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_dcbf(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_dcbt(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_dcbtst(_: &mut LowerCtx) -> bool { true }

pub(crate) fn handle_dcbst(ctx: &mut LowerCtx) -> bool {
    let ra = ctx.op_reg(0);
    let rb = ctx.op_reg(1);

    let ra_s = ctx.r(ra).to_string();
    let rb_s = ctx.r(rb).to_string();

    ctx.println_fmt(format_args!(
        "\t// dcbst {ra_s}, {rb_s}: data cache block store (no-op in recompiler)"
    ));

    true
}

/// dcbz  rA,rB  -> zero 32-byte cache line at EA=(rA?rA+rB:rB)
pub(crate) fn handle_dcbz(ctx: &mut LowerCtx) -> bool {
    let a = ctx.op_reg(0);
    let b = ctx.op_reg(1);

    let ea = ctx.ea().to_string(); // local `let mut ea: u32`
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    if a != 0 {
        // EA = rA + rB (union fields => unsafe)
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {ea} = {ra}.u32.wrapping_add({rb}.u32); }}",
            ea = ea,
            ra = ra,
            rb = rb,
        ));
    } else {
        // EA = rB
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {ea} = {rb}.u32; }}",
            ea = ea,
            rb = rb,
        ));
    }

    // Align EA down to 32-byte boundary (plain u32 local, no union)
    ctx.println_fmt(format_args!("\t{ea} &= !31;", ea = ea));

    // Zero 32 bytes at base + EA
    ctx.println_fmt(format_args!(
        "\tunsafe {{ crate::rt::memset_ea(base, {ea}, 0, 32) }};",
        ea = ea,
    ));

    true
}

/// dcbzl rA,rB  -> zero 128-byte cache line at EA=(rA ? rA+rB : rB)
pub(crate) fn handle_dcbzl(ctx: &mut LowerCtx) -> bool {
    let a = ctx.op_reg(0);
    let b = ctx.op_reg(1);

    let ea = ctx.ea().to_string(); // `let mut ea: u32`
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    if a != 0 {
        // EA = rA + rB (union fields => unsafe)
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {ea} = {ra}.u32.wrapping_add({rb}.u32); }}",
            ea = ea,
            ra = ra,
            rb = rb,
        ));
    } else {
        // EA = rB
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {ea} = {rb}.u32; }}",
            ea = ea,
            rb = rb,
        ));
    }

    // Align EA down to 128-byte boundary (plain u32)
    ctx.println_fmt(format_args!("\t{ea} &= !127;", ea = ea));

    // Zero 128 bytes at base + EA
    ctx.println_fmt(format_args!(
        "\tunsafe {{ crate::rt::memset_ea(base, {ea}, 0, 128) }};",
        ea = ea,
    ));

    true
}
