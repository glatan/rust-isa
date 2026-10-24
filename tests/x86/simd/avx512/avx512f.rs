use rust_isa::x86::simd::avx512f::{
    V512, v512_load, v512_store, vpaddd, vpaddq, vpandd, vpbroadcastd, vpermd, vpmulld, vpord,
    vpslld, vpsrld, vpsubd, vpsubq, vpxord,
};

use super::support::*;

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
fn avx512f_integer_arithmetic() {
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
fn avx512f_bitwise_permute_broadcast() {
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
fn subtract_bitwise_and_shift() {
    if !f_enabled() {
        return;
    }
    let a = i32x16(core::array::from_fn(|index| index as i32 + 1));
    let b = i32x16(core::array::from_fn(|index| (index as i32 + 1) * 10));
    assert_eq!(
        i32_lanes(unsafe { vpsubd(b, a) }),
        core::array::from_fn(|index| (index as i32 + 1) * 9)
    );
    assert_eq!(
        i64_lanes(unsafe {
            vpsubq(
                i64x8([0, 5, -1, 7, 9, -9, 2, 3]),
                i64x8([1, 2, 1, 3, 4, -1, 2, 4]),
            )
        }),
        [-1, 3, -2, 4, 5, -8, 0, -1]
    );

    let a = V512::from_bytes([0b1010_1010; 64]);
    let b = V512::from_bytes([0b1100_1100; 64]);
    assert_eq!(unsafe { vpord(a, b) }.to_bytes(), [0b1110_1110; 64]);
    assert_eq!(unsafe { vpxord(a, b) }.to_bytes(), [0b0110_0110; 64]);

    assert_eq!(
        i32_lanes(unsafe { vpslld(i32x16(core::array::from_fn(|index| index as i32 + 1)), 1) }),
        core::array::from_fn(|index| (index as i32 + 1) * 2)
    );
    assert_eq!(
        i32_lanes(unsafe { vpsrld(i32x16([-1; 16]), 1) }),
        [0x7fff_ffff; 16]
    );
}
