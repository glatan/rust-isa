use rust_isa::x86::simd::avx::{vroundpd, vroundps, vroundsd, vroundss};

use super::support::*;

#[test]
fn round() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = f32x8([1.7, 2.2, 3.8, 4.1, -1.7, -2.2, -3.8, -4.1]);
    assert_eq!(
        f32_lanes(unsafe { vroundps::<0>(a) }),
        [2.0, 2.0, 4.0, 4.0, -2.0, -2.0, -4.0, -4.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vroundps::<1>(a) }),
        [1.0, 2.0, 3.0, 4.0, -2.0, -3.0, -4.0, -5.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vroundps::<2>(a) }),
        [2.0, 3.0, 4.0, 5.0, -1.0, -2.0, -3.0, -4.0]
    );
    let a = f64x4([1.7, 2.2, 3.8, 4.1]);
    assert_eq!(f64_lanes(unsafe { vroundpd::<0>(a) }), [2.0, 2.0, 4.0, 4.0]);
    let a = f32x4([9.0; 4]);
    let b = f32x4([1.7, 2.2, 3.8, 4.1]);
    assert_eq!(
        f32x4_lanes(unsafe { vroundss::<0>(a, b) }),
        [2.0, 9.0, 9.0, 9.0]
    );
    let a = f64x2([9.0, 9.0]);
    let b = f64x2([1.7, 2.2]);
    assert_eq!(f64x2_lanes(unsafe { vroundsd::<0>(a, b) }), [2.0, 9.0]);
}
