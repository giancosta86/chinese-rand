use std::error::Error;
use std::fmt::{Debug, Display};

/// When a lower bound is not acceptable.
///
/// ```
/// use chinese_rand::*;
///
/// let err = InvalidLowerBound(90);
///
/// assert_eq!(err.to_string(), "Invalid lower bound: 90");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidLowerBound<T>(pub T);

impl<T: Display> Display for InvalidLowerBound<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid lower bound: {}", self.0)
    }
}

impl<T: Display + Debug> Error for InvalidLowerBound<T> {}
