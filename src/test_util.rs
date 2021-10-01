#![cfg(test)]
#![macro_use]

use rand::Rng;

const TEST_ITERS: usize = 100000;

pub fn random_float() -> f64 {
    let mut engine = rand::thread_rng();
    let mantissa_dist = rand::distributions::Uniform::new(0, 1u64 << 52);
    let exponent_dist = rand::distributions::Uniform::new(0, 2047u64);
    let x = f64::from_bits(engine.sample(mantissa_dist) | (engine.sample(exponent_dist) << 52));
    if engine.gen() {
        x
    } else {
        -x
    }
}

pub fn repeated_test<F>(test: F)
where
    F: Fn(),
{
    for _ in 0..TEST_ITERS {
        test();
    }
}

pub fn get_valid_pair<F: Fn(f64, f64) -> bool>(pred: F) -> (f64, f64) {
    loop {
        let a = random_float();
        let b = random_float();
        if pred(a, b) {
            return (a, b);
        }
    }
}

macro_rules! assert_eq_ulp {
    ($left:expr, $right:expr, $ulp:expr) => ({
        let left_val = $left;
        let right_val = $right;
        let ulp_val = $ulp;

        let a_bits = left_val.to_bits();
        let b_bits = right_val.to_bits();
        let fix_sign = |x| {
            if x & (1 << 63) == 0 {
                x
            } else {
                x ^ ((1 << 63) - 1)
            }
        };
        let diff = (fix_sign(a_bits) as i64)
            .saturating_sub(fix_sign(b_bits) as i64)
            .abs();
        if !(diff <= *ulp_val) {
            panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`"#, ulp_val, left_val, right_val, diff)
        }
    });
    ($left:expr, $right:expr, $ulp:expr, $($args:tt,)+) => ({
        let left_val = $left;
        let right_val = $right;
        let ulp_val = $ulp;

        let a_bits = left_val.to_bits();
        let b_bits = right_val.to_bits();
        let fix_sign = |x| {
            if x & (1 << 63) == 0 {
                x
            } else {
                x ^ ((1 << 63) - 1)
            }
        };
        let diff = (fix_sign(a_bits) as i64)
            .saturating_sub(fix_sign(b_bits) as i64)
            .abs();
        if !(diff <= ulp_val) {
            panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`: {}"#, ulp_val, left_val, right_val, diff, format_args!($($args,)+))
        }
    });
    ($left:expr, $right:expr, $ulp:expr, $($args:tt),+) => {
        assert_eq_ulp!($left, $right, $ulp, $($args,)+)
    };
}

macro_rules! test_all_eq {
    ($($name:ident: $expected:expr, $actual:expr);* $(;)?) => {
        $(#[test] fn $name() { assert_eq!($expected, $actual); })*
    };
}

macro_rules! test {
    ($name:ident: { $($tt:tt)* }) => {
        #[test] fn $name() { $($tt)* }
    };
}

macro_rules! exact {
    ($expected:expr, $actual:expr $(,)?) => {
        let expected = TwoFloat::from($expected);
        let actual = TwoFloat::from($actual);
        let message = format!(
            concat!(
                "\n",
                "Expected: {0}\n",
                "Actual:   {1}\n",
                "\n",
                "Components:\n",
                "  Expected: {2:<22e} {3:e}\n",
                "  Actual:   {4:<22e} {5:e}\n",
            ),
            expected, actual, expected.hi, expected.lo, actual.hi, actual.lo
        );
        if expected.is_nan() {
            assert!(actual.is_nan(), message);
        } else {
            assert!(expected == actual, message);
        };
    };
}

macro_rules! prec {
    ($expected:expr, $actual:expr, $digits:expr $(,)?) => {
        let expected = TwoFloat::from($expected);
        let actual = TwoFloat::from($actual);
        let mag = if expected.is_zero() {
            1
        } else {
            expected.hi.abs().log10().ceil() as i32
        };
        let epsilon = TwoFloat { hi: 10.0, lo: 0.0 }.powi(mag - $digits);
        let diff = (expected - actual).abs();
        let message = format!(
            concat!(
                "\n",
                "Expected: {0}\n",
                "Actual:   {1}\n",
                "\n",
                "Delta:    {2:e}\n",
                "Epsilon:  {3:e}\n",
                "\n",
                "Components:\n",
                "  Expected: {4:<22e} {5:e}\n",
                "  Actual:   {6:<22e} {7:e}\n",
            ),
            expected, actual, diff, epsilon, expected.hi, expected.lo, actual.hi, actual.lo
        );
        assert!(diff < epsilon, message);
    };
}

macro_rules! near {
    ($expected:expr, $actual:expr $(,)?) => {
        prec!($expected, $actual, 31);
    };
}

macro_rules! test_exact {
    ($name:ident: $expected:expr, $actual:expr $(,)?) => {
        #[test]
        fn $name() {
            exact!($expected, $actual);
        }
    };
}

macro_rules! test_all_exact {
    ($($name:ident: $expected:expr, $actual:expr);* $(;)? )=> {
        $(test_exact!($name: $expected, $actual);)*
    };
}
