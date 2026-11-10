use crate::stats::float::Float;

/// A "view" into the percentiles of a sample
pub struct Percentiles<A>(Box<[A]>)
where
    A: Float;

// TODO(rust-lang/rfcs#735) move this `impl` into a private percentiles module
impl<A> Percentiles<A>
where
    A: Float,
{
    /// Returns the percentile at `p`%
    ///
    /// Safety:
    ///
    /// - Make sure that `p` is in the range `[0, 100]`
    unsafe fn at_unchecked(&self, p: A) -> A {
        let _100 = A::from_usize(100);
        debug_assert!(p >= A::from_usize(0) && p <= _100);
        debug_assert!(!self.0.is_empty());
        let len = self.0.len() - 1;

        if p == _100 {
            self.0[len]
        } else {
            let rank = (p / _100) * A::from_usize(len);
            let integer = rank.floor();
            let fraction = rank - integer;
            // `integer` is the floor of `rank`, which is in `[0, len]` when `p`
            // is in `[0, 100]`, so the conversion can only fail on a broken
            // contract; fall back to the first element instead of panicking.
            let Some(n) = integer.to_usize() else {
                return A::from_usize(0);
            };
            let n = n.min(len - 1);
            let &floor = self.0.get_unchecked(n);
            let &ceiling = self.0.get_unchecked(n + 1);

            floor + (ceiling - floor) * fraction
        }
    }

    /// Returns the percentile at `p`%
    ///
    /// # Panics
    ///
    /// Panics if `p` is outside the closed `[0, 100]` range
    pub fn at(&self, p: A) -> A {
        let _0 = A::from_usize(0);
        let _100 = A::from_usize(100);

        assert!(p >= _0 && p <= _100);
        assert!(!self.0.is_empty());

        unsafe { self.at_unchecked(p) }
    }

    /// Returns the interquartile range
    pub fn iqr(&self) -> A {
        let q1 = self.at(A::from_usize(25));
        let q3 = self.at(A::from_usize(75));

        q3 - q1
    }

    /// Returns the 50th percentile
    pub fn median(&self) -> A {
        self.at(A::from_usize(50))
    }

    /// Returns the 25th, 50th and 75th percentiles
    pub fn quartiles(&self) -> (A, A, A) {
        (
            self.at(A::from_usize(25)),
            self.at(A::from_usize(50)),
            self.at(A::from_usize(75)),
        )
    }
}
