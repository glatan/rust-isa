//! RDSEED.
//!
//! RDSEED is not part of the x86-64 baseline. Every operation is `unsafe` and
//! the caller must enable RDSEED with `#[target_feature(enable = "rdseed")]` or
//! a global target feature.
//!
//! RDSEED is intended to seed another random-number generator. The function
//! returns `None` when the hardware reports that no seed is available; the
//! caller may retry.

unsafe extern "C" {
    fn rust_isa_rdseed_u32(out: *mut u32) -> i32;
    #[cfg(target_arch = "x86_64")]
    fn rust_isa_rdseed_u64(out: *mut u64) -> i32;
}

/// `rdseed` with a 32-bit destination.
///
/// # Safety
///
/// The caller must enable RDSEED.
#[target_feature(enable = "rdseed")]
#[inline]
pub unsafe fn rdseed_u32() -> Option<u32> {
    let mut value = 0u32;
    if unsafe { rust_isa_rdseed_u32(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}

/// `rdseed` with a 64-bit destination.
///
/// # Safety
///
/// The caller must enable RDSEED.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "rdseed")]
#[inline]
pub unsafe fn rdseed_u64() -> Option<u64> {
    let mut value = 0u64;
    if unsafe { rust_isa_rdseed_u64(&mut value) } != 0 {
        Some(value)
    } else {
        None
    }
}
