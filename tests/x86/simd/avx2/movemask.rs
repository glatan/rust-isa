use rust_isa::x86::simd::avx2::{V256, vpmovmskb};

use super::support::*;

#[test]
fn movemask() {
    if !enabled() {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let mut bytes = [0u8; 32];
    bytes[0] = 0x80;
    bytes[3] = 0x81;
    bytes[31] = 0xff;
    assert_eq!(unsafe { vpmovmskb(V256::from_bytes(bytes)) }, 0x8000_0009);
}
