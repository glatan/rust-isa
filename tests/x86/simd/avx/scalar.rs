use rust_isa::x86::simd::avx::{vaddsd, vaddss};

use crate::simd::support::*;

#[test]
fn scalar_values() {
    if !is_x86_feature_detected!("avx") {
        return;
    }
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    let out = unsafe { vaddss(a, b) };
    assert_eq!(f32_lanes(out), [11.0, 2.0, 3.0, 4.0]);

    let a = f64x2([1.0, 2.0]);
    let b = f64x2([10.0, 20.0]);
    assert_eq!(f64_lanes(unsafe { vaddsd(a, b) }), [11.0, 2.0]);
}
