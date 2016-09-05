// AUTOGENERATED FROM index-iso-8859-3.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-iso-8859-3.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: af8f1e12df79b768322b5e83613698cdc619438270a2fc359554331c805054a3
// Date: 2016-01-20

#[allow(dead_code)] const X: u16 = 0xffff;

const FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 294, 728, 163, 164, X, 292, 167, 168, 304, 350, 286, 308,
    173, X, 379, 176, 295, 178, 179, 180, 181, 293, 183, 184, 305, 351, 287,
    309, 189, X, 380, 192, 193, 194, X, 196, 266, 264, 199, 200, 201, 202, 203,
    204, 205, 206, 207, X, 209, 210, 211, 212, 288, 214, 215, 284, 217, 218,
    219, 220, 364, 348, 223, 224, 225, 226, X, 228, 267, 265, 231, 232, 233,
    234, 235, 236, 237, 238, 239, X, 241, 242, 243, 244, 289, 246, 247, 285,
    249, 250, 251, 252, 365, 349, 729,
]; // 128 entries

/// Returns the index code point for pointer `code` in this index.
#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as usize]
}

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 162, 255, 0, 0, 0, 0, 0, 0, 216, 248, 171, 187,
    213, 245, 0, 0, 166, 182, 161, 177, 169, 185, 0, 0, 172, 188, 0, 0, 241,
    242, 243, 244, 0, 246, 247, 192, 193, 194, 0, 196, 0, 0, 199, 176, 0, 178,
    179, 180, 181, 0, 183, 0, 249, 250, 251, 252, 0, 0, 0, 0, 221, 253, 0, 0,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 0, 0, 163, 164, 0, 0, 167, 168, 0, 0, 0, 0, 173, 0, 0, 0,
    175, 191, 0, 0, 0, 209, 210, 211, 212, 0, 214, 215, 200, 201, 202, 203,
    204, 205, 206, 207, 184, 0, 0, 0, 0, 189, 0, 0, 217, 218, 219, 220, 0, 0,
    223, 224, 225, 226, 0, 228, 0, 0, 231, 232, 233, 234, 235, 236, 237, 238,
    239, 198, 230, 197, 229, 0, 0, 0, 0, 222, 254, 170, 186,
]; // 184 entries

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 80, 88, 96, 104, 112,
    51, 141, 43, 133, 125, 148, 156, 164, 35, 59, 0, 172, 0, 12, 20, 0, 28, 0,
    0, 0, 0, 176, 0, 64, 0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 8,
]; // 92 entries

/// Returns the index pointer for code point `code` in this index.
#[inline]
#[cfg(not(feature = "no-optimized-legacy-encoding"))]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 3) as usize;
    let offset = if offset < 92 {BACKWARD_TABLE_UPPER[offset] as usize} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 7) as usize)]
}

/// Returns the index pointer for code point `code` in this index.
#[cfg(feature = "no-optimized-legacy-encoding")]
pub fn backward(code: u32) -> u8 {
    if code > 729 || ((0x400ff0u32 >> (code >> 5)) & 1) == 0 { return 0; }
    let code = code as u16;
    for i in 0..0x80 {
        if FORWARD_TABLE[i as usize] == code { return 0x80 + i; }
    }
    0
}

#[cfg(test)]
single_byte_tests! {
}
