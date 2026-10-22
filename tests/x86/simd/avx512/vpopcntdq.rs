use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512vpopcntdq::vpopcntd;

use super::support::*;

#[test]
fn avx512vpopcntdq() {
    if !is_x86_feature_detected!("avx512vpopcntdq") {
        return;
    }
    let all = V512::from_bytes([0xff; 64]);
    let out = i32_lanes(unsafe { vpopcntd(all) });
    assert_eq!(&out[..4], &[32, 32, 32, 32]);
}
