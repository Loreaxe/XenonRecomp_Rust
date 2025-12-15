// shift.rs
use super::*;

// ---------- 64-bit shifts ----------

pub(crate) fn handle_sld(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    // PPC64 sld: shift = rB[58..63] == rB & 0x3F
    ctx.println("\tunsafe {");
    ctx.println_fmt(format_args!(
        "\t\t{rd}.u64 = ({ra}.u64) << (({rb}.u8 & 0x3F) as u32);"
    ));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_sldi(ctx: &mut LowerCtx) -> bool {
    // sldi ra, rs, sh   (alias of rldicr)
    let ra_idx = ctx.op_reg(0);
    let rs_idx = ctx.op_reg(1);
    let sh     = (ctx.op_imm(2) as u32) & 0x3F; // 0..63

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');

    let ra  = ctx.r(ra_idx).to_string();
    let rs  = ctx.r(rs_idx).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();
    let t   = ctx.temp().to_string();

    // result = rs << sh  (64-bit), then keep low 32 bits in u32 view
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {t}.u64 = {rs}.u64.wrapping_shl({sh}); }}"
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ra}.u64 = {t}.u64; {ra}.u32 = {t}.u64 as u32; }}"
    ));

    if dotted {
        // sldi. updates CR0 based on 64-bit result
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i64({ra}.s64, 0, &mut {xer}); }}"
        ));
    }

    true
}

pub(crate) fn handle_srd(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();

    // If rB[6] set => shift >= 64 => result = 0, else shift by low 6
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            if ({rb}.u8 & 0x40) != 0 {{ \
                {rd}.u64 = 0; \
            }} else {{ \
                {rd}.u64 = ({ra}.u64) >> (({rb}.u8 & 0x3F) as u32); \
            }} \
        }}"
    ));
    true
}

// ---------- 32-bit shifts ----------

pub(crate) fn handle_slw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');

    // If rB[5] set => shift >= 32 => result = 0, else use low 5 bits
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            if ({rb}.u8 & 0x20) != 0 {{ \
                {rd}.u64 = 0; \
            }} else {{ \
                {rd}.u64 = (({ra}.u32) << (({rb}.u8 & 0x1F) as u32)) as u64; \
            }} \
        }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_slwi(ctx: &mut LowerCtx) -> bool {
    // slwi ra, rs, sh
    let ra_idx = ctx.op_reg(0);
    let rs_idx = ctx.op_reg(1);
    let sh     = (ctx.op_imm(2) as u32) & 0x1F; // 0..31

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let ra = ctx.r(ra_idx).to_string();
    let rs = ctx.r(rs_idx).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ra}.u32 = {rs}.u32.wrapping_shl({sh}); }}"
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ra}.u64 = {ra}.u32 as u64; }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({ra}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_srw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');

    // If rB[5] set => shift >= 32 => result = 0, else use low 5 bits
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            if ({rb}.u8 & 0x20) != 0 {{ \
                {rd}.u64 = 0; \
            }} else {{ \
                {rd}.u64 = (({ra}.u32) >> (({rb}.u8 & 0x1F) as u32)) as u64; \
            }} \
        }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_srwi(ctx: &mut LowerCtx) -> bool {
    // srwi ra, rs, sh
    let ra_idx = ctx.op_reg(0);
    let rs_idx = ctx.op_reg(1);
    let sh     = (ctx.op_imm(2) as u32) & 0x1F; // 0..31

    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');
    let ra  = ctx.r(ra_idx).to_string();
    let rs  = ctx.r(rs_idx).to_string();
    let cr0 = ctx.cr(0).to_string();
    let xer = ctx.xer().to_string();

    // Union field accesses must be in unsafe blocks.
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ra}.u32 = {rs}.u32.wrapping_shr({sh}); }}"
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ra}.u64 = {ra}.u32 as u64; }}"
    ));

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({ra}.s32, 0, &mut {xer}); }}",
        ));
    }
    true
}

// ---------- arithmetic right shifts ----------

pub(crate) fn handle_srad(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);
    let t = ctx.temp().to_string();

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();

    // Clamp shift to 63; CA = (sign && any 1s shifted out)
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {t}.u64 = ({rb}.u64 & 0x7F) as u64; \
            if {t}.u64 > 0x3F {{ {t}.u64 = 0x3F; }} \
            let sign = {ra}.s64 < 0; \
            let shifted_out = ({ra}.u64 & ((1u64 << {t}.u64) - 1)) != 0; \
            {xer}.ca = if sign && shifted_out {{ 1 }} else {{ 0 }}; \
            {rd}.s64 = {ra}.s64 >> {t}.u64; \
        }}"
    ));
    true
}

pub(crate) fn handle_sradi(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let a  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();

    if sh != 0 {
        // CA = (sign && any 1s shifted out) -> low `sh` bits
        // mask_low = (1<<sh)-1; sh is 0..63 so this is safe when sh!=0
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                let sign = {ra}.s64 < 0; \
                let shifted_out = ({ra}.u64 & ((1u64 << {sh}) - 1)) != 0; \
                {xer}.ca = if sign && shifted_out {{ 1 }} else {{ 0 }}; \
                {rd}.s64 = {ra}.s64 >> {sh}; \
            }}"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                {xer}.ca = 0; \
                {rd}.s64 = {ra}.s64; \
            }}"
        ));
    }
    true
}

pub(crate) fn handle_sraw(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);
    let t = ctx.temp().to_string();

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let rb  = ctx.r(b).to_string();
    let xer = ctx.xer().to_string();
    let cr0 = ctx.cr(0).to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');

    // Clamp shift to 31; CA = (sign && any 1s shifted out)
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {t}.u32 = {rb}.u32 & 0x3F; \
            if {t}.u32 > 0x1F {{ {t}.u32 = 0x1F; }} \
            let sign = {ra}.s32 < 0; \
            let shifted_out = ({ra}.u32 & ((1u32 << {t}.u32) - 1)) != 0; \
            {xer}.ca = if sign && shifted_out {{ 1 }} else {{ 0 }}; \
            {rd}.s64 = ({ra}.s32 >> {t}.u32) as i64; \
        }}"
    ));
    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

pub(crate) fn handle_srawi(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let a  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;

    let rd  = ctx.r(d).to_string();
    let ra  = ctx.r(a).to_string();
    let xer = ctx.xer().to_string();
    let cr0 = ctx.cr(0).to_string();
    let dotted = ctx.insn.mnemonic().unwrap_or_default().ends_with('.');

    if sh != 0 {
        // CA = 1 iff sign bit was 1 *and* any 1s were shifted out.
        ctx.println_fmt(format_args!(
            "\tunsafe {{ \
                let sign = {ra}.s32 < 0; \
                let shifted_out = ({ra}.u32 & ((1u32 << {sh}) - 1)) != 0; \
                {xer}.ca = if sign && shifted_out {{ 1 }} else {{ 0 }}; \
            }}"
        ));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.s64 = ({ra}.s32 >> {sh}) as i64; }}"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {xer}.ca = 0; }}"
        ));
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.s64 = {ra}.s32 as i64; }}"
        ));
    }

    if dotted {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}
