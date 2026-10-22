use rust_isa::x86::simd::fma4::{
    macc_pd, macc_ps, vfmaddpd, vfmaddps, vfmsubps, vfnmaddps, vfnmsubps,
};
use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::xop::{vpcomb, vpcomub, vphaddbw};

fn fma4_available() -> bool {
    if !is_x86_feature_detected!("avx") {
        return false;
    }
    let leaf = core::arch::x86_64::__cpuid(0x8000_0001);
    leaf.ecx & (1 << 16) != 0
}

fn xop_available() -> bool {
    if !is_x86_feature_detected!("avx") {
        return false;
    }
    let leaf = core::arch::x86_64::__cpuid(0x8000_0001);
    leaf.ecx & (1 << 11) != 0
}

fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

#[test]
fn fma4_values() {
    if !fma4_available() {
        return;
    }

    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    let c = f32x4([0.5, 1.5, 2.5, 3.5]);

    assert_eq!(
        f32_lanes(unsafe { vfmaddps(a, b, c) }),
        [10.5, 41.5, 92.5, 163.5]
    );
    assert_eq!(
        f32_lanes(unsafe { macc_ps(a, b, c) }),
        [10.5, 41.5, 92.5, 163.5]
    );
    assert_eq!(
        f32_lanes(unsafe { vfmsubps(a, b, c) }),
        [9.5, 38.5, 87.5, 156.5]
    );
    assert_eq!(
        f32_lanes(unsafe { vfnmaddps(a, b, c) }),
        [-9.5, -38.5, -87.5, -156.5]
    );
    assert_eq!(
        f32_lanes(unsafe { vfnmsubps(a, b, c) }),
        [-10.5, -41.5, -92.5, -163.5]
    );

    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    let c = f64x2([0.5, 1.5]);
    assert_eq!(f64_lanes(unsafe { vfmaddpd(a, b, c) }), [10.5, 41.5]);
    assert_eq!(f64_lanes(unsafe { macc_pd(a, b, c) }), [10.5, 41.5]);
}

#[test]
fn xop_values() {
    if !xop_available() {
        return;
    }

    let a = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let b = V128::from_bytes([1, 0, 3, 0, 5, 0, 7, 0, 9, 0, 11, 0, 13, 0, 15, 0]);
    assert_eq!(
        unsafe { vpcomb(a, b, 4) }.to_bytes(),
        [
            0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0, 0xff, 0,
        ]
    );
    assert_eq!(
        unsafe { vpcomb(V128::from_bytes([1; 16]), V128::from_bytes([2; 16]), 0) }.to_bytes(),
        [0xff; 16]
    );
    assert_eq!(
        unsafe { vpcomub(V128::from_bytes([1; 16]), V128::from_bytes([2; 16]), 0) }.to_bytes(),
        [0xff; 16]
    );
    assert_eq!(
        unsafe { vpcomub(V128::from_bytes([0xff; 16]), V128::from_bytes([1; 16]), 0) }.to_bytes(),
        [0; 16]
    );

    let input = V128::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(
        i16_lanes(unsafe { vphaddbw(input) }),
        [3, 7, 11, 15, 19, 23, 27, 31]
    );
}
