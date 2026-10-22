use rust_isa::x86::simd::avx::{
    vaddpd, vaddps, vaddsubpd, vaddsubps, vandnpd, vandnps, vandpd, vdivps, vmaxpd, vmaxps, vminpd,
    vminps, vmulpd, vmulps, vrcpps, vsqrtpd, vsqrtps, vsubps, vxorpd,
};
use rust_isa::x86::simd::avx2::V256;

use super::support::*;

#[test]
fn float_arithmetic() {
    if !is_x86_feature_detected!("avx") {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    assert_eq!(
        f32x8_lanes(unsafe { vaddps(a, b) }),
        [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vmulps(a, b) }),
        [10.0, 40.0, 90.0, 160.0, 250.0, 360.0, 490.0, 640.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vdivps(b, a) }),
        [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vsubps(b, a) }),
        [9.0, 18.0, 27.0, 36.0, 45.0, 54.0, 63.0, 72.0]
    );

    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        f64x4_lanes(unsafe { vaddpd(a, b) }),
        [11.0, 22.0, 33.0, 44.0]
    );
    assert_eq!(
        f64x4_lanes(unsafe { vmulpd(a, b) }),
        [10.0, 40.0, 90.0, 160.0]
    );
}

#[test]
fn arithmetic_and_bitwise() {
    if !enabled() {
        return;
    }
    assert_eq!(
        f32_lanes(unsafe { vaddsubps(f32x8([1.0; 8]), f32x8([2.0; 8])) }),
        [-1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vaddsubpd(f64x4([1.0; 4]), f64x4([2.0; 4])) }),
        [-1.0, 3.0, -1.0, 3.0]
    );

    let a = V256::from_bytes([0b1010_1010; 32]);
    let b = V256::from_bytes([0b1100_1100; 32]);
    assert_eq!(unsafe { vandpd(a, b) }.to_bytes(), [0b1000_1000; 32]);
    assert_eq!(unsafe { vandnps(a, b) }.to_bytes(), [0b0100_0100; 32]);
    assert_eq!(unsafe { vandnpd(a, b) }.to_bytes(), [0b0100_0100; 32]);
    assert_eq!(unsafe { vxorpd(a, b) }.to_bytes(), [0b0110_0110; 32]);

    let one = f32x8([1.0; 8]);
    let two = f32x8([2.0; 8]);
    assert_eq!(f32_lanes(unsafe { vmaxps(one, two) }), [2.0; 8]);
    assert_eq!(f32_lanes(unsafe { vminps(one, two) }), [1.0; 8]);
    let one = f64x4([1.0; 4]);
    let two = f64x4([2.0; 4]);
    assert_eq!(f64_lanes(unsafe { vmaxpd(one, two) }), [2.0; 4]);
    assert_eq!(f64_lanes(unsafe { vminpd(one, two) }), [1.0; 4]);

    assert_eq!(
        f32_lanes(unsafe { vsqrtps(f32x8([4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0, 81.0])) }),
        [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vsqrtpd(f64x4([4.0, 9.0, 16.0, 25.0])) }),
        [2.0, 3.0, 4.0, 5.0]
    );
    let reciprocal = unsafe { vrcpps(f32x8([4.0; 8])) };
    for lane in f32_lanes(reciprocal) {
        assert!((lane - 0.25).abs() < 0.001);
    }
}
