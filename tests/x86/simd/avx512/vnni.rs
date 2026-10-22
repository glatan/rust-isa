use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512vnni::vpdpbusd;

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
