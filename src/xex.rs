// src/xex.rs
#![allow(clippy::needless_return)]

use anyhow::*;
use byteorder::{BigEndian as BE, LittleEndian as LE, ByteOrder};
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use aes::Aes128;
use sha1::{Sha1, Digest};
use crate::recompiler::symbol::build_export_maps;

use crate::image::{Image, Section, SectionFlags};
use crate::log::Phase;
use crate::xlog;

// ------------------------------ externs ------------------------------
use crate::lzx::lzxDecompress;

// ------------------------------ constants ------------------------------
const XEX_MAGIC: &[u8; 4] = b"XEX2";
const XEX_ENCRYPTION_NONE:   u16 = 0;
const XEX_ENCRYPTION_NORMAL: u16 = 1;

const XEX_COMPRESSION_NONE:   u16 = 0;
const XEX_COMPRESSION_BASIC:  u16 = 1;
const XEX_COMPRESSION_NORMAL: u16 = 2;

const IMAGE_SCN_CNT_CODE: u32 = 0x0000_0020;

// Opt header keys (subset)
const XEX_HEADER_FILE_FORMAT_INFO:   u32 = 0x0000_03FF;
const XEX_HEADER_IMAGE_BASE_ADDRESS: u32 = 0x0001_0201;
const XEX_HEADER_ENTRY_POINT:        u32 = 0x0001_0100;
const XEX_HEADER_IMPORT_LIBRARIES:   u32 = 0x0001_03FF;

// Retail key / blank IV
const XEX2_RETAIL_KEY: [u8; 16] = [
    0x20, 0xB1, 0x85, 0xA5, 0x9D, 0x28, 0xFD, 0xC3,
    0x40, 0x58, 0x3F, 0xBB, 0x08, 0x96, 0xBF, 0x91
];
const BLANK_IV: [u8; 16] = [0u8; 16];

type Aes128CbcDec = cbc::Decryptor<Aes128>;

// ------------------------------ tiny readers ------------------------------

fn rd_be_u16(b: &[u8], off: usize) -> Result<u16> {
    ensure!(off + 2 <= b.len(), "OOR u16");
    Ok(BE::read_u16(&b[off..off + 2]))
}
fn rd_be_u32(b: &[u8], off: usize) -> Result<u32> {
    ensure!(off + 4 <= b.len(), "OOR u32");
    Ok(BE::read_u32(&b[off..off + 4]))
}

// XEX2 header (subset)
struct Xex2Header {
    module_flags: u32,
    header_size:  u32,
    security_off: u32,
    header_count: u32,
}
fn parse_xex2_header(x: &[u8]) -> Result<Xex2Header> {
    ensure!(x.len() >= 0x18, "xex too small");
    ensure!(&x[0..4] == XEX_MAGIC, "bad XEX2 magic");
    let _reserved = rd_be_u32(x, 0x0C)?; // explicit read

    Ok(Xex2Header {
        module_flags:  rd_be_u32(x, 0x04)?,
        header_size:   rd_be_u32(x, 0x08)?,
        security_off:  rd_be_u32(x, 0x10)?,
        header_count:  rd_be_u32(x, 0x14)?,
    })
}

// Security info (subset) -- match C++ struct offsets exactly
struct Xex2SecurityInfo {
    pub image_size:  u32,
    pub load_addr:   u32,
    pub aes_key_be:  [u8; 16],
}

fn parse_security_info(x: &[u8], off: usize) -> Result<Xex2SecurityInfo> {
    // We need up to aesKey (0x150) + 16 bytes
    const AES_KEY_OFF: usize = 0x150;
    const LOAD_ADDR_OFF: usize = 0x110;

    ensure!(
        off + AES_KEY_OFF + 16 <= x.len(),
        "security OOR (need security header up to aesKey)"
    );

    // imageSize at +0x04
    let image_size = rd_be_u32(x, off + 0x04)?;

    // loadAddress at +0x110
    let load_addr  = rd_be_u32(x, off + LOAD_ADDR_OFF)?;

    // aesKey at +0x150 .. +0x160
    let mut aes_key_be = [0u8; 16];
    aes_key_be.copy_from_slice(&x[off + AES_KEY_OFF .. off + AES_KEY_OFF + 16]);

    Ok(Xex2SecurityInfo {
        image_size,
        load_addr,
        aes_key_be,
    })
}

// Optional headers table
#[derive(Clone, Copy)]
enum OptRef<'a> {
    InlineU32(u32),      // (key & 0xFF) == 0
    InlinePtr(&'a [u8]), // (key & 0xFF) == 1
    Blob(&'a [u8]),      // offset into module bytes
}
fn get_opt_ptr<'a>(x: &'a [u8], hdr: &Xex2Header, key: u32) -> Option<OptRef<'a>> {
    let table_off = 0x18;
    for i in 0..hdr.header_count as usize {
        let off = table_off + i * 8;
        if off + 8 > x.len() { break; }
        let k = BE::read_u32(&x[off..off + 4]);
        let v = BE::read_u32(&x[off + 4..off + 8]);
        if k == key {
            match key & 0xFF {
                0 => return Some(OptRef::InlineU32(v)),
                1 => {
                    let p = v as usize;
                    if p + 4 <= x.len() { return Some(OptRef::InlinePtr(&x[p..p + 4])); }
                    else { return None; }
                }
                _ => {
                    let p = v as usize;
                    if p <= x.len() { return Some(OptRef::Blob(&x[p..])); }
                    else { return None; }
                }
            }
        }
    }
    None
}

// File format info (be)
#[derive(Clone, Copy)]
struct FileFormatInfo {
    info_size: u32,
    encryption: u16,
    compression: u16,
}
fn parse_file_format_info(b: &[u8]) -> Result<FileFormatInfo> {
    ensure!(b.len() >= 8, "ffi too small");
    Ok(FileFormatInfo {
        info_size:   BE::read_u32(&b[0..4]),
        encryption:  BE::read_u16(&b[4..6]),
        compression: BE::read_u16(&b[6..8]),
    })
}

// Normal-compression block header
#[derive(Clone, Copy)]
struct NormalFirstBlock<'a> { window_size: u32, first_block: &'a [u8] }
fn parse_normal_info(b: &[u8]) -> Result<NormalFirstBlock<'_>> {
    ensure!(b.len() >= 4 + 24, "normal info too small");
    let window_size = BE::read_u32(&b[0..4]);
    let first_block = &b[4..];
    Ok(NormalFirstBlock { window_size, first_block })
}

// ---------------- PE helpers (LE) — FIXED OFFSETS + BOUNDS ----------------

fn pe_e_lfanew(img: &[u8]) -> Result<usize> {
    // DOS header must be at least 0x40 bytes, e_lfanew at 0x3C..0x40
    ensure!(img.len() >= 0x40, "PE: DOS header too small");
    let e_lfanew = LE::read_u32(&img[0x3C..0x40]) as usize;
    ensure!(e_lfanew + 4 <= img.len(), "PE: e_lfanew outside file");
    ensure!(&img[e_lfanew..e_lfanew + 4] == b"PE\0\0", "PE: bad signature");
    Ok(e_lfanew)
}

fn pe_num_sections(file_hdr: &[u8]) -> Result<usize> {
    // IMAGE_FILE_HEADER = 20 bytes
    ensure!(file_hdr.len() >= 20, "PE: file header too small");
    // offset 0x02..0x04 = NumberOfSections
    Ok(LE::read_u16(&file_hdr[0x02..0x04]) as usize)
}

fn pe_optional_header_size(file_hdr: &[u8]) -> Result<usize> {
    ensure!(file_hdr.len() >= 20, "PE: file header too small");
    // offset 0x10..0x12 = SizeOfOptionalHeader
    Ok(LE::read_u16(&file_hdr[0x10..0x12]) as usize)
}

fn pe_sections_slice(img: &[u8], nt_off: usize) -> Result<&[u8]> {
    // NT headers: [Signature(4)] [FileHeader(20)] [OptionalHeader(SizeOfOptionalHeader)] [Sections…]
    ensure!(nt_off + 4 + 20 <= img.len(), "PE: NT headers OOR");
    let file_hdr = &img[nt_off + 4 .. nt_off + 4 + 20];
    let oh_size = pe_optional_header_size(file_hdr)?;
    let sec_off = nt_off + 4 + 20 + oh_size;
    ensure!(sec_off <= img.len(), "PE: section table OOR");
    Ok(&img[sec_off..])
}

// Return (optional_header_slice, magic) where magic is 0x10B (PE32) or 0x20B (PE32+)
fn pe_optional_header(img: &[u8], nt_off: usize) -> Result<(&[u8], u16)> {
    ensure!(nt_off + 4 + 20 <= img.len(), "PE: NT headers OOR");
    let file_hdr = &img[nt_off + 4 .. nt_off + 4 + 20];
    let oh_size = pe_optional_header_size(file_hdr)?;
    let oh_off  = nt_off + 4 + 20;
    ensure!(oh_off + oh_size <= img.len(), "PE: optional header OOR");
    let oh = &img[oh_off .. oh_off + oh_size];
    ensure!(oh.len() >= 2, "PE: optional header too small");
    let magic = LE::read_u16(&oh[0..2]); // 0x10B (PE32) or 0x20B (PE32+)
    Ok((oh, magic))
}

fn pe_read_image_base(img: &[u8], nt_off: usize) -> Option<u32> {
    let (oh, magic) = pe_optional_header(img, nt_off).ok()?;
    match magic {
        // PE32: ImageBase at 0x34 (DWORD)
        0x10B => if oh.len() >= 0x38 { Some(LE::read_u32(&oh[0x34 .. 0x38])) } else { None },
        // PE32+: ImageBase at 0x30 (QWORD). Truncate; Xenon is 32-bit PPC anyway.
        0x20B => if oh.len() >= 0x38 { Some(LE::read_u32(&oh[0x30 .. 0x34])) } else { None },
        _ => None,
    }
}

fn pe_try_nt_headers_off(img: &[u8]) -> Option<usize> {
    // Need at least a DOS header.
    if img.len() < 0x40 {
        return None;
    }

    let e_lfanew = LE::read_u32(&img[0x3C..0x40]) as usize;

    // ---- First try using e_lfanew directly (the "normal" case) ----
    if e_lfanew + 4 + 20 <= img.len() && &img[e_lfanew..e_lfanew + 4] == b"PE\0\0" {
        return Some(e_lfanew);
    }

    // ---- Fallback: scan for a plausible "PE\0\0" signature ----
    // Some titles seem to have weird DOS/e_lfanew but still contain proper NT headers.
    // We scan the first 0x2000 bytes (or up to len) for "PE\0\0".
    let scan_limit = img.len().min(0x2000);

    for off in 0..scan_limit.saturating_sub(4) {
        if &img[off..off + 4] == b"PE\0\0" {
            // Make sure there's at least a FileHeader after the signature.
            if off + 4 + 20 <= img.len() {
                // Optional: you could log this to see when it happens.
                // xlog!("PE: using scanned NT header at offset 0x{:X} (e_lfanew=0x{:X} was invalid)", off, e_lfanew);
                return Some(off);
            }
        }
    }

    // Nothing that looks like a valid NT header → treat as flat image.
    None
}

// AES
fn aes128_cbc_decrypt_in_place(key: &[u8; 16], iv: &[u8; 16], buf: &mut [u8]) {
    let decryptor = Aes128CbcDec::new_from_slices(key, iv).expect("aes init");
    let _ = decryptor
        .decrypt_padded_mut::<cbc::cipher::block_padding::NoPadding>(buf)
        .expect("AES-CBC decrypt failed (padding)");
}

/// NORMAL collector (sha1-verified chunks → temp stream) — safe & tolerant
fn collect_normal_stream<'a>(mut p: &'a [u8], out: &mut [u8]) -> Result<usize> {
    let mut written = 0usize;

    loop {
        if p.len() < 24 {
            eprintln!("Warning: reached end of NORMAL stream (need 24 bytes, have {})", p.len());
            break;
        }

        // ---- Safe read of block header ----
        let block_size = u32::from_be_bytes([p[0], p[1], p[2], p[3]]) as usize;
        if block_size == 0 {
            break;
        }

        // ---- Validate block size before slicing ----
        if block_size < 24 {
            eprintln!("Warning: malformed block (size {} < 24) — stopping", block_size);
            break;
        }
        if block_size > p.len() {
            eprintln!(
                "Warning: block size {} exceeds remaining {} bytes — stopping",
                block_size, p.len()
            );
            break;
        }

        // ---- Now safe to slice ----
        let block_hash = &p[4..24];
        let payload    = &p[24..block_size];

        // ---- Verify SHA1 over payload ----
        let mut sha = Sha1::new();
        sha.update(payload);
        let digest = sha.finalize();
        if digest.as_slice() != block_hash {
            eprintln!(
                "Warning: SHA1 mismatch for NORMAL block (payload {} bytes)",
                payload.len()
            );
        }

        // ---- Walk chunks ----
        let mut q = payload;
        while q.len() >= 2 {
            let sz = u16::from_be_bytes([q[0], q[1]]) as usize;
            if sz == 0 {
                q = &q[2.min(q.len())..];
                break;
            }

            if q.len() < 2 + sz {
                eprintln!(
                    "Warning: truncated NORMAL chunk (need {}, have {})",
                    2 + sz, q.len()
                );
                break;
            }

            if written + sz > out.len() {
                eprintln!(
                    "Warning: output overflow ({} + {} > {}) — truncating output",
                    written, sz, out.len()
                );
                return Ok(written);
            }

            out[written..written + sz].copy_from_slice(&q[2..2 + sz]);
            written += sz;
            q = &q[2 + sz..];
        }

        // ---- Move to next block safely ----
        if block_size > p.len() {
            break;
        }
        p = &p[block_size..];
    }

    Ok(written)
}

// ------------------------------ public loader ------------------------------

/// Load a XEX2 image (decrypt/decompress -> PE laydown -> Image sections).
pub fn load_xex2(xex: &[u8]) -> Result<Image> {
    let _phase = Phase::new("xex::load_xex2");
    ensure!(xex.len() >= 0x18, "buffer too small");
    ensure!(&xex[0..4] == XEX_MAGIC, "not XEX2");

    let hdr = parse_xex2_header(xex)?;
    let sec = parse_security_info(xex, hdr.security_off as usize)?;

    xlog!(
        "XEX: header_size=0x{:X} security_off=0x{:X} image_size=0x{:X} load_addr=0x{:08X}",
        hdr.header_size,
        hdr.security_off,
        sec.image_size,
        sec.load_addr
    );

    // ---------------- file format info ----------------
    let ffi_blob = match get_opt_ptr(xex, &hdr, XEX_HEADER_FILE_FORMAT_INFO) {
        Some(OptRef::Blob(b)) => b,
        Some(OptRef::InlinePtr(p)) => p,
        _ => bail!("missing/invalid FILE_FORMAT_INFO"),
    };
    let ffi = parse_file_format_info(ffi_blob)?;
    xlog!(
        "XEX: file_format_info: info_size={} encryption={} compression={}",
        ffi.info_size,
        ffi.encryption,
        ffi.compression
    );

    // Prepare source bytes (after header)
    ensure!(xex.len() >= hdr.header_size as usize, "xex shorter than header");
    let mut src = &xex[hdr.header_size as usize..];

    // If encrypted: decrypt AES image payload with decrypted key
    let work_decrypted_vec;
    if ffi.encryption == XEX_ENCRYPTION_NORMAL {
        let _p = Phase::new("xex::decrypt_image");
        xlog!("XEX: decrypting title key...");
        let mut key = [0u8; 16];
        key.copy_from_slice(&parse_security_info(xex, hdr.security_off as usize)?.aes_key_be);
        aes128_cbc_decrypt_in_place(&XEX2_RETAIL_KEY, &BLANK_IV, &mut key);
        xlog!("XEX: decrypting image payload (AES-128-CBC) ...");
        let mut tmp = src.to_vec();
        aes128_cbc_decrypt_in_place(&key, &BLANK_IV, &mut tmp);
        xlog!("XEX: decryption complete ({} bytes)", tmp.len());
        work_decrypted_vec = tmp;
        src = &work_decrypted_vec;
    } else if ffi.encryption != XEX_ENCRYPTION_NONE {
        bail!("unsupported encryption type {}", ffi.encryption);
    }

    // ---------------- decompression ----------------
    let image_bytes: Vec<u8> = match ffi.compression {
        XEX_COMPRESSION_NONE => {
            let _p = Phase::new("xex::copy_no_compress");
            let need = sec.image_size as usize;
            ensure!(src.len() >= need, "no-compress src too small");
            xlog!("XEX: no compression; copying {} bytes", need);
            let mut out = vec![0u8; need];
            out[..need].copy_from_slice(&src[..need]);
            out
        }
        XEX_COMPRESSION_BASIC => {
            let _p = Phase::new("xex::decomp_basic");
            ensure!(ffi.info_size >= 8, "BASIC info too small");
            let inf_rest = &ffi_blob[8..];
            let block_sz = 8;
            ensure!(inf_rest.len() >= block_sz, "no basic blocks");
            let num_blocks = (ffi.info_size as usize / block_sz) - 1;

            let mut total = 0usize;
            for i in 0..num_blocks {
                let off = i * block_sz;
                let data_sz = BE::read_u32(&inf_rest[off..off + 4]) as usize;
                let zero_sz = BE::read_u32(&inf_rest[off + 4..off + 8]) as usize;
                total += data_sz + zero_sz;
            }
            xlog!(
                "XEX: BASIC blocks={} total_unpacked={} bytes",
                num_blocks,
                total
            );

            let mut out = vec![0u8; total];
            let mut d = 0usize;
            let mut p = src;
            for i in 0..num_blocks {
                let off = i * block_sz;
                let data_sz = BE::read_u32(&inf_rest[off..off + 4]) as usize;
                let zero_sz = BE::read_u32(&inf_rest[off + 4..off + 8]) as usize;
                ensure!(p.len() >= data_sz, "basic block OOR");
                out[d..d + data_sz].copy_from_slice(&p[..data_sz]);
                d += data_sz;
                p = &p[data_sz..];
                out[d..d + zero_sz].fill(0);
                d += zero_sz;
                xlog!(
                    "BASIC: block #{} data={} zero={} d={}",
                    i,
                    data_sz,
                    zero_sz,
                    d
                );
            }
            ensure!(d == total, "basic total mismatch");
            out
        }
        XEX_COMPRESSION_NORMAL => {
            let _p = Phase::new("xex::decomp_normal");
            // The NORMAL info is stored after the first 8 bytes of FILE_FORMAT_INFO blob
            let ninfo = parse_normal_info(&ffi_blob[8..])?;
            xlog!(
                "XEX: NORMAL window={} first_block_len={} (src_len={})",
                ninfo.window_size,
                ninfo.first_block.len(),
                src.len()
            );

            let mut temp = vec![0u8; src.len()];
            let written = collect_normal_stream(ninfo.first_block, &mut temp)?;
            xlog!(
                "XEX: collected NORMAL stream: {} bytes (into temp)",
                written
            );

            let dst_len = sec.image_size as usize;
            let mut out: Vec<u8> = Vec::with_capacity(dst_len);

            let src_ptr = temp.as_ptr();
            let src_len = written;
            let dst_ptr = out.as_mut_ptr();

            unsafe {
                let rc = lzxDecompress(
                    src_ptr,
                    src_len,
                    dst_ptr,
                    dst_len,
                    ninfo.window_size,
                    std::ptr::null(),
                    0,
                );
                ensure!(rc == 0, "lzxDecompress failed rc={}", rc);
                out.set_len(dst_len);
            }
            xlog!("XEX: LZX done → {} bytes", out.len());
            out
        }
        other => bail!("unsupported compression {}", other),
    };

    // ---------------- set base / entry from opt headers ----------------
    // Start from the XEX security load address (what the console would use),
    // but let XEX headers refine it.
    let mut base = sec.load_addr;
    xlog!("XEX: security load_addr = 0x{:08X}", base);

    // XEX IMAGE_BASE_ADDRESS optional header may override the security load_addr.
    if let Some(OptRef::InlinePtr(p)) | Some(OptRef::Blob(p)) =
        get_opt_ptr(xex, &hdr, XEX_HEADER_IMAGE_BASE_ADDRESS)
    {
        ensure!(p.len() >= 4, "image_base OOR");
        let xex_base = BE::read_u32(&p[0..4]);
        if xex_base != 0 {
            xlog!(
                "XEX: IMAGE_BASE_ADDRESS overrides load_addr: 0x{:08X} -> 0x{:08X}",
                base,
                xex_base
            );
            base = xex_base;
        }
    }

    // XEX ENTRY_POINT — mirror C++: treat it as an absolute EA, not RVA.
    let mut entry: u32 = 0;
    if let Some(OptRef::InlinePtr(p)) | Some(OptRef::Blob(p)) =
        get_opt_ptr(xex, &hdr, XEX_HEADER_ENTRY_POINT)
    {
        ensure!(p.len() >= 4, "entry OOR");
        entry = BE::read_u32(&p[0..4]);
    }

    // Final fallback: allow env override or use a common Xenon default (0x8200_0000)
    if base == 0 {
        if let std::result::Result::Ok(s) = std::env::var("XENON_IMAGE_BASE") {
            // accept "0x…" hex or decimal
            let s_trim = s.trim();
            let parsed = if let Some(hex) =
                s_trim.strip_prefix("0x").or_else(|| s_trim.strip_prefix("0X"))
            {
                u32::from_str_radix(hex, 16)
            } else {
                s_trim.parse::<u32>()
            };
            if let std::result::Result::Ok(v) = parsed {
                xlog!(
                    "PE: image_base overridden by env XENON_IMAGE_BASE=0x{:08X}",
                    v
                );
                base = v;
            }
        }
    }
    if base == 0 {
        const DEFAULT_XENON_BASE: u32 = 0x8200_0000;
        xlog!(
            "PE: image_base still 0 → falling back to 0x{:08X}",
            DEFAULT_XENON_BASE
        );
        base = DEFAULT_XENON_BASE;
    }

    xlog!("PE: image_base=0x{:08X} entry=0x{:08X}", base, entry);

    let dos = image_bytes.as_slice();
    let nt_off_opt = pe_try_nt_headers_off(dos);

    // Base Image object
    let mut img = Image {
        data: image_bytes.clone(),
        base,
        size: image_bytes.len() as u32,
        entry_point: entry,
        sections: Vec::new(),
    };

    // If no valid PE header, behave like a very forgiving loader: treat the entire
    // decompressed blob as one big CODE section. This lets "odd" XEX images (Iruka,
    // test apps, etc.) still be analysed/recompiled.
    let nt_off = match nt_off_opt {
        None => {
            xlog!(
                "PE: no valid NT headers; using flat '.text' layout for image (size=0x{:X})",
                image_bytes.len()
            );

            img.sections.push(Section {
                name: ".text".to_string(),
                base: img.base,
                data: image_bytes.clone(),
                flags: SectionFlags::CODE,
            });

            // --- DEBUG: peek at specific addresses to compare with IDA ---
            fn debug_word(img: &Image, ea: u32) {
                if let Some(word) = img.read_be_u32(ea) {
                    xlog!("DEBUG: word[0x{:08X}] = 0x{:08X}", ea, word);
                } else {
                    xlog!("DEBUG: word[0x{:08X}] = <OOR>", ea);
                }
            }

            debug_word(&img, 0x82177E40);
            debug_word(&img, 0x82170440);

            return Ok(img);
        }
        Some(off) => off,
    };

    // At this point we know at least Signature + FileHeader fit.
    let file_hdr_off = nt_off + 4;
    ensure!(file_hdr_off + 20 <= dos.len(), "NT header OOR");

    let file_hdr = &dos[file_hdr_off..file_hdr_off + 20];
    let nsects = pe_num_sections(file_hdr)?;
    ensure!(nsects > 0, "PE reports zero sections");

    let secs_slice = pe_sections_slice(dos, nt_off)?;
    xlog!("PE: NumberOfSections={}", nsects);

    // IMAGE_SECTION_HEADER is 40 bytes
    for i in 0..nsects {
        let off = i * 40;
        ensure!(off + 40 <= secs_slice.len(), "section header OOR");
        let sh = &secs_slice[off..off + 40];

        let name_bytes = &sh[0..8];
        let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(8);
        let name = std::str::from_utf8(&name_bytes[..name_end])
            .unwrap_or("")
            .to_string();

        let virt_size = LE::read_u32(&sh[8..12]) as usize; // VirtualSize
        let virt_addr = LE::read_u32(&sh[12..16]) as usize; // VirtualAddress (RVA)
        let size_raw = LE::read_u32(&sh[16..20]) as usize; // SizeOfRawData
        let _ptr_raw = LE::read_u32(&sh[20..24]) as usize; // PointerToRawData
        let characteristics = LE::read_u32(&sh[36..40]);

        if virt_size == 0 {
            xlog!(
                "PE: section '{}' has VirtualSize=0 (RVA=0x{:06X}) — mapping empty data",
                name,
                virt_addr
            );
        }

        // -------- tolerant mapping (matches C++ "just trust VirtualAddress") --------
        let bytes = if virt_size == 0 {
            Vec::new()
        } else if virt_addr >= image_bytes.len() {
            // whole section is beyond image → treat as BSS / zero-filled
            xlog!(
                "PE: section '{}' virt RVA=0x{:06X} beyond image (image_size=0x{:X}) — zero-filling {} bytes",
                name,
                virt_addr,
                image_bytes.len(),
                virt_size
            );
            vec![0u8; virt_size]
        } else {
            let available = image_bytes.len() - virt_addr;
            let copy_len = virt_size.min(available);

            let mut v = vec![0u8; virt_size];
            v[..copy_len]
                .copy_from_slice(&image_bytes[virt_addr..virt_addr + copy_len]);

            if copy_len < virt_size {
                xlog!(
                    "PE: section '{}' partially beyond image (RVA=0x{:06X}+0x{:X}, image=0x{:X}); copied {}, zero-filled {}",
                    name,
                    virt_addr,
                    virt_size,
                    image_bytes.len(),
                    copy_len,
                    virt_size - copy_len
                );
            }

            v
        };

        let mut flags = SectionFlags::DATA;
        if (characteristics & IMAGE_SCN_CNT_CODE) != 0 {
            flags |= SectionFlags::CODE;
        }

        let va = img.base.wrapping_add(virt_addr as u32);
        xlog!(
            "PE: section {:>2} name='{}' RVA=0x{:06X} VA=0x{:08X} raw={} virt={} CODE={}",
            i,
            name,
            virt_addr,
            va,
            size_raw,
            virt_size,
            flags.contains(SectionFlags::CODE)
        );

        img.sections.push(Section {
            name,
            base: va,
            data: bytes,
            flags,
        });
    }

    // --- DEBUG: peek at specific addresses to compare with IDA ---
    fn debug_word(img: &Image, ea: u32) {
        if let Some(word) = img.read_be_u32(ea) {
            xlog!("DEBUG: word[0x{:08X}] = 0x{:08X}", ea, word);
        } else {
            xlog!("DEBUG: word[0x{:08X}] = <OOR>", ea);
        }
    }

    debug_word(&img, 0x82177E40);
    debug_word(&img, 0x82170440);

    Ok(img)
}

// ------------------------------ import symbol collection ------------------------------

#[derive(Clone, Copy)]
struct ImportHeader {
    size_of_header: u32,
    size_of_string_table: u32,
    num_imports: u32,
}

#[derive(Clone, Copy)]
struct ImportLibrary {
    size: u32,
    name_off: u16,
    number_of_imports: u16,
}

#[derive(Clone, Copy)]
struct ImportDescriptor {
    first_thunk: u32,
}

fn parse_import_header(b: &[u8]) -> Result<ImportHeader> {
    // C++: Xex2ImportHeader is exactly 3 * be<u32> (12 bytes)
    ensure!(b.len() >= 12, "imp hdr too small");
    Ok(ImportHeader {
        size_of_header:       rd_be_u32(b, 0)?,
        size_of_string_table: rd_be_u32(b, 4)?,
        num_imports:          rd_be_u32(b, 8)?,
    })
}

// Header size of Xex2ImportLibrary in bytes:
const IMPORT_LIB_HDR_LEN: usize = 4 + 20 + 4 + 4 + 4 + 2 + 2; // 40

fn parse_import_library(b: &[u8]) -> Result<ImportLibrary> {
    // C++ layout:
    // be<u32> size;
    // char    nextImportDigest[0x14];
    // be<u32> id;
    // be<u32> version;
    // be<u32> minVersion;
    // be<u16> name;
    // be<u16> numberOfImports;
    ensure!(b.len() >= IMPORT_LIB_HDR_LEN, "imp lib too small");

    Ok(ImportLibrary {
        size: rd_be_u32(b, 0)?, // kept for debugging/logging only
        name_off: rd_be_u16(b, 4 + 20 + 4 + 4 + 4)?,
        number_of_imports: rd_be_u16(b, 4 + 20 + 4 + 4 + 4 + 2)?,
    })
}

fn parse_import_descriptor(b: &[u8]) -> Result<ImportDescriptor> {
    ensure!(b.len() >= 4, "imp desc too small");
    Ok(ImportDescriptor {
        first_thunk: rd_be_u32(b, 0)?,
    })
}

/// Read a BE u32 from the PE-laid out image by VA.
fn image_read_be_u32(img: &Image, ea: u32) -> Option<u32> {
    for s in &img.sections {
        let start = s.base;
        let end   = s.base.wrapping_add(s.data.len() as u32);
        if ea >= start && ea + 4 <= end {
            let off = (ea - start) as usize;
            if off + 4 <= s.data.len() {
                return Some(BE::read_u32(&s.data[off..off + 4]));
            }
        }
    }
    None
}

/// Collect `(thunk_va, name)` for imported **functions** by reading the thunk words.
pub fn collect_import_symbols(xex: &[u8], img: &Image) -> Result<Vec<(u32, String)>> {
    let _p = Phase::new("xex::collect_import_symbols");
    let hdr = parse_xex2_header(xex)?;

    let Some(OptRef::Blob(imp_blob)) =
        get_opt_ptr(xex, &hdr, XEX_HEADER_IMPORT_LIBRARIES)
    else {
        xlog!("imports: no XEX_HEADER_IMPORT_LIBRARIES");
        return Ok(Vec::new());
    };

    // ---------------- Parse import header + string table ----------------

    const IMPORT_HDR_STRUCT_SIZE: usize = 12;

    ensure!(
        imp_blob.len() >= IMPORT_HDR_STRUCT_SIZE,
        "imp hdr blob too small"
    );
    let ih = parse_import_header(imp_blob)?;

    let str_tbl_off = IMPORT_HDR_STRUCT_SIZE;
    let str_tbl_end = str_tbl_off + ih.size_of_string_table as usize;
    ensure!(
        imp_blob.len() >= str_tbl_end,
        "imp table OOR (string table)"
    );

    let string_tbl = &imp_blob[str_tbl_off..str_tbl_end];

    // read library names (padded to 4), but be tolerant of non-UTF8
    let mut lib_names: Vec<String> = Vec::with_capacity(ih.num_imports as usize);
    let mut p = string_tbl;

    for _ in 0..ih.num_imports {
        if p.is_empty() {
            xlog!(
                "imports: string table exhausted early (have {} name(s), expected {})",
                lib_names.len(),
                ih.num_imports
            );
            break;
        }

        let nul = match p.iter().position(|&c| c == 0) {
            Some(n) => n,
            None => {
                return Err(anyhow!("unterminated lib name in import string table"));
            }
        };

        let raw = &p[..nul];

        // Try strict UTF-8 first, fall back to lossy if needed
        let utf8 = std::str::from_utf8(raw);
        let mut name = if utf8.is_ok() {
            utf8.unwrap().to_string()
        } else {
            let s = String::from_utf8_lossy(raw).into_owned();
            xlog!(
                "imports: non-UTF8 lib name bytes {:?}, decoded (lossy) as '{}'",
                raw,
                s
            );
            s
        };

        let padded = ((nul + 1) + 3) & !3;
        if p.len() < padded {
            return Err(anyhow!("string pad OOR in import string table"));
        }
        p = &p[padded..];

        // normalise: lowercase, strip any directory components
        name = name.replace('\\', "/");
        if let Some(pos) = name.rfind('/') {
            name = name[pos + 1..].to_string();
        }
        name = name.to_ascii_lowercase();
        lib_names.push(name);
    }

    let (xam_map, krnl_map) = build_export_maps();
    xlog!("imports: libraries(raw)={:?}", lib_names);

    // ---------------- Walk libraries ----------------

    // C++:
    // auto* library =
    //   (Xex2ImportLibrary*)((char*)imports
    //       + sizeof(Xex2ImportHeader)
    //       + imports->sizeOfStringTable);
    //
    // Then:
    //   descriptors = (Xex2ImportDescriptor*)(library + 1);
    //   library = (Xex2ImportLibrary*)((char*)(library + 1)
    //              + library->numberOfImports * sizeof(Xex2ImportDescriptor));

    let mut lib_ptr = &imp_blob[str_tbl_end..];

    let mut out = Vec::new();
    let mut total_seen = 0usize;

    for lib_name in lib_names {
        if lib_ptr.len() < IMPORT_LIB_HDR_LEN {
            xlog!(
                "imports: not enough bytes for library header '{}' (have {}, need {})",
                lib_name,
                lib_ptr.len(),
                IMPORT_LIB_HDR_LEN
            );
            break;
        }

        let lib = parse_import_library(lib_ptr)?;

        // Descriptors start immediately after the header
        let mut cur = &lib_ptr[IMPORT_LIB_HDR_LEN..];

        // pick map by normalised name
        let names: &std::collections::HashMap<u32, &'static str> =
            if lib_name == "xam.xex" {
                &xam_map
            } else if lib_name == "xboxkrnl.exe" {
                &krnl_map
            } else {
                xlog!("imports: skipping non-XAM/KRNL '{}'", lib_name);
                // Advance past this header + its descriptors just like C++:
                let skip_bytes = IMPORT_LIB_HDR_LEN
                    .saturating_add((lib.number_of_imports as usize).saturating_mul(4));
                if lib_ptr.len() < skip_bytes {
                    xlog!(
                        "imports: '{}' truncated while skipping (have {}, need {})",
                        lib_name,
                        lib_ptr.len(),
                        skip_bytes
                    );
                    break;
                }
                lib_ptr = &lib_ptr[skip_bytes..];
                continue;
            };

        let mut added_this_lib = 0usize;

        for _ in 0..lib.number_of_imports {
            if cur.len() < 4 {
                xlog!(
                    "imports: descriptor OOR in '{}' (need 4, have {})",
                    lib_name,
                    cur.len()
                );
                break;
            }

            let d = parse_import_descriptor(cur)?;
            cur = &cur[4..];

            let thunk_va = d.first_thunk;
            let data = match image_read_be_u32(img, thunk_va) {
                Some(v) => v,
                None => {
                    xlog!(
                        "imports: thunk VA 0x{:08X} outside image sections",
                        thunk_va
                    );
                    continue;
                }
            };

            // [type (8)] [unk (8)] [ordinal (16)]
            let _typ    = ((data >> 24) & 0xFF) as u8;
            let ordinal = (data & 0xFFFF) as u32;

            let sym = if let Some(sym) = names.get(&ordinal) {
                (*sym).to_string()
            } else {
                // Stable generated name
                format!(
                    "{}_ord_{:04X}",
                    if lib_name == "xam.xex" { "xam" } else { "xboxkrnl" },
                    ordinal
                )
            };

            xlog!(
                "imports: 0x{:08X} -> {} (ord {:04X})",
                thunk_va,
                sym,
                ordinal
            );
            out.push((thunk_va, sym));
            added_this_lib += 1;
            total_seen += 1;
        }

        xlog!("imports: {} → added {} thunk(s)", lib_name, added_this_lib);

        // Advance to next library header, C++-style:
        let consumed_bytes =
            IMPORT_LIB_HDR_LEN.saturating_add((lib.number_of_imports as usize).saturating_mul(4));
        if lib_ptr.len() < consumed_bytes {
            xlog!(
                "imports: '{}' truncated at end (have {}, need {})",
                lib_name,
                lib_ptr.len(),
                consumed_bytes
            );
            break;
        }
        lib_ptr = &lib_ptr[consumed_bytes..];
    }

    xlog!("imports: collected {} thunk name(s) total", total_seen);
    Ok(out)
}