// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package main

import (
	"blitstr/codegen/lib"
	"bytes"
	"fmt"
	"image"
	"image/png"
	"io/ioutil"
	"os"
	"path"
	"strings"
	"text/template"
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
func fonts() []lib.FontSpec {
	return []lib.FontSpec{
		lib.FontSpec{"Emoji", "img/emoji_13_0.png", 32, 16, 0, 0, twemoji,
			lib.EmojiMap(16, emojiIndex), lib.EmojiAliases(emojiAliases),
			"emoji.rs"},
		lib.FontSpec{"Bold", "img/bold.png", 30, 16, 2, 2, chicago,
			lib.SysLatinMap(), lib.SysLatinAliases(),
			"bold.rs"},
		lib.FontSpec{"Regular", "img/regular.png", 30, 16, 2, 2, geneva,
			lib.SysLatinMap(), lib.SysLatinAliases(),
			"regular.rs"},
		lib.FontSpec{"Small", "img/small.png", 24, 16, 2, 2, geneva,
			lib.SysLatinMap(), lib.SysLatinAliases(),
			"small.rs"},
	}
}

// Generate rust source code files for fonts
func codegen() {
	for _, f := range fonts() {
		// Find all the glyphs and pack them into a list of blit pattern objects
		pl := patternListFromSpriteSheet(f)
		// Make rust code for the blit pattern DATA array, plus an index list
		gs := lib.NewGlyphSetFrom(pl, Murmur3Seed)
		gs.AddAliasesToIndex(f.AliasList, Murmur3Seed)
		data := renderTemplate(lib.DataTemplate, "data", struct {
			GS     lib.GlyphSet
			M3Seed uint32
		}{gs, Murmur3Seed})
		context := struct {
			Font    lib.FontSpec
			OutPath string
			Data    string
		}{f, outPath, data}
		// Generate rust source code and write it to a file
		code := renderTemplate(lib.FontFileTemplate, "font", context)
		op := path.Join(outPath, f.RustOut)
		fmt.Println("Writing to", op)
		ioutil.WriteFile(op, []byte(code), 0644)
	}
}

// Extract glyph sprites from a PNG grid and pack them into a list of blit pattern objects
func patternListFromSpriteSheet(fs lib.FontSpec) []lib.BlitPattern {
	// Read glyphs from png file
	img := readPNGFile(fs.Sprites)
	var patternList []lib.BlitPattern
	for _, cs := range fs.CSList {
		blitPattern := lib.NewBlitPattern(img, fs, cs, enableDebug)
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
	context := struct {
		Confirm string
		OutPath string
		Fonts   []lib.FontSpec
	}{confirm, outPath, fonts()}
	s := renderTemplate(lib.UsageTemplate, "usage", context)
	fmt.Println(s)
}

// Return a string from rendering the given template and context data
func renderTemplate(templateString string, name string, context interface{}) string {
	fmap := template.FuncMap{"ToLower": strings.ToLower}
	t := template.Must(template.New(name).Funcs(fmap).Parse(templateString))
	var buf bytes.Buffer
	err := t.Execute(&buf, context)
	if err != nil {
		panic(err)
	}
	return buf.String()
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
