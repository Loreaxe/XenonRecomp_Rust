// /src/recompiler/instructions/handle/stores_vec.rs
use super::*;

/// Core STVEWX / STVEWX128 using explicit (vd, ra, rb).
pub(crate) fn emit_stvewx_at(ctx: &mut LowerCtx, vd: usize, ra: usize, rb: usize) -> bool {
    // STVEWX / STVEWX128: store one 32-bit lane chosen by (EA & 0xF)
    let ea  = ctx.ea().to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let rrb = ctx.r(rb).to_string();
    let vvd = ctx.v(vd).to_string();

    // ea = (ra + rb) & !3  (word-aligned)
    ctx.print(format!("\t{ea} = ("));
    if let Some(ra_s) = rra.as_ref() {
        ctx.print(format!("{ra_s}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32) & !3u32;"));

    // lane idx (Altivec BE word select): 3 - ((ea & 0xF) >> 2)
    ctx.println_fmt(format_args!("\tlet idx = 3usize - ((({ea} & 0xFu32) >> 2) as usize);"));

    // store_u32(base, ea, v[vd].u32[idx])
    ctx.println_fmt(format_args!(
        "\tunsafe {{ crate::rt::store_u32(base as *mut u8, {ea}, {vvd}.u32[idx]); }}"
    ));
    true
}

pub(crate) fn handle_stvewx_like(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);
    emit_stvewx_at(ctx, vd, ra, rb)
}


/// Core STVLX / STVLX128 using explicit (vd, ra, rb).
pub(crate) fn emit_stvlx_at(ctx: &mut LowerCtx, vd: usize, ra: usize, rb: usize) -> bool {
    // STVLX / STVLX128: store bytes from the *left* up to the next 16B boundary.
    let ea  = ctx.ea().to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let rrb = ctx.r(rb).to_string();
    let vvd = ctx.v(vd).to_string();

    // addr = ra + rb
    ctx.print(format!("\t{ea} = "));
    if let Some(ra_s) = rra.as_ref() {
        ctx.print(format!("{ra_s}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));

    // n = 16 - (addr & 0xF)
    ctx.println_fmt(format_args!("\tlet n = 16usize - ((({ea} & 0xFu32) as usize) & 0xF);"));

    // for i in 0..n { *(addr+i) = v[i] }
    ctx.println("\tfor i in 0..n {");
    ctx.println_fmt(format_args!(
        "\t\tunsafe {{ crate::rt::store_u8(base as *mut u8, {ea}.wrapping_add(i as u32), {vvd}.u8[i]); }}"
    ));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_stvlx_like(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);
    emit_stvlx_at(ctx, vd, ra, rb)
}


/// Core STVRX / STVRX128 using explicit (vd, ra, rb).
pub(crate) fn emit_stvrx_at(ctx: &mut LowerCtx, vd: usize, ra: usize, rb: usize) -> bool {
    // STVRX / STVRX128: store bytes from the *right* up to the previous 16B boundary.
    let ea  = ctx.ea().to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let rrb = ctx.r(rb).to_string();
    let vvd = ctx.v(vd).to_string();

    // addr = ra + rb
    ctx.print(format!("\t{ea} = "));
    if let Some(ra_s) = rra.as_ref() {
        ctx.print(format!("{ra_s}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));

    // n = (addr & 0xF); start = addr - n
    ctx.println_fmt(format_args!("\tlet n = ((({ea} & 0xFu32) as usize) & 0xF);"));
    ctx.println_fmt(format_args!("\tlet start = {ea}.wrapping_sub(n as u32);"));

    // for i in 0..n { *(start+i) = v[16-n+i] }
    ctx.println("\tfor i in 0..n {");
    ctx.println_fmt(format_args!(
        "\t\tunsafe {{ crate::rt::store_u8(base as *mut u8, start.wrapping_add(i as u32), {vvd}.u8[16 - n + i]); }}"
    ));
    ctx.println("\t}");
    true
}

pub(crate) fn handle_stvrx_like(ctx: &mut LowerCtx) -> bool {
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);
    emit_stvrx_at(ctx, vd, ra, rb)
}

pub(crate) fn handle_stvx_like(ctx: &mut LowerCtx) -> bool {
    // STVX / STVX128: store the full 16B vector at an unaligned address.
    // Split across two aligned 16B lines: first (16-off) bytes to base_aligned+off, then off bytes to base_aligned+16.
    let vd = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);

    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let rrb = ctx.r(rb).to_string();
    let vvd = ctx.v(vd).to_string();

    // addr = ra + rb
    ctx.print(format!("\tlet addr = "));
    if let Some(ra_s) = rra.as_ref() {
        ctx.print(format!("{ra_s}.u32 + "));
    }
    ctx.println_fmt(format_args!("{rrb}.u32;"));

    // off = addr & 0xF; base_aligned = addr & !0xF
    ctx.println("\tlet off = (addr & 0xFu32) as usize;");
    ctx.println("\tlet base_aligned = addr & !0xFu32;");

    // first: 16 - off bytes to [base_aligned+off .. base_aligned+15] from v[0..)
    ctx.println("\tlet first = 16usize - off;");
    ctx.println("\tfor i in 0..first {");
    ctx.println_fmt(format_args!(
        "\t\tunsafe {{ crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), {vvd}.u8[i]); }}"
    ));
    ctx.println("\t}");

    // then: off bytes to [base_aligned+16 .. base_aligned+16+off-1] from v[first..)
    ctx.println("\tfor i in 0..off {");
    ctx.println_fmt(format_args!(
        "\t\tunsafe {{ crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), {vvd}.u8[first + i]); }}"
    ));
    ctx.println("\t}");
    true
}


