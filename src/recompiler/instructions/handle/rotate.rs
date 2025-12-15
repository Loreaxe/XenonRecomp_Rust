use super::*; // brings `compute_mask` into scope

pub(crate) fn handle_rldicl(ctx: &mut LowerCtx) -> bool {
    // RLDICL rA,rS,sh,mb  -> rotl64(rs,sh) & mask(mb..63)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let mb = ctx.op_imm(3) as u32;
    let mask = compute_mask64(mb, 63);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = ({rs}.u64).rotate_left({sh}) & 0x{mask:016X}; }}"
    ));
    true
}

pub(crate) fn handle_rldicr(ctx: &mut LowerCtx) -> bool {
    // RLDICR rA,rS,sh,me  -> rotl64(rs,sh) & mask(0..me)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let me = ctx.op_imm(3) as u32;
    let mask = compute_mask64(0, me);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = ({rs}.u64).rotate_left({sh}) & 0x{mask:016X}; }}"
    ));
    true
}

pub(crate) fn handle_rldimi(ctx: &mut LowerCtx) -> bool {
    // RLDIMI rA,rS,sh,mb -> (rotl64(rs,sh) & mask(mb..(63-sh))) | (rA & ~mask)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = ctx.op_imm(2) as u32;
    let mb = ctx.op_imm(3) as u32;

    // 63 - sh (wrap-safe cast)
    let stop  = 63u32.wrapping_sub(sh);
    let mask  = compute_mask64(mb, stop);
    let nmask = !mask;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = (({rs}.u64).rotate_left({sh}) & 0x{mask:016X}) | ({rd}.u64 & 0x{nmask:016X}); }}"
    ));
    true
}

pub(crate) fn handle_rlwimi(ctx: &mut LowerCtx) -> bool {
    // RLWIMI rA,rS,sh,mb,me  (word rotate and mask insert)
    let d  = ctx.op_reg(0);               // rA
    let s  = ctx.op_reg(1);               // rS
    let sh = (ctx.op_imm(2) as u32) & 31; // SH 0..31
    let mb = (ctx.op_imm(3) as u32) & 31; // MB 0..31
    let me = (ctx.op_imm(4) as u32) & 31; // ME 0..31

    // Proper 32-bit mask for RLWIMI
    let mask = compute_mask32(mb, me);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            let tmp = ({rs}.u32.rotate_left({sh}) & 0x{mask:08X}); \
            {rd}.u32 = ({rd}.u32 & !0x{mask:08X}) | tmp; \
            {rd}.u64 = {rd}.u32 as u64; \
        }}"
    ));

    // Record form RLWIMI.
    if ctx
        .insn
        .mnemonic()
        .map(|m| m.ends_with('.'))
        .unwrap_or(false)
    {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }

    true
}


pub(crate) fn handle_rlwinm(ctx: &mut LowerCtx) -> bool {
    // RLWINM rA,rS,sh,mb,me  (word rotate/mask)
    let d  = ctx.op_reg(0);               // rA
    let s  = ctx.op_reg(1);               // rS
    let sh = (ctx.op_imm(2) as u32) & 31; // SH is 0..31
    let mb = (ctx.op_imm(3) as u32) & 31; // MB 0..31
    let me = (ctx.op_imm(4) as u32) & 31; // ME 0..31

    // Proper 32-bit RLWINM mask
    let mask = compute_mask32(mb, me);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    // Result is 32-bit, then zero-extended to 64-bit
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {rd}.u32 = ({rs}.u32.rotate_left({sh}) & 0x{mask:08X}); \
            {rd}.u64 = {rd}.u32 as u64; \
        }}"
    ));

    // Record form: rlwinm.
    if ctx
        .insn
        .mnemonic()
        .map(|m| m.ends_with('.'))
        .unwrap_or(false)
    {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
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

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = ({rs}.u64).rotate_left({sh}); }}"
    ));
    true
}

pub(crate) fn handle_rotlw(ctx: &mut LowerCtx) -> bool {
    // ROTLW rA,rS,rB  (word rotate, shift from rB)
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();
    let rb = ctx.r(b).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            let sh = ({rb}.u8 & 0x1F) as u32; \
            {rd}.u32 = {rs}.u32.rotate_left(sh); \
            {rd}.u64 = {rd}.u32 as u64; \
        }}"
    ));
    true
}

pub(crate) fn handle_rotlwi(ctx: &mut LowerCtx) -> bool {
    // ROTLWI rA,rS,sh  (word rotate)
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let sh = (ctx.op_imm(2) as u32) & 31;

    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {rd}.u32 = {rs}.u32.rotate_left({sh}); \
            {rd}.u64 = {rd}.u32 as u64; \
        }}"
    ));

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}"
        ));
    }
    true
}

