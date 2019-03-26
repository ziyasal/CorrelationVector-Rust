
extern crate num;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod enums;

use num_traits::FromPrimitive;
use std::fmt;
use enums::SpinEntropy;

/// This struct stores parameters used by the CorrelationVector Spin operator.
#[derive(Debug, Default)]
pub struct SpinParameters {
    // Internal value for entropy bytes.
    entropy_bytes: i32,
}

impl SpinParameters {
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
}

impl fmt::Display for SpinParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " YO: ({})", self.entropy_bytes)
    }
}
