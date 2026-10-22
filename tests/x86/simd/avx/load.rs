use rust_isa::x86::simd::avx::vlddqu;

use super::support::*;

#[test]
fn unaligned_load() {
    if !enabled() {
        return;
    }
    let source = [0u8; 33];
    assert_eq!(
        unsafe { vlddqu(source.as_ptr().add(1)) }.to_bytes(),
        [0; 32]
    );
}
