// src/recompiler/instructions/mod.rs

#![allow(clippy::needless_return)]
#![allow(clippy::useless_format)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

mod ctx;
mod dispatch;

pub mod handle;

// Make helpers visible to sibling modules inside this crate.
// (They are not part of the public API.)
pub(crate) use ctx::{
    LowerCtx,
    has_mn,
    has_any_mn,
    compute_mask,
    handle_link_if_needed,
    id_u32,
};

// Public entry point used by the main recompile loop.
pub use dispatch::lower_one;
