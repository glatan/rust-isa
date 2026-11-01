//! Non-temporal store instructions.

use super::V128;

unsafe extern "C" {
    fn rust_isa_sse2_movntps(p: *mut f32, a: *const u8);
    fn rust_isa_sse2_movntpd(p: *mut f64, a: *const u8);
    fn rust_isa_sse2_movntdq(p: *mut u8, a: *const u8);
    fn rust_isa_sse2_movnti(p: *mut i32, value: i32);
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_sse2_movnti_u64(p: *mut u64, value: u64);
}

/// `movntps`
///
/// Store four 32-bit float lanes with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes and aligned to 16 bytes.
#[inline(always)]
pub unsafe fn movntps(p: *mut f32, a: V128) {
    unsafe {
        rust_isa_sse2_movntps(p, a.as_ptr());
    }
}

/// `movntpd`
///
/// Store two 64-bit float lanes with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes and aligned to 16 bytes.
#[inline(always)]
pub unsafe fn movntpd(p: *mut f64, a: V128) {
    unsafe {
        rust_isa_sse2_movntpd(p, a.as_ptr());
    }
}

/// `movntdq`
///
/// Store 16 bytes with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 16 bytes and aligned to 16 bytes.
#[inline(always)]
pub unsafe fn movntdq(p: *mut u8, a: V128) {
    unsafe {
        rust_isa_sse2_movntdq(p, a.as_ptr());
    }
}

/// `movnti`
///
/// Store a 32-bit integer with a non-temporal hint.
///
/// # Safety
///
/// `p` must be valid for writing 4 bytes.
#[inline(always)]
pub unsafe fn movnti(p: *mut i32, value: i32) {
    unsafe {
        rust_isa_sse2_movnti(p, value);
    }
}

/// `movnti` with a 64-bit destination.
///
/// Store 64 bits with a non-temporal hint.
///
/// # Safety
///
/// The caller must enable SSE2. `p` must be valid for writing 8 bytes and
/// aligned to 8 bytes. Non-temporal stores require an `sfence` before another
/// agent reads the stored data.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn movnti_u64(p: *mut u64, value: u64) {
    unsafe {
        rust_isa_sse2_movnti_u64(p, value);
    }
}
