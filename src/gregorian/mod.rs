//! Module supporting the random generation of date/time objects
//! according to the Gregorian calendar.
//!
//! **Required feature**: `gregorian`.
mod date;
mod time;

pub use date::*;
pub use time::*;

use crate::{ChineseFormatGenerator, RawGenerator};

/// Random generator dedicated to the date/time concepts
/// in the Gregorian calendar.
///
/// It is worth noting that it must be created via the
/// [ChineseFormatGenerator::gregorian] method;
/// furthermore, it actually just keeps a reference to
/// the [RawGenerator] owned by [ChineseFormatGenerator].
pub struct GregorianGenerator<'a> {
    raw_generator: &'a dyn RawGenerator,
}

impl ChineseFormatGenerator {
    /// Creates a reusable [GregorianGenerator] instance, for generating
    /// date/time values according to the Gregorian calendar.
    ///
    /// ```
    /// use chinese_rand::{*, gregorian::*};
    /// use chinese_format::{ChineseFormat, Variant, gregorian::*};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let date = gregorian.date(DateParams {
    ///     pattern: DatePattern::YearMonthDayWeekDay,
    ///     year_range: Some(2000..=2019),
    ///     formal: true,
    ///     week_format: Some(WeekFormat::Zhou)
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二零一三年五月二十三号周一"
    /// );
    ///
    /// fastrand::seed(90);
    /// let time = gregorian.linear_time(LinearTimeParams {
    ///     day_part: true,
    ///     include_second: true
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "下午四点二十分四十三秒");
    /// ```
    ///
    /// **Required feature**: `gregorian`.
    pub fn gregorian(&self) -> GregorianGenerator {
        GregorianGenerator {
            raw_generator: self.raw_generator.as_ref(),
        }
    }
}
