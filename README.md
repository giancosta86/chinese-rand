# chinese-rand

_Random generation of data structures in Chinese, using Rust_

[![Crates.io Version](https://img.shields.io/crates/v/chinese-rand?style=for-the-badge&logo=rust&color=forestgreen)](https://crates.io/crates/chinese-rand)

This crate is designed to instantiate random instances
of the data structures implementing the `ChineseFormat` trait, made available by [chinese_format](https://crates.io/crates/chinese-format).

The core concept is the `ChineseFormatGenerator` struct,
which can be instantiated by passing an object implementing
the `RawGenerator` trait:

```rust
use chinese_format::{ChineseFormat, Fraction, Variant};
use chinese_rand::*;

fn main() -> GenericResult<()> {
    let generator = ChineseFormatGenerator::new(FastRandGenerator::new());

    // Now setting the random seed just in order to
    //predict the generated values
    fastrand::seed(90);

    let fraction: Fraction = generator.fraction(1..=10, 1..=10)?;

    let chinese = fraction.to_chinese(Variant::Simplified);

    assert_eq!(chinese, "六分之七");

    #[cfg(feature = "gregorian")]
    {
      // Now setting the random seed just in order to
      //predict the generated values
      fastrand::seed(91);

      let gregorianGenerator = generator.gregorian();

      let delta_time = gregorianGenerator.delta_time(
        gregorian::DeltaTimeParams {
          formal: true
        }
      );

      assert_eq!(
        delta_time.to_chinese(Variant::Simplified),
        "差六分五点"
      );
    }

    Ok(())
}
```

## Features

- `fastrand`: enables `FastRandGenerator`, based on [fastrand](https://crates.io/crates/fastrand). **Enabled by default**.

- `digit-sequence`: enables random generation of data types - like `Decimal` - based on [DigitSequence](https://crates.io/crates/digit-sequence).

- `currency`: enables the random generation of the data types in the `currency` module.

- `gregorian`: enables the random generation of the data types in the `gregorian` module, which is dedicated to dates and times.

  _Also enables_: `digit-sequence`.

## Crates.io

https://crates.io/crates/chinese-rand

## Documentation

https://docs.rs/chinese-rand

## License

[MIT](LICENSE)
