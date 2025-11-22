// src/recompiler/instuctions/loads.rs
use super::*;

// ---------------------------------
// Byte/half/word/dword scalar loads
// ---------------------------------

pub(crate) fn handle_lbz(ctx: &mut LowerCtx) -> bool {
    // RT is operand 0, MEM (disp(ra)) is operand 1
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u8( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }} as u64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u8( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }} as u64;"
        ));
    }

    true
}

pub(crate) fn handle_lbzu(ctx: &mut LowerCtx) -> bool {
    // LBZU rD, disp(rA)  (update: rA <- EA)
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let ea  = ctx.ea().to_string();
    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!("\t{ea} = {ra_s}.u32.wrapping_add({imm} as u32);"));
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u8(base as *const u8, {ea}) }} as u64;"
        ));
        ctx.println_fmt(format_args!("\t{ra_s}.u32 = {ea};"));
    } else {
        ctx.println_fmt(format_args!("\t{ea} = {imm}u32;"));
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u8(base as *const u8, {ea}) }} as u64;"
        ));
    }

    true
}


pub(crate) fn handle_lbzx(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u8(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }} as u64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u8(base as *const u8, {rrb}.u32) }} as u64;"
        ));
    }
    true
}

pub(crate) fn handle_ld(ctx: &mut LowerCtx) -> bool {
    // LD rD, disp(rA)
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u64( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u64( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }};"
        ));
    }

    true
}

pub(crate) fn handle_ldarx(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let rrb = ctx.r(rb).to_string();
    let t   = ctx.reserved().to_string();

    ctx.print(format!("\t{t}.u64 = unsafe {{ *(base.add("));
    if let Some(ra_s) = rra {
        ctx.print(format!("{ra_s}.u32.wrapping_add("));
    }
    ctx.println_fmt(format_args!("{rrb}.u32) as usize) as *const u64 }} as u64;"));
    ctx.println_fmt(format_args!("\t{rd}.u64 = {t}.u64.swap_bytes();"));
    true
}

pub(crate) fn handle_ldu(ctx: &mut LowerCtx) -> bool {
    // LDU rD, disp(rA)  (update: rA <- EA)
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let ea  = ctx.ea().to_string();
    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!("\t{ea} = {ra_s}.u32.wrapping_add({imm} as u32);"));
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u64(base as *const u8, {ea}) }};"
        ));
        ctx.println_fmt(format_args!("\t{ra_s}.u32 = {ea};"));
    } else {
        ctx.println_fmt(format_args!("\t{ea} = {imm}u32;"));
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u64(base as *const u8, {ea}) }};"
        ));
    }

    true
}

pub(crate) fn handle_ldx(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u64(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u64(base as *const u8, {rrb}.u32) }};"
        ));
    }
    true
}

// -----------------
// Floating loads
// -----------------

pub(crate) fn handle_lfd(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let fd  = ctx.f(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{fd}.u64 = unsafe {{ \
                crate::rt::load_u64( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{fd}.u64 = unsafe {{ \
                crate::rt::load_u64( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }};"
        ));
    }

    true
}


pub(crate) fn handle_lfdx(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let fd  = ctx.f(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{fd}.u64 = unsafe {{ crate::rt::load_u64(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{fd}.u64 = unsafe {{ crate::rt::load_u64(base as *const u8, {rrb}.u32) }};"
        ));
    }
    true
}

pub(crate) fn handle_lfs(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let t   = ctx.temp().to_string();
    let fd  = ctx.f(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{t}.u32 = unsafe {{ \
                crate::rt::load_u32( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }};"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{t}.u32 = unsafe {{ \
                crate::rt::load_u32( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }};"
        ));
    }
    ctx.println_fmt(format_args!("\t{fd}.f64 = ({t}.f32 as f64);"));
    true
}

pub(crate) fn handle_lfsx(ctx: &mut LowerCtx) -> bool {
    ctx.set_flush_mode(false);
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let t   = ctx.temp().to_string();
    let fd  = ctx.f(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!("\t{t}.u32 = unsafe {{ crate::rt::load_u32(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }};"));
    } else {
        ctx.println_fmt(format_args!("\t{t}.u32 = unsafe {{ crate::rt::load_u32(base as *const u8, {rrb}.u32) }};"));
    }
    ctx.println_fmt(format_args!("\t{fd}.f64 = ({t}.f32 as f64);"));
    true
}

// -----------------
// Half/word variants
// -----------------

pub(crate) fn handle_lha(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ \
                crate::rt::load_u16( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }} as i16) as i64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ \
                crate::rt::load_u16( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }} as i16) as i64;"
        ));
    }

    true
}

pub(crate) fn handle_lhax(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ crate::rt::load_u16(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }} as i16) as i64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ crate::rt::load_u16(base as *const u8, {rrb}.u32) }} as i16) as i64;"
        ));
    }
    true
}

pub(crate) fn handle_lhz(ctx: &mut LowerCtx) -> bool {
    // RT is operand 0
    let d = ctx.op_reg(0);
    // MEM (disp(ra)) is operand 1
    let (ra_opt, disp) = ctx.op_mem(1);

    let rd  = ctx.r(d).to_string();
    let imm = disp as i32; // signed, like int32_t in the C++ emitter

    if let Some(ra) = ra_opt {
        // RA != 0 → EA = rA.u32 + imm
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u16( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }} as u64;"
        ));
    } else {
        // RA field == 0 → absolute EA = imm
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u16( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }} as u64;"
        ));
    }

    true
}

pub(crate) fn handle_lhzu(ctx: &mut LowerCtx) -> bool {
    // lhzu rD, disp(rA)
    let d = ctx.op_reg(0);
    let (base, disp) = ctx.op_mem(1); // mem operand is the second operand

    let rd = ctx.r(d).to_string();
    let ea_name = ctx.ea();
    let ea_expr = ctx.ea_expr_u32(base, disp);

    // RA is the base GPR in the MEM operand
    let ra_idx = base.unwrap_or(0);
    let ra = ctx.r(ra_idx).to_string();

    ctx.println_fmt(format_args!("\t{ea_name} = {ea_expr};"));
    ctx.println_fmt(format_args!(
        "\t{rd}.u32 = unsafe {{ crate::rt::load_u16(base as *mut u8, {ea_name}) }} as u32;"
    ));
    ctx.println_fmt(format_args!("\t{ra}.u32 = {ea_name};"));

    true
}

pub(crate) fn handle_lhzx(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u16(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }} as u64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u16(base as *const u8, {rrb}.u32) }} as u64;"
        ));
    }
    true
}

// -----------------
// Immediates
// -----------------

pub(crate) fn handle_li(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let imm = ctx.op_imm(1) as i32;

    let rd = ctx.r(d).to_string();
    ctx.println_fmt(format_args!("\t{rd}.s64 = {imm};"));
    true
}

pub(crate) fn handle_lis(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let imm = (ctx.op_imm(1) as i32) << 16;

    let rd = ctx.r(d).to_string();
    ctx.println_fmt(format_args!("\t{rd}.s64 = {imm};"));
    true
}

// -----------------
// More scalar loads
// -----------------

pub(crate) fn handle_lwa(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ \
                crate::rt::load_u32( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }} as i32) as i64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ \
                crate::rt::load_u32( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }} as i32) as i64;"
        ));
    }

    true
}

pub(crate) fn handle_lwarx(ctx: &mut LowerCtx) -> bool {
    // LWARX rD, rA, rB  -> load word with reservation
    let d  = ctx.op_reg(0);
    let ra = ctx.op_reg(1);
    let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };
    let t   = ctx.reserved().to_string();

    ctx.println("\t// lwarx");

    // Effective address: if RA == 0, EA = RB; else EA = RA + RB
    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!("\tlet ea = {ra}.u32.wrapping_add({rb}.u32);", ra = ra_s, rb = rrb));
    } else {
        ctx.println_fmt(format_args!("\tlet ea = {rb}.u32;", rb = rrb));
    }

    // Load, set reservation, and write to rD
    ctx.println_fmt(format_args!(
        "\t{t}.u32 = unsafe {{ crate::rt::load_u32(base as *const u8, ea) }};"
    ));
    ctx.println_fmt(format_args!("\t{rd}.u64 = {t}.u32 as u64;"));

    true
}

pub(crate) fn handle_lwax(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ crate::rt::load_u32(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }} as i32) as i64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.s64 = (unsafe {{ crate::rt::load_u32(base as *const u8, {rrb}.u32) }} as i32) as i64;"
        ));
    }
    true
}

pub(crate) fn handle_lwbrx(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = (unsafe {{ crate::rt::load_u32(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }}).swap_bytes() as u64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = (unsafe {{ crate::rt::load_u32(base as *const u8, {rrb}.u32) }}).swap_bytes() as u64;"
        ));
    }
    true
}

pub(crate) fn handle_lhbrx(ctx: &mut LowerCtx) -> bool {
    // lhbrx rD, rA, rB  — EA = rA + rB, load halfword, byte-reverse, zero-extend.
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let rd = ctx.r(d).to_string();
    let ra = ctx.r(a).to_string();
    let rb = ctx.r(b).to_string();
    let ea_name = ctx.ea();

    ctx.println_fmt(format_args!(
        "\t{ea_name} = {ra}.u32.wrapping_add({rb}.u32);"
    ));
    ctx.println_fmt(format_args!(
        "\t{rd}.u32 = (unsafe {{ crate::rt::load_u16(base as *mut u8, {ea_name}) }} as u16).swap_bytes() as u32;"
    ));

    true
}

pub(crate) fn handle_lwsync(_: &mut LowerCtx) -> bool { true }

pub(crate) fn handle_lwz(ctx: &mut LowerCtx) -> bool {
    // RT is operand 0
    let d = ctx.op_reg(0);
    // MEM (disp(ra)) is operand 1
    let (ra_opt, disp) = ctx.op_mem(1);

    let rd = ctx.r(d).to_string();
    let imm = disp as i32; // signed offset, like int32_t in C++

    if let Some(ra) = ra_opt {
        // have a base register → rA.u32 + imm
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u32( \
                    base as *const u8, \
                    {ra_s}.u32.wrapping_add({imm} as u32) \
                ) \
            }} as u64;"
        ));
    } else {
        // RA field == 0 → use absolute EA = imm
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ \
                crate::rt::load_u32( \
                    base as *const u8, \
                    {imm}u32 \
                ) \
            }} as u64;"
        ));
    }

    true
}

pub(crate) fn handle_lwzu(ctx: &mut LowerCtx) -> bool {
    // LWZU rD, disp(rA)  (update: rA <- EA)
    let d = ctx.op_reg(0);
    let (ra_opt, disp) = ctx.op_mem(1);

    let ea  = ctx.ea().to_string();
    let rd  = ctx.r(d).to_string();
    let imm = disp as i32;

    if let Some(ra) = ra_opt {
        let ra_s = ctx.r(ra).to_string();
        ctx.println_fmt(format_args!("\t{ea} = {ra_s}.u32.wrapping_add({imm} as u32);"));
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u32(base as *const u8, {ea}) }} as u64;"
        ));
        ctx.println_fmt(format_args!("\t{ra_s}.u32 = {ea};"));
    } else {
        ctx.println_fmt(format_args!("\t{ea} = {imm}u32;"));
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u32(base as *const u8, {ea}) }} as u64;"
        ));
    }

    true
}

pub(crate) fn handle_lwzx(ctx: &mut LowerCtx) -> bool {
    let d = ctx.op_reg(0); let ra = ctx.op_reg(1); let rb = ctx.op_reg(2);

    let rd  = ctx.r(d).to_string();
    let rrb = ctx.r(rb).to_string();
    let rra = if ra != 0 { Some(ctx.r(ra).to_string()) } else { None };

    if let Some(ra_s) = rra {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u32(base as *const u8, {ra_s}.u32.wrapping_add({rrb}.u32)) }} as u64;"
        ));
    } else {
        ctx.println_fmt(format_args!(
            "\t{rd}.u64 = unsafe {{ crate::rt::load_u32(base as *const u8, {rrb}.u32) }} as u64;"
        ));
    }
    true
}
