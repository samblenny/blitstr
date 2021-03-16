// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

use crate::framebuffer::{LINES, WIDTH};
use crate::pt::Pt;

/// ClipRect specifies a region of pixels. X and y pixel ranges are inclusive of
/// min and exclusive of max (i.e. it's min.x..max.x rather than min.x..=max.x)
/// Coordinate System Notes:
/// - (0,0) is top left
/// - Increasing Y moves downward on the screen, increasing X moves right
#[derive(Copy, Clone, Debug, PartialEq, rkyv::Archive, rkyv::Serialize)]
pub struct ClipRect {
    pub min: Pt,
    pub max: Pt,
}

impl ClipRect {
    /// Initialize a rectangle using automatic min/max fixup for corner points
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> ClipRect {
        // Make sure min_x <= max_x && min_y <= max_y
        let mut min = Pt { x: min_x, y: min_y };
        let mut max = Pt { x: max_x, y: max_y };
        if min_x > max_x {
            min.x = max_x;
            max.x = min_x;
        }
        if min_y > max_y {
            min.y = max_y;
            max.y = min_y;
        }
        ClipRect { min, max }
    }

    /// Make a rectangle of the full screen size (0,0)..(WIDTH,LINES)
    pub fn full_screen() -> ClipRect {
        ClipRect::new(0, 0, WIDTH as i32, LINES as i32)
    }

    /// Make a rectangle of the screen size minus padding (6,6)..(WIDTH-6,LINES-6)
    pub fn padded_screen() -> ClipRect {
        let pad = 6;
        ClipRect::new(pad, pad, WIDTH as i32 - pad, LINES as i32 - pad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cliprect_equivalence() {
        let cr1 = ClipRect {
            min: Pt { x: 1, y: 2 },
            max: Pt { x: 8, y: 9 },
        };
        // Called properly:
        let cr2 = ClipRect::new(1, 2, 8, 9);
        // Called with mixed up corners that should get auto-corrected
        let cr3 = ClipRect::new(8, 2, 1, 9);
        let cr4 = ClipRect::new(1, 9, 8, 2);
        assert_eq!(cr1, cr2);
        assert_eq!(cr2, cr3);
        assert_eq!(cr3, cr4);
    }

    #[test]
    fn test_cliprect_full_screen() {
        let clip = ClipRect::full_screen();
        assert_eq!(clip.min, Pt::new(0, 0));
        assert_eq!(clip.max, Pt::new(WIDTH, LINES));
    }

    #[test]
    fn test_cliprect_padded_screen() {
        let c1 = ClipRect::full_screen();
        let c2 = ClipRect::padded_screen();
        assert!(c2.min > c1.min);
        assert!(c2.max < c1.max);
    }
}
