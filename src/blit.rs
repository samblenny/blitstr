// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

use crate::cliprect::ClipRect;
use crate::cursor::Cursor;
use crate::fonts;
use crate::fonts::{GlyphSet, NoGlyphErr};
use crate::framebuffer::{FrBuf, LINES, WIDTH, WORDS_PER_LINE};
use crate::glyphstyle::GlyphStyle;

/// Clear a screen region bounded by (clip.min.x,clip.min.y)..(clip.min.x,clip.max.y)
pub fn clear_region(fb: &mut FrBuf, clip: ClipRect) {
    if clip.max.y > LINES
        || clip.min.y >= clip.max.y
        || clip.max.x > WIDTH
        || clip.min.x >= clip.max.x
    {
        return;
    }
    // Calculate word alignment for destination buffer
    let dest_low_word = clip.min.x >> 5;
    let dest_high_word = clip.max.x >> 5;
    let px_in_dest_low_word = 32 - (clip.min.x & 0x1f);
    let px_in_dest_high_word = clip.max.x & 0x1f;
    // Blit it
    for y in clip.min.y..clip.max.y {
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

/// XOR blit a string with specified style, clip rect, starting at cursor
pub fn paint_str(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, st: GlyphStyle, s: &str) {
    // Look up the latin GlyphSet for the requested GlyphStyle (emoji & hanzi are always included)
    let gs_latin = match st {
        GlyphStyle::Bold => GlyphSet::Bold,
        GlyphStyle::Regular => GlyphSet::Regular,
        GlyphStyle::Small => GlyphSet::Small,
    };
    // Parse the string, consuming one grapheme cluster for each iteration of
    // the for loop. Since grapheme cluster length varies, s.len() is just an
    // upper bound that's only exact for pure ASCII strings.
    let mut cluster = s;
    for _ in 0..s.len() {
        if cluster.len() < 1 {
            break; // All grapheme clusters have been consumed
        }
        if Some('\n') == cluster.chars().next() {
            // Handle whitespace, note that '\n' uses 1 byte
            newline(clip, c);
            cluster = &cluster[1..];
        } else if let Ok(bytes_used) = xor_char(fb, clip, c, cluster, GlyphSet::Emoji) {
            cluster = &cluster[bytes_used..];
        } else if let Ok(bytes_used) = xor_char(fb, clip, c, cluster, gs_latin) {
            cluster = &cluster[bytes_used..];
        } else if let Ok(bytes_used) = xor_char(fb, clip, c, cluster, GlyphSet::Hanzi) {
            cluster = &cluster[bytes_used..];
        } else {
            // Fallback: use replacement character
            if let Ok(_) = xor_char(fb, clip, c, &"\u{FFFD}", gs_latin) {
                // Advance string slice position by consuming one UTF-8 character
                if let Some((i, _)) = cluster.char_indices().nth(1) {
                    cluster = &cluster[i..];
                } else {
                    //cluster = &cluster[1..];
                    //break; // That was the last char, so stop now
                }
            }
        }
    }
}

/// Advance the cursor to the start of a new line within the clip rect
fn newline(clip: ClipRect, c: &mut Cursor) {
    c.pt.x = clip.min.x;
    if c.line_height < fonts::small::MAX_HEIGHT as usize {
        c.line_height = fonts::small::MAX_HEIGHT as usize;
    }
    c.pt.y += c.line_height + 1;
    c.line_height = 0;
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
fn xor_char(
    fb: &mut FrBuf,
    clip: ClipRect,
    c: &mut Cursor,
    cluster: &str,
    gs: GlyphSet,
) -> Result<usize, NoGlyphErr> {
    if clip.max.y > LINES || clip.max.x > WIDTH || clip.min.x >= clip.max.x {
        return Ok(0);
    }
    // Look up glyph for grapheme cluster and unpack its header
    let (glyph_data, bytes_used) = match gs {
        GlyphSet::Emoji => fonts::emoji::get_blit_pattern_offset(cluster)?,
        GlyphSet::Bold => fonts::bold::get_blit_pattern_offset(cluster)?,
        GlyphSet::Regular => fonts::regular::get_blit_pattern_offset(cluster)?,
        GlyphSet::Small => fonts::small::get_blit_pattern_offset(cluster)?,
        GlyphSet::Hanzi => fonts::hanzi::get_blit_pattern_offset(cluster)?,
    };
    let gh = glyph_data.header();
    if gh.w > 32 {
        return Ok(0);
    }
    // Don't clip if cursor is left of clip rect; instead, advance the cursor
    if c.pt.x < clip.min.x {
        c.pt.x = clip.min.x;
    }
    // Add 1px pad to left
    let mut x0 = c.pt.x + 1;
    // Adjust for word wrapping
    if x0 + gh.w + 2 >= clip.max.x {
        newline(clip, c);
        x0 = c.pt.x + 1;
    }
    // Calculate word alignment for destination buffer
    let x1 = x0 + gh.w;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it
    let y0 = c.pt.y + gh.y_offset;
    if y0 > clip.max.y {
        return Ok(0); // Entire glyph is outside clip rect, so clip it
    }
    let y_max = if (y0 + gh.h) <= clip.max.y {
        gh.h
    } else {
        clip.max.y - y0 // Clip bottom of glyph
    };
    for y in 0..y_max {
        // Skip rows that are above the clip region
        if y0 + y < clip.min.y {
            continue; // Clip top of glyph
        }
        // Unpack pixels for this glyph row.
        // px_in_low_word can include some or all of the pixels for this row of
        // the pattern. It may also include pixels for the next row, or, in the
        // case of the last row, it may include padding bits.
        let px_offset = y * gh.w;
        let low_word = 1 + (px_offset >> 5);
        let px_in_low_word = 32 - (px_offset & 0x1f);
        let mut pattern = glyph_data.nth_word(low_word);
        // Mask and align pixels from low word of glyph data array
        pattern <<= 32 - px_in_low_word;
        pattern >>= 32 - gh.w;
        if gh.w > px_in_low_word {
            // When pixels for this row span two words in the glyph data array,
            // get pixels from the high word too
            let px_in_high_word = gh.w - px_in_low_word;
            let mut pattern_h = glyph_data.nth_word(low_word + 1);
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
    let width_of_blitted_pixels = gh.w + 3;
    c.pt.x += width_of_blitted_pixels;
    let font_line_height = match gs {
        GlyphSet::Bold => fonts::bold::MAX_HEIGHT,
        GlyphSet::Regular => fonts::regular::MAX_HEIGHT,
        GlyphSet::Small => fonts::small::MAX_HEIGHT,
        GlyphSet::Emoji => fonts::emoji::MAX_HEIGHT,
        GlyphSet::Hanzi => fonts::hanzi::MAX_HEIGHT,
    } as usize;
    if font_line_height > c.line_height {
        c.line_height = font_line_height;
    }
    return Ok(bytes_used);
}
