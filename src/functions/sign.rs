use crate::TwoFloat;
use std::num::FpCategory;

impl TwoFloat {
    /// Returns the absolute value root of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1.0e-300).abs();
    /// let b = TwoFloat::new_add(-1.0, 1.0e-300).abs();
    ///
    /// assert_eq!(a, TwoFloat::new_add(1.0, 1.0e-300));
    /// assert_eq!(b, TwoFloat::new_add(1.0, -1.0e-300));
    pub fn abs(self) -> Self {
        if self.hi > 0.0
            || (self.hi == 0.0 && self.hi.is_sign_positive() && self.lo.is_sign_positive())
        {
            self
        } else {
            -self
        }
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(0.0, 0.0).is_sign_positive();
    /// let b = TwoFloat::new_add(1.0, 1.0e-300).is_sign_positive();
    /// let c = TwoFloat::new_add(-1.0, 1.0e-300).is_sign_positive();
    ///
    /// assert!(a);
    /// assert!(b);
    /// assert!(!c);
    pub fn is_sign_positive(&self) -> bool {
        self.hi.is_sign_positive()
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(-1.0, 1.0e-300).is_sign_negative();
    /// let b = TwoFloat::new_add(0.0, 0.0).is_sign_negative();
    /// let c = TwoFloat::new_add(1.0, 1.0e-300).is_sign_negative();
    ///
    /// assert!(a);
    /// assert!(!b);
    /// assert!(!c);
    pub fn is_sign_negative(&self) -> bool {
        self.hi.is_sign_negative()
    }

    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same,
    /// otherwise equal to `-self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(-1.0, 1.0e-200);
    /// let b = TwoFloat::new_add(1.0, 0.3);
    /// let c = a.copysign(b);
    ///
    /// assert_eq!(c, -a);
    pub fn copysign(self, sign: Self) -> Self {
        if self.is_sign_positive() == sign.is_sign_positive() {
            self
        } else {
            -self
        }
    }

    /// Returns a number that represents the sign of the value.
    ///
    /// * `1.0` if the number is positive or `+0.0`
    /// * `-1.0` if the number is negative or `-0.0`
    /// * Invalid value otherwise
    ///
    /// # Examples
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(3.5);
    /// let b = TwoFloat::from(-0.0);
    ///
    /// assert_eq!(a.signum(), 1.0);
    /// assert_eq!(b.signum(), -1.0);
    pub fn signum(self) -> Self {
        if self.is_valid() {
            if self.is_sign_positive() {
                Self::from(1.0)
            } else {
                Self::from(-1.0)
            }
        } else {
            Self::NAN
        }
    }

    /// Returns the floating point category of the `Double`.
    ///
    /// The possible return values are the members of [`FpCategory`], as follows:
    ///
    /// * `FpCategory::Zero` if the number is ±0;
    /// * `FpCategory::Infinite` if the number is ±∞;
    /// * `FpCategory::Nan` if the number is not a number;
    /// * `FpCategory::Subnormal` if the number is ±[`MIN_POSITIVE`] (numbers this small can
    ///     be represented, but they lose some accuracy);
    /// * `FpCategory::Normal` if the number is anything else.
    ///
    /// # Examples
    /// ```
    /// # use twofloat::TwoFloat;
    /// use std::num::FpCategory;
    ///
    /// let num = dd!(12.4);
    /// let inf = Double::INFINITY;
    ///
    /// assert!(num.classify() == FpCategory::Normal);
    /// assert!(inf.classify() == FpCategory::Infinite);
    /// ```
    ///
    /// [`FpCategory`]: https://doc.rust-lang.org/std/num/enum.FpCategory.html
    /// [`MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    pub fn classify(self) -> FpCategory {
        self.hi.classify()
    }

    /// Returns `true` if the `Double` is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// # Examples
    /// ```
    /// # use twofloat::TwoFloat;
    /// let min = Double::MIN_POSITIVE;
    /// let max = Double::MAX;
    /// let lower = dd!(1e-308);
    /// let zero = Double::ZERO;
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!Double::NAN.is_normal());
    /// assert!(!Double::INFINITY.is_normal());
    /// // Values between `0` and `MIN_POSITIVE` are subnormal.
    /// assert!(!lower.is_normal());
    /// ```
    pub fn is_normal(self) -> bool {
        self.classify() == FpCategory::Normal
    }

    /// Returns `true` if the `Double` is either positive or negative zero.
    ///
    /// # Examples
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(Double::ZERO.is_zero());
    /// assert!(Double::NEG_ZERO.is_zero());
    /// assert!(!Double::PI.is_zero());
    /// ```
    pub fn is_zero(self) -> bool {
        self.hi.abs() < std::f64::EPSILON
    }

    /// Returns `true` if the `TwoFloat` has an absolute value of less than [`MIN_POSITIVE`].
    ///
    /// Numbers this small can be represented by floating point numbers, but they are not as
    /// accurate. This inaccuracy is inherent in the IEEE-754 format for 64-bit numbers;
    /// making a double-double out of an inaccurate number means the double-double is also
    /// going to be inaccurate.
    ///
    /// # Examples
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(!TwoFloat::PI.is_subnormal());
    /// assert!(dd!(1e-308).is_subnormal());
    /// ```
    ///
    /// [`MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    pub fn is_subnormal(self) -> bool {
        self.hi.classify() == FpCategory::Subnormal
    }

    /// Returns `true` if the `TwoFloat` is positive or negative infinity.
    ///
    /// # Examples
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(TwoFloat::INFINITY.is_infinite());
    /// assert!(TwoFloat::NEG_INFINITY.is_infinite());
    /// assert!(!TwoFloat::NAN.is_infinite());
    /// assert!(!dd!(7.0).is_infinite());
    /// ```
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.hi.is_infinite()
    }

    /// Returns `true` if the `TwoFloat` is neither infinite nor `NaN`.
    ///
    /// # Examples
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(!TwoFloat::INFINITY.is_finite());
    /// assert!(!TwoFloat::NEG_INFINITY.is_finite());
    /// assert!(!TwoFloat::NAN.is_finite());
    /// assert!(dd!(7.0).is_finite());
    /// ```
    #[inline]
    pub fn is_finite(self) -> bool {
        self.hi.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use crate::TwoFloat;

    #[test]
    fn abs_test() {
        assert_eq!(
            TwoFloat { hi: 0.0, lo: 0.0 }.abs(),
            TwoFloat { hi: 0.0, lo: 0.0 }
        );
        assert!(TwoFloat { hi: 0.0, lo: -0.0 }.abs().lo.is_sign_positive());
        assert!(TwoFloat { hi: -0.0, lo: 0.0 }.abs().lo.is_sign_negative());
    }

    #[test]
    fn is_sign_positive_test() {
        assert!(TwoFloat { hi: 0.0, lo: -0.0 }.is_sign_positive());
        assert!(!TwoFloat { hi: -0.0, lo: 0.0 }.is_sign_positive());
        assert!(!TwoFloat { hi: -0.0, lo: -0.0 }.is_sign_positive());
        assert!(TwoFloat {
            hi: 1.0,
            lo: -1e-300
        }
        .is_sign_positive());
        assert!(!TwoFloat {
            hi: -1.0,
            lo: -1e-300
        }
        .is_sign_positive());
    }

    #[test]
    fn is_sign_negative_test() {
        assert!(!TwoFloat { hi: 0.0, lo: -0.0 }.is_sign_negative());
        assert!(TwoFloat { hi: -0.0, lo: 0.0 }.is_sign_negative());
        assert!(TwoFloat { hi: -0.0, lo: -0.0 }.is_sign_negative());
        assert!(!TwoFloat {
            hi: 1.0,
            lo: -1e-300
        }
        .is_sign_negative());
        assert!(TwoFloat {
            hi: -1.0,
            lo: -1e-300
        }
        .is_sign_negative());
    }
}
