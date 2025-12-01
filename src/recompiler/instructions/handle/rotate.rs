use super::*; // brings `compute_mask` into scope

pub(crate) fn handle_rldicl(ctx: &mut LowerCtx) -> bool {
    // RLDICL rA,rS,sh,mb  -> rotl64(rs,sh) & mask(mb..63)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let mb = ctx.op_imm(3) as u32;
    let mask = compute_mask(mb, 63);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = ({rs}.u64).rotate_left({sh}) & 0x{mask:016X};"));
    true
}

pub(crate) fn handle_rldicr(ctx: &mut LowerCtx) -> bool {
    // RLDICR rA,rS,sh,me  -> rotl64(rs,sh) & mask(0..me)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let me = ctx.op_imm(3) as u32;
    let mask = compute_mask(0, me);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = ({rs}.u64).rotate_left({sh}) & 0x{mask:016X};"));
    true
}

pub(crate) fn handle_rldimi(ctx: &mut LowerCtx) -> bool {
    // RLDIMI rA,rS,sh,mb -> (rotl64(rs,sh) & mask(mb..(63-sh))) | (rA & ~mask)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let mb = ctx.op_imm(3) as u32;

    // 63 - sh (wrap-safe cast)
    let stop = 63u32.wrapping_sub(sh);
    let mask  = compute_mask(mb, stop);
    let nmask = !mask;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = (({rs}.u64).rotate_left({sh}) & 0x{mask:016X}) | ({rd}.u64 & 0x{nmask:016X});"
    ));
    true
}

pub(crate) fn handle_rlwimi(ctx: &mut LowerCtx) -> bool {
    // RLWIMI rA,rS,sh,mb,me (word) — model with 64-bit mask using +32 trick
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let mb = (ctx.op_imm(3) as u32) + 32;
    let me = (ctx.op_imm(4) as u32) + 32;

    let mask  = compute_mask(mb, me);
    let nmask = !mask;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = ((({rs}.u32).rotate_left({sh}) as u64) & 0x{mask:016X}) | ({rd}.u64 & 0x{nmask:016X});"
    ));
    true
}

pub(crate) fn handle_rlwinm(ctx: &mut LowerCtx) -> bool {
    // RLWINM rA,rS,sh,mb,me  (word rotate/mask)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;

    // Use +32 trick so the 32-bit field lands in the *lower* 32 bits.
    let mb = (ctx.op_imm(3) as u32) + 32;
    let me = (ctx.op_imm(4) as u32) + 32;

    let mask = compute_mask(mb, me);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    // Result is 32-bit, zero-extended
    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = ((({rs}.u32).rotate_left({sh})) as u64) & 0x{mask:016X};"
    ));

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!(
            "\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"
        ));
    }

    true
}

pub(crate) fn handle_rotldi(ctx: &mut LowerCtx) -> bool {
    // ROTLDI rA,rS,sh
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = ({rs}.u64).rotate_left({sh});"));
    true
}

pub(crate) fn handle_rotlw(ctx: &mut LowerCtx) -> bool {
    // ROTLW rA,rS,rB  (low 5 bits of rB)
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = (({rs}.u32).rotate_left(({rb}.u8 & 0x1F) as u32)) as u64;"
    ));
    true
}

pub(crate) fn handle_rotlwi(ctx: &mut LowerCtx) -> bool {
    // ROTLWI rA,rS,sh  (word rotate)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = (({rs}.u32).rotate_left({sh})) as u64;"));

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}
