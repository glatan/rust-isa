use rust_isa::x86::simd::avx512ifma::vpmadd52huq as vpmadd52huq_512;
use rust_isa::x86::simd::avx512vl::*;
use rust_isa::x86::simd::sse2::V128;

use super::support::*;

#[test]
fn ifma_extensions() {
    if !ifma_enabled() {
        return;
    }
    let acc = i64x8([7; 8]);
    let a = i64x8([1 << 26; 8]);
    let b = i64x8([1 << 27; 8]);
    assert_eq!(i64_lanes(unsafe { vpmadd52huq_512(acc, a, b) }), [9; 8]);
    assert_eq!(
        i32x4_lanes(unsafe {
            vpmadd52luq_v128(
                V128::from_bytes([7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                V128::from_bytes([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                V128::from_bytes([5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            )
        })[0],
        22
    );
}
