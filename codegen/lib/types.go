// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

// An index entry for translating from grapheme cluster to blit pattern
type ClusterOffsetEntry struct {
	M3Hash     uint32
	Cluster    string // Parsed UTF-8 form (not hex codepoints)
	DataOffset int
}
