//! Range representation for arithmetic coding.
//!
//! Represents the [low, high) interval used during encoding/decoding.

/// A range [low, high) used in arithmetic coding.
#[derive(Debug, Clone, Copy)]
pub struct Range {
    low: f64,
    high: f64,
}

impl Range {
    /// Create a new range.
    pub fn new(low: f64, high: f64) -> Self {
        assert!(low < high, "low must be less than high");
        Self { low, high }
    }

    /// Get the lower bound.
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Get the upper bound.
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Get the width of the range.
    pub fn width(&self) -> f64 {
        self.high - self.low
    }

    /// Narrow this range to the subrange [new_low, new_high) within [0, 1).
    pub fn narrow(&self, new_low: f64, new_high: f64) -> Self {
        let width = self.width();
        Self {
            low: self.low + new_low * width,
            high: self.low + new_high * width,
        }
    }

    /// Get a subrange at the given normalized position.
    pub fn subrange(&self, lo: f64, hi: f64) -> Self {
        let width = self.width();
        Self {
            low: self.low + lo * width,
            high: self.low + hi * width,
        }
    }

    /// Check if range contains a value.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.low && value < self.high
    }

    /// Normalize value to [0, 1) within this range.
    pub fn normalize(&self, value: f64) -> f64 {
        (value - self.low) / self.width()
    }
}
