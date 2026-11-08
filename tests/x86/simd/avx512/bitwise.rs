use rust_isa::x86::simd::avx512f::*;

use super::support::*;

#[test]
fn avx512f_bitwise() {
    if !f_enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let a = V512::from_bytes([0xf0; 64]);
    let b = V512::from_bytes([0xcc; 64]);
    assert_eq!(unsafe { vpandq(a, b) }.to_bytes(), [0xc0; 64]);
    assert_eq!(unsafe { vpandnd(a, b) }.to_bytes(), [0x0c; 64]);
    assert_eq!(unsafe { vporq(a, b) }.to_bytes(), [0xfc; 64]);
    assert_eq!(unsafe { vpxorq(a, b) }.to_bytes(), [0x3c; 64]);

    let c = V512::from_bytes([0xaa; 64]);
    assert_eq!(
        unsafe { vpternlogd::<0x96>(a, b, c) }.to_bytes(),
        [0x96; 64]
    );
    assert_eq!(
        unsafe { vpternlogd::<0x00>(a, b, c) }.to_bytes(),
        [0x00; 64]
    );
    assert_eq!(
        unsafe { vpternlogd::<0xff>(a, b, c) }.to_bytes(),
        [0xff; 64]
    );
}
