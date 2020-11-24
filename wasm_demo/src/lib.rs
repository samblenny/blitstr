// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
extern crate blitstr;

use blitstr::state::FrameBuf;

static mut FB: FrameBuf = FrameBuf::new();

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

/// Initialize state
#[no_mangle]
pub extern "C" fn init() {
    blitstr::api::repaint(unsafe { &mut FB });
}

/// Export pointer to frame buffer shared memory for javascript + wasm32
#[no_mangle]
pub extern "C" fn frame_buf_ptr() -> *const u32 {
    unsafe { FB.buf.as_ptr() }
}
