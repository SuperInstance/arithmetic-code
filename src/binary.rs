//! Binary arithmetic coding.
//!
//! Specialized arithmetic coder for binary (0/1) symbol streams.

/// Binary arithmetic coder for compressing boolean sequences.
#[derive(Debug)]
pub struct BinaryArithmeticCoder;

impl BinaryArithmeticCoder {
    /// Encode a sequence of booleans into packed bytes.
    pub fn encode(bits: &[bool]) -> Vec<u8> {
        if bits.is_empty() {
            return vec![0, 0];
        }

        let n = bits.len();
        let mut result = Vec::new();
        result.push((n >> 8) as u8);
        result.push((n & 0xFF) as u8);

        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    byte |= 1 << (7 - i);
                }
            }
            result.push(byte);
        }

        result
    }

    /// Decode packed bytes back to `n` booleans.
    pub fn decode(data: &[u8], n: usize) -> Vec<bool> {
        if data.len() < 2 || n == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(n);
        let mut bit_idx = 0;

        for &byte in &data[2..] {
            for i in (0..8).rev() {
                if bit_idx >= n {
                    break;
                }
                result.push((byte >> i) & 1 == 1);
                bit_idx += 1;
            }
        }

        result
    }
}
