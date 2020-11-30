// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package font

import (
	"fmt"
	"image"
	"strings"
)

type Matrix [][]int
type MatrixRow []int
type BlitPattern struct {
	Words []uint32
	CS    CharSpec
}

// Holds description of sprite sheet and character map for generating a font
type FontSpec struct {
	Name    string // Name of font
	Sprites string // Which file holds the sprite sheet image with the grid of glyphs?
	Size    int    // How many pixels on a side is each glyph (precondition: square glyphs)
	Cols    int    // How many glyphs wide is the grid?
	Gutter  int    // How many px between glyphs?
	Border  int    // How many px wide are top and left borders?
	Legal   string // What credits or license notices need to be included in font file comments?
	RustOut string // Where should the generated source code go?
}

// Extract matrix of pixels from an image containing grid of glyphs
// - img: image.Image from png file containing glyph grid
// - font: Glyph sheet specs (glyph size, border/gutter, etc)
// - cs: Character specs (source row and column in glyph grid)
func ConvertGlyphToBlitPattern(img image.Image, font FontSpec, cs CharSpec, dbg bool) BlitPattern {
	row := cs.Row
	col := cs.Col
	imgRect := img.Bounds()
	rows := (imgRect.Max.Y - font.Border) / (font.Size + font.Gutter)
	if row < 0 || row >= rows || col < 0 || col >= font.Cols {
		panic("row or column out of range")
	}
	// Get pixels for grid cell, converting from RGBA to 1-bit
	gridSize := font.Size + font.Gutter
	border := font.Border
	pxMatrix := Matrix{}
	for y := border + (row * gridSize); y < (row+1)*gridSize; y++ {
		var row MatrixRow
		for x := border + (col * gridSize); x < (col+1)*gridSize; x++ {
			r, _, _, _ := img.At(x, y).RGBA()
			//fmt.Println(r, g, b, a)
			if r == 0 {
				row = append(row, 1)
			} else {
				row = append(row, 0)
			}
		}
		pxMatrix = append(pxMatrix, row)
	}
	pxMatrix, yOffset := trimMatrix(font, row, col, pxMatrix)
	debugMatrix(cs, pxMatrix, dbg)
	patternBytes := convertMatrixToPattern(pxMatrix, yOffset)
	return BlitPattern{patternBytes, cs}
}

// Trim pixel matrix to remove whitespace around the glyph. Return the trimmed
// matrix and the y-offset (pixels of top whitespace that were trimmed).
func trimMatrix(font FontSpec, row int, col int, pxMatrix Matrix) (Matrix, uint32) {
	// Trim left whitespace
	trblTrimLimit := trimLimits(font, row, col)
	pxMatrix = matrixTranspose(pxMatrix)
	pxMatrix = trimLeadingEmptyRows(pxMatrix, trblTrimLimit[3])
	// Trim right whitespace
	pxMatrix = reverseRows(pxMatrix)
	pxMatrix = trimLeadingEmptyRows(pxMatrix, trblTrimLimit[1])
	pxMatrix = reverseRows(pxMatrix)
	pxMatrix = matrixTranspose(pxMatrix)
	// Trim top whitespace and calculate y-offset
	preTrimH := len(pxMatrix)
	pxMatrix = trimLeadingEmptyRows(pxMatrix, trblTrimLimit[0])
	yOffset := preTrimH - len(pxMatrix)
	// Trim bottom whitespace
	pxMatrix = reverseRows(pxMatrix)
	pxMatrix = trimLeadingEmptyRows(pxMatrix, trblTrimLimit[2])
	pxMatrix = reverseRows(pxMatrix)
	return pxMatrix, uint32(yOffset)
}

// Dump an ASCII art approximation of the blit pattern to stdout. This can help
// with troubleshooting character map setup when adding a new font.
func debugMatrix(cs CharSpec, matrix Matrix, enable bool) {
	if enable {
		cp := cs.FirstCodepoint
		cluster := cs.GraphemeCluster()
		fmt.Printf("%X: '%s' = %+q\n", cp, cluster, cluster)
		fmt.Println(convertMatrixToText(matrix))
	}
}

// Return glyph as text with one ASCII char per pixel
func convertMatrixToText(matrix Matrix) string {
	var ascii string
	for _, row := range matrix {
		for _, px := range row {
			if px == 1 {
				ascii += "#"
			} else {
				ascii += "."
			}
		}
		ascii += "\n"
	}
	return ascii
}

// Reverse the order of rows in a matrix
func reverseRows(src Matrix) Matrix {
	var dest Matrix
	for i := len(src) - 1; i >= 0; i-- {
		dest = append(dest, src[i])
	}
	return dest
}

// Trim whitespace rows from top of matrix
func trimLeadingEmptyRows(pxMatrix Matrix, limit int) Matrix {
	if len(pxMatrix) < 1 {
		return pxMatrix
	}
	for i := 0; i < limit; i++ {
		sum := 0
		for _, n := range pxMatrix[0] {
			sum += n
		}
		if len(pxMatrix) > 0 && sum == 0 {
			pxMatrix = pxMatrix[1:]
		} else {
			break
		}
	}
	return pxMatrix
}

// Look up trim limits based on row & column in glyph grid
func trimLimits(font FontSpec, row int, col int) [4]int {
	if font.Name == "Bold" || font.Name == "Regular" || font.Name == "Small" {
		// Space gets 4px width and 2px height
		if col == 2 && row == 0 {
			lr := (font.Size / 2) - 2
			tb := (font.Size / 2) - 1
			return [4]int{tb, lr, tb, lr}
		}
	}
	// Everything else gets max trim
	return [4]int{font.Size, font.Size, font.Size, font.Size}
}

// Return pixel matrix as pattern packed into a [u32] array.
// pat[0]: ((width of blit pattern in px for trimmed glyph as u8) << 16)
//         | (height of blit pattern in px for trimmed glyph as u8) << 8)
//         | (number of blank rows trimmed from top of glyph as u8)
// pat[1:(1+ceiling(w*h/32))]: 1-bit pixels packed into u32 words
//
// Pixel bit values are intended as a background/foreground mask for use with
// XOR blit. Color palette is not set. Rather, palette depends on contents of
// whatever bitmap the blit pattern gets XOR'ed with.
//
// Meaning of pixel bit values in blit pattern:
// - bit=0: keep color of pixel from background bitmap
// - bit=1: invert color of pixel from background bitmap
//
// Pixel packing happens in row-major order (first left to right, then top to
// bottom) with the glyph's top-left pixel placed in the most significant bit
// of the first pixel word. Patterns that need padding because their size is
// not a multiple of 32 bits (width*height % 32 != 0) get padded with zeros in
// the least significant bits of the last word.
func convertMatrixToPattern(pxMatrix Matrix, yOffset uint32) []uint32 {
	// Pack trimmed pattern into a byte array
	patW := uint32(0)
	patH := uint32(0)
	if len(pxMatrix) > 0 && len(pxMatrix[0]) > 0 {
		patW = uint32(len(pxMatrix[0]))
		patH = uint32(len(pxMatrix))
	}
	pattern := []uint32{(patW << 16) | (patH << 8) | yOffset}
	bufWord := uint32(0)
	flushed := false
	for y := uint32(0); y < patH; y++ {
		for x := uint32(0); x < patW; x++ {
			if pxMatrix[y][patW-1-x] > 0 {
				bufWord = (bufWord << 1) | 1
			} else {
				bufWord = (bufWord << 1)
			}
			flushed = false
			if (y*patW+x)%32 == 31 {
				pattern = append(pattern, bufWord)
				bufWord = 0
				flushed = true
			}
		}
	}
	if !flushed {
		finalShift := 32 - ((patW * patH) % 32)
		pattern = append(pattern, bufWord<<finalShift)
	}
	return pattern
}

// Convert blit pattern to rust source code for part of an array of bytes
func ConvertPatternToRust(pattern BlitPattern, comment string) string {
	patternStr := fmt.Sprintf("    // %s\n    ", comment)
	wordsPerRow := uint32(8)
	ceilRow := uint32(len(pattern.Words)) / wordsPerRow
	if uint32(len(pattern.Words))%wordsPerRow > 0 {
		ceilRow += 1
	}
	for i := uint32(0); i < ceilRow; i++ {
		start := i * wordsPerRow
		end := min(uint32(len(pattern.Words)), (i+1)*wordsPerRow)
		line := pattern.Words[start:end]
		var s []string
		for _, word := range line {
			s = append(s, fmt.Sprintf("0x%08x", word))
		}
		patternStr += strings.Join(s, ", ") + ","
		if end < uint32(len(pattern.Words)) {
			patternStr += "\n    "
		}
	}
	patternStr += "\n"
	return patternStr
}

// Transpose a matrix (flip around diagonal)
func matrixTranspose(matrix Matrix) Matrix {
	if len(matrix) < 1 {
		return matrix
	}
	w := len(matrix[0])
	h := len(matrix)
	var transposed Matrix
	for col := 0; col < w; col++ {
		var trRow []int
		for row := 0; row < h; row++ {
			trRow = append(trRow, matrix[row][col])
		}
		transposed = append(transposed, trRow)
	}
	return transposed
}

// Return lowest value among two integers
func min(a uint32, b uint32) uint32 {
	if b > a {
		return a
	}
	return b
}
