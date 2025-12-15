// src/recompiler/templates.rs

// Canonical copies of the PPC context and runtime source, embedded at compile time.
// These paths are relative to this file: src/recompiler/.
pub const PPC_CONTEXT_RS: &str = include_str!("ppc_context.rs");
pub const PPC_RT_RS: &str       = include_str!("rt.rs");
