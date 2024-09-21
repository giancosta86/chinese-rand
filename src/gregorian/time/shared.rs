use crate::gregorian::GregorianGenerator;
use chinese_format::gregorian::{Hour12, Hour24, Minute, Second};

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
}
