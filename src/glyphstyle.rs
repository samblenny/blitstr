// Copyright (c) 2020 Sam Blenny
// Copyright (c) 2020 bunnie
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Xous compatibility stuff derives from commits 1b5efc0 and 69f7c04
//
#![forbid(unsafe_code)]

use crate::fonts;

/// Style options for Latin script fonts
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GlyphStyle {
    Small = 0,
    Regular = 1,
    Bold = 2,
}

/// Convert number to style for use with register-based message passing sytems
// [by bunnie for Xous]
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
// [by bunnie for Xous]
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
// [by bunnie for Xous]
pub fn glyph_to_height_hint(g: GlyphStyle) -> usize {
    match g {
        GlyphStyle::Small => fonts::small::MAX_HEIGHT as usize,
        GlyphStyle::Regular => fonts::regular::MAX_HEIGHT as usize,
        GlyphStyle::Bold => fonts::regular::MAX_HEIGHT as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyphstyle_arg_conversions() {
        let s: usize = GlyphStyle::Small.into();
        let r: usize = GlyphStyle::Regular.into();
        let b: usize = GlyphStyle::Bold.into();
        assert_eq!(GlyphStyle::Small, GlyphStyle::from(s));
        assert_eq!(GlyphStyle::Regular, GlyphStyle::from(r));
        assert_eq!(GlyphStyle::Bold, GlyphStyle::from(b));
        let bad_arg = 255;
        assert_eq!(GlyphStyle::Regular, GlyphStyle::from(bad_arg));
    }

    #[test]
    fn test_glyphstyle_glyph_to_height_hint() {
        let h1 = glyph_to_height_hint(GlyphStyle::Small);
        let h2 = glyph_to_height_hint(GlyphStyle::Regular);
        let h3 = glyph_to_height_hint(GlyphStyle::Bold);
        assert_eq!(h1, 24);
        assert_eq!(h2, 30);
        assert_eq!(h3, 30);
    }
}
