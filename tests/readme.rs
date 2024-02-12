//
// CODE FROM README
//

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

    Ok(())
}

//
// Test
//
use speculate2::*;

speculate! {
    describe "The source code in the README file" {
        it "must compile and run" {
            main().unwrap();
        }
    }
}
