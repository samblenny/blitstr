// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
use blitstr::demo;

/// This runs the demo with no visible output, which is mostly only useful for
/// debugging changes to the blitting code. Since main() links with the standard
/// library, any panics will get printed (unlike with wasm).
fn main() {
    // Allocate frame buffer
    let fb = &mut blitstr::new_fr_buf();

    // Call painting code
    demo::sample_text(fb);
    demo::short_greeting(fb);

    // Unimplemented: copy frame buffer to a display device
}
