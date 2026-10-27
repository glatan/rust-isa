use rust_isa::x86::simd::avx512cd::{vpconflictd, vpconflictq, vplzcntd};

use super::support::*;

#[test]
fn avx512cd_leading_zeros_and_conflicts() {
    if !is_x86_feature_detected!("avx512cd") {
        return;
    }
    let values = i32x16([1, 2, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = i32_lanes(unsafe { vplzcntd(values) });
    assert_eq!(&out[..5], &[31, 30, 30, 0, 32]);

    let values = i32x16([1, 2, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = i32_lanes(unsafe { vpconflictd(values) });
    assert_eq!(out[0], 0);
    assert_eq!(out[1], 0);
    assert_eq!(out[2], 0b001);
    assert_eq!(out[3], 0);
    assert_eq!(out[4], 0b0101);

    let values = i64x8([1, 2, 1, 3, 1, 0, 0, 0]);
    let out = i64_lanes(unsafe { vpconflictq(values) });
    assert_eq!(&out[..5], &[0, 0, 0b001, 0, 0b0101]);
}
