use rust_isa::x86::simd::avx512f::V512;
use rust_isa::x86::simd::avx512vbmi::{vpermb, vpermi2b, vpermt2b};

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

    let table_a = V512::from_bytes(core::array::from_fn::<u8, 64, _>(|index| index as u8));
    let table_b = V512::from_bytes(core::array::from_fn::<u8, 64, _>(|index| index as u8 + 64));
    let index_a = table_a;
    let index_b = table_b;
    assert_eq!(
        unsafe { vpermi2b(table_a, table_b, index_a) }.to_bytes(),
        table_a.to_bytes()
    );
    assert_eq!(
        unsafe { vpermi2b(table_a, table_b, index_b) }.to_bytes(),
        table_b.to_bytes()
    );
    assert_eq!(
        unsafe { vpermt2b(table_a, table_b, index_a) }.to_bytes(),
        table_a.to_bytes()
    );
    assert_eq!(
        unsafe { vpermt2b(table_a, table_b, index_b) }.to_bytes(),
        table_b.to_bytes()
    );
}
