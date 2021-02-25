// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
pub mod bold;
pub mod emoji;
pub mod hanzi;
pub mod regular;
pub mod small;

use super::m3hash;
use core::fmt;

/// Holds an offset into the glyph data array of a particular glyph set
#[derive(Copy, Clone, Debug)]
pub enum GlyphData {
    Emoji(usize),
    Bold(usize),
    Regular(usize),
    Small(usize),
    Hanzi(usize),
}
impl GlyphData {
    /// Unpack glyph header of format: (w:u8)<<16 | (h:u8)<<8 | yOffset:u8
    pub fn header(self) -> GlyphHeader {
        let header = match self {
            GlyphData::Emoji(offset) => emoji::DATA[offset],
            GlyphData::Bold(offset) => bold::DATA[offset],
            GlyphData::Regular(offset) => regular::DATA[offset],
            GlyphData::Small(offset) => small::DATA[offset],
            GlyphData::Hanzi(offset) => hanzi::DATA[offset],
        };
        let w = ((header << 8) >> 24) as usize;
        let h = ((header << 16) >> 24) as usize;
        let y_offset = (header & 0x000000ff) as usize;
        GlyphHeader { w, h, y_offset }
    }

    /// Unpack the nth pixel data word following the header
    pub fn nth_word(self, n: usize) -> u32 {
        match self {
            GlyphData::Emoji(offset) => emoji::DATA[offset + n],
            GlyphData::Bold(offset) => bold::DATA[offset + n],
            GlyphData::Regular(offset) => regular::DATA[offset + n],
            GlyphData::Small(offset) => small::DATA[offset + n],
            GlyphData::Hanzi(offset) => hanzi::DATA[offset + n],
        }
    }
}

/// Holds header data for a font glyph
pub struct GlyphHeader {
    pub w: usize,
    pub h: usize,
    pub y_offset: usize,
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
