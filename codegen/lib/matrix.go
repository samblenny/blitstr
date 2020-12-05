// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"fmt"
)

// Holds a matrix of pixel values
type Matrix [][]int

// Holds one row of pixel values from a matrix
type MatrixRow []int

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
func (m Matrix) convertToPattern(yOffset uint32) []uint32 {
	// Pack trimmed pattern into a byte array
	patW := uint32(0)
	patH := uint32(0)
	if len(m) > 0 && len(m[0]) > 0 {
		patW = uint32(len(m[0]))
		patH = uint32(len(m))
	}
	pattern := []uint32{(patW << 16) | (patH << 8) | yOffset}
	bufWord := uint32(0)
	flushed := false
	for y := uint32(0); y < patH; y++ {
		for x := uint32(0); x < patW; x++ {
			if m[y][patW-1-x] > 0 {
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

// Trim pixel matrix to remove whitespace around the glyph. Return the trimmed
// matrix and the y-offset (pixels of top whitespace that were trimmed).
func (m Matrix) Trim(font FontSpec, row int, col int) (Matrix, uint32) {
	// Trim left whitespace
	trblTrimLimit := font.TrimLimits(row, col)
	m = m.transpose()
	m = m.trimLeadingEmptyRows(trblTrimLimit[3])
	// Trim right whitespace
	m = m.reverseRows()
	m = m.trimLeadingEmptyRows(trblTrimLimit[1])
	m = m.reverseRows()
	m = m.transpose()
	// Trim top whitespace and calculate y-offset
	preTrimH := len(m)
	m = m.trimLeadingEmptyRows(trblTrimLimit[0])
	yOffset := preTrimH - len(m)
	// Trim bottom whitespace
	m = m.reverseRows()
	m = m.trimLeadingEmptyRows(trblTrimLimit[2])
	m = m.reverseRows()
	return m, uint32(yOffset)
}

// Transpose a matrix (flip around diagonal)
func (m Matrix) transpose() Matrix {
	if len(m) < 1 {
		return m
	}
	w := len(m[0])
	h := len(m)
	var transposed Matrix
	for col := 0; col < w; col++ {
		var trRow []int
		for row := 0; row < h; row++ {
			trRow = append(trRow, m[row][col])
		}
		transposed = append(transposed, trRow)
	}
	return transposed
}

// Reverse the order of rows in a matrix
func (m Matrix) reverseRows() Matrix {
	var reversed Matrix
	for i := len(m) - 1; i >= 0; i-- {
		reversed = append(reversed, m[i])
	}
	return reversed
}

// Trim whitespace rows from top of matrix
func (m Matrix) trimLeadingEmptyRows(limit int) Matrix {
	if len(m) < 1 {
		return m
	}
	for i := 0; i < limit; i++ {
		sum := 0
		for _, n := range m[0] {
			sum += n
		}
		if len(m) > 0 && sum == 0 {
			m = m[1:]
		} else {
			break
		}
	}
	return m
}

// Dump an ASCII art approximation of the blit pattern to stdout. This can help
// with troubleshooting character map setup when adding a new font.
func (m Matrix) Debug(cs CharSpec, enable bool) {
	if enable {
		cp := cs.FirstCodepoint
		cluster := cs.GraphemeCluster()
		fmt.Printf("%X: '%s' = %+q\n", cp, cluster, cluster)
		fmt.Println(m.convertToText())
	}
}

// Return glyph as text with one ASCII char per pixel
func (m Matrix) convertToText() string {
	var ascii string
	for _, row := range m {
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
