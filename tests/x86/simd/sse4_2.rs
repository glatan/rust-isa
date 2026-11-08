use rust_isa::x86::simd::sse2::V128;
use rust_isa::x86::simd::sse4_2::{
    crc32_u8, crc32_u16, crc32_u32, crc32_u64, pcmpestri, pcmpestrm, pcmpgtq, pcmpistri, pcmpistrm,
};

#[test]
fn sse42_compare_and_crc32() {
    if !is_x86_feature_detected!("sse4.2") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
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
        eprintln!("rust-isa-skip: CPU lacks required feature");
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

#[test]
fn crc32_widths_match_reference() {
    if !is_x86_feature_detected!("sse4.2") {
        eprintln!("rust-isa-skip: CPU lacks required feature");
        return;
    }
    let crc = 0xffff_ffffu32;
    let value16 = 0x1234u16;
    let value32 = 0x1234_5678u32;
    let value64 = 0x0123_4567_89ab_cdefu64;

    assert_eq!(
        unsafe { crc32_u16(crc, value16) },
        crc32c(crc, &value16.to_le_bytes())
    );
    assert_eq!(
        unsafe { crc32_u32(crc, value32) },
        crc32c(crc, &value32.to_le_bytes())
    );
    assert_eq!(
        unsafe { crc32_u64(u64::from(crc), value64) },
        u64::from(crc32c(crc, &value64.to_le_bytes()))
    );
}

fn crc32c(mut crc: u32, bytes: &[u8]) -> u32 {
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0x82f6_3b78
            } else {
                crc >> 1
            };
        }
    }
    crc
}
