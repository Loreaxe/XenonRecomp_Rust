// lib.rs
pub mod config;
pub mod disasm;
pub mod function;
pub mod image;
pub mod ppc;
pub mod recompiler;
pub mod switch;
pub mod db;
pub mod pipeline;
pub mod xex;
pub mod lzx;
pub mod log;

// re-exports (optional)
pub use config::*;
pub use disasm::*;
pub use function::*;
pub use image::*;
pub use switch::*;

pub mod passes {
    pub mod analyse_function;     // <-- matches src/passes/analyse_function.rs
    pub mod analyse_switch_scan;  // <-- matches src/passes/analyse_switch_scan.rs
    pub mod analyse_switch_bind;
}