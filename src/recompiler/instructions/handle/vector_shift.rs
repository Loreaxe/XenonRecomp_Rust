use super::*;

pub(crate) fn handle_vslb_scalar(ctx: &mut LowerCtx) -> bool {
    // VSLB (scalar loop form)
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    for i in 0..16 {
        ctx.println_fmt(format_args!(
            "\tunsafe {{ {vd}.u8[{i}] = {va}.u8[{i}] << ({vb}.u8[{i}] & 0x7); }}",
        ));
    }

    true
}

pub(crate) fn handle_vsldoi_like(ctx: &mut LowerCtx) -> bool {
    // VSLDOI / VSLDOI128, scalarised:
    // vd = (va||vb) << (imm bytes), taking the high 16 bytes after the shift.
    let d   = ctx.op_reg(0);
    let a   = ctx.op_reg(1);
    let b   = ctx.op_reg(2);
    let imm = (ctx.op_imm(3) as u32) & 0xFF;

    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    // We only care about imm mod 16 for the 16-byte result.
    let sh = (imm & 0x0F) as usize;

    // Emit an unsafe block because we're touching union fields (vd/va/vb.u8).
    ctx.println("\tunsafe {");
    ctx.println_fmt(format_args!(
        "\t\tlet sh = {sh}usize;  // VSLDOI byte shift (imm & 0xF)"
    ));
    ctx.println("\t\tfor i in 0..16 {");
    ctx.println("\t\t\tlet src = i + sh;");
    ctx.println("\t\t\tif src < 16 {");
    ctx.println_fmt(format_args!(
        "\t\t\t\t{vd}.u8[i] = {va}.u8[src];"
    ));
    ctx.println("\t\t\t} else {");
    ctx.println_fmt(format_args!(
        "\t\t\t\t{vd}.u8[i] = {vb}.u8[src - 16];"
    ));
    ctx.println("\t\t\t}");
    ctx.println("\t\t}");
    ctx.println("\t}");

    true
}

pub(crate) fn handle_vslw_scalar(ctx: &mut LowerCtx) -> bool {
    // VSLW / VSLW128 (scalar loop; ensure endianness matches your storage)
    let d = ctx.op_reg(0);
    let a = ctx.op_reg(1);
    let b = ctx.op_reg(2);

    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    ctx.println("\tunsafe {");
    for i in 0..4 {
        let bi = i * 4;
        ctx.println_fmt(format_args!(
            "\t\t{vd}.u32[{i}] = {va}.u32[{i}] << ({vb}.u8[{bi}] & 0x1F);"
        ));
    }
    ctx.println("\t}");

    true
}

pub(crate) fn handle_vsr(ctx: &mut LowerCtx) -> bool {
    // VSR (byte-wise right shift with vector count, scalar loop form)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();

    for i in 0..16 {
        ctx.println_fmt(format_args!(
            "\t{vd}.u8[{i}] = {va}.u8[{i}] >> ({vb}.u8[{i}] & 0x7);"
        ));
    }
    true
}

pub(crate) fn handle_vsraw_scalar(ctx: &mut LowerCtx) -> bool {
    // VSRAW / VSRAW128 (scalar loop)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();
    for i in 0..4 {
        let bi = i * 4;
        ctx.println_fmt(format_args!(
            "\t{vd}.s32[{i}] = {va}.s32[{i}] >> ({vb}.u8[{bi}] & 0x1F);"
        ));
    }
    true
}

pub(crate) fn handle_vsrw_scalar(ctx: &mut LowerCtx) -> bool {
    // VSRW / VSRW128 (scalar loop)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();
    for i in 0..4 {
        let bi = i * 4;
        ctx.println_fmt(format_args!(
            "\t{vd}.u32[{i}] = {va}.u32[{i}] >> ({vb}.u8[{bi}] & 0x1F);"
        ));
    }
    true
}
