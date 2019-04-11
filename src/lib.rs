extern crate num;
extern crate rand;
extern crate chrono;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod vector_impl;
pub mod enums;
pub mod spin_parameters;

use vector_impl::CorrelationVector;

pub fn create_vector() -> CorrelationVector {
    CorrelationVector::default()
}