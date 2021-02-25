# blitstr

A safe* no_std multi-lingual string blitter for 1-bit monochrome

\* "safe" meaning `#![forbid(unsafe_code)]` in the blitstr library
(wasm demo unavoidably uses `unsafe`)


## Usage

Blitstr is designed to blit strings of text into a mutable frame buffer using a
best-effort strategy to look up suitable glyphs from a table of built-in fonts.

To use blitstr:
1. Allocate a frame buffer
2. Call blitstr to paint strings into your frame buffer
3. Copy the contents of the frame buffer to a display device

For example:
```rust
use blitstr::{clear_region, paint_str, ClipRect, Cursor, FrBuf, GlyphStyle};

fn main() {
    // Allocate frame buffer (or potentially get a reference to a hardware buffer)
    let fb = &mut blitstr::new_fr_buf();

    // Call your painting code, passing a mutable frame buffer reference for blitstr
    short_greeting(fb);

    // Copy the painted frame buffer to a display device
    // TODO: You need to provide a suitable implementation for your hardware
}

pub fn short_greeting(fb: &mut FrBuf) {
    // Clear entire screen
    let clip = ClipRect::full_screen();
    clear_region(fb, clip);

    // Prepare to paint with small margin of whitespace around edges of screen
    let clip = ClipRect::padded_screen();

    // Get a text cursor positioned to begin painting from clip rectangle's top left corner
    let cursor = &mut Cursor::from_top_left_of(clip);

    // Paint two lines of text within the clip rectangle, reusing the same cursor
    paint_str(fb, clip, cursor, GlyphStyle::Regular, "Hello, world!\n");
    paint_str(fb, clip, cursor, GlyphStyle::Regular, "Hello, 😸!\n");
}
```

If you modify the WebAssembly demo (see below) to run the example code above,
the top of the wasm_demo screen should look like this:

![paint_short_greeting wasm demo screenshot](doc/short_greeting.png)

The `GlyphStyle` argument to `paint_str()` is used for resolving ambiguity
about which glyph variant blitstr should use to paint a grapheme cluster when
it finds more than one suitable option in the built-in font tables.

Currently, grapheme clusters in Latin Unicode blocks have `Small`, `Regular`,
and `Bold` glyph variants. There is no `GlyphStyle` for emoji because each
emoji character has only one glyph (no ambiguity about variants).

In the future, `GlyphStyle` may be extended to include styles for selecting
between regional variants of CJK ideogram glyphs.

Coordinates for `Cursor` and `ClipRect` use a fourth quadrant coordinate
system: origin point (x=0,y=0) is top left, +x is right, and +y is down.

The `Cursor` is mutable and advances left-to-right, then top-to-bottom as calls
to `paint_str()` automatically wrap text within the `ClipRect`. The cursor
point determines where the top left corner of the next blit pattern for a glyph
will begin. Glyphs get blitted downward (+y) and rightward (+x) relative to the
current `Cursor` point.

Newlines and word-wraps advance the `Cursor` downward (+y) by the height of the
tallest glyph on the active line and leftward (-x) to the minimum x value of
the `ClipRect`.

Currently, word-wrapping does not follow any rules about proper word-break
locations. Rather, `paint_str()` just inserts a newline ahead of any glyph that
would get clipped at the right edge of the `ClipRect`.

Glyphs have different heights: 24px for Latin `Small`, 30px for Latin `Regular`
and `Bold`, and 32px for ideograms including emoji. The `Cursor` tracks
line-height based on the tallest glyph used in its current line. For example,
`"hello\n"` would be 24px high in `GlyphStyle::Small` or 30px high in
`GlyphStyle::Regular`. But, `"hello 😸\n"`, because it includes an emoji, would
increase the line-height to 32px, regardless of `GlyphStyle`.


### Developer Tools Setup

| Tool | Purpose |
|--|--|
| rustup | Get rustc, cargo, and wasm32-unknown-unknown |
| ruby v2.3+ | Local web server for WebAssembly Demo |
| Go v1.16+ | Code generation for bitmap fonts |
| GNU make | Augment cargo for building the wasm demo |

1. Install rustc with rustup. See https://www.rust-lang.org/tools/install
2. Configure PATH environment variable: add `export PATH="$PATH:$HOME/.cargo/bin"`
   to .bash_profile or whatever
3. Add WebAssembly compile target: `rustup target add wasm32-unknown-unknown`
4. Make sure you have a ruby interpreter, v2.3 or later: `ruby --version`
   - For macOS Mojave or Catalina, default system ruby should work fine.
   - Debian may need `sudo apt install ruby`
5. **Optional:** Install the go toolchain. You only need go if you want to
   modify the bitmap fonts. See https://golang.org/


## WebAssembly Demo

Hosted: https://samblenny.github.io/blitstr/wasm_demo/

Local (requires ruby and make):

```
cd wasm_demo
# Build wasm32-unknown-unknown binary and copy to ./
make install
# Start a dev webserver to serve ./ on http://localhost:8000
ruby webserver.rb
```

Currently the demo looks like this:
![wasm demo screenshot](doc/goose_poem.png)


## Command Line Demo

You can do `cargo run`, but it just updates a frame buffer without displaying
the image anywhere. That sounds useless, but it actually helps for debugging
panics that can easily happen when making changes to the blitting code.


## Notes on Bitmap Fonts

This project uses bitmap fonts in the form of rust source code. The rust fonts
in [src/fonts/](src/fonts) were generated by the go codegen program in
[codegen/](codegen).

Codegen takes its input from:
- PNG glyph sprite sheets, txt and json grapheme cluster indices, and txt
grapheme cluster alias lists in [codegen/src_data/](codegen/src_data).
- Configuration in [codegen/config.json](codegen/config.json) specifying how to
convert the sprite sheets, indices, and alias lists into rust font files.

The rust fonts have static arrays of blit patterns and functions to translate
from grapheme clusters to appropriate glyphs. Grapheme clusters are rust string
slices with one or more Unicode codepoints (potentially including combining
diacritics, variant forms, nonspacing joins, etc).

Normally, it is not necessary to re-generate the font files. Possible reasons
to rebuild the fonts include adding new emoji or support for additional writing
systems. To run the codegen program, you need a go compiler (see golang.org). To
rebuild the emoji glyph sheet, refer to https://github.com/samblenny/hd1bemoji

Procedure to update source code for the bitmap fonts:

1. Update typeface bitmap glyphs in `codegen/src_data/*.png`

2. Update grapheme cluster index and alias files in `codegen/src_data/*.{txt|json}`

3. Update `codegen/config.json` (see [codegen/config_editor.rb](codegen/config_editor.rb)

3. Run `codgen/main.go` with `go run main.go`


## Credits

This project builds on the work of others. See [CREDITS.md](CREDITS.md)


## License

Dual licensed under the terms of [Apache 2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
