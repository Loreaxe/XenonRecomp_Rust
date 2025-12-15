use super::*;

// ======================
// Additions
// ======================

pub(crate) fn handle_add(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = {ra}.u64.wrapping_add({rb}.u64); }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_adde(ctx: &mut LowerCtx) -> bool {
    // rD = rA + rB + CA (low 32 bits), CA from 32-bit unsigned add
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd   = ctx.r(d).to_string();
    let ra   = ctx.r(a).to_string();
    let rb   = ctx.r(b).to_string();
    let xer  = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            let (sum1, c1) = {ra}.u32.overflowing_add({rb}.u32); \
            let (sum2, c2) = sum1.overflowing_add({xer}.ca as u32); \
            {rd}.u32 = sum2; \
            {rd}.u64 = {rd}.u32 as u64; \
            {xer}.ca = if c1 || c2 {{ 1 }} else {{ 0 }}; \
        }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_addi(ctx: &mut LowerCtx) -> bool {
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2);

    let rd = ctx.r(d).to_string();

    if a != 0 {
        let ra = ctx.r(a).to_string();
        // unsafe because of union field .s64
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.s64 = {ra}.s64 + {imm}; }}",
        ));
    } else {
        // RA == 0 → rD <- SIMM
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.s64 = {imm}; }}",
        ));
    }
    true
}

pub(crate) fn handle_addic(ctx: &mut LowerCtx) -> bool {
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as i32;

    let rd   = ctx.r(d).to_string();
    let ra   = ctx.r(a).to_string();
    let xer  = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    // reinterpret SIMM as u32 for the unsigned add that drives CA
    let imm_u = imm as u32;

    if a != 0 {
        // rD <- rA + SIMM; CA = carry-out of (rA.u32 + SIMM as u32)
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                let (res, carry) = {ra}.u32.overflowing_add({imm_u}); \
                {rd}.s64 = (res as i32) as i64; \
                {xer}.ca = if carry {{ 1 }} else {{ 0 }}; \
            }}"
        ));
    } else {
        // rA == 0 → use 0 as the input (PPC quirk)
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                let (res, carry) = (0u32).overflowing_add({imm_u}); \
                {rd}.s64 = (res as i32) as i64; \
                {xer}.ca = if carry {{ 1 }} else {{ 0 }}; \
            }}"
        ));
    }

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }

    true
}

pub(crate) fn handle_addis(ctx: &mut LowerCtx) -> bool {
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let imm = (ctx.op_imm(2) as i32) << 16;

    let rd = ctx.r(d).to_string();

    if a != 0 {
        let ra = ctx.r(a).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.s64 = {ra}.s64 + {imm}; }}"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.s64 = {imm}; }}"
        ));
    }
    true
}


pub(crate) fn handle_addze(ctx: &mut LowerCtx) -> bool {
    // rD = rA + CA (32-bit), CA from carry-out
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);

    let t   = ctx.temp().to_string();
    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {t}.s64 = {ra}.s64 + {xer}.ca as i64; \
            {xer}.ca = if {t}.u32 < {ra}.u32 {{ 1 }} else {{ 0 }}; \
            {rd}.s64 = {t}.s64; \
        }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

// ======================
// Subtractions
// ======================

pub(crate) fn handle_subf(ctx: &mut LowerCtx) -> bool {
    // subf rD, rA, rB  => rD = rB - rA
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.s64 = {rb}.s64 - {ra}.s64; }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_subfc(ctx: &mut LowerCtx) -> bool {
    // subfc rD, rA, rB => rD = rB - rA, CA = (rB >= rA) (unsigned 32-bit)
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {xer}.ca = if {rb}.u32 >= {ra}.u32 {{ 1 }} else {{ 0 }}; \
            {rd}.s64 = {rb}.s64 - {ra}.s64; \
        }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_subfe(ctx: &mut LowerCtx) -> bool {
    // subfe rD, rA, rB => rD = ~rA + rB + CA; CA from unsigned add
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

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            let x = !{ra}.u32; \
            let y = {rb}.u32; \
            let s = x.wrapping_add(y); \
            let res = s.wrapping_add({xer}.ca as u32); \
            {t}.u8 = (s < x) as u8 | (res < s) as u8; \
            {rd}.u32 = res; \
            {rd}.u64 = {rd}.u32 as u64; \
            {xer}.ca = if {t}.u8 != 0 {{ 1 }} else {{ 0 }}; \
        }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }

    true
}


pub(crate) fn handle_subfic(ctx: &mut LowerCtx) -> bool {
    // subfic rD, rA, SI  =>  rD = SI - rA;  CA = 1 iff (u32(SI) >= rA.u32)
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as i32;

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    // We avoid `<= IMM as u32` to keep Rust happy with negative immediates.
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            let si: i32  = {imm}; \
            let si_u: u32 = si as u32; \
            let ra_u: u32 = {ra}.u32; \
            {xer}.ca = if si_u >= ra_u {{ 1 }} else {{ 0 }}; \
            {rd}.s64 = (si as i64) - {ra}.s64; \
        }}"
    ));

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
        "\tunsafe {{ {rd}.s64 = ((({ra}.s64 as i128) * ({rb}.s64 as i128)) >> 64) as i64; }}"
    ));

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    if dotted {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
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
        "\tunsafe {{ {rd}.s64 = (({ra}.s32 as i64 * {rb}.s32 as i64) >> 32); }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }

    true
}

pub(crate) fn handle_mulhwu(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = (({ra}.u32 as u64 * {rb}.u32 as u64) >> 32); }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_mulld(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.s64 = {ra}.s64 * {rb}.s64; }}"
    ));
    true
}

pub(crate) fn handle_mulli(ctx: &mut LowerCtx) -> bool {
    // mulli rD, rA, SI
    // rD = low 32 bits of (rA.s32 * SI.s16), sign-extended to 64.
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let imm = ctx.op_imm(2) as i32; // SI already sign-extended

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {rd}.s32 = (({ra}.s32 as i64 * {imm} as i64) as i32); \
            {rd}.s64 = {rd}.s32 as i64; \
        }}"
    ));

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
        "\tunsafe {{ \
            {rd}.s32 = (({ra}.s32 as i64 * {rb}.s32 as i64) as i32); \
            {rd}.s64 = {rd}.s32 as i64; \
        }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }

    true
}

pub(crate) fn handle_neg(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);

    let rd  = ctx.r(d).to_string();
    let rs  = ctx.r(s).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let cr0  = if dotted { ctx.cr(0).to_string() } else { String::new() };

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.s64 = -{rs}.s64; }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_nop(_: &mut LowerCtx) -> bool { true }
