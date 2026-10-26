use rust_isa::x86::simd::avx512dq::{
    vandpd, vcmppd, vcmpps, vcvtpd2qq, vcvtqq2pd, vfixupimmpd, vfpclasspd, vfpclassps, vgetexppd,
    vgetmantpd, vpmovd2m, vpmovm2d, vpmullq, vrcp14ps, vreducepd, vscalefpd,
};
use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn dq_integer_mullo() {
    if !dq_enabled() {
        return;
    }
    assert_eq!(
        i64_lanes(unsafe { vpmullq(i64x8([3; 8]), i64x8([5; 8])) }),
        [15; 8]
    );
}

#[test]
fn dq_float_and_convert() {
    if !dq_enabled() {
        return;
    }
    let a = f64x8([1.0; 8]);
    let b = f64x8([2.0; 8]);
    assert_eq!(f64_lanes(unsafe { vandpd(a, b) }), [0.0; 8]);
    assert_eq!(
        unsafe { vcmpps::<0>(f32x16([1.0; 16]), f32x16([1.0; 16])) },
        0xffff
    );
    assert_eq!(unsafe { vcmppd::<0>(a, b) }, 0);

    let classes = f64x8([f64::INFINITY, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(unsafe { vfpclasspd::<0x08>(classes) }, 0b1);
    assert_eq!(unsafe { vfpclassps::<0x08>(f32x16([0.0; 16])) }, 0);

    assert_eq!(f64_lanes(unsafe { vsqrtpd(f64x8([4.0; 8])) }), [2.0; 8]);
    assert_eq!(f64_lanes(unsafe { vgetexppd(f64x8([3.0; 8])) })[0], 1.0);
    assert_eq!(
        f64_lanes(unsafe { vgetmantpd::<0>(f64x8([3.0; 8])) })[0],
        1.5
    );
    assert_eq!(
        f64_lanes(unsafe { vscalefpd(f64x8([1.0; 8]), f64x8([3.0; 8])) })[0],
        8.0
    );
    assert_eq!(
        f64_lanes(unsafe { vreducepd::<0>(f64x8([0.0; 8])) })[0],
        0.0
    );
    assert!((f32_lanes(unsafe { vrcp14ps(f32x16([4.0; 16])) })[0] - 0.25).abs() < 0.001);

    assert_eq!(i64_lanes(unsafe { vcvtpd2qq(f64x8([3.0; 8])) }), [3; 8]);
    assert_eq!(f64_lanes(unsafe { vcvtqq2pd(i64x8([5; 8])) }), [5.0; 8]);
    // Token 7 (positive finite) selects table bits 31:28. Use different
    // response codes per lane so lane-wise corruption is visible.
    let table = i64x8([
        0x8 << 28,
        0x9 << 28,
        0xa << 28,
        0xb << 28,
        0x8 << 28,
        0x9 << 28,
        0xa << 28,
        0xb << 28,
    ]);
    assert_eq!(
        f64_lanes(unsafe {
            vfixupimmpd::<0>(
                f64x8([2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
                f64x8([1.5; 8]),
                table,
            )
        }),
        [0.0, -1.0, 1.0, 0.5, 0.0, -1.0, 1.0, 0.5]
    );
}

#[test]
fn dq_mask_conversions() {
    if !dq_enabled() {
        return;
    }
    assert_eq!(i32_lanes(unsafe { vpmovm2d(0b0101) })[..4], [-1, 0, -1, 0]);
    assert_eq!(
        unsafe { vpmovd2m(i32x16([-1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])) },
        0b0101
    );
}
