// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
#![forbid(unsafe_code)]

mod blit;
mod fonts;
pub mod state;
mod views;

/// Public API for keyboard and screen events
pub mod api {
    use super::{state, views};

    /// Repaint the active view
    pub fn repaint(fb: &mut state::FrameBuf) {
        views::home_screen(fb);
    }
}
