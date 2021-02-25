// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

/// Frame buffer bounds
pub const WORDS_PER_LINE: usize = 11;
pub const WIDTH: usize = 336;
pub const LINES: usize = 536;
pub const FRAME_BUF_SIZE: usize = WORDS_PER_LINE * LINES;

/// Frame buffer of 1-bit pixels
pub type FrBuf = [u32; FRAME_BUF_SIZE];

/// Initialize a frame buffer with stripes
pub const fn new_fr_buf() -> FrBuf {
    [0xffff0000; FRAME_BUF_SIZE]
}
