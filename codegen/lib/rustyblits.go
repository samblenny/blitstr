// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"fmt"
	"sort"
)

// Holds an index list and rust source code for a font's worth of blit patterns
type RustyBlits struct {
	Code    string
	DataLen int
	Index   map[UBlock]BlockIndex
}

// Make a new RustyBlits
func NewRustyBlits() RustyBlits {
	return RustyBlits{"", 0, map[UBlock]BlockIndex{}}
}

// Add a list of grapheme cluster aliases to a RustyBlits.Index font index
func (rb RustyBlits) AddAliasesToIndex(aliasList []GCAlias, m3Seed uint32) {
	for _, gcAlias := range aliasList {
		// Find the glyph pattern data offset for the cannonical grapheme cluster
		canonUtf8Cluster := StringFromHexGC(gcAlias.CanonHex)
		firstCodepoint := uint32([]rune(canonUtf8Cluster)[0])
		block := Block(firstCodepoint)
		glyphDataOffset := rb.FindDataOffset(block, canonUtf8Cluster, m3Seed)
		// Add entry for alias grapheme cluster using same data offset.
		// Important note: the Unicode block for the first codepoint of
		// a Form C vs. Form D normalization may be *different*!
		aliasUtf8Cluster := StringFromHexGC(gcAlias.AliasHex)
		firstCodepoint = uint32([]rune(aliasUtf8Cluster)[0])
		block = Block(firstCodepoint)
		aliasEntry := ClusterOffsetEntry{
			Murmur3(aliasUtf8Cluster, m3Seed),
			aliasUtf8Cluster,
			glyphDataOffset,
		}
		// Insert Alias entry. Inserting each entry individually and
		// sorting after each one is an inefficient algorithm, but I'm
		// guessing the lists will be short enough that it won't matter.
		rb.Index[block] = append(rb.Index[block], aliasEntry)
		rb.SortIndex()
	}
}

// Find data offset for the grapheme cluster in a RustyBlits.index, or panic
func (rb RustyBlits) FindDataOffset(block UBlock, utf8Cluster string, m3Seed uint32) int {
	dex := rb.Index[block]
	hash := Murmur3(utf8Cluster, m3Seed)
	n := sort.Search(len(dex), func(i int) bool { return dex[i].M3Hash >= hash })
	if n == len(dex) || dex[n].M3Hash != hash {
		fmt.Printf("\nblock: %X..%X %s\n", block.Low, block.High, block.Name)
		fmt.Printf("rb.Index[block]:\n%+q\n", rb.Index[block])
		fmt.Printf("n := Search(key):\n  index[n]: %v,  key: %q\n", dex[n].M3Hash, utf8Cluster)
		panic(fmt.Errorf("Grapheme cluster %q was not in rb.Index[block]", utf8Cluster))
	}
	return dex[n].DataOffset
}

// Sort the index for each Unicode block of a RustyBlits.Index font index
func (rb RustyBlits) SortIndex() {
	for _, v := range rb.Index {
		sort.Slice(v, func(i, j int) bool { return v[i].M3Hash < v[j].M3Hash })
	}
}

// Get sorted list of Unicode Blocks in the RustyBlits index (this is called from templates)
func (rb RustyBlits) IndexKeys() []UBlock {
	blocks := []UBlock{}
	for k, _ := range rb.Index {
		blocks = append(blocks, k)
	}
	sort.Slice(blocks, func(i, j int) bool { return blocks[i].Low < blocks[j].Low })
	return blocks
}
