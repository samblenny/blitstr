// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
pub mod bold;
pub mod emoji;
pub mod regular;
pub mod small;

use super::m3hash;
use core::fmt;

/// Holds header data for a font glyph
pub struct GlyphHeader {
    pub w: usize,
    pub h: usize,
    pub y_offset: usize,
}
impl GlyphHeader {
    /// Unpack glyph header of format: (w:u8)<<16 | (h:u8)<<8 | yOffset:u8
    pub fn new(header: u32) -> GlyphHeader {
        let w = ((header << 8) >> 24) as usize;
        let h = ((header << 16) >> 24) as usize;
        let y_offset = (header & 0x000000ff) as usize;
        GlyphHeader { w, h, y_offset }
    }
}

/// Available typeface glyph sets
#[derive(Copy, Clone, Debug)]
pub enum GlyphSet {
    Emoji,
    Bold,
    Regular,
    Small,
}

/// Error type for when a font has no glyph to match a grapheme cluster query
#[derive(Debug, Clone)]
pub struct GlyphNotFound;
impl fmt::Display for GlyphNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Font has no glyph for requested grapheme cluster")
    }
}

/// Abstraction for working with typeface glyph sets
#[derive(Copy, Clone)]
pub struct Font {
    pub glyph_pattern_offset: GlyphPatternOffsetFnPtr,
    pub glyph_data: GlyphDataFnPtr,
}
pub type GlyphPatternOffsetFnPtr = fn(&str) -> Result<(usize, usize), GlyphNotFound>;
pub type GlyphDataFnPtr = fn(usize) -> u32;
impl Font {
    pub fn new(gs: GlyphSet) -> Font {
        match gs {
            GlyphSet::Emoji => Font {
                glyph_pattern_offset: emoji::get_blit_pattern_offset,
                glyph_data: emoji_data,
            },
            GlyphSet::Bold => Font {
                glyph_pattern_offset: bold::get_blit_pattern_offset,
                glyph_data: bold_data,
            },
            GlyphSet::Regular => Font {
                glyph_pattern_offset: regular::get_blit_pattern_offset,
                glyph_data: regular_data,
            },
            GlyphSet::Small => Font {
                glyph_pattern_offset: small::get_blit_pattern_offset,
                glyph_data: small_data,
            },
        }
    }
}

/// Get word of packed glyph data for emoji
pub fn emoji_data(index: usize) -> u32 {
    emoji::DATA[index]
}

/// Get word of packed glyph data for bold
pub fn bold_data(index: usize) -> u32 {
    bold::DATA[index]
}

/// Get word of packed glyph data for regular
pub fn regular_data(index: usize) -> u32 {
    regular::DATA[index]
}

/// Get word of packed glyph data for small
pub fn small_data(index: usize) -> u32 {
    small::DATA[index]
}

/// Compute Murmur3 hash function of the first limit codepoints of a string,
/// using each char as a u32 block. This wrapper function exists to provide a
/// stable font::murmur3(...) internal API that the font codegen system can use
/// to calculate grapheme cluster hashes in the fonts/*.rs font files.
/// Returns: (murmur3 hash, how many bytes of key were hashed (e.g. key[..n]))
pub fn murmur3(key: &str, seed: u32, limit: u32) -> (u32, usize) {
    m3hash::grapheme_cluster(key, seed, limit)
}
