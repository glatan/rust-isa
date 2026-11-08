use rust_isa::x86::simd::avx2::{vpslld, vpsllvd, vpsllvq, vpsravd, vpsrld, vpsrlvd, vpsrlvq};

use super::support::*;

#[test]
fn variable_shifts() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    assert_eq!(
        i32_lanes(unsafe {
            vpsllvd(
                i32x8([1, 2, 3, 4, 5, 6, 7, 8]),
                i32x8([0, 1, 2, 3, 4, 5, 6, 7]),
            )
        }),
        [1, 4, 12, 32, 80, 192, 448, 1024]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsrlvd(i32x8([16; 8]), i32x8([0, 1, 2, 3, 4, 5, 6, 7])) }),
        [16, 8, 4, 2, 1, 0, 0, 0]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsravd(i32x8([-16; 8]), i32x8([0, 1, 2, 3, 4, 5, 6, 7])) }),
        [-16, -8, -4, -2, -1, -1, -1, -1]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsllvq(i64x4([1, 2, 3, 4]), i64x4([1, 2, 3, 4])) }),
        [2, 8, 24, 64]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsrlvq(i64x4([64; 4]), i64x4([0, 1, 2, 3])) }),
        [64, 32, 16, 8]
    );
}

#[test]
fn immediate_shifts() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    assert_eq!(
        i32_lanes(unsafe { vpslld(i32x8([1, 2, 3, 4, 5, 6, 7, 8]), 1) }),
        [2, 4, 6, 8, 10, 12, 14, 16]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsrld(i32x8([-1; 8]), 1) }),
        [0x7fff_ffff; 8]
    );
    assert_eq!(
        i32_lanes(unsafe { vpslld(i32x8([1; 8]), 31) }),
        [i32::MIN; 8]
    );
    assert_eq!(i32_lanes(unsafe { vpslld(i32x8([1; 8]), 32) }), [0; 8]);
}
