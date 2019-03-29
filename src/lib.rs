extern crate num;
extern crate rand;
extern crate chrono;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod enums;
pub mod spin_parameters;

use enums::SpinEntropy;
use spin_parameters::SpinParameters;
use rand::prelude::*;
use crate::enums::{CorrelationVectorVersion, SpinCounterInterval, SpinCounterPeriodicity};
use std::str::FromStr;
use chrono::prelude::*;

/// This class represents a lightweight vector for identifying and measuring causality.
#[derive(Debug)]
pub struct CorrelationVector {
    base_vector: String,
    extension: i32,
    immutable: bool,
    version: CorrelationVectorVersion,
}

impl CorrelationVector {
    /// byte: 0 to 255	Unsigned 8-bit integer

    const BASE_LENGTH: u8 = 16;
    const MAX_VECTOR_LENGTH: u8 = 63;

    const BASE_LENGTH_V2: u8 = 22;
    const MAX_VECTOR_LENGTH_V2: u8 = 127;

    /// This is termination sign should be used when vector length exceeds max allowed length
    const TERMINATION_SIGN: &'static str = "!";

    /// This is the header that should be used between services to pass the correlation vector.
    const HEADER_NAME: &'static str = "MS-CV";

    pub fn new(base_vector: String, extension: i32, version: CorrelationVectorVersion, immutable: bool) -> CorrelationVector {
        CorrelationVector {
            base_vector,
            extension,
            version,
            immutable,
        }
    }

    /// Gets the version of the correlation vector implementation.
    pub fn version(&self) -> CorrelationVectorVersion {
        self.version
    }

    pub fn value(&self) -> Box<String> {
        let mut append = "";

        if self.immutable {
            append = CorrelationVector::TERMINATION_SIGN;
        }

        let value_str: String = format!("{}.{}{}", self.base_vector, self.extension, append);
        Box::new(value_str)
    }

    /// Creates a new correlation vector by extending an existing value. This should be
    /// done at the entry point of an operation.
    ///  # Arguments
    ///
    /// * `correlation_vector` - Taken from the message header indicated MS-CV
    /// * `validate_correlation_vector_during_creation` - Gets or sets a value indicating whether or not to validate the correlation vector on creation.
    /// # Returns
    ///  A new correlation vector extended from the current vector.
    ///
    #[inline]
    pub fn extend(correlation_vector: &str, validate_correlation_vector_during_creation: bool) -> CorrelationVector {
        if CorrelationVector::is_immutable(correlation_vector) {
            return CorrelationVector::parse(correlation_vector, true);
        }

        let version: CorrelationVectorVersion;

        version = CorrelationVector::infer_version(correlation_vector, validate_correlation_vector_during_creation);

        if validate_correlation_vector_during_creation {
            CorrelationVector::validate(correlation_vector, version);
        }

        if CorrelationVector::is_oversized(correlation_vector, 0, version) {
            // let immutable_vector = format!("{}{}", correlation_vector, CorrelationVector::TERMINATION_SIGN);
            return CorrelationVector::parse(correlation_vector, true);
        }

        return CorrelationVector::new(String::from(correlation_vector), 0, version, false);
    }

    #[inline]
    pub fn is_immutable(correlation_vector: &str) -> bool {
        !correlation_vector.is_empty() && correlation_vector.ends_with(CorrelationVector::TERMINATION_SIGN)
    }

    #[inline]
    pub fn parse(correlation_vector: &str, immutable: bool) -> CorrelationVector {
        if !correlation_vector.is_empty() {
            match correlation_vector.rfind('.') {
                Some(last_index) => {
                    let base_vector = &correlation_vector[0..last_index];
                    let mut extension = &correlation_vector[last_index + 1..correlation_vector.len()];

                    let version = CorrelationVector::infer_version(correlation_vector, false);

                    if immutable {
                        extension = &extension[0..extension.len() - 1]
                    }

                    return CorrelationVector {
                        base_vector: String::from_str(base_vector).unwrap(),
                        extension: i32::from_str(extension).unwrap_or(0),
                        version,
                        immutable,
                    };
                }
                None => panic!("Invalid CV")
            }
        }

        // panic here
        CorrelationVector::default()
    }

    #[inline]
    pub fn is_oversized(base_vector: &str, extension: u32, version: CorrelationVectorVersion) -> bool {
        if !base_vector.is_empty() {
            let mut here: u8 = 0;
            if extension > 0 {
                /* log10 available for f32, f64 */
                here = (extension as f32).log10() as u8;
            }

            let size: u8 = base_vector.len() as u8 + 1 /*.*/ + here + 1;

            return
                (version == CorrelationVectorVersion::V1 && size > CorrelationVector::MAX_VECTOR_LENGTH)
                    ||
                    (version == CorrelationVectorVersion::V2 && size > CorrelationVector::MAX_VECTOR_LENGTH_V2);
        }

        return false;
    }


    #[inline]
    pub fn infer_version(correlation_vector: &str, _report_errors: bool) -> CorrelationVectorVersion {
        let index_of = correlation_vector.find('.');
        let index = match index_of {
            Some(x) => x as u8,
            None => 0
        };

        if CorrelationVector::BASE_LENGTH == index {
            return CorrelationVectorVersion::V1;
        } else if CorrelationVector::BASE_LENGTH_V2 == index {
            return CorrelationVectorVersion::V2;
        } else {
            //By default not reporting error, just return V1
            return CorrelationVectorVersion::V1;
        }
    }

    #[inline]
    pub fn validate(correlation_vector: &str, version: CorrelationVectorVersion) -> bool {
        let max_vector_length: u8;
        let base_length: u8;

        if CorrelationVectorVersion::V1 == version {
            max_vector_length = CorrelationVector::MAX_VECTOR_LENGTH;
            base_length = CorrelationVector::BASE_LENGTH;
        } else if CorrelationVectorVersion::V2 == version {
            max_vector_length = CorrelationVector::MAX_VECTOR_LENGTH_V2;
            base_length = CorrelationVector::BASE_LENGTH_V2;
        } else {
            panic!(format!("Unsupported correlation vector version: {:?}", version));
        }

        if correlation_vector.is_empty() || correlation_vector.len() as u8 > max_vector_length {
            panic!(format!("The {:?} correlation vector can not be null or bigger than {} characters", version, max_vector_length));
        }

        let parts: Vec<&str> = correlation_vector.split('.').collect();

        if parts.len() < 2 || parts[0].len() as u8 != base_length
        {
            panic!(format!("Invalid correlation vector {}. Invalid base value {}", correlation_vector, parts[0]));
        }

        true
    }

    #[inline]
    pub fn spin(correlation_vector: &str, validate_correlation_vector_during_creation: bool) -> CorrelationVector {
        let default_parameters = SpinParameters::new(SpinCounterInterval::Coarse, SpinCounterPeriodicity::Short, SpinEntropy::Two);

        return CorrelationVector::spin_with_params(
            correlation_vector,
            default_parameters,
            validate_correlation_vector_during_creation,
        );
    }

    #[inline]
    pub fn spin_with_params(correlation_vector: &str, parameters: SpinParameters, validate_correlation_vector_during_creation: bool) -> CorrelationVector {
        if CorrelationVector::is_immutable(correlation_vector) {
            return CorrelationVector::parse(correlation_vector, true);
        }

        let version = CorrelationVector::infer_version(correlation_vector, validate_correlation_vector_during_creation);

        if validate_correlation_vector_during_creation {
            CorrelationVector::validate(correlation_vector, version);
        }

        println!("{:?}", parameters);

        let entropy: Vec<u8> = (0..parameters.entropy_bytes()).map(|_| { rand::random::<u8>() }).collect();

        let utc: DateTime<Utc> = Utc::now();

        // u64 is ulong here `type c_ulong = u64` in code.
        let mut value: u64 = (utc.timestamp_millis() >> parameters.ticks_bits_to_drop()) as u64;

        for i in 0..parameters.entropy_bytes() {
            value = (value << 8) | entropy[i as usize] as u64;
        }

        // Generate a bitmask and mask the lower total_bits in the value.
        // The mask is generated by (1 << total_bits) - 1. We need to handle the edge case when shifting 64 bits, as it wraps around.
        value &= if parameters.total_bits() == 64 { 0 } else { (1 as u64) << parameters.total_bits() - 1 };

        let mut s = value.to_string();

        if parameters.total_bits() > 32 {
            s = format!("{}.{}", (value >> 32), s);
        }

        let base_vector = format!("{}.{}", correlation_vector, s);

        if CorrelationVector::is_oversized(&base_vector, 0, version) {
            return CorrelationVector::parse(correlation_vector, true);
        }

        return CorrelationVector::new(base_vector, 0, version, false);
    }
}

impl<'a> Default for CorrelationVector {
    fn default() -> CorrelationVector {
        CorrelationVector {
            base_vector: String::from(""),
            extension: 0,
            immutable: false,
            version: CorrelationVectorVersion::V1,
        }
    }
}
