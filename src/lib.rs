//! # Number to words converter

#![deny(missing_docs)]

#[cfg(feature = "en_us")]
mod en_us;
#[cfg(test)]
mod test_utils;
mod types;
#[cfg(feature = "uk_ua")]
mod uk_ua;
mod utils;

#[cfg(feature = "en_us")]
pub use en_us::integer_to_en_us;
#[cfg(feature = "uk_ua")]
pub use uk_ua::integer_to_uk_ua;
