use super::*;

pub(crate) fn handle_vcmpbfp_like(ctx: &mut LowerCtx) -> bool {
    // VCMPBFP / VCMPBFP128 — still unimplemented (bitwise float compare);
    // keep it explicit and Rust-only.
    ctx.println("\t// TODO: VCMPBFP not implemented");
    ctx.println("\tunsafe { core::intrinsics::abort(); }");
    true
}

pub(crate) fn handle_vcmpeqfp_like(ctx: &mut LowerCtx) -> bool {
    // VCMPEQFP / VCMPEQFP128
    ctx.set_flush_mode(true);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    // Per-lane f32 equality -> 0xFFFF_FFFF / 0x0000_0000 in u32
    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = if {va}.f32[i] == {vb}.f32[i] {{ 0xFFFF_FFFF }} else {{ 0 }};"));
    ctx.println("\t}");

    // Dotted form: summarize into CR6 (simple convention: eq=all lanes true; others false)
    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr6 = ctx.cr(6).to_string();
        ctx.println("\tlet mut all_true = true;");
        ctx.println("\tfor i in 0..4 { all_true &= {vd}.u32[i] != 0; }");
        ctx.println_fmt(format_args!("\t{cr6}.lt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.gt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.so = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.eq = all_true;"));
    }
    true
}

pub(crate) fn handle_vcmpequb(ctx: &mut LowerCtx) -> bool {
    // Per-byte equality -> 0xFF or 0x00 in u8 lanes
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = if {va}.u8[i] == {vb}.u8[i] {{ 0xFF }} else {{ 0x00 }};"));
    ctx.println("\t}");

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr6 = ctx.cr(6).to_string();
        ctx.println("\tlet mut all_true = true;");
        ctx.println("\tfor i in 0..16 { all_true &= {vd}.u8[i] != 0; }");
        ctx.println_fmt(format_args!("\t{cr6}.lt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.gt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.so = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.eq = all_true;"));
    }
    true
}

pub(crate) fn handle_vcmpequw_like(ctx: &mut LowerCtx) -> bool {
    // VCMPEQUW / VCMPEQUW128 — per-u32 equality -> 0xFFFF_FFFF/0
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = if {va}.u32[i] == {vb}.u32[i] {{ 0xFFFF_FFFF }} else {{ 0 }};"));
    ctx.println("\t}");

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr6 = ctx.cr(6).to_string();
        ctx.println("\tlet mut all_true = true;");
        ctx.println("\tfor i in 0..4 { all_true &= {vd}.u32[i] != 0; }");
        ctx.println_fmt(format_args!("\t{cr6}.lt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.gt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.so = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.eq = all_true;"));
    }
    true
}

pub(crate) fn handle_vcmpgefp_like(ctx: &mut LowerCtx) -> bool {
    // VCMPGEFP / VCMPGEFP128 — per-lane f32 >=
    ctx.set_flush_mode(true);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = if {va}.f32[i] >= {vb}.f32[i] {{ 0xFFFF_FFFF }} else {{ 0 }};"));
    ctx.println("\t}");

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr6 = ctx.cr(6).to_string();
        ctx.println("\tlet mut all_true = true;");
        ctx.println("\tfor i in 0..4 { all_true &= {vd}.u32[i] != 0; }");
        ctx.println_fmt(format_args!("\t{cr6}.lt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.gt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.so = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.eq = all_true;"));
    }
    true
}

pub(crate) fn handle_vcmpgtfp_like(ctx: &mut LowerCtx) -> bool {
    // VCMPGTFP / VCMPGTFP128 — per-lane f32 >
    ctx.set_flush_mode(true);
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tfor i in 0..4 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u32[i] = if {va}.f32[i] > {vb}.f32[i] {{ 0xFFFF_FFFF }} else {{ 0 }};"));
    ctx.println("\t}");

    if ctx.insn.mnemonic().unwrap_or_default().ends_with('.') {
        let cr6 = ctx.cr(6).to_string();
        ctx.println("\tlet mut all_true = true;");
        ctx.println("\tfor i in 0..4 { all_true &= {vd}.u32[i] != 0; }");
        ctx.println_fmt(format_args!("\t{cr6}.lt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.gt = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.so = false;"));
        ctx.println_fmt(format_args!("\t{cr6}.eq = all_true;"));
    }
    true
}

pub(crate) fn handle_vcmpgtub(ctx: &mut LowerCtx) -> bool {
    // per-byte unsigned >
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tfor i in 0..16 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u8[i] = if {va}.u8[i] > {vb}.u8[i] {{ 0xFF }} else {{ 0x00 }};"));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_vcmpgtuh(ctx: &mut LowerCtx) -> bool {
    // per-halfword unsigned >
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tfor i in 0..8 {");
    ctx.println_fmt(format_args!("\t\t{vd}.u16[i] = if {va}.u16[i] > {vb}.u16[i] {{ 0xFFFF }} else {{ 0x0000 }};"));
    ctx.println("\t}");
    true
}
