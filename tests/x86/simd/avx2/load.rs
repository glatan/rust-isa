use rust_isa::x86::simd::avx2::{v256_load, v256_store};

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
