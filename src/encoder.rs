//! Arithmetic encoder.
//!
//! Encodes symbol sequences into compressed bitstreams using probability ranges.

use crate::model::FrequencyModel;

/// Arithmetic encoder.
pub struct ArithmeticEncoder;

impl ArithmeticEncoder {
    /// Encode a message using the given frequency model.
    ///
    /// Returns a vector of bytes representing the compressed bitstream.
    pub fn encode(message: &[usize], model: &FrequencyModel) -> Vec<u8> {
        if message.is_empty() {
            return vec![0, 0];
        }

        let precision_bits: u32 = 30;
        let whole = 1u64 << precision_bits;
        let half = whole >> 1;
        let quarter = half >> 1;

        let mut low_i: u64 = 0;
        let mut high_i: u64 = whole;
        let mut pending: usize = 0;
        let mut output_bits: Vec<u8> = Vec::new();

        for &sym in message {
            let range = high_i - low_i;
            let (sym_low, sym_high) = model.range_for(sym);

            high_i = low_i + ((range as f64 * sym_high) as u64).min(high_i);
            low_i += (range as f64 * sym_low) as u64;

            loop {
                if high_i <= half {
                    output_bits.push(0);
                    output_bits.extend(std::iter::repeat_n(1, pending));
                    pending = 0;
                    low_i <<= 1;
                    high_i <<= 1;
                } else if low_i >= half {
                    output_bits.push(1);
                    output_bits.extend(std::iter::repeat_n(0, pending));
                    pending = 0;
                    low_i = (low_i - half) << 1;
                    high_i = (high_i - half) << 1;
                } else if low_i >= quarter && high_i <= half + quarter {
                    pending += 1;
                    low_i = (low_i - quarter) << 1;
                    high_i = (high_i - quarter) << 1;
                } else {
                    break;
                }
            }
        }

        // Flush
        pending += 1;
        if low_i <= quarter {
            output_bits.push(0);
            output_bits.extend(std::iter::repeat_n(1, pending));
        } else {
            output_bits.push(1);
            output_bits.extend(std::iter::repeat_n(0, pending));
        }

        // Pack into bytes
        let mut bytes = Vec::new();
        let msg_len = message.len();
        bytes.push((msg_len >> 8) as u8);
        bytes.push((msg_len & 0xFF) as u8);

        for chunk in output_bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit != 0 {
                    byte |= 1 << (7 - i);
                }
            }
            bytes.push(byte);
        }

        bytes
    }
}
