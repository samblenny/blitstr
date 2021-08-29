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
    if clip.max.y > LINES as i32
        || clip.min.y >= clip.max.y
        || clip.max.x > WIDTH as i32
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
        let base = y * WORDS_PER_LINE as i32;
        fb[(base + dest_low_word) as usize] |= 0xffffffff << (32 - px_in_dest_low_word);
        for w in dest_low_word + 1..dest_high_word {
            fb[(base + w) as usize] = 0xffffffff;
        }
        if dest_low_word < dest_high_word {
            fb[(base + dest_high_word) as usize] |= 0xffffffff >> (32 - px_in_dest_high_word);
        }
    }
}

fn draw_ins(fb: &mut FrBuf, c: &Cursor) {
    // draw an insertion pointer that's a couple pixels short either side of the line height
    let y = (c.pt.y + 2) as usize;
    let x = c.pt.x as usize;
    let mut height = (c.pt.y + c.line_height - 2) as usize;
    let fb_rows = fb.len() / WORDS_PER_LINE;
    if height >= fb_rows {
        // Truncate height to avoid attempting to draw beyond last line of framebuffer
        height = fb_rows - 1;
    }
    for row in y..height {
        fb[((x + row * WORDS_PER_LINE * 32) / 32) as usize] ^= (1 << (x % 32));
        // set the dirty bit on the line that contains the pixel
        fb[row * WORDS_PER_LINE + (WORDS_PER_LINE - 1)] |= 0x1_0000;
    }
}

/// XOR blit a string with specified style, clip rect, starting at cursor; draw an insertion point at the designed optional ins offset, given *in characters*
pub fn paint_str(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, st: GlyphStyle, s: &str, xor: bool, ins: Option<i32>, ellipsis: bool,
    paintchar_fn: fn(fb: &mut FrBuf,
     clip: ClipRect,
     c: &mut Cursor,
     cluster: &str,
     gs: GlyphSet,
     xor: bool, ellipsis: bool) -> Result<Option<i32>, NoGlyphErr> ) {

    use log::info;
    let debug = false;
    if debug { info!("BLITSTR: in paint_str for str {}", s); }
    if debug { info!("BLITSTR: clip {:?}", clip); }
    if debug { info!("BLITSTR: cursor {:?}", c); }

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
    let mut ins_drawn = false;
    let mut start_c: Cursor = c.clone();
    for i in 0..s.len() {
        if cluster.len() < 1 {
            break; // All grapheme clusters have been consumed
        }
        if debug { info!("BLITSTR: cluster iter {} len {}", cluster, cluster.len()); }
        if Some('\n') == cluster.chars().next() {
            // Handle whitespace, note that '\n' uses 1 byte
            newline(clip, c);
            cluster = &cluster[1..];
        } else if let Ok(bytes_used) = paintchar_fn(fb, clip, c, cluster, GlyphSet::Emoji, xor, ellipsis) {
            match bytes_used {
                Some(bu) => cluster = &cluster[bu as usize..],
                None => break,
            }
        } else if let Ok(bytes_used) = paintchar_fn(fb, clip, c, cluster, gs_latin, xor, ellipsis) {
            match bytes_used {
                Some(bu) => cluster = &cluster[bu as usize..],
                None => break,
            }
        } else if let Ok(bytes_used) = paintchar_fn(fb, clip, c, cluster, GlyphSet::Hanzi, xor, ellipsis) {
            match bytes_used {
                Some(bu) => cluster = &cluster[bu as usize..],
                None => break,
            }
        } else {
            // Fallback: use replacement character
            if let Ok(_) = paintchar_fn(fb, clip, c, &"\u{FFFD}", gs_latin, xor, ellipsis) {
                // Advance string slice position by consuming one UTF-8 character
                if let Some((i, _)) = cluster.char_indices().nth(1) {
                    cluster = &cluster[i..];
                } else {
                    //cluster = &cluster[1..];
                    //break; // That was the last char, so stop now
                }
            }
        }
        if !ins_drawn {
            match ins {
                Some(ins_pt) => {
                    if (i+1) as i32 == ins_pt {
                        draw_ins(fb, c);
                        ins_drawn = true;
                    }
                },
                _ => ()
            }
        }
    }
    // case of putting an insertion point at the end or beginning of of the string
    if ins.is_some() && !ins_drawn {
        if ins.unwrap() != 0 {
            draw_ins(fb, c);
        } else {
            start_c.line_height = c.line_height;
            draw_ins(fb, &start_c);
        }
    }
}

/// Advance the cursor to the start of a new line within the clip rect
fn newline(clip: ClipRect, c: &mut Cursor) {
    c.pt.x = clip.min.x;
    if c.line_height < fonts::small::MAX_HEIGHT as i32 {
        c.line_height = fonts::small::MAX_HEIGHT as i32;
    }
    c.pt.y += c.line_height + 1;
    c.line_height = 0;
}

pub fn draw_ellipsis(fb: &mut FrBuf, c: Cursor, baseline: i32, max_y: i32, clip: ClipRect) {
    // rub out a small amount of area of the last characters drawn, and then draw the ellipsis
    let ellipsis_width = 10;
    let x0 = if c.pt.x + ellipsis_width <= clip.max.x {
        c.pt.x
    } else {
        clip.max.x - ellipsis_width
    };
    let y1 = if c.pt.y + max_y <= clip.max.y {
        c.pt.y + max_y
    } else {
        clip.max.y
    };
    let clear_rect = ClipRect::new(x0, c.pt.y, x0 + ellipsis_width, y1);
    clear_region(fb, clear_rect);
    // now draw the ellipsis at x0, baseline
    let h = if baseline <= max_y {
        c.pt.y + baseline
    } else {
        clip.max.y - 4
    };
    //log::info!("BLITSTR: ellipsis clear {:?}, h {}, c {:?}, x0 {}", clear_rect, h, c, x0);
    // draw the three dots
    for i in 0..ellipsis_width {
        if i == 1 || i == 2 || i == 4 || i == 5 || i == 7 || i == 8 {
            fb[((x0 + i + h * (WORDS_PER_LINE as i32) * 32) / 32) as usize] &= !(1 << ((x0 + i) % 32));
            fb[((x0 + i + (h+1) * (WORDS_PER_LINE as i32) * 32) / 32) as usize] &= !(1 << ((x0 + i) % 32));
        }
    }
    // no dirty bits set, it's assumed they were already set because previous character blittings
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
    clip: ClipRect,
    c: &mut Cursor,
    cluster: &str,
    gs: GlyphSet,
    xor: bool,
    ellipsis: bool,
) -> Result<Option<i32>, NoGlyphErr> {
    use log::info;
    let debug = false;
    if debug { info!("BLITSTR: in xor_char for str {} gs {:?}", cluster, gs); }
    if debug { info!("BLITSTR: clip {:?}", clip); }
    if clip.max.y > LINES as i32 || clip.max.x > WIDTH as i32 || clip.min.x >= clip.max.x {
        return Ok(None);
    }
    // Look up glyph for grapheme cluster and unpack its header
    if debug { info!("BLITSTR: glyph lookup"); }
    let (glyph_data, bytes_used) = match gs {
        GlyphSet::Emoji => fonts::emoji::get_blit_pattern_offset(cluster)?,
        GlyphSet::Bold => fonts::bold::get_blit_pattern_offset(cluster)?,
        GlyphSet::Regular => fonts::regular::get_blit_pattern_offset(cluster)?,
        GlyphSet::Small => fonts::small::get_blit_pattern_offset(cluster)?,
        GlyphSet::Hanzi => fonts::hanzi::get_blit_pattern_offset(cluster)?,
    };
    if debug { info!("BLITSTR: getting header on glyph_data {:?}", glyph_data); }
    let gh_maybe = glyph_data.header();
    if debug { info!("BLITSTR: after header, got {:?}", gh_maybe); }
    let gh = gh_maybe?;
    if gh.w > 32 {
        return Ok(None);
    }
    // Don't clip if cursor is left of clip rect; instead, advance the cursor
    if c.pt.x < clip.min.x {
        c.pt.x = clip.min.x;
    }
    // Add 1px pad to left
    let mut x0 = c.pt.x + 1;
    // Adjust for word wrapping
    if !ellipsis {
        if x0 + gh.w as i32 + 2 >= clip.max.x {
            newline(clip, c);
            x0 = c.pt.x + 1;
        }
    } else {
        // insert ellipsis if the wrap would go outside the clip rectangle
        let c_bak = Cursor {
            pt: crate::Pt::new(c.pt.x, c.pt.y),
            line_height: c.line_height,
        };
        if x0 + gh.w as i32 + 2 >= clip.max.x {
            newline(clip, c);
            x0 = c.pt.x + 1;
            // determine if any of the new glyphs on this line might fall outside the clip region
            if (c.pt.y + gh.y_offset as i32 + fonts::small::MAX_HEIGHT as i32) > clip.max.y {
                // clipping would happen
                draw_ellipsis(fb, c_bak, 20, fonts::small::MAX_HEIGHT as i32, clip);
                return Ok(None)
            }
        }
    }
    // Calculate word alignment for destination buffer
    let x1 = x0 + gh.w as i32;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it
    let y0 = c.pt.y + gh.y_offset as i32;
    if y0 > clip.max.y {
        return Ok(None); // Entire glyph is outside clip rect, so clip it
    }
    let y_max = if (y0 + gh.h as i32) <= clip.max.y {
        gh.h as i32
    } else {
        clip.max.y - y0 // Clip bottom of glyph
    };
    if debug { info!("BLITSTR: drawing with y0: {}, x0: {}, x1: {}", y0, x0, x1); }
    for y in 0..y_max {
        // Skip rows that are above the clip region
        if y0 + y < clip.min.y {
            continue; // Clip top of glyph
        }
        // Unpack pixels for this glyph row.
        // px_in_low_word can include some or all of the pixels for this row of
        // the pattern. It may also include pixels for the next row, or, in the
        // case of the last row, it may include padding bits.
        let px_offset = y * gh.w as i32;
        let low_word = 1 + (px_offset >> 5);
        let px_in_low_word = 32 - (px_offset & 0x1f);
        let mut pattern = glyph_data.nth_word(low_word as usize)?;
        // Mask and align pixels from low word of glyph data array
        pattern <<= 32 - px_in_low_word;
        pattern >>= 32 - gh.w;
        if gh.w as i32 > px_in_low_word as i32 {
            // When pixels for this row span two words in the glyph data array,
            // get pixels from the high word too
            let px_in_high_word = gh.w as i32 - px_in_low_word as i32;
            let mut pattern_h = glyph_data.nth_word(low_word as usize + 1)?;
            pattern_h >>= 32 - px_in_high_word;
            pattern |= pattern_h;
        }
        // XOR glyph pixels onto destination buffer
        let base = (y0 + y) * WORDS_PER_LINE as i32;
        if xor {
            fb[(base + dest_low_word) as usize] ^= pattern << (32 - px_in_dest_low_word);
        } else {
            fb[(base + dest_low_word) as usize] &= 0xffff_ffff ^ (pattern << (32 - px_in_dest_low_word));
        }
        if px_in_dest_low_word < gh.w as i32 {
            if xor {
                fb[(base + dest_high_word) as usize] ^= pattern >> px_in_dest_low_word;
            } else {
                fb[(base + dest_high_word) as usize] &= 0xffff_ffff ^ (pattern >> px_in_dest_low_word);
            }
        }
        fb[(base + WORDS_PER_LINE as i32 - 1) as usize] |= 0x1_0000; // set the dirty bit on the line
    }
    let width_of_blitted_pixels = gh.w + 3;
    c.pt.x += width_of_blitted_pixels as i32;
    let font_line_height = match gs {
        GlyphSet::Bold => fonts::bold::MAX_HEIGHT,
        GlyphSet::Regular => fonts::regular::MAX_HEIGHT,
        GlyphSet::Small => fonts::small::MAX_HEIGHT,
        GlyphSet::Emoji => fonts::emoji::MAX_HEIGHT,
        GlyphSet::Hanzi => fonts::hanzi::MAX_HEIGHT,
    } as usize;
    if font_line_height > c.line_height as usize {
        c.line_height = font_line_height as i32;
    }
    return Ok(Some(bytes_used as i32));
}


pub fn simulate_char(
    _fb: &mut FrBuf,
    clip: ClipRect,
    c: &mut Cursor,
    cluster: &str,
    gs: GlyphSet,
    _xor: bool,
    _ellipsis: bool,
) -> Result<Option<i32>, NoGlyphErr> {
    if clip.max.y > LINES as i32 || clip.max.x > WIDTH as i32 || clip.min.x >= clip.max.x {
        return Ok(None);
    }
    // Look up glyph for grapheme cluster and unpack its header
    let (glyph_data, bytes_used) = match gs {
        GlyphSet::Emoji => fonts::emoji::get_blit_pattern_offset(cluster)?,
        GlyphSet::Bold => fonts::bold::get_blit_pattern_offset(cluster)?,
        GlyphSet::Regular => fonts::regular::get_blit_pattern_offset(cluster)?,
        GlyphSet::Small => fonts::small::get_blit_pattern_offset(cluster)?,
        GlyphSet::Hanzi => fonts::hanzi::get_blit_pattern_offset(cluster)?,
    };
    let gh = glyph_data.header()?;
    if gh.w > 32 {
        return Ok(None);
    }
    // Don't clip if cursor is left of clip rect; instead, advance the cursor
    if c.pt.x < clip.min.x {
        c.pt.x = clip.min.x;
    }
    // Add 1px pad to left
    let x0 = c.pt.x + 1;
    if x0 + gh.w as i32 + 2 >= clip.max.x {
        newline(clip, c);
    }
    let y0 = c.pt.y + gh.y_offset as i32;
    if y0 > clip.max.y {
        return Ok(None); // Entire glyph is outside clip rect, so clip it
    }
    let width_of_blitted_pixels = gh.w + 3;
    c.pt.x += width_of_blitted_pixels as i32;
    let font_line_height = match gs {
        GlyphSet::Bold => fonts::bold::MAX_HEIGHT,
        GlyphSet::Regular => fonts::regular::MAX_HEIGHT,
        GlyphSet::Small => fonts::small::MAX_HEIGHT,
        GlyphSet::Emoji => fonts::emoji::MAX_HEIGHT,
        GlyphSet::Hanzi => fonts::hanzi::MAX_HEIGHT,
    } as usize;
    if font_line_height > c.line_height as usize {
        c.line_height = font_line_height as i32;
    }
    return Ok(Some(bytes_used as i32));
}
