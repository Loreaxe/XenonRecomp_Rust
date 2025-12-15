// src/recompiler/emit/mod.rs

pub mod helpers;
pub mod fn_emit;
pub mod batch;
pub mod mapping;
pub mod workspace;

// No re-exports needed: each submodule only adds inherent impl blocks
// for `Recompiler`, so everything is available as `self.method(...)`.
