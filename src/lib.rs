// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
#![forbid(unsafe_code)]

pub mod demo;
pub mod fonts;
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
#[derive(Copy, Clone, Debug)]
pub struct Pt {
    pub x: usize,
    pub y: usize,
}

/// Cursor specifies a drawing position along a line of text. Lines of text can
/// be different heights. Line_height is for keeping track of the tallest
/// character that has been drawn so far on the current line.
#[derive(Copy, Clone, Debug)]
pub struct Cursor {
    pub pt: Pt,
    pub line_height: usize,
}
impl Cursor {
    pub fn new(x: i16, y: i16, h: u16) -> Cursor {
        Cursor {
            pt: Pt { x: x as usize, y: y as usize },
            line_height: h as usize,
        }
    }

    pub fn from_top_left_of(r: Rect) -> Cursor {
        Cursor {
            pt: r.min,
            line_height: 0,
        }
    }
}

/// Rect specifies a region of pixels. X and y pixel ranges are inclusive of
/// min and exclusive of max (i.e. it's min.x..max.x rather than min.x..=max.x)
/// Coordinate System Notes:
/// - (0,0) is top left
/// - Increasing Y moves downward on the screen, increasing X moves right
/// - (WIDTH, LINES) is bottom right
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub min: Pt,
    pub max: Pt,
}
impl Rect {
    /// Initialize a rectangle
    pub fn new(min_x: usize, min_y: usize, max_x: usize, max_y: usize) -> Rect {
        Rect {
            min: Pt { x: min_x, y: min_y },
            max: Pt { x: max_x, y: max_y },
        }
    }
    /// Make a rectangle of the full screen size
    pub fn full_screen() -> Rect {
        Rect::new(0, 0, WIDTH, LINES)
    }
    /// Make a rectangle of the screen size minus padding
    pub fn padded_screen() -> Rect {
        let pad = 6;
        Rect::new(pad, pad, WIDTH - pad, LINES - pad)
    }
}

/// Style options for Latin script fonts
pub enum Style {
    Bold,
    Regular,
    Small,
}

/// XOR blit a string with specified style, clip rect, starting at cursor
pub fn paint_str(fb: &mut FrBuf, clip: Rect, c: &mut Cursor, st: Style, s: &str) {
    // Based on the requested style of Latin text, figure out a priority order
    // of glyph sets to use for looking up grapheme clusters
    let gs1 = GlyphSet::Emoji;
    let gs2 = match st {
        Style::Bold => GlyphSet::Bold,
        Style::Regular => GlyphSet::Regular,
        Style::Small => GlyphSet::Small,
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
fn newline(clip: Rect, c: &mut Cursor) {
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
    clip: Rect,
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
        return Ok(0);  // Entire glyph is outside clip rect, so clip it
    }
    let y_max = if (y0 + gh.h) <= clip.max.y {
        gh.h
    } else {
        clip.max.y - y0  // Clip bottom of glyph
    };
    for y in 0..y_max {
        // Skip rows that are above the clip region
        if y0 + y < clip.min.y {
            continue;  // Clip top of glyph
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

/// Clear a screen region bounded by (clip.min.x,clip.min.y)..(clip.min.x,clip.max.y)
pub fn clear_region(fb: &mut FrBuf, clip: Rect) {
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
