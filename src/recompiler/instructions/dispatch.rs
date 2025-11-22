// src/recompiler/instructions/dispatch.rs

use capstone::arch::ppc::PpcInsn;
use capstone::{Insn, Capstone};

use crate::function::Function;
use crate::recompiler::{CSRState, Recompiler, RecompilerLocalVariables};
use super::ctx::Vmx128MemOp;

use super::{
    LowerCtx,
    has_mn,
    has_any_mn,
    id_u32,
};

// Bring in all handler families
use super::handle::arith::*;
use super::handle::logical::*;
use super::handle::branch::*;
use super::handle::compare::*;
use super::handle::bitcount::*;
use super::handle::cache::*;
use super::handle::divide::*;
use super::handle::system::*;
use super::handle::extend::*;
use super::handle::float_ops::*;
use super::handle::loads::*;
use super::handle::loads_vec::*;
use super::handle::stores::*;
use super::handle::stores_vec::*;
use super::handle::moves_spr::*;
use super::handle::rotate::*;
use super::handle::shift::*;
use super::handle::vector_arith::*;
use super::handle::vector_cmp::*;
use super::handle::vector_pack::*;
use super::handle::vector_perm::*;
use super::handle::vector_shift::*;
use super::handle::vector_misc::*;

// ==================================================================================
// Dispatch table
// ==================================================================================

fn dispatch_by_id(ctx: &mut LowerCtx<'_>) -> bool {

    // ---- First: Xenon VMX128 vector loads/stores (decode from raw word) ----
    if let Some(op) = ctx.vmx128_mem_op() {
        match op {
            // mask/shift generators
            Vmx128MemOp::Lvsl128 => return handle_lvsl(ctx),
            Vmx128MemOp::Lvsr128 => return handle_lvsr(ctx),

            // load families (we reuse your existing handlers)
            Vmx128MemOp::Lvewx128 | Vmx128MemOp::Lvx128 | Vmx128MemOp::Lvxl128 => {
                return handle_lvewx_like(ctx);
            }
            Vmx128MemOp::Lvlx128 | Vmx128MemOp::Lvlxl128 => {
                return handle_lvlx_like(ctx);
            }
            Vmx128MemOp::Lvrx128 | Vmx128MemOp::Lvrxl128 => {
                return handle_lvrx_like(ctx);
            }

            // store families
            Vmx128MemOp::Stvewx128 | Vmx128MemOp::Stvx128 | Vmx128MemOp::Stvxl128 => {
                return handle_stvewx_like(ctx);
            }
            Vmx128MemOp::Stvlx128 | Vmx128MemOp::Stvlxl128 => {
                return handle_stvlx_like(ctx);
            }
            Vmx128MemOp::Stvrx128 | Vmx128MemOp::Stvrxl128 => {
                return handle_stvrx_like(ctx);
            }
        }
    }

    // ---- Fall back to normal Capstone-based dispatch ----    
    match id_u32(ctx.insn) {
        x if x == PpcInsn::PPC_INS_ADD as u32    => handle_add(ctx),
        x if x == PpcInsn::PPC_INS_ADDE as u32   => handle_adde(ctx),
        x if x == PpcInsn::PPC_INS_ADDI as u32   => handle_addi(ctx),
        x if x == PpcInsn::PPC_INS_ADDIC as u32  => handle_addic(ctx),
        x if x == PpcInsn::PPC_INS_ADDIS as u32  => handle_addis(ctx),
        x if x == PpcInsn::PPC_INS_ADDZE as u32  => handle_addze(ctx),
        x if x == PpcInsn::PPC_INS_AND as u32    => handle_and(ctx),
        x if x == PpcInsn::PPC_INS_ANDC as u32   => handle_andc(ctx),
        x if x == PpcInsn::PPC_INS_ANDI as u32   => handle_andi(ctx),
        x if x == PpcInsn::PPC_INS_ANDIS as u32  => handle_andis(ctx),
        x if x == PpcInsn::PPC_INS_B as u32      => handle_b(ctx),
        x if x == PpcInsn::PPC_INS_BCTR as u32   => handle_bctr(ctx),
        x if x == PpcInsn::PPC_INS_BCTRL as u32  => handle_bctrl(ctx),
        x if x == PpcInsn::PPC_INS_BDZ as u32    => handle_bdz(ctx),
        x if x == PpcInsn::PPC_INS_BDZLR as u32  => handle_bdzlr(ctx),
        x if x == PpcInsn::PPC_INS_BDNZ as u32   => handle_bdnz(ctx),
        x if x == PpcInsn::PPC_INS_BDNZF as u32  => handle_bdnzf(ctx),
        x if x == PpcInsn::PPC_INS_BDNZT as u32  => handle_bdnzt(ctx),
        x if x == PpcInsn::PPC_INS_BEQ as u32    => handle_beq(ctx),
        x if x == PpcInsn::PPC_INS_BEQLR as u32  => handle_beqlr(ctx),
        x if x == PpcInsn::PPC_INS_BGE as u32    => handle_bge(ctx),
        x if x == PpcInsn::PPC_INS_BGELR as u32  => handle_bgelr(ctx),
        x if x == PpcInsn::PPC_INS_BGT as u32    => handle_bgt(ctx),
        x if x == PpcInsn::PPC_INS_BGTLR as u32  => handle_bgtlr(ctx),
        x if x == PpcInsn::PPC_INS_BL as u32     => handle_bl(ctx),
        x if x == PpcInsn::PPC_INS_BLE as u32    => handle_ble(ctx),
        x if x == PpcInsn::PPC_INS_BLELR as u32  => handle_blelr(ctx),
        x if x == PpcInsn::PPC_INS_BLR as u32    => handle_blr(ctx),
        x if x == PpcInsn::PPC_INS_BLRL as u32   => handle_blrl(ctx),
        x if x == PpcInsn::PPC_INS_BLT as u32    => handle_blt(ctx),
        x if x == PpcInsn::PPC_INS_BLTLR as u32  => handle_bltlr(ctx),
        x if x == PpcInsn::PPC_INS_BNE as u32    => handle_bne(ctx),
        x if x == PpcInsn::PPC_INS_BNECTR as u32 => handle_bnectr(ctx),
        x if x == PpcInsn::PPC_INS_BNELR as u32  => handle_bnelr(ctx),

        _ if has_mn(ctx, "cctpl")      => handle_cctpl(ctx),
        _ if has_mn(ctx, "cctpm")      => handle_cctpm(ctx),
        x if x == PpcInsn::PPC_INS_CLRLDI as u32 => handle_clrldi(ctx),
        x if x == PpcInsn::PPC_INS_CLRLWI as u32 => handle_clrlwi(ctx),

        // ----- compares -----
        x if x == PpcInsn::PPC_INS_CMPD as u32
          || has_mn(ctx, "cmpd")               => handle_cmpd(ctx),
        x if x == PpcInsn::PPC_INS_CMPDI as u32
          || has_mn(ctx, "cmpdi")              => handle_cmpdi(ctx),

        x if x == PpcInsn::PPC_INS_CMPLD as u32
          || has_mn(ctx, "cmpld")              => handle_cmpld(ctx),
        x if x == PpcInsn::PPC_INS_CMPLDI as u32
          || has_mn(ctx, "cmpldi")             => handle_cmpldi(ctx),

        x if x == PpcInsn::PPC_INS_CMPLW as u32
          || has_mn(ctx, "cmplw")              => handle_cmplw(ctx),
        x if x == PpcInsn::PPC_INS_CMPLWI as u32
          || has_mn(ctx, "cmplwi")             => handle_cmplwi(ctx),

        x if x == PpcInsn::PPC_INS_CMPW as u32
          || has_mn(ctx, "cmpw")               => handle_cmpw(ctx),
        x if x == PpcInsn::PPC_INS_CMPWI as u32
          || has_mn(ctx, "cmpwi")              => handle_cmpwi(ctx),

        x if x == PpcInsn::PPC_INS_CNTLZD as u32 => handle_cntlzd(ctx),
        x if x == PpcInsn::PPC_INS_CNTLZW as u32 => handle_cntlzw(ctx),

        _ if has_mn(ctx, "db16cyc")    => handle_db16cyc(ctx),
        x if x == PpcInsn::PPC_INS_DCBF as u32    => handle_dcbf(ctx),
        x if x == PpcInsn::PPC_INS_DCBT as u32    => handle_dcbt(ctx),
        x if x == PpcInsn::PPC_INS_DCBTST as u32  => handle_dcbtst(ctx),
        x if x == PpcInsn::PPC_INS_DCBZ as u32    => handle_dcbz(ctx),
        x if x == PpcInsn::PPC_INS_DCBZL as u32   => handle_dcbzl(ctx),

        x if x == PpcInsn::PPC_INS_DIVD as u32   => handle_divd(ctx),
        x if x == PpcInsn::PPC_INS_DIVDU as u32  => handle_divdu(ctx),
        x if x == PpcInsn::PPC_INS_DIVW as u32   => handle_divw(ctx),
        x if x == PpcInsn::PPC_INS_DIVWU as u32  => handle_divwu(ctx),

        x if x == PpcInsn::PPC_INS_EIEIO as u32  => handle_eieio(ctx),

        x if x == PpcInsn::PPC_INS_EXTSB as u32  => handle_extsb(ctx),
        x if x == PpcInsn::PPC_INS_EXTSH as u32  => handle_extsh(ctx),
        x if x == PpcInsn::PPC_INS_EXTSW as u32  => handle_extsw(ctx),

        x if x == PpcInsn::PPC_INS_FABS as u32   => handle_fabs(ctx),
        x if x == PpcInsn::PPC_INS_FADD as u32   => handle_fadd(ctx),
        x if x == PpcInsn::PPC_INS_FADDS as u32  => handle_fadds(ctx),
        x if x == PpcInsn::PPC_INS_FCFID as u32  => handle_fcfid(ctx),
        x if x == PpcInsn::PPC_INS_FCMPU as u32  => handle_fcmpu(ctx),
        x if x == PpcInsn::PPC_INS_FCTID as u32  => handle_fctid(ctx),
        x if x == PpcInsn::PPC_INS_FCTIDZ as u32 => handle_fctidz(ctx),
        x if x == PpcInsn::PPC_INS_FCTIWZ as u32 => handle_fctiwz(ctx),
        x if x == PpcInsn::PPC_INS_FDIV as u32   => handle_fdiv(ctx),
        x if x == PpcInsn::PPC_INS_FDIVS as u32  => handle_fdivs(ctx),
        x if x == PpcInsn::PPC_INS_FMADD as u32  => handle_fmadd(ctx),
        x if x == PpcInsn::PPC_INS_FMADDS as u32 => handle_fmadds(ctx),
        x if x == PpcInsn::PPC_INS_FMR as u32    => handle_fmr(ctx),
        x if x == PpcInsn::PPC_INS_FMSUB as u32  => handle_fmsub(ctx),
        x if x == PpcInsn::PPC_INS_FMSUBS as u32 => handle_fmsubs(ctx),
        x if x == PpcInsn::PPC_INS_FMUL as u32   => handle_fmul(ctx),
        x if x == PpcInsn::PPC_INS_FMULS as u32  => handle_fmuls(ctx),
        x if x == PpcInsn::PPC_INS_FNABS as u32  => handle_fnabs(ctx),
        x if x == PpcInsn::PPC_INS_FNEG as u32   => handle_fneg(ctx),
        x if x == PpcInsn::PPC_INS_FNMADDS as u32=> handle_fnmadds(ctx),
        x if x == PpcInsn::PPC_INS_FNMSUB as u32 => handle_fnmsub(ctx),
        x if x == PpcInsn::PPC_INS_FNMSUBS as u32 => handle_fnmsubs(ctx),
        x if x == PpcInsn::PPC_INS_FRES as u32   => handle_fres(ctx),
        x if x == PpcInsn::PPC_INS_FRSQRTE as u32 => handle_frsqrte(ctx),
        x if x == PpcInsn::PPC_INS_FRSP as u32   => handle_frsp(ctx),
        x if x == PpcInsn::PPC_INS_FSEL as u32   => handle_fsel(ctx),
        x if x == PpcInsn::PPC_INS_FSQRT as u32  => handle_fsqrt(ctx),
        x if x == PpcInsn::PPC_INS_FSQRTS as u32 => handle_fsqrts(ctx),
        x if x == PpcInsn::PPC_INS_FSUB as u32   => handle_fsub(ctx),
        x if x == PpcInsn::PPC_INS_FSUBS as u32  => handle_fsubs(ctx),

        x if x == PpcInsn::PPC_INS_LBZ as u32     => handle_lbz(ctx),
        x if x == PpcInsn::PPC_INS_LBZU as u32    => handle_lbzu(ctx),
        x if x == PpcInsn::PPC_INS_LBZX as u32    => handle_lbzx(ctx),
        x if x == PpcInsn::PPC_INS_LD as u32      => handle_ld(ctx),
        x if x == PpcInsn::PPC_INS_LDARX as u32   => handle_ldarx(ctx),
        x if x == PpcInsn::PPC_INS_LDU as u32     => handle_ldu(ctx),
        x if x == PpcInsn::PPC_INS_LDX as u32     => handle_ldx(ctx),
        x if x == PpcInsn::PPC_INS_LFD as u32     => handle_lfd(ctx),
        x if x == PpcInsn::PPC_INS_LFDX as u32    => handle_lfdx(ctx),
        x if x == PpcInsn::PPC_INS_LFS as u32     => handle_lfs(ctx),
        x if x == PpcInsn::PPC_INS_LFSX as u32    => handle_lfsx(ctx),
        x if x == PpcInsn::PPC_INS_LHA as u32     => handle_lha(ctx),
        x if x == PpcInsn::PPC_INS_LHAX as u32    => handle_lhax(ctx),
        x if x == PpcInsn::PPC_INS_LHZ as u32     => handle_lhz(ctx),
        x if x == PpcInsn::PPC_INS_LHZU as u32    => handle_lhzu(ctx),
        x if x == PpcInsn::PPC_INS_LHZX as u32    => handle_lhzx(ctx),
        x if x == PpcInsn::PPC_INS_LI as u32      => handle_li(ctx),
        x if x == PpcInsn::PPC_INS_LIS as u32     => handle_lis(ctx),

        _ if has_any_mn(ctx, &["lvewx", "lvewx128", "lvx", "lvx128"]) => handle_lvewx_like(ctx),
        _ if has_any_mn(ctx, &["lvlx", "lvlx128"]) => handle_lvlx_like(ctx),
        _ if has_any_mn(ctx, &["lvrx", "lvrx128"]) => handle_lvrx_like(ctx),

        x if x == PpcInsn::PPC_INS_LVSL as u32    => handle_lvsl(ctx),
        x if x == PpcInsn::PPC_INS_LVSR as u32    => handle_lvsr(ctx),

        x if x == PpcInsn::PPC_INS_LWA as u32     => handle_lwa(ctx),
        x if x == PpcInsn::PPC_INS_LWARX as u32   => handle_lwarx(ctx),
        x if x == PpcInsn::PPC_INS_LWAX as u32    => handle_lwax(ctx),
        x if x == PpcInsn::PPC_INS_LWBRX as u32   => handle_lwbrx(ctx),
        x if x == PpcInsn::PPC_INS_LHBRX as u32   => handle_lhbrx(ctx),
        x if x == PpcInsn::PPC_INS_LWSYNC as u32  => handle_lwsync(ctx),
        x if x == PpcInsn::PPC_INS_LWZ as u32     => handle_lwz(ctx),
        x if x == PpcInsn::PPC_INS_LWZU as u32    => handle_lwzu(ctx),
        x if x == PpcInsn::PPC_INS_LWZX as u32    => handle_lwzx(ctx),

        x if x == PpcInsn::PPC_INS_MFCR as u32    => handle_mfcr(ctx),
        x if x == PpcInsn::PPC_INS_MFFS as u32    => handle_mffs(ctx),
        x if x == PpcInsn::PPC_INS_MFLR as u32    => handle_mflr(ctx),
        x if x == PpcInsn::PPC_INS_MFMSR as u32   => handle_mfmsr(ctx),
        x if x == PpcInsn::PPC_INS_MFOCRF as u32  => handle_mfocrf(ctx),
        x if x == PpcInsn::PPC_INS_MFTB as u32    => handle_mftb(ctx),
        x if x == PpcInsn::PPC_INS_MR as u32      => handle_mr(ctx),
        x if x == PpcInsn::PPC_INS_MTCR as u32    => handle_mtcr(ctx),
        x if x == PpcInsn::PPC_INS_MTCRF as u32   => handle_mtcrf(ctx),
        x if x == PpcInsn::PPC_INS_MTCTR as u32   => handle_mtctr(ctx),
        x if x == PpcInsn::PPC_INS_MTFSF as u32   => handle_mtfsf(ctx),
        x if x == PpcInsn::PPC_INS_MTLR as u32    => handle_mtlr(ctx),
        x if x == PpcInsn::PPC_INS_MTMSRD as u32  => handle_mtmsrd(ctx),
        x if x == PpcInsn::PPC_INS_MTXER as u32   => handle_mtxer(ctx),

        x if x == PpcInsn::PPC_INS_MULHD as u32  => handle_mulhd(ctx),
        x if x == PpcInsn::PPC_INS_MULHW as u32   => handle_mulhw(ctx),
        x if x == PpcInsn::PPC_INS_MULHWU as u32  => handle_mulhwu(ctx),
        x if x == PpcInsn::PPC_INS_MULLD as u32   => handle_mulld(ctx),
        x if x == PpcInsn::PPC_INS_MULLI as u32   => handle_mulli(ctx),
        x if x == PpcInsn::PPC_INS_MULLW as u32   => handle_mullw(ctx),
        x if x == PpcInsn::PPC_INS_NAND as u32    => handle_nand(ctx),
        x if x == PpcInsn::PPC_INS_NEG as u32     => handle_neg(ctx),
        x if x == PpcInsn::PPC_INS_NOP as u32     => handle_nop(ctx),
        x if x == PpcInsn::PPC_INS_NOR as u32     => handle_nor(ctx),

        x if x == PpcInsn::PPC_INS_NOT as u32    => handle_not(ctx),
        x if x == PpcInsn::PPC_INS_OR as u32     => handle_or(ctx),
        x if x == PpcInsn::PPC_INS_ORC as u32    => handle_orc(ctx),
        x if x == PpcInsn::PPC_INS_ORI as u32    => handle_ori(ctx),
        x if x == PpcInsn::PPC_INS_ORIS as u32   => handle_oris(ctx),

        x if x == PpcInsn::PPC_INS_RLDICL as u32 => handle_rldicl(ctx),
        x if x == PpcInsn::PPC_INS_RLDICR as u32 => handle_rldicr(ctx),
        x if x == PpcInsn::PPC_INS_RLDIMI as u32 => handle_rldimi(ctx),
        x if x == PpcInsn::PPC_INS_RLWIMI as u32 => handle_rlwimi(ctx),
        x if x == PpcInsn::PPC_INS_RLWINM as u32 => handle_rlwinm(ctx),
        x if x == PpcInsn::PPC_INS_ROTLDI as u32 => handle_rotldi(ctx),
        x if x == PpcInsn::PPC_INS_ROTLW as u32  => handle_rotlw(ctx),
        x if x == PpcInsn::PPC_INS_ROTLWI as u32 => handle_rotlwi(ctx),

        x if x == PpcInsn::PPC_INS_SLD as u32    => handle_sld(ctx),
        x if x == PpcInsn::PPC_INS_SLDI as u32   => handle_sldi(ctx),
        x if x == PpcInsn::PPC_INS_SLW as u32    => handle_slw(ctx),
        x if x == PpcInsn::PPC_INS_SLWI as u32   => handle_slwi(ctx),
        x if x == PpcInsn::PPC_INS_SRAD as u32   => handle_srad(ctx),
        x if x == PpcInsn::PPC_INS_SRADI as u32  => handle_sradi(ctx),
        x if x == PpcInsn::PPC_INS_SRAW as u32   => handle_sraw(ctx),
        x if x == PpcInsn::PPC_INS_SRAWI as u32  => handle_srawi(ctx),
        x if x == PpcInsn::PPC_INS_SRD as u32    => handle_srd(ctx),
        x if x == PpcInsn::PPC_INS_SRW as u32    => handle_srw(ctx),
        x if x == PpcInsn::PPC_INS_SRWI as u32   => handle_srwi(ctx),

        x if x == PpcInsn::PPC_INS_STB as u32    => handle_stb(ctx),
        x if x == PpcInsn::PPC_INS_STBU as u32   => handle_stbu(ctx),
        x if x == PpcInsn::PPC_INS_STBX as u32   => handle_stbx(ctx),
        x if x == PpcInsn::PPC_INS_STD as u32    => handle_std(ctx),
        x if x == PpcInsn::PPC_INS_STDCX as u32  => handle_stdcx(ctx),
        x if x == PpcInsn::PPC_INS_STDU as u32   => handle_stdu(ctx),
        x if x == PpcInsn::PPC_INS_STDX as u32   => handle_stdx(ctx),
        x if x == PpcInsn::PPC_INS_STFD as u32   => handle_stfd(ctx),
        x if x == PpcInsn::PPC_INS_STFDX as u32  => handle_stfdx(ctx),
        x if x == PpcInsn::PPC_INS_STFIWX as u32 => handle_stfiwx(ctx),
        x if x == PpcInsn::PPC_INS_STFS as u32   => handle_stfs(ctx),
        x if x == PpcInsn::PPC_INS_STFSX as u32  => handle_stfsx(ctx),
        x if x == PpcInsn::PPC_INS_STH as u32    => handle_sth(ctx),
        x if x == PpcInsn::PPC_INS_STHBRX as u32 => handle_sthbrx(ctx),
        x if x == PpcInsn::PPC_INS_STHX as u32   => handle_sthx(ctx),
        x if x == PpcInsn::PPC_INS_STVEHX as u32 => handle_stvehx(ctx),

        x if x == PpcInsn::PPC_INS_VCMPEQUD as u32
          || has_mn(ctx, "vcmpequd") =>
        {
            if let Some((_xo, vd, ra, rb)) = ctx.vmx128_decode() {
                // lvx128 / lvewx128 alias
                return emit_lvewx_at(ctx, vd, ra, rb);
            }
            false
        }

        x if x == PpcInsn::PPC_INS_VCMPNEB as u32
          || has_mn(ctx, "vcmpneb") =>
        {
            // Xenon: VCMPNEB* is always a VMX128 LVLX alias.
            if let Some((_xo, vd, ra, rb)) = ctx.vmx128_decode() {
                // lvlx128 v[vd], r[ra], r[rb]
                return emit_lvlx_at(ctx, vd, ra, rb);
            }
            // If this ever fires, something is *really* weird; keep it visible.
            false
        }

        x if x == PpcInsn::PPC_INS_VCMPNEH as u32
          || has_mn(ctx, "vcmpneh") =>
        {
            // Xenon: VCMPNEH* is always a VMX128 LVRX alias.
            if let Some((_xo, vd, ra, rb)) = ctx.vmx128_decode() {
                // lvrx128 v[vd], r[ra], r[rb]
                return emit_lvrx_at(ctx, vd, ra, rb);
            }
            false
        }

        x if x == PpcInsn::PPC_INS_VCMPNEZB as u32
          || has_mn(ctx, "vcmpnezb") =>
        {
            // Xenon: VCMPNEZB* is always a VMX128 STVLX alias.
            if let Some((_xo, vd, ra, rb)) = ctx.vmx128_decode() {
                // stvlx128 v[vd], r[ra], r[rb]
                return emit_stvlx_at(ctx, vd, ra, rb);
            }
            false
        }

        x if x == PpcInsn::PPC_INS_VCMPNEZH as u32
          || has_mn(ctx, "vcmpnezh") =>
        {
            // Xenon: VCMPNEZH* is always a VMX128 STVRX alias.
            if let Some((_xo, vd, ra, rb)) = ctx.vmx128_decode() {
                // stvrx128 v[vd], r[ra], r[rb]
                return emit_stvrx_at(ctx, vd, ra, rb);
            }
            false
        }

        _ if has_any_mn(ctx, &["stvewx", "stvewx128"]) => handle_stvewx_like(ctx),
        _ if has_any_mn(ctx, &["stvlx", "stvlx128"])   => handle_stvlx_like(ctx),
        _ if has_any_mn(ctx, &["stvrx", "stvrx128"])   => handle_stvrx_like(ctx),

        x if x == PpcInsn::PPC_INS_STVX as u32
          || has_any_mn(ctx, &["stvx", "stvx128"]) => handle_stvx_like(ctx),

        x if x == PpcInsn::PPC_INS_STW as u32    => handle_stw(ctx),
        x if x == PpcInsn::PPC_INS_STWBRX as u32 => handle_stwbrx(ctx),
        x if x == PpcInsn::PPC_INS_STWCX as u32  => handle_stwcx(ctx),
        x if x == PpcInsn::PPC_INS_STWU as u32   => handle_stwu(ctx),
        x if x == PpcInsn::PPC_INS_STWUX as u32  => handle_stwux(ctx),
        x if x == PpcInsn::PPC_INS_STWX as u32   => handle_stwx(ctx),

        x if x == PpcInsn::PPC_INS_SUBF as u32   => handle_subf(ctx),
        x if x == PpcInsn::PPC_INS_SUBFC as u32  => handle_subfc(ctx),
        x if x == PpcInsn::PPC_INS_SUBFE as u32  => handle_subfe(ctx),
        x if x == PpcInsn::PPC_INS_SUBFIC as u32 => handle_subfic(ctx),
        x if x == PpcInsn::PPC_INS_SYNC as u32   => handle_sync(ctx),

        x if x == PpcInsn::PPC_INS_TDLGEI as u32
          || has_mn(ctx, "tdlgei")      => handle_tdlgei(ctx),

        x if x == PpcInsn::PPC_INS_TDLLEI as u32
          || has_mn(ctx, "tdllei")      => handle_tdllei(ctx),

        x if x == PpcInsn::PPC_INS_TDI as u32
          || has_mn(ctx, "tdi")         => handle_tdi(ctx),

        x if x == PpcInsn::PPC_INS_TWI as u32
          || has_mn(ctx, "twi")         => handle_twi(ctx),

        x if x == PpcInsn::PPC_INS_TWLGEI as u32
          || has_mn(ctx, "twlgei")      => handle_twlgei(ctx),

        x if x == PpcInsn::PPC_INS_TWLLEI as u32
          || has_mn(ctx, "twllei")      => handle_twllei(ctx),

        x if x == PpcInsn::PPC_INS_TWUI as u32
          || has_mn(ctx, "twui")        => handle_twui(ctx),

        x if x == PpcInsn::PPC_INS_VADDFP as u32 || has_mn(ctx, "vaddfp128") => handle_vaddfp_like(ctx),
        x if x == PpcInsn::PPC_INS_VADDSHS as u32   => handle_vaddshs(ctx),
        x if x == PpcInsn::PPC_INS_VADDUBM as u32   => handle_vaddubm(ctx),
        x if x == PpcInsn::PPC_INS_VADDUBS as u32   => handle_vaddubs(ctx),
        x if x == PpcInsn::PPC_INS_VADDUHM as u32   => handle_vadduhm(ctx),
        x if x == PpcInsn::PPC_INS_VADDUWM as u32   => handle_vadduwm(ctx),
        x if x == PpcInsn::PPC_INS_VADDUWS as u32   => handle_vadduws(ctx),

        x if x == PpcInsn::PPC_INS_VAND as u32 || has_mn(ctx, "vand128") => handle_vand_like(ctx),
        _ if has_mn(ctx, "vandc128") => handle_vandc128(ctx),

        x if x == PpcInsn::PPC_INS_VAVGSB as u32    => handle_vavgsb(ctx),
        x if x == PpcInsn::PPC_INS_VAVGSH as u32    => handle_vavgsh(ctx),
        x if x == PpcInsn::PPC_INS_VAVGUB as u32    => handle_vavgub(ctx),

        _ if has_mn(ctx, "vcfpsxws128") => handle_vctsx_like(ctx),

        x if x == PpcInsn::PPC_INS_VCFSX as u32
          || has_mn(ctx, "vcfsx")
          || has_mn(ctx, "vcsxwfp128")  => handle_vcfsx_like(ctx),

        x if x == PpcInsn::PPC_INS_VCFUX as u32
          || has_mn(ctx, "vcuxwfp128")  => handle_vcfux_like(ctx),

        x if x == PpcInsn::PPC_INS_VCMPBFP as u32 || has_mn(ctx, "vcmpbfp128") => handle_vcmpbfp_like(ctx),
        x if x == PpcInsn::PPC_INS_VCMPEQFP as u32 || has_mn(ctx, "vcmpeqfp128") => handle_vcmpeqfp_like(ctx),

        x if x == PpcInsn::PPC_INS_VCMPEQUB as u32   => handle_vcmpequb(ctx),
        x if x == PpcInsn::PPC_INS_VCMPEQUW as u32 || has_mn(ctx, "vcmpequw128") => handle_vcmpequw_like(ctx),
        x if x == PpcInsn::PPC_INS_VCMPGEFP as u32 || has_mn(ctx, "vcmpgefp128") => handle_vcmpgefp_like(ctx),
        x if x == PpcInsn::PPC_INS_VCMPGTFP as u32 || has_mn(ctx, "vcmpgtfp128") => handle_vcmpgtfp_like(ctx),

        x if x == PpcInsn::PPC_INS_VCMPGTUB as u32   => handle_vcmpgtub(ctx),
        x if x == PpcInsn::PPC_INS_VCMPGTUH as u32   => handle_vcmpgtuh(ctx),

        x if x == PpcInsn::PPC_INS_VEXPTEFP as u32 || has_mn(ctx, "vexptefp128") => handle_vexptefp_like(ctx),
        x if x == PpcInsn::PPC_INS_VLOGEFP as u32  || has_mn(ctx, "vlogefp128")  => handle_vlogefp_like(ctx),

        _ if has_any_mn(ctx, &["vmaddfp", "vmaddfp128", "vmaddcfp128"]) => handle_vmaddfp_like(ctx),

        x if x == PpcInsn::PPC_INS_VMAXFP as u32 || has_mn(ctx, "vmaxfp128") => handle_vmaxfp_like(ctx),
        x if x == PpcInsn::PPC_INS_VMAXSW as u32     => handle_vmaxsw(ctx),

        x if x == PpcInsn::PPC_INS_VMINFP as u32 || has_mn(ctx, "vminfp128") => handle_vminfp_like(ctx),

        x if x == PpcInsn::PPC_INS_VMRGHB as u32     => handle_vmrg_hilo_common(ctx, "epi8",  true),
        x if x == PpcInsn::PPC_INS_VMRGHH as u32     => handle_vmrg_hilo_common(ctx, "epi16", true),
        x if x == PpcInsn::PPC_INS_VMRGHW as u32 || has_mn(ctx, "vmrghw128") => handle_vmrg_hilo_common(ctx, "epi32", true),

        x if x == PpcInsn::PPC_INS_VMRGLB as u32     => handle_vmrg_hilo_common(ctx, "epi8",  false),
        x if x == PpcInsn::PPC_INS_VMRGLH as u32     => handle_vmrg_hilo_common(ctx, "epi16", false),
        x if x == PpcInsn::PPC_INS_VMRGLW as u32 || has_mn(ctx, "vmrglw128") => handle_vmrg_hilo_common(ctx, "epi32", false),

        _ if has_mn(ctx, "vmsum3fp128") => handle_vmsum3fp128(ctx),
        _ if has_mn(ctx, "vmsum4fp128") => handle_vmsum4fp128(ctx),
        _ if has_mn(ctx, "vmulfp128")   => handle_vmulfp128(ctx),

        x if x == PpcInsn::PPC_INS_VNMSUBFP as u32
          || has_mn(ctx, "vnmsubfp128") => handle_vnmsubfp_like(ctx),

        x if x == PpcInsn::PPC_INS_VOR as u32
          || has_mn(ctx, "vor128")      => handle_vor_like(ctx),

        x if x == PpcInsn::PPC_INS_VMR as u32
          || has_mn(ctx, "vmr")         => handle_vmr(ctx),

        x if x == PpcInsn::PPC_INS_VPERM as u32
          || has_mn(ctx, "vperm128")    => handle_vperm_like(ctx),

        _ if has_mn(ctx, "vpermwi128")  => handle_vpermwi128(ctx),
        _ if has_mn(ctx, "vpkd3d128")   => handle_vpkd3d128(ctx),

        x if x == PpcInsn::PPC_INS_VPKSHUS as u32
          || has_mn(ctx, "vpkshus128")         => handle_vpkshus_like(ctx),

        x if x == PpcInsn::PPC_INS_VREFP as u32
          || has_mn(ctx, "vrefp128")           => handle_vrefp_like(ctx),

        x if x == PpcInsn::PPC_INS_VRFIM as u32
          || has_mn(ctx, "vrfim128")           => handle_vrfi_round(ctx, "neg"),
        x if x == PpcInsn::PPC_INS_VRFIN as u32
          || has_mn(ctx, "vrfin128")           => handle_vrfi_round(ctx, "nearest"),
        x if x == PpcInsn::PPC_INS_VRFIZ as u32
          || has_mn(ctx, "vrfiz128")           => handle_vrfi_round(ctx, "zero"),

        _ if has_mn(ctx, "vrlimi128")          => handle_vrlimi128(ctx),

        x if x == PpcInsn::PPC_INS_VRSQRTEFP as u32
          || has_mn(ctx, "vrsqrtefp128")       => handle_vrsqrtefp_like(ctx),

        x if x == PpcInsn::PPC_INS_VSEL as u32               => handle_vsel(ctx),

        x if x == PpcInsn::PPC_INS_VSLB as u32               => handle_vslb_scalar(ctx),

        x if x == PpcInsn::PPC_INS_VSLDOI as u32
          || has_mn(ctx, "vsldoi128")          => handle_vsldoi_like(ctx),

        x if x == PpcInsn::PPC_INS_VSLW as u32
          || has_mn(ctx, "vslw128")            => handle_vslw_scalar(ctx),

        x if x == PpcInsn::PPC_INS_VSPLTB as u32             => handle_vspltb(ctx),
        x if x == PpcInsn::PPC_INS_VSPLTH as u32             => handle_vsplth(ctx),
        x if x == PpcInsn::PPC_INS_VSPLTISB as u32           => handle_vspltisb(ctx),

        x if x == PpcInsn::PPC_INS_VSPLTISH as u32
          || has_mn(ctx, "vspltish")        => handle_vspltish(ctx),

        x if x == PpcInsn::PPC_INS_VSPLTISW as u32
          || has_mn(ctx, "vspltisw128")        => handle_vspltisw_like(ctx),

        x if x == PpcInsn::PPC_INS_VSPLTW as u32
          || has_mn(ctx, "vspltw128")          => handle_vspltw_like(ctx),

        x if x == PpcInsn::PPC_INS_VSR as u32                => handle_vsr(ctx),

        x if x == PpcInsn::PPC_INS_VSRAW as u32
          || has_mn(ctx, "vsraw128")           => handle_vsraw_scalar(ctx),

        x if x == PpcInsn::PPC_INS_VSRW as u32
          || has_mn(ctx, "vsrw128")            => handle_vsrw_scalar(ctx),

        x if x == PpcInsn::PPC_INS_VSUBFP as u32
          || has_mn(ctx, "vsubfp128")          => handle_vsubfp_like(ctx),

        x if x == PpcInsn::PPC_INS_VSUBSWS as u32            => handle_vsubsws_scalar(ctx),

        x if x == PpcInsn::PPC_INS_VSUBUBS as u32            => handle_vsububs(ctx),
        x if x == PpcInsn::PPC_INS_VSUBUHM as u32            => handle_vsubuhm(ctx),

        _ if has_mn(ctx, "vupkd3d128")         => handle_vupkd3d128(ctx),

        x if x == PpcInsn::PPC_INS_VUPKHSB as u32
          || has_mn(ctx, "vupkhsb128")         => handle_vupkhsb_like(ctx),

        x if x == PpcInsn::PPC_INS_VUPKHSH as u32
          || has_mn(ctx, "vupkhsh128")         => handle_vupkhsh_like(ctx),

        x if x == PpcInsn::PPC_INS_VUPKLSB as u32
          || has_mn(ctx, "vupklsb128")         => handle_vupklsb_like(ctx),

        x if x == PpcInsn::PPC_INS_VUPKLSH as u32
          || has_mn(ctx, "vupklsh128")         => handle_vupklsh_like(ctx),

        x if x == PpcInsn::PPC_INS_VXOR as u32
          || has_mn(ctx, "vxor128")            => handle_vxor_like(ctx),

        x if x == PpcInsn::PPC_INS_XOR as u32                => handle_xor_scalar(ctx),
        x if x == PpcInsn::PPC_INS_XORI as u32               => handle_xori_scalar(ctx, false),
        x if x == PpcInsn::PPC_INS_XORIS as u32              => handle_xori_scalar(ctx, true),

        _ => false,
    }
}

/// Public entry used by the main recompile loop.
/// Returns `true` if the instruction was recognized/emitted.
pub fn lower_one(
    rec: &mut Recompiler,
    fnc: &Function,
    base: u32,
    be_word: u32,
    insn: &Insn<'_>,
    locals: &mut RecompilerLocalVariables,
    csr: &mut CSRState,
    next_be_word: Option<u32>,
    cs: &Capstone,
    block_terminated: &mut bool,
) -> bool {
    let mmio = next_be_word.map_or(false, |n| n == Recompiler::C_EIEIO);

    let mut ctx = LowerCtx {
        rec,
        fnc,
        base,
        be_word,
        insn,
        cs,
        locals,
        csr,
        mmio,
        block_terminated,
    };

    ctx.emit_mid_asm_hook_if_any(false);
    ctx.emit_disasm_comment();

    let ok = dispatch_by_id(&mut ctx);

    if !ok {
        // Existing MMIO heuristic
        if let Some(nxt) = next_be_word {
            let _mmio = ctx.is_mmio_store(nxt);
            let _ = _mmio;
        }

        // NEW: record this as an unimplemented instruction.
        let id = id_u32(insn);
        let mnemonic = insn.mnemonic().unwrap_or("unknown");
        ctx.rec
            .note_unhandled_insn(id, mnemonic, insn.address() as u32);
    }

    ctx.emit_mid_asm_hook_if_any(true);
    ok
}
