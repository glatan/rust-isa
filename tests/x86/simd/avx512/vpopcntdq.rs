use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512vpopcntdq::{vpopcntd, vpopcntq};

use super::support::*;

#[test]
fn avx512vpopcntdq() {
    if !is_x86_feature_detected!("avx512vpopcntdq") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let all = V512::from_bytes([0xff; 64]);
    let out = i32_lanes(unsafe { vpopcntd(all) });
    assert_eq!(out, [32; 16]);

    let out = i64_lanes(unsafe { vpopcntq(all) });
    assert_eq!(out, [64; 8]);
    let mixed = i64x8([0, 1, 0b1011, -1, i64::MIN, 3, 7, 15]);
    assert_eq!(
        i64_lanes(unsafe { vpopcntq(mixed) }),
        [0, 1, 3, 64, 1, 2, 3, 4]
    );
}
