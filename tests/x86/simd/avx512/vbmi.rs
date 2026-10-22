use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512vbmi::vpermb;

use super::support::*;

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
