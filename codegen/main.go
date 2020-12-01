// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package main

import (
	. "blitstr/codegen/lib"
	"fmt"
	"image"
	"image/png"
	"io/ioutil"
	"os"
	"path"
)

// Command line switch to confirm intent of writing output files
const confirm = "--write"

// Main: check for confirmation switch before writing files
func main() {
	if len(os.Args) == 2 && os.Args[1] == confirm {
		codegen()
	} else {
		usage()
	}
}

// Change this to control the visibility of debug messages
const enableDebug = false

// Path for output files with generated font code
const outPath = "../src/fonts"

// Index and alias files for grapheme clusters that go with the emoji sprite sheet
const emojiIndex = "img/emoji_13_0_index.txt"
const emojiAliases = "img/emoji_13_0_aliases.txt"

// Seed for Murmur3 hashes; in the event of hash collisions, change this
const Murmur3Seed uint32 = 0

// Spec for how to generate font source code files from glyph grid sprite sheets
func fonts() []FontSpec {
	return []FontSpec{
		FontSpec{"Emoji", "img/emoji_13_0.png", 32, 16, 0, 0, twemoji,
			EmojiMap(16, emojiIndex), EmojiAliases(emojiAliases),
			"emoji.rs"},
		FontSpec{"Bold", "img/bold.png", 30, 16, 2, 2, chicago,
			SysLatinMap(), SysLatinAliases(),
			"bold.rs"},
		FontSpec{"Regular", "img/regular.png", 30, 16, 2, 2, geneva,
			SysLatinMap(), SysLatinAliases(),
			"regular.rs"},
		FontSpec{"Small", "img/small.png", 24, 16, 2, 2, geneva,
			SysLatinMap(), SysLatinAliases(),
			"small.rs"},
	}
}

// Generate rust source code files for fonts
func codegen() {
	for _, f := range fonts() {
		// Find all the glyphs and pack them into a list of blit pattern objects
		pl := patternListFromSpriteSheet(f)
		// Make rust code for the blit pattern DATA array, plus an index list
		gs := NewGlyphSetFrom(pl, Murmur3Seed)
		gs.AddAliasesToIndex(f.AliasList, Murmur3Seed)
		data := RenderDataTemplate(gs, Murmur3Seed)
		// Generate rust source code and write it to a file
		code := RenderFontFileTemplate(f, outPath, data)
		op := path.Join(outPath, f.RustOut)
		fmt.Println("Writing to", op)
		ioutil.WriteFile(op, []byte(code), 0644)
	}
}

// Extract glyph sprites from a PNG grid and pack them into a list of blit pattern objects
func patternListFromSpriteSheet(fs FontSpec) []BlitPattern {
	// Read glyphs from png file
	img := readPNGFile(fs.Sprites)
	var patternList []BlitPattern
	for _, cs := range fs.CSList {
		blitPattern := NewBlitPattern(img, fs, cs, enableDebug)
		patternList = append(patternList, blitPattern)
	}
	return patternList
}

// Read the specified PNG file and convert its data into an image object
func readPNGFile(name string) image.Image {
	pngFile, err := os.Open(name)
	if err != nil {
		panic("unable to open png file")
	}
	img, err := png.Decode(pngFile)
	if err != nil {
		panic("unable to decode png file")
	}
	pngFile.Close()
	return img
}

// Print usage message
func usage() {
	u := RenderUsageTemplate(confirm, outPath, fonts())
	fmt.Println(u)
}

// Emoji graphics legal notice
const twemoji = `// This code includes encoded bitmaps with modified versions of graphics from
// the twemoji project. The modified emoji PNG files were converted from color
// PNG format to monochrome PNG with dithered grayscale shading.
//
// - Twemoji License Notice
//   > Copyright 2019 Twitter, Inc and other contributors
//   >
//   > Code licensed under the MIT License: http://opensource.org/licenses/MIT
//   >
//   > Graphics licensed under CC-BY 4.0: https://creativecommons.org/licenses/by/4.0/
//
// - Twemoji Source Code Link:
//   https://github.com/twitter/twemoji
//`

// Bold font legal notice
const chicago = `// This code includes encoded bitmaps of glyphs from the Chicago typeface which
// was designed by Susan Kare and released by Apple in 1984. Chicago is a
// registered trademark of Apple Inc.
//`

// Regular font legal notice
const geneva = `// This code includes encoded bitmaps of glyphs from the Geneva typeface which
// was designed by Susan Kare and released by Apple in 1984. Geneva is a
// registered trademark of Apple Inc.
//`
