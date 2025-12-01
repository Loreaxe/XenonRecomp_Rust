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
        ctx.println_fmt(format_args!("\tmatch {rs}.u64 {{"));
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
        ctx.println("\t\t_ => unsafe { core::hint::unreachable_unchecked() },");
        ctx.println("\t}");
        // Every arm either goto's another block or returns; no fallthrough.
        ctx.mark_block_terminated();          // NEW
    } else {
        let ctr = ctx.ctr().to_string();
        ctx.println_fmt(format_args!("\tcrate::rt::call_indirect({ctr}.u32);"));
        ctx.println("\treturn;");
        ctx.mark_block_terminated();          // NEW: indirect tailcall
    }
    true
}

pub(crate) fn handle_bctrl(ctx: &mut LowerCtx) -> bool {
    handle_link_if_needed(ctx);
    let ctr = ctx.ctr().to_string();
    ctx.println_fmt(format_args!("\tcrate::rt::call_indirect({ctr}.u32);"));
    *ctx.csr = CSRState::Unknown;
    ctx.mark_block_terminated();             // NEW: no dispatcher fallthrough
    true
}

pub(crate) fn handle_bdz(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();
    ctx.println_fmt(format_args!("\t{ctr}.u64 = {ctr}.u64.wrapping_sub(1);"));
    ctx.println_fmt(format_args!("\tif {ctr}.u32 == 0 {{"));
    ctx.goto(tgt);
    ctx.println("\t}");
    true
}

pub(crate) fn handle_bdzlr(ctx: &mut LowerCtx) -> bool {
    let ctr = ctx.ctr().to_string();
    ctx.println_fmt(format_args!("\t{ctr}.u64 = {ctr}.u64.wrapping_sub(1);"));
    ctx.println_fmt(format_args!("\tif {ctr}.u32 == 0 {{ return; }}"));
    true
}

pub(crate) fn handle_bdnz(ctx: &mut LowerCtx) -> bool {
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();
    ctx.println_fmt(format_args!("\t{ctr}.u64 = {ctr}.u64.wrapping_sub(1);"));
    ctx.println_fmt(format_args!("\tif {ctr}.u32 != 0 {{"));
    if tgt < ctx.fnc.base as u32 || tgt >= (ctx.fnc.base + ctx.fnc.size) as u32 {
        ctx.print("\t\t");
        ctx.print_function_call(tgt);
        ctx.println("\t\treturn;");
    } else {
        ctx.print("\t\t");
        ctx.goto(tgt);
    }
    ctx.println("\t}");
    true
}

pub(crate) fn handle_bdnzf(ctx: &mut LowerCtx) -> bool {
    // NOTE from original C++: assuming .eq here because all game uses do that.
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();
    let cr_idx = ctx.branch_cr_index();
    let cr  = ctx.cr(cr_idx).to_string();

    ctx.println_fmt(format_args!("\t{ctr}.u64 = {ctr}.u64.wrapping_sub(1);"));
    ctx.println_fmt(format_args!("\tif {ctr}.u32 != 0 && !{cr}.eq {{"));
    ctx.goto(tgt);
    ctx.println("\t}");
    true
}

pub(crate) fn handle_bdnzt(ctx: &mut LowerCtx) -> bool {
    // Decrement CTR; branch if CTR != 0 && CR.eq
    let tgt = ctx.branch_target();
    let ctr = ctx.ctr().to_string();
    let cr_idx = ctx.branch_cr_index();
    let cr  = ctx.cr(cr_idx).to_string();

    ctx.println_fmt(format_args!("\t{ctr}.u64 = {ctr}.u64.wrapping_sub(1);"));
    ctx.println_fmt(format_args!("\tif {ctr}.u32 != 0 && {cr}.eq {{"));
    ctx.goto(tgt);
    ctx.println("\t}");
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
    ctx.println_fmt(format_args!("\tif {cr}.eq {{ return; }}"));
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
    ctx.println_fmt(format_args!("\tif !{cr}.lt {{ return; }}"));
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
    ctx.println_fmt(format_args!("\tif {cr}.gt {{ return; }}"));
    true
}

pub(crate) fn handle_bl(ctx: &mut LowerCtx) -> bool {
    handle_link_if_needed(ctx);
    let tgt = ctx.branch_target();

    // 1) Prefer externs: if this BL targets a known import thunk, call wrapper directly.
    if let Some(wrap) = ctx.rec.resolve_extern_wrapper(tgt).map(|s| s.to_string()) {
        // Nice debug comment:
        ctx.println_fmt(format_args!("\t// extern call 0x{tgt:08X} → {wrap}"));
        // Direct call to the generated wrapper (in xam.rs / xboxkrnl.rs):
        ctx.println_fmt(format_args!("\t{wrap}(ctx, base);"));
    }
    // 2) Otherwise, if it’s a normal recompiled function, call it.
    else if let Some(name) = ctx.resolve_callee_name(tgt) {
        ctx.println_fmt(format_args!("\t{name}(ctx, base);"));
    }
    // 3) Fallback: unknown target, still go through addr-based dispatcher.
    else {
        ctx.println_fmt(format_args!(
            "\tcrate::recompiler::externs::call(ctx, base, 0x{tgt:08X});"
        ));
    }

    // BL has fallthrough, so keep dispatcher alive.
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
    ctx.println_fmt(format_args!("\tif !{cr}.gt {{ return; }}"));
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
    ctx.println_fmt(format_args!("\tif {cr}.lt {{ return; }}"));
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
    ctx.println_fmt(format_args!("\tif !{cr}.eq {{"));
    ctx.println_fmt(format_args!("\t\tcrate::rt::call_indirect({ctr}.u32);"));
    ctx.println("\t\treturn;");
    ctx.println("\t}");
    true
}

pub(crate) fn handle_bnelr(ctx: &mut LowerCtx) -> bool {
    let cr_idx = ctx.branch_cr_index();
    let cr = ctx.cr(cr_idx).to_string();
    ctx.println_fmt(format_args!("\tif !{cr}.eq {{ return; }}"));
    true
}
