// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
#![forbid(unsafe_code)]

pub mod demo;
mod fonts;
use fonts::{Font, GlyphHeader, GlyphSet};

/// Frame buffer bounds
pub const WORDS_PER_LINE: usize = 11;
pub const WIDTH: usize = 336;
pub const LINES: usize = 536;
pub const FRAME_BUF_SIZE: usize = WORDS_PER_LINE * LINES;

/// Frame buffer of 1-bit pixels
pub type FrBuf = [u32; FRAME_BUF_SIZE];
/// Initialize a frame buffer with stripes
pub const fn new_fr_buf() -> FrBuf {
    [0xffff0000; FRAME_BUF_SIZE]
}

/// Point specifies a pixel coordinate
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pt {
    pub x: usize,
    pub y: usize,
}

/// Cursor specifies a drawing position along a line of text. Lines of text can
/// be different heights. Line_height is for keeping track of the tallest
/// character that has been drawn so far on the current line.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cursor {
    pub pt: Pt,
    pub line_height: usize,
}
impl Cursor {
    // Make a new Cursor. When in doubt, set line_height = 0.
    pub fn new(x: usize, y: usize, line_height: usize) -> Cursor {
        Cursor {
            pt: Pt { x, y },
            line_height,
        }
    }
    // Make a Cursor aligned at the top left corner of a ClipRect
    pub fn from_top_left_of(r: ClipRect) -> Cursor {
        Cursor {
            pt: r.min,
            line_height: 0,
        }
    }
}

/// ClipRect specifies a region of pixels. X and y pixel ranges are inclusive of
/// min and exclusive of max (i.e. it's min.x..max.x rather than min.x..=max.x)
/// Coordinate System Notes:
/// - (0,0) is top left
/// - Increasing Y moves downward on the screen, increasing X moves right
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClipRect {
    pub min: Pt,
    pub max: Pt,
}
impl ClipRect {
    /// Initialize a rectangle using automatic min/max fixup for corner points
    pub fn new(min_x: usize, min_y: usize, max_x: usize, max_y: usize) -> ClipRect {
        // Make sure min_x <= max_x && min_y <= max_y
        let mut min = Pt { x: min_x, y: min_y };
        let mut max = Pt { x: max_x, y: max_y };
        if min_x > max_x {
            min.x = max_x;
            max.x = min_x;
        }
        if min_y > max_y {
            min.y = max_y;
            max.y = min_y;
        }
        ClipRect { min, max }
    }
    /// Make a rectangle of the full screen size (0,0)..(WIDTH,LINES)
    pub fn full_screen() -> ClipRect {
        ClipRect::new(0, 0, WIDTH, LINES)
    }
    /// Make a rectangle of the screen size minus padding (6,6)..(WIDTH-6,LINES-6)
    pub fn padded_screen() -> ClipRect {
        let pad = 6;
        ClipRect::new(pad, pad, WIDTH - pad, LINES - pad)
    }
}

/// Style options for Latin script fonts
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GlyphStyle {
    Small = 0,
    Regular = 1,
    Bold = 2,
}

/// Convert number to style for use with register-based message passing sytems
impl From<usize> for GlyphStyle {
    fn from(gs: usize) -> Self {
        match gs {
            0 => GlyphStyle::Small,
            1 => GlyphStyle::Regular,
            2 => GlyphStyle::Bold,
            _ => GlyphStyle::Regular,
        }
    }
}

/// Convert style to number for use with register-based message passing sytems
impl Into<usize> for GlyphStyle {
    fn into(self) -> usize {
        match self {
            GlyphStyle::Small => 0,
            GlyphStyle::Regular => 1,
            GlyphStyle::Bold => 2,
        }
    }
}

/// Estimate line-height for Latin script text in the given style
pub fn glyph_to_height_hint(g: GlyphStyle) -> usize {
    match g {
        GlyphStyle::Small => fonts::small::MAX_HEIGHT as usize,
        GlyphStyle::Regular => fonts::regular::MAX_HEIGHT as usize,
        GlyphStyle::Bold => fonts::regular::MAX_HEIGHT as usize,
    }
}

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
    // Based on the requested style of Latin text, figure out a priority order
    // of glyph sets to use for looking up grapheme clusters
    let gs1 = GlyphSet::Emoji;
    let gs2 = match st {
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
            // Handle whitespace
            newline(clip, c);
            if let Some((i, _)) = cluster.char_indices().nth(1) {
                cluster = &cluster[i..];
            } else {
                break; // That was the last char, so stop now
            }
        } else if let Ok(bytes_used) = xor_char(fb, clip, c, cluster, gs1) {
            cluster = &cluster[bytes_used..];
        } else if let Ok(bytes_used) = xor_char(fb, clip, c, cluster, gs2) {
            cluster = &cluster[bytes_used..];
        } else {
            // Fallback: use replacement character
            if let Ok(_) = xor_char(fb, clip, c, &"\u{FFFD}", gs1) {
                // Advance string slice position by consuming one UTF-8 character
                if let Some((i, _)) = cluster.char_indices().nth(1) {
                    cluster = &cluster[i..];
                } else {
                    break; // That was the last char, so stop now
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
) -> Result<usize, fonts::GlyphNotFound> {
    if clip.max.y > LINES || clip.max.x > WIDTH || clip.min.x >= clip.max.x {
        return Ok(0);
    }
    // Look up glyph for grapheme cluster and unpack its header
    let f = Font::new(gs);
    let (gpo, bytes_used) = (f.glyph_pattern_offset)(cluster)?;
    let gh = GlyphHeader::new((f.glyph_data)(gpo));
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
    let width_of_blitted_pixels = gh.w + 3;
    c.pt.x += width_of_blitted_pixels;
    let font_line_height = match gs {
        GlyphSet::Bold => fonts::bold::MAX_HEIGHT,
        GlyphSet::Regular => fonts::regular::MAX_HEIGHT,
        GlyphSet::Small => fonts::small::MAX_HEIGHT,
        GlyphSet::Emoji => fonts::emoji::MAX_HEIGHT,
    } as usize;
    if font_line_height > c.line_height {
        c.line_height = font_line_height;
    }
    return Ok(bytes_used);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_equivalence() {
        let c1 = Cursor {
            pt: Pt { x: 1, y: 2 },
            line_height: 8,
        };
        let c2 = Cursor::new(1, 2, 8);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_clip_rect_equivalence() {
        let cr1 = ClipRect {
            min: Pt { x: 1, y: 2 },
            max: Pt { x: 8, y: 9 },
        };
        // Called properly:
        let cr2 = ClipRect::new(1, 2, 8, 9);
        // Called with mixed up corners that should get auto-corrected
        let cr3 = ClipRect::new(8, 2, 1, 9);
        let cr4 = ClipRect::new(1, 9, 8, 2);
        assert_eq!(cr1, cr2);
        assert_eq!(cr2, cr3);
        assert_eq!(cr3, cr4);
    }

    #[test]
    fn test_cursor_from_clip_rect() {
        let cr = ClipRect::new(1, 2, 8, 9);
        let c = Cursor::from_top_left_of(cr);
        assert_eq!(c.pt, cr.min);
    }

    #[test]
    fn test_style_arg_conversions() {
        assert_eq!(GlyphStyle::Small, (GlyphStyle::Small as usize).into());
        assert_eq!(GlyphStyle::Regular, (GlyphStyle::Regular as usize).into());
        assert_eq!(GlyphStyle::Bold, (GlyphStyle::Bold as usize).into());
        let bad_arg = 255;
        assert_eq!(GlyphStyle::Regular, bad_arg.into());
    }
}
