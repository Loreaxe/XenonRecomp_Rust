// src/recompiler/instructions/moves_spr.rs
use super::*;

// ===== Loads from SPRs =====

pub(crate) fn handle_mfcr(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let rd = ctx.r(d).to_string();

    ctx.println("\t// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR");
    // Start from zero before OR-ing in bits.
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = 0; }}"
    ));

    let fields = ["lt", "gt", "eq", "so"];

    for i in 0..32usize {
        let cr    = ctx.cr(i / 4).to_string();
        let field = fields[i % 4];
        let mask  = 1u32 << (31 - i);

        ctx.println_fmt(format_args!(
            "\tunsafe {{ if {cr}.{field} != 0 {{ {rd}.u64 |= 0x{mask:08X}u64; }} }}",
        ));
    }

    true
}

pub(crate) fn handle_mffs(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let fd = ctx.f(d).to_string();
    // fpscr.load_from_host() -> u32, store as raw bits in f-reg
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {fd}.u64 = ctx.fpscr.load_from_host() as u64; }}"
    ));
    true
}

pub(crate) fn handle_mflr(ctx: &mut LowerCtx) -> bool {
    if !ctx.rec.config.skip_lr {
        let d  = ctx.op_reg(0);
        let rd = ctx.r(d).to_string();
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.u64 = ctx.lr; }}"
        ));
    }
    true
}

pub(crate) fn handle_mfmsr(ctx: &mut LowerCtx) -> bool {
    // Only emit if the build was NOT compiled with `--features skip_msr`
    #[cfg(not(feature = "skip_msr"))]
    {
        let d  = ctx.op_reg(0);
        let rd = ctx.r(d).to_string();
        // ctx.msr is u32; move into GPR as 64-bit value
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {rd}.u64 = ctx.msr as u64; }}"
        ));
    }
    true
}

// MFocrf loads one CR field per mask normally; this skeleton copies CR6 like your source.
pub(crate) fn handle_mfocrf(ctx: &mut LowerCtx) -> bool {
    let d   = ctx.op_reg(0);
    let rd  = ctx.r(d).to_string();
    let cr6 = ctx.cr(6).to_string();
    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {rd}.u64 = \
                (({cr6}.lt as u64) << 7) | \
                (({cr6}.gt as u64) << 6) | \
                (({cr6}.eq as u64) << 5) | \
                (({cr6}.so as u64) << 4); \
        }}"
    ));
    true
}

pub(crate) fn handle_mftb(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let rd = ctx.r(d).to_string();
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = crate::rt::rdtsc_u64(); }}"
    ));
    true
}

// ===== Moves / Stores to SPRs =====

pub(crate) fn handle_mr(ctx: &mut LowerCtx) -> bool {
    let d  = ctx.op_reg(0);
    let s  = ctx.op_reg(1);
    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    // Union field copy must be in unsafe
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {rd}.u64 = {rs}.u64; }}"
    ));

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();

        // compare_i32 reads {rd}.s32 and mutates XER: wrap that too
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr0}.compare_i32({rd}.s32, 0, &mut {xer}); }}",
        ));
    }

    true
}

pub(crate) fn handle_mtcr(ctx: &mut LowerCtx) -> bool {
    let s  = ctx.op_reg(0);
    let rs = ctx.r(s).to_string();

    ctx.println("\t// MTCR: move GPR bits into CR fields");
    for i in 0..32usize {
        let fields = ["lt", "gt", "eq", "so"];
        let cr     = ctx.cr(i / 4).to_string();
        let mask   = 1u32 << (31 - i);

        ctx.println_fmt(format_args!(
            "\tunsafe {{ {cr}.{} = (({rs}.u32 & 0x{:X}) != 0) as u8; }}",
            fields[i % 4],
            mask
        ));
    }
    true
}

pub(crate) fn handle_mtcrf(ctx: &mut LowerCtx) -> bool {
    let fxm = ctx.op_imm(0) as u32 & 0xFF;
    let rs  = ctx.op_reg(1);
    let rsn = ctx.r(rs).to_string();

    ctx.println_fmt(format_args!(
        "\t// mtcrf 0x{fxm:02X}, {rsn}: CR update elided (TODO: implement MTCRF semantics)"
    ));
    true
}

pub(crate) fn handle_mtctr(ctx: &mut LowerCtx) -> bool {
    let s   = ctx.op_reg(0);
    let ctr = ctx.ctr().to_string();
    let rs  = ctx.r(s).to_string();
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ctr}.u64 = {rs}.u64; }}"
    ));
    true
}

pub(crate) fn handle_mtfsf(ctx: &mut LowerCtx) -> bool {
    // Note: PPC MTFSF uses a field mask; this assumes your runtime handles masking.
    let b  = ctx.op_reg(1);
    let fb = ctx.f(b).to_string();
    ctx.println_fmt(format_args!(
        "\tunsafe {{ ctx.fpscr.store_from_guest({fb}.u32); }}"
    ));
    true
}

pub(crate) fn handle_mtlr(ctx: &mut LowerCtx) -> bool {
    if !ctx.rec.config.skip_lr {
        let s  = ctx.op_reg(0);
        let rs = ctx.r(s).to_string();
        // mtlr rS  => LR = GPR[S]
        ctx.println_fmt(format_args!(
            "\tunsafe {{ ctx.lr = {rs}.u64; }}"
        ));
    }
    true
}

pub(crate) fn handle_mtmsrd(ctx: &mut LowerCtx) -> bool {
    // Only emit if the build was NOT compiled with `--features skip_msr`
    #[cfg(not(feature = "skip_msr"))]
    {
        let s  = ctx.op_reg(0);
        let rs = ctx.r(s).to_string();
        // Preserve all but EE/ME bits per your mask (0x8020)
        ctx.println_fmt(format_args!(
            "\tunsafe {{ ctx.msr = ({rs}.u32 & 0x8020) | (ctx.msr & !0x8020); }}"
        ));
    }
    true
}

pub(crate) fn handle_mtxer(ctx: &mut LowerCtx) -> bool {
    let s   = ctx.op_reg(0);
    let rs  = ctx.r(s).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ \
            {xer}.so = if ({rs}.u64 & 0x8000_0000) != 0 {{ 1 }} else {{ 0 }}; \
            {xer}.ov = if ({rs}.u64 & 0x4000_0000) != 0 {{ 1 }} else {{ 0 }}; \
            {xer}.ca = if ({rs}.u64 & 0x2000_0000) != 0 {{ 1 }} else {{ 0 }}; \
        }}"
    ));
    true
}
