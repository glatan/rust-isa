use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx10_1::{
    vaddps_v256, vcmppd_v256, vcvtdq2ps_v256, vpcompressb_v256, vpermi2b_v256,
};
use rust_isa::x86::simd::avx512vl::{
    vpaddb_v128, vpaddb_v256, vpermb_v256, vpmovd2m_v128, vpmovm2d_v128, vpshldd_v128,
};
use rust_isa::x86::simd::sse2::V128;

use super::avx512::support::*;

fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn f32x8_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x4(values: [f64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x4_values(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| {
        i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
    })
}

#[test]
fn avx10_1_alias_values() {
    if !(f_enabled()
        && is_x86_feature_detected!("avx512vl")
        && is_x86_feature_detected!("avx512bw")
        && is_x86_feature_detected!("avx512vbmi")
        && is_x86_feature_detected!("avx512vbmi2"))
    {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    assert_eq!(
        f32x8_lanes(unsafe { vaddps_v256(a, b) }),
        [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0]
    );

    let ints = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        f32x8_lanes(unsafe { vcvtdq2ps_v256(ints) }),
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );

    assert_eq!(
        unsafe { vcmppd_v256::<0>(f64x4([1.0; 4]), f64x4([1.0; 4])) },
        0x0f
    );

    let bytes = V256::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vpcompressb_v256(bytes, 0b0101) }.to_bytes();
    assert_eq!(&compressed[..4], &[0, 2, 0, 0]);
    assert!(compressed[4..].iter().all(|value| *value == 0));

    let other = V256::from_bytes(core::array::from_fn(|index| index as u8 + 32));
    let indices = V256::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(
        unsafe { vpermi2b_v256(bytes, other, indices) }.to_bytes(),
        bytes.to_bytes()
    );
}

#[test]
fn avx512vl_alias_values() {
    if !(f_enabled() && is_x86_feature_detected!("avx512vl")) {
        return;
    }
    if is_x86_feature_detected!("avx512bw") {
        assert_eq!(
            unsafe { vpaddb_v128(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])) }.to_bytes(),
            [3; 16]
        );
        assert_eq!(
            unsafe { vpaddb_v256(V256::from_bytes([1; 32]), V256::from_bytes([2; 32])) }.to_bytes(),
            [3; 32]
        );
    }
    if is_x86_feature_detected!("avx512vbmi") {
        let source = V256::from_bytes(core::array::from_fn(|index| index as u8));
        let indices = V256::from_bytes(core::array::from_fn(|index| 31 - index as u8));
        assert_eq!(
            unsafe { vpermb_v256(source, indices) }.to_bytes(),
            core::array::from_fn::<u8, 32, _>(|index| 31 - index as u8)
        );
    }
    if is_x86_feature_detected!("avx512vbmi2") {
        let shifted = unsafe {
            vpshldd_v128::<4>(
                V128::from_bytes([0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                V128::from_bytes([0; 16]),
            )
        };
        assert_eq!(i32x4_values(shifted)[0], 0x120);
    }
    if is_x86_feature_detected!("avx512dq") {
        let mask = unsafe {
            vpmovd2m_v128(V128::from_bytes([
                0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
            ]))
        };
        assert_eq!(mask, 0b0101);
        let values = unsafe { vpmovm2d_v128(0b0101) };
        assert_eq!(i32x4_values(values), [-1, 0, -1, 0]);
    }
}
