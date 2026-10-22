use rust_isa::x86::simd::avx::{
    vcvtdq2ps, vcvtpd2ps, vcvtps2dq, vcvtps2pd, vcvtsd2ss, vcvtss2sd, vcvttps2dq, vmovq,
    vmovq_to_u64,
};

use super::support::*;

#[test]
fn conversions_and_moves() {
    if !enabled() {
        return;
    }
    assert_eq!(
        f64_lanes(unsafe { vcvtps2pd(f32x4([1.5, 2.5, 3.5, 4.5])) }),
        [1.5, 2.5, 3.5, 4.5]
    );
    assert_eq!(
        f32x4_lanes(unsafe { vcvtpd2ps(f64x4([1.5, 2.5, 3.5, 4.5])) }),
        [1.5, 2.5, 3.5, 4.5]
    );
    assert_eq!(
        f64x2_lanes(unsafe { vcvtss2sd(f64x2([9.0, 8.0]), f32x4([1.5, 0.0, 0.0, 0.0])) }),
        [1.5, 8.0]
    );
    assert_eq!(
        f32x4_lanes(unsafe { vcvtsd2ss(f32x4([9.0, 8.0, 7.0, 6.0]), f64x2([1.5, 0.0])) }),
        [1.5, 8.0, 7.0, 6.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vcvtdq2ps(i32x8([1, -2, 3, 4, 5, 6, 7, 8])) }),
        [1.0, -2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );
    assert_eq!(
        i32_lanes(unsafe { vcvtps2dq(f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])) }),
        [1, 2, 3, 4, 5, 6, 7, 8]
    );
    assert_eq!(
        i32_lanes(unsafe { vcvttps2dq(f32x8([1.9, -1.9, 2.1, -2.1, 3.9, -3.9, 4.1, -4.1])) }),
        [1, -1, 2, -2, 3, -3, 4, -4]
    );
    assert_eq!(
        unsafe { vmovq_to_u64(vmovq(0x0123_4567_89ab_cdef)) },
        0x0123_4567_89ab_cdef
    );
}
