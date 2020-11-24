// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]
use super::blit;

/// LCD frame buffer
pub struct FrameBuf {
    pub buf: blit::FrBuf,
}
impl FrameBuf {
    pub const fn new() -> FrameBuf {
        FrameBuf {
            buf: [0; blit::FRAME_BUF_SIZE],
        }
    }
}
