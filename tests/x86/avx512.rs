use rust_isa::x86::simd::avx2::V256;
use rust_isa::x86::simd::avx512bw::vpaddb;
use rust_isa::x86::simd::avx512dq::vpmullq;
use rust_isa::x86::simd::avx512f::{
    V512, v512_load, v512_store, vpaddd, vpaddq, vpandd, vpbroadcastd, vpermd, vpmulld,
};
use rust_isa::x86::simd::avx512ifma::{vpmadd52huq, vpmadd52luq};
use rust_isa::x86::simd::avx512vbmi::vpermb;
use rust_isa::x86::simd::avx512vl::{vpmullq_v128, vpmullq_v256};
use rust_isa::x86::simd::avx512vnni::vpdpbusd;
use rust_isa::x86::simd::sse2::V128;

fn f_enabled() -> bool {
    is_x86_feature_detected!("avx512f")
}

fn bw_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512bw")
}

fn dq_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512dq")
}

fn ifma_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512ifma")
}

fn vbmi_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512vbmi")
}

fn vnni_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512vnni")
}

fn vl_enabled() -> bool {
    f_enabled() && is_x86_feature_detected!("avx512dq") && is_x86_feature_detected!("avx512vl")
}

fn i32x16(values: [i32; 16]) -> V512 {
    let mut bytes = [0u8; 64];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn i32_lanes(value: V512) -> [i32; 16] {
    let bytes = value.to_bytes();
    let mut lanes = [0i32; 16];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = i32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap());
    }
    lanes
}

fn i64x8(values: [i64; 8]) -> V512 {
    let mut bytes = [0u8; 64];
    for (index, value) in values.iter().enumerate() {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    V512::from_bytes(bytes)
}

fn i64_lanes(value: V512) -> [i64; 8] {
    let bytes = value.to_bytes();
    let mut lanes = [0i64; 8];
    for (index, lane) in lanes.iter_mut().enumerate() {
        *lane = i64::from_le_bytes(bytes[index * 8..index * 8 + 8].try_into().unwrap());
    }
    lanes
}

#[test]
fn load_store_roundtrip() {
    if !f_enabled() {
        return;
    }
    let source = core::array::from_fn::<u8, 64, _>(|index| index as u8);
    let value = unsafe { v512_load(source.as_ptr()) };
    assert_eq!(value.to_bytes(), source);
    let mut output = [0u8; 64];
    unsafe {
        v512_store(output.as_mut_ptr(), value);
    }
    assert_eq!(output, source);
}

#[test]
fn foundation_integer_arithmetic() {
    if !f_enabled() {
        return;
    }
    let a = i32x16(core::array::from_fn::<i32, 16, _>(|index| index as i32 + 1));
    let b = i32x16(core::array::from_fn::<i32, 16, _>(|index| {
        (index as i32 + 1) * 10
    }));
    assert_eq!(
        i32_lanes(unsafe { vpaddd(a, b) }),
        core::array::from_fn::<i32, 16, _>(|index| (index as i32 + 1) * 11)
    );
    assert_eq!(
        i32_lanes(unsafe { vpmulld(a, b) }),
        core::array::from_fn::<i32, 16, _>(|index| (index as i32 + 1) * (index as i32 + 1) * 10)
    );
    assert_eq!(
        i64_lanes(unsafe { vpaddq(i64x8([1; 8]), i64x8([2; 8])) }),
        [3; 8]
    );
}

#[test]
fn foundation_bitwise_permute_broadcast() {
    if !f_enabled() {
        return;
    }
    let a = V512::from_bytes([0b1010_1010; 64]);
    let b = V512::from_bytes([0b1100_1100; 64]);
    assert_eq!(unsafe { vpandd(a, b) }.to_bytes(), [0b1000_1000; 64]);

    let indices = i32x16(core::array::from_fn::<i32, 16, _>(|index| {
        15 - index as i32
    }));
    let values = i32x16(core::array::from_fn::<i32, 16, _>(|index| index as i32));
    assert_eq!(
        i32_lanes(unsafe { vpermd(values, indices) }),
        core::array::from_fn::<i32, 16, _>(|index| 15 - index as i32)
    );
    assert_eq!(i32_lanes(unsafe { vpbroadcastd(9) }), [9; 16]);
}

#[test]
fn bw_and_dq() {
    if !bw_enabled() {
        return;
    }
    let bytes = V512::from_bytes([250; 64]);
    assert_eq!(
        unsafe { vpaddb(bytes, V512::from_bytes([10; 64])) }.to_bytes(),
        [4; 64]
    );

    if !dq_enabled() {
        return;
    }
    assert_eq!(
        i64_lanes(unsafe { vpmullq(i64x8([3; 8]), i64x8([5; 8])) }),
        [15; 8]
    );
}

#[test]
fn ifma() {
    if !ifma_enabled() {
        return;
    }
    let acc = i64x8([7; 8]);
    let a = i64x8([3; 8]);
    let b = i64x8([5; 8]);
    assert_eq!(i64_lanes(unsafe { vpmadd52luq(acc, a, b) }), [22; 8]);
    assert_eq!(i64_lanes(unsafe { vpmadd52huq(acc, a, b) }), [7; 8]);
}

#[test]
fn vbmi() {
    if !vbmi_enabled() {
        return;
    }
    let source = V512::from_bytes(core::array::from_fn::<u8, 64, _>(|index| index as u8));
    let reverse = V512::from_bytes(core::array::from_fn::<u8, 64, _>(|index| 63 - index as u8));
    assert_eq!(
        unsafe { vpermb(source, reverse) }.to_bytes(),
        core::array::from_fn::<u8, 64, _>(|index| 63 - index as u8)
    );
}

#[test]
fn vnni() {
    if !vnni_enabled() {
        return;
    }
    let acc = V512::from_bytes([0; 64]);
    let a = V512::from_bytes([1; 64]);
    let b = V512::from_bytes([1; 64]);
    assert_eq!(i32_lanes(unsafe { vpdpbusd(acc, a, b) }), [4; 16]);
}

#[test]
fn vl_mullo() {
    if !vl_enabled() {
        return;
    }
    let a = V128::from_bytes([3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0]);
    let b = V128::from_bytes([7, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0]);
    let c = unsafe { vpmullq_v128(a, b) };
    assert_eq!(&c.to_bytes()[0..8], &21i64.to_le_bytes());
    assert_eq!(&c.to_bytes()[8..16], &55i64.to_le_bytes());

    let mut bytes = [0u8; 32];
    for index in 0..4 {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&(index as i64 + 2).to_le_bytes());
    }
    let a = V256::from_bytes(bytes);
    for index in 0..4 {
        bytes[index * 8..index * 8 + 8].copy_from_slice(&(index as i64 + 3).to_le_bytes());
    }
    let b = V256::from_bytes(bytes);
    let c = unsafe { vpmullq_v256(a, b) };
    let out = c.to_bytes();
    for index in 0..4 {
        assert_eq!(
            i64::from_le_bytes(out[index * 8..index * 8 + 8].try_into().unwrap()),
            (index as i64 + 2) * (index as i64 + 3)
        );
    }
}
