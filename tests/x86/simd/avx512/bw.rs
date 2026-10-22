use rust_isa::x86::simd::avx512bw::{vpaddb, vpmullw, vpsubb};
use rust_isa::x86::simd::avx512f::V512;

use super::support::*;

#[test]
fn bw_bytes() {
    if !bw_enabled() {
        return;
    }
    let bytes = V512::from_bytes([250; 64]);
    assert_eq!(
        unsafe { vpaddb(bytes, V512::from_bytes([10; 64])) }.to_bytes(),
        [4; 64]
    );
}

#[test]
fn byte_word_arithmetic() {
    if !bw_enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpsubb(V512::from_bytes([10; 64]), V512::from_bytes([3; 64])) }.to_bytes(),
        [7; 64]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmullw(i16x32([300; 32]), i16x32([3; 32])) }),
        [900; 32]
    );
}
