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

// Make rust source code and an index list from a list of glyph blit patterns.
// The point of this is to prepare data in a way that's convenient for including
// in the context data used to render a .rs source code file template.
func NewRustyBlitsFrom(pl []BlitPattern, m3Seed uint32) RustyBlits {
	rb := RustyBlits{"", 0, map[UBlock]BlockIndex{}}
	for _, p := range pl {
		label := LabelForCluster(p.CS.GraphemeCluster())
		comment := fmt.Sprintf("[%d]: %s %s", rb.DataLen, p.CS.HexCluster, label)
		rb.Code += ConvertPatternToRust(p, comment)
		// Update the block index with the correct offset (DATA[n]) for pattern header
		block := Block(p.CS.FirstCodepoint())
		rb.Index[block] = rb.Index[block].Insert(p.CS.GraphemeCluster(), m3Seed, rb.DataLen)
		rb.DataLen += len(p.Words)
	}
	rb.SortIndex()
	return rb
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
		rb.Index[block] = rb.Index[block].Insert(aliasUtf8Cluster, m3Seed, glyphDataOffset)
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
