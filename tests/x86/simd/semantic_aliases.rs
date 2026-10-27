use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx10_1::{
    vaddps_v256, vcmppd_v256, vcvtdq2ps_v256, vpcompressb_v256, vpermi2b_v256, vpermi2w_v256,
    vpopcntw_v256,
};
use rust_isa::x86::simd::avx10_2::{vfpclassps_v256, vgetexpps_v256, vrndscaleps_v256};
use rust_isa::x86::simd::avx512vl::{
    vpaddb_v128, vpaddb_v256, vpcmpd_v128, vpermb_v256, vpermq_v256, vpmovb2m_v128, vpmovd2m_v128,
    vpmovm2d_v128, vpopcntb_v128, vpshldd_v128, vpshldw_v128, vpshufbitqmb_v128, vpternlogd_v128,
    vptestmd_v128,
};
use rust_isa::x86::simd::sse2::V128;

use super::avx512::support::*;

fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn f32x8_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn f64x4(values: [f64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x4_values(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|index| {
        i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
    })
}

fn u16x16(values: [u16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn u16x16_lanes(value: V256) -> [u16; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u16::from_le_bytes(bytes[lane * 2..lane * 2 + 2].try_into().unwrap())
    })
}

fn u64x4(values: [u64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn u64x4_lanes(value: V256) -> [u64; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        u64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn avx10_2_available() -> bool {
    use core::arch::x86_64::__cpuid_count;
    let leaf7 = __cpuid_count(7, 0);
    if leaf7.eax < 1 {
        return false;
    }
    let avx10 = __cpuid_count(7, 1);
    if avx10.edx & (1 << 19) == 0 {
        return false;
    }
    let version = __cpuid_count(0x24, 0);
    (version.ebx & 0xff) >= 2
}

#[test]
fn avx10_1_alias_values() {
    if !(f_enabled()
        && is_x86_feature_detected!("avx512vl")
        && is_x86_feature_detected!("avx512bw")
        && is_x86_feature_detected!("avx512vbmi")
        && is_x86_feature_detected!("avx512vbmi2"))
    {
        return;
    }
    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
    assert_eq!(
        f32x8_lanes(unsafe { vaddps_v256(a, b) }),
        [11.0, 22.0, 33.0, 44.0, 55.0, 66.0, 77.0, 88.0]
    );

    let ints = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        f32x8_lanes(unsafe { vcvtdq2ps_v256(ints) }),
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );

    assert_eq!(
        unsafe { vcmppd_v256::<0>(f64x4([1.0; 4]), f64x4([1.0; 4])) },
        0x0f
    );

    let bytes = V256::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vpcompressb_v256(bytes, 0b0101) }.to_bytes();
    assert_eq!(&compressed[..4], &[0, 2, 0, 0]);
    assert!(compressed[4..].iter().all(|value| *value == 0));

    let other = V256::from_bytes(core::array::from_fn(|index| index as u8 + 32));
    let indices = V256::from_bytes(core::array::from_fn(|index| index as u8));
    assert_eq!(
        unsafe { vpermi2b_v256(bytes, other, indices) }.to_bytes(),
        bytes.to_bytes()
    );
}

#[test]
fn avx512vl_alias_values() {
    if !(f_enabled() && is_x86_feature_detected!("avx512vl")) {
        return;
    }
    if is_x86_feature_detected!("avx512bw") {
        assert_eq!(
            unsafe { vpaddb_v128(V128::from_bytes([1; 16]), V128::from_bytes([2; 16])) }.to_bytes(),
            [3; 16]
        );
        assert_eq!(
            unsafe { vpaddb_v256(V256::from_bytes([1; 32]), V256::from_bytes([2; 32])) }.to_bytes(),
            [3; 32]
        );
    }
    if is_x86_feature_detected!("avx512vbmi") {
        let source = V256::from_bytes(core::array::from_fn(|index| index as u8));
        let indices = V256::from_bytes(core::array::from_fn(|index| 31 - index as u8));
        assert_eq!(
            unsafe { vpermb_v256(source, indices) }.to_bytes(),
            core::array::from_fn::<u8, 32, _>(|index| 31 - index as u8)
        );
    }
    if is_x86_feature_detected!("avx512vbmi2") {
        let shifted = unsafe {
            vpshldd_v128::<4>(
                V128::from_bytes([0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                V128::from_bytes([0; 16]),
            )
        };
        assert_eq!(i32x4_values(shifted)[0], 0x120);
    }
    if is_x86_feature_detected!("avx512dq") {
        let mask = unsafe {
            vpmovd2m_v128(V128::from_bytes([
                0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
            ]))
        };
        assert_eq!(mask, 0b0101);
        let values = unsafe { vpmovm2d_v128(0b0101) };
        assert_eq!(i32x4_values(values), [-1, 0, -1, 0]);
    }
}

#[test]
fn avx10_1_permute_and_popcount_values() {
    if !(f_enabled() && is_x86_feature_detected!("avx512vl")) {
        return;
    }
    if is_x86_feature_detected!("avx512bw") {
        let a = u16x16(core::array::from_fn(|lane| lane as u16));
        let b = u16x16(core::array::from_fn(|lane| 100 + lane as u16));
        let indices = u16x16([0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]);
        assert_eq!(
            u16x16_lanes(unsafe { vpermi2w_v256(a, b, indices) }),
            [
                0, 100, 1, 101, 2, 102, 3, 103, 4, 104, 5, 105, 6, 106, 7, 107
            ]
        );
    }
    if is_x86_feature_detected!("avx512bitalg") {
        let counts = unsafe { vpopcntw_v256(V256::from_bytes([0xff; 32])) };
        assert_eq!(u16x16_lanes(counts), [16; 16]);
    }
}

#[test]
fn avx512vl_mask_and_ternlog_values() {
    if !(f_enabled() && is_x86_feature_detected!("avx512vl")) {
        return;
    }
    let all = V128::from_bytes([0xff; 16]);
    let low = V128::from_bytes([0x0f; 16]);
    let high = V128::from_bytes([0xf0; 16]);
    assert_eq!(
        unsafe { vpternlogd_v128::<0x96>(all, low, high) }.to_bytes(),
        [0; 16]
    );
    assert_eq!(
        unsafe { vpcmpd_v128::<0>(i32x4([1, 2, 3, 4]), i32x4([1, 9, 3, 9])) },
        0b0101
    );
    assert_eq!(
        unsafe { vptestmd_v128(i32x4([1, 2, 4, 8]), i32x4([1, 0, 4, 0])) },
        0b0101
    );
    assert_eq!(
        u64x4_lanes(unsafe { vpermq_v256(u64x4([10, 20, 30, 40]), u64x4([3, 2, 1, 0])) }),
        [40, 30, 20, 10]
    );
    let shifted = unsafe { vpshldw_v128::<4>(u16x8([0x0012, 0, 0, 0, 0, 0, 0, 0]), u16x8([0; 8])) };
    assert_eq!(
        u16::from_le_bytes(shifted.to_bytes()[..2].try_into().unwrap()),
        0x0120
    );
    if is_x86_feature_detected!("avx512bw") {
        let mask = unsafe {
            vpmovb2m_v128(V128::from_bytes([
                0x80, 0x7f, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]))
        };
        assert_eq!(mask, 0b0101);
    }
    if is_x86_feature_detected!("avx512bitalg") {
        let counts = unsafe {
            vpopcntb_v128(V128::from_bytes([
                0, 1, 3, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]))
        };
        assert_eq!(&counts.to_bytes()[..4], &[0, 1, 2, 8]);

        let mut source = [0u8; 16];
        source[7] = 0x80;
        source[8] = 1;
        let mut indices = [1u8; 16];
        indices[0] = 63;
        indices[1] = 62;
        indices[8] = 0;
        assert_eq!(
            unsafe { vpshufbitqmb_v128(V128::from_bytes(source), V128::from_bytes(indices),) },
            0x0101
        );
    }
}

#[test]
fn avx10_2_math_values() {
    if !(avx10_2_available() && f_enabled() && is_x86_feature_detected!("avx512dq")) {
        return;
    }
    assert_eq!(
        f32x8_lanes(unsafe { vgetexpps_v256(f32x8([1.0, 2.0, 4.0, 0.5, -8.0, 3.0, 1.5, 0.25])) }),
        [0.0, 1.0, 2.0, -1.0, 3.0, 1.0, 0.0, -2.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe {
            vrndscaleps_v256::<0>(f32x8([1.4, 1.6, 2.5, -2.5, 0.5, -0.5, 3.5, -3.5]))
        }),
        [1.0, 2.0, 2.0, -2.0, 0.0, 0.0, 4.0, -4.0]
    );
    assert_eq!(
        unsafe {
            vfpclassps_v256::<0x08>(f32x8([f32::INFINITY, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]))
        },
        0b0000_0001
    );
}
