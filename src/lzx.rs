// src/lzx.rs
#![allow(non_camel_case_types, dead_code)]

use core::ffi::c_void;
use core::{cmp, ptr, slice};
use libc::{c_int, c_long, size_t};

// -------- libmspack FFI (subset we need) --------

// mspack "file" is just an opaque handle passed to our callbacks.
#[repr(C)]
pub struct mspack_file {
    _priv: [u8; 0],
    _pad:  [u8; 0],
}

// Function pointer types in mspack_system (match <mspack.h> order/signatures)
type mspack_open  = Option<unsafe extern "C" fn(_: *mut mspack_system, _: *const i8, _: c_int) -> *mut mspack_file>;
type mspack_close = Option<unsafe extern "C" fn(_: *mut mspack_file)>;
type mspack_read  = Option<unsafe extern "C" fn(_: *mut mspack_file, _: *mut c_void, _: c_int) -> c_int>;
type mspack_write = Option<unsafe extern "C" fn(_: *mut mspack_file, _: *mut c_void, _: c_int) -> c_int>;
type mspack_seek  = Option<unsafe extern "C" fn(_: *mut mspack_file, _: c_long, _: c_int) -> c_long>;
type mspack_tell  = Option<unsafe extern "C" fn(_: *mut mspack_file) -> c_long>;
type mspack_msg   = Option<unsafe extern "C" fn(_: *mut mspack_system, _: *const i8, ...)>;
type mspack_alloc = Option<unsafe extern "C" fn(_: *mut mspack_system, _: size_t) -> *mut c_void>;
type mspack_free  = Option<unsafe extern "C" fn(_: *mut c_void)>;
type mspack_copy  = Option<unsafe extern "C" fn(_: *mut c_void, _: *mut c_void, _: size_t)>;

#[repr(C)]
pub struct mspack_system {
    pub open:  mspack_open,
    pub close: mspack_close,
    pub read:  mspack_read,
    pub write: mspack_write,
    pub seek:  mspack_seek,
    pub tell:  mspack_tell,
    pub message: mspack_msg,
    pub alloc: mspack_alloc,
    pub free:  mspack_free,
    pub copy:  mspack_copy,
}

// lzx.h (libmspack)
#[repr(C)]
pub struct lzxd_stream {
    pub window: *mut u8,         // used for reference prefill
    pub window_size: c_int,      // ..and size
    pub ref_data_size: c_int,    // you set this when using reference window
    // rest is private; we never touch it
}

extern "C" {
    fn lzxd_init(
        sys: *mut mspack_system,
        input: *mut mspack_file,
        output: *mut mspack_file,
        window_bits: c_int,
        reset_interval: c_int,
        buffer: c_int,
        length: c_long,
        _unused: c_int,
    ) -> *mut lzxd_stream;

    fn lzxd_decompress(stream: *mut lzxd_stream, out_bytes: c_long) -> c_int;
    fn lzxd_free(stream: *mut lzxd_stream);
}

// -------- In-memory "files" for mspack to read/write --------

#[repr(C)]
struct MemoryFile {
    buf: *mut u8,
    len: usize,
    off: usize,
}

unsafe extern "C" fn mem_read(f: *mut mspack_file, dst: *mut c_void, chars: c_int) -> c_int {
    let mf = f as *mut MemoryFile;
    if mf.is_null() { return 0; }
    let mf = &mut *mf;
    if chars <= 0 { return 0; }
    let want = chars as usize;
    let remaining = mf.len.saturating_sub(mf.off);
    let take = cmp::min(remaining, want);
    if take > 0 {
        ptr::copy_nonoverlapping(mf.buf.add(mf.off), dst as *mut u8, take);
        mf.off += take;
    }
    take as c_int
}

unsafe extern "C" fn mem_write(f: *mut mspack_file, src: *mut c_void, chars: c_int) -> c_int {
    let mf = f as *mut MemoryFile;
    if mf.is_null() { return 0; }
    let mf = &mut *mf;
    if chars <= 0 { return 0; }
    let want = chars as usize;
    let remaining = mf.len.saturating_sub(mf.off);
    let take = cmp::min(remaining, want);
    if take > 0 {
        ptr::copy_nonoverlapping(src as *const u8, mf.buf.add(mf.off), take);
        mf.off += take;
    }
    take as c_int
}

unsafe extern "C" fn mem_alloc(_sys: *mut mspack_system, bytes: size_t) -> *mut c_void {
    // calloc semantics expected
    let n = bytes as usize;
    let p = libc::calloc(1, n);
    p
}

unsafe extern "C" fn mem_free(p: *mut c_void) {
    if !p.is_null() { libc::free(p); }
}

unsafe extern "C" fn mem_copy(src: *mut c_void, dst: *mut c_void, n: size_t) {
    ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n as usize);
}

// Unused by our flow
unsafe extern "C" fn mem_open(_: *mut mspack_system, _: *const i8, _: c_int) -> *mut mspack_file { ptr::null_mut() }
unsafe extern "C" fn mem_close(_: *mut mspack_file) {}
unsafe extern "C" fn mem_seek(_: *mut mspack_file, _: c_long, _: c_int) -> c_long { -1 }
unsafe extern "C" fn mem_tell(_: *mut mspack_file) -> c_long { -1 }

// -------- Public Rust API --------

#[inline]
fn bit_scan_forward_u32(v: u32) -> Option<u32> {
    if v == 0 { None } else { Some(v.trailing_zeros()) }
}

/// Rust reimplementation of the old C++ bridge.
/// Returns 0 on success (matches your X_STATUS_SUCCESS).
pub fn lzx_decompress(
    lzx_data: *const u8,
    lzx_len: usize,
    dst: *mut u8,
    dst_len: usize,
    window_size: u32,
    window_data: *const u8,
    window_data_len: usize,
) -> i32 {
    // Quick param validation
    if lzx_data.is_null() || dst.is_null() { return 1; }

    let window_bits = match bit_scan_forward_u32(window_size) {
        Some(b) => b as c_int,
        None => return 1,
    };

    // Build a mspack_system with our in-memory callbacks
    let mut sys = mspack_system {
        open:  Some(mem_open),
        close: Some(mem_close),
        read:  Some(mem_read),
        write: Some(mem_write),
        seek:  Some(mem_seek),
        tell:  Some(mem_tell),
        message: None,
        alloc: Some(mem_alloc),
        free:  Some(mem_free),
        copy:  Some(mem_copy),
    };

    // "files"
    let mut _src = MemoryFile { buf: lzx_data as *mut u8, len: lzx_len, off: 0 };
    let mut _dst = MemoryFile { buf: dst,                  len: dst_len, off: 0 };

    // Safety: libmspack will only pass these pointers back to our callbacks
    let src_file = &_src as *const MemoryFile as *mut mspack_file;
    let dst_file = &_dst as *const MemoryFile as *mut mspack_file;

    let stream = unsafe {
        lzxd_init(
            &mut sys,
            src_file,
            dst_file,
            window_bits,
            0,
            0x8000,
            dst_len as c_long,
            0,
        )
    };

    if stream.is_null() {
        return 1;
    }

    // Optional reference prefill, same behavior as the C++ version
    if !window_data.is_null() && window_data_len > 0 {
        unsafe {
            let s = &mut *stream;
            let win = slice::from_raw_parts_mut(s.window, s.window_size as usize);
            let pad = (s.window_size as usize).saturating_sub(window_data_len);
            win[..pad].fill(0);
            ptr::copy_nonoverlapping(window_data, win.as_mut_ptr().add(pad), window_data_len);
            s.ref_data_size = s.window_size; // mark full window as ref data
        }
    }

    let rc = unsafe { lzxd_decompress(stream, dst_len as c_long) };
    unsafe { lzxd_free(stream) };

    rc
}

// -------- Optional: export a C symbol (not needed if only called from Rust) --------
#[no_mangle]
pub extern "C" fn rex_lzx_decompress(
    lzx_data: *const u8,
    lzx_len: usize,
    dst: *mut u8,
    dst_len: usize,
    window_size: u32,
    window_data: *mut c_void,
    window_data_len: usize,
) -> i32 {
    lzx_decompress(
        lzx_data,
        lzx_len,
        dst,
        dst_len,
        window_size,
        window_data as *const u8,
        window_data_len,
    )
}

#[no_mangle]
pub extern "C" fn lzxDecompress(
    lzx_data: *const u8,
    lzx_len: usize,
    dst: *mut u8,
    dst_len: usize,
    window_size: u32,
    window_data: *const u8,
    window_data_len: usize,
) -> i32 {
    lzx_decompress(
        lzx_data,
        lzx_len,
        dst,
        dst_len,
        window_size,
        window_data,
        window_data_len,
    )
}