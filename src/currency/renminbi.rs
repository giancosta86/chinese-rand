use crate::ChineseFormatGenerator;
use chinese_format::currency::{CurrencyStyle, RenminbiCurrency, RenminbiCurrencyBuilder};
use std::ops::RangeInclusive;

/// Parameters for the random creation of [RenminbiCurrency].
///
/// **Required feature**: `currency`.
pub struct RenminbiParams {
    /// The style of the generated currency - for example, financial.
    pub style: CurrencyStyle,

    /// The range of the integer part.
    pub yuan_range: RangeInclusive<u64>,

    /// Whether the `角` part should be generated.
    pub include_dimes: bool,

    /// Whether the `分` part should be generated.
    pub include_cents: bool,
}

impl ChineseFormatGenerator {
    /// Creates a random instance of [RenminbiCurrency].
    ///
    /// ```
    /// use chinese_rand::*;
    /// use chinese_format::{Variant, ChineseFormat, currency::{RenminbiCurrency, CurrencyStyle}};
    ///
    /// let raw_generator = FastRandGenerator::new();
    /// let generator = ChineseFormatGenerator::new(raw_generator);
    ///
    /// let mut currency: RenminbiCurrency;
    ///
    /// fastrand::seed(90);
    /// currency = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: true },
    ///     yuan_range: 0..=500,
    ///     include_dimes: true,
    ///     include_cents: true
    /// });
    /// assert_eq!(
    ///     currency.to_chinese(Variant::Simplified),
    ///     "二百九十五元三角七分"
    /// );
    ///
    /// fastrand::seed(90);
    /// currency = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: true },
    ///     yuan_range: 0..=500,
    ///     include_dimes: false,
    ///     include_cents: false
    /// });
    /// assert_eq!(
    ///     currency.to_chinese(Variant::Simplified),
    ///     "二百九十五元"
    /// );
    ///
    /// fastrand::seed(90);
    /// currency = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: true },
    ///     yuan_range: 0..=500,
    ///     include_dimes: true,
    ///     include_cents: false
    /// });
    /// assert_eq!(
    ///     currency.to_chinese(Variant::Simplified),
    ///     "二百九十五元三角"
    /// );
    ///
    /// fastrand::seed(90);
    /// currency = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: true },
    ///     yuan_range: 0..=500,
    ///     include_dimes: false,
    ///     include_cents: true
    /// });
    /// assert_eq!(
    ///     currency.to_chinese(Variant::Simplified),
    ///     "二百九十五元三分"
    /// );
    ///
    /// fastrand::seed(90);
    /// currency = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: false },
    ///     yuan_range: 0..=500,
    ///     include_dimes: true,
    ///     include_cents: true
    /// });
    /// assert_eq!(
    ///     currency.to_chinese(Variant::Simplified),
    ///     "二百九十五块三毛七分"
    /// );
    ///
    /// fastrand::seed(90);
    /// currency = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Financial,
    ///     yuan_range: 0..=500,
    ///     include_dimes: true,
    ///     include_cents: true
    /// });
    /// assert_eq!(
    ///     currency.to_chinese(Variant::Simplified),
    ///     "贰佰玖拾伍元叁角柒分整"
    /// );
    ///
    /// fastrand::seed(90);
    /// let fixed_yuan = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: true },
    ///     yuan_range: 73..=73,
    ///     include_dimes: true,
    ///     include_cents: true
    /// });
    /// assert_eq!(
    ///     fixed_yuan.to_chinese(Variant::Simplified),
    ///     "七十三元三角七分"
    /// );
    ///
    /// fastrand::seed(90);
    /// let zero = generator.renminbi(RenminbiParams {
    ///     style: CurrencyStyle::Everyday { formal: true },
    ///     yuan_range: 0..=0,
    ///     include_dimes: false,
    ///     include_cents: false
    /// });
    /// assert_eq!(
    ///     zero.to_chinese(Variant::Simplified),
    ///     "零元"
    /// );
    /// ```
    ///
    /// **Required feature**: `currency`.
    pub fn renminbi(&self, params: RenminbiParams) -> RenminbiCurrency {
        let mut builder = RenminbiCurrencyBuilder::new()
            .with_style(params.style)
            .with_yuan(self.raw_generator.u64(params.yuan_range));

        if params.include_dimes {
            builder = builder.with_dimes(self.raw_generator.u8(0..=9))
        }

        if params.include_cents {
            builder = builder.with_cents(self.raw_generator.u8(0..=9))
        }

        builder
            .build()
            .expect("Renminbi params correct by construction")
    }
}
