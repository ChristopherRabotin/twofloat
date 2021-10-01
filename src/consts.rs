use crate::TwoFloat;

// Constants determined by Sollya

/// Euler's number (e)
pub const E: TwoFloat = TwoFloat {
    hi: 2.718281828459045,
    lo: 1.4456468917292502e-16,
};

/// 1/π
pub const FRAC_1_PI: TwoFloat = TwoFloat {
    hi: 0.3183098861837907,
    lo: -1.9678676675182486e-17,
};

/// 2/π
pub const FRAC_2_PI: TwoFloat = TwoFloat {
    hi: 0.6366197723675814,
    lo: -3.935735335036497e-17,
};

/// 2/sqrt(π)
pub const FRAC_2_SQRT_PI: TwoFloat = TwoFloat {
    hi: 1.1283791670955126,
    lo: 1.533545961316588e-17,
};

/// 1/sqrt(2)
pub const FRAC_1_SQRT_2: TwoFloat = TwoFloat {
    hi: 0.7071067811865476,
    lo: -4.833646656726457e-17,
};

/// π/2
pub const FRAC_PI_2: TwoFloat = TwoFloat {
    hi: 1.5707963267948966,
    lo: 6.123233995736766e-17,
};

/// π/3
pub const FRAC_PI_3: TwoFloat = TwoFloat {
    hi: 1.0471975511965979,
    lo: -1.072081766451091e-16,
};

/// π/4
pub const FRAC_PI_4: TwoFloat = TwoFloat {
    hi: 0.7853981633974483,
    lo: 3.061616997868383e-17,
};

/// π/6
pub const FRAC_PI_6: TwoFloat = TwoFloat {
    hi: 0.5235987755982989,
    lo: -5.360408832255455e-17,
};

/// π/8
pub const FRAC_PI_8: TwoFloat = TwoFloat {
    hi: 0.39269908169872414,
    lo: 1.5308084989341915e-17,
};

/// ln(2)
pub const LN_2: TwoFloat = TwoFloat {
    hi: 0.6931471805599453,
    lo: 2.3190468138462996e-17,
};

/// ln(10)
pub const LN_10: TwoFloat = TwoFloat {
    hi: 2.302585092994046,
    lo: -2.1707562233822494e-16,
};

/// log<sub>2</sub>(e)
pub const LOG2_E: TwoFloat = TwoFloat {
    hi: 1.4426950408889634,
    lo: 2.0355273740931033e-17,
};

/// log<sub>10</sub>(e)
pub const LOG10_E: TwoFloat = TwoFloat {
    hi: 0.4342944819032518,
    lo: 1.098319650216765e-17,
};

/// log<sub>10</sub>(2)
pub const LOG10_2: TwoFloat = TwoFloat {
    hi: 0.3010299956639812,
    lo: -2.8037281277851704e-18,
};

/// log<sub>2</sub>(10)
pub const LOG2_10: TwoFloat = TwoFloat {
    hi: 3.321928094887362,
    lo: 1.661617516973592e-16,
};

/// Archimedes' constant (π)
pub const PI: TwoFloat = TwoFloat {
    hi: 3.141592653589793,
    lo: 1.2246467991473532e-16,
};

/// sqrt(2)
pub const SQRT_2: TwoFloat = TwoFloat {
    hi: 1.4142135623730951,
    lo: -9.667293313452913e-17,
};

/// The full circle constant (τ)
pub const TAU: TwoFloat = TwoFloat {
    hi: 6.283185307179586,
    lo: 2.4492935982947064e-16,
};

// The trivial constants

/// The radix or base of the internal representation of `TwoFloat`. This is the same as
/// the representation in the underlying f64.
pub const RADIX: u32 = 2;

/// Number of significant digits in base 2.
pub const MANTISSA_DIGITS: u32 = 106;

/// Approximate number of significant digits in base 10.
pub const DIGITS: u32 = 31;

/// [Machine epsilon] value for `TwoFloat`.
///
/// This is the difference between `1.0` and the next largest representable number.
///
/// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
pub const EPSILON: TwoFloat = TwoFloat {
    hi: 4.93038065763132e-32,
    lo: 0.0,
}; // 2^-104

/// Smallest finite `TwoFloat` value.
pub const MIN: TwoFloat = TwoFloat {
    hi: -1.7976931348623157e308,
    lo: -9.979201547673598e291,
};

/// Smallest positive normal `TwoFloat` value.
pub const MIN_POSITIVE: TwoFloat = TwoFloat {
    hi: 2.2250738585072014e-308,
    lo: 0.0,
};

/// Largest finite `TwoFloat` value.
pub const MAX: TwoFloat = TwoFloat {
    hi: 1.7976931348623157e308,
    lo: 9.979201547673598e291,
};

/// One greater than the minimum possible normal power of 2 exponent.
pub const MIN_EXP: i32 = -1021;

/// Maximum possible power of 2 exponent.
pub const MAX_EXP: i32 = 1024;

/// Minimum possible normal power of 10 exponent.
pub const MIN_10_EXP: i32 = -307;

/// Maximum possible power of 10 exponent.
pub const MAX_10_EXP: i32 = 308;

/// Not a Number (NaN).
pub const NAN: TwoFloat = TwoFloat {
    hi: f64::NAN,
    lo: f64::NAN,
};

/// Infinity (∞).
pub const INFINITY: TwoFloat = TwoFloat {
    hi: f64::INFINITY,
    lo: 0.0,
};

/// Negative infinity (-∞).
pub const NEG_INFINITY: TwoFloat = TwoFloat {
    hi: f64::NEG_INFINITY,
    lo: 0.0,
};

/// Zero (0)
pub const ZERO: TwoFloat = TwoFloat { hi: 0.0, lo: 0.0 };

/// Negative zero (-0)
pub const NEG_ZERO: TwoFloat = TwoFloat { hi: -0.0, lo: 0.0 };

/// One (1)
pub const ONE: TwoFloat = TwoFloat { hi: 1.0, lo: 0.0 };

/// Negative one (-1)
pub const NEG_ONE: TwoFloat = TwoFloat { hi: -1.0, lo: 0.0 };

/// π/16
/// WARNING: _Not_ computed by Sollya, so 2*FRAC_PI_8 != FRAC_PI_16
pub const FRAC_PI_16: TwoFloat = TwoFloat {
    hi: 1.9634954084936207e-1,
    lo: 7.654042494670953e-18,
};

/// 3π/2
/// WARNING: _Not_ computed by Sollya, might be inconsistent!
pub const FRAC_3_PI_2: TwoFloat = TwoFloat {
    hi: 4.71238898038469e0,
    lo: 1.8369701987210292e-16,
};

/// 3π/4
/// WARNING: _Not_ computed by Sollya, might be inconsistent!
pub const FRAC_3_PI_4: TwoFloat = TwoFloat {
    hi: 2.356194490192345e0,
    lo: 9.184850993605146e-17,
};

/// 5π/4
/// WARNING: _Not_ computed by Sollya, might be inconsistent!
pub const FRAC_5_PI_4: TwoFloat = TwoFloat {
    hi: 3.9269908169872414e0,
    lo: 1.5308084989341908e-16,
};

/// 7π/4
/// WARNING: _Not_ computed by Sollya, might be inconsistent!
pub const FRAC_7_PI_4: TwoFloat = TwoFloat {
    hi: 5.497787143782138e0,
    lo: 2.143131898507869e-16,
};

#[cfg(test)]
mod tests {
    use super::{
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4,
        FRAC_PI_6, FRAC_PI_8, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, SQRT_2, TAU,
    };

    macro_rules! const_check {
        ($name:ident) => {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $name {
                use super::$name;

                #[test]
                fn valid_test() {
                    assert!($name.is_valid());
                }

                #[test]
                fn value_test() {
                    assert_eq!($name.hi, core::f64::consts::$name);
                }
            }
        };
        ($name:ident, $($names:ident),+) => {
            const_check! { $name }
            const_check! { $($names),+ }
        };
        ($($names:ident,)+) => {
            const_check! { $($names),+ }
        };
        (#[cfg($feature:tt)] $name:ident) => {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $name {
                use super::$name;

                #[test]
                fn valid_test() {
                    assert!($name.is_valid());
                }

                #[cfg($feature)]
                #[test]
                fn value_test() {
                    assert_eq!($name.hi, core::f64::consts::$name);
                }
            }
        };
        (#[cfg($feature:tt)] $name:ident, $($names:ident),+) => {
            const_check! { #[cfg($feature)] $name }
            const_check! { #[cfg($feature)] $($names),+ }
        };
        (#[cfg($feature:tt)] $($names:ident,)+) => {
            const_check! { #[cfg($feature)] $($names),+ }
        }
    }

    const_check! {
        E, FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_1_SQRT_2, FRAC_PI_2,
        FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, LN_2, LN_10, LOG2_E,
        LOG10_E, PI, SQRT_2,
    }

    const_check! {
        #[cfg(extra_log_consts)]
        LOG10_2, LOG2_10, TAU
    }
}
