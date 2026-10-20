use rust_isa::wasm32::simd::simd128::*;

use super::lanes::{
    i8_lanes, i8x16, i16_lanes, i16x8, i32_lanes, i32x4, i64_lanes, i64x2, true_lane,
};

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

#[test]
fn i8x16_remaining_compare_and_saturation() {
    let minus_one = i8x16_splat(-1);
    let one = i8x16_splat(1);
    assert!(true_lane(i8x16_gt_u(minus_one, one), 0, 1));
    assert!(true_lane(i8x16_le_u(one, minus_one), 0, 1));
    assert!(true_lane(i8x16_ge_s(one, minus_one), 0, 1));
    assert_eq!(
        i8_lanes(i8x16_sub_sat_u(i8x16_splat(10), i8x16_splat(100))),
        [0; 16]
    );
    assert_eq!(i8_lanes(i8x16_min_u(minus_one, one)), [1; 16]);
    assert_eq!(i8_lanes(i8x16_max_s(minus_one, one)), [1; 16]);
}

#[test]
fn i16x8_lanes_compare_and_saturation() {
    assert_eq!(
        i16_lanes(i16x8_replace_lane::<2>(i16x8_splat(0), -9)),
        [0, 0, -9, 0, 0, 0, 0, 0]
    );
    assert!(!true_lane(i16x8_ne(i16x8_splat(4), i16x8_splat(4)), 0, 2));
    assert!(true_lane(i16x8_ne(i16x8_splat(4), i16x8_splat(5)), 0, 2));
    assert!(true_lane(i16x8_lt_s(i16x8_splat(-1), i16x8_splat(1)), 0, 2));
    assert!(!true_lane(
        i16x8_lt_u(i16x8_splat(-1), i16x8_splat(1)),
        0,
        2
    ));
    assert!(true_lane(i16x8_gt_s(i16x8_splat(1), i16x8_splat(-1)), 0, 2));
    assert!(!true_lane(
        i16x8_gt_u(i16x8_splat(1), i16x8_splat(-1)),
        0,
        2
    ));
    assert!(true_lane(
        i16x8_le_s(i16x8_splat(-1), i16x8_splat(-1)),
        0,
        2
    ));
    assert!(true_lane(i16x8_le_u(i16x8_splat(1), i16x8_splat(-1)), 0, 2));
    assert!(true_lane(i16x8_ge_s(i16x8_splat(1), i16x8_splat(-1)), 0, 2));
    assert!(true_lane(i16x8_ge_u(i16x8_splat(-1), i16x8_splat(1)), 0, 2));

    assert_eq!(i16_lanes(i16x8_abs(i16x8_splat(-3))), [3; 8]);
    assert_eq!(
        i16_lanes(i16x8_abs(i16x8_splat(i32::from(i16::MIN)))),
        [i16::MIN; 8]
    );
    assert_eq!(i16_lanes(i16x8_neg(i16x8_splat(-3))), [3; 8]);
    assert_eq!(
        i16_lanes(i16x8_neg(i16x8_splat(i32::from(i16::MIN)))),
        [i16::MIN; 8]
    );
    assert_eq!(i16x8_bitmask(i16x8([-1, 0, -1, 0, 0, 0, 0, 0])), 0b101);
    assert_eq!(i16_lanes(i16x8_shr_u(i16x8_splat(-1), 1)), [0x7fff; 8]);
    assert_eq!(
        i16_lanes(i16x8_add_sat_u(i16x8_splat(60_000), i16x8_splat(10_000))),
        [u16::MAX as i16; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_sub(i16x8_splat(1), i16x8_splat(2))),
        [-1; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_sub_sat_s(
            i16x8_splat(i32::from(i16::MIN)),
            i16x8_splat(1)
        )),
        [i16::MIN; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_sub_sat_u(i16x8_splat(1), i16x8_splat(2))),
        [0; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_min_s(i16x8_splat(-1), i16x8_splat(1))),
        [-1; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_min_u(i16x8_splat(-1), i16x8_splat(1))),
        [1; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_max_s(i16x8_splat(-1), i16x8_splat(1))),
        [1; 8]
    );
    assert_eq!(
        i16_lanes(i16x8_max_u(i16x8_splat(-1), i16x8_splat(1))),
        [-1; 8]
    );
}

#[test]
fn i16x8_narrow_extend_and_extmul() {
    let narrowed = i16x8_narrow_i32x4_s(i32x4([70_000, -70_000, 5, -5]), i32x4([1, 2, 3, 4]));
    assert_eq!(i16_lanes(narrowed), [i16::MAX, i16::MIN, 5, -5, 1, 2, 3, 4]);
    let unsigned = i16x8_narrow_i32x4_u(i32x4([70_000, -5, 5, -6]), i32x4([1, 2, 3, 4]));
    assert_eq!(i16_lanes(unsigned), [u16::MAX as i16, 0, 5, 0, 1, 2, 3, 4]);

    let source = V128::from_bytes([
        0, 1, 2, 3, 4, 5, 6, 7, 200, 201, 202, 203, 204, 205, 206, 207,
    ]);
    assert_eq!(
        i16_lanes(i16x8_extend_high_i8x16_u(source)),
        [200, 201, 202, 203, 204, 205, 206, 207]
    );
    assert_eq!(
        i16_lanes(i16x8_extadd_pairwise_i8x16_u(source)),
        [1, 5, 9, 13, 401, 405, 409, 413]
    );

    let factors = V128::from_bytes([200; 16]);
    let twos = V128::from_bytes([2; 16]);
    assert_eq!(i16_lanes(i16x8_extmul_low_i8x16_u(factors, twos)), [400; 8]);
    assert_eq!(
        i16_lanes(i16x8_extmul_high_i8x16_u(factors, twos)),
        [400; 8]
    );
}

#[test]
fn i32x4_compare_shift_and_widening() {
    assert!(!true_lane(i32x4_ne(i32x4_splat(4), i32x4_splat(4)), 0, 4));
    assert!(true_lane(i32x4_gt_s(i32x4_splat(1), i32x4_splat(-1)), 0, 4));
    assert!(!true_lane(
        i32x4_gt_u(i32x4_splat(1), i32x4_splat(-1)),
        0,
        4
    ));
    assert!(true_lane(
        i32x4_le_s(i32x4_splat(-1), i32x4_splat(-1)),
        0,
        4
    ));
    assert!(true_lane(i32x4_le_u(i32x4_splat(1), i32x4_splat(-1)), 0, 4));
    assert!(true_lane(
        i32x4_ge_s(i32x4_splat(-1), i32x4_splat(-1)),
        0,
        4
    ));
    assert!(true_lane(i32x4_ge_u(i32x4_splat(-1), i32x4_splat(1)), 0, 4));

    assert_eq!(i32_lanes(i32x4_abs(i32x4_splat(-3))), [3; 4]);
    assert_eq!(i32_lanes(i32x4_neg(i32x4_splat(-3))), [3; 4]);
    assert_eq!(i32x4_all_true(i32x4_splat(1)), 1);
    assert_eq!(i32x4_all_true(i32x4_splat(0)), 0);
    assert_eq!(i32x4_bitmask(i32x4([-1, 0, -1, 0])), 0b0101);
    assert_eq!(
        i32_lanes(i32x4_sub(i32x4_splat(1), i32x4_splat(2))),
        [-1; 4]
    );
    assert_eq!(
        i32_lanes(i32x4_min_u(i32x4_splat(-1), i32x4_splat(1))),
        [1; 4]
    );
    assert_eq!(
        i32_lanes(i32x4_max_s(i32x4_splat(-1), i32x4_splat(1))),
        [1; 4]
    );

    let source = i16x8([1, -2, 3, -4, 5, -6, 7, -8]);
    assert_eq!(i32_lanes(i32x4_extend_low_i16x8_s(source)), [1, -2, 3, -4]);
    assert_eq!(i32_lanes(i32x4_extend_high_i16x8_s(source)), [5, -6, 7, -8]);
    let unsigned = V128::from_bytes([1, 0, 0xff, 0xff, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0]);
    assert_eq!(
        i32_lanes(i32x4_extend_low_i16x8_u(unsigned)),
        [1, 0xffff, 3, 4]
    );
    assert_eq!(i32_lanes(i32x4_extend_high_i16x8_u(unsigned)), [5, 6, 7, 8]);

    assert_eq!(
        i32_lanes(i32x4_extadd_pairwise_i16x8_s(i16x8([
            1, 2, -3, -4, 5, 6, 7, 8
        ]))),
        [3, -7, 11, 15]
    );
    let unsigned = V128::from_bytes([0xff, 0xff, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    assert_eq!(
        i32_lanes(i32x4_extadd_pairwise_i16x8_u(unsigned)),
        [0x1_0000, 5, 9, 13]
    );

    let factors = i16x8([2, 3, 4, 5, 6, 7, 8, 9]);
    let twos = i16x8([10; 8]);
    assert_eq!(
        i32_lanes(i32x4_extmul_low_i16x8_s(factors, twos)),
        [20, 30, 40, 50]
    );
    assert_eq!(
        i32_lanes(i32x4_extmul_high_i16x8_s(factors, twos)),
        [60, 70, 80, 90]
    );
    let factors = V128::from_bytes([0xff, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0]);
    let twos = i16x8([2; 8]);
    assert_eq!(
        i32_lanes(i32x4_extmul_low_i16x8_u(factors, twos)),
        [510, 4, 6, 8]
    );
    assert_eq!(
        i32_lanes(i32x4_extmul_high_i16x8_u(factors, twos)),
        [10, 12, 14, 16]
    );
}

#[test]
fn i64x2_compare_and_widening() {
    assert_eq!(i64_lanes(i64x2_replace_lane::<1>(i64x2([1, 2]), 9)), [1, 9]);
    assert!(!true_lane(i64x2_ne(i64x2_splat(4), i64x2_splat(4)), 0, 8));
    assert!(true_lane(
        i64x2_le_s(i64x2_splat(-1), i64x2_splat(-1)),
        0,
        8
    ));
    assert!(true_lane(
        i64x2_ge_s(i64x2_splat(-1), i64x2_splat(-1)),
        0,
        8
    ));
    assert_eq!(i64_lanes(i64x2_neg(i64x2_splat(-3))), [3; 2]);
    assert_eq!(
        i64_lanes(i64x2_sub(i64x2_splat(1), i64x2_splat(2))),
        [-1; 2]
    );

    let source = i32x4([1, -2, 3, -4]);
    assert_eq!(i64_lanes(i64x2_extend_low_i32x4_s(source)), [1, -2]);
    assert_eq!(i64_lanes(i64x2_extend_high_i32x4_s(source)), [3, -4]);
    let unsigned = i32x4([-1, 2, 3, -4]);
    assert_eq!(
        i64_lanes(i64x2_extend_low_i32x4_u(unsigned)),
        [u32::MAX as i64, 2]
    );
    assert_eq!(
        i64_lanes(i64x2_extend_high_i32x4_u(unsigned)),
        [3, 4_294_967_292]
    );

    let factors = i32x4([2, 3, 4, 5]);
    let twos = i32x4([10; 4]);
    assert_eq!(i64_lanes(i64x2_extmul_low_i32x4_s(factors, twos)), [20, 30]);
    assert_eq!(
        i64_lanes(i64x2_extmul_high_i32x4_s(factors, twos)),
        [40, 50]
    );
    let unsigned = i32x4([-1, 2, 3, 4]);
    assert_eq!(
        i64_lanes(i64x2_extmul_low_i32x4_u(unsigned, twos)),
        [42_949_672_950, 20]
    );
    assert_eq!(
        i64_lanes(i64x2_extmul_high_i32x4_u(unsigned, twos)),
        [30, 40]
    );
}
