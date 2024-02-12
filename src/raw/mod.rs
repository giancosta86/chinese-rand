#[cfg(feature = "fastrand")]
mod fastrand_raw;

use std::ops::RangeInclusive;

#[cfg(feature = "fastrand")]
pub use fastrand_raw::*;

/// Generator of primitive values required by [ChineseFormatGenerator](crate::ChineseFormatGenerator).
pub trait RawGenerator {
    /// Generates a random [u8] in the given range.
    fn u8(&self, range: RangeInclusive<u8>) -> u8;

    /// Generates a random [u16] in the given range.
    fn u16(&self, range: RangeInclusive<u16>) -> u16;

    /// Generates a random [u32] in the given range.
    fn u32(&self, range: RangeInclusive<u32>) -> u32;

    /// Generates a random [u64] in the given range.
    fn u64(&self, range: RangeInclusive<u64>) -> u64;

    /// Generates a random [u128] in the given range.
    fn u128(&self, range: RangeInclusive<u128>) -> u128;

    /// Generates a random [i128] in the given range.
    fn i128(&self, range: RangeInclusive<i128>) -> i128;

    /// Generates a random [bool] in the given range.
    fn bool(&self) -> bool;
}
