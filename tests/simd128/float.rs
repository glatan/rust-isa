use rust_isa::wasm32::simd::simd128::*;

use super::lanes::{f32_lanes, f32x4, f64_lanes, f64x2, true_lane};

#[test]
fn f32x4_arithmetic_rounding_and_finite_minmax() {
    let left = f32x4([1.0, 6.0, 3.0, -2.0]);
    let right = f32x4([2.0, 2.0, 4.0, -8.0]);
    assert_eq!(f32_lanes(f32x4_add(left, right)), [3.0, 8.0, 7.0, -10.0]);
    assert_eq!(f32_lanes(f32x4_sub(left, right)), [-1.0, 4.0, -1.0, 6.0]);
    assert_eq!(f32_lanes(f32x4_mul(left, right)), [2.0, 12.0, 12.0, 16.0]);
    assert_eq!(f32_lanes(f32x4_div(left, right)), [0.5, 3.0, 0.75, 0.25]);
    assert_eq!(f32_lanes(f32x4_min(left, right)), [1.0, 2.0, 3.0, -8.0]);
    assert_eq!(f32_lanes(f32x4_max(left, right)), [2.0, 6.0, 4.0, -2.0]);
    assert_eq!(f32_lanes(f32x4_abs(f32x4_splat(-3.5))), [3.5; 4]);
    assert_eq!(f32_lanes(f32x4_neg(f32x4_splat(1.25))), [-1.25; 4]);
    assert_eq!(f32_lanes(f32x4_sqrt(f32x4_splat(9.0))), [3.0; 4]);
    assert_eq!(f32_lanes(f32x4_ceil(f32x4_splat(1.2))), [2.0; 4]);
    assert_eq!(f32_lanes(f32x4_floor(f32x4_splat(1.2))), [1.0; 4]);
    assert_eq!(f32_lanes(f32x4_trunc(f32x4_splat(-1.9))), [-1.0; 4]);
    assert_eq!(f32_lanes(f32x4_nearest(f32x4_splat(2.5))), [2.0; 4]);
    assert_eq!(
        f32x4_extract_lane::<2>(f32x4_replace_lane::<2>(f32x4_splat(0.0), 4.5)),
        4.5
    );
    assert!(true_lane(f32x4_eq(left, left), 0, 4));
    assert!(true_lane(
        f32x4_lt(f32x4_splat(1.0), f32x4_splat(2.0)),
        1,
        4
    ));
    assert!(!true_lane(
        f32x4_ge(f32x4_splat(1.0), f32x4_splat(2.0)),
        1,
        4
    ));
}

#[test]
fn f32x4_min_max_nan_and_signed_zero() {
    let nan = f32x4_splat(f32::NAN);
    let one = f32x4_splat(1.0);
    let neg_zero = f32x4_splat(-0.0);
    let pos_zero = f32x4_splat(0.0);
    assert!(f32_lanes(f32x4_min(nan, one))[0].is_nan());
    assert!(f32_lanes(f32x4_min(one, nan))[0].is_nan());
    assert!(f32_lanes(f32x4_max(nan, one))[0].is_nan());
    assert!(f32_lanes(f32x4_max(one, nan))[0].is_nan());
    assert_eq!(
        f32_lanes(f32x4_min(neg_zero, pos_zero))[0].to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        f32_lanes(f32x4_min(pos_zero, neg_zero))[0].to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        f32_lanes(f32x4_max(neg_zero, pos_zero))[0].to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        f32_lanes(f32x4_max(pos_zero, neg_zero))[0].to_bits(),
        0.0f32.to_bits()
    );
    assert!(!true_lane(f32x4_eq(nan, nan), 0, 4));
    assert!(true_lane(f32x4_ne(nan, one), 0, 4));
    assert!(!true_lane(f32x4_lt(nan, one), 0, 4));
    assert!(!true_lane(f32x4_ge(one, nan), 0, 4));
}

#[test]
fn f32x4_pmin_pmax_follow_the_comparison() {
    let low = f32x4_splat(1.0);
    let high = f32x4_splat(3.0);
    assert_eq!(f32_lanes(f32x4_pmin(high, low)), [1.0; 4]);
    assert_eq!(f32_lanes(f32x4_pmin(low, high)), [1.0; 4]);
    assert_eq!(f32_lanes(f32x4_pmax(low, high)), [3.0; 4]);
    assert_eq!(f32_lanes(f32x4_pmax(high, low)), [3.0; 4]);

    let nan = f32x4_splat(f32::NAN);
    assert!(f32_lanes(f32x4_pmin(nan, low))[0].is_nan());
    assert_eq!(f32_lanes(f32x4_pmin(low, nan))[0], 1.0);
    assert!(f32_lanes(f32x4_pmax(nan, high))[0].is_nan());
    assert_eq!(f32_lanes(f32x4_pmax(high, nan))[0], 3.0);
}

#[test]
fn f64x2_arithmetic_and_rounding() {
    let left = f64x2([1.5, -4.0]);
    let right = f64x2([0.5, 2.0]);
    assert_eq!(f64_lanes(f64x2_add(left, right)), [2.0, -2.0]);
    assert_eq!(f64_lanes(f64x2_sub(left, right)), [1.0, -6.0]);
    assert_eq!(f64_lanes(f64x2_mul(left, right)), [0.75, -8.0]);
    assert_eq!(f64_lanes(f64x2_div(left, right)), [3.0, -2.0]);
    assert_eq!(f64_lanes(f64x2_min(left, right)), [0.5, -4.0]);
    assert_eq!(f64_lanes(f64x2_max(left, right)), [1.5, 2.0]);
    assert_eq!(f64_lanes(f64x2_sqrt(f64x2_splat(4.0))), [2.0, 2.0]);
    assert_eq!(f64_lanes(f64x2_abs(f64x2_splat(-3.5))), [3.5, 3.5]);
    assert_eq!(f64_lanes(f64x2_neg(f64x2_splat(1.25))), [-1.25, -1.25]);
    assert_eq!(f64_lanes(f64x2_ceil(f64x2_splat(1.2))), [2.0, 2.0]);
    assert_eq!(f64_lanes(f64x2_floor(f64x2_splat(1.2))), [1.0, 1.0]);
    assert_eq!(f64_lanes(f64x2_trunc(f64x2_splat(-1.9))), [-1.0, -1.0]);
    assert_eq!(f64_lanes(f64x2_nearest(f64x2_splat(0.5))), [0.0, 0.0]);
    assert_eq!(
        f64x2_extract_lane::<1>(f64x2_replace_lane::<1>(f64x2_splat(0.0), 8.0)),
        8.0
    );
    assert!(true_lane(
        f64x2_le(f64x2_splat(-1.0), f64x2_splat(-1.0)),
        0,
        8
    ));
    assert!(true_lane(
        f64x2_gt(f64x2_splat(2.0), f64x2_splat(1.0)),
        1,
        8
    ));

    let nan = f64x2_splat(f64::NAN);
    assert!(f64_lanes(f64x2_min(nan, f64x2_splat(1.0)))[0].is_nan());
    assert!(f64_lanes(f64x2_max(f64x2_splat(1.0), nan))[0].is_nan());
    assert_eq!(
        f64_lanes(f64x2_min(f64x2_splat(-0.0), f64x2_splat(0.0)))[0].to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        f64_lanes(f64x2_max(f64x2_splat(-0.0), f64x2_splat(0.0)))[0].to_bits(),
        0.0f64.to_bits()
    );
    assert!(!true_lane(f64x2_eq(nan, nan), 0, 8));
    assert!(true_lane(f64x2_ne(nan, f64x2_splat(1.0)), 0, 8));
    assert!(f64_lanes(f64x2_pmin(nan, f64x2_splat(1.0)))[0].is_nan());
    assert_eq!(f64_lanes(f64x2_pmin(f64x2_splat(1.0), nan))[0], 1.0);
    assert_eq!(f64_lanes(f64x2_pmax(f64x2_splat(3.0), nan))[0], 3.0);
    assert!(f64_lanes(f64x2_pmax(nan, f64x2_splat(3.0)))[0].is_nan());
}
