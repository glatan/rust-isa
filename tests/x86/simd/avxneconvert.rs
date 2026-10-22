use rust_isa::x86::simd::avxneconvert::{
    vbcstnebf162ps, vcvtneebf162ps, vcvtneepbf162ps, vcvtneeph2ps, vcvtneepsh2ps,
    vcvtneps2bf16 as vcvtneps2bf16_256,
};

use crate::simd::avx512::support::*;

#[test]
fn avx_ne_convert_packed() {
    if !is_x86_feature_detected!("avxneconvert") {
        return;
    }
    let even_bf16 = u16x16(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x3f80 } else { 0x4000 }
    }));
    assert_eq!(f32x8_lanes(unsafe { vcvtneebf162ps(even_bf16) }), [1.0; 8]);
    assert_eq!(f32x8_lanes(unsafe { vcvtneepbf162ps(even_bf16) }), [2.0; 8]);

    let even_half = u16x16(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x3c00 } else { 0x4000 }
    }));
    assert_eq!(f32x8_lanes(unsafe { vcvtneeph2ps(even_half) }), [1.0; 8]);
    assert_eq!(f32x8_lanes(unsafe { vcvtneepsh2ps(even_half) }), [2.0; 8]);
}

#[test]
fn avx_ne_convert_broadcast() {
    if !is_x86_feature_detected!("avxneconvert") {
        return;
    }
    assert_eq!(f32x8_lanes(unsafe { vbcstnebf162ps(0x3f80) }), [1.0; 8]);
    let out = unsafe { vcvtneps2bf16_256(i32x8_as_f32([1.0; 8])) }.to_bytes();
    assert_eq!(u16::from_le_bytes(out[0..2].try_into().unwrap()), 0x3f80);
}
