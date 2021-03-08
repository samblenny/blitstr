// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

//! Export v1 api names. The point of using re-exports is to allow for splitting
//! the crate implementation into relatively small modules that are easy to
//! refactor without breaking the public api.

// Re-export names from modules into the v1 namespace
pub use crate::blit::{clear_region, paint_str, xor_char, simulate_char};
pub use crate::cliprect::ClipRect;
pub use crate::cursor::Cursor;
pub use crate::demo;
pub use crate::framebuffer::{new_fr_buf, FrBuf, FRAME_BUF_SIZE, LINES, WIDTH, WORDS_PER_LINE};
pub use crate::glyphstyle::{glyph_to_height_hint, GlyphStyle};
pub use crate::pt::Pt;

#[cfg(target_os = "none")]
pub use crate::fonts::{map_font, GlyphData};

/// These tests aim to cover all names exported in the v1 api
#[cfg(test)]
mod tests {
    use super::*;
    use crate::m3hash;

    #[test]
    fn test_api_v1_blit() {
        let fb = &mut new_fr_buf();
        let clip = ClipRect::full_screen();
        clear_region(fb, clip);
        let cursor = &mut Cursor::from_top_left_of(clip);
        paint_str(fb, clip, cursor, GlyphStyle::Regular, "abc", true, None, false, xor_char);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x529828DB);
    }

    #[test]
    fn test_api_v1_cliprect() {
        let cr1 = ClipRect {
            min: Pt { x: 1, y: 2 },
            max: Pt { x: 3, y: 4 },
        };
        assert_eq!(cr1, ClipRect::new(1, 2, 3, 4));
        assert_ne!(ClipRect::full_screen(), ClipRect::padded_screen());
    }

    #[test]
    fn test_api_v1_cursor() {
        let c1 = Cursor {
            pt: Pt { x: 1, y: 2 },
            line_height: 0,
        };
        assert_eq!(c1, Cursor::new(1, 2, 0));
        let clip = ClipRect::new(1, 2, 3, 4);
        let c2 = Cursor::from_top_left_of(clip);
        assert_eq!(c1.line_height, c2.line_height);
    }

    #[test]
    fn test_api_v1_demo() {
        let fb = &mut new_fr_buf();
        demo::sample_text(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x59AA26A1);
        demo::short_greeting(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x7AC16AC8);
        demo::goose_poem(fb);
        assert_eq!(m3hash::frame_buffer(fb, 0), 0x9bd28a96);
    }

    #[test]
    fn test_api_v1_framebuffer() {
        assert_eq!(LINES * WORDS_PER_LINE, FRAME_BUF_SIZE);
        assert!(LINES > 0);
        assert!(WIDTH > 0);
        let fb: FrBuf = new_fr_buf();
        assert!(fb.len() > 0);
    }

    #[test]
    fn test_api_v1_glyphstyle() {
        let s: usize = GlyphStyle::Small.into();
        let r: usize = GlyphStyle::Regular.into();
        let b: usize = GlyphStyle::Bold.into();
        assert_eq!(GlyphStyle::Small, GlyphStyle::from(s));
        assert_eq!(GlyphStyle::Regular, GlyphStyle::from(r));
        assert_eq!(GlyphStyle::Bold, GlyphStyle::from(b));
        assert_eq!(24, glyph_to_height_hint(GlyphStyle::Small));
        assert_eq!(30, glyph_to_height_hint(GlyphStyle::Regular));
        assert_eq!(30, glyph_to_height_hint(GlyphStyle::Bold));
    }

    #[test]
    fn test_api_v1_pt() {
        let p1 = Pt { x: 1, y: 2 };
        let p2 = Pt::new(1, 2);
        assert_eq!(p1, p2);
    }
}
