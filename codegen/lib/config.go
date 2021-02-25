// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
package lib

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"strings"
)

// Holds data from top level of JSON config file
type Config struct {
	Comment   []string
	GlyphSets []ConfigGlyphSet
}

// Holds data from elements of {GlyphSets:[...]} from the JSON config file
type ConfigGlyphSet struct {
	Name      string
	M3Seed    uint32
	Sprites   string
	Size      int
	Cols      int
	Gutter    int
	Border    int
	Legal     string
	Index     string
	IndexType string
	Aliases   string
	GlyphTrim string
	RustOut   string
}

// Holds data parsed from a json index file
type ConfigJsonIndex struct {
	Comment []string
	Map     []CharSpec
}

// Read the config file to make a config object
func NewConfig(configFile string) Config {
	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		panic(err)
	}
	var config Config
	err = json.Unmarshal(data, &config)
	if err != nil {
		panic(err)
	}
	return config
}

// Generate font glyph set specifications with character maps, aliases, etc.
func (c Config) Fonts() []FontSpec {
	list := []FontSpec{}
	for _, gs := range c.GlyphSets {
		fs := FontSpec{
			gs.Name, gs.Sprites, gs.Size, gs.Cols, gs.Gutter, gs.Border,
			gs.readLegal(),
			gs.graphemeClusterMap(), gs.graphemeClusterAliases(),
			gs.RustOut, gs.GlyphTrim, gs.M3Seed,
		}
		list = append(list, fs)
	}
	return list
}

// Read the legal notice for a config glyph set
func (c ConfigGlyphSet) readLegal() string {
	if c.Legal != "" {
		data, err := ioutil.ReadFile(c.Legal)
		if err != nil {
			panic(err)
		}
		return strings.TrimSpace(string(data))
	} else {
		return ""
	}
}

// Generate a list of grapheme cluster to sprite grid coordinate mappings
func (c ConfigGlyphSet) graphemeClusterMap() []CharSpec {
	switch c.IndexType {
	case "txt-row-major":
		return EmojiMap(c.Cols, c.Index)
	case "json-grid-coord":
		return c.readJsonClusterMap()
	default:
		panic(fmt.Errorf("bad indexType: %s", c.IndexType))
	}
}

// Generate a list of grapheme cluster aliases from the config's alias file
func (c ConfigGlyphSet) graphemeClusterAliases() []GCAlias {
	if c.Aliases != "" {
		return EmojiAliases(c.Aliases)
	} else {
		return []GCAlias{}
	}
}

// Generate a grapheme cluster map from a config glyph set json index file
func (c ConfigGlyphSet) readJsonClusterMap() []CharSpec {
	data, err := ioutil.ReadFile(c.Index)
	if err != nil {
		panic(err)
	}
	var cji ConfigJsonIndex
	err = json.Unmarshal(data, &cji)
	if err != nil {
		panic(err)
	}
	return cji.Map
}
