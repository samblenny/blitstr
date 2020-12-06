// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// This code includes an adaptation of the the murmur3 hash algorithm.
// The murmur3 public domain notice, as retrieved on August 3, 2020 from
// https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp,
// states:
// > MurmurHash3 was written by Austin Appleby, and is placed in the public
// > domain. The author hereby disclaims copyright to this source code.
//
#![forbid(unsafe_code)]

/// Compute Murmur3 hash function of the first limit codepoints of a grapheme
/// cluster string, using each char as a u32 block.
/// Returns: (murmur3 hash, how many bytes of key were hashed (e.g. key[..n]))
pub fn grapheme_cluster(gc: &str, seed: u32, limit: u32) -> (u32, usize) {
    let mut h = seed;
    let mut k;
    // Hash each character as its own u32 block
    let mut n = 0;
    let mut bytes_hashed = gc.len();
    for (i, c) in gc.char_indices() {
        if n >= limit {
            bytes_hashed = i;
            break;
        }
        k = c as u32;
        k = k.wrapping_mul(0xcc9e2d51);
        k = k.rotate_left(15);
        k = k.wrapping_mul(0x1b873593);
        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5);
        h = h.wrapping_add(0xe6546b64);
        n += 1;
    }
    h ^= bytes_hashed as u32;
    // Finalize with avalanche
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    (h, bytes_hashed)
}

/// Compute Murmur3 hash function of a u32 frame buffer.
/// This is intended for testing that the contents of a frame buffer match a
/// previous frame buffer state that was visually checked for correctness.
#[allow(dead_code)]
pub fn frame_buffer(fb: &[u32], seed: u32) -> u32 {
    let mut h = seed;
    let mut k;
    for word in fb.iter() {
        k = *word;
        k = k.wrapping_mul(0xcc9e2d51);
        k = k.rotate_left(15);
        k = k.wrapping_mul(0x1b873593);
        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5);
        h = h.wrapping_add(0xe6546b64);
    }
    h ^= fb.len() as u32;
    // Finalize with avalanche
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_cluster_ascii_seed0_limit1() {
        let gc = &"test";
        let seed = 0;
        let limit = 1;
        // This is hashing just the 't' from "test"
        assert_eq!(grapheme_cluster(gc, seed, limit), (0x31099644, 1));
    }

    #[test]
    fn test_grapheme_cluster_ascii_seed1_limit1() {
        let gc = &"test";
        let seed = 1;
        let limit = 1;
        // This is hashing just the 't' from "test"
        assert_eq!(grapheme_cluster(gc, seed, limit), (0xD667FA27, 1));
    }

    #[test]
    fn test_grapheme_cluster_simple_emoji_limit1() {
        let gc = &"😸test";
        let seed = 0;
        let limit = 1;
        // This is hashing just the "😸" from "😸test"
        // Return of (_, 4) means 4 bytes were used for limit of 1 codepoint
        assert_eq!(grapheme_cluster(gc, seed, limit), (0x86E5DD9A, 4));
    }

    #[test]
    fn test_grapheme_cluster_combo_emoji_limit2() {
        let gc = &"📺️"; // 1F4FA-FE0F
        let seed = 0;
        let limit = 2;
        // This is hashing all of "📺️" (1 grapheme cluster of 2 codepoints)
        // Return of (_, 7) means 7 bytes were used for limit of 2 codepoints
        assert_eq!(grapheme_cluster(gc, seed, limit), (0x7C5E300, 7));
    }

    #[test]
    fn test_frame_buffer_seed0_0x00000000_len1() {
        let fb: &[u32] = &[0x0];
        let seed = 0;
        // Note how the hash is not the same as for seed=1 in the test below
        assert_eq!(frame_buffer(fb, seed), 0x9B9CB39A);
    }

    #[test]
    fn test_frame_buffer_seed1_0x00000000_len1() {
        let fb: &[u32] = &[0x0];
        let seed = 1;
        // Note how the hash is not the same as for seed=0 in the test above
        assert_eq!(frame_buffer(fb, seed), 0xC8C1D2C1);
    }

    #[test]
    fn test_frame_buffer_seed0_0x00000100_len1() {
        let fb: &[u32] = &[0x00000500];
        let seed = 0;
        assert_eq!(frame_buffer(fb, seed), 0x7DEFDA4F);
    }

    #[test]
    fn test_frame_buffer_seed0_0x00000100_len2000() {
        let fb: &[u32] = &[0x00000500; 2000];
        let seed = 0;
        assert_eq!(frame_buffer(fb, seed), 0x4E61577A);
    }

    #[test]
    fn test_frame_buffer_seed0_0xffffffff_len2000() {
        let fb: &[u32] = &[0xFFFFFFFF; 2000];
        let seed = 0;
        assert_eq!(frame_buffer(fb, seed), 0x59F987C6);
    }
}
