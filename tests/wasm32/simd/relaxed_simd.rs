use rust_isa::wasm32::simd::relaxed_simd::{
    f32x4_relaxed_madd, f32x4_relaxed_max, f32x4_relaxed_min, f32x4_relaxed_nmadd,
    f64x2_relaxed_madd, f64x2_relaxed_max, f64x2_relaxed_min, f64x2_relaxed_nmadd,
    i8x16_relaxed_laneselect, i8x16_relaxed_swizzle, i16x8_relaxed_dot_i8x16_i7x16_s,
    i16x8_relaxed_laneselect, i16x8_relaxed_q15mulr_s, i32x4_relaxed_dot_i8x16_i7x16_add_s,
    i32x4_relaxed_laneselect, i32x4_relaxed_trunc_f32x4_s, i32x4_relaxed_trunc_f32x4_u,
    i32x4_relaxed_trunc_f64x2_s_zero, i32x4_relaxed_trunc_f64x2_u_zero, i64x2_relaxed_laneselect,
};
use rust_isa::wasm32::simd::simd128::{
    V128, f32x4_extract_lane, f32x4_splat, f64x2_extract_lane, f64x2_splat, i16x8_extract_lane_s,
    i32x4_extract_lane,
};

#[test]
fn exact_float_arithmetic() {
    let product = f32x4_relaxed_madd(f32x4_splat(2.0), f32x4_splat(3.0), f32x4_splat(4.0));
    assert_eq!(f32x4_extract_lane::<0>(product), 10.0);
    let neg = f32x4_relaxed_nmadd(f32x4_splat(2.0), f32x4_splat(3.0), f32x4_splat(4.0));
    assert_eq!(f32x4_extract_lane::<0>(neg), -2.0);
    assert_eq!(
        f32x4_extract_lane::<0>(f32x4_relaxed_min(f32x4_splat(1.0), f32x4_splat(3.0))),
        1.0
    );
    assert_eq!(
        f32x4_extract_lane::<0>(f32x4_relaxed_max(f32x4_splat(1.0), f32x4_splat(3.0))),
        3.0
    );

    let product = f64x2_relaxed_madd(f64x2_splat(2.0), f64x2_splat(3.0), f64x2_splat(4.0));
    assert_eq!(f64x2_extract_lane::<0>(product), 10.0);
    let neg = f64x2_relaxed_nmadd(f64x2_splat(2.0), f64x2_splat(3.0), f64x2_splat(4.0));
    assert_eq!(f64x2_extract_lane::<0>(neg), -2.0);
    assert_eq!(
        f64x2_extract_lane::<0>(f64x2_relaxed_min(f64x2_splat(1.0), f64x2_splat(3.0))),
        1.0
    );
    assert_eq!(
        f64x2_extract_lane::<0>(f64x2_relaxed_max(f64x2_splat(1.0), f64x2_splat(3.0))),
        3.0
    );
}

#[test]
fn in_range_truncation() {
    let mut bytes = [0u8; 16];
    bytes[0..4].copy_from_slice(&1.9f32.to_le_bytes());
    bytes[4..8].copy_from_slice(&(-1.9f32).to_le_bytes());
    let signed = i32x4_relaxed_trunc_f32x4_s(V128::from_bytes(bytes));
    assert_eq!(i32x4_extract_lane::<0>(signed), 1);
    assert_eq!(i32x4_extract_lane::<1>(signed), -1);
    let unsigned = i32x4_relaxed_trunc_f32x4_u(V128::from_bytes(bytes));
    assert_eq!(i32x4_extract_lane::<0>(unsigned), 1);

    bytes = [0; 16];
    bytes[..8].copy_from_slice(&1.9f64.to_le_bytes());
    bytes[8..].copy_from_slice(&(-1.9f64).to_le_bytes());
    let signed = i32x4_relaxed_trunc_f64x2_s_zero(V128::from_bytes(bytes));
    assert_eq!(i32x4_extract_lane::<0>(signed), 1);
    assert_eq!(i32x4_extract_lane::<1>(signed), -1);
    assert_eq!(i32x4_extract_lane::<2>(signed), 0);
    bytes = [0; 16];
    bytes[..8].copy_from_slice(&1.9f64.to_le_bytes());
    let unsigned = i32x4_relaxed_trunc_f64x2_u_zero(V128::from_bytes(bytes));
    assert_eq!(i32x4_extract_lane::<0>(unsigned), 1);
    assert_eq!(i32x4_extract_lane::<2>(unsigned), 0);
}

#[test]
fn swizzle_and_lane_masks() {
    let source = V128::from_bytes([0xab, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let indices = V128::from_bytes([16, 0xff, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = i8x16_relaxed_swizzle(source, indices).to_bytes();
    assert!(out[0] == 0 || out[0] == 0xab, "{}", out[0]);
    assert_eq!(out[1], 0);
    assert_eq!(out[2], 1);

    let a = V128::from_bytes([0x3c; 16]);
    let b = V128::from_bytes([0x0f; 16]);
    let selected = i8x16_relaxed_laneselect(a, b, V128::from_bytes([0x80; 16])).to_bytes();
    assert!(
        selected == [0x0f; 16] || selected == [0x3c; 16],
        "{selected:?}"
    );
    assert_eq!(
        i8x16_relaxed_laneselect(a, b, V128::from_bytes([0xff; 16])).to_bytes(),
        [0x3c; 16]
    );
    assert_eq!(
        i8x16_relaxed_laneselect(a, b, V128::from_bytes([0x00; 16])).to_bytes(),
        [0x0f; 16]
    );
    assert_eq!(
        i16x8_relaxed_laneselect(a, b, V128::from_bytes([0xff; 16])).to_bytes(),
        [0x3c; 16]
    );
    assert_eq!(
        i32x4_relaxed_laneselect(a, b, V128::from_bytes([0x00; 16])).to_bytes(),
        [0x0f; 16]
    );
    assert_eq!(
        i64x2_relaxed_laneselect(a, b, V128::from_bytes([0xff; 16])).to_bytes(),
        [0x3c; 16]
    );
}

#[test]
fn q15_and_dot() {
    let value = V128::from_bytes([0x00, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        i16x8_extract_lane_s::<0>(i16x8_relaxed_q15mulr_s(value, value)),
        0x2000
    );
    let min = V128::from_bytes([0x00, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let lane = i16x8_extract_lane_s::<0>(i16x8_relaxed_q15mulr_s(min, min));
    assert!(
        lane == i32::from(i16::MIN) || lane == i32::from(i16::MAX),
        "{lane}"
    );

    let a = V128::from_bytes([1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        i16x8_extract_lane_s::<0>(i16x8_relaxed_dot_i8x16_i7x16_s(a, b)),
        17
    );
    assert_eq!(
        i32x4_extract_lane::<0>(i32x4_relaxed_dot_i8x16_i7x16_add_s(
            a,
            b,
            V128::from_bytes([0; 16])
        )),
        70
    );

    let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let lane = i16x8_extract_lane_s::<0>(i16x8_relaxed_dot_i8x16_i7x16_s(a, b));
    assert!(lane == -1 || lane == 255, "{lane}");
}
