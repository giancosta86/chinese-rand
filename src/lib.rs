//! This crate is designed to instantiate random instances
//! of the data structures implementing the [ChineseFormat](chinese_format::ChineseFormat) trait, made available by [chinese_format].
//!
//! The core concept is the [ChineseFormatGenerator] struct,
//! which can be instantiated by passing an object implementing
//! the [RawGenerator] trait:
//!
//! ```
//! use chinese_rand::*;
//! use chinese_format::{ChineseFormat, Fraction, Variant};
//!
//! # fn main() -> GenericResult<()> {
//! let generator = ChineseFormatGenerator::new(
//!     FastRandGenerator::new()
//! );
//!
//! // Now setting the random seed just in order to
//! //predict the generated values
//! fastrand::seed(90);
//!
//! let fraction: Fraction = generator.fraction(
//!     1..=10,
//!     1..=10
//! )?;
//!
//! let chinese = fraction.to_chinese(Variant::Simplified);
//!
//! assert_eq!(chinese, "六分之七");
//!
//! Ok(())
//! }
//! ```
//!
//! # Features
//!
//! - `fastrand`: enables [FastRandGenerator], based on [fastrand]. **Enabled by default**.
//!
//! - `digit-sequence`: enables random generation of data types - like [Decimal](chinese_format::Decimal) - based on [DigitSequence](digit_sequence::DigitSequence).
//!
//! - `currency`: enables the random generation of data types in the [currency](chinese_format::currency) module.
//!
//! - `gregorian`: enables the random generation of data types in the [gregorian](chinese_format::gregorian) module, which is dedicated to dates and times.
//!
//!   _Also enables_: `digit-sequence`.

#[cfg(feature = "currency")]
mod currency;
#[cfg(feature = "digit-sequence")]
mod digit_sequences;
mod errors;
#[cfg(feature = "gregorian")]
pub mod gregorian;
mod numeric;
mod raw;

use std::error::Error;

#[cfg(feature = "currency")]
pub use currency::*;
pub use errors::*;
pub use raw::*;

/// The most generic [Error]-based [Result].
pub type GenericResult<T> = Result<T, Box<dyn Error>>;

/// Parametrically generates random instances of the data structures
/// provided by [chinese_format].
pub struct ChineseFormatGenerator {
    pub(crate) raw_generator: Box<dyn RawGenerator>,
}

impl ChineseFormatGenerator {
    /// Creating a [ChineseFormatGenerator] requires an object
    /// implementing the [RawGenerator] interface.
    ///
    /// ```
    /// use chinese_rand::*;
    ///
    /// // Now setting the random seed just in order to
    /// //predict the generated values
    /// fastrand::seed(90);
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let integer = generator.integer(i128::MIN..=i128::MAX);
    /// assert_eq!(integer, -139744823884027955216713073977120108615);
    /// ```
    pub fn new(raw_generator: impl RawGenerator + 'static) -> Self {
        Self {
            raw_generator: Box::new(raw_generator),
        }
    }
}
