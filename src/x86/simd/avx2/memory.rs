//! 256-bit load and store instructions.

use super::V256;

unsafe extern "C" {
    fn rust_isa_avx2_v256_load(p: *const u8, out: *mut u8);
    fn rust_isa_avx2_v256_store(p: *mut u8, value: *const u8);
}

/// `v256.load`
///
/// Load 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2. `p` must be valid for reading 32 bytes. The
/// load does not require alignment.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn v256_load(p: *const u8) -> V256 {
    let mut out = V256::from_bytes([0; 32]);
    unsafe {
        rust_isa_avx2_v256_load(p, out.as_mut_ptr());
    }
    out
}

/// `v256.store`
///
/// Store 32 bytes.
///
/// # Safety
///
/// The caller must enable AVX2. `p` must be valid for writing 32 bytes. The
/// store does not require alignment.
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn v256_store(p: *mut u8, value: V256) {
    unsafe {
        rust_isa_avx2_v256_store(p, value.as_ptr());
    }
}
