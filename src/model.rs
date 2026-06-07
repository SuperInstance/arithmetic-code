//! Frequency model for arithmetic coding.
//!
//! Maintains symbol frequency counts and provides cumulative range lookups.

/// A frequency model that tracks symbol counts and provides probability ranges.
#[derive(Debug, Clone)]
pub struct FrequencyModel {
    freq: Vec<u64>,
    total: u64,
}

impl FrequencyModel {
    /// Create a new model with `n` symbols, all starting at count 0.
    pub fn new(n: usize) -> Self {
        Self {
            freq: vec![1; n], // Start with count 1 for each (Laplace smoothing)
            total: n as u64,
        }
    }

    /// Create a uniform model with `n` symbols.
    pub fn new_uniform(n: usize) -> Self {
        Self {
            freq: vec![1; n],
            total: n as u64,
        }
    }

    /// Increment the count for symbol `s`.
    pub fn update(&mut self, s: usize) {
        self.freq[s] += 1;
        self.total += 1;
    }

    /// Get the cumulative range [lo, hi) for symbol `s`, normalized to [0, 1).
    pub fn range_for(&self, s: usize) -> (f64, f64) {
        let mut cum = 0u64;
        for i in 0..s {
            cum += self.freq[i];
        }
        let lo = cum as f64 / self.total as f64;
        let hi = (cum + self.freq[s]) as f64 / self.total as f64;
        (lo, hi)
    }

    /// Find which symbol corresponds to a value in [0, 1).
    pub fn symbol_for_value(&self, value: f64) -> usize {
        let mut cum = 0u64;
        for (i, &f) in self.freq.iter().enumerate() {
            cum += f;
            if (cum as f64 / self.total as f64) > value {
                return i;
            }
        }
        self.freq.len() - 1
    }

    /// Get total frequency count.
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Get frequency for a specific symbol.
    pub fn freq(&self, s: usize) -> u64 {
        self.freq[s]
    }

    /// Get number of symbols.
    pub fn alphabet_size(&self) -> usize {
        self.freq.len()
    }
}
