// src/log.rs
#![allow(unused_macros)]
use std::time::{Duration, Instant};

#[inline]
pub fn enabled() -> bool { true } // top-level "logging on" switch if you ever want it

// Env-gated levels:
//   XENON_RECOMP_DEBUG  → enables xdebug! (and you still get xlog!)
//   XENON_RECOMP_TRACE  → enables xtrace! (super noisy stuff)
#[inline]
pub fn debug_enabled() -> bool {
    std::env::var_os("XENON_RECOMP_DEBUG").is_some()
}

#[inline]
pub fn trace_enabled() -> bool {
    std::env::var_os("XENON_RECOMP_TRACE").is_some()
}

/// Lightweight phase timer. Prints on Drop.
pub struct Phase {
    name: &'static str,
    start: Instant,
}
impl Phase {
    pub fn new(name: &'static str) -> Self {
        eprintln!("[{}] ▶ start", name);
        Self { name, start: Instant::now() }
    }
    fn elapsed(&self) -> Duration { self.start.elapsed() }
}
impl Drop for Phase {
    fn drop(&mut self) {
        let ms = self.elapsed().as_secs_f64() * 1000.0;
        eprintln!("[{}] ◀ done in {:.2} ms", self.name, ms);
    }
}

#[macro_export]
macro_rules! xlog {
    ($($arg:tt)*) => {{
        // if you ever want a global off-switch, check log::enabled() here
        eprintln!($($arg)*);
    }}
}

#[macro_export]
macro_rules! xdebug {
    ($($arg:tt)*) => {{
        if $crate::log::debug_enabled() {
            eprintln!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! xtrace {
    ($($arg:tt)*) => {{
        if $crate::log::trace_enabled() {
            eprintln!($($arg)*);
        }
    }};
}
