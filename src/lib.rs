// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
// #![forbid(unsafe_code)] // must allow for mapping filesystem-based fonts in this crate

mod api;
mod blit;
mod cliprect;
mod cursor;
pub mod demo;
mod fonts;
mod framebuffer;
mod glyphstyle;
mod m3hash;
mod pt;

// Export v1 api names. The point of using re-exports is to allow for splitting
// the crate implementation into relatively small modules that are easy to
// refactor without breaking the public api.
pub use api::v1::*;

/// These are integration tests aimed at ensuring pixel-accurate stability of
/// string painting operations by exercising edge cases around glyph lookup,
/// word-wrapping, etc.
#[cfg(test)]
mod tests {
    use crate::api::v1::*;
    use crate::m3hash;

    #[test]
    fn test_clear_region() {
        let fb = &mut new_fr_buf();
        clear_region(fb, ClipRect::full_screen());
        let seed = 0;
        assert_eq!(m3hash::frame_buffer(fb, seed), 0x3A25F08C);
    }

    #[test]
    /// Test for hashed frame buffer match using the font sampler demo screen.
    /// This covers many string blitting features and edge cases all at once.
    /// If this test fails, try loading the wasm demo to look for what changed.
    fn test_demo_sample_text_frame_buffer_hash() {
        let fb = &mut new_fr_buf();
        demo::sample_text(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x59AA26A1);
        assert_eq!(m3hash::frame_buffer(fb, 1), 0xAE37C33B);
    }

    #[test]
    /// Test paint_str() with GlyphStyle::Small and short ascii string
    fn test_paint_str_glyphstyle_small_abc() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Small, "abc", true, xor_char);
        // Note how hash differs from tests with Regular and Bold (below)
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x5DE65BFC);
    }

    #[test]
    /// Test paint_str() with GlyphStyle::Regluar and short ascii string
    fn test_paint_str_glyphstyle_regular_abc() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Regular, "abc", true, xor_char);
        // Note how hash differs from tests with Small (above) and Bold (below)
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x529828DB);
    }

    #[test]
    /// Test paint_str() with GlyphStyle::Bold and short ascii string
    fn test_paint_str_glyphstyle_bold_abc() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Bold, "abc", true, xor_char);
        // Note how hash differs from tests with Small and Regular (above)
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x411C2E62);
    }

    #[test]
    /// Test paint_str() with an emoji cat in multiple styles.
    /// The point is that emoji glyphs are the same regardless of GlyphStyle.
    fn test_paint_str_emoji_cat_multi_style() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Small, "😸", true, xor_char); // Small
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x3A4FFDB5); // Same hash

        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Regular, "😸", true, xor_char); // Regular
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x3A4FFDB5); // Same hash

        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Bold, "😸", true, xor_char); // Bold
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x3A4FFDB5); // Same hash
    }

    #[test]
    /// Test paint_str() with a 4 line poem using hanzi.
    fn test_paint_str_hanzi_goose_poem() {
        let fb = &mut new_fr_buf();
        demo::goose_poem(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x9bd28a96);
    }

    #[test]
    /// Test paint_str() for full string at once vs. concatenating chars.
    /// The point of this is, you can call paint_str() repeatedly reusing the
    /// same cursor, and it will keep track of concatenation and word-wrap.
    fn test_paint_str_full_string_vs_char_by_char() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        let s = "The quick brown fox jumps over the lazy dog.";
        // Paint the whole string at once
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Regular, s, true, xor_char);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0xE5240DD1); // Same hash

        // Paint it again one char at a time
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        for i in 0..s.len() {
            // This slicing is sort of like &str.iter(), but I needed a thing to
            // yield &str instead of char, because paint_str() is designed to
            // take grapheme clusters that can be more than 1 char long.
            if let Some((j, _)) = s.char_indices().nth(i) {
                let c = match s.char_indices().nth(j + 1) {
                    Some((k, _)) => &s[j..k],
                    _ => &s[j..],
                };
                paint_str(fb, clip, cursor, GlyphStyle::Regular, c, true, xor_char);
            } else {
                break; // That was the last char, so stop now
            }
        }
        assert_eq!(m3hash::frame_buffer(fb, 0), 0xE5240DD1); // Same hash
    }
}
