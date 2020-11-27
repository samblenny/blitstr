// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// This code includes an adaptation of the the murmur3 hash algorithm.
// The murmur3 public domain notice, as retrieved on August 3, 2020 from
// https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp,
// states:
// > MurmurHash3 was written by Austin Appleby, and is placed in the public
// > domain. The author hereby disclaims copyright to this source code.
//
#![forbid(unsafe_code)]
pub mod bold;
pub mod emoji;
pub mod regular;
pub mod small;

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
#[derive(Copy, Clone)]
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
/// using each char as a u32 block.
/// Returns: (murmur3 hash, how many bytes of key were hashed (e.g. key[..n]))
pub fn murmur3(key: &str, seed: u32, limit: u32) -> (u32, usize) {
    let mut h = seed;
    let mut k;
    // Hash each character as its own u32 block
    let mut n = 0;
    let mut bytes_hashed = key.len();
    for (i, c) in key.char_indices() {
        if n >= limit {
            bytes_hashed = i;
            break;
        }
        k = c as u32;
        k = k.wrapping_mul(0xcc9e2d51);
        k = k.rotate_left(15);
        k = k.wrapping_mul(0x1b873593);
        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5);
        h = h.wrapping_add(0xe6546b64);
        n += 1;
    }
    h ^= bytes_hashed as u32;
    // Finalize with avalanche
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    (h, bytes_hashed)
}
