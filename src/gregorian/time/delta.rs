use crate::gregorian::GregorianGenerator;
use chinese_format::gregorian::{DeltaTime, Hour12, Minute};

/// Parameters for the random creation of [DeltaTime].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeltaTimeParams {
    /// Whether the generated time should have formal style.
    pub formal: bool,
}

impl<'a> GregorianGenerator<'a> {
    /// Generates a random [DeltaTime], given the provided [DeltaTimeParams].
    ///
    /// ```
    /// use chinese_rand::{*, gregorian::*};
    /// use chinese_format::{Variant, ChineseFormat, gregorian::*};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(91);
    /// let delta_time = gregorian.delta_time(DeltaTimeParams {
    ///   formal: false
    /// });
    /// assert_eq!(
    ///     delta_time.to_chinese(Variant::Simplified),
    ///     "五点差六分"
    /// );
    ///
    /// fastrand::seed(91);
    /// let delta_time = gregorian.delta_time(DeltaTimeParams {
    ///   formal: true
    /// });
    /// assert_eq!(
    ///     delta_time.to_chinese(Variant::Simplified),
    ///     "差六分五点"
    /// );
    /// ```
    ///
    /// It is interesting to note that only delta times having `差`
    /// are affected by the `formal` parameter:
    ///
    /// ```
    /// use chinese_rand::{*, gregorian::*};
    /// use chinese_format::{Variant, ChineseFormat, gregorian::*};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    /// let gregorian = generator.gregorian();
    ///
    /// fastrand::seed(90);
    /// let delta_time = gregorian.delta_time(DeltaTimeParams {
    ///   formal: false
    /// });
    /// assert_eq!(
    ///     delta_time.to_chinese(Variant::Simplified),
    ///     "九点过二十分"
    /// );
    ///
    /// fastrand::seed(90);
    /// let delta_time = gregorian.delta_time(DeltaTimeParams {
    ///   formal: true
    /// });
    /// assert_eq!(
    ///     delta_time.to_chinese(Variant::Simplified),
    ///     "九点过二十分"
    /// );
    /// ```
    pub fn delta_time(&self, params: DeltaTimeParams) -> DeltaTime {
        let hour: Hour12 = self.hour12();

        let minute: Minute = self.minute();

        DeltaTime {
            hour,
            minute,
            formal: params.formal,
        }
    }
}
