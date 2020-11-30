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
	HexCluster string
	Row        int
	Col        int
}

// Holds an alias for a hex-codepoint grapheme cluster in the primary index
type GCAlias struct {
	CanonHex string // Cannonical form in the index (has a CharSpec)
	AliasHex string // This one should map to same glyph as CanonHex
}

// Parse and return the first codepoint of a hex grapheme cluster string.
// For example, "1f3c4-200d-2640-fe0f" -> 0x1F3C4
func (cs CharSpec) FirstCodepoint() uint32 {
	utf8 := StringFromHexGC(cs.HexCluster)
	codepoints := []rune(utf8)
	return uint32(codepoints[0])
}

// Convert a hex grapheme cluster string to a regular utf8 string.
// For example, "1f3c4-200d-2640-fe0f" -> "\U0001F3C4\u200d\u2640\ufe0f"
func (cs CharSpec) GraphemeCluster() string {
	return StringFromHexGC(cs.HexCluster)
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

// Return mapping of hex-codepoint format grapheme clusters to grid coordinates
// in a glyph sprite sheet for the system latin fonts (Bold & Regular)
func SysLatinMap() []CharSpec {
	return []CharSpec{
		// Unicode Basic Latin block
		CharSpec{"20", 0, 2},  // " "
		CharSpec{"21", 1, 2},  // "!"
		CharSpec{"22", 2, 2},  // "\""
		CharSpec{"23", 3, 2},  // "#"
		CharSpec{"24", 4, 2},  // "$"
		CharSpec{"25", 5, 2},  // "%"
		CharSpec{"26", 6, 2},  // "&"
		CharSpec{"27", 7, 2},  // "'"
		CharSpec{"28", 8, 2},  // "("
		CharSpec{"29", 9, 2},  // ")"
		CharSpec{"2A", 10, 2}, // "*"
		CharSpec{"2B", 11, 2}, // "+"
		CharSpec{"2C", 12, 2}, // ","
		CharSpec{"2D", 13, 2}, // "-"
		CharSpec{"2E", 14, 2}, // "."
		CharSpec{"2F", 15, 2}, // "/"
		CharSpec{"30", 0, 3},  // "0"
		CharSpec{"31", 1, 3},  // "1"
		CharSpec{"32", 2, 3},  // "2"
		CharSpec{"33", 3, 3},  // "3"
		CharSpec{"34", 4, 3},  // "4"
		CharSpec{"35", 5, 3},  // "5"
		CharSpec{"36", 6, 3},  // "6"
		CharSpec{"37", 7, 3},  // "7"
		CharSpec{"38", 8, 3},  // "8"
		CharSpec{"39", 9, 3},  // "9"
		CharSpec{"3A", 10, 3}, // ":"
		CharSpec{"3B", 11, 3}, // ";"
		CharSpec{"3C", 12, 3}, // "<"
		CharSpec{"3D", 13, 3}, // "="
		CharSpec{"3E", 14, 3}, // ">"
		CharSpec{"3F", 15, 3}, // "?"
		CharSpec{"40", 0, 4},  // "@"
		CharSpec{"41", 1, 4},  // "A"
		CharSpec{"42", 2, 4},  // "B"
		CharSpec{"43", 3, 4},  // "C"
		CharSpec{"44", 4, 4},  // "D"
		CharSpec{"45", 5, 4},  // "E"
		CharSpec{"46", 6, 4},  // "F"
		CharSpec{"47", 7, 4},  // "G"
		CharSpec{"48", 8, 4},  // "H"
		CharSpec{"49", 9, 4},  // "I"
		CharSpec{"4A", 10, 4}, // "J"
		CharSpec{"4B", 11, 4}, // "K"
		CharSpec{"4C", 12, 4}, // "L"
		CharSpec{"4D", 13, 4}, // "M"
		CharSpec{"4E", 14, 4}, // "N"
		CharSpec{"4F", 15, 4}, // "O"
		CharSpec{"50", 0, 5},  // "P"
		CharSpec{"51", 1, 5},  // "Q"
		CharSpec{"52", 2, 5},  // "R"
		CharSpec{"53", 3, 5},  // "S"
		CharSpec{"54", 4, 5},  // "T"
		CharSpec{"55", 5, 5},  // "U"
		CharSpec{"56", 6, 5},  // "V"
		CharSpec{"57", 7, 5},  // "W"
		CharSpec{"58", 8, 5},  // "X"
		CharSpec{"59", 9, 5},  // "Y"
		CharSpec{"5A", 10, 5}, // "Z"
		CharSpec{"5B", 11, 5}, // "["
		CharSpec{"5C", 12, 5}, // "\\"
		CharSpec{"5D", 13, 5}, // "]"
		CharSpec{"5E", 14, 5}, // "^"
		CharSpec{"5F", 15, 5}, // "_"
		CharSpec{"60", 0, 6},  // "`"
		CharSpec{"61", 1, 6},  // "a"
		CharSpec{"62", 2, 6},  // "b"
		CharSpec{"63", 3, 6},  // "c"
		CharSpec{"64", 4, 6},  // "d"
		CharSpec{"65", 5, 6},  // "e"
		CharSpec{"66", 6, 6},  // "f"
		CharSpec{"67", 7, 6},  // "g"
		CharSpec{"68", 8, 6},  // "h"
		CharSpec{"69", 9, 6},  // "i"
		CharSpec{"6A", 10, 6}, // "j"
		CharSpec{"6B", 11, 6}, // "k"
		CharSpec{"6C", 12, 6}, // "l"
		CharSpec{"6D", 13, 6}, // "m"
		CharSpec{"6E", 14, 6}, // "n"
		CharSpec{"6F", 15, 6}, // "o"
		CharSpec{"70", 0, 7},  // "p"
		CharSpec{"71", 1, 7},  // "q"
		CharSpec{"72", 2, 7},  // "r"
		CharSpec{"73", 3, 7},  // "s"
		CharSpec{"74", 4, 7},  // "t"
		CharSpec{"75", 5, 7},  // "u"
		CharSpec{"76", 6, 7},  // "v"
		CharSpec{"77", 7, 7},  // "w"
		CharSpec{"78", 8, 7},  // "x"
		CharSpec{"79", 9, 7},  // "y"
		CharSpec{"7A", 10, 7}, // "z"
		CharSpec{"7B", 11, 7}, // "{"
		CharSpec{"7C", 12, 7}, // "|"
		CharSpec{"7D", 13, 7}, // "}"
		CharSpec{"7E", 14, 7}, // "~"

		// Unicode Latin 1 block
		CharSpec{"A0", 0, 2},   // No-Break Space
		CharSpec{"A1", 1, 12},  // "¡"
		CharSpec{"A2", 2, 10},  // "¢"
		CharSpec{"A3", 3, 10},  // "£"
		CharSpec{"A4", 15, 1},  // "¤"
		CharSpec{"A5", 4, 11},  // "¥"
		CharSpec{"A6", 15, 7},  // "¦"
		CharSpec{"A7", 4, 10},  // "§"
		CharSpec{"A8", 12, 10}, // "¨"
		CharSpec{"A9", 9, 10},  // "©"
		CharSpec{"AA", 11, 11}, // "ª"
		CharSpec{"AB", 7, 12},  // "«"
		CharSpec{"AC", 2, 12},  // "¬"
		CharSpec{"AD", 13, 2},  // Soft Hyphen
		CharSpec{"AE", 8, 10},  // "®"
		CharSpec{"AF", 8, 15},  // "¯" Macron
		CharSpec{"B0", 1, 10},  // "°" Degree Sign
		CharSpec{"B1", 1, 11},  // "±"
		CharSpec{"B2", 3, 1},   // "²"
		CharSpec{"B3", 4, 1},   // "³"
		CharSpec{"B4", 11, 10}, // "´"
		CharSpec{"B5", 5, 11},  // "µ"
		CharSpec{"B6", 6, 10},  // "¶"
		CharSpec{"B7", 1, 14},  // "·"
		CharSpec{"B8", 12, 15}, // "¸" Cedillia
		CharSpec{"B9", 2, 1},   // "¹"
		CharSpec{"BA", 12, 11}, // "º"
		CharSpec{"BB", 8, 12},  // "»"
		CharSpec{"BC", 5, 1},   // "¼"
		CharSpec{"BD", 6, 1},   // "½"
		CharSpec{"BE", 7, 1},   // "¾"
		CharSpec{"BF", 0, 12},  // "¿"
		CharSpec{"C0", 11, 12}, // "À"
		CharSpec{"C1", 7, 14},  // "Á"
		CharSpec{"C2", 5, 14},  // "Â"
		CharSpec{"C3", 12, 12}, // "Ã"
		CharSpec{"C4", 0, 8},   // "Ä"
		CharSpec{"C5", 1, 8},   // "Å"
		CharSpec{"C6", 14, 10}, // "Æ"
		CharSpec{"C7", 2, 8},   // "Ç"
		CharSpec{"C8", 9, 14},  // "È"
		CharSpec{"C9", 3, 8},   // "É"
		CharSpec{"CA", 6, 14},  // "Ê"
		CharSpec{"CB", 8, 14},  // "Ë"
		CharSpec{"CC", 13, 14}, // "Ì"
		CharSpec{"CD", 10, 14}, // "Í"
		CharSpec{"CE", 11, 14}, // "Î"
		CharSpec{"CF", 12, 14}, // "Ï"
		CharSpec{"D0", 8, 1},   // "Ð"
		CharSpec{"D1", 4, 8},   // "Ñ"
		CharSpec{"D2", 1, 15},  // "Ò"
		CharSpec{"D3", 14, 14}, // "Ó"
		CharSpec{"D4", 15, 14}, // "Ô"
		CharSpec{"D5", 13, 12}, // "Õ"
		CharSpec{"D6", 5, 8},   // "Ö"
		CharSpec{"D7", 9, 1},   // "×" Multiplication Sign
		CharSpec{"D8", 15, 10}, // "Ø"
		CharSpec{"D9", 4, 15},  // "Ù"
		CharSpec{"DA", 2, 15},  // "Ú"
		CharSpec{"DB", 3, 15},  // "Û"
		CharSpec{"DC", 6, 8},   // "Ü"
		CharSpec{"DD", 10, 1},  // "Ý"
		CharSpec{"DE", 11, 1},  // "Þ"
		CharSpec{"DF", 7, 10},  // "ß"
		CharSpec{"E0", 8, 8},   // "à"
		CharSpec{"E1", 7, 8},   // "á"
		CharSpec{"E2", 9, 8},   // "â"
		CharSpec{"E3", 11, 8},  // "ã"
		CharSpec{"E4", 10, 8},  // "ä"
		CharSpec{"E5", 12, 8},  // "å"
		CharSpec{"E6", 14, 11}, // "æ"
		CharSpec{"E7", 13, 8},  // "ç"
		CharSpec{"E8", 15, 8},  // "è"
		CharSpec{"E9", 14, 8},  // "é"
		CharSpec{"EA", 0, 9},   // "ê"
		CharSpec{"EB", 1, 9},   // "ë"
		CharSpec{"EC", 3, 9},   // "ì"
		CharSpec{"ED", 2, 9},   // "í"
		CharSpec{"EE", 4, 9},   // "î"
		CharSpec{"EF", 5, 9},   // "ï"
		CharSpec{"F0", 12, 1},  // "ð"
		CharSpec{"F1", 6, 9},   // "ñ"
		CharSpec{"F2", 8, 9},   // "ò"
		CharSpec{"F3", 7, 9},   // "ó"
		CharSpec{"F4", 9, 9},   // "ô"
		CharSpec{"F5", 11, 9},  // "õ"
		CharSpec{"F6", 10, 9},  // "ö"
		CharSpec{"F7", 6, 13},  // "÷"
		CharSpec{"F8", 15, 11}, // "ø"
		CharSpec{"F9", 13, 9},  // "ù"
		CharSpec{"FA", 12, 9},  // "ú"
		CharSpec{"FB", 14, 9},  // "û"
		CharSpec{"FC", 15, 9},  // "ü"
		CharSpec{"FD", 13, 1},  // "ý"
		CharSpec{"FE", 14, 1},  // "þ"
		CharSpec{"FF", 8, 13},  // "ÿ"

		// Unicode Latin Extended A block
		CharSpec{"152", 14, 12}, // "Œ"
		CharSpec{"153", 15, 12}, // "œ"

		// Unicode General Punctuation block
		CharSpec{"2018", 4, 13}, // "‘" Left Single Quotation Mark
		CharSpec{"2019", 5, 13}, // "’" Right Single Quotation Mark
		CharSpec{"201A", 2, 14}, // "‚" Single Low-9 Quotation Mark
		CharSpec{"201B", 7, 11}, // "‛" Single High-Reversed-9 Quotation Mark
		CharSpec{"201C", 2, 13}, // "“" Left Double Quotation Mark
		CharSpec{"201D", 3, 13}, // "”" Right Double Quotation Mark
		CharSpec{"201E", 3, 14}, // "„" Double Low-9 Quotation Mark
		CharSpec{"201F", 8, 11}, // "‟" Double High-Reversed-9 Quotation Mark
		CharSpec{"2020", 0, 10}, // "†"
		CharSpec{"2021", 0, 14}, // "‡"
		CharSpec{"2022", 5, 10}, // "•"

		// Unicode Currency Symbols block
		CharSpec{"20AC", 11, 13}, // "€"

		// Unicode Specials Block
		CharSpec{"FFFD", 0, 15}, // "�"
	}
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

// Return a list of grapheme cluster aliases for the system latin font so that
// Unicode normalization form D (decomposed) grapheme clusters can be associated
// with their corresponding normalization form C (composed) grapheme clusters in
// the primary index. This helps avoid the need to normalize UTF-8 strings.
func SysLatinAliases() []GCAlias {
	return []GCAlias{
		GCAlias{"C0", "41-300"}, // nfc: [C0, À],  nfd: [41-300, À]
		GCAlias{"C1", "41-301"}, // nfc: [C1, Á],  nfd: [41-301, Á]
		GCAlias{"C2", "41-302"}, // nfc: [C2, Â],  nfd: [41-302, Â]
		GCAlias{"C3", "41-303"}, // nfc: [C3, Ã],  nfd: [41-303, Ã]
		GCAlias{"C4", "41-308"}, // nfc: [C4, Ä],  nfd: [41-308, Ä]
		GCAlias{"C5", "41-30A"}, // nfc: [C5, Å],  nfd: [41-30A, Å]
		GCAlias{"C7", "43-327"}, // nfc: [C7, Ç],  nfd: [43-327, Ç]
		GCAlias{"C8", "45-300"}, // nfc: [C8, È],  nfd: [45-300, È]
		GCAlias{"C9", "45-301"}, // nfc: [C9, É],  nfd: [45-301, É]
		GCAlias{"CA", "45-302"}, // nfc: [CA, Ê],  nfd: [45-302, Ê]
		GCAlias{"CB", "45-308"}, // nfc: [CB, Ë],  nfd: [45-308, Ë]
		GCAlias{"CC", "49-300"}, // nfc: [CC, Ì],  nfd: [49-300, Ì]
		GCAlias{"CD", "49-301"}, // nfc: [CD, Í],  nfd: [49-301, Í]
		GCAlias{"CE", "49-302"}, // nfc: [CE, Î],  nfd: [49-302, Î]
		GCAlias{"CF", "49-308"}, // nfc: [CF, Ï],  nfd: [49-308, Ï]
		GCAlias{"D1", "4E-303"}, // nfc: [D1, Ñ],  nfd: [4E-303, Ñ]
		GCAlias{"D2", "4F-300"}, // nfc: [D2, Ò],  nfd: [4F-300, Ò]
		GCAlias{"D3", "4F-301"}, // nfc: [D3, Ó],  nfd: [4F-301, Ó]
		GCAlias{"D4", "4F-302"}, // nfc: [D4, Ô],  nfd: [4F-302, Ô]
		GCAlias{"D5", "4F-303"}, // nfc: [D5, Õ],  nfd: [4F-303, Õ]
		GCAlias{"D6", "4F-308"}, // nfc: [D6, Ö],  nfd: [4F-308, Ö]
		GCAlias{"D9", "55-300"}, // nfc: [D9, Ù],  nfd: [55-300, Ù]
		GCAlias{"DA", "55-301"}, // nfc: [DA, Ú],  nfd: [55-301, Ú]
		GCAlias{"DB", "55-302"}, // nfc: [DB, Û],  nfd: [55-302, Û]
		GCAlias{"DC", "55-308"}, // nfc: [DC, Ü],  nfd: [55-308, Ü]
		GCAlias{"DD", "59-301"}, // nfc: [DD, Ý],  nfd: [59-301, Ý]
		GCAlias{"E0", "61-300"}, // nfc: [E0, à],  nfd: [61-300, à]
		GCAlias{"E1", "61-301"}, // nfc: [E1, á],  nfd: [61-301, á]
		GCAlias{"E2", "61-302"}, // nfc: [E2, â],  nfd: [61-302, â]
		GCAlias{"E3", "61-303"}, // nfc: [E3, ã],  nfd: [61-303, ã]
		GCAlias{"E4", "61-308"}, // nfc: [E4, ä],  nfd: [61-308, ä]
		GCAlias{"E5", "61-30A"}, // nfc: [E5, å],  nfd: [61-30A, å]
		GCAlias{"E7", "63-327"}, // nfc: [E7, ç],  nfd: [63-327, ç]
		GCAlias{"E8", "65-300"}, // nfc: [E8, è],  nfd: [65-300, è]
		GCAlias{"E9", "65-301"}, // nfc: [E9, é],  nfd: [65-301, é]
		GCAlias{"EA", "65-302"}, // nfc: [EA, ê],  nfd: [65-302, ê]
		GCAlias{"EB", "65-308"}, // nfc: [EB, ë],  nfd: [65-308, ë]
		GCAlias{"EC", "69-300"}, // nfc: [EC, ì],  nfd: [69-300, ì]
		GCAlias{"ED", "69-301"}, // nfc: [ED, í],  nfd: [69-301, í]
		GCAlias{"EE", "69-302"}, // nfc: [EE, î],  nfd: [69-302, î]
		GCAlias{"EF", "69-308"}, // nfc: [EF, ï],  nfd: [69-308, ï]
		GCAlias{"F1", "6E-303"}, // nfc: [F1, ñ],  nfd: [6E-303, ñ]
		GCAlias{"F2", "6F-300"}, // nfc: [F2, ò],  nfd: [6F-300, ò]
		GCAlias{"F3", "6F-301"}, // nfc: [F3, ó],  nfd: [6F-301, ó]
		GCAlias{"F4", "6F-302"}, // nfc: [F4, ô],  nfd: [6F-302, ô]
		GCAlias{"F5", "6F-303"}, // nfc: [F5, õ],  nfd: [6F-303, õ]
		GCAlias{"F6", "6F-308"}, // nfc: [F6, ö],  nfd: [6F-308, ö]
		GCAlias{"F9", "75-300"}, // nfc: [F9, ù],  nfd: [75-300, ù]
		GCAlias{"FA", "75-301"}, // nfc: [FA, ú],  nfd: [75-301, ú]
		GCAlias{"FB", "75-302"}, // nfc: [FB, û],  nfd: [75-302, û]
		GCAlias{"FC", "75-308"}, // nfc: [FC, ü],  nfd: [75-308, ü]
		GCAlias{"FD", "79-301"}, // nfc: [FD, ý],  nfd: [79-301, ý]
		GCAlias{"FF", "79-308"}, // nfc: [FF, ÿ],  nfd: [79-308, ÿ]
	}
}
