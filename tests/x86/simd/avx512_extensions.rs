use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512bf16::{vcvtneps2bf16, vdpbf16ps};
use rust_isa::x86::simd::avx512bitalg::{vpopcntb, vpshufbitqmb};
use rust_isa::x86::simd::avx512cd::{vpconflictd, vplzcntd};
use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512fp16::{vaddph, vcvtph2psx, vcvtps2phx};
use rust_isa::x86::simd::avx512vbmi2::{vpcompressb, vpexpandb};
use rust_isa::x86::simd::avx512vp2intersect::vp2intersectd;
use rust_isa::x86::simd::avx512vpopcntdq::vpopcntd;
use rust_isa::x86::simd::avxifma::{vpmadd52huq_v128, vpmadd52luq, vpmadd52luq_v128};
use rust_isa::x86::simd::avxneconvert::{
    vbcstnebf162ps, vcvtneebf162ps, vcvtneepbf162ps, vcvtneeph2ps, vcvtneepsh2ps,
    vcvtneps2bf16 as vcvtneps2bf16_256,
};
use rust_isa::x86::simd::avxvnni::{vpdpbusd, vpdpbusd_v128, vpdpbusds_v128};
use rust_isa::x86::simd::avxvnniint8::{vpdpbssd, vpdpbuud, vpdpbuuds_v128};
use rust_isa::x86::simd::avxvnniint16::{vpdpwusds_v128, vpdpwuud, vpdpwuuds};
use rust_isa::x86::simd::sse2::V128;

fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i32x4_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn u16x8(values: [u16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V128::from_bytes(bytes)
}

fn i64x2(values: [i64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

fn i64x2_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    [
        i64::from_le_bytes(bytes[..8].try_into().unwrap()),
        i64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

fn u16x16(values: [u16; 16]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x16(values: [i32; 16]) -> V512 {
    let mut bytes = [0u8; 64];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn i32x16_lanes(value: V512) -> [i32; 16] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i32x8_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

fn i64x8(values: [i64; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}

fn i64x8_lanes(value: V256) -> [i64; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        i64::from_le_bytes(bytes[lane * 8..lane * 8 + 8].try_into().unwrap())
    })
}

fn f32x8_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    core::array::from_fn(|lane| {
        f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap())
    })
}

#[test]
fn avx512cd_leading_zeros_and_conflicts() {
    if !is_x86_feature_detected!("avx512cd") {
        return;
    }
    let values = i32x16([1, 2, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = i32x16_lanes(unsafe { vplzcntd(values) });
    assert_eq!(&out[..5], &[31, 30, 30, 0, 32]);

    let values = i32x16([1, 2, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = i32x16_lanes(unsafe { vpconflictd(values) });
    assert_eq!(out[0], 0);
    assert_eq!(out[1], 0);
    assert_eq!(out[2], 0b001);
    assert_eq!(out[3], 0);
    assert_eq!(out[4], 0b0101);
}

#[test]
fn avx512bitalg_and_vpopcntdq() {
    if !is_x86_feature_detected!("avx512bitalg") || !is_x86_feature_detected!("avx512vpopcntdq") {
        return;
    }
    let all = V512::from_bytes([0xff; 64]);
    let out = unsafe { vpopcntb(all) }.to_bytes();
    assert_eq!(&out[..4], &[8, 8, 8, 8]);

    let all = V512::from_bytes([0xff; 64]);
    assert_eq!(unsafe { vpshufbitqmb(all, all) }, u64::MAX);

    let all = V512::from_bytes([0xff; 64]);
    let out = i32x16_lanes(unsafe { vpopcntd(all) });
    assert_eq!(&out[..4], &[32, 32, 32, 32]);
}

#[test]
fn avx512vbmi2_compress_and_expand() {
    if !is_x86_feature_detected!("avx512vbmi2") {
        return;
    }
    let source = V512::from_bytes(core::array::from_fn(|index| index as u8));
    let compressed = unsafe { vpcompressb(source, 0b0101) }.to_bytes();
    assert_eq!(&compressed[..4], &[0, 2, 0, 0]);

    let source = V512::from_bytes(core::array::from_fn(|index| index as u8 + 10));
    let expanded = unsafe { vpexpandb(source, 0b0101) }.to_bytes();
    assert_eq!(&expanded[..4], &[10, 0, 11, 0]);
}

#[test]
fn avx512vp2intersect_masks() {
    if !is_x86_feature_detected!("avx512vp2intersect") {
        return;
    }
    let a = i32x16(core::array::from_fn(|index| index as i32));
    let b = i32x16(core::array::from_fn(|index| 15 - index as i32));
    assert_eq!(unsafe { vp2intersectd(a, b) }, (0xffff, 0xffff));
}

#[test]
fn avx512bf16_convert_and_dot() {
    if !is_x86_feature_detected!("avx512bf16") {
        return;
    }
    let mut floats = [0u8; 64];
    for lane in 0..16 {
        floats[lane * 4..lane * 4 + 4].copy_from_slice(&1.0f32.to_le_bytes());
    }
    let out = unsafe { vcvtneps2bf16(V512::from_bytes(floats)) }.to_bytes();
    assert_eq!(u16::from_le_bytes(out[0..2].try_into().unwrap()), 0x3f80);

    let mut a = [0u8; 64];
    let mut b = [0u8; 64];
    for lane in 0..32 {
        a[lane * 2..lane * 2 + 2].copy_from_slice(&0x3f80u16.to_le_bytes());
        b[lane * 2..lane * 2 + 2].copy_from_slice(&0x4000u16.to_le_bytes());
    }
    let out = unsafe {
        vdpbf16ps(
            V512::from_bytes([0; 64]),
            V512::from_bytes(a),
            V512::from_bytes(b),
        )
    };
    let bytes = out.to_bytes();
    for lane in 0..16 {
        assert_eq!(
            f32::from_le_bytes(bytes[lane * 4..lane * 4 + 4].try_into().unwrap()),
            4.0
        );
    }
}

#[test]
fn avx512fp16_arithmetic() {
    if !is_x86_feature_detected!("avx512fp16") {
        return;
    }
    let mut one = [0u8; 64];
    let mut two = [0u8; 64];
    for lane in 0..32 {
        one[lane * 2..lane * 2 + 2].copy_from_slice(&0x3c00u16.to_le_bytes());
        two[lane * 2..lane * 2 + 2].copy_from_slice(&0x4000u16.to_le_bytes());
    }
    let out = unsafe { vaddph(V512::from_bytes(one), V512::from_bytes(two)) }.to_bytes();
    assert_eq!(u16::from_le_bytes(out[0..2].try_into().unwrap()), 0x4200);

    let mut half = [0u8; 32];
    for lane in 0..16 {
        half[lane * 2..lane * 2 + 2].copy_from_slice(&0x3c00u16.to_le_bytes());
    }
    let out = unsafe { vcvtph2psx(V256::from_bytes(half)) }.to_bytes();
    assert_eq!(f32::from_le_bytes(out[0..4].try_into().unwrap()), 1.0);

    let mut floats = [0u8; 64];
    for lane in 0..16 {
        floats[lane * 4..lane * 4 + 4].copy_from_slice(&1.0f32.to_le_bytes());
    }
    let compressed = unsafe { vcvtps2phx(V512::from_bytes(floats)) }.to_bytes();
    assert_eq!(
        u16::from_le_bytes(compressed[0..2].try_into().unwrap()),
        0x3c00
    );
}

#[test]
fn avx_vnni_and_ifma() {
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
    if is_x86_feature_detected!("avxifma") {
        let a = i64x8([3; 8]);
        let b = i64x8([5; 8]);
        let zero = V256::from_bytes([0; 32]);
        assert_eq!(i64x8_lanes(unsafe { vpmadd52luq(zero, a, b) }), [15; 8]);

        assert_eq!(
            i64x2_lanes(unsafe { vpmadd52luq_v128(i64x2([0; 2]), i64x2([3; 2]), i64x2([5; 2])) }),
            [15; 2]
        );
        assert_eq!(
            i64x2_lanes(unsafe {
                vpmadd52huq_v128(
                    i64x2([0; 2]),
                    i64x2([1i64 << 26; 2]),
                    i64x2([1i64 << 27; 2]),
                )
            }),
            [2; 2]
        );
    }
}

#[test]
fn avx_ne_convert_packed() {
    if !is_x86_feature_detected!("avxneconvert") {
        return;
    }
    let even_bf16 = u16x16(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x3f80 } else { 0x4000 }
    }));
    assert_eq!(f32x8_lanes(unsafe { vcvtneebf162ps(even_bf16) }), [1.0; 8]);
    assert_eq!(f32x8_lanes(unsafe { vcvtneepbf162ps(even_bf16) }), [2.0; 8]);

    let even_half = u16x16(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x3c00 } else { 0x4000 }
    }));
    assert_eq!(f32x8_lanes(unsafe { vcvtneeph2ps(even_half) }), [1.0; 8]);
    assert_eq!(f32x8_lanes(unsafe { vcvtneepsh2ps(even_half) }), [2.0; 8]);
}

#[test]
fn avx_ne_convert_broadcast() {
    if !is_x86_feature_detected!("avxneconvert") {
        return;
    }
    assert_eq!(f32x8_lanes(unsafe { vbcstnebf162ps(0x3f80) }), [1.0; 8]);
    let out = unsafe { vcvtneps2bf16_256(i32x8_as_f32([1.0; 8])) }.to_bytes();
    assert_eq!(u16::from_le_bytes(out[0..2].try_into().unwrap()), 0x3f80);
}

fn i32x8_as_f32(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (lane, value) in values.iter().enumerate() {
        bytes[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V256::from_bytes(bytes)
}
