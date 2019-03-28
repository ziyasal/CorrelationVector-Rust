extern crate num;
extern crate rand;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod enums;
pub mod spin_parameters;

use enums::SpinEntropy;
use spin_parameters::SpinParameters;
use rand::prelude::*;
use crate::enums::CorrelationVectorVersion;
use std::str::FromStr;

/// Gets or sets a value indicating whether or not to validate the correlation vector on creation.
static mut VALIDATE_CORRELATION_VECTOR_DURING_CREATION: bool = false;

/// This class represents a lightweight vector for identifying and measuring causality.
#[derive(Debug)]
pub struct CorrelationVector {
    base_vector: String,
    extension: i32,
    immutable: bool,
    rng: Box<ThreadRng>,
    version: CorrelationVectorVersion,
}

impl CorrelationVector {
    /// byte: 0 to 255	Unsigned 8-bit integer
    const MAX_VECTOR_LENGTH: u8 = 63;
    const MAX_VECTOR_LENGTH_V2: u8 = 127;
    const BASE_LENGTH: u8 = 16;
    const BASE_LENGTH_V2: u8 = 22;

    /// This is termination sign should be used when vector length exceeds max allowed length
    const TERMINATION_SIGN: &'static str = "!";

    /// This is the header that should be used between services to pass the correlation vector.
    const HEADER_NAME: &'static str = "MS-CV";

    pub fn create(&mut self) -> i32 {
        let mut sp = SpinParameters::default();

        sp.set_entropy(Box::new(SpinEntropy::Four));

        println!("{}", *sp.entropy() as i32);
        println!("{}", *sp.entropy() as i32);

        println!("Rand: {}", self.rng.gen::<u32>());
        return 23;
    }

    pub fn new(base_vector: &str, extension: i32, version: CorrelationVectorVersion, immutable: bool) -> CorrelationVector {
        CorrelationVector {
            base_vector: String::from_str(base_vector).unwrap(),
            extension,
            version,
            immutable,
            rng: Box::new(rand::thread_rng()),
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

        let x: String = format!("{}.{}{}", self.base_vector, self.extension, append);
        Box::new(x)
    }

    #[inline]
    pub fn extend(correlation_vector: &str, validate_correlation_vector_during_creation: bool) -> CorrelationVector {
        if CorrelationVector::is_immutable(correlation_vector)
        {
            return CorrelationVector::parse(correlation_vector);
        }

        let version: enums::CorrelationVectorVersion;

        version = CorrelationVector::infer_version(correlation_vector, validate_correlation_vector_during_creation);

        if validate_correlation_vector_during_creation
        {
            CorrelationVector::validate(correlation_vector, version);
        }

        if CorrelationVector::is_oversized(correlation_vector, 0, version)
        {
            // TODO: Termination sign
            let vector = format!("{}{}", correlation_vector, "!");
            return CorrelationVector::parse(vector.as_str());
        }

        CorrelationVector::new(correlation_vector, 0, version, false)
    }

    #[inline]
    pub fn is_immutable(correlation_vector: &str) -> bool {
        !correlation_vector.is_empty() && correlation_vector.ends_with(CorrelationVector::TERMINATION_SIGN)
    }

    #[inline]
    pub fn parse(correlation_vector: &str) -> CorrelationVector {
        if !correlation_vector.is_empty() {

            match correlation_vector.rfind('.') {
                Some(last_index) => {
                    let base_vector = &correlation_vector[0..last_index];
                    let mut extension = &correlation_vector[last_index+1..correlation_vector.len()];

                    let immutable = CorrelationVector::is_immutable(correlation_vector);
                    let version = CorrelationVector::infer_version(correlation_vector, false);

                    if immutable {
                        extension = &extension[0..extension.len()-1]
                    }

                    return CorrelationVector {
                        base_vector: String::from_str(base_vector).unwrap(),
                        extension: i32::from_str(extension).unwrap_or(9),
                        version,
                        immutable,
                        rng: Box::new(rand::thread_rng()),
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
}

impl Default for CorrelationVector {
    fn default() -> CorrelationVector {
        CorrelationVector {
            base_vector: String::from(""),
            extension: 0,
            immutable: false,
            rng: Box::new(rand::thread_rng()),
            version: CorrelationVectorVersion::V1,
        }
    }
}
