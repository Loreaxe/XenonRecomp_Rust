// src/recompiler/instructions/moves_spr.rs
use super::*;

// ===== Loads from SPRs =====

pub(crate) fn handle_mfcr(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let rd = ctx.r(d).to_string();

    ctx.println("\t// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR");
    for i in 0..32usize {
        let fields = ["lt", "gt", "eq", "so"];
        let cr = ctx.cr(i / 4).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 {}= if {cr}.{} {{ 0x{:X} }} else {{ 0 }};",
            if i == 0 { "" } else { "|" },
            fields[i % 4],
            1u32 << (31 - i)
        ));
    }
    true
}

pub(crate) fn handle_mffs(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let fd = ctx.f(d).to_string();
    ctx.println_fmt(format_args!("\t{fd}.u64 = ctx.fpscr.load_from_host();"));
    true
}

pub(crate) fn handle_mflr(ctx: &mut LowerCtx) -> bool {
    if !ctx.rec.config.skip_lr {
        let d = ctx.op_reg(0);
        let rd = ctx.r(d).to_string();
        ctx.println_fmt(format_args!("\t{rd}.u64 = ctx.lr;"));
    }
    true
}

pub(crate) fn handle_mfmsr(ctx: &mut LowerCtx) -> bool {
    // Only emit if the build was NOT compiled with `--features skip_msr`
    #[cfg(not(feature = "skip_msr"))]
    {
        let d = ctx.op_reg(0);
        let rd = ctx.r(d).to_string();
        ctx.println_fmt(format_args!("\t{rd}.u64 = ctx.msr;"));
    }
    true
}

// MFocrf loads one CR field per mask normally; this skeleton copies CR6 like your source.
pub(crate) fn handle_mfocrf(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let rd = ctx.r(d).to_string();
    let cr6 = ctx.cr(6).to_string();
    ctx.println_fmt(format_args!(
        "\t{rd}.u64 = (({cr6}.lt as u64) << 7) | (({cr6}.gt as u64) << 6) | (({cr6}.eq as u64) << 5) | (({cr6}.so as u64) << 4);"
    ));
    true
}

pub(crate) fn handle_mftb(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let rd = ctx.r(d).to_string();
    ctx.println_fmt(format_args!("\t{rd}.u64 = crate::rt::rdtsc_u64();"));
    true
}

// ===== Moves / Stores to SPRs =====

pub(crate) fn handle_mr(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let s = ctx.op_reg(1);
    let rd = ctx.r(d).to_string();
    let rs = ctx.r(s).to_string();

    ctx.println_fmt(format_args!("\t{rd}.u64 = {rs}.u64;"));
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr0 = ctx.cr(0).to_string();
        let xer = ctx.xer().to_string();
        ctx.println_fmt(format_args!("\t{cr0}.compare_i32({rd}.s32, 0, &mut {xer});"));
    }
    true
}

pub(crate) fn handle_mtcr(ctx: &mut LowerCtx) -> bool {
    let s = ctx.op_reg(0);
    let rs = ctx.r(s).to_string();

    ctx.println("\t// MTCR: move GPR bits into CR fields");
    for i in 0..32usize {
        let fields = ["lt", "gt", "eq", "so"];
        let cr = ctx.cr(i / 4).to_string();
        ctx.println_fmt(format_args!(
            "\t{cr}.{} = ({rs}.u32 & 0x{:X}) != 0;",
            fields[i % 4],
            1u32 << (31 - i)
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
    let s = ctx.op_reg(0);
    let ctr = ctx.ctr().to_string();
    let rs  = ctx.r(s).to_string();
    ctx.println_fmt(format_args!("\t{ctr}.u64 = {rs}.u64;"));
    true
}

pub(crate) fn handle_mtfsf(ctx: &mut LowerCtx) -> bool {
    // Note: PPC MTFSF uses a field mask; this assumes your runtime handles masking.
    let b = ctx.op_reg(1);
    let fb = ctx.f(b).to_string();
    ctx.println_fmt(format_args!("\tctx.fpscr.store_from_guest({fb}.u32);"));
    true
}

pub(crate) fn handle_mtlr(ctx: &mut LowerCtx) -> bool {
    if !ctx.rec.config.skip_lr {
        let s = ctx.op_reg(0);
        let rs = ctx.r(s).to_string();
        ctx.println_fmt(format_args!("\tctx.lr = {rs}.u64;"));
    }
    true
}

pub(crate) fn handle_mtmsrd(ctx: &mut LowerCtx) -> bool {
    // Only emit if the build was NOT compiled with `--features skip_msr`
    #[cfg(not(feature = "skip_msr"))]
    {
        let s = ctx.op_reg(0);
        let rs = ctx.r(s).to_string();
        // Preserve all but EE/ME bits per your mask (0x8020)
        ctx.println_fmt(format_args!(
            "\tctx.msr = ({rs}.u32 & 0x8020) | (ctx.msr & !0x8020);"
        ));
    }
    true
}

pub(crate) fn handle_mtxer(ctx: &mut LowerCtx) -> bool {
    let s = ctx.op_reg(0);
    let rs  = ctx.r(s).to_string();
    let xer = ctx.xer().to_string();

    ctx.println_fmt(format_args!("\t{xer}.so = ({rs}.u64 & 0x8000_0000) != 0;"));
    ctx.println_fmt(format_args!("\t{xer}.ov = ({rs}.u64 & 0x4000_0000) != 0;"));
    ctx.println_fmt(format_args!("\t{xer}.ca = ({rs}.u64 & 0x2000_0000) != 0;"));
    true
}
