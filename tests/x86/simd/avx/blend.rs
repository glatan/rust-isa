use rust_isa::x86::simd::avx::{vblendpd, vblendps, vblendvpd, vblendvps};

use super::support::*;

#[test]
fn blends() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0]);
    assert_eq!(
        f32_lanes(unsafe { vblendps::<0x55>(a, b) }),
        [101.0, 2.0, 103.0, 4.0, 105.0, 6.0, 107.0, 8.0]
    );
    let a = f64x4([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4([101.0, 102.0, 103.0, 104.0]);
    assert_eq!(
        f64_lanes(unsafe { vblendpd::<0b0101>(a, b) }),
        [101.0, 2.0, 103.0, 4.0]
    );

    let a = f32x8([1.0; 8]);
    let b = f32x8([2.0; 8]);
    let mask = f32x8([-1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0]);
    assert_eq!(
        f32_lanes(unsafe { vblendvps(a, b, mask) }),
        [2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]
    );
    let a = f64x4([1.0; 4]);
    let b = f64x4([2.0; 4]);
    let mask = f64x4([-1.0, 0.0, -1.0, 0.0]);
    assert_eq!(
        f64_lanes(unsafe { vblendvpd(a, b, mask) }),
        [2.0, 1.0, 2.0, 1.0]
    );
}
