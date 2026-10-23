use rust_isa::x86::simd::avx2::{V256, vpblendvb};

use super::support::*;

#[test]
fn blend_variable() {
    if !enabled() {
        return;
    }
    let mask = V256::from_bytes(core::array::from_fn(|index| {
        if index % 2 == 0 { 0x80 } else { 0x00 }
    }));
    assert_eq!(
        unsafe { vpblendvb(V256::from_bytes([1; 32]), V256::from_bytes([2; 32]), mask) }.to_bytes(),
        core::array::from_fn(|index| if index % 2 == 0 { 2 } else { 1 })
    );
}
