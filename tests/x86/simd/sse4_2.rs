use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::sse4_2::{crc32_u8, pcmpestri, pcmpestrm, pcmpgtq, pcmpistri, pcmpistrm};

#[test]
fn sse42_compare_and_crc32() {
    if !is_x86_feature_detected!("sse4.2") {
        return;
    }
    let a = V128::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]);
    let b = V128::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out = unsafe { pcmpgtq(a, b) }.to_bytes();
    assert_eq!(&out[..8], &[0xff; 8]);
    assert_eq!(&out[8..], &[0; 8]);

    let mut crc = 0xffff_ffffu32;
    for byte in b"123456789" {
        crc = unsafe { crc32_u8(crc, *byte) };
    }
    assert_eq!(crc ^ 0xffff_ffff, 0xe306_9283);
}

#[test]
fn sse42_string_comparisons() {
    if !is_x86_feature_detected!("sse4.2") {
        return;
    }
    let mut needle = [0u8; 16];
    needle[..2].copy_from_slice(b"lo");
    let mut haystack = [0u8; 16];
    haystack[..5].copy_from_slice(b"hello");
    let needle = V128::from_bytes(needle);
    let haystack = V128::from_bytes(haystack);

    assert_eq!(unsafe { pcmpistri(needle, haystack, 0x00) }, 2);
    assert_eq!(unsafe { pcmpestri(needle, 2, haystack, 5, 0x00) }, 2);
    assert_eq!(
        &unsafe { pcmpistrm(needle, haystack, 0x00) }.to_bytes()[..2],
        &[0x1c, 0x00]
    );
    assert_eq!(
        &unsafe { pcmpestrm(needle, 2, haystack, 5, 0x00) }.to_bytes()[..2],
        &[0x1c, 0x00]
    );
}
