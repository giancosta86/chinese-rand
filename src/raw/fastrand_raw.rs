use crate::RawGenerator;
use std::ops::RangeInclusive;

/// Implementation of [RawGenerator] based on [fastrand].
pub struct FastRandGenerator;

impl FastRandGenerator {
    /// Creates a new instance of the generator.
    ///
    /// ```
    /// use chinese_rand::*;
    ///
    /// //Normally, you won't set the seed, except in tests
    /// //or any other context requiring predictability
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    /// let number = generator.u128(0..=50000);
    ///     
    /// assert_eq!(number, 29466);
    /// ```
    pub fn new() -> Self {
        Self
    }
}

/// [FastRandGenerator] can also be instantiated via its [Default] trait.
///
/// ```
/// use chinese_rand::*;
///
/// fastrand::seed(90);
///
/// let generator = FastRandGenerator::default();
/// let number = generator.u128(0..=50000);
///      
/// assert_eq!(number, 29466);
/// ```
impl Default for FastRandGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RawGenerator for FastRandGenerator {
    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let number = generator.u8(0..=u8::MAX);
    /// assert_eq!(number, 177);
    /// ```
    fn u8(&self, range: RangeInclusive<u8>) -> u8 {
        fastrand::u8(range)
    }

    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let number = generator.u16(0..=u16::MAX);
    /// assert_eq!(number, 51377);
    /// ```
    fn u16(&self, range: RangeInclusive<u16>) -> u16 {
        fastrand::u16(range)
    }

    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let number = generator.u32(0..=u32::MAX);
    /// assert_eq!(number, 2982070449);
    /// ```
    fn u32(&self, range: RangeInclusive<u32>) -> u32 {
        fastrand::u32(range)
    }

    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let number = generator.u64(0..=u64::MAX);
    /// assert_eq!(number, 10871161991276185777);
    /// ```
    fn u64(&self, range: RangeInclusive<u64>) -> u64 {
        fastrand::u64(range)
    }

    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let number = generator.u128(0..=u128::MAX);
    /// assert_eq!(number, 200537543036910508246661533454648102841);
    /// ```
    fn u128(&self, range: RangeInclusive<u128>) -> u128 {
        fastrand::u128(range)
    }

    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let number = generator.i128(i128::MIN..=i128::MAX);
    /// assert_eq!(number, -139744823884027955216713073977120108615);
    /// ```
    fn i128(&self, range: RangeInclusive<i128>) -> i128 {
        fastrand::i128(range)
    }

    /// ```
    /// use chinese_rand::*;
    ///
    /// fastrand::seed(90);
    ///
    /// let generator = FastRandGenerator::new();
    ///
    /// let random_bool = generator.bool();
    /// assert_eq!(random_bool, false);
    /// ```
    fn bool(&self) -> bool {
        fastrand::bool()
    }
}
