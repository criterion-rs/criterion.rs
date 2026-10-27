//! Float trait

use num_traits::float;

/// This is an extension of `num_traits::float::Float` that adds infallible
/// numeric conversions and Sync + Send. Once `num_traits` has these features
/// this can be removed.
pub trait Float: float::Float + Sync + Send {
    /// Converts a `usize` to this float type. Lossy for values larger than
    /// the float's mantissa can represent, but never panics.
    fn from_usize(n: usize) -> Self;

    /// Converts an `f64` to this float type. Lossy for `f32`, but never panics.
    fn from_f64(x: f64) -> Self;

    /// Converts this float to a `usize`, returning `None` if the value is
    /// negative, not finite, or too large to fit.
    fn to_usize(self) -> Option<usize>;
}

impl Float for f32 {
    #[inline]
    fn from_usize(n: usize) -> Self {
        n as f32
    }

    #[inline]
    fn from_f64(x: f64) -> Self {
        x as f32
    }

    #[inline]
    fn to_usize(self) -> Option<usize> {
        if self.is_finite() && self >= 0.0 && self <= (usize::MAX as f32) {
            Some(self as usize)
        } else {
            None
        }
    }
}

impl Float for f64 {
    #[inline]
    fn from_usize(n: usize) -> Self {
        n as f64
    }

    #[inline]
    fn from_f64(x: f64) -> Self {
        x
    }

    #[inline]
    fn to_usize(self) -> Option<usize> {
        if self.is_finite() && self >= 0.0 && self <= (usize::MAX as f64) {
            Some(self as usize)
        } else {
            None
        }
    }
}
