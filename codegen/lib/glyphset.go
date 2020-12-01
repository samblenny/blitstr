// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"fmt"
	"sort"
)

// Holds an index list and rust source code for a font's worth of blit patterns
type GlyphSet struct {
	Code    string
	DataLen int
	Index   map[UBlock]BlockIndex
}

// Make rust source code and an index list from a list of glyph blit patterns.
// The point of this is to prepare data in a way that's convenient for including
// in the context data used to render a .rs source code file template.
func NewGlyphSetFrom(pl []BlitPattern, m3Seed uint32) GlyphSet {
	g := GlyphSet{"", 0, map[UBlock]BlockIndex{}}
	for _, p := range pl {
		label := LabelForCluster(p.CS.GraphemeCluster())
		comment := fmt.Sprintf("[%d]: %s %s", g.DataLen, p.CS.HexCluster, label)
		g.Code += ConvertPatternToRust(p, comment)
		// Update the block index with the correct offset (DATA[n]) for pattern header
		g.Insert(p.CS.GraphemeCluster(), m3Seed, g.DataLen)
		g.DataLen += len(p.Words)
	}
	return g
}

// Add a list of grapheme cluster aliases to a GlyphSet.Index font index
func (g GlyphSet) AddAliasesToIndex(aliasList []GCAlias, m3Seed uint32) {
	for _, gcAlias := range aliasList {
		// Find the glyph pattern data offset for the cannonical grapheme cluster
		canonUtf8Cluster := StringFromHexGC(gcAlias.CanonHex)
		firstCodepoint := uint32([]rune(canonUtf8Cluster)[0])
		block := Block(firstCodepoint)
		glyphDataOffset := g.FindDataOffset(block, canonUtf8Cluster, m3Seed)
		// Add entry for alias grapheme cluster using same data offset.
		// Important note: the Unicode block for the first codepoint of
		// a Form C vs. Form D normalization may be *different*!
		aliasUtf8Cluster := StringFromHexGC(gcAlias.AliasHex)
		g.Insert(aliasUtf8Cluster, m3Seed, glyphDataOffset)
	}
}

// Insert entry into index of (grapheme cluster hash, glyph blit pattern data offset)
func (g GlyphSet) Insert(graphemeCluster string, m3Seed uint32, dataOffset int) {
	firstCodepoint := uint32([]rune(graphemeCluster)[0])
	block := Block(firstCodepoint)
	g.Index[block] = g.Index[block].Insert(graphemeCluster, m3Seed, dataOffset)
}

// Find data offset for the grapheme cluster in a GlyphSet.index, or panic
func (g GlyphSet) FindDataOffset(block UBlock, utf8Cluster string, m3Seed uint32) int {
	dex := g.Index[block]
	hash := Murmur3(utf8Cluster, m3Seed)
	n := sort.Search(len(dex), func(i int) bool { return dex[i].M3Hash >= hash })
	if n == len(dex) || dex[n].M3Hash != hash {
		fmt.Printf("\nblock: %X..%X %s\n", block.Low, block.High, block.Name)
		fmt.Printf("g.Index[block]:\n%+q\n", g.Index[block])
		fmt.Printf("n := Search(key):\n  index[n]: %v,  key: %q\n", dex[n].M3Hash, utf8Cluster)
		panic(fmt.Errorf("Grapheme cluster %q was not in g.Index[block]", utf8Cluster))
	}
	return dex[n].DataOffset
}

// Get sorted list of Unicode Blocks in the GlyphSet index (this is called from templates)
func (g GlyphSet) IndexKeys() []UBlock {
	blocks := []UBlock{}
	for k, _ := range g.Index {
		blocks = append(blocks, k)
	}
	sort.Slice(blocks, func(i, j int) bool { return blocks[i].Low < blocks[j].Low })
	return blocks
}
