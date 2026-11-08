use rust_isa::x86::simd::avx2::{vpgatherdd, vpgatherdq, vpgatherqd, vpgatherqq};

use super::support::*;

#[test]
fn gathers() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let data = [10i32, 20, 30, 40, 50, 60, 70, 80];
    assert_eq!(
        i32_lanes(unsafe {
            vpgatherdd(
                i32x8([0; 8]),
                data.as_ptr().cast(),
                i32x8([0, 4, 8, 12, 16, 20, 24, 28]),
                i32x8([-1; 8]),
            )
        }),
        data
    );
    assert_eq!(
        i32_lanes(unsafe {
            vpgatherdd(
                i32x8([99; 8]),
                data.as_ptr().cast(),
                i32x8([0, 4, 8, 12, 16, 20, 24, 28]),
                i32x8([0, -1, -1, -1, -1, -1, -1, -1]),
            )
        }),
        [99, 20, 30, 40, 50, 60, 70, 80]
    );

    let data = [10i32, 20, 30, 40];
    assert_eq!(
        i32x4_lanes(unsafe {
            vpgatherqd(
                i32x4([0; 4]),
                data.as_ptr().cast(),
                i64x4([0, 4, 8, 12]),
                i32x4([-1; 4]),
            )
        }),
        data
    );

    let data = [10i64, 20, 30, 40];
    assert_eq!(
        i64_lanes(unsafe {
            vpgatherdq(
                i64x4([0; 4]),
                data.as_ptr().cast(),
                i32x4([0, 8, 16, 24]),
                i64x4([-1; 4]),
            )
        }),
        data
    );
    assert_eq!(
        i64_lanes(unsafe {
            vpgatherqq(
                i64x4([0; 4]),
                data.as_ptr().cast(),
                i64x4([0, 8, 16, 24]),
                i64x4([-1; 4]),
            )
        }),
        data
    );
}
