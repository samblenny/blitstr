// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
use super::fonts;
use super::fonts::{Font, GlyphHeader, GlyphSet};

/// Frame buffer bounds
pub const WORDS_PER_LINE: usize = 11;
pub const PX_PER_LINE: usize = 336;
pub const LINES: usize = 536;
pub const FRAME_BUF_SIZE: usize = WORDS_PER_LINE * LINES;

/// For passing frame buffer references
pub type FrBuf = [u32; FRAME_BUF_SIZE];

/// For specifying a region of pixels in the frame buffer
/// Ranges are x0..x1 and y0..y1 (x0 & y0 are included, x1 & y1 are excluded)
#[derive(Copy, Clone)]
pub struct ClipRegion {
    pub x0: usize,
    pub x1: usize,
    pub y0: usize,
    pub y1: usize,
}

pub enum Style {
    Bold,
    Regular,
    Small,
}

/// Blit string with: XOR, specified font, align xr left yr top
pub fn string_left(style: Style, fb: &mut FrBuf, mut cr: ClipRegion, s: &str) {
    let f1 = match style {
        Style::Bold => Font::new(GlyphSet::Bold),
        Style::Regular => Font::new(GlyphSet::Regular),
        Style::Small => Font::new(GlyphSet::Small),
    };
    let f2 = Font::new(GlyphSet::Emoji);
    let mut cluster = s;
    for _ in 0..s.len() {
        if cluster.len() < 1 {
            // For pure ASCII, s.len() will be the right number of codepoints,
            // but for non-ASCII UTF-8, the bytes in s get consumed faster
            break;
        }
        if let Ok((glyph_width, bytes_used)) = xor_char(fb, cr, cluster, f1) {
            cluster = &cluster[bytes_used..];
            cr.x0 += glyph_width;
        } else if let Ok((glyph_width, bytes_used)) = xor_char(fb, cr, cluster, f2) {
            cluster = &cluster[bytes_used..];
            cr.x0 += glyph_width;
        } else {
            // Fallback: use replacement character
            if let Ok((glyph_width, _)) = xor_char(fb, cr, &"\u{FFFD}", f1) {
                // Advance width by replacement character width
                cr.x0 += glyph_width;
                // Advance string slice position by consuming one UTF-8 character
                if let Some((i, _)) = cluster.char_indices().nth(1) {
                    cluster = &cluster[i..];
                } else {
                    // Last resort... just fail silently and stop parsing
                    break;
                }
            }
        }
    }
}

/// Blit a char with: XOR, align left:xr.0 top:yr.0, pad L:1px R:2px
/// Return: width in pixels of character + padding that were blitted (0 if won't fit in clip region)
///
/// Examples of word alignment for source data (rows of glpyh pixels)
/// 1. Fits in one word:
///    row_width:8, row:1 => (data[0].bit_27)->(data[0].bit_24), mask:0x0f00_0000
///    | data[0]                                 |
///    | /- 8px -\ /- 8px -\ /- 8px -\ /- 8px -\ |
///    | 0123 4567           0123 4567           |
///    |           89ab cdef           89ab cdef |
///                ^^^^^^^^^
/// 2. Spans word boundary:
///    row_width:11, row:2 => (data[0].bit_09)->(data[1].bit_31), mask:[0x0000_03ff, 0x800_0000]
///    | data[0]                                 | data[1]    |
///    | /--- 11px --\/--- 11px ---\/---- 11px --+-\/-----... |
///    | 0123 4567 89a              67 89ab cdef | 0          |
///    |              b cdef 0123 45             |  123 45... |
///                                 ^^^^^^^^^^^^^^^^
///
/// Examples of word alignment for destination frame buffer:
/// 1. Fits in word: xr:1..7   => (data[0].bit_30)->(data[0].bit_26), mask:0x7c00_0000
/// 2. Spans words:  xr:30..36 => (data[0].bit_01)->(data[1].bit_29), mask:[0x0000_0003,0xe000_000]
///
pub fn xor_char(
    fb: &mut FrBuf,
    cr: ClipRegion,
    cluster: &str,
    f: Font,
) -> Result<(usize, usize), fonts::GlyphNotFound> {
    if cr.y1 > LINES || cr.x1 > PX_PER_LINE || cr.x0 >= cr.x1 {
        return Ok((0, 0));
    }
    // Look up glyph for grapheme cluster and unpack its header
    let (gpo, bytes_used) = (f.glyph_pattern_offset)(cluster)?;
    let gh = GlyphHeader::new((f.glyph_data)(gpo));
    if gh.w > 32 {
        return Ok((0, 0));
    }
    // Add 1px pad to left
    let x0 = cr.x0 + 1;
    // Calculate word alignment for destination buffer
    let x1 = x0 + gh.w;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it
    let y0 = cr.y0 + gh.y_offset;
    let y_max = if (y0 + gh.h) <= cr.y1 {
        gh.h
    } else {
        cr.y1 - y0
    };
    for y in 0..y_max {
        // Unpack pixels for this glyph row.
        // px_in_low_word can include some or all of the pixels for this row of
        // the pattern. It may also include pixels for the next row, or, in the
        // case of the last row, it may include padding bits.
        let px_offset = y * gh.w;
        let low_word = gpo + 1 + (px_offset >> 5);
        let px_in_low_word = 32 - (px_offset & 0x1f);
        let mut pattern = (f.glyph_data)(low_word);
        // Mask and align pixels from low word of glyph data array
        pattern <<= 32 - px_in_low_word;
        pattern >>= 32 - gh.w;
        if gh.w > px_in_low_word {
            // When pixels for this row span two words in the glyph data array,
            // get pixels from the high word too
            let px_in_high_word = gh.w - px_in_low_word;
            let mut pattern_h = (f.glyph_data)(low_word + 1);
            pattern_h >>= 32 - px_in_high_word;
            pattern |= pattern_h;
        }
        // XOR glyph pixels onto destination buffer
        let base = (y0 + y) * WORDS_PER_LINE;
        fb[base + dest_low_word] ^= pattern << (32 - px_in_dest_low_word);
        if px_in_dest_low_word < gh.w {
            fb[base + dest_high_word] ^= pattern >> px_in_dest_low_word;
        }
    }
    let width_of_blitted_pixels = (x0 + gh.w + 2) - cr.x0;
    return Ok((width_of_blitted_pixels, bytes_used));
}

/// Clear a screen region bounded by (cr.x0,cr.y0)..(cr.x0,cr.y1)
pub fn clear_region(fb: &mut FrBuf, cr: ClipRegion) {
    if cr.y1 > LINES || cr.y0 >= cr.y1 || cr.x1 > PX_PER_LINE || cr.x0 >= cr.x1 {
        return;
    }
    // Calculate word alignment for destination buffer
    let dest_low_word = cr.x0 >> 5;
    let dest_high_word = cr.x1 >> 5;
    let px_in_dest_low_word = 32 - (cr.x0 & 0x1f);
    let px_in_dest_high_word = cr.x1 & 0x1f;
    // Blit it
    for y in cr.y0..cr.y1 {
        let base = y * WORDS_PER_LINE;
        fb[base + dest_low_word] |= 0xffffffff << (32 - px_in_dest_low_word);
        for w in dest_low_word + 1..dest_high_word {
            fb[base + w] = 0xffffffff;
        }
        if dest_low_word < dest_high_word {
            fb[base + dest_high_word] |= 0xffffffff >> (32 - px_in_dest_high_word);
        }
    }
}
