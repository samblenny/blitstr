// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"fmt"
)

// Holds specification for a Unicode block (codepoint bounds and identifying string)
type UBlock struct {
	Low  uint32
	High uint32
	Name string
}

// Unicode blocks containing leading Unicode Scalars of grapheme clusters used
// in currently supported fonts. For full list of Unicode blocks, see
// https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt
func knownBlocks() []UBlock {
	return []UBlock{
		UBlock{0x0000, 0x007F, "BASIC_LATIN"},                             // Latin, Emoji
		UBlock{0x0080, 0x00FF, "LATIN_1_SUPPLEMENT"},                      // Latin, Emoji
		UBlock{0x0100, 0x017F, "LATIN_EXTENDED_A"},                        // Latin
		UBlock{0x2000, 0x206F, "GENERAL_PUNCTUATION"},                     // Latin, Emoji
		UBlock{0x20A0, 0x20CF, "CURRENCY_SYMBOLS"},                        // Latin
		UBlock{0x2100, 0x214F, "LETTERLIKE_SYMBOLS"},                      // Emoji
		UBlock{0x2190, 0x21FF, "ARROWS"},                                  // Emoji
		UBlock{0x2300, 0x23FF, "MISCELLANEOUS_TECHNICAL"},                 // Emoji
		UBlock{0x2460, 0x24FF, "ENCLOSED_ALPHANUMERICS"},                  // Emoji
		UBlock{0x25A0, 0x25FF, "GEOMETRIC_SHAPES"},                        // Emoji
		UBlock{0x2600, 0x26FF, "MISCELLANEOUS_SYMBOLS"},                   // Emoji
		UBlock{0x2700, 0x27BF, "DINGBATS"},                                // Emoji
		UBlock{0x2900, 0x297F, "SUPPLEMENTAL_ARROWS_B"},                   // Emoji
		UBlock{0x2B00, 0x2BFF, "MISCELLANEOUS_SYMBOLS_AND_ARROWS"},        // Emoji
		UBlock{0x3000, 0x303F, "CJK_SYMBOLS_AND_PUNCTUATION"},             // Emoji
		UBlock{0x3200, 0x32FF, "ENCLOSED_CJK_LETTERS_AND_MONTHS"},         // Emoji
		UBlock{0xE000, 0xF8FF, "PRIVATE_USE_AREA"},                        // Emoji (109)
		UBlock{0xFFF0, 0xFFFF, "SPECIALS"},                                // Latin (replacement char)
		UBlock{0x1F000, 0x1F02F, "MAHJONG_TILES"},                         // Emoji
		UBlock{0x1F0A0, 0x1F0FF, "PLAYING_CARDS"},                         // Emoji
		UBlock{0x1F100, 0x1F1FF, "ENCLOSED_ALPHANUMERIC_SUPPLEMENT"},      // Emoji
		UBlock{0x1F200, 0x1F2FF, "ENCLOSED_IDEOGRAPHIC_SUPPLEMENT"},       // Emoji
		UBlock{0x1F300, 0x1F5FF, "MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS"}, // Emoji
		UBlock{0x1F600, 0x1F64F, "EMOTICONS"},                             // Emoji
		UBlock{0x1F680, 0x1F6FF, "TRANSPORT_AND_MAP_SYMBOLS"},             // Emoji
		UBlock{0x1F780, 0x1F7FF, "GEOMETRIC_SHAPES_EXTENDED"},             // Emoji
		UBlock{0x1F900, 0x1F9FF, "SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS"},  // Emoji
		UBlock{0x1FA70, 0x1FAFF, "SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A"},    // Emoji
	}
}

// Given a Unicode codepoint, return the Unicode block it belongs to
func Block(c uint32) UBlock {
	for _, b := range knownBlocks() {
		if b.Low <= c && c <= b.High {
			return b
		}
	}
	panic(fmt.Errorf("Codepoint %X belongs to an unknown Unicode block", c))
}
