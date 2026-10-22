use rust_isa::x86::simd::avx2::{
    V256, v256_load, v256_store, vpaddb, vpaddd, vpaddq, vpaddw, vpcmpeqd, vpcmpeqw, vpmulld,
    vpmullw, vpslld, vpsrld, vpsubb, vpsubd, vpsubq, vpsubw,
};

use super::support::*;

#[test]
fn load_store_roundtrip() {
    if !enabled() {
        return;
    }
    let source = core::array::from_fn::<u8, 32, _>(|index| index as u8);
    let value = unsafe { v256_load(source.as_ptr()) };
    assert_eq!(value.to_bytes(), source);

    let mut output = [0u8; 32];
    unsafe {
        v256_store(output.as_mut_ptr(), value);
    }
    assert_eq!(output, source);
}

#[test]
fn integer_arithmetic() {
    if !enabled() {
        return;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i32_lanes(unsafe { vpaddd(a, b) }),
        [11, 22, 33, 44, 55, 66, 77, 88]
    );
    assert_eq!(
        i32_lanes(unsafe { vpmulld(a, b) }),
        [10, 40, 90, 160, 250, 360, 490, 640]
    );

    let bytes = V256::from_bytes([1; 32]);
    assert_eq!(unsafe { vpaddb(bytes, bytes) }.to_bytes(), [2; 32]);
}

#[test]
fn byte_and_word_arithmetic() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpsubb(V256::from_bytes([10; 32]), V256::from_bytes([3; 32])) }.to_bytes(),
        [7; 32]
    );
    let a = i16x16(core::array::from_fn(|index| index as i16 + 1));
    let b = i16x16(core::array::from_fn(|index| (index as i16 + 1) * 10));
    assert_eq!(
        i16_lanes(unsafe { vpaddw(a, b) }),
        core::array::from_fn(|index| (index as i16 + 1) * 11)
    );
    assert_eq!(
        i16_lanes(unsafe { vpsubw(b, a) }),
        core::array::from_fn(|index| (index as i16 + 1) * 9)
    );
    assert_eq!(
        i16_lanes(unsafe { vpmullw(i16x16([300; 16]), i16x16([3; 16])) }),
        [900; 16]
    );
}

#[test]
fn dword_and_qword_arithmetic() {
    if !enabled() {
        return;
    }
    let a = i32x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = i32x8([10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(
        i32_lanes(unsafe { vpsubd(b, a) }),
        [9, 18, 27, 36, 45, 54, 63, 72]
    );
    assert_eq!(
        i64_lanes(unsafe { vpaddq(i64x4([1, i64::MAX, -1, 0]), i64x4([2, 1, 1, 5])) }),
        [3, i64::MIN, 0, 5]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsubq(i64x4([0, 5, -1, 7]), i64x4([1, 2, 1, 3])) }),
        [-1, 3, -2, 4]
    );
}

#[test]
fn compare_and_shift() {
    if !enabled() {
        return;
    }
    assert_eq!(
        unsafe { vpcmpeqw(i16x16([5; 16]), i16x16([5; 16])) }.to_bytes(),
        [0xff; 32]
    );
    assert_eq!(
        unsafe { vpcmpeqd(i32x8([5; 8]), i32x8([6; 8])) }.to_bytes(),
        [0; 32]
    );
    assert_eq!(
        i32_lanes(unsafe { vpslld(i32x8([1, 2, 3, 4, 5, 6, 7, 8]), 1) }),
        [2, 4, 6, 8, 10, 12, 14, 16]
    );
    assert_eq!(
        i32_lanes(unsafe { vpsrld(i32x8([-1; 8]), 1) }),
        [0x7fff_ffff; 8]
    );
}
