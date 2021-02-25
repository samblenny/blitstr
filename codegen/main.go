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

// Generate rust source code files for fonts
func codegen() {
	conf := NewConfig("config.json")
	fsList := conf.Fonts()
	for _, f := range fsList {
		// Find all the glyphs and pack them into a list of blit pattern objects
		pl := patternListFromSpriteSheet(f)
		// Make rust code for the blit pattern DATA array, plus an index list
		gs := NewGlyphSetFrom(pl, f.M3Seed)
		gs.AddAliasesToIndex(f.AliasList, f.M3Seed)
		// Generate rust source code and write it to a file
		code := RenderFontFileTemplate(f, gs, f.M3Seed)
		fmt.Println("Writing to", f.RustOut)
		ioutil.WriteFile(f.RustOut, []byte(code), 0644)
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
	conf := NewConfig("config.json")
	fsList := conf.Fonts()
	u := RenderUsageTemplate(confirm, fsList)
	fmt.Println(u)
}
