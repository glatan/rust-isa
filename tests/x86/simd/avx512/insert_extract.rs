use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn avx512f_insert_and_extract() {
    if !f_enabled() {
        return;
    }
    let block = f32x16(core::array::from_fn(|lane| lane as f32));
    let insert = i32x4([100, 101, 102, 103]);
    let inserted = unsafe { vinsertf32x4::<1>(block, insert) };
    assert_eq!(i32_lanes(inserted)[4..8], [100, 101, 102, 103]);
    let extracted = unsafe { vextractf32x4::<2>(block) };
    assert_eq!(f32x4_lanes(extracted), [8.0, 9.0, 10.0, 11.0]);
}
