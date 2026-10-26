use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512f::*;

use crate::simd::avx512::support::*;

#[test]
fn avx10_1_integer_and_float() {
    if !f_enabled() {
        return;
    }
    use rust_isa::x86::simd::avx10_1::{
        vcvtps2pd_v256, vpaddb_v256, vpaddq_v512, vpcmpd_v512, vpopcntd_v512, vsqrtps_v512,
    };
    if bw_enabled() && is_x86_feature_detected!("avx512vl") {
        let sum = unsafe { vpaddb_v256(V256::from_bytes([1; 32]), V256::from_bytes([2; 32])) };
        assert_eq!(sum.to_bytes(), [3; 32]);
    }
    let sum = unsafe { vpaddq_v512(i64x8([1; 8]), i64x8([2; 8])) };
    assert_eq!(i64_lanes(sum), [3; 8]);
    assert_eq!(
        unsafe { vpcmpd_v512::<0>(i32x16([1; 16]), i32x16([1; 16])) },
        0xffff
    );
    if is_x86_feature_detected!("avx512vpopcntdq") {
        assert_eq!(
            i32_lanes(unsafe { vpopcntd_v512(V512::from_bytes([0xff; 64])) })[0],
            32
        );
    }
    assert_eq!(
        f32_lanes(unsafe { vsqrtps_v512(f32x16([9.0; 16])) })[0],
        3.0
    );
    if vl_enabled() {
        let doubles = unsafe { vcvtps2pd_v256(v256_from_f32x16_prefix(f32x16([2.0; 16]))) };
        assert_eq!(f64_lanes(doubles)[0], 2.0);
    }

    use rust_isa::x86::simd::avx10_1::{
        vcvtpd2qq_v512, vgatherdps_v256, vpdpbusds_v512, vpshldd_v256, vptestnmd_v512,
    };
    if is_x86_feature_detected!("avx512dq") {
        assert_eq!(
            i64_lanes(unsafe { vcvtpd2qq_v512(f64x8([3.0; 8])) }),
            [3; 8]
        );
    }
    assert_eq!(
        unsafe { vptestnmd_v512(i32x16([0; 16]), i32x16([0; 16])) },
        0xffff
    );
    if is_x86_feature_detected!("avx512vnni") {
        assert_eq!(
            i32_lanes(unsafe {
                vpdpbusds_v512(
                    V512::from_bytes([0; 64]),
                    V512::from_bytes([1; 64]),
                    V512::from_bytes([1; 64]),
                )
            }),
            [4; 16]
        );
    }
    if is_x86_feature_detected!("avx512vbmi2") && is_x86_feature_detected!("avx512vl") {
        let shifted = unsafe { vpshldd_v256::<4>(i32x8([0x12; 8]), i32x8([0; 8])) };
        assert_eq!(i32x8_lanes(shifted)[0], 0x120);
    }
    if vl_enabled() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        let gathered = unsafe {
            vgatherdps_v256(
                f32x8([0.0; 8]),
                data.as_ptr().cast(),
                i32x8([0, 1, 2, 3, 0, 0, 0, 0]),
                0x0f,
            )
        };
        assert_eq!(f32x8_lanes(gathered)[..4], [1.0, 2.0, 3.0, 4.0]);
    }
}

#[test]
fn avx10_1_compress_expand() {
    if !is_x86_feature_detected!("avx512vbmi2") {
        return;
    }
    use rust_isa::x86::simd::avx10_1::{
        vpcompressb_v256, vpcompressb_v512, vpcompressw_v256, vpcompressw_v512, vpexpandb_v256,
        vpexpandb_v512, vpexpandw_v256, vpexpandw_v512,
    };

    if is_x86_feature_detected!("avx512vl") {
        let bytes256 = V256::from_bytes(core::array::from_fn(|index| index as u8));
        let compressed = unsafe { vpcompressb_v256(bytes256, 0b0101) }.to_bytes();
        assert_eq!(compressed[..4], [0, 2, 0, 0]);
        assert!(compressed[4..].iter().all(|value| *value == 0));

        let expanded = unsafe { vpexpandb_v256(bytes256, 0b0101) }.to_bytes();
        assert_eq!(expanded[..4], [0, 0, 1, 0]);
        assert!(expanded[4..].iter().all(|value| *value == 0));
    }

    let bytes512 = V512::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vpcompressb_v512(bytes512, 0b0101) }.to_bytes();
    assert_eq!(compressed[..4], [0, 2, 0, 0]);
    assert!(compressed[4..].iter().all(|value| *value == 0));

    let expanded = unsafe { vpexpandb_v512(bytes512, 0b0101) }.to_bytes();
    assert_eq!(expanded[..4], [0, 0, 1, 0]);
    assert!(expanded[4..].iter().all(|value| *value == 0));

    if is_x86_feature_detected!("avx512vl") {
        let words = V256::from_bytes(core::array::from_fn(|index| index as u8));
        let compressed = unsafe { vpcompressw_v256(words, 0b0101) }.to_bytes();
        assert_eq!(compressed[..6], [0, 1, 4, 5, 0, 0]);
        assert!(compressed[6..].iter().all(|value| *value == 0));

        let expanded = unsafe { vpexpandw_v256(words, 0b0101) }.to_bytes();
        assert_eq!(expanded[..6], [0, 1, 0, 0, 2, 3]);
        assert!(expanded[6..].iter().all(|value| *value == 0));
    }

    let words = V512::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vpcompressw_v512(words, 0b0101) }.to_bytes();
    assert_eq!(compressed[..6], [0, 1, 4, 5, 0, 0]);
    assert!(compressed[6..].iter().all(|value| *value == 0));

    let expanded = unsafe { vpexpandw_v512(words, 0b0101) }.to_bytes();
    assert_eq!(expanded[..6], [0, 1, 0, 0, 2, 3]);
    assert!(expanded[6..].iter().all(|value| *value == 0));
}
