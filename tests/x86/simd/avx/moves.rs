use rust_isa::x86::simd::avx::{vmovd, vmovd_to_u32, vmovddup, vmovmskpd, vmovshdup, vmovsldup};

use super::support::*;

#[test]
fn moves() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    assert_eq!(
        u32x4_lanes(unsafe { vmovd(0x1234_5678) }),
        [0x1234_5678, 0, 0, 0]
    );
    assert_eq!(unsafe { vmovd_to_u32(vmovd(0x1234_5678)) }, 0x1234_5678);
    assert_eq!(unsafe { vmovmskpd(f64x4([-1.0, 1.0, -2.0, 2.0])) }, 0b0101);
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    assert_eq!(
        f32_lanes(unsafe { vmovshdup(a) }),
        [2.0, 2.0, 4.0, 4.0, 6.0, 6.0, 8.0, 8.0]
    );
    assert_eq!(
        f32_lanes(unsafe { vmovsldup(a) }),
        [1.0, 1.0, 3.0, 3.0, 5.0, 5.0, 7.0, 7.0]
    );
    assert_eq!(
        f64_lanes(unsafe { vmovddup(f64x4([1.0, 2.0, 3.0, 4.0])) }),
        [1.0, 1.0, 3.0, 3.0]
    );
}
