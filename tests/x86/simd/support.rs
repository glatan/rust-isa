use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::sse2::V128;

pub(super) fn i64_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    [
        i64::from_le_bytes(bytes[..8].try_into().unwrap()),
        i64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

pub(super) fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn f32_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| f32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&values[0].to_le_bytes());
    bytes[8..].copy_from_slice(&values[1].to_le_bytes());
    V128::from_bytes(bytes)
}

pub(super) fn f64_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    [
        f64::from_le_bytes(bytes[..8].try_into().unwrap()),
        f64::from_le_bytes(bytes[8..].try_into().unwrap()),
    ]
}

pub(super) fn i16x8(values: [i16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<2>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn i16_lanes(value: V128) -> [i16; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<2>().0.iter();
    core::array::from_fn(|_| i16::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn i32_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| i32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn u16x8(values: [u16; 8]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<2>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn u16_lanes(value: V128) -> [u16; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<2>().0.iter();
    core::array::from_fn(|_| u16::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V256::from_bytes(bytes)
}

pub(super) fn f32x8_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| f32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn f64x4_lanes(value: V256) -> [f64; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| f64::from_le_bytes(*lanes.next().unwrap()))
}
