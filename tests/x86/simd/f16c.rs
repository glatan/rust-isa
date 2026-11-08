use rust_isa::x86::simd::f16c::{vcvtph2ps, vcvtph2ps_v256, vcvtps2ph, vcvtps2ph_v256};

use super::support::*;

#[test]
fn convert() {
    if !is_x86_feature_detected!("f16c") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let floats = f32x4([1.0, 0.5, -2.0, 3.0]);
    let half = unsafe { vcvtps2ph::<0>(floats) };
    assert_eq!(f32_lanes(unsafe { vcvtph2ps(half) }), [1.0, 0.5, -2.0, 3.0]);

    let floats = f32x8([1.0, 0.5, -2.0, 3.0, 4.0, -0.25, 8.0, 16.0]);
    let half = unsafe { vcvtps2ph_v256::<0>(floats) };
    assert_eq!(
        f32x8_lanes(unsafe { vcvtph2ps_v256(half) }),
        [1.0, 0.5, -2.0, 3.0, 4.0, -0.25, 8.0, 16.0]
    );
}
