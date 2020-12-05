# blitstr API

This file is for people who prefer not to use `cargo doc`. In case of
disagreement between this file and the code in [../src/](../src/), the code
wins.


## Public exports from [lib.rs](../src/lib.rs) and [demo.rs](../src/demo.rs)

```rust
/// Frame buffer bounds
pub const WORDS_PER_LINE: usize = 11;
pub const WIDTH: usize = 336;
pub const LINES: usize = 536;
pub const FRAME_BUF_SIZE: usize = WORDS_PER_LINE * LINES;

/// Frame buffer of 1-bit pixels
pub type FrBuf = [u32; FRAME_BUF_SIZE];

/// Initialize a frame buffer with stripes
pub const fn new_fr_buf() -> FrBuf {}

/// Point specifies a pixel coordinate
pub struct Pt {
    pub x: usize,
    pub y: usize,
}

/// Cursor specifies a drawing position along a line of text. Lines of text can
/// be different heights. Line_height is for keeping track of the tallest
/// character that has been drawn so far on the current line.
pub struct Cursor {
    pub pt: Pt,
    pub line_height: usize,
}
impl Cursor {
    // Make a new Cursor. When in doubt, set line_height = 0.
    pub fn new(x: usize, y: usize, line_height: usize) -> Cursor {}

    // Make a Cursor aligned at the top left corner of a ClipRect
    pub fn from_top_left_of(r: ClipRect) -> Cursor {}
}

/// ClipRect specifies a region of pixels. X and y pixel ranges are inclusive of
/// min and exclusive of max (i.e. it's min.x..max.x rather than min.x..=max.x)
/// Coordinate System Notes:
/// - (0,0) is top left
/// - Increasing Y moves downward on the screen, increasing X moves right
/// - (WIDTH, LINES) is bottom right
pub struct ClipRect {
    pub min: Pt,
    pub max: Pt,
}
impl ClipRect {
    /// Initialize a rectangle using automatic min/max fixup for corner points
    pub fn new(min_x: usize, min_y: usize, max_x: usize, max_y: usize) -> ClipRect {}

    /// Make a rectangle of the full screen size
    pub fn full_screen() -> ClipRect {}

    /// Make a rectangle of the screen size minus padding
    pub fn padded_screen() -> ClipRect {}
}

/// Style options for Latin script fonts
pub enum GlyphStyle {
    Small = 0,
    Regular = 1,
    Bold = 2,
}

/// Convert number to style for use with register-based message passing sytems
impl From<usize> for GlyphStyle {
    fn from(gs: usize) -> Self {}
}

/// Convert style to number for use with register-based message passing sytems
impl Into<usize> for GlyphStyle {
    fn into(self) -> usize {}
}

/// Estimate line-height for Latin script text in the given style
pub fn glyph_to_height_hint(g: GlyphStyle) -> usize {}

/// XOR blit a string with specified style, clip rect, starting at cursor
pub fn paint_str(fb: &mut FrBuf, clip: ClipRect, c: &mut Cursor, st: GlyphStyle, s: &str) {}

/// Clear a screen region bounded by (clip.min.x,clip.min.y)..(clip.min.x,clip.max.y)
pub fn clear_region(fb: &mut FrBuf, clip: ClipRect) {}

pub mod demo {
    /// Demonstrate available fonts
    pub fn sample_text(fb: &mut FrBuf) {}

    /// Short example to greet world + cat
    pub fn short_greeting(fb: &mut FrBuf) {}
}

```
