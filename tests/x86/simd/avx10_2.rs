use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx10_2::{vcomisd_v128, vcomiss_v128, vminmaxps_v256};
use rust_isa::x86::simd::sse2::V128;

fn avx10_2_available() -> bool {
    use core::arch::x86_64::__cpuid_count;

    if !is_x86_feature_detected!("avx512f") {
        return false;
    }
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

fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

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

#[test]
fn avx10_2_compare_values() {
    if !avx10_2_available() {
        return;
    }

    let one = f32x4([1.0, 0.0, 0.0, 0.0]);
    let two = f32x4([2.0, 0.0, 0.0, 0.0]);
    let nan = f32x4([f32::NAN, 0.0, 0.0, 0.0]);
    assert_eq!(unsafe { vcomiss_v128(one, two) }, 1);
    assert_eq!(unsafe { vcomiss_v128(one, one) }, 4);
    assert_eq!(unsafe { vcomiss_v128(two, one) }, 0);
    assert_eq!(unsafe { vcomiss_v128(nan, one) }, 7);

    let one = f64x2([1.0, 0.0]);
    let two = f64x2([2.0, 0.0]);
    let nan = f64x2([f64::NAN, 0.0]);
    assert_eq!(unsafe { vcomisd_v128(one, two) }, 1);
    assert_eq!(unsafe { vcomisd_v128(one, one) }, 4);
    assert_eq!(unsafe { vcomisd_v128(two, one) }, 0);
    assert_eq!(unsafe { vcomisd_v128(nan, one) }, 7);
}

#[test]
fn avx10_2_minmax_values() {
    if !avx10_2_available() {
        return;
    }

    let a = f32x8([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = f32x8([8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
    // imm8[3:2] = 01 (sign control), imm8[1:0] = 00 (min) / 01 (max).
    assert_eq!(
        f32x8_lanes(unsafe { vminmaxps_v256::<0x4>(a, b) }),
        [1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vminmaxps_v256::<0x5>(a, b) }),
        [8.0, 7.0, 6.0, 5.0, 5.0, 6.0, 7.0, 8.0]
    );

    let a = f32x8([1.0, -1.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0]);
    let b = f32x8([-2.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]);
    assert_eq!(
        f32x8_lanes(unsafe { vminmaxps_v256::<0x4>(a, b) }),
        [-2.0, -1.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0]
    );
    assert_eq!(
        f32x8_lanes(unsafe { vminmaxps_v256::<0x5>(a, b) }),
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );
}
