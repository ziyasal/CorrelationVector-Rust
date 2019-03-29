use num_traits::FromPrimitive;
use std::fmt;

use crate::enums::SpinEntropy;
use crate::enums::SpinCounterInterval;
use crate::enums::SpinCounterPeriodicity;

/// This struct stores parameters used by the CorrelationVector Spin operator.
#[derive(Debug, Default)]
pub struct SpinParameters {
    // Internal value for entropy bytes.
    entropy_bytes: i32,

    /// The interval (proportional to time) by which the counter increments.
    pub  interval: SpinCounterInterval,

    /// How frequently the counter wraps around to zero, as determined by the amount
    /// of space to store the counter.
    pub  periodicity: SpinCounterPeriodicity,

}

impl SpinParameters {
    pub fn new(interval: SpinCounterInterval,
               periodicity: SpinCounterPeriodicity,
               entropy: SpinEntropy) -> SpinParameters {
        SpinParameters {
            interval,
            periodicity,
            entropy_bytes: entropy as i32,
        }
    }

    /// The number of bytes to use for entropy. Valid values from a
    /// minimum of 0 to a maximum of 4.
    pub fn entropy(&self) -> Box<SpinEntropy> {
        let option = SpinEntropy::from_i32(self.entropy_bytes);

        match option {
            Some(spin_entropy) => Box::new(spin_entropy),
            _ => panic!("Valid value for Entropy should be from a minimum of 0 to a maximum of 4.")
        }
    }

    pub fn set_entropy(&mut self, value: Box<SpinEntropy>) {
        self.entropy_bytes = *value as i32;
    }

    /// The number of bytes used to store the entropy.
    pub(crate) fn entropy_bytes(&self) -> i32 {
        self.entropy_bytes
    }

    /// The number of least significant bits to drop in DateTime.Ticks when computing the counter.
    pub(crate) fn ticks_bits_to_drop(&self) -> i32 {
        match &self.interval {
            SpinCounterInterval::Coarse => 24,
            SpinCounterInterval::Fine => 16,
        }
    }

    pub(crate) fn total_bits(&self) -> i32 {
        let counter_bits: i32 = match &self.periodicity {
            SpinCounterPeriodicity::None => 0,
            SpinCounterPeriodicity::Short => 16,
            SpinCounterPeriodicity::Medium => 24,
            SpinCounterPeriodicity::Long => 32,
        };

        return counter_bits + (self.entropy_bytes * 8);
    }
}

impl fmt::Display for SpinParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " YO: ({})", self.entropy_bytes)
    }
}