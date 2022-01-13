//! Encoding and decoding of the (15, 11, 3) standard and (10, 6, 3) shortened Hamming
//! codes described by P25.
//!
//! Both codes can correct up to 1 error. These algorithms are sourced from *Coding Theory
//! and Cryptography: The Essentials*, Hankerson, Hoffman, et al, 2000.

use binfield_matrix::{matrix_mul, matrix_mul_systematic};
use num_traits::PrimInt;

/// Encoding and decoding of the (15, 11, 3) code.
pub mod standard {
    use super::*;

    /// Encode the given 11 bits of data into a 15-bit codeword.
    pub fn encode(data: u16) -> u16 {
        assert!(data >> 11 == 0);
        matrix_mul_systematic(data, GEN)
    }

    /// Try to decode the given 15-bit word to the nearest codeword, correcting up to 1
    /// error.
    ///
    /// If decoding was successful, return `Some((data, err))`, where `data` is the 11
    /// data bits and `err` is the number of corrected bits. Otherwise, return `None` to
    /// indicate an unrecoverable error.
    pub fn decode(word: u16) -> Option<(u16, usize)> {
        assert!(word >> 15 == 0);
        super::decode(word, PAR, LOCATIONS).map(|(w, n)| (w >> 4, n))
    }

    /// Generator matrix from the standard, without identity part.
    const GEN: &[u16] = &[0b11111110000, 0b11110001110, 0b11001101101, 0b10101011011];

    /// Parity-check matrix derived from generator using standard method.
    const PAR: &[u16] = &[
        0b111111100001000,
        0b111100011100100,
        0b110011011010010,
        0b101010110110001,
    ];

    /// Maps 4-bit syndrome values to bit error locations.
    const LOCATIONS: &[u16] = &[
        0,
        0b0000000000000001,
        0b0000000000000010,
        0b0000000000010000,
        0b0000000000000100,
        0b0000000000100000,
        0b0000000001000000,
        0b0000000010000000,
        0b0000000000001000,
        0b0000000100000000,
        0b0000001000000000,
        0b0000010000000000,
        0b0000100000000000,
        0b0001000000000000,
        0b0010000000000000,
        0b0100000000000000,
    ];
}

/// Encoding and decoding of the (10, 6, 3) code.
pub mod shortened {
    use super::*;

    /// Encode the given 6 data bits into a 10-bit codeword.
    pub fn encode(data: u8) -> u16 {
        assert!(data >> 6 == 0);
        matrix_mul_systematic(data, GEN)
    }

    /// Try to decode the given 10-bit word to the nearest codeword, correcting up to 1
    /// error.
    ///
    /// If decoding was successful, return `Some((data, err))`, where `data` is the 6
    /// data bits and `err` is the number of corrected bits. Otherwise, return `None` to
    /// indicate an unrecoverable error.
    pub fn decode(word: u16) -> Option<(u8, usize)> {
        assert!(word >> 10 == 0);
        super::decode(word, PAR, LOCATIONS).map(|(w, n)| ((w >> 4) as u8, n))
    }

    const GEN: &[u8] = &[0b111001, 0b110101, 0b101110, 0b011110];

    const PAR: &[u16] = &[0b1110011000, 0b1101010100, 0b1011100010, 0b0111100001];

    const LOCATIONS: &[u16] = &[
        0,
        0b0000000000000001,
        0b0000000000000010,
        0b0000000000100000,
        0b0000000000000100,
        0,
        0,
        0b0000000001000000,
        0b0000000000001000,
        0,
        0,
        0b0000000010000000,
        0b0000000000010000,
        0b0000000100000000,
        0b0000001000000000,
        0,
    ];
}

fn decode<T: PrimInt>(word: T, par: &[T], locs: &[T]) -> Option<(T, usize)> {
    let s: usize = matrix_mul(word, par);

    if s == 0 {
        return Some((word, 0));
    }

    locs.get(s).and_then(|&loc| {
        if loc == T::zero() {
            None
        } else {
            Some((word ^ loc, 1))
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_standard() {
        assert_eq!(standard::encode(0), 0);
        assert_eq!(standard::encode(0b11111111111), 0b11111111111_1111);

        for w in 0..1 << 11 {
            assert_eq!(standard::decode(standard::encode(w)), Some((w, 0)));
        }

        let w = standard::encode(0b10101010101);
        assert_eq!(w, 0b10101010101_0101);
        assert_eq!(standard::decode(w), Some((0b10101010101, 0)));

        for i in 0..15 {
            assert_eq!(standard::decode(w ^ 1 << i), Some((0b10101010101, 1)));
        }

        for (i, j) in (0..15).zip(0..15) {
            if i != j {
                // Two-bit errors are detected as one-bit errors with an incorrect bit
                // correction, so just check for the detection.
                let (_, n) = standard::decode(w ^ (1 << i) ^ (1 << j)).unwrap();
                assert_eq!(n, 1);
            }
        }
    }

    #[test]
    fn test_shortened() {
        assert_eq!(shortened::encode(0), 0);
        assert_eq!(shortened::encode(0b111111), 0b111111_0000);

        for w in 0..1 << 6 {
            assert_eq!(shortened::decode(shortened::encode(w)), Some((w, 0)));
        }

        let w = shortened::encode(0b101010);
        assert_eq!(w, 0b101010_0110);
        assert_eq!(shortened::decode(w), Some((0b101010, 0)));

        for i in 0..10 {
            assert_eq!(shortened::decode(w ^ 1 << i), Some((0b101010, 1)));
        }

        for (i, j) in (0..10).zip(0..10) {
            if i != j {
                // Two-bit errors are detected as one-bit errors with an incorrect bit
                // correction, so just check for the detection.
                let (_, n) = shortened::decode(w ^ (1 << i) ^ (1 << j)).unwrap();
                assert_eq!(n, 1);
            }
        }
    }
}
