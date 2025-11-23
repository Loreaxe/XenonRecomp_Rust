use super::*;

// ======================
// Additions
// ======================

pub(crate) fn handle_add(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 + {rb}.u64;"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_adde(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let t  = ctx.temp().to_string();
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\t{t}.u8 = (({ra}.u32.wrapping_add({rb}.u32) < {ra}.u32) as u8) | (({ra}.u32.wrapping_add({rb}.u32).wrapping_add({xer}.ca as u32) < {xer}.ca as u32) as u8);"
    ));
    ctx.println_fmt(format_args!("\t{rd}.u64 = {ra}.u64 + {rb}.u64 + {xer}.ca as u64;"));
    ctx.println_fmt(format_args!("\t{xer}.ca = ({t}.u8 != 0);"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_addi(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = ctx.op_imm(2);
    let rd = ctx.r(d).to_string();
    if a != 0 {
        let ra = ctx.r(a).to_string();
        ctx.println_fmt(format_args!("\t{rd}.s64 = {ra}.s64 + {imm};"));
    } else {
        ctx.println_fmt(format_args!("\t{rd}.s64 = {imm};"));
    }
    true
}

pub(crate) fn handle_addic(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = ctx.op_imm(2) as i32;
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!("\t{xer}.ca = ({ra}.u32 > (!({imm} as u32)));"));
    ctx.println_fmt(format_args!("\t{rd}.s64 = {ra}.s64 + {imm};"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_addis(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = (ctx.op_imm(2) as i32) << 16;
    let rd = ctx.r(d).to_string();
    if a != 0 {
        let ra = ctx.r(a).to_string();
        ctx.println_fmt(format_args!("\t{rd}.s64 = {ra}.s64 + {imm};"));
    } else {
        ctx.println_fmt(format_args!("\t{rd}.s64 = {imm};"));
    }
    true
}

pub(crate) fn handle_addze(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1);
    let t  = ctx.temp().to_string();
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!("\t{t}.s64 = {ra}.s64 + {xer}.ca as i64;"));
    ctx.println_fmt(format_args!("\t{xer}.ca = ({t}.u32 < {ra}.u32);"));
    ctx.println_fmt(format_args!("\t{rd}.s64 = {t}.s64;"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

// ======================
// Subtractions
// ======================

pub(crate) fn handle_subf(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!("\t{rd}.s64 = {rb}.s64 - {ra}.s64;"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_subfc(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!("\t{xer}.ca = {rb}.u32 >= {ra}.u32;"));
    ctx.println_fmt(format_args!("\t{rd}.s64 = {rb}.s64 - {ra}.s64;"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_subfe(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let t   = ctx.temp().to_string();
    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0    = if dotted { ctx.cr(0).to_string() } else { String::new() };

    // x = ~RA (low 32 bits), y = RB
    ctx.println_fmt(format_args!("\tlet x = (!{ra}.u32);"));
    ctx.println_fmt(format_args!("\tlet y = {rb}.u32;"));

    // First 32-bit add: s = x + y
    ctx.println("\tlet s = x.wrapping_add(y);");

    // Second add: res = s + CA
    ctx.println_fmt(format_args!("\tlet res = s.wrapping_add({xer}.ca as u32);"));

    // Carry out of x + y + CA
    ctx.println_fmt(format_args!(
        "\t{t}.u8 = (s < x) as u8 | (res < s) as u8;"
    ));

    // Result is 32-bit; mirror to 64-bit view
    ctx.println_fmt(format_args!("\t{rd}.u32 = res;"));
    ctx.println_fmt(format_args!("\t{rd}.u64 = {rd}.u32 as u64;"));

    // Update XER.CA
    ctx.println_fmt(format_args!("\t{xer}.ca = ({t}.u8 != 0);"));

    // Dotted form updates CR0 from the 32-bit signed result
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }

    true
}

pub(crate) fn handle_subfic(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let imm = ctx.op_imm(2) as i32;
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{xer}.ca = {ra}.u32 <= {imm} as u32;"));
    ctx.println_fmt(format_args!("\t{rd}.s64 = ({imm} as i64) - {ra}.s64;"));
    true
}

// ======================
// Multiply / Neg / NOP
// ======================

pub(crate) fn handle_mulhd(ctx: &mut LowerCtx) -> bool {
    // mulhd rD, rA, rB
    // rD = high 64 bits of (rA.s64 * rB.s64)
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.s64 = ((({ra}.s64 as i128) * ({rb}.s64 as i128)) >> 64) as i64;"
    ));

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    if dotted {
        // Follow same convention as other 64-bit ALU ops: CR0 compares low 32 bits.
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!(
            "\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"
        ));
    }

    true
}

pub(crate) fn handle_mulhw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let xer    = ctx.xer().to_string();
    let cr0    = if dotted { ctx.cr(0).to_string() } else { String::new() };

    // High 32 bits of signed 32×32 product, sign-extended to 64.
    ctx.println_fmt(format_args!(
        "\t{rd}.s64 = (({ra}.s32 as i64 * {rb}.s32 as i64) >> 32);"
    ));

    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }

    true
}

pub(crate) fn handle_mulhwu(ctx: &mut LowerCtx) -> bool {
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!("\t{rd}.u64 = (({ra}.u32 as u64 * {rb}.u32 as u64) >> 32);"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_mulld(ctx: &mut LowerCtx) -> bool {
    let d=ctx.op_reg(0); let a=ctx.op_reg(1); let b=ctx.op_reg(2);
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    ctx.println_fmt(format_args!("\t{rd}.s64 = {ra}.s64 * {rb}.s64;"));
    true
}

pub(crate) fn handle_mulli(ctx: &mut LowerCtx) -> bool {
    // mulli rD, rA, SI
    // rD = low 32 bits of (rA.s32 * SI.s16), sign-extended to 64.
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as i32; // SI already sign-extended from 16 bits

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();

    ctx.println_fmt(format_args!(
        "\t{rd}.s32 = (({ra}.s32 as i64 * {imm} as i64) as i32);"
    ));
    ctx.println_fmt(format_args!("\t{rd}.s64 = {rd}.s32 as i64;"));

    true
}


pub(crate) fn handle_mullw(ctx: &mut LowerCtx) -> bool {
    // mullw rD, rA, rB
    // rD = low 32 bits of (rA.s32 * rB.s32), sign-extended to 64.
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    let xer    = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0    = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\t{rd}.s32 = (({ra}.s32 as i64 * {rb}.s32 as i64) as i32);"
    ));
    ctx.println_fmt(format_args!("\t{rd}.s64 = {rd}.s32 as i64;"));

    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }

    true
}

pub(crate) fn handle_neg(ctx: &mut LowerCtx) -> bool {
    let d=ctx.op_reg(0); let s=ctx.op_reg(1);
    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0 = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!("\t{rd}.s64 = -{rs}.s64;"));
    if dotted {
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_nop(_: &mut LowerCtx) -> bool { true }
