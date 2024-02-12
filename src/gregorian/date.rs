use super::GregorianGenerator;
use chinese_format::gregorian::{Date, DateBuilder, DatePattern, WeekFormat};
use std::ops::RangeInclusive;

/// The year range used when the related parameter is missing.
pub const DEFAULT_YEAR_RANGE: RangeInclusive<u16> = 1800..=2140;

/// Parameters for the random creation of [Date].
pub struct DateParams {
    /// How the date should appear - for example, [YearMonthDay](DatePattern::YearMonthDay).
    pub pattern: DatePattern,

    /// The year range, if applicable in the pattern.
    /// If set to [None], then [DEFAULT_YEAR_RANGE] is used.
    pub year_range: Option<RangeInclusive<u16>>,

    /// Applies to the date format - for example,
    /// 号 instead of 日 after the day ordinal.
    pub formal: bool,

    /// How *week* should be translated into logograms, if applicable.
    /// If set to [None], then [WeekFormat]'s default value is applied.
    pub week_format: Option<WeekFormat>,
}

impl<'a> GregorianGenerator<'a> {
    /// Generates a random [Date] using the given parameters.
    ///
    /// The date generated is always consistent in the context of the Gregorian calendar,
    /// with the exception of its [WeekDay](chinese_format::gregorian::WeekDay) part,
    /// if present - because it is created entirely at random, within its own range
    /// of validity.
    ///
    /// ```
    /// use chinese_rand::{*, gregorian::*};
    /// use chinese_format::{Variant, ChineseFormat, gregorian::{Date, DatePattern, WeekFormat}};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// let mut date: Date;
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
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
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::YearMonthDayWeekDay,
    ///     year_range: Some(2000..=2019),
    ///     formal: false,
    ///     week_format: Some(WeekFormat::XingQi)
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二零一三年五月二十三日星期一"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::YearMonthDay,
    ///     year_range: Some(2000..=2019),
    ///     formal: false,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二零一三年五月二十三日"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::YearMonth,
    ///     year_range: Some(2000..=2019),
    ///     formal: false,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二零一三年五月"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::Year,
    ///     year_range: Some(2000..=2019),
    ///     formal: true,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二零一三年"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::Year,
    ///     year_range: Some(2007..=2007),
    ///     formal: true,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二零零七年"
    /// );
    ///  
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::Month,
    ///     year_range: None,
    ///     formal: true,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "九月"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::Day,
    ///     year_range: None,
    ///     formal: true,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二十二号"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::Day,
    ///     year_range: None,
    ///     formal: false,
    ///     week_format: None
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "二十二日"
    /// );
    ///
    /// fastrand::seed(90);
    /// date = gregorian.date(DateParams {
    ///     pattern: DatePattern::WeekDay,
    ///     year_range: None,
    ///     formal: true,
    ///     week_format: Some(WeekFormat::Zhou)
    /// });
    /// assert_eq!(
    ///     date.to_chinese(Variant::Simplified),
    ///     "周四"
    /// );
    /// ```
    pub fn date(&self, params: DateParams) -> Date {
        loop {
            let mut builder = DateBuilder::new()
                .with_formal(params.formal)
                .with_week_format(params.week_format.unwrap_or_default());

            let pattern = &params.pattern;

            if pattern.has_year() {
                let actual_year_range = params.year_range.clone().unwrap_or(DEFAULT_YEAR_RANGE);

                builder = builder.with_year(self.raw_generator.u16(actual_year_range))
            }

            if pattern.has_month() {
                builder = builder.with_month(self.raw_generator.u8(1..=12));
            }

            if pattern.has_day() {
                builder = builder.with_day(self.raw_generator.u8(1..=31))
            }

            if pattern.has_week_day() {
                builder = builder.with_week_day(
                    self.raw_generator
                        .u8(0..=6)
                        .try_into()
                        .expect("Weekday valid by construction"),
                );
            }

            let date_result = builder.build();

            if let Ok(date) = date_result {
                break date;
            }
        }
    }
}
