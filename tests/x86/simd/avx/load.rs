use rust_isa::x86::simd::avx::vlddqu;

use super::support::*;

#[test]
fn unaligned_load() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let source = core::array::from_fn::<u8, 33, _>(|index| index as u8);
    assert_eq!(
        unsafe { vlddqu(source.as_ptr().add(1)) }.to_bytes(),
        core::array::from_fn::<u8, 32, _>(|index| index as u8 + 1)
    );
}
