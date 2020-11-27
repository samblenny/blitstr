// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
use super::{clear_region, paint_str, Cursor, FrBuf, Rect, Style};

/// Demonstrate available fonts
pub fn sample_text(fb: &mut FrBuf) {
    let note = &"Hello, world! ää 🀄🃏\u{1F170}\u{1F170}\u{FE0F}\n"; // This has Unicode NFC and NFD
    let sas1 = &"\n   🍎       🎸       🕶        🍎\n";
    let sas2 = &" apple  guitar  glasses  apple\n";
    let sas3 = &"\n           😸     🎩    🔑\n";
    let sas4 = &"           cat    hat    key\n";

    clear_region(fb, Rect::full_screen());
    let clip = Rect::padded_screen();
    let c = &mut Cursor::from_top_left_of(clip);
    paint_str(fb, clip, c, Style::Bold, note);
    paint_str(fb, clip, c, Style::Regular, note);
    paint_str(fb, clip, c, Style::Small, note);
    paint_str(fb, clip, c, Style::Regular, sas1);
    paint_str(fb, clip, c, Style::Regular, sas2);
    paint_str(fb, clip, c, Style::Regular, sas3);
    paint_str(fb, clip, c, Style::Regular, sas4);
}
