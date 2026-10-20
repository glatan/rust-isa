use rust_isa::wasm32::simd::simd128::*;

use super::lanes::{i8_lanes, i8x16, i16_lanes, i32_lanes, i32x4, i64_lanes, i64x2, true_lane};

#[test]
fn i8x16_splat_extract_replace_and_wrap() {
    let value = i8x16_splat(-1);
    assert_eq!(i8_lanes(value), [-1; 16]);
    assert_eq!(i8x16_extract_lane_s::<0>(value), -1);
    assert_eq!(i8x16_extract_lane_u::<15>(value), 0xff);
    let replaced = i8x16_replace_lane::<3>(value, 9);
    assert_eq!(i8x16_extract_lane_s::<3>(replaced), 9);
    assert_eq!(i8x16_extract_lane_s::<2>(replaced), -1);

    let sum = i8x16_add(i8x16_splat(127), i8x16_splat(1));
    assert_eq!(i8_lanes(sum), [-128; 16]);
    let difference = i8x16_sub(i8x16_splat(-128), i8x16_splat(1));
    assert_eq!(i8_lanes(difference), [127; 16]);
    assert_eq!(i8_lanes(i8x16_neg(i8x16_splat(-3))), [3; 16]);
    assert_eq!(
        i8_lanes(i8x16_neg(i8x16_splat(i32::from(i8::MIN)))),
        [i8::MIN; 16]
    );
    assert_eq!(i8_lanes(i8x16_abs(i8x16_splat(-3))), [3; 16]);
    assert_eq!(
        i8_lanes(i8x16_abs(i8x16_splat(i32::from(i8::MIN)))),
        [i8::MIN; 16]
    );
}

#[test]
fn i8x16_compare_shift_sat_and_reductions() {
    let left = i8x16_splat(-1);
    let right = i8x16_splat(1);
    assert!(true_lane(i8x16_lt_s(left, right), 0, 1));
    assert!(!true_lane(i8x16_lt_u(left, right), 0, 1));
    assert!(true_lane(i8x16_eq(left, left), 4, 1));
    assert!(!true_lane(i8x16_ne(left, left), 4, 1));
    assert!(true_lane(i8x16_gt_s(right, left), 0, 1));
    assert!(true_lane(i8x16_le_s(left, left), 0, 1));
    assert!(true_lane(i8x16_ge_u(left, right), 0, 1));

    let shifted = i8x16([-8, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(i8_lanes(i8x16_shl(shifted, 1))[0], -16);
    assert_eq!(i8_lanes(i8x16_shr_s(shifted, 1))[0], -4);
    assert_eq!(i8_lanes(i8x16_shr_u(shifted, 1))[0] as u8, 0x7c);
    assert_eq!(
        i8_lanes(i8x16_shl(shifted, 9)),
        i8_lanes(i8x16_shl(shifted, 1))
    );

    assert_eq!(
        i8_lanes(i8x16_add_sat_s(i8x16_splat(127), i8x16_splat(1))),
        [127; 16]
    );
    assert_eq!(
        i8_lanes(i8x16_sub_sat_s(i8x16_splat(-128), i8x16_splat(1))),
        [-128; 16]
    );
    let wide = V128::from_bytes([200; 16]);
    let addend = V128::from_bytes([100; 16]);
    assert_eq!(i8x16_add_sat_u(wide, addend).to_bytes(), [255; 16]);
    assert_eq!(i8_lanes(i8x16_min_s(left, right)), [-1; 16]);
    assert_eq!(i8_lanes(i8x16_max_u(left, right)), [-1; 16]);
    assert_eq!(
        i8x16_avgr_u(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])).to_bytes(),
        [2; 16]
    );

    assert_eq!(i8x16_all_true(i8x16_splat(1)), 1);
    assert_eq!(
        i8x16_all_true(i8x16_replace_lane::<3>(i8x16_splat(1), 0)),
        0
    );
    assert_eq!(i8_lanes(i8x16_popcnt(i8x16_splat(0x0f))), [4; 16]);
    let mut bits = [0u8; 16];
    bits[0] = 0x80;
    bits[2] = 0x80;
    assert_eq!(i8x16_bitmask(V128::from_bytes(bits)), 0b101);
}

#[test]
fn i16x8_arith_shift_and_average() {
    assert_eq!(
        i16_lanes(i16x8_add(i16x8_splat(i32::from(i16::MAX)), i16x8_splat(1))),
        [i16::MIN; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_add_sat_s(
            i16x8_splat(i32::from(i16::MAX)),
            i16x8_splat(1)
        )),
        [i16::MAX; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_mul(i16x8_splat(300), i16x8_splat(3))),
        [900; 8]
    );
    assert_eq!(i16_lanes(i16x8_shr_s(i16x8_splat(-4), 1)), [-2; 8]);
    assert_eq!(
        i16_lanes(i16x8_shl(i16x8_splat(1), 17)),
        i16_lanes(i16x8_shl(i16x8_splat(1), 1))
    );
    assert_eq!(i16x8_extract_lane_s::<7>(i16x8_splat(-2)), -2);
    assert_eq!(i16x8_extract_lane_u::<7>(i16x8_splat(-2)), 0xfffe);
    assert_eq!(
        i16_lanes(i16x8_avgr_u(i16x8_splat(1), i16x8_splat(2))),
        [2; 8]
    );
    assert_eq!(i16x8_all_true(i16x8_splat(0)), 0);
    assert!(true_lane(i16x8_eq(i16x8_splat(4), i16x8_splat(4)), 3, 2));
    assert!(!true_lane(i16x8_eq(i16x8_splat(4), i16x8_splat(5)), 3, 2));
}

#[test]
fn i32x4_and_i64x2_wrap_shift_and_compare() {
    assert_eq!(
        i32_lanes(i32x4_add(i32x4([i32::MAX, -1, 0, 5]), i32x4([1, 1, 0, 5]))),
        [i32::MIN, 0, 0, 10]
    );
    assert_eq!(
        i32_lanes(i32x4_mul(i32x4_splat(1000), i32x4_splat(-3))),
        [-3000; 4]
    );
    assert_eq!(i32_lanes(i32x4_shr_s(i32x4_splat(-4), 1)), [-2; 4]);
    assert_eq!(
        i32_lanes(i32x4_shr_u(i32x4_splat(-1), 1))[0] as u32,
        0x7fff_ffff
    );
    assert_eq!(
        i32_lanes(i32x4_shl(i32x4_splat(1), 33)),
        i32_lanes(i32x4_shl(i32x4_splat(1), 1))
    );
    assert!(true_lane(i32x4_eq(i32x4_splat(5), i32x4_splat(5)), 1, 4));
    assert!(true_lane(i32x4_lt_s(i32x4_splat(-1), i32x4_splat(1)), 0, 4));
    assert!(!true_lane(
        i32x4_lt_u(i32x4_splat(-1), i32x4_splat(1)),
        0,
        4
    ));
    assert_eq!(
        i32x4_extract_lane_s::<2>(i32x4_replace_lane::<2>(i32x4_splat(0), 9)),
        9
    );
    assert_eq!(
        i32x4_min_s(i32x4_splat(-3), i32x4_splat(4)),
        i32x4_splat(-3)
    );
    assert_eq!(
        i32x4_max_u(i32x4_splat(-1), i32x4_splat(1)),
        i32x4_splat(-1)
    );

    assert_eq!(
        i64_lanes(i64x2_add(i64x2([i64::MAX, -4]), i64x2([1, 1]))),
        [i64::MIN, -3]
    );
    assert_eq!(
        i64_lanes(i64x2_mul(i64x2_splat(6), i64x2_splat(7))),
        [42, 42]
    );
    assert_eq!(i64_lanes(i64x2_shr_s(i64x2_splat(-4), 1)), [-2, -2]);
    assert_eq!(
        i64_lanes(i64x2_shr_u(i64x2_splat(-1), 1))[0] as u64,
        0x7fff_ffff_ffff_ffff
    );
    assert_eq!(
        i64_lanes(i64x2_shl(i64x2_splat(1), 65)),
        i64_lanes(i64x2_shl(i64x2_splat(1), 1))
    );
    assert_eq!(i64x2_extract_lane_s::<1>(i64x2([1, -5])), -5);
    assert_eq!(i64x2_extract_lane_u::<1>(i64x2([1, -5])), u64::MAX - 4);
    assert!(true_lane(i64x2_eq(i64x2_splat(8), i64x2_splat(8)), 0, 8));
    assert!(true_lane(i64x2_lt_s(i64x2_splat(-1), i64x2_splat(0)), 1, 8));
    assert!(true_lane(i64x2_gt_s(i64x2_splat(2), i64x2_splat(1)), 0, 8));
    assert_eq!(i64x2_abs(i64x2_splat(-9)), i64x2_splat(9));
    assert_eq!(i64x2_all_true(i64x2_splat(1)), 1);
    assert_eq!(i64x2_bitmask(i64x2_splat(-1)), 0b11);
}
