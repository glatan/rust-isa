use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::sse2::V128;

pub(super) fn enabled() -> bool {
    is_x86_feature_detected!("avx")
}

pub(super) fn f32x8(values: [f32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V256::from_bytes(bytes)
}

pub(super) fn f32x8_lanes(value: V256) -> [f32; 8] {
    f32_lanes(value)
}

pub(super) fn f32_lanes(value: V256) -> [f32; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| f32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn f64x4(values: [f64; 4]) -> V256 {
    let mut bytes = [0u8; 32];
    for (chunk, value) in bytes.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V256::from_bytes(bytes)
}

pub(super) fn f64x4_lanes(value: V256) -> [f64; 4] {
    f64_lanes(value)
}

pub(super) fn f64_lanes(value: V256) -> [f64; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| f64::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn i32x8(values: [i32; 8]) -> V256 {
    let mut bytes = [0u8; 32];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V256::from_bytes(bytes)
}

pub(super) fn i32_lanes(value: V256) -> [i32; 8] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| i32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn f32x4(values: [f32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn f32x4_lanes(value: V128) -> [f32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| f32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn f64x2(values: [f64; 2]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn f64x2_lanes(value: V128) -> [f64; 2] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| f64::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn i32x4(values: [i32; 4]) -> V128 {
    let mut bytes = [0u8; 16];
    for (chunk, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(values) {
        *chunk = value.to_le_bytes();
    }
    V128::from_bytes(bytes)
}

pub(super) fn i32x4_lanes(value: V128) -> [i32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| i32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn u32x4_lanes(value: V128) -> [u32; 4] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<4>().0.iter();
    core::array::from_fn(|_| u32::from_le_bytes(*lanes.next().unwrap()))
}

pub(super) fn i64x2_lanes(value: V128) -> [i64; 2] {
    let bytes = value.to_bytes();
    let mut lanes = bytes.as_chunks::<8>().0.iter();
    core::array::from_fn(|_| i64::from_le_bytes(*lanes.next().unwrap()))
}
