//! Load, store, and masked store instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_v128_load(p: *const u8, out: *mut u8);
    fn rust_isa_sse2_v128_store(p: *mut u8, value: *const u8);
    fn rust_isa_sse2_maskmovdqu(a: *const u8, mask: *const u8, p: *mut u8);
}

/// `v128.load`
///
/// Load 16 bytes.
///
/// # Safety
///
/// `p` must be valid for reading 16 bytes. The load does not require alignment.
#[inline(always)]
pub unsafe fn v128_load(p: *const u8) -> V128 {
    let mut out = V128::from_bytes([0; 16]);
    unsafe {
        rust_isa_sse2_v128_load(p, out.as_mut_ptr());
    }
    out
}

/// `v128.store`
///
/// Store 16 bytes.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes. The store does not require alignment.
#[inline(always)]
pub unsafe fn v128_store(p: *mut u8, value: V128) {
    unsafe {
        rust_isa_sse2_v128_store(p, value.as_ptr());
    }
}

/// `maskmovdqu`
///
/// Store each byte of `a` to `p` when the corresponding byte of `mask` has its
/// most significant bit set.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes. Bytes whose mask bit is not set are
/// not written.
#[inline(always)]
pub unsafe fn maskmovdqu(a: V128, mask: V128, p: *mut u8) {
    unsafe {
        rust_isa_sse2_maskmovdqu(a.as_ptr(), mask.as_ptr(), p);
    }
}
