// src/log.rs
#![allow(unused_macros)]
use std::time::{Duration, Instant};

#[inline] pub fn enabled() -> bool { true } // <-- always on now

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
        eprintln!($($arg)*);
    }}
}
