// DO NOT MAKE EDITS HERE because this file is automatically generated.
// To make changes, see blitstr/codegen/main.go
//
// Copyright (c) 2020 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// NOTE: The copyright notice above applies to the rust source code in this
// file, but not to the bitmap graphics encoded in the DATA array (see credits).
//
// CREDITS:
// This code includes encoded bitmaps of glyphs from the Geneva typeface which
// was designed by Susan Kare and released by Apple in 1984. Geneva is a
// registered trademark of Apple Inc.
//
//! Small Font
#![forbid(unsafe_code)]
#![allow(dead_code)]

use super::{GlyphData, NoGlyphErr};

/// Maximum height of glyph patterns in this bitmap typeface.
/// This will be true: h + y_offset <= MAX_HEIGHT
pub const MAX_HEIGHT: u8 = 24;

/// Seed for Murmur3 hashes in the HASH_* index arrays
pub const M3_SEED: u32 = 0;

/// Return Okay(offset into DATA[]) for start of blit pattern for grapheme cluster.
///
/// Before doing an expensive lookup for the whole cluster, this does a pre-filter
/// check to see whether the first character falls into one of the codepoint ranges
/// for Unicode blocks included in this font.
///
/// Returns: Result<(blit pattern offset into DATA, bytes of cluster used by match)>
pub fn get_blit_pattern_offset(cluster: &str) -> Result<(GlyphData, usize), NoGlyphErr> {
    let first_char: u32;
    match cluster.chars().next() {
        Some(c) => first_char = c as u32,
        None => return Err(NoGlyphErr),
    }
    return match first_char {
        0x0..=0x7F => {
            if let Some((offset, bytes_used)) = find_basic_latin(cluster, 2) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else if let Some((offset, bytes_used)) = find_basic_latin(cluster, 1) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else {
                Err(NoGlyphErr)
            }
        }
        0x80..=0xFF => {
            if let Some((offset, bytes_used)) = find_latin_1_supplement(cluster, 1) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else {
                Err(NoGlyphErr)
            }
        }
        0x100..=0x17F => {
            if let Some((offset, bytes_used)) = find_latin_extended_a(cluster, 1) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else {
                Err(NoGlyphErr)
            }
        }
        0x2000..=0x206F => {
            if let Some((offset, bytes_used)) = find_general_punctuation(cluster, 1) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else {
                Err(NoGlyphErr)
            }
        }
        0x20A0..=0x20CF => {
            if let Some((offset, bytes_used)) = find_currency_symbols(cluster, 1) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else {
                Err(NoGlyphErr)
            }
        }
        0xFFF0..=0xFFFF => {
            if let Some((offset, bytes_used)) = find_specials(cluster, 1) {
                Ok((GlyphData::Small(offset), bytes_used))
            } else {
                Err(NoGlyphErr)
            }
        }
        _ => Err(super::NoGlyphErr),
    };
}

/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_basic_latin(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_BASIC_LATIN.binary_search(&key) {
        Ok(index) => return Some((OFFSET_BASIC_LATIN[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_BASIC_LATIN
const HASH_BASIC_LATIN: [u32; 148] = [
    0x0323CD4F,  // "ë" 65-308
    0x049BF55E,  // "}"
    0x0537C05E,  // "È" 45-300
    0x0B9EA876,  // "8"
    0x0D4E2B64,  // "è" 65-300
    0x11DDB5BC,  // "Ù" 55-300
    0x129F2A7F,  // "ã" 61-303
    0x13156136,  // "F"
    0x131AB870,  // "O"
    0x1403A883,  // "Ê" 45-302
    0x1586C0AA,  // "j"
    0x1672836B,  // "`"
    0x16F9A05F,  // "#"
    0x18D53BB6,  // "("
    0x198B90FC,  // "ù" 75-300
    0x1BF82D35,  // "y"
    0x1C40E42F,  // "Ö" 4F-308
    0x1CFCD293,  // ")"
    0x1F2B17F3,  // "Î" 49-302
    0x1F79A4D0,  // "$"
    0x21840E11,  // "q"
    0x22ABE92F,  // "K"
    0x2379B646,  // "Õ" 4F-303
    0x24189EC6,  // "ä" 61-308
    0x259E68D3,  // "6"
    0x26FF6E36,  // "3"
    0x2A3184F1,  // "U"
    0x2B038801,  // "a"
    0x2C17F13B,  // "r"
    0x2D2269F8,  // "m"
    0x30F856B5,  // "Â" 41-302
    0x31099644,  // "t"
    0x315F5687,  // "I"
    0x3673CB35,  // "{"
    0x39BC06BB,  // "L"
    0x435F26F6,  // "Ñ" 4E-303
    0x43603571,  // "C"
    0x44C216BA,  // "R"
    0x455E0578,  // "Ü" 55-308
    0x45EF573F,  // ";"
    0x465D3B9D,  // "í" 69-301
    0x47EF8034,  // "ÿ" 79-308
    0x4938EA00,  // "7"
    0x4AB0FC13,  // "4"
    0x4BCD3197,  // "A"
    0x4CC5A899,  // "|"
    0x524C4F7E,  // "B"
    0x52D3CB36,  // "%"
    0x55C7F2AE,  // "ý" 79-301
    0x56DEE618,  // "v"
    0x5A352E5E,  // "c"
    0x5B06F60A,  // "1"
    0x5C904E6C,  // "_"
    0x5CA0F32F,  // "&"
    0x5FF80125,  // "S"
    0x61901224,  // "5"
    0x67BD451E,  // "ñ" 6E-303
    0x681839B8,  // "Ò" 4F-300
    0x68E75BE7,  // "@"
    0x6C058218,  // "Û" 55-302
    0x6E85A379,  // "x"
    0x72DE1EFB,  // "*"
    0x73688484,  // "p"
    0x7627CA6C,  // "D"
    0x779CDA2E,  // "Y"
    0x7834A19C,  // "ó" 6F-301
    0x78FD8C32,  // "u"
    0x7A435EEE,  // "Ï" 49-308
    0x7B6B1369,  // "9"
    0x7BA23029,  // "J"
    0x7F4EBF76,  // "Ç" 43-327
    0x803AF153,  // "["
    0x804744C4,  // "e"
    0x84510F7D,  // "o"
    0x8499B628,  // "å" 61-30A
    0x86A8B043,  // ","
    0x8B86BA97,  // "="
    0x8C1A54EE,  // "g"
    0x8C69C315,  // " "
    0x8C94654B,  // "ê" 65-302
    0x8DB20A6B,  // "!"
    0x933CEC52,  // "Ä" 41-308
    0x9500C437,  // "<"
    0x9582C02E,  // "G"
    0x98453FD6,  // "?"
    0x98C6B546,  // "ü" 75-308
    0x99933C47,  // "w"
    0x9B44CABE,  // "á" 61-301
    0x9DD6D1DF,  // "â" 61-302
    0x9F99932F,  // "À" 41-300
    0xA03D6D22,  // "Å" 41-30A
    0xA13B361A,  // "/"
    0xA1A410C7,  // "2"
    0xA26A0E29,  // "Ã" 41-303
    0xA30CAD6B,  // "û" 75-302
    0xA5872342,  // "k"
    0xA891E88A,  // "0"
    0xABE208D0,  // "P"
    0xADABEA12,  // "d"
    0xADC91A24,  // "ç" 63-327
    0xAE495307,  // "õ" 6F-303
    0xAF41DB71,  // "]"
    0xB12BF2EE,  // "V"
    0xB35F1351,  // "E"
    0xB454A383,  // "ú" 75-301
    0xB5097BAB,  // "ö" 6F-308
    0xB8ED6BE2,  // "X"
    0xBE856396,  // "ì" 69-300
    0xBFE47D0D,  // "Q"
    0xC06E932F,  // "T"
    0xC0AFD8ED,  // "Ô" 4F-302
    0xC1209A73,  // "f"
    0xC2DDC575,  // "\""
    0xC3AB3492,  // "ò" 6F-300
    0xC57A2C66,  // "Ó" 4F-301
    0xC5E4D96B,  // "î" 69-302
    0xC890E3DD,  // ">"
    0xC99309A2,  // "-"
    0xC9BEA311,  // "Z"
    0xCAECCF17,  // "s"
    0xCAF83468,  // "H"
    0xCB07FD3E,  // "'"
    0xCB7242E0,  // "+"
    0xCD3FDBE3,  // "M"
    0xCE94AE25,  // "b"
    0xD363E17B,  // "à" 61-300
    0xD4FFA898,  // ":"
    0xD7B03F23,  // "z"
    0xD822B857,  // "Ì" 49-300
    0xDA2B151D,  // "\\"
    0xDB01AF22,  // "n"
    0xDC135ABD,  // "N"
    0xE2AA1EBB,  // "."
    0xE5CA55BF,  // "i"
    0xE6556D01,  // "h"
    0xE7C19BA9,  // "é" 65-301
    0xED67FA54,  // "Ý" 59-301
    0xEF026B52,  // "l"
    0xEF302693,  // "Í" 49-301
    0xF04B9422,  // "Ú" 55-301
    0xF0F19A38,  // "~"
    0xF29AB82D,  // "W"
    0xF31D8BFB,  // "ï" 69-308
    0xF3A5A351,  // "Á" 41-301
    0xF3BF2609,  // "É" 45-301
    0xF85223EB,  // "Ë" 45-308
    0xFC2C2430,  // "^"
    0xFFF1D70C,  // "ô" 6F-302
];

/// Lookup table of blit pattern offsets; sort matches HASH_BASIC_LATIN
const OFFSET_BASIC_LATIN: [usize; 148] = [
    898,  // "ë" 65-308
    447,  // "}"
    682,  // "È" 45-300
    112,  // "8"
    883,  // "è" 65-300
    791,  // "Ù" 55-300
    856,  // "ã" 61-303
    180,  // "F"
    230,  // "O"
    694,  // "Ê" 45-302
    361,  // "j"
    320,  // "`"
    6,    // "#"
    36,   // "("
    963,  // "ù" 75-300
    429,  // "y"
    771,  // "Ö" 4F-308
    41,   // ")"
    714,  // "Î" 49-302
    11,   // "$"
    392,  // "q"
    205,  // "K"
    763,  // "Õ" 4F-303
    861,  // "ä" 61-308
    100,  // "6"
    82,   // "3"
    266,  // "U"
    322,  // "a"
    397,  // "r"
    373,  // "m"
    636,  // "Â" 41-302
    405,  // "t"
    197,  // "I"
    439,  // "{"
    211,  // "L"
    731,  // "Ñ" 4E-303
    163,  // "C"
    248,  // "R"
    815,  // "Ü" 55-308
    126,  // ";"
    906,  // "í" 69-301
    997,  // "ÿ" 79-308
    106,  // "7"
    88,   // "4"
    151,  // "A"
    444,  // "|"
    157,  // "B"
    18,   // "%"
    983,  // "ý" 79-301
    413,  // "v"
    331,  // "c"
    73,   // "1"
    318,  // "_"
    26,   // "&"
    254,  // "S"
    94,   // "5"
    922,  // "ñ" 6E-303
    739,  // "Ò" 4F-300
    143,  // "@"
    807,  // "Û" 55-302
    424,  // "x"
    46,   // "*"
    387,  // "p"
    169,  // "D"
    292,  // "Y"
    932,  // "ó" 6F-301
    409,  // "u"
    719,  // "Ï" 49-308
    118,  // "9"
    199,  // "J"
    675,  // "Ç" 43-327
    303,  // "["
    340,  // "e"
    383,  // "o"
    866,  // "å" 61-30A
    56,   // ","
    132,  // "="
    349,  // "g"
    0,    // " "
    893,  // "ê" 65-302
    2,    // "!"
    652,  // "Ä" 41-308
    129,  // "<"
    185,  // "G"
    138,  // "?"
    978,  // "ü" 75-308
    418,  // "w"
    846,  // "á" 61-301
    851,  // "â" 61-302
    620,  // "À" 41-300
    659,  // "Å" 41-30A
    62,   // "/"
    76,   // "2"
    644,  // "Ã" 41-303
    973,  // "û" 75-302
    366,  // "k"
    67,   // "0"
    236,  // "P"
    335,  // "d"
    878,  // "ç" 63-327
    942,  // "õ" 6F-303
    312,  // "]"
    272,  // "V"
    175,  // "E"
    968,  // "ú" 75-301
    947,  // "ö" 6F-308
    286,  // "X"
    903,  // "ì" 69-300
    242,  // "Q"
    260,  // "T"
    755,  // "Ô" 4F-302
    344,  // "f"
    4,    // "\""
    927,  // "ò" 6F-300
    747,  // "Ó" 4F-301
    909,  // "î" 69-302
    135,  // ">"
    58,   // "-"
    298,  // "Z"
    401,  // "s"
    191,  // "H"
    34,   // "'"
    51,   // "+"
    216,  // "M"
    326,  // "b"
    841,  // "à" 61-300
    124,  // ":"
    435,  // "z"
    706,  // "Ì" 49-300
    307,  // "\\"
    379,  // "n"
    224,  // "N"
    60,   // "."
    359,  // "i"
    354,  // "h"
    888,  // "é" 65-301
    822,  // "Ý" 59-301
    371,  // "l"
    710,  // "Í" 49-301
    799,  // "Ú" 55-301
    452,  // "~"
    278,  // "W"
    913,  // "ï" 69-308
    628,  // "Á" 41-301
    688,  // "É" 45-301
    700,  // "Ë" 45-308
    316,  // "^"
    937,  // "ô" 6F-302
];

/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_latin_1_supplement(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_LATIN_1_SUPPLEMENT.binary_search(&key) {
        Ok(index) => return Some((OFFSET_LATIN_1_SUPPLEMENT[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_LATIN_1_SUPPLEMENT
const HASH_LATIN_1_SUPPLEMENT: [u32; 96] = [
    0x00EAC56E,  // "°"
    0x0254FD66,  // "®"
    0x056DE9E6,  // "Î"
    0x06FE368D,  // "Ö"
    0x137E3259,  // "Õ"
    0x18E6F281,  // "à"
    0x1A5A66CD,  // "ø"
    0x1B0DE252,  // "¢"
    0x1B43C661,  // "¤"
    0x1C505947,  // "í"
    0x252158D3,  // "¥"
    0x25872B22,  // "ë"
    0x259437AC,  // "§"
    0x268B196D,  // "ä"
    0x275E5021,  // "¨"
    0x2766361E,  // "Ú"
    0x278A377A,  // "Ì"
    0x28CA6AD7,  // "Ø"
    0x291E971E,  // "Ï"
    0x3280724D,  // "»"
    0x36953E7B,  // "õ"
    0x388FB155,  // "ï"
    0x39056A43,  // "É"
    0x3A073FBE,  // "È"
    0x3AE00F65,  // "ü"
    0x3E168B4D,  // "ó"
    0x3E82329E,  // "û"
    0x402DCFF6,  // "×"
    0x4236CDD1,  // "¾"
    0x44D0C2C4,  // "ê"
    0x450DB83D,  // "Û"
    0x4F92356C,  // "µ"
    0x511BCA1D,  // "á"
    0x554A5349,  // "Æ"
    0x5E39BCEE,  // "´"
    0x5EB9D2A0,  // "ç"
    0x5EE10367,  // "ò"
    0x62D494F4,  // "Ý"
    0x6984AA2D,  // "Í"
    0x6BA96CC3,  // "Ü"
    0x6E3C05CF,  // "ý"
    0x6EF8ED06,  // "\u00AD" Soft Hyphen
    0x6FA3C127,  // "ã"
    0x71EE15F2,  // "Ò"
    0x7619D892,  // "ª"
    0x76554D10,  // "Ç"
    0x79026F8E,  // "Å"
    0x7E2A203C,  // "÷"
    0x839D40CB,  // "£"
    0x84FF8F94,  // "Ô"
    0x8AE0A2B9,  // "å"
    0x8D741045,  // "Þ"
    0x8EA3F31F,  // "³"
    0x9194CCB5,  // "¬"
    0x92979158,  // "ô"
    0x95C4DC63,  // "Ã"
    0x97A9C1DA,  // "¶"
    0x97C63B05,  // "\u00A0" No-Break Space
    0xA10C8120,  // "·"
    0xA5F3368B,  // "¹"
    0xA7883CA5,  // "¸"
    0xA7B0FE42,  // "©"
    0xABF561B2,  // "«"
    0xAD97621D,  // "Ð"
    0xB1A64F1E,  // "Â"
    0xB1BF38B8,  // "º"
    0xB55DA0FA,  // "î"
    0xB5CA05F5,  // "¦"
    0xB7001103,  // "¡"
    0xB9229669,  // "¿"
    0xBA8E9829,  // "Ä"
    0xBB608964,  // "ú"
    0xC0837C42,  // "¼"
    0xC0E7AB63,  // "¯"
    0xC20FE7B7,  // "þ"
    0xC6EF143E,  // "ß"
    0xC98BE718,  // "Á"
    0xCAD0511F,  // "é"
    0xCAF24984,  // "Ù"
    0xD040D3E3,  // "è"
    0xD4D6097A,  // "Ë"
    0xD83534E2,  // "Ñ"
    0xDD061C7A,  // "²"
    0xDE3FB757,  // "ì"
    0xE14F2E81,  // "â"
    0xE2532EDC,  // "ð"
    0xE51FF77C,  // "½"
    0xE5DA3FED,  // "±"
    0xE9131AA1,  // "ù"
    0xEBC8143D,  // "ö"
    0xEC99A516,  // "À"
    0xF446CD1A,  // "Ó"
    0xF520EF41,  // "æ"
    0xF64ED921,  // "ñ"
    0xFB89D3F4,  // "ÿ"
    0xFBFECC1C,  // "Ê"
];

/// Lookup table of blit pattern offsets; sort matches HASH_LATIN_1_SUPPLEMENT
const OFFSET_LATIN_1_SUPPLEMENT: [usize; 96] = [
    529,  // "°"
    518,  // "®"
    714,  // "Î"
    771,  // "Ö"
    763,  // "Õ"
    841,  // "à"
    957,  // "ø"
    459,  // "¢"
    471,  // "¤"
    906,  // "í"
    477,  // "¥"
    898,  // "ë"
    486,  // "§"
    861,  // "ä"
    493,  // "¨"
    799,  // "Ú"
    706,  // "Ì"
    783,  // "Ø"
    719,  // "Ï"
    571,  // "»"
    942,  // "õ"
    913,  // "ï"
    688,  // "É"
    682,  // "È"
    978,  // "ü"
    932,  // "ó"
    973,  // "û"
    778,  // "×"
    602,  // "¾"
    893,  // "ê"
    807,  // "Û"
    545,  // "µ"
    846,  // "á"
    667,  // "Æ"
    543,  // "´"
    878,  // "ç"
    927,  // "ò"
    822,  // "Ý"
    710,  // "Í"
    815,  // "Ü"
    983,  // "ý"
    516,  // "\u00AD" Soft Hyphen
    856,  // "ã"
    739,  // "Ò"
    504,  // "ª"
    675,  // "Ç"
    659,  // "Å"
    952,  // "÷"
    465,  // "£"
    755,  // "Ô"
    866,  // "å"
    830,  // "Þ"
    540,  // "³"
    513,  // "¬"
    937,  // "ô"
    644,  // "Ã"
    552,  // "¶"
    455,  // "\u00A0" No-Break Space
    560,  // "·"
    564,  // "¹"
    562,  // "¸"
    495,  // "©"
    508,  // "«"
    724,  // "Ð"
    636,  // "Â"
    567,  // "º"
    909,  // "î"
    483,  // "¦"
    457,  // "¡"
    615,  // "¿"
    652,  // "Ä"
    968,  // "ú"
    576,  // "¼"
    527,  // "¯"
    991,  // "þ"
    835,  // "ß"
    628,  // "Á"
    888,  // "é"
    791,  // "Ù"
    883,  // "è"
    700,  // "Ë"
    731,  // "Ñ"
    537,  // "²"
    903,  // "ì"
    851,  // "â"
    917,  // "ð"
    589,  // "½"
    532,  // "±"
    963,  // "ù"
    947,  // "ö"
    620,  // "À"
    747,  // "Ó"
    872,  // "æ"
    922,  // "ñ"
    997,  // "ÿ"
    694,  // "Ê"
];

/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_latin_extended_a(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_LATIN_EXTENDED_A.binary_search(&key) {
        Ok(index) => return Some((OFFSET_LATIN_EXTENDED_A[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_LATIN_EXTENDED_A
const HASH_LATIN_EXTENDED_A: [u32; 2] = [
    0x1A01594C,  // "Œ"
    0x8C60DA30,  // "œ"
];

/// Lookup table of blit pattern offsets; sort matches HASH_LATIN_EXTENDED_A
const OFFSET_LATIN_EXTENDED_A: [usize; 2] = [
    1004, // "Œ"
    1012, // "œ"
];

/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_general_punctuation(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_GENERAL_PUNCTUATION.binary_search(&key) {
        Ok(index) => return Some((OFFSET_GENERAL_PUNCTUATION[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_GENERAL_PUNCTUATION
const HASH_GENERAL_PUNCTUATION: [u32; 11] = [
    0x0D0042B1,  // "•"
    0x18E68A7D,  // "‚"
    0x372ED469,  // "“"
    0x3A3C25C2,  // "”"
    0x80E4E277,  // "‡"
    0x885E576B,  // "‟"
    0x8B80C01D,  // "‛"
    0x8F0BE1DB,  // "„"
    0x8F56D335,  // "†"
    0xA6C64F29,  // "‘"
    0xE29813B0,  // "’"
];

/// Lookup table of blit pattern offsets; sort matches HASH_GENERAL_PUNCTUATION
const OFFSET_GENERAL_PUNCTUATION: [usize; 11] = [
    1046, // "•"
    1022, // "‚"
    1026, // "“"
    1029, // "”"
    1041, // "‡"
    1035, // "‟"
    1024, // "‛"
    1032, // "„"
    1038, // "†"
    1018, // "‘"
    1020, // "’"
];

/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_currency_symbols(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_CURRENCY_SYMBOLS.binary_search(&key) {
        Ok(index) => return Some((OFFSET_CURRENCY_SYMBOLS[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_CURRENCY_SYMBOLS
const HASH_CURRENCY_SYMBOLS: [u32; 1] = [
    0x1ACA36BB,  // "€"
];

/// Lookup table of blit pattern offsets; sort matches HASH_CURRENCY_SYMBOLS
const OFFSET_CURRENCY_SYMBOLS: [usize; 1] = [
    1051, // "€"
];

/// Use binary search on table of grapheme cluster hashes to find blit pattern for grapheme cluster.
/// Only attempt to match grapheme clusters of length limit codepoints.
fn find_specials(cluster: &str, limit: u32) -> Option<(usize, usize)> {
    let (key, bytes_hashed) = super::murmur3(cluster, M3_SEED, limit);
    match HASH_SPECIALS.binary_search(&key) {
        Ok(index) => return Some((OFFSET_SPECIALS[index], bytes_hashed)),
        _ => None,
    }
}

/// Index of murmur3(grapheme cluster); sort matches OFFSET_SPECIALS
const HASH_SPECIALS: [u32; 1] = [
    0x58A5DA35,  // "�"
];

/// Lookup table of blit pattern offsets; sort matches HASH_SPECIALS
const OFFSET_SPECIALS: [usize; 1] = [
    1058, // "�"
];

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
pub const DATA: [u32; 1071] = [
    // [0]: 20 " "
    0x0004020b, 0x00000000,
    // [2]: 21 "!"
    0x00020e06, 0xfffff0f0,
    // [4]: 22 "\""
    0x00060406, 0xcf3cf300,
    // [6]: 23 "#"
    0x000a0a04, 0xcc330fff, 0xff330ccf, 0xffff0cc3, 0x30000000,
    // [11]: 24 "$"
    0x000a1204, 0x0c0303f0, 0xfcccf330, 0xcc333f0f, 0xccc330cc, 0xf333f0fc, 0x0c030000,
    // [18]: 25 "%"
    0x00100e06, 0xfffcfffc, 0x30c330c3, 0x0cc30cc3, 0x3f3c3f3c, 0xc3c0c3c0, 0xc330c330, 0x3c0c3c0c,
    // [26]: 26 "&"
    0x000e1004, 0x03c00f00, 0xc3030c03, 0x300cc00c, 0x00303330, 0xccc0c0c3, 0x03330ccc, 0x3c0f303c,
    // [34]: 27 "'"
    0x00020406, 0xff000000,
    // [36]: 28 "("
    0x00061204, 0xc3030c0c, 0x30c30c30, 0xc30c330c, 0xc3000000,
    // [41]: 29 ")"
    0x00061204, 0x0c330cc3, 0x0c30c30c, 0x30c3030c, 0x0c300000,
    // [46]: 2A "*"
    0x000a0a06, 0x0c030ccf, 0x333f0fc3, 0x30ccc0f0, 0x30000000,
    // [51]: 2B "+"
    0x000a0a08, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x00000000,
    // [56]: 2C ","
    0x00040612, 0xcccc3300,
    // [58]: 2D "-"
    0x0008020c, 0xffff0000,
    // [60]: 2E "."
    0x00020212, 0xf0000000,
    // [62]: 2F "/"
    0x00081004, 0xc0c0c0c0, 0x30303030, 0x0c0c0c0c, 0x03030303,
    // [67]: 30 "0"
    0x000a0e06, 0x3f0fcc0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f033f, 0x0fc00000,
    // [73]: 31 "1"
    0x00040e06, 0xccffcccc, 0xcccccc00,
    // [76]: 32 "2"
    0x000a0e06, 0x3f0fcc0f, 0x03c03003, 0x00c00c03, 0x00300cff, 0xfff00000,
    // [82]: 33 "3"
    0x000a0e06, 0xfffff300, 0xc00c0303, 0xf0fcc030, 0x0c0f033f, 0x0fc00000,
    // [88]: 34 "4"
    0x000a0e06, 0x300c03c0, 0xf0330cc3, 0x0cc3ffff, 0xf300c030, 0x0c000000,
    // [94]: 35 "5"
    0x000a0e06, 0xfffff00c, 0x033fcffc, 0x0300c030, 0x0c0f033f, 0x0fc00000,
    // [100]: 36 "6"
    0x000a0e06, 0x3c0f0030, 0x0c00c033, 0xfcffc0f0, 0x3c0f033f, 0x0fc00000,
    // [106]: 37 "7"
    0x000a0e06, 0xfffffc03, 0x00300c03, 0x00c00c03, 0x00c0300c, 0x03000000,
    // [112]: 38 "8"
    0x000a0e06, 0x3f0fcc0f, 0x03c0f033, 0xf0fcc0f0, 0x3c0f033f, 0x0fc00000,
    // [118]: 39 "9"
    0x000a0e06, 0x3f0fcc0f, 0x03c0f03f, 0xf3fcc030, 0x0300c00f, 0x03c00000,
    // [124]: 3A ":"
    0x00020a0a, 0xf000f000,
    // [126]: 3B ";"
    0x00040e0a, 0xcc000000, 0xcccc3300,
    // [129]: 3C "<"
    0x00060a08, 0xc3030c0c, 0x330cc300,
    // [132]: 3D "="
    0x000a060a, 0xfffff000, 0x00fffff0,
    // [135]: 3E ">"
    0x00060a08, 0x0c330cc3, 0x030c0c30,
    // [138]: 3F "?"
    0x00080e06, 0x3c3cc3c3, 0xc0c03030, 0x0c0c0000, 0x0c0c0000,
    // [143]: 40 "@"
    0x000e1006, 0x0fc03f03, 0x030c0ccf, 0x0f3c3ccc, 0xf333cccf, 0x3333f0cf, 0xc3003000, 0xc0fc03f0,
    // [151]: 41 "A"
    0x000a0e06, 0x0c0300c0, 0x30330cc3, 0x30ccffff, 0xfc0f03c0, 0xf0300000,
    // [157]: 42 "B"
    0x000a0e06, 0x3fcffc0f, 0x03c0f033, 0xfcffc0f0, 0x3c0f033f, 0xcff00000,
    // [163]: 43 "C"
    0x000a0e06, 0x3f0fcc0f, 0x0300c030, 0x0c0300c0, 0x3c0f033f, 0x0fc00000,
    // [169]: 44 "D"
    0x000a0e06, 0x0fc3f30c, 0xc3c0f03c, 0x0f03c0f0, 0x330cc30f, 0xc3f00000,
    // [175]: 45 "E"
    0x00080e06, 0xffff0303, 0x03033f3f, 0x03030303, 0xffff0000,
    // [180]: 46 "F"
    0x00080e06, 0xffff0303, 0x03033f3f, 0x03030303, 0x03030000,
    // [185]: 47 "G"
    0x000a0e06, 0x3f0fcc0f, 0x0300c03f, 0x0fc3c0f0, 0x3c0f033f, 0x0fc00000,
    // [191]: 48 "H"
    0x000a0e06, 0xc0f03c0f, 0x03c0f03f, 0xffffc0f0, 0x3c0f03c0, 0xf0300000,
    // [197]: 49 "I"
    0x00020e06, 0xfffffff0,
    // [199]: 4A "J"
    0x000a0e06, 0xc0300c03, 0x00c0300c, 0x0300c0f0, 0x3c0f033f, 0x0fc00000,
    // [205]: 4B "K"
    0x000a0e06, 0xc0f0330c, 0xc30cc330, 0x3c0f0cc3, 0x330cc3c0, 0xf0300000,
    // [211]: 4C "L"
    0x00080e06, 0x03030303, 0x03030303, 0x03030303, 0xffff0000,
    // [216]: 4D "M"
    0x000e0e06, 0xc00f003f, 0x03fc0fcc, 0xcf333c30, 0xf0c3c00f, 0x003c00f0, 0x03c00f00, 0x30000000,
    // [224]: 4E "N"
    0x000a0e06, 0xc3f0fc3f, 0x0fccf33c, 0xcf33f0fc, 0x3f0fc3c0, 0xf0300000,
    // [230]: 4F "O"
    0x000a0e06, 0x3f0fcc0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f033f, 0x0fc00000,
    // [236]: 50 "P"
    0x000a0e06, 0x3fcffc0f, 0x03c0f033, 0xfcff00c0, 0x300c0300, 0xc0300000,
    // [242]: 51 "Q"
    0x000a1006, 0x3f0fcc0f, 0x03c0f03c, 0x0f03c0f0, 0x3ccf333f, 0x0fc300c0,
    // [248]: 52 "R"
    0x000a0e06, 0x3fcffc0f, 0x03c0f033, 0xfcff0cc3, 0x330cc3c0, 0xf0300000,
    // [254]: 53 "S"
    0x000a0e06, 0x3f0fcc0f, 0x0300c033, 0xf0fcc030, 0x0c0f033f, 0x0fc00000,
    // [260]: 54 "T"
    0x000a0e06, 0xfffff0c0, 0x300c0300, 0xc0300c03, 0x00c0300c, 0x03000000,
    // [266]: 55 "U"
    0x000a0e06, 0xc0f03c0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f033f, 0x0fc00000,
    // [272]: 56 "V"
    0x000a0e06, 0xc0f03c0f, 0x03c0f033, 0x30cc330c, 0xc0c0300c, 0x03000000,
    // [278]: 57 "W"
    0x000e0e06, 0xc00f003c, 0x00f00333, 0x30ccc333, 0x0ccc0cc0, 0x3300cc03, 0x300cc033, 0x00000000,
    // [286]: 58 "X"
    0x000a0e06, 0xc0f03c0f, 0x03330cc0, 0xc030330c, 0xcc0f03c0, 0xf0300000,
    // [292]: 59 "Y"
    0x000a0e06, 0xc0f03c0f, 0x03330cc0, 0xc0300c03, 0x00c0300c, 0x03000000,
    // [298]: 5A "Z"
    0x00080e06, 0xffffc0c0, 0x30300c0c, 0x03030303, 0xffff0000,
    // [303]: 5B "["
    0x00041204, 0xff333333, 0x33333333, 0xff000000,
    // [307]: 5C "\\"
    0x00081004, 0x03030303, 0x0c0c0c0c, 0x30303030, 0xc0c0c0c0,
    // [312]: 5D "]"
    0x00041204, 0xffcccccc, 0xcccccccc, 0xff000000,
    // [316]: 5E "^"
    0x00060406, 0x30ccf300,
    // [318]: 5F "_"
    0x000c0212, 0xffffff00,
    // [320]: 60 "`"
    0x00040406, 0x33cc0000,
    // [322]: 61 "a"
    0x00080a0a, 0x3c3cc0c0, 0xfcfcc3c3, 0xfcfc0000,
    // [326]: 62 "b"
    0x00080e06, 0x03030303, 0x3f3fc3c3, 0xc3c3c3c3, 0x3f3f0000,
    // [331]: 63 "c"
    0x00080a0a, 0x3c3cc3c3, 0x0303c3c3, 0x3c3c0000,
    // [335]: 64 "d"
    0x00080e06, 0xc0c0c0c0, 0xfcfcc3c3, 0xc3c3c3c3, 0xfcfc0000,
    // [340]: 65 "e"
    0x00080a0a, 0x3c3cc3c3, 0xffff0303, 0x3c3c0000,
    // [344]: 66 "f"
    0x00080e06, 0xf0f00c0c, 0x3f3f0c0c, 0x0c0c0c0c, 0x0c0c0000,
    // [349]: 67 "g"
    0x00080e0a, 0xfcfcc3c3, 0xc3c3c3c3, 0xfcfcc0c0, 0x3c3c0000,
    // [354]: 68 "h"
    0x00080e06, 0x03030303, 0x3f3fc3c3, 0xc3c3c3c3, 0xc3c30000,
    // [359]: 69 "i"
    0x00020e06, 0xf0fffff0,
    // [361]: 6A "j"
    0x00061206, 0xc30000c3, 0x0c30c30c, 0x30c30c30, 0x3cf00000,
    // [366]: 6B "k"
    0x00080e06, 0x03030303, 0xc3c33333, 0x0f0f3333, 0xc3c30000,
    // [371]: 6C "l"
    0x00020e06, 0xfffffff0,
    // [373]: 6D "m"
    0x000e0a0a, 0x3cfcf3fc, 0x30f0c3c3, 0x0f0c3c30, 0xf0c3c30f, 0x0c300000,
    // [379]: 6E "n"
    0x00080a0a, 0x3f3fc3c3, 0xc3c3c3c3, 0xc3c30000,
    // [383]: 6F "o"
    0x00080a0a, 0x3c3cc3c3, 0xc3c3c3c3, 0x3c3c0000,
    // [387]: 70 "p"
    0x00080e0a, 0x3f3fc3c3, 0xc3c3c3c3, 0x3f3f0303, 0x03030000,
    // [392]: 71 "q"
    0x00080e0a, 0xfcfcc3c3, 0xc3c3c3c3, 0xfcfcc0c0, 0xc0c00000,
    // [397]: 72 "r"
    0x00080a0a, 0xf3f30f0f, 0x03030303, 0x03030000,
    // [401]: 73 "s"
    0x00080a0a, 0xfcfc0303, 0x3c3cc0c0, 0x3f3f0000,
    // [405]: 74 "t"
    0x00060e06, 0x30c30cff, 0xf30c30c3, 0x0cc30000,
    // [409]: 75 "u"
    0x00080a0a, 0xc3c3c3c3, 0xc3c3c3c3, 0xfcfc0000,
    // [413]: 76 "v"
    0x000a0a0a, 0xc0f03330, 0xcc330cc0, 0xc0300c03, 0x00000000,
    // [418]: 77 "w"
    0x000e0a0a, 0xc00f0033, 0x330ccc33, 0x30ccc0cc, 0x03300cc0, 0x33000000,
    // [424]: 78 "x"
    0x000a0a0a, 0xc0f03330, 0xcc0c0303, 0x30ccc0f0, 0x30000000,
    // [429]: 79 "y"
    0x000a0e0a, 0xc0f03c0f, 0x03330cc3, 0x30cc0c03, 0x00c03003, 0xc0f00000,
    // [435]: 7A "z"
    0x00080a0a, 0xffff3030, 0x0c0c0303, 0xffff0000,
    // [439]: 7B "{"
    0x00061204, 0xc3030c30, 0xc30c0c33, 0x0c30c30c, 0xc3000000,
    // [444]: 7C "|"
    0x00021204, 0xffffffff, 0xf0000000,
    // [447]: 7D "}"
    0x00061204, 0x0c330c30, 0xc30cc303, 0x0c30c30c, 0x0c300000,
    // [452]: 7E "~"
    0x000a0406, 0xcf33c3cc, 0xf3000000,
    // [455]: A0 "\u00A0" No-Break Space
    0x0004020b, 0x00000000,
    // [457]: A1 "¡"
    0x00020e06, 0xf0fffff0,
    // [459]: A2 "¢"
    0x000a0e06, 0x0c0303f0, 0xfcccf330, 0xcc33ccf3, 0x33f0fc0c, 0x03000000,
    // [465]: A3 "£"
    0x000a0e06, 0x3c0f0030, 0x0c0300c0, 0xfc3f0300, 0xcc330c3f, 0xcff00000,
    // [471]: A4 "¤"
    0x000a0e06, 0xc0f033f0, 0xfcc0f03c, 0x0f03c0f0, 0x33f0fcc0, 0xf0300000,
    // [477]: A5 "¥"
    0x000a0e06, 0xc0f03330, 0xccfffff0, 0xc030ffff, 0xf0c0300c, 0x03000000,
    // [483]: A6 "¦"
    0x00021204, 0xffff0fff, 0xf0000000,
    // [486]: A7 "§"
    0x000a1204, 0x3f0fcc0f, 0x0300c033, 0xf0fcc0f0, 0x33f0fcc0, 0x300c0f03, 0x3f0fc000,
    // [493]: A8 "¨"
    0x00060206, 0xcf300000,
    // [495]: A9 "©"
    0x00101004, 0x0ff00ff0, 0x300c300c, 0xc3c3c3c3, 0xc033c033, 0xc033c033, 0xc3c3c3c3, 0x300c300c,
    0x0ff00ff0,
    // [504]: AA "ª"
    0x00060e04, 0x30ccf3f3, 0xccf3f3c0, 0x00fff000,
    // [508]: AB "«"
    0x000c0a0a, 0xc30c3030, 0xc30c0c30, 0xc330c30c, 0xc30c3000,
    // [513]: AC "¬"
    0x0008060c, 0xffffc0c0, 0xc0c00000,
    // [516]: AD "\u00AD" Soft Hyphen
    0x0008020c, 0xffff0000,
    // [518]: AE "®"
    0x00101004, 0x0ff00ff0, 0x300c300c, 0xc3f3c3f3, 0xcc33cc33, 0xc3f3c3f3, 0xcc33cc33, 0x300c300c,
    0x0ff00ff0,
    // [527]: AF "¯"
    0x00060206, 0xfff00000,
    // [529]: B0 "°"
    0x00080804, 0x3c3cc3c3, 0xc3c33c3c,
    // [532]: B1 "±"
    0x000a0c08, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x0fffff00,
    // [537]: B2 "²"
    0x00060a02, 0xfffc30ff, 0xf0c3fff0,
    // [540]: B3 "³"
    0x00060a02, 0xfffc30ff, 0xfc30fff0,
    // [543]: B4 "´"
    0x00040406, 0xcc330000,
    // [545]: B5 "µ"
    0x000c0e0a, 0xc30c30c3, 0x0c3030c3, 0x0c30c30c, 0xcfccfc00, 0x30030030, 0x03000000,
    // [552]: B6 "¶"
    0x000c1206, 0xffcffc33, 0xf33f33f3, 0x3f33c33c, 0x33033033, 0x03303303, 0x30330330, 0x33033000,
    // [560]: B7 "·"
    0x0002020e, 0xf0000000,
    // [562]: B8 "¸"
    0x00040414, 0xcc330000,
    // [564]: B9 "¹"
    0x00060a02, 0x30c3cf30, 0xc30cfff0,
    // [567]: BA "º"
    0x00080c06, 0x3c3cc3c3, 0xc3c33c3c, 0x0000ffff,
    // [571]: BB "»"
    0x000c0a0a, 0x0c30c330, 0xc30cc30c, 0x3030c30c, 0x0c30c300,
    // [576]: BC "¼"
    0x00121400, 0x0c030300, 0xc0c03c30, 0x0f030300, 0xc0c03030, 0x0c0c00cf, 0xc033fccc, 0x033300cc,
    0x30330c0f, 0xc303f0c0, 0xc00c3003, 0x0c00c300, 0x30000000,
    // [589]: BD "½"
    0x00121400, 0x0c030300, 0xc0c03c30, 0x0f030300, 0xc0c03030, 0x0c0c00cf, 0xc033ffcc, 0x03f300c0,
    0x30300c0f, 0xc303f0c0, 0x0c0c0303, 0x0fc0c3f0, 0x30000000,
    // [602]: BE "¾"
    0x00121400, 0x0c0fc303, 0xf0c0c030, 0x30030fc0, 0xc3f030c0, 0x0c3000cf, 0xc033fccc, 0x033300cc,
    0x30330c0f, 0xc303f0c0, 0xc00c3003, 0x0c00c300, 0x30000000,
    // [615]: BF "¿"
    0x00080e06, 0x30300000, 0x30300c0c, 0x0303c3c3, 0x3c3c0000,
    // [620]: C0 "À"
    0x000a1400, 0x0300c0c0, 0x30000000, 0xc0300c03, 0x0330cc33, 0x0ccfffff, 0xc0f03c0f, 0x03000000,
    // [628]: C1 "Á"
    0x000a1400, 0x300c00c0, 0x30000000, 0xc0300c03, 0x0330cc33, 0x0ccfffff, 0xc0f03c0f, 0x03000000,
    // [636]: C2 "Â"
    0x000a1400, 0x0c030330, 0xcc000000, 0xc0300c03, 0x0330cc33, 0x0ccfffff, 0xc0f03c0f, 0x03000000,
    // [644]: C3 "Ã"
    0x000a1400, 0xcf33c3cc, 0xf3000000, 0xc0300c03, 0x0330cc33, 0x0ccfffff, 0xc0f03c0f, 0x03000000,
    // [652]: C4 "Ä"
    0x000a1202, 0x330cc000, 0x000c0300, 0xc030330c, 0xc330ccff, 0xfffc0f03, 0xc0f03000,
    // [659]: C5 "Å"
    0x000a1400, 0x3f0fcc0f, 0x033f0fc0, 0xc0300c03, 0x0330cc33, 0x0ccfffff, 0xc0f03c0f, 0x03000000,
    // [667]: C6 "Æ"
    0x00100e06, 0xffc0ffc0, 0x03300330, 0x03300330, 0x3ffc3ffc, 0x030c030c, 0x03030303, 0xff03ff03,
    // [675]: C7 "Ç"
    0x000a1206, 0x3f0fcc0f, 0x0300c030, 0x0c0300c0, 0x3c0f033f, 0x0fc0c030, 0x0300c000,
    // [682]: C8 "È"
    0x00081400, 0x0c0c3030, 0x0000ffff, 0x03030303, 0x3f3f0303, 0x0303ffff,
    // [688]: C9 "É"
    0x00081400, 0x30300c0c, 0x0000ffff, 0x03030303, 0x3f3f0303, 0x0303ffff,
    // [694]: CA "Ê"
    0x00081400, 0x0c0c3333, 0x0000ffff, 0x03030303, 0x3f3f0303, 0x0303ffff,
    // [700]: CB "Ë"
    0x00081202, 0x33330000, 0xffff0303, 0x03033f3f, 0x03030303, 0xffff0000,
    // [706]: CC "Ì"
    0x00041400, 0x33cc00cc, 0xcccccccc, 0xcccc0000,
    // [710]: CD "Í"
    0x00041400, 0xcc330033, 0x33333333, 0x33330000,
    // [714]: CE "Î"
    0x00061400, 0x30ccf300, 0x030c30c3, 0x0c30c30c, 0x30c30c00,
    // [719]: CF "Ï"
    0x00061202, 0xcf300030, 0xc30c30c3, 0x0c30c30c, 0x30c00000,
    // [724]: D0 "Ð"
    0x000c0e06, 0x0fc0fc30, 0xc30cc0cc, 0x0cc3fc3f, 0xc0cc0c30, 0xc30c0fc0, 0xfc000000,
    // [731]: D1 "Ñ"
    0x000a1400, 0xcf33c3cc, 0xf300000c, 0x3f0fc3f0, 0xfccf33cc, 0xf33f0fc3, 0xf0fc3c0f, 0x03000000,
    // [739]: D2 "Ò"
    0x000a1400, 0x0300c0c0, 0x30000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [747]: D3 "Ó"
    0x000a1400, 0x300c00c0, 0x30000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [755]: D4 "Ô"
    0x000a1400, 0x0c030330, 0xcc000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [763]: D5 "Õ"
    0x000a1400, 0xcf33c3cc, 0xf3000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [771]: D6 "Ö"
    0x000a1202, 0x330cc000, 0x003f0fcc, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0x3f0fc000,
    // [778]: D7 "×"
    0x000a0a0a, 0xc0f03330, 0xcc0c0303, 0x30ccc0f0, 0x30000000,
    // [783]: D8 "Ø"
    0x000e0e06, 0xcfc33f03, 0x030c0c3c, 0x30f0c333, 0x0ccc30f0, 0xc3c3030c, 0x0c0fcc3f, 0x30000000,
    // [791]: D9 "Ù"
    0x000a1400, 0x0300c0c0, 0x3000000c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [799]: DA "Ú"
    0x000a1400, 0x300c00c0, 0x3000000c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [807]: DB "Û"
    0x000a1400, 0x0c030330, 0xcc00000c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [815]: DC "Ü"
    0x000a1202, 0x330cc000, 0x00c0f03c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0x3f0fc000,
    // [822]: DD "Ý"
    0x000a1400, 0x300c00c0, 0x3000000c, 0x0f03c0f0, 0x3330cc0c, 0x0300c030, 0x0c0300c0, 0x30000000,
    // [830]: DE "Þ"
    0x00080e06, 0x0303033f, 0x3fc3c3c3, 0xc33f3f03, 0x03030000,
    // [835]: DF "ß"
    0x000a0e06, 0x0f03c30c, 0xc30cc330, 0xcc3330cc, 0x3c0f033c, 0xcf300000,
    // [841]: E0 "à"
    0x00081004, 0x0c0c3030, 0x00003c3c, 0xc3c3fcfc, 0xc3c3fcfc,
    // [846]: E1 "á"
    0x00081004, 0x30300c0c, 0x00003c3c, 0xc3c3fcfc, 0xc3c3fcfc,
    // [851]: E2 "â"
    0x00081004, 0x3030cccc, 0x00003c3c, 0xc3c3fcfc, 0xc3c3fcfc,
    // [856]: E3 "ã"
    0x00081004, 0xcccc3333, 0x00003c3c, 0xc3c3fcfc, 0xc3c3fcfc,
    // [861]: E4 "ä"
    0x00080e06, 0xcccc0000, 0x3c3cc3c3, 0xfcfcc3c3, 0xfcfc0000,
    // [866]: E5 "å"
    0x00081202, 0x3c3cc3c3, 0x3c3c0000, 0x3c3cc3c3, 0xfcfcc3c3, 0xfcfc0000,
    // [872]: E6 "æ"
    0x000e0a0a, 0x3cf0f3cc, 0x3030c0ff, 0xf3ffc030, 0xc0c33cf0, 0xf3c00000,
    // [878]: E7 "ç"
    0x00080e0a, 0x3c3cc3c3, 0x0303c3c3, 0x3c3c3030, 0x0c0c0000,
    // [883]: E8 "è"
    0x00081004, 0x0c0c3030, 0x00003c3c, 0xc3c3ffff, 0x03033c3c,
    // [888]: E9 "é"
    0x00081004, 0x30300c0c, 0x00003c3c, 0xc3c3ffff, 0x0303fcfc,
    // [893]: EA "ê"
    0x00081004, 0x0c0c3333, 0x00003c3c, 0xc3c3ffff, 0x03033c3c,
    // [898]: EB "ë"
    0x00080e06, 0xc3c30000, 0x3c3cc3c3, 0xffff0303, 0xfcfc0000,
    // [903]: EC "ì"
    0x00041004, 0x33cc00cc, 0xcccccccc,
    // [906]: ED "í"
    0x00041004, 0xcc330033, 0x33333333,
    // [909]: EE "î"
    0x00061004, 0x30ccf300, 0x030c30c3, 0x0c30c30c,
    // [913]: EF "ï"
    0x00060e06, 0xcf300030, 0xc30c30c3, 0x0c30c000,
    // [917]: F0 "ð"
    0x00081004, 0x03333c0f, 0x3330cccc, 0xc3c3c3c3, 0xc3c33c3c,
    // [922]: F1 "ñ"
    0x00081004, 0xcccc3333, 0x00003f3f, 0xc3c3c3c3, 0xc3c3c3c3,
    // [927]: F2 "ò"
    0x00081004, 0x0c0c3030, 0x00003c3c, 0xc3c3c3c3, 0xc3c33c3c,
    // [932]: F3 "ó"
    0x00081004, 0x30300c0c, 0x00003c3c, 0xc3c3c3c3, 0xc3c33c3c,
    // [937]: F4 "ô"
    0x00081004, 0x0c0c3333, 0x00003c3c, 0xc3c3c3c3, 0xc3c33c3c,
    // [942]: F5 "õ"
    0x00081004, 0xcccc3333, 0x00003c3c, 0xc3c3c3c3, 0xc3c33c3c,
    // [947]: F6 "ö"
    0x00080e06, 0xc3c30000, 0x3c3cc3c3, 0xc3c3c3c3, 0x3c3c0000,
    // [952]: F7 "÷"
    0x000a0a08, 0x0c030000, 0x00fffff0, 0x00000c03, 0x00000000,
    // [957]: F8 "ø"
    0x000c0c08, 0xcf0cf030, 0xc30c3cc3, 0xcc33c33c, 0x30c30c0f, 0x30f30000,
    // [963]: F9 "ù"
    0x00081004, 0x0c0c3030, 0x0000c3c3, 0xc3c3c3c3, 0xc3c3fcfc,
    // [968]: FA "ú"
    0x00081004, 0x30300c0c, 0x0000c3c3, 0xc3c3c3c3, 0xc3c3fcfc,
    // [973]: FB "û"
    0x00081004, 0x0c0c3333, 0x0000c3c3, 0xc3c3c3c3, 0xc3c3fcfc,
    // [978]: FC "ü"
    0x00080e06, 0xc3c30000, 0xc3c3c3c3, 0xc3c3c3c3, 0xfcfc0000,
    // [983]: FD "ý"
    0x000a1404, 0x300c00c0, 0x3000000c, 0x0f03c0f0, 0x3330cc33, 0x0cc0c030, 0x0c03003c, 0x0f000000,
    // [991]: FE "þ"
    0x00081206, 0x03030303, 0x3f3fc3c3, 0xc3c3c3c3, 0x3f3f0303, 0x03030000,
    // [997]: FF "ÿ"
    0x000a1206, 0x330cc000, 0x00c0f03c, 0x0f03330c, 0xc330cc0c, 0x0300c030, 0x03c0f000,
    // [1004]: 152 "Œ"
    0x00100e06, 0xfffcfffc, 0x03030303, 0x03030303, 0x3f033f03, 0x03030303, 0x03030303, 0xfffcfffc,
    // [1012]: 153 "œ"
    0x000e0a0a, 0x3cf0f3cc, 0x30f0c3ff, 0x0ffc3030, 0xc0c33ff0, 0xffc00000,
    // [1018]: 2018 "‘"
    0x00040604, 0xcc333300,
    // [1020]: 2019 "’"
    0x00040604, 0xcccc3300,
    // [1022]: 201A "‚"
    0x00040612, 0xcccc3300,
    // [1024]: 201B "‛"
    0x00040604, 0x3333cc00,
    // [1026]: 201C "“"
    0x00080604, 0xcccc3333, 0x33330000,
    // [1029]: 201D "”"
    0x00080604, 0xcccccccc, 0x33330000,
    // [1032]: 201E "„"
    0x00080612, 0xcccccccc, 0x33330000,
    // [1035]: 201F "‟"
    0x00080604, 0x33333333, 0xcccc0000,
    // [1038]: 2020 "†"
    0x00060a04, 0x30cfff30, 0xc30c30c0,
    // [1041]: 2021 "‡"
    0x00061206, 0x30c30cff, 0xf30c30c3, 0x0cfff30c, 0x30c00000,
    // [1046]: 2022 "•"
    0x000a0a08, 0x3f0fcfff, 0xffffffff, 0xffff3f0f, 0xc0000000,
    // [1051]: 20AC "€"
    0x000c0e04, 0x3f03f0c0, 0xcc0c0ff0, 0xff00c00c, 0x0ff0ffc0, 0xcc0c3f03, 0xf0000000,
    // [1058]: FFFD "�"
    0x00121402, 0x00c00030, 0x003f000f, 0xc00f3c03, 0xcf03ccf0, 0xf33cfcff, 0xff3ffff3, 0xfffcff3f,
    0xff0fffc0, 0xf3c03cf0, 0x03f000fc, 0x000c0003, 0x00000000,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // If this fails, there's probably a hash collision, so change the seed.
    fn test_hashes_unique_and_sorted() {
        for i in 0..HASH_BASIC_LATIN.len()-1 {
            assert!(HASH_BASIC_LATIN[i] < HASH_BASIC_LATIN[i+1]);
        }
        for i in 0..HASH_LATIN_1_SUPPLEMENT.len()-1 {
            assert!(HASH_LATIN_1_SUPPLEMENT[i] < HASH_LATIN_1_SUPPLEMENT[i+1]);
        }
        for i in 0..HASH_LATIN_EXTENDED_A.len()-1 {
            assert!(HASH_LATIN_EXTENDED_A[i] < HASH_LATIN_EXTENDED_A[i+1]);
        }
        for i in 0..HASH_GENERAL_PUNCTUATION.len()-1 {
            assert!(HASH_GENERAL_PUNCTUATION[i] < HASH_GENERAL_PUNCTUATION[i+1]);
        }
        for i in 0..HASH_CURRENCY_SYMBOLS.len()-1 {
            assert!(HASH_CURRENCY_SYMBOLS[i] < HASH_CURRENCY_SYMBOLS[i+1]);
        }
        for i in 0..HASH_SPECIALS.len()-1 {
            assert!(HASH_SPECIALS[i] < HASH_SPECIALS[i+1]);
        }
    }
}
