// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package main

import (
	"bytes"
	"fmt"
	"blitstr/codegen/font"
	"image"
	"image/png"
	"io/ioutil"
	"math/bits"
	"os"
	"path"
	"sort"
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

// Index and alias files for grapheme clusters that go with img/emoji48x48_o3x3.png
const emojiIndex = "img/emoji_13_0_index.txt"
const emojiAliases = "img/emoji_13_0_aliases.txt"

// Seed for Murmur3 hashes; in the event of hash collisions, change this
const Murmur3Seed uint32 = 0

// Spec for how to generate font source code files from glyph grid sprite sheets
func fonts() []font.FontSpec {
	return []font.FontSpec{
		font.FontSpec{"Emoji", "img/emoji_13_0_32x32_o3x3.png", 32, 16, 0, 0, twemoji, "emoji.rs"},
		font.FontSpec{"Bold", "img/bold.png", 30, 16, 2, 2, chicago, "bold.rs"},
		font.FontSpec{"Regular", "img/regular.png", 30, 16, 2, 2, geneva, "regular.rs"},
		font.FontSpec{"Small", "img/small.png", 24, 16, 2, 2, geneva, "small.rs"},
	}
}

// Generate rust source code files for fonts
func codegen() {
	for _, f := range fonts() {
		var data string
		switch f.Name {
		case "Emoji":
			data = genRustyFontData(f, font.EmojiMap(f, emojiIndex), font.EmojiAliases(emojiAliases))
		case "Bold":
			data = genRustyFontData(f, font.SysLatinMap(), font.SysLatinAliases())
		case "Regular":
			data = genRustyFontData(f, font.SysLatinMap(), font.SysLatinAliases())
		case "Small":
			data = genRustyFontData(f, font.SysLatinMap(), font.SysLatinAliases())
		default:
			panic("unexpected FontSpec.Name")
		}
		context := struct {
			Font    font.FontSpec
			OutPath string
			Data    string
		}{f, outPath, data}
		// Generate rust source code and write it to a file
		code := renderTemplate(fontFileTemplate, "font", context)
		op := path.Join(outPath, f.RustOut)
		fmt.Println("Writing to", op)
		ioutil.WriteFile(op, []byte(code), 0644)
	}
}

// Generate rust code for glyph blit pattern data and related grapheme cluster index
func genRustyFontData(fs font.FontSpec, csList []font.CharSpec, aliasList []font.GCAlias) string {
	if len(csList) == 0 {
		return fmt.Sprintf("/* TODO: %s data */", fs.Name)
	}
	// Find all the glyphs and pack them into a list of blit pattern objects
	pl := patternListFromSpriteSheet(fs, csList)
	// Make rust code for the blit pattern DATA array, plus an index list
	rb := rustyBlitsFromPatternList(pl)
	rb.AddAliasesToIndex(aliasList)
	return renderTemplate(dataTemplate, "data", struct {
		RB     RustyBlits
		M3Seed uint32
	}{rb, 0})
}

// Extract glyph sprites from a PNG grid and pack them into a list of blit pattern objects
func patternListFromSpriteSheet(fs font.FontSpec, csList []font.CharSpec) []font.BlitPattern {
	// Read glyphs from png file
	img := readPNGFile(fs.Sprites)
	var patternList []font.BlitPattern
	for _, cs := range csList {
		blitPattern := font.ConvertGlyphToBlitPattern(img, fs, cs, enableDebug)
		patternList = append(patternList, blitPattern)
	}
	return patternList
}

// Make rust source code and an index list from a list of glyph blit patterns.
// When this finishes, rust source code for the `DATA: [u32; n] = [...];` array
// of concatenated blit patterns is in the return values's .Code. The length (n)
// of the `DATA: [u32; n]...` blit pattern array is in .DataLen, and the
// ClusterOffsetEntry{...} index entries are in .Index.
func rustyBlitsFromPatternList(pl []font.BlitPattern) RustyBlits {
	rb := RustyBlits{"", 0, FontIndex{}}
	for _, p := range pl {
		label := labelForCluster(p.CS.GraphemeCluster())
		comment := fmt.Sprintf("[%d]: %s %s", rb.DataLen, p.CS.HexCluster, label)
		rb.Code += font.ConvertPatternToRust(p, comment)
		// Update the block index with the correct offset (DATA[n]) for pattern header
		indexEntry := ClusterOffsetEntry{
			murmur3(p.CS.GraphemeCluster(), Murmur3Seed),
			p.CS.GraphemeCluster(),
			rb.DataLen,
		}
		block := font.Block(p.CS.FirstCodepoint())
		rb.Index[block] = append(rb.Index[block], indexEntry)
		rb.DataLen += len(p.Words)
	}
	rb.SortIndex()
	return rb
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

// Make label for grapheme cluster with special handling for UI sprites in PUA block
func labelForCluster(c string) string {
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

// Return Murmur3 hash function of a string using each character as a u32 block
func murmur3(key string, seed uint32) uint32 {
	h := seed
	k := uint32(0)
	// Hash each codepoint in the string as its own uint32 block
	for _, c := range key {
		k = uint32(c)
		k *= 0xcc9e2d51
		k = bits.RotateLeft32(k, 15)
		k *= 0x1b873593
		h ^= k
		h = bits.RotateLeft32(h, 13)
		h *= 5
		h += 0xe6546b64
	}
	h ^= uint32(len(key))
	// Finalize with avalanche
	h ^= h >> 16
	h *= 0x85ebca6b
	h ^= h >> 13
	h *= 0xc2b2ae35
	return h ^ (h >> 16)
}

// Holds an index list and rust source code for a font's worth of blit patterns
type RustyBlits struct {
	Code    string
	DataLen int
	Index   FontIndex
}

// Index for all the Unicode blocks in a font
type FontIndex map[font.UBlock]BlockIndex

// Index for grapheme clusters in the same Unicode block
type BlockIndex []ClusterOffsetEntry

// An index entry for translating from grapheme cluster to blit pattern
type ClusterOffsetEntry struct {
	M3Hash     uint32
	Cluster    string // Parsed UTF-8 form (not hex codepoints)
	DataOffset int
}

// Add a list of grapheme cluster aliases to a RustyBlits.Index FontIndex
func (rb RustyBlits) AddAliasesToIndex(aliasList []font.GCAlias) {
	for _, gcAlias := range aliasList {
		// Find the glyph pattern data offset for the cannonical grapheme cluster
		canonUtf8Cluster := font.StringFromHexGC(gcAlias.CanonHex)
		firstCodepoint := uint32([]rune(canonUtf8Cluster)[0])
		block := font.Block(firstCodepoint)
		glyphDataOffset := rb.FindDataOffset(block, canonUtf8Cluster)
		// Add entry for alias grapheme cluster using same data offset.
		// Important note: the Unicode block for the first codepoint of
		// a Form C vs. Form D normalization may be *different*!
		aliasUtf8Cluster := font.StringFromHexGC(gcAlias.AliasHex)
		firstCodepoint = uint32([]rune(aliasUtf8Cluster)[0])
		block = font.Block(firstCodepoint)
		aliasEntry := ClusterOffsetEntry{
			murmur3(aliasUtf8Cluster, Murmur3Seed),
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
func (rb RustyBlits) FindDataOffset(block font.UBlock, utf8Cluster string) int {
	dex := rb.Index[block]
	hash := murmur3(utf8Cluster, Murmur3Seed)
	n := sort.Search(len(dex), func(i int) bool { return dex[i].M3Hash >= hash })
	if n == len(dex) || dex[n].M3Hash != hash {
		fmt.Printf("\nblock: %X..%X %s\n", block.Low, block.High, block.Name)
		fmt.Printf("rb.Index[block]:\n%+q\n", rb.Index[block])
		fmt.Printf("n := Search(key):\n  index[n]: %v,  key: %q\n", dex[n].M3Hash, utf8Cluster)
		panic(fmt.Errorf("Grapheme cluster %q was not in rb.Index[block]", utf8Cluster))
	}
	return dex[n].DataOffset
}

// Sort the index for each Unicode block of a RustyBlits.Index FontIndex
func (rb RustyBlits) SortIndex() {
	for _, v := range rb.Index {
		sort.Slice(v, func(i, j int) bool { return v[i].M3Hash < v[j].M3Hash })
	}
}

func (rb RustyBlits) IndexKeys() []font.UBlock {
	blocks := []font.UBlock{}
	for k, _ := range rb.Index {
		blocks = append(blocks, k)
	}
	sort.Slice(blocks, func(i, j int) bool { return blocks[i].Low < blocks[j].Low })
	return blocks
}

// Format the inner elements of a [u32; n] cluster hash index table for one block
func (coIndex BlockIndex) RustCodeForClusterHashes() string {
	var rustCode []string
	for _, entry := range coIndex {
		hash := fmt.Sprintf("0x%08X", entry.M3Hash)
		label := labelForCluster(entry.Cluster)
		rustCode = append(rustCode, fmt.Sprintf("%s,  // %s", hash, label))
	}
	return strings.Join(rustCode, "\n    ")
}

// Format the inner elements of a [u32; n] blit pattern offset table for one block
func (coIndex BlockIndex) RustCodeForOffsets() string {
	var rustCode []string
	for _, entry := range coIndex {
		offset := fmt.Sprintf("%d,", entry.DataOffset)
		label := labelForCluster(entry.Cluster)
		rustCode = append(rustCode, fmt.Sprintf("%-5s // %s", offset, label))
	}
	return strings.Join(rustCode, "\n    ")
}

// Make a grapheme cluster length list for a BlockIndex. The point of this is to
// facilitate efficient greedy matching. For example, when the index for a block
// has grapheme clusters of length 1 or 5 codepoints long, the grapheme cluster
// matching code for that block need not look ahead beyond 5 codepoints.
func (bDex BlockIndex) ClusterLengthList() []int {
	// Make a histogram
	blockHisto := map[int]int{}
	for _, entry := range bDex {
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

// Print usage message
func usage() {
	context := struct {
		Confirm string
		OutPath string
		Fonts   []font.FontSpec
	}{confirm, outPath, fonts()}
	s := renderTemplate(usageTemplate, "usage", context)
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

// Template with usage instructions for this command line tool
const usageTemplate = `
This tool generates fonts in the form of rust source code.
To confirm that you want to write the files, use the {{.Confirm}} switch.

Font files that will be generated:{{range $f := .Fonts}}
  {{$.OutPath}}/{{$f.RustOut}}{{end}}

Usage:
    go run main.go {{.Confirm}}
`

// Template with rust source code for a outer structure of a font file
const fontFileTemplate = `// DO NOT MAKE EDITS HERE because this file is automatically generated.
// To make changes, see blitstr/codegen/main.go
//
// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// NOTE: The copyright notice above applies to the rust source code in this
// file, but not to the bitmap graphics encoded in the DATA array (see credits).
//
// CREDITS:
{{.Font.Legal}}
//! {{.Font.Name}} Font
#![forbid(unsafe_code)]
#![allow(dead_code)]

/// Maximum height of glyph patterns in this bitmap typeface.
/// This will be true: h + y_offset <= MAX_HEIGHT
pub const MAX_HEIGHT: u8 = {{.Font.Size}};

{{.Data}}
`

// Template with rust source code for the data and index portion of a font file
const dataTemplate = `/// Seed for Murmur3 hashes in the HASH_* index arrays
pub const M3_SEED: u32 = {{.M3Seed}};

/// Return Okay(offset into DATA[]) for start of blit pattern for grapheme cluster.
///
/// Before doing an expensive lookup for the whole cluster, this does a pre-filter
/// check to see whether the first character falls into one of the codepoint ranges
/// for Unicode blocks included in this font.
///
/// Returns: Result<(blit pattern offset into DATA, bytes of cluster used by match)>
pub fn get_blit_pattern_offset(cluster: &str) -> Result<(usize, usize), super::GlyphNotFound> {
    let first_char: u32;
    match cluster.chars().next() {
        Some(c) => first_char = c as u32,
        None => return Err(super::GlyphNotFound),
    }
    return match first_char {
        {{ range $_, $k := .RB.IndexKeys -}}
        {{- with $dex := index $.RB.Index $k -}}
        0x{{printf "%X" $k.Low}}..=0x{{printf "%X" $k.High}} => {
            {{ range $_, $gcLen := $dex.ClusterLengthList -}}
            if let Some((offset, bytes_used)) = find_{{ToLower $k.Name}}(cluster, {{$gcLen}}) {
                Ok((offset, bytes_used))
            } else {{ end }}{
                Err(super::GlyphNotFound)
            }
        }
        {{ end -}}
        {{- end -}}
        _ => Err(super::GlyphNotFound),
    };
}

{{ range $_, $k := .RB.IndexKeys -}}
{{- with $dex := index $.RB.Index $k -}}
/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_{{ToLower $k.Name}}(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_{{$k.Name}}.binary_search(&key) {
        Ok(index) => return Some((OFFSET_{{$k.Name}}[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_{{$k.Name}}
const HASH_{{$k.Name}}: [u32; {{len $dex}}] = [
    {{$dex.RustCodeForClusterHashes}}
];

/// Lookup table of blit pattern offsets; sort matches HASH_{{$k.Name}}
const OFFSET_{{$k.Name}}: [usize; {{len $dex}}] = [
    {{$dex.RustCodeForOffsets}}
];

{{ end -}}
{{- end -}}

/// Packed glyph pattern data.
/// Record format:
///  [offset+0]: ((w as u8) << 16) | ((h as u8) << 8) | (yOffset as u8)
///  [offset+1..=ceil(w*h/32)]: packed 1-bit pixels; 0=clear, 1=set
/// Pixels are packed in top to bottom, left to right order with MSB of first
/// pixel word containing the top left pixel.
///  w: Width of pattern in pixels
///  h: Height of pattern in pixels
///  yOffset: Vertical offset (pixels downward from top of line) to position
///     glyph pattern properly relative to text baseline
pub const DATA: [u32; {{.RB.DataLen}}] = [
{{.RB.Code}}];`

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
