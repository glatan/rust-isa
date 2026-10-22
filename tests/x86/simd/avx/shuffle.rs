use rust_isa::x86::simd::avx::{
    vpsadbw, vshufpd, vshufps, vunpckhpd, vunpckhps, vunpcklpd, vunpcklps,
};
use rust_isa::x86::simd::sse2::V128;

use super::support::*;

#[test]
fn shuffle_and_unpack() {
    if !enabled() {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0]);
    assert_eq!(
        f32_lanes(unsafe { vshufps::<0x1b>(a, b) }),
        [4.0, 3.0, 102.0, 101.0, 8.0, 7.0, 106.0, 105.0]
    );
    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([101.0, 102.0, 103.0, 104.0]);
    assert_eq!(
        f64_lanes(unsafe { vshufpd::<0b01>(a, b) }),
        [2.0, 101.0, 3.0, 103.0]
    );

    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0]);
    assert_eq!(
        f32_lanes(unsafe { vunpcklps(a, b) }),
        [1.0, 101.0, 2.0, 102.0, 5.0, 105.0, 6.0, 106.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vunpckhps(a, b) }),
        [3.0, 103.0, 4.0, 104.0, 7.0, 107.0, 8.0, 108.0]
    );
    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([101.0, 102.0, 103.0, 104.0]);
    assert_eq!(
        f64_lanes(unsafe { vunpcklpd(a, b) }),
        [1.0, 101.0, 3.0, 103.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vunpckhpd(a, b) }),
        [2.0, 102.0, 4.0, 104.0]
    );

    assert_eq!(
        i64x2_lanes(unsafe { vpsadbw(V128::from_bytes([1; 16]), V128::from_bytes([0; 16])) }),
        [8; 2]
    );
}
