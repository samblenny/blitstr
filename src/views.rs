// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
use super::{blit, blit::Style, state};

/// Screen bounds
pub const SCREEN_W: usize = blit::PX_PER_LINE;
pub const SCREEN_H: usize = blit::LINES;

/// Home screen with status bar, main content box, and keyboard
pub fn home_screen(fb: &mut state::FrameBuf) {
    let note = &"Hello, world! ää 🀄🃏\u{1F170}\u{1F170}\u{FE0F}"; // This has Unicode NFC and NFD
    let sas1 = &"   🍎       🎸       🕶        🍎";
    let sas2 = &" apple  guitar  glasses  apple";
    let sas3 = &"           😸     🎩    🔑";
    let sas4 = &"           cat    hat    key";
    // Status bar: view title, battery level icon, wifi strength icon, clock
    let mut cr = blit::ClipRegion {
        x0: 0,
        x1: SCREEN_W,
        y0: 0,
        y1: SCREEN_H,
    };
    blit::clear_region(&mut fb.buf, cr);
    cr.x0 = 8;
    cr.y0 = 5;
    blit::string_left(Style::Bold, &mut fb.buf, cr, note);
    cr.y0 += 33;
    blit::string_left(Style::Regular, &mut fb.buf, cr, note);
    cr.y0 += 33;
    blit::string_left(Style::Small, &mut fb.buf, cr, note);
    cr.y0 += 66;
    blit::string_left(Style::Regular, &mut fb.buf, cr, sas1);
    cr.y0 += 33;
    blit::string_left(Style::Regular, &mut fb.buf, cr, sas2);
    cr.y0 += 66;
    blit::string_left(Style::Regular, &mut fb.buf, cr, sas3);
    cr.y0 += 33;
    blit::string_left(Style::Regular, &mut fb.buf, cr, sas4);
}
