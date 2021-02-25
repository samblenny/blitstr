#!/usr/bin/ruby
# coding: utf-8
# Copyright (c) 2021 Sam Blenny
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Generate json config, index, and alias files for codegen. Resorting to this
# depth of meta-config feels a bit silly. But, working from ruby dictionaries
# allows for better commenting and the chance to avoid json syntax errors in
# manual edits. Ruby also has good library support for translating between
# Unicode Normalization Forms, which is useful for making alias lists.
#
# Usage:
#
#     ruby config_editor.rb
#
require 'json'

config_outfile = "config.json"
latin_index_outfile = "src_data/latin_index.json"
latin_alias_outfile = "src_data/latin_aliases.txt"
icon_index_outfile = "src_data/icon_index.json"

config = {
  comment: [
    "Config for use with main.go. Before making changes here, see config_editor.rb.",
  ],
  glyphSets: [
    {
      name: "Emoji",
      m3Seed: 0,
      sprites: "src_data/emoji_13_0.png", size: 32, cols: 16, gutter: 0, border: 0,
      legal: "src_data/twemoji_legal.txt",
      index: "src_data/emoji_13_0_index.txt",
      indexType: "txt-row-major",
      aliases: "src_data/emoji_13_0_aliases.txt",
      glyphTrim: "CJK",
      rustout: "../src/fonts/emoji.rs",
    },
    {
      name: "Hanzi",
      m3Seed: 0,
      sprites: "src_data/hanzi_core2020_g.png", size: 32, cols: 20, gutter: 2, border: 2,
      legal: "src_data/noto_sans_sc_legal.txt",
      index: "src_data/hanzi_core2020_g_index.txt",
      indexType: "txt-row-major",
      aliases: "",
      glyphTrim: "CJK",
      rustout: "../src/fonts/hanzi.rs",
    },
    {
      name: "Bold",
      m3Seed: 0,
      sprites: "src_data/bold.png", size: 30, cols: 16, gutter: 2, border: 2,
      legal: "src_data/chicago_legal.txt",
      index: latin_index_outfile,
      indexType: "json-grid-coord",
      aliases: "src_data/latin_aliases.txt",
      glyphTrim: "proportional",
      rustout: "../src/fonts/bold.rs",
    },
    {
      name: "Regular",
      m3Seed: 0,
      sprites: "src_data/regular.png", size: 30, cols: 16, gutter: 2, border: 2,
      legal: "src_data/geneva_legal.txt",
      index: latin_index_outfile,
      indexType: "json-grid-coord",
      aliases: "src_data/latin_aliases.txt",
      glyphTrim: "proportional",
      rustout: "../src/fonts/regular.rs",
    },
    {
      name: "Small",
      m3Seed: 0,
      sprites: "src_data/small.png", size: 24, cols: 16, gutter: 2, border: 2,
      legal: "src_data/geneva_legal.txt",
      index: latin_index_outfile,
      indexType: "json-grid-coord",
      aliases: "src_data/latin_aliases.txt",
      glyphTrim: "proportional",
      rustout: "../src/fonts/small.rs",
    }
    # ,{
    #   name: "Icons",
    #   m3seed: 0,
    #   sprites: "src_data/regular.png", size: 30, cols: 16, gutter: 2, border: 2,
    #   legal: "",
    #   index: icon_index_outfile,
    #   indexType: "json-grid-coord",
    #   aliases: "",
    #   glyphTrim: "proportional",
    #   rustout: "../src/fonts/icons.rs",
    # }
  ]
}

latin_index = {
  comment: [
    "Before making changes here, see ../config_editor.rb.",
  ],
  map: [
    # Unicode Basic Latin block
    {hex: "20", row: 0, col: 2, label: " "},
    {hex: "21", row: 1, col: 2, label: "!"},
    {hex: "22", row: 2, col: 2, label: "\""},
    {hex: "23", row: 3, col: 2, label: "#"},
    {hex: "24", row: 4, col: 2, label: "$"},
    {hex: "25", row: 5, col: 2, label: "%"},
    {hex: "26", row: 6, col: 2, label: "&"},
    {hex: "27", row: 7, col: 2, label: "'"},
    {hex: "28", row: 8, col: 2, label: "("},
    {hex: "29", row: 9, col: 2, label: ")"},
    {hex: "2A", row: 10, col: 2, label: "*"},
    {hex: "2B", row: 11, col: 2, label: "+"},
    {hex: "2C", row: 12, col: 2, label: ","},
    {hex: "2D", row: 13, col: 2, label: "-"},
    {hex: "2E", row: 14, col: 2, label: "."},
    {hex: "2F", row: 15, col: 2, label: "/"},
    {hex: "30", row: 0, col: 3, label: "0"},
    {hex: "31", row: 1, col: 3, label: "1"},
    {hex: "32", row: 2, col: 3, label: "2"},
    {hex: "33", row: 3, col: 3, label: "3"},
    {hex: "34", row: 4, col: 3, label: "4"},
    {hex: "35", row: 5, col: 3, label: "5"},
    {hex: "36", row: 6, col: 3, label: "6"},
    {hex: "37", row: 7, col: 3, label: "7"},
    {hex: "38", row: 8, col: 3, label: "8"},
    {hex: "39", row: 9, col: 3, label: "9"},
    {hex: "3A", row: 10, col: 3, label: ":"},
    {hex: "3B", row: 11, col: 3, label: ";"},
    {hex: "3C", row: 12, col: 3, label: "<"},
    {hex: "3D", row: 13, col: 3, label: "="},
    {hex: "3E", row: 14, col: 3, label: ">"},
    {hex: "3F", row: 15, col: 3, label: "?"},
    {hex: "40", row: 0, col: 4, label: "@"},
    {hex: "41", row: 1, col: 4, label: "A"},
    {hex: "42", row: 2, col: 4, label: "B"},
    {hex: "43", row: 3, col: 4, label: "C"},
    {hex: "44", row: 4, col: 4, label: "D"},
    {hex: "45", row: 5, col: 4, label: "E"},
    {hex: "46", row: 6, col: 4, label: "F"},
    {hex: "47", row: 7, col: 4, label: "G"},
    {hex: "48", row: 8, col: 4, label: "H"},
    {hex: "49", row: 9, col: 4, label: "I"},
    {hex: "4A", row: 10, col: 4, label: "J"},
    {hex: "4B", row: 11, col: 4, label: "K"},
    {hex: "4C", row: 12, col: 4, label: "L"},
    {hex: "4D", row: 13, col: 4, label: "M"},
    {hex: "4E", row: 14, col: 4, label: "N"},
    {hex: "4F", row: 15, col: 4, label: "O"},
    {hex: "50", row: 0, col: 5, label: "P"},
    {hex: "51", row: 1, col: 5, label: "Q"},
    {hex: "52", row: 2, col: 5, label: "R"},
    {hex: "53", row: 3, col: 5, label: "S"},
    {hex: "54", row: 4, col: 5, label: "T"},
    {hex: "55", row: 5, col: 5, label: "U"},
    {hex: "56", row: 6, col: 5, label: "V"},
    {hex: "57", row: 7, col: 5, label: "W"},
    {hex: "58", row: 8, col: 5, label: "X"},
    {hex: "59", row: 9, col: 5, label: "Y"},
    {hex: "5A", row: 10, col: 5, label: "Z"},
    {hex: "5B", row: 11, col: 5, label: "["},
    {hex: "5C", row: 12, col: 5, label: "\\"},
    {hex: "5D", row: 13, col: 5, label: "]"},
    {hex: "5E", row: 14, col: 5, label: "^"},
    {hex: "5F", row: 15, col: 5, label: "_"},
    {hex: "60", row: 0, col: 6, label: "`"},
    {hex: "61", row: 1, col: 6, label: "a"},
    {hex: "62", row: 2, col: 6, label: "b"},
    {hex: "63", row: 3, col: 6, label: "c"},
    {hex: "64", row: 4, col: 6, label: "d"},
    {hex: "65", row: 5, col: 6, label: "e"},
    {hex: "66", row: 6, col: 6, label: "f"},
    {hex: "67", row: 7, col: 6, label: "g"},
    {hex: "68", row: 8, col: 6, label: "h"},
    {hex: "69", row: 9, col: 6, label: "i"},
    {hex: "6A", row: 10, col: 6, label: "j"},
    {hex: "6B", row: 11, col: 6, label: "k"},
    {hex: "6C", row: 12, col: 6, label: "l"},
    {hex: "6D", row: 13, col: 6, label: "m"},
    {hex: "6E", row: 14, col: 6, label: "n"},
    {hex: "6F", row: 15, col: 6, label: "o"},
    {hex: "70", row: 0, col: 7, label: "p"},
    {hex: "71", row: 1, col: 7, label: "q"},
    {hex: "72", row: 2, col: 7, label: "r"},
    {hex: "73", row: 3, col: 7, label: "s"},
    {hex: "74", row: 4, col: 7, label: "t"},
    {hex: "75", row: 5, col: 7, label: "u"},
    {hex: "76", row: 6, col: 7, label: "v"},
    {hex: "77", row: 7, col: 7, label: "w"},
    {hex: "78", row: 8, col: 7, label: "x"},
    {hex: "79", row: 9, col: 7, label: "y"},
    {hex: "7A", row: 10, col: 7, label: "z"},
    {hex: "7B", row: 11, col: 7, label: "{"},
    {hex: "7C", row: 12, col: 7, label: "|"},
    {hex: "7D", row: 13, col: 7, label: "}"},
    {hex: "7E", row: 14, col: 7, label: "~"},

    # Unicode Latin 1 block
    {hex: "A0", row: 0, col: 2, label: "No-Break Space"},
    {hex: "A1", row: 1, col: 12, label: "¡"},
    {hex: "A2", row: 2, col: 10, label: "¢"},
    {hex: "A3", row: 3, col: 10, label: "£"},
    {hex: "A4", row: 15, col: 1, label: "¤"},
    {hex: "A5", row: 4, col: 11, label: "¥"},
    {hex: "A6", row: 15, col: 7, label: "¦"},
    {hex: "A7", row: 4, col: 10, label: "§"},
    {hex: "A8", row: 12, col: 10, label: "¨"},
    {hex: "A9", row: 9, col: 10, label: "©"},
    {hex: "AA", row: 11, col: 11, label: "ª"},
    {hex: "AB", row: 7, col: 12, label: "«"},
    {hex: "AC", row: 2, col: 12, label: "¬"},
    {hex: "AD", row: 13, col: 2, label: "Soft Hyphen"},
    {hex: "AE", row: 8, col: 10, label: "®"},
    {hex: "AF", row: 8, col: 15, label: "¯ Macron"},
    {hex: "B0", row: 1, col: 10, label: "° Degree Sign"},
    {hex: "B1", row: 1, col: 11, label: "±"},
    {hex: "B2", row: 3, col: 1, label: "²"},
    {hex: "B3", row: 4, col: 1, label: "³"},
    {hex: "B4", row: 11, col: 10, label: "´"},
    {hex: "B5", row: 5, col: 11, label: "µ"},
    {hex: "B6", row: 6, col: 10, label: "¶"},
    {hex: "B7", row: 1, col: 14, label: "·"},
    {hex: "B8", row: 12, col: 15, label: "¸ Cedillia"},
    {hex: "B9", row: 2, col: 1, label: "¹"},
    {hex: "BA", row: 12, col: 11, label: "º"},
    {hex: "BB", row: 8, col: 12, label: "»"},
    {hex: "BC", row: 5, col: 1, label: "¼"},
    {hex: "BD", row: 6, col: 1, label: "½"},
    {hex: "BE", row: 7, col: 1, label: "¾"},
    {hex: "BF", row: 0, col: 12, label: "¿"},
    {hex: "C0", row: 11, col: 12, label: "À"},
    {hex: "C1", row: 7, col: 14, label: "Á"},
    {hex: "C2", row: 5, col: 14, label: "Â"},
    {hex: "C3", row: 12, col: 12, label: "Ã"},
    {hex: "C4", row: 0, col: 8, label: "Ä"},
    {hex: "C5", row: 1, col: 8, label: "Å"},
    {hex: "C6", row: 14, col: 10, label: "Æ"},
    {hex: "C7", row: 2, col: 8, label: "Ç"},
    {hex: "C8", row: 9, col: 14, label: "È"},
    {hex: "C9", row: 3, col: 8, label: "É"},
    {hex: "CA", row: 6, col: 14, label: "Ê"},
    {hex: "CB", row: 8, col: 14, label: "Ë"},
    {hex: "CC", row: 13, col: 14, label: "Ì"},
    {hex: "CD", row: 10, col: 14, label: "Í"},
    {hex: "CE", row: 11, col: 14, label: "Î"},
    {hex: "CF", row: 12, col: 14, label: "Ï"},
    {hex: "D0", row: 8, col: 1, label: "Ð"},
    {hex: "D1", row: 4, col: 8, label: "Ñ"},
    {hex: "D2", row: 1, col: 15, label: "Ò"},
    {hex: "D3", row: 14, col: 14, label: "Ó"},
    {hex: "D4", row: 15, col: 14, label: "Ô"},
    {hex: "D5", row: 13, col: 12, label: "Õ"},
    {hex: "D6", row: 5, col: 8, label: "Ö"},
    {hex: "D7", row: 9, col: 1, label: "× Multiplication Sign"},
    {hex: "D8", row: 15, col: 10, label: "Ø"},
    {hex: "D9", row: 4, col: 15, label: "Ù"},
    {hex: "DA", row: 2, col: 15, label: "Ú"},
    {hex: "DB", row: 3, col: 15, label: "Û"},
    {hex: "DC", row: 6, col: 8, label: "Ü"},
    {hex: "DD", row: 10, col: 1, label: "Ý"},
    {hex: "DE", row: 11, col: 1, label: "Þ"},
    {hex: "DF", row: 7, col: 10, label: "ß"},
    {hex: "E0", row: 8, col: 8, label: "à"},
    {hex: "E1", row: 7, col: 8, label: "á"},
    {hex: "E2", row: 9, col: 8, label: "â"},
    {hex: "E3", row: 11, col: 8, label: "ã"},
    {hex: "E4", row: 10, col: 8, label: "ä"},
    {hex: "E5", row: 12, col: 8, label: "å"},
    {hex: "E6", row: 14, col: 11, label: "æ"},
    {hex: "E7", row: 13, col: 8, label: "ç"},
    {hex: "E8", row: 15, col: 8, label: "è"},
    {hex: "E9", row: 14, col: 8, label: "é"},
    {hex: "EA", row: 0, col: 9, label: "ê"},
    {hex: "EB", row: 1, col: 9, label: "ë"},
    {hex: "EC", row: 3, col: 9, label: "ì"},
    {hex: "ED", row: 2, col: 9, label: "í"},
    {hex: "EE", row: 4, col: 9, label: "î"},
    {hex: "EF", row: 5, col: 9, label: "ï"},
    {hex: "F0", row: 12, col: 1, label: "ð"},
    {hex: "F1", row: 6, col: 9, label: "ñ"},
    {hex: "F2", row: 8, col: 9, label: "ò"},
    {hex: "F3", row: 7, col: 9, label: "ó"},
    {hex: "F4", row: 9, col: 9, label: "ô"},
    {hex: "F5", row: 11, col: 9, label: "õ"},
    {hex: "F6", row: 10, col: 9, label: "ö"},
    {hex: "F7", row: 6, col: 13, label: "÷"},
    {hex: "F8", row: 15, col: 11, label: "ø"},
    {hex: "F9", row: 13, col: 9, label: "ù"},
    {hex: "FA", row: 12, col: 9, label: "ú"},
    {hex: "FB", row: 14, col: 9, label: "û"},
    {hex: "FC", row: 15, col: 9, label: "ü"},
    {hex: "FD", row: 13, col: 1, label: "ý"},
    {hex: "FE", row: 14, col: 1, label: "þ"},
    {hex: "FF", row: 8, col: 13, label: "ÿ"},

    # Unicode Latin Extended A block
    {hex: "152", row: 14, col: 12, label: "Œ"},
    {hex: "153", row: 15, col: 12, label: "œ"},

    # Unicode General Punctuation block
    {hex: "2018", row: 4, col: 13, label: "‘ Left Single Quotation Mark"},
    {hex: "2019", row: 5, col: 13, label: "’ Right Single Quotation Mark"},
    {hex: "201A", row: 2, col: 14, label: "‚ Single Low-9 Quotation Mark"},
    {hex: "201B", row: 7, col: 11, label: "‛ Single High-Reversed-9 Quotation Mark"},
    {hex: "201C", row: 2, col: 13, label: "“ Left Double Quotation Mark"},
    {hex: "201D", row: 3, col: 13, label: "” Right Double Quotation Mark"},
    {hex: "201E", row: 3, col: 14, label: "„ Double Low-9 Quotation Mark"},
    {hex: "201F", row: 8, col: 11, label: "‟ Double High-Reversed-9 Quotation Mark"},
    {hex: "2020", row: 0, col: 10, label: "†"},
    {hex: "2021", row: 0, col: 14, label: "‡"},
    {hex: "2022", row: 5, col: 10, label: "•"},

    # Unicode Currency Symbols block
    {hex: "20AC", row: 11, col: 13, label: "€"},

    # Unicode Specials Block
    {hex: "FFFD", row: 0, col: 15, label: "�"},
  ]
}

icon_index = {
  comment: [
    "Before making changes here, see ../config_editor.rb.",
  ],
  map: [
    # Unicode Private Use Area assignments for UI icons
    {hex: "E700", row: 0, col: 0, label: "Battery_05"},
    {hex: "E701", row: 1, col: 0, label: "Battery_25"},
    {hex: "E702", row: 2, col: 0, label: "Battery_50"},
    {hex: "E703", row: 3, col: 0, label: "Battery_75"},
    {hex: "E704", row: 4, col: 0, label: "Battery_99"},
    {hex: "E705", row: 5, col: 0, label: "Radio_3"},
    {hex: "E706", row: 6, col: 0, label: "Radio_2"},
    {hex: "E707", row: 7, col: 0, label: "Radio_1"},
    {hex: "E708", row: 8, col: 0, label: "Radio_0"},
    {hex: "E709", row: 9, col: 0, label: "Radio_Off"},
    {hex: "E70A", row: 13, col: 0, label: "Shift_Arrow"},
    {hex: "E70B", row: 14, col: 0, label: "Backspace_Symbol"},
    {hex: "E70C", row: 15, col: 0, label: "Enter_Symbol"},
  ]
}

puts "Preparing to overwrite files..."
puts "  #{config_outfile}"
puts "  #{latin_index_outfile}"
puts "  #{icon_index_outfile}"
puts "  #{latin_alias_outfile}"
print "Do you want to proceed? [y/N]: "
abort "Operation canceled" if !["y", "Y"].include? gets.chomp

puts "writing #{latin_index_outfile}"
File.open(latin_index_outfile, "w") {|f|
  f.write JSON.generate(latin_index, {space: " ", object_nl: " ", array_nl: "\n"})
}
puts "writing #{latin_alias_outfile}"
File.open(latin_alias_outfile, "w") {|f|
  # Loop through all the Normalization Form C hex grapheme clusters in the latin index
  for hex_C in latin_index[:map].map {|x| x[:hex]}
    # Convert hex cluster to a Unicode string
    cluster_C = hex_C.split("-").map {|scalar| scalar.to_i(16).chr(Encoding::UTF_8)}.join()
    # Compute normalization form D
    cluster_D = cluster_C.unicode_normalize(:nfd)
    hex_D = cluster_D.codepoints.map {|c| c.to_s(16).upcase}.join("-")
    # Print a line of the alias file if form D differs from form C
    if cluster_C == cluster_D then next end
    f.puts "#{hex_C} #{hex_D}   # nfc: [#{hex_C}, #{cluster_C}],  nfd: [#{hex_D}, #{cluster_D}]"
  end
}
puts "writing #{icon_index_outfile}"
File.open(icon_index_outfile, "w") {|f|
  f.write JSON.generate(icon_index, {space: " ", object_nl: " ", array_nl: "\n"})
}
puts "writing #{config_outfile}"
File.open(config_outfile, "w") {|f|
  f.puts JSON.pretty_generate(config)
}
