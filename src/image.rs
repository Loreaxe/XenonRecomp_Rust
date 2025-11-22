// src/image.rs
use anyhow::*;
use byteorder::{BigEndian, ByteOrder};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct SectionFlags: u32 {
        const CODE = 1 << 0;
        const DATA = 1 << 1;
    }
}

#[derive(Clone, Debug)]
pub struct Section {
    pub name: String,
    pub base: u32,      // guest EA of the section
    pub data: Vec<u8>,  // raw bytes
    pub flags: SectionFlags,
}

#[derive(Clone, Debug, Default)]
pub struct Image {
    /// Full owned image bytes (mirrors C++: we keep a private copy)
    pub data: Vec<u8>,
    pub base: u32,
    pub size: u32,
    pub entry_point: u32,
    pub sections: Vec<Section>,
}

impl Image {
    /// C++ Image::Map(name, rva, size, flags, data)
    ///
    /// `rva` is the *virtual* offset relative to `self.base`.
    /// `src` points at the bytes to copy:
    ///   - either a slice of the original file buffer, or
    ///   - a slice inside `self.data` (decompressed/relocated image).
    pub fn map(
        &mut self,
        name: Option<&str>,
        rva: u32,
        size: u32,
        flags: SectionFlags,
        src: &[u8],
    ) {
        let name = name.unwrap_or("").to_string();
        let mut bytes = Vec::with_capacity(size as usize);

        // If src points into self.data we must copy from *that* offset, not from the RVA.
        if !self.data.is_empty()
            && !src.is_empty()
            && (src.as_ptr() as usize) >= (self.data.as_ptr() as usize)
            && (src.as_ptr() as usize + src.len()) <= (self.data.as_ptr() as usize + self.data.len())
        {
            let base_ptr = self.data.as_ptr() as usize;
            let src_ptr = src.as_ptr() as usize;
            let src_off = src_ptr.saturating_sub(base_ptr);
            let end = src_off.saturating_add(size as usize).min(self.data.len());
            let start = src_off.min(self.data.len());

            bytes.extend_from_slice(&self.data[start..end]);
        } else {
            // src is some external buffer; assume it’s at least `size` long.
            bytes.extend_from_slice(&src[..size as usize]);
        }

        // Virtual base = image.base + RVA (same as C++)
        self.sections.push(Section {
            name,
            base: self.base.wrapping_add(rva),
            data: bytes,
            flags,
        });
    }

    /// Find section containing a guest address; returns (&Section, offset_in_section).
    pub fn find(&self, ea: u32) -> Option<(&Section, usize)> {
        for s in &self.sections {
            let lo = s.base;
            let hi = s.base.wrapping_add(s.data.len() as u32);
            if ea >= lo && ea < hi {
                return Some((s, (ea - lo) as usize));
            }
        }
        None
    }

    pub fn find_section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == name)
    }

    pub fn read_be_u32(&self, ea: u32) -> Option<u32> {
        let (sec, off) = self.find(ea)?;
        if off + 4 <= sec.data.len() {
            Some(BigEndian::read_u32(&sec.data[off..off + 4]))
        } else {
            None
        }
    }

    /// Auto-detect parser (like C++ Image::ParseImage).
    pub fn parse_image(bytes: &[u8]) -> Result<Self> {
        ensure!(bytes.len() >= 4, "buffer too small");

        // ELF? \x7F 'E' 'L' 'F'
        if bytes[0] == 0x7F && bytes[1] == b'E' && bytes[2] == b'L' && bytes[3] == b'F' {
            return Self::parse_elf(bytes);
        }

        // XEX2?
        if bytes[0] == b'X' && bytes[1] == b'E' && bytes[2] == b'X' && bytes[3] == b'2' {
            return Self::parse_xex(bytes);
        }

        bail!("unknown image format (no ELF/XEX2 magic)");
    }

    /// Minimal ELF32 (big-endian) loader equivalent to your C++ `ElfLoadImage`.
    pub fn parse_elf(data: &[u8]) -> Result<Self> {
        // --- ELF header sanity ---
        ensure!(data.len() >= 0x34, "ELF too small");
        let ei_class  = data[4]; // EI_CLASS
        let ei_data   = data[5]; // EI_DATA
        ensure!(ei_class == 1, "ELF64 not supported here (expected ELF32)");
        ensure!(ei_data  == 2, "ELF not big-endian (EI_DATA != ELFDATA2MSB)");

        // Offsets in ELF32_Ehdr (big-endian)
        let e_entry = BigEndian::read_u32(&data[0x18..0x1C]);
        let e_phoff = BigEndian::read_u32(&data[0x1C..0x20]) as usize;
        let e_shoff = BigEndian::read_u32(&data[0x20..0x24]) as usize;
        let e_phnum = BigEndian::read_u16(&data[0x2C..0x2E]) as usize;
        let e_shnum = BigEndian::read_u16(&data[0x30..0x32]) as usize;
        let e_shstrndx = BigEndian::read_u16(&data[0x32..0x34]) as usize;

        ensure!(e_phoff + e_phnum * 0x20 <= data.len(), "ELF PHDR out of range");
        ensure!(e_shoff + e_shnum * 0x28 <= data.len(), "ELF SHDR out of range");

        // Find base from first PT_LOAD (C++ scans PHDRs for PT_LOAD = 1)
        let mut base = 0u32;
        for i in 0..e_phnum {
            let off = e_phoff + i * 0x20;
            let p_type  = BigEndian::read_u32(&data[off + 0x00 .. off + 0x04]);
            let p_vaddr = BigEndian::read_u32(&data[off + 0x08 .. off + 0x0C]);
            if p_type == 1 { base = p_vaddr; break; }
        }
        ensure!(base != 0, "no PT_LOAD found");

        // Own a copy of the whole image (C++ copies).
        let mut img = Image {
            data: data.to_vec(),
            base,
            size: data.len() as u32,
            entry_point: e_entry,
            sections: Vec::new(),
        };

        // Section name string table
        ensure!(e_shstrndx < e_shnum, "bad shstrndx");
        let shstr_off = e_shoff + e_shstrndx * 0x28;
        let shstr_tab_off = BigEndian::read_u32(&data[shstr_off + 0x10 .. shstr_off + 0x14]) as usize;
        let shstr_tab_size = BigEndian::read_u32(&data[shstr_off + 0x14 .. shstr_off + 0x18]) as usize;
        ensure!(shstr_tab_off + shstr_tab_size <= data.len(), "shstrtab OOR");
        let shstr = &data[shstr_tab_off .. shstr_tab_off + shstr_tab_size];

        // Map each non-NULL section
        const SHF_EXECINSTR: u32 = 0x4;
        for i in 0..e_shnum {
            let off = e_shoff + i * 0x28;
            let sh_type  = BigEndian::read_u32(&data[off + 0x04 .. off + 0x08]);
            if sh_type == 0 { continue; } // SHT_NULL

            let sh_name  = BigEndian::read_u32(&data[off + 0x00 .. off + 0x04]) as usize;
            let sh_flags = BigEndian::read_u32(&data[off + 0x08 .. off + 0x0C]);
            let sh_addr  = BigEndian::read_u32(&data[off + 0x0C .. off + 0x10]);
            let sh_offset= BigEndian::read_u32(&data[off + 0x10 .. off + 0x14]) as usize;
            let sh_size  = BigEndian::read_u32(&data[off + 0x14 .. off + 0x18]) as usize;

            let name = if sh_name < shstr.len() {
                let tail = &shstr[sh_name..];
                let end = tail.iter().position(|&c| c == 0).unwrap_or(tail.len());
                std::str::from_utf8(&tail[..end]).unwrap_or("").to_string()
            } else { String::new() };

            ensure!(sh_offset + sh_size <= data.len(), "section out of range");

            let flags = if (sh_flags & SHF_EXECINSTR) != 0 {
                SectionFlags::CODE
            } else {
                SectionFlags::DATA
            };

            // Map relative to image base (C++: rva = sh_addr - image.base)
            let rva = sh_addr.wrapping_sub(base);
            let bytes = &data[sh_offset .. sh_offset + sh_size];
            img.map(Some(&name), rva, sh_size as u32, flags, bytes);
        }

        Ok(img)
    }

    /// XEX2 loader — delegate to the real parser (fixes import thunk visibility).
    pub fn parse_xex(bytes: &[u8]) -> Result<Self> {
        // call the real loader, not a stub
        let mut img = crate::xex::load_xex2(bytes)?;
        // keep a copy of the original bytes (needed for import thunk scanning)
        img.data = bytes.to_vec();
        Ok(img)
    }
}
