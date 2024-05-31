use super::GregorianGenerator;
use chinese_format::gregorian::{DeltaTime, Hour12, Hour24, LinearTime, Minute, Second};

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
    /// Generates a random [Hour24].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Variant, ChineseFormat};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let hour = gregorian.hour24();
    /// assert_eq!(
    ///     hour.to_chinese(Variant::Simplified),
    ///     "十六点"
    /// );
    /// ```
    pub fn hour24(&self) -> Hour24 {
        self.raw_generator
            .u8(0..=23)
            .try_into()
            .expect("Hour valid by construction")
    }

    /// Generates a random [Hour12].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Variant, ChineseFormat};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let hour = gregorian.hour12();
    /// assert_eq!(
    ///     hour.to_chinese(Variant::Simplified),
    ///     "九点"
    /// );
    /// ```
    pub fn hour12(&self) -> Hour12 {
        self.raw_generator
            .u8(1..=12)
            .try_into()
            .expect("Hour valid by construction")
    }

    /// Generates a random [Minute].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Variant, ChineseFormat};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let minute = gregorian.minute();
    /// assert_eq!(
    ///     minute.to_chinese(Variant::Simplified),
    ///     "四十一分"
    /// );
    /// ```
    pub fn minute(&self) -> Minute {
        self.raw_generator
            .u8(0..=59)
            .try_into()
            .expect("Minute valid by construction")
    }

    /// Generates a random [Second].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Variant, ChineseFormat};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let second = gregorian.second();
    /// assert_eq!(
    ///     second.to_chinese(Variant::Simplified),
    ///     "四十一秒"
    /// );
    /// ```
    pub fn second(&self) -> Second {
        self.raw_generator
            .u8(0..=59)
            .try_into()
            .expect("Second valid by construction")
    }

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
    /// let mut time: LinearTime;
    ///
    /// fastrand::seed(90);
    /// time = gregorian.linear_time(LinearTimeParams {
    ///     day_part: true,
    ///     include_second: true
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "下午四点二十分四十三秒");
    ///
    /// fastrand::seed(90);
    /// time = gregorian.linear_time(LinearTimeParams {
    ///     day_part: false,
    ///     include_second: true
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "十六点二十分四十三秒");  
    ///
    /// fastrand::seed(90);
    /// time = gregorian.linear_time(LinearTimeParams {
    ///     day_part: true,
    ///     include_second: false
    /// });
    /// assert_eq!(time.to_chinese(Variant::Simplified), "下午四点二十分");
    ///
    /// fastrand::seed(90);
    /// time = gregorian.linear_time(LinearTimeParams {
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

    /// Generates a random [DeltaTime].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Variant, ChineseFormat, gregorian::DeltaTime};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let delta_time = gregorian.delta_time();
    /// assert_eq!(
    ///     delta_time.to_chinese(Variant::Simplified),
    ///     "九点过二十分"
    /// );
    ///
    /// fastrand::seed(91);
    /// let delta_time = gregorian.delta_time();
    /// assert_eq!(
    ///     delta_time.to_chinese(Variant::Simplified),
    ///     "五点差六分"
    /// );
    /// ```
    pub fn delta_time(&self) -> DeltaTime {
        let hour: Hour12 = self.hour12();

        let minute: Minute = self.minute();

        DeltaTime { hour, minute }
    }
}
