use rust_isa::x86::simd::avx512dq::vpmullq;

use super::support::*;

#[test]
fn dq_mullo() {
    if !dq_enabled() {
        return;
    }
    assert_eq!(
        i64_lanes(unsafe { vpmullq(i64x8([3; 8]), i64x8([5; 8])) }),
        [15; 8]
    );
}
