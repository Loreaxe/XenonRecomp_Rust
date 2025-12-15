// src/recompiler/instructions/handel/branch.rs
use super::*;
use crate::recompiler::CSRState;

// These emit *Rust* code only. Runtime provides `crate::rt::call_indirect(target: u32)`.

pub(crate) fn handle_b(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();

    if ctx.target_in_current_function(tgt) {
        // Intra-function jump.
        ctx.goto(tgt);
        ctx.mark_block_terminated();
    } else {
        // Cross-function or external: tail call & return.
        ctx.print_function_call(tgt);
        ctx.println("\treturn;");
        ctx.mark_block_terminated();
    }

    true
}


pub(crate) fn handle_bctr(ctx: &mut LowerCtx) -> bool {
    if let Some(sw) = ctx.rec.switch_tables.remove(&ctx.base) {
        let rs = ctx.r(sw.r as usize).to_string();

        // Accessing union field rs.u64 and calling unreachable_unchecked => unsafe block.
        ctx.println_fmt(format_args!("\tunsafe {{ match {rs}.u64 {{"));
        for (i, label) in sw.labels.iter().enumerate() {
            ctx.println_fmt(format_args!("\t\t{i} => {{"));
            if *label < ctx.fnc.base as u32 || *label >= (ctx.fnc.base + ctx.fnc.size) as u32 {
                ctx.println_fmt(format_args!("\t\t\t// ERROR: 0x{label:08X}"));
                ctx.println("\t\t\treturn;");
            } else {
                ctx.goto(*label);
            }
            ctx.println("\t\t},");
        }
        ctx.println("\t\t_ => core::hint::unreachable_unchecked(),");
        ctx.println("\t}}");
        // Every arm either goto's another block or returns; no fallthrough.
        ctx.mark_block_terminated();
    } else {
        let ctr = ctx.ctr().to_string();
        // call_indirect is unsafe and ctr.u32 is a union field
        ctx.println_fmt(format_args!(
            "\tunsafe {{ crate::rt::call_indirect({ctr}.u32, ctx, base); }}",
            ctr = ctr,
        ));
        ctx.println("\treturn;");
        ctx.mark_block_terminated();
    }
    true
}

pub(crate) fn handle_bctrl(ctx: &mut LowerCtx) -> bool {
    // Set LR to return address if needed (BL-style link semantics).
    handle_link_if_needed(ctx);

    let ctr = ctx.ctr().to_string();

    // Indirect call through CTR. This may re-enter guest code, but we
    // expect to return here afterwards (normal call, not tailcall).
    ctx.println_fmt(format_args!(
        "\tunsafe {{ crate::rt::call_indirect({ctr}.u32, ctx, base); }}",
        ctr = ctr,
    ));

    // After a call, CSR is unknown, same as for BL.
    *ctx.csr = CSRState::Unknown;

    // NOTE: NO `ctx.mark_block_terminated()` here.
    // We want to keep lowering instructions after the bctrl, so the
    // rest of the function body (epilogue, etc.) is emitted.
    true
}

pub(crate) fn handle_bdz(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();

    // Decrement CTR (union field) in unsafe
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ctr}.u64 = {ctr}.u64.wrapping_sub(1); }}",
        ctr = ctr,
    ));
    // Test CTR == 0 in unsafe as well
    ctx.println_fmt(format_args!(
        "\tunsafe {{ if {ctr}.u32 == 0 {{",
        ctr = ctr,
    ));
    ctx.goto(tgt);
    ctx.println("\t}}");
    true
}

pub(crate) fn handle_bdzlr(ctx: &mut LowerCtx) -> bool {
    let ctr = ctx.ctr().to_string();
    // Decrement CTR and branch-to-LR if zero
    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ctr}.u64 = {ctr}.u64.wrapping_sub(1); }}",
        ctr = ctr,
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ if {ctr}.u32 == 0 {{ return; }} }}",
        ctr = ctr,
    ));
    true
}

pub(crate) fn handle_bdnz(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ctr}.u64 = {ctr}.u64.wrapping_sub(1); }}",
        ctr = ctr,
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ if {ctr}.u32 != 0 {{",
        ctr = ctr,
    ));

    if tgt < ctx.fnc.base as u32 || tgt >= (ctx.fnc.base + ctx.fnc.size) as u32 {
        ctx.print("\t\t");
        ctx.print_function_call(tgt);
        ctx.println("\t\treturn;");
    } else {
        ctx.print("\t\t");
        ctx.goto(tgt);
    }

    ctx.println("\t\t}");
    ctx.println("\t}");
    true
}

pub(crate) fn handle_bdnzf(ctx: &mut LowerCtx) -> bool {
    // branch if CTR != 0 && CR.eq == 0
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ctr}.u64 = {ctr}.u64.wrapping_sub(1); }}",
        ctr = ctr,
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ if {ctr}.u32 != 0 && {cr}.eq == 0 {{",
        ctr = ctr,
        cr = cr,
    ));
    ctx.goto(tgt);
    ctx.println("\t}}");
    true
}

pub(crate) fn handle_bdnzt(ctx: &mut LowerCtx) -> bool {
    // branch if CTR != 0 && CR.eq != 0
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();

    ctx.println_fmt(format_args!(
        "\tunsafe {{ {ctr}.u64 = {ctr}.u64.wrapping_sub(1); }}",
        ctr = ctr,
    ));
    ctx.println_fmt(format_args!(
        "\tunsafe {{ if {ctr}.u32 != 0 && {cr}.eq != 0 {{",
        ctr = ctr,
        cr = cr,
    ));
    ctx.goto(tgt);
    ctx.println("\t}}");
    true
}

// --- simple conditional branches using CR0 by default ---

pub(crate) fn handle_beq(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    ctx.print_conditional_branch(false, "eq", tgt);
    true
}

pub(crate) fn handle_beqlr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    // branch to LR if EQ bit set
    ctx.println_fmt(format_args!("\tif {cr}.eq != 0 {{ return; }}", cr = cr));
    true
}

pub(crate) fn handle_bge(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    ctx.print_conditional_branch(true, "lt", tgt);
    true
}

pub(crate) fn handle_bgelr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    // GE == !LT  → branch if lt == 0
    ctx.println_fmt(format_args!("\tif {cr}.lt == 0 {{ return; }}", cr = cr));
    true
}

pub(crate) fn handle_bgt(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    ctx.print_conditional_branch(false, "gt", tgt);
    true
}

pub(crate) fn handle_bgtlr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    // GT bit set
    ctx.println_fmt(format_args!("\tif {cr}.gt != 0 {{ return; }}", cr = cr));
    true
}

pub(crate) fn handle_bl(ctx: &mut LowerCtx) -> bool {
    handle_link_if_needed(ctx);
    let tgt = ctx.branch_target();

    // 1) Prefer extern wrappers: if this BL targets a known import thunk,
    //    call wrapper directly with a nice comment.
    if let Some(wrap) = ctx.rec.resolve_extern_wrapper(tgt).map(|s| s.to_string()) {
        ctx.println_fmt(format_args!("\t// extern call 0x{tgt:08X} → {wrap}"));
        ctx.println_fmt(format_args!("\t{wrap}(ctx, base);"));
    } else {
        // 2) Everything else goes through the generic helper
        //    (which now only direct-calls real functions).
        ctx.print_function_call(tgt);
    }

    // BL has fallthrough, so keep the CSR unknown.
    *ctx.csr = CSRState::Unknown;
    true
}

pub(crate) fn handle_ble(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    ctx.print_conditional_branch(true, "gt", tgt);
    true
}

pub(crate) fn handle_blelr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    // LE == !GT → branch if gt == 0
    ctx.println_fmt(format_args!("\tif {cr}.gt == 0 {{ return; }}", cr = cr));
    true
}

pub(crate) fn handle_blr(ctx: &mut LowerCtx) -> bool {
    // plain return
    ctx.println("\treturn;");
    ctx.mark_block_terminated();             // NEW
    true
}


pub(crate) fn handle_blrl(ctx: &mut LowerCtx) -> bool {
    ctx.println("\tunsafe { crate::rt::debugtrap() }");
    true
}

pub(crate) fn handle_blt(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    ctx.print_conditional_branch(false, "lt", tgt);
    true
}


pub(crate) fn handle_bltlr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    // LT bit set
    ctx.println_fmt(format_args!("\tif {cr}.lt != 0 {{ return; }}", cr = cr));
    true
}

pub(crate) fn handle_bne(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    ctx.print_conditional_branch(true, "eq", tgt);
    true
}

pub(crate) fn handle_bnectr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    let ctr = ctx.ctr().to_string();
    // if EQ bit == 0, indirect call via CTR
    ctx.println_fmt(format_args!("\tif {cr}.eq == 0 {{", cr = cr));
    ctx.println_fmt(format_args!(
        "\t\tunsafe {{ crate::rt::call_indirect({ctr}.u32, ctx, base); }}",
        ctr = ctr,
    ));
    ctx.println("\t\treturn;");
    ctx.println("\t}");
    true
}

pub(crate) fn handle_bnelr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    // BNE == !EQ → branch if eq == 0
    ctx.println_fmt(format_args!("\tif {cr}.eq == 0 {{ return; }}", cr = cr));
    true
}
