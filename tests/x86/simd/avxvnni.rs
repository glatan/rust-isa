use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avxvnni::{vpdpbusd, vpdpbusd_v128, vpdpbusds_v128};
use rust_isa::x86::simd::avxvnniint8::{vpdpbssd, vpdpbuud, vpdpbuuds_v128};
use rust_isa::x86::simd::avxvnniint16::{vpdpwusds_v128, vpdpwuud, vpdpwuuds};
use rust_isa::x86::simd::sse2::V128;

use crate::simd::avx512::support::*;

#[test]
fn avx_vnni() {
    let zero128 = V128::from_bytes([0; 16]);
    if is_x86_feature_detected!("avxvnni") {
        let a = V256::from_bytes([1; 32]);
        let b = V256::from_bytes([2; 32]);
        let zero = V256::from_bytes([0; 32]);
        assert_eq!(i32x8_lanes(unsafe { vpdpbusd(zero, a, b) }), [8; 8]);

        let a = V128::from_bytes([1; 16]);
        let b = V128::from_bytes([2; 16]);
        assert_eq!(i32x4_lanes(unsafe { vpdpbusd_v128(zero128, a, b) }), [8; 4]);
        assert_eq!(
            i32x4_lanes(unsafe { vpdpbusds_v128(i32x4([i32::MAX; 4]), a, b) }),
            [i32::MAX; 4]
        );

        let a = u16x8([1; 8]);
        let b = u16x8([2; 8]);
        assert_eq!(
            i32x4_lanes(unsafe { rust_isa::x86::simd::avxvnni::vpdpwssd_v128(zero128, a, b) }),
            [4; 4]
        );
        assert_eq!(
            i32x4_lanes(unsafe {
                rust_isa::x86::simd::avxvnni::vpdpwssds_v128(i32x4([i32::MAX; 4]), a, b)
            }),
            [i32::MAX; 4]
        );
    }
    if is_x86_feature_detected!("avxvnniint8") {
        let a = V256::from_bytes([1; 32]);
        let b = V256::from_bytes([2; 32]);
        let zero = V256::from_bytes([0; 32]);
        assert_eq!(i32x8_lanes(unsafe { vpdpbssd(zero, a, b) }), [8; 8]);

        let a = V128::from_bytes([1; 16]);
        let b = V128::from_bytes([2; 16]);
        assert_eq!(
            i32x4_lanes(unsafe { rust_isa::x86::simd::avxvnniint8::vpdpbssd_v128(zero128, a, b) }),
            [4; 4]
        );

        let all = V256::from_bytes([0xff; 32]);
        assert_eq!(
            i32x8_lanes(unsafe { vpdpbuud(zero, all, all) }),
            [260100; 8]
        );
        assert_eq!(
            i32x4_lanes(unsafe {
                vpdpbuuds_v128(
                    i32x4([-1; 4]),
                    V128::from_bytes([0xff; 16]),
                    V128::from_bytes([0xff; 16]),
                )
            }),
            [-1; 4]
        );
    }
    if is_x86_feature_detected!("avxvnniint16") {
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        for lane in 0..16 {
            a[lane * 2..lane * 2 + 2].copy_from_slice(&1u16.to_le_bytes());
            b[lane * 2..lane * 2 + 2].copy_from_slice(&2u16.to_le_bytes());
        }
        let zero = V256::from_bytes([0; 32]);
        assert_eq!(
            i32x8_lanes(unsafe { vpdpwuud(zero, V256::from_bytes(a), V256::from_bytes(b)) }),
            [4; 8]
        );

        let all = u16x16([0xffff; 16]);
        assert_eq!(
            i32x8_lanes(unsafe { vpdpwuuds(i32x8([i32::MAX; 8]), all, all) }),
            [-1; 8]
        );
        assert_eq!(
            i32x4_lanes(unsafe {
                vpdpwusds_v128(i32x4([i32::MAX; 4]), u16x8([0xffff; 8]), i16x8([1; 8]))
            }),
            [i32::MAX; 4]
        );
    }
}
