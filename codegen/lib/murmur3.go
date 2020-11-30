// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"math/bits"
)

// Return Murmur3 hash function of a string using each character as a u32 block
func Murmur3(key string, seed uint32) uint32 {
	h := seed
	k := uint32(0)
	// Hash each codepoint in the string as its own uint32 block
	for _, c := range key {
		k = uint32(c)
		k *= 0xcc9e2d51
		k = bits.RotateLeft32(k, 15)
		k *= 0x1b873593
		h ^= k
		h = bits.RotateLeft32(h, 13)
		h *= 5
		h += 0xe6546b64
	}
	h ^= uint32(len(key))
	// Finalize with avalanche
	h ^= h >> 16
	h *= 0x85ebca6b
	h ^= h >> 13
	h *= 0xc2b2ae35
	return h ^ (h >> 16)
}
