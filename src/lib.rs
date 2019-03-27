extern crate num;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod enums;
pub mod spin_parameters;

use enums::SpinEntropy;
use spin_parameters::SpinParameters;
use std::io::Bytes;

mod statics {
    /// byte 0 to 255	Unsigned 8-bit integer
    pub(crate) static MaxVectorLength: u8 = 63;
    pub(crate) static MaxVectorLengthV2: u8 = 127;
    pub(crate) static BaseLength: u8 = 16;
    pub(crate) static BaseLengthV2: u8 = 22;
}

/// This class represents a lightweight vector for identifying and measuring causality.
#[derive(Debug)]
pub struct CorrelationVector {
    base_vector: &'static str,
    extension: i32,
    immutable: bool,
    /// This is the header that should be used between services to pass the correlation vector.
    header_name: &'static str,
    /// This is termination sign should be used when vector length exceeds max allowed length
    termination_sign: &'static str,
}

impl CorrelationVector {
    pub fn create(&self) -> i32 {
        let mut sp = SpinParameters::default();

        sp.set_entropy(Box::new(SpinEntropy::Four));

        println!("{}", *sp.entropy() as i32);

        return 23;
    }
}

impl Default for CorrelationVector {
    fn default() -> CorrelationVector {
        CorrelationVector {
            base_vector: "",
            extension: 0,
            immutable: false,
            header_name: "MS-CV",
            termination_sign: "!",
        }
    }
}
