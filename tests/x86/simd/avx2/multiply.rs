use rust_isa::x86::simd::avx2::{vpmaddubsw, vpmaddwd, vpmulhrsw, vpmulhuw, vpmulhw, vpsadbw};

use super::support::*;

#[test]
fn multiply_and_sum_of_differences() {
    if !enabled() {
        return;
    }
    assert_eq!(
        i16_lanes(unsafe { vpmulhw(i16x16([1_000; 16]), i16x16([1_000; 16])) }),
        [15; 16]
    );
    assert_eq!(
        u16_lanes(unsafe { vpmulhuw(u16x16([1_000; 16]), u16x16([1_000; 16])) }),
        [15; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmulhrsw(i16x16([16_384; 16]), i16x16([16_384; 16])) }),
        [8_192; 16]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmulhrsw(i16x16([i16::MIN; 16]), i16x16([i16::MIN; 16])) }),
        [i16::MIN; 16]
    );

    let words = i16x16(core::array::from_fn(|lane| lane as i16 + 1));
    assert_eq!(
        i32_lanes(unsafe { vpmaddwd(words, i16x16([1; 16])) }),
        [3, 7, 11, 15, 19, 23, 27, 31]
    );
    assert_eq!(
        i16_lanes(unsafe { vpmaddubsw(u8x32([1; 32]), i8x32([1; 32])) }),
        [2; 16]
    );
    assert_eq!(
        i64_lanes(unsafe { vpsadbw(u8x32([1; 32]), u8x32([0; 32])) }),
        [8; 4]
    );
}
