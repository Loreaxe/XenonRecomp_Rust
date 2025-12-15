// system.rs
use super::*;

pub(crate) fn handle_eieio(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_sync(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_tdlgei(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_tdllei(_: &mut LowerCtx) -> bool { true }
/// TDI — trap doubleword immediate
pub(crate) fn handle_tdi(ctx: &mut LowerCtx) -> bool {
    ctx.println("\t// tdi: trap doubleword immediate — TODO: implement trap semantics");
    true
}
pub(crate) fn handle_twi(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_twui(ctx: &mut LowerCtx) -> bool {
    ctx.println("\t// twui: trap word unsigned immediate — TODO: implement trap semantics");
    true
}
pub(crate) fn handle_twlgei(_: &mut LowerCtx) -> bool { true }
pub(crate) fn handle_twllei(_: &mut LowerCtx) -> bool { true }

pub(crate) fn handle_clrldi(ctx: &mut LowerCtx) -> bool {
    // rD = rS & (u64::MAX >> n)   (clear leftmost n bits)
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);
    let n = ctx.op_imm(2) as u32;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    let mask: u64 = if n >= 64 { 0 } else { u64::MAX >> n };

    // union field access must be unsafe
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = {rs}.u64 & 0x{mask:016X}u64; }}"
    ));
    true
}

pub(crate) fn handle_clrlwi(ctx: &mut LowerCtx) -> bool {
    // rD = (rS.u32 as u64) & (u32::MAX >> n)
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);
    let n = ctx.op_imm(2) as u32;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    let mask: u32 = if n >= 32 { 0 } else { u32::MAX >> n };

    // read {rs}.u32 and write {rd}.u64 under unsafe
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = ({rs}.u32 as u64) & 0x{mask:08X}u64; }}"
    ));

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        // compare_i32 reads {rd}.s32 (union) and mutates XER, so also unsafe
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}
