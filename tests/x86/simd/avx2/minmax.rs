use rust_isa::x86::simd::avx2::{
    vpmaxsb, vpmaxsd, vpmaxsw, vpmaxub, vpmaxud, vpminsb, vpminsd, vpminsw, vpminub, vpminud,
};

use super::support::*;

#[test]
fn min_and_max() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    assert_eq!(
        unsafe { vpminub(u8x32([10; 32]), u8x32([20; 32])) }.to_bytes(),
        [10; 32]
    );
    assert_eq!(
        unsafe { vpmaxub(u8x32([10; 32]), u8x32([20; 32])) }.to_bytes(),
        [20; 32]
    );
    assert_eq!(
        unsafe { vpminsb(i8x32([-10; 32]), i8x32([10; 32])) }.to_bytes(),
        [0xf6; 32]
    );
    assert_eq!(
        unsafe { vpmaxsb(i8x32([-10; 32]), i8x32([10; 32])) }.to_bytes(),
        [10; 32]
    );
    assert_eq!(
        i16_lanes(unsafe { vpminsw(i16x16([-1_000; 16]), i16x16([1_000; 16])) }),
        [-1_000; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmaxsw(i16x16([-1_000; 16]), i16x16([1_000; 16])) }),
        [1_000; 16]
    );
    assert_eq!(
        i32_lanes(unsafe { vpminud(u32x8([u32::MAX; 8]), u32x8([1; 8])) }),
        [1; 8]
    );
    assert_eq!(
        unsafe { vpmaxud(u32x8([u32::MAX; 8]), u32x8([1; 8])) }.to_bytes(),
        [0xff; 32]
    );
    assert_eq!(
        i32_lanes(unsafe { vpminsd(i32x8([-1; 8]), i32x8([1; 8])) }),
        [-1; 8]
    );
    assert_eq!(
        i32_lanes(unsafe { vpmaxsd(i32x8([-1; 8]), i32x8([1; 8])) }),
        [1; 8]
    );
}
