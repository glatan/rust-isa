use rust_isa::x86::simd::avx2::{
    vpaddsb, vpaddsw, vpaddusb, vpaddusw, vpsubsb, vpsubsw, vpsubusb, vpsubusw,
};

use super::support::*;

#[test]
fn saturating_arithmetic() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpaddsb(i8x32([100; 32]), i8x32([100; 32])) }.to_bytes(),
        [127; 32]
    );
    assert_eq!(
        unsafe { vpsubsb(i8x32([-100; 32]), i8x32([100; 32])) }.to_bytes(),
        [0x80; 32]
    );
    assert_eq!(
        unsafe { vpaddusb(u8x32([200; 32]), u8x32([100; 32])) }.to_bytes(),
        [255; 32]
    );
    assert_eq!(
        unsafe { vpsubusb(u8x32([100; 32]), u8x32([200; 32])) }.to_bytes(),
        [0; 32]
    );

    assert_eq!(
        i16_lanes(unsafe { vpaddsw(i16x16([30_000; 16]), i16x16([30_000; 16])) }),
        [32_767; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpsubsw(i16x16([-30_000; 16]), i16x16([30_000; 16])) }),
        [-32_768; 16]
    );
    assert_eq!(
        u16_lanes(unsafe { vpaddusw(u16x16([60_000; 16]), u16x16([10_000; 16])) }),
        [65_535; 16]
    );
    assert_eq!(
        u16_lanes(unsafe { vpsubusw(u16x16([10_000; 16]), u16x16([60_000; 16])) }),
        [0; 16]
    );
}
