use rust_isa::x86::simd::avx512f::*;
use rust_isa::x86::simd::avx512vnni::{vpdpbusds, vpdpwssd, vpdpwssds};

use super::support::*;

#[test]
fn vnni_extensions() {
    if !vnni_enabled() {
        return;
    }
    let acc = V512::from_bytes([0; 64]);
    let bytes = V512::from_bytes([1; 64]);
    let words = i16x32([1; 32]);
    assert_eq!(i32_lanes(unsafe { vpdpbusds(acc, bytes, bytes) }), [4; 16]);
    assert_eq!(i32_lanes(unsafe { vpdpwssd(acc, words, words) }), [2; 16]);
    assert_eq!(i32_lanes(unsafe { vpdpwssds(acc, words, words) }), [2; 16]);
}
