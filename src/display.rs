// Copied from the MIT licensed qd library by Thomas J. Otterson
// https://github.com/Barandis/qd/blob/0fb276d70346f11f4b2a5b30568d8a26d0dd88df/src/double/display.rs

use crate::display_helpers as d;
use crate::TwoFloat;
use std::char;
use std::fmt::{Debug, Display, Formatter, LowerExp, Result, UpperExp};

const TEN: TwoFloat = TwoFloat { hi: 10.0, lo: 0.0 };
const MAX_ACCURACY: usize = 31;

impl Display for TwoFloat {
    /// Formats a `TwoFloat` for display.
    ///
    /// All formatting options that are shown in [`std::fmt`] are supported *except* for
    /// ones that are typically meant only for integers (hexadecimal, binary, octal, and
    /// pointer formats). Because of this, the "alternate" (`#`) flag is only recognized
    /// along with `?`, pretty-printing the `Debug` output.
    ///
    /// By default, `TwoFloat`s are printed with 31 digits but drop trailing zeros.
    ///
    /// This function also provides the formatting for [`to_string`], which renders the
    /// `TwoFloat` as if formatted with an empty format specifier (`"{}"`).
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, TwoFloat};
    /// assert!(format!("{}", dd!(1.5)) == "1.5");
    ///
    /// // The next digit in π is 0, which is why it's one digit shorter than e
    /// assert!(format!("{}", PI) == "3.14159265358979323846264338328");
    /// assert!(format!("{}", TwoFloat::E) == "2.718281828459045235360287471353");
    ///
    /// // to_string renders as if formatted with "{}"
    /// assert!(PI.to_string() == "3.14159265358979323846264338328");
    ///
    /// // debug
    /// assert!(format!("{:?}", PI) ==
    ///     "TwoFloat(3.141592653589793e0, 1.2246467991473532e-16)");
    /// assert!(format!("{:#?}", PI) ==
    /// "TwoFloat(
    ///     3.141592653589793e0,
    ///     1.2246467991473532e-16
    /// )");
    ///
    /// // precision and exponents
    /// let value = dd!(0.016_777_216);
    /// assert!(format!("{:.0}", value) == "0");
    /// assert!(format!("{:.5}", value) == "0.01678");
    /// assert!(format!("{:.12}", value) == "0.016777216000");
    /// assert!(format!("{:.3e}", value) == "1.678e-2");
    /// assert!(format!("{:.*e}", 3, value) == "1.678e-2");
    /// assert!(format!("{0:.1$E}", value, 4) == "1.6777E-2");
    /// assert!(format!("{:.prec$E}", value, prec = 10) == "1.6777216000E-2");
    ///
    /// // width, alignment, and fill
    /// let value = dd!(123_456);
    /// assert!(format!("{:10}", value) == "    123456"); // right-align is the default
    /// assert!(format!("{:>10}", value) == "    123456");
    /// assert!(format!("{:<10}", value) == "123456    ");
    /// assert!(format!("{:^10}", value) == "  123456  ");
    /// assert!(format!("{:0>10}", value) == "0000123456");
    /// assert!(format!("{:*<10}", value) == "123456****");
    /// assert!(format!("{:'^10}", value) == "''123456''");
    ///
    /// // plus sign and sign-aware zero fill
    /// let value = dd!(123_456);
    /// assert!(format!("{:+}", value) == "+123456");
    /// assert!(format!("{:0>10}", -value) == "000-123456");
    /// assert!(format!("{:010}", -value) == "-000123456");
    /// assert!(format!("{:+012e}", value) == "+001.23456e5");
    /// ```
    ///
    /// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
    /// [`to_string`]: #tymethod.to_string
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            d::push_nan(&mut result);
        } else if self.is_infinite() {
            d::push_inf(&mut result);
        } else if self.is_zero() {
            d::push_zero(&mut result, f);
        } else {
            push_digits_fixed(&mut result, self, f);
        }
        d::align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl LowerExp for TwoFloat {
    /// Formats a `TwoFloat` for display when the "`e`" formatting option is specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            d::push_nan(&mut result);
        } else if self.is_infinite() {
            d::push_inf(&mut result);
        } else if self.is_zero() {
            d::push_zero(&mut result, f);
        } else {
            push_digits_exp(&mut result, self, f);
        }

        if self.is_finite() {
            let exp = if self.is_zero() {
                0
            } else {
                self.hi.abs().log10().floor() as i32
            };
            d::push_exp(&mut result, 'e', exp)
        }

        d::align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl UpperExp for TwoFloat {
    /// Formats a `TwoFloat` for display when the "`E`" formatting option is specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            d::push_nan(&mut result);
        } else if self.is_infinite() {
            d::push_inf(&mut result);
        } else if self.is_zero() {
            d::push_zero(&mut result, f);
        } else {
            push_digits_exp(&mut result, self, f);
        }

        if self.is_finite() {
            let exp = if self.is_zero() {
                0
            } else {
                self.hi.abs().log10().floor() as i32
            };
            d::push_exp(&mut result, 'E', exp)
        }

        d::align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl Debug for TwoFloat {
    /// Formats a `TwoFloat` for display when the "`?`" formatting option is specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let alt = f.alternate();
        let mut str = String::from("TwoFloat(");
        if alt {
            str.push_str("\n    ");
        }
        str.push_str(format!("{:e},", self.hi).as_str());
        if alt {
            str.push_str("\n    ");
        } else {
            str.push(' ');
        }
        str.push_str(format!("{:e}", self.lo).as_str());
        if alt {
            str.push('\n');
        }
        str.push(')');
        write!(f, "{}", str)
    }
}

fn push_sign(chars: &mut Vec<char>, value: &TwoFloat, f: &Formatter) -> bool {
    if value.is_sign_negative() {
        chars.push('-');
        true
    } else if f.sign_plus() {
        chars.push('+');
        true
    } else {
        false
    }
}

fn push_digits_fixed(chars: &mut Vec<char>, value: &TwoFloat, f: &mut Formatter) {
    let value = value.abs();
    let exp = value.hi.log10().floor() as i32;
    let prec = f.precision();

    let mut digits = extract_digits(&value, exp);
    d::adjust_zeros(&mut digits, exp);
    d::adjust_prec(&mut digits, exp, prec);

    chars.append(&mut d::place_decimal(digits, exp));
}

fn push_digits_exp(chars: &mut Vec<char>, value: &TwoFloat, f: &mut Formatter) {
    let value = value.abs();
    let exp = value.hi.log10().floor() as i32;
    let prec = f.precision();

    let mut digits = extract_digits(&value, exp);
    d::adjust_zeros(&mut digits, 0);
    d::adjust_prec(&mut digits, 0, prec);

    chars.append(&mut d::place_decimal(digits, 0));
}

// Extracts the decimal digits of `value` into an array of unsigned integers.
//
// This function assumes that `value` is positive. Zero and non-finite values are handled
// before we get to this function, and the sign is already pushed to the output vector. With
// that assumption, this function will return a vector of numbers from 0-9 - digits.
fn extract_digits(value: &TwoFloat, exp: i32) -> Vec<u8> {
    // Normalize the number to have an exponent of 0 (i.e., one digit before the decimal
    // point). We don't actually otherwise need the exponent in this function, as all we're
    // doing is parsing digits from the mantissa. This normalization makes the math involved
    // much faster. It also ensures that really large numbers don't overflow on
    // multiplication by ten.
    let divisor = TEN.powi(exp);

    let mut value = value / divisor;
    let mut digits = vec![];

    for _ in 0..(MAX_ACCURACY + 1) {
        let digit = value.hi.trunc();

        value -= TwoFloat { hi: digit, lo: 0.0 };
        value *= TEN;

        digits.push(digit as u8);
    }

    // We will not record digits after the 323rd (308 for the largest negative exponent,
    // plus 15 digits for the `f64` accurate width) decimal place; since the `f64`s that
    // make up `TwoFloat`s can't do that, it's just garbage after that point.
    //
    // The 324 is the 323 digits plus the one needed because the first digit of the number
    // is at position exp + 1 in a negative exponent number.
    //
    // If this isn't an issue, we still truncate by one because we produced an extra digit
    // for rounding.
    d::round_and_trunc(&mut digits, (324 + exp).min(MAX_ACCURACY as i32) as usize);

    digits
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use crate::dd;
    use std::str::FromStr;

    // debug tests
    test_all_eq!(
        debug_zero:
            "TwoFloat(0e0, 0e0)",
            format!("{:?}", ZERO);
        debug_pi:
            "TwoFloat(3.141592653589793e0, 1.2246467991473532e-16)",
            format!("{:?}", PI);
        debug_alt_zero:
            "TwoFloat(\n    0e0,\n    0e0\n)",
            format!("{:#?}", ZERO);
        debug_alt_pi:
            "TwoFloat(\n    3.141592653589793e0,\n    1.2246467991473532e-16\n)",
            format!("{:#?}", PI);
    );

    // special number tests
    test_all_eq!(
        nan:
            "NaN",
            format!("{}", NAN);
        neg_nan:
            "-NaN",
            format!("{}", -NAN);
        plus_nan:
            "+NaN",
            format!("{:+}", NAN);
        plus_neg_nan:
            "-NaN",
            format!("{:+}", -NAN);
        inf:
            "inf",
            format!("{}", INFINITY);
        neg_inf:
            "-inf",
            format!("{}", NEG_INFINITY);
        plus_inf:
            "+inf",
            format!("{:+}", INFINITY);
        plus_neg_inf:
            "-inf",
            format!("{:+}", NEG_INFINITY);
        zero:
            "0",
            format!("{}", ZERO);
        neg_zero:
            "-0",
            format!("{}", NEG_ZERO);
        plus_zero:
            "+0",
            format!("{:+}", ZERO);
        plus_neg_zero:
            "-0",
            format!("{:+}", NEG_ZERO);
    );

    // Basic fixed number tests
    test_all_eq!(
        pi:
            "3.14159265358979323846264338328",
            format!("{}", PI);
        neg_pi:
            "-3.14159265358979323846264338328",
            format!("{}", -PI);
        plus_pi:
            "+3.14159265358979323846264338328",
            format!("{:+}", PI);
        plus_neg_pi:
            "-3.14159265358979323846264338328",
            format!("{:+}", -PI);

        ln2:
            "0.6931471805599453094172321214582",
            format!("{}", LN_2);
        neg_ln2:
            "-0.6931471805599453094172321214582",
            format!("{}", -LN_2);
        plus_ln2:
            "+0.6931471805599453094172321214582",
            format!("{:+}", LN_2);
        plus_neg_ln2:
            "-0.6931471805599453094172321214582",
            format!("{:+}", -LN_2);

        ln2_100:
            "0.006931471805599453094172321214582",
            format!("{}", LN_2 / dd!(100));
        neg_ln2_100:
            "-0.006931471805599453094172321214582",
            format!("{}", -LN_2 / dd!(100));
        plus_ln2_100:
            "+0.006931471805599453094172321214582",
            format!("{:+}", LN_2 / dd!(100));
        plus_neg_ln2_100:
            "-0.006931471805599453094172321214582",
            format!("{:+}", -LN_2 / dd!(100));

        large:
            format!("1234567890123456789{}", "0".repeat(290)),
            format!("{}", dd!("1.234567890123456789e308"));
        neg_large:
            format!("-1234567890123456789{}", "0".repeat(290)),
            format!("{}", dd!("-1.234567890123456789e308"));
        plus_large:
            format!("+1234567890123456789{}", "0".repeat(290)),
            format!("{:+}", dd!("1.234567890123456789e308"));

        small:
            format!("0.{}1234567890123457", "0".repeat(307)),
            format!("{}", dd!("1.234567890123456789e-308"));
        neg_small:
            format!("-0.{}1234567890123457", "0".repeat(307)),
            format!("{}", dd!("-1.234567890123456789e-308"));
        plus_small:
            format!("+0.{}1234567890123457", "0".repeat(307)),
            format!("{:+}", dd!("1.234567890123456789e-308"));
    );

    // precision tests
    test_all_eq!(
        prec_zero:
            "0.0000",
            format!("{:.4}", ZERO);
        prec_neg_zero:
            "-0.0",
            format!("{:.1}", NEG_ZERO);
        prec_plus_zero:
            "+0.000000000000",
            format!("{:+.12}", ZERO);
        prec_plus_neg_zero:
            "-0.000000",
            format!("{:+.6}", NEG_ZERO);

        pi_prec_5:
            "3.14159",
            format!("{:.5}", PI);
        pi_prec_7:
            "3.1415927",
            format!("{:.7}", PI);
        pi_prec_20:
            "3.14159265358979323846",
            format!("{:.20}", PI);
        pi_prec_40:
            "3.1415926535897932384626433832800000000000",
            format!("{:.40}", PI);
        pi_prec_0:
            "3",
            format!("{:.0}", PI);
        neg_pi_prec_40:
            "-3.1415926535897932384626433832800000000000",
            format!("{:.40}", -PI);
        neg_pi_prec_0:
            "-3",
            format!("{:.0}", -PI);
        plus_pi_prec_40:
            "+3.1415926535897932384626433832800000000000",
            format!("{:+.40}", PI);
        plus_pi_prec_0:
            "+3",
            format!("{:+.0}", PI);


        ln2_prec_5:
            "0.69315",
            format!("{:.5}", LN_2);
        ln2_prec_7:
            "0.6931472",
            format!("{:.7}", LN_2);
        ln2_prec_20:
            "0.69314718055994530942",
            format!("{:.20}", LN_2);
        ln2_prec_40:
            "0.6931471805599453094172321214582000000000",
            format!("{:.40}", LN_2);
        ln2_prec_0:
            "1",
            format!("{:.0}", LN_2);
        neg_ln2_prec_40:
            "-0.6931471805599453094172321214582000000000",
            format!("{:.40}", -LN_2);
        neg_ln2_prec_0:
            "-1",
            format!("{:.0}", -LN_2);
        plus_ln2_prec_40:
            "+0.6931471805599453094172321214582000000000",
            format!("{:+.40}", LN_2);
        plus_ln2_prec_0:
            "+1",
            format!("{:+.0}", LN_2);

        ln2_100_prec_5:
            "0.00693",
            format!("{:.5}", LN_2 / dd!(100));
        ln2_100_prec_7:
            "0.0069315",
            format!("{:.7}", LN_2 / dd!(100));
        ln2_100_prec_20:
            "0.00693147180559945309",
            format!("{:.20}", LN_2 / dd!(100));
        ln2_100_prec_40:
            "0.0069314718055994530941723212145820000000",
            format!("{:.40}", LN_2 / dd!(100));
        ln2_100_prec_0:
            "0",
            format!("{:.0}", LN_2 / dd!(100));
        neg_ln2_100_prec_40:
            "-0.0069314718055994530941723212145820000000",
            format!("{:.40}", -LN_2 / dd!(100));
        neg_ln2_100_prec_0:
            "-0",
            format!("{:.0}", -LN_2 / dd!(100));
        plus_ln2_100_prec_40:
            "+0.0069314718055994530941723212145820000000",
            format!("{:+.40}", LN_2 / dd!(100));
        plus_ln2_100_prec_0:
            "+0",
            format!("{:+.0}", LN_2 / dd!(100));


        large_prec_0:
            format!("1234567890123456789{}", "0".repeat(290)),
            format!("{:.0}", dd!("1.234567890123456789e308"));
        large_prec_8:
            format!("1234567890123456789{}.00000000", "0".repeat(290)),
            format!("{:.8}", dd!("1.234567890123456789e308"));


        small_prec_0:
            "0",
            format!("{:.0}", dd!("1.234567890123456789e-308"));
        small_prec_8:
            "0.00000000",
            format!("{:.8}", dd!("1.234567890123456789e-308"));
        small_prec_316:
            format!("0.{}123456789", "0".repeat(307)),
            format!("{:.316}", dd!("1.234567890123456789e-308"));
    );

    // width tests (default right align)
    test_all_eq!(
        zero_width:
            "         0",
            format!("{:10}", ZERO);
        neg_zero_width:
            "        -0",
            format!("{:10}", NEG_ZERO);
        inf_width:
            "       inf",
            format!("{:10}", INFINITY);
        neg_inf_width:
            "      -inf",
            format!("{:10}", NEG_INFINITY);
        nan_width:
            "       NaN",
            format!("{:10}", NAN);
        one_width:
            "         1",
            format!("{:10}", ONE);
        neg_one_width:
            "        -1",
            format!("{:10}", NEG_ONE);
        plus_one_width:
            "        +1",
            format!("{:+10}", ONE);
        pi_width:
            "3.14159265358979323846264338328",
            format!("{:10}", PI);
        pi_width_prec:
            "   3.14159",
            format!("{:10.5}", PI);
    );

    // right-align tests
    test_all_eq!(
        zero_right:
            "         0",
            format!("{:>10}", ZERO);
        neg_zero_right:
            "        -0",
            format!("{:>10}", NEG_ZERO);
        inf_right:
            "       inf",
            format!("{:>10}", INFINITY);
        neg_inf_right:
            "      -inf",
            format!("{:>10}", NEG_INFINITY);
        nan_right:
            "       NaN",
            format!("{:>10}", NAN);
        one_right:
            "         1",
            format!("{:>10}", ONE);
        neg_one_right:
            "        -1",
            format!("{:>10}", NEG_ONE);
        plus_one_right:
            "        +1",
            format!("{:>+10}", ONE);
        pi_right:
            "3.14159265358979323846264338328",
            format!("{:>10}", PI);
        pi_right_prec:
            "   3.14159",
            format!("{:>10.5}", PI);
    );

    // left-align tests
    test_all_eq!(
        zero_left:
            "0         ",
            format!("{:<10}", ZERO);
        neg_zero_left:
            "-0        ",
            format!("{:<10}", NEG_ZERO);
        inf_left:
            "inf       ",
            format!("{:<10}", INFINITY);
        neg_inf_left:
            "-inf      ",
            format!("{:<10}", NEG_INFINITY);
        nan_left:
            "NaN       ",
            format!("{:<10}", NAN);
        one_left:
            "1         ",
            format!("{:<10}", ONE);
        neg_one_left:
            "-1        ",
            format!("{:<10}", NEG_ONE);
        plus_one_left:
            "+1        ",
            format!("{:<+10}", ONE);
        pi_left:
            "3.14159265358979323846264338328",
            format!("{:<10}", PI);
        pi_left_prec:
            "3.14159   ",
            format!("{:<10.5}", PI);
    );

    // center-align tests
    test_all_eq!(
        zero_center:
            "    0     ",
            format!("{:^10}", ZERO);
        neg_zero_center:
            "    -0    ",
            format!("{:^10}", NEG_ZERO);
        inf_center:
            "   inf    ",
            format!("{:^10}", INFINITY);
        neg_inf_center:
            "   -inf   ",
            format!("{:^10}", NEG_INFINITY);
        nan_center:
            "   NaN    ",
            format!("{:^10}", NAN);
        one_center:
            "    1     ",
            format!("{:^10}", ONE);
        neg_one_center:
            "    -1    ",
            format!("{:^10}", NEG_ONE);
        plus_one_center:
            "    +1    ",
            format!("{:^+10}", ONE);
        pi_center:
            "3.14159265358979323846264338328",
            format!("{:^10}", PI);
        pi_center_prec:
            " 3.14159  ",
            format!("{:^10.5}", PI);
    );

    // fill tests
    test_all_eq!(
        zero_fill:
            "_________0",
            format!("{:_>10}", ZERO);
        neg_zero_fill:
            "-0________",
            format!("{:_<10}", NEG_ZERO);
        inf_fill:
            "___inf____",
            format!("{:_^10}", INFINITY);
        neg_inf_fill:
            "______-inf",
            format!("{:_>10}", NEG_INFINITY);
        nan_fill:
            "NaN_______",
            format!("{:_<10}", NAN);
        one_fill:
            "____1_____",
            format!("{:_^10}", ONE);
        neg_one_fill:
            "________-1",
            format!("{:_>10}", NEG_ONE);
        plus_one_fill:
            "+1________",
            format!("{:_<+10}", ONE);
        pi_fill:
            "3.14159265358979323846264338328",
            format!("{:_^10}", PI);
        pi_fill_prec:
            "___3.14159",
            format!("{:_>10.5}", PI);
    );

    // sign-aware zero-poading tests
    test_all_eq!(
        zero_zero_pad:
            "0000000000",
            format!("{:010}", ZERO);
        neg_zero_zero_pad:
            "-000000000",
            format!("{:010}", NEG_ZERO);
        inf_zero_pad:
            "0000000inf",
            format!("{:010}", INFINITY);
        neg_inf_zero_pad:
            "-000000inf",
            format!("{:010}", NEG_INFINITY);
        nan_zero_pad:
            "0000000NaN",
            format!("{:010}", NAN);
        one_zero_pad:
            "0000000001",
            format!("{:010}", ONE);
        neg_one_zero_pad:
            "-000000001",
            format!("{:010}", NEG_ONE);
        plus_one_zero_pad:
            "+000000001",
            format!("{:+010}", ONE);
        pi_zero_pad:
            "3.14159265358979323846264338328",
            format!("{:010}", PI);
        pi_width_zero_pad_prec:
            "0003.14159",
            format!("{:010.5}", PI);
    );

    // special number exp tests
    test_all_eq!(
        nan_lexp:
            "NaN",
            format!("{:e}", NAN);
        neg_nan_lexp:
            "-NaN",
            format!("{:e}", -NAN);
        plus_nan_lexp:
            "+NaN",
            format!("{:+e}", NAN);
        plus_neg_nan_lexp:
            "-NaN",
            format!("{:+e}", -NAN);
        inf_lexp:
            "inf",
            format!("{:e}", INFINITY);
        neg_inf_lexp:
            "-inf",
            format!("{:e}", NEG_INFINITY);
        plus_inf_lexp:
            "+inf",
            format!("{:+e}", INFINITY);
        plus_neg_inf_lexp:
            "-inf",
            format!("{:+e}", NEG_INFINITY);
        zero_lexp:
            "0e0",
            format!("{:e}", ZERO);
        neg_zero_lexp:
            "-0e0",
            format!("{:e}", NEG_ZERO);
        plus_zero_lexp:
            "+0e0",
            format!("{:+e}", ZERO);
        plus_neg_zero_lexp:
            "-0e0",
            format!("{:+e}", NEG_ZERO);
    );

    // special number Exp tests
    test_all_eq!(
        nan_uexp:
            "NaN",
            format!("{:E}", NAN);
        neg_nan_uexp:
            "-NaN",
            format!("{:E}", -NAN);
        plus_nan_uexp:
            "+NaN",
            format!("{:+E}", NAN);
        plus_neg_nan_uexp:
            "-NaN",
            format!("{:+E}", -NAN);
        inf_uexp:
            "inf",
            format!("{:E}", INFINITY);
        neg_inf_uexp:
            "-inf",
            format!("{:E}", NEG_INFINITY);
        plus_inf_uexp:
            "+inf",
            format!("{:+E}", INFINITY);
        plus_neg_inf_uexp:
            "-inf",
            format!("{:+E}", NEG_INFINITY);
        zero_uexp:
            "0E0",
            format!("{:E}", ZERO);
        neg_zero_uexp:
            "-0E0",
            format!("{:E}", NEG_ZERO);
        plus_zero_uexp:
            "+0E0",
            format!("{:+E}", ZERO);
        plus_neg_zero_uexp:
            "-0E0",
            format!("{:+E}", NEG_ZERO);
    );

    // Basic exp tests
    test_all_eq!(
        pi_lexp:
            "3.14159265358979323846264338328e0",
            format!("{:e}", PI);
        neg_pi_lexp:
            "-3.14159265358979323846264338328e0",
            format!("{:e}", -PI);
        plus_pi_lexp:
            "+3.14159265358979323846264338328e0",
            format!("{:+e}", PI);
        plus_neg_pi_lexp:
            "-3.14159265358979323846264338328e0",
            format!("{:+e}", -PI);

        ln2_lexp:
            "6.931471805599453094172321214582e-1",
            format!("{:e}", LN_2);
        neg_ln2_lexp:
            "-6.931471805599453094172321214582e-1",
            format!("{:e}", -LN_2);
        plus_ln2_lexp:
            "+6.931471805599453094172321214582e-1",
            format!("{:+e}", LN_2);
        plus_neg_ln2_lexp:
            "-6.931471805599453094172321214582e-1",
            format!("{:+e}", -LN_2);

        ln2_100_lexp:
            "6.931471805599453094172321214582e-3",
            format!("{:e}", LN_2 / dd!(100));
        neg_ln2_100_lexp:
            "-6.931471805599453094172321214582e-3",
            format!("{:e}", -LN_2 / dd!(100));
        plus_ln2_100_lexp:
            "+6.931471805599453094172321214582e-3",
            format!("{:+e}", LN_2 / dd!(100));
        plus_neg_ln2_100_lexp:
            "-6.931471805599453094172321214582e-3",
            format!("{:+e}", -LN_2 / dd!(100));

        large_lexp:
            "1.234567890123456789e308",
            format!("{:e}", dd!("1.234567890123456789e308"));
        neg_large_lexp:
            "-1.234567890123456789e308",
            format!("{:e}", dd!("-1.234567890123456789e308"));
        plus_large_lexp:
            "+1.234567890123456789e308",
            format!("{:+e}", dd!("1.234567890123456789e308"));

        small_lexp:
            "1.234567890123457e-308",
            format!("{:e}", dd!("1.234567890123456789e-308"));
        neg_small_lexp:
            "-1.234567890123457e-308",
            format!("{:e}", dd!("-1.234567890123456789e-308"));
        plus_small_lexp:
            "+1.234567890123457e-308",
            format!("{:+e}", dd!("1.234567890123456789e-308"));
    );

    // Basic EXP tests
    test_all_eq!(
        pi_uexp:
            "3.14159265358979323846264338328E0",
            format!("{:E}", PI);
        neg_pi_uexp:
            "-3.14159265358979323846264338328E0",
            format!("{:E}", -PI);
        plus_pi_uexp:
            "+3.14159265358979323846264338328E0",
            format!("{:+E}", PI);
        plus_neg_pi_uexp:
            "-3.14159265358979323846264338328E0",
            format!("{:+E}", -PI);

        ln2_uexp:
            "6.931471805599453094172321214582E-1",
            format!("{:E}", LN_2);
        neg_ln2_uexp:
            "-6.931471805599453094172321214582E-1",
            format!("{:E}", -LN_2);
        plus_ln2_uexp:
            "+6.931471805599453094172321214582E-1",
            format!("{:+E}", LN_2);
        plus_neg_ln2_uexp:
            "-6.931471805599453094172321214582E-1",
            format!("{:+E}", -LN_2);

        ln2_100_uexp:
            "6.931471805599453094172321214582E-3",
            format!("{:E}", LN_2 / dd!(100));
        neg_ln2_100_uexp:
            "-6.931471805599453094172321214582E-3",
            format!("{:E}", -LN_2 / dd!(100));
        plus_ln2_100_uexp:
            "+6.931471805599453094172321214582E-3",
            format!("{:+E}", LN_2 / dd!(100));
        plus_neg_ln2_100_uexp:
            "-6.931471805599453094172321214582E-3",
            format!("{:+E}", -LN_2 / dd!(100));

        large_uexp:
            "1.234567890123456789E308",
            format!("{:E}", dd!("1.234567890123456789e308"));
        neg_large_uexp:
            "-1.234567890123456789E308",
            format!("{:E}", dd!("-1.234567890123456789e308"));
        plus_large_uexp:
            "+1.234567890123456789E308",
            format!("{:+E}", dd!("1.234567890123456789e308"));

        small_uexp:
            "1.234567890123457E-308",
            format!("{:E}", dd!("1.234567890123456789e-308"));
        neg_small_uexp:
            "-1.234567890123457E-308",
            format!("{:E}", dd!("-1.234567890123456789e-308"));
        plus_small_uexp:
            "+1.234567890123457E-308",
            format!("{:+E}", dd!("1.234567890123456789e-308"));
    );

    // NOTE
    //
    // The code is so similar between UpperExp and LowerExp that after those tests there's
    // not much need to continue to test them both. Further tests are being done solely with
    // LowerExp.

    // precision exp tests
    test_all_eq!(
        prec_zero_exp:
            "0.0000e0",
            format!("{:.4e}", ZERO);
        prec_neg_zero_exp:
            "-0.0e0",
            format!("{:.1e}", NEG_ZERO);
        prec_plus_zero_exp:
            "+0.000000000000e0",
            format!("{:+.12e}", ZERO);
        prec_plus_neg_zero_exp:
            "-0.000000e0",
            format!("{:+.6e}", NEG_ZERO);

        pi_prec_5_exp:
            "3.14159e0",
            format!("{:.5e}", PI);
        pi_prec_7_exp:
            "3.1415927e0",
            format!("{:.7e}", PI);
        pi_prec_20_exp:
            "3.14159265358979323846e0",
            format!("{:.20e}", PI);
        pi_prec_40_exp:
            "3.1415926535897932384626433832800000000000e0",
            format!("{:.40e}", PI);
        pi_prec_0_exp:
            "3e0",
            format!("{:.0e}", PI);
        neg_pi_prec_40_exp:
            "-3.1415926535897932384626433832800000000000e0",
            format!("{:.40e}", -PI);
        neg_pi_prec_0_exp:
            "-3e0",
            format!("{:.0e}", -PI);
        plus_pi_prec_40_exp:
            "+3.1415926535897932384626433832800000000000e0",
            format!("{:+.40e}", PI);
        plus_pi_prec_0_exp:
            "+3e0",
            format!("{:+.0e}", PI);


        ln2_prec_5_exp:
            "6.93147e-1",
            format!("{:.5e}", LN_2);
        ln2_prec_7_exp:
            "6.9314718e-1",
            format!("{:.7e}", LN_2);
        ln2_prec_20_exp:
            "6.93147180559945309417e-1",
            format!("{:.20e}", LN_2);
        ln2_prec_40_exp:
            "6.9314718055994530941723212145820000000000e-1",
            format!("{:.40e}", LN_2);
        ln2_prec_0_exp:
            "7e-1",
            format!("{:.0e}", LN_2);
        neg_ln2_prec_40_exp:
            "-6.9314718055994530941723212145820000000000e-1",
            format!("{:.40e}", -LN_2);
        neg_ln2_prec_0_exp:
            "-7e-1",
            format!("{:.0e}", -LN_2);
        plus_ln2_prec_40_exp:
            "+6.9314718055994530941723212145820000000000e-1",
            format!("{:+.40e}", LN_2);
        plus_ln2_prec_0_exp:
            "+7e-1",
            format!("{:+.0e}", LN_2);

        ln2_100_prec_5_exp:
            "6.93147e-3",
            format!("{:.5e}", LN_2 / dd!(100));
        ln2_100_prec_7_exp:
            "6.9314718e-3",
            format!("{:.7e}", LN_2 / dd!(100));
        ln2_100_prec_20_exp:
            "6.93147180559945309417e-3",
            format!("{:.20e}", LN_2 / dd!(100));
        ln2_100_prec_40_exp:
            "6.9314718055994530941723212145820000000000e-3",
            format!("{:.40e}", LN_2 / dd!(100));
        ln2_100_prec_0_exp:
            "7e-3",
            format!("{:.0e}", LN_2 / dd!(100));
        neg_ln2_100_prec_40_exp:
            "-6.9314718055994530941723212145820000000000e-3",
            format!("{:.40e}", -LN_2 / dd!(100));
        neg_ln2_100_prec_0_exp:
            "-7e-3",
            format!("{:.0e}", -LN_2 / dd!(100));
        plus_ln2_100_prec_40_exp:
            "+6.9314718055994530941723212145820000000000e-3",
            format!("{:+.40e}", LN_2 / dd!(100));
        plus_ln2_100_prec_0_exp:
            "+7e-3",
            format!("{:+.0e}", LN_2 / dd!(100));


        large_prec_0_exp:
            "1e308",
            format!("{:.0e}", dd!("1.234567890123456789e308"));
        large_prec_8_exp:
            "1.23456789e308",
            format!("{:.8e}", dd!("1.234567890123456789e308"));


        small_prec_0_exp:
            "1e-308",
            format!("{:.0e}", dd!("1.234567890123456789e-308"));
        small_prec_8_exp:
            "1.23456789e-308",
            format!("{:.8e}", dd!("1.234567890123456789e-308"));
    );

    // width tests (default right align)
    test_all_eq!(
        zero_width_exp:
            "       0e0",
            format!("{:10e}", ZERO);
        neg_zero_width_exp:
            "      -0e0",
            format!("{:10e}", NEG_ZERO);
        inf_width_exp:
            "       inf",
            format!("{:10e}", INFINITY);
        neg_inf_width_exp:
            "      -inf",
            format!("{:10e}", NEG_INFINITY);
        nan_width_exp:
            "       NaN",
            format!("{:10e}", NAN);
        one_width_exp:
            "       1e0",
            format!("{:10e}", ONE);
        neg_one_width_exp:
            "      -1e0",
            format!("{:10e}", NEG_ONE);
        plus_one_width_exp:
            "      +1e0",
            format!("{:+10e}", ONE);
        pi_width_exp:
            "3.14159265358979323846264338328e0",
            format!("{:10e}", PI);
        pi_width_prec_exp:
            " 3.14159e0",
            format!("{:10.5e}", PI);
    );

    // right-align tests
    test_all_eq!(
        zero_right_exp:
            "       0e0",
            format!("{:>10e}", ZERO);
        neg_zero_right_exp:
            "      -0e0",
            format!("{:>10e}", NEG_ZERO);
        inf_right_exp:
            "       inf",
            format!("{:>10e}", INFINITY);
        neg_inf_right_exp:
            "      -inf",
            format!("{:>10e}", NEG_INFINITY);
        nan_right_exp:
            "       NaN",
            format!("{:>10e}", NAN);
        one_right_exp:
            "       1e0",
            format!("{:>10e}", ONE);
        neg_one_right_exp:
            "      -1e0",
            format!("{:>10e}", NEG_ONE);
        plus_one_right_exp:
            "      +1e0",
            format!("{:>+10e}", ONE);
        pi_right_exp:
            "3.14159265358979323846264338328e0",
            format!("{:>10e}", PI);
        pi_right_prec_exp:
            " 3.14159e0",
            format!("{:>10.5e}", PI);
    );

    // left-align tests
    test_all_eq!(
        zero_left_exp:
            "0e0       ",
            format!("{:<10e}", ZERO);
        neg_zero_left_exp:
            "-0e0      ",
            format!("{:<10e}", NEG_ZERO);
        inf_left_exp:
            "inf       ",
            format!("{:<10e}", INFINITY);
        neg_inf_left_exp:
            "-inf      ",
            format!("{:<10e}", NEG_INFINITY);
        nan_left_exp:
            "NaN       ",
            format!("{:<10e}", NAN);
        one_left_exp:
            "1e0       ",
            format!("{:<10e}", ONE);
        neg_one_left_exp:
            "-1e0      ",
            format!("{:<10e}", NEG_ONE);
        plus_one_left_exp:
            "+1e0      ",
            format!("{:<+10e}", ONE);
        pi_left_exp:
            "3.14159265358979323846264338328e0",
            format!("{:<10e}", PI);
        pi_left_prec_exp:
            "3.14159e0 ",
            format!("{:<10.5e}", PI);
    );

    // center-align tests
    test_all_eq!(
        zero_center_exp:
            "   0e0    ",
            format!("{:^10e}", ZERO);
        neg_zero_center_exp:
            "   -0e0   ",
            format!("{:^10e}", NEG_ZERO);
        inf_center_exp:
            "   inf    ",
            format!("{:^10e}", INFINITY);
        neg_inf_center_exp:
            "   -inf   ",
            format!("{:^10e}", NEG_INFINITY);
        nan_center_exp:
            "   NaN    ",
            format!("{:^10e}", NAN);
        one_center_exp:
            "   1e0    ",
            format!("{:^10e}", ONE);
        neg_one_center_exp:
            "   -1e0   ",
            format!("{:^10e}", NEG_ONE);
        plus_one_center_exp:
            "   +1e0   ",
            format!("{:^+10e}", ONE);
        pi_center_exp:
            "3.14159265358979323846264338328e0",
            format!("{:^10e}", PI);
        pi_center_prec_exp:
            "3.14159e0 ",
            format!("{:^10.5e}", PI);
    );

    // fill tests
    test_all_eq!(
        zero_fill_exp:
            "_______0e0",
            format!("{:_>10e}", ZERO);
        neg_zero_fill_exp:
            "-0e0______",
            format!("{:_<10e}", NEG_ZERO);
        inf_fill_exp:
            "___inf____",
            format!("{:_^10e}", INFINITY);
        neg_inf_fill_exp:
            "______-inf",
            format!("{:_>10e}", NEG_INFINITY);
        nan_fill_exp:
            "NaN_______",
            format!("{:_<10e}", NAN);
        one_fill_exp:
            "___1e0____",
            format!("{:_^10e}", ONE);
        neg_one_fill_exp:
            "______-1e0",
            format!("{:_>10e}", NEG_ONE);
        plus_one_fill_exp:
            "+1e0______",
            format!("{:_<+10e}", ONE);
        pi_fill_exp:
            "3.14159265358979323846264338328e0",
            format!("{:_^10e}", PI);
        pi_fill_prec_exp:
            "_3.14159e0",
            format!("{:_>10.5e}", PI);
    );

    // sign-aware zero-poading tests
    test_all_eq!(
        zero_zero_pad_exp:
            "00000000e0",
            format!("{:010e}", ZERO);
        neg_zero_zero_pad_exp:
            "-0000000e0",
            format!("{:010e}", NEG_ZERO);
        inf_zero_pad_exp:
            "0000000inf",
            format!("{:010e}", INFINITY);
        neg_inf_zero_pad_exp:
            "-000000inf",
            format!("{:010e}", NEG_INFINITY);
        nan_zero_pad_exp:
            "0000000NaN",
            format!("{:010e}", NAN);
        one_zero_pad_exp:
            "00000001e0",
            format!("{:010e}", ONE);
        neg_one_zero_pad_exp:
            "-0000001e0",
            format!("{:010e}", NEG_ONE);
        plus_one_zero_pad_exp:
            "+0000001e0",
            format!("{:+010e}", ONE);
        pi_zero_pad_exp:
            "3.14159265358979323846264338328e0",
            format!("{:010e}", PI);
        pi_width_zero_pad_prec_exp:
            "03.14159e0",
            format!("{:010.5e}", PI);
    );
}
