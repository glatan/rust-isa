use rust_isa::x86::simd::avx2::{vpblendd, vpblendw};

use super::support::*;

#[test]
fn blends_immediate() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 101));
    assert_eq!(
        i16_lanes(unsafe { vpblendw::<0x55>(a, b) }),
        [
            101, 2, 103, 4, 105, 6, 107, 8, 109, 10, 111, 12, 113, 14, 115, 16
        ]
    );

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 101));
    assert_eq!(
        i32_lanes(unsafe { vpblendd::<0x55>(a, b) }),
        [101, 2, 103, 4, 105, 6, 107, 8]
    );
}
