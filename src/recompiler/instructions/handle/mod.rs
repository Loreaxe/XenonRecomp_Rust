// src/recompiler/instructions/handle/mod.rs

#![allow(clippy::needless_return)]
#![allow(clippy::useless_format)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

// One module per handler family
pub mod arith;
pub mod logical;
pub mod branch;
pub mod compare;
pub mod bitcount;
pub mod cache;
pub mod divide;
pub mod system;
pub mod extend;
pub mod float_ops;
pub mod loads;
pub mod loads_vec;
pub mod stores;
pub mod stores_vec;
pub mod moves_spr;
pub mod rotate;
pub mod shift;
pub mod vector_arith;
pub mod vector_cmp;
pub mod vector_pack;
pub mod vector_perm;
pub mod vector_shift;
pub mod vector_mem;
pub mod vector_misc;

pub(crate) use crate::recompiler::instructions::{
    LowerCtx,
    compute_mask,
    handle_link_if_needed,
};