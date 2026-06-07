//! # arithmetic-code
//!
//! Arithmetic coding with adaptive frequency models, range encoding, and
//! binary arithmetic coding — pure Rust, no dependencies.

pub mod encoder;
pub mod decoder;
pub mod model;
pub mod range;
pub mod binary;

pub use encoder::ArithmeticEncoder;
pub use decoder::ArithmeticDecoder;
pub use model::FrequencyModel;
pub use range::Range;
pub use binary::BinaryArithmeticCoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encode_decode() {
        let mut model = FrequencyModel::new(4);
        model.update(0); model.update(1); model.update(2); model.update(3);

        let msg = vec![0, 1, 2, 3];
        let encoded = ArithmeticEncoder::encode(&msg, &model);
        let decoded = ArithmeticDecoder::decode(&encoded, msg.len(), &model);
        assert_eq!(decoded, msg);
    }

    #[test]
    fn test_single_symbol() {
        let mut model = FrequencyModel::new(2);
        model.update(0);

        let msg = vec![0, 0, 0, 0];
        let encoded = ArithmeticEncoder::encode(&msg, &model);
        let decoded = ArithmeticDecoder::decode(&encoded, msg.len(), &model);
        assert_eq!(decoded, msg);
    }

    #[test]
    fn test_repeated_pattern() {
        let mut model = FrequencyModel::new(3);
        for &s in &[0, 1, 2, 0, 1, 2] {
            model.update(s);
        }

        let msg = vec![0, 1, 2, 0, 1, 2, 0, 1, 2];
        let encoded = ArithmeticEncoder::encode(&msg, &model);
        let decoded = ArithmeticDecoder::decode(&encoded, msg.len(), &model);
        assert_eq!(decoded, msg);
    }

    #[test]
    fn test_biased_distribution() {
        let mut model = FrequencyModel::new(3);
        for _ in 0..90 { model.update(0); }
        for _ in 0..5 { model.update(1); }
        for _ in 0..5 { model.update(2); }

        let msg = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let encoded = ArithmeticEncoder::encode(&msg, &model);
        assert!(encoded.len() < 80); // Should compress well with biased source
        let decoded = ArithmeticDecoder::decode(&encoded, msg.len(), &model);
        assert_eq!(decoded, msg);
    }

    #[test]
    fn test_range_basic() {
        let r = Range::new(0.0, 1.0);
        assert!((r.low() - 0.0).abs() < 1e-10);
        assert!((r.high() - 1.0).abs() < 1e-10);
        assert!((r.width() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_range_narrow() {
        let r = Range::new(0.25, 0.75);
        let narrowed = r.narrow(0.5, 1.0);
        assert!((narrowed.low() - 0.5).abs() < 1e-10);
        assert!((narrowed.high() - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_range_subrange() {
        let r = Range::new(0.0, 1.0);
        let s = r.subrange(0.25, 0.75);
        assert!((s.low() - 0.25).abs() < 1e-10);
        assert!((s.high() - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_frequency_model_uniform() {
        let model = FrequencyModel::new_uniform(4);
        for i in 0..4 {
            let (lo, hi) = model.range_for(i);
            assert!(hi > lo);
        }
    }

    #[test]
    fn test_frequency_model_update() {
        let mut model = FrequencyModel::new(3);
        model.update(0);
        model.update(0);
        model.update(1);
        // After 3 updates + 3 initial counts: freq = [3, 2, 1], total = 6
        let (lo, hi) = model.range_for(0);
        assert!((hi - lo - 3.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_frequency_model_total() {
        let mut model = FrequencyModel::new(3);
        model.update(0);
        model.update(1);
        model.update(2);
        assert_eq!(model.total(), 6); // 3 initial + 3 updates
    }

    #[test]
    fn test_binary_coder_basic() {
        let bits = vec![true, false, true, true, false];
        let encoded = BinaryArithmeticCoder::encode(&bits);
        let decoded = BinaryArithmeticCoder::decode(&encoded, bits.len());
        assert_eq!(decoded, bits);
    }

    #[test]
    fn test_binary_coder_all_same() {
        let bits = vec![false; 20];
        let encoded = BinaryArithmeticCoder::encode(&bits);
        assert!(encoded.len() < 20); // Should compress
        let decoded = BinaryArithmeticCoder::decode(&encoded, bits.len());
        assert_eq!(decoded, bits);
    }

    #[test]
    fn test_binary_coder_alternating() {
        let bits: Vec<bool> = (0..10).map(|i| i % 2 == 0).collect();
        let encoded = BinaryArithmeticCoder::encode(&bits);
        let decoded = BinaryArithmeticCoder::decode(&encoded, bits.len());
        assert_eq!(decoded, bits);
    }

    #[test]
    fn test_model_symbol_from_range() {
        let mut model = FrequencyModel::new(4);
        for i in 0..4 { model.update(i); }

        // With uniform model, value 0.5 should map to symbol 2
        let sym = model.symbol_for_value(0.5);
        assert_eq!(sym, 2);
    }

    #[test]
    fn test_encoder_output_not_empty() {
        let model = FrequencyModel::new_uniform(4);
        let msg = vec![0, 1, 2, 3];
        let encoded = ArithmeticEncoder::encode(&msg, &model);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_longer_message() {
        let mut model = FrequencyModel::new(4);
        for &s in &[0, 1, 2, 3, 0, 1, 2, 3] {
            model.update(s);
        }
        let msg = vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
        let encoded = ArithmeticEncoder::encode(&msg, &model);
        let decoded = ArithmeticDecoder::decode(&encoded, msg.len(), &model);
        assert_eq!(decoded, msg);
    }
}
