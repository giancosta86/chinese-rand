use crate::{ChineseFormatGenerator, InvalidLowerBound};
use chinese_format::{Count, CountBase, Fraction};
use std::ops::RangeInclusive;

impl ChineseFormatGenerator {
    /// Generates a random [i128] in the given range.
    ///
    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let integer = generator.integer(i128::MIN..=i128::MAX);
    /// assert_eq!(integer, -139744823884027955216713073977120108615);
    ///
    /// let fixed = generator.integer(90..=90);
    /// assert_eq!(fixed, 90);
    /// ```
    pub fn integer(&self, range: RangeInclusive<i128>) -> i128 {
        self.raw_generator.i128(range)
    }

    /// Generates a [Fraction] having its components in the given ranges.
    ///
    /// The lower bound for the denominator cannot be 0, or the function
    /// will fail with [InvalidLowerBound].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::Fraction;
    ///
    /// # fn main() -> GenericResult<()> {
    /// fastrand::seed(90);
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let fraction = generator.fraction(
    ///     1..=u128::MAX,
    ///     i128::MIN..=i128::MAX
    /// )?;
    /// assert_eq!(
    ///     fraction,
    ///     Fraction::try_new(
    ///         200537543036910508246661533454648102841,
    ///         -116432671981703681494469200238719654801
    ///     )?
    /// );
    ///
    /// let fixed = generator.fraction(5..=5, 4..=4)?;
    /// assert_eq!(
    ///     fixed,
    ///     Fraction::try_new(5, 4)?
    /// );
    ///
    /// let fraction_result = generator.fraction(0..=5, 4..=4);
    /// assert_eq!(
    ///     fraction_result,
    ///     Err(InvalidLowerBound(0))
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn fraction(
        &self,
        denominator_range: RangeInclusive<u128>,
        numerator_range: RangeInclusive<i128>,
    ) -> Result<Fraction, InvalidLowerBound<u128>> {
        if *denominator_range.start() == 0 {
            return Err(InvalidLowerBound(0));
        }

        let denominator = self.raw_generator.u128(denominator_range);

        let numerator = self.raw_generator.i128(numerator_range);

        Ok(
            Fraction::try_new(denominator, numerator)
                .expect("Denominator non-zero by construction"),
        )
    }

    /// Generates a random [Count] in the given range.
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Count, CountBase};
    ///
    /// fastrand::seed(90);
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let count = generator.count(0..=CountBase::MAX);
    /// assert_eq!(count, Count(200537543036910508246661533454648102841));
    ///
    /// let fixed = generator.count(90..=90);
    /// assert_eq!(fixed, Count(90));
    /// ```
    pub fn count(&self, range: RangeInclusive<CountBase>) -> Count {
        Count(self.raw_generator.u128(range))
    }
}
