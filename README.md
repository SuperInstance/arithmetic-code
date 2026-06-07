# arithmetic-code

Arithmetic coding encoder/decoder with adaptive frequency models, range encoding, and binary arithmetic coding — pure Rust, no external dependencies.

## Features

- **Arithmetic encoder/decoder**: Near-optimal entropy coding
- **Frequency model**: Adaptive probability tracking with Laplace smoothing
- **Range encoding**: Interval-based compression with E1/E2/E3 scaling
- **Binary arithmetic coding**: Specialized coder for boolean streams

## Usage

```rust
use arithmetic_code::*;

// Create a frequency model
let mut model = FrequencyModel::new(4);
for &s in &[0, 1, 2, 3] { model.update(s); }

// Encode and decode
let msg = vec![0, 1, 2, 3, 0, 1, 2, 3];
let encoded = ArithmeticEncoder::encode(&msg, &model);
let decoded = ArithmeticDecoder::decode(&encoded, msg.len(), &model);
assert_eq!(decoded, msg);

// Binary arithmetic coding
let bits = vec![true, false, true, true, false];
let encoded = BinaryArithmeticCoder::encode(&bits);
let decoded = BinaryArithmeticCoder::decode(&encoded, bits.len());
assert_eq!(decoded, bits);
```

## License

MIT
