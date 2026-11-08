use rust_isa::x86::simd::avx2::{vphaddd, vphaddsw, vphaddw, vphsubd, vphsubsw, vphsubw};

use super::support::*;

#[test]
fn horizontal() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    let b = i16x16(core::array::from_fn(|lane| lane as i16 + 17));
    assert_eq!(
        i16_lanes(unsafe { vphaddw(a, b) }),
        [3, 7, 11, 15, 35, 39, 43, 47, 19, 23, 27, 31, 51, 55, 59, 63]
    );
    assert_eq!(
        i16_lanes(unsafe { vphaddsw(i16x16([30_000; 16]), i16x16([30_000; 16])) }),
        [32_767; 16]
    );
    assert_eq!(i16_lanes(unsafe { vphsubw(a, b) }), [-1; 16]);

    let a = i32x8(core::array::from_fn(|lane| lane as i32 + 1));
    let b = i32x8(core::array::from_fn(|lane| lane as i32 + 9));
    assert_eq!(
        i32_lanes(unsafe { vphaddd(a, b) }),
        [3, 7, 19, 23, 11, 15, 27, 31]
    );
    assert_eq!(i32_lanes(unsafe { vphsubd(a, b) }), [-1; 8]);
    let a = i16x16(core::array::from_fn(|lane| {
        if lane % 2 == 0 { 30_000 } else { -30_000 }
    }));
    assert_eq!(i16_lanes(unsafe { vphsubsw(a, a) }), [32_767; 16]);
}
