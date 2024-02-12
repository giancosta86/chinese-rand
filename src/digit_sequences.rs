use crate::ChineseFormatGenerator;
use chinese_format::{Decimal, IntegerPart};
use digit_sequence::DigitSequence;
use std::{iter::repeat_with, ops::RangeInclusive};

impl ChineseFormatGenerator {
    /// Generates a random [DigitSequence] with length in the given range.
    ///
    /// ```
    /// use chinese_rand::*;
    /// use digit_sequence::DigitSequence;
    ///
    /// fastrand::seed(90);
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let sequence = generator.digit_sequence(0..=10);
    /// assert_eq!(sequence, DigitSequence::from(3724260u32));
    ///
    /// let fixed_length = generator.digit_sequence(5..=5);
    /// assert_eq!(fixed_length, DigitSequence::from(85241u32));
    ///
    /// let empty = generator.digit_sequence(0..=0);
    /// assert_eq!(empty, DigitSequence::new());
    /// ```
    ///
    /// **Required feature**: `digit-sequence`.
    pub fn digit_sequence(&self, length_range: RangeInclusive<u8>) -> DigitSequence {
        let length = self.raw_generator.u8(length_range);

        let digits: Vec<u8> = repeat_with(|| self.raw_generator.u8(0..=9))
            .take(length as usize)
            .collect();

        digits.try_into().expect("Digits valid by construction")
    }

    /// Generates a random [Decimal].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::Decimal;
    /// use digit_sequence::DigitSequence;
    ///
    /// fastrand::seed(90);
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let decimal = generator.decimal(
    ///     i128::MIN..=i128::MAX,
    ///     0..=4
    /// );
    /// assert_eq!(decimal, Decimal {
    ///     integer: -139744823884027955216713073977120108615,
    ///     fractional: 242u8.into()
    /// });
    ///
    /// let fixed = generator.decimal(90..=90, 5..=5);
    /// assert_eq!(fixed, Decimal {
    ///     integer: 90,
    ///     fractional: 85241u32.into()
    /// });
    ///
    /// let zero = generator.decimal(0..=0, 0..=0);
    /// assert_eq!(zero, Decimal {
    ///     integer: 0,
    ///     fractional: DigitSequence::new()
    /// });
    /// ```
    ///
    /// **Required feature**: `digit-sequence`.
    pub fn decimal(
        &self,
        integer_range: RangeInclusive<IntegerPart>,
        fractional_length_range: RangeInclusive<u8>,
    ) -> Decimal {
        let integer: IntegerPart = self.raw_generator.i128(integer_range);

        let fractional: DigitSequence = self.digit_sequence(fractional_length_range);

        Decimal {
            integer,
            fractional,
        }
    }
}
