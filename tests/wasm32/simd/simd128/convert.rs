use rust_isa::wasm32::simd::simd128::*;

use super::lanes::{f32_lanes, f32x4, f64_lanes, f64x2, i16_lanes, i16x8, i32_lanes, i32x4};

fn q15mulr(a: i16, b: i16) -> i16 {
    let product = i32::from(a) * i32::from(b);
    let shifted = (product + 0x4000) >> 15;
    shifted.clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16
}

#[test]
fn extend_narrow_extmul_and_dot() {
    let mut bytes = [0u8; 16];
    bytes[0] = 0xff;
    bytes[1] = 0x01;
    bytes[8] = 0xff;
    bytes[9] = 0x02;
    let source = V128::from_bytes(bytes);
    assert_eq!(i16_lanes(i16x8_extend_low_i8x16_s(source))[0], -1);
    assert_eq!(i16_lanes(i16x8_extend_low_i8x16_s(source))[1], 1);
    assert_eq!(i16_lanes(i16x8_extend_low_i8x16_u(source))[0], 255);
    assert_eq!(i16_lanes(i16x8_extend_high_i8x16_s(source))[0], -1);
    assert_eq!(i16_lanes(i16x8_extend_high_i8x16_s(source))[1], 2);

    let wide = i16x8([200, -200, 1, 0, 0, 0, 0, 0]);
    let zero = i16x8([0; 8]);
    let signed = i8x16_narrow_i16x8_s(wide, zero);
    assert_eq!(signed.to_bytes()[0], 127);
    assert_eq!(signed.to_bytes()[1] as i8, -128);
    let unsigned = i8x16_narrow_i16x8_u(wide, zero);
    assert_eq!(unsigned.to_bytes()[0], 200);
    assert_eq!(unsigned.to_bytes()[1], 0);

    let left = i16x8([2, 3, 4, 5, 0, 0, 0, 0]);
    let right = i16x8([4, 5, 6, 7, 0, 0, 0, 0]);
    assert_eq!(i32_lanes(i32x4_dot_i16x8_s(left, right))[0], 23);
    assert_eq!(i32_lanes(i32x4_dot_i16x8_s(left, right))[1], 59);

    let factors_a = i8_from([2, 3, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0]);
    let factors_b = i8_from([4, 5, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        i16_lanes(i16x8_extmul_low_i8x16_s(factors_a, factors_b))[0],
        8
    );
    assert_eq!(
        i16_lanes(i16x8_extmul_low_i8x16_s(factors_a, factors_b))[1],
        15
    );
    assert_eq!(
        i16_lanes(i16x8_extmul_high_i8x16_s(factors_a, factors_b))[0],
        18
    );
    assert_eq!(
        i16_lanes(i16x8_extadd_pairwise_i8x16_s(i8_from([
            100, 100, -100, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]))),
        [200, -150, 0, 0, 0, 0, 0, 0]
    );
}

fn i8_from(values: [i8; 16]) -> V128 {
    V128::from_bytes(values.map(|value| value as u8))
}

#[test]
fn q15mulr_saturates_and_rounds() {
    let left = i16x8([0x4000, -32768, 1, 100, 0, 0, 0, 0]);
    let right = i16x8([0x4000, -32768, 1, -50, 0, 0, 0, 0]);
    let expected =
        [0, 1, 2, 3, 4, 5, 6, 7].map(|lane| q15mulr(i16_lanes(left)[lane], i16_lanes(right)[lane]));
    assert_eq!(i16_lanes(i16x8_q15mulr_sat_s(left, right)), expected);
    assert_eq!(expected[0], 0x2000);
    assert_eq!(expected[1], 32767);
}

#[test]
fn trunc_sat_convert_promote_and_swizzle() {
    let floats = f32x4([1.9, -1.9, 1.0e20, -1.0e20]);
    assert_eq!(
        i32_lanes(i32x4_trunc_sat_f32x4_s(floats)),
        f32_lanes(floats).map(|lane| lane as i32)
    );
    assert_eq!(
        i32_lanes(i32x4_trunc_sat_f32x4_u(floats)).map(|lane| lane as u32),
        f32_lanes(floats).map(|lane| lane as u32)
    );
    assert_eq!(
        i32_lanes(f32x4_convert_i32x4_s(i32x4([-2, 3, 0, 100]))),
        i32_lanes(f32x4([-2.0, 3.0, 0.0, 100.0]))
    );

    let promoted = f64x2_promote_low_f32x4(f32x4([1.5, -2.0, 9.0, 8.0]));
    assert_eq!(f64_lanes(promoted), [1.5, -2.0]);
    let demoted = f32x4_demote_f64x2_zero(f64x2([1.5, -2.0]));
    assert_eq!(f32_lanes(demoted), [1.5, -2.0, 0.0, 0.0]);
    assert_eq!(
        f64_lanes(f64x2_convert_low_i32x4_s(i32x4([-4, 6, 1, 1]))),
        [-4.0, 6.0]
    );
    let unsigned = i32x4([-1, 6, 1, 1]);
    assert_eq!(
        f32_lanes(f32x4_convert_i32x4_u(unsigned)),
        i32_lanes(unsigned).map(|lane| lane as u32 as f32)
    );
    assert_eq!(
        f64_lanes(f64x2_convert_low_i32x4_u(unsigned)),
        [u32::MAX as f64, 6.0]
    );

    let special = f32x4([f32::NAN, f32::NEG_INFINITY, f32::INFINITY, -1.0]);
    assert_eq!(
        i32_lanes(i32x4_trunc_sat_f32x4_s(special)),
        f32_lanes(special).map(|lane| lane as i32)
    );
    assert_eq!(
        i32_lanes(i32x4_trunc_sat_f32x4_u(special)).map(|lane| lane as u32),
        f32_lanes(special).map(|lane| lane as u32)
    );
    assert_eq!(
        i32_lanes(i32x4_trunc_sat_f64x2_s_zero(f64x2([1.9, -1.9]))),
        [1, -1, 0, 0]
    );
    assert_eq!(
        i32_lanes(i32x4_trunc_sat_f64x2_u_zero(f64x2([-1.5, 4.2]))).map(|lane| lane as u32),
        [0, 4, 0, 0]
    );

    let source = V128::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let indices = V128::from_bytes([15, 0, 2, 16, 1, 1, 7, 8, 9, 10, 11, 12, 13, 14, 20, 3]);
    let mut expected = [0u8; 16];
    for (lane, elem) in expected.iter_mut().enumerate() {
        let index = indices.to_bytes()[lane];
        *elem = if index < 16 {
            source.to_bytes()[index as usize]
        } else {
            0
        };
    }
    assert_eq!(i8x16_swizzle(source, indices).to_bytes(), expected);
}
