// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

pub mod bold;
pub mod emoji;
pub mod hanzi;
pub mod regular;
pub mod small;

use super::m3hash;
use core::fmt;

#[cfg(any(target_os = "none", target_os = "xous"))]
pub fn map_font(font_map: GlyphData) {
    use core::sync::atomic::Ordering::Relaxed;
    use log::info;
    let debug = false;
    match font_map {
        GlyphData::Emoji(addr) => {emoji::DATA_LOCATION.store(addr as u32, Relaxed);
            if debug {info!("BLITSTR: emoji addr 0x{:08x}", addr as u32)} },
        GlyphData::Bold(addr) => {bold::DATA_LOCATION.store(addr as u32, Relaxed);
            if debug {info!("BLITSTR: bold addr 0x{:08x}", addr as u32)} },
        GlyphData::Regular(addr) => {regular::DATA_LOCATION.store(addr as u32, Relaxed);
            if debug {info!("BLITSTR: regular addr 0x{:08x}", addr as u32)} },
        GlyphData::Small(addr) => {small::DATA_LOCATION.store(addr as u32, Relaxed);
            if debug {info!("BLITSTR: small addr 0x{:08x}", addr as u32)} },
        GlyphData::Hanzi(addr) => {hanzi::DATA_LOCATION.store(addr as u32, Relaxed);
            if debug {info!("BLITSTR: hanzi addr 0x{:08x}", addr as u32)} },
    }
}

/// Holds an offset into the glyph data array of a particular glyph set
#[derive(Copy, Clone, Debug)]
pub enum GlyphData {
    Emoji(usize),
    Bold(usize),
    Regular(usize),
    Small(usize),
    Hanzi(usize),
}
#[cfg(any(target_os = "none", target_os = "xous"))]
impl GlyphData {
    /// Unpack glyph header of format: (w:u8)<<16 | (h:u8)<<8 | yOffset:u8
    pub fn header(self) -> Result<GlyphHeader, NoGlyphErr> {
        use core::sync::atomic::Ordering::Relaxed;
        use log::info;
        let debug = false;
        if debug { info!("BLITSTR: header unpack: {:?}", self); }
        let header = match self {
            GlyphData::Emoji(offset) => {
                if debug { info!("BLITSTR: emoji at 0x{:08x}", emoji::DATA_LOCATION.load(Relaxed) ); }
                if emoji::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(emoji::DATA_LOCATION.load(Relaxed));
                    data.add(offset).read_volatile()
                }
            },
            GlyphData::Bold(offset) => {
                if debug { info!("BLITSTR: bold at 0x{:08x}", bold::DATA_LOCATION.load(Relaxed) ); }
                if bold::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(bold::DATA_LOCATION.load(Relaxed));
                    data.add(offset).read_volatile()
                }
            },
            GlyphData::Regular(offset) => {
                if debug { info!("BLITSTR: regular at 0x{:08x}", regular::DATA_LOCATION.load(Relaxed) ); }
                if regular::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(regular::DATA_LOCATION.load(Relaxed));
                    data.add(offset).read_volatile()
                }
            },
            GlyphData::Small(offset) => {
                if debug { info!("BLITSTR: small at 0x{:08x}, offset {}", small::DATA_LOCATION.load(Relaxed), offset ); }
                if small::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(small::DATA_LOCATION.load(Relaxed));
                    data.add(offset).read_volatile()
                }
            },
            GlyphData::Hanzi(offset) => {
                if debug { info!("BLITSTR: hanzi at 0x{:08x}", hanzi::DATA_LOCATION.load(Relaxed) ); }
                if hanzi::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(hanzi::DATA_LOCATION.load(Relaxed));
                    data.add(offset).read_volatile()
                }
            },
        };
        let w = (header << 8) >> 24;
        let h = (header << 16) >> 24;
        let y_offset = header & 0x000000ff;
        Ok(GlyphHeader { w, h, y_offset })
    }

    /// Unpack the nth pixel data word following the header
    pub fn nth_word(self, n: usize) -> Result<u32, NoGlyphErr> {
        use log::info;
        let debug = false;
        if debug { info!("BLITSTR: nth word: {:?}, n {}", self, n); }

        use core::sync::atomic::Ordering::Relaxed;
        let word = match self {
            GlyphData::Emoji(offset) => {
                if emoji::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(emoji::DATA_LOCATION.load(Relaxed));
                    data.add(offset+n).read_volatile()
                }
            },
            GlyphData::Bold(offset) => {
                if bold::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(bold::DATA_LOCATION.load(Relaxed));
                    data.add(offset+n).read_volatile()
                }
            },
            GlyphData::Regular(offset) => {
                if regular::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(regular::DATA_LOCATION.load(Relaxed));
                    data.add(offset+n).read_volatile()
                }
            },
            GlyphData::Small(offset) => {
                if small::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(small::DATA_LOCATION.load(Relaxed));
                    data.add(offset+n).read_volatile()
                }
            },
            GlyphData::Hanzi(offset) => {
                if hanzi::DATA_LOCATION.load(Relaxed) == 0 {
                    return Err(NoGlyphErr)
                }
                unsafe {
                    let data: *const u32 = core::mem::transmute(hanzi::DATA_LOCATION.load(Relaxed));
                    data.add(offset+n).read_volatile()
                }
            },
        };
        Ok(word)
    }
}

#[cfg(not(any(target_os = "none", target_os = "xous")))]
impl GlyphData {
    /// Unpack glyph header of format: (w:u8)<<16 | (h:u8)<<8 | yOffset:u8
    pub fn header(self) -> Result<GlyphHeader, NoGlyphErr> {
        let header = match self {
            GlyphData::Emoji(offset) => emoji::DATA[offset],
            GlyphData::Bold(offset) => bold::DATA[offset],
            GlyphData::Regular(offset) => regular::DATA[offset],
            GlyphData::Small(offset) => small::DATA[offset],
            GlyphData::Hanzi(offset) => hanzi::DATA[offset],
        };
        let w = (header << 8) >> 24;
        let h = (header << 16) >> 24;
        let y_offset = header & 0x000000ff;
        Ok(GlyphHeader { w, h, y_offset })
    }

    /// Unpack the nth pixel data word following the header
    pub fn nth_word(self, n: usize) -> Result<u32, NoGlyphErr> {
        Ok(match self {
            GlyphData::Emoji(offset) => emoji::DATA[offset + n],
            GlyphData::Bold(offset) => bold::DATA[offset + n],
            GlyphData::Regular(offset) => regular::DATA[offset + n],
            GlyphData::Small(offset) => small::DATA[offset + n],
            GlyphData::Hanzi(offset) => hanzi::DATA[offset + n],
        })
    }
}

/// Holds header data for a font glyph
#[derive(Debug)]
pub struct GlyphHeader {
    pub w: u32,
    pub h: u32,
    pub y_offset: u32,
}

/// Available typeface glyph sets
#[derive(Copy, Clone, Debug)]
pub enum GlyphSet {
    Emoji,
    Bold,
    Regular,
    Small,
    Hanzi,
}

/// Error type for when a font has no glyph to match a grapheme cluster query
#[derive(Debug, Clone)]
pub struct NoGlyphErr;
impl fmt::Display for NoGlyphErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Font has no glyph for requested grapheme cluster")
    }
}

/// Compute Murmur3 hash function of the first limit codepoints of a string,
/// using each char as a u32 block. This wrapper function exists to provide a
/// stable font::murmur3(...) internal API that the font codegen system can use
/// to calculate grapheme cluster hashes in the fonts/*.rs font files.
/// Returns: (murmur3 hash, how many bytes of key were hashed (e.g. key[..n]))
pub fn murmur3(key: &str, seed: u32, limit: u32) -> (u32, usize) {
    m3hash::grapheme_cluster(key, seed, limit)
}
