use rust_isa::x86::simd::avx512ifma::{vpmadd52huq, vpmadd52luq};

use super::support::*;

#[test]
fn ifma() {
    if !ifma_enabled() {
        return;
    }
    let acc = i64x8([7; 8]);
    let a = i64x8([3; 8]);
    let b = i64x8([5; 8]);
    assert_eq!(i64_lanes(unsafe { vpmadd52luq(acc, a, b) }), [22; 8]);
    let a_high = i64x8([1 << 26; 8]);
    let b_high = i64x8([1 << 27; 8]);
    assert_eq!(
        i64_lanes(unsafe { vpmadd52huq(acc, a_high, b_high) }),
        [9; 8]
    );
}
