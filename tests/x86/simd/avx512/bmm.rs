//! AVX512BMM value tests.

#[cfg(target_arch = "x86_64")]
fn available() -> bool {
    use core::arch::x86_64::__cpuid;

    let max = unsafe { __cpuid(0x8000_0000) }.eax;
    max >= 0x8000_0021 && unsafe { __cpuid(0x8000_0021) }.eax & (1 << 23) != 0
}

#[cfg(target_arch = "x86_64")]
fn identity16() -> [u8; 32] {
    let mut matrix = [0u8; 32];
    for row in 0..16 {
        matrix[row * 2 + row / 8] |= 1 << (row % 8);
    }
    matrix
}

#[cfg(target_arch = "x86_64")]
#[test]
fn avx512bmm_bitrevb_values() {
    use rust_isa::x86::simd::avx2::V256;
    use rust_isa::x86::simd::avx512bmm::{vbitrevb_v128, vbitrevb_v256, vbitrevb_v512};
    use rust_isa::x86::simd::avx512f::V512;
    use rust_isa::x86::simd::sse2::V128;

    if !available() {
        return;
    }
    let bytes = [0x01, 0x02, 0x80, 0xff, 0x0f, 0xf0, 0x55, 0xaa];
    let reversed = [0x80, 0x40, 0x01, 0xff, 0xf0, 0x0f, 0xaa, 0x55];
    let v128 = V128::from_bytes(core::array::from_fn(|index| bytes[index % 8]));
    let v256 = V256::from_bytes(core::array::from_fn(|index| bytes[index % 8]));
    let v512 = V512::from_bytes(core::array::from_fn(|index| bytes[index % 8]));
    assert_eq!(
        unsafe { vbitrevb_v128(v128) }.to_bytes(),
        core::array::from_fn::<u8, 16, _>(|index| reversed[index % 8])
    );
    assert_eq!(
        unsafe { vbitrevb_v256(v256) }.to_bytes(),
        core::array::from_fn::<u8, 32, _>(|index| reversed[index % 8])
    );
    assert_eq!(
        unsafe { vbitrevb_v512(v512) }.to_bytes(),
        core::array::from_fn::<u8, 64, _>(|index| reversed[index % 8])
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn avx512bmm_matrix_values() {
    use rust_isa::x86::simd::avx2::V256;
    use rust_isa::x86::simd::avx512bmm::{
        vbmacor16x16x16_v256, vbmacor16x16x16_v512, vbmacxor16x16x16_v256, vbmacxor16x16x16_v512,
    };
    use rust_isa::x86::simd::avx512f::V512;

    if !available() {
        return;
    }
    let identity = identity16();
    let pattern = [0xaa; 32];
    let zeros = [0u8; 32];
    let ones = [0xff; 32];

    let identity256 = V256::from_bytes(identity);
    let pattern256 = V256::from_bytes(pattern);
    let zeros256 = V256::from_bytes(zeros);
    let ones256 = V256::from_bytes(ones);

    assert_eq!(
        unsafe { vbmacor16x16x16_v256(identity256, pattern256, zeros256) }.to_bytes(),
        pattern
    );
    assert_eq!(
        unsafe { vbmacxor16x16x16_v256(identity256, pattern256, zeros256) }.to_bytes(),
        pattern
    );
    assert_eq!(
        unsafe { vbmacor16x16x16_v256(identity256, pattern256, ones256) }.to_bytes(),
        ones
    );
    assert_eq!(
        unsafe { vbmacxor16x16x16_v256(identity256, pattern256, ones256) }.to_bytes(),
        core::array::from_fn::<u8, 32, _>(|index| !pattern[index])
    );

    let mut identity512 = [0u8; 64];
    identity512[..32].copy_from_slice(&identity);
    identity512[32..].copy_from_slice(&identity);
    let mut pattern512 = [0u8; 64];
    pattern512[..32].copy_from_slice(&[0xaa; 32]);
    pattern512[32..].copy_from_slice(&[0x55; 32]);
    assert_eq!(
        unsafe {
            vbmacor16x16x16_v512(
                V512::from_bytes(identity512),
                V512::from_bytes(pattern512),
                V512::from_bytes([0; 64]),
            )
        }
        .to_bytes(),
        pattern512
    );
    assert_eq!(
        unsafe {
            vbmacxor16x16x16_v512(
                V512::from_bytes(identity512),
                V512::from_bytes(pattern512),
                V512::from_bytes([0; 64]),
            )
        }
        .to_bytes(),
        pattern512
    );
}
