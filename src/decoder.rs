//! Arithmetic decoder.
//!
//! Decodes compressed bitstreams back to symbol sequences.

use crate::model::FrequencyModel;

/// Arithmetic decoder.
pub struct ArithmeticDecoder;

impl ArithmeticDecoder {
    /// Decode a compressed byte stream back to symbols using the given model.
    pub fn decode(data: &[u8], _n: usize, model: &FrequencyModel) -> Vec<usize> {
        if data.len() < 2 {
            return Vec::new();
        }

        let expected = ((data[0] as usize) << 8) | data[1] as usize;
        if expected == 0 {
            return Vec::new();
        }

        // Extract bit stream
        let bitstream: Vec<u8> = data[2..].iter()
            .flat_map(|&byte| {
                (0..8).rev().map(move |i| (byte >> i) & 1)
            })
            .collect();

        let precision_bits: u32 = 30;
        let whole = 1u64 << precision_bits;
        let half = whole >> 1;
        let quarter = half >> 1;

        let mut low: u64 = 0;
        let mut high: u64 = whole;
        let mut code: u64 = 0;

        // Initialize code from first bits
        for i in 0..precision_bits as usize {
            code <<= 1;
            if i < bitstream.len() {
                code |= bitstream[i] as u64;
            }
        }

        let mut bit_pos = precision_bits as usize;
        let mut result = Vec::new();

        for _ in 0..expected {
            let range = high - low;
            if range == 0 {
                break;
            }

            let value = ((code - low) as f64 / range as f64).clamp(0.0, 0.999999);
            let sym = model.symbol_for_value(value);

            let (sym_low, sym_high) = model.range_for(sym);

            high = low + ((range as f64 * sym_high) as u64).min(high);
            low += (range as f64 * sym_low) as u64;

            result.push(sym);

            loop {
                if high <= half {
                    low <<= 1;
                    high <<= 1;
                    code <<= 1;
                } else if low >= half {
                    low = (low - half) << 1;
                    high = (high - half) << 1;
                    code = (code - half) << 1;
                } else if low >= quarter && high <= half + quarter {
                    low = (low - quarter) << 1;
                    high = (high - quarter) << 1;
                    code = (code - quarter) << 1;
                } else {
                    break;
                }

                if bit_pos < bitstream.len() {
                    code |= bitstream[bit_pos] as u64;
                    bit_pos += 1;
                }
            }
        }

        result
    }
}
