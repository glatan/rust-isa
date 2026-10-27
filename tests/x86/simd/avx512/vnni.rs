use rust_isa::x86::simd::avx512f::*;
use rust_isa::x86::simd::avx512vnni::{vpdpbusd, vpdpbusds, vpdpwssd, vpdpwssds};

use super::support::*;

#[test]
fn vnni() {
    if !vnni_enabled() {
        return;
    }
    let acc = V512::from_bytes([0; 64]);
    let a = V512::from_bytes([1; 64]);
    let b = V512::from_bytes([1; 64]);
    assert_eq!(i32_lanes(unsafe { vpdpbusd(acc, a, b) }), [4; 16]);
}

#[test]
fn vnni_additional_accumulate() {
    if !vnni_enabled() {
        return;
    }
    let acc = V512::from_bytes([0; 64]);
    let bytes = V512::from_bytes([1; 64]);
    let words = i16x32([1; 32]);
    assert_eq!(i32_lanes(unsafe { vpdpbusds(acc, bytes, bytes) }), [4; 16]);
    assert_eq!(i32_lanes(unsafe { vpdpwssd(acc, words, words) }), [2; 16]);
    assert_eq!(i32_lanes(unsafe { vpdpwssds(acc, words, words) }), [2; 16]);

    let saturated_acc = i32x16([i32::MAX; 16]);
    let large_bytes = V512::from_bytes([0x7f; 64]);
    assert_eq!(
        i32_lanes(unsafe { vpdpbusds(saturated_acc, large_bytes, large_bytes) }),
        [i32::MAX; 16]
    );
    let large_words = i16x32([0x7fff; 32]);
    assert_eq!(
        i32_lanes(unsafe { vpdpwssds(saturated_acc, large_words, large_words) }),
        [i32::MAX; 16]
    );
}
