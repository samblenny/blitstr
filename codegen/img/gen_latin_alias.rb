# Build an alias list mapping from Unicode normalization form C to form D for
# grapheme clusters in the hex cluster list below (Unicode Latin* blocks).
#
# Usage:
#    ruby gen_latin_alias.rb > latin_alias.txt
#

# Hex formatted grapheme clusters in normalization form C for Unicode blocks
# Basic Latin, Latin 1 Supplement, and Latin Extended A
hex_clusters_norm_C = [
"20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2A", "2B", "2C",
"2D", "2E", "2F", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
"3A", "3B", "3C", "3D", "3E", "3F", "40", "41", "42", "43", "44", "45", "46",
"47", "48", "49", "4A", "4B", "4C", "4D", "4E", "4F", "50", "51", "52", "53",
"54", "55", "56", "57", "58", "59", "5A", "5B", "5C", "5D", "5E", "5F", "60",
"61", "62", "63", "64", "65", "66", "67", "68", "69", "6A", "6B", "6C", "6D",
"6E", "6F", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7A",
"7B", "7C", "7D", "7E", "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8",
"A9", "AA", "AB", "AC", "AD", "AE", "AF", "B0", "B1", "B2", "B3", "B4", "B5",
"B6", "B7", "B8", "B9", "BA", "BB", "BC", "BD", "BE", "BF", "C0", "C1", "C2",
"C3", "C4", "C5", "C6", "C7", "C8", "C9", "CA", "CB", "CC", "CD", "CE", "CF",
"D0", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "DA", "DB", "DC",
"DD", "DE", "DF", "E0", "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9",
"EA", "EB", "EC", "ED", "EE", "EF", "F0", "F1", "F2", "F3", "F4", "F5", "F6",
"F7", "F8", "F9", "FA", "FB", "FC", "FD", "FE", "FF", "152", "153",
]

for hex_C in hex_clusters_norm_C
        # Convert hex cluster to a Unicode string
        cluster_C = hex_C.split("-").map {|scalar| scalar.to_i(16).chr(Encoding::UTF_8)}.join()
        # Compute normalization form D
        cluster_D = cluster_C.unicode_normalize(:nfd)
        hex_D = cluster_D.codepoints.map {|c| c.to_s(16).upcase}.join("-")
        # Print a line of the alias file if form D differs from form C
        if cluster_C == cluster_D then next end
        puts "#{hex_C} #{hex_D}   # nfc: [#{hex_C}, #{cluster_C}],  nfd: [#{hex_D}, #{cluster_D}]"
end
