// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

// Holds mappings from extended grapheme clusters to sprite sheet glyph grid coordinates
type CharSpec struct {
	Hex string
	Row int
	Col int
}

// Holds an alias for a hex-codepoint grapheme cluster in the primary index
type GCAlias struct {
	CanonHex string // Cannonical form in the index (has a CharSpec)
	AliasHex string // This one should map to same glyph as CanonHex
}

// Parse and return the first codepoint of a hex grapheme cluster string.
// For example, "1f3c4-200d-2640-fe0f" -> 0x1F3C4
func (cs CharSpec) FirstCodepoint() uint32 {
	utf8 := StringFromHexGC(cs.Hex)
	codepoints := []rune(utf8)
	return uint32(codepoints[0])
}

// Convert a hex grapheme cluster string to a regular utf8 string.
// For example, "1f3c4-200d-2640-fe0f" -> "\U0001F3C4\u200d\u2640\ufe0f"
func (cs CharSpec) GraphemeCluster() string {
	return StringFromHexGC(cs.Hex)
}

// Parse a hex-codepoint format grapheme cluster into a utf-8 string
// For example, "1f3c4-200d-2640-fe0f" -> "\U0001F3C4\u200d\u2640\ufe0f"
func StringFromHexGC(hexGC string) string {
	base := 16
	bits := 32
	cluster := ""
	hexCodepoints := strings.Split(hexGC, "-")
	if len(hexCodepoints) < 1 {
		panic(fmt.Errorf("unexpected value for hexGC: %q", hexGC))
	}
	for _, hc := range hexCodepoints {
		n, err := strconv.ParseUint(hc, base, bits)
		if err != nil {
			panic(fmt.Errorf("unexpected value for hexGC: %q", hexGC))
		}
		cluster += string(rune(n))
	}
	return cluster
}

// Return mapping of hex-codepoint format grapheme clusters to grid coordinates
// in a glyph sprite sheet for the emoji font
func EmojiMap(columns int, inputFile string) []CharSpec {
	text, err := ioutil.ReadFile(inputFile)
	if err != nil {
		panic(err)
	}
	// Start at top left corner of the sprite sheet glyph grid
	row := 0
	col := 0
	// Parse hex format grapheme cluster lines that should look like
	// "1f4aa-1f3fc\n" "1f4e1\n", etc. Comments starting with "#" are
	// possible. Order of grapheme cluster lines in the file should match a
	// row-major order traversal of the glyph grid.
	csList := []CharSpec{}
	for _, line := range strings.Split(string(text), "\n") {
		// Trim comments and leading/trailing whitespace
		txt := strings.TrimSpace(strings.SplitN(line, "#", 2)[0])
		if len(txt) > 0 {
			// Add a CharSpec for this grapheme cluster
			csList = append(csList, CharSpec{txt, row, col})
			// Advance to next glyph position by row-major order
			col += 1
			if col == columns {
				row += 1
				col = 0
			}
		}
		// Skip blank lines and comments
	}
	return csList
}

// Return a list of grapheme cluster aliases for the emoji font
func EmojiAliases(inputFile string) []GCAlias {
	text, err := ioutil.ReadFile(inputFile)
	if err != nil {
		panic(err)
	}
	// Parse hex format grapheme cluster alias lines that should look like
	// "1f004 1f004-fe0f\n". First grapheme cluster is from the primary
	// index, second cluster is the alias which should get the same glyph.
	// Comments starting with "#" are possible.
	gcaList := []GCAlias{}
	for _, line := range strings.Split(string(text), "\n") {
		// Trim comments and leading/trailing whitespace
		txt := strings.TrimSpace(strings.SplitN(line, "#", 2)[0])
		clusters := strings.Split(txt, " ")
		if len(clusters) == 2 && len(clusters[0]) > 0 && len(clusters[1]) > 0 {
			primary := clusters[0]
			alias := clusters[1]
			gcaList = append(gcaList, GCAlias{primary, alias})
		}
		// Skip blank lines, comments, etc.
	}
	return gcaList
}
