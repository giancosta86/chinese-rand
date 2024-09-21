use crate::gregorian::GregorianGenerator;
use chinese_format::gregorian::{Hour24, LinearTime, Minute, Second};

/// Parameters for the random creation of [LinearTime].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinearTimeParams {
    /// If set to `true`, the result will include the day part,
    /// plus a 12-hour format; otherwise, the 24-hour format applies.
    pub day_part: bool,

    /// If set to `true`, the *second* part will be generated.
    pub include_second: bool,
}

impl<'a> GregorianGenerator<'a> {
    /// Generates a random [LinearTime], given the provided [LinearTimeParams].
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
    /// let time: LinearTime = gregorian.linear_time(LinearTimeParams {
    ///     day_part: true,
    ///     include_second: true
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "下午四点二十分四十三秒");
    ///
    /// fastrand::seed(90);
    /// let time: LinearTime = gregorian.linear_time(LinearTimeParams {
    ///     day_part: false,
    ///     include_second: true
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "十六点二十分四十三秒");
    ///
    /// fastrand::seed(90);
    /// let time: LinearTime = gregorian.linear_time(LinearTimeParams {
    ///     day_part: true,
    ///     include_second: false
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "下午四点二十分");
    ///
    /// fastrand::seed(90);
    /// let time: LinearTime = gregorian.linear_time(LinearTimeParams {
    ///     day_part: false,
    ///     include_second: false
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "十六点二十分");
    /// ```
    pub fn linear_time(&self, params: LinearTimeParams) -> LinearTime {
        let hour: Hour24 = self.hour24();

        let minute: Minute = self.minute();

        let second: Option<Second> = if params.include_second {
            Some(self.second())
        } else {
            None
        };

        LinearTime {
            day_part: params.day_part,
            hour,
            minute,
            second,
        }
    }
}
