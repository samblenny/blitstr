// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"fmt"
	"sort"
	"strings"
)

// Index for grapheme clusters in the same Unicode block
type BlockIndex []clusterOffsetEntry

// An index entry for translating from grapheme cluster to blit pattern
type clusterOffsetEntry struct {
	M3Hash     uint32
	Cluster    string // Parsed UTF-8 form (not hex codepoints)
	DataOffset int
}

// Insert an index entry for (grapheme cluster hash, glyph blit pattern data offset).
// Maintain sort order according to grapheme cluster hashes.
func (b BlockIndex) Insert(graphemeCluster string, m3Seed uint32, dataOffset int) BlockIndex {
	indexEntry := clusterOffsetEntry{
		Murmur3(graphemeCluster, m3Seed),
		graphemeCluster,
		dataOffset,
	}
	b = append(b, indexEntry)
	// Sort by m3 hash
	sort.Slice(b, func(i, j int) bool { return b[i].M3Hash < b[j].M3Hash })
	return b
}

// Format the inner elements of a [u32; n] cluster hash index table for one block
func (b BlockIndex) RustCodeForClusterHashes() string {
	var rustCode []string
	for _, entry := range b {
		hash := fmt.Sprintf("0x%08X", entry.M3Hash)
		label := LabelForCluster(entry.Cluster)
		rustCode = append(rustCode, fmt.Sprintf("%s,  // %s", hash, label))
	}
	return strings.Join(rustCode, "\n    ")
}

// Format the inner elements of a [u32; n] blit pattern offset table for one block
func (b BlockIndex) RustCodeForOffsets() string {
	var rustCode []string
	for _, entry := range b {
		offset := fmt.Sprintf("%d,", entry.DataOffset)
		label := LabelForCluster(entry.Cluster)
		rustCode = append(rustCode, fmt.Sprintf("%-5s // %s", offset, label))
	}
	return strings.Join(rustCode, "\n    ")
}

// Make a grapheme cluster length list for a BlockIndex. The point of this is to
// facilitate efficient greedy matching. For example, when the index for a block
// has grapheme clusters of length 1 or 5 codepoints long, the grapheme cluster
// matching code for that block need not look ahead beyond 5 codepoints.
func (b BlockIndex) ClusterLengthList() []int {
	// Make a histogram
	blockHisto := map[int]int{}
	for _, entry := range b {
		codepoints := []rune(entry.Cluster)
		blockHisto[len(codepoints)] += 1
	}
	// Reduce histogram to a descending sorted list of cluster lengths
	gcLenList := []int{}
	for gcLen, _ := range blockHisto {
		gcLenList = append(gcLenList, gcLen)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(gcLenList)))
	return gcLenList
}

// Make label for grapheme cluster with special handling for UI sprites in PUA block
func LabelForCluster(c string) string {
	switch c {
	case "\u00AD":
		return "\"\\u00AD\" Soft Hyphen"
	case "\u00a0":
		return "\"\\u00A0\" No-Break Space"
	default:
		// For single codepoint grapheme clusters, such as Normalization
		// Form C, just print the character. But, for multi-codepoint
		// grapheme clusters, also print the hex cluster string
		s := fmt.Sprintf("%q", c)
		if len([]rune(c)) > 1 {
			hexCodepoints := []string{}
			for _, r := range []rune(c) {
				hcp := fmt.Sprintf("%X", uint32(r))
				hexCodepoints = append(hexCodepoints, hcp)
			}
			s += " " + strings.Join(hexCodepoints, "-")
		}
		return s
	}
}
