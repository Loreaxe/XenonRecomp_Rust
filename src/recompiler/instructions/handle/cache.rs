use super::*;

/// Optionally: treat the “do nothing” cache hints as hooks for future modeling.
/// For now they’re nops that still return `true`.
pub(crate) fn handle_db16cyc(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_dcbf(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_dcbt(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_dcbtst(_: &mut LowerCtx) -> bool { true }

/// dcbz  rA,rB  -> zero 32-byte cache line at EA=(rA?rA+rB:rB)
pub(crate) fn handle_dcbz(ctx: &mut LowerCtx) -> bool {
    let a = ctx.op_reg(0);
    let b = ctx.op_reg(1);

    let ea = ctx.ea().to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    if a != 0 {
        ctx.println_fmt(format_args!("\t{ea}.u32 = {ra}.u32.wrapping_add({rb}.u32);"));
    } else {
        ctx.println_fmt(format_args!("\t{ea}.u32 = {rb}.u32;"));
    }

    // Align to 32B and zero via runtime
    ctx.println_fmt(format_args!("\t{ea}.u32 &= !31;"));
    ctx.println_fmt(format_args!("\tunsafe {{ crate::rt::memset_ea({ea}.u32, 0, 32) }};"));
    true
}

/// dcbzl rA,rB  -> zero 128-byte cache line at EA=(rA?rA+rB:rB)
pub(crate) fn handle_dcbzl(ctx: &mut LowerCtx) -> bool {
    let a = ctx.op_reg(0);
    let b = ctx.op_reg(1);

    let ea = ctx.ea().to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    if a != 0 {
        ctx.println_fmt(format_args!("\t{ea}.u32 = {ra}.u32.wrapping_add({rb}.u32);"));
    } else {
        ctx.println_fmt(format_args!("\t{ea}.u32 = {rb}.u32;"));
    }

    // Align to 128B and zero via runtime
    ctx.println_fmt(format_args!("\t{ea}.u32 &= !127;"));
    ctx.println_fmt(format_args!("\tunsafe {{ crate::rt::memset_ea({ea}.u32, 0, 128) }};"));
    true
}
