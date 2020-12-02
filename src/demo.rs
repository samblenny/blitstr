// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
use super::{clear_region, paint_str, ClipRect, Cursor, FrBuf, Style};

/// Demonstrate available fonts
pub fn sample_text(fb: &mut FrBuf) {
    let note = &"Hello, world! ää 🀄🃏\u{1F170}\u{1F170}\u{FE0F}\n"; // This has Unicode NFC and NFD
    let sas1 = &"\n   🍎       🎸       🕶        🍎\n";
    let sas2 = &" apple  guitar  glasses  apple\n\n";
    let sas3 = &"           😸     🎩    🔑\n";
    let sas4 = &"           cat    hat    key\n\n";
    let wrap = &concat!(
        "The quick brown fox jumps over the lazy dog. ",
        "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich.\n"
    );

    clear_region(fb, ClipRect::full_screen());
    let mut clip = ClipRect::padded_screen();
    let c = &mut Cursor::from_top_left_of(clip);
    paint_str(fb, clip, c, Style::Bold, note);
    paint_str(fb, clip, c, Style::Regular, note);
    paint_str(fb, clip, c, Style::Small, note);
    paint_str(fb, clip, c, Style::Regular, sas1);
    paint_str(fb, clip, c, Style::Regular, sas2);
    paint_str(fb, clip, c, Style::Regular, sas3);
    paint_str(fb, clip, c, Style::Regular, sas4);
    paint_str(fb, clip, c, Style::Regular, wrap);
    // Demonstrate messing with the clip region and cursor:
    // 1. Convenience function to make a new cursor
    let c = &mut Cursor::new(c.pt.x, c.pt.y, c.line_height);
    // 2. Reduce the ClipRect to a small-ish box at the bottom of the screen
    //    with big margins on its left and right sides
    clip.min.y = c.pt.y + 12;
    clip.max.y -= 8;
    clip.min.x += 30;
    clip.max.x -= 40;
    // 3. Convenience function to make a new ClipRect with min/max auto-correct
    //    Note: fn def is `new(min_x: usize, min_y: usize, max_x: usize, max_y: usize)`
    let clip = ClipRect::new(clip.max.x, clip.min.y, clip.min.x, clip.max.y);
    // Blit the string
    paint_str(fb, clip, c, Style::Small, wrap);
}
