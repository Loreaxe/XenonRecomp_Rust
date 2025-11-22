use super::*;

pub(crate) fn handle_vslb_scalar(ctx: &mut LowerCtx) -> bool {
    // VSLB (scalar loop form)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();
    for i in 0..16 {
        ctx.println_fmt(format_args!(
            "\t{vd}.u8[{i}] = {va}.u8[{i}] << ({vb}.u8[{i}] & 0x7);"
        ));
    }
    true
}

pub(crate) fn handle_vsldoi_like(ctx: &mut LowerCtx) -> bool {
    // VSLDOI / VSLDOI128
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let imm = (ctx.op_imm(3) as u32) & 0xFF;
    // NOTE: VSLOI = left shift of [a||b] by imm bytes. With SSSE3 alignr:
    // alignr(a,b,imm) ≈ (concat(b,a) >> imm). If your previous C++ used (a,b),
    // keep that ordering for equivalence.
    let sh = 16u32.wrapping_sub(imm);

    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();
    ctx.println_fmt(format_args!(
        "\tsimde_mm_store_si128((simde__m128i*){vd}.u8, \
           simde_mm_alignr_epi8(simde_mm_load_si128((simde__m128i*){va}.u8), \
                                simde_mm_load_si128((simde__m128i*){vb}.u8), {sh}));"
    ));
    true
}

pub(crate) fn handle_vslw_scalar(ctx: &mut LowerCtx) -> bool {
    // VSLW / VSLW128 (scalar loop; ensure endianness matches your storage)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();
    for i in 0..4 {
        let bi = i * 4;
        ctx.println_fmt(format_args!(
            "\t{vd}.u32[{i}] = {va}.u32[{i}] << ({vb}.u8[{bi}] & 0x1F);"
        ));
    }
    true
}

pub(crate) fn handle_vsr(ctx: &mut LowerCtx) -> bool {
    // VSR (byte-wise right shift with vector count)
    let d = ctx.op_reg(0); let a = ctx.op_reg(1); let b = ctx.op_reg(2);
    let vd = ctx.v(d).to_string();
    let va = ctx.v(a).to_string();
    let vb = ctx.v(b).to_string();
    ctx.println_fmt(format_args!(
        "\tsimde_mm_store_si128((simde__m128i*){vd}.u8, \
           simde_mm_vsr(simde_mm_load_si128((simde__m128i*){va}.u8), \
                        simde_mm_load_si128((simde__m128i*){vb}.u8)));"
    ));
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
