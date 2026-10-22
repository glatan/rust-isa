use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn foundation_float_and_convert() {
    if !f_enabled() {
        return;
    }
    let roots = f32_lanes(unsafe {
        vsqrtps(f32x16([
            4.0, 9.0, 16.0, 25.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]))
    });
    assert_eq!(roots[..4], [2.0, 3.0, 4.0, 5.0]);

    assert_eq!(f32_lanes(unsafe { vgetexpps(f32x16([3.0; 16])) })[0], 1.0);
    assert_eq!(
        f32_lanes(unsafe { vgetmantps::<0>(f32x16([3.0; 16])) })[0],
        1.5
    );
    assert_eq!(
        f32_lanes(unsafe { vscalefps(f32x16([1.0; 16]), f32x16([2.0; 16])) })[0],
        4.0
    );
    assert_eq!(
        f32_lanes(unsafe {
            vrndscaleps::<0>(f32x16([
                1.7, 2.2, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]))
        })[..3],
        [2.0, 2.0, -2.0]
    );

    let doubles = unsafe { vcvtps2pd(v256_from_f32x16_prefix(f32x16([1.0; 16]))) };
    assert_eq!(f64_lanes(doubles)[0], 1.0);
    let singles = unsafe { vcvtpd2ps(f64x8([2.0; 8])) };
    assert_eq!(f32x8_lanes(singles)[0], 2.0);
    assert_eq!(i32_lanes(unsafe { vcvttps2dq(f32x16([1.9; 16])) })[0], 1);
    assert_eq!(f32_lanes(unsafe { vcvtdq2ps(i32x16([7; 16])) })[0], 7.0);
}
